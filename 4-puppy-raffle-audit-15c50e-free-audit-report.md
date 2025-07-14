# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### 🐾 PuppyRaffle Protocol
PuppyRaffle is an on-chain raffle that mints a dog-themed ERC-721 NFT to each round’s winner while routing a configurable fee to the project treasury.

1. Initialization
   * Deployed with an immutable entry fee, a fee-collector address, and the duration for each raffle round.
   * Inherits ERC721 for NFT logic and Ownable for admin control.

2. Entering the Raffle
   * Anyone can call `enterRaffle` and pay `entranceFee` per address submitted. All paid addresses are appended to `players`.
   * The contract tracks each participant’s index for later look-ups and optional refunds.

3. Optional Refunds
   * Before a winner is selected, a participant may call `refund` to exit, receiving their stake back and freeing their slot.

4. Selecting a Winner
   * After `raffleDuration` has elapsed, anyone can invoke `selectWinner`.
   * A pseudo-random index (blockhash & timestamp) chooses the winner.
   * 90 % of the pot is transferred to the winner; 10 % accrues to `feeAddress` and can be withdrawn by the owner.
   * A new NFT with rarity-based metadata is minted to the winning address.

5. Admin Functions
   * Owner can change the fee address and withdraw accumulated fees.

The design keeps state simple, uses minimal external calls, and delivers provably fair, permissionless raffles coupled with collectible NFTs.
## High Risk Findings
[H-1]. Timestamp Dependent Logic issue found with High severity
[H-2]. Reentrancy issue found with High severity
[H-3]. Randomness issue found with High severity
[H-4]. DOS issue found with High severity
[H-5]. Unexpected Eth issue found with High severity
## Medium Risk Findings
[M-1]. Gas Grief BlockLimit issue found with Medium severity
[M-2]. Integer Overflow issue found with Medium severity
## Info Risk Findings
[I-1]. Pausable Emergency Stop issue in PuppyRaffle::NA
[I-2]. Pragma issue in PuppyRaffle::NA
[I-3]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees


### Number of Findings
- H: 5
- M: 2
- L: 0
- I: 3



# Info Risk Findings

## [I-1]. Pausable Emergency Stop issue in PuppyRaffle::NA

## Description
The contract handles user funds and has a defined lifecycle, but it lacks an emergency stop or pause mechanism. If a critical vulnerability is discovered post-deployment, the owner has no way to halt the contract's operation. Malicious actors could continue to exploit the vulnerability, or legitimate users could continue to deposit funds into a contract known to be unsafe.

## Impact
Because the contract cannot be paused, the owner has no on-chain tool to mitigate unforeseen vulnerabilities discovered after deployment. Although this does not in itself lead to loss of funds, it removes an important safety circuit that could reduce the blast-radius of future bugs.

## Proof of Concept
1. A critical flaw (e.g., the weak randomness vulnerability) is discovered and publicly disclosed.
2. The contract owner is notified but has no function to call to pause the contract.
3. An attacker proceeds to exploit the flaw to steal the prize pool.
4. Meanwhile, users who are unaware of the vulnerability may continue to call `enterRaffle`, adding more funds for the attacker to steal.

## Proof of Code
```solidity
// This is a conceptual proof, as it demonstrates a missing feature.
// No test can be written to exploit the *absence* of a function.
// The scenario is as described in the proof_of_concept.
```

## Suggested Mitigation
Inherit from OpenZeppelin's `Pausable` contract and apply the `whenNotPaused` modifier to all functions that perform state changes or handle fund transfers. This gives the owner the ability to halt the contract in an emergency.

```diff
+ import {Pausable} from "@openzeppelin/contracts/security/Pausable.sol";

- contract PuppyRaffle is ERC721, Ownable {
+ contract PuppyRaffle is ERC721, Ownable, Pausable {

-   function enterRaffle(address[] memory newPlayers) public payable {
+   function enterRaffle(address[] memory newPlayers) public payable whenNotPaused {
        // ...
    }

-   function refund(uint256 playerIndex) public {
+   function refund(uint256 playerIndex) public whenNotPaused {
        // ...
    }

-   function selectWinner() external {
+   function selectWinner() external whenNotPaused {
        // ...
    }

    // Add pause/unpause functions callable only by owner
    function pause() external onlyOwner {
        _pause();
    }

    function unpause() external onlyOwner {
        _unpause();
    }
}
```

