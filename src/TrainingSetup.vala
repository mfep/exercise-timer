using Json;

namespace ExerciseTimer {
    public class TrainingSetup : GLib.Object, Json.Serializable {
        public string Title { get; set; }
        public int PreparationSec {
            get {
                return preparation_sec;
            }
            set {
                preparation_sec = value;
                notify_properties ();
            }
        }

        public int ExerciseSec {
            get {
                return exercise_sec;
            }
            set {
                exercise_sec = value;
                notify_properties ();
            }
        }

        public int RestSec {
            get {
                return rest_sec;
            }
            set {
                rest_sec = value;
                notify_properties ();
            }
        }

        public int Sets {
            get {
                return sets;
            }
            set {
                sets = value;
                notify_properties ();
            }
        }

        public signal void deleted (TrainingSetup setup);

        public override unowned GLib.ParamSpec? find_property (string name)
        {
            // Ensuring compatibility with previous versions
            GLib.Type type = this.get_type ();
            GLib.ObjectClass ocl = (GLib.ObjectClass) type.class_ref ();
            switch (name) {
            case "name" :
            case "Title":
                return ocl.find_property ("Title");

            case "sets":
            case "Sets":
                return ocl.find_property ("Sets");

            case "warmup_s":
            case "PreparationSec":
                return ocl.find_property ("PreparationSec");

            case "exercise_s":
            case "ExerciseSec":
                return ocl.find_property ("ExerciseSec");

            case "rest_s":
            case "RestSec":
                return ocl.find_property ("RestSec");
            }
            return null;
        }

        public override (unowned GLib.ParamSpec)[] list_properties () {
            GLib.Type type = this.get_type ();
            GLib.ObjectClass ocl = (GLib.ObjectClass) type.class_ref ();
            return new (unowned GLib.ParamSpec)[] {
                       ocl.find_property ("Title"),
                       ocl.find_property ("Sets"),
                       ocl.find_property ("PreparationSec"),
                       ocl.find_property ("ExerciseSec"),
                       ocl.find_property ("RestSec"),
            };
        }

        public int TotalTimeSec {
            get {
                return PreparationSec + Sets * (ExerciseSec + RestSec) - RestSec;
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

        private void notify_properties () {
            notify_property ("TotalTimeSec");
            notify_property ("TotalTimeMinuteSecFormatted");
            notify_property ("SetInfoFormatted");
        }

        private int preparation_sec;
        private int exercise_sec;
        private int rest_sec;
        private int sets;
    }
}