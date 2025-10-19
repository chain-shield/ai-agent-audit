pub const POST_CREATE_POC: &str = r#"

### OUTPUT REQUIREMENTS

*Please respond with ONLY valid JSON in the following exact format:*

{
    "poc_test_code": "fully runnable PoC code",
    "command_to_run_test": "command to run poc test",
    "cannot_create_poc_because_finding_invalid": true|false (optional field, if field not present -> same as setting to false)
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON.

"#;
