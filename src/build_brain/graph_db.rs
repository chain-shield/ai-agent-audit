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
    pub project_id: String,
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
    pub fn create(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS functions(
              id TEXT PRIMARY KEY,   -- 3895_changeFeeAddress
              project_id TEXT,
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
            CREATE INDEX IF NOT EXISTS idx_project_id_functions_contract_name ON functions(project_id, contract, name);

            CREATE TABLE IF NOT EXISTS edges(
            project_id TEXT,
            caller TEXT,
            callee TEXT
            );

            /* NEW ↓ */
            CREATE TABLE IF NOT EXISTS inheritance(
            project_id TEXT,
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
        project_id: &str,
        contract: &str,
        name: &str,
        ir: &str,
        visibility: &str,
        modifiers: &str,
        mutability: &str,
    ) -> Result<()> {
        self.0.execute(
            "INSERT OR IGNORE INTO functions(id, project_id, contract, name, ir, visibility, modifiers, mutability) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
            params![id, project_id, contract, name, ir, visibility, modifiers, mutability],
        )?;
        Ok(())
    }

    pub fn insert_edge(&self, project_id: &str, caller: &str, callee: &str) -> Result<()> {
        self.0.execute(
            "INSERT INTO edges(project_id, caller, callee) VALUES (?1, ?2, ?3);",
            params![project_id, caller, callee],
        )?;
        Ok(())
    }

    pub fn insert_inheritance(&self, project_id: &str, child: &str, parent: &str) -> Result<()> {
        self.0.execute(
            "INSERT INTO inheritance(project_id, child, parent) VALUES (?1, ?2, ?3);",
            params![project_id, child, parent],
        )?;
        Ok(())
    }
}
