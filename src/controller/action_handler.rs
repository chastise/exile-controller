use std::{thread, time, collections::HashMap, ops::Index};

use rdev::{simulate, Button, EventType, Key, SimulateError};

use enigo::Enigo;

#[derive(PartialEq)]
pub enum ActionType {
    PRESS,
    RELEASE,
}

pub struct ActionHandler {
    left_mouse_held: bool,
    middle_mouse_held: bool,
    right_mouse_held: bool,
    //held_mouse_buttons: HashMap::<Button, bool>,
    held_keys: Vec::<Key>,
}

impl Default for ActionHandler {
    fn default() -> Self {
        ActionHandler {
            left_mouse_held: false,
            middle_mouse_held: false,
            right_mouse_held: false,
            //held_mouse_buttons: HashMap::from([(Button::Left, false), (Button::Middle, false), (Button::Right, false)]),
            held_keys: Vec::<Key>::with_capacity(20),
        }
    }
}

impl ActionHandler {
    pub fn handle_action(&mut self, action_type: ActionType, action: String) {
        match action.as_str() {
            "AltLeftClick" => { 
                // TODO when we're ready to implement loot keys
            },
            "LeftClick" => {self.handle_mouse_action(self.match_mouse_str_to_button(action.as_str()), action_type);},
            "MiddleClick" => {self.handle_mouse_action(self.match_mouse_str_to_button(action.as_str()), action_type);},
            "RightClick" => {self.handle_mouse_action(self.match_mouse_str_to_button(action.as_str()), action_type);},
            &_ => { // If it's not a mouse or special case, assume it's a key press
                self.handle_keypress_action(self.match_key_str_to_key(action.to_lowercase().as_str()), action_type);
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
                if action == ActionType::PRESS && !self.left_mouse_held {
                    self.left_mouse_held = true;
                    rdev_send_event(&EventType::ButtonPress(mouse_button))
                } else if action == ActionType::RELEASE && self.left_mouse_held {
                    self.left_mouse_held = false;
                    rdev_send_event(&EventType::ButtonRelease(mouse_button))
                }
            },
            Button::Middle => {
                if action == ActionType::PRESS && !self.middle_mouse_held {
                    self.middle_mouse_held = true;
                    rdev_send_event(&EventType::ButtonPress(mouse_button))
                } else if action == ActionType::RELEASE && self.middle_mouse_held {
                    self.middle_mouse_held = false;
                    rdev_send_event(&EventType::ButtonRelease(mouse_button))
                }
            },
            Button::Right => {
                if action == ActionType::PRESS && !self.right_mouse_held {
                    self.right_mouse_held = true;
                    rdev_send_event(&EventType::ButtonPress(mouse_button))
                } else if action == ActionType::RELEASE && self.right_mouse_held {
                    self.right_mouse_held = false;
                    rdev_send_event(&EventType::ButtonRelease(mouse_button))
                }
            },
            _ => ()
        }
    }

    fn handle_keypress_action(&mut self, keypress: Key, action: ActionType) {
        // KeyPress(Key) KeyRelease(Key)
        if action == ActionType::PRESS {
            if !self.held_keys.contains(&keypress) {
                rdev_send_event(&EventType::KeyPress(keypress));
                self.held_keys.push(keypress);
            }
        } else if action == ActionType::RELEASE {
            if self.held_keys.contains(&keypress) {
                rdev_send_event(&EventType::KeyRelease(keypress));
                for (index, _val) in self.held_keys.iter().enumerate() {
                    if self.held_keys.get(index).unwrap() == &keypress {
                        self.held_keys.swap_remove(index);
                        break;
                    }
                }
                
            }
        }

    }

    pub fn current_mouse_position(&self) -> (f32, f32) {
        let (x, y) = Enigo::mouse_location();
        (x as f32, y as f32)
    }
}

fn rdev_send_event(event_type: &EventType) {
    //let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let the OS catchup (at least MacOS)
    //thread::sleep(delay);
}