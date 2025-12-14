use crate::llm_review::findings::findings::Finding;

pub fn finding_status_to_string(finding: &Finding) -> String {
    let mut statuses = finding
        .status
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    statuses.sort_by(|a, b| b.cmp(a));

    statuses.join(" + ")
}
