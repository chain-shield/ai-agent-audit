## Relevant Code Snippets/nhttps://github.com/Cyfrin/4-puppy-raffle-audit/blob/3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960/src/PuppyRaffle.sol#L133-L152/nhttps://github.com/Cyfrin/4-puppy-raffle-audit/blob/3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960/src/PuppyRaffle.sol#L155-L161/nhttps://github.com/Cyfrin/4-puppy-raffle-audit/blob/3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960/src/PuppyRaffle.sol#L27-L28/n

# Integer Overflow in Fee Accumulation Permanently Locks Protocol Revenue

**Severity:** High
**Affected Contracts:** `PuppyRaffle.sol` — function `selectWinner` at line 139 and `withdrawFees` at line 156

## Summary

The `totalFees` state variable is declared as `uint64`, but fee amounts computed from raffle rounds can easily exceed the maximum value representable by a 64-bit unsigned integer (2^64 - 1 ≈ 18.44 ether). When `selectWinner()` casts a large `uint256` fee to `uint64` and accumulates it without checked arithmetic in Solidity 0.7.6, the value truncates or wraps around. This causes a permanent mismatch between the contract's actual ETH balance and the recorded `totalFees`, rendering `withdrawFees()` permanently inoperable and locking all protocol revenue in the contract.

## Description

### Root Cause Analysis

The vulnerability originates from two design flaws in the fee accounting mechanism:

1. **Undersized storage type**: `totalFees` is declared as `uint64` (line 28):
   ```solidity
   uint64 public totalFees = 0;
   ```
   A `uint64` can hold at most 18,446,744,073,709,551,615 wei (approximately 18.44 ether).

2. **Unsafe downcasting without overflow protection**: In `selectWinner()` (line 139), the computed fee is cast from `uint256` to `uint64` and added to `totalFees`:
   ```solidity
   uint256 fee = (totalAmountCollected * 20) / 100;
   totalFees = totalFees + uint64(fee);
   ```
   
   Solidity 0.7.6 does **not** have built-in overflow checks. When `fee` exceeds `type(uint64).max`, the cast silently truncates the high-order bits, storing only the lower 64 bits. Subsequent addition can further wrap around.

For example, with the default `entranceFee = 1 ether` and 100 players:
- Total collected: 100 ether
- Fee (20%): 20 ether = 20,000,000,000,000,000,000 wei
- `type(uint64).max` = 18,446,744,073,709,551,615 wei
- Since 20 ether > `type(uint64).max`, casting to `uint64` truncates the value to approximately 1.55 ether (the remainder after modulo 2^64).

After `selectWinner()` sends 80 ether to the winner, the contract retains the full 20 ether as fees. However, `totalFees` records only the truncated ~1.55 ether.

3. **Strict balance check in withdrawal**: The `withdrawFees()` function (line 156) enforces:
   ```solidity
   require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
   ```
   
   This invariant assumes `totalFees` accurately reflects all accumulated fees. Once truncation occurs, `address(this).balance` (20 ether) will never equal `totalFees` (~1.55 ether), causing the `require` to fail permanently—even when no players are active.

### Exploit Scenario

1. **Initial State**: The `PuppyRaffle` contract is deployed with `entranceFee = 1 ether` and `raffleDuration = 1 day`.

2. **Step 1 — Mass Entry**: An attacker (or any group of users) enters the raffle with 100 unique addresses, paying 100 ether total:
   ```solidity
   raffle.enterRaffle{value: 100 ether}(arrayOf100Addresses);
   ```

3. **Step 2 — Raffle Concludes**: After the raffle duration elapses, anyone calls `selectWinner()`:
   - Winner is selected pseudo-randomly.
   - Prize pool (80 ether) is transferred to the winner.
   - Fee calculation: `fee = (100 ether * 20) / 100 = 20 ether`.
   - Fee accumulation: `totalFees = totalFees + uint64(20 ether)`.
   - Because 20 ether exceeds `uint64` capacity, the cast truncates the value. `totalFees` now holds approximately 1.55 ether instead of 20 ether.
   - Contract balance: 20 ether (the actual fee retained).

4. **Step 3 — Withdrawal Attempt Fails**: The protocol owner attempts to withdraw fees:
   ```solidity
   raffle.withdrawFees();
   ```
   - The function checks `require(address(this).balance == uint256(totalFees))`.
   - `address(this).balance` = 20 ether.
   - `uint256(totalFees)` ≈ 1.55 ether.
   - The condition fails, reverting with `"PuppyRaffle: There are currently players active!"`.

5. **Step 4 — Permanent Lock**: No subsequent action can reconcile the mismatch. Even if future rounds complete successfully, the discrepancy persists (and may worsen with additional truncations). The 20 ether is permanently locked in the contract.

### Relevant Context

- This issue is **permissionless**: any user can trigger it by entering a sufficiently large raffle (≥93 players at 1 ether entrance fee).
- The vulnerability is **deterministic**: it will occur in every deployment where a single round's fee exceeds ~18.44 ether.
- Solidity 0.7.6 lacks automatic overflow/underflow protection (introduced in 0.8.0), making unchecked arithmetic operations dangerous.

## Impact

This vulnerability has **High** severity due to the following impacts:

1. **Total Loss of Protocol Revenue**: All fees accumulated in affected rounds become permanently unwithdrawable. For a raffle with 100 players at 1 ether each, 20 ether (~$40,000 at $2,000/ETH) is locked per round.

