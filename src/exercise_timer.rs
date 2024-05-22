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

use crate::{exercise_setup::ExerciseSetup, settings::GlobalExerciseSetup};
use audio_player::{AudioPlayerInput, AudioPlayerModel};

use self::audio_player::AudioPlayerModelInit;

#[derive(PartialEq)]
enum ExerciseState {
    Warmup,
    Exercise,
    Rest,
}

pub struct ExerciseTimer {
    setup: ExerciseSetup,
    global_setup: GlobalExerciseSetup,
    state: ExerciseState,
    remaining_sets: usize,
    remaining_s: usize,
    running: bool,
    timer: Option<relm4::WorkerController<TimerModel>>,
    audio_player: relm4::WorkerController<AudioPlayerModel>,
}

impl ExerciseTimer {
    fn new(
        exercise: ExerciseSetup,
        global_setup: GlobalExerciseSetup,
        output: rodio::OutputStreamHandle,
        sender: &ComponentSender<ExerciseTimer>,
    ) -> Self {
        let warmup_s = global_setup.warmup_s.get() as usize;
        let beep_volume = global_setup.beep_volume.get();
        Self {
            state: if warmup_s > 0 {
                ExerciseState::Warmup
            } else {
                ExerciseState::Exercise
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
                .forward(sender.input_sender(), |_msg| ExerciseTimerInput::Tick),
        }
    }

    fn reset(&mut self, sender: &ComponentSender<ExerciseTimer>) {
        let warmup_s = self.global_setup.warmup_s.get() as usize;
        self.state = if warmup_s > 0 {
            ExerciseState::Warmup
        } else {
            ExerciseState::Exercise
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
pub enum ExerciseTimerInput {
    Tick,
    StartStop,
    Pause,
    Reset,
}

fn build_timer(
    sender: &ComponentSender<ExerciseTimer>,
) -> Option<relm4::WorkerController<TimerModel>> {
    Some(
        TimerModel::builder()
            .detach_worker(())
            .forward(sender.input_sender(), |timer_output| match timer_output {
                TimerOutput::Tick => ExerciseTimerInput::Tick,
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

pub struct ExerciseTimerInit {
    pub setup: ExerciseSetup,
    pub global_setup: GlobalExerciseSetup,
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
                    add_css_class: "timer",
                    add_css_class: "card",
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
                        add_css_class: "title-2",
                        #[watch]
                        set_label: &match model.state {
                            ExerciseState::Warmup => gettext("Warm up"),
                            ExerciseState::Exercise => gettext("Exercise"),
                            ExerciseState::Rest => gettext("Rest"),
                        },
                    },
                    gtk::Box {
                        add_css_class: "timer-label",
                        set_orientation: gtk::Orientation::Horizontal,
                        set_halign: gtk::Align::Center,
                        gtk::Label {
                            set_width_chars: 5,
                            set_xalign: 1.0,
                            #[watch]
                            set_label: &remaining_str_mins(model.remaining_s),
                        },
                        gtk::Label {
                            set_width_chars: 1,
                            #[watch]
                            set_label: &remaining_str_colon(model.remaining_s),
                        },
                        gtk::Label {
                            set_width_chars: 5,
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
                            connect_clicked => ExerciseTimerInput::Reset,
                            #[watch]
                            set_class_active: ("suggested-action", model.remaining_s == 0),
                        },
                        gtk::Button {
                            set_css_classes: &["circular", "huge-button"],
                            #[watch]
                            set_sensitive: model.remaining_s != 0,
                            connect_clicked => ExerciseTimerInput::StartStop,
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
        let model = ExerciseTimer::new(init.setup, init.global_setup, init.output_handle, &sender);
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
            ExerciseTimerInput::StartStop => {
                if self.remaining_s == 0 && self.remaining_sets == 0 {
                    return;
                } else if self.running {
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
                                self.timer = None;
                                self.running = false;
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
