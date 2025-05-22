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

    pub fn all_seeds(&self) -> Result<Vec<Seed>> {
        let mut stmt = self.0.prepare(
            "SELECT id,
                detector,
                file,
                start_line,
                end_line,
                severity,
                message,
                created_at
         FROM seeds",
        )?;

        // query_map → Iterator<Result<Seed>>
        let rows = stmt.query_map(params![], |row| {
            Ok(Seed {
                id: row.get(0)?,
                detector: row.get(1)?,
                file: row.get(2)?,
                start_line: row.get(3)?,
                end_line: row.get(4)?,
                severity: row.get(5)?,
                message: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;

        // transpose Iterator<Result<…>>  → Result<Vec<…>>
        let seeds: Result<Vec<Seed>, _> = rows.collect();
        let seeds = seeds?;

        Ok(seeds)
    }
}
