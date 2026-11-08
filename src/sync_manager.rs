use rusqlite::Connection;
use serde::{Serialize, Deserialize};
use std::io::Write;
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncEvent {
    pub id: i64,
    pub timestamp: i64,
    pub hash: String,
    pub manifold_weight: f64,
}

pub struct SyncManager {
    db_path: String,
}

impl SyncManager {
    pub fn new(db_path: &str) -> Self {
        Self {
            db_path: db_path.to_string(),
        }
    }

    pub fn verify_state_integrity(&self) -> rusqlite::Result<bool> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS telemetry_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER,
                hash TEXT,
                manifold_weight REAL
            )",
            [],
        )?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM telemetry_events;")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(count >= 0)
    }

    pub fn fetch_pending_events(&self, limit: i64) -> rusqlite::Result<Vec<SyncEvent>> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, hash, manifold_weight FROM telemetry_events ORDER BY id ASC LIMIT ?1"
        )?;
        let event_iter = stmt.query_map([limit], |row| {
            Ok(SyncEvent {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                hash: row.get(2)?,
                manifold_weight: row.get(3)?,
            })
        })?;

        let mut events = Vec::new();
        for event in event_iter {
            events.push(event?);
        }
        Ok(events)
    }

    pub fn handle_client(&self, mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let events = self.fetch_pending_events(50)?;
        let payload = serde_json::to_string(&events).unwrap_or_else(|_| "[]".to_string());
        stream.write_all(payload.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_manager_integrity() {
        let manager = SyncManager::new("manifold_state.db");
        assert!(manager.verify_state_integrity().is_ok());
    }

    #[test]
    fn test_fetch_pending_events() {
        let manager = SyncManager::new("manifold_state.db");
        let events = manager.fetch_pending_events(10);
        assert!(events.is_ok());
    }
}
