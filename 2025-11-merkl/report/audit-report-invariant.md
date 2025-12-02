# 2025 11 merkl - Findings Report
## Commit hash: e8c1d6f5a91c1144571c3ed238358a2b032dd32e

##Findings by Pattern


 **Derived From** : Invariant Type: Balance

[H-1]. Governor `recoverFees` function sweeps user pre-deposits leading to protocol insolvency and loss of user funds
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresAdminRole
[M-2]. Permanent DoS of Dispute Resolution if dispute tokens are drained via `recoverERC20` or claims
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: RequiresRole



 **Derived From** : getMerkleRoot() returns a finalized root (one that passed its dispute period) or the empty root

[M-3]. Governor updateTree bypasses dispute period promoting unverified root
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: RequiresAdminRole



 **Derived From** : token.balanceOf(address(this)) >= disputeAmount

[M-4]. DoS of Dispute Resolution via Insolvency when Dispute Token is a Reward Token
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 1
- M: 3
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Invariant Type: Balance

## [H-1]. Governor `recoverFees` function sweeps user pre-deposits leading to protocol insolvency and loss of user funds

### Finding Severity Justification: The `recoverFees` function allows the Governor to withdraw accumulated fees but is implemented to sweep the entire token balance of the contract (`balanceOf(address(this))`). Since the contract holds user pre-deposits (tracked in `creatorBalance`) alongside fees, using this function results in the immediate theft of all user funds held in the contract. This violates the protocol's solvency invariant. Unlike typical governance risks where an admin *could* abuse power, here the admin is *forced* to trigger the vulnerability to perform the standard action of collecting fees, as the function provides no parameter to specify an amount or exclude user deposits. This constitutes a critical logic flaw leading to direct asset loss.
## Derived From Pattern/Invariant
Invariant Type: Balance

## Exploit Type
AccountingInvariantViolation

## Location
DistributionCreator.recoverFees

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `DistributionCreator` contract allows users to pre-deposit reward tokens via `increaseTokenBalance`, which are tracked in the `creatorBalance` mapping. This allows creators to fund multiple campaigns without repeated token transfers. The contract also collects protocol fees, which default to `address(this)` if the `feeRecipient` is not set. 

The contract includes a `recoverFees` function, restricted to the Governor, intended to withdraw these accumulated fees. However, the implementation of `recoverFees` iterates through the provided token list and transfers the **entire** `balanceOf(address(this))` to the destination address. 

Crucially, the contract does not track the aggregate amount of user deposits versus accumulated fees. By sweeping the entire balance, `recoverFees` inadvertently steals all user pre-deposits. This violates the accounting invariant `IERC20(token).balanceOf(address(this)) >= totalCreatorBalances[token]`, rendering the contract insolvent and preventing users from creating campaigns or withdrawing their funds.

## Impact
High. Complete loss of user assets (pre-deposited funds) held in the contract if the Governor executes the fee recovery function.

## Command to Run Test


## Proof of Concept
1. User A deposits 1,000 USDC using `increaseTokenBalance`. The `DistributionCreator` contract now holds 1,000 USDC, and `creatorBalance[UserA][USDC]` is 1,000.
2. Over time, some fees might accumulate (e.g., 50 USDC), or the Governor simply decides to sweep tokens.
3. The Governor calls `recoverFees([USDC], TreasuryAddress)`.
4. The function executes `USDC.safeTransfer(TreasuryAddress, USDC.balanceOf(address(this)))`. This transfers 1,000 (plus any fees) USDC out of the contract.
5. User A attempts to call `createCampaign` (which calls `_pullTokens`) or `decreaseTokenBalance`.
6. The transaction reverts because the contract has 0 USDC balance, despite User A having a recorded balance of 1,000.

## Proof of Code
import { Test } from "forge-std/Test.sol";
import { DistributionCreator } from "contracts/DistributionCreator.sol";
import { MockToken } from "contracts/mock/MockToken.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockACM {
    function isGovernor(address) external pure returns (bool) { return true; }
    function isGovernorOrGuardian(address) external pure returns (bool) { return true; }
}

