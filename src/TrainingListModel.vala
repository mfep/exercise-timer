namespace ExerciseTimer {
    public class TrainingListModel : GLib.Object, GLib.ListModel {

        public signal void item_deleted (uint remaining_items);

        public GLib.Object? get_item (uint position) {
            if (position >= setup_list.length ()) {
                return null;
            }
            return setup_list.nth_data (position);
        }

        public GLib.Type get_item_type () {
            return typeof (TrainingSetup);
        }

        public uint get_n_items () {
            return setup_list.length ();
        }

        public void append (TrainingSetup setup) {
            setup.deleted.connect (on_setup_deleted);

            setup_list.append (setup);
            items_changed (setup_list.length () - 1, 0, 1);
        }

        private void on_setup_deleted (TrainingSetup deleted_setup) {
            var idx = setup_list.index (deleted_setup);
            setup_list.remove (deleted_setup);
            items_changed (idx, 1, 0);
            item_deleted (setup_list.length ());
        }

        private List<TrainingSetup> setup_list = new List<TrainingSetup> ();
    }
}