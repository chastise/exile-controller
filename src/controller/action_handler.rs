use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time, collections::HashMap};

#[derive(PartialEq)]
pub enum ActionType {
    Press,
    Release,
}

pub struct ActionHandler {
    left_mouse_held: bool,
    middle_mouse_held: bool,
    right_mouse_held: bool,
    held_keys: HashMap<Key, String>
}

impl Default for ActionHandler {
    fn default() -> Self {
        ActionHandler {
            left_mouse_held: false,
            middle_mouse_held: false,
            right_mouse_held: false,
            held_keys: HashMap::<Key, String>::with_capacity(20),
        }
    }
}

impl ActionHandler {
    pub fn handle_action(&mut self, action_type: ActionType, action: String) {
        match action.as_str() {
            "AltLeftClick" => { 
                self.handle_action_with_modifier_key("LeftClick".to_owned(), "Alt".to_owned(), 0);
            },
            "LeftClick" => {self.handle_mouse_action(self.match_mouse_str_to_button(action.as_str()), action_type);},
            "MiddleClick" => {self.handle_mouse_action(self.match_mouse_str_to_button(action.as_str()), action_type);},
            "RightClick" => {self.handle_mouse_action(self.match_mouse_str_to_button(action.as_str()), action_type);},
            &_ => { // If it's not a mouse or special case, assume it's a key press
                self.handle_keypress_action(self.match_key_str_to_key(action.to_lowercase().as_str()), action_type, action);
            }
        }
    }

    fn match_mouse_str_to_button(&self, mouse_str: &str) -> Button {
        match mouse_str {
            "LeftClick" => {Button::Left},
            "MiddleClick" => {Button::Middle},
            "RightClick" => {Button::Right},
            &_ => {
                panic!("Invalid mouse_button {:?}", mouse_str); // TODO: Make this raise a config error to the user in the overlay
            }
        }
    }

    fn match_key_str_to_key(&self, key_str: &str) -> Key {
        match key_str {
            "f1" => {Key::F1},
            "f2" => {Key::F2},
            "f3" => {Key::F3},
            "f4" => {Key::F4},
            "f5" => {Key::F5},
            "f6" => {Key::F6},
            "f7" => {Key::F7},
            "f8" => {Key::F8},
            "f9" => {Key::F9},
            "f10" => {Key::F10},
            "f11" => {Key::F11},
            "f12" => {Key::F12},
            "a" => {Key::KeyA},
            "b" => {Key::KeyB},
            "c" => {Key::KeyC},
            "d" => {Key::KeyD},
            "e" => {Key::KeyE},
            "f" => {Key::KeyF},
            "g" => {Key::KeyG},
            "h" => {Key::KeyH},
            "i" => {Key::KeyI},
            "j" => {Key::KeyJ},
            "k" => {Key::KeyK},
            "l" => {Key::KeyL},
            "m" => {Key::KeyM},
            "n" => {Key::KeyN},
            "o" => {Key::KeyO},
            "p" => {Key::KeyP},
            "q" => {Key::KeyQ},
            "r" => {Key::KeyR},
            "s" => {Key::KeyS},
            "t" => {Key::KeyT},
            "u" => {Key::KeyU},
            "v" => {Key::KeyV},
            "w" => {Key::KeyW},
            "x" => {Key::KeyX},
            "y" => {Key::KeyY},
            "z" => {Key::KeyZ},
            "0" => {Key::Num0},
            "1" => {Key::Num1},
            "2" => {Key::Num2},
            "3" => {Key::Num3},
            "4" => {Key::Num4},
            "5" => {Key::Num5},
            "6" => {Key::Num6},
            "7" => {Key::Num7},
            "8" => {Key::Num8},
            "9" => {Key::Num9},
            "`" => {Key::BackQuote},
            "[" => {Key::LeftBracket},
            "]" => {Key::RightBracket},
            ";" => {Key::SemiColon},
            "/" => {Key::Slash},
            "," => {Key::Comma},
            "=" => {Key::Equal},
            "escape" => {Key::Escape},
            "space" => {Key::Space},
            "tab" => {Key::Tab},
            "backspace" => {Key::Backspace},
            "delete" => {Key::Delete},
            "uparrow" => {Key::UpArrow},
            "downarrow" => {Key::DownArrow},
            "leftarrow" => {Key::LeftArrow},
            "rightarrow" => {Key::RightArrow},
            "alt" => {Key::Alt},
            "shift" => {Key::ShiftLeft},
            "control" => {Key::ControlLeft},
            &_ => {
                panic!("Invalid key {:?}", key_str); // TODO: Make this raise a config error to the user in the overlay
            }
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

    fn handle_action_with_modifier_key(&mut self, action: String, modifier: String, delay_ms: u64) {
        self.handle_action(ActionType::Press, modifier.to_owned());
        thread::sleep(time::Duration::from_millis(delay_ms));

        self.handle_action(ActionType::Press, action);
        self.handle_action(ActionType::Release, modifier);
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