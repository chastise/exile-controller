use std::time::{Duration, Instant};
use std::process::exit;

use super::egui_overlay;
use crate::controller::action_manager::ActionManager;
use crate::controller::input::GamepadManager;
use crate::settings::{OverlaySettings, ControllerSettings};

use egui::{Vec2, Context, epaint, Color32};
use egui_backend::{egui, UserApp};
use egui_backend::egui::{Rect, Pos2};
use crate::overlay::egui_render_wgpu::egui_render_wgpu::WgpuBackend;
use egui_extras::RetainedImage;


struct OverlayImages {
    button_d_up: RetainedImage,
    button_d_down: RetainedImage,
    button_d_left: RetainedImage,
    button_d_right: RetainedImage,

    button_a: RetainedImage,
    button_b: RetainedImage,
    button_x: RetainedImage,
    button_y: RetainedImage,

    button_l3: RetainedImage,
    button_r3: RetainedImage,
}

impl Default for OverlayImages {
    #[cfg(target_os = "windows")]
    fn default() -> Self {
        Self {
            button_d_up: RetainedImage::from_image_bytes("dpad_up.png", include_bytes!("..\\img\\dpad_up.png")).unwrap(),
            button_d_down: RetainedImage::from_image_bytes("dpad_down.png", include_bytes!("..\\img\\dpad_down.png")).unwrap(),
            button_d_left: RetainedImage::from_image_bytes("dpad_left.png", include_bytes!("..\\img\\dpad_left.png")).unwrap(),
            button_d_right: RetainedImage::from_image_bytes("dpad_right.png", include_bytes!("..\\img\\dpad_right.png")).unwrap(),

            button_a: RetainedImage::from_image_bytes("button_a.png",  include_bytes!("..\\img\\button_a.png")).unwrap(),
            button_b: RetainedImage::from_image_bytes("button_b.png", include_bytes!("..\\img\\button_b.png")).unwrap(),
            button_x: RetainedImage::from_image_bytes("button_x.png", include_bytes!("..\\img\\button_x.png")).unwrap(),
            button_y: RetainedImage::from_image_bytes("button_y.png", include_bytes!("..\\img\\button_y.png")).unwrap(),

            button_l3: RetainedImage::from_image_bytes("button_l3.png", include_bytes!("..\\img\\button_l3.png")).unwrap(),
            button_r3: RetainedImage::from_image_bytes("button_r3.png", include_bytes!("..\\img\\button_r3.png")).unwrap(),
        }
    }

