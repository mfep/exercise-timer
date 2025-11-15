namespace ExerciseTimer {
    [GtkTemplate(ui = "/xyz/safeworlds/hiit/ui/training_editor.ui")]
    public class TrainingEditor : Adw.Dialog {
        public signal void Applied(TrainingSetup setup);

        public TrainingEditor(TrainingSetup _setup, bool new_setup) {
            setup = new TrainingSetup(){
                Title = _setup.Title,
                Sets = _setup.Sets,
                ExerciseSec = _setup.ExerciseSec,
                RestSec = _setup.RestSec,
                PreparationSec = _setup.PreparationSec,
            };
            setup.bind_property("Title", name_row, "text", GLib.BindingFlags.BIDIRECTIONAL | GLib.BindingFlags.SYNC_CREATE, null, null);
            setup.bind_property("Sets", sets_adjustment, "value", GLib.BindingFlags.BIDIRECTIONAL | GLib.BindingFlags.SYNC_CREATE, null, null);
            setup.bind_property("ExerciseSec", exercise_adjustment, "value", GLib.BindingFlags.BIDIRECTIONAL | GLib.BindingFlags.SYNC_CREATE, null, null);
            setup.bind_property("RestSec", rest_adjustment, "value", GLib.BindingFlags.BIDIRECTIONAL | GLib.BindingFlags.SYNC_CREATE, null, null);
            setup.bind_property("PreparationSec", preparation_adjustment, "value", GLib.BindingFlags.BIDIRECTIONAL | GLib.BindingFlags.SYNC_CREATE, null, null);

            this.new_setup = new_setup;
            notify_property("DialogTitle");
            notify_property("DialogAcceptStr");
        }

        public string DialogTitle {
            get {
                if (new_setup) {
                    // Translators: The editor window's title when creating a new training
                    return _("New Training");
                } else {
                    // Translators: The editor window's title when modifying a training
                    return _("Edit Training");
                }
            }
        }

        public string DialogAcceptStr {
            get {
                if (new_setup) {
                    // Translators: Button to close the editor window and create a new training
                    return _("Create");
                } else {
                    // Translators: Button to close the editor window and update an existing training
                    return _("Update");
                }
            }
        }

        [GtkCallback]
        private void on_apply_clicked() {
            Applied(setup);
            this.close();
        }

        [GtkCallback]
        private void on_cancel_clicked() {
            this.close();
        }

        private TrainingSetup setup;
        private bool new_setup;
        [GtkChild]
        private unowned Adw.EntryRow name_row;
        [GtkChild]
        private unowned Gtk.Adjustment sets_adjustment;
        [GtkChild]
        private unowned Gtk.Adjustment exercise_adjustment;
        [GtkChild]
        private unowned Gtk.Adjustment rest_adjustment;
        [GtkChild]
        private unowned Gtk.Adjustment preparation_adjustment;
    }
}