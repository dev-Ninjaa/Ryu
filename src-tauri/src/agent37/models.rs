use serde::{Deserialize, Serialize};

// ─── Stored configuration (URL only — token lives in OS keyring) ──────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Agent37Config {
    /// Base URL of the remote runner, e.g. "https://host-3738.agent37.app"
    /// Empty string means not configured.
    #[serde(rename = "runnerUrl")]
    pub runner_url: String,
}

// ─── Health-check response shape expected from the runner ─────────────────────

/// Exact shape returned by `GET /health` on the runner.
/// We verify all three fields rather than treating any 200 as success.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerHealthResponse {
    pub status: String,
    pub service: String,
    pub version: u32,
}

// ─── Results surfaced to the frontend ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ConnectionResult {
    /// Health check passed and fields are valid.
    Ok {
        #[serde(rename = "runnerUrl")]
        runner_url: String,
        /// Echoes back the `version` field from the runner.
        version: u32,
    },
    /// Runner replied with a non-2xx status.
    HttpError { status: u16, message: String },
    /// The response was 200 but body fields did not match the expected shape.
    InvalidResponse { message: String },
    /// The configured URL could not be parsed or uses a disallowed scheme.
    InvalidUrl { message: String },
    /// The request timed out.
    Timeout { message: String },
    /// Any other network / transport error.
    NetworkError { message: String },
}

// ─── Token status (never exposes the token value) ─────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStatus {
    /// Whether a token is currently stored in the keyring.
    pub stored: bool,
}
