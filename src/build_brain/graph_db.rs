use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::Path;

pub struct GraphDb(Connection);

impl GraphDb {
    pub fn create(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            CREATE TABLE IF NOT EXISTS functions(
              id TEXT PRIMARY KEY,   -- 3895_changeFeeAddress
              contract TEXT,
              name TEXT
            );
            CREATE TABLE IF NOT EXISTS edges(
              caller TEXT,
              callee TEXT
            );
            "#,
        )?;
        Ok(Self(conn))
    }

    pub fn insert_function(&self, id: &str, contract: &str, name: &str) -> Result<()> {
        self.0.execute(
            "INSERT OR IGNORE INTO functions(id, contract, name) VALUES (?1, ?2, ?3);",
            params![id, contract, name],
        )?;
        Ok(())
    }

    pub fn insert_edge(&self, caller: &str, callee: &str) -> Result<()> {
        self.0.execute(
            "INSERT INTO edges(caller, callee) VALUES (?1, ?2);",
            params![caller, callee],
        )?;
        Ok(())
    }
}
