use crate::exercise_setup::*;
use relm4::{
    adw::{self, prelude::*},
    binding::*,
    gtk, RelmObjectExt, RelmWidgetExt,
};

#[derive(Debug)]
pub struct ExerciseEditor {
    role: ExerciseEditorRole,
    name: StringBinding,
    sets: U32Binding,
    exercise_s: U32Binding,
    rest_s: U32Binding,
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
}

#[derive(Debug)]
pub enum ExerciseEditorOutput {
    Create(ExerciseSetup),
}

pub const SPIN_ROW_LOWER: f64 = 1f64;
pub const SPIN_ROW_UPPER: f64 = 1000000f64;
pub const SPIN_ROW_STEP: f64 = 1f64;

#[relm4::component(pub)]
impl relm4::SimpleComponent for ExerciseEditor {
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
                        add_binding: (&model.name, "text"),
                    },
                    adw::SpinRow {
                        set_title: "Number of sets",
                        #[wrap(Some)]
                        set_adjustment = &gtk::Adjustment {
                            set_lower: SPIN_ROW_LOWER,
                            set_upper: SPIN_ROW_UPPER,
                            set_step_increment: SPIN_ROW_STEP,
                            add_binding: (&model.sets, "value"),
                        },
                    },
                    adw::SpinRow {
                        set_title: "Rest time",
                        set_subtitle: "seconds",
                        #[wrap(Some)]
                        set_adjustment = &gtk::Adjustment {
                            set_lower: SPIN_ROW_LOWER,
                            set_upper: SPIN_ROW_UPPER,
                            set_step_increment: SPIN_ROW_STEP,
                            add_binding: (&model.rest_s, "value"),
                        },
                    },
                    adw::SpinRow {
                        set_title: "Exercise time",
                        set_subtitle: "seconds",
                        #[wrap(Some)]
                        set_adjustment = &gtk::Adjustment {
                            set_lower: SPIN_ROW_LOWER,
                            set_upper: SPIN_ROW_UPPER,
                            set_step_increment: SPIN_ROW_STEP,
                            add_binding: (&model.exercise_s, "value"),
                        },
                    },
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = ExerciseEditor {
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
            ExerciseEditorInput::Cancel => sender.output(None).unwrap(),
            ExerciseEditorInput::Create => {
                sender
                    .output(Some(ExerciseEditorOutput::Create(ExerciseSetup {
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