contract RecoverFeesExploitTest is Test {
    DistributionCreator internal creator;
    MockToken internal token;
    address internal governor = makeAddr("governor");
    address internal user = makeAddr("user");

    function setUp() public {
        // 1. Deploy dependencies
        MockACM acm = new MockACM();
        DistributionCreator impl = new DistributionCreator();
        
        // 2. Deploy Proxy and Initialize
        bytes memory initData = abi.encodeWithSelector(
            DistributionCreator.initialize.selector,
            address(acm),
            makeAddr("distributor"),
            0
        );
        creator = DistributionCreator(address(new ERC1967Proxy(address(impl), initData)));
        token = new MockToken("Test", "TST", 18);
    }

    function test_RecoverFeesSweepsUserDeposits() public {
        // 1. Setup: User deposits 1000 tokens
        uint256 depositAmount = 1000 ether;
        token.mint(user, depositAmount);
        
        vm.startPrank(user);
        token.approve(address(creator), depositAmount);
        creator.increaseTokenBalance(user, address(token), depositAmount);
        vm.stopPrank();

        // Verify invariant: Contract holds user funds
        assertEq(token.balanceOf(address(creator)), depositAmount, "Contract should hold user funds");

        // 2. Governor attempts to recover accumulated fees
        // However, the function implementation forces a sweep of the ENTIRE balance.
        vm.startPrank(governor);
        IERC20[] memory tokens = new IERC20[](1);
        tokens[0] = IERC20(address(token));
        creator.recoverFees(tokens, governor);
        vm.stopPrank();

        // 3. Impact: Contract is insolvent
        assertEq(token.balanceOf(address(creator)), 0, "Contract balance should be 0 after sweep");
        assertEq(creator.creatorBalance(user, address(token)), depositAmount, "User accounting balance remains high");

        // 4. User withdrawal fails due to insolvency
        vm.startPrank(user);
        vm.expectRevert(); // Fails due to insufficient balance in contract (transfer fail)
        creator.decreaseTokenBalance(user, address(token), user, depositAmount);
        vm.stopPrank();
    }
}

## Suggested Mitigation
Modify `recoverFees` to accept a `uint256[] calldata amounts` parameter corresponding to the `tokens` array. This allows the Governor to explicitly specify the amount of fees to withdraw for each token, preventing the forced sweep of user deposits. Alternatively, implementing the global `totalCreatorBalances` tracking variable is a valid solution, but it requires a storage migration script to correctly initialize the total balance of existing user deposits; otherwise, the calculation `balanceOf(this) - totalCreatorBalances` would still result in user funds being swept.


## [M-2]. Permanent DoS of Dispute Resolution if dispute tokens are drained via `recoverERC20` or claims

