use super::egui_overlay;
use crate::settings::OverlaySettings;

use egui::{Vec2, Context, Frame};
use egui_backend::{egui, UserApp};
use egui_backend::egui::{Rect, Pos2};
use crate::overlay::egui_render_wgpu::egui_render_wgpu::WgpuBackend;
use egui_extras::RetainedImage;



struct OverlayImages {
    button_a: RetainedImage,
    button_b: RetainedImage,
    button_x: RetainedImage,
    button_y: RetainedImage,
    crosshair: RetainedImage,
}

impl Default for OverlayImages {
    fn default() -> Self {
        Self {
            button_a: RetainedImage::from_image_bytes("buttonA.png",  include_bytes!("..\\img\\buttonA.png")).unwrap(),
            button_b: RetainedImage::from_image_bytes("buttonB.png", include_bytes!("..\\img\\buttonB.png")).unwrap(),
            button_x: RetainedImage::from_image_bytes("buttonX.png", include_bytes!("..\\img\\buttonX.png")).unwrap(),
            button_y: RetainedImage::from_image_bytes("buttonY.png", include_bytes!("..\\img\\buttonY.png")).unwrap(),
            crosshair: RetainedImage::from_image_bytes("crosshair.png", include_bytes!("..\\img\\crosshair.png")).unwrap(),
        }
    }
}

struct GameOverlay {
    pub window_height: f32,
    pub window_width: f32,
    pub text: String,
    pub overlay_images: OverlayImages,
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
        self.place_overlay_image(ctx, &images.crosshair, 
                        Pos2 { x: (self.window_width / 2.0) - 16.0, y: (self.window_height / 2.0) - 100.0 }, 
                        "crosshair");
        /*
        	// g.sprites.sprites[0] = loadSprite(overlayImgCross, 0, 0)

	// g.sprites.sprites[1] = loadSprite(overlayImgX, 0.8215-0.02875*3, 0.97)
	// g.sprites.sprites[2] = loadSprite(overlayImgA, 0.8215-0.02875*2, 0.97)
	// g.sprites.sprites[3] = loadSprite(overlayImgB, 0.8215-0.02875*1, 0.97)
	// g.sprites.sprites[4] = loadSprite(overlayImgY, 0.8215, 0.97)
        */

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

        let window_rectangle = Rect::from_two_pos(Pos2 { x: 0.0, y: 0.0 },
                                                        Pos2 {x: self.window_width as f32, y: self.window_height as f32 });
        let mut open_me = false;
        egui_backend::egui::Window::new("Exile Controller Remote")
                                    .resizable(false)
                                    .drag_bounds(window_rectangle)
                                    .collapsible(true)
                                    .default_pos(Pos2 { x: 0.0, y: 0.0 })
                                    .show(egui_context,|ui| {
                                        ui.text_edit_multiline(&mut self.text);
                                    });
        egui_backend::egui::Window::new("Exile Minimized Remote")
                                    .resizable(false)
                                    .drag_bounds(window_rectangle)
                                    .default_pos(Pos2 { x: 100.0, y: 100.0 })
                                    .title_bar(false)
                                    .fixed_size(Vec2{x:100.0,y:100.0})
                                    .frame(egui::Frame::default()
                                                .outer_margin(egui::style::Margin{ left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 })
                                                .rounding(egui::Rounding{ nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 })
                                                .shadow(egui::epaint::Shadow{extrusion: 0.0, color: egui::Color32::TRANSPARENT})
                                                .stroke(egui::Stroke{width:1.5, color:egui::Color32::WHITE})
                                        )
                                    .show(egui_context,|ui| {
                                        ui.button("X");
                                    });

        self.place_abxy_overlay_images(egui_context, &self.overlay_images);

        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            glfw_backend.window.set_mouse_passthrough(false);
        } else {
            glfw_backend.window.set_mouse_passthrough(true);
        }
    }
}

pub fn start_overlay(overlay_settings: OverlaySettings) {
    let game_overlay = GameOverlay{
        window_height: overlay_settings.screen_height(),
        window_width: overlay_settings.screen_width(),
        text: "asdw".to_string(),
        overlay_images: OverlayImages::default(),
    };
    egui_overlay::start_egui_overlay(game_overlay);
}