namespace ExerciseTimer {
    public interface ITimerNotifier {
        public signal void preparation_started ();
        public signal void exercise_started ();
        public signal void rest_started ();
        public signal void finished ();
        public signal void countdown (int remaining_s);
    }
}