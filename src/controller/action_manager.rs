use std::collections::HashMap;

use egui::PointerState;

use crate::settings::{ControllerSettings, ApplicationSettings};

use super::input::{ControllerButton, AnalogStick};
use super::action_handler::{ActionHandler, ActionType};


struct PlannedAction {
    name: String,
    just_pressed: bool,
    aimable: bool,
}

pub struct ActionManager {
    action_handler: ActionHandler,
    planned_actions: Vec<PlannedAction>,
    settings: ApplicationSettings,
    walking: bool,
    walking_angle: f32,
    aiming: bool,
    aiming_angle: f32,
    aiming_stick_direction: Vec<f32>,
    
}

impl ActionManager {
    pub fn initialize (application_settings: ApplicationSettings) -> ActionManager {
        ActionManager {
            action_handler: ActionHandler::default(),
            planned_actions: Vec::<PlannedAction>::with_capacity(application_settings.button_mapping_settings().keys().count()), 
            settings: application_settings,
            walking: false,
            walking_angle: 0.0,
            aiming: false,
            aiming_angle: 0.0,
            aiming_stick_direction: vec![0.0, 0.0],
        }
    }



    pub fn process_input_buttons(&mut self, named_controller_buttons: HashMap<String, &mut ControllerButton>) {
        // add to planned_actions by iterating through pressed and unpressed buttons
        for (name, button) in named_controller_buttons {
            // TODO: add holdable flag management
            if button.just_pressed && button.just_unpressed {
                panic!("This should never be possible!")
            }
            if button.just_pressed {
                println!("Just pressed {:?} ", name);
                let can_be_aimed = self.settings.aimable_buttons().contains(&name);
                self.planned_actions.push(PlannedAction {name: name, 
                                                        just_pressed: true, 
                                                        aimable: can_be_aimed,
                                                    });
                button.just_pressed = false;
            }
            else if button.just_unpressed {
                println!("Just unpressed {:?} ", name);
                let can_be_aimed = self.settings.aimable_buttons().contains(&name);
                self.planned_actions.push(PlannedAction {name: name, 
                                                        just_pressed: false, 
                                                        aimable: can_be_aimed,
                });
                button.just_unpressed = false;
            }
        }

    }
    pub fn process_input_analogs(&mut self, left_stick: AnalogStick, right_stick: AnalogStick) {
        // walking
        if !left_stick.joystick_in_deadzone() {
            self.walking = true;
            self.walking_angle = left_stick.stick_angle();
        } else {
            self.walking = false;
        }
        // aiming. If not aiming, aim_angle == walk angle
        if !right_stick.joystick_in_deadzone() {
            self.aiming = true;
            self.aiming_angle = right_stick.stick_angle();
            self.aiming_stick_direction = right_stick.stick_direction();
        }else {
            self.aiming = false;
        }
        // if not aiming and not walking, aim pos is wherever mouse is
    }

    pub fn handle_character_actions(&mut self, ctx: &egui::Context) {
        /* 
        TODO: figure out how to do custom action distance for specific buttons
        */

        /*
        holding_aimable = holding an aimable button, per the configs
        1.  if (holding_aimable and moving and aiming)
                - set mouse position
                - maybe wait slightly for the mouse to be in the right place
            ((if the above is false)) we assume we're already in the right position, so we don't move the cursor

        2. iterate through planned actions, calling handle_action on each
        
        3.  if not (holding_aimable and moving and aiming)
                if not moving and aiming (free-aiming, possibly holding_aimable)
                    - handle_button left mouse up
                    - move the mouse by a small amount in the aim direction
                else if not moving 
                    - handle_button left mouse up
                else (we're moving and either aiming but not holding_aimable or holding but not aiming)
                    - 
                    if not holding_aimable
                        - handle_button left mouse down 
                        - drag the mouse along the walk circle
           
            else if holding_aimable && not aiming
                - move the mouse to the walk circle
        
        */

        // let's try clicking a button
        while let Some(planned_action) = self.planned_actions.pop() {
            let key_name = self.settings.button_mapping_settings().get(&planned_action.name).unwrap().to_string();
            println!("{key_name}");
            if planned_action.just_pressed {
                self.action_handler.handle_action(ActionType::PRESS, key_name)
            } else {
                self.action_handler.handle_action(ActionType::RELEASE, key_name)
            }
            
        }

        // if aiming and not moving!
        if self.aiming && !self.walking {
            let (new_x_pos, new_y_pos) = self.get_free_move_update(ctx);
            self.action_handler.move_mouse(new_x_pos, new_y_pos);
        }

        // if moving!
        if self.walking {
            let (new_x, new_y) = self.get_radial_location(self.settings.controller_settings().walk_circle_radius_px(), self.walking_angle);
            self.action_handler.move_mouse(new_x as f64, new_y as f64);
            self.action_handler.handle_action(ActionType::PRESS, "LeftClick".to_string());
        } else {
            self.action_handler.handle_action(ActionType::RELEASE, "LeftClick".to_string());
        }
  
    }

    fn get_radial_location(&self, circle_radius: f32, angle: f32) -> (f32, f32) {
        let screen_adjustment_x = angle.cos() * circle_radius;
        let screen_adjustment_y = angle.sin() * circle_radius;
        let new_x = self.settings.overlay_settings().screen_width()/2.0 + screen_adjustment_x + self.settings.controller_settings().character_x_offset_px();
        let new_y = self.settings.overlay_settings().screen_height()/2.0 - screen_adjustment_y - self.settings.controller_settings().character_y_offset_px();
        (new_x, new_y)
    }

    fn get_attack_circle_radius(&self, key_name: String) -> f32 {
        if self.settings.action_distances().contains_key(&key_name) {
            let key_distance = self.settings.action_distances().get(&key_name).unwrap().to_owned();
            match key_distance.as_str() {
                "close" => {
                    return self.settings.controller_settings().close_circle_radius_px();
                },
                "mid" => {
                    return self.settings.controller_settings().mid_circle_radius_px();
                },
                "far" => {
                    return self.settings.controller_settings().far_circle_radius_px();
                },
                // If none of the above, config typo, fail gracefully with walking dist
                _ => {return self.settings.controller_settings().walk_circle_radius_px();}
            }
        } else {
            return self.settings.controller_settings().walk_circle_radius_px();
        }
    }

    fn get_free_move_update(&self, ctx: &egui::Context) -> (f64, f64){
        let screen_adjustment_x = self.aiming_stick_direction[0] * self.settings.controller_settings().free_mouse_sensitivity_px();
        let screen_adjustment_y = -1.0 * self.aiming_stick_direction[1] * self.settings.controller_settings().free_mouse_sensitivity_px();
        
        // There is a chance that there _is_ no mouse position.
        match ctx.input().pointer.hover_pos() {
            Some(position) => ((position.x + screen_adjustment_x) as f64, (position.y + screen_adjustment_y) as f64),
            // Should we just panic here?
            None => (0.0f64, 0.0f64),
        }
    }
}





