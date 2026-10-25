pub mod credentials;
pub mod health;
pub mod models;
pub mod store;

pub use models::{Agent37Config, ConnectionResult, TokenStatus};
pub use store::Agent37Store;

use std::sync::Arc;
use tauri::State;

// ─── Shared state slice ───────────────────────────────────────────────────────

pub struct Agent37State {
    pub store: Arc<Agent37Store>,
}

// ─── Tauri commands ───────────────────────────────────────────────────────────

/// Return the current (non-secret) configuration.
/// The bearer token is never included in this response.
#[tauri::command]
pub fn agent37_get_config(state: State<'_, crate::AppState>) -> Result<Agent37Config, String> {
    state
        .agent37
        .store
        .load()
        .map_err(|e| format!("Failed to load Agent37 config: {e}"))
}

/// Persist the runner base URL.
/// Whitespace is trimmed; an empty string clears the setting.
#[tauri::command]
pub fn agent37_set_runner_url(
    url: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let trimmed = url.trim();

    // Basic scheme check before storage — full validation happens at connect time.
    if !trimmed.is_empty() {
        health::build_health_url(trimmed).map_err(|e| format!("Invalid URL: {e}"))?;
    }

    state
        .agent37
        .store
        .save_runner_url(trimmed)
        .map_err(|e| format!("Failed to save runner URL: {e}"))
}

/// Store the bearer token in the OS credential vault.
/// The token is accepted from the frontend exactly once and is never echoed back.
#[tauri::command]
pub fn agent37_set_token(token: String) -> Result<(), String> {
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return Err("Token must not be empty.".to_string());
    }
    credentials::store_token(trimmed)
}

/// Delete the stored bearer token from the OS credential vault.
#[tauri::command]
pub fn agent37_delete_token() -> Result<(), String> {
    credentials::delete_token()
}

/// Report whether a token is currently stored, without revealing its value.
#[tauri::command]
pub fn agent37_token_status() -> Result<TokenStatus, String> {
    Ok(TokenStatus {
        stored: credentials::token_is_stored()?,
    })
}

/// Perform an authenticated health check against the configured runner URL.
/// Returns a typed `ConnectionResult` — never a raw bool.
#[tauri::command]
pub async fn agent37_test_connection(
    state: State<'_, crate::AppState>,
) -> Result<ConnectionResult, String> {
    let config = state
        .agent37
        .store
        .load()
        .map_err(|e| format!("Failed to load config: {e}"))?;

    Ok(health::check(&config.runner_url).await)
}
