use rusqlite::{Connection, Result};
use std::path::Path;

pub struct SqliteStore {
    pub conn: Connection,
}

impl SqliteStore {
    pub fn new(path: &str) -> Result<Self> {
        let exists = Path::new(path).exists();
        let conn = Connection::open(path)?;
        
        if !exists {
            conn.execute_batch(
                "CREATE TABLE events (
                    id TEXT PRIMARY KEY,
                    epoch INTEGER NOT NULL,
                    seq INTEGER NOT NULL,
                    payload TEXT NOT NULL,
                    parent_hash TEXT NOT NULL,
                    event_hash TEXT NOT NULL
                );
                CREATE TABLE facts (
                    id TEXT PRIMARY KEY,
                    subject TEXT NOT NULL,
                    predicate TEXT NOT NULL,
                    object TEXT NOT NULL,
                    confidence REAL NOT NULL
                );
                CREATE TABLE goals (
                    id TEXT PRIMARY KEY,
                    description TEXT NOT NULL,
                    status TEXT NOT NULL
                );"
            )?;
        }
        
        Ok(SqliteStore { conn })
    }

    pub fn save_event(&self, id: &str, epoch: u64, seq: u64, payload: &str, parent_hash: &str, event_hash: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO events VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![id, epoch, seq, payload, parent_hash, event_hash],
        )?;
        Ok(())
    }

    pub fn save_fact(&self, id: &str, subject: &str, predicate: &str, object: &str, confidence: f64) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO facts VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, subject, predicate, object, confidence],
        )?;
        Ok(())
    }
}
