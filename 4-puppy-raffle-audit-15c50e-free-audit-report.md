# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol

Puppy Raffle is an on-chain game where users pay to enter a periodic raffle for a dog-themed ERC-721 NFT and a share of the pot.

• Entry  
  – Anyone can call `enterRaffle(address[] newPlayers)` sending `entranceFee` per address.  
  – Duplicate addresses are rejected, keeping the player set clean.  
  – Players can cancel their entry with `refund(uint256 index)` to get their stake back before the draw.

• Raffle cycle  
  – Each round lasts `raffleDuration` seconds from `raffleStartTime`.  
  – Once the timer expires and there are ≥4 live players, anyone can call `selectWinner()`.  
  – A pseudo-random index picks the winner who receives:  
    • 80 % of accumulated entrance fees.  
    • A freshly minted Puppy NFT whose rarity is randomly assigned and stored.

• Economics & Fees  
  – 20 % of every pot is retained as protocol fees and accumulates in `totalFees`.  
  – The owner can redirect fees with `changeFeeAddress()` and withdraw them via `withdrawFees()` once all rounds are settled.

• NFT Metadata  
  – `tokenURI()` composes on-chain JSON using `Base64`, mapping each rarity to trait names and image URIs.

Built with Solidity 0.7.6, OpenZeppelin ERC721, Ownable, and Foundry for testing.
## High Risk Findings
[H-1]. Randomness issue found with High severity
[H-2]. Integer Overflow issue found with High severity
[H-3]. DOS issue found with High severity
[H-4]. Reentrancy issue found with High severity
[H-5]. DOS issue found with High severity
[H-6]. Timestamp Dependent Logic issue found with High severity
[H-7]. Gas Grief BlockLimit issue found with High severity
[H-8]. DOS issue found with High severity
## Medium Risk Findings
[M-1]. Gas Grief BlockLimit issue found with Medium severity
[M-2]. DOS issue found with Medium severity
[M-3]. Unexpected Eth issue found with Medium severity
[M-4]. Integer Overflow/Math issue found with Medium severity
[M-5]. Array Limits issue found with Medium severity
## Low Risk Findings
[L-1]. Pausable Emergency Stop issue in PuppyRaffle::NA
[L-2]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees
[L-3]. Integer Overflow issue in PuppyRaffle::enterRaffle
[L-4]. Integer Overflow issue in PuppyRaffle::enterRaffle, selectWinner
## Info Risk Findings
[I-1]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 8
- M: 5
- L: 4
- I: 1



# Low Risk Findings

## [L-1]. Pausable Emergency Stop issue in PuppyRaffle::NA

## Description
The contract lacks an emergency stop or pause mechanism. If a critical vulnerability is discovered, the owner has no way to halt the contract's core functions, such as `enterRaffle` or `selectWinner`. This leaves the contract and its users' funds exposed until a fix can be deployed (which is not possible with this immutable contract).

## Impact
In the event of a critical bug, the inability to pause the contract can lead to continued exploitation and financial losses for users. The owner would be powerless to prevent further damage, harming the protocol's reputation and user trust.

## Proof of Concept
1. A critical vulnerability, such as the predictable randomness in `selectWinner`, is discovered and disclosed.
2. Malicious actors begin exploiting this vulnerability to unfairly win raffles.
3. Honest users, unaware of the exploit, continue to enter the raffle by sending ETH to the contract.
4. The contract owner, despite knowing about the vulnerability, has no function to call to pause new entries or stop the `selectWinner` function from being called, leading to further fund drainage.

## Proof of Code
// This is an architectural issue and does not have a PoC in code.
// The proof is the absence of a pause function.
// The following code demonstrates the fix, not the vulnerability.

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

contract FixedPuppyRaffle is Ownable, Pausable {
    function enterRaffle(address[] memory newPlayers) public payable whenNotPaused {
        // ... logic
    }

    function selectWinner() external whenNotPaused {
        // ... logic
    }

    // Owner can pause the contract in case of emergency
    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }
}

## Suggested Mitigation
Implement a pausable mechanism to allow the owner to halt critical functions in an emergency. This can be easily achieved by inheriting from OpenZeppelin's `Pausable` contract and applying the `whenNotPaused` modifier to critical functions like `enterRaffle` and `selectWinner`.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

