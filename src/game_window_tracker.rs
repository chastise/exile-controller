use crate::settings::ApplicationSettings;


#[derive(Clone, Copy)]
pub struct GameWindowTracker {
    // TODO(chastise): This probably should be re-usable in a refactor, one for game-overlay specific things, one for remote overlay
    windowed_mode: bool,
    window_pos_x: f32,
    window_pos_y: f32,
    game_window_width: f32,
    game_window_height: f32,
}


impl GameWindowTracker {
    pub fn new(application_settings: ApplicationSettings) -> GameWindowTracker {
        GameWindowTracker { 
            windowed_mode: application_settings.overlay_settings().windowed_mode(),
            window_pos_x: 0.0,
            window_pos_y: 0.0,
            game_window_width: application_settings.overlay_settings().screen_width(),
            game_window_height: application_settings.overlay_settings().screen_height(),
        }
    }
    pub fn windowed_mode(&self) -> bool {self.windowed_mode} // TODO: Remove this when we allow in-gui settings editing
    pub fn window_pos_x(&self) -> f32 {self.window_pos_x}
    pub fn window_pos_y(&self) -> f32 {self.window_pos_y}
    pub fn game_window_width(&self) -> f32 {self.game_window_width}
    pub fn game_window_height(&self) -> f32 {self.game_window_height}

    pub fn is_poe_active(&self) -> bool {
        let active_window = active_win_pos_rs::get_active_window();
            match active_window {
                Ok(active_window) => {
                    active_window.title.to_lowercase() == "Path of Exile".to_lowercase()
                },
                Err(_) => false,
            }
    }

    pub fn update_window_tracker(&mut self) {
        (self.window_pos_x, self.window_pos_y) = self.window_position();
        (self.game_window_width, self.game_window_height) = self.window_size();
    }

    fn window_position(&self) -> (f32, f32) {
        if self.is_poe_active() {
            match active_win_pos_rs::get_position() {
                Ok(position) => (position.x as f32, position.y as f32),
                Err(_) => (0.0, 0.0),
            }
        } else {
            (0.0, 0.0)
        }
    }

    fn window_size(&self) -> (f32, f32) {
        if self.windowed_mode && self.is_poe_active() {
            match active_win_pos_rs::get_position() {
                Ok(position) => (position.width as f32, position.height as f32),
                Err(_) => (0.0, 0.0),
            }
        } else {
            (self.game_window_width, self.game_window_height)
        }
    }
}