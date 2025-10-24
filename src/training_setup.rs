use crate::icon_names;
use crate::settings;
use crate::training_editor::*;
use futures::prelude::*;
use gettextrs::gettext;
use libadwaita::prelude::AdwDialogExt;
use relm4::{
    gtk::{self, prelude::*},
    prelude::*,
    RelmWidgetExt,
};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TrainingSetup {
    pub name: String,
    pub exercise_s: usize,
    pub rest_s: usize,
    pub sets: usize,
    pub prepare_s: usize,
}

impl TrainingSetup {
    pub fn total_duration(&self) -> Duration {
        Duration::from_secs(
            (self.exercise_s * self.sets + self.rest_s * (self.sets - 1) + self.prepare_s) as u64,
        )
    }
}

impl Default for TrainingSetup {
    fn default() -> Self {
        settings::load_default_training_setup()
    }
}

#[derive(Debug)]
pub enum TrainingSetupInput {
    Edit(gtk::Root),
    Update(TrainingSetup),
}

#[derive(Debug)]
pub enum TrainingSetupOutput {
    Remove(DynamicIndex),
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
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::ListBoxRow {
            inline_css: "padding: 12px",
            set_activatable: true,
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
                    set_margin_top: 5,
                    #[wrap(Some)]
                    set_start_widget = &gtk::Label {
                        #[watch]
                        set_label: &if true {
                            gettext!("{} × {} s", self.sets, self.exercise_s + self.rest_s)
                        } else {
                            // Translators: detail label for each training in the training list. The first placeholder stands for the number of sets,
                            // while the second stands for the length of the exercise in seconds. "s" stands for seconds, this might be needed to be localized.
                            gettext("{} × {} s")
                        }
                    },
                    #[wrap(Some)]
                    set_end_widget = &gtk::Box {
                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_valign: gtk::Align::End,
                            gtk::Button {
                                set_icon_name: icon_names::EDIT,
                                connect_clicked[sender] => move |btn| {
                                    sender.input(TrainingSetupInput::Edit(btn.root().unwrap()));
                                },
                                set_margin_end: 10,
                                // Translators: tooltip text for exercise card button to open the training editor
                                set_tooltip: &gettext("Edit Training"),
                            },
                            gtk::Button {
                                set_icon_name: "user-trash-symbolic",
                                connect_clicked[sender, index] => move |_| {
                                    sender.output(TrainingSetupOutput::Remove(index.clone())).unwrap();
                                },
                                // Translators: tooltip text for exercise card button to delete the training
                                set_tooltip: &gettext("Delete Training"),
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
                let editor =
                    TrainingEditor::builder().launch((TrainingEditorRole::Edit, self.clone()));
                editor
                    .widget()
                    .present(Some(&root.toplevel_window().unwrap()));
                let mut editor = editor.into_stream();
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
        }
    }
}
