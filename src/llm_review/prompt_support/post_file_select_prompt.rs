pub const POST_FILE_SELECT: &str = r#"

### OUTPUT REQUIREMENTS 

*Please respond with ONLY valid JSON in the following exact format:*

{
  "files": ["file1","file2",...]  // array of project files
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
**Please double-check opening and closing brakets: `}` and `]`, make sure 
they match up correctly.

"#;
