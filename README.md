<div align="center">

# Exercise Timer

Exercise timer is a simple utility to conduct interval training. It is built for the GNOME desktop using [libadwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1.4/index.html) and [Relm4](https://relm4.org/).

![A screenshot of Exercise Timer's exercise list UI](./data/screenshots/dark_exercise_list.png) ![A screenshot of Exercise Timer's timer UI](./data/screenshots/dark_timer.png)

</div>

## 🏋️ Features 
- 💾 Save and recall presets containing the number of sets and the duration of the exercise and rest periods. 
- ⏲️ Set the duration of the preparation in the settings.
- 🔊 A beeping sound is played at- and prior to each transition. 
- 🗣️ The volume of the sound can be adjusted.
- ☯️ Light and dark mode follows the system's setting.

## Installing

The recommended way of installing Exercise Timer is via Flathub.

<a href="https://flathub.org/apps/details/xyz.safeworlds.hiit" target="_blank"><img alt="Download on Flathub" src="https://flathub.org/assets/badges/flathub-badge-en.png" title="Download on Flathub" width="240"></a>

## 🛠️ Building the Flatpak

1. If not present, install `flatpak-builder`. It is most probably available in the operating system's package repository. E.g. on Fedora:

```bash
$ sudo dnf install -y flatpak-builder
```

2. If not present, add Flathub as a flatpak remote.

```bash
$ flatpak remote-add --user --if-not-exists flathub-verified https://flathub.org/repo/flathub.flatpakrepo
```

3. If not present, install the GNOME 45 Flatpak runtime and SDK and the Freedesktop SDK Rust and LLVM extensions.

```bash
$ flatpak install --user org.gnome.{Sdk,Platform}//45 org.freedesktop.Sdk.Extension.{rust-stable,llvm16}//23.08
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

PRs and feedback in the form of issues is welcome. Please be considerate though, and try to provide complete reports and code.

## ✍️ License

This work is licensed under the GNU GPLv3. See [LICENSE](./LICENSE) for details.
