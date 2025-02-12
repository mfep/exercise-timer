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
    Preparation,
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
        setup: TrainingSetup,
        global_setup: GlobalTrainingSetup,
        output: rodio::OutputStreamHandle,
        sender: &ComponentSender<TrainingTimer>,
    ) -> Self {
        let beep_volume = global_setup.beep_volume.get();
        Self {
            state: if setup.prepare_s > 0 {
                TrainingState::Preparation
            } else {
                TrainingState::Exercise
            },
            global_setup,
            remaining_sets: setup.sets,
            remaining_s: if setup.prepare_s > 0 {
                setup.prepare_s
            } else {
                setup.exercise_s
            },
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
        self.state = if self.setup.prepare_s > 0 {
            TrainingState::Preparation
        } else {
            TrainingState::Exercise
        };
        self.remaining_sets = self.setup.sets;
        self.remaining_s = if self.setup.prepare_s > 0 {
            self.setup.prepare_s
        } else {
            self.setup.exercise_s
        };
        self.running = true;
        self.timer = build_timer(sender);
    }

    fn is_finished(&self) -> bool {
        self.remaining_s == 0
    }

    fn remaining_str_mins(&self) -> String {
        if self.remaining_s == 0 {
            String::from("")
        } else {
            format!("{:02}", self.remaining_s / 60)
        }
    }

    fn remaining_str_colon(&self) -> String {
        if self.is_finished() {
            // Translators: Shown in the timer page when the training has come to the end
            gettext("Finished!")
        } else {
            String::from("∶")
        }
    }

    fn remaining_str_secs(&self) -> String {
        if self.is_finished() {
            String::from("")
        } else {
            format!("{:02}", self.remaining_s % 60)
        }
    }

    fn width_chars(&self, default: i32) -> i32 {
        if self.is_finished() {
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
                adw::Clamp {
                    set_orientation: gtk::Orientation::Vertical,
                    set_maximum_size: 250,
                    gtk::Box {
                        add_css_class: "card",
                        #[watch]
                        set_class_active: ("timer-warmup", model.state == TrainingState::Preparation),
                        #[watch]
                        set_class_active: ("timer-exercise", model.state == TrainingState::Exercise),
                        #[watch]
                        set_class_active: ("timer-rest", model.state == TrainingState::Rest),
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
                                    TrainingState::Preparation => gettext("Preparation"),
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
                                    set_class_active: ("huge-button", model.is_finished()),
                                    // Translators: tooltip text for the reset button
                                    set_tooltip: &gettext("Restart Training"),
                                },
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
                                    set_visible: !model.is_finished(),
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
                                    set_visible: !model.is_finished(),
                                }
                            }
                        },
                    },
                },
                gtk::Label {
                    #[watch]
                    set_label: &if model.is_finished() {
                        String::default()
                    } else if false {
                        // Translators: Label showing the number of remaining sets on the timer page
                        gettext("Remaining Sets: {}")
                    } else {
                        gettext!("Remaining Sets: {}", model.remaining_sets)
                    },
                    set_margin_bottom: 12,
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
                        TrainingState::Preparation => {
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
