mod exercise_timer;

pub use exercise_timer::ExerciseSetup;
use exercise_timer::ExerciseTimer;
use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::factory::FactoryVecDeque;
use relm4::prelude::{DynamicIndex, FactoryComponent};
use relm4::Controller;
use relm4::{
    adw,
    gtk::{self, prelude::ObjectExt},
    Component, ComponentController, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt,
    SimpleComponent,
};

#[derive(Debug)]
pub enum ExerciseSetupInput {
    Remove,
}

#[derive(Debug)]
pub enum ExerciseSetupOutput {
    Remove(DynamicIndex),
}

#[derive(Debug)]
pub enum AppModelInput {
    PromptNewExercise,
    RemoveExerciseSetup(DynamicIndex),
    None,
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
            gtk::Label {
                set_class_active: ("title-4", true),
                #[watch]
                set_label: &self.name,
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
                    connect_clicked[sender, index] => move |_| {
                        sender.output(ExerciseSetupOutput::Remove(index.clone()))
                    },
                },
                gtk::Button {
                    set_class_active: ("suggested-action", true),
                    set_icon_name: "play",
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
        })
    }
}

struct AppModel {
    exerciser: Controller<ExerciseTimer>,
    list_exercises: FactoryVecDeque<ExerciseSetup>,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppModelInput;
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
                        pack_start = &gtk::Button {
                            set_icon_name: "plus",
                            connect_clicked => AppModelInput::PromptNewExercise,
                        },
                    },
                    gtk::ScrolledWindow {
                        set_vexpand: true,
                        #[local_ref]
                        list_exercises -> gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                        }
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
        let list_exercises = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        let model = AppModel {
            exerciser: ExerciseTimer::builder()
                .launch(())
                .forward(sender.input_sender(), |_| AppModelInput::None),
            list_exercises,
        };
        let exerciser = model.exerciser.widget();
        let list_exercises = model.list_exercises.widget();
        let widgets = view_output!();
        widgets
            .leaflet
            .bind_property("folded", &widgets.left_header, "show_end_title_buttons")
            .sync_create()
            .build();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppModelInput::PromptNewExercise => {
                self.list_exercises
                    .guard()
                    .push_front(ExerciseSetup::default());
            }
            AppModelInput::RemoveExerciseSetup(index) => {
                let index = index.current_index();
                self.list_exercises.guard().remove(index);
            }
            AppModelInput::None => {}
        }
    }
}

fn main() {
    let app = RelmApp::new("org.safeworlds.hiit");
    relm4_icons::initialize_icons();
    app.run::<AppModel>(());
}
