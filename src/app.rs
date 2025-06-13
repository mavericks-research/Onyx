mod pages;
use eframe::{egui, App, CreationContext};

// Enum to represent different pages/routes
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Route {
    Dashboard,
    Repositories,
    Issues,
    Activity,
    Settings,
}

// Main application struct
pub struct GitHubApp {
    current_route: Route,
}

impl GitHubApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (Context) to create graphics shaders and buffers that eframe renders.
        Self {
            current_route: Route::Dashboard, // Default route
        }
    }
}

impl App for GitHubApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("nav_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("GitHub Dashboard");
            });
            ui.separator();

            if ui.selectable_label(self.current_route == Route::Dashboard, "Dashboard").clicked() {
                self.current_route = Route::Dashboard;
            }
            if ui.selectable_label(self.current_route == Route::Repositories, "Repositories").clicked() {
                self.current_route = Route::Repositories;
            }
            if ui.selectable_label(self.current_route == Route::Issues, "Issues & PRs").clicked() {
                self.current_route = Route::Issues;
            }
            if ui.selectable_label(self.current_route == Route::Activity, "Activity Feed").clicked() {
                self.current_route = Route::Activity;
            }
            if ui.selectable_label(self.current_route == Route::Settings, "Settings").clicked() {
                self.current_route = Route::Settings;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_route {
                Route::Dashboard => {
                    // ui.heading("Dashboard Page"); // Remove or keep as title
                    pages::dashboard::view(ui);
                }
                Route::Repositories => {
                    // ui.heading("Repositories Page");
                    pages::repositories::view(ui);
                }
                Route::Issues => {
                    // ui.heading("Issues & PRs Page");
                    pages::issues::view(ui);
                }
                Route::Activity => {
                    // ui.heading("Activity Feed Page");
                    pages::activity::view(ui);
                }
                Route::Settings => {
                    // ui.heading("Settings Page");
                    pages::settings::view(ui);
                }
            }
        });
    }
}
