Name:           cosmic-session
Epoch:          1
Version:        %{getenv:COSMIC_SESSION_VERSION}
Release:        1%{?dist}
Summary:        COSMIC Session Manager (Playtron fork)

License:        GPL-3.0-only
URL:            https://github.com/pop-os/cosmic-session

# No BuildRequires - binary is pre-built

Requires:       systemd
Requires:       dbus

# Session components (from upstream cosmic-session dependencies)
# Pin to 1.0.x series (allow patch updates, not minor/major)
# Forked packages have Epoch:1 so must include epoch in constraint
Requires:       (cosmic-comp >= 1:1.0.0 with cosmic-comp < 1:1.1.0)
Requires:       (cosmic-greeter >= 1:1.0.0 with cosmic-greeter < 1:1.1.0)
Requires:       (cosmic-app-library >= 1.0.0 with cosmic-app-library < 1.1.0)
Requires:       (cosmic-applets >= 1.0.0 with cosmic-applets < 1.1.0)
Requires:       (cosmic-bg >= 1.0.0 with cosmic-bg < 1.1.0)
Requires:       (cosmic-icon-theme >= 1.0.0 with cosmic-icon-theme < 1.1.0)
Requires:       (cosmic-idle >= 1.0.0 with cosmic-idle < 1.1.0)
Requires:       (cosmic-launcher >= 1.0.0 with cosmic-launcher < 1.1.0)
Requires:       (cosmic-notifications >= 1:1.0.0 with cosmic-notifications < 1:1.1.0)
Requires:       (cosmic-osd >= 1.0.0 with cosmic-osd < 1.1.0)
Requires:       (cosmic-panel >= 1:1.0.0 with cosmic-panel < 1:1.1.0)
Requires:       (cosmic-randr >= 1.0.0 with cosmic-randr < 1.1.0)
Requires:       (cosmic-screenshot >= 1.0.0 with cosmic-screenshot < 1.1.0)
Requires:       (cosmic-settings >= 1:1.0.0 with cosmic-settings < 1:1.1.0)
Requires:       (cosmic-settings-daemon >= 1.0.0 with cosmic-settings-daemon < 1.1.0)
# Requires:       cosmic-workspaces  # Disabled - workspace switcher UI not needed
Requires:       xdg-desktop-portal-cosmic
Requires:       xorg-x11-server-Xwayland
Requires:       google-noto-sans-mono-fonts
Requires:       open-sans-fonts

# Override the upstream cosmic-session
Provides:       cosmic-session = %{epoch}:%{version}-%{release}
Obsoletes:      cosmic-session < %{epoch}:%{version}

%description
Session manager for the COSMIC desktop environment.
Handles starting and managing COSMIC desktop components.

%prep
%build

%install
# COSMIC_SESSION_SOURCE is set by the build script
install -Dm0755 "%{getenv:COSMIC_SESSION_SOURCE}/target/release/cosmic-session" "%{buildroot}%{_bindir}/cosmic-session"
install -Dm0755 "%{getenv:COSMIC_SESSION_SOURCE}/data/start-cosmic" "%{buildroot}%{_bindir}/start-cosmic"
install -Dm0644 "%{getenv:COSMIC_SESSION_SOURCE}/data/cosmic-session.target" "%{buildroot}%{_userunitdir}/cosmic-session.target"
install -Dm0644 "%{getenv:COSMIC_SESSION_SOURCE}/data/cosmic.desktop" "%{buildroot}%{_datadir}/wayland-sessions/cosmic.desktop"
install -Dm0644 "%{getenv:COSMIC_SESSION_SOURCE}/data/cosmic-mimeapps.list" "%{buildroot}%{_datadir}/applications/cosmic-mimeapps.list"
install -Dm0644 "%{getenv:COSMIC_SESSION_SOURCE}/data/dconf/profile/cosmic" "%{buildroot}%{_datadir}/dconf/profile/cosmic"
install -Dm0644 "%{getenv:COSMIC_SESSION_SOURCE}/LICENSE.md" "%{buildroot}%{_datadir}/licenses/cosmic-session/LICENSE.md"

%files
%license %{_datadir}/licenses/cosmic-session/LICENSE.md
%{_bindir}/cosmic-session
%{_bindir}/start-cosmic
%{_userunitdir}/cosmic-session.target
%{_datadir}/wayland-sessions/cosmic.desktop
%{_datadir}/applications/cosmic-mimeapps.list
%{_datadir}/dconf/profile/cosmic

%changelog
* Thu Jan 09 2026 Playtron <dev@playtron.one> - 1.0.0-1
- Initial RPM package
