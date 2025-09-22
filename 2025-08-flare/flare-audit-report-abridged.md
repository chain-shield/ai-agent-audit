# 2025 08 flare - Findings Report
## Commit hash: b703ea27ee98e488d245083c63011cdbf43a74c4

LEGIT HIGHS: H-15, H-17, H-32, H-14a (DONE)
LEGIT MEDIUMS: M-7 (DONE), M-11 u, M-12 (DONE), M-13 u, M-21, M-27, M-30 u, M-33 u

DUP CHECKS: M-7, M-12 , H-14a ==> report done
DUP CHECK: H-15, H-17 ==> NOT DUP
DUP CHECK: M-21,M-27,H-32 ==> NOT DUP

##Findings by Pattern


 
 **Derived From** : FTSO price used without freshness/heartbeat checks for CR and pricing - ORACLE

[M-7]. CollateralPool exits can bypass exit-CR using stale spot oracle via AssetManager.assetPriceNatWei (no max-age) enabling premature withdrawals -- LEGIT
REPORT DONE


 **Derived From** : (pre.totalCollateral - post.totalCollateral) == ret && (pre.wNatBalance - wNat.balanceOf(address(this))) == ret

[M-11]. exitTo allows sending ETH to WNat, re-wrapping back to pool and desynchronizing accounting (ret != ΔwNat) -- LEGIT



 **Derived From** : Oracle price used without staleness check to value challenger rewards

[M-12]. Stale FTSO price (no heartbeat) inflates challenger payout via Conversion.currentAmgPriceInTokenWei -- LEGIT
REPORT DONE


 **Derived From** : USD5→token conversion skips decimals when no FTSO symbol (mispriced rewards)

[M-13]. Underpaid USD-fixed rewards when vault token has no FTSO symbol: Conversion.convertFromUSD5 ignores token.decimals -- LEGIT



 **Derived From** : Collateral payout uses untrusted spot price without staleness checks

[H-14a]. redeemFromAgentInCollateral uses untrusted spot FTSO price without heartbeat, enabling stale/manipulated price to inflate or short-change collateral payouts  -- LEGIT

REPORT DONE



 **Derived From** : if request.transferToCoreVault == false then (let ev = last RedemptionDefault(agentVault, redeemer, id, underlyingValueUBA, paidC1Wei, paidPoolWei)): (paidC1Wei + paidPoolWei) > 0 && ERC20(Agents.getVaultCollateral(Agent.get(request.agentVault)).token).balanceOf(request.redeemer)_post - _pre == paidC1Wei && IWNat(Globals.getWNat()).balanceOf(request.redeemer)_post - _pre == paidPoolWei; else (Core-Vault) last RedemptionDefault has paidC1Wei == 0 && paidPoolWei == 0

[H-15]. Partial vault payout mis-accounted in redemptionPaymentDefault causes RedemptionDefault event/balance mismatch and underpayment -- LEGIT


 **Derived From** : sum(execFeeWei for each RedemptionRequested event emitted in this tx) == msg.value - (msg.value % Conversion.GWEI)

[H-17]. Executor fee gets stuck if redeem creates 0 requests (front-of-queue sub‑lot tickets + ticket cap) — fee conservation breaks -- LEGIT




 **Derived From** : post.totalCollateral == 0 && post.wNat.balanceOf(address(this)) == 0

[M-21]. Permissionless pool entry DoS prevents CollateralPool.destroy from ever reaching zero-balance post-state -- LEGIT

 **Derived From** : (post.totalCollateral == 0 || post.totalCollateral >= MIN_NAT_BALANCE_AFTER_EXIT) && (post.token.totalSupply() == 0 || post.token.totalSupply() >= MIN_TOKEN_SUPPLY_AFTER_EXIT)

[M-27]. Exits can be permanently DoS’ed when pool NAT falls below MIN_NAT_BALANCE_AFTER_EXIT via protocol payout before user exit -- LEGIT


 **Derived From** : Zero price => infinite CR; liquidation can be skipped/ended via price desync

[M-30]. Zero-price path inflates CR to 1e10 and, via max(ratio,ratioTrusted), suppresses/ends liquidation -- LEGIT


 **Derived From** : totalCollateral == old(totalCollateral) + msg.value

[H-32]. First-entrant share inflation in CollateralPool.enter drains pre-existing pool fees and collateral
-- LEGIT


 **Derived From** : Emergency pause bypass in executeMinting allows minting while paused

[M-33]. executeMinting lacks pause/attachment gating, allowing mint finalization during emergency pause -- LEGIT



### Number of Findings
- H: 4 
- M: 8 

##Findings by Pattern



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

## Suggested Mitigation
Enforce price freshness and/or trusted-quality reads for all CR/pricing paths used to gate exits:

