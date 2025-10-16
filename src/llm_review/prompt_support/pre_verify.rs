pub const PRE_VERIFY: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
    "severity": "High | Medium | Low | informational",
    "severity_justification": "Explain why you assigned this severity",
    "status": "Valid | Invalid | OutOfScope | NeedsMoreInfo",
    "status_justification": "if invalid, out of scope, or needs more info, please explain why",
    "status_confidence": "VeryConfident | Confident | SomewhatConfident",
    "status_confidence_justification": "if Somewhat Confident, please explain why",
    "finding_complexity": How likely is it that other security researchers would find this?  1-10 scale, 10 being very unlikely. Higher the score the better as it will earn the researcher a higher bounty. This value is a number (NOT a string)
}

"#;

pub const PRE_IN_SCOPE_VERIFY: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
    "is_vulnerability_in_scope": true|false,
    "why_its_not_in_scope": "explain why NOT in scope"
}

"#;