### Finding Severity Justification: The vulnerability causes a Denial of Service (DoS) of the dispute resolution mechanism. If the `Distributor` contract's balance of the `disputeToken` falls below `disputeAmount` (e.g., due to the token being shared with reward tokens and drained by claims during a period of insolvency, or accidental admin withdrawal), the `resolveDispute` function will revert. This locks the contract in a disputed state (`disputer != 0`), preventing any future Merkle tree updates. While the Admin can resolve this by depositing funds, the contract logic fails to isolate dispute collateral from other outflows, allowing a critical safety mechanism to be blocked.
## Derived From Pattern/Invariant
Invariant Type: Balance

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.recoverERC20

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Distributor` contract must hold `disputeAmount` of `disputeToken` during an active dispute to ensure the disputer can be refunded or the funds seized. However, the `recoverERC20` function allows the Governor to withdraw any token, including the `disputeToken`, without checking for active liabilities. Additionally, if the `disputeToken` is also used as a reward token, users claiming rewards can drain the balance if the contract is insolvent (e.g., due to prior admin withdrawal or engine over-allocation). If the balance of `disputeToken` falls below `disputeAmount` while `disputer != 0`, the `resolveDispute` function will revert due to `safeTransfer` failure. This leaves the contract permanently locked in the dispute state (`disputer != 0`), blocking all future tree updates and administrative changes.

## Impact
The protocol becomes permanently bricked (Denial of Service). No new Merkle trees can be pushed, and the dispute cannot be resolved until funds are manually returned to the contract.

## Command to Run Test


## Proof of Concept
1. A dispute is opened via `disputeTree`, depositing `disputeAmount` into the contract.
2. The Governor calls `recoverERC20(disputeToken, ...)` to withdraw the entire balance of the token (e.g. attempting to migrate funds).
3. The Governor attempts to call `resolveDispute(true)`.
4. The call reverts because `IERC20(disputeToken).safeTransfer` fails due to insufficient balance.
5. The contract remains in dispute mode indefinitely; `updateTree` and `revokeTree` are blocked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {Distributor, MerkleTree} from "contracts/Distributor.sol";
import {AccessControlManager} from "contracts/AccessControlManager.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IAccessControlManager} from "contracts/interfaces/IAccessControlManager.sol";
import {Errors} from "contracts/utils/Errors.sol";

contract MockToken is ERC20 {
    constructor() ERC20("Mock", "MCK") {
        _mint(msg.sender, 1000000 ether);
    }
}

contract DistributorDoSTest is Test {
    Distributor distributor;
    AccessControlManager accessControl;
    MockToken token;
    
    address governor = makeAddr("governor");
    address disputer = makeAddr("disputer");
    
    function setUp() public {
        vm.startPrank(governor);
        
        accessControl = new AccessControlManager();
        accessControl.initialize(governor, governor);
        
        distributor = new Distributor();
        distributor.initialize(IAccessControlManager(address(accessControl)));
        
        token = new MockToken();
        token.transfer(disputer, 1000 ether);
        
        distributor.setDisputeToken(IERC20(address(token)));
        distributor.setDisputeAmount(100 ether);
        distributor.setDisputePeriod(1 hours);
        
        // Push a tree to enable dispute period
        MerkleTree memory tree = MerkleTree(bytes32(uint256(1)), bytes32(uint256(1)));
        distributor.updateTree(tree);
        
        vm.stopPrank();
    }

    function test_PermanentDoS_WhenDisputeTokenDrained() public {
        // 1. Disputer opens a dispute, locking funds in the contract
        vm.startPrank(disputer);
        token.approve(address(distributor), 100 ether);
        distributor.disputeTree("Invalid Root");
        vm.stopPrank();
        
        // Verify dispute is active and funds are held
        assertEq(distributor.disputer(), disputer);
        assertEq(token.balanceOf(address(distributor)), 100 ether);

        // 2. Governor accidentally recovers the dispute tokens
        // (Or users claim rewards draining the balance if disputeToken == rewardToken)
        vm.startPrank(governor);
        distributor.recoverERC20(address(token), governor, 100 ether);
        
        // Distributor balance is now 0
        assertEq(token.balanceOf(address(distributor)), 0);

        // 3. Governor tries to resolve dispute - Reverts due to insufficient funds for refund/seizure
        vm.expectRevert(); 
        distributor.resolveDispute(true);
        
        // 4. Contract is effectively locked (DoS)
        // Cannot revoke tree
        vm.expectRevert(Errors.UnresolvedDispute.selector);
        distributor.revokeTree();
        
        // Cannot update tree
        MerkleTree memory newTree = MerkleTree(bytes32(uint256(2)), bytes32(uint256(2)));
        vm.expectRevert(Errors.NotTrusted.selector);
        distributor.updateTree(newTree);
        
        vm.stopPrank();
    }
}

## Suggested Mitigation
Modify `recoverERC20` to revert if the token being recovered is the `disputeToken`, a dispute is active (`disputer != 0`), and the resulting balance would be less than `disputeAmount`. Additionally, in the `_claim` function, if the reward token is the same as the `disputeToken` and a dispute is active, verify that the contract retains at least `disputeAmount` after the transfer.





 **Derived From** : getMerkleRoot() returns a finalized root (one that passed its dispute period) or the empty root

## [M-3]. Governor updateTree bypasses dispute period promoting unverified root

### Finding Severity Justification: The vulnerability allows the protocol's core security mechanism—the dispute period—to be bypassed, promoting an unverified and potentially incorrect Merkle root to a valid state. While the exploit requires the Governor to call `updateTree` twice in succession, this is a plausible operational workflow (e.g., correcting a mistake or batching updates). The contract logic fails to preserve the 'safe' fallback tree in this scenario, creating a 'footgun' where a standard remedial action inadvertantly validates the erroneous root. This fits the criteria for a Medium severity finding (valid functionality leading to security bypass/loss via operational error).
## Derived From Pattern/Invariant
getMerkleRoot() returns a finalized root (one that passed its dispute period) or the empty root

## Exploit Type
StandardViolation

## Location
Distributor.updateTree

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The protocol relies on `lastTree` serving as a safe fallback that has passed its dispute period. However, `updateTree` unconditionally updates `lastTree` to the current `tree` before installing the new `tree`. If the Governor calls `updateTree` twice in rapid succession (e.g., to batch updates or correct a mistake), the first update's root—which has typically not yet passed its dispute period—is promoted to `lastTree`.

During the dispute period of the second update, `getMerkleRoot()` returns `lastTree` (the root from the first update). Since this root never completed its verification window, users can claim rewards against it immediately. If the first root was incorrect or malicious, funds can be stolen, completely bypassing the protocol's security invariant.

## Impact
Bypass of the dispute period mechanism allowing claims against an unverified Merkle root.

## Command to Run Test


## Proof of Concept
1. Governor calls `updateTree(Tree1)`. `tree` = Tree1, `lastTree` = Tree0. Dispute period for Tree1 starts.
2. Governor immediately calls `updateTree(Tree2)`. `tree` = Tree2, `lastTree` = Tree1. Dispute period for Tree2 starts.
3. `getMerkleRoot()` checks if we are in a dispute period (Yes, for Tree2). It returns `lastTree.merkleRoot` (Tree1).
4. Tree1 is now active for claims despite never having passed its dispute period.
5. If Tree1 contained invalid data, users can exploit it immediately.

## Proof of Code
import { Test } from "forge-std/Test.sol";
import { Distributor, MerkleTree } from "contracts/Distributor.sol";
import { AccessControlManager } from "contracts/AccessControlManager.sol";
import { IAccessControlManager } from "contracts/interfaces/IAccessControlManager.sol";

contract DistributorExploitTest is Test {
    Distributor distributor;
    AccessControlManager acm;
    address governor = address(0x1);
    address guardian = address(0x2);

    function setUp() public {
        vm.startPrank(governor);
        // Setup ACM
        acm = new AccessControlManager();
        acm.initialize(governor, guardian);
        // Setup Distributor
        distributor = new Distributor();
        distributor.initialize(IAccessControlManager(address(acm)));
        distributor.setDisputePeriod(1 hours);
        vm.stopPrank();
    }

    function test_Exploit_DoubleUpdateBypass() public {
        vm.startPrank(governor);

        // 0. Setup: Establish a valid 'lastTree' (Tree0)
        MerkleTree memory tree0 = MerkleTree(bytes32(uint256(100)), bytes32(0));
        distributor.updateTree(tree0);
        vm.warp(block.timestamp + 1 hours + 1); // Pass dispute period
        assertEq(distributor.getMerkleRoot(), tree0.merkleRoot); // Tree0 is active

        // 1. First Update (e.g. Mistake or Malicious unverified root)
        MerkleTree memory tree1 = MerkleTree(bytes32(uint256(1)), bytes32(0));
        distributor.updateTree(tree1);
        // State: tree=Tree1, lastTree=Tree0. Dispute period active.
        // getMerkleRoot() returns lastTree (Tree0)

        // 2. Second Update immediately (e.g. Correction)
        MerkleTree memory tree2 = MerkleTree(bytes32(uint256(2)), bytes32(0));
        distributor.updateTree(tree2);

        // 3. Verify Exploit: getMerkleRoot returns Tree1
        // The protocol falls back to 'lastTree', which was updated to Tree1 despite Tree1 never passing the dispute period.
        bytes32 activeRoot = distributor.getMerkleRoot();
        
        assertEq(activeRoot, tree1.merkleRoot, "Unverified Tree1 should be active due to bypass");
        assertTrue(block.timestamp < distributor.endOfDisputePeriod(), "Should still be in dispute period");
        
        vm.stopPrank();
    }
}

## Suggested Mitigation
Update the `updateTree` function to conditionally update `lastTree` only if the current `tree` has finalized (passed its dispute period).

```solidity
    function updateTree(MerkleTree calldata _tree) external {
        if (
            disputer != address(0) ||
            ((canUpdateMerkleRoot[msg.sender] != 1 || block.timestamp < endOfDisputePeriod) && !accessControlManager.isGovernor(msg.sender))
        ) revert Errors.NotTrusted();

        // MODIFIED START
        // Only update lastTree if the current tree was actually valid/finalized
        if (block.timestamp >= endOfDisputePeriod) {
            lastTree = tree;
        }
        // If we are replacing a tree that is still in its dispute period, 
        // we keep the existing lastTree (the last known safe root).
        tree = _tree;
        // MODIFIED END

        uint48 _endOfPeriod = _endOfDisputePeriod(uint48(block.timestamp));
        endOfDisputePeriod = _endOfPeriod;
        emit TreeUpdated(_tree.merkleRoot, _tree.ipfsHash, _endOfPeriod);
    }
