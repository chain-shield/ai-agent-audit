use crate::llm_review::dynamic_prompts::prompt_index;

pub fn generate_post_round_verify_json_requirement(verify_json: &str) -> String {
    let section_11_header = prompt_index::generated_section_header("OUTPUT REQUIREMENTS", 11);
    let section_11_1_sub_header =
        prompt_index::generated_sub_header("JSON OUTPUT REQUIREMENT", 11, 1);

    format!(
        r#"
{section_11_header}

{section_11_1_sub_header}

*Please respond with ONLY valid JSON in the following exact format:*

{verify_json}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON.

"#
    )
}
