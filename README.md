<div align="center">

# Exercise Timer

[![CI badge](https://img.shields.io/github/actions/workflow/status/mfep/exercise-timer/ci.yml?branch=main)](https://github.com/mfep/midiconn/actions/workflows/ci.yml)
[![License badge](https://img.shields.io/github/license/mfep/exercise-timer)](./LICENSE.txt)
[![Flathub badge](https://img.shields.io/flathub/downloads/xyz.safeworlds.hiit?logo=flathub&logoColor=white)](https://flathub.org/apps/details/xyz.safeworlds.hiit)
![Platforms badge](https://img.shields.io/badge/platform-linux-informational)
[![Translations badge](https://hosted.weblate.org/widget/exercise-timer/exercise-timer/svg-badge.svg?native=1)](https://hosted.weblate.org/projects/exercise-timer/exercise-timer/)
[![dependency status](https://deps.rs/repo/github/mfep/exercise-timer/status.svg)](https://deps.rs/repo/github/mfep/exercise-timer)

Exercise Timer is a simple utility to conduct interval training. It is built for the GNOME desktop using [libadwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1.4/index.html) and [Relm4](https://relm4.org/).

![A screenshot of Exercise Timer's exercise list UI](./data/screenshots/dark_exercise_list.png) ![A screenshot of Exercise Timer's timer UI](./data/screenshots/dark_timer.png)

</div>

## 🏋️ Features 
- 💾 Save and recall presets containing the number of sets and the duration of the exercise, rest and preparation periods. 
- 🔊 A beeping sound is played at- and prior to each transition. 
- 🗣️ The volume of the sound can be adjusted.
- ☯️ Light and dark mode follows the system's setting.

## Installing

The recommended way of installing Exercise Timer is via Flathub.

<a href="https://flathub.org/apps/details/xyz.safeworlds.hiit" target="_blank"><img width='240' alt='Get it on Flathub' src='https://flathub.org/api/badge?locale=en'/></a>

## 🛠️ Building the Flatpak

1. If not present, install `flatpak-builder`. It is most probably available in the operating system's package repository. E.g. on Fedora:

```bash
$ sudo dnf install -y flatpak-builder
```

2. If not present, add Flathub as a flatpak remote.

```bash
$ flatpak remote-add --user --if-not-exists flathub-verified https://flathub.org/repo/flathub.flatpakrepo
```

3. If not present, install the GNOME 47 Flatpak runtime and SDK and the Freedesktop SDK Rust and LLVM extensions.

```bash
$ flatpak install --user org.gnome.{Sdk,Platform}//47 org.freedesktop.Sdk.Extension.{rust-stable,llvm18}//24.08
```

4. Clone the current repository.

```bash
$ git clone https://github.com/mfep/exercise-timer.git
```

5. Build and install Exercise Timer with `flatpak-builder`!

```bash
$ cd exercise-timer
$ flatpak-builder --user --install --force-clean build ./build-aux/xyz.safeworlds.hiit.Devel.yml
```

## 🧑‍🤝‍🧑 Contributing

### Translations

Exercise Timer is translated via [Weblate](https://hosted.weblate.org/projects/exercise-timer/exercise-timer/). Fixes to existing translations as well as translating to new languages are welcome!

<div align="center">
  <a href="https://hosted.weblate.org/engage/exercise-timer/">
    <img src="https://hosted.weblate.org/widget/exercise-timer/exercise-timer/multi-auto.svg" alt="Translation status" />
  </a>
</div>

### Development

PRs and feedback in the form of issues are most welcome.

### Code of Conduct

This project follows the [GNOME Code of Conduct](https://conduct.gnome.org/).

## ✍️ License

This work is licensed under the GNU GPLv3. See [LICENSE](./LICENSE) for details.