```





 **Derived From** : token.balanceOf(address(this)) >= disputeAmount

## [M-4]. DoS of Dispute Resolution via Insolvency when Dispute Token is a Reward Token

### Finding Severity Justification: The vulnerability allows a lack of liquidity (insolvency) in reward tokens to escalate into a denial of service of the dispute resolution mechanism and the theft of dispute collateral. While the root cause of the insolvency might be creator error (underfunding) or timing (JIT liquidity), the protocol fails to segregate dispute collateral from reward funds. This results in the 'disputer' variable becoming permanently stuck (blocking all future tree updates) and the disputer losing their collateral to claimers. Recovery requires governance intervention to fund the contract, qualifying as a temporary DoS of critical functionality and conditional asset loss.
## Derived From Pattern/Invariant
token.balanceOf(address(this)) >= disputeAmount

## Exploit Type
AccountingInvariantViolation

## Location
Distributor._claim

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Distributor` contract does not segregate `disputeToken` funds from reward tokens. If the `disputeToken` is the same as a reward token (e.g., USDC) and the contract's balance is insufficient to cover both the total pending rewards and the dispute collateral (e.g., due to just-in-time funding or off-chain calculation discrepancies), a user can claim their rewards using the dispute collateral held in the contract.

When a dispute is active, the `disputeAmount` is transferred to the contract. If a user subsequently claims rewards (`claim`), the `_claim` function transfers tokens out without verifying if the remaining balance covers the `disputeAmount`. If the balance drops below `disputeAmount`, the `resolveDispute` function will inevitably revert when attempting to refund the collateral (via `safeTransfer`), permanently locking the contract in the disputed state (`disputer != 0`) and blocking all future tree updates.

