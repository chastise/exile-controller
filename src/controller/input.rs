use std::collections::HashMap;

use gilrs::{Gilrs, GamepadId, Axis, Button, Event, EventType};

#[derive(Default)]
pub struct ControllerButton {
    pub held: bool,
    pub just_pressed: bool,
    pub just_unpressed: bool,
}

impl ControllerButton {
    fn changed_button_event(&mut self, value: f32) {
        let just_pressed = value > 0.0;
        self.just_pressed = just_pressed && !self.held;
        self.just_unpressed = !just_pressed && self.held;
        self.held = just_pressed;
    }
}

// This intermediate struct lets everyone outside ControllerState treat trigger buttons like regular ControllerButtons
#[derive(Default)]
struct TriggerButton {
    hold_amount: f32,
    trigger_threshold: f32,
    button: ControllerButton,
}

impl TriggerButton {
    fn set_trigger_threshold(&mut self, value: f32) {
        self.trigger_threshold = value;
    }

    fn changed_button_event(&mut self, value: f32) {
        self.button.just_pressed = value >= self.trigger_threshold && self.hold_amount < self.trigger_threshold;
        self.button.just_unpressed = value < self.trigger_threshold && self.hold_amount >= self.trigger_threshold;
        self.button.held = self.button.just_pressed;
        self.hold_amount = value;
    }
}

#[derive(Default)]
struct AnalogStickWithButton  {
    button: ControllerButton,
    analog_stick: AnalogStick,
}

impl AnalogStickWithButton {
    fn analog_stick(&self) -> AnalogStick {self.analog_stick}
}

// Making this clone-able because there's no need for analog sticks to be updated by the action manager, unlike controller buttons
#[derive(Default, Clone, Copy)]
pub struct AnalogStick  {
	stick_x: f32,
    stick_y: f32,
    deadzone: f32,
}

impl AnalogStick {
    fn changed_axis_event(&mut self, value: f32, is_stick_x: bool) {
        if is_stick_x {
            self.stick_x = self.compute_joystick_deadzone(value);
        } else {
            self.stick_y = self.compute_joystick_deadzone(value);
        }
    }

    fn set_joystick_deadzone(&mut self, value: f32) {
        self.deadzone = value;
    }

    fn compute_joystick_deadzone(&self, value: f32) -> f32 {
        if value > self.deadzone {
            (value - self.deadzone) / (1.0 - self.deadzone)
        } else if value < -self.deadzone {
            (value + self.deadzone) / (1.0 - self.deadzone)
        } else { 
            0.0_f32
        }
    }

    pub fn stick_direction(&self) -> Vec<f32> {
        vec![self.stick_x, self.stick_y]
    }

    pub fn stick_angle(&self) -> f32 {
        self.stick_y.atan2(self.stick_x)
    }

    pub fn joystick_in_deadzone(&self) -> bool {
        self.stick_x == 0.0 && self.stick_y == 0.0
    }

}


#[derive(Default)]
pub struct ControllerState {
    dpad_up: ControllerButton,
	dpad_down: ControllerButton,
	dpad_left: ControllerButton,
	dpad_right: ControllerButton,
	start: ControllerButton,
    back: ControllerButton,
    a: ControllerButton,
    b: ControllerButton,
    x: ControllerButton,
    y: ControllerButton,
    bumper_left: ControllerButton,
    trigger_left: TriggerButton,
    bumper_right: ControllerButton,
    trigger_right: TriggerButton,
    left_analog: AnalogStickWithButton,
	right_analog: AnalogStickWithButton,
}

impl ControllerState {
    pub fn get_all_buttons(&mut self) -> HashMap<String, &mut ControllerButton> {
        let mut buttons = HashMap::new();

        buttons.insert("dpad_up".to_string(), &mut self.dpad_up);
        buttons.insert("dpad_down".to_string(), &mut self.dpad_down);
        buttons.insert("dpad_left".to_string(), &mut self.dpad_left);
        buttons.insert("dpad_right".to_string(), &mut self.dpad_right);
        buttons.insert("start".to_string(), &mut self.start);
        buttons.insert("back".to_string(), &mut self.back);
        buttons.insert("a".to_string(), &mut self.a);
        buttons.insert("b".to_string(), &mut self.b);
        buttons.insert("x".to_string(), &mut self.x);
        buttons.insert("y".to_string(), &mut self.y);
        buttons.insert("bumper_left".to_string(), &mut self.bumper_left);
        buttons.insert("trigger_left".to_string(), &mut self.trigger_left.button);
        buttons.insert("bumper_right".to_string(), &mut self.bumper_right);
        buttons.insert("trigger_right".to_string(), &mut self.trigger_right.button);
        buttons.insert("left_analog".to_string(), &mut self.left_analog.button);
        buttons.insert("right_analog".to_string(), &mut self.right_analog.button);

        buttons
    }

    pub fn get_left_analog_stick(&self) -> AnalogStick {
        self.left_analog.analog_stick()
    }

    pub fn get_right_analog_stick(&self) -> AnalogStick {
        self.right_analog.analog_stick()
    }

}

pub struct GamepadManager {
    gilrs_context: Gilrs,
    gamepad_id: Option<GamepadId>,
    pub controller_state: ControllerState,
}

