namespace ExerciseTimer {
    public class TrainingSetup : Object {
        public string Title { get; set; }
        public int WarmupSec { get; set; }
        public int ExerciseSec { get; set; }
        public int RestSec { get; set; }
        public int Sets { get; set; }

        public int TotalTimeSec {
            get {
                return WarmupSec + Sets * (ExerciseSec + RestSec) - RestSec;
            }
        }

        public string TotalTimeMinuteSecFormatted {
            owned get {
                int minutes = TotalTimeSec / 60;
                int seconds = TotalTimeSec % 60;
                return "%d:%02d".printf (minutes, seconds);
            }
        }

        public string SetInfoFormatted {
            owned get {
                // Translators: detail label for each training in the training list. The first placeholder stands for the number of sets,
                // while the second stands for the length of the exercise in seconds. "s" stands for seconds, this might be needed to be localized.
                return _("%d × %d s").printf (Sets, ExerciseSec + RestSec);
            }
        }
    }
}