2. **Irreversible State Corruption**: Once `totalFees` diverges from the actual balance, the contract cannot self-correct. The owner has no administrative function to override the balance check or reset `totalFees`.

3. **Cumulative Damage**: Each subsequent large round worsens the mismatch, compounding locked funds. A protocol running multiple rounds could lock hundreds of ether.

4. **Reputational Harm**: Users and winners receive their prizes, but the protocol operator loses all revenue, undermining trust and sustainability.

5. **No Privileged Access Required**: Any participant can inadvertently or maliciously trigger the bug by organizing a large raffle entry, making this a **permissionless exploit**.

In the worst credible scenario, a malicious actor could deliberately trigger this bug in the first round, immediately bricking the fee withdrawal mechanism and forcing the protocol to redeploy (losing all locked funds).

## Recommended Mitigation

Implement the following fixes to eliminate the vulnerability:

### 1. Upgrade `totalFees` to `uint256`

Change the storage type to match the fee calculation precision:

```solidity
// Line 28
uint256 public totalFees = 0;
```

This prevents truncation for any realistic fee amount (up to the total ETH supply).

### 2. Use Checked Arithmetic (SafeMath for Solidity 0.7.6)

Since Solidity 0.7.6 does not have built-in overflow checks, import and use OpenZeppelin's `SafeMath` library:

```solidity
import {SafeMath} from "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle is ERC721, Ownable {
    using SafeMath for uint256;
    
    // ...
    
    function selectWinner() external {
        // ...
        uint256 fee = (totalAmountCollected * 20) / 100;
        totalFees = totalFees.add(fee);  // SafeMath prevents overflow
        // ...
    }
}
```

This ensures that any overflow in fee accumulation reverts the transaction instead of silently corrupting state.

### 3. Replace Balance-Based Withdrawal Check with Explicit State Tracking

The current `withdrawFees()` relies on an exact balance match, which is fragile. Instead, check whether the raffle is idle:

```solidity
// Line 156-161
function withdrawFees() external onlyOwner {
    require(players.length == 0, "PuppyRaffle: Raffle is currently active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

This decouples fee withdrawal from balance arithmetic, preventing edge cases where refunds or rounding errors cause mismatches.

### 4. (Optional) Upgrade to Solidity 0.8.x

For long-term safety, migrate to Solidity 0.8.0 or later, which includes automatic overflow/underflow checks. This eliminates the need for `SafeMath` and prevents an entire class of vulnerabilities.

### Summary of Changes

| Location | Current Code | Recommended Fix |
|----------|--------------|------------------|
| Line 28 | `uint64 public totalFees = 0;` | `uint256 public totalFees = 0;` |
| Line 139 | `totalFees = totalFees + uint64(fee);` | `totalFees = totalFees.add(fee);` (with SafeMath) |
| Line 156 | `require(address(this).balance == uint256(totalFees), ...);` | `require(players.length == 0, ...);` |

Implementing these changes will ensure fee accounting remains accurate and withdrawals function correctly regardless of raffle size.

## References

- **Solidity 0.8.0 Release Notes**: [Checked Arithmetic by Default](https://blog.soliditylang.org/2020/12/16/solidity-0.8.0-release-announcement/)
- **OpenZeppelin SafeMath Documentation**: [SafeMath Library](https://docs.openzeppelin.com/contracts/3.x/api/math#SafeMath)
- **Integer Overflow/Underflow Vulnerabilities**: [Consensys Smart Contract Best Practices](https://consensys.github.io/smart-contract-best-practices/attacks/insecure-arithmetic/)

// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
pragma abicoder v2;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract H_uint64_totalFees_truncates_wraps_on_larg is Test {
    PuppyRaffle internal raffle;

    function setUp() public {
        // entranceFee = 1 ether, feeAddress arbitrary, raffleDuration = 1 second
        raffle = new PuppyRaffle(1 ether, address(0xBEEF), 1);
    }

    function testFeesTruncationLocksWithdrawal() public {
        // Prepare 100 unique player addresses
        uint256 numPlayers = 100;
        address[] memory newPlayers = new address[](numPlayers);
        for (uint256 i = 0; i < numPlayers; i++) {
            // Deterministically generate unique non-zero addresses
            newPlayers[i] = address(uint160(uint256(keccak256(abi.encodePacked(i + 1, address(this))))));
            require(newPlayers[i] != address(0), "generated zero addr");
        }

        // Fund this test contract to pay the entrance fees (100 ether)
        deal(address(this), 100 ether);

        // Enter raffle with 100 addresses paying exactly entranceFee * participants
        raffle.enterRaffle{value: 100 ether}(newPlayers);

        // Let the raffle duration elapse
        vm.warp(block.timestamp + 2);

        // Anyone can call selectWinner; this will:
        // - Compute fee = 20 ether
        // - Accumulate totalFees via uint64 cast (truncation/wrap)
        // - Send 80 ether to the winner
        raffle.selectWinner();

        // Contract should retain the full fee amount (20 ether)
        uint256 contractBal = address(raffle).balance;
        assertEq(contractBal, 20 ether, "Contract should hold full fee (20 ether)");

        // totalFees is a uint64 and will be truncated since 20 ether (2e19 wei) > 2^64-1 (~1.844e19 wei)
        uint256 recordedFees = uint256(raffle.totalFees());
        assertTrue(recordedFees != contractBal, "totalFees should not equal actual balance due to truncation");
        assertLt(recordedFees, contractBal, "Truncated uint64 totalFees should be less than the true fee balance");

        // Withdraw should revert because it requires balance == totalFees, which is now false
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}
