use relm4::{
    gtk::{self, prelude::*, Root},
    prelude::*,
    Component, RelmWidgetExt,
};

use crate::{exercise_editor::ExerciseEditor, AppModelInput};
use crate::{
    exercise_editor::{ExerciseEditorOutput, ExerciseEditorRole},
    settings,
};
use futures::StreamExt;

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
        settings::load_default_exercise_setup()
    }
}

#[derive(Debug)]
pub enum ExerciseSetupInput {
    Edit(Root),
    Update(ExerciseSetup),
    Load,
}

#[derive(Debug)]
pub enum ExerciseSetupOutput {
    Remove(DynamicIndex),
    Load(ExerciseSetup),
}

#[relm4::factory(pub)]
impl FactoryComponent for ExerciseSetup {
    type Init = ExerciseSetup;
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
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                gtk::Label {
                    set_class_active: ("title-4", true),
                    set_halign: gtk::Align::Start,
                    #[watch]
                    set_label: &self.name,
                },
                // ToDo Grid
                gtk::Label {
                    set_halign: gtk::Align::Start,
                    #[watch]
                    set_label: &format!("Sets: {}", self.sets),
                },
                gtk::Label {
                    set_halign: gtk::Align::Start,
                    #[watch]
                    set_label: &format!("Warmup: {}s", self.warmup_s),
                },
                gtk::Label {
                    set_halign: gtk::Align::Start,
                    #[watch]
                    set_label: &format!("Exercise: {}s", self.exercise_s),
                },
                gtk::Label {
                    set_halign: gtk::Align::Start,
                    #[watch]
                    set_label: &format!("Rest: {}s", self.rest_s),
                },
            },
            gtk::Box {
                set_class_active: ("linked", true),
                set_hexpand: true,
                set_halign: gtk::Align::End,
                gtk::Button {
                    set_icon_name: "edit",
                    connect_clicked[sender] => move |btn| {
                        sender.input(ExerciseSetupInput::Edit(btn.root().unwrap()));
                    },
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
                    connect_clicked => ExerciseSetupInput::Load,
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
            ExerciseSetupOutput::Load(setup) => AppModelInput::LoadExercise(setup),
        })
    }

    fn update(&mut self, message: Self::Input, sender: relm4::FactorySender<Self>) {
        match message {
            ExerciseSetupInput::Edit(root) => {
                let mut editor = ExerciseEditor::builder()
                    .transient_for(root)
                    .launch((ExerciseEditorRole::Edit, self.clone()))
                    .into_stream();
                relm4::spawn_local(async move {
                    if let Some(ExerciseEditorOutput::Create(setup)) = editor.next().await.unwrap()
                    {
                        sender.input(ExerciseSetupInput::Update(setup));
                    }
                });
            }
            ExerciseSetupInput::Update(setup) => {
                *self = setup;
            }
            ExerciseSetupInput::Load => {
                sender.output(ExerciseSetupOutput::Load(self.clone()));
            }
        }
    }
}
