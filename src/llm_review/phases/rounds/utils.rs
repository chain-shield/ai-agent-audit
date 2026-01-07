use crate::{
    config::JSON_REQUIREMENT_SECTION,
    llm_review::{
        agent::agent_enums::generate_enum_list,
        dynamic_prompts::prompt_index,
        phases::rounds::all_rounds::{Impact, Likelihood},
    },
};
use strum::IntoEnumIterator;

pub fn generate_post_round_verify_json_requirement(verify_json: &str) -> String {
    let section_11_header =
        prompt_index::generated_section_header("OUTPUT REQUIREMENTS", JSON_REQUIREMENT_SECTION);
    let impact_list = generate_enum_list(&Impact::iter().collect::<Vec<_>>());
    let likelihood_list = generate_enum_list(&Likelihood::iter().collect::<Vec<_>>());

    format!(
        r#"
{section_11_header}

Please continue until you have carefully evaulated ALL findings on EACH of the 11 checks.

Based on your assessment please provided the following for EACH finding:

*finding id*: insert finding id (from 'id' field)
*finding_title*: insert finding 'title'
*does bug exist*: true | false
*safeguard against it*: true | false
*in scope*: true | false
*by design*: true | false
*exploitable*: true | false
*impact*: {impact_list}
*likelihood*: {likelihood_list}
*user error or mistake*: true | false
*governance risk*: true | false
*future speculation*: true | false
*non standard token*: true | false
*justification:*: Please provide justification for your choices (under 400 words)


════════════════════════════════════════════════════════════════════
███ SECTION 11.1: JSON OUTPUT REQUIREMENT ███
════════════════════════════════════════════════════════════════════

*Please respond with ONLY valid JSON in the following exact format:*

{verify_json}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON.

"#
    )
}
