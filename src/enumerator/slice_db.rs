use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};

// pub struct SliceDb(Connection);

#[derive(Debug)]
pub struct SeedSlice {
    pub id: String,           // UUID
    pub seed_id: String,      // FK → seeds.id
    pub codeblock_id: String, // codeblocks.id
    pub status: String,       // NEW / DONE / ERROR
}

#[derive(Debug, Clone)]
pub struct MarkdownCodeblock {
    pub id: String, // sha256(body)
    pub tokens: usize,
    pub content: String, // concatenated Markdown
}

pub struct SliceDb {
    path: PathBuf,
}

impl SliceDb {
    /// Create or open an on-disk DB.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db = Self {
            path: path.as_ref().to_path_buf(),
        };

        // initialise schema once
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
                tokens  INTEGER,
                content TEXT
                );
               "#,
        )?;
        Ok(db)
    }

    /// Return the Markdown body for a given Slither `seed_id`.
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

    /// (Used by the enumerator) – insert a new slice.
    pub fn insert_seed_slice(&self, s: &SeedSlice) -> Result<()> {
        let conn = Connection::open(&self.path)?;
        conn.execute(
            "INSERT INTO seed_slices VALUES (?1,?2,?3,?4);",
            params![s.id, s.seed_id, s.codeblock_id, s.status],
        )?;
        Ok(())
    }

    /// (Used by the enumerator) – insert a new slice.
    pub fn insert_codeblock(&self, c: &MarkdownCodeblock) -> Result<()> {
        let conn = Connection::open(&self.path)?;

        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM codeblocks WHERE id = ?1);",
            params![c.id],
            |row| row.get(0),
        )?;

        if exists {
            // Optionally log or silently skip
            log::debug!("Skipping duplicate contract_slice with id {}", c.id);
            return Ok(());
        }

        conn.execute(
            "INSERT INTO codeblocks VALUES (?1,?2,?3);",
            params![c.id, c.tokens as i64, c.content],
        )?;
        Ok(())
    }
}
