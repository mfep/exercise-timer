mod audio_player;
mod exercise_editor;
mod exercise_setup;
mod exercise_timer;

use exercise_editor::{ExerciseEditor, ExerciseEditorOutput, ExerciseEditorRole};
use exercise_setup::ExerciseSetup;
use exercise_timer::{ExerciseTimer, ExerciseTimerInput};
use futures::StreamExt;
use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::factory::FactoryVecDeque;
use relm4::prelude::DynamicIndex;
use relm4::{
    adw,
    gtk::{self, gio, prelude::*},
    Component, ComponentController, ComponentParts, ComponentSender, RelmApp,
};
use relm4::{Controller, WidgetRef};
use rodio;

#[derive(Debug)]
pub enum AppModelInput {
    PromptNewExercise,
    CreateExerciseSetup(ExerciseSetup),
    RemoveExerciseSetup(DynamicIndex),
    LoadExercise(ExerciseSetup),
    None,
}

struct AppModel {
    exercise_timer: Option<Controller<ExerciseTimer>>,
    list_exercises: FactoryVecDeque<ExerciseSetup>,
    output_stream: rodio::OutputStreamHandle,
}

#[relm4::component(pub)]
impl Component for AppModel {
    type Init = rodio::OutputStreamHandle;
    type Input = AppModelInput;
    type Output = ();
    type CommandOutput = ();

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
                #[name = "right_leaflet"]
                append = &gtk::Box {
                    set_hexpand: true,
                    set_orientation: gtk::Orientation::Vertical,
                    adw::HeaderBar {
                        set_title_widget: Some(&adw::WindowTitle::new("Main Title", "Main Subtitle")),
                    },
                    #[name = "status_page"]
                    adw::StatusPage {
                        set_title: "No exercises created yet",
                        set_icon_name: Some("weight2"),
                    }
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let list_exercises = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        let model = AppModel {
            exercise_timer: None,
            list_exercises,
            output_stream: init,
        };
        let list_exercises = model.list_exercises.widget();
        let widgets = view_output!();
        widgets
            .leaflet
            .bind_property("folded", &widgets.left_header, "show_end_title_buttons")
            .sync_create()
            .build();
        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            AppModelInput::PromptNewExercise => {
                if let Some(timer) = self.exercise_timer.as_ref() {
                    timer.sender().emit(ExerciseTimerInput::Pause);
                }
                let mut editor = ExerciseEditor::builder()
                    .transient_for(root.widget_ref())
                    .launch((ExerciseEditorRole::New, ExerciseSetup::default()))
                    .into_stream();
                relm4::spawn_local(async move {
                    if let Some(ExerciseEditorOutput::Create(setup)) = editor.next().await.unwrap()
                    {
                        sender.input(AppModelInput::CreateExerciseSetup(setup));
                    }
                });
            }
            AppModelInput::RemoveExerciseSetup(index) => {
                let index = index.current_index();
                self.list_exercises.guard().remove(index);
            }
            AppModelInput::CreateExerciseSetup(setup) => {
                println!("Exercise created: {:?}", setup);
                self.list_exercises.guard().push_back(setup);
            }
            AppModelInput::LoadExercise(setup) => {
                self.exercise_timer = Some(
                    ExerciseTimer::builder()
                        .launch((setup, self.output_stream.clone()))
                        .forward(sender.input_sender(), |_msg| AppModelInput::None),
                );
                widgets.status_page.set_visible(false);
                widgets
                    .right_leaflet
                    .append(self.exercise_timer.as_ref().unwrap().widget());
            }
            AppModelInput::None => {}
        }
    }
}

fn main() {
    let (_stream, stream_handle) =
        rodio::OutputStream::try_default().expect("Could not create audio output stream");
    gio::resources_register_include!("hiit.gresource").expect("Could not register resources");
    let app = RelmApp::new("org.safeworlds.hiit");
    relm4_icons::initialize_icons();
    app.run::<AppModel>(stream_handle);
}
