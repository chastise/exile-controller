use std::collections::HashMap;

use config::Config;

#[derive(Clone)]
pub struct OverlaySettings {
    screen_height: f32,
    screen_width: f32,
    show_crosshair: bool,
    show_buttons: bool,
}

impl OverlaySettings {
    pub fn screen_height(&self) -> f32 {self.screen_height}
    pub fn screen_width(&self) -> f32 {self.screen_width}
    pub fn show_crosshair(&self) -> bool {self.show_crosshair}
    pub fn show_buttons(&self) -> bool {self.show_buttons}
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

pub fn load_settings() -> ApplicationSettings {
    let settings = Config::builder()
                    .add_source(config::File::with_name("settings.toml"))
                    .build()
                    .unwrap_or_else(|error| {
                        panic!("config failed to load. Error: {error}")
                    });
    let valid_ability_buttons = ["a", "b", "x", "y", "bumper_left", "bumper_right", "trigger_left", "trigger_right"].map(|x| x.to_string());
    let valid_ability_ranges = ["close", "mid", "far"].map(|x| x.to_string());
    ApplicationSettings {
        overlay_settings: OverlaySettings {
                            screen_height: settings.get_int("overlay.screen_height").unwrap() as f32,
                            screen_width: settings.get_int("overlay.screen_width").unwrap() as f32,
                            show_crosshair: settings.get_bool("overlay.show_crosshair").unwrap(),
                            show_buttons: settings.get_bool("overlay.show_buttons").unwrap(),
                        },

        button_mapping_settings: {
            let mut map = HashMap::<String, String>::new(); 
            map.insert("dpad_up".to_owned(), settings.get_string("button_mapping.dpad_up").unwrap());
            map.insert("dpad_down".to_owned(), settings.get_string("button_mapping.dpad_down").unwrap());
            map.insert("dpad_left".to_owned(), settings.get_string("button_mapping.dpad_left").unwrap());
            map.insert("dpad_right".to_owned(), settings.get_string("button_mapping.dpad_right").unwrap());
            map.insert("start".to_owned(), settings.get_string("button_mapping.start").unwrap());
            map.insert("back".to_owned(), settings.get_string("button_mapping.back").unwrap());
            map.insert("a".to_owned(), settings.get_string("button_mapping.a").unwrap());
            map.insert("b".to_owned(), settings.get_string("button_mapping.b").unwrap());
            map.insert("x".to_owned(), settings.get_string("button_mapping.x").unwrap());
            map.insert("y".to_owned(), settings.get_string("button_mapping.y").unwrap());
            map.insert("left_analog".to_owned(), settings.get_string("button_mapping.left_analog").unwrap());
            map.insert("right_analog".to_owned(), settings.get_string("button_mapping.right_analog").unwrap());
            map.insert("bumper_left".to_owned(), settings.get_string("button_mapping.bumper_left").unwrap());
            map.insert("bumper_right".to_owned(), settings.get_string("button_mapping.bumper_right").unwrap());
            map.insert("trigger_left".to_owned(), settings.get_string("button_mapping.trigger_left").unwrap());
            map.insert("trigger_right".to_owned(), settings.get_string("button_mapping.trigger_right").unwrap());
            map
        },
        ability_mapping_settings: {
            let mut map = HashMap::<String, String>::new();
            map.insert(settings.get_string("button_mapping.a").unwrap(), "a".to_owned());
            map.insert(settings.get_string("button_mapping.b").unwrap(), "b".to_owned());
            map.insert(settings.get_string("button_mapping.x").unwrap(), "x".to_owned());
            map.insert(settings.get_string("button_mapping.y").unwrap(), "y".to_owned());
            map.insert(settings.get_string("button_mapping.bumper_left").unwrap(), "bumper_left".to_owned());
            map.insert(settings.get_string("button_mapping.bumper_right").unwrap(), "bumper_right".to_owned());
            map.insert(settings.get_string("button_mapping.trigger_left").unwrap(), "trigger_left".to_owned());
            map.insert(settings.get_string("button_mapping.trigger_right").unwrap(), "trigger_right".to_owned());
            map
        },
        aimable_buttons: {
            settings.get_array("aimable_buttons.aimable").unwrap().iter().map(|x| x.to_string()).filter(|x| valid_ability_buttons.contains(x)).collect()
        },
        action_distances: {
            let mut map = HashMap::<String, String>::new();
            for (key, value) in settings.get_table("action_distance").unwrap() {
                let value_str = value.to_string();
                if valid_ability_buttons.contains(&key) && valid_ability_ranges.contains(&value_str) {
                    map.insert(key, value_str);
                } else {
                    println!("invalid config");
                }
            }
            map
        },
        controller_settings: ControllerSettings {  
            controller_deadzone: settings.get_float("controller.dead_zone_percentage").unwrap() as f32,
            character_x_offset_px: settings.get_float("controller.character_x_offset_px").unwrap() as f32,
            character_y_offset_px: settings.get_float("controller.character_y_offset_px").unwrap() as f32,
            walk_circle_radius_px: settings.get_int("controller.walk_circle_radius_px").unwrap() as f32,
            close_circle_radius_px: settings.get_int("controller.close_circle_radius_px").unwrap() as f32,
            mid_circle_radius_px: settings.get_int("controller.mid_circle_radius_px").unwrap() as f32,
            far_circle_radius_px: settings.get_int("controller.far_circle_radius_px").unwrap() as f32,
            free_mouse_sensitivity_px: settings.get_int("controller.free_mouse_sensitivity_px").unwrap() as f32,
        },

    }
}


