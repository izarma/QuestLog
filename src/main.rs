use egui::Pos2;
use gui::app_setup::QuestLog;
use test::dummy_data::*;

mod gui;
pub mod model;
mod test;

fn calculate_top_right_position() -> Pos2 {
    let screen_size = egui::vec2(1920.0, 1080.0); // Example - implement dynamic detection
    egui::pos2(screen_size.x - 400.0 - 20.0, 20.0) // 400px width + 20px margin
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("QuestLog")
            .with_inner_size([400.0, 600.0])
            .with_min_inner_size([300.0, 400.0])
            .with_position(calculate_top_right_position())
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_window_level(egui::viewport::WindowLevel::AlwaysOnTop)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("..\\assets\\icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        persist_window: true,
        // default_theme: eframe::Theme::Dark,
        // mouse_passthrough: true, // Requires nightly
        ..Default::default()
    };
    let (epic, quest) = create_dummy_data();
    println!("Hello, Quest {:#?}", quest);
    println!("Hello, Quest Epic {:#?}", epic);
    eframe::run_native(
        "Quest Log Overlay",
        native_options,
        Box::new(|cc| Ok(Box::new(QuestLog::new(cc, epic)))),
    )
}
