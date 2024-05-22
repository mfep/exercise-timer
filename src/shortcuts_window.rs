use gettextrs::gettext;
use relm4::{
    self,
    gtk::{self, glib, prelude::*},
    prelude::*,
};

pub struct ShortcutsWindowModel {
    visible: bool,
}

/// Messages
#[derive(Debug)]
pub enum ShortcutsWindowInput {
    Show,
    Hide,
}

fn build_shortcuts_window() -> gtk::ShortcutsWindow {
    gtk::Builder::from_resource("/xyz/safeworlds/hiit/gtk/help-overlay.ui")
        .object("help_overlay")
        .expect(&gettext("Couldn't build the Help Overlay"))
}

#[relm4::component(pub)]
impl SimpleComponent for ShortcutsWindowModel {
    type Init = ();
    type Input = ShortcutsWindowInput;
    type Output = ();
    type Widgets = Widgets;

    view! {
        build_shortcuts_window() -> gtk::ShortcutsWindow {
            #[watch]
            set_visible: model.visible,
            connect_close_request[sender] => move |_| {
                sender.input(ShortcutsWindowInput::Hide);
                glib::Propagation::Stop
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { visible: false };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ShortcutsWindowInput::Show => self.visible = true,
            ShortcutsWindowInput::Hide => self.visible = false,
        }
    }
}
