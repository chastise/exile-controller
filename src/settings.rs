use std::{collections::{HashMap, HashSet}, process::exit};

use config::{Config, ConfigError};
use native_dialog::MessageDialog;
use serde::Deserialize;
use crate::controller::{input::ControllerTypeDetection, action_manager::ActionDistance};

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

fn string_to_buttonorkey (buttonorkey_string: &str) -> ButtonOrKey {
    let lower = buttonorkey_string.to_lowercase();

    // TODO(Samantha): Pattern matching here would be cleaner, but I don't think it's worth pulling in regex?
    // These if guards check to see if we should be making a buttonkeychord.
    if lower.ends_with("leftclick") && lower.len() > "leftclick".len() {
        if let ButtonOrKey::Key(k) = string_to_buttonorkey(lower.trim_end_matches("leftclick")) {
            return ButtonOrKey::ButtonKeyChord(rdev::Button::Left, k);
        }
    }
    if lower.ends_with("rightclick") && lower.len() > "rightclick".len(){
        if let ButtonOrKey::Key(k) = string_to_buttonorkey(lower.trim_end_matches("rightclick")) {
            return ButtonOrKey::ButtonKeyChord(rdev::Button::Right, k);
        }
    }
    if lower.ends_with("middleclick") && lower.len() > "middleclick".len(){
        if let ButtonOrKey::Key(k) = string_to_buttonorkey(lower.trim_end_matches("middleclick")) {
            return ButtonOrKey::ButtonKeyChord(rdev::Button::Middle, k);
        }
    }

    match lower.as_str() {
        "leftclick" => ButtonOrKey::Button(rdev::Button::Left),
        "middleclick" => ButtonOrKey::Button(rdev::Button::Middle),
        "rightclick" => ButtonOrKey::Button(rdev::Button::Right),
        "f1" => ButtonOrKey::Key(rdev::Key::F1),
        "f2" => ButtonOrKey::Key(rdev::Key::F2),
        "f3" => ButtonOrKey::Key(rdev::Key::F3),
        "f4" => ButtonOrKey::Key(rdev::Key::F4),
        "f5" => ButtonOrKey::Key(rdev::Key::F5),
        "f6" => ButtonOrKey::Key(rdev::Key::F6),
        "f7" => ButtonOrKey::Key(rdev::Key::F7),
        "f8" => ButtonOrKey::Key(rdev::Key::F8),
        "f9" => ButtonOrKey::Key(rdev::Key::F9),
        "f10" => ButtonOrKey::Key(rdev::Key::F10),
        "f11" => ButtonOrKey::Key(rdev::Key::F11),
        "f12" => ButtonOrKey::Key(rdev::Key::F12),
        "a" => ButtonOrKey::Key(rdev::Key::KeyA),
        "b" => ButtonOrKey::Key(rdev::Key::KeyB),
        "c" => ButtonOrKey::Key(rdev::Key::KeyC),
        "d" => ButtonOrKey::Key(rdev::Key::KeyD),
        "e" => ButtonOrKey::Key(rdev::Key::KeyE),
        "f" => ButtonOrKey::Key(rdev::Key::KeyF),
        "g" => ButtonOrKey::Key(rdev::Key::KeyG),
        "h" => ButtonOrKey::Key(rdev::Key::KeyH),
        "i" => ButtonOrKey::Key(rdev::Key::KeyI),
        "j" => ButtonOrKey::Key(rdev::Key::KeyJ),
        "k" => ButtonOrKey::Key(rdev::Key::KeyK),
        "l" => ButtonOrKey::Key(rdev::Key::KeyL),
        "m" => ButtonOrKey::Key(rdev::Key::KeyM),
        "n" => ButtonOrKey::Key(rdev::Key::KeyN),
        "o" => ButtonOrKey::Key(rdev::Key::KeyO),
        "p" => ButtonOrKey::Key(rdev::Key::KeyP),
        "q" => ButtonOrKey::Key(rdev::Key::KeyQ),
        "r" => ButtonOrKey::Key(rdev::Key::KeyR),
        "s" => ButtonOrKey::Key(rdev::Key::KeyS),
        "t" => ButtonOrKey::Key(rdev::Key::KeyT),
        "u" => ButtonOrKey::Key(rdev::Key::KeyU),
        "v" => ButtonOrKey::Key(rdev::Key::KeyV),
        "w" => ButtonOrKey::Key(rdev::Key::KeyW),
        "x" => ButtonOrKey::Key(rdev::Key::KeyX),
        "y" => ButtonOrKey::Key(rdev::Key::KeyY),
        "z" => ButtonOrKey::Key(rdev::Key::KeyZ),
        "0" => ButtonOrKey::Key(rdev::Key::Num0),
        "1" => ButtonOrKey::Key(rdev::Key::Num1),
        "2" => ButtonOrKey::Key(rdev::Key::Num2),
        "3" => ButtonOrKey::Key(rdev::Key::Num3),
        "4" => ButtonOrKey::Key(rdev::Key::Num4),
        "5" => ButtonOrKey::Key(rdev::Key::Num5),
        "6" => ButtonOrKey::Key(rdev::Key::Num6),
        "7" => ButtonOrKey::Key(rdev::Key::Num7),
        "8" => ButtonOrKey::Key(rdev::Key::Num8),
        "9" => ButtonOrKey::Key(rdev::Key::Num9),
        "`" => ButtonOrKey::Key(rdev::Key::BackQuote),
        "[" => ButtonOrKey::Key(rdev::Key::LeftBracket),
        "]" => ButtonOrKey::Key(rdev::Key::RightBracket),
        ";" => ButtonOrKey::Key(rdev::Key::SemiColon),
        "/" => ButtonOrKey::Key(rdev::Key::Slash),
        "," => ButtonOrKey::Key(rdev::Key::Comma),
        "=" => ButtonOrKey::Key(rdev::Key::Equal),
        "escape" => ButtonOrKey::Key(rdev::Key::Escape),
        "space" => ButtonOrKey::Key(rdev::Key::Space),
        "tab" => ButtonOrKey::Key(rdev::Key::Tab),
        "backspace" => ButtonOrKey::Key(rdev::Key::Backspace),
        "delete" => ButtonOrKey::Key(rdev::Key::Delete),
        "uparrow" => ButtonOrKey::Key(rdev::Key::UpArrow),
        "downarrow" => ButtonOrKey::Key(rdev::Key::DownArrow),
        "leftarrow" => ButtonOrKey::Key(rdev::Key::LeftArrow),
        "rightarrow" => ButtonOrKey::Key(rdev::Key::RightArrow),
        "alt" => ButtonOrKey::Key(rdev::Key::Alt),
        "shift" => ButtonOrKey::Key(rdev::Key::ShiftLeft),
        "control" => ButtonOrKey::Key(rdev::Key::ControlLeft),
        "" => ButtonOrKey::Empty,
        _ => {
            alert_and_exit_on_invalid_settings(&format!("Unrecognized mouse button or key: {:}!", buttonorkey_string.to_lowercase().as_str()));
            panic!("Unrecognized mouse button or key: {:}!", buttonorkey_string.to_lowercase().as_str());
        },
    }
}

