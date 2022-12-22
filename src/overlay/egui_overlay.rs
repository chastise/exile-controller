use egui_backend::{GfxBackend, UserApp, WindowBackend};
use egui_window_glfw_passthrough;
use crate::overlay::egui_render_wgpu::egui_render_wgpu;

/*
Based on https://github.com/coderedart/egui_overlay 
Reproduced here instead of importing because the egui_overlay crate relies on egui_render_wgpu, 
  which needed to be fixed to allow properly drawing color images.
*/

pub fn start_egui_overlay(app: impl UserApp<egui_window_glfw_passthrough::GlfwWindow, egui_render_wgpu::WgpuBackend> + 'static,) {
    let mut glfw_backend =
        egui_window_glfw_passthrough::GlfwWindow::new(Default::default(), Default::default());
    let wgpu_backend = egui_render_wgpu::WgpuBackend::new(&mut glfw_backend, Default::default());

    glfw_backend.run_event_loop(wgpu_backend, app);
}