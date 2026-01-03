use std::process::exit;

use egui;
use egui::{Area, Color32, Context, epaint, Pos2, Rect, Vec2};

// use egui_overlay::EguiOverlay;
use crate::overlay::overlay_backend::{self, EguiOverlay};

#[cfg(not(target_os = "macos"))]
use egui_overlay::egui_render_three_d::ThreeDBackend as DefaultGfxBackend;

// Mac is not supported until wgpu dependencies get fixed by several crates with pinned incompatible versions
// #[cfg(target_os = "macos")]
// use egui_render_wgpu::WgpuBackend as DefaultGfxBackend;

use egui_extras;

use crate::controller::action_manager::ActionManager;
use crate::controller::input::GamepadManager;
use crate::game_window_tracker::GameWindowTracker;
use crate::settings::{OverlaySettings, ControllerSettings};
use crate::overlay::overlay_images::OverlayImages;


struct GameOverlay {
    overlay_settings: OverlaySettings,
    game_window_tracker: GameWindowTracker,
    screen_rect: Rect,
    overlay_images: OverlayImages,
    controller_settings: ControllerSettings,
    gamepad_manager: GamepadManager, 
    game_action_handler: ActionManager,
    remote_open: bool,
    remote_pos: Pos2,
    game_input_started: bool,
}

impl GameOverlay {
    // TODO: Figure these offsets out dynamically, updating via game_window_tracker, using the screen ratio since that seems to be what scales the game's display UI. 
    // May also need to chop off the window header in windowed mode and downsize the images for smaller windows.
    fn place_overlay_image(&self, ctx: &Context, image_path: &String, position: Pos2, id_source: String) {
        egui::Area::new(egui::Id::new(id_source.clone()))
                        .movable(false)
                        .fixed_pos(position)
                        .interactable(false)
                        .default_size(Vec2 { x: 32.0, y: 32.0 })
                        .show(ctx,|ui| {
                            ui.image(image_path)
                        });
    }

