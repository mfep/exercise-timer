run:
    meson compile -C builddir && builddir/src/hiit

update-pos:
    meson compile -C builddir hiit-update-po

build-flatpak:
    flatpak-builder --install --user --force-clean ../exercise-timer-build build-aux/xyz.safeworlds.hiit.Devel.json
