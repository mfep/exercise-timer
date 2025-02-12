mod app;
mod config;
mod settings;
mod setup;
mod shortcuts_window;
mod training_editor;
mod training_setup;
mod training_timer;
use gettextrs::gettext;
use relm4::{actions::AccelsPlus, gtk::prelude::*};

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(CloseAction, AppActionGroup, "close");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

fn main() {
    setup::setup();
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap_or_else(|err| {
        panic!(
            "{}: {}",
            // Translators: Error message when cannot connect to the audio output
            gettext("Could not create audio output stream"),
            err
        )
    });
    let app = relm4::main_adw_application();

    let mut actions = relm4::actions::RelmActionGroup::<AppActionGroup>::new();
    let quit_action = {
        let app = app.clone();
        relm4::actions::RelmAction::<QuitAction>::new_stateless(move |_| {
            app.quit();
        })
    };
    let close_action = {
        let app = app.clone();
        relm4::actions::RelmAction::<CloseAction>::new_stateless(move |_| {
            app.active_window().unwrap().close();
        })
    };

    actions.add_action(quit_action);
    actions.add_action(close_action);
    actions.register_for_main_application();
    app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);
    app.set_accelerators_for_action::<CloseAction>(&["<Control>w"]);

    let app = relm4::RelmApp::from_app(app);
    app.run::<app::AppModel>(stream_handle);
}
