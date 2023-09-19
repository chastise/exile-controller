use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time, collections::HashMap};

use crate::settings::ButtonOrKey;

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
    pub fn handle_action(&mut self, action_type: ActionType, action: &ButtonOrKey) {
        match action {
            ButtonOrKey::ButtonKeyCombo(button, key) => {
                self.handle_action_with_modifier_key(action_type, *button, *key, 20, 10);
            }
            ButtonOrKey::Button(button) => self.handle_mouse_action(*button, action_type),
            ButtonOrKey::Key(key) => self.handle_keypress_action(*key, action_type),
            ButtonOrKey::Empty => (),
        }
    }

    pub fn holding_left_click_for_action(&self) -> bool {
        self.holding_left_click_for_action
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

    fn handle_keypress_action(&mut self, keypress: Key, action: ActionType) {
        if action == ActionType::Press {
            if !self.held_keys.contains_key(&keypress) {
                rdev_send_event(&EventType::KeyPress(keypress));
                self.held_keys.insert(keypress, format!("{:#?}", keypress));
            }
        } else if action == ActionType::Release {
            if self.held_keys.contains_key(&keypress) {
                rdev_send_event(&EventType::KeyRelease(keypress));
                self.held_keys.remove(&keypress);
            }
        }
    }

    fn handle_action_with_modifier_key(&mut self, action_type: ActionType, action: rdev::Button, modifier_key: rdev::Key, delay_ms_before: u64, delay_ms_after: u64) {
        // We can trust this lookup so long as we only call this function with known inputs. 
        // If inputs are user-specified, we must refactor to check them.
        let modifier_already_held = self.held_keys.contains_key(&modifier_key);

        if action_type == ActionType::Press {
            if !modifier_already_held {
                self.handle_keypress_action(modifier_key, ActionType::Press);
                thread::sleep(time::Duration::from_millis(delay_ms_before));
            }
            self.handle_mouse_action(action, ActionType::Press);
            self.holding_left_click_for_action = true;
            self.handle_keypress_action(modifier_key, ActionType::Press);

            if !modifier_already_held {
                thread::sleep(time::Duration::from_millis(delay_ms_after));
                self.handle_keypress_action(modifier_key, ActionType::Release);
            }
        } 
        else if action_type == ActionType::Release {
            self.handle_mouse_action(action, ActionType::Release);
            self.holding_left_click_for_action = false;
            self.handle_keypress_action(modifier_key, ActionType::Release);
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
    pub fn get_held_ability_actions(&self) -> Vec<ButtonOrKey> {
        let mut held_ability_actions = Vec::<ButtonOrKey>::new();
        if self.middle_mouse_held {held_ability_actions.push(ButtonOrKey::Button(rdev::Button::Middle));}
        if self.right_mouse_held {held_ability_actions.push(ButtonOrKey::Button(rdev::Button::Right));}
        if self.held_keys.contains_key(&Key::KeyQ) {held_ability_actions.push(ButtonOrKey::Key(Key::KeyQ));}
        if self.held_keys.contains_key(&Key::KeyW) {held_ability_actions.push(ButtonOrKey::Key(Key::KeyW));}
        if self.held_keys.contains_key(&Key::KeyE) {held_ability_actions.push(ButtonOrKey::Key(Key::KeyE));}
        if self.held_keys.contains_key(&Key::KeyR) {held_ability_actions.push(ButtonOrKey::Key(Key::KeyR));}
        if self.held_keys.contains_key(&Key::KeyT) {held_ability_actions.push(ButtonOrKey::Key(Key::KeyT));}
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