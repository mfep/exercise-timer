run:
    meson compile -C builddir && builddir/src/hiit

update-pos:
    meson compile -C builddir hiit-update-po
