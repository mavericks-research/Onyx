use eframe::egui;
use std::sync::mpsc::{channel, Receiver};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)] // Added Clone
struct OwnerInfo {
    login: String,
}

#[derive(Debug, Deserialize, Clone)] // Added Clone
struct Repo {
    name: String,
    stargazers_count: u32,
    forks_count: u32,
    open_issues_count: u32,
    owner: OwnerInfo, // New field
}

#[derive(Debug, Deserialize)]
struct CommitInfo {
    sha: String,
}

#[derive(Debug, Deserialize)]
struct BranchInfo {
    name: String,
    commit: CommitInfo,
}

#[derive(Default)]
pub struct DashboardApp {
    github_token: String,
    token_saved: bool,
    repos: Vec<Repo>,
    fetch_error: Option<String>,
    fetch_triggered: bool,
    receiver: Option<Receiver<Result<Vec<Repo>, String>>>,
    selected_repo_owner: Option<String>,
    selected_repo_name: Option<String>,
    branches: Vec<BranchInfo>,
    branch_fetch_error: Option<String>,
    is_fetching_branches: bool,
    branch_receiver: Option<Receiver<Result<Vec<BranchInfo>, String>>>,
}

impl Default for DashboardApp {
    fn default() -> Self {
        Self {
            github_token: String::new(),
            token_saved: false,
            repos: Vec::new(),
            fetch_error: None,
            fetch_triggered: false,
            receiver: None,
            selected_repo_owner: None,
            selected_repo_name: None,
            branches: Vec::new(),
            branch_fetch_error: None,
            is_fetching_branches: false,
            branch_receiver: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // To import BranchInfo, CommitInfo etc.
    // serde_json is used by reqwest::Response::json() which is already a dependency,
    // so it should be available in the test context.
    // If not, it would need to be added to dev-dependencies in Cargo.toml.

    #[test]
    fn test_deserialize_branch_info() {
        let json_data = r#"
        {
          "name": "feature/new-ux",
          "commit": {
            "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8g9h0",
            "url": "https://api.github.com/repos/user/repo/commits/a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8g9h0"
          },
          "protected": false
        }
        "#;
        let result: Result<BranchInfo, _> = serde_json::from_str(json_data);
        assert!(result.is_ok(), "Failed to deserialize BranchInfo: {:?}", result.err());
        let branch_info = result.unwrap();
        assert_eq!(branch_info.name, "feature/new-ux");
        assert_eq!(branch_info.commit.sha, "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8g9h0");
    }
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

                // Make repos clickable
                let repos_clone = self.repos.clone(); // Clone to avoid borrow checker issues
                for repo in repos_clone {
                    ui.separator();
                    let repo_display_name = format!(
                        "📦 {} (Owner: {}) | ⭐ {} | 🍴 {} | 🐞 {}",
                        repo.name, repo.owner.login, repo.stargazers_count, repo.forks_count, repo.open_issues_count
                    );
                    if ui.selectable_label(self.selected_repo_name.as_ref() == Some(&repo.name), repo_display_name).clicked() {
                        if self.selected_repo_name.as_ref() != Some(&repo.name) {
                            self.selected_repo_name = Some(repo.name.clone());
                            self.selected_repo_owner = Some(repo.owner.login.clone());
                            self.is_fetching_branches = true;
                            self.branches.clear();
                            self.branch_fetch_error = None;
                            self.branch_receiver = None; // Clear previous receiver
                        }
                    }
                }

                // Fetch branches if a repo is selected and not already fetching for this selection
                if self.is_fetching_branches && self.branch_receiver.is_none() {
                    if let (Some(owner), Some(repo_name)) = (self.selected_repo_owner.clone(), self.selected_repo_name.clone()) {
                        let token = self.github_token.clone();
                        let (tx, rx) = channel();
                        self.branch_receiver = Some(rx);

                        std::thread::spawn(move || {
                            let result = tokio::runtime::Runtime::new()
                                .unwrap()
                                .block_on(fetch_branches(owner, repo_name, token));
                            tx.send(result).ok();
                        });
                    }
                }

                // Check for branch fetch results
                if let Some(receiver) = &self.branch_receiver {
                    if let Ok(result) = receiver.try_recv() {
                        match result {
                            Ok(branches_data) => {
                                self.branches = branches_data;
                                self.branch_fetch_error = None;
                            }
                            Err(err_msg) => {
                                self.branch_fetch_error = Some(err_msg);
                                self.branches.clear();
                            }
                        }
                        self.is_fetching_branches = false;
                        self.branch_receiver = None; // Fetch operation complete
                    }
                }

                // Display branches or loading/error messages
                if let Some(repo_name) = &self.selected_repo_name {
                    ui.separator();
                    ui.heading(format!("Branches for {}:", repo_name));

                    if self.is_fetching_branches {
                        ui.label("⏳ Loading branches...");
                    } else if let Some(err) = &self.branch_fetch_error {
                        ui.colored_label(egui::Color32::RED, format!("❌ Error fetching branches: {}", err));
                    } else if !self.branches.is_empty() {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for branch in &self.branches {
                                ui.label(format!("🌳 {} (commit: {})", branch.name, &branch.commit.sha[0..std::cmp::min(7, branch.commit.sha.len())]));
                            }
                        });
                    } else {
                        ui.label("No branches found for this repository.");
                    }
                } else if self.token_saved && !self.repos.is_empty() && !self.fetch_triggered {
                    // This case implies repos are not yet fetched, initial state after token save
                     ui.label("Fetching repositories...");
                }
                 else if self.token_saved && self.repos.is_empty() && self.fetch_triggered && self.fetch_error.is_none() && self.receiver.is_some() {
                    // This case implies repos are being fetched
                     ui.label("Fetching repositories...");
                }
                else if self.token_saved && !self.repos.is_empty() && self.selected_repo_name.is_none() {
                     ui.separator();
                     ui.label("Select a repository to view its branches.");
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

async fn fetch_branches(
    owner: String,
    repo_name: String,
    token: String,
) -> Result<Vec<BranchInfo>, String> {
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/branches", owner, repo_name);

    let response = client
        .get(&url)
        .bearer_auth(token)
        .header("User-Agent", "egui-client")
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    if response.status().is_success() {
        response
            .json::<Vec<BranchInfo>>()
            .await
            .map_err(|e| format!("JSON parse error: {}", e))
    } else {
        Err(format!(
            "GitHub API error: {} - {}",
            response.status(),
            response.text().await.unwrap_or_default()
        ))
    }
}
