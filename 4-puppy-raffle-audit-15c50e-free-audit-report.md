# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol

Puppy Raffle is an on-chain raffle that rewards one participant with a freshly minted dog-themed NFT while routing a configurable fee to the protocol owner.  

1. **Enter** – Players call `enterRaffle`, sending `entranceFee` ETH for each address supplied.  Duplicates are rejected, keeping the `players` array unique.
2. **Refund** – Before a draw, any entrant can call `refund` with their index to leave the game and reclaim their ETH.  
3. **Raffle Cycle** – Each round lasts `raffleDuration` seconds from `raffleStartTime`. When time elapses, anyone may call `selectWinner`.
4. **Winner Selection & Payouts** – `selectWinner` uses on-chain pseudo-randomness to pick a player, mints a new NFT whose metadata (URI, name, rarity) is derived from internal mappings, and transfers the pooled ETH minus fees to the winner. Accrued fees are added to `totalFees`.
5. **Fee Management** – The owner can `withdrawFees` and update `feeAddress` via `changeFeeAddress`.

The contract targets Solidity 0.7.6, is self-contained, and keeps all critical data on chain, delivering a transparent, trust-minimised NFT raffle experience.
## High Risk Findings
[H-1]. Reentrancy issue found with High severity
[H-2]. Randomness issue found with High severity
[H-3]. Integer Overflow issue found with High severity
[H-4]. DOS issue found with High severity
[H-5]. Integer Overflow issue found with High severity
[H-6]. Integer Overflow issue found with High severity
[H-7]. Timestamp Dependent Logic issue found with High severity
[H-8]. DOS issue found with High severity
[H-9]. Integer Overflow issue found with High severity
## Medium Risk Findings
[M-1]. DOS issue found with Medium severity
[M-2]. Unexpected Eth issue found with Medium severity
[M-3]. Frontrun/Backrun/Sandwhich MEV issue found with Medium severity
[M-4]. Gas Grief BlockLimit issue found with Medium severity
[M-5]. Randomness issue found with Medium severity
[M-6]. Gas Grief BlockLimit issue found with Medium severity
[M-7]. DOS issue found with Medium severity
[M-8]. Gas Grief BlockLimit issue found with Medium severity
[M-9]. DOS issue found with Medium severity
## Low Risk Findings
[L-1]. Access Control issue in PuppyRaffle::withdrawFees
[L-2]. Access Control issue in PuppyRaffle::constructor
[L-3]. Access Control issue in PuppyRaffle::changeFeeAddress
## Info Risk Findings
[I-1]. Pausable Emergency Stop issue in PuppyRaffle::NA
[I-2]. Event Consistency issue in PuppyRaffle::selectWinner(), withdrawFees()
[I-3]. Pragma issue in PuppyRaffle::NA
[I-4]. Event Consistency issue in PuppyRaffle::selectWinner
[I-5]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees
[I-6]. Pragma issue in PuppyRaffle::NA
[I-7]. Default Visibility issue in PuppyRaffle::NA
[I-8]. DOS issue in PuppyRaffle::selectWinner


### Number of Findings
- H: 9
- M: 9
- L: 3
- I: 8



# Low Risk Findings

## [L-1]. Access Control issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function, which allows for the withdrawal of accumulated fees, lacks an `onlyOwner` or similar access control modifier. Although the function has a check `require(address(this).balance == uint256(totalFees), ...)` that indirectly prevents it from being called while a raffle is active, anyone can trigger the withdrawal once the condition is met (i.e., between raffles). While the funds are sent to the correct `feeAddress`, allowing a public-facing withdrawal function for administrative tasks is against the principle of least privilege and can be abused for griefing.

## Impact
There is no direct loss of funds, as the fees are sent to the pre-configured `feeAddress`. However, this flaw allows anyone to trigger a core administrative function. An attacker could front-run the legitimate owner's transaction, causing nuisance and potentially forcing the owner to pay for a failed transaction. It represents a weakness in the contract's administrative controls.

## Proof of Concept
1. A raffle round ends, and the `players` array is cleared.
2. The contract now holds some amount in `totalFees`, and `address(this).balance` equals this amount.
3. A random user (not the owner) calls `withdrawFees()`.
4. The transaction succeeds because there is no ownership check.
5. The collected fees are sent to `feeAddress`, and `totalFees` is reset to 0.
6. The actual owner is pre-empted from performing this action.

