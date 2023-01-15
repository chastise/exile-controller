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
    pub fn handle_action(&mut self, action_type: ActionType, action: &ButtonOrKey) {
        match action {
            ButtonOrKey::ButtonKeyChord(button, key) => {
                self.handle_action_with_modifier_key(ButtonOrKey::Button(*button), ButtonOrKey::Key(*key), 20);
            }
            ButtonOrKey::Button(button) => self.handle_mouse_action(*button, action_type),
            ButtonOrKey::Key(key) => self.handle_keypress_action(*key, action_type),
            ButtonOrKey::Empty => (),
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

    fn handle_action_with_modifier_key(&mut self, action: ButtonOrKey, modifier: ButtonOrKey, delay_ms: u64) {
        self.handle_action(ActionType::Press, &modifier);
        thread::sleep(time::Duration::from_millis(delay_ms));

        self.handle_action(ActionType::Press, &action);
        self.handle_action(ActionType::Release, &modifier);
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