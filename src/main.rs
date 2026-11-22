#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
mod feature_ui;
mod help;
mod layout_ui;
mod preparation_ui;
mod renderer;
mod section_ui;
mod ui;
use macroquad::prelude::*;
fn config() -> Conf {
    Conf {
        window_title: "Rust 3D Print Studio".into(),
        window_width: 1320,
        window_height: 830,
        high_dpi: false,
        ..Default::default()
    }
}
#[macroquad::main(config)]
async fn main() {
    prevent_quit();
    let mut app = ui::Studio::new(std::env::args_os().nth(1).map(std::path::PathBuf::from));
    loop {
        if is_quit_requested() {
            app.request_quit();
        }
        egui_macroquad::ui(|ctx| app.frame(ctx));
        clear_background(if app.dark {
            Color::from_hex(0x171d27)
        } else {
            Color::from_hex(0xe6eeed)
        });
        app.render();
        egui_macroquad::draw();
        if app.quit {
            break;
        }
        next_frame().await;
    }
}
