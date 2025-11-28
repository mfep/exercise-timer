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
                { "shortcuts", this.on_shortcuts_action },
                { "restart", this.on_restart_action },
                { "quit", this.quit },
            };
            this.add_action_entries (action_entries, this);
            this.set_accels_for_action ("app.close", { "<primary>w" });
            this.set_accels_for_action ("app.shortcuts", { "<primary>question" });
            this.set_accels_for_action ("app.restart", { "<primary>r" });
            this.set_accels_for_action ("app.quit", { "<primary>q" });
        }

        public override void activate () {
            base.activate ();
            var win = this.active_window ?? new ExerciseTimer.Window (this);

            var css_provider = new Gtk.CssProvider ();
            css_provider.load_from_resource ("/xyz/safeworlds/hiit/style.css");
            Gtk.StyleContext.add_provider_for_display (Gdk.Display.get_default (), css_provider, Gtk.STYLE_PROVIDER_PRIORITY_USER);

            Gtk.IconTheme.get_for_display (Gdk.Display.get_default ()).add_resource_path ("/xyz/safeworlds/hiit/icons");

            win.present ();
        }

        private void on_about_action () {
            var devs = new string[Config.Developers.length];
            for (int i = 0; i < Config.Developers.length; i++)
                devs[i] = Config.Developers[i];

            var designers = new string[Config.Designers.length];
            for (int i = 0; i < Config.Designers.length; i++)
                designers[i] = Config.Designers[i];

            var about_dialog =
                new Adw.AboutDialog.from_appdata (
                                                  "xyz/safeworlds/hiit/%s.metainfo.xml".printf (Config.AppId),
                                                  Config.Version)
            {
                copyright = Config.Copyright,
                designers = designers,
                developers = devs,
                // Translators: Replace this with your name for it to show up in the about window
                translator_credits = _("translator_credits"),
                version = Config.Version,
            };

            about_dialog.present (get_active_window ());
        }

        private void on_close_action () {
            this.active_window.close ();
        }

        private void on_shortcuts_action () {
            var builder = new Gtk.Builder.from_resource ("/xyz/safeworlds/hiit/ui/shortcuts_dialog.ui");
            var dialog = builder.get_object ("shortcuts_dialog") as Adw.ShortcutsDialog;
            dialog.present (this.get_active_window ());
        }

        private void on_restart_action () {
            var window = this.active_window as ExerciseTimer.Window;
            window?.on_restart_action ();
        }
    }
}