mod exercise_timer;

use std::convert::identity;

use exercise_timer::{ExerciseSetup, ExerciseTimer};
use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::typed_list_view::{RelmListItem, TypedListView};
use relm4::Controller;
use relm4::{
    adw,
    gtk::{self, prelude::ObjectExt},
    Component, ComponentController, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt,
    SimpleComponent,
};

struct AppModel {
    exerciser: Controller<ExerciseTimer>,
    list_exercises: TypedListView<ExerciseSetup, gtk::NoSelection>,
}

pub struct ExerciseSetupWidgets {
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

        let widgets = ExerciseSetupWidgets { label };

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
            exerciser: ExerciseTimer::builder()
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
