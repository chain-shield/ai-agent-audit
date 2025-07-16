# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle Protocol

PuppyRaffle is an Ethereum-based raffle system that lets anyone compete for a dog-themed ERC-721 NFT. The owner sets an `entranceFee`, a `raffleDuration`, and a `feeAddress` that collects protocol fees.

1. Entering
   * Users call `enterRaffle(address[] calldata)` with one or more wallet addresses and pay `entranceFee` × number of addresses.
   * The contract checks that the raffle is still open, blocks duplicate entries, and records each new player in `players`.

2. Exiting
   * Before the raffle ends, a participant can call `refund()` to leave and get their fee back. Their slot in `players` is set to `address(0)` so indices remain stable.

3. Closing & Winner Selection
   * After `raffleDuration` has elapsed, anyone can trigger `selectWinner()`. A pseudo-random index derived from block data picks a non-empty entry. The winner receives a newly minted Puppy NFT.
   * Each token id is assigned a rarity tier that maps to a metadata URI and name.

4. Fees & Admin
   * The owner can withdraw accumulated `totalFees` or change `feeAddress`.
   * NFT metadata uses `_baseURI()` override plus rarity-specific extensions.

The contract leverages OpenZeppelin’s ERC721, Ownable, and SafeMath libraries for security and standard compliance.
## High Risk Findings
[H-1]. Randomness issue found with High severity
[H-2]. Reentrancy issue found with High severity
[H-3]. Integer Overflow/Math issue found with High severity
[H-4]. Unexpected Eth issue found with High severity
## Medium Risk Findings
[M-1]. Timestamp Dependent Logic issue found with Medium severity
[M-2]. DOS issue found with Medium severity
[M-3]. DOS issue found with Medium severity
[M-4]. Gas Grief BlockLimit issue found with Medium severity
[M-5]. Integer Overflow/Math issue found with Medium severity
[M-6]. Unexpected Eth issue found with Medium severity
[M-7]. DOS issue found with Medium severity
[M-8]. Integer Overflow issue found with Medium severity
[M-9]. Integer Overflow issue found with Medium severity
## Low Risk Findings
[L-1]. Pausable Emergency Stop issue in PuppyRaffle::NA
[L-2]. Integer Overflow issue in PuppyRaffle::enterRaffle
[L-3]. Access Control issue in PuppyRaffle::withdrawFees
[L-4]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees
## Info Risk Findings
[I-1]. Event Consistency issue in PuppyRaffle::selectWinner
[I-2]. Pragma issue in PuppyRaffle::NA
[I-3]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 4
- M: 9
- L: 4
- I: 3



# Low Risk Findings

## [L-1]. Pausable Emergency Stop issue in PuppyRaffle::NA

## Description
The contract lacks an emergency stop or pause mechanism. If a critical vulnerability (like the identified integer overflow or potential reentrancy) is discovered after deployment, the owner has no way to halt the contract's operations. Malicious actors could continue to interact with the vulnerable functions, potentially leading to further financial loss or contract malfunction. The only owner-restricted function is `changeFeeAddress`, which is insufficient for managing a crisis.

## Impact
Because the contract has no circuit–breaker, the owner cannot temporarily stop user-facing functions if an unforeseen bug or economic attack is found after deployment. While this absence does not by itself steal or lock funds, it removes an important operational safety-valve and can enlarge the damage window of any future bug.

## Proof of Concept
1. Deploy PuppyRaffle.
2. Owner tries to pause the contract using the conventional `pause()` interface many projects expose:
   `(bool success, ) = address(raffle).call(abi.encodeWithSignature("pause()"));`
   `success` is false because the selector does not exist.
3. Any externally owned account can still call `enterRaffle` or `selectWinner` without hindrance, proving there is no way to halt the contract in an emergency.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract LackOfPauseTest is Test {
    PuppyRaffle raffle;
    address feeAddress = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeAddress, 1); // 1-second raffle for test
    }

    function test_NoPauseFunction() public {
        // Owner (msg.sender) attempts to pause the contract
        (bool success, ) = address(raffle).call(abi.encodeWithSignature("pause()"));
        assertTrue(!success, "pause() unexpectedly exists");

        // Prove contract is still callable
        address[] memory addrs = new address[](1);
        addrs[0] = address(1);
        raffle.enterRaffle{value: 1 ether}(addrs); // should succeed because no pause mechanism
    }
}

## Suggested Mitigation
Implement a pausable mechanism using OpenZeppelin's `Pausable` contract. This will allow the owner to halt key functions (`enterRaffle`, `selectWinner`, `refund`) in an emergency.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/utils/Pausable.sol"; // Import Pausable

