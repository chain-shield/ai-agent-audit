pub const DEDUP_PROMPT: &str = r#"

SYSTEM
You are a Solidity-security triager.  
Answer with exactly **YES** or **NO** (no punctuation, no prose).

USER
Are these two vulnerability reports describing the *same root-cause*?

---- REPORT A ----
Issue Type : {issue_type}
Contract    : {contract}
Function    : {function}
Description : {description_a}

---- REPORT B ----
Issue Type : {issue_type}
Contract    : {contract}
Function    : {function}
Description : {description_b}

Remember: root-cause means the exact same bug, not just similar wording.
Answer:
"#;
