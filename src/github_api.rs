use crate::state::RepositoryInfo;
use reqwest::blocking::Client; // Use blocking client
use reqwest::header::{ACCEPT, USER_AGENT};

// Sticking to reqwest::Error as per plan for now for the return type.
// The channel in app.rs is Result<Vec<state::RepositoryInfo>, String>,
// so the calling code in app.rs will map this reqwest::Error to String.
pub fn fetch_repositories_blocking(username: &str) -> Result<Vec<RepositoryInfo>, reqwest::Error> {
    if username.trim().is_empty() {
        // The GitHub API will return a 404 for an empty username,
        // which will be handled by the status check below.
        // Alternatively, could return a custom error:
        // return Err(reqwest::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Username is empty")));
    }

    let url = format!("https://api.github.com/users/{}/repos", username);

    let client = Client::new();
    let response = client
        .get(&url)
        .header(USER_AGENT, "github-dashboard-rust-app-blocking") // GitHub API requires User-Agent
        .header(ACCEPT, "application/vnd.github.v3+json")
        .send()?; // Propagates reqwest::Error

    if !response.status().is_success() {
        // Return an error with status code and perhaps body
        return Err(response.error_for_status().unwrap_err()); // Converts non-2xx to Error
    }

    let repos: Vec<RepositoryInfo> = response.json()?; // Propagates reqwest::Error for parsing

    Ok(repos)
}

// Keep the async version structure for future, but comment out or make it distinct
// pub async fn fetch_repositories_async(username: &str) -> Result<Vec<RepositoryInfo>, reqwest::Error> { ... }


pub fn api_exists() -> bool {
    true
}
