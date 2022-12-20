use std::vec;

use gilrs::{Gilrs, GamepadId, Axis, Button, Event, EventType};

#[derive(Default)]
struct ControllerButton {
    held: bool,
    just_pressed: bool,
    just_unpressed: bool,
}

impl ControllerButton {
    fn changed_button_event(&mut self, value: f32) {
        let just_pressed = value > 0.0;
        self.just_pressed = just_pressed && !self.held;
        self.just_unpressed = !just_pressed && self.held;
        self.held = just_pressed;
        println!("button change event {:?}", self.held);
    }
}

#[derive(Default)]
struct TriggerButton {
    held: bool,
    hold_amount: f32,
    just_pressed: bool,
    just_unpressed: bool,
    trigger_threshold: f32,
}

impl TriggerButton {
    fn set_trigger_threshold(&mut self, value: f32) {
        self.trigger_threshold = value;
    }
    fn changed_button_event(&mut self, value: f32) {
        self.just_pressed = value >= self.trigger_threshold && self.hold_amount < self.trigger_threshold;
        self.just_unpressed = value <= self.trigger_threshold && self.hold_amount >= self.trigger_threshold;
        if self.held != self.just_pressed {println!("trigger button change event | old: {:?} | new: {:?}", self.held, self.just_pressed);}
        self.held = self.just_pressed;
    }
}

#[derive(Default)]
struct DPad {
	dup: ControllerButton,
	ddown: ControllerButton,
	dleft: ControllerButton,
	dright: ControllerButton,
}

#[derive(Default)]
struct AnalogStick  {
	stick_x: f32,
    stick_y: f32,
    deadzone: f32,
	button: ControllerButton,
}

impl AnalogStick {
    fn changed_button_event(&mut self, value: f32) {
        self.button.changed_button_event(value);
    }

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
            return (value - self.deadzone) / (1.0 - self.deadzone);
        } else if value < -self.deadzone {
            return (value + self.deadzone) / (1.0 - self.deadzone);
        } else { 
            return 0.0 as f32; 
        }
    }

    pub fn stick_direction(&self) -> Vec<f32>{
        vec![self.stick_x, self.stick_y]
    }

    pub fn joystick_in_deadzone(&self) -> bool {
        self.stick_x == 0.0 && self.stick_y == 0.0
    }

}


#[derive(Default)]
struct ControllerState {
	dpad: DPad,
	start: ControllerButton,
    back: ControllerButton,
    a: ControllerButton,
    b: ControllerButton,
    x: ControllerButton,
    y: ControllerButton,
    left_bumper: ControllerButton,
    left_trigger: TriggerButton,
    right_bumper: ControllerButton,
    right_trigger: TriggerButton,
    left_analog: AnalogStick,
	right_analog: AnalogStick,
}

impl ControllerState {
    fn get_button_state_changes(&self) {
        // return an iterable of all buttons where just_pressed or just_unpressed is true, tupled to the button name
    }

    fn get_joystick_state(&self) {
        // return direction vector
    }
}

pub struct GamepadManager {
    gilrs_context: Gilrs,
    gamepad_id: GamepadId,
    controller_state: ControllerState,
}

pub fn load_gamepad_manager(gamepad_triggers_threshold: f32, analog_deadzone: f32) -> GamepadManager {
    let gilrs = Gilrs::new().unwrap();
    
    let connected_gamepad_id = gilrs.gamepads().next().unwrap_or_else(
        || panic!("No gamepad connected!")).0;

    let mut gamepad_manager = GamepadManager{
        gilrs_context: gilrs,
        gamepad_id: connected_gamepad_id,
        controller_state: ControllerState::default(),
    };

    // initialize triggers and joy stick deadzones
    gamepad_manager.controller_state.left_trigger.set_trigger_threshold(gamepad_triggers_threshold);
    gamepad_manager.controller_state.right_trigger.set_trigger_threshold(gamepad_triggers_threshold);
    gamepad_manager.controller_state.left_analog.set_joystick_deadzone(analog_deadzone);
    gamepad_manager.controller_state.right_analog.set_joystick_deadzone(analog_deadzone);
    return gamepad_manager;
}

impl GamepadManager {
    pub fn read_latest_input(&mut self) {
        let mut did_axis_change = false;
        while let Some(Event { id, event, time: _ }) = self.gilrs_context.next_event() {
            if id == self.gamepad_id {
                match event {
                    EventType::ButtonChanged(button, value, _code) => {
                        //println!("Button Changed! {:?}: {value} : {code}!", button);
                        match button {
                            Button::South => {self.controller_state.a.changed_button_event(value)},
                            Button::East => {self.controller_state.b.changed_button_event(value)},
                            Button::North => {self.controller_state.y.changed_button_event(value)},
                            Button::West => {self.controller_state.x.changed_button_event(value)},
                            Button::LeftTrigger => {self.controller_state.left_bumper.changed_button_event(value)},
                            Button::LeftTrigger2 => {self.controller_state.left_trigger.changed_button_event(value)},
                            Button::RightTrigger => {self.controller_state.right_bumper.changed_button_event(value)},
                            Button::RightTrigger2 => {self.controller_state.right_trigger.changed_button_event(value)},
                            Button::Select => {self.controller_state.back.changed_button_event(value)},
                            Button::Start => {self.controller_state.start.changed_button_event(value)},
                            Button::LeftThumb => {self.controller_state.left_analog.changed_button_event(value)},
                            Button::RightThumb => {self.controller_state.right_analog.changed_button_event(value)},
                            Button::DPadUp => {self.controller_state.dpad.dup.changed_button_event(value)},
                            Button::DPadDown => {self.controller_state.dpad.ddown.changed_button_event(value)},
                            Button::DPadLeft => {self.controller_state.dpad.dleft.changed_button_event(value)},
                            Button::DPadRight => {self.controller_state.dpad.dright.changed_button_event(value)},
                            _ => (),
                        }
                    },
                    EventType::AxisChanged(axis, value, _code) => {
                        //println!("Axis Changed! {:?}: {value} : {_code}!", axis);
                        match axis {
                            Axis::LeftStickX => {self.controller_state.left_analog.changed_axis_event(value, true); did_axis_change = true;}
                            Axis::LeftStickY => {self.controller_state.left_analog.changed_axis_event(value, false); did_axis_change = true;}
                            Axis::RightStickX => {self.controller_state.right_analog.changed_axis_event(value, true); did_axis_change = true;}
                            Axis::RightStickY => {self.controller_state.right_analog.changed_axis_event(value, false); did_axis_change = true;}
                            _ => (),
                        }
                    },
                    _ => (),
                }
            }
        }
        if did_axis_change {
            // println!("Axis before deadzones: Left Stick {:?} | Right Stick {:?}", self.controller_input.left_analog.stick_direction(), self.controller_input.right_analog.stick_direction());
            // // Apply deadzones to joysticks:
            // self.controller_input.left_analog.apply_deadzones();
            // self.controller_input.right_analog.apply_deadzones();
            println!("Axis after deadzones: Left Stick {:?} | Right Stick {:?}", self.controller_state.left_analog.stick_direction(), self.controller_state.right_analog.stick_direction());
        }
    }
}



pub fn check_gamepads() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }   

    let mut active_gamepad = None;

    let mut controller_input = ControllerState::default();

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);
        }
    
        // You can also use cached gamepad state
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            if gamepad.is_pressed(Button::South) {
                println!("Button South is pressed (XBox - A, PS - X)");
            }
        }
    }

}