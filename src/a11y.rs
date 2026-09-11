use futures_util::StreamExt;
use launch_pad::ProcessManager;
use tokio::sync::mpsc;
use tracing::Instrument;

const ORCA: Option<&'static str> = option_env!("ORCA");

/// Resolve the screen reader executable.
///
/// Precedence: the `ORCA` environment variable (runtime override), then the
/// `ORCA` value baked in at build time by the packager, then the bare name so
/// that `PATH` resolves it. The bare name is what keeps the same binary working
/// on distributions that do not have an `/usr/bin` at all; on an FHS system
/// `PATH` resolves it to `/usr/bin/orca`, exactly as before.
fn screen_reader_executable(runtime: Option<&str>, build_time: Option<&str>) -> String {
	runtime
		.filter(|value| !value.is_empty())
		.or(build_time.filter(|value| !value.is_empty()))
		.unwrap_or("orca")
		.to_string()
}

pub async fn start_a11y(
	env_vars: Vec<(String, String)>,
	pman: ProcessManager,
) -> color_eyre::Result<()> {
	let (tx, mut rx) = mpsc::unbounded_channel();
	let mut process_key = None;
	let conn = zbus::Connection::session().await?;
	let proxy = cosmic_dbus_a11y::StatusProxy::new(&conn).await?;

	tokio::spawn(async move {
		let mut watch_changes = proxy.receive_screen_reader_enabled_changed().await;
		let mut enabled = false;
		if let Ok(status) = proxy.screen_reader_enabled().await {
			_ = tx.send(status);

			enabled = status;
		}
		while let Some(change) = watch_changes.next().await {
			let Ok(new_enabled) = change.get().await else {
				tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
				continue;
			};
			if enabled != new_enabled {
				_ = tx.send(new_enabled);
				enabled = new_enabled;
			}
		}
	});

	let orca = screen_reader_executable(std::env::var("ORCA").ok().as_deref(), ORCA);

	while let Some(enabled) = rx.recv().await {
		let stdout_span = info_span!(parent: None, "screen-reader");
		let stderr_span = stdout_span.clone();
		if enabled && process_key.is_none() {
			// spawn orca
			match pman
				.start(
					launch_pad::process::Process::new()
						.with_executable(&orca)
						.with_env(env_vars.clone())
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
						}),
				)
				.await
			{
				Ok(key) => {
					process_key = Some(key);
				}
				Err(err) => {
					tracing::error!("Failed to start screen reader {err:?}");
				}
			}
		} else if !enabled && process_key.is_some() {
			// kill orca
			info!("Stopping screen reader");
			if let Err(err) = pman.stop_process(process_key.take().unwrap()).await {
				tracing::error!("Failed to stop screen reader. {err:?}")
			}
		}
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::screen_reader_executable;

	#[test]
	fn defaults_to_a_path_resolvable_name() {
		// No override anywhere: PATH decides, which resolves to /usr/bin/orca on
		// an FHS distribution and to the store path elsewhere.
		assert_eq!(screen_reader_executable(None, None), "orca");
	}

	#[test]
	fn build_time_value_is_used_when_set() {
		assert_eq!(
			screen_reader_executable(None, Some("/usr/bin/orca")),
			"/usr/bin/orca"
		);
	}

	#[test]
	fn runtime_override_wins_over_build_time_value() {
		assert_eq!(
			screen_reader_executable(
				Some("/nix/store/00000000000000000000000000000000-orca-47.0/bin/orca"),
				Some("/usr/bin/orca"),
			),
			"/nix/store/00000000000000000000000000000000-orca-47.0/bin/orca"
		);
	}

	#[test]
	fn empty_values_are_ignored() {
		assert_eq!(screen_reader_executable(Some(""), Some("")), "orca");
		assert_eq!(screen_reader_executable(Some(""), Some("orca")), "orca");
	}
}
