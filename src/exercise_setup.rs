use std::time::Duration;

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
    pub exercise_s: usize,
    pub rest_s: usize,
    pub sets: usize,
}

impl ExerciseSetup {
    pub fn total_duration(&self) -> Duration {
        Duration::from_secs((self.exercise_s * self.sets + self.rest_s * (self.sets - 1)) as u64)
    }
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

fn format_duration(d: &Duration) -> String {
    let total_seconds = d.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{}:{:02}", minutes, seconds)
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
                    set_label: &format!("{} ({})", &self.name, format_duration(&self.total_duration())),
                },
                gtk::Grid {
                    set_column_spacing: 8,
                    attach[0, 0, 1, 1] = &gtk::Label {
                        set_halign: gtk::Align::Start,
                        set_label: "Sets:",
                    },
                    attach[1, 0, 1, 1] = &gtk::Label {
                        set_halign: gtk::Align::End,
                        #[watch]
                        set_label: &self.sets.to_string(),
                    },
                    attach[0, 1, 1, 1] = &gtk::Label {
                        set_halign: gtk::Align::Start,
                        set_label: "Exercise (s):",
                    },
                    attach[1, 1, 1, 1] = &gtk::Label {
                        set_halign: gtk::Align::End,
                        #[watch]
                        set_label: &self.exercise_s.to_string(),
                    },
                    attach[0, 2, 1, 1] = &gtk::Label {
                        set_halign: gtk::Align::Start,
                        set_label: "Rest (s):",
                    },
                    attach[1, 2, 1, 1] = &gtk::Label {
                        set_halign: gtk::Align::End,
                        #[watch]
                        set_label: &self.rest_s.to_string(),
                    },
                },
            },
            gtk::Box {
                set_hexpand: true,
                set_halign: gtk::Align::End,
                gtk::Box {
                    set_class_active: ("linked", true),
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,
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
