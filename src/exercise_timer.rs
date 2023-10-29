mod audio_player;
mod timer;

use relm4::{
    adw,
    gtk::{self, prelude::*},
    Component, ComponentParts, ComponentSender, RelmWidgetExt, WorkerController,
};
use timer::{TimerModel, TimerOutput};

use crate::exercise_setup::ExerciseSetup;
use audio_player::{AudioPlayerInput, AudioPlayerModel};

#[derive(PartialEq)]
enum ExerciseState {
    Warmup,
    Exercise,
    Rest,
}

pub struct ExerciseTimer {
    setup: ExerciseSetup,
    warmup_s: usize,
    state: ExerciseState,
    remaining_sets: usize,
    remaining_s: usize,
    running: bool,
    timer: Option<WorkerController<TimerModel>>,
    audio_player: WorkerController<AudioPlayerModel>,
}

impl ExerciseTimer {
    fn new(
        exercise: ExerciseSetup,
        warmup_s: usize,
        output: rodio::OutputStreamHandle,
        sender: &ComponentSender<ExerciseTimer>,
    ) -> Self {
        Self {
            state: if warmup_s > 0 {
                ExerciseState::Warmup
            } else {
                ExerciseState::Exercise
            },
            warmup_s,
            remaining_sets: exercise.sets,
            remaining_s: if warmup_s > 0 {
                warmup_s
            } else {
                exercise.exercise_s
            },
            running: true,
            timer: build_timer(sender),
            setup: exercise,
            audio_player: AudioPlayerModel::builder()
                .detach_worker(output)
                .forward(sender.input_sender(), |_msg| ExerciseTimerInput::Tick),
        }
    }

    fn reset(&mut self, sender: &ComponentSender<ExerciseTimer>) {
        self.state = if self.warmup_s > 0 {
            ExerciseState::Warmup
        } else {
            ExerciseState::Exercise
        };
        self.remaining_sets = self.setup.sets;
        self.remaining_s = if self.warmup_s > 0 {
            self.warmup_s
        } else {
            self.setup.exercise_s
        };
        self.running = true;
        self.timer = build_timer(sender);
    }
}

#[derive(Debug)]
pub enum ExerciseTimerInput {
    Tick,
    StartStop,
    Pause,
    Reset,
}

fn build_timer(sender: &ComponentSender<ExerciseTimer>) -> Option<WorkerController<TimerModel>> {
    Some(
        TimerModel::builder()
            .detach_worker(())
            .forward(sender.input_sender(), |timer_output| match timer_output {
                TimerOutput::Tick => ExerciseTimerInput::Tick,
            }),
    )
}

fn remaining_str(remaining_s: usize) -> String {
    if remaining_s == 0 {
        String::from("Finished")
    } else {
        format!("{}", remaining_s)
    }
}

pub struct ExerciseTimerInit {
    pub setup: ExerciseSetup,
    pub warmup_s: usize,
    pub output_handle: rodio::OutputStreamHandle,
}

#[relm4::component(pub)]
impl Component for ExerciseTimer {
    type Init = ExerciseTimerInit;
    type Input = ExerciseTimerInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[name = "root_clamp"]
        adw::Clamp {
            set_orientation: gtk::Orientation::Horizontal,
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_valign: gtk::Align::Center,
                gtk::Box {
                    set_class_active: ("timer", true),
                    set_class_active: ("card", true),
                    #[watch]
                    set_class_active: ("timer-warmup", model.state == ExerciseState::Warmup),
                    #[watch]
                    set_class_active: ("timer-exercise", model.state == ExerciseState::Exercise),
                    #[watch]
                    set_class_active: ("timer-rest", model.state == ExerciseState::Rest),
                    set_spacing: 5,
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,
                    set_margin_all: 20,
                    set_vexpand: true,
                    gtk::Label {
                        set_class_active: ("title-2", true),
                        #[watch]
                        set_label: match model.state {
                            ExerciseState::Warmup => "Warm up",
                            ExerciseState::Exercise => "Exercise",
                            ExerciseState::Rest => "Rest",
                        },
                    },
                    gtk::Label {
                        set_class_active: ("title-1", true),
                        #[watch]
                        set_label: &remaining_str(model.remaining_s),
                    },
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_halign: gtk::Align::Center,
                        set_class_active: ("linked", true),
                        gtk::Button {
                            #[watch]
                            set_label: if model.running { "Pause" } else { "Resume" },
                            #[watch]
                            set_sensitive: model.remaining_s != 0,
                            connect_clicked => ExerciseTimerInput::StartStop,
                        },
                        gtk::Button {
                            set_label: "Restart",
                            connect_clicked => ExerciseTimerInput::Reset,
                            #[watch]
                            set_class_active: ("suggested-action", model.remaining_s == 0),
                        }
                    }
                },
                gtk::Label {
                    #[watch]
                    set_label: &format!("Remaining sets: {}", model.remaining_sets),
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ExerciseTimer::new(init.setup, init.warmup_s, init.output_handle, &sender);
        let widgets = view_output!();
        model.audio_player.emit(AudioPlayerInput::NextWarmup);
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            ExerciseTimerInput::StartStop => {
                if self.running {
                    self.timer = None;
                } else {
                    self.timer = build_timer(&sender);
                }
                self.running = !self.running;
            }
            ExerciseTimerInput::Pause => {
                self.timer = None;
                self.running = false;
            }
            ExerciseTimerInput::Tick => {
                assert!(self.running);
                self.remaining_s -= 1;
                if self.remaining_s == 0 {
                    match self.state {
                        ExerciseState::Warmup => {
                            self.state = ExerciseState::Exercise;
                            self.remaining_s = self.setup.exercise_s;
                            self.audio_player.emit(AudioPlayerInput::NextExercise);
                        }
                        ExerciseState::Exercise => {
                            self.remaining_sets -= 1;
                            if self.remaining_sets == 0 {
                                sender
                                    .input_sender()
                                    .send(ExerciseTimerInput::StartStop)
                                    .unwrap();
                                self.audio_player.emit(AudioPlayerInput::Finished);
                            } else {
                                self.state = ExerciseState::Rest;
                                self.remaining_s = self.setup.rest_s;
                                self.audio_player.emit(AudioPlayerInput::NextRest);
                            }
                        }
                        ExerciseState::Rest => {
                            self.state = ExerciseState::Exercise;
                            self.remaining_s = self.setup.exercise_s;
                            self.audio_player.emit(AudioPlayerInput::NextExercise);
                        }
                    }
                } else if self.remaining_s <= 5 {
                    self.audio_player.emit(AudioPlayerInput::Ping);
                }
            }
            ExerciseTimerInput::Reset => {
                self.reset(&sender);
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        if let Some(parent) = widgets.root_clamp.parent() {
            parent
                .downcast::<adw::ToolbarView>()
                .unwrap()
                .remove(&widgets.root_clamp);
        }
    }
}
