
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

resources = gnome.compile_resources ('hiit-resources',
  'resources.gresource.xml',
  c_name: 'hiit',
  dependencies: blueprints,
)

subdir('icons')
