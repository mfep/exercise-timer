use crate::settings;
use crate::training_editor::*;
use futures::prelude::*;
use gettextrs::gettext;
use relm4::{
    gtk::{self, prelude::*},
    prelude::*,
    RelmWidgetExt,
};
use relm4_icons::icon_names;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TrainingSetup {
    pub name: String,
    pub exercise_s: usize,
    pub rest_s: usize,
    pub sets: usize,
}

impl TrainingSetup {
    pub fn total_duration(&self) -> Duration {
        Duration::from_secs((self.exercise_s * self.sets + self.rest_s * (self.sets - 1)) as u64)
    }
}

impl Default for TrainingSetup {
    fn default() -> Self {
        settings::load_default_exercise_setup()
    }
}

#[derive(Debug)]
pub enum TrainingSetupInput {
    Edit(gtk::Root),
    Update(TrainingSetup),
    Load,
}

#[derive(Debug)]
pub enum TrainingSetupOutput {
    Remove(DynamicIndex),
    Load(TrainingSetup),
}

fn format_duration(d: &Duration) -> String {
    let total_seconds = d.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{}:{:02}", minutes, seconds)
}

#[relm4::factory(pub)]
impl FactoryComponent for TrainingSetup {
    type Init = TrainingSetup;
    type Input = TrainingSetupInput;
    type Output = TrainingSetupOutput;
    type CommandOutput = ();
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
                gtk::CenterBox {
                    set_hexpand: true,
                    set_orientation: gtk::Orientation::Horizontal,
                    #[wrap(Some)]
                    set_start_widget = &gtk::Label {
                        add_css_class: "title-4",
                        #[watch]
                        set_label: &self.name,
                    },
                    #[wrap(Some)]
                    set_end_widget = &gtk::Label {
                        add_css_class: "title-4",
                        #[watch]
                        set_label: &format_duration(&self.total_duration()),
                    },
                },
                gtk::CenterBox {
                    set_hexpand: true,
                    set_orientation: gtk::Orientation::Horizontal,
                    #[wrap(Some)]
                    set_start_widget = &gtk::Grid {
                        set_column_spacing: 24,
                        attach[0, 0, 1, 1] = &gtk::Label {
                            set_halign: gtk::Align::Start,
                            set_label: &gettext("Sets"),
                        },
                        attach[1, 0, 1, 1] = &gtk::Label {
                            set_halign: gtk::Align::Start,
                            #[watch]
                            set_label: &self.sets.to_string(),
                        },
                        attach[0, 1, 1, 1] = &gtk::Label {
                            set_halign: gtk::Align::Start,
                            set_label: &gettext("Exercise"),
                        },
                        attach[1, 1, 1, 1] = &gtk::Label {
                            set_halign: gtk::Align::Start,
                            #[watch]
                            set_label: &format!("{} s", self.exercise_s),
                        },
                        attach[0, 2, 1, 1] = &gtk::Label {
                            set_halign: gtk::Align::Start,
                            set_label: &gettext("Rest"),
                        },
                        attach[1, 2, 1, 1] = &gtk::Label {
                            set_halign: gtk::Align::Start,
                            #[watch]
                            set_label: &format!("{} s", self.rest_s),
                        },
                    },
                    #[wrap(Some)]
                    set_end_widget = &gtk::Box {
                        gtk::Box {
                            set_class_active: ("linked", true),
                            set_orientation: gtk::Orientation::Horizontal,
                            set_valign: gtk::Align::End,
                            gtk::Button {
                                set_icon_name: icon_names::EDIT,
                                connect_clicked[sender] => move |btn| {
                                    sender.input(TrainingSetupInput::Edit(btn.root().unwrap()));
                                },
                                // Translators: tooltip text for exercise card button to open the training editor
                                set_tooltip: &gettext("Edit training"),
                            },
                            gtk::Button {
                                set_class_active: ("destructive-action", true),
                                set_icon_name: icon_names::ENTRY_CLEAR,
                                connect_clicked[sender, index] => move |_| {
                                    sender.output(TrainingSetupOutput::Remove(index.clone())).unwrap();
                                },
                                // Translators: tooltip text for exercise card button to delete the training
                                set_tooltip: &gettext("Delete training"),
                            },
                            gtk::Button {
                                set_class_active: ("suggested-action", true),
                                set_icon_name: icon_names::PLAY,
                                connect_clicked => TrainingSetupInput::Load,
                                // Translators: tooltip text for exercise card button to start the training timer
                                set_tooltip: &gettext("Start training"),
                            },
                        },
                    },
                },
            },
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &Self::Index,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        init
    }

    fn update(&mut self, message: Self::Input, sender: relm4::FactorySender<Self>) {
        match message {
            TrainingSetupInput::Edit(root) => {
                let mut editor = TrainingEditor::builder()
                    .transient_for(root)
                    .launch((TrainingEditorRole::Edit, self.clone()))
                    .into_stream();
                relm4::spawn_local(async move {
                    if let Some(TrainingEditorOutput::Create(setup)) = editor.next().await.unwrap()
                    {
                        sender.input(TrainingSetupInput::Update(setup));
                    }
                });
            }
            TrainingSetupInput::Update(setup) => {
                *self = setup;
            }
            TrainingSetupInput::Load => {
                sender
                    .output(TrainingSetupOutput::Load(self.clone()))
                    .unwrap();
            }
        }
    }
}
