use std::collections::HashMap;

use crate::game_window_tracker::GameWindowTracker;
use crate::settings:: ApplicationSettings;

use super::input::{ControllerButton, AnalogStick};
use super::action_handler::{ActionHandler, ActionType};

#[derive(PartialEq)]
pub enum ActionDistance {
    Close,
    Mid,
    Far,
    None,
}

struct PlannedAction {
    name: String,
    just_pressed: bool,
    aimable: bool,
    distance: ActionDistance,
}

pub struct ActionManager {
    action_handler: ActionHandler,
    planned_actions: Vec<PlannedAction>,
    game_window_tracker: GameWindowTracker,
    settings: ApplicationSettings,
    holding_walk: bool,
    walking_angle: f32,
    holding_aim: bool,
    aiming_angle: f32,
    aiming_stick_direction: Vec<f32>,
    aiming_stick_pull_amount: f32,
    holding_ability: bool,
    
}

impl ActionManager {
    pub fn initialize (application_settings: ApplicationSettings, game_window_tracker: GameWindowTracker) -> ActionManager {
        ActionManager {
            action_handler: ActionHandler::default(),
            planned_actions: Vec::<PlannedAction>::with_capacity(application_settings.button_mapping_settings().keys().count()), 
            game_window_tracker: game_window_tracker,
            settings: application_settings,
            holding_walk: false,
            walking_angle: 0.0,
            holding_aim: false,
            aiming_angle: 0.0,
            aiming_stick_direction: vec![0.0, 0.0],
            aiming_stick_pull_amount: 0.0,
            holding_ability: false,

        }
    }

    pub fn process_input_buttons(&mut self, named_controller_buttons: HashMap<String, &mut ControllerButton>) {
        for (action_name, button) in named_controller_buttons {
            if button.just_pressed && button.just_unpressed {
                panic!("This should never be possible!")
            }
            if button.just_pressed {
                println!("Just pressed {:?} ", action_name);
                let can_be_aimed = self.settings.aimable_buttons().contains(&action_name);
                let action_distance = self.get_ability_action_distance(&action_name);
                self.planned_actions.push(PlannedAction {name: action_name, 
                                                        just_pressed: true, 
                                                        aimable: can_be_aimed,
                                                        distance: action_distance,
                                                    });
                button.just_pressed = false;
            }
            else if button.just_unpressed {
                println!("Just unpressed {:?} ", action_name);
                self.planned_actions.push(PlannedAction {name: action_name, 
                                                        just_pressed: false, 
                                                        aimable: false, // Don't need this for unpress
                                                        distance: ActionDistance::None, // Don't need this for unpress
                });
                button.just_unpressed = false;
            }
        }

    }
    pub fn process_input_analogs(&mut self, left_stick: AnalogStick, right_stick: AnalogStick) {
        if !left_stick.joystick_in_deadzone() {
            self.holding_walk = true;
            self.walking_angle = left_stick.stick_angle();
        } else {
            self.holding_walk = false;
        }
        if !right_stick.joystick_in_deadzone() {
            self.holding_aim = true;
            self.aiming_angle = right_stick.stick_angle();
            self.aiming_stick_direction = right_stick.stick_direction();
            self.aiming_stick_pull_amount = right_stick.joystick_pull_amount_smoothed();
        } else {
            self.holding_aim = false;
            self.aiming_stick_pull_amount = 0.0_f32;
        }
    }

    pub fn update_window_tracker (&mut self) {self.game_window_tracker.update_window_tracker()}

