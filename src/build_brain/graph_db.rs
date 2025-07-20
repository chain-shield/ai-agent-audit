/// Graph database operations for semantic data storage.
///
/// This module provides a SQLite-based graph database for storing and querying
/// smart contract semantic data including functions, call relationships, and
/// inheritance hierarchies extracted from Slither analysis.
use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::Path;

/// Represents a smart contract function with complete metadata
#[derive(Debug, Clone)]
pub struct SmartContractFunction {
    /// Unique function identifier from Slither
    pub id: String,
    /// Contract name containing the function
    pub contract: String,
    /// Function name
    pub name: String,
    /// Function visibility level
    pub visibility: String,
    /// Applied function modifiers
    pub modifiers: Vec<String>,
    /// State mutability specification
    pub mutability: String,
}

/// SQLite-based graph database for semantic contract data
pub struct GraphDb(Connection);

impl GraphDb {
    /// Creates a new graph database with required schema.
    ///
    /// Initializes SQLite database with tables for functions, call edges,
    /// and inheritance relationships. Uses WAL mode for better concurrency.
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
