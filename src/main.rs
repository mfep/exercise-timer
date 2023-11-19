mod app;
mod config;
mod exercise_editor;
mod exercise_setup;
mod exercise_timer;
mod settings;
mod settings_dialog;
mod setup;
use relm4::{actions::AccelsPlus, gtk::prelude::*};

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

fn main() {
    setup::setup();
    let (_stream, stream_handle) =
        rodio::OutputStream::try_default().expect("Could not create audio output stream");
    let app = relm4::main_adw_application();

    let mut actions = relm4::actions::RelmActionGroup::<AppActionGroup>::new();
    let quit_action = {
        let app = app.clone();
        relm4::actions::RelmAction::<QuitAction>::new_stateless(move |_| {
            app.quit();
        })
    };
    actions.add_action(quit_action);
    actions.register_for_main_application();
    app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

    let app = relm4::RelmApp::from_app(app);
    app.run::<app::AppModel>(stream_handle);
}
