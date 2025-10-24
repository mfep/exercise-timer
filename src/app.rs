use crate::config;
use crate::icon_names;
use crate::settings;
use crate::shortcuts_window::*;
use crate::training_editor::*;
use crate::training_setup::*;
use crate::training_timer::*;
use futures::prelude::*;
use gettextrs::gettext;
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
    PromptNewTraining,
    CreateTrainingSetup(TrainingSetup),
    RemoveTrainingSetup(DynamicIndex),
    LoadTraining(usize),
    Popped,
    StartStop,
    Reset,
    OpenMenu,
}

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
relm4::new_stateless_action!(StartStopAction, WindowActionGroup, "start-stop");
relm4::new_stateless_action!(ResetAction, WindowActionGroup, "reset");
relm4::new_stateless_action!(MenuAction, WindowActionGroup, "menu");

pub struct AppModel {
    training_timer: Option<Controller<TrainingTimer>>,
    list_trainings: relm4::factory::FactoryVecDeque<TrainingSetup>,
    window_geometry: settings::WindowGeometry,
    global_settings: settings::GlobalTrainingSetup,
    shortcuts_window: Controller<ShortcutsWindowModel>,
}

#[relm4::component(pub)]
impl Component for AppModel {
    type Init = ();
    type Input = AppModelInput;
    type Output = ();
    type CommandOutput = ();