- Prefer trusted reads: have Conversion.readFtsoPrice use getPriceFromTrustedProviders with a configured providers threshold and spread checks; or expose a dedicated IAssetManager.assetPriceNatWeiWithTs that internally calls currentAmgPriceInTokenWeiWithTs(_fromTrustedProviders=true) and returns both price and timestamps.
- Add a max-age check: plumb timestamps through assetPriceNatWei (or add a new method) and in CollateralPool._getAssetPrice reject prices with block.timestamp - priceTimestamp > settings.maxTrustedPriceAgeSeconds.
- As a defense-in-depth, consider using the trusted price variant by default for CR gating, and fall back to last known good trusted price if the current one is stale (rather than allowing exits).

These changes ensure exits cannot be greenlit by stale or low-quality oracle data.







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


## Suggested Mitigation
Scale by token.decimals even when tokenFtsoSymbol is empty. Two equivalent approaches:
- Direct scaling: if (_token.decimals >= 5) return SafePct.mulDiv(_amountUSD5, 10**(_token.decimals - 5), 1); else return SafePct.mulDiv(_amountUSD5, 1, 10**(5 - _token.decimals));
- Unified branch: treat missing FTSO as tokenPrice = 1 with tokenFtsoDec = 5 and reuse the existing mulDiv path, i.e., expPlus = _token.decimals + 5 - 5 = _token.decimals, then mulDiv(_amountUSD5, 10**_token.decimals, 10**5).
Add unit tests for typical decimals (2, 6, 18).





 **Derived From** : Collateral payout uses untrusted spot price without staleness checks

## [H-14a]. redeemFromAgentInCollateral uses untrusted spot FTSO price without heartbeat, enabling stale/manipulated price to inflate or short-change collateral payouts

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

## Suggested Mitigation
Add a guard after building redemptionList and before fee distribution:
- Revert if no requests were created and msg.value > 0, e.g.: require(redemptionList.length > 0, RedeemZeroLots()); This preserves user funds by reverting the whole tx (recommended, simplest, no external calls).

Alternatively, explicitly refund the rounded executor fee if no requests were created:
- If redemptionList.length == 0, compute uint256 refundWei = (msg.value / Conversion.GWEI) * Conversion.GWEI; then transfer refundWei back to msg.sender and return 0. This preserves the fee-conservation invariant and avoids stuck funds.

Either approach should be placed immediately after the ticket-processing loop and before calculating/distributing executorFeeNatGWei.



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

## Suggested Mitigation
Introduce a two-phase pool closure and forced-settlement mechanism:
- Phase 1 (Lock): When the agent announces destroy, the AssetManager locks the pool (enter() and any action that can increase totalSupply are disabled). Existing holders can still exit/transfer per current rules.
- Phase 2 (Close): After a grace window, enable a claim-style forced settlement that lets any CPT holder burn their tokens (ignoring timelocks) to claim their pro‑rata WNat; do not require the pool to enumerate holders. Implement this as: token.burn(holderBalance, ignoreTimelocked=true) callable by holder, which pulls pro‑rata WNat from the pool to the holder.
- Allow CollateralPool.destroy() to proceed once the pool is locked and the forced-settlement window has elapsed, even if residual dust remains, by moving remaining WNat into an escrow/cleaner contract where late holders can still self-claim against burning their CPTs. This removes the zero-supply hard gate, eliminates the griefing vector, and preserves holder redeemability without giving the agent the power to seize third-party balances.
Additionally, add a guard in enter() that reverts if the agent status is DESTROYING (checked via AssetManager), and provide an AssetManager-only function to lock pool entries immediately upon announceDestroyAgent().






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

## Suggested Mitigation
Never return a healthy sentinel for zero/invalid prices. In AgentCollateral.collateralRatioBIPS, treat backingTokenWei == 0 as unusable (revert or return 0 and mark invalid). In Liquidation.getCollateralRatioBIPS, ignore zero/invalid paths and require both ratio and ratioTrusted to be >0 and fresh; if one is invalid, use the other; if both invalid, revert. For safety-critical checks (start/end liquidation), require both vaultCR and poolCR from valid, fresh feeds to satisfy thresholds, e.g., use the minimum of ratio and ratioTrusted or require both to pass, never Math.max. Add explicit staleness/zero guards at call sites and enforce max price age.








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

## Suggested Mitigation
Gate executeMinting consistently with other mint paths: add onlyAttached and notEmergencyPaused modifiers and require(AssetManagerState.get().mintingPausedAt == 0). For example: function executeMinting(...) external onlyAttached notEmergencyPaused { require(AssetManagerState.get().mintingPausedAt == 0, MintingPaused()); ... } This aligns behavior with selfMint and mintFromFreeUnderlying and ensures both global emergency pause and minting pause are enforced.



