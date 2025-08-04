/// Graph database operations for semantic data storage.
///
/// This module provides a SQLite-based graph database for storing and querying
/// smart contract semantic data including functions, call relationships, and
/// inheritance hierarchies extracted from Slither analysis.
use anyhow::Result;
use rusqlite::{params, Connection};
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
    /// IR of function
    pub ir: String,
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
              ir TEXT,
              visibility TEXT,
              modifiers TEXT,
              mutability TEXT
            );
            
            -- Add indexes to speed up queries on contract and name
            CREATE INDEX IF NOT EXISTS idx_functions_contract ON functions(contract);
            CREATE INDEX IF NOT EXISTS idx_functions_name ON functions(name);

            -- Composite index for contract + name
            CREATE INDEX IF NOT EXISTS idx_functions_contract_name ON functions(contract, name);

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
        ir: &str,
        visibility: &str,
        modifiers: &str,
        mutability: &str,
    ) -> Result<()> {
        self.0.execute(
            "INSERT OR IGNORE INTO functions(id, contract, name, ir, visibility, modifiers, mutability) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
            params![id, contract, name, ir, visibility, modifiers, mutability],
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