    pub fn handle_character_actions(&mut self, ctx: &egui::Context) {
        let mut set_cursor = false;

        // Execute planned actions
        while let Some(planned_action) = self.planned_actions.pop() {
            let key_name = self.settings.button_mapping_settings().get(&planned_action.name).unwrap().to_string();
            if key_name == "" {continue} // Empty string is how we set keymaps to not taking any action.
            // println!("{key_name}");
            if planned_action.just_pressed {
                if planned_action.aimable {
                    if self.holding_walk && self.holding_aim {
                        let (new_x, new_y) = self.get_radial_location(self.get_attack_circle_radius(planned_action.distance), self.aiming_angle);
                        self.safe_move_mouse(new_x as f64, new_y as f64);
                        set_cursor = true;
                    } else if self.holding_walk && !self.holding_aim {
                        let (new_x, new_y) = self.get_radial_location(self.get_attack_circle_radius(planned_action.distance), self.walking_angle);
                        self.safe_move_mouse(new_x as f64, new_y as f64);
                        set_cursor = true;
                    }
                    // todo probably inject a delay for the two above
                } else if planned_action.distance != ActionDistance::None && self.holding_walk {
                        let (new_x, new_y) = self.get_radial_location(self.get_attack_circle_radius(planned_action.distance), self.walking_angle);
                        self.safe_move_mouse(new_x as f64, new_y as f64);
                        set_cursor = true;
                }
                self.action_handler.handle_action(ActionType::Press, key_name);
            } else {
                self.action_handler.handle_action(ActionType::Release, key_name);
            }
        }

        // if we're holding an ability but didn't just press something, we need the cursor to swivel if we're also holding a stick.
        // This block accomplishes that swivel, prioritizing aiming_angle if any held buttons are aimable, and targeting the longest distance
        // If none of the held abilities are aimable or have preset distances, this causes the cursor to snap to the walking circle if held.
        // If none of the held abilities are aimable or have preset distances, AND we're not walking, this lets you free-aim the ability with right stick
        self.holding_ability = self.action_handler.is_ability_key_held();
        

        if self.holding_ability && self.holding_walk {
            let held_ability_actions: Vec<String> = self.action_handler.get_held_ability_actions()
                                                                        .iter()
                                                                        .map(|key| self.settings.ability_mapping_settings().get(key).unwrap().to_owned())
                                                                        .collect();
            
            // Check if any of the held actions are aimable, even if they have no action distance set
            let mut some_held_action_aimable = false;
            for action in held_ability_actions.clone() {
                println!("checking if {:?} is aimable", action);
                if self.settings.aimable_buttons().contains(&action) {
                    some_held_action_aimable = true;
                }
            }

            // Of the abilities with an action distance, check for the farthest distance.
            let held_abilities_with_action_distance_set = held_ability_actions.into_iter()
                                                                                    .filter(|action| self.get_ability_action_distance(action) != ActionDistance::None);
            let mut chosen_distance =  0.0;
            for (_action, distance) in held_abilities_with_action_distance_set.map(|action| (action.to_owned(), self.get_attack_circle_radius(self.get_ability_action_distance(&action)))) {
                if distance > chosen_distance {
                    chosen_distance = distance;
                }
            }
            // println!("{:?},,, {:?}", some_held_action_aimable, chosen_distance);
            if chosen_distance == 0.0 {
                // no held ability had a preset distance, use walking distance
                chosen_distance = self.settings.controller_settings().walk_circle_radius_px();
            }
            if some_held_action_aimable && self.holding_aim {
                let (new_x, new_y) = self.get_radial_location(chosen_distance, self.aiming_angle);
                self.safe_move_mouse(new_x as f64, new_y as f64);
                set_cursor = true;
            } else {
                let (new_x, new_y) = self.get_radial_location(chosen_distance, self.walking_angle);
                self.safe_move_mouse(new_x as f64, new_y as f64);
                set_cursor = true;
            }
        }
        
        // if aiming and not moving!
        if self.holding_aim && !self.holding_walk {
            let (new_x_pos, new_y_pos) = self.get_free_move_update(ctx);
            self.safe_move_mouse(new_x_pos, new_y_pos);
            set_cursor = true;
        } 

        // if moving!
        if self.holding_walk && !set_cursor {
            let (new_x, new_y) = self.get_radial_location(self.settings.controller_settings().walk_circle_radius_px(), self.walking_angle);
            self.safe_move_mouse(new_x as f64, new_y as f64);
        }
        if self.holding_walk {
            self.action_handler.handle_action(ActionType::Press, "leftclick".to_string());
        } else { // TODO: How does this work with held move skills? Might need to add "if not holding walk"
            self.action_handler.handle_action(ActionType::Release, "leftclick".to_string());
        }
  
    }

