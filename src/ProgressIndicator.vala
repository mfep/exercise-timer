namespace ExerciseTimer {
    public class ProgressIndicator : Gtk.Widget {
        public ProgressIndicator (TrainingSetup training_setup) {
            this.training_setup = training_setup;
            height_request = 20;
        }

        public int CurrentSec {
            get {
                return current_sec;
            }
            set {
                current_sec = value;
                queue_draw ();
            }
        }

        public override void snapshot (Gtk.Snapshot snapshot) {
            var width = get_width ();
            var height = get_height ();
            var bounds = Graphene.Rect () {
                origin = { 0, 0 },
                size = { width, height }
            };

            Cairo.Context cr = snapshot.append_cairo (bounds);

            double triangle_width = 10;
            double capsule_width = width - triangle_width;
            double capsule_height = height / 2;
            double r = capsule_height / 2.0;

            // Capsule path
            cr.new_path ();

            // Left semicircle
            cr.arc (
                    triangle_width / 2 + r,
                    r,
                    r,
                    Math.PI / 2,
                    Math.PI * 3 / 2
            );

            // Top line
            cr.line_to (triangle_width / 2 + capsule_width - r, 0);

            // Right semicircle
            cr.arc (
                    triangle_width / 2 + capsule_width - r,
                    r,
                    r,
                    -Math.PI / 2,
                    Math.PI / 2
            );

            // Bottom line
            cr.close_path ();

            // Use capsule as clipping region
            cr.save ();
            cr.clip ();

            double current_x = triangle_width / 2;
            double total_sec = training_setup.TotalTimeSec;
            if (training_setup.PreparationSec > 0) {
                double ratio = training_setup.PreparationSec / total_sec;
                double this_width = ratio * capsule_width;
                // #63452c
                cr.set_source_rgb (99.0 / 255, 69.0 / 255, 44.0 / 255);
                cr.rectangle (current_x, 0, this_width, capsule_height);
                cr.fill ();
                current_x += this_width;
            }

            double exercise_ratio = training_setup.ExerciseSec / total_sec;
            double exercise_width = exercise_ratio * capsule_width;
            double rest_ratio = training_setup.RestSec / total_sec;
            double rest_width = rest_ratio * capsule_width;

            for (int set_idx = 0; set_idx < training_setup.Sets; ++set_idx) {
                // #26a269
                cr.set_source_rgb (38.0 / 255, 162.0 / 255, 105.0 / 255);
                cr.rectangle (current_x, 0, exercise_width, capsule_height);
                cr.fill ();
                current_x += exercise_width;

                if (set_idx != training_setup.Sets - 1 && training_setup.RestSec > 0) {
                    // #1a5fb4
                    cr.set_source_rgb (26.0 / 255, 95.0 / 255, 180.0 / 255);
                    cr.rectangle (current_x, 0, rest_width, capsule_height);
                    cr.fill ();
                    current_x += rest_width;
                }
            }
            cr.restore ();

            if (Adw.StyleManager.get_default ().dark) {
                cr.set_source_rgb (1, 1, 1);
            } else {
                cr.set_source_rgb (0, 0, 0);
            }

            double cursor_pos = triangle_width / 2 + current_sec / total_sec * capsule_width;
            cr.move_to (cursor_pos, 0);
            cr.line_to (cursor_pos, height);
            cr.stroke ();

            cr.move_to (cursor_pos, capsule_height);
            cr.line_to (cursor_pos - triangle_width / 2, height);
            cr.line_to (cursor_pos + triangle_width / 2, height);
            cr.close_path ();
            cr.fill ();
        }

        private TrainingSetup training_setup;
        private int current_sec = 0;
    }
}