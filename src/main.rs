mod settings;
fn main() {
    println!("Loading settings.toml...");
    let application_settings = settings::load_settings();
    println!("Configured resolution: height:{} | width:{}", 
                application_settings.overlay_settings().screen_height(), 
                application_settings.overlay_settings().screen_width(),
            );
}