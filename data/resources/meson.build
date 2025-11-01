
blueprints = custom_target ('blueprints',
  input: files (
    'ui/shortcuts_dialog.blp',
    'ui/window.blp',
    # 'ui/training_setup_row.blp',
  ),
  output: '.',
  command: [ find_program ('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@' ],
)

resources = gnome.compile_resources ('hiit-resources',
  'resources.gresource.xml',
  c_name: 'hiit',
  dependencies: blueprints,
)
