mod audio_player;
mod timer;

use gettextrs::gettext;
use relm4::{
    adw,
    binding::*,
    gtk::{self, prelude::*},
    prelude::*,
    RelmObjectExt, RelmWidgetExt,
};
use relm4_icons::icon_names;
use timer::{TimerModel, TimerOutput};

use crate::{settings::GlobalTrainingSetup, training_setup::TrainingSetup};
use audio_player::{AudioPlayerInput, AudioPlayerModel};

use self::audio_player::AudioPlayerModelInit;

#[derive(PartialEq)]
enum TrainingState {
    Warmup,
    Exercise,
    Rest,
}

pub struct TrainingTimer {
    setup: TrainingSetup,
    global_setup: GlobalTrainingSetup,
    state: TrainingState,
    remaining_sets: usize,
    remaining_s: usize,
    running: bool,
    timer: Option<relm4::WorkerController<TimerModel>>,
    audio_player: relm4::WorkerController<AudioPlayerModel>,
}

impl TrainingTimer {
    fn new(
        exercise: TrainingSetup,
        global_setup: GlobalTrainingSetup,
        output: rodio::OutputStreamHandle,
        sender: &ComponentSender<TrainingTimer>,
    ) -> Self {
        let warmup_s = global_setup.warmup_s.get() as usize;
        let beep_volume = global_setup.beep_volume.get();
        Self {
            state: if warmup_s > 0 {
                TrainingState::Warmup
            } else {
                TrainingState::Exercise
            },
            global_setup,
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
                .detach_worker(AudioPlayerModelInit {
                    output_stream: output,
                    volume: beep_volume,
                })
                .forward(sender.input_sender(), |_msg| TrainingTimerInput::Tick),
        }
    }

    fn reset(&mut self, sender: &ComponentSender<TrainingTimer>) {
        let warmup_s = self.global_setup.warmup_s.get() as usize;
        self.state = if warmup_s > 0 {
            TrainingState::Warmup
        } else {
            TrainingState::Exercise
        };
        self.remaining_sets = self.setup.sets;
        self.remaining_s = if warmup_s > 0 {
            warmup_s
        } else {
            self.setup.exercise_s
        };
        self.running = true;
        self.timer = build_timer(sender);
    }
}

#[derive(Debug)]
pub enum TrainingTimerInput {
    Tick,
    StartStop,
    Pause,
    Reset,
}

fn build_timer(
    sender: &ComponentSender<TrainingTimer>,
) -> Option<relm4::WorkerController<TimerModel>> {
    Some(
        TimerModel::builder()
            .detach_worker(())
            .forward(sender.input_sender(), |timer_output| match timer_output {
                TimerOutput::Tick => TrainingTimerInput::Tick,
            }),
    )
}

fn remaining_str_mins(remaining_s: usize) -> String {
    if remaining_s == 0 {
        String::from("")
    } else {
        format!("{:02}", remaining_s / 60)
    }
}

fn remaining_str_colon(remaining_s: usize) -> String {
    if remaining_s == 0 {
        // Translators: Shown in the timer page when the training has come to the end
        gettext("Finished!")
    } else {
        String::from(":")
    }
}

fn remaining_str_secs(remaining_s: usize) -> String {
    if remaining_s == 0 {
        String::from("")
    } else {
        format!("{:02}", remaining_s % 60)
    }
}

fn width_chars(remaining_s: usize, default: i32) -> i32 {
    if remaining_s == 0 {
        -1
    } else {
        default
    }
}

pub struct TrainingTimerInit {
    pub setup: TrainingSetup,
    pub global_setup: GlobalTrainingSetup,
    pub output_handle: rodio::OutputStreamHandle,
}

