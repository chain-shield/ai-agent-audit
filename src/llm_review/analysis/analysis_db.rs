use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::path::Path;

use crate::{
    config::{CHAINSHIELD_DB_FOLDER, FINDINGS_DB},
    llm_review::findings::{
        finding_enums::{Severity, VulnerabilityType},
        findings::{Finding, Findings, PrivilegeLevel},
    },
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
            "INSERT INTO findings VALUES (?1,?2,?3,?4,?5,?6,?7);",
            params![
                f.id,
                repo.project_id,
                f.title,
                f.description,
                f.impact,
                f.proof_of_concept,
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
                "INSERT INTO findings VALUES (?1,?2,?3,?4,?5,?6,?7);",
                params![
                    finding_db.id,
                    finding_db.project_id,
                    finding_db.title,
                    finding_db.description,
                    finding_db.impact,
                    finding_db.proof_of_concept,
                    finding_db.severity
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    /// Retrieve all findings for a given project
    pub fn get_findings_by_project(&self, repo: &RepoPaths) -> Result<Findings> {
        let mut stmt = self.0.prepare(
            "SELECT id, project_id, title, description, impact, proof_of_concept, severity
             FROM findings
             WHERE project_id = ?1",
        )?;

        let findings_iter = stmt.query_map([&repo.project_id], |row| {
            Ok(FindingDb {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                impact: row.get(4)?,
                proof_of_concept: row.get(5)?,
                severity: row.get(6)?,
            })
        })?;

        let mut findings = Vec::new();
        for finding_result in findings_iter {
            let finding_db = finding_result?;
            findings.push(finding_db.to_finding());
        }

        Ok(Findings { findings })
    }
}

impl FindingDb {
    /// Convert a Finding to a FindingDb for database insertion
    pub fn from_finding(finding: &Finding, repo: &RepoPaths) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: repo.project_id.clone(),
            title: finding.title.clone(),
            description: finding.description.clone(),
            impact: finding.impact.clone(),
            proof_of_concept: finding.proof_of_concept.clone(),
            severity: finding.severity.to_string(),
        }
    }

    /// Convert a FindingDb back to a Finding
    /// Note: Some fields are not stored in the database and will be set to defaults
    pub fn to_finding(&self) -> Finding {
        use std::str::FromStr;

        Finding {
            id: Some(self.id.clone()),
            derived_from: None,
            title: self.title.clone(),
            exploit_type: VulnerabilityType::default(),
            privilege: PrivilegeLevel::Permissionless,
            contract: String::new(),
            function: String::new(),
            description: self.description.clone(),
            impact: self.impact.clone(),
            proof_of_concept: self.proof_of_concept.clone(),
            justification: None,
            proof_of_code: None,
            poc_test_file: None,
            poc_test_command: None,
            poc_test_status: None,
            severity: Severity::from_str(&self.severity).unwrap_or_default(),
            mitigation: String::new(),
            status: None,
            status_justification: None,
            competition_report: None,
            finding_complexity: None,
        }
    }
}
