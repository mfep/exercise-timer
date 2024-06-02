project(
  'hiit',
  'rust',
  version: '1.3.0',
  meson_version: '>= 0.59',
  # license: 'MIT',
)

i18n = import('i18n')
gnome = import('gnome')

base_id = 'xyz.safeworlds.hiit'
homepage = 'https://github.com/mfep/exercise-timer'
issue_tracker = homepage + '/issues'
translate_page = 'https://hosted.weblate.org/projects/exercise-timer/exercise-timer/'
copyright = '© 2023 Exercise Timer developers'
developers = ['Lőrinc Serfőző https://the.safeworlds.xyz']
designers = ['Lőrinc Serfőző https://the.safeworlds.xyz']

dependency('glib-2.0', version: '>= 2.66')
dependency('gio-2.0', version: '>= 2.66')
dependency('gtk4', version: '>= 4.0.0')

glib_compile_schemas = find_program('glib-compile-schemas', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)
appstream_util = find_program('appstream-util', required: false)
cargo = find_program('cargo', required: true)

version = meson.project_version()

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')

datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'
podir = meson.project_source_root() / 'po'
gettext_package = meson.project_name()

if get_option('profile') == 'development'
  profile = 'Devel'
  vcs_tag = run_command('git', 'rev-parse', '--short', 'HEAD', check: false).stdout().strip()
  if vcs_tag == ''
    version_suffix = '-devel'
  else
    version_suffix = '-@0@'.format(vcs_tag)
  endif
  application_id = '@0@.@1@'.format(base_id, profile)
else
  profile = ''
  version_suffix = ''
  application_id = base_id
endif

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + version,
  meson.project_source_root()
)

if get_option('profile') == 'development'
  # Setup pre-commit hook for ensuring coding style is always consistent
  message('Setting up git pre-commit hook..')
  run_command('cp', '-f', 'hooks/pre-commit.hook', '.git/hooks/pre-commit', check: false)
endif

subdir('data')
subdir('po')
subdir('src')

gnome.post_install(
  gtk_update_icon_cache: true,
  glib_compile_schemas: true,
  update_desktop_database: true,
)

flatpak_cargo_generator = find_program(meson.project_source_root() / 'build-aux/flatpak-cargo-generator.py')

custom_target(
  'cargo-update-sources',
  build_by_default: false,
  output: 'cargo-sources.json',
  input: meson.project_source_root() / 'Cargo.lock',
  command: [
    flatpak_cargo_generator,
    '@INPUT@',
    '-o', '@SOURCE_ROOT@/cargo-sources.json'
  ]
)
