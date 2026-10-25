use super::models::Agent37Config;
use rusqlite::{Connection, Result as SqlResult};
use std::sync::Mutex;

/// Persists the non-secret Agent37 configuration (runner URL) in SQLite.
/// The bearer token is never stored here — it lives in the OS keyring.
pub struct Agent37Store {
    conn: Mutex<Connection>,
}

const TABLE_DDL: &str = "
    CREATE TABLE IF NOT EXISTS agent37_config (
        key   TEXT PRIMARY KEY,
        value TEXT NOT NULL
    )
";

impl Agent37Store {
    pub fn new(db_path: &str) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(TABLE_DDL, [])?;
        Ok(Agent37Store {
            conn: Mutex::new(conn),
        })
    }

    /// Load the stored configuration.  Returns a default (empty) config if
    /// nothing has been saved yet.
    pub fn load(&self) -> SqlResult<Agent37Config> {
        let conn = self.conn.lock().unwrap();
        let runner_url: String = conn
            .query_row(
                "SELECT value FROM agent37_config WHERE key = 'runner_url'",
                [],
                |row| row.get(0),
            )
            .unwrap_or_default();

        Ok(Agent37Config { runner_url })
    }

    /// Persist the runner base URL.
    pub fn save_runner_url(&self, url: &str) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO agent37_config (key, value) VALUES ('runner_url', ?1)",
            [url],
        )?;
        Ok(())
    }

    /// Remove the stored URL, effectively resetting the configuration.
    #[allow(dead_code)]
    pub fn clear(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM agent37_config", [])?;
        Ok(())
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn in_memory() -> Agent37Store {
        Agent37Store::new(":memory:").unwrap()
    }

    #[test]
    fn test_default_config_is_empty() {
        let store = in_memory();
        let cfg = store.load().unwrap();
        assert_eq!(cfg.runner_url, "");
    }

    #[test]
    fn test_save_and_load_url() {
        let store = in_memory();
        store
            .save_runner_url("https://example.agent37.app")
            .unwrap();
        let cfg = store.load().unwrap();
        assert_eq!(cfg.runner_url, "https://example.agent37.app");
    }

    #[test]
    fn test_overwrite_url() {
        let store = in_memory();
        store.save_runner_url("https://first.example.com").unwrap();
        store.save_runner_url("https://second.example.com").unwrap();
        let cfg = store.load().unwrap();
        assert_eq!(cfg.runner_url, "https://second.example.com");
    }

    #[test]
    fn test_clear_resets_url() {
        let store = in_memory();
        store
            .save_runner_url("https://example.agent37.app")
            .unwrap();
        store.clear().unwrap();
        let cfg = store.load().unwrap();
        assert_eq!(cfg.runner_url, "");
    }
}
