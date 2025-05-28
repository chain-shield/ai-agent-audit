

// crates/findings_db/src/lib.rs
use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct Finding {
    pub id:           String,   // UUID v4
    pub seed_id:      String,   // FK → seeds.id
    pub severity:     String,   // e.g. "High", "Info", …
    pub title:        String,
    pub message:      String,
    // pub confidence:   Option<f32>,
    // pub sources_used: Vec<u8>,
}

pub struct FindingsDb(Connection);

impl FindingsDb {
    pub fn open(p: &Path) -> Result<Self> {
        let conn = Connection::open(p)?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS findings(
              id            TEXT PRIMARY KEY,
              seed_id       TEXT,
              severity      TEXT,
              title         TEXT,
              message       TEXT,
            );
            "#,
        )?;
        Ok(Self(conn))
    }

    pub fn insert(&self, f: &Finding) -> Result<()> {
        self.0.execute(
            "INSERT INTO findings VALUES (?1,?2,?3,?4,?5);",
            params![
                f.id,
                f.seed_id,
                f.severity,
                f.title,
                f.message,
            ],
        )?;
        Ok(())
    }
}