## Proof of Code
pragma solidity 0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract AccessControlTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = makeAddr("fee");
    uint256 raffleDuration = 1 days;
    address attacker = makeAddr("attacker");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);

        address[] memory players = new address[](4);
        for(uint i=0; i<4; i++) {
            players[i] = makeAddr(string(abi.encodePacked("p", vm.toString(i))));
            vm.deal(players[i], entranceFee);
            vm.prank(players[i]);
            address[] memory p_arr = new address[](1); p_arr[0] = players[i];
            puppyRaffle.enterRaffle{value: entranceFee}(p_arr);
        }

        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();
        assertTrue(puppyRaffle.totalFees() > 0);
    }

    function testNonOwnerCanWithdrawFees() public {
        uint256 feesBefore = puppyRaffle.totalFees();
        uint256 feeAddressBalanceBefore = feeAddress.balance;

        vm.prank(attacker); // Attacker is not the owner
        puppyRaffle.withdrawFees();

        assertEq(puppyRaffle.totalFees(), 0, "Fees should be reset");
        assertEq(feeAddress.balance, feeAddressBalanceBefore + feesBefore, "feeAddress should receive fees");
    }
}

## Suggested Mitigation
Add the `onlyOwner` modifier to the `withdrawFees` function to restrict its execution to the contract owner. This is standard practice for administrative functions.

```solidity
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract PuppyRaffle is ERC721, Ownable {
    // ...

    function withdrawFees() external onlyOwner { // Add modifier
        require(
            players.length == 0, // Using improved check
            "PuppyRaffle: There are currently players active!"
        );
        uint256 feesToWithdraw = totalFees;
        require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
        totalFees = 0;

        (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
        require(success, "PuppyRaffle: Failed to withdraw fees");
    }
}
```

## [L-2]. Access Control issue in PuppyRaffle::constructor

## Description
The constructor does not check if the `_feeAddress` parameter is the zero address (`address(0)`). If the contract is deployed with `_feeAddress` set to `address(0)`, any fees collected by the protocol during `selectWinner` will be transferred to the zero address, where they will be permanently and irrecoverably lost.

## Impact
A simple deployment error can lead to a permanent loss of all protocol revenue. This is an unrecoverable mistake that renders the fee collection mechanism useless.

## Proof of Concept
1. Deploy the `PuppyRaffle` contract, passing `address(0)` for the `_feeAddress` parameter in the constructor.
2. Run a full raffle.
3. When `selectWinner` is called, it calculates the fee and stores it in `totalFees`.
4. Later, when `withdrawFees` is called (assuming its logic is fixed), it will send the `totalFees` amount to `address(0)`.
5. The funds are now burned and cannot be recovered.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressTest is Test {
    PuppyRaffle puppyRaffle;

    function testDeployWithZeroAddress() public {
        // This deployment should revert if properly protected.
        // With the current code, it succeeds.
        puppyRaffle = new PuppyRaffle(
            1 ether, 
            address(0), // feeAddress is the zero address
            60
        );
        assertEq(puppyRaffle.feeAddress(), address(0));
    }
}
```

## Suggested Mitigation
Add `require(newFeeAddress != address(0), "PuppyRaffle: fee address cannot be zero")` in both the constructor and `changeFeeAddress()` so the value can never be set to the zero address.

## [L-3]. Access Control issue in PuppyRaffle::changeFeeAddress

## Description
The `constructor` and `changeFeeAddress` function do not validate the address being set for `feeAddress`. An owner can accidentally or maliciously set the `feeAddress` to `address(0)`. If this happens, any fees withdrawn via the `withdrawFees` function will be sent to the zero address, where they will be permanently and irrecoverably burned.

## Impact
This vulnerability can lead to the permanent loss of all collected fees if the owner makes a mistake when setting the fee address. While it requires an action from the owner, the lack of a simple sanity check creates an unnecessary risk of fund loss.

## Proof of Concept
1. The owner of the contract calls `changeFeeAddress(address(0))`.
2. The transaction succeeds, and `feeAddress` is now the zero address.
3. A raffle completes, and fees are available for withdrawal.
4. Someone calls `withdrawFees()`.
5. The contract sends the accumulated fees to `address(0)`, burning them forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("fee");
    uint256 raffleDuration = 1 seconds;

    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
    }

    function testCanSetZeroFeeAddressAndBurnFees() public {
        // Owner sets fee address to address(0)
        vm.prank(owner);
        puppyRaffle.changeFeeAddress(address(0));
        assertEq(puppyRaffle.feeAddress(), address(0));

        // A raffle runs
        address[] memory players = new address[](4);
        for(uint i=0; i < 4; i++) {
            players[i] = address(uint160(i+1));
        }
        vm.deal(address(this), 4 * entranceFee);
        puppyRaffle.enterRaffle{value: 4 * entranceFee}(players);
        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();

        uint256 fees = uint256(puppyRaffle.totalFees());
        assertTrue(fees > 0);
        
        uint256 zeroAddressBalanceBefore = address(0).balance;
        // Withdraw fees
        puppyRaffle.withdrawFees();
        
        // Check that fees were sent to address(0)
        uint256 zeroAddressBalanceAfter = address(0).balance;
        assertEq(zeroAddressBalanceAfter - zeroAddressBalanceBefore, fees);
        assertEq(puppyRaffle.totalFees(), 0);
    }
}
```

