use egui_backend::{GfxBackend, UserApp, WindowBackend};
use crate::overlay::egui_render_wgpu::egui_render_wgpu;
use egui_window_glfw_passthrough::glfw::PixelImage;

/*
Based on https://github.com/coderedart/egui_overlay 
Reproduced here instead of importing because the egui_overlay crate relies on egui_render_wgpu, which needed to be fixed to allow properly drawing color images.
*/


fn load_pixel_icon() -> PixelImage {
    #[cfg(target_os = "windows")]
    let icon_image: &[u8] = include_bytes!("..\\img\\icon.ico");

    #[cfg(target_os = "linux")]
    let icon_image: &[u8] = include_bytes!("../img/icon.ico");

    let img = image::load_from_memory(icon_image).unwrap();
    let img_width = img.width();
    let img_height = img.height();

    let img_rgba: Vec<u8> = img.into_rgba8().to_vec();
    let mut img_pixels: Vec<u32> = Vec::with_capacity(img_rgba.len() / 4);
    for chunk in img_rgba.chunks(4) {
        let rgba_pixel = ((chunk[3] as u32) << 24) | ((chunk[2] as u32) << 16) | ((chunk[1] as u32) << 8) | ((chunk[0] as u32));
        img_pixels.push(rgba_pixel);
    }
    PixelImage {width: img_width, height: img_height, pixels: img_pixels}
}

pub fn start_egui_overlay(app: impl UserApp<egui_window_glfw_passthrough::GlfwWindow, egui_render_wgpu::WgpuBackend> + 'static, screen_width: i32, screen_height: i32) {
    let mut glfw_backend =
        egui_window_glfw_passthrough::GlfwWindow::new(Default::default(), Default::default());
    let wgpu_backend = egui_render_wgpu::WgpuBackend::new(&mut glfw_backend, Default::default());

    glfw_backend.window.set_icon_from_pixels(vec![load_pixel_icon()]);
    glfw_backend.window.set_title("Exile Controller");
    glfw_backend.window.set_size(screen_width, screen_height);
    glfw_backend.run_event_loop(wgpu_backend, app);
}