pub const PRE_CREATE_POC: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
    "poc_test_code": "fully runnable PoC code as a string (escape newlines and quotes properly). Test function MUST start with 'test' (e.g., testExploit, testReentrancy)",
    "commentary": "brief explanation of your PoC approach and what it demonstrates",
    "cannot_create_poc_because_finding_invalid": true|false
}
"#;
