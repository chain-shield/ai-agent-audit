pub fn generated_section_header(section_title: &str, section_num: u8) -> String {
    format!(
        r#"
<!-- ═════════════════════════════════════════════════════════════════════════════════════════ -->
<!-- SECTION {}: {} -->
<!-- ═════════════════════════════════════════════════════════════════════════════════════════ -->
                "#,
        section_num, section_title
    )
}

pub fn generated_sub_header(
    section_title: &str,
    section_num: u8,
    sub_section_num: usize,
) -> String {
    format!(
        r#"
════════════════════════════════════════════════════════════════════
███ SECTION {}.{}: {} ███
════════════════════════════════════════════════════════════════════
                "#,
        section_num, sub_section_num, section_title
    )
}

pub fn generated_table_of_context_header(section_title: &str, section_num: u8) -> String {
    format!("### {}. {}\n", section_num, section_title)
}

/// Generate table of contents for Actor Discovery prompts
/// Structure: Sections 1, 7, 8, 9 (Section 1 OUTPUT FORMAT is at the end of the prompt)
pub fn generate_actor_discovery_table_of_contents() -> String {
    String::from(
        r#"
## Table Of Contents

### 1. Core Instructions
- 1.1 Systematic Actor Discovery Process
- 1.2 Step 1: Map All Entry Points
- 1.3 Step 2: Identify Role-Based Actors
- 1.4 Step 3: Identify Interaction-Based Actors
- 1.5 Step 4: Identify Temporal/MEV Actors
- 1.6 Step 5: Identify Missing Actors Checklist
- 1.7 Deliverables

### 7. Solidity Code to Review
- 7.1 Main Target Contract
- 7.2 Supporting Contracts, Libraries & Interfaces
- 7.3 Interfaces and Root Implementations
- 7.4 External Libraries
- 7.5 Deployment Scripts

### 8. Additional Context
- 8.1 Protocol Overview
- 8.2 Main List of Files in Project
- 8.3 Documentation
- 8.4 Package.json Headers of Lib Packages
- 8.5 Config Files

### 9. Audit Scope and Key Invariants Provided by Client
- 9.1 Privileged Roles
- 9.2 Client Provided Audit Scope
"#,
    )
}

/// Generate table of contents for Invariant Discovery prompts
/// Structure: Sections 1, 7, 8, 9 (Section 1 OUTPUT FORMAT is at the end of the prompt)
pub fn generate_invariant_discovery_toc() -> String {
    String::from(
        r#"
## Table Of Contents

### 1. Core Instructions
- 1.1 Your Goals
- 1.2 Sources of Truth
- 1.3 Invariant Types to Focus On
- 1.4 How to Think

### 7. Solidity Code to Review
- 7.1 Main Target Contract
- 7.2 Supporting Contracts, Libraries & Interfaces
- 7.3 Interfaces and Root Implementations
- 7.4 External Libraries
- 7.5 Deployment Scripts

### 8. Additional Context
- 8.1 Protocol Overview
- 8.2 Main List of Files in Project
- 8.3 Documentation
- 8.4 Package.json Headers of Lib Packages
- 8.5 Config Files

### 9. Audit Scope and Key Invariants Provided by Client
- 9.1 Privileged Roles
- 9.2 Client Provided Audit Scope
"#,
    )
}

/// Generate table of contents for Verification Round prompts
/// Structure: Sections 1 (with 1.1-1.11), 2, 8, 9, 10, 11
pub fn generate_verification_round_toc() -> String {
    String::from(
        r#"
## Table Of Contents

### 1. Core Instructions
- 1.1 Verify Security Finding Exists
- 1.2 Existing Safeguards Check
- 1.3 Scope Check
- 1.4 By Design Check
- 1.5 Exploitability Check
- 1.6 Impact Classification Check
- 1.7 Likelihood Assessment Check
- 1.8 User Error Check
- 1.9 Governance/Centralization Risk Check
- 1.10 Speculation Check
- 1.11 Non-Standard ERC20 Token Check

### 2. Security Findings to Evaluate

### 8. Solidity Code to Review

### 9. Additional Context
- 9.1 Protocol Overview
- 9.2 Main List of Files in Project
- 9.3 Documentation
- 9.4 Package.json Headers of Lib Packages
- 9.5 Config Files

### 10. Audit Scope and Key Invariants Provided by Client
- 10.1 Privileged Roles
- 10.2 Client Provided Audit Scope

### 11. Output Requirements
- 11.1 JSON Output Requirement
"#,
    )
}

/// Generate table of contents for Validation Round prompts
/// Structure: Main instructions, Section 10, Codebase, Output requirements
pub fn generate_validation_round_toc() -> String {
    String::from(
        r#"
## Table Of Contents

### 1. Core Instructions
- Task Overview
- Security Findings + Reasons They Were Downgraded

### 2. Audit Scope and Key Invariants Provided by Client
- 2.1 Privileged Roles
- 2.2 Client Provided Audit Scope

### 3. Codebase Where Findings Were Found

### 4. Output Requirements
- JSON Output Format
"#,
    )
}

pub fn generate_pattern_category_to_finding_discovery_prompt(
    pattern_index: &str,
    actor_or_invariant_index: &str,
) -> String {
    format!(
        r#"
## Table Of Contents

### 1. Output Format Requirements
- 1.1 JSON Output Schema

### 2. Core Instructions
- 2.1 Analysis Objectives
- 2.2 Vulnerability Patterns to Detect

{pattern_index}

### 4. Analysis Guidelines
- 4.1 Exploit Guidelines
- 4.2 Analysis Rules
- 4.3 Semantic Multi-Step Hunting Checklist

### 5. Attack Pattern Examples
- 5.1 Incentives / Game Theory Example
- 5.2 Snapshot vs Live Read Example
- 5.3 Probabilistic / Randomness Example

### 6. Severity Framework
- 6.1 Severity Classifications
- 6.2 Risk Estimation
- 6.3 Asset Loss Guidelines
- 6.4 Special Cases & Edge Rules

{actor_or_invariant_index}

### 8. Solidity Code to Review
- 8.1 Main Target Contract
- 8.2 Supporting Contracts, Libraries & Interfaces
- 8.3 Interfaces and Root Implementations
- 8.4 External Libraries
- 8.5 Deployment Scripts

### 9. Additional Context
- 9.1 Protocol Overview
- 9.2 Main List of Files in Project
- 9.3 Documentation
- 9.4 Package.json Headers of Lib Packages
- 9.5 Config Files

### 10. Audit Scope and Key Invariants Provided by Client
- 10.1 Privileged Roles

### 11. Output Requirements
- 11.1 JSON Output Format

    "#
    )
}
