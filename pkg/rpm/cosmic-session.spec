Name:           cosmic-session
Epoch:          1
Version: 1.2.5
Release:        1%{?dist}
Summary:        COSMIC Session Manager (Playtron fork)

License:        GPL-3.0-only
URL:            https://github.com/pop-os/cosmic-session
Source0:        %{name}-%{_arch}.tar.gz

%global debug_package %{nil}

Requires:       systemd
Requires:       dbus

# Session components (from upstream cosmic-session dependencies).
# Bound to the COSMIC 1.x series (< 2.0.0), NOT a single minor: these are
# independent processes / noarch data (icons, wallpaper), not ABI deps — libcosmic
# is statically linked into each binary — so they track Fedora's COSMIC release
# train (now 1.1.x). Pinning them to < 1.1.0 made a routine Fedora lib bump
# unsatisfiable, which downgraded this fork back to the upstream package.
# Forked packages carry Epoch:1, so their constraints include the epoch.
Requires:       (cosmic-comp >= 1:1.0.0 with cosmic-comp < 1:2.0.0)
Requires:       (cosmic-bg >= 1.0.0 with cosmic-bg < 2.0.0)
Requires:       (cosmic-icon-theme >= 1.0.0 with cosmic-icon-theme < 2.0.0)
Requires:       (cosmic-idle >= 1.0.0 with cosmic-idle < 2.0.0)
Requires:       (cosmic-osd >= 1:1.0.0 with cosmic-osd < 1:2.0.0)
Requires:       (cosmic-randr >= 1.0.0 with cosmic-randr < 2.0.0)
Requires:       (cosmic-settings-daemon >= 1:1.0.0 with cosmic-settings-daemon < 1:2.0.0)
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
%autosetup -n %{name} -p1

%build

%install
install -Dm0755 "usr/bin/cosmic-session" "%{buildroot}%{_bindir}/cosmic-session"
install -Dm0755 "usr/bin/start-cosmic" "%{buildroot}%{_bindir}/start-cosmic"
install -Dm0644 "usr/lib/systemd/user/cosmic-session.target" "%{buildroot}%{_prefix}/lib/systemd/user/cosmic-session.target"
install -Dm0644 "usr/share/wayland-sessions/cosmic.desktop" "%{buildroot}%{_datadir}/wayland-sessions/cosmic.desktop"
install -Dm0644 "usr/share/applications/cosmic-mimeapps.list" "%{buildroot}%{_datadir}/applications/cosmic-mimeapps.list"
install -Dm0644 "usr/share/dconf/profile/cosmic" "%{buildroot}%{_datadir}/dconf/profile/cosmic"
install -Dm0644 "usr/share/licenses/cosmic-session/LICENSE.md" "%{buildroot}%{_datadir}/licenses/cosmic-session/LICENSE.md"

%files
%license %{_datadir}/licenses/cosmic-session/LICENSE.md
%{_bindir}/cosmic-session
%{_bindir}/start-cosmic
%{_prefix}/lib/systemd/user/cosmic-session.target
%{_datadir}/wayland-sessions/cosmic.desktop
%{_datadir}/applications/cosmic-mimeapps.list
%{_datadir}/dconf/profile/cosmic

%changelog
* Tue Feb 03 2026 Playtron <dev@playtron.one> - 1.0.9-1
- Initial RPM package
