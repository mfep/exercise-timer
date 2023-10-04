use std::rc::Rc;

use relm4::gtk::prelude::*;
use relm4::ComponentController;
use relm4::{
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
    Controller, RelmWidgetExt,
};

use crate::exercise_editor::{ExerciseEditorInput, ExerciseEditorRole};
use crate::{exercise_editor::ExerciseEditor, AppModelInput};

#[derive(Debug, Clone)]
pub struct ExerciseSetup {
    pub name: String,
    pub warmup_s: usize,
    pub exercise_s: usize,
    pub rest_s: usize,
    pub sets: usize,
}

impl Default for ExerciseSetup {
    fn default() -> Self {
        Self {
            name: String::from("Good Exercise"),
            warmup_s: 2,
            exercise_s: 2,
            rest_s: 2,
            sets: 2,
        }
    }
}

#[derive(Debug)]
pub struct ExerciseSetupModel {
    setup: ExerciseSetup,
    edit_dialog: Rc<Controller<ExerciseEditor>>,
}

#[derive(Debug)]
pub enum ExerciseSetupInput {
    Edit,
}

#[derive(Debug)]
pub enum ExerciseSetupOutput {
    Remove(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for ExerciseSetupModel {
    type Init = (ExerciseSetup, Rc<Controller<ExerciseEditor>>);
    type Input = ExerciseSetupInput;
    type Output = ExerciseSetupOutput;
    type CommandOutput = ();
    type ParentInput = AppModelInput;
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            set_hexpand: true,
            set_class_active: ("card", true),
            set_margin_top: 5,
            set_margin_start: 5,
            set_margin_end: 5,
            inline_css: "padding: 10px",
            gtk::Label {
                set_class_active: ("title-4", true),
                #[watch]
                set_label: &self.setup.name,
            },
            gtk::Box {
                set_class_active: ("linked", true),
                set_hexpand: true,
                set_halign: gtk::Align::End,
                gtk::Button {
                    set_icon_name: "edit",
                    connect_clicked => ExerciseSetupInput::Edit,
                },
                gtk::Button {
                    set_class_active: ("destructive-action", true),
                    set_icon_name: "entry-clear",
                    connect_clicked[sender, index] => move |_| {
                        sender.output(ExerciseSetupOutput::Remove(index.clone()))
                    },
                },
                gtk::Button {
                    set_class_active: ("suggested-action", true),
                    set_icon_name: "play",
                }
            }
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &Self::Index,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        Self {
            setup: init.0,
            edit_dialog: init.1,
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(match output {
            ExerciseSetupOutput::Remove(index) => AppModelInput::RemoveExerciseSetup(index),
        })
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::FactorySender<Self>) {
        match message {
            ExerciseSetupInput::Edit => {
                self.edit_dialog
                    .emit(ExerciseEditorInput::Show(ExerciseEditorRole::Edit));
            }
        }
    }
}
