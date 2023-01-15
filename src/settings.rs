use std::{collections::{HashMap, HashSet}, process::exit};

use config::{Config, ConfigError};
use native_dialog::MessageDialog;
use serde::Deserialize;
use crate::controller::input::ControllerTypeDetection;

#[derive(Clone, Deserialize)]
pub struct OverlaySettings {
    screen_height: f32,
    screen_width: f32,
    show_crosshair: bool,
    show_buttons: bool,
    always_show_overlay: bool,
    windowed_mode: bool,
}

impl OverlaySettings {
    pub fn screen_height(&self) -> f32 {self.screen_height}
    pub fn screen_width(&self) -> f32 {self.screen_width}
    pub fn show_crosshair(&self) -> bool {self.show_crosshair}
    pub fn show_buttons(&self) -> bool {self.show_buttons}
    pub fn always_show_overlay(&self) -> bool {self.always_show_overlay}
    pub fn windowed_mode(&self) -> bool {self.windowed_mode}
}

#[derive(Clone, Deserialize)]
pub struct ControllerSettings {
    #[serde(rename(deserialize = "dead_zone_percentage"))]
    controller_deadzone: f32,
    character_x_offset_px: f32,
    character_y_offset_px: f32,
    walk_circle_radius_px: f32,
    close_circle_radius_px: f32,
    mid_circle_radius_px: f32,
    far_circle_radius_px: f32,
    free_mouse_sensitivity_px: f32,
    controller_type: ControllerTypeDetection,
}

impl ControllerSettings {
    pub fn controller_deadzone(&self) -> f32 {self.controller_deadzone}
    pub fn character_x_offset_px(&self) -> f32 {self.character_x_offset_px}
    pub fn character_y_offset_px(&self) -> f32 {self.character_y_offset_px}
    pub fn walk_circle_radius_px(&self) -> f32 {self.walk_circle_radius_px}
    pub fn close_circle_radius_px(&self) -> f32 {self.close_circle_radius_px}
    pub fn mid_circle_radius_px(&self) -> f32 {self.mid_circle_radius_px}
    pub fn far_circle_radius_px(&self) -> f32 {self.far_circle_radius_px}
    pub fn free_mouse_sensitivity_px(&self) -> f32 {self.free_mouse_sensitivity_px}
    pub fn controller_type(&self) -> ControllerTypeDetection {self.controller_type.clone()}
}
#[derive(Clone, Deserialize, Eq, Hash, PartialEq)]
pub enum ButtonOrKey {
    Button(rdev::Button),
    Key(rdev::Key),
    ButtonKeyChord(rdev::Button, rdev::Key),
    Empty,
}

#[derive(Clone, Deserialize)]
pub struct ApplicationSettings {
    #[serde(rename(deserialize = "overlay"))]
    overlay_settings: OverlaySettings,
    #[serde(rename(deserialize = "button_mapping"))]
    button_mapping_settings: HashMap<gilrs::Button, ButtonOrKey>,
    #[serde(skip_deserializing)]
    ability_mapping_settings: HashMap<ButtonOrKey, gilrs::Button>,
    aimable_buttons: Vec<gilrs::Button>,
    action_distances: HashMap<gilrs::Button, String>,
    #[serde(rename(deserialize = "controller"))]
    controller_settings: ControllerSettings,
}

impl ApplicationSettings {
    pub fn overlay_settings(&self) -> OverlaySettings {self.overlay_settings.clone()}
    pub fn button_mapping_settings(&self) -> HashMap<gilrs::Button, ButtonOrKey> {self.button_mapping_settings.clone()}
    pub fn ability_mapping_settings(&self) -> HashMap<ButtonOrKey, gilrs::Button> {self.ability_mapping_settings.clone()}
    pub fn aimable_buttons(&self) -> Vec<gilrs::Button> {self.aimable_buttons.clone()}
    pub fn action_distances(&self) -> HashMap<gilrs::Button, String> {self.action_distances.clone()}
    pub fn controller_settings(&self) -> ControllerSettings {self.controller_settings.clone()}

