mod exercise_editor;
mod exercise_setup;
mod exercise_timer;

use std::rc::Rc;

use exercise_editor::{ExerciseEditor, ExerciseEditorInput, ExerciseEditorOutput, ExerciseEditorRole};
use exercise_setup::{ExerciseSetup, ExerciseSetupModel};
use exercise_timer::ExerciseTimer;
use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::factory::FactoryVecDeque;
use relm4::prelude::DynamicIndex;
use relm4::Controller;
use relm4::{
    adw,
    gtk::{self, prelude::ObjectExt},
    Component, ComponentController, ComponentParts, ComponentSender, RelmApp, SimpleComponent,
};

#[derive(Debug)]
pub enum AppModelInput {
    PromptNewExercise,
    CreateExerciseSetup(ExerciseSetup),
    RemoveExerciseSetup(DynamicIndex),
    None,
}

struct AppModel {
    exerciser: Controller<ExerciseTimer>,
    list_exercises: FactoryVecDeque<ExerciseSetupModel>,
    exercise_editor: Rc<Controller<ExerciseEditor>>,
}

#[relm4::component(pub)]
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
            exercise_editor: Rc::new(
                ExerciseEditor::builder()
                    .transient_for(root)
                    .launch(ExerciseSetup::default())
                    .forward(sender.input_sender(), |message| match message {
                        ExerciseEditorOutput::Create(setup) => {
                            AppModelInput::CreateExerciseSetup(setup)
                        }
                    }),
            ),
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
                self.exercise_editor.emit(ExerciseEditorInput::Show(ExerciseEditorRole::New));
            }
            AppModelInput::RemoveExerciseSetup(index) => {
                let index = index.current_index();
                self.list_exercises.guard().remove(index);
            }
            AppModelInput::CreateExerciseSetup(setup) => {
                println!("Exercise created: {:?}", setup);
                self.list_exercises
                    .guard()
                    .push_front((setup, self.exercise_editor.clone()));
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