    fn place_face_overlay_images (&self, ctx: &Context, images: &OverlayImages) {
        let x_offset = 0.828;
        let x_offset_offset = 0.029;
        let y_offset = 0.97;

        let controller_type = self.gamepad_manager.determine_controller_type();
        self.place_overlay_image(ctx, &images.button_face_left(controller_type),
                        Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*3.0) + self.game_window_tracker.game_window_pos_x(), 
                            y: self.game_window_tracker.game_window_height() * y_offset + self.game_window_tracker.game_window_pos_y() },
                       "button_face_left".to_string());
        self.place_overlay_image(ctx, &images.button_face_down(controller_type),
                        Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*2.0) + self.game_window_tracker.game_window_pos_x(), 
                            y: self.game_window_tracker.game_window_height() * y_offset + self.game_window_tracker.game_window_pos_y() },
                        "button_face_down".to_string());
        self.place_overlay_image(ctx, &images.button_face_right(controller_type),
                        Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*1.0) + self.game_window_tracker.game_window_pos_x(), 
                            y: self.game_window_tracker.game_window_height() * y_offset + self.game_window_tracker.game_window_pos_y() },
                        "button_face_right".to_string());
        self.place_overlay_image(ctx, &images.button_face_up(controller_type),
                        Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset) + self.game_window_tracker.game_window_pos_x(), 
                            y: self.game_window_tracker.game_window_height() * y_offset + self.game_window_tracker.game_window_pos_y() },
                        "button_face_up".to_string());
    }

    fn place_flask_overlay_images (&self, ctx: &Context, images: &OverlayImages) {
        let x_offset = 0.2615;
        let x_offset_offset = 0.0242;
        let y_offset = 0.97;

        let controller_type = self.gamepad_manager.determine_controller_type();
        self.place_overlay_image(ctx, &images.button_d_left(controller_type),
            Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*4.0) + self.game_window_tracker.game_window_pos_x(), 
                y: self.game_window_tracker.game_window_height() * y_offset + self.game_window_tracker.game_window_pos_y() },
           "button_d_left".to_string());
        self.place_overlay_image(ctx, &images.button_d_down(controller_type),
                        Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*3.0) + self.game_window_tracker.game_window_pos_x(), 
                            y: self.game_window_tracker.game_window_height() * y_offset + self.game_window_tracker.game_window_pos_y() },
                       "button_d_down".to_string());
        self.place_overlay_image(ctx, &images.button_d_right(controller_type),
                        Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*2.0) + self.game_window_tracker.game_window_pos_x(), 
                            y: self.game_window_tracker.game_window_height() * y_offset + self.game_window_tracker.game_window_pos_y() },
                        "button_d_right".to_string());
        self.place_overlay_image(ctx, &images.button_d_up(controller_type),
                        Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*1.0) + self.game_window_tracker.game_window_pos_x(), 
                            y: self.game_window_tracker.game_window_height() * y_offset + self.game_window_tracker.game_window_pos_y() },
                        "button_d_up".to_string());
        self.place_overlay_image(ctx, &images.button_r3(controller_type),
                        Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset) + self.game_window_tracker.game_window_pos_x(), 
                            y: self.game_window_tracker.game_window_height() * y_offset + self.game_window_tracker.game_window_pos_y() },
                        "button_r3".to_string());
    }

    fn place_mouse_button_overlay_images (&self, ctx: &Context, images: &OverlayImages) {
        let x_offset = 0.8585;
        let x_offset_offset = 0.029;
        let y_offset = 0.909;

        let controller_type = self.gamepad_manager.determine_controller_type();
        self.place_overlay_image(ctx, &images.left_stick(controller_type),
            Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*2.0) + self.game_window_tracker.game_window_pos_x(), 
                y: self.game_window_tracker.game_window_height() * y_offset + self.game_window_tracker.game_window_pos_y() },
            "left_stick".to_string());
        self.place_overlay_image(ctx, &images.button_bumper_left(controller_type),
                        Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*1.0) + self.game_window_tracker.game_window_pos_x(), 
                            y: self.game_window_tracker.game_window_height() * y_offset + self.game_window_tracker.game_window_pos_y() },
                       "button_bumper_left".to_string());
        self.place_overlay_image(ctx, &images.button_bumper_right(controller_type),
                        Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset) + self.game_window_tracker.game_window_pos_x(), 
                            y: self.game_window_tracker.game_window_height() * y_offset + self.game_window_tracker.game_window_pos_y() },
                        "button_bumper_right".to_string());
    }

    fn paint_crosshair (&self, ctx: &Context) {
        let crosshair_radius = 5.0;
        // offset radius*2.0 because the paint area is radius * 4 across
        let crosshair_position = Pos2 { x: (self.game_window_tracker.game_window_width() / 2.0) - self.controller_settings.character_x_offset_px() - crosshair_radius*2.0  + self.game_window_tracker.game_window_pos_x(), 
                                                y: (self.game_window_tracker.game_window_height() / 2.0) - self.controller_settings.character_y_offset_px() - crosshair_radius*2.0 + self.game_window_tracker.game_window_pos_y()};
        Area::new(egui::Id::new("crosshair"))
                        .movable(false)
                        .fixed_pos(crosshair_position)
                        .interactable(false)
                        .show(ctx,|ui| {
                            let paint_size = Vec2::splat(crosshair_radius * 4.0);
                            let (response, painter) = ui.allocate_painter(paint_size, egui::Sense::hover());
                            painter.circle_stroke( response.rect.center(), crosshair_radius,  egui::Stroke{width:2.0, color: Color32::RED});
                        });
    }

    fn draw_remote(&mut self, ctx: &Context) {
        let new_pos;
        let mut gui_style = (*ctx.style()).clone();

        gui_style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(16.0, egui::FontFamily::Monospace)),
            (egui::TextStyle::Body, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Button, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(10.0, egui::FontFamily::Proportional)),
          ].into();
        ctx.set_style(gui_style);

    
        let mut gui_visuals = ctx.style().visuals.clone();
        gui_visuals.window_shadow = epaint::Shadow{offset: Vec2 { x: (0.0), y: (0.0) }, blur: 0.0, spread: 0.0, color: Color32::DARK_GRAY};
        gui_visuals.widgets.noninteractive.bg_stroke = epaint::Stroke {width: 1.5, color: Color32::from_rgb(138, 90, 62)};
        gui_visuals.widgets.inactive.bg_stroke = epaint::Stroke {width: 1.0, color: Color32::from_rgb(100,100,100)};
        gui_visuals.widgets.noninteractive.fg_stroke = epaint::Stroke {width: 1.0, color: Color32::from_rgb(215,210,210)};
        gui_visuals.widgets.active.fg_stroke = epaint::Stroke {width: 1.0, color: Color32::from_rgb(215,210,210)};
        ctx.set_visuals(gui_visuals);

        if self.remote_open {
            if self.gamepad_manager.is_controller_connected() {
                // The user could update this setting as often as every frame?
                // TODO(Samantha): Move this somewhere sensible.
                let configured_controller_type = self.controller_settings.controller_type();
                self.gamepad_manager.set_controller_type_detection(configured_controller_type);
            }
            // Draw the remote
            new_pos = egui::Window::new(egui::RichText::new("Exile Controller").color(Color32::from_rgb(227, 117, 0)).strong())
                                    .resizable(false)
                                    .current_pos(self.remote_pos)
                                    .constrain_to(self.screen_rect)
                                    .collapsible(false)
                                    .show(ctx,|ui| {
                                        egui::Grid::new("Remote Grid ID").min_col_width(220.0).show(ui, |ui| {
                                            let mut can_overlay_start = true;
                                            if self.gamepad_manager.is_controller_connected() {
                                                let controller_label =  self.gamepad_manager.get_connected_controller_label();
                                                ui.label(String::from("Controller connected: ") + controller_label.as_str());
                                            //     let mut selected = 0 as usize;
                                            //     egui::ComboBox::from_label("Select Connected Controller:")
                                            //     .selected_text(format!("{:?}", selected))
                                            //     .show_index(ui, &mut selected, connected_controllers.len(), |i| connected_controllers[i].1.to_owned());
                                            //     self.gamepad_manager.select_connected_controller(connected_controllers.get(selected).unwrap().0);
                                            // } else {
                                            //     let mut selected = 0 as usize;
                                            //     egui::ComboBox::from_label("Connect a controller").selected_text("None connected").show_index(ui, &mut selected, 1, |_i| "".to_string());
                                            } else {
                                                ui.label(String::from("No controller connected."));
                                                can_overlay_start = false;
                                            }
                                            ui.end_row();
                                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                                let start_button = ui.add_enabled(can_overlay_start, egui::Button::new("Start Controller Input"));
                                                if start_button.clicked() {
                                                    self.remote_open = false;
                                                    self.game_input_started = true;
                                                }
                                                let quit_button = ui.add_enabled(true, egui::Button::new("Quit"));
                                                if quit_button.clicked() {
                                                    exit(0);
                                                }
                                            });
                                        });
                                    }).unwrap().response.rect.left_top();
        } else {
            // Draw the minimized remote
            new_pos =  egui::Window::new("Exile Controller Minimized Remote")
                                    .resizable(false)
                                    .current_pos(self.remote_pos)
                                    .constrain_to(self.screen_rect)
                                    .title_bar(false)
                                    .show(ctx,|ui| {
                                        egui::Grid::new("Pause Grid ID").min_col_width(220.0).show(ui, |ui| {
                                            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                                let pause_button = ui.button(egui::RichText::new("Pause Overlay")
                                                                                            .color(Color32::from_rgb(227, 117, 0))
                                                                                            .size(14.0)
                                                                                        ).on_hover_text("Pause Controller Input");
                                                if pause_button.clicked() {
                                                    self.remote_open = true;
                                                    self.game_input_started = false;
                                                }
                                            })
                                        })
                                    }).unwrap().response.rect.left_top();
        }
        self.update_remote_pos(new_pos);
    }

    fn update_remote_pos(&mut self, new_position: Pos2) {
        self.remote_pos = new_position;
    }

    // fn draw_controller_connected_label(&mut self, ctx: &Context, is_connected controller_id: String) {
    //     let label = egui::widgets::Label::new(controller_id);
    //     label.show()

    // }

    fn handle_controller_input_loop (&mut self, ctx: &Context) {
        if self.overlay_settings.windowed_mode() && self.game_window_tracker.is_poe_active() {
            self.game_action_handler.update_window_tracker();
        }
        self.game_action_handler.process_input_buttons(self.gamepad_manager.controller_state.get_all_buttons());
        self.game_action_handler.process_input_analogs(self.gamepad_manager.controller_state.get_left_analog_stick(), 
                                            self.gamepad_manager.controller_state.get_right_analog_stick());
        self.game_action_handler.handle_character_actions(ctx);
    }
}

