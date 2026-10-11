mod ui;

use eframe::egui;
use ui::Apricot;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Apricot")
            .with_inner_size([ui::DEFAULT_WIDTH, ui::DEFAULT_HEIGHT])
            .with_min_inner_size([440.0, 520.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Apricot",
        native_options,
        Box::new(|cc| Ok(Box::new(Apricot::new(cc)))),
    )
}
