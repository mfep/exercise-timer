use crate::training_setup::*;
use gettextrs::gettext;
use relm4::{
    self,
    binding::*,
    gtk::gio::{self, prelude::*},
};

#[derive(Clone, Debug, Default)]
pub struct WindowGeometry {
    pub width: I32Binding,
    pub height: I32Binding,
    pub is_maximized: BoolBinding,
}

impl WindowGeometry {
    pub fn new_from_gsettings() -> Self {
        let settings = gio::Settings::new(crate::config::APP_ID);
        Self {
            width: I32Binding::new(settings.int("window-width")),
            height: I32Binding::new(settings.int("window-height")),
            is_maximized: BoolBinding::new(settings.boolean("window-is-maximized")),
        }
    }
}

impl Drop for WindowGeometry {
    fn drop(&mut self) {
        let settings = gio::Settings::new(crate::config::APP_ID);
        settings.delay();
        let _ = settings.set_int("window-width", self.width.get());
        let _ = settings.set_int("window-height", self.height.get());
        let _ = settings.set_boolean("window-is-maximized", self.is_maximized.get());
        settings.apply();
    }
}

#[derive(Clone, Debug, Default)]
pub struct GlobalTrainingSetup {
    pub beep_volume: F64Binding,
}

impl GlobalTrainingSetup {
    pub fn new_from_gsettings() -> Self {
        let settings = gio::Settings::new(crate::config::APP_ID);
        Self {
            beep_volume: F64Binding::new(settings.double("beep-volume")),
        }
    }
}

impl Drop for GlobalTrainingSetup {
    fn drop(&mut self) {
        let settings = gio::Settings::new(crate::config::APP_ID);
        settings.delay();
        let _ = settings.set_double("beep-volume", self.beep_volume.get());
        settings.apply();
    }
}

fn parse_json_to_training_setup(value: &json::JsonValue) -> TrainingSetup {
    let name = value["name"]
        .as_str()
        // Translators: Error message printed to the console when key 'name' is not found in the JSON formatted training
        .unwrap_or_else(|| panic!("{}", gettext("Cannot find 'name' in settings dictionary")));
    let sets = value["sets"]
        .as_usize()
        // Translators: Error message printed to the console when key 'sets' is not found in the JSON formatted training
        .unwrap_or_else(|| panic!("{}", gettext("Cannot find 'sets' in settings dictionary")));
    let exercise_s = value["exercise_s"].as_usize().unwrap_or_else(|| {
        panic!(
            "{}",
            // Translators: Error message printed to the console when key 'exercise_s' is not found in the JSON formatted training
            gettext("Cannot find 'exercise_s' in settings dictionary")
        )
    });
    let rest_s = value["rest_s"]
        .as_usize()
        // Translators: Error message printed to the console when key 'rest_s' is not found in the JSON formatted training
        .unwrap_or_else(|| panic!("{}", gettext("Cannot find 'rest_s' in settings dictionary")));
    let prepare_s = value["prepare_s"].as_usize().unwrap_or(5);

    TrainingSetup {
        name: gettext(name),
        sets,
        exercise_s,
        rest_s,
        prepare_s,
    }
}

pub fn load_default_training_setup() -> TrainingSetup {
    let settings = gio::Settings::new(crate::config::APP_ID);
    let raw_json = settings.string("default-exercise-json");
    parse_json_to_training_setup(&json::parse(&raw_json).unwrap_or_else(|err| {
        panic!(
            "{}: {}",
            // Translators: Error message printed to the console when the default training setup loaded from the settings cannot be parsed
            gettext("Could not parse default training setup"),
            err
        )
    }))
}

pub fn load_training_list_from_gsettings() -> Vec<TrainingSetup> {
    let settings = gio::Settings::new(crate::config::APP_ID);
    let raw_json = settings.string("exercise-json-list");
    let parsed = json::parse(&raw_json)
        // Translators: Error message printed to the console when the JSON formatted list of user-created trainings cannot be parsed
        .unwrap_or_else(|err| panic!("{}: {}", gettext("Could not parse exercise list"), err));
    parsed.members().map(parse_json_to_training_setup).collect()
}

pub fn save_training_list_to_gsettings<'a>(exercises: impl Iterator<Item = &'a TrainingSetup>) {
    let settings = gio::Settings::new(crate::config::APP_ID);
    let json_list: Vec<json::JsonValue> = exercises
        .map(|training| {
            json::object! {
                name: training.name.clone(),
                sets: training.sets,
                exercise_s: training.exercise_s,
                rest_s: training.rest_s,
                prepare_s: training.prepare_s,
            }
        })
        .collect();
    settings
        .set("exercise-json-list", json::stringify(json_list))
        .unwrap_or_else(|err| {
            panic!(
                "{}: {}",
                // Translators: Error message printed to the console when the JSON formatted list of user-created trainings cannot be written to the settings
                gettext("Could not update settings with training list"),
                err
            )
        });
}
