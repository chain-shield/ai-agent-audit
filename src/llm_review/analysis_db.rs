use anyhow::Result;
use rusqlite::{Connection, params};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct FindingDb {
    pub id: String, // UUID v4
    pub title: String,
    pub description: String,
    pub impact: String,           // FK → seeds.id
    pub proof_of_concept: String, // e.g. "High", "Info", …
    pub proof_of_code: String,
    pub severity: String,
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
              id                  TEXT PRIMARY KEY,
              title               TEXT,
              description         TEXT,
              impact              TEXT,
              proof_of_concept    TEXT,
              proof_of_code       TEXT,
              severity            TEXT,
            );
            "#,
        )?;
        Ok(Self(conn))
    }

    pub fn insert(&self, f: &FindingDb) -> Result<()> {
        self.0.execute(
            "INSERT INTO findings VALUES (?1,?2,?3,?4,?5,?6,?7);",
            params![
                f.id,
                f.title,
                f.description,
                f.impact,
                f.proof_of_concept,
                f.proof_of_code,
                f.severity
            ],
        )?;
        Ok(())
    }
}
