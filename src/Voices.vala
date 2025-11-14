namespace ExerciseTimer {
    public class Voices : Object {
        public double Volume { get; set; }

        public Voices(ITimerNotifier notifier) {
            for (var i = 0; i < ping_files.length; ++i) {
                var resource_path = "/xyz/safeworlds/hiit/audio/ping%d.wav".printf(i + 1);
                ping_files[i] = Gtk.MediaFile.for_resource(resource_path);
            }

            var settings = new GLib.Settings(Config.AppId);
            settings.bind("beep-volume", this, "Volume", GLib.SettingsBindFlags.DEFAULT);

            notifier.preparation_started.connect(() => { ping(2); });
            notifier.exercise_started.connect(() => { ping(2); });
            notifier.rest_started.connect(() => { ping(2); });
            notifier.finished.connect(() => { ping(3); });
            notifier.countdown.connect((_) => { ping(1); });
        }

        private void ping(int times) {
            ping_files[times - 1].volume = Volume;
            ping_files[times - 1].seek(0);
            ping_files[times - 1].play_now();
        }

        private Gtk.MediaFile[] ping_files = new Gtk.MediaFile[3];
    }
}