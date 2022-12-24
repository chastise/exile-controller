//use std::{thread::{sleep}, time::Duration};

mod settings;
mod controller;
use controller::{input, action_manager};
mod overlay;
use overlay::game_overlay;


fn main() {
    println!("Loading settings.toml...");
    let application_settings = settings::load_settings();
    println!("Configured resolution: height:{} | width:{}", 
                application_settings.overlay_settings().screen_height(), 
                application_settings.overlay_settings().screen_width(),
            );

    println!("Starting gamepad manager.");    
    let gamepad_trigger_threshold = 0.8 as f32; // For quick tweaking
    let gamepad_manager = input::load_gamepad_manager(gamepad_trigger_threshold, 
                                                                                    application_settings.controller_settings().controller_deadzone());
    println!("Initializing action handler."); 
    let game_action_handler = action_manager::ActionManager::initialize(application_settings.clone());

    // println!("Success! Starting main loop!");
    // loop {
    //     gamepad_manager.read_latest_input();
    //     game_action_handler.process_input_buttons(gamepad_manager.controller_state.get_all_buttons());
    //     game_action_handler.process_input_analogs(gamepad_manager.controller_state.get_left_analog_stick(), 
    //                                                 gamepad_manager.controller_state.get_right_analog_stick());
    //     game_action_handler.handle_character_actions();
    //     sleep(Duration::from_millis(5 as u64));
    // }

    println!("Starting overlay");
    game_overlay::start_overlay(application_settings.overlay_settings(), application_settings.controller_settings(), gamepad_manager, game_action_handler);
}