namespace ExerciseTimer {
    [GtkTemplate(ui = "/xyz/safeworlds/hiit/ui/training_list_row.ui")]
    public class TrainingListRow : Gtk.ListBoxRow {
        public TrainingSetup Setup { get; set; }

        public signal void setup_edited();

        [GtkCallback]
        private void on_edit_clicked() {
            var dialog = new TrainingEditor(Setup);
            dialog.Applied.connect((setup) => {
                Setup.Title = setup.Title;
                Setup.Sets = setup.Sets;
                Setup.PreparationSec = setup.PreparationSec;
                Setup.ExerciseSec = setup.ExerciseSec;
                Setup.RestSec = setup.RestSec;
                setup_edited();
            });
            dialog.present(this);
        }

        [GtkCallback]
        private void on_delete_clicked() {
            Setup.deleted(Setup);
        }
    }
}