use crate::app::GitHubApp; // Import GitHubApp
use eframe::egui;

pub fn view(ui: &mut egui::Ui, app: &mut GitHubApp) { // Updated function signature
    ui.heading("Repositories");
    ui.separator();

    // Button to trigger repository fetch
    // Disable button if a fetch is already in progress
    let fetch_button_enabled = app.repo_fetch_status != "Fetching...";
    if ui.add_enabled(fetch_button_enabled, egui::Button::new("Fetch Repositories")).clicked() {
        // The app.trigger_repo_fetch() method needs access to egui::Context.
        // We can get context from `ui.ctx()`.
        app.trigger_repo_fetch(ui.ctx());
    }

    ui.add_space(5.0);

    // Display current fetch status
    ui.label(format!("Status: {}", app.repo_fetch_status));

    ui.add_space(10.0);
    ui.separator();
    ui.heading("Fetched Repositories:");
    ui.add_space(5.0);

    if app.repositories.is_empty() {
        // Specific messages based on status
        if app.repo_fetch_status == "Idle" {
            ui.label("Enter GitHub username in Settings and click 'Fetch Repositories'.");
        } else if app.repo_fetch_status.starts_with("Fetching") {
            ui.label("Loading repositories...");
             // Optionally, add a spinner here
            // ui.spinner();
        } else if app.repo_fetch_status.starts_with("Success") {
            // This covers "Success" and "Success: No repositories found..."
             ui.label("No repositories found for the user, or none match criteria.");
        } else if app.repo_fetch_status.starts_with("Error: Username is empty") {
            ui.label("Please enter a GitHub username in the Settings page.");
        }
        // If it's a different error, the status label itself will show it.
        // No specific message here if status is like "Error: API Error..."
    } else {
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            for repo in &app.repositories {
                ui.group(|ui| {
                    ui.heading(&repo.name);
                    if !repo.description.is_empty() {
                        ui.label(&repo.description);
                    } else {
                        ui.label("No description provided.");
                    }
                    ui.horizontal(|ui| {
                        let lang = if repo.language.is_empty() { "N/A" } else { &repo.language };
                        ui.label(format!("Language: {}", lang));
                        ui.label(format!("Stars: {}", repo.stargazers_count));
                    });
                    ui.hyperlink_to("Open on GitHub", &repo.html_url).on_hover_text("Opens in browser");
                });
                ui.separator(); // Separator between repo groups
            }
        });
    }
}
