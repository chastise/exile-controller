use super::egui_overlay;
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
    pub window_height: f32,
    pub window_width: f32,
    pub window_rect: Rect,
    pub overlay_images: OverlayImages,
    pub draw_abxy: bool,
    pub draw_crosshair: bool,
    pub character_x_offset_px: f32,
    pub character_y_offset_px: f32,
    pub remote_open: bool,
    pub remote_pos: Pos2,
    // pub remote_close_widget: egui_backend::egui::Window<'overlay>,
    // pub remote_open_widget: egui_backend::egui::Window<'overlay>,
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
                        Pos2 { x: self.window_width * (x_offset-x_offset_offset*3.0), y: self.window_height * y_offset }, 
                       "button_x");
        self.place_overlay_image(ctx, &images.button_a, 
                        Pos2 { x: self.window_width * (x_offset-x_offset_offset*2.0), y: self.window_height * y_offset }, 
                        "button_a");
        self.place_overlay_image(ctx, &images.button_b, 
                        Pos2 { x: self.window_width * (x_offset-x_offset_offset*1.0), y: self.window_height * y_offset }, 
                        "button_b");
        self.place_overlay_image(ctx, &images.button_y, 
                        Pos2 { x: self.window_width * (x_offset), y: self.window_height * y_offset },
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
        let crosshair_position = Pos2 { x: (self.window_width / 2.0), y: (self.window_height / 2.0) - self.character_y_offset_px };
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
            new_pos = egui_backend::egui::Window::new("Exile Controller Remote")
                                    .resizable(false)
                                    .drag_bounds(self.window_rect)
                                    .collapsible(true)
                                    .current_pos(self.remote_pos)
                                    .show(ctx,|ui| {
                                        if ui.button("Minimize me").clicked() {
                                            self.remote_open = false;
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
                                            if ui.button("Open Remote").clicked() {
                                                self.remote_open = true;
                                                //self.remote_open_widget.current_pos(self.remote_pos);
                                            }
                                    }).unwrap().response.rect.left_top();
        }
        self.update_remote_pos(new_pos);
    }
    fn update_remote_pos(&mut self, new_position: Pos2) {
        self.remote_pos = new_position;
    }
}



impl UserApp<egui_window_glfw_passthrough::GlfwWindow, WgpuBackend> for GameOverlay {
    fn run(
        &mut self,
        egui_context: &egui_backend::egui::Context,
        glfw_backend: &mut egui_window_glfw_passthrough::GlfwWindow,
        _: &mut WgpuBackend,
    ) {
        glfw_backend.window.set_size(self.window_width as i32, self.window_height as i32);
        glfw_backend.window.set_resizable(false);
        glfw_backend.window.set_decorated(false);
        glfw_backend.window.set_pos(0, 0);

        self.draw_remote(egui_context);

        if self.draw_abxy {self.place_abxy_overlay_images(egui_context, &self.overlay_images);}

        if self.draw_crosshair {self.paint_crosshair(egui_context);}

        if !self.draw_crosshair {
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


        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            glfw_backend.window.set_mouse_passthrough(false);
        } else {
            glfw_backend.window.set_mouse_passthrough(true);
        }
    }
}

pub fn start_overlay(overlay_settings: OverlaySettings, controller_settings: ControllerSettings) {
    let game_overlay = GameOverlay{
        window_height: overlay_settings.screen_height(),
        window_width: overlay_settings.screen_width(),
        window_rect: Rect::from_two_pos(Pos2 { x: 0.0, y: 0.0 }, Pos2 {x: overlay_settings.screen_width() as f32, y: overlay_settings.screen_height() as f32 }),
        overlay_images: OverlayImages::default(),
        draw_abxy: overlay_settings.show_buttons(),
        draw_crosshair: overlay_settings.show_crosshair(),
        character_x_offset_px: controller_settings.character_x_offset_px(),
        character_y_offset_px: controller_settings.character_y_offset_px(),
        remote_open: false,
        remote_pos: Pos2 { x: 200.0, y: 200.0 },
        // remote_open_widget: egui_backend::egui::Window::new("Exile Controller Remote"),
        // remote_close_widget: egui_backend::egui::Window::new("Exile Controller Minimized Remote"),                                    
    };
    // initialize remote widgets:
    // game_overlay.remote_open_widget.resizable(false)
    //                                 .drag_bounds(game_overlay.window_rect)
    //                                 .collapsible(true)
    //                                 .default_pos(game_overlay.remote_pos);
    // game_overlay.remote_close_widget.resizable(false)
    //                                 .drag_bounds(game_overlay.window_rect)
    //                                 .default_pos(game_overlay.remote_pos)
    //                                 .title_bar(false)
    //                                 .fixed_size(Vec2{x:100.0,y:100.0})
    //                                 .frame(egui::Frame::default()
    //                                             //.outer_margin(egui::style::Margin{ left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 })
    //                                             .rounding(egui::Rounding{ nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 })
    //                                             .shadow(egui::epaint::Shadow{extrusion: 0.0, color: egui::Color32::TRANSPARENT})
    //                                             .stroke(egui::Stroke{width:1.5, color:egui::Color32::WHITE})
    //                                 );

    egui_overlay::start_egui_overlay(game_overlay);
}