mod exercise_editor;
mod exercise_setup;
mod exercise_timer;
mod settings;

use exercise_editor::{ExerciseEditor, ExerciseEditorOutput, ExerciseEditorRole};
use exercise_setup::ExerciseSetup;
use exercise_timer::{ExerciseTimer, ExerciseTimerInit, ExerciseTimerInput};
use futures::StreamExt;
use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::factory::FactoryVecDeque;
use relm4::gtk::gdk::Display;
use relm4::gtk::CssProvider;
use relm4::prelude::DynamicIndex;
use relm4::{
    adw,
    binding::Binding,
    gtk::{self, gio, prelude::*},
    Component, ComponentController, ComponentParts, ComponentSender, RelmApp, RelmObjectExt,
};
use relm4::{Controller, WidgetRef};
use settings::{GlobalExerciseSetup, WindowGeometry};

#[derive(Debug)]
pub enum AppModelInput {
    PromptNewExercise,
    CreateExerciseSetup(ExerciseSetup),
    RemoveExerciseSetup(DynamicIndex),
    LoadExercise(ExerciseSetup),
    Back,
    None,
}

struct AppModel {
    exercise_timer: Option<Controller<ExerciseTimer>>,
    list_exercises: FactoryVecDeque<ExerciseSetup>,
    output_stream: rodio::OutputStreamHandle,
    window_geometry: WindowGeometry,
    global_settings: GlobalExerciseSetup,
}

#[relm4::component(pub)]
impl Component for AppModel {
    type Init = rodio::OutputStreamHandle;
    type Input = AppModelInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::Window {
            add_binding: (&model.window_geometry.width, "default_width"),
            add_binding: (&model.window_geometry.height, "default_height"),
            add_binding: (&model.window_geometry.is_maximized, "maximized"),
            #[name = "leaflet"]
            adw::Leaflet {
                set_can_navigate_back: true,
                #[name = "left_leaflet"]
                append = &gtk::Box {
                    set_width_request: 300,
                    set_orientation: gtk::Orientation::Vertical,
                    append: left_header = &adw::HeaderBar {
                        set_title_widget: Some(&adw::WindowTitle::new("Exercises", "")),
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
                        set_title_widget: Some(&adw::WindowTitle::new("Trainer", "")),
                        #[name = "back_btn"]
                        pack_start = &gtk::Button {
                            set_icon_name: "left",
                            connect_clicked => AppModelInput::Back,
                        }
                    },
                    #[name = "status_page"]
                    adw::StatusPage {
                        set_vexpand: true,
                        set_title: "No exercise selected",
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
        let list_exercises = FactoryVecDeque::from_iter(
            settings::load_exercise_list_from_gsettings().into_iter(),
            gtk::Box::default(),
            sender.input_sender(),
        );
        let model = AppModel {
            exercise_timer: None,
            list_exercises,
            output_stream: init,
            window_geometry: WindowGeometry::new_from_gsettings(),
            global_settings: GlobalExerciseSetup::new_from_gsettings(),
        };
        let list_exercises = model.list_exercises.widget();
        let widgets = view_output!();
        widgets
            .leaflet
            .bind_property("folded", &widgets.left_header, "show_end_title_buttons")
            .sync_create()
            .build();
        widgets
            .leaflet
            .bind_property("folded", &widgets.back_btn, "visible")
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
                        .launch(ExerciseTimerInit {
                            setup,
                            warmup_s: self.global_settings.warmup_s.get() as usize,
                            output_handle: self.output_stream.clone(),
                        })
                        .forward(sender.input_sender(), |_msg| AppModelInput::None),
                );
                widgets.status_page.set_visible(false);
                widgets
                    .right_leaflet
                    .append(self.exercise_timer.as_ref().unwrap().widget());
                widgets.leaflet.set_visible_child(&widgets.right_leaflet);
            }
            AppModelInput::Back => {
                widgets.leaflet.set_visible_child(&widgets.left_leaflet);
                widgets.status_page.set_visible(true);
                self.exercise_timer = None;
            }
            AppModelInput::None => {}
        }
    }
}

impl Drop for AppModel {
    fn drop(&mut self) {
        settings::save_exercise_list_to_gsettings(self.list_exercises.iter());
    }
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/xyz/safeworlds/hiit/style.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    let (_stream, stream_handle) =
        rodio::OutputStream::try_default().expect("Could not create audio output stream");
    gio::resources_register_include!("hiit.gresource").expect("Could not register resources");
    let app = RelmApp::new("org.safeworlds.hiit");
    relm4_icons::initialize_icons();
    load_css();
    app.run::<AppModel>(stream_handle);
}
