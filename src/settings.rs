use std::{collections::HashMap, process::exit};

use config::{Config, ConfigError};
use native_dialog::MessageDialog;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct ControllerSettings {
    controller_deadzone: f32,
    character_x_offset_px: f32,
    character_y_offset_px: f32,
    walk_circle_radius_px: f32,
    close_circle_radius_px: f32,
    mid_circle_radius_px: f32,
    far_circle_radius_px: f32,
    free_mouse_sensitivity_px: f32,
    controller_type: String,
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
    pub fn controller_type(&self) -> String {self.controller_type.clone()}
}

#[derive(Clone)]
pub struct ApplicationSettings {
    overlay_settings: OverlaySettings,
    button_mapping_settings: HashMap<String, String>,
    ability_mapping_settings: HashMap<String, String>,
    aimable_buttons: Vec<String>,
    action_distances: HashMap<String, String>,
    controller_settings: ControllerSettings,
}

impl ApplicationSettings {
    pub fn overlay_settings(&self) -> OverlaySettings {self.overlay_settings.clone()}
    pub fn button_mapping_settings(&self) -> HashMap<String, String> {self.button_mapping_settings.clone()}
    pub fn ability_mapping_settings(&self) -> HashMap<String, String> {self.ability_mapping_settings.clone()}
    pub fn aimable_buttons(&self) -> Vec<String> {self.aimable_buttons.clone()}
    pub fn action_distances(&self) -> HashMap<String, String> {self.action_distances.clone()}
    pub fn controller_settings(&self) -> ControllerSettings {self.controller_settings.clone()}
}