## Suggested Mitigation
Add a `require` statement in both the `constructor` and the `changeFeeAddress` function to ensure the new fee address is not `address(0)`.

```solidity
// In PuppyRaffle.sol

constructor(
    uint256 _entranceFee,
    address _feeAddress,
    uint256 _raffleDuration
) ERC721("Puppy Raffle", "PR") {
    require(_feeAddress != address(0), "PuppyRaffle: Fee address cannot be zero");
    // ...
}

function changeFeeAddress(address newFeeAddress) external onlyOwner {
    require(newFeeAddress != address(0), "PuppyRaffle: Fee address cannot be zero");
    feeAddress = newFeeAddress;
    emit FeeAddressChanged(newFeeAddress);
}
```



# Info Risk Findings

## [I-1]. Pausable Emergency Stop issue in PuppyRaffle::NA

## Description
The contract handles user funds but lacks an emergency stop or pause mechanism. If a critical vulnerability is discovered (like the ones identified in this audit), the owner has no way to halt contract operations to prevent further exploitation or loss of funds. Attackers could continue to interact with the vulnerable functions until a fix is deployed and users migrate.

## Impact
Because the contract cannot be paused, the owner has no on-chain lever to temporarily stop user-facing functions if a separate critical bug is discovered off-chain. While this does not by itself create a new avenue for stealing or locking funds, it removes an important mitigation tool and can prolong the exploitation window of other vulnerabilities.

## Proof of Concept
1. A severe vulnerability is discovered in the `selectWinner` function that allows an attacker to steal the entire prize pool.
2. The owner is notified but cannot pause the contract.
3. Before a fixed contract can be deployed, the attacker repeatedly calls the vulnerable `selectWinner` function at the end of each raffle period, draining all funds collected.
4. Legitimate users lose all their entry fees.

## Proof of Code
```solidity
// This is a conceptual issue, so a provable test is not applicable.
// The PoC is the absence of a pausing mechanism.
```

## Suggested Mitigation
Implement a pausable mechanism using OpenZeppelin's `Pausable` contract. Apply the `whenNotPaused` modifier to all critical functions that perform state changes or handle funds, such as `enterRaffle`, `refund`, `selectWinner`, and `withdrawFees`.

```solidity
import "@openzeppelin/contracts/utils/Pausable.sol";

contract PuppyRaffle is ERC721, Ownable, Pausable {
    // ...

    function enterRaffle(address[] memory newPlayers) public payable whenNotPaused {
        // ...
    }

    function refund(uint256 playerIndex) public whenNotPaused {
        // ...
    }

    function selectWinner() external whenNotPaused {
        // ...
    }

    // Add pause and unpause functions callable by the owner
    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }
}
```

## [I-2]. Event Consistency issue in PuppyRaffle::selectWinner(), withdrawFees()

## Description
The contract performs several critical operations without emitting events. The `selectWinner` function changes significant state (winner, prize distribution, raffle reset) and `withdrawFees` moves all collected fees out of the contract. The absence of events for these actions makes it difficult for off-chain services, monitoring tools, and users to track the contract's activity and history reliably. They would need to rely on tracing transactions, which is less efficient and robust.

## Impact
Lack of events for critical operations reduces observability and makes it harder to build a reliable history of the contract's state. This complicates integration with dapps, dashboards, and monitoring systems, and makes auditing past activity more difficult for users.

