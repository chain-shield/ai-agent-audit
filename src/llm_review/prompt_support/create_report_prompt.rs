use crate::{
    build_brain::slither_ffi::get_all_files_src,
    config::AuditType,
    llm_review::{
        findings::Finding,
        prompt_support::report_templates::CODE4RENA_REPORT_TEMPLATE,
        utils::prompt_context::{get_finding_report, FindingReportType},
    },
    prepare_code::git_clone::RepoPaths,
};

pub fn generate_create_report_prompt(
    repo: impl AsRef<RepoPaths>,
    finding: &Finding,
    code_and_context: &str,
) -> anyhow::Result<String> {
    let repo = repo.as_ref();
    let file_structure = get_all_files_src(repo)?;
    let github_url = &repo.github_url;
    let report_template = match repo.audit_type {
        AuditType::Sherlock => CODE4RENA_REPORT_TEMPLATE,
        _ => CODE4RENA_REPORT_TEMPLATE,
    };
    let raw_finding_report = get_finding_report(finding, None, FindingReportType::Enhanced);

    Ok(format!(
        r#"

    Before instructions are provided on the task please note required output format:

    ## JSON Output Requirement

    **Output must be strictly valid JSON** with this structure (no extra text or code fencing):
    {CREATE_REPORT_POC_JSON}

    Your task: write professional report for below finding using template provided

    ## Deliverables
    1. **github_urls**: Array of GitHub URLs with line numbers for all code referenced in the report
       - Format: https://github.com/{{org}}/{{repo}}/blob/{{commit}}/{{file}}#L{{start}}-L{{end}}
       - Include: vulnerable function, related functions, state variables, inheritance chain
       - Use the github URL provided + file locations from the file structure to construct complete URLs
       - Example: ["https://github.com/example/repo/blob/abc123/src/Vault.sol#L45-L67"]
    2. **report**: Professional markdown report following the provided template exactly
       - Use the exact template structure provided below
       - Include all required sections: Summary, Description, Impact, Mitigation
       - Justify the severity level with specific impact analysis
       - Provide step-by-step exploit scenario

    ## NOTE: DO NOT include PoC in your report, because PoC is already complete and validated for this finding.

    ## Base Github repo url (https://github.com/{{org}}/{{repo}}/blob/{{commit}} is provided just add relative path to file + line numbers)
    {github_url}

    ## Protocol Files
    {file_structure}

    ## Report Template (MUST USE)
    {report_template}

    ## Create Report based on below Finding
    {raw_finding_report}

    ## Code Finding was Discovered in + Additional Context
    {code_and_context}

    ### OUTPUT REQUIREMENTS

    **Please respond with ONLY valid JSON in the following exact format:**

    {CREATE_REPORT_POC_JSON}

    **IMPORTANT:**
    - Use proper JSON escaping for all string fields (escape `"` as `\"` and newlines as `\n`).
    - Do NOT include markdown code fences (```) in your response.
    - Do NOT include any explanatory text before or after the JSON.
    - Respond with ONLY the raw JSON object.
    "#
    ))
}

pub const CREATE_REPORT_POC_JSON: &str = r#"

{
   "github_urls": ["https://github.com/.../file3.sol#L23-L54", "https://github.com/.../file2.sol#L182-L188"],
   "report": "Markdown Report for Finding made using provided Template"
}
"#;
