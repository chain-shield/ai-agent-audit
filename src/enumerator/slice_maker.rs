// crates/slice_maker/src/lib.rs
use anyhow::Result;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use crate::static_scanning::seed_db::SeedDb;

use super::{path_enum::generate_codeblock_from_slither_seed, slice_db::SliceDb};

/// Build slices for every NEW seed in `seeds.db` (optionally capped).
pub async fn generate_and_save_code_slices_from_slither_seeds(
    repo_root: &Path,
    semantics_db: &Path,
    max_depth: usize,
    token_budget: usize,
) -> Result<PathBuf> {
    // ── open the three databases ───────────────────────────
    let semantic_conn = Connection::open(semantics_db)?;
    let seed_db = SeedDb::open(&repo_root.join(".cache").join("seeds.db"))?;
    let slice_path = repo_root.join(".cache").join("slice.db");
    let slice_db = SliceDb::open(&slice_path)?;

    // ── fetch queue & iterate  ─────────────────────────────
    let seeds = seed_db.all_seeds()?; // you already promised this fn
    for seed in seeds.into_iter() {
        generate_codeblock_from_slither_seed(
            repo_root,
            &seed,
            &semantic_conn,
            &slice_db,
            max_depth,
            token_budget,
        )
        .await?;
    }
    Ok(slice_path)
}
