use crate::training_setup::*;
use gettextrs::gettext;
use relm4::{
    adw::{self, prelude::*},
    binding::*,
    gtk, RelmObjectExt, RelmWidgetExt,
};

#[derive(Debug)]
pub struct TrainingEditor {
    role: TrainingEditorRole,
    name: StringBinding,
    sets: U32Binding,
    exercise_s: U32Binding,
    rest_s: U32Binding,
}

#[derive(Debug)]
pub enum TrainingEditorRole {
    New,
    Edit,
}

#[derive(Debug)]
pub enum TrainingEditorInput {
    Create,
    Cancel,
}

#[derive(Debug)]
pub enum TrainingEditorOutput {
    Create(TrainingSetup),
}

pub const SPIN_ROW_LOWER: f64 = 1f64;
pub const SPIN_ROW_UPPER: f64 = 1000000f64;
pub const SPIN_ROW_STEP: f64 = 1f64;

#[relm4::component(pub)]
impl relm4::SimpleComponent for TrainingEditor {
    type Init = (TrainingEditorRole, TrainingSetup);
    type Input = TrainingEditorInput;
    type Output = Option<TrainingEditorOutput>;

    view! {
        window = adw::Dialog {
            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: &match model.role {
                            // Translators: The editor window's title when creating a new training
                            TrainingEditorRole::New => gettext("New Training"),
                            // Translators: The editor window's title when modifying a training
                            TrainingEditorRole::Edit => gettext("Edit Training"),
                        },
                    },
                    set_show_end_title_buttons: false,
                    pack_start = &gtk::Button {
                        // Translators: Button to close the training editor window without modifications
                        set_label: &gettext("Cancel"),
                        connect_clicked => TrainingEditorInput::Cancel,
                    },
                    pack_end = &gtk::Button {
                        set_label: &match model.role {
                            // Translators: Button to close the editor window and create a new training
                            TrainingEditorRole::New => gettext("Create"),
                            // Translators: Button to close the editor window and update an existing training
                            TrainingEditorRole::Edit => gettext("Update"),
                        },
                        set_class_active: ("suggested-action", true),
                        connect_clicked => TrainingEditorInput::Create,
                    }
                },
                adw::Clamp {
                    set_margin_all: 20,
                    gtk::Box
                    {
                        set_orientation: gtk::Orientation::Vertical,
                        adw::PreferencesGroup
                        {
                            set_margin_bottom: 10,
                            adw::EntryRow {
                                // Translators: The title of the field for the name of the training in the editor window
                                set_title: &gettext("Name"),
                                add_binding: (&model.name, "text"),
                            },
                        },
                        adw::PreferencesGroup
                        {
                            adw::SpinRow {
                                // Translators: The title of the field for the number of sets in the training in the editor window
                                set_title: &gettext("Number of Sets"),
                                #[wrap(Some)]
                                set_adjustment = &gtk::Adjustment {
                                    set_lower: SPIN_ROW_LOWER,
                                    set_upper: SPIN_ROW_UPPER,
                                    set_step_increment: SPIN_ROW_STEP,
                                    add_binding: (&model.sets, "value"),
                                },
                            },
                            adw::SpinRow {
                                // Translators: The title of the field for the rest duration in the training in the editor window
                                set_title: &gettext("Rest Time"),
                                // Translators: The subtitle of the field for the duration which refers to the unit. Singular form in some localizations.
                                set_subtitle: &gettext("seconds"),
                                #[wrap(Some)]
                                set_adjustment = &gtk::Adjustment {
                                    set_lower: SPIN_ROW_LOWER,
                                    set_upper: SPIN_ROW_UPPER,
                                    set_step_increment: SPIN_ROW_STEP,
                                    add_binding: (&model.rest_s, "value"),
                                },
                            },
                            adw::SpinRow {
                                // Translators: The title of the field for the exercise duration in the training in the editor window
                                set_title: &gettext("Exercise Time"),
                                // Translators: The subtitle of the field for the duration which refers to the unit. Singular form in some localizations.
                                set_subtitle: &gettext("seconds"),
                                #[wrap(Some)]
                                set_adjustment = &gtk::Adjustment {
                                    set_lower: SPIN_ROW_LOWER,
                                    set_upper: SPIN_ROW_UPPER,
                                    set_step_increment: SPIN_ROW_STEP,
                                    add_binding: (&model.exercise_s, "value"),
                                },
                            },
                        },
                    }
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = TrainingEditor {
            name: StringBinding::new(init.1.name.clone()),
            sets: U32Binding::new(init.1.sets as u32),
            rest_s: U32Binding::new(init.1.rest_s as u32),
            exercise_s: U32Binding::new(init.1.exercise_s as u32),
            role: init.0,
        };
        let widgets = view_output!();
        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            TrainingEditorInput::Cancel => sender.output(None).unwrap(),
            TrainingEditorInput::Create => {
                sender
                    .output(Some(TrainingEditorOutput::Create(TrainingSetup {
                        name: self.name.get(),
                        exercise_s: self.exercise_s.get() as usize,
                        rest_s: self.rest_s.get() as usize,
                        sets: self.sets.get() as usize,
                    })))
                    .unwrap();
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.window.close();
    }
}
