// SPDX-License-Identifier: GPL-3.0-only
#[macro_use]
extern crate tracing;

mod a11y;
mod comp;
mod notifications;
mod process;
mod service;
mod systemd;

use async_signals::Signals;
use color_eyre::{Result, eyre::WrapErr};
use cosmic_notifications_util::{DAEMON_NOTIFICATIONS_FD, PANEL_NOTIFICATIONS_FD};
use futures_util::StreamExt;
#[cfg(feature = "autostart")]
use itertools::Itertools;
use launch_pad::{ProcessManager, process::Process};
use service::SessionRequest;
#[cfg(feature = "autostart")]
use std::collections::HashSet;
#[cfg(feature = "autostart")]
use std::path::PathBuf;
#[cfg(feature = "autostart")]
use std::process::{Command, Stdio};
use std::{borrow::Cow, env, os::fd::AsRawFd, sync::Arc};
#[cfg(feature = "systemd")]
use systemd::{get_systemd_env, is_systemd_used, spawn_scope};
use tokio::{
	sync::{
		Mutex,
		mpsc::{Receiver, Sender},
		oneshot,
	},
	time::Duration,
};
use tokio_util::sync::CancellationToken;
use tracing::{Instrument, metadata::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::notifications::notifications_process;
#[cfg(feature = "autostart")]
const AUTOSTART_DIR: &'static str = "autostart";
#[cfg(feature = "autostart")]
const ENVIRONMENT_NAME: &'static str = "COSMIC";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	color_eyre::install().wrap_err("failed to install color_eyre error handler")?;

	let trace = tracing_subscriber::registry();
	let env_filter = EnvFilter::builder()
		.with_default_directive(LevelFilter::INFO.into())
		.from_env_lossy();

	#[cfg(feature = "systemd")]
	if let Ok(journald) = tracing_journald::layer() {
		trace
			.with(journald)
			.with(env_filter)
			.try_init()
			.wrap_err("failed to initialize logger")?;
	} else {
		trace
			.with(fmt::layer())
			.with(env_filter)
			.try_init()
			.wrap_err("failed to initialize logger")?;
		warn!("failed to connect to journald")
	}

	#[cfg(not(feature = "systemd"))]
	trace
		.with(fmt::layer())
		.with(env_filter)
		.try_init()
		.wrap_err("failed to initialize logger")?;

	log_panics::init();

	let (session_tx, mut session_rx) = tokio::sync::mpsc::channel(10);
	let session_tx_clone = session_tx.clone();
	let _conn = zbus::connection::Builder::session()?
		.name("com.system76.CosmicSession")?
		.serve_at(
			"/com/system76/CosmicSession",
			service::SessionService { session_tx },
		)?
		.build()
		.await?;

	loop {
		match start(session_tx_clone.clone(), &mut session_rx).await {
			Ok(Status::Exited) => {
				info!("Exited cleanly");
				break;
			}
			Ok(Status::Restarted) => {
				info!("Restarting");
			}
			Err(error) => {
				error!("Restarting after error: {:?}", error);
			}
		};
		// Drain the session channel.
		while session_rx.try_recv().is_ok() {}
	}
	Ok(())
}

// Attach-mode startup tracing. Set COSMIC_SESSION_DEBUG=1 to append markers to
// /tmp/cs-debug.log (used to pin the systemd/logind-stall bug on the greetd vt=none
// session); off by default so release sessions leave no cruft.
macro_rules! dbg_log {
	($($arg:tt)*) => {{
		if std::env::var_os("COSMIC_SESSION_DEBUG").is_some() {
			use std::io::Write as _;
			if let Ok(mut f) = std::fs::OpenOptions::new()
				.create(true)
				.append(true)
				.open("/tmp/cs-debug.log")
			{
				let _ = writeln!(f, $($arg)*);
			}
		}
	}};
}

#[derive(Debug)]
pub enum Status {
	Restarted,
	Exited,
}

