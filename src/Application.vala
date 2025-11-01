namespace ExerciseTimer {
    public class Application : Adw.Application {
        public Application () {
            Object (
                    application_id: Config.AppId,
                    flags: ApplicationFlags.DEFAULT_FLAGS
            );
        }

        construct {
            ActionEntry[] action_entries = {
                { "about", this.on_about_action },
                { "close", this.on_close_action },
                { "quit", this.quit },
            };
            this.add_action_entries (action_entries, this);
            this.set_accels_for_action ("app.quit", { "<primary>q" });
            this.set_accels_for_action ("app.close", { "<primary>w" });
        }

        public override void activate () {
            base.activate ();
            var win = this.active_window ?? new ExerciseTimer.Window (this);

            var css_provider = new Gtk.CssProvider ();
            css_provider.load_from_resource ("/xyz/safeworlds/hiit/style.css");
            Gtk.StyleContext.add_provider_for_display (Gdk.Display.get_default (), css_provider, Gtk.STYLE_PROVIDER_PRIORITY_USER);

            win.present ();
        }

        private void on_about_action () {
            var devs = new string[Config.Developers.length];
            for (int i = 0; i < Config.Developers.length; i++)
                devs[i] = Config.Developers[i];

            var designers = new string[Config.Designers.length];
            for (int i = 0; i < Config.Designers.length; i++)
                designers[i] = Config.Designers[i];

            var about_dialog = new Adw.AboutDialog () {
                application_icon = Config.AppId,
                // Translators: The name of the application. Feel free to localize it!
                application_name = _("Exercise Timer"),
                copyright = Config.Copyright,
                designers = designers,
                developers = devs,
                issue_url = Config.IssueTracker,
                license_type = Gtk.License.GPL_3_0,
                // Translators: Replace this with your name for it to show up in the about window
                translator_credits = _("translator_credits"),
                version = Config.Version,
                website = Config.Homepage,
            };

            about_dialog.present (get_active_window ());
        }

        private void on_close_action () {
            this.active_window.close ();
        }
    }
}