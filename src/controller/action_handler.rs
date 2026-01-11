use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time, collections::HashMap};

use crate::settings::ButtonOrKey;

#[derive(PartialEq)]
pub enum ActionType {
    Press,
    Release,
}

pub struct ActionHandler {
    mouse_button_down: HashMap<Button, bool>,
    held_keys: HashMap<Key, String>,
    holding_left_click_for_action: bool,
}

impl Default for ActionHandler {
    fn default() -> Self {
        ActionHandler {
            mouse_button_down: HashMap::from([
                (Button::Left, false),
                (Button::Middle, false),
                (Button::Right, false),]),
            held_keys: HashMap::<Key, String>::with_capacity(20),
            holding_left_click_for_action: false,
        }
    }
}

impl ActionHandler {
    pub fn handle_action(&mut self, action_type: ActionType, action: &ButtonOrKey) {
        match action {
            ButtonOrKey::ButtonKeyCombo(mouse_button, key) => self.handle_action_with_mouse_and_key(key, mouse_button, action_type, 20, 10),
            ButtonOrKey::Button(mouse_button) => self.handle_mouse_action(mouse_button, action_type),
            ButtonOrKey::Key(key) => self.handle_keypress_action(key, action_type),
            ButtonOrKey::Empty => (),
        }
    }

    pub fn holding_left_click_for_action(&self) -> bool {
        self.holding_left_click_for_action
    }

    pub fn move_mouse(&self, x: f64, y: f64) {
        rdev_send_event(&EventType::MouseMove { x, y });
    }

    fn handle_mouse_action(&mut self, mouse_button: &Button, action_type: ActionType) {
        // Action handler tracks held mouse button state to avoid safely spamming events
        match (action_type, self.mouse_button_down.get(&mouse_button)) {
            (ActionType::Press, Some(false)) => {
                self.mouse_button_down.insert(*mouse_button, true);
                rdev_send_event(&EventType::ButtonPress(*mouse_button))
            },
            (ActionType::Release, Some(true)) => {
                self.mouse_button_down.insert(*mouse_button, false);
                rdev_send_event(&EventType::ButtonRelease(*mouse_button))
            },
            _ => (),
        }
    }

    fn handle_keypress_action(&mut self, keypress: &Key, action: ActionType) {
        if action == ActionType::Press {
            if !self.held_keys.contains_key(&keypress) {
                rdev_send_event(&EventType::KeyPress(*keypress));
                self.held_keys.insert(*keypress, format!("{:#?}", keypress));
            }
        } else if action == ActionType::Release {
            if self.held_keys.contains_key(&keypress) {
                rdev_send_event(&EventType::KeyRelease(*keypress));
                self.held_keys.remove(keypress);
            }
        }
    }

    fn handle_action_with_mouse_and_key(&mut self, modifier_key: &Key, mouse_button: &Button, action_type: ActionType, delay_ms_before: u64, delay_ms_after: u64) {
        let modifier_already_held = self.held_keys.contains_key(&modifier_key);
        match action_type {
            ActionType::Press => {
                if !modifier_already_held {
                    self.handle_keypress_action(modifier_key, ActionType::Press);
                    thread::sleep(time::Duration::from_millis(delay_ms_before));
                }
                self.handle_mouse_action(mouse_button, ActionType::Press);
                self.holding_left_click_for_action = true;
                if !modifier_already_held {
                    thread::sleep(time::Duration::from_millis(delay_ms_after));
                    self.handle_keypress_action(modifier_key, ActionType::Release);
                }
            },
            ActionType::Release => {
                self.handle_mouse_action(mouse_button, ActionType::Release);
                self.holding_left_click_for_action = false;
                // This will unpress even if another controller button is already holding this key
                self.handle_keypress_action(modifier_key, ActionType::Release);
            },
        }
    }

    pub fn is_ability_key_held(&self) -> bool {
        let mut is_holding = false;
        if *self.mouse_button_down.get(&Button::Middle).unwrap() || *self.mouse_button_down.get(&Button::Right).unwrap() {
            is_holding = true;
        }
        for key in self.held_keys.keys() {
            match key {
                Key::KeyQ | Key::KeyW | Key::KeyE | Key::KeyR | Key::KeyT => is_holding = true,
                _ => (),
            }
        }
        is_holding
    }
    pub fn get_held_ability_actions(&self) -> Vec<ButtonOrKey> {
        let mut held_ability_actions = Vec::<ButtonOrKey>::new();
        if *self.mouse_button_down.get(&Button::Middle).unwrap() {held_ability_actions.push(ButtonOrKey::Button(Button::Middle));}
        if *self.mouse_button_down.get(&Button::Right).unwrap() {held_ability_actions.push(ButtonOrKey::Button(Button::Right));}
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
