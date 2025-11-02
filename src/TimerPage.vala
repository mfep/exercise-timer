namespace ExerciseTimer {
    [GtkTemplate(ui = "/xyz/safeworlds/hiit/ui/timer_page.ui")]
    public class TimerPage : Adw.NavigationPage {
        public TimerPage(TrainingSetup setup) {
            Setup = setup;
            remaining_sets = setup.Sets;
            if (setup.WarmupSec > 0) {
                current_state = State.Preparation;
                remaining_sec = setup.WarmupSec;
            } else {
                current_state = State.Exercise;
                remaining_sec = setup.ExerciseSec;
            }

            var timer_id = GLib.Timeout.add(1000, onTimeout);
            this.hidden.connect((_) => {
                if (current_state != State.Finished) {
                    GLib.Source.remove(timer_id);
                }
            });

            notifyProperties();
            updateCssClass();
            timer_label_box.set_direction(Gtk.TextDirection.LTR);
        }

        public TrainingSetup Setup { get; private set; }

        public bool Finished {
            get {
                return current_state == State.Finished;
            }
        }

        public string RemainingMinFormatted {
            owned get {
                var min = remaining_sec / 60;
                return "%02d".printf(min);
            }
        }

        public string RemainingSecFormatted {
            owned get {
                var sec = remaining_sec % 60;
                return "%02d".printf(sec);
            }
        }

        public string StateFormatted {
            owned get {
                switch (current_state) {
                case State.Preparation:
                    // Translators: Shown on the timer page during preparation
                    return _("Preparation");
                case State.Exercise:
                    // Translators: Shown on the timer page during exercise
                    return _("Exercise");
                case State.Rest:
                    // Translators: Shown on the timer page during rest
                    return _("Rest");
                case State.Finished:
                    return "";
                default:
                    assert_not_reached();
                }
            }
        }

        private bool onTimeout() {
            bool retval;
            if (remaining_sec > 1) {
                --remaining_sec;
                retval = true;
            } else {
                switch (current_state) {
                case State.Preparation:
                    current_state = State.Exercise;
                    remaining_sec = Setup.ExerciseSec;
                    retval = true;
                    break;
                case State.Exercise:
                    --remaining_sets;
                    if (remaining_sets > 0) {
                        current_state = State.Rest;
                        remaining_sec = Setup.RestSec;
                        retval = true;
                        break;
                    } else {
                        current_state = State.Finished;
                        retval = false;
                        break;
                    }
                case State.Rest:
                    current_state = State.Exercise;
                    remaining_sec = Setup.ExerciseSec;
                    retval = true;
                    break;
                default:
                    assert_not_reached();
                }
                updateCssClass();
            }

            notifyProperties();
            return retval;
        }

        private void notifyProperties() {
            notify_property("RemainingMinFormatted");
            notify_property("RemainingSecFormatted");
            notify_property("StateFormatted");
            notify_property("Finished");
        }

        private void updateCssClass() {
            var classes = new string[2];
            classes[0] = "card";
            switch (current_state) {
            case State.Preparation:
                classes[1] = "timer-warmup";
                break;
            case State.Exercise:
            case State.Finished:
                classes[1] = "timer-exercise";
                break;
            case State.Rest:
                classes[1] = "timer-rest";
                break;
            default:
                assert_not_reached();
            }
            timer_card.css_classes = classes;
        }

        private enum State {
            Preparation,
            Exercise,
            Rest,
            Finished,
        }

        [GtkChild]
        unowned Gtk.Box timer_card;
        [GtkChild]
        unowned Gtk.Box timer_label_box;

        private State current_state;
        private int remaining_sec;
        private int remaining_sets;
    }
}