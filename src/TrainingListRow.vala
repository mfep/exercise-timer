namespace ExerciseTimer {
    [GtkTemplate(ui = "/xyz/safeworlds/hiit/ui/training_list_row.ui")]
    public class TrainingListRow : Gtk.ListBoxRow {
        public TrainingSetup Setup { get; set; }

        public signal void Deleted(Gtk.Widget sender);

        [GtkCallback]
        private void on_delete_clicked() {
            Deleted(this);
        }
    }
}