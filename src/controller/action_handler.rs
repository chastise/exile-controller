use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time, collections::HashMap};

use crate::settings::{alert_and_exit_on_invalid_settings};

#[derive(PartialEq)]
pub enum ActionType {
    Press,
    Release,
}

pub struct ActionHandler {
    left_mouse_held: bool,
    middle_mouse_held: bool,
    right_mouse_held: bool,
    held_keys: HashMap<Key, String>,
    holding_left_click_for_action: bool,
}

impl Default for ActionHandler {
    fn default() -> Self {
        ActionHandler {
            left_mouse_held: false,
            middle_mouse_held: false,
            right_mouse_held: false,
            held_keys: HashMap::<Key, String>::with_capacity(20),
            holding_left_click_for_action: false,
        }
    }
}

impl ActionHandler {
    pub fn handle_action(&mut self, action_type: ActionType, action: String) {
        let action_lower = action.to_lowercase();
        let action_str = action_lower.as_str();
        match action_str {
            // Check for known "special" cases first.
            "altleftclick" => { 
                self.handle_action_with_modifier_key(action_type, "leftclick".to_owned(), "alt".to_owned(), 20, 10);
            },
            // Not a special case, fall through
            _ => { 
                if let Some(mouse_button) = self.match_mouse_str_to_button(action_str) {
                    // "LeftClick" => {self.handle_mouse_action(self.match_mouse_str_to_button(action.as_str()), action_type);},
                    // "MiddleClick" => {self.handle_mouse_action(self.match_mouse_str_to_button(action.as_str()), action_type);},
                    // "RightClick" => {self.handle_mouse_action(self.match_mouse_str_to_button(action.as_str()), action_type);},
                    self.handle_mouse_action(mouse_button, action_type);
                } else if let Some(key_button) = self.match_key_str_to_key(action_str) {
                    // self.handle_keypress_action(self.match_key_str_to_key(action.to_lowercase().as_str()), action_type, action);
                    self.handle_keypress_action(key_button, action_type, action);
                } else {
                    // TODO: We should probably check this on config load, but keeping it here for now because if bindable keys / actions changes, it'll happen here.
                    alert_and_exit_on_invalid_settings(&format!("Invalid action configured in settings: {:?}", action_str));
                    panic!("Invalid action configured in settings: {:?}", action_str); 
                }
                
            }
        }
    }

    pub fn holding_left_click_for_action(&self) -> bool {
        self.holding_left_click_for_action
    }

    fn match_mouse_str_to_button(&self, mouse_str: &str) -> Option<Button> {
        match mouse_str {
            "leftclick" => {Some(Button::Left)},
            "middleclick" => {Some(Button::Middle)},
            "rightclick" => {Some(Button::Right)},
            &_ => { None }
        }
    }

    fn match_key_str_to_key(&self, key_str: &str) -> Option<Key> {
        match key_str {            
            "f1" => {Some(Key::F1)},
            "f2" => {Some(Key::F2)},
            "f3" => {Some(Key::F3)},
            "f4" => {Some(Key::F4)},
            "f5" => {Some(Key::F5)},
            "f6" => {Some(Key::F6)},
            "f7" => {Some(Key::F7)},
            "f8" => {Some(Key::F8)},
            "f9" => {Some(Key::F9)},
            "f10" => {Some(Key::F10)},
            "f11" => {Some(Key::F11)},
            "f12" => {Some(Key::F12)},
            "a" => {Some(Key::KeyA)},
            "b" => {Some(Key::KeyB)},
            "c" => {Some(Key::KeyC)},
            "d" => {Some(Key::KeyD)},
            "e" => {Some(Key::KeyE)},
            "f" => {Some(Key::KeyF)},
            "g" => {Some(Key::KeyG)},
            "h" => {Some(Key::KeyH)},
            "i" => {Some(Key::KeyI)},
            "j" => {Some(Key::KeyJ)},
            "k" => {Some(Key::KeyK)},
            "l" => {Some(Key::KeyL)},
            "m" => {Some(Key::KeyM)},
            "n" => {Some(Key::KeyN)},
            "o" => {Some(Key::KeyO)},
            "p" => {Some(Key::KeyP)},
            "q" => {Some(Key::KeyQ)},
            "r" => {Some(Key::KeyR)},
            "s" => {Some(Key::KeyS)},
            "t" => {Some(Key::KeyT)},
            "u" => {Some(Key::KeyU)},
            "v" => {Some(Key::KeyV)},
            "w" => {Some(Key::KeyW)},
            "x" => {Some(Key::KeyX)},
            "y" => {Some(Key::KeyY)},
            "z" => {Some(Key::KeyZ)},
            "0" => {Some(Key::Num0)},
            "1" => {Some(Key::Num1)},
            "2" => {Some(Key::Num2)},
            "3" => {Some(Key::Num3)},
            "4" => {Some(Key::Num4)},
            "5" => {Some(Key::Num5)},
            "6" => {Some(Key::Num6)},
            "7" => {Some(Key::Num7)},
            "8" => {Some(Key::Num8)},
            "9" => {Some(Key::Num9)},
            "`" => {Some(Key::BackQuote)},
            "[" => {Some(Key::LeftBracket)},
            "]" => {Some(Key::RightBracket)},
            ";" => {Some(Key::SemiColon)},
            "/" => {Some(Key::Slash)},
            "," => {Some(Key::Comma)},
            "=" => {Some(Key::Equal)},
            "escape" => {Some(Key::Escape)},
            "space" => {Some(Key::Space)},
            "tab" => {Some(Key::Tab)},
            "backspace" => {Some(Key::Backspace)},
            "delete" => {Some(Key::Delete)},
            "uparrow" => {Some(Key::UpArrow)},
            "downarrow" => {Some(Key::DownArrow)},
            "leftarrow" => {Some(Key::LeftArrow)},
            "rightarrow" => {Some(Key::RightArrow)},
            "alt" => {Some(Key::Alt)},
            "shift" => {Some(Key::ShiftLeft)},
            "control" => {Some(Key::ControlLeft)},
            &_ => { None }
        }
    }

