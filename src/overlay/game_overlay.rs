use super::egui_overlay;
use crate::controller::action_manager::ActionManager;
use crate::controller::input::GamepadManager;
use crate::settings::{OverlaySettings, ControllerSettings};

use egui::{Vec2, Context};
use egui_backend::{egui, UserApp};
use egui_backend::egui::{Rect, Pos2};
use crate::overlay::egui_render_wgpu::egui_render_wgpu::WgpuBackend;
use egui_extras::RetainedImage;


struct OverlayImages {
    button_a: RetainedImage,
    button_b: RetainedImage,
    button_x: RetainedImage,
    button_y: RetainedImage,
    //crosshair: RetainedImage,
}

impl Default for OverlayImages {
    fn default() -> Self {
        Self {
            button_a: RetainedImage::from_image_bytes("buttonA.png",  include_bytes!("..\\img\\buttonA.png")).unwrap(),
            button_b: RetainedImage::from_image_bytes("buttonB.png", include_bytes!("..\\img\\buttonB.png")).unwrap(),
            button_x: RetainedImage::from_image_bytes("buttonX.png", include_bytes!("..\\img\\buttonX.png")).unwrap(),
            button_y: RetainedImage::from_image_bytes("buttonY.png", include_bytes!("..\\img\\buttonY.png")).unwrap(),
            //crosshair: RetainedImage::from_image_bytes("crosshair.png", include_bytes!("..\\img\\crosshair.png")).unwrap(),
        }
    }
}

struct GameOverlay {
    overlay_settings: OverlaySettings,
    window_rect: Rect,
    overlay_images: OverlayImages,
    controller_settings: ControllerSettings,
    gamepad_manager: GamepadManager, 
    game_action_handler: ActionManager,
    remote_open: bool,
    remote_pos: Pos2,
    game_input_started: bool,
}

impl GameOverlay {
    fn place_overlay_image(&self, ctx: &Context, image: &RetainedImage, position: Pos2, id_source: &str) {
        egui_backend::egui::Area::new(id_source)
                                    .movable(false)
                                    .fixed_pos(position)
                                    .show(ctx,|ui| {
                                        ui.image(image.texture_id(ctx), image.size_vec2());
                                    });
    }

    fn place_abxy_overlay_images (&self, ctx: &Context, images: &OverlayImages) {
        let x_offset = 0.8215;
        let x_offset_offset = 0.029;
        let y_offset = 0.965;
        self.place_overlay_image(ctx, &images.button_x, 
                        Pos2 { x: self.overlay_settings.screen_width() * (x_offset-x_offset_offset*3.0), y: self.overlay_settings.screen_height() * y_offset }, 
                       "button_x");
        self.place_overlay_image(ctx, &images.button_a, 
                        Pos2 { x: self.overlay_settings.screen_width() * (x_offset-x_offset_offset*2.0), y: self.overlay_settings.screen_height() * y_offset }, 
                        "button_a");
        self.place_overlay_image(ctx, &images.button_b, 
                        Pos2 { x: self.overlay_settings.screen_width() * (x_offset-x_offset_offset*1.0), y: self.overlay_settings.screen_height() * y_offset }, 
                        "button_b");
        self.place_overlay_image(ctx, &images.button_y, 
                        Pos2 { x: self.overlay_settings.screen_width() * (x_offset), y: self.overlay_settings.screen_height() * y_offset },
                        "button_y");
        /*
        // g.sprites.sprites[1] = loadSprite(overlayImgX, 0.8215-0.02875*3, 0.97)
        // g.sprites.sprites[2] = loadSprite(overlayImgA, 0.8215-0.02875*2, 0.97)
        // g.sprites.sprites[3] = loadSprite(overlayImgB, 0.8215-0.02875*1, 0.97)
        // g.sprites.sprites[4] = loadSprite(overlayImgY, 0.8215, 0.97)
        */
    }

