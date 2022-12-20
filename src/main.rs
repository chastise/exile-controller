use std::{thread::{sleep}, time::Duration};

mod settings;
mod controller;
use controller::{input, action_handler};



fn main() {
    println!("Loading settings.toml...");
    let application_settings = settings::load_settings();
    println!("Configured resolution: height:{} | width:{}", 
                application_settings.overlay_settings().screen_height(), 
                application_settings.overlay_settings().screen_width(),
            );
    println!("Starting gamepad manager.");    
    let gamepad_trigger_threshold = 0.8 as f32; // For quick tweaking
    let mut gamepad_manager = input::load_gamepad_manager(gamepad_trigger_threshold, 
                                                                                    application_settings.controller_settings().controller_deadzone());
    println!("Success! Starting main loop!");
    loop {
        gamepad_manager.read_latest_input();
        sleep(Duration::from_millis(5 as u64));
    }
}