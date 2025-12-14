pub fn generate_pre_round_verify_json_requirement(verify_json: &str) -> String {
    format!(
        r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{verify_json}

"#
    )
}

pub fn generate_post_round_verify_json_requirement(verify_json: &str) -> String {
    format!(
        r#"

### OUTPUT REQUIREMENTS 
*Please respond with ONLY valid JSON in the following exact format:*

{verify_json}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#
    )
}