// Inherit from Pausable
contract PuppyRaffle is ERC721, Ownable, Pausable {
    // ...

    // Add pause and unpause functions callable only by owner
    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    // Add the `whenNotPaused` modifier to critical functions
    function enterRaffle(address[] memory newPlayers) public payable virtual whenNotPaused {
        // ...
    }

    function refund(uint256 playerIndex) public virtual whenNotPaused {
        // ...
    }

    function selectWinner() external virtual whenNotPaused {
        // ...
    }
}

```

## [L-2]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
In the `enterRaffle` function, the duplicate check loop `for (uint256 i = 0; i < players.length - 1; i++)` is vulnerable to an integer underflow. If the function is called when `players.length` is 0, the expression `players.length - 1` underflows to `type(uint256).max`. This causes the loop to run a virtually infinite number of times, consuming all transaction gas and causing it to revert. An attacker can trigger this by calling `enterRaffle` with an empty array of new players.

## Impact
Calling `enterRaffle` with an empty `newPlayers` array causes the duplicate-check loop bound `players.length - 1` to underflow when the raffle has no players. The call will consume all provided gas and revert. No state is modified and subsequent, well-formed calls with at least one player proceed normally, so the issue is limited to wasted gas for the caller and does **not** prevent others from using the raffle.

## Proof of Concept
1. The `PuppyRaffle` contract is newly deployed, so the `players` array is empty (`players.length == 0`).
2. An attacker calls `enterRaffle` with an empty `newPlayers` array `[]` and `msg.value` of 0.
3. The first loop for adding players is skipped.
4. The second loop for duplicate checks evaluates its condition: `i < players.length - 1` becomes `i < 0 - 1`.
5. The subtraction underflows, and the condition becomes `i < type(uint256).max`.
6. The transaction immediately runs out of gas and reverts.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffle_Overflow_Test is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address constant FEE_ADDRESS = address(0xdeadbeef);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, 1 days);
    }

    function testUnderflowCausesOOG() public {
        address[] memory empty;
        // Expect the call to revert for *any* reason (out-of-gas produces no data)
        vm.expectRevert();
        raffle.enterRaffle{gas: 100_000}(empty); // give bounded gas to avoid hanging the test runner
    }
}


## Suggested Mitigation
Add an early check that `newPlayers.length > 0` or wrap the duplicate loop with `if (players.length > 1)` to avoid evaluating `players.length - 1` when there are fewer than two players.

## [L-3]. Access Control issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function is responsible for transferring collected fees to the `feeAddress`. This function lacks any access control, such as an `onlyOwner` modifier. As a result, any external account can call this function at any time. While the check `require(address(this).balance == uint256(totalFees), ...)` prevents this call during an active raffle, it can be successfully called by anyone after a raffle has ended and the winner has been paid, at which point the contract balance will equal `totalFees`.

## Impact
Any externally owned account can front-run or arbitrarily trigger fee withdrawals once a raffle has finished. While this does not redirect or steal the funds (they are still sent to `feeAddress`), it removes the owner’s discretion over when withdrawals occur, breaking expected admin control and potentially complicating accounting or batching of multiple raffles.

## Proof of Concept
1. A raffle is run and completes successfully.
2. The winner calls `selectWinner`, and the prize pool is transferred to them.
3. The contract balance is now equal to the value of `totalFees`.
4. An arbitrary attacker calls `withdrawFees()`.
5. The call succeeds, and the fees are transferred to `feeAddress`, without the owner's initiation.

## Proof of Code
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract AccessControlTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = makeAddr("fee");
    uint256 raffleDuration = 1 days;

    address player1 = makeAddr("player1");
    address player2 = makeAddr("player2");
    address player3 = makeAddr("player3");
    address player4 = makeAddr("player4");
    address attacker = makeAddr("attacker");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
        vm.deal(address(this), 10 ether);

        address[] memory players = new address[](4);
        players[0] = player1; players[1] = player2; players[2] = player3; players[3] = player4;
        puppyRaffle.enterRaffle{value: 4 * entranceFee}(players);
    }

    function test_AnyoneCanWithdrawFees() public {
        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();

        uint256 fees = puppyRaffle.totalFees();
        assert(fees > 0);
        assertEq(address(puppyRaffle).balance, fees);

        uint256 feeAddressBalanceBefore = feeAddress.balance;

        // Attacker calls withdrawFees
        vm.prank(attacker);
        puppyRaffle.withdrawFees();

        assertEq(feeAddress.balance, feeAddressBalanceBefore + fees);
        assertEq(address(puppyRaffle).balance, 0);
        assertEq(puppyRaffle.totalFees(), 0);
    }
}

## Suggested Mitigation
Add the `onlyOwner` modifier to the `withdrawFees` function to ensure that only the contract owner can initiate the fee withdrawal process.

```solidity
// src/PuppyRaffle.sol:156-165
function withdrawFees() external onlyOwner {
    // ... function body ...
}
```

## [L-4]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees

## Description
Several critical functions that alter state and move funds do not emit events. Specifically, `selectWinner` determines a winner, transfers the prize pool, and resets the raffle, but no event is emitted to log this outcome. Similarly, `withdrawFees` allows the owner to transfer all collected fees, but this action is not logged on-chain with an event. This lack of event emission reduces transparency and makes it difficult for users and off-chain monitoring tools to track the contract's lifecycle and key activities.

## Impact
The absence of events for significant actions makes the contract opaque. It becomes hard for users to verify raffle outcomes, for front-ends to display activity history, and for security tools to monitor for malicious or anomalous behavior. This erodes trust and complicates any potential incident analysis.

## Proof of Concept
1. Call the `selectWinner` function.
2. Observe the transaction logs on a block explorer like Etherscan.
3. Note that while `Transfer` events for the NFT minting are present (from ERC721), there is no custom event like `WinnerSelected` that indicates who won, the prize amount, or the new raffle starting.
4. Call `withdrawFees`.
5. Observe the transaction logs and note the absence of an event like `FeesWithdrawn`.

## Proof of Code
pragma solidity ^0.8.13;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract NoEventTest is Test {
    PuppyRaffle raffle;
    address owner = address(0xBEEF);
    address payable feeAddr = payable(address(0xFEE));
    address player1 = address(1);
    address player2 = address(2);
    address player3 = address(3);
    address player4 = address(4);

    function setUp() public {
        vm.deal(owner, 100 ether);
        vm.startPrank(owner);
        raffle = new PuppyRaffle(1 ether, feeAddr, 1); // raffleDuration = 1 second
        vm.stopPrank();

        vm.deal(player1, 10 ether);
        vm.deal(player2, 10 ether);
        vm.deal(player3, 10 ether);
        vm.deal(player4, 10 ether);
    }

    function _enter(address p) internal {
        address[] memory arr = new address[](1);
        arr[0] = p;
        vm.prank(p);
        raffle.enterRaffle{value: 1 ether}(arr);
    }

    function test_noWinnerSelectedEvent() public {
        _enter(player1);
        _enter(player2);
        _enter(player3);
        _enter(player4);

        // Fast-forward so raffle is over
        vm.warp(block.timestamp + 2);

        vm.recordLogs();
        raffle.selectWinner();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        bytes32 expectedTopic = keccak256("WinnerSelected(address,uint256,uint256)");
        bool found;
        for (uint256 i; i < logs.length; i++) {
            if (logs[i].topics.length > 0 && logs[i].topics[0] == expectedTopic) {
                found = true;
            }
        }
        assertFalse(found, "WinnerSelected event should not exist");
    }
}

## Suggested Mitigation
Define and emit events for all critical state-changing functions.

```solidity
// Add event declarations
event WinnerSelected(address indexed winner, uint256 prize, uint256 tokenId);
event FeesWithdrawn(address indexed to, uint256 amount);

