diff --git a/.DS_Store b/.DS_Store
index 20723ed..0d97987 100644
Binary files a/.DS_Store and b/.DS_Store differ
diff --git a/.augment/rules/imported/audit-verify-rules.md b/.augment/rules/imported/audit-verify-rules.md
new file mode 100644
index 0000000..a84d31b
--- /dev/null
+++ b/.augment/rules/imported/audit-verify-rules.md
@@ -0,0 +1,19 @@
+---
+type: "manual"
+---
+
+## You are a world class Code4rena judge and skeptical verifier. 
+
+**Default posture: INVALID until proven by code path + realistic exploit.**
+
+### Before severity: classify the finding into exactly one of:
+- User-error dependent (invalid/low unless protocol forces it),
+- Governance/admin risk (invalid/QA unless in threat model),
+- By-design behavior (not a vuln unless it causes user harm per rubric),
+- True code vuln (then evaluate impact + likelihood).
+
+### Admin threat model: assume admin/governance is trusted and competent unless the contest/spec explicitly includes malicious admin risk 
+- Impact must be reported as (A) max theoretical and (B) worst credible; severity follows (B), not (A). 
+- Likelihood must list explicit preconditions and map them to Common/Occasional/Rare. If preconditions aren’t listed → default to Rare.
+- “By design” requires evidence (NatSpec/docs/tests). If no evidence, treat as not proven by design. 
+- You must explicitly answer: **Does this require user mistake? If yes → invalidate/downgrade.**? 
\ No newline at end of file
diff --git a/src/config.rs b/src/config.rs
index 2972202..bae7554 100644
--- a/src/config.rs
+++ b/src/config.rs
@@ -39,12 +39,12 @@ pub const SKIP_INVARIANT_RUNS: bool = false;
 // SKIP or RUN MAIN PATTERN RUNS
 pub const SKIP_ACTOR_PATTERN_RUNS: bool = false;
 // RUNS R1 (basic) and R2 (complex) patterns
-pub const R1_RUNS: usize = 5; // default: 10 , testing: 5
-pub const R2_RUNS: usize = 5; // default: 10 , testing: 5
+pub const R1_RUNS: usize = 7; // default: 10 , testing: 5
+pub const R2_RUNS: usize = 7; // default: 10 , testing: 5
 
 // NOTE: for large protocols consider reducing scale, skip libs
 /// Number of discovery rounds per contract during analysis
-pub const INVARIANT_RUNS: usize = 3; // default: 5 , testing:3
+pub const INVARIANT_RUNS: usize = 4; // default: 5 , testing:3
 pub const MAX_PATTERN_RUN_TOP: usize = 3; // 2 for large protocol, default: 3
 pub const MAX_PATTERN_RUN_RARE: usize = 3; // 2 for large protocol, default: 3
 pub const MAX_PATTERN_RUN_MOST: usize = 3; // 0 for large protocol, default: 3
