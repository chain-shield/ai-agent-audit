pub const PRE_FILE_SELECT: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
  "files": ["file1","file2",...]  // array of project files
}

"#;