    fn paint_crosshair (&self, ctx: &Context) {
        let crosshair_radius = 4.0;
        let crosshair_position = Pos2 { x: (self.overlay_settings.screen_width() / 2.0) - self.controller_settings.character_x_offset_px(), 
                                                y: (self.overlay_settings.screen_height() / 2.0) - self.controller_settings.character_y_offset_px() };
        egui_backend::egui::Area::new("crosshair")
                                        .movable(false)
                                        .fixed_pos(crosshair_position)
                                        .show(ctx,|ui| {
                                            let paint_size = Vec2::splat(crosshair_radius * 4.0);
                                            let (response, painter) = ui.allocate_painter(paint_size, egui::Sense::hover());
                                            painter.circle_stroke( response.rect.center(), crosshair_radius,  egui::Stroke{width:1.5, color:egui::Color32::RED});
                                        });
    }
    fn draw_remote(&mut self, ctx: &Context) {
        let new_pos;
        if self.remote_open {
            //let connected_controllers = self.gamepad_manager.get_connected_controllers();
            new_pos = egui_backend::egui::Window::new("Exile Controller Remote")
                                    .resizable(false)
                                    .drag_bounds(self.window_rect)
                                    .collapsible(true)
                                    .current_pos(self.remote_pos)
                                    .show(ctx,|ui| {
                                        // if connected_controllers.len() > 0 {
                                        //     let mut selected = 0 as usize;
                                        //     egui::ComboBox::from_label("Select Connected Controller:")
                                        //     .selected_text(format!("{:?}", selected))
                                        //     .show_index(ui, &mut selected, connected_controllers.len(), |i| connected_controllers[i].1.to_owned());
                                        //     self.gamepad_manager.select_connected_controller(connected_controllers.get(selected).unwrap().0);
                                        // } else {
                                        //     let mut selected = 0 as usize;
                                        //     egui::ComboBox::from_label("Connect a controller").selected_text("None connected").show_index(ui, &mut selected, 1, |_i| "".to_string());
                                        // }
                                        let start_button = ui.button("Start Controller Input");
                                        if start_button.clicked() {
                                            self.remote_open = false;
                                            self.game_input_started = true;
                                            //self.remote_close_widget.current_pos(self.remote_pos);
                                        }
                                    }).unwrap().response.rect.left_top();
        } else {
            new_pos =  egui_backend::egui::Window::new("Exile Controller Minimized Remote")
                                    .resizable(false)
                                    .drag_bounds(self.window_rect)
                                    .current_pos(self.remote_pos)
                                    .title_bar(false)
                                    .fixed_size(Vec2{x:100.0,y:100.0})
                                    .frame(egui::Frame::default()
                                                //.outer_margin(egui::style::Margin{ left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 })
                                                .rounding(egui::Rounding{ nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 })
                                                .shadow(egui::epaint::Shadow{extrusion: 0.0, color: egui::Color32::TRANSPARENT})
                                                .stroke(egui::Stroke{width:1.5, color:egui::Color32::WHITE})
                                    )
                                    .show(ctx,|ui| {
                                        let pause_button = ui.button("X").on_hover_text("Pause Controller Input");
                                        if pause_button.clicked() {
                                            self.remote_open = true;
                                            self.game_input_started = false;
                                            //self.remote_open_widget.current_pos(self.remote_pos);
                                        }
                                    }).unwrap().response.rect.left_top();
        }
        self.update_remote_pos(new_pos);
    }
    fn update_remote_pos(&mut self, new_position: Pos2) {
        self.remote_pos = new_position;
    }

    fn handle_controller_input_loop (&mut self) {
        self.gamepad_manager.read_latest_input();
        self.game_action_handler.process_input_buttons(self.gamepad_manager.controller_state.get_all_buttons());
        self.game_action_handler.process_input_analogs(self.gamepad_manager.controller_state.get_left_analog_stick(), 
                                            self.gamepad_manager.controller_state.get_right_analog_stick());
        self.game_action_handler.handle_character_actions();
    }
}

impl UserApp<egui_window_glfw_passthrough::GlfwWindow, WgpuBackend> for GameOverlay {
    fn run(
        &mut self,
        egui_context: &egui_backend::egui::Context,
        glfw_backend: &mut egui_window_glfw_passthrough::GlfwWindow,
        _: &mut WgpuBackend,
    ) {
        glfw_backend.window.set_size(self.overlay_settings.screen_width() as i32, self.overlay_settings.screen_height() as i32);
        glfw_backend.window.set_resizable(false);
        glfw_backend.window.set_decorated(false);
        glfw_backend.window.set_pos(0, 0);

        self.draw_remote(egui_context);

        if self.game_input_started {
            if self.overlay_settings.show_buttons() {self.place_abxy_overlay_images(egui_context, &self.overlay_images);}

            if self.overlay_settings.show_crosshair() {
                self.paint_crosshair(egui_context);
            } else {
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
            }

            self.handle_controller_input_loop();
        }

        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            glfw_backend.window.set_mouse_passthrough(false);
        } else {
            glfw_backend.window.set_mouse_passthrough(true);
        }
    }
}

pub fn start_overlay(overlay_settings: OverlaySettings, controller_settings: ControllerSettings, gamepad_manager: GamepadManager, game_action_handler: ActionManager) {
    let screen_width = overlay_settings.screen_width() as f32;
    let screen_height = overlay_settings.screen_height() as f32;
    let game_overlay = GameOverlay{
        overlay_settings: overlay_settings,
        window_rect: Rect::from_two_pos(Pos2 { x: 0.0, y: 0.0 }, Pos2 {x: screen_width as f32, y: screen_height as f32 }),
        overlay_images: OverlayImages::default(),
        controller_settings: controller_settings,
        gamepad_manager: gamepad_manager,
        game_action_handler: game_action_handler,
        remote_open: true,
        remote_pos: Pos2 { x: 200.0, y: 200.0 },
        game_input_started: false,
    };


    egui_overlay::start_egui_overlay(game_overlay);
}