namespace ExerciseTimer {
    public interface ITimerPageActionNotifier : Object {
        public signal void restart_action_called ();
    }
}