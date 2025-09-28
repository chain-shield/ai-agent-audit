use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::Path;

use crate::{
    build_brain::summarize::SrcFileSummary,
    config::{CHAINSHIELD_DB_FOLDER, SUMMARY_DB},
    prepare_code::git_clone::RepoPaths,
};

/// SQLite-based graph database for semantic contract data
pub struct SummaryDb(Connection);

impl SummaryDb {
    /// Creates a new graph database with required schema.
    ///
    /// Initializes SQLite database with tables for functions, call edges,
    fn create(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS summaries(
              project_id TEXT,
              filename TEXT,
              summary TEXT,
              PRIMARY KEY (project_id, filename)
            );
            "#,
        )?;
        Ok(Self(conn))
    }

    fn insert_summary(&self, filename: &str, summary: &str, repo: &RepoPaths) -> Result<()> {
        self.0.execute(
            r#"
        INSERT INTO summaries (project_id, filename, summary) VALUES (?1, ?2, ?3)
        "#,
            params![&repo.project_id, filename, summary],
        )?;
        Ok(())
    }

    // get files summaries EXCLUDING protocol summary
    fn get_summaries(&self, repo: &RepoPaths) -> Result<Vec<SrcFileSummary>> {
        let mut query = self
            .0
            .prepare("SELECT filename, summary FROM summaries WHERE project_id = ?1")?;

        let rows = query.query_map([&repo.project_id], |row| {
            Ok(SrcFileSummary {
                filename: row.get(0)?,
                summary: row.get(1)?,
            })
        })?;

        let mut summaries = Vec::new();
        for row in rows {
            let summary = row?;
            if summary.filename != "protocol-summary" {
                summaries.push(summary);
            }
        }
        Ok(summaries)
    }

    // get specific summary file
    fn get_summary_file(&self, filename: &str, repo: &RepoPaths) -> Result<Option<SrcFileSummary>> {
        let mut query = self.0.prepare(
            "SELECT filename, summary FROM summaries WHERE project_id = ?1 AND filename = ?2",
        )?;

        let result = query.query_row([&repo.project_id, filename], |row| {
            Ok(SrcFileSummary {
                filename: row.get(0)?,
                summary: row.get(1)?,
            })
        });

        match result {
            Ok(file_summary) => Ok(Some(file_summary)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

pub fn insert_file_summaries_to_db(summaries: &[SrcFileSummary], repo: &RepoPaths) -> Result<()> {
    let summary_db = SummaryDb::create(Path::new(&format!(
        "{}/{}",
        CHAINSHIELD_DB_FOLDER, SUMMARY_DB
    )))?;

    for summary in summaries {
        summary_db.insert_summary(&summary.filename, &summary.summary, repo)?;
    }
    Ok(())
}

pub fn insert_file_summary_to_db(filename: &str, summary: &str, repo: &RepoPaths) -> Result<()> {
    let summary_db = SummaryDb::create(Path::new(&format!(
        "{}/{}",
        CHAINSHIELD_DB_FOLDER, SUMMARY_DB
    )))?;

    summary_db.insert_summary(&filename, &summary, repo)?;
    Ok(())
}

pub fn get_summaries_from_db(repo: &RepoPaths) -> Result<Vec<SrcFileSummary>> {
    let summary_db = SummaryDb::create(Path::new(&format!(
        "{}/{}",
        CHAINSHIELD_DB_FOLDER, SUMMARY_DB
    )))?;

    Ok(summary_db.get_summaries(repo)?)
}

pub fn get_file_summary_from_db(
    filename: &str,
    repo: &RepoPaths,
) -> Result<Option<SrcFileSummary>> {
    let summary_db = SummaryDb::create(Path::new(&format!(
        "{}/{}",
        CHAINSHIELD_DB_FOLDER, SUMMARY_DB
    )))?;

    Ok(summary_db.get_summary_file(filename, repo)?)
}