## [I-2]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma version (`pragma solidity ^0.8.7;`). This allows the contract to be compiled with any compiler version from 0.8.7 up to (but not including) 0.9.0. Using a floating pragma is risky because it can lead to deployment with a compiler version that has not been tested, potentially introducing bugs or unexpected behavior from newer compiler versions.

## Impact
This practice can lead to deploying a contract with unintended behavior due to compiler changes or bugs introduced in newer patch versions. It reduces the determinism and reproducibility of the build, which is a security risk.

## Proof of Concept
1. The contract is developed and thoroughly tested using compiler version `0.8.7`.
2. Some time later, a new compiler version, `0.8.15`, is released which contains a subtle, unknown code generation bug.
3. A user deploys the `PuppyRaffle` contract using a development environment that defaults to the latest `0.8.x` compiler.
4. The contract is compiled and deployed with `0.8.15`, and the unknown compiler bug is now part of the on-chain bytecode, potentially creating a vulnerability.

## Proof of Code
```solidity
// The vulnerability is in the pragma statement itself.
// No test case can exploit this directly, it's a best practice violation.

// In PuppyRaffle.sol:
// pragma solidity ^0.8.7;
```

## Suggested Mitigation
Use a fixed pragma version to ensure the contract is always compiled with the exact compiler version it was developed and audited for. This improves security and ensures deterministic builds.

```diff
- pragma solidity ^0.8.7;
+ pragma solidity 0.8.20; // Or other specific, audited version.
```

## [I-3]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees

## Description
The contract's most critical functions, `selectWinner` and `withdrawFees`, execute significant state changes and value transfers but do not emit corresponding events. `selectWinner` transfers the prize pool and mints the winning NFT, while `withdrawFees` transfers all collected fees to the owner. The absence of events makes it difficult for off-chain services, monitoring tools, and users to track these important activities efficiently.

## Impact
The lack of events reduces transparency and observability. It complicates the development of user interfaces, analytics dashboards, and security monitoring tools that rely on event logs to track a contract's state and history. Users and integrators must resort to more complex and less reliable methods like transaction tracing.

## Proof of Concept
1. A raffle concludes and `selectWinner` is called. A winner receives the prize pool.
2. A dApp frontend wants to display a list of all past winners and their prizes.
3. Without a `WinnerSelected` event, the frontend cannot simply query event logs. It must instead parse the transaction history of the contract, inspecting the inputs and internal state changes of every `selectWinner` call, which is inefficient and unreliable.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PuppyRaffle_NoEvent_Test is Test {
    PuppyRaffle raffle;

    function setUp() public {
        vm.deal(address(this), 1 ether);
        raffle = new PuppyRaffle(0.1 ether, address(0xdead), 1);
    }

    function testMissingWinnerSelectedEvent() public {
        // prepare 4 distinct players
        address[] memory players = new address[](4);
        for (uint256 i; i < 4; ++i) {
            players[i] = address(uint160(i + 1));
        }

        // enter raffle with the 4 players in a single call
        raffle.enterRaffle{value: 0.4 ether}(players);

        // fast-forward so raffle is over
        vm.warp(block.timestamp + 2);

        // record emitted logs during winner selection
        vm.recordLogs();
        raffle.selectWinner();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        // signature of the expected (but missing) event
        bytes32 WINNER_EVENT_SIG = keccak256("WinnerSelected(address,uint256,uint256)");

        // ensure that no WinnerSelected event was emitted
        for (uint256 i; i < logs.length; ++i) {
            assertTrue(logs[i].topics[0] != WINNER_EVENT_SIG, "WinnerSelected event should exist but is missing");
        }
    }
}


## Suggested Mitigation
Emit events for all critical state changes and actions. Add a `WinnerSelected` event in `selectWinner` and a `FeesWithdrawn` event in `withdrawFees`.

```diff
contract PuppyRaffle is ERC721, Ownable {
    // ...
+   event WinnerSelected(address indexed winner, uint256 prize, uint256 indexed tokenId);
+   event FeesWithdrawn(address indexed feeAddress, uint256 amount);

    function selectWinner() external {
        // ...
        _safeMint(winner, tokenId);
+       emit WinnerSelected(winner, prizePool, tokenId);
    }

    function withdrawFees() external {
        uint256 feesToWithdraw = totalFees;
        // ...
        require(success, "PuppyRaffle: Failed to withdraw fees");
+       emit FeesWithdrawn(feeAddress, feesToWithdraw);
    }
}
```



