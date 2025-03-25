mod audio_player;
mod timer;

use gettextrs::gettext;
use relm4::{adw::prelude::*, binding::*, gtk, prelude::*, RelmObjectExt, RelmWidgetExt};
use relm4_icons::icon_names;
use timer::{TimerModel, TimerOutput};

use crate::{settings::GlobalTrainingSetup, training_setup::TrainingSetup};
use audio_player::{AudioPlayerInput, AudioPlayerModel};

use self::audio_player::AudioPlayerModelInit;

#[derive(Clone)]
enum TrainingState {
    Preparation {
        remaining_s: usize,
    },
    Exercise {
        remaining_s: usize,
        remaining_sets: usize,
    },
    Rest {
        remaining_s: usize,
        remaining_sets: usize,
    },
    Finished,
}

impl TrainingState {
    fn new(setup: &TrainingSetup) -> Self {
        if setup.prepare_s > 0 {
            Self::Preparation {
                remaining_s: setup.prepare_s,
            }
        } else {
            Self::Exercise {
                remaining_s: setup.exercise_s,
                remaining_sets: setup.sets,
            }
        }
    }

    fn tick(self, setup: &TrainingSetup) -> Self {
        match self {
            Self::Preparation { remaining_s } => {
                if remaining_s > 1 {
                    Self::Preparation {
                        remaining_s: remaining_s - 1,
                    }
                } else {
                    Self::Exercise {
                        remaining_s: setup.exercise_s,
                        remaining_sets: setup.sets,
                    }
                }
            }
            Self::Exercise {
                remaining_s,
                remaining_sets,
            } => {
                if remaining_s > 1 {
                    Self::Exercise {
                        remaining_s: remaining_s - 1,
                        remaining_sets,
                    }
                } else if remaining_sets == 1 {
                    Self::Finished
                } else if setup.rest_s == 0 {
                    Self::Exercise {
                        remaining_s: setup.exercise_s,
                        remaining_sets: remaining_sets - 1,
                    }
                } else {
                    Self::Rest {
                        remaining_s: setup.rest_s,
                        remaining_sets,
                    }
                }
            }
            Self::Rest {
                remaining_s,
                remaining_sets,
            } => {
                if remaining_s > 1 {
                    Self::Rest {
                        remaining_s: remaining_s - 1,
                        remaining_sets,
                    }
                } else {
                    Self::Exercise {
                        remaining_s: setup.exercise_s,
                        remaining_sets: remaining_sets - 1,
                    }
                }
            }
            Self::Finished => panic!("Finished state cannot be ticked"),
        }
    }
}

pub struct TrainingTimer {
    setup: TrainingSetup,
    global_setup: GlobalTrainingSetup,
    state: TrainingState,
    running: bool,
    timer: Option<relm4::WorkerController<TimerModel>>,
    audio_player: relm4::WorkerController<AudioPlayerModel>,
}

impl TrainingTimer {
    fn new(
        setup: TrainingSetup,
        global_setup: GlobalTrainingSetup,
        output: rodio::OutputStreamHandle,
        sender: &ComponentSender<TrainingTimer>,
    ) -> Self {
        let beep_volume = global_setup.beep_volume.get();
        Self {
            state: TrainingState::new(&setup),
            global_setup,
            running: true,
            timer: build_timer(sender),
            setup,
            audio_player: AudioPlayerModel::builder()
                .detach_worker(AudioPlayerModelInit {
                    output_stream: output,
                    volume: beep_volume,
                })
                .forward(sender.input_sender(), |_msg| TrainingTimerInput::Tick),
        }
    }

    fn reset(&mut self, sender: &ComponentSender<TrainingTimer>) {
        self.state = TrainingState::new(&self.setup);
        self.running = true;
        self.timer = build_timer(sender);
    }

    fn remaining_str_mins(&self) -> String {
        match self.state {
            TrainingState::Finished => String::new(),
            TrainingState::Exercise { remaining_s, .. }
            | TrainingState::Preparation { remaining_s }
            | TrainingState::Rest { remaining_s, .. } => format!("{:02}", remaining_s / 60),
        }
    }

    fn remaining_str_colon(&self) -> String {
        if matches!(self.state, TrainingState::Finished) {
            // Translators: Shown in the timer page when the training has come to the end
            gettext("Finished!")
        } else {
            String::from("∶")
        }
    }

    fn remaining_str_secs(&self) -> String {
        match self.state {
            TrainingState::Finished => String::new(),
            TrainingState::Exercise { remaining_s, .. }
            | TrainingState::Preparation { remaining_s }
            | TrainingState::Rest { remaining_s, .. } => format!("{:02}", remaining_s % 60),
        }
    }

    fn width_chars(&self, default: i32) -> i32 {
        if matches!(self.state, TrainingState::Finished) {
            -1
        } else {
            default
        }
    }
}