async fn start(
	session_tx: Sender<SessionRequest>,
	session_rx: &mut Receiver<SessionRequest>,
) -> Result<Status> {
	info!("Starting cosmic-session");

	let mut args = env::args().skip(1);
	let (executable, args) = (
		args.next().unwrap_or_else(|| String::from("cosmic-comp")),
		args.collect::<Vec<_>>(),
	);

	// Attach mode (global compositor): a persistent cosmic-comp already owns
	// the display from boot. Instead of spawning our own compositor and reading its
	// env back over the IPC channel, we inherit the running compositor's
	// WAYLAND_DISPLAY and bring up only the session components as its clients — no
	// second compositor, no VT switch, no hand-off.
	let attach = env::var_os("COSMIC_SESSION_ATTACH").is_some();

	let process_manager = ProcessManager::new().await;
	_ = process_manager.set_max_restarts(usize::MAX).await;
	_ = process_manager
		.set_restart_mode(launch_pad::RestartMode::ExponentialBackoff(
			Duration::from_millis(10),
		))
		.await;
	let token = CancellationToken::new();

	let (compositor_handle, mut env_vars) = if attach {
		let wayland_display = env::var("WAYLAND_DISPLAY").wrap_err(
			"COSMIC_SESSION_ATTACH is set but WAYLAND_DISPLAY was not inherited from the compositor",
		)?;
		let mut env_vars = vec![("WAYLAND_DISPLAY".to_string(), wayland_display)];
		if let Ok(display) = env::var("DISPLAY") {
			env_vars.push(("DISPLAY".to_string(), display));
		}
		info!("attach mode: using inherited compositor env: {:?}", env_vars);
		(None, env_vars)
	} else {
		let (env_tx, env_rx) = oneshot::channel();
		let compositor_handle = comp::run_compositor(
			&process_manager,
			executable.clone(),
			args,
			token.child_token(),
			env_tx,
			session_tx,
		)
		.wrap_err("failed to start compositor")?;
		let env_vars = env_rx
			.await
			.expect("failed to receive environmental variables")
			.into_iter()
			.collect::<Vec<_>>();
		info!(
			"got environmental variables from cosmic-comp: {:?}",
			env_vars
		);
		(Some(compositor_handle), env_vars)
	};

	// now that cosmic-comp is ready, set XDG_SESSION_TYPE=wayland for new processes
	env_vars.push(("XDG_SESSION_TYPE".to_string(), "wayland".to_string()));
	dbg_log!("CS-DBG attach={}: about to set_systemd_environment", attach);
	let _ = tokio::time::timeout(
		std::time::Duration::from_secs(4),
		systemd::set_systemd_environment("XDG_SESSION_TYPE", "wayland"),
	)
	.await;
	dbg_log!(
		"CS-DBG: set_systemd_environment done; is_systemd_used={}",
		*is_systemd_used()
	);

	#[cfg(feature = "systemd")]
	let _inhibit_fd = if *is_systemd_used() {
		dbg_log!("CS-DBG: about to get_systemd_env");
		match tokio::time::timeout(std::time::Duration::from_secs(4), get_systemd_env()).await {
			Err(_) => {
				dbg_log!("CS-DBG: get_systemd_env TIMED OUT");
			}
			Ok(Err(err)) => {
				warn!("Failed to sync systemd environment {}.", err);
			}
			Ok(Ok(env)) => {
				for systemd_env in env {
					// Only update the envvar if unset
					if std::env::var_os(&systemd_env.key) == None {
						// Blacklist of envvars that we shouldn't touch (taken from KDE)
						if (!systemd_env.key.starts_with("XDG_")
							|| systemd_env.key == "XDG_DATA_DIRS"
							|| systemd_env.key == "XDG_CONFIG_DIRS")
							&& systemd_env.key != "DISPLAY"
							&& systemd_env.key != "XAUTHORITY"
							&& systemd_env.key != "WAYLAND_DISPLAY"
							&& systemd_env.key != "WAYLAND_SOCKET"
							&& systemd_env.key != "_"
							&& systemd_env.key != "SHELL"
							&& systemd_env.key != "SHLVL"
						{
							env_vars.push((systemd_env.key, systemd_env.value));
						}
					}
				}
			}
		};
		dbg_log!("CS-DBG: about to logind inhibit (system-bus connect)");
		#[cfg(feature = "logind")]
		let inhibit = match tokio::time::timeout(std::time::Duration::from_secs(4), async {
			let connection = zbus::Connection::system().await?;
			let proxy = logind_zbus::manager::ManagerProxy::new(&connection).await?;
			proxy
				.inhibit(
					logind_zbus::manager::InhibitType::HandlePowerKey,
					"Cosmic Session",
					"Show confirmation dialog.",
					"block",
				)
				.await
		})
		.await
		{
			Ok(Ok(fd)) => Some(fd),
			Ok(Err(err)) => {
				error!("Failed to set up power-key inhibitor {err:?}");
				None
			}
			Err(_) => {
				dbg_log!("CS-DBG: logind inhibit TIMED OUT");
				None
			}
		};
		dbg_log!("CS-DBG: logind inhibit handled");
		#[cfg(feature = "logind")]
		{
			inhibit
		}
		#[cfg(not(feature = "logind"))]
		None
	} else {
		dbg_log!("CS-DBG: is_systemd_used=false; skipped systemd block");
		None
	};
	dbg_log!("CS-DBG: past _inhibit_fd block; starting settings-daemon");

	let stdout_span = info_span!(parent: None, "cosmic-settings-daemon");
	let stderr_span = stdout_span.clone();
	let (settings_exit_tx, settings_exit_rx) = oneshot::channel();
	let settings_exit_tx = Arc::new(std::sync::Mutex::new(Some(settings_exit_tx)));
	let settings_daemon = process_manager
		.start(
			Process::new()
				.with_executable("cosmic-settings-daemon")
				.with_env(env_vars.iter().cloned())
				.with_on_stdout(move |_, _, line| {
					let stdout_span = stdout_span.clone();
					async move {
						info!("{}", line);
					}
					.instrument(stdout_span)
				})
				.with_on_stderr(move |_, _, line| {
					let stderr_span = stderr_span.clone();
					async move {
						warn!("{}", line);
					}
					.instrument(stderr_span)
				})
				.with_on_exit(move |_, _, _, will_restart| {
					if !will_restart {
						if let Some(tx) = settings_exit_tx.lock().unwrap().take() {
							_ = tx.send(());
						}
					}
					async {}
				}),
		)
		.await
		.expect("failed to start settings daemon");

	// notifying the user service manager that we've reached the
	// graphical-session.target, which should only happen after:
	// - cosmic-comp is ready
	// - we've set any related variables
	// - cosmic-settings-daemon is ready
	dbg_log!("CS-DBG: settings-daemon started; about to start_systemd_target");
	let _ = tokio::time::timeout(std::time::Duration::from_secs(6), systemd::start_systemd_target())
		.await
		.map_err(|_| dbg_log!("CS-DBG: start_systemd_target TIMED OUT"));
	dbg_log!("CS-DBG: start_systemd_target done");
	// Always stop the target when the process exits or panics.
	scopeguard::defer! {
		systemd::stop_systemd_target();
	}

	// start a11y if configured
	tokio::spawn(a11y::start_a11y(env_vars.clone(), process_manager.clone()));

	let (panel_notifications_fd, daemon_notifications_fd) =
		notifications::create_socket().expect("Failed to create notification socket");

	let mut daemon_env_vars = env_vars.clone();
	daemon_env_vars.push((
		DAEMON_NOTIFICATIONS_FD.to_string(),
		daemon_notifications_fd.as_raw_fd().to_string(),
	));
	let mut panel_env_vars = env_vars.clone();
	panel_env_vars.push((
		PANEL_NOTIFICATIONS_FD.to_string(),
		panel_notifications_fd.as_raw_fd().to_string(),
	));

	let panel_key = Arc::new(Mutex::new(None));
	let notif_key = Arc::new(Mutex::new(None));

	// SESSION_PANEL env var allows overriding the panel binary (defaults to cosmic-panel)
	let panel_name: String =
		std::env::var("SESSION_PANEL").unwrap_or_else(|_| "cosmic-panel".to_string());
	let panel_name_static: &'static str = Box::leak(panel_name.clone().into_boxed_str());

	// SESSION_NOTIFICATIONS env var allows overriding the notifications binary
	let notifications_name: String = std::env::var("SESSION_NOTIFICATIONS").expect(
		"SESSION_NOTIFICATIONS env var must be set to the notifications daemon executable name",
	);
	let notifications_name_static: &'static str =
		Box::leak(notifications_name.clone().into_boxed_str());

	let notifications_span = info_span!(parent: None, "notifications", name = %notifications_name);
	let panel_span =
		tracing::span!(parent: None, tracing::Level::INFO, "panel", name = %panel_name);

	let mut guard = notif_key.lock().await;
	*guard = Some(
		process_manager
			.start(notifications_process(
				notifications_span.clone(),
				notifications_name_static,
				notif_key.clone(),
				daemon_env_vars.clone(),
				daemon_notifications_fd,
				panel_span.clone(),
				panel_name_static,
				panel_key.clone(),
				panel_env_vars.clone(),
			))
			.await
			.expect("failed to start notifications daemon"),
	);
	drop(guard);

	let mut guard = panel_key.lock().await;
	*guard = Some(
		process_manager
			.start(notifications_process(
				panel_span,
				panel_name_static,
				panel_key.clone(),
				panel_env_vars,
				panel_notifications_fd,
				notifications_span,
				notifications_name_static,
				notif_key,
				daemon_env_vars,
			))
			.await
			.expect("failed to start panel"),
	);
	drop(guard);

	dbg_log!("CS-DBG: reached component-launch section (cosmic-app-library)");
	let span = info_span!(parent: None, "cosmic-app-library");
	start_component("cosmic-app-library", span, &process_manager, &env_vars).await;

	// SESSION_COMPONENTS env var allows specifying which components to start (comma-separated)
	// Defaults to: cosmic-launcher,cosmic-workspaces,cosmic-osd,cosmic-bg,cosmic-greeter,cosmic-idle
	let default_components =
		"cosmic-launcher,cosmic-workspaces,cosmic-osd,cosmic-bg,cosmic-greeter,cosmic-idle";
	let components =
		std::env::var("SESSION_COMPONENTS").unwrap_or_else(|_| default_components.to_string());

	for component in components
		.split(',')
		.map(str::trim)
		.filter(|s| !s.is_empty())
	{
		let component_owned = component.to_string();
		dbg_log!("CS-DBG: start_component {}", component_owned);
		let span = tracing::span!(parent: None, tracing::Level::INFO, "component", name = %component_owned);
		start_component(component_owned.clone(), span, &process_manager, &env_vars).await;
		dbg_log!("CS-DBG: start_component {} returned", component_owned);
	}

	#[cfg(feature = "autostart")]
	if !*is_systemd_used() {
		info!("looking for autostart folders");
		let mut directories_to_scan = Vec::new();

		// we start by taking user specific directories, so that we can deduplicate and
		// ensure user overrides are respected

		// user specific directories
		if let Some(user_config_dir) = dirs::config_dir() {
			directories_to_scan.push(user_config_dir.join(AUTOSTART_DIR));
		}

		// system-wide directories
		if let Some(xdg_config_dirs) = env::var_os("XDG_CONFIG_DIRS") {
			let xdg_config_dirs = xdg_config_dirs
				.into_string()
				.expect("Invalid XDG_CONFIG_DIRS");
			let dir_list = xdg_config_dirs.split(":");

			for dir in dir_list {
				directories_to_scan.push(PathBuf::from(dir).join(AUTOSTART_DIR));
			}
		} else {
			directories_to_scan.push(PathBuf::from("/etc/xdg/").join(AUTOSTART_DIR));
		}

		info!("found autostart folders: {:?}", directories_to_scan);

		let mut dedupe = HashSet::new();

		let iter = freedesktop_desktop_entry::Iter::new(directories_to_scan.into_iter());
		let autostart_env = env_vars.clone();
		for entry in iter.entries::<&str>(None) {
			// we've already tried to execute this!
			if dedupe.contains(&entry.appid) {
				continue;
			}

			// skip if we have an OnlyShowIn entry that doesn't include COSMIC
			if let Some(only_show_in) = entry.only_show_in() {
				if !only_show_in.contains(&ENVIRONMENT_NAME) {
					continue;
				}
			}

			// ... OR we have a NotShowIn entry that includes COSMIC
			if let Some(not_show_in) = entry.not_show_in() {
				if not_show_in.contains(&ENVIRONMENT_NAME) {
					continue;
				}
			}

			info!(
				"trying to start appid {} ({})",
				entry.appid,
				entry.path.display()
			);

			if let Some(exec_raw) = entry.exec() {
				let mut exec_words = exec_raw.split(" ");

				if let Some(program_name) = exec_words.next() {
					// filter out any placeholder args, since we might not be able to deal with them
					let filtered_args = exec_words.filter(|s| !s.starts_with("%")).collect_vec();

					// escape them
					let escaped_args = shell_words::split(&*filtered_args.join(" "));
					if let Ok(args) = escaped_args {
						info!("trying to start {} {}", program_name, args.join(" "));

						let mut command = Command::new(program_name);
						command.args(args);

						// add relevant envs
						for (k, v) in &autostart_env {
							command.env(k, v);
						}

						// detach stdin/out/err (should we?)
						let child = command
							.stdin(Stdio::null())
							.stdout(Stdio::null())
							.stderr(Stdio::null())
							.spawn();

						if let Ok(child) = child {
							info!(
								"successfully started program {} {}",
								entry.appid,
								child.id()
							);
							dedupe.insert(entry.appid);
						} else {
							info!("could not start program {}", entry.appid);
						}
					} else {
						let why = escaped_args.unwrap_err();
						error!(?why, "could not parse arguments");
					}
				}
			}
		}
		info!("started {} programs", dedupe.len());
	}

	let mut signals = Signals::new(vec![libc::SIGTERM, libc::SIGINT]).unwrap();
	let mut status = Status::Exited;
	let session_dbus_rx_next = session_rx.recv();
	tokio::select! {
		res = session_dbus_rx_next => {
			match res {
				Some(service::SessionRequest::Exit) => {
					info!("EXITING: session exited by request");
				}
				Some(service::SessionRequest::Restart) => {
					info!("RESTARTING: session restarted by request");
					status = Status::Restarted;
				}
				None => {
					warn!("exit channel dropped session");
				}
			}
		},
		signal = signals.next() => match signal {
			Some(libc::SIGTERM | libc::SIGINT) => {
				info!("EXITING: received request to terminate");
			}
			Some(signal) => unreachable!("EXITING: received unhandled signal {}", signal),
			None => {},
		}
	}

	if let Some(compositor_handle) = compositor_handle {
		compositor_handle.abort();
	}
	token.cancel();

	let _ = process_manager.set_max_restarts(0).await;
	let components_stopped_at = std::time::Instant::now();
	process_manager.stop();
	if let Err(err) = process_manager.stop_process(settings_daemon).await {
		tracing::error!(?err, "Failed to gracefully stop settings daemon.");
	} else {
		match tokio::time::timeout(Duration::from_secs(1), settings_exit_rx).await {
			Ok(Ok(_)) => {}
			_ => {
				tracing::error!("Settings daemon process did not respond to the request to stop.");
			}
		};
	};

	const TEARDOWN_GRACE: std::time::Duration = std::time::Duration::from_millis(350);
	let elapsed = components_stopped_at.elapsed();
	if elapsed < TEARDOWN_GRACE {
		tokio::time::sleep(TEARDOWN_GRACE - elapsed).await;
	}
	Ok(status)
}

