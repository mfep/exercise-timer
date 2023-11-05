use relm4::gtk;

// use gettextrs::{gettext, LocaleCategory};
use gtk::{gio, glib};

pub fn setup() {
    gtk::init().unwrap();
    relm4_icons::initialize_icons();

    // setup_gettext();

    // glib::set_application_name(&gettext("GTK Rust Template"));
    glib::set_application_name("Exercise Timer");
    gio::resources_register_include!("hiit.gresource").expect("Could not register resources");
    setup_css();
    gtk::Window::set_default_icon_name(crate::config::APP_ID);
}

// fn setup_gettext() {
//     // Prepare i18n
//     gettextrs::setlocale(LocaleCategory::LcAll, "");
//     gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
//     gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
// }

fn setup_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/xyz/safeworlds/hiit/style.css");

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