## Proof of Concept
1. An off-chain dashboard wants to display a history of all raffle winners and the prize amounts.
2. The `selectWinner` function is called, a winner is chosen, and an NFT is minted. The NFT `Transfer` event is emitted, but it doesn't specify that it was from a raffle win, nor the ETH prize amount.
3. The dashboard cannot easily determine who won the prize pool or how much it was without parsing the internal transaction traces of the `selectWinner` call.
4. Similarly, when the owner calls `withdrawFees`, there is no on-chain log of this event, making it opaque to users tracking the protocol's fee revenue.

## Proof of Code
```solidity
// This is a best-practice issue, so a provable test is not applicable.
// The PoC is the absence of `emit` statements in the specified functions.
```

## Suggested Mitigation
Add events for all critical state-changing functions to improve observability.

```solidity
// In PuppyRaffle.sol

// Add new events
event WinnerSelected(address indexed winner, uint256 prizePool, uint256 tokenId);
event FeesWithdrawn(address indexed feeAddress, uint256 amount);

// In selectWinner()
function selectWinner() external {
    // ... after winner is selected and prize pool calculated
    _safeMint(winner, tokenId);
    emit WinnerSelected(winner, prizePool, tokenId);
}

// In withdrawFees()
function withdrawFees() external {
    // ...
    require(success, "PuppyRaffle: Failed to withdraw fees");
    emit FeesWithdrawn(feeAddress, feesToWithdraw);
}
```

## [I-3]. Pragma issue in PuppyRaffle::NA

## Description
The contract is built using `pragma solidity 0.7.6;`. This version is outdated and does not include significant security enhancements introduced in later versions, most notably the automatic overflow and underflow checks that became standard in Solidity 0.8.0. Relying on an old compiler version increases the risk of introducing vulnerabilities (like the integer overflow found in this contract) and missing out on gas optimizations and other language improvements.

## Impact
Using an outdated pragma makes the contract more susceptible to certain classes of bugs, such as integer overflows, which must be manually checked for. It also prevents the use of newer, more efficient language features and could expose the contract to known compiler bugs from that version.

## Proof of Concept
The `IntegerOverflow` vulnerability identified in this audit (downcasting `fee` to `uint64`) would have been prevented at compile time or would have caused a revert at runtime if the contract had been compiled with Solidity 0.8.0 or newer. The use of `pragma 0.7.6` allowed this bug to exist.

## Proof of Code
```solidity
// This is a configuration/best-practice issue, so a provable test is not applicable.
// The PoC is the pragma line itself in the contract.
// pragma solidity 0.7.6;
```

## Suggested Mitigation
Upgrade the Solidity pragma to a recent, stable version, for example, `^0.8.20`. After upgrading, review the code for any necessary changes. For instance, `SafeMath` is no longer needed for basic arithmetic, as overflow/underflow checks are built-in.

```solidity
// pragma solidity 0.7.6;
pragma solidity ^0.8.20;

// Then, remove any SafeMath library usage if it were present, and
// ensure all arithmetic operations are safe under the new compiler rules.
```

## [I-4]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function executes several critical state changes: it determines a winner, resets the `players` array for the next round, updates `previousWinner`, and sets a new `raffleStartTime`. While the `_safeMint` call emits a standard `Transfer` event for the NFT, the contract does not emit a dedicated event for the raffle conclusion itself. This makes it difficult for off-chain services, frontends, and users to track raffle history, winners, and prize amounts efficiently.

## Impact
The lack of events for critical state changes reduces transparency and makes the contract harder to integrate with external tools. DApp frontends cannot easily display a history of winners, and monitoring services cannot effectively track the protocol's health and activity. Users have to rely on inspecting internal contract state, which is less user-friendly and efficient.

