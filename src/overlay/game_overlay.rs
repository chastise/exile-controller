use std::process::exit;
use std::time::Duration;


use egui::{self, IconData, Rgba, Sense, ViewportCommand};
use egui::{Area, Color32, Context, epaint, Pos2, Vec2};

use egui_extras;

use crate::controller::action_manager::ActionManager;
use crate::controller::input::GamepadManager;
use crate::game_window_tracker::GameWindowTracker;
use crate::settings::{OverlaySettings, ControllerSettings};
use crate::overlay::overlay_images::OverlayImages;

// This is right around an fps of 144 (142.9).
const REPAINT_AFTER_IDLE_TIME: Duration = Duration::from_millis(7);

struct GameOverlay {
    overlay_settings: OverlaySettings,
    game_window_tracker: GameWindowTracker,
    overlay_images: OverlayImages,
    controller_settings: ControllerSettings,
    gamepad_manager: GamepadManager, 
    game_action_handler: ActionManager,
    remote_open: bool,
    game_input_started: bool,
    selected_controller_dropdown_index: usize,
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
                                 Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*3.0),
                                        y: self.game_window_tracker.game_window_height() * y_offset},
                                 "button_face_left".to_string());
        self.place_overlay_image(ctx, &images.button_face_down(controller_type),
                                 Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*2.0),
                                        y: self.game_window_tracker.game_window_height() * y_offset},
                                 "button_face_down".to_string());
        self.place_overlay_image(ctx, &images.button_face_right(controller_type),
                                 Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*1.0),
                                        y: self.game_window_tracker.game_window_height() * y_offset},
                                 "button_face_right".to_string());
        self.place_overlay_image(ctx, &images.button_face_up(controller_type),
                                 Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset),
                                        y: self.game_window_tracker.game_window_height() * y_offset},
                                 "button_face_up".to_string());
    }

    fn place_flask_overlay_images (&self, ctx: &Context, images: &OverlayImages) {
        let x_offset = 0.2615;
        let x_offset_offset = 0.0242;
        let y_offset = 0.97;

        let controller_type = self.gamepad_manager.determine_controller_type();
        self.place_overlay_image(ctx, &images.button_d_left(controller_type),
                                 Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*4.0),
                                        y: self.game_window_tracker.game_window_height() * y_offset},
                                 "button_d_left".to_string());
        self.place_overlay_image(ctx, &images.button_d_down(controller_type),
                                 Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*3.0),
                                        y: self.game_window_tracker.game_window_height() * y_offset},
                                 "button_d_down".to_string());
        self.place_overlay_image(ctx, &images.button_d_right(controller_type),
                                 Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*2.0),
                                        y: self.game_window_tracker.game_window_height() * y_offset},
                                 "button_d_right".to_string());
        self.place_overlay_image(ctx, &images.button_d_up(controller_type),
                                 Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*1.0),
                                        y: self.game_window_tracker.game_window_height() * y_offset},
                                 "button_d_up".to_string());
        self.place_overlay_image(ctx, &images.button_r3(controller_type),
                                 Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset),
                                        y: self.game_window_tracker.game_window_height() * y_offset},
                                 "button_r3".to_string());
    }

    fn place_mouse_button_overlay_images (&self, ctx: &Context, images: &OverlayImages) {
        let x_offset = 0.8585;
        let x_offset_offset = 0.029;
        let y_offset = 0.909;

        let controller_type = self.gamepad_manager.determine_controller_type();
        self.place_overlay_image(ctx, &images.left_stick(controller_type),
                                 Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*2.0),
                                        y: self.game_window_tracker.game_window_height() * y_offset},
                                 "left_stick".to_string());
        self.place_overlay_image(ctx, &images.button_bumper_left(controller_type),
                                 Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset-x_offset_offset*1.0),
                                        y: self.game_window_tracker.game_window_height() * y_offset},
                                 "button_bumper_left".to_string());
        self.place_overlay_image(ctx, &images.button_bumper_right(controller_type),
                                 Pos2 { x: self.game_window_tracker.game_window_width() * (x_offset),
                                        y: self.game_window_tracker.game_window_height() * y_offset},
                                 "button_bumper_right".to_string());
    }

    fn paint_crosshair (&self, ctx: &Context) {
        let crosshair_radius = 5.0;
        // offset radius*2.0 because the paint area is radius * 4 across
        let crosshair_position = Pos2 { x: (self.game_window_tracker.game_window_width() / 2.0) - self.controller_settings.character_x_offset_px() - crosshair_radius*2.0,
                                        y: (self.game_window_tracker.game_window_height() / 2.0) - self.controller_settings.character_y_offset_px() - crosshair_radius*2.0};
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
        let mut gui_style = (*ctx.style()).clone();

        gui_style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(16.0, egui::FontFamily::Monospace)),
            (egui::TextStyle::Body, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Button, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(10.0, egui::FontFamily::Proportional)),
          ].into();
        ctx.set_style(gui_style);

        // Force dark mode to work around OS differences.
        let mut gui_visuals = egui::Visuals::dark();
        gui_visuals.window_shadow = epaint::Shadow{offset: [0, 0], blur: 0, spread: 0, color: Color32::DARK_GRAY};
        gui_visuals.widgets.noninteractive.bg_stroke = epaint::Stroke {width: 1.5, color: Color32::from_rgb(138, 90, 62)};
        gui_visuals.widgets.inactive.bg_stroke = epaint::Stroke {width: 1.0, color: Color32::from_rgb(100,100,100)};
        gui_visuals.widgets.noninteractive.fg_stroke = epaint::Stroke {width: 1.0, color: Color32::from_rgb(215,210,210)};
        gui_visuals.widgets.active.fg_stroke = epaint::Stroke {width: 1.0, color: Color32::from_rgb(215,210,210)};
        ctx.set_visuals(gui_visuals);

        if self.gamepad_manager.is_controller_connected() {
            // The user could update this setting as often as every frame?
            // TODO(Samantha): Move this somewhere sensible.
            let configured_controller_type = self.controller_settings.controller_type();
            self.gamepad_manager.set_controller_type_detection(configured_controller_type);
        }
        let response = egui::Window::new(egui::RichText::new("Exile Controller").color(Color32::from_rgb(227, 117, 0)).strong())
            .resizable(false)
            .collapsible(false)
            .drag_to_scroll(false)
            .title_bar(false)
            .movable(false)
            .show(ctx, |ui| {
                // This must be placed first or it will override interacting with buttons.
                let drag_rect = ui.max_rect();
                let drag_response = ui.interact(
                    drag_rect,
                    egui::Id::new("remote_drag_area"),
                    Sense::click_and_drag(),
                );

                egui::Grid::new("Remote Grid ID").min_col_width(220.0).show(ui, |ui| {
                    if self.remote_open {
                        let mut can_overlay_start = true;
                        if self.gamepad_manager.is_controller_connected() {
                            let connected_controllers = self.gamepad_manager.get_connected_controllers();
                            ui.add(egui::Label::new(egui::RichText::new("Select from connected controllers:").size(14.0)).selectable(false));
                            ui.end_row();
                            // FIXME: These should be set to a max length in both the closed and open combobox display.
                            egui::ComboBox::from_id_salt("controller-select-dropdown")
                                .selected_text(format!("{:?}", &mut self.selected_controller_dropdown_index))
                                .show_index(ui, &mut self.selected_controller_dropdown_index, connected_controllers.len(), |i| connected_controllers[i].1.to_owned());
                            self.gamepad_manager.connect_to_controller(connected_controllers, self.selected_controller_dropdown_index);
                            ui.end_row();
                        } else {
                            ui.add(egui::Label::new("No controller connected.").selectable(false));
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
                    } else {
                        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                            let pause_button = ui.button(egui::RichText::new("Pause Overlay")
                                                         .color(Color32::from_rgb(227, 117, 0))
                                                         .size(14.0)
                            ).on_hover_text("Pause Controller Input");
                            if pause_button.clicked() {
                                self.remote_open = true;
                                self.game_input_started = false;
                            }
                        });
                    }
                    // TODO(Samantha): Investigate using a ui builder here and passing the drag send event to the viewport.
                    if drag_response.drag_started_by(egui::PointerButton::Primary) {
                        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
                    }
                })
            }).unwrap().response;
        ctx.send_viewport_cmd(ViewportCommand::InnerSize(response.rect.size()));
    }

    fn handle_controller_input_loop (&mut self, ctx: &Context) {
        self.game_action_handler.process_input_buttons(self.gamepad_manager.controller_state.get_all_buttons());
        self.game_action_handler.process_input_analogs(self.gamepad_manager.controller_state.get_left_analog_stick(), 
                                            self.gamepad_manager.controller_state.get_right_analog_stick());
        self.game_action_handler.handle_character_actions(ctx);
    }

    fn draw_overlay_viewport(&mut self, egui_context: &egui::Context) {
        if self.overlay_settings.windowed_mode() && self.game_window_tracker.is_poe_active() {
            self.game_action_handler.update_window_tracker();
        }
        let window_position = Pos2{x: self.game_window_tracker.game_window_pos_x(), y: self.game_window_tracker.game_window_pos_y()};
        let inner_size = [self.game_window_tracker.game_window_width(), self.game_window_tracker.game_window_height()];
        egui_context.show_viewport_immediate(
            egui::ViewportId::from_hash_of("Overlay Images"),
            egui::ViewportBuilder::default()
                .with_decorations(false)
                .with_position(window_position)
                .with_has_shadow(false)
                .with_transparent(true)
                .with_mouse_passthrough(true)
                .with_always_on_top()
                .with_taskbar(false)
                .with_inner_size(inner_size),
            |inner_ctx, _class|{
                inner_ctx.send_viewport_cmd(ViewportCommand::OuterPosition(window_position));
                if self.overlay_settings.windowed_mode() && self.game_window_tracker.is_poe_active() {
                    self.game_window_tracker.update_window_tracker();
                }
                if self.overlay_settings.show_buttons() && (self.overlay_settings.always_show_overlay() || self.game_window_tracker.is_poe_active()) {
                    self.place_flask_overlay_images(inner_ctx, &self.overlay_images);
                    self.place_face_overlay_images(inner_ctx, &self.overlay_images);
                    self.place_mouse_button_overlay_images(inner_ctx, &self.overlay_images);
                }

                if self.overlay_settings.show_crosshair() && (self.overlay_settings.always_show_overlay() || self.game_window_tracker.is_poe_active()) {
                    self.paint_crosshair(inner_ctx);
                }

                self.handle_controller_input_loop(egui_context);
                if !self.gamepad_manager.is_controller_connected() {
                    self.game_input_started = false;
                    self.remote_open = true;
                }
            });
    }
}

