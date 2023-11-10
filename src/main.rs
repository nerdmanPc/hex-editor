use eframe::egui;

#[allow(unused)]
mod app;
mod model;

use app::EditorInstance;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1360.0, 768.0)),
        ..Default::default()
    };
    eframe::run_native(
        "HexEditor",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<EditorInstance>::default()
        }),
    )
}
