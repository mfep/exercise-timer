global_conf = configuration_data()
global_conf.set_quoted('APP_ID', application_id)
global_conf.set_quoted('VERSION', version + version_suffix)
global_conf.set_quoted('HOMEPAGE', homepage)
global_conf.set_quoted('ISSUE_TRACKER', issue_tracker)
global_conf.set_quoted('COPYRIGHT', copyright)
global_conf.set_quoted('GETTEXT_PACKAGE', gettext_package)
global_conf.set_quoted('LOCALEDIR', localedir)

developers_str = ''
foreach dev : developers
  developers_str += '"' + dev + '", '
endforeach
designers_str = ''
foreach des : designers
  designers_str += '"' + des + '", '
endforeach
global_conf.set('DEVELOPERS', developers_str)
global_conf.set('DESIGNERS', designers_str)

config_source = configure_file(
  input: 'Config.vala.in',
  output: 'Config.vala',
  configuration: global_conf
)

sources = [
  'Application.vala',
  'ITimerNotifier.vala',
  'ITimerPageActionNotifier.vala',
  'TimerPage.vala',
  'TrainingEditor.vala',
  'TrainingListModel.vala',
  'TrainingListRow.vala',
  'TrainingSetup.vala',
  'Voices.vala',
  'Window.vala',
  'main.vala',
  config_source
]

sources += resources

executable ('hiit', sources,
  dependencies: dependencies,
  vala_args: [ '--gresourcesdir=' + meson.project_source_root() + '/data/resources', '--pkg', 'json-glib-1.0' ],
  install: true,
)
