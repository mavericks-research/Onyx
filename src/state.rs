use serde::Deserialize;

// Information about a single GitHub repository
#[derive(Deserialize, Debug, Clone)]
pub struct RepositoryInfo {
    pub name: String,
    pub html_url: String,
    #[serde(default)] // Handle cases where description might be null
    pub description: String,
    #[serde(default)] // Handle cases where language might be null
    pub language: String,
    pub stargazers_count: u32,
    // Add other fields if needed, e.g., forks_count, open_issues_count
}

// Shared application state
// For now, we'll put these directly in `GitHubApp` struct in `app.rs`.
// If state becomes more complex, we can move it here.
// pub struct AppState {
//     pub github_username: String,
//     pub repositories: Vec<RepositoryInfo>,
//     pub repo_fetch_status: String, // e.g., "Idle", "Fetching...", "Error"
// }

// impl AppState {
//     pub fn new() -> Self {
//         Self {
//             github_username: String::new(),
//             repositories: Vec::new(),
//             repo_fetch_status: "Idle".to_string(),
//         }
//     }
// }

// Add a simple function to confirm file creation, will be removed later
pub fn state_exists() -> bool {
    true
}
