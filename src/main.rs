mod timer;

use std::convert::identity;

use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::gtk::gio;
use relm4::gtk::prelude::GtkWindowExt;
use relm4::Controller;
use relm4::typed_list_view::{RelmListItem, TypedListView};
use relm4::{
    adw,
    gtk::{self, prelude::ObjectExt},
    Component, ComponentController, ComponentParts, ComponentSender, RelmApp, RelmObjectExt,
    RelmWidgetExt, SimpleComponent, WorkerController,
};
use timer::{TimerModel, TimerOutput};

#[derive(Debug)]
struct ExerciseSetup {
    name: String,
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

struct ExerciseModel {
    setup: ExerciseSetup,
    state: ExerciseState,
    remaining_sets: usize,
    remaining_s: usize,
    running: bool,
    timer: Option<WorkerController<TimerModel>>,
}

impl ExerciseModel {
    fn new(exercise: ExerciseSetup, sender: &ComponentSender<ExerciseModel>) -> Self {
        Self {
            state: ExerciseState::Warmup,
            remaining_sets: exercise.sets,
            remaining_s: exercise.warmup_s,
            running: true,
            timer: build_timer(sender),
            setup: exercise,
        }
    }

    fn reset(&mut self, sender: &ComponentSender<ExerciseModel>) {
        self.state = ExerciseState::Warmup;
        self.remaining_sets = self.setup.sets;
        self.remaining_s = self.setup.warmup_s;
        self.running = true;
        self.timer = build_timer(sender);
    }
}

#[derive(Debug)]
enum AppInput {
    Tick,
    StartStop,
    Reset,
}

fn build_timer(sender: &ComponentSender<ExerciseModel>) -> Option<WorkerController<TimerModel>> {
    Some(
        TimerModel::builder()
            .detach_worker(())
            .forward(sender.input_sender(), |timer_output| match timer_output {
                TimerOutput::Tick => AppInput::Tick,
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

#[relm4::component]
impl Component for ExerciseModel {
    type Init = ();
    type Input = AppInput;
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
                            connect_clicked => AppInput::StartStop,
                        },
                        gtk::Button {
                            set_label: "Restart",
                            connect_clicked => AppInput::Reset,
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
        let model = ExerciseModel::new(ExerciseSetup::default(), &sender);
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
            AppInput::StartStop => {
                if self.running {
                    self.timer = None;
                } else {
                    self.timer = build_timer(&sender);
                }
                self.running = !self.running;
            }
            AppInput::Tick => {
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
                                sender.input_sender().send(AppInput::StartStop).unwrap();
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
            AppInput::Reset => {
                self.reset(&sender);
            }
        }
    }
}

struct AppModel {
    exerciser: Controller<ExerciseModel>,
    list_exercises: TypedListView<ExerciseSetup, gtk::NoSelection>,
}

struct ExerciseSetupWidgets {
    label: gtk::Label,
}

impl RelmListItem for ExerciseSetup {
    type Root = gtk::Box;
    type Widgets = ExerciseSetupWidgets;
    
    fn setup(_list_item: &gtk::ListItem) -> (Self::Root, Self::Widgets) {
        relm4::view! {
            #[name = "container_box"]
            gtk::Box {
                set_hexpand: true,
                set_class_active: ("card", true),
                set_margin_top: 5,
                set_margin_start: 5,
                set_margin_end: 5,
                inline_css: "padding: 10px",
                #[name = "label"]
                gtk::Label {
                    set_class_active: ("title-4", true),
                },
                gtk::Box {
                    set_class_active: ("linked", true),
                    set_hexpand: true,
                    set_halign: gtk::Align::End,
                    gtk::Button {
                        set_icon_name: "edit",
                    },
                    gtk::Button {
                        set_class_active: ("destructive-action", true),
                        set_icon_name: "entry-clear",
                    },
                    gtk::Button {
                        set_class_active: ("suggested-action", true),
                        set_icon_name: "play",
                    }
                }
            }
        }

        let widgets = ExerciseSetupWidgets {
            label,
        };

        (container_box, widgets)
    }

    fn bind(&mut self, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        widgets.label.set_label(&self.name);
    }
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::Window {
            #[name = "leaflet"]
            adw::Leaflet {
                set_can_navigate_back: true,
                append = &gtk::Box {
                    set_width_request: 300,
                    set_orientation: gtk::Orientation::Vertical,
                    append: left_header = &adw::HeaderBar {
                        set_title_widget: Some(&adw::WindowTitle::new("Test Title", "Test Subtitle")),
                    },
                    gtk::ScrolledWindow {
                        set_vexpand: true,
                        #[local_ref]
                        list_exercises -> gtk::ListView {}
                    }
                },
                append = &gtk::Separator::new(gtk::Orientation::Vertical),
                append = &gtk::Box {
                    set_hexpand: true,
                    set_orientation: gtk::Orientation::Vertical,
                    adw::HeaderBar {
                        set_title_widget: Some(&adw::WindowTitle::new("Main Title", "Main Subtitle")),
                    },
                    #[local_ref]
                    exerciser -> adw::Clamp,
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = AppModel {
            exerciser: ExerciseModel::builder()
                .launch(())
                .forward(sender.input_sender(), identity),
            list_exercises: TypedListView::default(),
        };
        for _i in 0..10 {
            model.list_exercises.append(ExerciseSetup::default());
        }
        let exerciser = model.exerciser.widget();
        let list_exercises = &model.list_exercises.view;
        let widgets = view_output!();
        widgets
            .leaflet
            .bind_property("folded", &widgets.left_header, "show_end_title_buttons")
            .sync_create()
            .build();
        ComponentParts { model, widgets }
    }
}

fn main() {
    // gio::resources_register_include!("hiit.gresource").expect("Failed to register resources.");
    let app = RelmApp::new("org.safeworlds.hiit");
    relm4_icons::initialize_icons();
    app.run::<AppModel>(());
}
