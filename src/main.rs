mod app; // Import the app module
mod pages; // Import the pages module

use app::GitHubApp; // Use the GitHubApp struct
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };
    eframe::run_native(
        "GitHub Dashboard App", // App title
        options,
        Box::new(|cc| Box::new(GitHubApp::new(cc))),
    )
}
