use std::collections::{HashMap, HashSet};

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

    pub fn joystick_pull_amount_smoothed(&self) -> f32 {
        // todo: Figure out how this feels in practice
        if self.deadzone > 0.0 {
            if self.deadzone < 0.5 {
                0.5_f32
            } else if self.deadzone < 0.9 {
                self.deadzone * 1.25_f32 - 0.125_f32
            } else {
                1.0_f32
            }
        } else {
            0.0_f32
        }
        
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

#[derive(Copy, Clone, serde::Deserialize)]
pub enum ControllerType {
    Playstation,
    Xbox,
}

#[derive(Copy, Clone, serde::Deserialize)]
pub enum ControllerTypeDetection {
    Auto,
    Forced(ControllerType),
}

pub struct GamepadManager {
    gilrs_context: Gilrs,
    gamepad_id: Option<GamepadId>,
    controller_type: Option<ControllerType>,
    pub controller_type_detection: ControllerTypeDetection,
    pub controller_state: ControllerState,
}

pub fn load_gamepad_manager(gamepad_triggers_threshold: f32, analog_deadzone: f32) -> GamepadManager {
    let gilrs = Gilrs::new().unwrap();

    let mut gamepad_manager = GamepadManager{
        gilrs_context: gilrs,
        gamepad_id: None,
        controller_type: None,
        controller_type_detection: ControllerTypeDetection::Auto,
        controller_state: ControllerState::default(),
    };

    // Gilrs will not initially throw a connected event if a controller is connected from the start
    let connected_controllers = gamepad_manager.get_connected_controllers();
    if !connected_controllers.is_empty() {
        gamepad_manager.connect_to_controller(connected_controllers, 0);
    }

    // initialize triggers and joy stick deadzones
    gamepad_manager.controller_state.trigger_left.set_trigger_threshold(gamepad_triggers_threshold);
    gamepad_manager.controller_state.trigger_right.set_trigger_threshold(gamepad_triggers_threshold);
    gamepad_manager.controller_state.left_analog.analog_stick.set_joystick_deadzone(analog_deadzone);
    gamepad_manager.controller_state.right_analog.analog_stick.set_joystick_deadzone(analog_deadzone);

    gamepad_manager
}

impl GamepadManager {
    pub fn process_gamepad_events(&mut self) {
        while let Some(Event { id, event, time: _ }) = self.gilrs_context.next_event() {
            match self.gamepad_id {
                Some(gamepad_id) => {
                    if id == gamepad_id {
                        match event {
                            EventType::Disconnected => self.disconnect_connected_controller(),
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
                                    Axis::LeftStickX => {self.controller_state.left_analog.analog_stick.changed_axis_event(value, true);}
                                    Axis::LeftStickY => {self.controller_state.left_analog.analog_stick.changed_axis_event(value, false);}
                                    Axis::RightStickX => {self.controller_state.right_analog.analog_stick.changed_axis_event(value, true);}
                                    Axis::RightStickY => {self.controller_state.right_analog.analog_stick.changed_axis_event(value, false);}
                                    _ => (),
                                }
                            },
                            _ => (),
                        }
                    }
                },
                // gilrs doesn't register new gamepads in gilrs_context.gamepads() until EventType::Connected events have been pulled off the events queue.
                None => {
                    match event {
                        EventType::Connected => {
                            // TODO(Samantha): Is this really what we want to do here? Reconsider when we allow changing controllers.
                            let connected_controllers = self.get_connected_controllers();
                            self.connect_to_controller(connected_controllers, 0)
                        },
                        _ => (),
                    }
                },
            }
        }
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
        self.controller_type = Some(self.infer_controller_type());
        println!("Controller connected!");
    }

    pub fn get_connected_controller_label(&self) -> String {
        if self.is_controller_connected() {
            self.gilrs_context.gamepad(self.gamepad_id.unwrap()).os_name().to_owned()
        } else {
            "none".to_owned()
        }
    }

    pub fn get_connected_controller_map_name(&self) -> String {
        if self.is_controller_connected() {
            match self.gilrs_context.gamepad(self.gamepad_id.unwrap()).map_name() {
                Some(mapper) => mapper.to_owned(),
                None => "none".to_owned(),
            }
        } else {
            "none".to_owned()
        }
    }

    pub fn disconnect_connected_controller(&mut self) {
        if self.is_controller_connected() {
            self.gamepad_id = None;
            println!("Controller disconnected!");
        } else {
            println!("Failed to disconnect controller. Already disconnected?");
        }
    }

    fn infer_controller_type(&self) -> ControllerType {
        // Matching on both of these gives us a greater chance at automatic
        let map_name = self.get_connected_controller_map_name().to_lowercase();
        let label = self.get_connected_controller_label().to_lowercase();
        
        // TODO(Samantha): This is a candidate for lazy static, but I'm not convinced that the compiler isn't optimizing this.
        let playstation_names = HashSet::from(["ps5 controller", "ps4 controller", "ps3 controller", "ps2 controller", "ps1 controller", "playstation", "sony"]);
        if playstation_names.contains(map_name.as_str()) || playstation_names.contains(label.as_str()) {
            ControllerType::Playstation
        } else {
            ControllerType::Xbox
        }
    }

    pub fn determine_controller_type(&self) -> ControllerType {
        match self.controller_type_detection {
            ControllerTypeDetection::Forced(controller_type) => controller_type,
            ControllerTypeDetection::Auto => self.controller_type.unwrap(),
        }
    }

    pub fn set_controller_type_detection(&mut self, controller_type_detection: ControllerTypeDetection) {
        self.controller_type_detection = controller_type_detection;
    }
}