## Proof of Concept
1. A user wants to build a dashboard showing the history of all PuppyRaffle winners and the prize amounts they won.
2. They subscribe to contract events to get this data.
3. They will receive `RaffleEnter` and `RaffleRefunded` events, and ERC721 `Transfer` events.
4. However, there is no `WinnerSelected` event, so they cannot easily correlate the NFT transfer to a specific raffle round or determine the prize amount won. They would have to resort to complex and less reliable methods like tracing transactions that call `selectWinner`.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract WinnerEventAbsentTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 60;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(0xdead), RAFFLE_DURATION);

        // create 4 unique entrant addresses
        address[] memory entrants = new address[](4);
        for (uint256 i; i < 4; i++) {
            entrants[i] = address(uint160(i + 1));
        }

        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(entrants);
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
    }

    function test_noWinnerSelectedEvent() public {
        // keccak256 hash of the missing event signature we expect not to see
        bytes32 WINNER_SELECTED_SIG = keccak256("WinnerSelected(address,uint256,uint256,uint256)");

        vm.recordLogs();
        raffle.selectWinner();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        for (uint256 i; i < logs.length; i++) {
            if (logs[i].topics.length > 0) {
                // first topic is the event signature hash
                assertTrue(logs[i].topics[0] != WINNER_SELECTED_SIG, "WinnerSelected event should not exist");
            }
        }
    }
}

## Suggested Mitigation
Define and emit a dedicated event in the `selectWinner` function that captures all relevant information about the concluded raffle round.

```diff
contract PuppyRaffle is ERC721, Ownable {
    // ...

+   event WinnerSelected(
+       address indexed winner,
+       uint256 indexed tokenId,
+       uint256 prizePool,
+       uint256 raffleNumber // Optional: add a raffle counter
+   );

    function selectWinner() external {
        // ... (winner selection logic)
        address winner = players[winnerIndex];
        uint256 prizePool = (totalAmountCollected * 80) / 100;
        uint256 tokenId = totalSupply();

        // ...

        _safeMint(winner, tokenId);

+       emit WinnerSelected(winner, tokenId, prizePool);
    }
}
```

## [I-5]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees

## Description
Several functions that execute critical state transitions do not emit events. This lack of event emission reduces the contract's observability and makes it difficult for off-chain services, user interfaces, and monitoring tools to accurately track the contract's lifecycle and state. The missing events include winner selection, raffle restarts, and fee withdrawals.

## Impact
While not a direct security vulnerability that can be exploited for financial gain, poor event logging severely hampers transparency, monitoring, and debugging. It forces users and developers to rely on expensive state-reading calls or complex transaction tracing to understand what happened in the contract, which is inefficient and error-prone.

## Proof of Concept
1. A user participates in the raffle.
2. The `selectWinner` function is called, and a winner is chosen. The prize is sent, an NFT is minted (which does emit a `Transfer` event), the `players` array is cleared, and `raffleStartTime` is reset.
3. An off-chain application monitoring the raffle has no single, clear event like `WinnerSelected` to parse. It would need to infer the raffle's end by watching for a `Transfer` event to a player, which is an indirect and potentially ambiguous signal.

## Proof of Code
```solidity
// N/A. This is an observability issue and cannot be demonstrated as a reverting exploit. A test can only confirm the absence of an event log, which is clear from reading the code.
```

## Suggested Mitigation
Emit events for all significant state changes. This provides a clear, reliable, and cheap way for external parties to track the contract's operations.

```solidity
// Add new events
event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId);
event FeesWithdrawn(address indexed feeAddress, uint256 amount);

// In selectWinner function
function selectWinner() external {
    // ... logic to determine winner and prizePool ...
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");

    _safeMint(winner, tokenId);

    emit WinnerSelected(winner, prizePool, tokenId);

    // ... rest of the logic
}

// In withdrawFees function
function withdrawFees() external {
    // ...
    emit FeesWithdrawn(feeAddress, feesToWithdraw);
}
```

## [I-6]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma `pragma solidity ^0.7.6;`. This allows the contract to be compiled with any compiler version from `0.7.6` up to (but not including) `0.8.0`. This can lead to the contract being deployed with different, untested compiler versions, some of which might have bugs. It also reduces the determinism of deployments, making verification on block explorers more difficult.

## Impact
Using a floating pragma may lead to the contract being compiled and deployed with a compiler version that has known or unknown bugs, potentially introducing vulnerabilities. It is a deviation from security best practices.

## Proof of Concept
1. A developer compiles the contract with `solc` version `0.7.6` and tests it.
2. The deployment script or environment uses a different compiler, say `0.7.9`, which is allowed by the pragma.
3. An unknown bug or slight behavior change in the `0.7.9` compiler could introduce subtle issues into the deployed bytecode that were not present during testing.

## Proof of Code
NA

## Suggested Mitigation
Lock the pragma to a specific compiler version that is known to be stable and has been audited. This ensures that the contract bytecode is always the same for a given source code.
```diff
- pragma solidity ^0.7.6;
+ pragma solidity 0.7.6;
```

