project(
  'hiit',
  ['c', 'vala'],
  version: '1.9.0',
  meson_version: '>= 0.59',
)

i18n = import('i18n')
gnome = import('gnome')

base_id = 'xyz.safeworlds.hiit'
homepage = 'https://apps.gnome.org/Hiit/'
issue_tracker = 'https://github.com/mfep/exercise-timer/issues'
translate_page = 'https://hosted.weblate.org/projects/exercise-timer/exercise-timer/'
donation_page = 'https://ko-fi.com/safeworlds'
copyright = '© 2023-2025 Exercise Timer developers'
developers = ['Lőrinc Serfőző https://the.safeworlds.xyz']
designers = ['Lőrinc Serfőző https://the.safeworlds.xyz']

dependencies = [
  dependency('gtk4', version: '>= 4.0.0'),
  dependency('libadwaita-1', version: '>= 1.0.0')
]

glib_compile_schemas = find_program('glib-compile-schemas', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)
appstream_util = find_program('appstream-util', required: false)

version = meson.project_version()

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')

datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'
podir = meson.project_source_root() / 'po'
gettext_package = meson.project_name()
add_project_arguments('-DGETTEXT_PACKAGE="' + gettext_package + '"', language: 'c')

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
