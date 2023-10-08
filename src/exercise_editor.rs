use crate::exercise_setup::ExerciseSetup;
use gtk::prelude::{ButtonExt, EditableExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{
    adw::{self, prelude::*},
    gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};

#[derive(Debug)]
pub struct ExerciseEditor {
    setup: ExerciseSetup,
    role: ExerciseEditorRole,
}

#[derive(Debug)]
pub enum ExerciseEditorRole {
    New,
    Edit,
}

#[derive(Debug)]
pub enum ExerciseEditorInput {
    Create,
    Cancel,
    NameChanged(String),
    SetsChanged(usize),
    WarmupChanged(usize),
    ExerciseChanged(usize),
    RestChanged(usize),
}

#[derive(Debug)]
pub enum ExerciseEditorOutput {
    Create(ExerciseSetup),
}

#[relm4::component(pub)]
impl SimpleComponent for ExerciseEditor {
    type Init = (ExerciseEditorRole, ExerciseSetup);
    type Input = ExerciseEditorInput;
    type Output = Option<ExerciseEditorOutput>;

    view! {
        window = adw::Window {
            set_modal: true,
            set_visible: true,
            set_default_width: 400,
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: match model.role {
                            ExerciseEditorRole::New => "New exercise",
                            ExerciseEditorRole::Edit => "Edit exercise",
                        },
                    },
                    set_show_end_title_buttons: false,
                    pack_start = &gtk::Button {
                        set_label: "Cancel",
                        connect_clicked => ExerciseEditorInput::Cancel,
                    },
                    pack_end = &gtk::Button {
                        set_label: match model.role {
                            ExerciseEditorRole::New => "Create",
                            ExerciseEditorRole::Edit => "Update",
                        },
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
                        set_text: &model.setup.name,
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
                                set_value: model.setup.sets as f64,
                                connect_value_changed[sender] => move |adj| {
                                    sender
                                        .input_sender()
                                        .send(ExerciseEditorInput::SetsChanged(adj.value() as usize))
                                        .unwrap()
                                },
                            }
                        }
                    },
                    adw::ActionRow {
                        set_title: "Warmup time",
                        set_subtitle: "seconds",
                        add_suffix = &gtk::SpinButton {
                            set_margin_top: 8,
                            set_margin_bottom: 8,
                            set_adjustment = &gtk::Adjustment {
                                set_lower: 0f64,
                                set_upper: 999f64,
                                set_step_increment: 1f64,
                                set_value: model.setup.warmup_s as f64,
                                connect_value_changed[sender] => move |adj| {
                                    sender
                                        .input_sender()
                                        .send(ExerciseEditorInput::WarmupChanged(adj.value() as usize))
                                        .unwrap()
                                },
                            }
                        }
                    },
                    adw::ActionRow {
                        set_title: "Rest time",
                        set_subtitle: "seconds",
                        add_suffix = &gtk::SpinButton {
                            set_margin_top: 8,
                            set_margin_bottom: 8,
                            set_adjustment = &gtk::Adjustment {
                                set_lower: 1f64,
                                set_upper: 999f64,
                                set_step_increment: 1f64,
                                set_value: model.setup.rest_s as f64,
                                connect_value_changed[sender] => move |adj| {
                                    sender
                                        .input_sender()
                                        .send(ExerciseEditorInput::RestChanged(adj.value() as usize))
                                        .unwrap()
                                },
                            }
                        }
                    },
                    adw::ActionRow {
                        set_title: "Exercise time",
                        set_subtitle: "seconds",
                        add_suffix = &gtk::SpinButton {
                            set_margin_top: 8,
                            set_margin_bottom: 8,
                            set_adjustment = &gtk::Adjustment {
                                set_lower: 1f64,
                                set_upper: 999f64,
                                set_step_increment: 1f64,
                                set_value: model.setup.exercise_s as f64,
                                connect_value_changed[sender] => move |adj| {
                                    sender
                                        .input_sender()
                                        .send(ExerciseEditorInput::ExerciseChanged(adj.value() as usize))
                                        .unwrap()
                                },
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
            setup: init.1,
            role: init.0,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ExerciseEditorInput::Cancel => sender.output(None).unwrap(),
            ExerciseEditorInput::Create => {
                sender
                    .output(Some(ExerciseEditorOutput::Create(self.setup.clone())))
                    .unwrap();
            }
            ExerciseEditorInput::NameChanged(name) => {
                self.setup.name = name;
            }
            ExerciseEditorInput::SetsChanged(val) => {
                self.setup.sets = val;
            }
            ExerciseEditorInput::WarmupChanged(val) => {
                self.setup.warmup_s = val;
            }
            ExerciseEditorInput::ExerciseChanged(val) => {
                self.setup.exercise_s = val;
            }
            ExerciseEditorInput::RestChanged(val) => {
                self.setup.rest_s = val;
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.window.close();
    }
}