#[derive(Debug)]
pub enum TrainingTimerInput {
    Tick,
    StartStop,
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
        adw::NavigationPage {
            set_title: &model.setup.name,
            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},
                adw::Clamp {
                    set_orientation: gtk::Orientation::Horizontal,
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_valign: gtk::Align::Center,
                        adw::Clamp {
                            set_orientation: gtk::Orientation::Vertical,
                            set_maximum_size: 250,
                            gtk::Box {
                                add_css_class: "card",
                                #[watch]
                                set_class_active: ("timer-warmup", matches!(model.state, TrainingState::Preparation{ .. })),
                                #[watch]
                                set_class_active: ("timer-exercise", matches!(model.state, TrainingState::Exercise{ .. } | TrainingState::Finished)),
                                #[watch]
                                set_class_active: ("timer-rest", matches!(model.state, TrainingState::Rest{ .. })),
                                set_spacing: 5,
                                set_orientation: gtk::Orientation::Vertical,
                                set_valign: gtk::Align::Fill,
                                set_margin_start: 12,
                                set_margin_end: 12,
                                set_margin_bottom: 12,
                                set_vexpand: true,
                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_valign: gtk::Align::Center,
                                    set_vexpand: true,
                                    gtk::Label {
                                        add_css_class: "timer-title",
                                        #[watch]
                                        set_label: &match model.state {
                                            // Translators: Shown on the timer page during preparation
                                            TrainingState::Preparation{ .. } => gettext("Preparation"),
                                            // Translators: Shown on the timer page during exercise
                                            TrainingState::Exercise{ .. } => gettext("Exercise"),
                                            // Translators: Shown on the timer page during rest
                                            TrainingState::Rest{ .. } => gettext("Rest"),
                                            TrainingState::Finished => String::new(),
                                        },
                                    },
                                    gtk::Box {
                                        add_css_class: "timer-label",
                                        set_orientation: gtk::Orientation::Horizontal,
                                        set_halign: gtk::Align::Center,
                                        set_direction: gtk::TextDirection::Ltr,
                                        gtk::Label {
                                            #[watch]
                                            set_width_chars: model.width_chars(2),
                                            set_xalign: 1.0,
                                            #[watch]
                                            set_label: &model.remaining_str_mins(),
                                        },
                                        gtk::Label {
                                            #[watch]
                                            set_width_chars: model.width_chars(1),
                                            #[watch]
                                            set_label: &model.remaining_str_colon(),
                                        },
                                        gtk::Label {
                                            #[watch]
                                            set_width_chars: model.width_chars(2),
                                            set_xalign: 0.0,
                                            #[watch]
                                            set_label: &model.remaining_str_secs(),
                                        },
                                    },
                                    #[name = "button_box"]
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
                                            set_class_active: ("huge-button", matches!(model.state, TrainingState::Finished)),
                                            // Translators: tooltip text for the reset button
                                            set_tooltip: &gettext("Restart Training"),
                                        },
                                        #[name = "play_pause_button"]
                                        gtk::Button {
                                            set_css_classes: &["circular", "huge-button"],
                                            connect_clicked => TrainingTimerInput::StartStop,
                                            gtk::Image {
                                                #[watch]
                                                set_icon_name: Some(if model.running { icon_names::PAUSE } else { icon_names::PLAY }),
                                            },
                                            #[watch]
                                            // Translators: tooltip text for the pause/resume button
                                            set_tooltip: &if model.running { gettext("Pause Training") } else { gettext("Resume Training") },
                                            #[watch]
                                            set_visible: !matches!(model.state, TrainingState::Finished),
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
                                            },
                                            // Translators: tooltip text for the volume button
                                            set_tooltip: &gettext("Set Volume"),
                                            #[watch]
                                            set_visible: !matches!(model.state, TrainingState::Finished),
                                        }
                                    }
                                },
                            },
                        },
                        gtk::Label {
                            #[watch]
                            set_label: &if matches!(model.state, TrainingState::Finished) {
                                String::default()
                            } else if false {
                                // Translators: Label showing the number of remaining sets on the timer page
                                gettext("Remaining Sets: {}")
                            } else {
                                gettext!("Remaining Sets: {}", match model.state {
                                    TrainingState::Preparation { .. } => model.setup.sets,
                                    TrainingState::Exercise { remaining_sets, .. }
                                    | TrainingState::Rest { remaining_sets, .. } => remaining_sets,
                                    TrainingState::Finished => 0,
                                })
                            },
                            set_margin_bottom: 12,
                        },
                    },
                }
            }
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
                if self.running {
                    self.timer = None;
                } else {
                    self.timer = build_timer(&sender);
                }
                self.running = !self.running;
            }
            TrainingTimerInput::Tick => {
                assert!(self.running);
                self.state = self.state.clone().tick(&self.setup);
                const SOUND_THRESHOLD: usize = 5;
                match self.state {
                    TrainingState::Preparation { remaining_s } => {
                        if remaining_s <= SOUND_THRESHOLD {
                            self.audio_player.emit(AudioPlayerInput::Ping);
                        }
                    }
                    TrainingState::Exercise { remaining_s, .. } => {
                        if remaining_s == self.setup.exercise_s {
                            self.audio_player.emit(AudioPlayerInput::NextExercise);
                        } else if remaining_s <= SOUND_THRESHOLD {
                            self.audio_player.emit(AudioPlayerInput::Ping);
                        }
                    }
                    TrainingState::Rest { remaining_s, .. } => {
                        if remaining_s == self.setup.rest_s {
                            self.audio_player.emit(AudioPlayerInput::NextRest);
                        } else if remaining_s <= SOUND_THRESHOLD {
                            self.audio_player.emit(AudioPlayerInput::Ping);
                        }
                    }
                    TrainingState::Finished => {
                        self.audio_player.emit(AudioPlayerInput::Finished);
                        self.running = false;
                        self.timer = None;
                    }
                }
            }
            TrainingTimerInput::Reset => {
                self.reset(&sender);
            }
        }
    }
}