contract PuppyRaffle is ERC721, Ownable, Pausable {
    // ...

    function enterRaffle(address[] memory newPlayers) public payable whenNotPaused {
        // ...
    }

    function selectWinner() external whenNotPaused {
        // ...
    }

    function refund(uint256 playerIndex) public whenNotPaused {
        // ...
    }

    // Add functions for owner to pause/unpause
    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }
}
```

## [L-2]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees

## Description
Several critical state-changing functions in the contract do not emit events. Specifically, `selectWinner` finalizes a raffle, transfers the prize, and mints an NFT without emitting a dedicated event summarizing the outcome. Similarly, `withdrawFees` transfers all collected fees to the owner without emitting an event.

## Impact
The absence of events for critical operations makes it difficult for off-chain services, monitoring tools, and users to track the contract's activity. This lack of transparency complicates auditing, debugging, and building a reliable user interface or backend service that depends on the contract's state changes.

## Proof of Concept
1. An external monitoring service is set up to track all raffle winners and prize amounts.
2. The service listens for events from the `PuppyRaffle` contract.
3. When `selectWinner` is successfully called, no specific `WinnerSelected` event is emitted. 
4. The service must resort to complex and less reliable methods, like parsing transaction data or correlating ERC721 `Transfer` events with contract state reads, to determine who won and how much they received.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract MissingEventTest is Test {
    PuppyRaffle raffle;
    address feeAddress = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(0.1 ether, feeAddress, 1); // entranceFee=0.1 eth, raffleDuration=1s

        // Prepare 4 unique players and fund this contract to pay their entry fees
        address[] memory addrs = new address[](4);
        addrs[0] = address(0x1);
        addrs[1] = address(0x2);
        addrs[2] = address(0x3);
        addrs[3] = address(0x4);

        vm.deal(address(this), 1 ether);
        raffle.enterRaffle{value: 0.4 ether}(addrs); // 4 * 0.1 ether

        // Fast-forward so the raffle is over
        vm.warp(block.timestamp + 2);
    }

    function testWinnerEventIsMissing() public {
        // Start recording logs that will be emitted during selectWinner
        vm.recordLogs();
        raffle.selectWinner();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        // Topic for the expected (but missing) event
        bytes32 expectedTopic = keccak256("WinnerSelected(address,uint256,uint256)");
        bool found;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics.length > 0 && logs[i].topics[0] == expectedTopic) {
                found = true;
                break;
            }
        }
        // The assertion passes only if the event is indeed missing
        assertTrue(!found, "WinnerSelected event unexpectedly present");
    }
} 

## Suggested Mitigation
Emit events for all critical state changes. This provides a transparent and reliable on-chain log of the contract's operations.

```solidity
// Add new events
event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId);
event FeesWithdrawn(address indexed to, uint256 amount);

function selectWinner() external {
    // ... existing logic ...
    
    // Before _safeMint and prize transfer
    emit WinnerSelected(winner, prizePool, tokenId);

    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");

    _safeMint(winner, tokenId);
}