async fn start_component(
	cmd: impl Into<Cow<'static, str>>,
	span: tracing::Span,
	process_manager: &ProcessManager,
	env_vars: &[(String, String)],
) {
	let stdout_span = span.clone();
	let stderr_span = span.clone();
	let stderr_span_clone = stderr_span.clone();
	let cmd = cmd.into();
	let cmd_clone = cmd.clone();

	if let Err(err) = process_manager
		.start(
			Process::new()
				.with_executable(cmd.clone())
				.with_cancel_timeout(Duration::from_millis(250))
				.with_env(env_vars.iter().cloned())
				.with_on_stdout(move |_, _, line| {
					let stdout_span = stdout_span.clone();
					async move {
						info!("{}", line);
					}
					.instrument(stdout_span)
				})
				.with_on_stderr(move |_, _, line| {
					let stderr_span = stderr_span.clone();
					async move {
						warn!("{}", line);
					}
					.instrument(stderr_span)
				})
				.with_on_start(move |pman, pkey, _will_restart| async move {
					#[cfg(feature = "systemd")]
					if *is_systemd_used() {
						if let Ok((innr_cmd, Some(pid))) = pman.get_exe_and_pid(pkey).await {
							if let Err(err) = spawn_scope(innr_cmd.clone(), vec![pid]).await {
								warn!(
									"Failed to spawn scope for {}. Creating transient unit failed \
									 with {}",
									innr_cmd, err
								);
							};
						}
					}
				})
				.with_on_exit(move |mut _pman, _key, err_code, _will_restart| {
					if let Some(err) = err_code {
						error!("{cmd_clone} exited with error {}", err.to_string());
					}
					async {}
				}),
		)
		.await
	{
		let _enter = stderr_span_clone.enter();
		error!("failed to start {}: {}", cmd, err);
	}
}
