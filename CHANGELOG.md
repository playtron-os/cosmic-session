## [1.2.1](https://github.com/playtron-os/cosmic-session/compare/v1.2.0...v1.2.1) (2026-05-29)


### Bug Fixes

* remove cosmic-session from RPM spec ([2a723ac](https://github.com/playtron-os/cosmic-session/commit/2a723ac524ed5800de325f9e9751db63f635d053))

# [1.2.0](https://github.com/playtron-os/cosmic-session/compare/v1.1.2...v1.2.0) (2026-04-07)


### Features

* add SESSION_NOTIFICATIONS env var to specify which notifications app to use ([3d9073e](https://github.com/playtron-os/cosmic-session/commit/3d9073efcd193c62da419cdb7544c48756fd7159))

## [1.1.2](https://github.com/playtron-os/cosmic-session/compare/v1.1.1...v1.1.2) (2026-04-01)


### Bug Fixes

* forward COSMIC_ACTIVATION_TRUSTED_APPS var ([148fc5b](https://github.com/playtron-os/cosmic-session/commit/148fc5b875598922c93f58415e0d2e088c992395))

## [1.1.1](https://github.com/playtron-os/cosmic-session/compare/v1.1.0...v1.1.1) (2026-03-27)


### Bug Fixes

* remove cosmic-screenshots from rpm spec ([f4c5350](https://github.com/playtron-os/cosmic-session/commit/f4c53501cebd641197fd5e01809068889d4e8c96))

# [1.1.0](https://github.com/playtron-os/cosmic-session/compare/v1.0.3...v1.1.0) (2026-02-04)


### Features

* forward cosmic-comp related vars ([7c5bdff](https://github.com/playtron-os/cosmic-session/commit/7c5bdff1d6da3f408f9f47aa16eab95bdfbf8145))

## [1.0.3](https://github.com/playtron-os/cosmic-session/compare/v1.0.2...v1.0.3) (2026-02-04)


### Bug Fixes

* add pr/issues permissions to release CI ([04fd475](https://github.com/playtron-os/cosmic-session/commit/04fd47503965887554dc8095bd6a2ef9d52ef569))
* update rpm spec package requires ([3467ef0](https://github.com/playtron-os/cosmic-session/commit/3467ef0620fe2580b97d8355d73c05498bf02b5f))

## [1.0.2](https://github.com/playtron-os/cosmic-session/compare/v1.0.1...v1.0.2) (2026-02-04)


### Bug Fixes

* in CI upload rpms and add concurrency config ([4e94557](https://github.com/playtron-os/cosmic-session/commit/4e94557d84765828bd93dd1054882d37ded9adfd))

## [1.0.1](https://github.com/playtron-os/cosmic-session/compare/v1.0.0...v1.0.1) (2026-02-04)


### Bug Fixes

* use sed in CI version replacement and exit early if no release generated ([89da3be](https://github.com/playtron-os/cosmic-session/commit/89da3be6718def9e7e23a4eec71eaf4e9769d976))

# 1.0.0 (2026-02-04)


### Bug Fixes

* add extra env vars to start-cosmic script ([0f6072c](https://github.com/playtron-os/cosmic-session/commit/0f6072c5fd9edec9d51f2748b2c66e8266664aff))
* add more dependencies ([84427af](https://github.com/playtron-os/cosmic-session/commit/84427afcdf5e5564d87adc2002c3fe4837c6a89e))
* add rpm packaging ([b065cba](https://github.com/playtron-os/cosmic-session/commit/b065cba49ebf2cd0cbf46daf282f9b49ed26fe05))
* add SSH_AUTH_SOCK to systemd import-environment ([ec64522](https://github.com/playtron-os/cosmic-session/commit/ec64522110482f44987028135361a88bfdbe5a61))
* add upstream dependencies to rpm spec ([33d8ec2](https://github.com/playtron-os/cosmic-session/commit/33d8ec24e750738645c5b5284364321b6c3716bc))
* allow calls for systemd target to fail ([0bedac0](https://github.com/playtron-os/cosmic-session/commit/0bedac052a2ce9141891f12f9c18517692391fd4))
* applets dep ([fab540d](https://github.com/playtron-os/cosmic-session/commit/fab540d54eb2dde72983060d669f9db8a627d19a))
* avoid hardcoding keyring daemon path for distros like NixOS ([2441be2](https://github.com/playtron-os/cosmic-session/commit/2441be2ad02fc28b397f67b704f2b41de7a96ed6))
* check before using `systemctl` ([bc87ac9](https://github.com/playtron-os/cosmic-session/commit/bc87ac9918fd66654a0953094fd33123a8fe7954))
* **debian:** dconf profile now installed by justfile ([37c95bc](https://github.com/playtron-os/cosmic-session/commit/37c95bc3aa38ab77736330e0ca33e4283f3886cb))
* **debian:** remove titlebar-font gsettings key ([957fd52](https://github.com/playtron-os/cosmic-session/commit/957fd526cb569e2247a73ee3315d2a3d1d8b0628))
* don't set XCURSOR_THEME in start-cosmic ([6650b91](https://github.com/playtron-os/cosmic-session/commit/6650b910b54f11004f274340ccc550305abbdb1e))
* enable auto start feature by default ([8594ba8](https://github.com/playtron-os/cosmic-session/commit/8594ba8501eee64f85cff0968e42c080b2be737f))
* ignore error when starting service ([ba561b4](https://github.com/playtron-os/cosmic-session/commit/ba561b4097abc387a7e9d3068985888468f24ce9))
* include correct cosmic-comp dependency in rpm spec ([f4553e5](https://github.com/playtron-os/cosmic-session/commit/f4553e5a634552a11a2854e98e88d8db894fd784))
* **just:** wrong path for dconf profile in start-cosmic ([be18d37](https://github.com/playtron-os/cosmic-session/commit/be18d37bfab3eb85a9348a6426cc3151b85997ad))
* log cosmic-settings-daemon output ([c3a35f3](https://github.com/playtron-os/cosmic-session/commit/c3a35f323b88a760e17da0637a7df24e6bd949e0))
* make sure proper xdg vars are set during new session start ([e753ea9](https://github.com/playtron-os/cosmic-session/commit/e753ea99d75efc8a87643e001f47da999d71c5c7))
* mark comp sock as not cloexec ([c41c108](https://github.com/playtron-os/cosmic-session/commit/c41c108c4f4c8a08a0bb4cd0943e1d22434fe56a))
* prevent session from exiting ([e1a174f](https://github.com/playtron-os/cosmic-session/commit/e1a174f61dfb152d934961e795d5b8cfd47077e9))
* rely on channels for synchronisation/sequence instead of `sleep()` ([8e73c0f](https://github.com/playtron-os/cosmic-session/commit/8e73c0f6940288c4a24a102a7ba9f20eb6bd754f))
* remove comment about safety as it's not necessary anymore ([8590c67](https://github.com/playtron-os/cosmic-session/commit/8590c67be5edb123731929f016d543102b4cfb16))
* set cursor theme to COSMIC ([5613bc6](https://github.com/playtron-os/cosmic-session/commit/5613bc660649c65b4a4c3fb41605491b9765729a))
* set nonblocking to true for the client Fd ([3746d20](https://github.com/playtron-os/cosmic-session/commit/3746d20225d86c78872fa153b759089c3b02d3a8))
* set OZONE default to auto ([fae8ac6](https://github.com/playtron-os/cosmic-session/commit/fae8ac6a9f50e6ad3571bac75d2c6ec49f2bb111))
* start gnome-keyring before cosmic-session & export `SSH_AUTH_SOCK` ([4598b7c](https://github.com/playtron-os/cosmic-session/commit/4598b7cdf4a2d41737d68415966a475759d34d55))
* start gnome-keyring components if the daemon is active ([4c72d42](https://github.com/playtron-os/cosmic-session/commit/4c72d42731f96cf146c1ab664d0cc4f292e2527b))
* start systemd target _after_ cosmic-comp is ready ([be418fd](https://github.com/playtron-os/cosmic-session/commit/be418fde284e39ba518da4eebff70312c7c06443))
* start xdg-desktop-portal-cosmic ([3094e46](https://github.com/playtron-os/cosmic-session/commit/3094e4698efef6fa4ae3c52c8ac1a6de39f40687))
* steam fails when dconf profile lacks ending newline ([38e3686](https://github.com/playtron-os/cosmic-session/commit/38e368657916cf7e5db07259a6450167aba132d7))
* typos in error messages ([eabfa58](https://github.com/playtron-os/cosmic-session/commit/eabfa58bd52a599c6fe3a4b456aebba5e0d3ee4c))
* update asyinc-signals to fix loongarch build ([e9858d9](https://github.com/playtron-os/cosmic-session/commit/e9858d914cd0c07bacbadf7f5672cb9d0f53a72b))
* use preferred SHELL in login shell mode ([9f25600](https://github.com/playtron-os/cosmic-session/commit/9f256004e772593a7158427d6f623c74894f64dc))
* use tokio Mutex and clear other clippy warnings ([c016d25](https://github.com/playtron-os/cosmic-session/commit/c016d254d9964e340884f8ded1483b0114ab42a1))
* xdg autostart was not executing ([1ef6af1](https://github.com/playtron-os/cosmic-session/commit/1ef6af14bbaecccecf8337edcf75468779b75a4f))


### Features

* a11y ([2612282](https://github.com/playtron-os/cosmic-session/commit/2612282fcfcc2c12c8a7227631cb936d17edc494))
* additions for optional experimental X support via wayland-proxy-virtwl ([9a9861f](https://github.com/playtron-os/cosmic-session/commit/9a9861f8e3a0b2964f7064bb64258f41ac2d5b11))
* cosmic-bg & update deps ([c350099](https://github.com/playtron-os/cosmic-session/commit/c350099b764c279da1598f694c5bfdf3aeb13212))
* implement dynamic way to declare session components ([c54ce14](https://github.com/playtron-os/cosmic-session/commit/c54ce1481f8ad479177cc5ffa2963e4b96133fe5))
* Import environment variables from systemd user manager ([6e48e12](https://github.com/playtron-os/cosmic-session/commit/6e48e124434e394683b0048a9c0f2f5fca562189))
* inhibit poweroff button ([b2f4277](https://github.com/playtron-os/cosmic-session/commit/b2f42771222b1d0acd267355a83776abd465eff7))
* send notifications and panel Fds ([4c9557f](https://github.com/playtron-os/cosmic-session/commit/4c9557f42b528c2d86c20706e7951a5f10a676c0))
* set XDG_SESSION_TYPE=wayland just before graphical-session.target ([b724bdd](https://github.com/playtron-os/cosmic-session/commit/b724bddf494576bbecc51d0c98c2f2cfdab7558b))
* try to gracefully shut down settings daemon when exiting ([60ff224](https://github.com/playtron-os/cosmic-session/commit/60ff2241ed08ec4102f635e1bb75c70bbaba822c))
* update to version 1.0.8 ([112f3ec](https://github.com/playtron-os/cosmic-session/commit/112f3ec70f3dbcafd1cc0fb0d0e27a26e6dbbadc))
* use privileged sockets ([fd2dc23](https://github.com/playtron-os/cosmic-session/commit/fd2dc23fac183d98e33bd5f5be9bd499493d77f1))


### Reverts

* Revert "update deps" ([30ce09f](https://github.com/playtron-os/cosmic-session/commit/30ce09f54a928a36e43439baf3a21d0b944378b0))
