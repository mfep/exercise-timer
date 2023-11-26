use crate::config;
use crate::exercise_editor::*;
use crate::exercise_setup::*;
use crate::exercise_timer::*;
use crate::settings;
use crate::settings_dialog::*;
use crate::shortcuts_window::*;
use futures::prelude::*;
use relm4::actions::AccelsPlus;
use relm4::{
    self,
    adw::{self, prelude::*},
    gtk,
    prelude::*,
    RelmObjectExt,
};

#[derive(Debug)]
pub enum AppModelInput {
    PromptNewExercise,
    CreateExerciseSetup(ExerciseSetup),
    RemoveExerciseSetup(DynamicIndex),
    LoadExercise(ExerciseSetup),
    Popped,
    StartStop,
    Reset,
}

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
relm4::new_stateless_action!(StartStopAction, WindowActionGroup, "start-stop");
relm4::new_stateless_action!(ResetAction, WindowActionGroup, "reset");

pub struct AppModel {
    exercise_timer: Option<Controller<ExerciseTimer>>,
    list_exercises: relm4::factory::FactoryVecDeque<ExerciseSetup>,
    output_stream: rodio::OutputStreamHandle,
    window_geometry: settings::WindowGeometry,
    global_settings: settings::GlobalExerciseSetup,
    shortcuts_window: Controller<ShortcutsWindowModel>,
}

