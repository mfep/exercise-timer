namespace ExerciseTimer {

    [GtkTemplate(ui = "/xyz/safeworlds/hiit/ui/window.ui")]
    public class Window : Adw.ApplicationWindow {
        public Window(Gtk.Application app) {
            Object(application: app);

            var settings = new GLib.Settings(Config.AppId);
            settings.bind("window-width", this, "default_width", GLib.SettingsBindFlags.DEFAULT);
            settings.bind("window-height", this, "default_height", GLib.SettingsBindFlags.DEFAULT);
            settings.bind("window-is-maximized", this, "maximized", GLib.SettingsBindFlags.DEFAULT);

            training_list_stack.set_visible_child(training_list_status);
        }

        [GtkCallback]
        private void on_add_training() {
            training_list_stack.set_visible_child(training_list_scrolled);
            var row = new TrainingListRow(){
                Setup = new TrainingSetup(){ Title = "Sample Exercise %d".printf(i++), WarmupSec = 5, ExerciseSec = 30, RestSec = 10, Sets = 4 }
            };
            row.Deleted.connect(on_training_deleted);
            training_listbox.append(row);
        }

        private void on_training_deleted(Gtk.Widget row) {
            training_listbox.remove(row);
            if (training_listbox.get_first_child() == null) {
                training_list_stack.set_visible_child(training_list_status);
            }
        }

        [GtkChild]
        private unowned Gtk.Stack training_list_stack;
        [GtkChild]
        private unowned Gtk.Widget training_list_status;
        [GtkChild]
        private unowned Gtk.Widget training_list_scrolled;
        [GtkChild]
        private unowned Gtk.ListBox training_listbox;

        private int i = 0;
    }
}