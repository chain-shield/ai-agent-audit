use crate::enumerator::slice_db::SliceDb;
use anyhow::Result;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Args {
    pub slice_id: String,
}

#[derive(Debug, Serialize)]
pub struct Out {
    pub content: String,
}
/* ────────────── Thread-safe error type ──────────────────────────── */

#[derive(Debug, thiserror::Error)]
#[error("retrival error")]
// struct RetrieveSliceError;
pub enum RetrieveSliceError {
    Sql(#[from] rusqlite::Error),
}

pub struct RetrieveSliceTool {
    pub db: SliceDb, // <- now Sync because it’s just a PathBuf
}

impl Tool for RetrieveSliceTool {
    const NAME: &'static str = "retrieve_slice";

    type Args = Args;
    type Output = Out;
    type Error = RetrieveSliceError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Load a code slice (by ID) from SQLite and return its Markdown.".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "slice_id": { "type": "string", "description": "UUID of the slice" }
                },
                "required": ["slice_id"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let content = self.db.get_code_for_seed(&args.slice_id)?;
        Ok(Out { content })
    }
}
