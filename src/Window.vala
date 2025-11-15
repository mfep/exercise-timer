using Json;

namespace ExerciseTimer {

    [GtkTemplate(ui = "/xyz/safeworlds/hiit/ui/window.ui")]
    public class Window : Adw.ApplicationWindow, ITimerPageActionNotifier {
        public Window(Gtk.Application app) {
            GLib.Object(application: app);

            settings.bind("window-width", this, "default_width", GLib.SettingsBindFlags.DEFAULT);
            settings.bind("window-height", this, "default_height", GLib.SettingsBindFlags.DEFAULT);
            settings.bind("window-is-maximized", this, "maximized", GLib.SettingsBindFlags.DEFAULT);

            training_listbox.bind_model(training_list_model, training_list_create_widget);
            training_list_model.item_deleted.connect((remaining_items) => {
                save_training_list();
                if (remaining_items == 0) {
                    training_list_stack.set_visible_child(training_list_status);
                }
            });
            training_list_stack.set_visible_child(training_list_status);
            load_trainings_from_json();
        }

        construct {
            settings = new GLib.Settings(Config.AppId);
            load_default_setup();
        }

        public void on_restart_action() {
            restart_action_called();
        }

        private void load_trainings_from_json() {
            var exercise_list_str = settings.get_string("exercise-json-list");
            Json.Node root;
            try {
                root = Json.from_string(exercise_list_str);
            } catch (GLib.Error err) {
                message("Could not load training list from JSON: %s", err.message);
                return;
            }
            var root_array = root.get_array();
            if (root_array == null) {
                message("Could not load training list from JSON: root is not array");
                return;
            }
            root_array.foreach_element((_, _1, node) => {
                var setup = Json.gobject_deserialize(typeof (TrainingSetup), node) as TrainingSetup;
                if (setup != null) {
                    add_training_to_list(setup);
                }
            });
        }

        private void save_training_list() {
            var array = new Json.Array();
            for (uint i = 0; i < training_list_model.get_n_items(); ++i) {
                array.add_element(Json.gobject_serialize(training_list_model.get_item(i)));
            }
            var node = new Json.Node(Json.NodeType.ARRAY);
            node.take_array(array);
            var exercise_list_str = Json.to_string(node, false);
            settings.set_string("exercise-json-list", exercise_list_str);
        }

        [GtkCallback]
        private void on_add_training() {
            var editor_dialog = new TrainingEditor(default_setup);
            editor_dialog.Applied.connect((new_setup) => {
                add_training_to_list(new_setup);
                save_training_list();
            });
            editor_dialog.present(this);
        }

        private void add_training_to_list(TrainingSetup setup) {
            training_list_stack.set_visible_child(training_list_scrolled);
            training_list_model.append(setup);
        }

        [GtkCallback]
        private void on_training_activated(Gtk.ListBoxRow row) {
            var training_list_row = row as TrainingListRow;
            var setup = training_list_row.Setup;
            timer_page = new TimerPage(setup, this);
            navigation_view.push(timer_page);
            voices = new Voices(timer_page);
        }

        private Gtk.Widget training_list_create_widget(GLib.Object obj) {
            var widget = new TrainingListRow(){
                Setup = obj as TrainingSetup
            };
            widget.setup_edited.connect(() => {
                save_training_list();
            });
            return widget;
        }

        private static void load_default_setup() {
            var default_setup_str = settings.get_string("default-exercise-json");
            Json.Node node;
            try {
                node = Json.from_string(default_setup_str);
                default_setup = Json.gobject_deserialize(typeof (TrainingSetup), node) as TrainingSetup;
            } catch (GLib.Error err) {
                message("Could not load default training setup: %s", err.message);
            }

            if (default_setup == null) {
                default_setup = new TrainingSetup() {
                    Title = "Exercise",
                    PreparationSec = 5,
                    ExerciseSec = 30,
                    RestSec = 10,
                    Sets = 16,
                };
            }
        }

        [GtkChild]
        private unowned Adw.NavigationView navigation_view;
        [GtkChild]
        private unowned Gtk.Stack training_list_stack;
        [GtkChild]
        private unowned Gtk.Widget training_list_status;
        [GtkChild]
        private unowned Gtk.Widget training_list_scrolled;
        [GtkChild]
        private unowned Gtk.ListBox training_listbox;

        private Voices voices;
        private TimerPage timer_page;
        private TrainingListModel training_list_model = new TrainingListModel();

        private static GLib.Settings settings;
        private static TrainingSetup default_setup;
    }
}