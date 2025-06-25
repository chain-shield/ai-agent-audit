/// This module provides functionality for storing and retrieving code slices in a SQLite database.
/// It manages the persistence of markdown codeblocks generated from smart contract analysis.
use anyhow::Result;
use rusqlite::{Connection, params};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Represents a markdown codeblock containing code relevant to a vulnerability.
///
/// This struct stores the content of a code slice along with metadata such as
/// token count for LLM processing.
#[derive(Debug, Clone)]
pub struct MarkdownCodeblock {
    /// Unique identifier for the codeblock
    pub id: String,
    // i.e. PuppyRaffle.sol
    pub contract: String,
    /// Number of tokens in the content (for LLM context window management)
    pub tokens: usize,
    /// The actual markdown content containing code, IR, and storage information
    pub content: String,
}

/// Database manager for storing and retrieving code slices.
///
/// This struct provides an interface to the SQLite database that stores
/// seed slices and codeblocks.
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
                filename TEXT,
                tokens  INTEGER,
                content TEXT
                );
               "#,
        )?;
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
    // pub fn insert_seed_slice(&self, s: &SeedSlice) -> Result<()> {
    //     let conn = Connection::open(&self.path)?;
    //     conn.execute(
    //         "INSERT INTO seed_slices VALUES (?1,?2,?3,?4);",
    //         params![s.id, s.seed_id, s.codeblock_id, s.status],
    //     )?;
    //     Ok(())
    // }
    //
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

        // Check if codeblock already exists
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM codeblocks WHERE id = ?1);",
            params![c.id],
            |row| row.get(0),
        )?;

        if exists {
            // Skip insertion if codeblock already exists
            log::debug!("Skipping duplicate contract_slice with id {}", c.id);
            return Ok(());
        }

        // Insert new codeblock
        conn.execute(
            "INSERT INTO codeblocks VALUES (?1,?2,?3,?4);",
            params![c.id, c.contract, c.tokens as i64, c.content],
        )?;
        Ok(())
    }

    /// Retrieves all contracts and their content as a HashMap.
    ///
    /// # Returns
    /// * `rusqlite::Result<HashMap<String, String>>` - HashMap mapping contract names to their content
    pub fn get_all_contracts(&self) -> rusqlite::Result<HashMap<String, String>> {
        let conn = Connection::open(&self.path)?;

        let mut stmt = conn.prepare("SELECT filename, content FROM codeblocks")?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?, // filename/contract
                row.get::<_, String>(1)?, // content
            ))
        })?;

        let mut contracts = HashMap::new();
        for row in rows {
            let (contract, content) = row?;
            contracts.insert(contract, content);
        }

        Ok(contracts)
    }

    pub fn get_code_for_contract(&self, contract: &str) -> rusqlite::Result<String> {
        let conn = Connection::open(&self.path)?;

        conn.query_row(
            r#"
                SELECT content
                FROM   codeblocks
                WHERE  contract = ?1
                LIMIT  1;
                "#,
            params![contract],
            |row| row.get(0),
        )
    }
}