#[derive(Clone, Deserialize)]
pub struct ApplicationSettings {
    #[serde(rename(deserialize = "overlay"))]
    overlay_settings: OverlaySettings,

    // TODO(Samantha): See if we can't pass the functions to turn the user friendly strings into their proper types into the config loader.
    // These duplicated fields are intermediaries to load the config values into their concrete types.
    #[serde(rename(deserialize = "button_mapping"))]
    button_mapping_settings_strings: HashMap<String, String>,
    #[serde(skip_deserializing)]
    button_mapping_settings: HashMap<gilrs::Button, ButtonOrKey>,

    #[serde(skip_deserializing)]
    ability_mapping_settings: HashMap<ButtonOrKey, gilrs::Button>,

    #[serde(rename(deserialize = "aimable_buttons"))]
    aimable_buttons_strings: Vec<String>,
    #[serde(skip_deserializing)]
    aimable_buttons: Vec<gilrs::Button>,

    #[serde(rename(deserialize = "action_distances"))]
    action_distances_strings: HashMap<String, ActionDistance>,
    #[serde(skip_deserializing)]
    action_distances: HashMap<gilrs::Button, ActionDistance>,

    #[serde(rename(deserialize = "controller"))]
    controller_settings: ControllerSettings,
}

fn string_to_controller_button(button_str: &str) -> gilrs::Button {
    match button_str.to_lowercase().as_str() {
        // We attempt to allow for different names of buttons.
        "a" | "cross" => gilrs::Button::South,
        "b" | "circle" => gilrs::Button::East,
        "x" | "square" => gilrs::Button::West,
        "y" | "triangle" => gilrs::Button::North,
        "start" | "options" => gilrs::Button::Start,
        "back" | "select" | "share" => gilrs::Button::Select,
        "dpad_down" => gilrs::Button::DPadDown,
        "dpad_left" => gilrs::Button::DPadLeft,
        "dpad_right" => gilrs::Button::DPadRight,
        "dpad_up" => gilrs::Button::DPadUp,
        "left_analog" | "l3" => gilrs::Button::LeftThumb,
        "right_analog" | "r3" => gilrs::Button::RightThumb,
        "bumper_left" | "l1" => gilrs::Button::LeftTrigger,
        "bumper_right" | "r1" => gilrs::Button::RightTrigger,
        "trigger_left" | "l2" => gilrs::Button::LeftTrigger2,
        "trigger_right" | "r2" => gilrs::Button::RightTrigger2,
        _ => gilrs::Button::Unknown,
    }
}

impl ApplicationSettings {
    pub fn overlay_settings(&self) -> OverlaySettings {self.overlay_settings.clone()}
    pub fn button_mapping_settings(&self) -> HashMap<gilrs::Button, ButtonOrKey> {self.button_mapping_settings.clone()}
    pub fn ability_mapping_settings(&self) -> HashMap<ButtonOrKey, gilrs::Button> {self.ability_mapping_settings.clone()}
    pub fn aimable_buttons(&self) -> Vec<gilrs::Button> {self.aimable_buttons.clone()}
    pub fn action_distances(&self) -> HashMap<gilrs::Button, ActionDistance> {self.action_distances.clone()}
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

        // Finish deserializing these from strings into their concrete types.
        self.action_distances = HashMap::from_iter(
            self.action_distances_strings.iter()
            .map(|(button_string, distance)|
                (string_to_controller_button(button_string), distance.clone())));

        self.button_mapping_settings = HashMap::from_iter(
            self.button_mapping_settings_strings.iter()
            .map(|(key, value)|
            (string_to_controller_button(key), string_to_buttonorkey(value))));

        self.aimable_buttons = self.aimable_buttons_strings.iter()
            .map(|button| string_to_controller_button(button)).collect();

        let buttons: Vec<gilrs::Button> = self.action_distances.keys().cloned().collect();

        // Ensure ability ranges!
        for button in &buttons {
            if !valid_ability_buttons.contains(button) {
                alert_and_exit_on_invalid_settings(&format!("{:?} is not a valid button ({:#?})", button, valid_ability_buttons));
                panic!("{:?} is not a valid button ({:#?})", button, valid_ability_buttons);
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


