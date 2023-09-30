mod timer;

use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::{
    adw,
    gtk::{self},
    Component, ComponentParts, ComponentSender, RelmWidgetExt, WorkerController,
};
use timer::{TimerModel, TimerOutput};

#[derive(Debug)]
pub struct ExerciseSetup {
    pub name: String,
    warmup_s: usize,
    exercise_s: usize,
    rest_s: usize,
    sets: usize,
}

#[derive(PartialEq)]
enum ExerciseState {
    Warmup,
    Exercise,
    Rest,
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

pub struct ExerciseTimer {
    setup: ExerciseSetup,
    state: ExerciseState,
    remaining_sets: usize,
    remaining_s: usize,
    running: bool,
    timer: Option<WorkerController<TimerModel>>,
}

impl ExerciseTimer {
    fn new(exercise: ExerciseSetup, sender: &ComponentSender<ExerciseTimer>) -> Self {
        Self {
            state: ExerciseState::Warmup,
            remaining_sets: exercise.sets,
            remaining_s: exercise.warmup_s,
            running: true,
            timer: build_timer(sender),
            setup: exercise,
        }
    }

    fn reset(&mut self, sender: &ComponentSender<ExerciseTimer>) {
        self.state = ExerciseState::Warmup;
        self.remaining_sets = self.setup.sets;
        self.remaining_s = self.setup.warmup_s;
        self.running = true;
        self.timer = build_timer(sender);
    }
}

#[derive(Debug)]
pub enum ExerciseTimerInput {
    Tick,
    StartStop,
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

#[relm4::component(pub)]
impl Component for ExerciseTimer {
    type Init = ();
    type Input = ExerciseTimerInput;
    type Output = ();
    type CommandOutput = ();

    view! {
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
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        relm4::set_global_css(
            ".timer {
                padding: 20px;
            }
            .timer-warmup {
                background: @warning_bg_color;
                color: @warning_fg_color;
            }
            .timer-exercise {
                background: @success_bg_color;
                color: @success_fg_color;
            }
            .timer-rest {
                background: @accent_bg_color;
                color: @accent_fg_color;
            }
            .timer-label {
                font-size: 48px;
            }
            ",
        );
        let model = ExerciseTimer::new(ExerciseSetup::default(), &sender);
        let widgets = view_output!();
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
            ExerciseTimerInput::Tick => {
                assert!(self.running);
                self.remaining_s -= 1;
                if self.remaining_s == 0 {
                    match self.state {
                        ExerciseState::Warmup => {
                            self.state = ExerciseState::Exercise;
                            self.remaining_s = self.setup.exercise_s;
                        }
                        ExerciseState::Exercise => {
                            self.remaining_sets -= 1;
                            if self.remaining_sets == 0 {
                                sender
                                    .input_sender()
                                    .send(ExerciseTimerInput::StartStop)
                                    .unwrap();
                            } else {
                                self.state = ExerciseState::Rest;
                                self.remaining_s = self.setup.rest_s;
                            }
                        }
                        ExerciseState::Rest => {
                            self.state = ExerciseState::Exercise;
                            self.remaining_s = self.setup.exercise_s;
                        }
                    }
                }
            }
            ExerciseTimerInput::Reset => {
                self.reset(&sender);
            }
        }
    }
}
