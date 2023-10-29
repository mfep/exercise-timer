use relm4::{
    self,
    adw::{self, prelude::*},
    gtk,
    prelude::*,
    RelmObjectExt,
};

use crate::settings;

pub struct SettingsDialogModel;

#[relm4::component(pub)]
impl Component for SettingsDialogModel {
    type Init = settings::GlobalExerciseSetup;
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::Window {
            set_modal: true,
            set_visible: true,
            set_width_request: 400,
            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Preferences",
                    },
                },
                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    set_vexpand: true,
                    adw::Clamp {
                        gtk::Box {
                            set_margin_all: 12,
                            set_spacing: 8,
                            set_hexpand: true,
                            adw::PreferencesGroup {
                                set_title: "Exercise defaults",
                                gtk::ListBox {
                                    add_css_class: "boxed-list",
                                    adw::SpinRow {
                                        set_title: "Warmup time",
                                        set_subtitle: "seconds",
                                        #[wrap(Some)]
                                        #[name = "warmup_adjust"]
                                        set_adjustment = &gtk::Adjustment {
                                            set_lower: 0.0,
                                            set_upper: 999.0,
                                            set_step_increment: 1.0,
                                            add_binding: (&init.warmup_s, "value"),
                                        },
                                    },
                                }
                            }
                        }
                    }
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();
        ComponentParts {
            model: SettingsDialogModel,
            widgets,
        }
    }
}