    menu! {
        primary_menu: {
            section! {
                // Translators: The title of the keyboard shortcuts menu entry
                &gettext("_Keyboard Shortcuts") => ShortcutsAction,
                // Translators: The title of the about dialog menu entry
                &gettext("_About Exercise Timer") => AboutAction,
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
                    // Translators: This is the title of the page which lists all trainings. Currently this is the app name
                    set_title: &gettext("Exercise Timer"),
                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            pack_start = &gtk::Button {
                                set_icon_name: "list-add",
                                connect_clicked => AppModelInput::PromptNewTraining,
                                // Translators: tooltip for the add training image button
                                set_tooltip: &gettext("Add Training"),
                            },
                            #[name = "menu_button"]
                            pack_end = &gtk::MenuButton {
                                set_icon_name: "open-menu-symbolic",
                                set_menu_model: Some(&primary_menu),
                                // Translators: tooltip for main menu image button
                                set_tooltip: &gettext("Main Menu"),
                            },
                        },
                        #[wrap(Some)]
                        #[name = "training_list_stack"]
                        set_content = &gtk::Stack {
                            #[name = "training_list_scrolled"]
                            gtk::ScrolledWindow {
                                adw::Clamp
                                {
                                    #[local_ref]
                                    list_trainings -> gtk::ListBox {
                                        set_margin_top: 12,
                                        set_margin_bottom: 12,
                                        set_margin_start: 12,
                                        set_margin_end: 12,
                                        set_size_request: (200, -1),
                                        set_selection_mode: gtk::SelectionMode::None,
                                        set_valign: gtk::Align::Start,
                                        set_css_classes: &["boxed-list"],
                                        connect_row_activated[sender] => move |_, row| {
                                            sender.input(AppModelInput::LoadTraining(row.index() as usize));
                                        },
                                    }
                                },
                            },
                            #[name = "training_list_status"]
                            adw::StatusPage {
                                set_icon_name: Some(icon_names::WEIGHT2),
                                // Translators: The message which is shown on the background of the empty training list
                                set_title: &gettext("No Trainings"),
                                gtk::Button {
                                    set_css_classes: &["suggested-action", "pill"],
                                    // Translators: Big label button to create the first training if none exists
                                    set_label: &gettext("Create Training"),
                                    set_halign: gtk::Align::Center,
                                    connect_clicked => AppModelInput::PromptNewTraining,
                                }
                            },
                        },
                    },
                },
                connect_popped[sender] => move |_, _| { sender.input(AppModelInput::Popped); },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut list_trainings = relm4::factory::FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |output| match output {
                TrainingSetupOutput::Remove(index) => AppModelInput::RemoveTrainingSetup(index),
            });
        {
            let mut guard = list_trainings.guard();
            for training_setup in settings::load_training_list_from_gsettings().into_iter() {
                guard.push_back(training_setup);
            }
        }
        let model = AppModel {
            training_timer: None,
            list_trainings,
            window_geometry: settings::WindowGeometry::new_from_gsettings(),
            global_settings: settings::GlobalTrainingSetup::new_from_gsettings(),
            shortcuts_window: ShortcutsWindowModel::builder()
                .transient_for(&root)
                .launch(())
                .detach(),
        };
        let mut actions = relm4::actions::RelmActionGroup::<WindowActionGroup>::new();
        let about_action = {
            let root = root.clone();
            relm4::actions::RelmAction::<AboutAction>::new_stateless(move |_| {
                let about_dialog = adw::AboutDialog::builder()
                    .application_icon(config::APP_ID)
                    // Translators: The name of the application. Feel free to localize it!
                    .application_name(gettext("Exercise Timer"))
                    .copyright(config::COPYRIGHT)
                    .designers(config::DESIGNERS)
                    .developers(config::DEVELOPERS)
                    .issue_url(config::ISSUE_TRACKER)
                    .license_type(gtk::License::Gpl30)
                    // Translators: Replace this with your name for it to show up in the about window
                    .translator_credits(gettext("translator_credits"))
                    .version(config::VERSION)
                    .website(config::HOMEPAGE)
                    .build();
                about_dialog.present(Some(&root));
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
        let menu_action = {
            let sender = sender.clone();
            relm4::actions::RelmAction::<MenuAction>::new_stateless(move |_| {
                sender.input(AppModelInput::OpenMenu);
            })
        };
        actions.add_action(about_action);
        actions.add_action(shortcuts_action);
        actions.add_action(start_stop_action);
        actions.add_action(reset_action);
        actions.add_action(menu_action);
        let list_trainings = model.list_trainings.widget();
        let widgets = view_output!();
        actions.register_for_widget(&widgets.main_window);
        relm4::main_application()
            .set_accelerators_for_action::<ShortcutsAction>(&["<Control>question"]);
        relm4::main_application()
            .set_accelerators_for_action::<StartStopAction>(&["<Control>space"]);
        relm4::main_application().set_accelerators_for_action::<ResetAction>(&["<Control>r"]);
        relm4::main_application().set_accelerators_for_action::<MenuAction>(&["F10"]);

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
            AppModelInput::PromptNewTraining => {
                let editor = TrainingEditor::builder()
                    .launch((TrainingEditorRole::New, TrainingSetup::default()));
                editor.widget().present(Some(root.widget_ref()));
                let mut editor = editor.into_stream();
                relm4::spawn_local(async move {
                    if let Some(TrainingEditorOutput::Create(setup)) = editor.next().await.unwrap()
                    {
                        sender.input(AppModelInput::CreateTrainingSetup(setup));
                    }
                });
            }
            AppModelInput::RemoveTrainingSetup(index) => {
                let index = index.current_index();
                self.list_trainings.guard().remove(index);
            }
            AppModelInput::CreateTrainingSetup(setup) => {
                self.list_trainings.guard().push_back(setup);
            }
            AppModelInput::LoadTraining(idx) => {
                if let Some(setup) = self.list_trainings.get(idx) {
                    self.training_timer = Some(
                        TrainingTimer::builder()
                            .launch(TrainingTimerInit {
                                setup: setup.clone(),
                                global_setup: self.global_settings.clone(),
                            })
                            .detach(),
                    );
                    widgets
                        .navigation_view
                        .add(self.training_timer.as_ref().unwrap().widget());
                    widgets
                        .navigation_view
                        .push(self.training_timer.as_ref().unwrap().widget());
                    self.training_timer
                        .as_ref()
                        .unwrap()
                        .widgets()
                        .play_pause_button
                        .grab_focus();
                }
            }
            AppModelInput::Popped => {
                self.training_timer = None;
            }
            AppModelInput::StartStop => {
                if let Some(controller) = &self.training_timer {
                    controller.emit(TrainingTimerInput::StartStop);
                }
            }
            AppModelInput::Reset => {
                if let Some(controller) = &self.training_timer {
                    controller.emit(TrainingTimerInput::Reset);
                }
            }
            AppModelInput::OpenMenu => {
                widgets.menu_button.emit_activate();
            }
        }
        update_status_visible(widgets, self);
    }
}

fn update_status_visible(widgets: &AppModelWidgets, model: &AppModel) {
    if model.list_trainings.is_empty() {
        widgets
            .training_list_stack
            .set_visible_child(&widgets.training_list_status);
    } else {
        widgets
            .training_list_stack
            .set_visible_child(&widgets.training_list_scrolled);
    }
}

impl Drop for AppModel {
    fn drop(&mut self) {
        settings::save_training_list_to_gsettings(self.list_trainings.iter());
    }
}
