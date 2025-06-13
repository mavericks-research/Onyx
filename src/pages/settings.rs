use eframe::egui;
use crate::app::GitHubApp; // Import GitHubApp

pub fn view(ui: &mut egui::Ui, app: &mut GitHubApp) { // Updated function signature
    ui.heading("Settings");
    ui.separator();

    ui.horizontal(|ui| {
        ui.label("GitHub Username:");
        ui.text_edit_singleline(&mut app.github_username);
    });

    ui.add_space(10.0);
    ui.label(format!("Current username: {}", app.github_username));
    // Add more settings specific UI elements here, e.g., for theme, sync interval
}
