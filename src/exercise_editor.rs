use crate::exercise_setup::ExerciseSetup;
use gtk::prelude::{ButtonExt, EditableExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{
    adw::{self, prelude::*},
    gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};

pub struct ExerciseEditor {
    setup: ExerciseSetup,
    is_active: bool,
}

#[derive(Debug)]
pub enum ExerciseEditorInput {
    Show,
    Create,
    Cancel,
    NameChanged(String),
}

#[derive(Debug)]
pub enum ExerciseEditorOutput {
    Create(ExerciseSetup),
}

#[relm4::component(pub)]
impl SimpleComponent for ExerciseEditor {
    type Init = ExerciseSetup;
    type Input = ExerciseEditorInput;
    type Output = ExerciseEditorOutput;

    view! {
        adw::Window {
            #[watch]
            set_visible: model.is_active,
            set_modal: true,
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar {
                    set_title_widget: Some(&adw::WindowTitle::new("Create a new Exercise", "")),
                    set_show_end_title_buttons: false,
                    pack_start = &gtk::Button {
                        set_label: "Cancel",
                        connect_clicked => ExerciseEditorInput::Cancel,
                    },
                    pack_end = &gtk::Button {
                        set_label: "Create",
                        set_class_active: ("suggested-action", true),
                        connect_clicked => ExerciseEditorInput::Create,
                    }
                },
                gtk::Box {
                    set_margin_all: 20,
                    set_class_active: ("card", true),
                    set_orientation: gtk::Orientation::Vertical,
                    adw::EntryRow {
                        set_title: "Name",
                        connect_changed[sender] => move |row| {
                            sender
                                .input_sender()
                                .send(ExerciseEditorInput::NameChanged(row.text().to_string()))
                                .unwrap();
                        }
                    },
                    adw::ActionRow {
                        set_title: "Number of sets",
                        add_suffix = &gtk::SpinButton {
                            set_margin_top: 8,
                            set_margin_bottom: 8,
                            set_adjustment = &gtk::Adjustment {
                                set_lower: 1f64,
                                set_upper: 999f64,
                                set_step_increment: 1f64,
                                #[watch]
                                set_value: model.setup.sets as f64,
                            }
                        }
                    },
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = ExerciseEditor {
            setup: init,
            is_active: false,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ExerciseEditorInput::Show => {
                self.is_active = true;
            }
            ExerciseEditorInput::Cancel => {
                self.is_active = false;
            }
            ExerciseEditorInput::Create => {
                self.is_active = false;
                sender
                    .output_sender()
                    .send(ExerciseEditorOutput::Create(self.setup.clone()))
                    .unwrap();
            }
            ExerciseEditorInput::NameChanged(name) => {
                self.setup.name = name;
            }
        }
    }
}