// In selectWinner()
// ... after prize calculation and before the external call ...
_safeMint(winner, tokenId);
emit WinnerSelected(winner, prizePool, tokenId);

// In withdrawFees()
function withdrawFees() external {
    // ... require check ...
    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
    emit FeesWithdrawn(feeAddress, feesToWithdraw);
}
```



# Info Risk Findings

## [I-1]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
Several critical state-changing actions in the contract do not emit events. Specifically, `selectWinner` executes the entire winner selection, prize distribution, and NFT minting process without emitting any event. Similarly, `withdrawFees` transfers all collected fees to the `feeAddress` without an event. This lack of event emission makes it difficult for off-chain services, monitoring tools, and users to track the contract's most important activities. It harms transparency and observability.

## Impact
While this issue does not lead to a direct loss of funds, it severely hinders the usability and trustworthiness of the protocol. DApp front-ends, analytics platforms, and users cannot easily track raffle outcomes or fee withdrawals. They would need to resort to complex and unreliable methods like tracing transactions, which is inefficient and not standard practice.

## Proof of Concept
1. A user participates in the raffle.
2. The raffle period ends, and another user calls `selectWinner`.
3. The first user wants to know if they won. They check their wallet for the prize and the NFT, but what if they want to see the history of all winners?
4. There is no `WinnerSelected` event to subscribe to. The only way to find the winner is to call the `previousWinner` view function, which only shows the most recent winner, or to parse the internal transactions of the `selectWinner` call.
5. This makes building a transparent 'Past Winners' list on a website difficult and inefficient.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract EventConsistencyTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 60;
    address feeAddress;
    address owner;

    function setUp() public {
        owner = msg.sender;
        feeAddress = makeAddr("feeAddress");
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, RAFFLE_DURATION);

        address[] memory players = new address[](4);
        for(uint i = 0; i < 4; i++) {
            players[i] = makeAddr(string(abi.encodePacked("player", vm.toString(i))));
        }
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
    }

    function test_SelectWinner_LacksEvent() public {
        // We expect no custom events from `selectWinner` other than ERC721 `Transfer`.
        // We will check the log count to demonstrate this.
        vm.recordLogs();
        puppyRaffle.selectWinner();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        // The only event emitted is the ERC721 Transfer event.
        // A `WinnerSelected` event is missing.
        assertEq(logs.length, 1); // Only Transfer(address(0), winner, tokenId)
        bytes32 transferTopic = keccak256("Transfer(address,address,uint256)");
        assertEq(logs[0].topics[0], transferTopic);
    }

    function test_WithdrawFees_LacksEvent() public {
        // Setup: run a raffle to generate fees
        puppyRaffle.selectWinner();

        // We expect no events from `withdrawFees`.
        vm.prank(owner);
        vm.recordLogs();
        puppyRaffle.withdrawFees();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        // No event is emitted.
        assertEq(logs.length, 0);
    }
}
```

