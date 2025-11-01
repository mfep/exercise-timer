namespace ExerciseTimer {

    [GtkTemplate(ui = "/xyz/safeworlds/hiit/ui/window.ui")]
    public class Window : Adw.ApplicationWindow {
        public Window(Gtk.Application app) {
            Object(application: app);
        }

        [GtkCallback]
        private void on_add_training() {
            //  var row = new SetupRow(){ Setup = new TrainingSetup(){ Name = "Oreg Allat" } };
            //  training_listbox.append(row);
        }

        // [GtkChild]
        // private unowned Gtk.Stack training_list_stack;
        // [GtkChild]
        // private unowned Gtk.Widget training_list_status;
        //  [GtkChild]
        //  private unowned Gtk.ListBox training_listbox;
    }
}
