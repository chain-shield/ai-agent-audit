pub const POST_VERIFY: &str = r#"

### OUTPUT REQUIREMENTS 

*Please respond with ONLY valid JSON in the following exact format:*

{
    "severity": "High | Medium | Low | informational",
    "severity_justification": "Explain why you assigned this severity",
    "status": "Valid | Invalid | OutOfScope | NeedsMoreInfo",
    "status_justification": "if invalid, out of scope, or needs more info, please explain why",
    "status_confidence": "VeryConfident | Confident | SomewhatConfident",
    "status_confidence_justification": "if Somewhat Confident, please explain why",
    "finding_complexity": How likely is it that other security researchers would find this?  1-10 scale, 10 being very unlikely. Higher the score the better as it will earn the researcher a higher bounty. This value is a number (NOT a string)
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#;

pub const POST_IN_SCOPE_VERIFY: &str = r#"
### OUTPUT REQUIREMENTS 

*Please respond with ONLY valid JSON in the following exact format:*

{
    "severity": "High | Medium | Low | informational",
    "severity_justification": "Explain why you assigned this severity",
    "status": "Valid | Invalid | OutOfScope | NeedsMoreInfo",
    "status_justification": "if invalid, out of scope, or needs more info, please explain why",
    "status_confidence": "VeryConfident | Confident | SomewhatConfident",
    "status_confidence_justification": "if Somewhat Confident, please explain why",
    "finding_complexity": How likely is it that other security researchers would find this?  1-10 scale, 10 being very unlikely. Higher the score the better as it will earn the researcher a higher bounty. This value is a number (NOT a string)
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
"#;