#[relm4::component(pub)]
impl Component for TrainingTimer {
    type Init = TrainingTimerInit;
    type Input = TrainingTimerInput;
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
                    add_css_class: "timer",
                    add_css_class: "card",
                    #[watch]
                    set_class_active: ("timer-warmup", model.state == TrainingState::Warmup),
                    #[watch]
                    set_class_active: ("timer-exercise", model.state == TrainingState::Exercise),
                    #[watch]
                    set_class_active: ("timer-rest", model.state == TrainingState::Rest),
                    set_spacing: 5,
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,
                    set_margin_all: 20,
                    set_vexpand: true,
                    gtk::Label {
                        add_css_class: "timer-title",
                        #[watch]
                        set_label: &match model.state {
                            // Translators: Shown on the timer page during preparation
                            TrainingState::Warmup => gettext("Preparation"),
                            // Translators: Shown on the timer page during exercise
                            TrainingState::Exercise => gettext("Exercise"),
                            // Translators: Shown on the timer page during rest
                            TrainingState::Rest => gettext("Rest"),
                        },
                    },
                    gtk::Box {
                        add_css_class: "timer-label",
                        set_orientation: gtk::Orientation::Horizontal,
                        set_halign: gtk::Align::Center,
                        gtk::Label {
                            #[watch]
                            set_width_chars: width_chars(model.remaining_s, 2),
                            set_xalign: 1.0,
                            #[watch]
                            set_label: &remaining_str_mins(model.remaining_s),
                        },
                        gtk::Label {
                            #[watch]
                            set_width_chars: width_chars(model.remaining_s, 1),
                            #[watch]
                            set_label: &remaining_str_colon(model.remaining_s),
                        },
                        gtk::Label {
                            #[watch]
                            set_width_chars: width_chars(model.remaining_s, 2),
                            set_xalign: 0.0,
                            #[watch]
                            set_label: &remaining_str_secs(model.remaining_s),
                        },
                    },
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_halign: gtk::Align::Center,
                        set_spacing: 12,
                        gtk::Button {
                            set_css_classes: &["circular", "large-button"],
                            set_icon_name: icon_names::REFRESH,
                            set_valign: gtk::Align::Center,
                            connect_clicked => TrainingTimerInput::Reset,
                            #[watch]
                            set_class_active: ("suggested-action", model.remaining_s == 0),
                        },
                        gtk::Button {
                            set_css_classes: &["circular", "huge-button"],
                            #[watch]
                            set_sensitive: model.remaining_s != 0,
                            connect_clicked => TrainingTimerInput::StartStop,
                            gtk::Image {
                                #[watch]
                                set_icon_name: Some(if model.running { icon_names::PAUSE } else { icon_names::PLAY }),
                            },
                        },
                        #[name = "volume_button"]
                        gtk::ScaleButton {
                            set_valign: gtk::Align::Center,
                            set_icons: &["audio-volume-muted-symbolic", "audio-volume-high-symbolic", "audio-volume-medium-symbolic"],
                            set_adjustment = &gtk::Adjustment {
                                set_lower: 0f64,
                                set_upper: 1f64,
                                add_binding: (&model.global_setup.beep_volume, "value"),
                                connect_value_changed[audio_sender] => move |adj| {
                                    audio_sender.emit(AudioPlayerInput::SetVolume(adj.value()))
                                },
                            }
                        }
                    }
                },
                gtk::Label {
                    #[watch]
                    set_label: &if false {
                        // Translators: Label showing the number of remaining sets on the timer page
                        gettext("Remaining sets: {}")
                    } else {
                        gettext!("Remaining sets: {}", model.remaining_sets)
                    }
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = TrainingTimer::new(init.setup, init.global_setup, init.output_handle, &sender);
        let audio_sender = model.audio_player.sender();
        let widgets = view_output!();
        widgets
            .volume_button
            .first_child()
            .unwrap()
            .set_css_classes(&["circular", "toggle", "large-button"]);
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
            TrainingTimerInput::StartStop => {
                if self.remaining_s == 0 && self.remaining_sets == 0 {
                    return;
                } else if self.running {
                    self.timer = None;
                } else {
                    self.timer = build_timer(&sender);
                }
                self.running = !self.running;
            }
            TrainingTimerInput::Pause => {
                self.timer = None;
                self.running = false;
            }
            TrainingTimerInput::Tick => {
                assert!(self.running);
                self.remaining_s -= 1;
                if self.remaining_s == 0 {
                    match self.state {
                        TrainingState::Warmup => {
                            self.state = TrainingState::Exercise;
                            self.remaining_s = self.setup.exercise_s;
                            self.audio_player.emit(AudioPlayerInput::NextExercise);
                        }
                        TrainingState::Exercise => {
                            self.remaining_sets -= 1;
                            if self.remaining_sets == 0 {
                                self.timer = None;
                                self.running = false;
                                self.audio_player.emit(AudioPlayerInput::Finished);
                            } else {
                                self.state = TrainingState::Rest;
                                self.remaining_s = self.setup.rest_s;
                                self.audio_player.emit(AudioPlayerInput::NextRest);
                            }
                        }
                        TrainingState::Rest => {
                            self.state = TrainingState::Exercise;
                            self.remaining_s = self.setup.exercise_s;
                            self.audio_player.emit(AudioPlayerInput::NextExercise);
                        }
                    }
                } else if self.remaining_s <= 5 {
                    self.audio_player.emit(AudioPlayerInput::Ping);
                }
            }
            TrainingTimerInput::Reset => {
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