## Suggested Mitigation
Add events for all critical state changes. This improves transparency and allows for easier integration with off-chain services.

```solidity
// In PuppyRaffle.sol

// Define new events
event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId);
event FeesWithdrawn(address indexed to, uint256 amount);

function selectWinner() external {
    // ... existing logic up to prize calculation ...
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    // ...

    // Emit the event before external calls
    emit WinnerSelected(winner, prizePool, tokenId);

    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    _safeMint(winner, tokenId);
}

function withdrawFees() external {
    // ... existing logic ...
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;

    // Emit event before external call
    emit FeesWithdrawn(feeAddress, feesToWithdraw);

    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [I-2]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma `pragma solidity ^0.7.6;`. This allows the contract to be compiled with any compiler version from 0.7.6 up to, but not including, 0.8.0. Deploying a contract with a different compiler version than the one it was tested with can introduce unexpected behavior or bugs, as compiler-generated bytecode may differ. It also harms deterministic build processes.

## Impact
The use of a floating pragma can lead to the contract being deployed with a slightly different and untested compiler version, which may introduce subtle bugs. This reduces the overall security assurance of the deployed code.

## Proof of Concept
1. A developer writes and tests the contract using Solidity compiler version 0.7.6.
2. A deployment script or a third-party platform uses a newer compiler, say 0.7.9, to compile the contract before deployment.
3. If version 0.7.9 has a bug or a change in how it generates bytecode for certain opcodes, the deployed contract may not behave exactly as the tested contract did.
4. This discrepancy could lead to security vulnerabilities or operational failures.

## Proof of Code
NA

## Suggested Mitigation
Lock the pragma to a specific Solidity version that the contract has been developed and tested with. This ensures that the contract is always compiled with the intended compiler, producing deterministic bytecode.

```solidity
//- pragma solidity ^0.7.6;
//+ pragma solidity 0.7.6;
```

## [I-3]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses `pragma solidity 0.7.6;`. While using a fixed pragma is good practice, version 0.7.6 is outdated. Since its release, numerous bugs have been fixed in the Solidity compiler, and important security features, such as built-in overflow/underflow protection (introduced in 0.8.0), have been added. Using an old compiler version may expose the contract to known vulnerabilities that have since been patched.

## Impact
Compiling with an old version of Solidity may introduce subtle bugs or security vulnerabilities that have been fixed in newer versions. It also prevents the use of modern language features that improve code safety and clarity. In this specific contract, the lack of default overflow checks in <0.8.0 is directly related to the `IntegerMath` vulnerability found.

## Proof of Concept
1. Review the `PuppyRaffle.sol` file.
2. Observe the line `pragma solidity 0.7.6;`.
3. Compare this version to the latest stable release of Solidity (e.g., 0.8.2x).
4. Note the significant number of bug fixes and security improvements listed in the Solidity release notes between 0.7.6 and the latest version.

## Proof of Code
```solidity
// No code needed. The proof is the pragma line in the contract itself.
// File: src/PuppyRaffle.sol
// pragma solidity 0.7.6;
```

## Suggested Mitigation
It is recommended to use a more recent and stable version of the Solidity compiler. Upgrading to a version >= 0.8.0 is highly encouraged as it provides automatic overflow and underflow checks.

```diff
- pragma solidity 0.7.6;
+ pragma solidity ^0.8.20;
```
Note: Upgrading to 0.8.x is a breaking change and will require code modifications to be compliant with the new version (e.g., explicit type conversions, changes in `super` calls), but it significantly enhances security.



