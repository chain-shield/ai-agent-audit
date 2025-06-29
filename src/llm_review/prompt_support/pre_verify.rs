pub const PRE_VERIFY: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
    "is_legit_vulnerability": true|false,
    "why_its_not_legit": "explain why NOT legit (OMIT if legit)"
}

"#;