#[relm4::component(pub)]
impl Component for AppModel {
    type Init = rodio::OutputStreamHandle;
    type Input = AppModelInput;
    type Output = ();
    type CommandOutput = ();

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard Shortcuts" => ShortcutsAction,
                "_About Exercise Timer" => AboutAction,
            }
        }
    }

    view! {
        #[name = "main_window"]
        adw::ApplicationWindow {
            set_size_request: (300, 300),
            add_binding: (&model.window_geometry.width, "default_width"),
            add_binding: (&model.window_geometry.height, "default_height"),
            add_binding: (&model.window_geometry.is_maximized, "maximized"),
            #[name = "navigation_view"]
            adw::NavigationView {
                add = &adw::NavigationPage {
                    set_title: "Exercise List",
                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            pack_start = &gtk::Button {
                                set_icon_name: "plus",
                                connect_clicked => AppModelInput::PromptNewExercise,
                            },
                            pack_end = &gtk::MenuButton {
                                set_icon_name: "open-menu-symbolic",
                                set_menu_model: Some(&primary_menu),
                            },
                        },
                        #[wrap(Some)]
                        #[name = "exercise_list_stack"]
                        set_content = &gtk::Stack {
                            #[name = "exercise_list_scrolled"]
                            gtk::ScrolledWindow {
                                set_vexpand: true,
                                #[local_ref]
                                list_exercises -> gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_margin_start: 12,
                                    set_margin_end: 12,
                                    set_spacing: 8,
                                }
                            },
                            #[name = "exercise_list_status"]
                            adw::StatusPage {
                                set_icon_name: Some("weight2"),
                                set_title: "No exercise is created yet",
                                gtk::Button {
                                    set_css_classes: &["suggested-action", "pill"],
                                    set_label: "Create exercise",
                                    set_halign: gtk::Align::Center,
                                    connect_clicked => AppModelInput::PromptNewExercise,
                                }
                            },
                        },
                    },
                },
                #[name = "main_navigation_page"]
                add = &adw::NavigationPage {
                    set_title: "Timer",
                    #[wrap(Some)]
                    #[name = "main_view"]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {},
                    }
                },
                connect_popped[sender] => move |_, _| { sender.input(AppModelInput::Popped); },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let list_exercises = relm4::factory::FactoryVecDeque::from_iter(
            settings::load_exercise_list_from_gsettings().into_iter(),
            gtk::Box::default(),
            sender.input_sender(),
        );
        let model = AppModel {
            exercise_timer: None,
            list_exercises,
            output_stream: init,
            window_geometry: settings::WindowGeometry::new_from_gsettings(),
            global_settings: settings::GlobalExerciseSetup::new_from_gsettings(),
            shortcuts_window: ShortcutsWindowModel::builder()
                .transient_for(&root)
                .launch(())
                .detach(),
        };
        let mut actions = relm4::actions::RelmActionGroup::<WindowActionGroup>::new();
        let about_action = {
            let root = root.clone();
            relm4::actions::RelmAction::<AboutAction>::new_stateless(move |_| {
                let about_window = adw::AboutWindow::builder()
                    .transient_for(&root)
                    .application_icon(config::APP_ID)
                    .application_name("Exercise Timer")
                    .copyright(config::COPYRIGHT)
                    .designers(config::DESIGNERS)
                    .developers(config::DEVELOPERS)
                    .issue_url(config::ISSUE_TRACKER)
                    .license_type(gtk::License::Gpl30)
                    // .translator_credits("translator-credits")
                    .version(config::VERSION)
                    .website(config::HOMEPAGE)
                    .build();
                about_window.present();
            })
        };
        let preferences_action = {
            let root = root.clone();
            let global_settings = model.global_settings.clone();
            relm4::actions::RelmAction::<PreferencesAction>::new_stateless(move |_| {
                SettingsDialogModel::builder()
                    .transient_for(&root)
                    .launch(global_settings.clone())
                    .detach();
            })
        };
        let shortcuts_action = {
            let shortcuts_window_sender = model.shortcuts_window.sender().clone();
            relm4::actions::RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                shortcuts_window_sender
                    .send(ShortcutsWindowInput::Show)
                    .unwrap();
            })
        };
        let start_stop_action = {
            let sender = sender.clone();
            relm4::actions::RelmAction::<StartStopAction>::new_stateless(move |_| {
                sender.input(AppModelInput::StartStop);
            })
        };
        let reset_action = {
            let sender = sender.clone();
            relm4::actions::RelmAction::<ResetAction>::new_stateless(move |_| {
                sender.input(AppModelInput::Reset);
            })
        };
        actions.add_action(about_action);
        actions.add_action(preferences_action);
        actions.add_action(shortcuts_action);
        actions.add_action(start_stop_action);
        actions.add_action(reset_action);
        let list_exercises = model.list_exercises.widget();
        let widgets = view_output!();
        actions.register_for_widget(&widgets.main_window);
        relm4::main_application()
            .set_accelerators_for_action::<PreferencesAction>(&["<Control>comma"]);
        relm4::main_application()
            .set_accelerators_for_action::<ShortcutsAction>(&["<Control>question"]);
        relm4::main_application().set_accelerators_for_action::<StartStopAction>(&["space"]);
        relm4::main_application().set_accelerators_for_action::<ResetAction>(&["r"]);

        update_status_visible(&widgets, &model);
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
                self.list_exercises.guard().push_back(setup);
            }
            AppModelInput::LoadExercise(setup) => {
                self.exercise_timer = Some(
                    ExerciseTimer::builder()
                        .launch(ExerciseTimerInit {
                            setup,
                            global_setup: self.global_settings.clone(),
                            output_handle: self.output_stream.clone(),
                        })
                        .detach(),
                );
                widgets
                    .main_view
                    .set_content(Some(self.exercise_timer.as_ref().unwrap().widget()));
                widgets.navigation_view.push(&widgets.main_navigation_page);
            }
            AppModelInput::Popped => {
                self.exercise_timer = None;
            }
            AppModelInput::StartStop => {
                if let Some(controller) = &self.exercise_timer {
                    controller.emit(ExerciseTimerInput::StartStop);
                }
            }
            AppModelInput::Reset => {
                if let Some(controller) = &self.exercise_timer {
                    controller.emit(ExerciseTimerInput::Reset);
                }
            }
        }
        update_status_visible(&widgets, &self);
    }
}

fn update_status_visible(widgets: &AppModelWidgets, model: &AppModel) {
    if model.list_exercises.is_empty() {
        widgets
            .exercise_list_stack
            .set_visible_child(&widgets.exercise_list_status);
    } else {
        widgets
            .exercise_list_stack
            .set_visible_child(&widgets.exercise_list_scrolled);
    }
}

impl Drop for AppModel {
    fn drop(&mut self) {
        settings::save_exercise_list_to_gsettings(self.list_exercises.iter());
    }
}
