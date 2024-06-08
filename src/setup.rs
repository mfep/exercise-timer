use crate::config::*;
use gettextrs::gettext;
use relm4::gtk::{self, gio, glib};

pub fn setup() {
    gtk::init().unwrap();
    relm4_icons::initialize_icons();

    setup_gettext();

    glib::set_application_name(&gettext("Exercise Timer"));
    gio::resources_register_include!("hiit.gresource")
        // Translators: Error message printed to the console when the GIO resource file cannot be registered
        .unwrap_or_else(|err| panic!("{}: {}", gettext("Could not register resources"), err));
    setup_css();
    gtk::Window::set_default_icon_name(crate::config::APP_ID);
}

fn setup_gettext() {
    gettextrs::setlocale(gettextrs::LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR)
        // Translators: Error message printed to the console when the i18n text domain cannot be bound
        .unwrap_or_else(|err| panic!("{}: {}", gettext("Unable to bind the text domain"), err));
    gettextrs::textdomain(GETTEXT_PACKAGE).unwrap_or_else(|err| {
        panic!(
            "{}: {}",
            // Translators: Error message printed to the console when the i18n text domain cannot be switched to
            gettext("Unable to switch to the text domain"),
            err
        )
    });
}

fn setup_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/xyz/safeworlds/hiit/style.css");

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default()
            // Translators: Error message printed to the console when there is no Display to apply the custom CSS to
            .unwrap_or_else(|| panic!("{}", gettext("Could not connect to a display"))),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