fn alert_and_exit_on_invalid_settings(error_message: &str) {
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

fn safe_unwrap_config_results<T>(result: Result<T, ConfigError>, table: &str, key: &str) -> T {
    result.unwrap_or_else(|_error|{
        alert_and_exit_on_invalid_settings(&format!("Unable to load '{:?}' under '{:?}'. Please check settings.toml", key, table));
        panic!("Unable to load '{:?}' under '{:?}'. Please check settings.toml", key, table)
    })
}
fn safe_get_int_from_settings(settings: &Config, table: &str, key: &str) -> i64 { 
    safe_unwrap_config_results(settings.get_int((table.to_owned() + "." + key).as_str()), table, key)
}
fn safe_get_bool_from_settings(settings: &Config, table: &str, key: &str) -> bool {
    safe_unwrap_config_results(settings.get_bool((table.to_owned() + "." + key).as_str()), table, key)
}
fn safe_get_float_from_settings(settings: &Config, table: &str, key: &str) -> f64 {
    safe_unwrap_config_results(settings.get_float((table.to_owned() + "." + key).as_str()), table, key)
}
fn safe_get_string_from_settings(settings: &Config, table: &str, key: &str) -> String {
    safe_unwrap_config_results(settings.get_string((table.to_owned() + "." + key).as_str()), table, key)
}

pub fn load_settings() -> ApplicationSettings {
    let settings = Config::builder()
                    .add_source(config::File::with_name("settings.toml"))
                    .build()
                    .unwrap_or_else(|error| {
                        alert_and_exit_on_invalid_settings(&format!("Settings failed to load. Error: {:?}", error.to_string()));
                        panic!("config failed to load. Error: {error}")
                    });

    let valid_ability_buttons = ["a", "b", "x", "y", "bumper_left", "bumper_right", "trigger_left", "trigger_right"].map(|x| x.to_string());
    let valid_ability_ranges = ["close", "mid", "far"].map(|x| x.to_string());
    let valid_controller_types = ["auto", "xbox", "ps"].map(|x| x.to_string());
    ApplicationSettings {
        overlay_settings: {
            let overlay_settings = OverlaySettings {
                screen_height: safe_get_int_from_settings(&settings, "overlay", "screen_height") as f32,
                screen_width: safe_get_int_from_settings(&settings, "overlay", "screen_width") as f32,
                show_crosshair: safe_get_bool_from_settings(&settings, "overlay", "show_crosshair"),
                show_buttons: safe_get_bool_from_settings(&settings, "overlay", "show_buttons"),
                always_show_overlay: safe_get_bool_from_settings(&settings, "overlay", "always_show_overlay"),
                windowed_mode: safe_get_bool_from_settings(&settings, "overlay", "windowed_mode"),
            };
            if overlay_settings.windowed_mode() && overlay_settings.always_show_overlay() {
                alert_and_exit_on_invalid_settings("Invalid settings in overlay: always_show_overlay and windowed_mode cannot both be enabled.");
            };
            overlay_settings
        },

        button_mapping_settings: {
            let mut map = HashMap::<String, String>::new(); 
            map.insert("dpad_up".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "dpad_up"));
            map.insert("dpad_down".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "dpad_down"));
            map.insert("dpad_left".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "dpad_left"));
            map.insert("dpad_right".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "dpad_right"));
            map.insert("start".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "start"));
            map.insert("back".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "back"));
            map.insert("a".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "a"));
            map.insert("b".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "b"));
            map.insert("x".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "x"));
            map.insert("y".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "y"));
            map.insert("left_analog".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "left_analog"));
            map.insert("right_analog".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "right_analog"));
            map.insert("bumper_left".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "bumper_left"));
            map.insert("bumper_right".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "bumper_right"));
            map.insert("trigger_left".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "trigger_left"));
            map.insert("trigger_right".to_owned(), safe_get_string_from_settings(&settings, "button_mapping", "trigger_right"));
            map
        },
        ability_mapping_settings: {
            let mut map = HashMap::<String, String>::new();
            map.insert(safe_get_string_from_settings(&settings, "button_mapping", "a"), "a".to_owned());
            map.insert(safe_get_string_from_settings(&settings, "button_mapping", "b"), "b".to_owned());
            map.insert(safe_get_string_from_settings(&settings, "button_mapping", "x"), "x".to_owned());
            map.insert(safe_get_string_from_settings(&settings, "button_mapping", "y"), "y".to_owned());
            map.insert(safe_get_string_from_settings(&settings, "button_mapping", "bumper_left"), "bumper_left".to_owned());
            map.insert(safe_get_string_from_settings(&settings, "button_mapping", "bumper_right"), "bumper_right".to_owned());
            map.insert(safe_get_string_from_settings(&settings, "button_mapping", "trigger_left"), "trigger_left".to_owned());
            map.insert(safe_get_string_from_settings(&settings, "button_mapping", "trigger_right"), "trigger_right".to_owned());
            map
        },
        aimable_buttons: {
            let aimable_settings = settings.get_array("aimable_buttons.aimable").unwrap().iter().map(|x| x.to_string()).collect();
            for button in &aimable_settings {
                if !valid_ability_buttons.contains(button) {
                    alert_and_exit_on_invalid_settings(&format!("Invalid setting for aimable_buttons.\n\nCheck that {:?} is a, b, x, y, one of the triggers or bumpers.", button));
                }
            }
            aimable_settings 
        },
        action_distances: {
            let mut map = HashMap::<String, String>::new();
            for (key, value) in settings.get_table("action_distance").unwrap() {
                let value_str = value.to_string();
                if valid_ability_buttons.contains(&key) && valid_ability_ranges.contains(&value_str) {
                    map.insert(key, value_str);
                } else {
                    alert_and_exit_on_invalid_settings(&format!("Invalid setting for action_distance.\n\nCheck that {:?} is a valid button and {:?} is one of: \"close\" \"mid\" or \"far\" ", key, value_str));
                }
            }
            map
        },
        controller_settings: ControllerSettings {  
            controller_deadzone: safe_get_float_from_settings(&settings, "controller", "dead_zone_percentage") as f32,
            character_x_offset_px: safe_get_float_from_settings(&settings, "controller", "character_x_offset_px") as f32,
            character_y_offset_px: safe_get_float_from_settings(&settings, "controller", "character_y_offset_px") as f32,
            walk_circle_radius_px: safe_get_int_from_settings(&settings, "controller", "walk_circle_radius_px") as f32,
            close_circle_radius_px: safe_get_int_from_settings(&settings, "controller", "close_circle_radius_px") as f32,
            mid_circle_radius_px: safe_get_int_from_settings(&settings, "controller", "mid_circle_radius_px") as f32,
            far_circle_radius_px: safe_get_int_from_settings(&settings, "controller", "far_circle_radius_px") as f32,
            free_mouse_sensitivity_px: safe_get_int_from_settings(&settings, "controller", "free_mouse_sensitivity_px") as f32,
            controller_type: {
                let controller_type_string = safe_get_string_from_settings(&settings, "controller", "controller_type");
                if valid_controller_types.contains(&controller_type_string) {
                    controller_type_string
                } else { 
                    alert_and_exit_on_invalid_settings(&format!("Invalid setting for controller_type. Found: {:?}\n\nMust be one of: \"auto\" \"xbox\" or \"ps\" ", controller_type_string.as_str()));
                    panic!("This panic will never happen")
                }
            }
        },
    }
}


