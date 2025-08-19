/// Repository data storage for complete project metadata.
///
/// This module provides a SQLite-based database for storing and querying
/// complete repository information including paths, file lists, documentation,
/// and generated context data from RepoPaths and metadata analysis.
use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::{Path, PathBuf};

use crate::{
    config::{CHAINSHIELD_DB_FOLDER, REPO_DATA_DB},
    llm_review::context_state::get_metadata_context,
    prepare_code::git_clone::RepoPaths,
};

/// Represents complete repository data with all metadata
#[derive(Debug, Clone)]
pub struct RepoData {
    /// Unique project identifier
    pub project_id: String,
    /// Repository root path
    pub root: String,
    /// Repository name
    pub repo_name: String,
    /// Full commit hash
    pub commit_hash: String,
    /// Source code folder path
    pub source_code_folder: String,
    /// Serialized list of Solidity files
    pub sol_files: String,
    /// Serialized list of test files
    pub test_files: String,
    /// Serialized list of script files
    pub script_files: String,
    /// Serialized list of config files
    pub config_files: String,
    /// Serialized list of documentation files
    pub docs: String,
    /// Audit scope file path (optional)
    pub audit_scope: Option<String>,
    /// Serialized list of excluded folders
    pub excluded_folders: Option<String>,
    /// Generated metadata context for AI analysis
    pub context: String,
}

pub async fn save_repo_data_to_db(repo: &RepoPaths) -> anyhow::Result<()> {
    let repodata_db = RepoDataDb::create(Path::new(&format!(
        "{}/{}",
        CHAINSHIELD_DB_FOLDER, REPO_DATA_DB
    )))?;
    let context = get_metadata_context(repo).await.expect("no context found!");

    repodata_db.insert_repo_data(repo, &context)?;

    Ok(())
}
/// SQLite-based database for repository data storage
pub struct RepoDataDb(Connection);

impl RepoDataDb {
    /// Creates a new repository data database with required schema.
    ///
    /// Initializes SQLite database with table for complete repository metadata.
    pub fn create(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS repo_data(
              project_id TEXT PRIMARY KEY,
              root TEXT NOT NULL,
              repo_name TEXT NOT NULL,
              commit_hash TEXT NOT NULL,
              source_code_folder TEXT NOT NULL,
              sol_files TEXT NOT NULL,
              test_files TEXT NOT NULL,
              script_files TEXT NOT NULL,
              config_files TEXT NOT NULL,
              docs TEXT NOT NULL,
              audit_scope TEXT,
              excluded_folders TEXT,
              context TEXT NOT NULL,
              created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            
            "#,
        )?;
        Ok(Self(conn))
    }

    /// Inserts complete repository data into the database.
    ///
    /// Stores all RepoPaths information plus generated metadata context
    /// for later retrieval and analysis.
    pub fn insert_repo_data(&self, repo: &RepoPaths, context: &str) -> Result<()> {
        // Serialize path vectors to JSON strings
        let sol_files = serde_json::to_string(&repo.sol_files)?;
        let test_files = serde_json::to_string(&repo.test_files)?;
        let script_files = serde_json::to_string(&repo.script_files)?;
        let config_files = serde_json::to_string(&repo.config_files)?;
        let docs = serde_json::to_string(&repo.docs)?;
        let excluded_folders = repo
            .excluded_folders
            .as_ref()
            .map(|folders| serde_json::to_string(folders))
            .transpose()?;

        self.0.execute(
            "INSERT OR REPLACE INTO repo_data(
                project_id, root, repo_name, commit_hash, source_code_folder,
                sol_files, test_files, script_files, config_files, docs,
                audit_scope, excluded_folders, context
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13);",
            params![
                repo.project_id,
                repo.root.to_string_lossy(),
                repo.repo_name,
                repo.commit_hash,
                repo.source_code_folder.to_string_lossy(),
                sol_files,
                test_files,
                script_files,
                config_files,
                docs,
                repo.audit_scope
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string()),
                excluded_folders,
                context
            ],
        )?;
        Ok(())
    }

    /// Retrieves repository data by project ID.
    ///
    /// Returns complete repository information including all file paths
    /// and generated metadata context.
    pub fn get_repo_data(&self, project_id: &str) -> Result<Option<RepoData>> {
        let mut stmt = self.0.prepare(
            "SELECT project_id, root, repo_name, commit_hash, source_code_folder,
                    sol_files, test_files, script_files, config_files, docs,
                    audit_scope, excluded_folders, context
             FROM repo_data WHERE project_id = ?1",
        )?;

        let mut rows = stmt.query_map([project_id], |row| {
            Ok(RepoData {
                project_id: row.get(0)?,
                root: row.get(1)?,
                repo_name: row.get(2)?,
                commit_hash: row.get(3)?,
                source_code_folder: row.get(4)?,
                sol_files: row.get(5)?,
                test_files: row.get(6)?,
                script_files: row.get(7)?,
                config_files: row.get(8)?,
                docs: row.get(9)?,
                audit_scope: row.get(10)?,
                excluded_folders: row.get(11)?,
                context: row.get(12)?,
            })
        })?;

        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    /// Checks if repository data exists for the given project ID.
    pub fn exists(&self, project_id: &str) -> Result<bool> {
        let mut stmt = self
            .0
            .prepare("SELECT 1 FROM repo_data WHERE project_id = ?1 LIMIT 1")?;
        let exists = stmt.exists([project_id])?;
        Ok(exists)
    }

    /// Lists all stored repository projects with basic information.
    pub fn list_projects(&self) -> Result<Vec<(String, String, String)>> {
        let mut stmt = self.0.prepare(
            "SELECT project_id, repo_name, commit_hash FROM repo_data ORDER BY created_at DESC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        let mut projects = Vec::new();
        for row in rows {
            projects.push(row?);
        }
        Ok(projects)
    }
}

impl RepoData {
    /// Reconstructs RepoPaths from stored repository data.
    ///
    /// Deserializes file path lists and recreates the original RepoPaths structure
    /// for use in analysis workflows.
    pub fn to_repo_paths(&self) -> Result<RepoPaths> {
        let sol_files: Vec<PathBuf> = serde_json::from_str(&self.sol_files)?;
        let test_files: Vec<PathBuf> = serde_json::from_str(&self.test_files)?;
        let script_files: Vec<PathBuf> = serde_json::from_str(&self.script_files)?;
        let config_files: Vec<PathBuf> = serde_json::from_str(&self.config_files)?;
        let docs: Vec<PathBuf> = serde_json::from_str(&self.docs)?;
        let excluded_folders: Option<Vec<PathBuf>> = self
            .excluded_folders
            .as_ref()
            .map(|s| serde_json::from_str(s))
            .transpose()?;

        Ok(RepoPaths {
            project_id: self.project_id.clone(),
            root: PathBuf::from(&self.root),
            sol_files,
            test_files,
            script_files,
            config_files,
            source_code_folder: PathBuf::from(&self.source_code_folder),
            scoped_files: None,
            docs,
            repo_name: self.repo_name.clone(),
            audit_scope: self.audit_scope.as_ref().map(PathBuf::from),
            excluded_folders,
            commit_hash: self.commit_hash.clone(),
        })
    }
}
