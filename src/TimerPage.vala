namespace ExerciseTimer {
    [GtkTemplate(ui = "/xyz/safeworlds/hiit/ui/timer_page.ui")]
    public class TimerPage : Adw.NavigationPage, ITimerNotifier {
        public TimerPage(TrainingSetup setup, ITimerPageActionNotifier action_notifier) {
            Setup = setup;
            this.action_notifier = action_notifier;
            updateCssClass();
            timer_label_box.set_direction(Gtk.TextDirection.LTR);
            volume_button.get_first_child().css_classes = new string[] { "circular", "toggle", "large-button" };

            var settings = new GLib.Settings(Config.AppId);
            settings.bind("beep-volume", volume_adjustment, "value", GLib.SettingsBindFlags.DEFAULT);
            settings.bind("countdown-sec", this, "CountdownSec", GLib.SettingsBindFlags.DEFAULT);

            this.shown.connect(() => {
                restart();
            });
            this.hidden.connect((_) => {
                Running = false;
                this.action_notifier.restart_action_called.disconnect(restart);
            });
            this.action_notifier.restart_action_called.connect(restart);
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

        public string RemainingSetsFormatted {
            owned get {
                if (Finished) {
                    return "";
                }
                // Translators: Label showing the number of remaining sets on the timer page
                return _("Remaining Sets: %d").printf(remaining_sets);
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

        private bool Running {
            get {
                return running;
            }
            set {
                if (timer_id != null) {
                    GLib.Source.remove(timer_id);
                    timer_id = null;
                }
                if (value) {
                    timer_id = GLib.Timeout.add(1000, onTimeout);
                }
                running = value;
            }
        }

        public string PlayIconName {
            owned get {
                if (Running) {
                    return "pause-symbolic";
                } else {
                    return "play-symbolic";
                }
            }
        }

        public string PlayIconTooltip {
            owned get {
                if (Running) {
                    // Translators: tooltip text for the pause/resume button when the training is running
                    return _("Pause Training");
                } else {
                    // Translators: tooltip text for the pause/resume button when the training is paused
                    return _("Resume Training");
                }
            }
        }

        public int CountdownSec { get; set; }

        [GtkCallback]
        public void restart() {
            remaining_sets = Setup.Sets;
            if (Setup.PreparationSec > 0) {
                current_state = State.Preparation;
                remaining_sec = Setup.PreparationSec;
                preparation_started();
            } else {
                current_state = State.Exercise;
                remaining_sec = Setup.ExerciseSec;
                exercise_started();
            }
            Running = true;
            notifyProperties();
            updateCssClass();
        }

        [GtkCallback]
        public void playPause() {
            Running = !Running;
            notifyProperties();
        }

        private bool onTimeout() {
            bool retval;
            if (remaining_sec > 1) {
                --remaining_sec;
                retval = true;
                if (remaining_sec <= CountdownSec) {
                    countdown(remaining_sec);
                }
            } else {
                switch (current_state) {
                case State.Preparation:
                    current_state = State.Exercise;
                    remaining_sec = Setup.ExerciseSec;
                    retval = true;
                    exercise_started();
                    break;
                case State.Exercise:
                    --remaining_sets;
                    if (remaining_sets > 0) {
                        current_state = State.Rest;
                        remaining_sec = Setup.RestSec;
                        retval = true;
                        rest_started();
                        break;
                    } else {
                        current_state = State.Finished;
                        retval = false;
                        running = false;
                        timer_id = null;
                        finished();
                        break;
                    }
                case State.Rest:
                    current_state = State.Exercise;
                    remaining_sec = Setup.ExerciseSec;
                    exercise_started();
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
            notify_property("RemainingSetsFormatted");
            notify_property("StateFormatted");
            notify_property("Finished");
            notify_property("PlayIconName");
            notify_property("PlayIconTooltip");
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
        [GtkChild]
        unowned Gtk.Adjustment volume_adjustment;
        [GtkChild]
        unowned Gtk.ScaleButton volume_button;

        private State current_state;
        private int remaining_sec;
        private int remaining_sets;
        private uint? timer_id;
        private bool running;
        ITimerPageActionNotifier action_notifier;
    }
}