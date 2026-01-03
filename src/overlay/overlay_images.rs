use crate::controller::input::ControllerType;

struct ControllerImage {
    playstation: String,
    xbox: String,
}

impl ControllerImage {
    fn new(playstation: &str, xbox: &str) -> Self {
        Self {
            playstation: playstation.to_string(),
            xbox: xbox.to_string(),
        }
    }
    pub fn choose_image(&self, controller_type: ControllerType) -> &String {
        match controller_type {
            ControllerType::Playstation => &self.playstation,
            ControllerType::Xbox => &self.xbox,
        }
    }
}

pub struct OverlayImages {
    button_d_up: ControllerImage,
    button_d_down: ControllerImage,
    button_d_left: ControllerImage,
    button_d_right: ControllerImage,

    button_face_up: ControllerImage,
    button_face_down: ControllerImage,
    button_face_left: ControllerImage,
    button_face_right: ControllerImage,

    button_bumper_left: ControllerImage,
    button_bumper_right: ControllerImage,
    // button_trigger_left: ControllerImage,
    // button_trigger_right: ControllerImage,

    left_stick: ControllerImage,
    // right_stick: ControllerImage,

    // button_l3: ControllerImage,
    button_r3: ControllerImage,
}

impl OverlayImages {
    pub fn button_d_up(&self, controller_type: ControllerType) -> &String {self.button_d_up.choose_image(controller_type)}
    pub fn button_d_down(&self, controller_type: ControllerType) -> &String {self.button_d_down.choose_image(controller_type)}
    pub fn button_d_left(&self, controller_type: ControllerType) -> &String {self.button_d_left.choose_image(controller_type)}
    pub fn button_d_right(&self, controller_type: ControllerType) -> &String {self.button_d_right.choose_image(controller_type)}

    pub fn button_face_up(&self, controller_type: ControllerType) -> &String {self.button_face_up.choose_image(controller_type)}
    pub fn button_face_down(&self, controller_type: ControllerType) -> &String {self.button_face_down.choose_image(controller_type)}
    pub fn button_face_left(&self, controller_type: ControllerType) -> &String {self.button_face_left.choose_image(controller_type)}
    pub fn button_face_right(&self, controller_type: ControllerType) -> &String {self.button_face_right.choose_image(controller_type)}

    pub fn button_bumper_left(&self, controller_type: ControllerType) -> &String {self.button_bumper_left.choose_image(controller_type)}
    pub fn button_bumper_right(&self, controller_type: ControllerType) -> &String {self.button_bumper_right.choose_image(controller_type)}
    // pub fn button_trigger_left(&self, controller_type: ControllerType) -> &String {self.button_trigger_left.choose_image(controller_type)}
    // pub fn button_trigger_right(&self, controller_type: ControllerType) -> &String {self.button_trigger_right.choose_image(controller_type)}

    pub fn left_stick(&self, controller_type: ControllerType) -> &String {self.left_stick.choose_image(controller_type)}
    // pub fn right_stick(&self, controller_type: ControllerType) -> &String {self.right_stick.choose_image(controller_type)}

    // pub fn button_l3(&self, controller_type: ControllerType) -> &String {self.button_l3.choose_image(controller_type)}
    pub fn button_r3(&self, controller_type: ControllerType) -> &String {self.button_r3.choose_image(controller_type)}

}


impl Default for OverlayImages {
    fn default() -> Self {
        Self {
            button_d_up: ControllerImage::new("file://img/playstation/dpad_up.png", "file://img/xbox/dpad_up.png"),
            button_d_down: ControllerImage::new("file://img/playstation/dpad_down.png", "file://img/xbox/dpad_down.png"),
            button_d_left: ControllerImage::new("file://img/playstation/dpad_left.png", "file://img/xbox/dpad_left.png"),
            button_d_right: ControllerImage::new("file://img/playstation/dpad_right.png", "file://img/xbox/dpad_right.png"),

            button_face_down: ControllerImage::new("file://img/playstation/ps_button_x.png",  "file://img/xbox/xb_button_a.png"),
            button_face_right: ControllerImage::new("file://img/playstation/ps_button_o.png", "file://img/xbox/xb_button_b.png"),
            button_face_left: ControllerImage::new("file://img/playstation/ps_button_sq.png", "file://img/xbox/xb_button_x.png"),
            button_face_up: ControllerImage::new("file://img/playstation/ps_button_tri.png", "file://img/xbox/xb_button_y.png"),

            button_bumper_left: ControllerImage::new("file://img/playstation/ps_lb.png",  "file://img/xbox/xb_lb.png"),
            button_bumper_right: ControllerImage::new("file://img/playstation/ps_rb.png", "file://img/xbox/xb_rb.png"),
            // button_trigger_left: ControllerImage::new("file://img/playstation/ps_lt.png", "file://img/xbox/xb_lt.png"),
            // button_trigger_right: ControllerImage::new("file://img/playstation/ps_rb.png", "file://img/xbox/xb_rb.png"),

            left_stick: ControllerImage::new("file://img/playstation/left_analog.png", "file://img/xbox/left_analog.png"),
            // right_stick: ControllerImage::new("file://img/playstation/right_analog.png", "file://img/xbox/right_analog.png"),

            // button_l3: ControllerImage::new("file://img/playstation/button_l3.png", "file://img/xbox/button_l3.png"),
            button_r3: ControllerImage::new("file://img/playstation/button_r3.png", "file://img/xbox/button_r3.png"),
        }
    }
}