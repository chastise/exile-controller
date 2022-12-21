use egui_backend::{GfxBackend, UserApp, WindowBackend};
use egui_window_glfw_passthrough;

/*
Based on https://github.com/coderedart/egui_overlay 
Reproduced here instead of importing because the egui_overlay crate is built with an unavailable egui_render_gpu package version.
*/

pub fn start_egui_overlay(app: impl UserApp<egui_window_glfw_passthrough::GlfwWindow, egui_render_wgpu::WgpuBackend> + 'static,) {
    let mut glfw_backend =
        egui_window_glfw_passthrough::GlfwWindow::new(Default::default(), Default::default());
    let wgpu_backend = egui_render_wgpu::WgpuBackend::new(&mut glfw_backend, Default::default());

    glfw_backend.run_event_loop(wgpu_backend, app);
}

