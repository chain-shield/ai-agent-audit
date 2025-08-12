use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::path::Path;

use crate::{
    config::{CHAINSHIELD_DB_FOLDER, FINDINGS_DB},
    llm_review::config::{Finding, Findings},
    prepare_code::git_clone::RepoPaths,
};

#[derive(Debug, Serialize)]
pub struct FindingDb {
    pub id: String, // UUID v4
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub impact: String,           // FK → seeds.id
    pub proof_of_concept: String, // e.g. "High", "Info", …
    pub proof_of_code: String,
    pub severity: String,
    // pub confidence:   Option<f32>,
    // pub sources_used: Vec<u8>,
}

pub struct FindingsDb(Connection);

impl FindingsDb {
    pub fn open() -> Result<Self> {
        let p = Path::new(&format!("{}/{}", CHAINSHIELD_DB_FOLDER, FINDINGS_DB)).to_path_buf();

        let conn = Connection::open(p)?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS findings(
              id                  TEXT PRIMARY KEY,
              project_id          TEXT,
              title               TEXT,
              description         TEXT,
              impact              TEXT,
              proof_of_concept    TEXT,
              proof_of_code       TEXT,
              severity            TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_findings_project_id
                ON findings(project_id);
            "#,
        )?;
        Ok(Self(conn))
    }

    pub fn insert(&self, f: &FindingDb, repo: &RepoPaths) -> Result<()> {
        self.0.execute(
            "INSERT INTO findings VALUES (?1,?2,?3,?4,?5,?6,?7,?8);",
            params![
                f.id,
                repo.project_id,
                f.title,
                f.description,
                f.impact,
                f.proof_of_concept,
                f.proof_of_code,
                f.severity
            ],
        )?;
        Ok(())
    }

    /// Mass insert findings from a Findings struct into the database
    /// Uses a transaction for better performance and atomicity
    pub fn insert_findings(&self, findings: &Findings, repo: &RepoPaths) -> Result<()> {
        let tx = self.0.unchecked_transaction()?;

        for finding in &findings.findings {
            let finding_db = FindingDb::from_finding(finding, repo);
            tx.execute(
                "INSERT INTO findings VALUES (?1,?2,?3,?4,?5,?6,?7,?8);",
                params![
                    finding_db.id,
                    finding_db.title,
                    finding_db.project_id,
                    finding_db.description,
                    finding_db.impact,
                    finding_db.proof_of_concept,
                    finding_db.proof_of_code,
                    finding_db.severity
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }
}

impl FindingDb {
    /// Convert a Finding to a FindingDb for database insertion
    pub fn from_finding(finding: &Finding, repo: &RepoPaths) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: repo.project_id.clone(),
            title: finding.title(),
            description: finding.description.clone().unwrap_or_default(),
            impact: finding.impact.clone().unwrap_or_default(),
            proof_of_concept: finding.proof_of_concept.clone().unwrap_or_default(),
            proof_of_code: finding.proof_of_code.clone().unwrap_or_default(),
            severity: finding.severity.as_str().to_string(),
        }
    }
}
