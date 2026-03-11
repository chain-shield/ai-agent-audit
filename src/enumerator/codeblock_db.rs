/// SQLite database for code block storage and retrieval.
///
/// This module manages persistent storage of generated markdown code blocks,
/// providing efficient storage and retrieval of contextual code slices for
/// AI analysis with metadata and token counting.
use anyhow::Result;
use rusqlite::{Connection, params};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    llm_review::contract::contract_category::ContractCategory, prepare_code::git_clone::RepoPaths,
};

/// Represents a contextual markdown code block for AI analysis.
///
/// Contains generated code slices with associated metadata including token
/// counts for LLM context management and contract identification.
#[derive(Debug, Clone)]
pub struct MarkdownCodeblock {
    /// Unique identifier for the code block
    pub id: String,
    /// Contract name (e.g., "PuppyRaffle")
    pub project_id: String,
    pub contract: String,
    pub contract_category: ContractCategory,
    /// Token count for LLM context window management
    pub tokens: usize,
    /// Markdown content with code, IR, and storage information
    pub content: String,
}

/// SQLite database manager for code block persistence.
///
/// Provides high-level interface for storing and retrieving generated
/// code blocks with efficient querying and metadata management.
pub struct CodeBlocksDb {
    /// Path to the SQLite database file
    path: PathBuf,
}

impl CodeBlocksDb {
    /// Creates a new SliceDb instance or opens an existing one at the specified path.
    ///
    /// This function initializes the database schema if it doesn't already exist,
    /// creating tables for seed slices and codeblocks with appropriate indexes.
    ///
    /// # Arguments
    /// * `path` - Path to the SQLite database file
    ///
    /// # Returns
    /// * `Result<Self>` - A new SliceDb instance if successful, Error otherwise
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db = Self {
            path: path.as_ref().to_path_buf(),
        };

        // Initialize schema once
        let conn = Connection::open(&db.path)?;
        conn.execute_batch(
            r#"
                /* ───────── seed → contract link table ───────── */
                CREATE TABLE IF NOT EXISTS seed_slices(
                id                 TEXT PRIMARY KEY,      -- UUID you assign
                seed_id            TEXT,                  -- FK → seeds.id
                codeblock_id       TEXT,                  -- FK → codeblocks.id
                status             TEXT
                );

                /* speed up look-ups by seed_id or by slice_id */
                CREATE INDEX IF NOT EXISTS idx_seed_slices_seed_id
                    ON seed_slices(seed_id);
                CREATE INDEX IF NOT EXISTS idx_seed_slices_slice_id
                    ON seed_slices(codeblock_id);

                /* ───────── deduped contract bodies ─────────── */
                CREATE TABLE IF NOT EXISTS codeblocks(
                id      TEXT PRIMARY KEY,  -- sha256(body)
                project_id TEXT,
                contract TEXT,
                contract_category TEXT,
                tokens  INTEGER,
                content TEXT
                );

                CREATE INDEX IF NOT EXISTS idx_codeblocks_project_id
                    ON codeblocks(project_id);

                CREATE UNIQUE INDEX IF NOT EXISTS ux_codeblocks_project_contract
                    ON codeblocks(project_id, contract);
               "#,
        )?;

        // Migration: Add contract_category column if it doesn't exist (for existing databases)
        // This will fail silently if the column already exists
        let _ = conn.execute(
            "ALTER TABLE codeblocks ADD COLUMN contract_category TEXT",
            [],
        );

        Ok(db)
    }

    /// Retrieves the markdown content for a given seed ID.
    ///
    /// This function performs a join between the seed_slices and codeblocks tables
    /// to find the markdown content associated with a specific seed.
    ///
    /// # Arguments
    /// * `seed_id` - The ID of the seed to retrieve code for
    ///
    /// # Returns
    /// * `rusqlite::Result<String>` - The markdown content if found, Error otherwise
    pub fn get_code_for_seed(&self, seed_id: &str) -> rusqlite::Result<String> {
        let conn = Connection::open(&self.path)?;

        conn.query_row(
            r#"
                SELECT c.content
                FROM   codeblocks AS c
                JOIN   seed_slices     AS s  ON s.codeblock_id = c.id
                WHERE  s.seed_id = ?1
                LIMIT  1;
                "#,
            params![seed_id],
            |row| row.get(0),
        )
    }

    /// Inserts a new seed slice into the database.
    ///
    /// This function creates a mapping between a seed and a codeblock in the database.
    ///
    /// # Arguments
    /// * `s` - The SeedSlice to insert
    ///
    /// # Returns
    /// * `Result<()>` - Ok if successful, Error otherwise
    ///
    /// Inserts a new codeblock into the database if it doesn't already exist.
    ///
    /// This function checks if a codeblock with the same ID already exists in the database
    /// and only inserts it if it doesn't, preventing duplicate entries.
    ///
    /// # Arguments
    /// * `c` - The MarkdownCodeblock to insert
    ///
    /// # Returns
    /// * `Result<()>` - Ok if successful, Error otherwise
    pub fn insert_codeblock(&self, c: &MarkdownCodeblock) -> Result<()> {
        let conn = Connection::open(&self.path)?;

        // Insert new codeblock
        conn.execute(
            r#"INSERT INTO codeblocks VALUES (?1,?2,?3,?4,?5,?6)
                  ON CONFLICT(project_id, contract) DO UPDATE SET
                    id      = excluded.id,
                    tokens  = excluded.tokens,
                    content = excluded.content,
                    contract_category = excluded.contract_category
                    "#,
            params![
                c.id,
                c.project_id,
                c.contract,
                c.contract_category.to_string(),
                c.tokens as i64,
                c.content
            ],
        )?;
        Ok(())
    }

    /// Retrieves all contracts and their content as a HashMap.
    ///
    /// # Returns
    /// * `rusqlite::Result<HashMap<String, String>>` - HashMap mapping contract names to their content
    pub fn get_all_contracts(
        &self,
        repo: &RepoPaths,
    ) -> anyhow::Result<HashMap<String, (String, ContractCategory)>> {
        let conn = Connection::open(&self.path)?;

        let mut stmt = conn.prepare(
            "SELECT contract, contract_category, content FROM codeblocks WHERE project_id = ?1",
        )?;

        let rows = stmt.query_map([&repo.project_id.clone()], |row| {
            Ok((
                row.get::<_, String>(0)?, // contract
                row.get::<_, String>(1)?, // contract_category
                row.get::<_, String>(2)?, // content
            ))
        })?;

        let mut contracts = HashMap::new();
        for row in rows {
            let (contract, contract_category_str, content) = row?;
            let contract_category: ContractCategory = contract_category_str.parse()?;

            contracts.insert(contract, (content, contract_category));
        }

        Ok(contracts)
    }

    pub fn get_code_for_contract(
        &self,
        contract: &str,
        repo: &RepoPaths,
    ) -> rusqlite::Result<String> {
        let conn = Connection::open(&self.path)?;

        conn.query_row(
            r#"
                SELECT content
                FROM   codeblocks
                WHERE  project_id = ?1 AND contract = ?2
                LIMIT  1;
                "#,
            params![&repo.project_id, contract],
            |row| row.get(0),
        )
    }
}