impl EguiOverlay for GameOverlay {
    fn gui_run(
        &mut self,
        egui_context: &Context,
        _default_gfx_backend_: &mut DefaultGfxBackend,
        glfw_backend: &mut egui_overlay::egui_window_glfw_passthrough::GlfwBackend,
    ) {
        egui_extras::install_image_loaders(egui_context);

        // Sets a GLFW window to the configured size on screen. Game-rect overlay bounds are set later.
        glfw_backend.window.set_size(self.overlay_settings.screen_width() as i32, self.overlay_settings.screen_height() as i32);
        glfw_backend.window.set_pos(0, 0);

        self.draw_remote(egui_context);

        // Make sure we process gamepad events no matter what, lest we lose disconnections and connections.
        self.gamepad_manager.process_gamepad_events();
        if self.game_input_started {
            if self.overlay_settings.windowed_mode() && self.game_window_tracker.is_poe_active() {
                self.game_window_tracker.update_window_tracker();
            }
            if self.overlay_settings.show_buttons() && (self.overlay_settings.always_show_overlay() || self.game_window_tracker.is_poe_active()) {
                self.place_flask_overlay_images(egui_context, &self.overlay_images);
                self.place_face_overlay_images(egui_context, &self.overlay_images);
                self.place_mouse_button_overlay_images(egui_context, &self.overlay_images);
            }

            if self.overlay_settings.show_crosshair() && (self.overlay_settings.always_show_overlay() || self.game_window_tracker.is_poe_active()) {
                self.paint_crosshair(egui_context);
            }

            self.handle_controller_input_loop(egui_context);
            if !self.gamepad_manager.is_controller_connected() {
                self.game_input_started = false;
                self.remote_open = true;
            }
        }
        
        // The wgpu renderer panics when a frame has no vertices onscreen. 
        // This includes an offscreen remote or only images being drawn.

        #[cfg(target_os = "linux")]
        egui_backend::egui::Area::new("No Crash Rectangle")
                                        .default_pos(Pos2{x:0.0,y:0.0})
                                        .show(egui_context,|ui| { 
                                            let size = Vec2::splat(1.0);
                                            let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
                                            painter.rect(response.rect, 
                                                            egui::Rounding{ nw: 0.0, ne: 0.0, sw: 0.0, se: 0.0 }, 
                                                            egui::Color32::RED, 
                                                            egui::Stroke{width:0.0, color:egui::Color32::TRANSPARENT});
                                        });

        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            glfw_backend.window.set_mouse_passthrough(false);
        } else {
            glfw_backend.window.set_mouse_passthrough(true);
        }
        egui_context.request_repaint();
    }
}

pub fn start_overlay(overlay_settings: OverlaySettings, 
                     controller_settings: ControllerSettings, 
                     gamepad_manager: GamepadManager,
                     game_action_handler: ActionManager, 
                     game_window_tracker: GameWindowTracker) {
    let screen_width = overlay_settings.screen_width();
    let screen_height = overlay_settings.screen_height();
    let game_overlay = GameOverlay{
        game_window_tracker: game_window_tracker,
        overlay_settings: overlay_settings,
        screen_rect: Rect::from_two_pos(Pos2 { x: 0.0, y: 0.0 }, Pos2 {x: screen_width, y: screen_height}),
        overlay_images: OverlayImages::default(),
        controller_settings: controller_settings,
        gamepad_manager: gamepad_manager,
        game_action_handler: game_action_handler,
        remote_open: true,
        remote_pos: Pos2 { x: screen_width / 2.0 , y: screen_height / 16.0 },
        game_input_started: false,
    };
    
    overlay_backend::start(game_overlay);
}