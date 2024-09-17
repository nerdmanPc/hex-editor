use eframe::egui;

//#[allow(unused)]
mod app;

use app::Editor;

fn main() -> Result<(), eframe::Error> {
    /*let options = eframe::NativeOptions {
        viewport: Some(egui::vec2(1360.0, 768.0)),
        ..Default::default()
    };*/
    eframe::run_native(
        "HexEditor",
        Default::default(),
        Box::new(|cc| {
            Ok(Box::new(Editor::new(&cc)))
        })
        //Box::new(Box::new(|cc| Ok(Box::new(&dyn Editor::new(cc))))),
    )
}