    fn safe_move_mouse(&self, new_x: f64, new_y: f64) {
        if self.game_window_tracker.windowed_mode() {
            let (new_safe_x, new_safe_y) = self.get_window_bounded_position(new_x, new_y);
            self.action_handler.move_mouse(new_safe_x, new_safe_y);
        } else {
            self.action_handler.move_mouse(new_x as f64, new_y as f64);
        }
    }
    
    fn get_window_bounded_position(&self, new_x: f64, new_y: f64) -> (f64, f64) {
        let mut return_x = new_x;
        let mut return_y = new_y;

        #[cfg(target_os = "linux")]
        let (title_bar_height, window_shadow_amount) = (2.0, 2.0); // magic numbers for linux
        #[cfg(target_os = "windows")]
        let (title_bar_height, window_shadow_amount) = (32.0, 10.0); // magic numbers, may only be correct on windows

        let min_x_pos = (self.game_window_tracker.window_pos_x() + window_shadow_amount) as f64;
        let min_y_pos = (self.game_window_tracker.window_pos_y() + title_bar_height) as f64;
        let max_x_pos = (self.game_window_tracker.window_pos_x() + self.game_window_tracker.game_window_width() - window_shadow_amount) as f64;
        let max_y_pos = (self.game_window_tracker.window_pos_y() + self.game_window_tracker.game_window_height()- window_shadow_amount) as f64;
        if new_x < min_x_pos {
            return_x = min_x_pos;
        } else if new_x > max_x_pos {
            return_x = max_x_pos;
        }
        if new_y < min_y_pos {
            return_y = min_y_pos;
        } else if new_y > max_y_pos {
            return_y = max_y_pos;
        }
        (return_x, return_y)
    }


    fn get_radial_location(&self, circle_radius: f32, angle: f32) -> (f32, f32) {
        let screen_adjustment_x = angle.cos() * circle_radius;
        let screen_adjustment_y = angle.sin() * circle_radius;
        let new_x = self.game_window_tracker.game_window_width()/2.0 + screen_adjustment_x + self.settings.controller_settings().character_x_offset_px() + self.game_window_tracker.window_pos_x();
        let new_y = self.game_window_tracker.game_window_height()/2.0 - screen_adjustment_y - self.settings.controller_settings().character_y_offset_px() + self.game_window_tracker.window_pos_y();
        (new_x, new_y)
    }

    fn get_attack_circle_radius(&self, action_distance: ActionDistance) -> f32 {
        match action_distance {
            ActionDistance::Close => {self.settings.controller_settings().close_circle_radius_px()},
            ActionDistance::Mid => {self.settings.controller_settings().mid_circle_radius_px()},
            ActionDistance::Far => {self.settings.controller_settings().far_circle_radius_px()},
            _ => {self.settings.controller_settings().walk_circle_radius_px()}
        }
    }

    fn get_free_move_update(&self, ctx: &egui::Context) -> (f64, f64){
        let screen_adjustment_x = self.aiming_stick_direction[0] * self.settings.controller_settings().free_mouse_sensitivity_px() ;
        let screen_adjustment_y = -1.0 * self.aiming_stick_direction[1] * self.settings.controller_settings().free_mouse_sensitivity_px();
        // There is a chance that there _is_ no mouse position.
        match ctx.input().pointer.hover_pos() {
            Some(position) => ((position.x + screen_adjustment_x) as f64, (position.y + screen_adjustment_y) as f64),
            // Should we just panic here?
            None => (0.0f64, 0.0f64),
        }
    }

    fn get_ability_action_distance(&self, name: &String) -> ActionDistance {
        if self.settings.action_distances().contains_key(name) {
            // println!("herp {:?}", name);
            match self.settings.action_distances().get(name).unwrap().as_str() {
                "close" => {ActionDistance::Close},
                "mid" => {ActionDistance::Mid},
                "far" => {ActionDistance::Far},
                _ => {ActionDistance::None}
            }
        } else {
            // println!("derp {:?}", name);
            ActionDistance::None}
    } 
}





