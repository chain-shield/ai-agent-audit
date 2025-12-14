/// Phase 3: Deduplication and verification of discovered security findings
///
/// This phase removes duplicate findings and verifies the legitimacy of each
/// discovered vulnerability using AI-powered analysis.
use crate::{
    error::Result,
    llm_review::{
        agent::agent_enums::AIAgent,
        analysis::{
            context_state::{generate_audit_scope, get_metadata_context},
            semaphore::GENERAL_SEM,
        },
        findings::findings::{Finding, Findings},
        phases::verify_rounds::FindingStatus,
        prompt_support::severity_rubics::CODE4RENA_SEVERITY_RUBRIC,
        utils::prompt_context::{FindingReportType, generate_prompt_for_issue_check},
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;

use crate::{
    config::AuditType,
    llm_review::{
        agent::agent_enums::{all_enum_variants, generate_enum_list},
        prompt_support::severity_rubics::{CANTINA_SEVERITY_RUBRIC, SHERLOCK_SEVERITY_RUBRIC},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Verification result for a potential vulnerability
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegitVulnerability {
    pub status: FindingStatus,
    pub status_justification: Option<String>,
    #[serde(deserialize_with = "deserialize_finding_complexity")]
    pub finding_complexity: u8,
}

/// Helper function to deserialize u8 from string or number with validation (1-10)
fn deserialize_finding_complexity<'de, D>(deserializer: D) -> std::result::Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let val: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
    let num = match val {
        serde_json::Value::Number(n) => {
            n.as_u64()
                .ok_or_else(|| serde::de::Error::custom("expected valid number"))? as u8
        }
        serde_json::Value::String(s) => s
            .parse::<u8>()
            .map_err(|_| serde::de::Error::custom("expected numeric string"))?,
        _ => return Err(serde::de::Error::custom("expected number or string")),
    };

    // Validate range 1-10
    if num < 1 || num > 10 {
        log::warn!(
            "finding_complexity {} is out of range (1-10), clamping to valid range",
            num
        );
        Ok(num.clamp(1, 10))
    } else {
        Ok(num)
    }
}

/// Executes the verification phase
///
/// Deduplicates findings and verifies each one using AI analysis to ensure
/// only legitimate vulnerabilities are retained.
pub async fn execute(
    findings: Findings,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    info!("🔍 Phase 4: Deduplicating and verifying findings...");

    let mut handles = vec![];
    let deduped_findings = Arc::new(findings.dedup().await?);
    let context = get_metadata_context(repo)
        .await
        .expect("could not extract context");
    let code_and_context = generate_content_plus_context_block(code, &context);
    let arc_code_context = Arc::new(code_and_context);

    let dedup_finding_count = deduped_findings.findings.len();
    let is_legit_finding_vec: Arc<Mutex<Vec<LegitVulnerability>>> = Arc::new(Mutex::new(vec![
            LegitVulnerability::default();
            dedup_finding_count
        ]));

    info!("# of findings AFTER deduping => {}", dedup_finding_count);
    info!("now verifying each finding...");

    let audit_scope = generate_audit_scope(repo).await?;

    let verify_prompt = generate_verify_prompt(repo);

    let updated_verify_prompt = if audit_scope.is_empty() {
        Arc::new(verify_prompt.to_string())
    } else {
        Arc::new(format!(
            "{}\n\n ## SCOPE FOR SECURITY AUDIT - ONLY FINDINGS WITHIN BELOW SCOPE ARE LEGIT\n\n{}",
            &verify_prompt, &audit_scope
        ))
    };
    // info!("verify prompt + scope => {}", verify_prompt_plus_scope);

    for i in 0..dedup_finding_count {
        let codeblock_plus_context = Arc::clone(&arc_code_context);
        let arc_findings = Arc::clone(&deduped_findings);
        let arc_agent = Arc::clone(&agent);
        let arc_legit_findings_vec = Arc::clone(&is_legit_finding_vec);
        let verify_prompt_and_scope = Arc::clone(&updated_verify_prompt);
        let sem = Arc::clone(&GENERAL_SEM);

        handles.push(tokio::spawn(async move {
            // ── acquire permit ────────────────────────
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result: Result<()> = async {
                let post_verify_json = generate_post_verify_json_requirement();
                let instruction_prompt = generate_prompt_for_issue_check(
                    &codeblock_plus_context,
                    &arc_findings.findings[i],
                    &verify_prompt_and_scope,
                    &post_verify_json,
                    FindingReportType::Standard,
                );

                // add to cost
                info!("verifying finding #{}", i + 1);
                let is_legit_struct: LegitVulnerability =
                    arc_agent.extract_with_retry(&instruction_prompt).await?;

                let is_finding_legit = is_legit_struct.status == FindingStatus::Valid;
                if !is_finding_legit {
                    info!(
                        "{} is {} => {}",
                        arc_findings.findings[i].title,
                        is_legit_struct.status.clone(),
                        &is_legit_struct
                            .status_justification
                            .clone()
                            .unwrap_or_default()
                    );
                }

                let mut legit_findings_vec = arc_legit_findings_vec.lock().await;
                legit_findings_vec[i] = is_legit_struct;

                Ok(())
            }
            .await;

            if let Err(e) = result {
                log::error!("Error verifying finding {}: {:?}", i, e);
            }
        }));
    }

    // Wait for all verification tasks to complete
    for h in handles {
        let _ = h.await;
    }

    let legit_findings_vec = is_legit_finding_vec.lock().await;
    let verified_findings: Vec<Finding> = deduped_findings
        .as_ref()
        .findings
        .iter()
        .enumerate()
        // NOTE: no longer filtering out non-valids, instead separating by status
        // .filter(|(idx, _)| {
        //     legit_findings_vec[*idx].status == FindingStatus::Valid
        //         || legit_findings_vec[*idx].status == FindingStatus::NeedsMoreInfo
        // })
        .map(|(idx, f)| {
            let legit_findings = legit_findings_vec[idx].clone();
            let enriched_finding = Finding {
                status: Some(vec![legit_findings.status]),
                status_justification: legit_findings.status_justification,
                finding_complexity: Some(legit_findings.finding_complexity),
                ..f.clone()
            };
            enriched_finding
        })
        .collect();

    info!(
        "✅ Phase 4 complete: {} Verified Findings!",
        verified_findings.len()
    );

    Ok(Findings {
        findings: verified_findings,
    })
}

/// Generates the combined content and context block for verification analysis
///
/// Combines the contract code with additional context information
/// in a structured format for optimal verification processing.
pub fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();

    code_plus_context.push_str("\n\n# SOLIDITY CONTRACT + STORAGE TO CODE REVIEW\n\n");
    code_plus_context.push_str(codeblock);

    code_plus_context
        .push_str("\n\n ## ADDITIONAL CONTEXT TO ASSIST WITH SECURITY REVIEW OF ABOVE CODE \n\n");
    code_plus_context.push_str(&added_context);
    code_plus_context.push_str("\n\n");

    code_plus_context
}

pub fn generate_verify_prompt(repo: &RepoPaths) -> String {
    let (_, contest) = match repo.audit_type {
        AuditType::Code4rena => (CODE4RENA_SEVERITY_RUBRIC, "Code4rena"),
        AuditType::Sherlock => (SHERLOCK_SEVERITY_RUBRIC, "Sherlock"),
        AuditType::Cantina => (CANTINA_SEVERITY_RUBRIC, "Cantina"),
        _ => (CODE4RENA_SEVERITY_RUBRIC, "Private Audit"),
    };

    let pre_verify_json = generate_pre_verify_json_requirement();
    let finding_status_list = generate_enum_list(all_enum_variants::<FindingStatus>().as_slice());

    format!(
        r#"
        {pre_verify_json}

    Your task: decide if a reported finding is Valid and to accurately assess its Severity in a {contest} contest.

    # 🚨 CRITICAL VERIFICATION GATES - ALL MUST PASS

    You MUST verify the finding passes ALL gates. If ANY gate fails, the finding is Invalid or QA/Low.

    ---

    ## 🔍 PRE-GATE SANITY CHECK - VERIFY BUG EXISTS

    **BEFORE checking any gates, verify the bug actually exists in the code:**

    **Step 1: Trace the Code Path**
    - Locate exact function/contract, verify vulnerable code path exists (not hallucinated)
    - Trace execution flow step-by-step, check if code matches finding's description

    **INVALID if:** Function/contract doesn't exist, code path impossible, execution flow doesn't match

    **Step 2: Verify Invariant Actually Exists**
    - Check if claimed invariant is documented (NatSpec, comments, docs)
    - Verify invariant is enforced elsewhere, confirm it's a real protocol requirement

    **INVALID if:** Invariant not documented, not enforced elsewhere, assumed but not required

    **Step 3: Reproduce the Issue**
    - Can you trace exact steps to trigger the bug? Does PoC demonstrate claimed issue?

    **INVALID if:** Cannot trace execution path, PoC doesn't trigger vulnerability, preconditions impossible

    **⚠️ If sanity check fails → INVALID (hallucination). If passes → Proceed to GATE 1**

    ---

    ## GATE 1: SCOPE CHECK

    **INVALID:** Root cause in OOS library, OOS token (except USDT), view-only cosmetic
    **VALID:** Root cause in-scope OR in-scope code misuses OOS library

    ---

    ## GATE 2: USER ERROR CHECK 🚨

    **INVALID if requires:** User chooses bad recipient, provides bad parameters, approves malicious contract, signs malicious data
    **VALID if:** Protocol forces vulnerable state, attacker exploits without user involvement, user follows normal flow but protocol fails

    ---

    ## GATE 3: IMPACT CLASSIFICATION

    **HIGH:** Theft/permanent loss of assets, unauthorized drains, economic attacks (non-dust)
    **MEDIUM:** DoS of critical actions, accounting drift, mispricing, privilege escalation
    **QA/LOW:** Dust amounts, stylistic issues, event inconsistencies, view-function errors

    ---

    ## GATE 4: LIKELIHOOD ASSESSMENT 🚨

    **COMMON:** No preconditions, works anytime/anywhere, no special resources
    **OCCASIONAL:** Specific but realistic conditions, some chains, moderate setup
    **RARE:** Multiple unlikely conditions, extreme market states, significant resources

    **Severity Matrix:**

    **CRITICAL Impact** (bricks entire protocol, steals ALL funds, complete takeover):
    - Common/Occasional → HIGH | Rare → **MEDIUM** ✅ (Exception: critical overrides rare)

    **HIGH Impact** (substantial loss, core function break, major DoS):
    - Common → HIGH | Occasional → HIGH/MEDIUM | Rare → LOW ❌

    **MEDIUM Impact** (temporary DoS, accounting drift, bounded loss):
    - Common → MEDIUM | Occasional → MEDIUM/LOW | Rare → QA ❌

    **🚨 Key: CRITICAL = entire protocol/ALL funds/complete takeover | HIGH = substantial/core/major**

    **Exception:** CRITICAL + Rare → still MEDIUM (protocol-ending bugs always valid)

    ---

    ## GATE 5: GOVERNANCE/CENTRALIZATION RISK 🚨🚨

    **Question: Can governance/team prevent this by acting responsibly?**

    **INVALID/QA if YES:**
    - ❌ Admin sets wrong parameters, chooses malicious oracle, misconfigures
    - ❌ Team deploys on wrong chain, doesn't verify addresses
    - ❌ Team chooses malicious integration, configures incorrectly
    - ❌ **"If [TrustedComponent] fails/has bug/behaves unexpectedly"** (assumes future bug)

    **VALID if NO (code vulnerability):**
    - ✅ Code should verify/check/validate but doesn't (missing runtime verification)
    - ✅ Non-privileged user gains privileged access (privilege escalation)

    **Key: Code logic/access control = VALID | Deployment/parameters/trusted component = INVALID**

    **Red flags:** "Team should verify", "Only on chain X", "If [Component] fails", "Admin chooses"

    ---

    ## GATE 6: UNSUPPORTED TOKEN CHECK

    **INVALID:** Fee-on-transfer/rebasing/decimals edge cases (unless explicitly supported or USDT)

    ---

    ## GATE 7: SPECULATION CHECK 🚨🚨

    **Question: Does root cause exist NOW and is exploitable with TODAY's code?**

    **INVALID if speculative:**
    - ❌ "If protocol integrates/adds/upgrades in future..."
    - ❌ **"If [Component] fails/has bug/behaves unexpectedly/is paused..."** (assumes future bug)
    - ❌ "Could/might/potentially happen if..." (hypothetical)

    **VALID if current:**
    - ✅ Bug in current code, exploit works now, no future changes needed
    - ✅ Plausible future integration (docs mention it, code has hooks, strong evidence)

    **Red flags:** "If [Component] fails", "Could happen if", "When protocol adds", "Future integration"

    ---

    ## GATE 8: "BY DESIGN" CHECK 🚨🚨

    **Question: Is this documented as intentional? Check NatSpec, comments, docs, function naming.**

    **🚨 CRITICAL EXCEPTION: Documentation ≠ Not a Vulnerability**

    **VALID despite documentation if creates:**
    - ✅ Economic risk/loss for users (liquidators, LPs, depositors)
    - ✅ Missing standard protection (slippage, deadline, minOut, price bounds)
    - ✅ MEV/value extraction opportunity
    - ✅ Incentive misalignment harming protocol

    **Examples VALID despite docs:**
    - ✅ Missing slippage/deadline/minOut → controllable loss (Medium) - C4 consistently awards Medium
    - ✅ Unfair fee structure → systematic disadvantage (Low/Medium)

    **INVALID if documented + no harm:**
    - ❌ Admin emergency pause, governance timelock (protective measures)

    **When in doubt:** Mark VALID + SomeWhatConfident (false negatives worse than false positives)

    ---

    ## GATE 9: EXPLOITABILITY (PoC)

    **Requirements:** Minimal reproducible PoC showing state change, non-dust effect, realistic actors

    ---

    ## GATE 10: CONFIGURATION CHECK

    **If finding relies on constants:** Check for testnet comments, suspiciously small values, commented-out production values

    ---

    ## GATE 11: EXISTING SAFEGUARDS CHECK 🚨🚨

    **Question: Does code already have safeguards that mitigate/eliminate this vulnerability?**

    **INVALID if safeguards exist and work:**
    - ❌ Reentrancy → has `nonReentrant`, CEI pattern, or guard
    - ❌ Integer overflow → Solidity 0.8+ with built-in checks
    - ❌ Access control → has `onlyOwner`, `onlyRole`, role checks
    - ❌ Front-running → has commit-reveal, deadlines, slippage protection
    - ❌ Oracle manipulation → has TWAP, multiple sources, price bounds
    - ❌ DoS → has pagination, gas limits, circuit breakers
    - ❌ Precision loss → has proper scaling, rounding checks

    **VALID if safeguards missing or insufficient:**
    - ✅ No safeguard exists for attack vector
    - ✅ Safeguard bypassable (show bypass in PoC)
    - ✅ Safeguard incomplete (only some functions protected)
    - ✅ Safeguard has wrong parameters (deadline too long, slippage too high)
    - ✅ Safeguard incorrectly implemented (show flaw in PoC)

    **How to Check:** Search codebase for modifiers/guards, verify vulnerable function uses them, test if bypassable

    **Red Flags (Check for safeguards first):**
    - "Missing reentrancy guard" → Search for `nonReentrant`, CEI pattern
    - "Integer overflow" → Check Solidity version (0.8+ has built-in checks)
    - "Missing access control" → Search for `onlyOwner`, `onlyRole`, role checks
    - "Oracle manipulation" → Search for TWAP, multiple oracles, price validation
    - "Front-running" → Search for `deadline`, `minAmountOut`, slippage checks

    # OUTPUT REQUIREMENTS

    Based on your assessment please provided the following:

    *Finding Status (pick best fit):* {finding_status_list}
    *Status Justification:* If invalid, or low/qa. please provide detailed Justification (under 400 words - format with bullets and linebreaks). **MUST cite specific gate failures (e.g., "GATE 7 FAIL: Assumes future Distributor bug").**
    *Finding Complexity:* How likely is it that other security researchers would find this?  1-10 scale, 10 being very unlikely. Higher the score the better as it will earn the researcher a higher bounty.

"#
    )
}

pub fn generate_post_verify_json_requirement() -> String {
    let json = generate_verify_json();

    format!(
        r#"

### OUTPUT REQUIREMENTS 
*Please respond with ONLY valid JSON in the following exact format:*

{json}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#
    )
}

pub fn generate_pre_verify_json_requirement() -> String {
    let json = generate_verify_json();

    format!(
        r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{json}

"#
    )
}

fn generate_verify_json() -> String {
    let finding_status_list = generate_enum_list(all_enum_variants::<FindingStatus>().as_slice());
    format!(
        r#"
{{
    "status": "{finding_status_list}",
    "status_justification": "If invalid, or low/qa. please provide detailed Justification (under 200 words). **MUST cite specific gate failures (e.g., "GATE 7 FAIL: Assumes future Distributor bug").",
    "finding_complexity": 5  // Number 1-10: How likely is it that other security researchers would find this? 10 = very unlikely (higher score = higher bounty). MUST be a number, NOT a string.
}}
"#
    )
}
