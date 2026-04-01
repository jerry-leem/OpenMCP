mod app;
mod models;
mod opencode;
mod storage;

use app::OpenMcpApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1360.0, 900.0])
            .with_min_inner_size([1120.0, 720.0])
            .with_title("OpenMCP"),
        ..Default::default()
    };

    eframe::run_native(
        "OpenMCP",
        options,
        Box::new(|cc| Ok(Box::new(OpenMcpApp::new(cc)))),
    )
}
