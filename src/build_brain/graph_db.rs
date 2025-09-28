/// Graph database operations for semantic data storage.
///
/// This module provides a SQLite-based graph database for storing and querying
/// smart contract semantic data including functions, call relationships, and
/// inheritance hierarchies extracted from Slither analysis.
use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::Path;

/// Represents a smart contract function with complete metadata
#[derive(Debug, Clone, Default)]
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
              func_id TEXT,    -- 3895_changeFeeAddress
              project_id TEXT,
              contract TEXT,
              name TEXT,
              ir TEXT,
              visibility TEXT,
              modifiers TEXT,
              mutability TEXT,
              PRIMARY KEY (func_id, contract, project_id)
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

            CREATE UNIQUE INDEX IF NOT EXISTS ux_edges_triplet
              ON edges(project_id, caller, callee);

            /* NEW ↓ */
            CREATE TABLE IF NOT EXISTS inheritance(
            project_id TEXT,
            child TEXT,
            parent TEXT
            );

            CREATE UNIQUE INDEX IF NOT EXISTS ux_inheritance_triplet
              ON inheritance(project_id, child, parent);
            "#,
        )?;
        Ok(Self(conn))
    }

    pub fn insert_function(
        &self,
        func_id: &str,
        project_id: &str,
        contract: &str,
        name: &str,
        ir: &str,
        visibility: &str,
        modifiers: &str,
        mutability: &str,
    ) -> Result<()> {
        self.0.execute(
        r#"
        INSERT INTO functions (func_id, project_id, contract, name, ir, visibility, modifiers, mutability)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        ON CONFLICT(func_id, contract, project_id) DO UPDATE SET
            name       = excluded.name,
            ir         = excluded.ir,
            visibility = excluded.visibility,
            modifiers  = excluded.modifiers,
            mutability = excluded.mutability
        "#,
        params![func_id, project_id, contract, name, ir, visibility, modifiers, mutability],
    )?;
        Ok(())
    }

    pub fn insert_edge(&self, project_id: &str, caller: &str, callee: &str) -> Result<()> {
        self.0.execute(
            r#"
        INSERT INTO edges (project_id, caller, callee)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(project_id, caller, callee) DO NOTHING
        "#,
            params![project_id, caller, callee],
        )?;
        Ok(())
    }

    pub fn insert_inheritance(&self, project_id: &str, child: &str, parent: &str) -> Result<()> {
        self.0.execute(
            r#"
        INSERT INTO inheritance (project_id, child, parent)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(project_id, child, parent) DO NOTHING
        "#,
            params![project_id, child, parent],
        )?;
        Ok(())
    }
}