    #[cfg(target_os = "linux")]
    fn default() -> Self {
        Self {
            button_d_up: RetainedImage::from_image_bytes("dpad_up.png", include_bytes!("../img/dpad_up.png")).unwrap(),
            button_d_down: RetainedImage::from_image_bytes("dpad_down.png", include_bytes!("../img/dpad_down.png")).unwrap(),
            button_d_left: RetainedImage::from_image_bytes("dpad_left.png", include_bytes!("../img/dpad_left.png")).unwrap(),
            button_d_right: RetainedImage::from_image_bytes("dpad_right.png", include_bytes!("../img/dpad_right.png")).unwrap(),

            button_a: RetainedImage::from_image_bytes("button_a.png",  include_bytes!("../img/button_a.png")).unwrap(),
            button_b: RetainedImage::from_image_bytes("button_b.png", include_bytes!("../img/button_b.png")).unwrap(),
            button_x: RetainedImage::from_image_bytes("button_x.png", include_bytes!("../img/button_x.png")).unwrap(),
            button_y: RetainedImage::from_image_bytes("button_y.png", include_bytes!("../img/button_y.png")).unwrap(),

            button_l3: RetainedImage::from_image_bytes("button_l3.png", include_bytes!("../img/button_l3.png")).unwrap(),
            button_r3: RetainedImage::from_image_bytes("button_r3.png", include_bytes!("../img/button_r3.png")).unwrap(),
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
    controller_check_timer: Instant,
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
        let y_offset = 0.97;
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
    }

    fn place_flask_overlay_images (&self, ctx: &Context, images: &OverlayImages) {
        let x_offset = 0.26;
        let x_offset_offset = 0.0242;
        let y_offset = 0.97;
        self.place_overlay_image(ctx, &images.button_d_left, 
            Pos2 { x: self.overlay_settings.screen_width() * (x_offset-x_offset_offset*4.0), y: self.overlay_settings.screen_height() * y_offset }, 
           "button_d_left");
        self.place_overlay_image(ctx, &images.button_d_down, 
                        Pos2 { x: self.overlay_settings.screen_width() * (x_offset-x_offset_offset*3.0), y: self.overlay_settings.screen_height() * y_offset }, 
                       "button_d_down");
        self.place_overlay_image(ctx, &images.button_d_right, 
                        Pos2 { x: self.overlay_settings.screen_width() * (x_offset-x_offset_offset*2.0), y: self.overlay_settings.screen_height() * y_offset }, 
                        "button_d_right");
        self.place_overlay_image(ctx, &images.button_d_up, 
                        Pos2 { x: self.overlay_settings.screen_width() * (x_offset-x_offset_offset*1.0), y: self.overlay_settings.screen_height() * y_offset }, 
                        "button_d_up");
        self.place_overlay_image(ctx, &images.button_r3, 
                        Pos2 { x: self.overlay_settings.screen_width() * (x_offset), y: self.overlay_settings.screen_height() * y_offset },
                        "button_r3");
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
        gui_visuals.window_shadow = epaint::Shadow{extrusion: 0.0, color: Color32::DARK_GRAY};
        gui_visuals.widgets.noninteractive.bg_stroke = epaint::Stroke {width: 1.5, color: Color32::from_rgb(138, 90, 62)};
        gui_visuals.widgets.inactive.bg_stroke = epaint::Stroke {width: 1.0, color: Color32::from_rgb(100,100,100)};
        gui_visuals.widgets.noninteractive.fg_stroke = epaint::Stroke {width: 1.0, color: Color32::from_rgb(215,210,210)};
        gui_visuals.widgets.active.fg_stroke = epaint::Stroke {width: 1.0, color: Color32::from_rgb(215,210,210)};
        ctx.set_visuals(gui_visuals);

        if self.remote_open {
            // Check for connected controllers
            if !self.gamepad_manager.is_controller_connected() && Instant::now() > self.controller_check_timer + Duration::from_secs(1) {
                self.controller_check_timer = Instant::now();
                self.gamepad_manager.force_check_new_controllers();
                let connected_controllers = self.gamepad_manager.get_connected_controllers();
                println!("Connected controller count: {:?}", connected_controllers.len());
                if connected_controllers.len() > 0 {
                    self.gamepad_manager.connect_to_controller(connected_controllers, 0);
                }
            }
            // Draw the remote
            new_pos = egui::Window::new(egui::RichText::new("Exile Controller").color(Color32::from_rgb(227, 117, 0)).strong())
                                    .resizable(false)
                                    .current_pos(self.remote_pos)
                                    .drag_bounds(self.window_rect)
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
            self.gamepad_manager.check_if_controller_disconnected();
        } else {
            // Draw the minimized remote
            new_pos =  egui::Window::new("Exile Controller Minimized Remote")
                                    .resizable(false)
                                    .current_pos(self.remote_pos)
                                    .drag_bounds(self.window_rect)
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
        self.gamepad_manager.read_latest_input();
        self.game_action_handler.process_input_buttons(self.gamepad_manager.controller_state.get_all_buttons());
        self.game_action_handler.process_input_analogs(self.gamepad_manager.controller_state.get_left_analog_stick(), 
                                            self.gamepad_manager.controller_state.get_right_analog_stick());
        self.game_action_handler.handle_character_actions(ctx);
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
            if self.overlay_settings.show_buttons() {
                self.place_flask_overlay_images(egui_context, &self.overlay_images);
                self.place_abxy_overlay_images(egui_context, &self.overlay_images);
            }

            if self.overlay_settings.show_crosshair() {
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

        // egui_backend::egui::Area::new("No Crash Rectangle")
        //                                 .default_pos(Pos2{x:0.0,y:0.0})
        //                                 .show(egui_context,|ui| { 
        //                                     let size = Vec2::splat(1.0);
        //                                     let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        //                                     painter.rect(response.rect, 
        //                                                     egui::Rounding{ nw: 0.0, ne: 0.0, sw: 0.0, se: 0.0 }, 
        //                                                     egui::Color32::RED, 
        //                                                     egui::Stroke{width:0.0, color:egui::Color32::TRANSPARENT});
        //                                 });

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
        remote_pos: Pos2 { x: screen_width / 2.0 , y: screen_height / 16.0 },
        game_input_started: false,
        controller_check_timer: Instant::now(),
    };

    egui_overlay::start_egui_overlay(game_overlay, screen_width as i32, screen_height as i32);
}