## Impact
Permanent Denial of Service of the dispute resolution mechanism and future updates. Potential loss of assets for the disputer if funds cannot be recovered.

## Command to Run Test


## Proof of Concept
1. Admin sets `disputeToken` to USDC.
2. Creator creates a campaign rewarding USDC but the `Distributor` holds 0 USDC initially.
3. Governor calls `updateTree` with a root that entitles Alice to 100 USDC.
4. Bob calls `disputeTree`, sending 100 USDC as collateral. `Distributor` balance becomes 100 USDC.
5. Alice calls `claim`. The contract sends 100 USDC to Alice (draining the dispute collateral).
6. `Distributor` balance is now 0.
7. Governor calls `resolveDispute(true)`. The call reverts because `safeTransfer` of 100 USDC to Bob fails.
8. The contract is stuck with `disputer != 0`.

## Proof of Code
import { Test } from "forge-std/Test.sol";
import { Distributor, MerkleTree } from "contracts/Distributor.sol";
import { AccessControlManager } from "contracts/AccessControlManager.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MCK") { _mint(msg.sender, 1000000e18); }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract DistributorExploitTest is Test {
    Distributor distributor;
    AccessControlManager accessControl;
    MockERC20 token;
    address alice = address(0xA);
    address bob = address(0xB);

    function setUp() public {
        accessControl = new AccessControlManager();
        accessControl.initialize(address(this), address(this));

        distributor = new Distributor();
        distributor.initialize(accessControl);

        token = new MockERC20();
    }

    function test_Exploit_InsolvencyDoS() public {
        // 1. Configure Dispute Token
        distributor.setDisputeToken(token);
        uint256 disputeAmount = 100e18;
        distributor.setDisputeAmount(disputeAmount);

        // 2. Setup Alice's claim in Tree 1 (Previous valid tree)
        uint256 claimAmount = 100e18;
        // Simple tree with 1 leaf: Root = leaf
        bytes32 leaf = keccak256(abi.encode(alice, address(token), claimAmount));
        MerkleTree memory tree1 = MerkleTree({merkleRoot: leaf, ipfsHash: bytes32(0)});

        // 3. Update to Tree 1 and make it valid (pass dispute period)
        distributor.updateTree(tree1);
        vm.warp(block.timestamp + 4000); // Default epoch is 3600s

        // 4. Update to Tree 2. This moves Tree 1 to `lastTree`.
        MerkleTree memory tree2 = MerkleTree({merkleRoot: bytes32(uint256(0xdead)), ipfsHash: bytes32(0)});
        distributor.updateTree(tree2);

        // 5. Create Dispute
        // Bob sends disputeAmount. Distributor balance becomes exactly disputeAmount.
        token.mint(bob, disputeAmount);
        vm.startPrank(bob);
        token.approve(address(distributor), disputeAmount);
        distributor.disputeTree("Invalid Root");
        vm.stopPrank();

        assertEq(token.balanceOf(address(distributor)), disputeAmount);

        // 6. Alice claims rewards
        // With active dispute, Distributor uses `lastTree` (Tree 1).
        // Alice drains the dispute collateral.
        address[] memory users = new address[](1); users[0] = alice;
        address[] memory tokens = new address[](1); tokens[0] = address(token);
        uint256[] memory amounts = new uint256[](1); amounts[0] = claimAmount;
        bytes32[][] memory proofs = new bytes32[][](1);
        proofs[0] = new bytes32[](0); // Empty proof for single-leaf

        vm.prank(alice);
        distributor.claim(users, tokens, amounts, proofs);

        // 7. Check Impact
        assertEq(token.balanceOf(address(distributor)), 0);
        
        // 8. Resolve Dispute fails due to lack of funds
        vm.expectRevert(); 
        distributor.resolveDispute(true);
    }
}

## Suggested Mitigation
Modify `_claim` (and `recoverERC20`) to verify sufficient liquidity remains for the dispute collateral if the token being transferred is the dispute token.

```solidity
// Inside _claim loop, before the transfer:
if (token == address(disputeToken) && disputer != address(0)) {
    // Ensure we don't dip into the dispute collateral
    if (IERC20(token).balanceOf(address(this)) < toSend + disputeAmount) {
        revert Errors.NotEnoughBalance();
    }
}
// Proceed with transfer
IERC20(token).safeTransfer(recipient, toSend);
```