pub fn load_gamepad_manager(gamepad_triggers_threshold: f32, analog_deadzone: f32) -> GamepadManager {
    let gilrs = Gilrs::new().unwrap();
      
    let potentially_connected_gamepad = gilrs.gamepads().next();
    let connected_gamepad_id = match potentially_connected_gamepad {
        Some((gamepad_id, _gamepad)) => Some(gamepad_id),
        _ => None,
    };

    let mut gamepad_manager = GamepadManager{
        gilrs_context: gilrs,
        gamepad_id: connected_gamepad_id,
        controller_state: ControllerState::default(),
    };

    // initialize triggers and joy stick deadzones
    gamepad_manager.controller_state.trigger_left.set_trigger_threshold(gamepad_triggers_threshold);
    gamepad_manager.controller_state.trigger_right.set_trigger_threshold(gamepad_triggers_threshold);
    gamepad_manager.controller_state.left_analog.analog_stick.set_joystick_deadzone(analog_deadzone);
    gamepad_manager.controller_state.right_analog.analog_stick.set_joystick_deadzone(analog_deadzone);

    gamepad_manager
}

impl GamepadManager {
    pub fn read_latest_input(&mut self) {
        let mut did_axis_change = false;
        let gamepad_id = self.gamepad_id.unwrap();
        while let Some(Event { id, event, time: _ }) = self.gilrs_context.next_event() {
            if id == gamepad_id {
                match event {
                    EventType::Disconnected => {
                        let was_disconnected = self.disconect_connected_controller();
                        println!("controller disconnected during input read? {:?}", was_disconnected);
                    }
                    EventType::ButtonChanged(button, value, _code) => {
                        //println!("Button Changed! {:?}: {value} : {code}!", button);
                        match button {
                            Button::South => {self.controller_state.a.changed_button_event(value)},
                            Button::East => {self.controller_state.b.changed_button_event(value)},
                            Button::North => {self.controller_state.y.changed_button_event(value)},
                            Button::West => {self.controller_state.x.changed_button_event(value)},
                            Button::LeftTrigger => {self.controller_state.bumper_left.changed_button_event(value)},
                            Button::LeftTrigger2 => {self.controller_state.trigger_left.changed_button_event(value)},
                            Button::RightTrigger => {self.controller_state.bumper_right.changed_button_event(value)},
                            Button::RightTrigger2 => {self.controller_state.trigger_right.changed_button_event(value)},
                            Button::Select => {self.controller_state.back.changed_button_event(value)},
                            Button::Start => {self.controller_state.start.changed_button_event(value)},
                            Button::LeftThumb => {self.controller_state.left_analog.button.changed_button_event(value)},
                            Button::RightThumb => {self.controller_state.right_analog.button.changed_button_event(value)},
                            Button::DPadUp => {self.controller_state.dpad_up.changed_button_event(value)},
                            Button::DPadDown => {self.controller_state.dpad_down.changed_button_event(value)},
                            Button::DPadLeft => {self.controller_state.dpad_left.changed_button_event(value)},
                            Button::DPadRight => {self.controller_state.dpad_right.changed_button_event(value)},
                            _ => (),
                        }
                    },
                    EventType::AxisChanged(axis, value, _code) => {
                        //println!("Axis Changed! {:?}: {value} : {_code}!", axis);
                        match axis {
                            Axis::LeftStickX => {self.controller_state.left_analog.analog_stick.changed_axis_event(value, true); did_axis_change = true;}
                            Axis::LeftStickY => {self.controller_state.left_analog.analog_stick.changed_axis_event(value, false); did_axis_change = true;}
                            Axis::RightStickX => {self.controller_state.right_analog.analog_stick.changed_axis_event(value, true); did_axis_change = true;}
                            Axis::RightStickY => {self.controller_state.right_analog.analog_stick.changed_axis_event(value, false); did_axis_change = true;}
                            _ => (),
                        }
                    },
                    _ => (),
                }
            }
        }
        if did_axis_change {
            // println!("Axis after deadzones: Left Stick {:?} | Right Stick {:?}", self.controller_state.left_analog.stick_direction(), self.controller_state.right_analog.stick_direction());
        }
    }

    pub fn check_if_controller_disconnected(&mut self) -> bool {
        let mut was_disconnected = false;
        if self.is_controller_connected() {
            let gamepad_id = self.gamepad_id.unwrap();
            while let Some(Event { id, event, time: _ }) = self.gilrs_context.next_event() {
                if id == gamepad_id {
                    if event == EventType::Disconnected {
                        was_disconnected = self.disconect_connected_controller();
                    }
                }
            }
        }
        was_disconnected
    }
    
    pub fn force_check_new_controllers(&mut self) {
        // Force a new Gilrs instance because hotplugging doesn't seem to be working
        self.gilrs_context = Gilrs::new().unwrap();
    }

    pub fn get_connected_controllers(&mut self) -> Vec<(GamepadId, String)> { 
        let mut connected_controllers = Vec::<(GamepadId, String)>::new();
        for (gamepad_id, gamepad) in self.gilrs_context.gamepads() {
            connected_controllers.push((gamepad_id, gamepad.name().to_string()));
        }
        connected_controllers
    }

    pub fn is_controller_connected(&self) -> bool {
        match self.gamepad_id {
            Some(_c) => true,
            None => false,
        }
    }

    pub fn connect_to_controller(&mut self, connected_controllers: Vec<(GamepadId, String)>, index: usize) { 
        let gamepad_id = connected_controllers[index].0;
        self.gamepad_id = Some(gamepad_id);
        println!("Controller connected!");
    }

    pub fn get_connected_controller_label(&self) -> String {
        if self.is_controller_connected() {
            self.gilrs_context.gamepad(self.gamepad_id.unwrap()).os_name().to_owned()
        } else {
            "none".to_owned()
        }
    }

    pub fn disconect_connected_controller(&mut self) -> bool {
        if self.is_controller_connected() {
            self.gamepad_id = None;
            true
        } else {
            println!("Failed to disconnect controller. Already disconnected?");
            false
        }
    }
}
