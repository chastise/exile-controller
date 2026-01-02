use std::fs;
use std::time::Duration;


use egui::{Context, PlatformOutput};

#[cfg(not(target_os = "macos"))]
use egui_overlay::egui_render_three_d;
#[cfg(not(target_os = "macos"))]
use egui_overlay::egui_render_three_d::ThreeDBackend as DefaultGfxBackend;

// Mac is not supported until wgpu dependencies get fixed by several crates with pinned incompatible versions
// #[cfg(target_os = "macos")]
// use egui_render_wgpu;
// #[cfg(target_os = "macos")]
// use egui_render_wgpu::WgpuBackend as DefaultGfxBackend;

use egui_window_glfw_passthrough::{GlfwBackend, GlfwConfig, glfw::PixelImage};

fn load_pixel_icon() -> PixelImage {
    let icon_image: &[u8] = &fs::read("img/icon.ico").unwrap();

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

pub fn start<T: EguiOverlay + 'static>(user_data: T) {
    let mut glfw_backend = GlfwBackend::new(GlfwConfig {
        glfw_callback: Box::new(|gtx| {
            (egui_window_glfw_passthrough::GlfwConfig::default().glfw_callback)(gtx);
            // Can add more glfw window hints here
            // gtx.window_hint(egui_window_glfw_passthrough::glfw::WindowHint::ScaleToMonitor(true));
        }),
        #[cfg(not(target_os = "macos"))]
        opengl_window: Some(true),
        #[cfg(target_os = "macos")]
        opengl_window: Some(false),

        transparent_window: Some(true),
        ..Default::default()
    });
    // Disable user resizing
    glfw_backend.window.set_resizable(false);
    // Always on top
    glfw_backend.window.set_floating(true);
    // Disable borders/titlebar
    glfw_backend.window.set_decorated(false);

    // Set overlay app icon & title
    glfw_backend.window.set_icon_from_pixels(vec![load_pixel_icon()]);
    glfw_backend.window.set_title("Exile Controller");

    let latest_size = glfw_backend.window.get_framebuffer_size();
    let latest_size = [latest_size.0 as _, latest_size.1 as _];

    #[cfg(not(target_os = "macos"))]
    let default_gfx_backend = {
        DefaultGfxBackend::new(
            egui_render_three_d::ThreeDConfig {
                ..Default::default()
            },
            |s| glfw_backend.get_proc_address(s),
            latest_size,
        )
    };
    #[cfg(target_os = "macos")]
    let default_gfx_backend = DefaultGfxBackend::new(
        egui_render_wgpu::WgpuConfig {
            ..Default::default()
        },
        Some(Box::new(glfw_backend.window.render_context())),
        latest_size,
    );

    let overlap_app = OverlayApp {
        user_data,
        egui_context: Default::default(),
        default_gfx_backend,
        glfw_backend,
    };
    overlap_app.enter_event_loop();
}


/// Implement this trait for your struct containing data you need. Then, call [`start`] fn with that data
pub trait EguiOverlay {
    fn gui_run(
        &mut self,
        egui_context: &Context,
        default_gfx_backend: &mut DefaultGfxBackend,
        glfw_backend: &mut GlfwBackend,
    );
    fn run(
        &mut self,
        egui_context: &Context,
        default_gfx_backend: &mut DefaultGfxBackend,
        glfw_backend: &mut GlfwBackend,
    ) -> Option<(PlatformOutput, Duration)> {
        let input = glfw_backend.take_raw_input();
        // takes a closure that can provide latest framebuffer size.
        // because some backends like vulkan/wgpu won't work without reconfiguring the surface after some sort of resize event unless you give it the latest size
        default_gfx_backend.prepare_frame(|| {
            let latest_size = glfw_backend.window.get_framebuffer_size();
            [latest_size.0 as _, latest_size.1 as _]
        });
        egui_context.begin_pass(input);
        self.gui_run(egui_context, default_gfx_backend, glfw_backend);

        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output,
        } = egui_context.end_pass();
        let meshes = egui_context.tessellate(shapes, pixels_per_point);
        let repaint_after = viewport_output
            .into_iter()
            .map(|f| f.1.repaint_delay)
            .collect::<Vec<Duration>>()[0];

        default_gfx_backend.render_egui(meshes, textures_delta, glfw_backend.window_size_logical);
        use egui_window_glfw_passthrough::glfw::Context; // TODO (chastise): Why does this only work here?
        glfw_backend.window.swap_buffers();

        Some((platform_output, repaint_after))
    }
}

pub struct OverlayApp<T: EguiOverlay + 'static> {
    pub user_data: T,
    pub egui_context: Context,
    pub default_gfx_backend: DefaultGfxBackend,
    pub glfw_backend: GlfwBackend,
}

impl<T: EguiOverlay + 'static> OverlayApp<T> {
    pub fn enter_event_loop(mut self) {
        // polls for events and returns if there's some activity.
        // But if there is no event for the specified duration, it will return anyway.
        // used by "reactive" apps which don't do anything unless there's some event.
        // tracing::info!("entering glfw event loop");
        let mut wait_events_duration = std::time::Duration::ZERO;
        let callback = move || {
            let Self {
                user_data,
                egui_context,
                default_gfx_backend,
                glfw_backend,
            } = &mut self;
            glfw_backend
                .glfw
                .wait_events_timeout(wait_events_duration.as_secs_f64());

            // gather events
            glfw_backend.tick();

            if glfw_backend.resized_event_pending {
                let latest_size = glfw_backend.window.get_framebuffer_size();
                default_gfx_backend.resize_framebuffer([latest_size.0 as _, latest_size.1 as _]);
                glfw_backend.resized_event_pending = false;
            }
            // run userapp gui function. let user do anything he wants with window or gfx backends
            if let Some((platform_output, timeout)) =
                user_data.run(egui_context, default_gfx_backend, glfw_backend)
            {
                wait_events_duration = timeout.min(std::time::Duration::from_secs(1));
                if !platform_output.copied_text.is_empty() {
                    glfw_backend
                        .window
                        .set_clipboard_string(&platform_output.copied_text);
                }
                glfw_backend.set_cursor(platform_output.cursor_icon);
            } else {
                wait_events_duration = std::time::Duration::ZERO;
            }
            glfw_backend.window.should_close()
        };

        {
            let mut callback = callback;
            loop {
                // returns if loop should close.
                if callback() {
                    // tracing::warn!("event loop is exiting");
                    break;
                }
            }
        }
    }
}