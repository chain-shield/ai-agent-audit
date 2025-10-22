pub const POST_CREATE_POC: &str = r#"

### OUTPUT REQUIREMENTS

**Please respond with ONLY valid JSON in the following exact format:**

{
    "poc_test_code": "fully runnable PoC code as a string (escape newlines and quotes properly). Test function MUST start with 'test' (e.g., testExploit, testReentrancy)",
    "commentary": "brief explanation of your PoC approach and what it demonstrates",
    "cannot_create_poc_because_finding_invalid": true|false
}

**IMPORTANT:**
- The `commentary` field should briefly explain your PoC approach, what it demonstrates, and any important notes.
- The `cannot_create_poc_because_finding_invalid` field is OPTIONAL. Only include it and set to `true` if the finding is invalid.
- If omitted or set to `false`, the system assumes you created a valid PoC.
- Use proper JSON escaping for all string fields (escape `"` as `\"` and newlines as `\n`).
- Do NOT include markdown code fences (```) in your response.
- Do NOT include any explanatory text before or after the JSON.
- Respond with ONLY the raw JSON object.

**Example Response:**

{
    "poc_test_code": "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\nimport \"forge-std/Test.sol\";\n\ncontract ExploitTest is Test {\n    function testExploit() public {\n        // exploit code here\n    }\n}",
    "commentary": "This PoC demonstrates reentrancy by calling refund() recursively before state update, draining the contract balance"
}

"#;