## [I-7]. Default Visibility issue in PuppyRaffle::NA

## Description
Several state variables in the contract are declared without an explicit visibility keyword (e.g., `public`, `private`, `internal`). For example, `uint256 entranceFee;` and `address[] players;`. While the Solidity compiler assigns a default visibility (`internal` for state variables since version 0.5.0), explicitly declaring visibility is a best practice that improves code clarity and maintainability. It prevents ambiguity about which variables are part of the contract's public API.

## Impact
This does not pose a direct security risk in this contract, but it reduces code quality and can lead to misunderstandings or errors during future upgrades or by developers integrating with the contract. For example, a developer might mistakenly assume `players` is readable externally.

## Proof of Concept
A developer reading the code `uint256 entranceFee;` might not immediately know if it's publicly accessible. They would have to know Solidity's default visibility rules. If it were `uint256 public entranceFee;`, the intent is clear, and a public getter is automatically created.

## Proof of Code
```solidity
// This is a code quality finding. No test case can 'exploit' it.
// The vulnerability is in the code itself.

contract PuppyRaffle {
    // Lacks explicit visibility. Is it public? private? internal?
    // The compiler defaults to internal.
    uint256 entranceFee; 
    address[] players;
    uint256 raffleDuration;
    uint256 raffleStartTime;
    // ... and others
}
```

## Suggested Mitigation
Add an explicit visibility specifier to every state variable to make the contract's storage and API clear and unambiguous.

```solidity
// Before
uint256 entranceFee;
address[] players;

// After (example)
// Make fee public so anyone can read it.
uint256 public entranceFee;
// Keep players private as it's managed internally.
address[] private players;
// Make raffle duration public for transparency.
uint256 public raffleDuration;
```

## [I-8]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function calls `delete players` to clear the array of participants for the next round. For a dynamic array, this operation's gas cost is proportional to the number of elements. If the raffle has a very large number of players, the gas required to delete the array can exceed the block gas limit. This would cause any call to `selectWinner` to fail, effectively creating a permanent Denial of Service where no winner can be selected and all funds are locked.

## Impact
Calling `delete players` only resets the array length to 0, incurring constant-cost storage writes regardless of player count. It cannot run out of gas for having many players, so there is no Denial-of-Service or fund-locking risk attributable to this line.

## Proof of Concept
1. The `players` array is filled with a large number of unique addresses (e.g., several thousand).
2. The raffle duration passes.
3. A user calls `selectWinner()`.
4. The transaction executes up to the `delete players` statement.
5. The gas cost of this operation is too high, causing the transaction to revert with an 'out of gas' error.
6. Since the state is not cleared, any subsequent call will also fail. The contract is now bricked.

## Proof of Code
// A conceptual test. Actually hitting the gas limit depends on the chain's block gas limit.
// The principle is that gas usage scales linearly with array size, and a large enough array will fail.

function test_DoS_OnDeleteLargePlayerArray() public {
    puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, DURATION);

    // This number would need to be very high, e.g., 5000+, depending on the block gas limit.
    // In a test environment with high gas limits, this won't revert but will show high gas usage.
    uint256 largePlayerCount = 1500;
    address[] memory players = new address[](largePlayerCount);
    for(uint i=0; i<largePlayerCount; i++) {
        players[i] = address(uint160(i + 1));
    }
    puppyRaffle.enterRaffle{value: largePlayerCount * ENTRANCE_FEE}(players);

    vm.warp(block.timestamp + DURATION + 1);

    // On a real network, this call would likely fail due to the gas cost of `delete players`.
    // We can't use `vm.expectRevert` for out-of-gas, so we demonstrate by checking gas usage.
    uint256 gasStart = gasleft();
    puppyRaffle.selectWinner();
    uint256 gasUsed = gasStart - gasleft();

    console.log("Gas used for selectWinner with %s players: %s", largePlayerCount, gasUsed);

    // We assert that the gas cost is very high, implying risk of hitting the block limit.
    assertTrue(gasUsed > 5_000_000, "Gas cost for deleting a large array is excessively high");
}

## Suggested Mitigation
Instead of deleting the array, which is costly, re-initialize it. Replace `delete players;` with `players = new address[](0);`. This has a constant, low gas cost regardless of the array's size as it only replaces the storage pointer.



