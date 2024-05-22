use crate::{
    exercise_editor::{SPIN_ROW_STEP, SPIN_ROW_UPPER},
    settings::*,
};

use gettextrs::gettext;
use relm4::{
    self,
    adw::{self, prelude::*},
    gtk,
    prelude::*,
    RelmObjectExt,
};

pub struct SettingsDialogModel;

#[relm4::component(pub)]
impl Component for SettingsDialogModel {
    type Init = GlobalExerciseSetup;
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::PreferencesWindow {
            set_visible: true,
            set_default_height: 400,
            set_default_width: 400,
            set_search_enabled: false,
            add = &adw::PreferencesPage {
                add = &adw::PreferencesGroup {
                    set_title: &gettext("Exercise defaults"),
                    gtk::ListBox {
                        add_css_class: "boxed-list",
                        adw::SpinRow {
                            set_title: &gettext("Warmup time"),
                            set_subtitle: &gettext("seconds"),
                            #[wrap(Some)]
                            #[name = "warmup_adjust"]
                            set_adjustment = &gtk::Adjustment {
                                set_lower: 0.0,
                                set_upper: SPIN_ROW_UPPER,
                                set_step_increment: SPIN_ROW_STEP,
                                add_binding: (&init.warmup_s, "value"),
                            },
                        },
                    }
                }
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();
        ComponentParts {
            model: SettingsDialogModel,
            widgets,
        }
    }
}
