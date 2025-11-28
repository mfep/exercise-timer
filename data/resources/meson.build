
blueprints = custom_target ('blueprints',
  input: files (
    'ui/shortcuts_dialog.blp',
    'ui/timer_page.blp',
    'ui/training_editor.blp',
    'ui/training_list_row.blp',
    'ui/window.blp',
  ),
  output: '.',
  command: [ find_program ('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@' ],
)

resource_conf = configuration_data()
resource_conf.set('app-id', application_id)
resource_file = configure_file(
    input: 'resources.gresource.xml.in',
    output: '@BASENAME@',
    configuration: resource_conf
  )

resources = gnome.compile_resources ('hiit-resources',
  resource_file,
  c_name: 'hiit',
  dependencies: blueprints,
  source_dir: meson.build_root() / 'data'
)

subdir('icons')
