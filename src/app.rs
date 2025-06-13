use eframe::{egui, App, CreationContext};
mod pages;
mod state;
mod github_api; // Add this line

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Route {
    Dashboard,
    Repositories,
    Issues,
    Activity,
    Settings,
}

pub struct GitHubApp {
    current_route: Route,
    pub github_username: String,
    pub repositories: Vec<state::RepositoryInfo>,
    pub repo_fetch_status: String,
    // Channel to receive repository data or error string from the fetch thread
    repo_receiver: Option<std::sync::mpsc::Receiver<Result<Vec<state::RepositoryInfo>, String>>>,
}

impl GitHubApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            current_route: Route::Dashboard,
            github_username: String::from(""), // Or load from storage
            repositories: Vec::new(),
            repo_fetch_status: "Idle".to_string(),
            repo_receiver: None,
        }
    }

    // Method to trigger fetching repositories
    pub fn trigger_repo_fetch(&mut self, ctx: &egui::Context) {
        if self.github_username.trim().is_empty() {
            self.repo_fetch_status = "Error: Username is empty".to_string();
            self.repositories.clear(); // Clear any previous results
            return;
        }

        // Prevent multiple fetches if one is already in progress
        if self.repo_fetch_status == "Fetching..." && self.repo_receiver.is_some() {
            return;
        }

        self.repo_fetch_status = "Fetching...".to_string();
        self.repositories.clear(); // Clear previous results before new fetch
        let (sender, receiver) = std::sync::mpsc::channel();
        self.repo_receiver = Some(receiver);

        let username_clone = self.github_username.clone();
        let egui_ctx = ctx.clone(); // Clone context for repaint request

        std::thread::spawn(move || {
            match github_api::fetch_repositories_blocking(&username_clone) {
                Ok(repos) => {
                    let _ = sender.send(Ok(repos));
                }
                Err(e) => {
                    // Convert reqwest::Error to String for the channel
                    let _ = sender.send(Err(format!("API Error: {}", e)));
                }
            }
            egui_ctx.request_repaint(); // Request a repaint from the thread
        });
    }

    // Method to check for updates from the repository fetch thread
    fn check_for_repo_updates(&mut self) {
        if let Some(receiver) = &self.repo_receiver {
            match receiver.try_recv() {
                Ok(Ok(repos)) => {
                    self.repositories = repos;
                    if self.repositories.is_empty() {
                        self.repo_fetch_status = "Success: No repositories found or user has no public repos.".to_string();
                    } else {
                        self.repo_fetch_status = "Success".to_string();
                    }
                    self.repo_receiver = None; // Clear receiver
                }
                Ok(Err(err_msg)) => {
                    self.repositories.clear(); // Clear any old repo data
                    self.repo_fetch_status = err_msg; // Display the error string
                    self.repo_receiver = None; // Clear receiver
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    // Still fetching or no new message, status is "Fetching..."
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    // This case should ideally not happen if sender is managed correctly
                    self.repo_fetch_status = "Error: Fetch thread disconnected unexpectedly".to_string();
                    self.repo_receiver = None;
                }
            }
        }
    }
}

impl App for GitHubApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_for_repo_updates(); // Check for async updates

        egui::SidePanel::left("nav_panel").show(ctx, |ui| {
            // ... (navigation UI remains the same)
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
                    pages::dashboard::view(ui);
                }
                Route::Repositories => {
                    pages::repositories::view(ui, self); // Pass &mut GitHubApp
                }
                Route::Issues => {
                    pages::issues::view(ui);
                }
                Route::Activity => {
                    pages::activity::view(ui);
                }
                Route::Settings => {
                    pages::settings::view(ui, self); // Pass &mut GitHubApp
                }
            }
        });
    }
}
