use config::Config;
use std::collections::HashMap;

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

// impl Default for OverlaySettings {
//     fn default() -> Self {
//         OverlaySettings {
//             screen_height: 1080,
//             screen_width: 1920,
//             show_crosshair: true,
//             show_buttons: true,
//         }
//     }
// }

pub struct ButtonMappingSettings {
    a: String,
    b: String,
    x: String,
    y: String,
}

pub struct ControllerSettings {
    controller_deadzone: f32,
}

impl ControllerSettings {
    pub fn controller_deadzone(&self) -> f32 {self.controller_deadzone}
}

pub struct ApplicationSettings {
    overlay_settings: OverlaySettings,
    button_mapping_settings: ButtonMappingSettings,
    controller_settings: ControllerSettings,
}

impl ApplicationSettings {
    pub fn overlay_settings(&self) -> OverlaySettings {self.overlay_settings.clone()}

    pub fn button_mapping_settings(&self) -> &ButtonMappingSettings {&self.button_mapping_settings}
    
    pub fn controller_settings(&self) -> &ControllerSettings {&self.controller_settings}
}

pub fn load_settings() -> ApplicationSettings {
    let settings = Config::builder()
                    .add_source(config::File::with_name("settings.toml"))
                    .build()
                    .unwrap_or_else(|error| {
                        panic!("config failed to load. Error: {error}")
                    });
    // println!("{:?}", 
    //         settings.clone()
    //                 .try_deserialize::<HashMap<String, HashMap<String, String>>>()
    //                 .unwrap()
    // );
    
    return ApplicationSettings {
        overlay_settings: OverlaySettings {
                            screen_height: settings.get_int("overlay.screen_height").unwrap() as f32,
                            screen_width: settings.get_int("overlay.screen_width").unwrap() as f32,
                            show_crosshair: settings.get_bool("overlay.show_crosshair").unwrap(),
                            show_buttons: settings.get_bool("overlay.show_buttons").unwrap(),
                        },
        button_mapping_settings: ButtonMappingSettings { 
            a: settings.get_string("button_mapping.a").unwrap(),
            b: settings.get_string("button_mapping.b").unwrap(),
            x: settings.get_string("button_mapping.x").unwrap(),
            y: settings.get_string("button_mapping.y").unwrap(),
         },
        controller_settings: ControllerSettings {  
            controller_deadzone: settings.get_float("controller.dead_zone_percentage").unwrap() as f32,
        },
    }
}