    pub fn move_mouse(&self, x: f64, y: f64) {
        rdev_send_event(&EventType::MouseMove { x, y });
    }

    fn handle_mouse_action(&mut self, mouse_button: Button, action: ActionType) {
        // Action handler tracks held mouse button state to avoid safely spamming events
        match mouse_button {
            Button::Left => {
                if action == ActionType::Press && !self.left_mouse_held {
                    self.left_mouse_held = true;
                    rdev_send_event(&EventType::ButtonPress(mouse_button))
                } else if action == ActionType::Release && self.left_mouse_held {
                    self.left_mouse_held = false;
                    rdev_send_event(&EventType::ButtonRelease(mouse_button))
                }
            },
            Button::Middle => {
                if action == ActionType::Press && !self.middle_mouse_held {
                    self.middle_mouse_held = true;
                    rdev_send_event(&EventType::ButtonPress(mouse_button))
                } else if action == ActionType::Release && self.middle_mouse_held {
                    self.middle_mouse_held = false;
                    rdev_send_event(&EventType::ButtonRelease(mouse_button))
                }
            },
            Button::Right => {
                if action == ActionType::Press && !self.right_mouse_held {
                    self.right_mouse_held = true;
                    rdev_send_event(&EventType::ButtonPress(mouse_button))
                } else if action == ActionType::Release && self.right_mouse_held {
                    self.right_mouse_held = false;
                    rdev_send_event(&EventType::ButtonRelease(mouse_button))
                }
            },
            _ => ()
        }
    }

    fn handle_keypress_action(&mut self, keypress: Key, action: ActionType, action_string: String) {
        if action == ActionType::Press {
            if !self.held_keys.contains_key(&keypress) {
                rdev_send_event(&EventType::KeyPress(keypress));
                self.held_keys.insert(keypress, action_string);
            }
        } else if action == ActionType::Release {
            if self.held_keys.contains_key(&keypress) {
                rdev_send_event(&EventType::KeyRelease(keypress));
                self.held_keys.remove(&keypress);
            }
        }
    }

    fn handle_action_with_modifier_key(&mut self, action_type: ActionType, action: String, modifier: String, delay_ms_before: u64, delay_ms_after: u64) {
        // We can trust this lookup so long as we only call this function with known inputs. 
        // If inputs are user-specified, we must refactor to check them.
        let modifier_key = self.match_key_str_to_key(&modifier).unwrap();
        let modifier_already_held = self.held_keys.contains_key(&modifier_key);
        let mut action_string = modifier.to_owned();
        action_string.push_str(action.as_str());

        if action_type == ActionType::Press {
            if !modifier_already_held {
                self.handle_keypress_action(modifier_key, ActionType::Press, action_string.clone());
                thread::sleep(time::Duration::from_millis(delay_ms_before));
            }

            if let Some(mouse_button) = self.match_mouse_str_to_button(&action) {
                // println!("pushing mouse action {:?}", &action);
                self.handle_mouse_action(mouse_button, ActionType::Press);
                self.holding_left_click_for_action = true;
            } else if let Some(key_button) = self.match_key_str_to_key(&action) {
                // This will not press again if another controller button is already holding this key
                self.handle_keypress_action(key_button, ActionType::Press, action.clone());
            }

            if !modifier_already_held {
                thread::sleep(time::Duration::from_millis(delay_ms_after));
                self.handle_keypress_action(modifier_key, ActionType::Release, action_string);
            }
        } 
        else if action_type == ActionType::Release {
            if let Some(mouse_button) = self.match_mouse_str_to_button(&action) {
                self.handle_mouse_action(mouse_button, ActionType::Release);
                self.holding_left_click_for_action = false;
            } else if let Some(key_button) = self.match_key_str_to_key(&action) {
                // This will unpress even if another controller button is already holding this key
                self.handle_keypress_action(key_button, ActionType::Release, action.clone());
            }
        } 
    }

    pub fn is_ability_key_held(&self) -> bool {
        let mut is_holding = false;
        if self.middle_mouse_held || self.right_mouse_held {
            is_holding = true;
        } 
        for key in self.held_keys.keys() {
            match key {
                Key::KeyQ => {is_holding = true;},
                Key::KeyW => {is_holding = true;},
                Key::KeyE => {is_holding = true;},
                Key::KeyR => {is_holding = true;},
                Key::KeyT => {is_holding = true;},
                _ => (),
            }
        }
        is_holding
    }
    pub fn get_held_ability_actions(&self) -> Vec<String> {
        let mut held_ability_actions = Vec::<String>::new();
        if self.middle_mouse_held {held_ability_actions.push("MiddleClick".to_owned());}
        if self.right_mouse_held {held_ability_actions.push("RightClick".to_owned());}
        if self.held_keys.contains_key(&Key::KeyQ) {held_ability_actions.push("q".to_owned());}
        if self.held_keys.contains_key(&Key::KeyW) {held_ability_actions.push("w".to_owned());}
        if self.held_keys.contains_key(&Key::KeyE) {held_ability_actions.push("e".to_owned());}
        if self.held_keys.contains_key(&Key::KeyR) {held_ability_actions.push("r".to_owned());}
        if self.held_keys.contains_key(&Key::KeyT) {held_ability_actions.push("t".to_owned());}
        held_ability_actions
    }
}

fn rdev_send_event(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
}