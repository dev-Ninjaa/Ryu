use keyring::Entry;

const SERVICE: &str = "ryu-agent37-runner";
const USERNAME: &str = "runner-token";

/// Returns an OS-keyring entry handle for the runner bearer token.
fn entry() -> Result<Entry, String> {
    Entry::new(SERVICE, USERNAME).map_err(|e| format!("Keyring init error: {e}"))
}

/// Store `token` in the OS credential vault.
/// The token is never written to disk, logs, or the frontend.
pub fn store_token(token: &str) -> Result<(), String> {
    entry()?
        .set_password(token)
        .map_err(|e| format!("Failed to store token: {e}"))
}

/// Retrieve the stored token.  Returns `None` when no token has been saved yet.
pub fn load_token() -> Result<Option<String>, String> {
    match entry()?.get_password() {
        Ok(t) => Ok(Some(t)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Failed to load token: {e}")),
    }
}

/// Remove the stored token from the OS credential vault.
pub fn delete_token() -> Result<(), String> {
    match entry()?.delete_credential() {
        Ok(()) => Ok(()),
        // Deleting a non-existent entry is a no-op, not an error.
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Failed to delete token: {e}")),
    }
}

/// Returns whether a token is currently stored (does not expose the value).
pub fn token_is_stored() -> Result<bool, String> {
    Ok(load_token()?.is_some())
}

// ─── Tests ────────────────────────────────────────────────────────────────────
// These tests exercise the real OS keyring and are therefore integration-level.
// They MUST run single-threaded to avoid races on the shared credential slot.
#[cfg(test)]
mod tests {
    use super::*;

    /// Round-trip: store → load → delete → confirm gone.
    /// Run with: cargo test -p ryu test_credential_roundtrip -- --nocapture
    #[test]
    fn test_credential_roundtrip() {
        // Always start from a clean state regardless of prior test runs.
        let _ = delete_token();

        let token = "test-token-ryu-agent37-roundtrip";

        // Store
        store_token(token).expect("store should succeed");

        // Load
        let loaded = load_token().expect("load should succeed");
        assert_eq!(
            loaded.as_deref(),
            Some(token),
            "loaded token should match what was stored"
        );

        // Confirm stored flag
        assert!(token_is_stored().unwrap());

        // Delete
        delete_token().expect("delete should succeed");

        // Confirm gone
        assert!(!token_is_stored().unwrap());
    }

    #[test]
    fn test_delete_nonexistent_is_ok() {
        // Ensure no leftover entry before testing.
        let _ = delete_token();
        // Second delete must not error
        delete_token().expect("deleting absent entry should be a no-op");
    }
}
