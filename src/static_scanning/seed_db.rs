// crates/seed_db/src/lib.rs
use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug)]
pub struct Seed {
    pub id: String,       // UUID
    pub detector: String, // "slither.reentrancy-local"
    pub file: String,     // relative path
    pub start_line: u64,
    pub end_line: u64,
    pub severity: String, // "High" / "Medium" / …
    pub message: String,  // markdown preferred
    pub created_at: i64,  // epoch millis
}

pub struct SeedDb(Connection);

impl SeedDb {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS seeds(
              id TEXT PRIMARY KEY,
              detector TEXT,
              file TEXT,
              start_line INTEGER,
              end_line INTEGER,
              severity TEXT,
              message TEXT,
              created_at INTEGER
            );",
        )?;
        Ok(Self(conn))
    }

    pub fn insert(&self, seed: &Seed) -> Result<()> {
        self.0.execute(
            "INSERT INTO seeds VALUES (?1,?2,?3,?4,?5,?6,?7,?8);",
            params![
                seed.id,
                seed.detector,
                seed.file,
                seed.start_line,
                seed.end_line,
                seed.severity,
                seed.message,
                seed.created_at
            ],
        )?;
        Ok(())
    }
}
