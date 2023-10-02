use relm4::{prelude::{FactoryComponent, DynamicIndex}, gtk, RelmWidgetExt};
use relm4::gtk::prelude::*;

use crate::AppModelInput;

#[derive(Debug)]
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
pub enum ExerciseSetupOutput {
    Remove(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for ExerciseSetup {
    type Init = ExerciseSetup;
    type Input = ();
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
                set_label: &self.name,
            },
            gtk::Box {
                set_class_active: ("linked", true),
                set_hexpand: true,
                set_halign: gtk::Align::End,
                gtk::Button {
                    set_icon_name: "edit",
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
        init
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(match output {
            ExerciseSetupOutput::Remove(index) => AppModelInput::RemoveExerciseSetup(index),
        })
    }
}