diff --git a/src/llm_review/analysis/context_state.rs b/src/llm_review/analysis/context_state.rs
index f2b0f70..276587f 100644
--- a/src/llm_review/analysis/context_state.rs
+++ b/src/llm_review/analysis/context_state.rs
@@ -251,16 +251,25 @@ pub async fn generate_multi_modal_context(
     // Release lock before expensive operation
     drop(multimodal_cache);
 
-    let actors = if !SKIP_ACTOR_PATTERN_RUNS {
-        pre_audit_analysis::generate_actors(codeblock, repo).await?
-    } else {
-        Actors::default()
-    };
-    let invariants = if !SKIP_INVARIANT_RUNS {
-        pre_audit_analysis::generate_invariants(codeblock, repo).await?
-    } else {
-        ContractInvariants::default()
-    };
+    let (invariants_res, actors_res) = tokio::join!(
+        async {
+            if !SKIP_INVARIANT_RUNS {
+                pre_audit_analysis::generate_invariants(codeblock, repo).await
+            } else {
+                Ok(ContractInvariants::default())
+            }
+        },
+        async {
+            if !SKIP_ACTOR_PATTERN_RUNS {
+                pre_audit_analysis::generate_actors(codeblock, repo).await
+            } else {
+                Ok(Actors::default())
+            }
+        }
+    );
+
+    let invariants = invariants_res.expect("error extracting invariants");
+    let actors = actors_res.expect("error extracting actors");
 
     let multimodal_context = MultiModalContext { actors, invariants };
 
diff --git a/src/llm_review/analysis/pre_audit_analysis.rs b/src/llm_review/analysis/pre_audit_analysis.rs
index 314bb8d..e097ab2 100644
--- a/src/llm_review/analysis/pre_audit_analysis.rs
+++ b/src/llm_review/analysis/pre_audit_analysis.rs
@@ -35,7 +35,7 @@ pub async fn generate_actors(codeblock: &str, repo: &RepoPaths) -> Result<Actors
 pub async fn generate_invariants(codeblock: &str, repo: &RepoPaths) -> Result<ContractInvariants> {
     let invariant_prompt = IssuePrompt::Invariant(InvariantType::iter().collect());
 
-    let invariant_discovery_agent = generate_openai_agent(repo, "medium")?;
+    let invariant_discovery_agent = generate_invariant_openai_agent(repo, "medium")?;
     let invariant_verify_agent = generate_openai_agent(repo, "high")?;
 
     // Phase 1: Generate actors and their capabilities
@@ -102,6 +102,66 @@ pub fn generate_openai_agent(repo: &RepoPaths, reasoning_effort: &str) -> Result
     Ok(agent)
 }
 
+pub fn generate_invariant_openai_agent(
+    repo: &RepoPaths,
+    reasoning_effort: &str,
+) -> Result<Arc<AIAgent>> {
+    // custom agent for digging up list of actors
+    let config = AgentConfig::new(Some(repo.clone()))
+        .with_model("gpt-5.2")
+        .with_preamble(r#"
+You are a world-class expert at Solidity EVM smart contract auditing. You specialize in
+discovering invariants in complex solidity codebases.
+
+## How to think
+
+When designing each invariant:
+
+1. Model the contract and its role
+   - Identify what the contract is for (e.g. signature validation, vault, permissions, recovery module, oracle, router).
+   - Identify who the key actors are (owners, signers, admins, modules, external protocols).
+
+2. Extract candidate invariants from the spec and context
+   - Translate any stated invariants or assumptions in the docs/scope into precise, checkable properties.
+   - Think about:
+     - Access control and privilege boundaries.
+     - Balance and accounting relationships.
+     - Nonces, counters, and sequencing.
+     - Configuration / image hash / checkpointer behavior.
+     - Cross-contract or cross-chain relationships if referenced.
+
+3. Make them machine-checkable
+   - Express each invariant as a clear predicate over contract state and/or events.
+   - Use concrete conditions like:
+     - Relationships between balances and totals.
+     - Relationships between stored configuration and computed hashes.
+     - Conditions on who is allowed to perform which actions under which flags/modes.
+     - Temporal properties across function calls (e.g. nonces, cooldowns, checkpoints).
+
+4. Actively search for violations
+   - For each invariant, scan the code for:
+     - Branches that skip checks (e.g. flag bits, mode switches, early returns).
+     - Edge cases in loops, array indexing, or boundary conditions.
+     - Multi-step flows (chained signatures, batched calls, upgradable configs) where state may drift from the intended invariant.
+   - If you find a credible way the invariant could be broken, mark it as PossibleViolation (or equivalent status) and describe:
+     - The pre-state (relevant configuration / storage / role assumptions).
+     - The actions or sequence of calls.
+     - The post-state and why it violates the invariant.
+     - The likely impact.
+
+5. Coverage vs signal
+   - It is acceptable to include some simpler invariants if they help cover more potential High/Medium issues.
+   - Still avoid vague or purely stylistic "invariants"; each one should correspond to a concrete, checkable property whose violation could matter in practice.
+
+
+"#)
+        .with_file_retrieval(false)
+        .with_openai_reasoning_effort(reasoning_effort);
+
+    let agent = Arc::new(AgentFactory::create_openai_agent(&config)?);
+
+    Ok(agent)
+}
 pub fn generate_gemini_agent(repo: &RepoPaths) -> Result<Arc<AIAgent>> {
     // custom agent for digging up list of actors
     let config = AgentConfig::new(Some(repo.clone()))
diff --git a/src/llm_review/dynamic_prompts/findings_template.rs b/src/llm_review/dynamic_prompts/findings_template.rs
index 2d50750..deaa195 100644
--- a/src/llm_review/dynamic_prompts/findings_template.rs
+++ b/src/llm_review/dynamic_prompts/findings_template.rs
@@ -3,108 +3,16 @@ use std::collections::HashSet;
 use crate::{
     config::AuditType,
     llm_review::{
-        agent::agent_enums::{
-            all_enum_variants, generate_enum_bulleted_list, generate_enum_list, EnumData,
-        },
+        agent::agent_enums::{all_enum_variants, generate_enum_list, EnumData},
         findings::{
             finding_enums::{Severity, VulnerabilityType},
             findings::PrivilegeLevel,
         },
-        prompt_support::severity_rubics::{
-            CANTINA_SEVERITY_RUBRIC, CODE4RENA_SEVERITY_RUBRIC, SHERLOCK_SEVERITY_RUBRIC,
-        },
     },
     prepare_code::git_clone::RepoPaths,
 };
 use strum::IntoEnumIterator;
 
-pub fn generate_findings_prompt_for_multiple_patterns<T>(
-    issue_type: &str,
-    full_spec_of_issues: &str,
-    enum_issues: &[T],
-    repo: &RepoPaths,
-) -> String
-where
-    T: EnumData + std::fmt::Display + Default,
-{
-    let exploit_enums: Vec<VulnerabilityType> = enum_issues
-        .iter()
-        .flat_map(|e| e.to_types())
-        .copied()
-        .collect::<HashSet<_>>()
-        .into_iter()
-        .collect();
-    let exploit_bullets = generate_enum_bulleted_list(&exploit_enums); // "- Oracle\n- Reentrancy\n..."
-    let severity_rubic = match repo.audit_type {
-        AuditType::Sherlock => SHERLOCK_SEVERITY_RUBRIC,
-        AuditType::Cantina => CANTINA_SEVERITY_RUBRIC,
-        _ => CODE4RENA_SEVERITY_RUBRIC,
-    };
-
-    let json = get_pre_json_requirement_for_multipattern(enum_issues, issue_type, repo);
-
-    format!(
-        r#"
-
-        {json}
-
-        ===================== # INSTRUCTIONS =====================
-
-        Your job: analyze the main target contract **through the lens of the provided {pattern_type}** and enumerate the **top exploits/attack vectors** a hacker may deploy.
-
-        ## **Persist until all patterns are considered**
-        - Do **not** stop at the first interesting exploit.
-        - Your goal is **maximum coverage** – find every valid finding.
-        - Systematically go through **every** candidate block in "{title_all_caps} TO ANALYZE" and decide:
-            - "Real in-scope vulnerability keep as a finding"
-
-        ------------ ## Rules ------------
-
-        - Only report exploits **directly tied** to the provided {pattern_type} (patterns or invariants), **not** unrelated issues.
-        - Only analyze code **actually present** in the codebase. 
-        - Prefer exploits accessible to **unprivileged EOAs**; if an exploit requires a trusted role, make that clear via the `"privilege"` field (as specified in the JSON instructions).
-        - Focus on **present-state** bugs in the current code. Ignore one-time deployment/upgrade windows unless the same condition can be recreated or abused permissionlessly later.
-        - A valid finding must be:
-        - In-scope,
-        - Backed by a credible exploit path,
-        - And clearly severity according to the rubric.
-        - If nothing meets these criteria, return `{{"findings":[]}}`.
-
-        ------------ ## Severity rubric ------------
-
-        {rubric}
-
-
-        ------------ ## Exploit guidelines ------------
-
-        - Severity priority: **Theft > DoS > accounting mismatch**.
-        - Bigger **blast radius** and simpler execution are more valuable.
-        - Assert conditions using `assertGt` / `assertEq`, not just logs.
-        - For `"proof_of_code"`, the PoC should correspond to a **compilable Foundry test** (for example using `forge-std`, `vm.prank(attacker)`, etc.), as required by the JSON schema that follows.
-
-        ---
-
-        ------------ ## {issue_type} Overview ------------ 
-
-        ### Common Exploits
-
-        {exploit_bullets}
-
-        ---
-
-        ------------ ## {title_all_caps} TO ANALYZE ------------ 
-
-        {full_spec}
-
-        "#,
-        pattern_type = issue_type,
-        rubric = severity_rubic,
-        exploit_bullets = exploit_bullets,
-        full_spec = full_spec_of_issues,
-        title_all_caps = issue_type.to_uppercase()
-    )
-}
-
 pub fn get_post_json_requirement_for_multipattern<T>(
     patterns: &[T],
     pattern_type: &str,
diff --git a/src/llm_review/dynamic_prompts/invariants.rs b/src/llm_review/dynamic_prompts/invariants.rs
index 49b6cc6..3005b53 100644
--- a/src/llm_review/dynamic_prompts/invariants.rs
+++ b/src/llm_review/dynamic_prompts/invariants.rs
@@ -52,47 +52,6 @@ You must base your invariants on the following invariant types and their descrip
 
 You are not required to use every type, but you should prefer types that clearly match the contract's role (e.g. Balance, Permission, Temporal, StateMachine, Referential, Arithmetic, etc.).
 
----
-
-## How to think
-
-When designing each invariant:
-
-1. Model the contract and its role
-   - Identify what the contract is for (e.g. signature validation, vault, permissions, recovery module, oracle, router).
-   - Identify who the key actors are (owners, signers, admins, modules, external protocols).
-
-2. Extract candidate invariants from the spec and context
-   - Translate any stated invariants or assumptions in the docs/scope into precise, checkable properties.
-   - Think about:
-     - Access control and privilege boundaries.
-     - Balance and accounting relationships.
-     - Nonces, counters, and sequencing.
-     - Configuration / image hash / checkpointer behavior.
-     - Cross-contract or cross-chain relationships if referenced.
-
-3. Make them machine-checkable
-   - Express each invariant as a clear predicate over contract state and/or events.
-   - Use concrete conditions like:
-     - Relationships between balances and totals.
-     - Relationships between stored configuration and computed hashes.
-     - Conditions on who is allowed to perform which actions under which flags/modes.
-     - Temporal properties across function calls (e.g. nonces, cooldowns, checkpoints).
-
-4. Actively search for violations
-   - For each invariant, scan the code for:
-     - Branches that skip checks (e.g. flag bits, mode switches, early returns).
-     - Edge cases in loops, array indexing, or boundary conditions.
-     - Multi-step flows (chained signatures, batched calls, upgradable configs) where state may drift from the intended invariant.
-   - If you find a credible way the invariant could be broken, mark it as PossibleViolation (or equivalent status) and describe:
-     - The pre-state (relevant configuration / storage / role assumptions).
-     - The actions or sequence of calls.
-     - The post-state and why it violates the invariant.
-     - The likely impact.
-
-5. Coverage vs signal
-   - It is acceptable to include some simpler invariants if they help cover more potential High/Medium issues.
-   - Still avoid vague or purely stylistic "invariants"; each one should correspond to a concrete, checkable property whose violation could matter in practice.
     "#,
         invariants = invariant_categories
     )
diff --git a/src/llm_review/threat_models/issues.rs b/src/llm_review/threat_models/issues.rs
index 9006813..ef942f5 100644
--- a/src/llm_review/threat_models/issues.rs
+++ b/src/llm_review/threat_models/issues.rs
@@ -7,11 +7,8 @@ use crate::{
             agent_enums::AIAgent,
             agent_factory::{AgentConfig, AgentFactory},
         },
-        dynamic_prompts::{
-            findings_template::get_post_json_requirement_for_multipattern,
-            invariants::{
-                self, generate_all_invariants_verify_prompt, get_post_all_invariants_verify_json,
-            },
+        dynamic_prompts::invariants::{
+            self, generate_all_invariants_verify_prompt, get_post_all_invariants_verify_json,
         },
         findings::findings::{Finding, Findings},
         phases::{rounds::all_rounds::AllRoundLegitAnalysis, verify_rounds::FindingAnalysis},
@@ -19,7 +16,6 @@ use crate::{
         threat_models::actors::Actors,
         utils::prompt_context::{self, FindingReportType},
     },
-    prepare_code::git_clone::RepoPaths,
     utils::semantic_compare,
 };
 
@@ -54,8 +50,6 @@ pub trait IssueStructTrait: Send + Sync + Sized + 'static {
     fn new(issues: Vec<Self::Spec>) -> Self;
     fn generate_verify_prompt(&self) -> String;
     fn verify_json_required_prompt() -> String;
-    fn multi_issue_to_findings_prompt(&self, repo: &RepoPaths) -> String;
-    fn multi_issue_findings_json_required_prompt(&self, repo: &RepoPaths) -> String;
     async fn dedup(self) -> anyhow::Result<Self>;
     fn issue_title(&self) -> String;
 }
@@ -158,13 +152,6 @@ impl IssueStructTrait for ContractInvariants {
     fn issue_title(&self) -> String {
         "invariant".to_string()
     }
-    fn multi_issue_to_findings_prompt(&self, _: &RepoPaths) -> String {
-        unimplemented!("Not implimented for ContractInvariants");
-    }
-    fn multi_issue_findings_json_required_prompt(&self, repo: &RepoPaths) -> String {
-        let invariants: Vec<InvariantType> = self.issues().iter().map(|p| p.inv_type).collect();
-        get_post_json_requirement_for_multipattern(&invariants, "Invariant", repo)
-    }
 }
 
 #[async_trait]
@@ -186,14 +173,6 @@ impl IssueStructTrait for Findings {
         "finding".to_string()
     }
     // NOTE: not need for this case
-    fn multi_issue_to_findings_prompt(&self, _repo: &RepoPaths) -> String {
-        unimplemented!("Not implimented for Findings");
-    }
-    // NOTE: not need for this case
-    fn multi_issue_findings_json_required_prompt(&self, _repo: &RepoPaths) -> String {
-        unimplemented!("Not implimented for Findings");
-    }
-    // NOTE: not need for this case
     fn generate_verify_prompt(&self) -> String {
         unimplemented!("Not implimented for Findings");
     }
diff --git a/src/llm_review/threat_models/pattern_category.rs b/src/llm_review/threat_models/pattern_category.rs
index 73c5549..4190cec 100644
--- a/src/llm_review/threat_models/pattern_category.rs
+++ b/src/llm_review/threat_models/pattern_category.rs
@@ -1,9 +1,5 @@
 use crate::{
-    config::{
-        MAX_PATTERN_GENERAL, MAX_PATTERN_LIBRARY, MAX_PATTERN_NICHE, MAX_PATTERN_RELEVANT_FREQUENT,
-        MAX_PATTERN_RUN_FREQUENT, MAX_PATTERN_RUN_MOST, MAX_PATTERN_RUN_RARE, MAX_PATTERN_RUN_TOP,
-        R1_RUNS, R2_RUNS,
-    },
+    config::{MAX_PATTERN_LIBRARY, MAX_PATTERN_NICHE, R1_RUNS, R2_RUNS},
     llm_review::threat_models::patterns::VulnerabilityPattern,
 };
 use std::{collections::HashMap, sync::OnceLock};
@@ -243,41 +239,6 @@ pub const PATTERN_CATEGORY_LIBRARY: &[PatternCategorySpec] = &[
         tier: PatternTier::Tier1,
         runs: MAX_PATTERN_LIBRARY,
     },
-    PatternCategorySpec {
-        category: PatternCategory::SimulationTestingHelper,
-        title: "Simulation/Testing Helper",
-        issues: SIMULATION_TESTING_HELPER_PATTERNS,
-        tier: PatternTier::Tier1,
-        runs: MAX_PATTERN_LIBRARY,
-    },
-    PatternCategorySpec {
-        category: PatternCategory::General,
-        title: "Common High/Medium Vulnerabilities",
-        issues: COMMON_PATTERNS,
-        tier: PatternTier::Tier1,
-        runs: MAX_PATTERN_GENERAL,
-    },
-    PatternCategorySpec {
-        category: PatternCategory::Top,
-        title: "Top Code4rena",
-        issues: TOP_PAID_PATTERNS,
-        tier: PatternTier::Tier1,
-        runs: MAX_PATTERN_RUN_TOP,
-    },
-    PatternCategorySpec {
-        category: PatternCategory::Rare,
-        title: "Top Code4rena",
-        issues: RARE_PATTERNS,
-        tier: PatternTier::Tier1,
-        runs: MAX_PATTERN_RUN_RARE,
-    },
-    PatternCategorySpec {
-        category: PatternCategory::Frequent,
-        title: "Most Frequent Code4rena",
-        issues: FREQUENT_PATTERNS,
-        tier: PatternTier::Tier1,
-        runs: MAX_PATTERN_RUN_FREQUENT,
-    },
     PatternCategorySpec {
         category: PatternCategory::R1,
         title: "Common Code4rena",
@@ -292,20 +253,6 @@ pub const PATTERN_CATEGORY_LIBRARY: &[PatternCategorySpec] = &[
         tier: PatternTier::Tier1,
         runs: R2_RUNS,
     },
-    PatternCategorySpec {
-        category: PatternCategory::Relevant,
-        title: "Most Relevant Code4rena",
-        issues: RELEVANT_PATTERNS,
-        tier: PatternTier::Tier1,
-        runs: MAX_PATTERN_RELEVANT_FREQUENT,
-    },
-    PatternCategorySpec {
-        category: PatternCategory::MostObserved,
-        title: "Most Frequent Code4rena",
-        issues: TOP_OBSERVED_PATTERNS,
-        tier: PatternTier::Tier1,
-        runs: MAX_PATTERN_RUN_MOST,
-    },
     PatternCategorySpec {
         category: PatternCategory::Library,
         title: "Most Frequent Library",
@@ -573,40 +520,6 @@ pub const RANDOMNESS_RAFFLE_LOTTERY_PATTERNS: &[VulnerabilityPattern; 10] = &[
     VulnerabilityPattern::ConfigFootgun,
 ];
 
-// RELEVANT_PATTERNS moved to patterns.rs - use super::patterns::RELEVANT_PATTERNS
-
-pub const COMMON_PATTERNS: &[VulnerabilityPattern; 20] = &[
-    // Core Access Control & Auth (3 patterns) - Keep most critical, remove signature-specific
-    VulnerabilityPattern::AccessControlOrAuthByPass, // High - universal auth bypass
-    VulnerabilityPattern::DoubleExecutionOrReplay,   // HighMedium - replay attacks
-    VulnerabilityPattern::UnprotectedPauseOrStop,    // HighMedium - emergency controls
-    // Reentrancy & Call Order (3 patterns) - Consolidate overlaps
-    VulnerabilityPattern::Reentrancy, // High - covers CEIViolation + classic reentrancy
-    VulnerabilityPattern::MulticallCrossPathReentrancy,
-    VulnerabilityPattern::ExternalCallAfterStateChange, // Medium - CEI violations
-    // Economic & Flash Loans (2 patterns) - Remove oracle-specific patterns
-    VulnerabilityPattern::FlashLoanEconomicManipulation, // High - universal economic attack
-    VulnerabilityPattern::SlippageMissingOrInsufficient, // Medium - common in any swap/trade
-    // Accounting & Invariants (3 patterns) - Remove vault-specific
-    VulnerabilityPattern::AccountingInvariantViolation, // HighMedium - universal accounting
-    VulnerabilityPattern::FeeAccountingDrift,
-    VulnerabilityPattern::PrecisionDriftAccumulation, // Medium - rounding errors
-    VulnerabilityPattern::UnsafeRecipient,            // HighMedium - recipient validation
-    // Token Standards (2 patterns) - Keep most common
-    VulnerabilityPattern::AllowanceRace, // Medium - approve race condition
-    // Upgradeability & Proxies (2 patterns) - Keep highest impact, remove proxy-specific
-    VulnerabilityPattern::UpgradeAuthBypass, // High - universal upgrade risk
-    VulnerabilityPattern::InitOrderOrUnintialized, // HighMedium - init vulnerabilities
-    // DoS & Complexity (2 patterns) - Keep most common
-    VulnerabilityPattern::UnboundedLoops,     // Medium - gas/DoS
-    VulnerabilityPattern::GriefableCallbacks, // Medium - callback griefing
-    // Low-Level & Assembly (2 patterns) - Keep critical
-    VulnerabilityPattern::UncheckedLowLevelCallResults, // Medium - silent failures
-    VulnerabilityPattern::UnsafeAssembyTypeCasts,       // HighMedium - type safety
-    // Universal Edge Cases (1 pattern)
-    VulnerabilityPattern::ForcedAssetVsStrictEquality, // High - strict equality bugs
-];
-
 pub const LIBRARY_ANALYSIS_PATTERNS: &[VulnerabilityPattern; 20] = &[
     // Math & Precision (4)
     VulnerabilityPattern::PricePrecisionOrRoundingError,
@@ -637,137 +550,8 @@ pub const LIBRARY_ANALYSIS_PATTERNS: &[VulnerabilityPattern; 20] = &[
     VulnerabilityPattern::PermitMisuse,
 ];
 
-pub const TOP_OBSERVED_PATTERNS: &[VulnerabilityPattern; 10] = &[
-    VulnerabilityPattern::AccountingInvariantViolation,
-    VulnerabilityPattern::GriefableCallbacks,
-    VulnerabilityPattern::UnboundedLoops,
-    VulnerabilityPattern::FlashLoanEconomicManipulation,
-    VulnerabilityPattern::FeeOnTransferAssumption,
-    VulnerabilityPattern::AccessControlOrAuthByPass,
-    VulnerabilityPattern::FeeOnTransferAssumption,
-    VulnerabilityPattern::ReserveOrPriceDesync,
-    VulnerabilityPattern::MaturityorGatingByPass,
-    VulnerabilityPattern::StandardViolation,
-];
-
-pub const RARE_PATTERNS: &[VulnerabilityPattern; 13] = &[
-    VulnerabilityPattern::AccountingInvariantViolation,
-    VulnerabilityPattern::FeeAccountingDrift,
-    VulnerabilityPattern::ERC4626SharePriceMismatch,
-    VulnerabilityPattern::ERC20DecimalsMismatch,
-    VulnerabilityPattern::MulticallCrossPathReentrancy,
-    VulnerabilityPattern::TWAPWindowPinningOrLowLiquidity,
-    VulnerabilityPattern::StaleOracleAcceptance,
-    VulnerabilityPattern::ForcedAssetVsStrictEquality,
-    VulnerabilityPattern::UntrustedDelegateCall,
-    VulnerabilityPattern::SlippageMissingOrInsufficient,
-    VulnerabilityPattern::PermitMisuse,
-    VulnerabilityPattern::PermitFrontRun,
-    VulnerabilityPattern::BeaconOrFactoryAuthorityDrift,
-];
-
-pub const FREQUENT_PATTERNS: &[VulnerabilityPattern; 15] = &[
-    VulnerabilityPattern::SlippageMissingOrInsufficient,
-    VulnerabilityPattern::UnboundedLoops,
-    VulnerabilityPattern::FeeOnTransferAssumption,
-    VulnerabilityPattern::ReserveOrPriceDesync,
-    VulnerabilityPattern::PricePrecisionOrRoundingError,
-    VulnerabilityPattern::PrecisionDriftAccumulation,
-    VulnerabilityPattern::ExternalCallAfterStateChange,
-    VulnerabilityPattern::GriefableCallbacks,
-    VulnerabilityPattern::StateGrowthOrStorageBloat,
-    VulnerabilityPattern::EpochOrIndexMonotonicity,
-    VulnerabilityPattern::TimestampOrBlockManipulation,
-    VulnerabilityPattern::BlockhashOrPRNGWeakness,
-    // NEW:
-    VulnerabilityPattern::MulticallCrossPathReentrancy,
-    VulnerabilityPattern::TWAPWindowPinningOrLowLiquidity,
-    VulnerabilityPattern::ForcedAssetVsStrictEquality,
-];
-
 // TOP VULNERABILITY PATTERNS
 
-/// Curated list of High/Medium severity patterns for focused analysis.
-/// Excludes Low-severity patterns (TimestampOrBlockManipulation, ChainIdorDomainDrift).
-pub const RELEVANT_PATTERNS: &[VulnerabilityPattern; 61] = &[
-    // auth bypass
-    VulnerabilityPattern::AccessControlOrAuthByPass, // High
-    VulnerabilityPattern::GovernanceDelegationFlaw,
-    VulnerabilityPattern::DoubleExecutionOrReplay,
-    VulnerabilityPattern::PermitOrSignatureReplay,
-    VulnerabilityPattern::EIP1271ByPass,
-    VulnerabilityPattern::ConfigFootgun, // untrusted admin
-    // Call Order, Reentrancy, External calls
-    VulnerabilityPattern::CEIViolation,
-    VulnerabilityPattern::Reentrancy, // High
-    VulnerabilityPattern::ReadOnlyReentrancy,
-    VulnerabilityPattern::ExternalCallAfterStateChange, // Medium
-    // Economic, Market, & Oracle
-    VulnerabilityPattern::SlippageMissingOrInsufficient,
-    VulnerabilityPattern::OracleUsingDEXorTWAP,
-    VulnerabilityPattern::FlashLoanEconomicManipulation, // High
-    VulnerabilityPattern::FeeOnTransferAssumption,
-    VulnerabilityPattern::ReserveOrPriceDesync,
-    // Accounting & Invariants
-    VulnerabilityPattern::AccountingInvariantViolation,
-    VulnerabilityPattern::UnsafeRecipient,
-    VulnerabilityPattern::PrecisionDriftAccumulation, // Medium
-    VulnerabilityPattern::PricePrecisionOrRoundingError,
-    // Token Standard Allowance
-    VulnerabilityPattern::StandardViolation,
-    VulnerabilityPattern::AllowanceRace, // Medium
-    VulnerabilityPattern::PermitMisuse,
-    // Upgradeability, Proxies, & Init
-    VulnerabilityPattern::UpgradeAuthBypass,
-    VulnerabilityPattern::InitOrderOrUnintialized,
-    VulnerabilityPattern::StorageCollisionOrSelectorClash,
-    VulnerabilityPattern::SelfdestructOrMetamorphicFootguns,
-    // Lifecycle & State Machines
-    VulnerabilityPattern::MaturityorGatingByPass,
-    VulnerabilityPattern::EpochOrIndexMonotonicity,
-    // DoS, Gas, and Complexity - all Mediums
-    VulnerabilityPattern::UnboundedLoops,
-    VulnerabilityPattern::GriefableCallbacks,
-    VulnerabilityPattern::StateGrowthOrStorageBloat,
-    // Randomness - Medium
-    VulnerabilityPattern::BlockhashOrPRNGWeakness,
-    // Cross-Chain & Bridging - only for L2 or bridges
-    VulnerabilityPattern::CrossChainMessageSpoofing,
-    VulnerabilityPattern::FinalityOrReplayAcrossDomains,
-    // EVM/Assembly & Low-Level
-    VulnerabilityPattern::UncheckedLowLevelCallResults,
-    VulnerabilityPattern::UnsafeAssembyTypeCasts,
-    VulnerabilityPattern::DivideByZeroOrOverFlowInCustomMath,
-    // ETH/WETH & Payment Flows
-    VulnerabilityPattern::EthVsWethConfusion,
-    VulnerabilityPattern::PullorPushPaymentbugs,
-    // Token behaviors
-    VulnerabilityPattern::NonStandardERC20Behavior,
-    VulnerabilityPattern::ERC20DecimalsMismatch,
-    VulnerabilityPattern::ERC777HookReentrancy,
-    VulnerabilityPattern::StaleOracleAcceptance,
-    VulnerabilityPattern::SandwichableOracle,
-    VulnerabilityPattern::PermitFrontRun,
-    VulnerabilityPattern::UnprotectedPauseOrStop,
-    VulnerabilityPattern::UntrustedDelegateCall,
-    VulnerabilityPattern::ReplayAcrossForksOrL2s,
-    VulnerabilityPattern::ERC4626SharePriceMismatch,
-    VulnerabilityPattern::FeeAccountingDrift,
-    // NEW (10/20/2025)
-    VulnerabilityPattern::BeaconOrFactoryAuthorityDrift,
-    VulnerabilityPattern::TimelockEdgeCase,
-    VulnerabilityPattern::MulticallCrossPathReentrancy,
-    VulnerabilityPattern::TWAPWindowPinningOrLowLiquidity,
-    VulnerabilityPattern::ForcedAssetVsStrictEquality,
-    // NEW (12/16/2025) - Missing Megapot patterns
-    VulnerabilityPattern::ArbitraryExternalCall,
-    VulnerabilityPattern::GlobalParamMidFlowManipulation,
-    VulnerabilityPattern::GovernanceFrontrunDoS,
-    VulnerabilityPattern::ExternalProtocolKeyCollision,
-    VulnerabilityPattern::EmergencyModeStateStuck,
-    VulnerabilityPattern::IncentiveMisalignmentOrGameTheory,
-];
-
 /// Curated list of High/Medium severity patterns for focused analysis.
 /// Excludes Low-severity patterns (TimestampOrBlockManipulation, ChainIdorDomainDrift).
 pub const R1_PATTERNS: &[VulnerabilityPattern; 49] = &[
@@ -932,31 +716,3 @@ pub const REENTRANCY_GUARD_LIBRARY_PATTERNS: &[VulnerabilityPattern; 4] = &[
     VulnerabilityPattern::MulticallCrossPathReentrancy,
     VulnerabilityPattern::CEIViolation,
 ];
-
-pub const SIMULATION_TESTING_HELPER_PATTERNS: &[VulnerabilityPattern; 5] = &[
-    VulnerabilityPattern::ExternalCallAfterStateChange,
-    VulnerabilityPattern::UnboundedLoops,
-    VulnerabilityPattern::UncheckedLowLevelCallResults,
-    VulnerabilityPattern::GriefableCallbacks,
-    VulnerabilityPattern::StateGrowthOrStorageBloat,
-];
-
-pub const TOP_PAID_PATTERNS: &[VulnerabilityPattern; 16] = &[
-    VulnerabilityPattern::AccessControlOrAuthByPass,
-    VulnerabilityPattern::GovernanceDelegationFlaw,
-    VulnerabilityPattern::DoubleExecutionOrReplay,
-    VulnerabilityPattern::EIP1271ByPass,
-    VulnerabilityPattern::PermitOrSignatureReplay,
-    VulnerabilityPattern::Reentrancy,
-    VulnerabilityPattern::FlashLoanEconomicManipulation,
-    VulnerabilityPattern::OracleUsingDEXorTWAP,
-    VulnerabilityPattern::AccountingInvariantViolation,
-    VulnerabilityPattern::StandardViolation,
-    VulnerabilityPattern::PermitMisuse,
-    VulnerabilityPattern::InitOrderOrUnintialized,
-    VulnerabilityPattern::UpgradeAuthBypass,
-    VulnerabilityPattern::StorageCollisionOrSelectorClash,
-    // NEW:
-    VulnerabilityPattern::BeaconOrFactoryAuthorityDrift,
-    VulnerabilityPattern::TimelockEdgeCase,
-];