    fn sanitize_settings(&mut self) {
        if self.overlay_settings.always_show_overlay() && self.overlay_settings.windowed_mode() {
            alert_and_exit_on_invalid_settings("Windowed Mode is unsupported when coupled with Always Show Overlay!");
            panic!("Windowed Mode is unsupported when coupled with Always Show Overlay!");
        }

        let valid_ability_buttons: HashSet<gilrs::Button> = HashSet::from(
            [
                gilrs::Button::South,
                gilrs::Button::East,
                gilrs::Button::West,
                gilrs::Button::North,
                gilrs::Button::LeftTrigger,
                gilrs::Button::RightTrigger,
                gilrs::Button::LeftTrigger2,
                gilrs::Button::RightTrigger2]);

        let valid_ability_ranges: HashSet<String>= HashSet::from(["close", "mid", "far"].map(|x| x.to_owned()));
        let buttons: Vec<gilrs::Button> = self.action_distances.keys().cloned().collect();
        let distances: Vec<String> = self.action_distances.values().cloned().collect();

        // Ensure ability ranges!
        for button in &buttons {
            if !valid_ability_buttons.contains(button) {
                alert_and_exit_on_invalid_settings(&format!("{:?} is not a valid button ({:#?})", button, valid_ability_buttons));
                panic!("{:?} is not a valid button ({:#?})", button, valid_ability_buttons);
            }
        }
        for distance in &distances {
            if !valid_ability_ranges.contains(distance) {
                alert_and_exit_on_invalid_settings(&format!("{:} is not a valid distance ({:#?})", distance, valid_ability_ranges));
                panic!("{:} is not a valid distance ({:#?})", distance, valid_ability_ranges);
            }
        }
        // Ensure buttons are valid!
        let valid_buttons_set = HashSet::from(
            [
                gilrs::Button::West,
                gilrs::Button::North,
                gilrs::Button::South,
                gilrs::Button::East,
                gilrs::Button::Start,
                gilrs::Button::Select,
                gilrs::Button::DPadDown,
                gilrs::Button::DPadLeft,
                gilrs::Button::DPadRight,
                gilrs::Button::DPadUp,
                gilrs::Button::LeftThumb,
                gilrs::Button::RightThumb,
                gilrs::Button::LeftTrigger,
                gilrs::Button::RightTrigger,
                gilrs::Button::LeftTrigger2,
                gilrs::Button::RightTrigger2]);

        let button_mapping_keys: Vec<gilrs::Button> = self.button_mapping_settings.keys().cloned().collect();
        let button_mapping_key_set: HashSet<gilrs::Button>= HashSet::from_iter(button_mapping_keys);
        if !ensure_initialized(&button_mapping_key_set, &valid_buttons_set) {
            incorrect_keys(&button_mapping_key_set, &valid_buttons_set)
        }

        // Ensure aimables
        let valid_aimable_buttons_set = HashSet::from(
            [
                gilrs::Button::West,
                gilrs::Button::North,
                gilrs::Button::South,
                gilrs::Button::East,
                gilrs::Button::LeftTrigger,
                gilrs::Button::RightTrigger,
                gilrs::Button::LeftTrigger2,
                gilrs::Button::RightTrigger2]);

        for button in &self.aimable_buttons {
            if !valid_aimable_buttons_set.contains(button) {
                alert_and_exit_on_invalid_settings(&format!("{:#?} is not a valid aimable button ({:#?})", button, valid_aimable_buttons_set));
                panic!("{:#?} is not a valid aimable button ({:#?})", button, valid_aimable_buttons_set);
            }
        }

        // Setup ability_mapping_settings
        self.ability_mapping_settings.insert(self.button_mapping_settings.get(&gilrs::Button::South).unwrap().clone(), gilrs::Button::South);
        self.ability_mapping_settings.insert(self.button_mapping_settings.get(&gilrs::Button::East).unwrap().clone(), gilrs::Button::East);
        self.ability_mapping_settings.insert(self.button_mapping_settings.get(&gilrs::Button::West).unwrap().clone(), gilrs::Button::West);
        self.ability_mapping_settings.insert(self.button_mapping_settings.get(&gilrs::Button::North).unwrap().clone(), gilrs::Button::North);
        self.ability_mapping_settings.insert(self.button_mapping_settings.get(&gilrs::Button::LeftTrigger).unwrap().clone(), gilrs::Button::LeftTrigger);
        self.ability_mapping_settings.insert(self.button_mapping_settings.get(&gilrs::Button::RightTrigger).unwrap().clone(), gilrs::Button::RightTrigger);
        self.ability_mapping_settings.insert(self.button_mapping_settings.get(&gilrs::Button::LeftTrigger2).unwrap().clone(), gilrs::Button::LeftTrigger2);
        self.ability_mapping_settings.insert(self.button_mapping_settings.get(&gilrs::Button::RightTrigger2).unwrap().clone(), gilrs::Button::RightTrigger2);
    }
}

fn ensure_initialized(test: &HashSet<gilrs::Button>, control: &HashSet<gilrs::Button>) -> bool {
    test.is_subset(&control) && control.is_subset(&test)
}

fn incorrect_keys(test: &HashSet<gilrs::Button>, control: &HashSet<gilrs::Button>) {
    let missing: Vec<&gilrs::Button> = control.difference(&test).collect();
    if missing.len() > 0 {
        alert_and_exit_on_invalid_settings(&format!("Must initialize button_mapping! You are missing {:#?}", missing));
        panic!("Must initialize button_mapping! You are missing {:#?}", missing);
    } else {
        let extra: Vec<&gilrs::Button> = test.difference(&control).collect();
        alert_and_exit_on_invalid_settings(&format!("Only initialize proper buttons: {:#?}! \n Your extras are: {:#?}", extra, control));
        panic!("Only initialize proper buttons: {:#?}! \n Your extras are: {:#?}", extra, control);
    }
}


pub fn alert_and_exit_on_invalid_settings(error_message: &str) {
    // Blocks execution on message display until closes the message

    // Uncomment this if you want panics instead of popups while debugging
    // #![cfg(not(debug_assertions))]
    {
        MessageDialog::new()
        .set_type(native_dialog::MessageType::Info)
        .set_title("Invalid configuration")
        .set_text(error_message)
        .show_alert()
        .unwrap();
    
        exit(78);
    }
}

pub fn load_settings() -> ApplicationSettings {
    let settings = Config::builder()
                    .add_source(config::File::with_name("settings.toml"))
                    .build()
                    .unwrap_or_else(|error| {
                        alert_and_exit_on_invalid_settings(&format!("Settings failed to load. Error: {:?}", error.to_string()));
                        panic!("config failed to load. Error: {error}")
                    });
 
    let deserialized: Result<ApplicationSettings, ConfigError>= settings.try_deserialize();
    match deserialized {
        Ok(mut result) => {
            result.sanitize_settings();
            result
        },
        Err(e) => {
            match e {
                ConfigError::Type { origin: _, unexpected: _, expected: _, key } => {
                    alert_and_exit_on_invalid_settings(&format!("Unable to load '{:?}''. Please check settings.toml", key));
                    panic!("Unable to load '{:?}''. Please check settings.toml", key)
                },
                _ => {
                    alert_and_exit_on_invalid_settings(&format!("{:?}", e));
                    panic!("{:?}", e)
                }
            }
        }
    }
}