impl eframe::App for GameOverlay {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, egui_context: &egui::Context, _frame: &mut eframe::Frame) {
        // Make sure we process gamepad events no matter what, lest we lose disconnections and connections.
        self.gamepad_manager.process_gamepad_events();

        self.draw_remote(egui_context);
        if self.game_input_started {
            self.draw_overlay_viewport(&egui_context);
        }

        // FIXME(Samantha): This is necessary so that we keep processing gamepad events at the moment.
        // Move gamepad events/processing to a different thread?
        egui_context.request_repaint_after(REPAINT_AFTER_IDLE_TIME);
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
        overlay_images: OverlayImages::default(),
        controller_settings: controller_settings,
        gamepad_manager: gamepad_manager,
        game_action_handler: game_action_handler,
        remote_open: true,
        game_input_started: false,
        selected_controller_dropdown_index: 0,
    };
    
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 130.0])
            .with_position(Pos2 { x: screen_width / 2.0 , y: screen_height / 16.0 })
            .with_decorations(false)
            .with_transparent(true)
            .with_has_shadow(false)
            .with_active(true)
            .with_always_on_top()
            .with_taskbar(true)
            .with_icon({
                let icon_bytes =  include_bytes!("../../img/icon.ico");
                let image = image::load_from_memory(icon_bytes)
                    .expect("failed to find img/icon.ico!")
                    .into_rgba8();
                let (width, height) = image.dimensions();
                let rgba = image.into_raw();
                IconData{rgba, width, height}
            }),
        ..Default::default()
    };
    eframe::run_native(
        "Exile Controller",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(game_overlay))
        })).expect("Failed initialize eframe app!");
}
