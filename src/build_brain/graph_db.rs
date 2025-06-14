use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SmartContractFunction {
    pub id: String,
    pub contract: String,
    pub name: String,
    pub visibility: String,
    pub modifiers: Vec<String>,
    pub mutability: String,
}

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
              name TEXT,
              visibility TEXT,
              modifiers TEXT,
              mutability TEXT
            );
            CREATE TABLE IF NOT EXISTS edges(
            caller TEXT,
            callee TEXT
            );
            /* NEW ↓ */
            CREATE TABLE IF NOT EXISTS inheritance(
            child TEXT,
            parent TEXT
            );
            "#,
        )?;
        Ok(Self(conn))
    }

    pub fn insert_function(
        &self,
        id: &str,
        contract: &str,
        name: &str,
        visibility: &str,
        modifiers: &str,
        mutability: &str,
    ) -> Result<()> {
        self.0.execute(
            "INSERT OR IGNORE INTO functions(id, contract, name, visibility, modifiers, mutability) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
            params![id, contract, name, visibility, modifiers, mutability],
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

    pub fn insert_inheritance(&self, child: &str, parent: &str) -> Result<()> {
        self.0.execute(
            "INSERT INTO inheritance(child, parent) VALUES (?1, ?2);",
            params![child, parent],
        )?;
        Ok(())
    }
}
