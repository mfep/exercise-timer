namespace ExerciseTimer {
    [GtkTemplate(ui = "/xyz/safeworlds/hiit/ui/training_editor.ui")]
    public class TrainingEditor : Adw.Dialog {
        public signal void Applied(TrainingSetup setup);

        public TrainingEditor(TrainingSetup _setup) {
            setup = new TrainingSetup(){
                Title = _setup.Title,
                Sets = _setup.Sets,
                ExerciseSec = _setup.ExerciseSec,
                RestSec = _setup.RestSec,
                WarmupSec = _setup.WarmupSec,
            };
            setup.bind_property("Title", name_row, "text", GLib.BindingFlags.BIDIRECTIONAL | GLib.BindingFlags.SYNC_CREATE, null, null);
            setup.bind_property("Sets", sets_adjustment, "value", GLib.BindingFlags.BIDIRECTIONAL | GLib.BindingFlags.SYNC_CREATE, null, null);
            setup.bind_property("ExerciseSec", exercise_adjustment, "value", GLib.BindingFlags.BIDIRECTIONAL | GLib.BindingFlags.SYNC_CREATE, null, null);
            setup.bind_property("RestSec", rest_adjustment, "value", GLib.BindingFlags.BIDIRECTIONAL | GLib.BindingFlags.SYNC_CREATE, null, null);
            setup.bind_property("WarmupSec", preparation_adjustment, "value", GLib.BindingFlags.BIDIRECTIONAL | GLib.BindingFlags.SYNC_CREATE, null, null);
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