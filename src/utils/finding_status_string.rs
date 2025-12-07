use crate::llm_review::findings::findings::Finding;

pub fn finding_status_to_string(finding: &Finding) -> String {
    finding
        .status
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(" + ")
}
