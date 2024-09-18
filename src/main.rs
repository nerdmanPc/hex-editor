//use eframe::egui;

//#[allow(unused)]
mod app;

use app::Editor;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "HexEditor",
        options,
        Box::new(|cc| {
            Ok(Box::new(Editor::new(&cc)))
        })
    )
}
