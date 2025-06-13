use eframe::egui;
use std::sync::mpsc::{channel, Sender, Receiver};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Repo {
    name: String,
    stargazers_count: u32,
    forks_count: u32,
    open_issues_count: u32,
}

#[derive(Default)]
pub struct DashboardApp {
    github_token: String,
    token_saved: bool,
    repos: Vec<Repo>,
    fetch_error: Option<String>,
    fetch_triggered: bool,
    receiver: Option<Receiver<Result<Vec<Repo>, String>>>,
}


impl eframe::App for DashboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GitHub Repo Dashboard");

            if self.token_saved {
                ui.label("✅ GitHub token saved!");

                if !self.fetch_triggered {
                    // Only trigger fetch once
                    let token = self.github_token.clone();
                    let (tx, rx) = channel();
                    self.receiver = Some(rx);
                    self.fetch_triggered = true;

                    std::thread::spawn(move || {
                        let result = tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(fetch_repos(token));
                        tx.send(result).ok();
                    });
                }

                if let Some(receiver) = &self.receiver {
                    if let Ok(result) = receiver.try_recv() {
                        match result {
                            Ok(repos) => {
                                self.repos = repos;
                                self.fetch_error = None;
                            }
                            Err(err) => {
                                self.fetch_error = Some(err);
                            }
                        }
                    }
                }

                if let Some(err) = &self.fetch_error {
                    ui.colored_label(egui::Color32::RED, format!("❌ Error: {}", err));
                }

                for repo in &self.repos {
                    ui.separator();
                    ui.label(format!(
                        "📦 {}  | ⭐ {} | 🍴 {} | 🐞 {}",
                        repo.name, repo.stargazers_count, repo.forks_count, repo.open_issues_count
                    ));
                }
            } else {
                ui.label("🔑 Enter your GitHub Personal Access Token:");

                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.github_token)
                            .hint_text("ghp_xxx...")
                            .password(true),
                    );

                    if ui.button("Save").clicked() {
                        if !self.github_token.is_empty() {
                            self.token_saved = true;
                        }
                    }
                });

                ui.label("⚠️ This token is not stored permanently.");
            }
        });
    }
}


use reqwest::Client;

async fn fetch_repos(token: String) -> Result<Vec<Repo>, String> {
    let client = Client::new();

    let response = client
        .get("https://api.github.com/user/repos")
        .bearer_auth(token)
        .header("User-Agent", "egui-client")
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    if response.status().is_success() {
        response
            .json::<Vec<Repo>>()
            .await
            .map_err(|e| format!("JSON parse error: {}", e))
    } else {
        Err(format!("GitHub API error: {}", response.status()))
    }
}