function withdrawFees() external {
    // ... existing logic ...
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;

    emit FeesWithdrawn(feeAddress, feesToWithdraw);

    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [L-3]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
The contract uses Solidity version 0.7.6, which does not have built-in protection against integer overflows and underflows. Several arithmetic operations are performed without using a safe math library.
1. In `enterRaffle`, `entranceFee * newPlayers.length` can overflow if `entranceFee` and `newPlayers.length` are large enough. This would wrap the result, allowing an attacker to enter many players for a very small `msg.value`.
2. In `selectWinner`, `players.length * entranceFee`, `totalAmountCollected * 80`, and `totalAmountCollected * 20` can all overflow.
3. Also in `selectWinner`, the raffle fee is cast from `uint256` to `uint64` before being added to `totalFees`. If the calculated fee exceeds the maximum value of a `uint64` (approx. 18.4 ether), it will be truncated, leading to incorrect fee accounting and a loss of funds for the fee recipient.

## Impact
The only practical arithmetic issue is the truncation that happens when `fee` is cast to `uint64` before being added to `totalFees`. Once the per-raffle fee exceeds 18.446 ether ( 2**64-1 wei ), the amount stored in `totalFees` wraps around modulo 2**64, leading to permanent loss of fees for the protocol owner. Players’ funds and the prize pool are not affected.

## Proof of Concept
1. Deploy `PuppyRaffle` with an `entranceFee` of 1 ether and `raffleDuration` of 1 day.
2. Enter 100 different addresses (100 ether in total).
3. Wait for the raffle to end and call `selectWinner`.
4. The correct fee should be 20 ether, but only `(uint64)(20 ether)` (= 0.001553… ether) is added to `totalFees`, proving the truncation/loss.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract FeeTruncationTest is Test {
    function testFeeTruncation() public {
        // Deploy raffle with 1 ether entrance fee
        PuppyRaffle raffle = new PuppyRaffle(1 ether, makeAddr("fee"), 1 days);

        // Prepare 100 unique players
        address[] memory addrs = new address[](100);
        for (uint256 i; i < 100; ++i) {
            addrs[i] = vm.addr(i + 1);
        }

        // Pay 100 ether to enter 100 players
        vm.deal(address(this), 120 ether);
        raffle.enterRaffle{value: 100 ether}(addrs);

        // Fast-forward so the raffle is over
        vm.warp(block.timestamp + 1 days + 1);

        // Select the winner (fee = 20 ether > 2**64-1 wei)
        raffle.selectWinner();

        uint256 expectedFee = 20 ether;
        uint256 storedFee   = raffle.totalFees();
        uint256 truncated   = uint256(uint64(expectedFee));

        // The fee recorded in contract must equal the truncated value and be < expectedFee
        assertEq(storedFee, truncated, "fee truncated");
        assertLt(storedFee, expectedFee, "loss of owner fees");
    }
}

## Suggested Mitigation
Change `totalFees` to `uint256` and remove the narrowing cast:

```solidity
// before
uint64  public totalFees;
...
uint64 feeU64 = uint64(fee);
totalFees += feeU64;

// after
uint256 public totalFees;
...
totalFees += fee; // no cast
```

If the project decides to stay on Solidity 0.7, consider importing OpenZeppelin’s `SafeMath` for all arithmetic; otherwise upgrade to ≥0.8.0 so that overflow/underflow checks are performed automatically.

## [L-4]. Integer Overflow issue in PuppyRaffle::enterRaffle, selectWinner

## Description
The contract uses Solidity version 0.7.6, which does not provide default protection against integer overflows or underflows. Multiple arithmetic operations are unsafe:
1. `enterRaffle`: `entranceFee * newPlayers.length` can overflow, allowing entry for a negligible cost.
2. `selectWinner`: `players.length * entranceFee` can overflow, causing incorrect prize calculations and locking the majority of funds in the contract.
3. `selectWinner`: `totalFees` is a `uint64` and is incremented by `uint64(fee)`. This can both truncate a large `fee` and cause `totalFees` to overflow, leading to a permanent DoS of the `withdrawFees` function.

## Impact
Because arithmetic is unchecked in Solidity 0.7.x, a malicious deployer (or an upgradeable implementation that later changes the entranceFee) can choose values that make multiplications overflow. This allows anyone to:
• enter the raffle without paying the required ether (entranceFee * n overflows to 0),
• mis-calculate prize / fee amounts, and
• truncate `totalFees` (uint64) once the total collected fees exceed 2^64-1.
The bug only becomes exploitable when the deployer selects an entranceFee that is deliberately close to 2^256-1 or when an implementation mistake later changes the fee to such a value. Therefore the issue is real but requires an unlikely mis-configuration.

## Proof of Concept
1. Deploy the contract with a huge entrance fee:
   entranceFee = (type(uint256).max / 2) + 1  
2. Prepare two player addresses `[A, B]`.
3. Call `enterRaffle{value:0}([A,B])`.  
   Calculation: entranceFee * 2 overflows to 0, so the `require` check passes, letting the attacker register the two players for free.
4. The same technique works for any `newPlayers.length` that makes the product overflow to the desired payable amount (including 0).

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract PuppyRaffleOverflowTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        uint256 hugeFee = type(uint256).max / 2 + 1; // will overflow when multiplied by 2
        raffle = new PuppyRaffle(hugeFee, address(this), 1 hours);
    }

    function test_freeEntryViaOverflow() public {
        address p1 = address(0x1);
        address p2 = address(0x2);
        address[] memory players = new address[](2);
        players[0] = p1;
        players[1] = p2;

        // call as an arbitrary user with 0 ether
        vm.prank(address(0xBEEF));
        raffle.enterRaffle{value: 0}(players);

        // verify that p1 is now registered without paying entranceFee
        uint256 idx = raffle.getActivePlayerIndex(p1);
        assertEq(idx, 0, "player should have been added for free");
    }
}

## Suggested Mitigation
Migrate to Solidity ^0.8.0 so that all arithmetic is checked by default, or import OpenZeppelin SafeMath for every arithmetic operation. In addition, change `totalFees` to `uint256` to avoid type-down-casting.



# Info Risk Findings

## [I-1]. Pragma issue in PuppyRaffle::NA

## Description
The contract is built using `pragma solidity 0.7.6;`, which is an outdated version of the Solidity compiler. Older versions may contain known bugs and vulnerabilities that have been patched in more recent releases. Furthermore, this version predates Solidity 0.8.0, which introduced crucial safety features like default overflow and underflow checks on arithmetic operations.

## Impact
Using an outdated compiler version increases the risk of the contract being susceptible to known compiler bugs. It also misses out on significant security enhancements, gas optimizations, and improved language features, making the contract less secure and efficient than it could be.

## Proof of Concept
N/A. This is a general best-practice finding. The risk is latent and depends on bugs present in the specific compiler version used. For example, the `IntegerOverflow` finding in this report is made more severe by the lack of default revert-on-overflow behavior in versions prior to 0.8.0.

## Proof of Code
// The vulnerability is the pragma line itself.

// In PuppyRaffle.sol:
// pragma solidity 0.7.6;

// This should be updated to a modern, stable version.

## Suggested Mitigation
It is strongly recommended to update the Solidity pragma to a more recent and stable version, such as `0.8.20` or higher. This will provide access to important security features like built-in overflow/underflow checks, as well as other compiler improvements and bug fixes.

```solidity
// Recommended change in PuppyRaffle.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
```



