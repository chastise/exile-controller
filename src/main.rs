#![cfg_attr(
    all(
      target_os = "windows",
      not(debug_assertions),
    ),
    windows_subsystem = "windows"
  )]
// ^ Disables terminal when building a release on windows


mod settings;
mod controller;
use controller::{input, action_manager};
mod overlay;
use overlay::game_overlay;

use crate::game_window_tracker::GameWindowTracker;
mod game_window_tracker;

fn main() {
    println!("Loading settings.toml...");
    let application_settings = settings::load_settings();
    println!("Configured resolution: height:{} | width:{}", 
                application_settings.overlay_settings().screen_height(), 
                application_settings.overlay_settings().screen_width(),
            );

    println!("Starting gamepad manager.");
    let gamepad_manager = input::load_gamepad_manager(application_settings.controller_settings().controller_deadzone());
    println!("Initializing action handler."); 
    let action_manager = action_manager::ActionManager::initialize(application_settings.clone());
    let game_window_tracker = GameWindowTracker::new(application_settings.clone());

    println!("Starting overlay");
    game_overlay::start_overlay(application_settings.overlay_settings(), application_settings.controller_settings(), gamepad_manager, action_manager, game_window_tracker);
}