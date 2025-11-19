## Verified Patterns Found: 12

## Verified Patterns Found in following Categories:

- UnboundedLoops
- StaleOracleAcceptance
- TWAPWindowPinningOrLowLiquidity
- OracleUsingDEXorTWAP
- ERC20DecimalsMismatch
- PricePrecisionOrRoundingError



## Summary of Patterns

Asymmetric impact price fallback math can severely distort mark price and liquidations

Impact-price and basis-spread TWAP/EMA vulnerable to low-liquidity pinning

Mark price / funding oracle derived directly from manipulable CLOB state

Funding and mark components use TWAP without staleness/age bounds

Perps margin and price math assume 18‑decimals while USDC collateral may be 6‑decimals

Perps mark price & funding TWAP can be pinned via low-liquidity orderbook impact price

Impact price oracle can be pinned via thin orderbook liquidity, skewing mark price and funding

Asymmetric impact price math leads to large, manipulable mark price skew

Perps account operations iterate over unbounded asset lists, enabling gas-based DoS

Funding and mark price rely on TWAP/impact prices from low-liquidity CLOB without safeguards

Perps mark price and funding use TWAPs with no freshness or liquidity checks

Orderbook-based impact price oracle lets traders skew mark price and liquidations

## Patterns



 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: MarketLib.getImpactPrice

 ### Title
Asymmetric impact price fallback math can severely distort mark price and liquidations
 ### Description/Code Snippet
In `MarketLib.getImpactPrice`, when there is insufficient orderbook liquidity to fill the configured `impactNotional`, the function applies two different fallback formulas for the missing notional on bids vs asks:

```solidity
function getImpactPrice(Market storage self, uint256 impactNotional) internal view returns (uint256 impactPrice) {
    (uint256 baseAmount, uint256 quoteUsed) = StorageLib.loadBook(self.asset).quoteBidInQuote(impactNotional);

    if (impactNotional > quoteUsed) 
        baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);

    uint256 impactBid = baseAmount == 0 ? 0 : impactNotional.fullMulDiv(1e18, baseAmount);

    (baseAmount, quoteUsed) = StorageLib.loadBook(self.asset).quoteAskInQuote(impactNotional);

    if (impactNotional > quoteUsed) 
        baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, 1);

    uint256 impactAsk = impactNotional.fullMulDiv(1e18, baseAmount);

    return (impactBid + impactAsk) / 2;
}
```

On the **bid** side, any shortfall `impactNotional - quoteUsed` is converted to `baseAmount` using a denominator of `type(uint256).max`:

```solidity
(impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);
```

For realistic sizes this evaluates to (effectively) **zero** additional base. However, `impactBid` is then computed using the full `impactNotional` in the numerator but only the actually-executed base in the denominator, giving a **significantly overstated bid impact price** whenever there is not enough depth to fill the entire impact notional.

On the **ask** side, the shortfall is converted using a denominator of `1`:

```solidity
(impactNotional - quoteUsed).fullMulDiv(1e18, 1);
```

which effectively adds `(impactNotional - quoteUsed) * 1e18` base to the denominator, massively **overstating available base** and therefore **understating the impact ask price**.

Consequences:
- When the book is shallow relative to `impactNotional`, `impactBid` can be much larger than the true executable price, and `impactAsk` much smaller. Their average is then fed into the mark price calculation via `_cacheImpactPrice` and `getImpactPriceTwap()`.
- This biased impact price pollutes `markPrice` in `setMarkPrice`:

```solidity
_cacheImpactPrice(self);
...
uint256 p3 = self.getImpactPriceTwap();
self.markPrice = markPrice = _getMedian(p1, p2, p3);
```

Since mark price drives funding, maintenance margin, and liquidation decisions, an attacker can:
- Thin or remove liquidity on one side of the book so that `quoteUsed << impactNotional` but leave a small stub of depth.
- The mis-scaled fallback then produces extreme impactBid/impactAsk values, skewing `p3` and thus the mark price over time.
- That in turn can make healthy accounts appear under- or over-collateralized, triggering **wrongful liquidations** or letting under-margined positions persist, pushing bad debt into the insurance fund.

This is not just an oracle-noise issue: the math uses fundamentally inconsistent denominators (`type(uint256).max` vs `1`), so the fallback behavior is qualitatively wrong, and can be deterministically exploited whenever the available depth is below the configured impact notional.
 ### Static Signals
price derived from internal orderbook as oracle component, fallback path when impactNotional > quoteUsed, uses fullMulDiv(..., type(uint256).max) on bid side, uses fullMulDiv(..., 1) on ask side, markPrice uses impactPrice TWAP as one of three components
 ### Assets at Risk
trader collateral, liquidation correctness, insurance fund, system-wide solvency (bad debt via mis-priced liquidations)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: MarketLib / PriceHistoryLib.setMarkPrice / getImpactPriceTwap / getBasisSpreadEMA / PriceHistoryLib.twap / PriceHistoryLib.ema

 ### Title
Impact-price and basis-spread TWAP/EMA vulnerable to low-liquidity pinning
 ### Description/Code Snippet
The mark price for each perps market is derived from three components, two of which are TWAP/EMA-based and can be skewed under low-liquidity or short-window conditions without additional safety checks.

Mark price computation:
```solidity
function setMarkPrice(Market storage self, uint256 indexPrice) internal returns (uint256 markPrice) {
    MarketMetadata storage metadata = StorageLib.loadMarketMetadata(self.asset);

    _cacheBasisSpread(self, indexPrice);
    _cacheImpactPrice(self);

    uint256 p1 = self.getFundingRateComponent(indexPrice);
    uint256 p2 = (indexPrice.toInt256() + self.getBasisSpreadEMA()).toUint256();
    uint256 p3 = self.getImpactPriceTwap();

    self.markPrice = markPrice = _getMedian(p1, p2, p3);
    ...
}
```

Where:

- Impact-price TWAP:
```solidity
function getImpactPriceTwap(Market storage self) internal view returns (uint256 impactPriceTwap) {
    bytes32 asset = self.asset;
    return StorageLib.loadMarketMetadata(asset).impactPriceHistory.twap(
        StorageLib.loadFundingRateEngine(asset).getFundingInterval(asset)
    );
}
```

`impactPriceHistory` is updated based on simulated large trades against the LOB:
```solidity
function _cacheImpactPrice(Market storage self) private returns (uint256 impactPrice) {
    // impact notional is 500 * max leverage
    uint256 impactNotional =
        uint256(500e18).fullMulDiv(StorageLib.loadMarketSettings(self.asset).maxOpenLeverage, 1e18);

    impactPrice = self.getImpactPrice(impactNotional);
    StorageLib.loadMarketMetadata(self.asset).impactPriceHistory.snapshot(impactPrice);
}

function getImpactPrice(Market storage self, uint256 impactNotional) internal view returns (uint256 impactPrice) {
    (uint256 baseAmount, uint256 quoteUsed) = StorageLib.loadBook(self.asset).quoteBidInQuote(impactNotional);
    ...
    uint256 impactBid = baseAmount == 0 ? 0 : impactNotional.fullMulDiv(1e18, baseAmount);
    ...
    uint256 impactAsk = impactNotional.fullMulDiv(1e18, baseAmount);
    return (impactBid + impactAsk) / 2;
}
```

- Basis-spread EMA:
```solidity
function _cacheBasisSpread(Market storage self, uint256 indexPrice) private {
    uint256 midPrice = self.getMidPrice();
    if (midPrice == 0) return;

    int256 basisSpread = midPrice.toInt256() - indexPrice.toInt256();
    StorageLib.loadMarketMetadata(self.asset).basisSpreadHistory.snapshotBasisSpread(basisSpread);
}

function getBasisSpreadEMA(Market storage self) internal view returns (int256 basisSpreadEMA) {
    return StorageLib.loadMarketMetadata(self.asset).basisSpreadHistory.ema(15 minutes);
}
```

Both `impactPriceHistory.twap(...)` and `basisSpreadHistory.ema(...)`:

- Perform **no liquidity checks** (e.g. minimum OI, minimum number of active orders, minimum depth at TOB).
- Do not require a minimum number of observations or minimum age of data; `twap` and `ema` happily operate on a single snapshot.
- Use a window parameter (`fundingInterval` for TWAP, literal `15 minutes` for EMA) interpreted purely as a count-of-snapshots proxy in EMA, and as a time window in TWAP, but without constraints that it must be reasonably large (e.g. >=10–30 minutes).

As a result, in a thinly traded or newly created market an attacker can:

1. Use a small amount of capital to place and reshuffle very few orders around TOB to massively move `midPrice` and the synthetic impact price (because impactNotional is constant and not liquidity-aware).
2. Force `_cacheBasisSpread` and `_cacheImpactPrice` to record extreme basis spreads and impact prices into histories.
3. Because there is no guard that **TWAP window** has enough distinct, time-separated observations, a single manipulated snapshot can dominate `impactPriceHistory.twap(...)` when `getFundingInterval(asset)` is short or the history is short.
4. Similarly, `basisSpreadHistory.ema(15 minutes)` uses `period` purely as a count parameter (`count = min(period, nSnapshots)`), so early epochs with only a few snapshots effectively compute an EMA over attacker-chosen values without any smoothing.

That results in `p2` or `p3` being skewed far away from true fair value. Since mark price uses a **median** of (funding component, EMA basis-augmented index, impact-price TWAP), manipulating any two of the three terms (or even one, when the others are close) can:

- Shift `markPrice` enough to incorrectly trigger or avoid liquidations.
- Distort funding-rate calculations (`p1` depends on historical mark vs index spread as well).

No minimum-liquidity floor, trade volume requirement, or observation-age-based filtering is applied anywhere in these flows.

This matches `TWAPWindowPinningOrLowLiquidity`:

- TWAP/EMA windows are effectively unbounded or mis-specified (`15 minutes` used as a count, `fundingInterval` programmable, potentially very small).
- The oracle derivation uses a single CLOB-based source with no protection against thin books.
- There are **no** liquidity floors, freshness checks, or minimum observation counts, making it relatively easy (especially in new markets) for an attacker to pin or swing the TWAP/EMA by manipulating just a few orders.
 ### Static Signals
twapWindow = fundingInterval (configurable, may be short), ema period literal 15 minutes used as observation count, no min liquidity / OI checks around mid/impact price, no min observation count or age checks, markPrice = median(p1, p2, p3) where p2/p3 depend on manipulable histories
 ### Assets at Risk
user margin (incorrect liquidations / missed liquidations), insurance fund (funding mispricing and bad-debt paths), overall perps market pricing integrity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: MarketLib.getImpactPrice / setMarkPrice / _cacheImpactPrice

 ### Title
Mark price / funding oracle derived directly from manipulable CLOB state
 ### Description/Code Snippet
The perpetuals mark price and funding rate rely heavily on price components derived directly from the on-chain orderbook without robust manipulation resistance. An attacker who controls the book (or a large fraction of it) can systematically skew mark price and funding, potentially forcing liquidations or creating profitable funding arbitrage.

Key code paths:

1) `MarketLib.getImpactPrice` computes an 'impact price' based on quoting a fixed notional against the current orderbook:
```solidity
function getImpactPrice(Market storage self, uint256 impactNotional) internal view returns (uint256 impactPrice) {
    (uint256 baseAmount, uint256 quoteUsed) = StorageLib.loadBook(self.asset).quoteBidInQuote(impactNotional);

    if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);

    uint256 impactBid = baseAmount == 0 ? 0 : impactNotional.fullMulDiv(1e18, baseAmount);

    (baseAmount, quoteUsed) = StorageLib.loadBook(self.asset).quoteAskInQuote(impactNotional);

    if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, 1);

    uint256 impactAsk = impactNotional.fullMulDiv(1e18, baseAmount);

    return (impactBid + impactAsk) / 2;
}
```
This uses only current book quotes (no time-weighting or depth constraints beyond a fixed impact notional), and does not check staleness, minimum volume, or participation from independent liquidity providers. An attacker can cheaply populate both sides of the book with their own orders at extreme prices (subject only to tickSize & maxNumOrders caps) to push `impactPrice` arbitrarily high or low while taking minimal execution risk (they can widen spreads so external takers are unlikely to hit them).

2) The impact price is cached and fed into mark price:
```solidity
function _cacheImpactPrice(Market storage self) private returns (uint256 impactPrice) {
    // impact notional is 500 * max leverage
    uint256 impactNotional = uint256(500e18).fullMulDiv(
        StorageLib.loadMarketSettings(self.asset).maxOpenLeverage,
        1e18
    );

    impactPrice = self.getImpactPrice(impactNotional);

    StorageLib.loadMarketMetadata(self.asset).impactPriceHistory.snapshot(impactPrice);
}

function setMarkPrice(Market storage self, uint256 indexPrice) internal returns (uint256 markPrice) {
    MarketMetadata storage metadata = StorageLib.loadMarketMetadata(self.asset);

    _cacheBasisSpread(self, indexPrice);
    _cacheImpactPrice(self);

    uint256 p1 = self.getFundingRateComponent(indexPrice);
    uint256 p2 = (indexPrice.toInt256() + self.getBasisSpreadEMA()).toUint256();
    uint256 p3 = self.getImpactPriceTwap();

    self.markPrice = markPrice = _getMedian(p1, p2, p3);

    metadata.markPriceHistory.snapshot(markPrice);
    metadata.indexPriceHistory.snapshot(indexPrice);
}
```
`getImpactPriceTwap()` TWAPs the impact price over the funding interval, but the underlying snapshots come solely from the manipulable orderbook state. Because markPrice is the median of (funding-rate-derived component, basis-spread-adjusted index, and impactPrice TWAP), an attacker who systematically biases the book can shift this median. Over time they can:

* Move markPrice away from the trusted `indexPrice` in a chosen direction.
* Affect when accounts become liquidatable (`getUpnl`, maintenance margin, min open margin all use `self.markPrice`).
* Influence funding payments via `FundingRateEngine.settleFunding` (mark TWAP and basis spread feed into fundingRate and cumulative funding index), possibly earning predictable positive funding on offsetting positions across accounts.

Static vulnerability signals:

* Oracle derived from an internal DEX/book: `getImpactPrice` uses CLOB quotes as source-of-truth without external oracle or multi-source sanity-checks.
* No explicit staleness or sanity bounds: there is no check that the book depth is organic or that impact price hasn’t deviated too far from a trusted external reference beyond the simple divergenceCap used elsewhere for execution, and that divergenceCap does not directly cap the mark price path here.
* TWAP is over previous snapshots of the same manipulable data; an attacker can repeatedly manipulate the book at each keeper call to slowly steer the TWAP.

Impact sketch:

*Liquidation skew:* An attacker accumulates positions and, over many funding intervals, maintains a lopsided book around the asset (e.g., very high bids or low asks posted only by themselves). Each keeper call to `setMarkPrice` incorporates these manipulated quotes into `p3`, nudging `markPrice` above (or below) the true index. Because UPNL and maintenance margin checks use `markPrice`, the attacker may keep their own account appearing solvent while pushing other accounts into liquidation earlier than they should, then profit as a liquidator.

*Funding arbitrage:* By chronic manipulation of impact prices, the attacker can bias the perceived basis and thus the funding rate. Holding compensating positions across accounts allows them to collect systematically positive funding from honest traders while keeping overall directional risk neutral.

Given the protocol documentation explicitly calls out price-history manipulation and mark price manipulation as in-scope, and given there are no robust manipulation mitigations in this oracle construction, this matches the `OracleUsingDEXorTWAP` pattern and is a strong candidate for economic attack vectors when the attacker can cheaply dominate the orderbook liquidity for a given market.
 ### Static Signals
price derived from internal CLOB quotes, no staleness or min-liquidity checks on impact price, impactPrice TWAP used directly in markPrice, markPrice drives UPNL, margin, funding
 ### Assets at Risk
user margin collateral, insurance fund, perps traders’ positions, liquidation safety
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: MarketLib.settleFunding

 ### Title
Funding and mark components use TWAP without staleness/age bounds
 ### Description/Code Snippet
Oracle-like prices for funding and mark components are taken from internal `PriceHistory` TWAPs with **no freshness or max-age validation**, so stale snapshots can be used indefinitely if keepers fail to update, or if snapshots were manipulated in the past.

Key call sites:

1. `MarketLib.settleFunding`:

```solidity
function settleFunding(Market storage self) internal {
    bytes32 asset = self.asset;

    FundingRateEngine storage fundingRateEngine = StorageLib.loadFundingRateEngine(asset);
    MarketMetadata storage metadata = StorageLib.loadMarketMetadata(asset);

    uint256 interval = fundingRateEngine.getTimeSinceLastFunding();

    (int256 funding, int256 cumulativeFunding) = fundingRateEngine.settleFunding({
        asset: asset,
        markTwap: metadata.markPriceHistory.twap(interval),
        indexTwap: metadata.indexPriceHistory.twap(interval)
    });
    ...
}
```

2. `MarketLib.getImpactPriceTwap` for mark computation:

```solidity
function getImpactPriceTwap(Market storage self) internal view returns (uint256 impactPriceTwap) {
    bytes32 asset = self.asset;
    return StorageLib.loadMarketMetadata(asset).impactPriceHistory.twap(
        StorageLib.loadFundingRateEngine(asset).getFundingInterval(asset)
    );
}
```

The underlying implementation in `PriceHistoryLib.twap`:

```solidity
function twap(PriceHistory storage history, uint256 twapInterval) internal view returns (uint256) {
    uint256 idx = history.snapshots.length;
    if (idx == 0) return 0;

    PriceSnapshot memory currentSnapshot = history.snapshots[--idx];
    if (idx == 0) return currentSnapshot.price;

    uint256 targetTime = block.timestamp - twapInterval;
    uint256 timePeriod = block.timestamp - currentSnapshot.timestamp;
    uint256 elapsedTime = timePeriod;
    uint256 weightedPrice = currentSnapshot.price * timePeriod;
    uint256 previousTime = currentSnapshot.timestamp;

    while (currentSnapshot.timestamp > targetTime) {
        if (idx == 0) break;
        currentSnapshot = history.snapshots[--idx];

        if (currentSnapshot.timestamp < targetTime) {
            elapsedTime += timePeriod = previousTime - targetTime;
        } else {
            elapsedTime += timePeriod = previousTime - currentSnapshot.timestamp;
        }

        weightedPrice += currentSnapshot.price * timePeriod;
        previousTime = currentSnapshot.timestamp;
    }

    return weightedPrice / elapsedTime;
}
```

There is no check that:

* The last snapshot is recent (e.g. within a heartbeat or max age), or
* Enough observations exist within the requested window.

In `settleFunding`, `twapInterval` is set to `fundingRateEngine.getTimeSinceLastFunding()`. If a keeper misses updates and `markPriceHistory` / `indexPriceHistory` haven't received new snapshots for hours or days, `twap(interval)` will happily reuse very old prices to compute `markTwap` and `indexTwap`, driving funding and thus value transfer between longs and shorts based on stale data.

Similarly, `getImpactPriceTwap` uses `impactPriceHistory.twap(fundingInterval)` with no minimum observation cardinality or liquidity checks; if `impactPriceHistory.snapshot` hasn't been called regularly, the TWAP degenerates to the last recorded impact price regardless of its age.

Given the contest explicitly calls out **price history manipulation** as in-scope, using unboundedly stale snapshots in these TWAPs opens room for:

* Exploiting periods where keepers fail to call `setMarkPrice` / `snapshot` (funding and mark-based components become based on ancient prices);
* Disproportionate funding transfers or liquidation thresholds that do not reflect current market conditions.

This matches the `StaleOracleAcceptance` pattern: the system accepts potentially hours- or days-old `PriceSnapshot` data solely because `twap` doesn't enforce any age/heartbeat constraints.
 ### Static Signals
no max age / heartbeat on PriceHistory snapshots, twap() used with arbitrary interval, no updatedAt or recency checks before using markTwap/indexTwap/impactPriceTwap
 ### Assets at Risk
user margin, funding transfers between longs and shorts, liquidation safety margins
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: MarketLib.getIntendedMargin

 ### Title
Perps margin and price math assume 18‑decimals while USDC collateral may be 6‑decimals
 ### Description/Code Snippet
Across the perps stack, all notional and margin math is done with 1e18 fixed‑point scaling, while the only configured collateral asset is `USDC` at `Constants.USDC`, whose decimals are not queried anywhere.

In `MarketLib`, position notionals and margins are computed as:

```solidity
uint256 currentNotional = position.amount.fullMulDiv(self.markPrice, 1e18);
...
intendedMargin = currentNotional.fullMulDiv(1e18, position.leverage);
...
minMargin = currentNotional.fullMulDiv(self.getMinMarginRatio(bookType), 1e18);
...
uint256 positionNotional = positionAmount.fullMulDiv(self.markPrice, 1e18);
minOpenMargin = positionNotional.fullMulDiv(1e18, StorageLib.loadMarketSettings(self.asset).maxOpenLeverage);
```

All of these computations assume that:
- `position.amount` is in 1e18 base units,
- `markPrice` and ratios like `maintenanceMarginRatio` and `maxOpenLeverage` are 1e18‑scaled,
- therefore `currentNotional`, `intendedMargin`, `minMargin`, `minOpenMargin` are also in 1e18‑scaled **quote units**.

However, the actual collateral is USDC, with address hard‑coded in `CollateralManagerLib`:

```solidity
address constant USDC = Constants.USDC;
...
function depositFreeCollateral(..., uint256 amount) internal {
    USDC.safeTransferFrom(from, address(this), amount);
    self.creditAccount(to, amount);
}
...
function creditAccount(..., uint256 amount) internal {
    self.freeCollateral[account] += amount;
}
```

`freeCollateral` and `margin` values are stored as the raw USDC amounts passed in by users (no rescaling). In practice, standard USDC has 6 decimals; a deposit of 1 USDC is stored as `1e6`. But all margin requirements and intended margins are computed as 1e18‑scaled notional in `MarketLib`, then compared directly to `margin` in the `ClearingHouseLib` checks:

```solidity
// Liquidation check
return (margin + cache.totalUpnl) < cache.totalMinMargin.toInt256();

// Open‑margin requirement
return margin + upnl >= minOpenMargin.toInt256();

// Post‑withdrawal margin requirement
if (margin + upnl < intendedMargin.toInt256()) revert MarginRequirementUnmet();
```

No conversion is performed between the 18‑decimals notional space and the 6‑decimals collateral space. This creates a systematic **1e12 scaling mismatch** whenever the configured USDC token is 6‑decimals (or generally any non‑18‑decimals asset). As a consequence:

- Margin requirements (`minOpenMargin`, maintenance margin, intended margin) will be off by 1e12 relative to actual USDC balances.
- The protocol may accept positions that are in fact deeply undercollateralized (if USDC has 6 decimals but code treats balances as 18‑decimals), or conversely immediately liquidate safe positions if a different scaling relationship holds.
- All invariants in the docs that compare `Σ margin` and `Σ freeCollateral` against notionals implicitly assume consistent decimals; this mismatch breaks those invariants in a way that an attacker can potentially exploit by opening large positions with far less real USDC than required by the intended risk model.

Because the token address is hard‑coded and no decimals query is performed, this bug will not self‑correct if `Constants.USDC` points to a 6‑decimals asset, which is common. The fix would be to normalize all collateral and notional amounts into a single agreed scale (e.g. 1e18) using the USDC `decimals()` value, or to enforce at deployment that the configured collateral token uses 18 decimals.

This is a classic `ERC20DecimalsMismatch` pattern: **18‑decimals math for prices and notional values is mixed directly with raw ERC‑20 balances without rescaling**, leading to systematically wrong collateralization checks.

 ### Static Signals
mixes token amounts with 18-decimal math unscaled, collateral token address hard-coded, decimals never queried, margin and notional compared directly despite different implicit units
 ### Assets at Risk
user collateral, insurance fund, protocol solvency
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: MarketLib.getImpactPrice / _cacheImpactPrice / getImpactPriceTwap

 ### Title
Perps mark price & funding TWAP can be pinned via low-liquidity orderbook impact price
 ### Description/Code Snippet
The perps mark price and funding calculations depend on an `impactPrice` derived directly from the current CLOB state, without any liquidity floor or robustness checks. An attacker can cheaply manipulate this impact price (and thus its TWAP) in thin markets by placing/removing their own orders.

Key flows:

- `MarketLib.setMarkPrice` combines three components to derive the mark price:
  ```solidity
  function setMarkPrice(Market storage self, uint256 indexPrice) internal returns (uint256 markPrice) {
      MarketMetadata storage metadata = StorageLib.loadMarketMetadata(self.asset);

      _cacheBasisSpread(self, indexPrice);
      _cacheImpactPrice(self);

      uint256 p1 = self.getFundingRateComponent(indexPrice);
      uint256 p2 = (indexPrice.toInt256() + self.getBasisSpreadEMA()).toUint256();
      uint256 p3 = self.getImpactPriceTwap();

      self.markPrice = markPrice = _getMedian(p1, p2, p3);
      ...
  }
  ```

- `p3` is the **impact price TWAP**:
  ```solidity
  function getImpactPriceTwap(Market storage self) internal view returns (uint256 impactPriceTwap) {
      bytes32 asset = self.asset;
      return StorageLib.loadMarketMetadata(asset).impactPriceHistory.twap(
          StorageLib.loadFundingRateEngine(asset).getFundingInterval(asset)
      );
  }
  ```

- Snapshots for that TWAP are fed by `_cacheImpactPrice`, which uses the live orderbook with **no liquidity floor**:
  ```solidity
  function _cacheImpactPrice(Market storage self) private returns (uint256 impactPrice) {
      // impact notional is 500 * max leverage
      uint256 impactNotional =
          uint256(500e18).fullMulDiv(StorageLib.loadMarketSettings(self.asset).maxOpenLeverage, 1e18);

      impactPrice = self.getImpactPrice(impactNotional);

      StorageLib.loadMarketMetadata(self.asset).impactPriceHistory.snapshot(impactPrice);
  }

  function getImpactPrice(Market storage self, uint256 impactNotional) internal view returns (uint256 impactPrice) {
      (uint256 baseAmount, uint256 quoteUsed) = StorageLib.loadBook(self.asset).quoteBidInQuote(impactNotional);

      if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);
      uint256 impactBid = baseAmount == 0 ? 0 : impactNotional.fullMulDiv(1e18, baseAmount);

      (baseAmount, quoteUsed) = StorageLib.loadBook(self.asset).quoteAskInQuote(impactNotional);

      if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, 1);
      uint256 impactAsk = impactNotional.fullMulDiv(1e18, baseAmount);

      return (impactBid + impactAsk) / 2;
  }
  ```

  - `quoteBidInQuote` / `quoteAskInQuote` simply walk the book limits and sum fills. If the orderbook has **very little resting liquidity**, the `if (impactNotional > quoteUsed)` branches kick in and synthesize additional base amount using extreme denominators (`type(uint256).max` or `1`), making the computed impactBid/impactAsk highly sensitive to small changes in book structure.
  - There is **no minimum-liquidity requirement** or guard to ignore/weight down quotes below some depth.

- The TWAP for `impactPrice` is then computed in `PriceHistoryLib.twap` without any volume or liquidity awareness:
  ```solidity
  function twap(PriceHistory storage history, uint256 twapInterval) internal view returns (uint256) {
      uint256 idx = history.snapshots.length;
      if (idx == 0) return 0;
      PriceSnapshot memory currentSnapshot = history.snapshots[--idx];
      if (idx == 0) return currentSnapshot.price;
      uint256 targetTime = block.timestamp - twapInterval;
      ... // simple time-weighted average over snapshots
      return weightedPrice / elapsedTime;
  }
  ```

As a result, in a low-liquidity or one-sided book, an attacker can:

1. Place small, strategically priced orders on both sides of the book at or near the top of book.
2. Call (or wait for a keeper to call) `setMarkPrice`, which records a manipulated `impactPrice` via `_cacheImpactPrice`.
3. Repeat or maintain this pattern to pin the **impactPrice TWAP** over the funding interval.
4. Since the final `markPrice` is the median of `[p1 (funding component), p2 (index+EMA basis), p3 (impact TWAP)]`, controlling `p3` allows the attacker to skew the median when the other two are close or noisy.
5. This mark skew directly affects **funding payments**, liquidation thresholds, and PnL realization across all positions in that market.

Because there is **no liquidity floor, no min-volume checks, and no multiple independent price sources** for the impact component, the attacker’s own tiny orders can dominate the impactPrice calculation. On thin books this is economically cheap and highly repeatable, enabling TWAP pinning / predictable distortion of mark and funding.

This matches the TWAPWindowPinningOrLowLiquidity pattern: a TWAP or price feed is derived from a manipulable on-chain source (the CLOB) with no safeguards for depth or robustness, making it possible to skew prices/funding over the TWAP window. Assets at risk include traders’ margin (funding, liquidations, PnL) and the insurance fund when bad mark/funding induces systematic bad debt.
 ### Static Signals
impact TWAP used in mark price and funding, impact price derives from single orderbook with no liquidity floor, twapWindow = fundingInterval, no volume constraints, manipulable quoteBidInQuote/quoteAskInQuote without checks
 ### Assets at Risk
traders_margin, insurance_fund, perps_mark_price, funding_payments
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: MarketLib.getImpactPrice

 ### Title
Impact price oracle can be pinned via thin orderbook liquidity, skewing mark price and funding
 ### Description/Code Snippet
The perps mark price and funding logic relies on a CLOB-derived impact price oracle that has no liquidity floor and can be heavily skewed with very little actual liquidity, making the TWAP/EMA computations manipulable.

Key code paths:

1. `MarketLib.getImpactPrice` (perps/types/Market.sol) builds an impact price by simulating buying and selling a fixed notional (`impactNotional`) through the orderbook:

```solidity
function getImpactPrice(Market storage self, uint256 impactNotional) internal view returns (uint256 impactPrice) {
    (uint256 baseAmount, uint256 quoteUsed) = StorageLib.loadBook(self.asset).quoteBidInQuote(impactNotional);

    if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);

    uint256 impactBid = baseAmount == 0 ? 0 : impactNotional.fullMulDiv(1e18, baseAmount);

    (baseAmount, quoteUsed) = StorageLib.loadBook(self.asset).quoteAskInQuote(impactNotional);

    if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, 1);

    uint256 impactAsk = impactNotional.fullMulDiv(1e18, baseAmount);

    return (impactBid + impactAsk) / 2;
}
```

* There is **no minimum depth or liquidity floor** on the underlying book. If there is very little resting liquidity, `baseAmount` can be extremely small, causing `impactBid = impactNotional * 1e18 / baseAmount` (and similarly `impactAsk`) to explode to arbitrarily large values.
* When `impactNotional > quoteUsed` on the bid side, they attempt to adjust `baseAmount` by:

```solidity
baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);
```

  but since `1e18 / 2^256-1` underflows to ~0, this adjustment is essentially **no-op**: unfilled notional does not meaningfully increase `baseAmount`, leaving the calculated impact price dominated by whatever tiny base was available.
* On the ask side, if `impactNotional > quoteUsed`, they add `(impactNotional - quoteUsed) * 1e18 / 1`, which can make `baseAmount` enormous and force `impactAsk` arbitrarily close to 0. Together, a manipulator can generate extreme asymmetric prices with very small capital.

2. That impact price is then cached and used in the mark price and funding oracles:

```solidity
function _cacheImpactPrice(Market storage self) private returns (uint256 impactPrice) {
    // impact notional is 500 * max leverage
    uint256 impactNotional = uint256(500e18).fullMulDiv(
        StorageLib.loadMarketSettings(self.asset).maxOpenLeverage,
        1e18
    );

    impactPrice = self.getImpactPrice(impactNotional);

    StorageLib.loadMarketMetadata(self.asset).impactPriceHistory.snapshot(impactPrice);
}

function getImpactPriceTwap(Market storage self) internal view returns (uint256 impactPriceTwap) {
    bytes32 asset = self.asset;
    return StorageLib.loadMarketMetadata(asset).impactPriceHistory.twap(
        StorageLib.loadFundingRateEngine(asset).getFundingInterval(asset)
    );
}

function setMarkPrice(Market storage self, uint256 indexPrice) internal returns (uint256 markPrice) {
    MarketMetadata storage metadata = StorageLib.loadMarketMetadata(self.asset);

    _cacheBasisSpread(self, indexPrice);
    _cacheImpactPrice(self);

    uint256 p1 = self.getFundingRateComponent(indexPrice);
    uint256 p2 = (indexPrice.toInt256() + self.getBasisSpreadEMA()).toUint256();
    uint256 p3 = self.getImpactPriceTwap();

    self.markPrice = markPrice = _getMedian(p1, p2, p3);

    metadata.markPriceHistory.snapshot(markPrice);
    metadata.indexPriceHistory.snapshot(indexPrice);
}
```

* `_cacheImpactPrice` calls `getImpactPrice` every time `setMarkPrice` is invoked, then snapshots it into `impactPriceHistory`.
* `getImpactPriceTwap` takes a TWAP over this history using a window equal to the funding interval. However, there is **no minimum observation count or liquidity check**; TWAP is computed directly over whatever snapshots exist, even if they are derived from extremely thin, easily manipulable books.
* Finally, the *mark price* used for PnL, liquidations and funding is the median of three components: `p1` (funding-based), `p2` (index plus basis spread EMA), and `p3` (impactPrice TWAP). By pushing `p3` to an extreme value via a few carefully placed/matched orders, an attacker can materially move the median mark price if the other two components are near the current index.

This matches the **TWAPWindowPinningOrLowLiquidity** pattern:

* Oracle is derived from a single CLOB with **no liquidity floor** and no validation of depth or orderbook health.
* The effective window length is `fundingInterval`, but there is no enforcement of a robust, high-cardinality price history; a few snapshots can dominate.
* A motivated attacker can:
  1. Place a small amount of resting liquidity at extreme prices to make `quoteBidInQuote` or `quoteAskInQuote` return very skewed `baseAmount`.
  2. Trigger `setMarkPrice` (via keeper/front-end) to snapshot this manipulated `impactPrice` and feed the TWAP.
  3. Repeat around funding events to bias mark price/funding in a profitable direction, with relatively small cost compared to the impact on collateral requirements and PnL of other traders.

Because this oracle is used directly to compute the mark price and thus liquidation thresholds and funding, this low-liquidity manipulability can produce **systematic value transfer** between traders (for example, manipulating mark up before short liquidation sweeps, or shifting funding in favor of a large position), especially in early or low-volume markets.

While some amount of oracle manipulation risk is inherent in orderbook-based pricing for illiquid markets, the combination of:

* arithmetic bugs around the fallback `quoteUsed < impactNotional` branches,
* lack of liquidity floors or min-depth checks,
* and unguarded TWAP construction

makes this significantly easier and cheaper to exploit than a typical robust CLOB-based impact oracle.

 ### Static Signals
uses single CLOB as price source, no liquidity floor / depth check, twapWindow = fundingInterval; no cardinality enforcement, fallback path when quoteUsed < impactNotional uses effectively-zero adjustment, markPrice median includes impactPriceTwap
 ### Assets at Risk
user margin, liquidation outcomes, funding transfers, insurance fund
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: MarketLib.getImpactPrice

 ### Title
Asymmetric impact price math leads to large, manipulable mark price skew
 ### Description/Code Snippet
In `MarketLib.getImpactPrice`, the protocol computes a synthetic impact price by querying the orderbook in quote terms on both sides:

```solidity
function getImpactPrice(Market storage self, uint256 impactNotional) internal view returns (uint256 impactPrice) {
    (uint256 baseAmount, uint256 quoteUsed) = StorageLib.loadBook(self.asset).quoteBidInQuote(impactNotional);

    if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);

    uint256 impactBid = baseAmount == 0 ? 0 : impactNotional.fullMulDiv(1e18, baseAmount);

    (baseAmount, quoteUsed) = StorageLib.loadBook(self.asset).quoteAskInQuote(impactNotional);

    if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, 1);

    uint256 impactAsk = impactNotional.fullMulDiv(1e18, baseAmount);

    return (impactBid + impactAsk) / 2;
}
```

The handling of the case `impactNotional > quoteUsed` is **asymmetric and dimensionally inconsistent**:
- On the bid side: `baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);` — this adds a negligible `≈ (impactNotional-quoteUsed)/1e40` amount of base (effectively no adjustment).
- On the ask side: `baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, 1);` — this adds `(impactNotional-quoteUsed) * 1e18` base, which can be astronomically larger than the real base amount.

As a result, when the book is thin such that `quoteUsed < impactNotional`, the computed `impactAsk` becomes artificially **small** (huge base, same quote), and the final `impactPrice` (average of impactBid and impactAsk) can be pulled far away from the true executable price. This value feeds directly into mark price calculation:

```solidity
function setMarkPrice(Market storage self, uint256 indexPrice) internal returns (uint256 markPrice) {
    _cacheBasisSpread(self, indexPrice);
    _cacheImpactPrice(self);

    uint256 p1 = self.getFundingRateComponent(indexPrice);
    uint256 p2 = (indexPrice.toInt256() + self.getBasisSpreadEMA()).toUint256();
    uint256 p3 = self.getImpactPriceTwap();

    self.markPrice = markPrice = _getMedian(p1, p2, p3);
}

function _cacheImpactPrice(Market storage self) private returns (uint256 impactPrice) {
    uint256 impactNotional = uint256(500e18).fullMulDiv(StorageLib.loadMarketSettings(self.asset).maxOpenLeverage, 1e18);
    impactPrice = self.getImpactPrice(impactNotional);
    StorageLib.loadMarketMetadata(self.asset).impactPriceHistory.snapshot(impactPrice);
}
```

Because `impactPrice` is recorded into `impactPriceHistory` and used as one of the three components for mark price, this rounding/error asymmetry can:
- systematically bias the mark price toward the attacker’s preferred direction in low-liquidity states,
- influence liquidations and maintenance margin checks (which use mark price),
- affect funding calculations that depend on mark vs index.

The core issue matches **PricePrecisionOrRoundingError**: incorrect scaling and inconsistent numerator/denominator use (`fullMulDiv(1e18, type(uint256).max)` vs `fullMulDiv(1e18, 1)`) introduce a huge exploitable distortion in an internal price used for risk.

 ### Static Signals
divide-before-multiply style scaling via fullMulDiv, inconsistent denominators: type(uint256).max vs 1, impact price feeds into markPrice and liquidations, thin-book path (impactNotional > quoteUsed) uses special-case math
 ### Assets at Risk
user margin, insurance fund, liquidation correctness, funding payments
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: ClearingHouseLib.isLiquidatable

 ### Title
Perps account operations iterate over unbounded asset lists, enabling gas-based DoS
 ### Description/Code Snippet
Several core risk functions in the perps ClearingHouse iterate over the full set of markets (`assets`) an account has exposure to, with **no explicit bound per account**. The size of this set is user-controlled: a trader can open tiny positions on many markets (all with `crossMarginEnabled = true`), inflating `assets.length` for their subaccount.

Key locations:

- `ClearingHouseLib.getAccount` / `getAccountAndMargin`:
  ```solidity
  function getAccount(ClearingHouse storage self, address account, uint256 subaccount)
      internal
      view
      returns (DynamicArrayLib.DynamicArray memory assets, Position[] memory positions)
  {
      assets = self.assets[account][subaccount].values().wrap();
      positions = _getPositions(self, assets, account, subaccount, false);
  }
  ```
  `values()` on an `EnumerableSet` allocates and copies all asset ids for that account.

- `ClearingHouseLib.isLiquidatable` (used in liquidations and deleverage flows):
  ```solidity
  function isLiquidatable(
      ClearingHouse storage self,
      DynamicArrayLib.DynamicArray memory assets,
      Position[] memory positions,
      int256 margin,
      BookType bookType
  ) internal view returns (bool liquidatable) {
      __LiquidatableCheckCache__ memory cache;
      for (uint256 i; i < assets.length(); ++i) {
          (cache.upnl, cache.minMargin) =
              self.market[assets.getBytes32(i)].getUpnlAndMinMargin(positions[i], bookType);

          cache.totalUpnl += cache.upnl;
          cache.totalMinMargin += cache.minMargin;
      }
      // ...
  }
  ```

- `ClearingHouseLib.rebalanceAccount` → `rebalanceOpen` / `rebalanceClose` → `_getIntendedMarginAndUpnl`:
  ```solidity
  function _getIntendedMarginAndUpnl(
      ClearingHouse storage self,
      DynamicArrayLib.DynamicArray memory assets,
      Position[] memory positions
  ) internal view returns (uint256 totalIntendedMargin, int256 totalUpnl) {
      uint256 length = assets.length();
      uint256 intendedMargin;
      int256 upnl;
      for (uint256 i; i < length; ++i) {
          (intendedMargin, upnl) = self.market[assets.getBytes32(i)].getIntendedMarginAndUpnl(positions[i]);
          totalIntendedMargin += intendedMargin;
          totalUpnl += upnl;
      }
  }
  ```

- Similar unbounded loops appear in `getUpnl`, `getIntendedMargin`, `getNotionalAccountValue`, `hasBadDebt`, and margin add/remove flows (`addMargin`, `removeMargin` in `PerpManager` call into these helpers).

There is **no explicit cap** on how many assets a single subaccount can hold positions in, beyond the global number of listed markets. As more markets are added, a malicious user can:

1. Open dust-sized positions on as many markets as allowed (all with `crossMarginEnabled = true`), thereby inserting each asset into `self.assets[account][subaccount]`:
   ```solidity
   self.assets[account][subaccount].add(asset);
   ```
2. This causes every liquidation, margin update, leverage change, or risk check for that subaccount to iterate over an increasingly large `assets` array.
3. Eventually, gas usage for functions like `isLiquidatable`, `setPositionLeverage`, `addMargin`, `removeMargin`, and liquidation paths can exceed the block gas limit or typical gas budgets, making:
   - Liquidations of that account infeasible (account becomes effectively unliquidatable).
   - Admin/liquidator functions that must process this account revert due to OOG.

Because bad-debt handling relies on actually executing these loops to compute total `upnl` and margin requirements, an attacker can create an account that is:

- Cross-margined
- Potentially deeply negative in equity
- Yet practically **un-liquidatable** due to gas exhaustion when evaluating `isLiquidatable` and rebalancing logic.

This is a classic "unbounded loop over user-controlled state" pattern in a hot path (liquidations and margin checks). It threatens the key invariant that bad positions can always be liquidated and that insurance fund / system solvency can be defended in a timely manner.

Mitigations typically include:
- Hard-capping the number of markets per subaccount (e.g. via a `maxAssetsPerSubaccount` setting enforced in `_assetCanBeAddedToAccount`).
- Introducing paginated / partial liquidation and risk-check mechanisms that only scan a bounded subset of assets per call.
- Or maintaining aggregated per-subaccount risk metrics that can be updated incrementally on each trade instead of recomputed across all markets.
 ### Static Signals
loops over assets.length() for a single account, assets set is user-controlled via opening many small positions, no explicit cap on number of markets per subaccount, same unbounded loop used in liquidation and margin checks
 ### Assets at Risk
insurance fund, protocol solvency, ability to liquidate undercollateralized accounts
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: MarketLib.getImpactPriceTwap

 ### Title
Funding and mark price rely on TWAP/impact prices from low-liquidity CLOB without safeguards
 ### Description/Code Snippet
The perp funding and mark-price system relies on orderbook-derived impact prices and TWAPs without any liquidity or window-safety checks, which allows a permissionless attacker to manipulate funding and mark pricing by briefly spoofing the book.

Key flows:

1. **Impact price snapshot and TWAP**

```solidity
function _cacheImpactPrice(Market storage self) private returns (uint256 impactPrice) {
    // impact notional is 500 * max leverage
    uint256 impactNotional =
        uint256(500e18).fullMulDiv(StorageLib.loadMarketSettings(self.asset).maxOpenLeverage, 1e18);

    impactPrice = self.getImpactPrice(impactNotional);

    StorageLib.loadMarketMetadata(self.asset).impactPriceHistory.snapshot(impactPrice);
}

function getImpactPrice(Market storage self, uint256 impactNotional) internal view returns (uint256 impactPrice) {
    (uint256 baseAmount, uint256 quoteUsed) = StorageLib.loadBook(self.asset).quoteBidInQuote(impactNotional);

    if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);

    uint256 impactBid = baseAmount == 0 ? 0 : impactNotional.fullMulDiv(1e18, baseAmount);

    (baseAmount, quoteUsed) = StorageLib.loadBook(self.asset).quoteAskInQuote(impactNotional);

    if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, 1);

    uint256 impactAsk = impactNotional.fullMulDiv(1e18, baseAmount);

    return (impactBid + impactAsk) / 2;
}

function getImpactPriceTwap(Market storage self) internal view returns (uint256 impactPriceTwap) {
    bytes32 asset = self.asset;
    return StorageLib.loadMarketMetadata(asset).impactPriceHistory.twap(
        StorageLib.loadFundingRateEngine(asset).getFundingInterval(asset)
    );
}
```

* `_cacheImpactPrice` is called from `setMarkPrice` to snapshot an **impact price** into `impactPriceHistory`.
* `getImpactPrice` computes impact bid/ask by walking the CLOB via `quoteBidInQuote` / `quoteAskInQuote`. These functions traverse price levels until `impactNotional` is consumed, but **no minimum-liquidity or depth requirement is enforced**.
* If the book is thin, the logic handling `impactNotional > quoteUsed` uses `fullMulDiv(1e18, type(uint256).max)` or `fullMulDiv(1e18, 1)`, which can blow up the effective price when book liquidity is insufficient.
* `getImpactPriceTwap` later TWAPs these potentially manipulated snapshots over a window equal to `fundingRateEngine.getFundingInterval(asset)`.

2. **Mark price and funding depend on these values**

```solidity
function setMarkPrice(Market storage self, uint256 indexPrice) internal returns (uint256 markPrice) {
    MarketMetadata storage metadata = StorageLib.loadMarketMetadata(self.asset);

    _cacheBasisSpread(self, indexPrice);
    _cacheImpactPrice(self);

    uint256 p1 = self.getFundingRateComponent(indexPrice);
    uint256 p2 = (indexPrice.toInt256() + self.getBasisSpreadEMA()).toUint256();
    uint256 p3 = self.getImpactPriceTwap();

    self.markPrice = markPrice = _getMedian(p1, p2, p3);
    ...
}

function getFundingRateComponent(Market storage self, uint256 indexPrice)
    internal
    view
    returns (uint256 fundingRateComponent)
{
    bytes32 asset = self.asset;
    FundingRateEngine storage fundingRateEngine = StorageLib.loadFundingRateEngine(asset);

    return indexPrice.fullMulDiv(
        1e18
            + fundingRateEngine.fundingRate.abs().fullMulDiv(
                block.timestamp - fundingRateEngine.lastFundingTime, fundingRateEngine.getFundingInterval(asset)
            ),
        1e18
    );
}

function settleFunding(Market storage self) internal {
    bytes32 asset = self.asset;

    FundingRateEngine storage fundingRateEngine = StorageLib.loadFundingRateEngine(asset);
    MarketMetadata storage metadata = StorageLib.loadMarketMetadata(asset);

    uint256 interval = fundingRateEngine.getTimeSinceLastFunding();

    (int256 funding, int256 cumulativeFunding) = fundingRateEngine.settleFunding({
        asset: asset,
        markTwap: metadata.markPriceHistory.twap(interval),
        indexTwap: metadata.indexPriceHistory.twap(interval)
    });
    ...
}
```

* The mark price is the median of three components: `p1` (funding-rate adjusted index), `p2` (index plus basis spread EMA), and `p3` (impact price TWAP). So **p3 directly influences mark**.
* Funding settlement uses a TWAP of `markPriceHistory` and `indexPriceHistory` over `interval = getTimeSinceLastFunding()`. Since mark price itself is influenced by impact-price TWAP, any manipulation in `impactPriceHistory` bleeds into both mark and funding.

3. **No guards on TWAP window size or liquidity**

* `FundingRateSettings.fundingInterval` is configurable and directly used as the TWAP window for `impactPriceHistory`. There are **no bounds enforced** (e.g., minimum 10–30 minutes) and no checks that there are enough snapshots or that they span a minimum time range.
* `PriceHistory.twap` walks backwards through snapshots until timestamps fall before `targetTime = now - twapInterval`. In a low-activity market with few snapshots, an attacker can:
  * place and cancel a few small orders to dominate impact price,
  * ensure one or a few manipulated snapshots occur right before the keeper calls `setMarkPrice` / `settleFunding`,
  * make these snapshots dominate the TWAP (because old snapshots either don’t exist or are outside the window).

4. **Realistic manipulation scenario**

*Assumptions*: The market’s CLOB is thin (common for new or illiquid perps), and `fundingInterval` is relatively short (e.g., a few minutes).

1. Attacker builds a position (long or short) in the perp market.
2. Right before a keeper calls `setMarkPrice` and/or `settleFunding`, attacker:
   * posts aggressive quotes at extreme prices with very small sizes such that `quoteBidInQuote` / `quoteAskInQuote` consume only these spoofed orders for `impactNotional` (or return very low `quoteUsed` so the `impactNotional > quoteUsed` path is hit),
   * causing `getImpactPrice` to produce abnormally high or low impact prices.
3. Keeper calls `setMarkPrice` once or a few times in this manipulated state, snapshotting distorted impact prices.
4. Attacker cancels spoofed orders; book returns to normal; but **historical impact price snapshots remain skewed**, so `getImpactPriceTwap` and `markPriceHistory.twap` in the next `settleFunding` call are still biased.
5. Attacker profits via:
   * favorable funding payments (receive funding, avoid paying),
   * potentially mark-based margin/liquidation thresholds being temporarily skewed.

This matches the **TWAPWindowPinningOrLowLiquidity** pattern: a TWAP / oracle reading derived from a thin, manipulable CLOB with too-short and unbounded windows, no liquidity floor, and no safeguards on observation cardinality.

5. **Why this is exploitable in this protocol context**

* The manipulation is **permissionless**: any trader can place CLOB orders and call matching functions; they do not need admin or keeper roles.
* Even if keeper calls `setMarkPrice` deterministically (e.g. timed), attackers can front-run or back-run those calls on a high-throughput L2, pinning the TWAP window.
* Funding is designed to transfer value between longs and shorts based on mark-index deviations. Skewing mark/impact TWAP directly translates to **systematic value transfer**, which is economically material.
* There is no safeguard such as:
  * requiring a minimum notional depth when computing `getImpactPrice`,
  * ignoring extreme `impactPrice` values or clamping relative to index/mark,
  * requiring a minimum number of snapshots / minimum window length before trusting TWAP,
  * separate, robust oracle for funding vs. thin-book impact prices.

This is a protocol-specific instantiation of low-liquidity TWAP oracle abuse rather than generic DEX oracle risk, and directly affects perpetual funding and liquidation behaviour.
 ### Static Signals
impact price derived from orderbook without liquidity floor, impactPriceHistory.snapshot called every mark update, twap(window = fundingInterval) with no min window / cardinality, mark price median includes impactPriceTwap, funding uses mark/index TWAPs
 ### Assets at Risk
perpetual trader PnL, insurance fund, liquidation safety margins
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: MarketLib.setMarkPrice

 ### Title
Perps mark price and funding use TWAPs with no freshness or liquidity checks
 ### Description/Code Snippet
The perpetuals `MarketLib` relies on several TWAP-based prices derived from on-chain history without any freshness or liquidity safeguards, making them manipulable or stale.

Key code paths:

1. **Funding settlement uses potentially stale TWAPs**
```solidity
function settleFunding(Market storage self) internal {
    bytes32 asset = self.asset;

    FundingRateEngine storage fundingRateEngine = StorageLib.loadFundingRateEngine(asset);
    MarketMetadata storage metadata = StorageLib.loadMarketMetadata(asset);

    uint256 interval = fundingRateEngine.getTimeSinceLastFunding();

    (int256 funding, int256 cumulativeFunding) = fundingRateEngine.settleFunding({
        asset: asset,
        markTwap: metadata.markPriceHistory.twap(interval),
        indexTwap: metadata.indexPriceHistory.twap(interval)
    });
    ...
}
```
`PriceHistory.twap` simply walks backwards through snapshots until `timestamp > block.timestamp - twapInterval`, but it **never checks** that the last snapshot is recent, nor that there are enough observations:
```solidity
function twap(PriceHistory storage history, uint256 twapInterval) internal view returns (uint256) {
    uint256 idx = history.snapshots.length;
    if (idx == 0) return 0;

    PriceSnapshot memory currentSnapshot = history.snapshots[--idx];
    if (idx == 0) return currentSnapshot.price;

    uint256 targetTime = block.timestamp - twapInterval;
    ... // iterate backwards, but no age/heartbeat check
}
```
If keepers stop calling `setMarkPrice` or if the underlying `indexPrice` feed (set by AdminPanel) stops updating, the TWAP will still return an average over **very old** prices. `FundingLib.settleFunding` will then compute funding using those stale `markTwap` and `indexTwap` values without any `updatedAt`/max-age/heartbeat constraints.

2. **Mark price depends on impact-price TWAP from thin books**
```solidity
function setMarkPrice(Market storage self, uint256 indexPrice) internal returns (uint256 markPrice) {
    MarketMetadata storage metadata = StorageLib.loadMarketMetadata(self.asset);

    _cacheBasisSpread(self, indexPrice);
    _cacheImpactPrice(self);

    uint256 p1 = self.getFundingRateComponent(indexPrice);
    uint256 p2 = (indexPrice.toInt256() + self.getBasisSpreadEMA()).toUint256();
    uint256 p3 = self.getImpactPriceTwap();

    self.markPrice = markPrice = _getMedian(p1, p2, p3);
    ...
}

function getImpactPriceTwap(Market storage self) internal view returns (uint256 impactPriceTwap) {
    bytes32 asset = self.asset;
    return StorageLib.loadMarketMetadata(asset).impactPriceHistory.twap(
        StorageLib.loadFundingRateEngine(asset).getFundingInterval(asset)
    );
}

function _cacheImpactPrice(Market storage self) private returns (uint256 impactPrice) {
    // impact notional is 500 * max leverage
    uint256 impactNotional = uint256(500e18).fullMulDiv(StorageLib.loadMarketSettings(self.asset).maxOpenLeverage, 1e18);
    impactPrice = self.getImpactPrice(impactNotional);
    StorageLib.loadMarketMetadata(self.asset).impactPriceHistory.snapshot(impactPrice);
}
```
`getImpactPrice` walks the on-chain orderbook via `quoteBidInQuote` / `quoteAskInQuote` from `BookLib`:
```solidity
function getImpactPrice(Market storage self, uint256 impactNotional) internal view returns (uint256 impactPrice) {
    (uint256 baseAmount, uint256 quoteUsed) = StorageLib.loadBook(self.asset).quoteBidInQuote(impactNotional);
    if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);
    uint256 impactBid = baseAmount == 0 ? 0 : impactNotional.fullMulDiv(1e18, baseAmount);

    (baseAmount, quoteUsed) = StorageLib.loadBook(self.asset).quoteAskInQuote(impactNotional);
    if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, 1);
    uint256 impactAsk = impactNotional.fullMulDiv(1e18, baseAmount);

    return (impactBid + impactAsk) / 2;
}
```
There are **no liquidity floors, observation cardinality checks, or minimum-depth constraints**. In a thin market:
* An attacker can place a few tiny orders at extreme prices so that `quoteBidInQuote` / `quoteAskInQuote` consume only a small `quoteUsed`, then the remainder of `impactNotional` is effectively priced at ~0 or an extreme implied rate.
* Those distorted impact prices are snapshotted into `impactPriceHistory` and then fed into `impactPriceHistory.twap(...)`.
* Because `setMarkPrice` takes the median of `[funding component, index+basis EMA, impact TWAP]`, a malicious actor can, with enough coordination on the orderbook, pull two of the three (basis & impact) in the same direction, causing mark price and hence liquidation thresholds and PnL to move significantly.

3. **Basis spread EMA over unbounded history**
```solidity
function _cacheBasisSpread(Market storage self, uint256 indexPrice) private {
    uint256 midPrice = self.getMidPrice();
    if (midPrice == 0) return;
    int256 basisSpread = midPrice.toInt256() - indexPrice.toInt256();
    StorageLib.loadMarketMetadata(self.asset).basisSpreadHistory.snapshotBasisSpread(basisSpread);
}

function getBasisSpreadEMA(Market storage self) internal view returns (int256 basisSpreadEMA) {
    return StorageLib.loadMarketMetadata(self.asset).basisSpreadHistory.ema(15 minutes);
}
```
`basisSpreadHistory.snapshotBasisSpread` stores a sequence of `(basisSpread, timestamp=0)` entries, and `ema(15 minutes)` simply uses the *last N entries* regardless of real time. There is no enforcement that these snapshots were taken over a 15-minute wall-clock window, nor any constraint on observation spacing. A short-lived manipulation of the mid-price followed by a flurry of `_cacheBasisSpread` calls can bias the EMA for a long time.

**Why this matches the pattern**

* **StaleOracleAcceptance** – Funding settlement uses `markPriceHistory.twap(interval)` and `indexPriceHistory.twap(interval)` with **no maximum-age or heartbeat check** on the last snapshot. If mark/index prices are not updated for a long time, future settlements will use effectively stale prices, causing mispriced funding and possible unfair transfers between longs and shorts.
* **TWAPWindowPinningOrLowLiquidity** – Mark price computation weighs an impact-price TWAP based on `getImpactPrice`, which in turn reads directly from the orderbook with no liquidity floor or minimum depth. In a thin market, an attacker can cheaply position tiny orders at extreme prices and call `setMarkPrice` enough times to pin both `impactPriceHistory` and `basisSpreadHistory` in a favorable direction. Because `setMarkPrice` uses the median of three components, once two are biased the mark price — and thus liquidation conditions — become attacker-controlled until honest activity builds sufficient history.

**Impact sketch**

* An undercollateralized long/short can be kept alive artificially by pinning mark price closer to their entry level, preventing liquidation while the real market has moved.
* Conversely, an attacker can push mark price beyond fair value to prematurely liquidate victims (who have no control over mark computation) and profit as liquidator.
* Funding payments become unfair: stale or manipulated TWAPs can systematically transfer value from one side of the book to the other over many intervals.

There are no guards such as:
* `require(block.timestamp - lastSnapshot <= maxAge)` before using TWAPs,
* enforcing a minimum number of distinct-time snapshots over the requested interval,
* a liquidity threshold check on `getImpactPrice`.

These missing checks make the price observables stale- or manipulation-prone, which cascades into funding, mark price, and liquidation logic.

 ### Static Signals
twap() used without max-age or heartbeat, impact price oracle from single CLOB without liquidity floor, ema(period) over N entries with no time-spacing guarantees, no min observation cardinality enforced, mark price derived from manipulable TWAP components
 ### Assets at Risk
user margin balances, insurance fund, perps traders PnL, liquidation safety
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: MarketLib.setMarkPrice

 ### Title
Orderbook-based impact price oracle lets traders skew mark price and liquidations
 ### Description/Code Snippet
The perp mark price for a market is partially derived from a DEX-style spot orderbook without sufficient robustness, making it manipulable by any trader and directly affecting margin checks, liquidation thresholds, and funding.

Key flow:
- `MarketLib.setMarkPrice()` computes three candidate prices and sets `self.markPrice` to their median:
  - `p1 = self.getFundingRateComponent(indexPrice)` (index price with funding adjustment)
  - `p2 = (indexPrice.toInt256() + self.getBasisSpreadEMA()).toUint256()` (index price plus basis-EMA)
  - `p3 = self.getImpactPriceTwap()` (TWAP of impact price derived from the orderbook)
- `p3` is computed via:
  - `_cacheImpactPrice()` → `impactPrice = self.getImpactPrice(impactNotional)` then `impactPriceHistory.snapshot(impactPrice)`
  - `getImpactPriceTwap()` → `impactPriceHistory.twap(StorageLib.loadFundingRateEngine(asset).getFundingInterval(asset))`
- `getImpactPrice()` is purely orderbook-based:
```solidity
function getImpactPrice(Market storage self, uint256 impactNotional) internal view returns (uint256 impactPrice) {
    (uint256 baseAmount, uint256 quoteUsed) = StorageLib.loadBook(self.asset).quoteBidInQuote(impactNotional);

    if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);

    uint256 impactBid = baseAmount == 0 ? 0 : impactNotional.fullMulDiv(1e18, baseAmount);

    (baseAmount, quoteUsed) = StorageLib.loadBook(self.asset).quoteAskInQuote(impactNotional);

    if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, 1);

    uint256 impactAsk = impactNotional.fullMulDiv(1e18, baseAmount);

    return (impactBid + impactAsk) / 2;
}
```
- This uses **spot orderbook depth only**, with no checks on:
  - minimum liquidity or minimum number of levels filled,
  - maximum allowed slippage from an external anchor (index price),
  - adversarial order placement/cancellation patterns.
- When the orderbook has insufficient depth (`quoteUsed < impactNotional`), fallback branches use extreme denominators:
  - For bids: `(impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max)`, effectively adding ~0 to base and forcing an extremely **high** impactBid.
  - For asks: `(impactNotional - quoteUsed).fullMulDiv(1e18, 1)`, adding a very large base amount and forcing an extremely **low** impactAsk.
- `impactPrice` is the average of these two and then used in the median; under asymmetric liquidity or skewed books, it can be far from any reasonable fair price.

`impactPriceHistory.twap(...)` offers only minimal protection:
- `PriceHistory.twap()`:
```solidity
function twap(PriceHistory storage history, uint256 twapInterval) internal view returns (uint256) {
    uint256 idx = history.snapshots.length;
    if (idx == 0) return 0;
    PriceSnapshot memory currentSnapshot = history.snapshots[--idx];
    if (idx == 0) return currentSnapshot.price;
    // ... walk backwards until targetTime ...
}
```
- If only one snapshot exists (common just after market start or after long inactivity), it **returns that single value regardless of its age or how it was obtained**.
- The TWAP interval is taken from the *funding engine* (`getFundingInterval`), which is configurable and not enforced to be long (e.g. can be much shorter than the usual 10–30 minutes standard). That makes even a 'TWAP' susceptible to short-lived manipulations.

Implications:
- Mark price is directly used in:
  - `MarketLib.getUpnl` / `getUpnlAndMinMargin` → PnL and maintenance margin requirements.
  - `getMinOpenMargin`, `getMaintenanceMargin`, `getMaxDivergingBidPrice`/`AskPrice`.
  - Liquidation logic via `ClearingHouseLib.isLiquidatable` and liquidation paths in `LiquidatorPanel` (not shown here but part of the same system).
- An attacker can:
  - Place or cancel large, off-market orders (or exploit thin books) to alter `quoteBidInQuote` / `quoteAskInQuote` behaviour.
  - Cause `impactNotional > quoteUsed` on one or both sides, hitting the extreme branches that push `impactBid` and `impactAsk` to pathological values, especially when `impactNotional` is large (`impactNotional` is fixed to `500e18 * maxOpenLeverage / 1e18`).
  - Do this repeatedly across blocks to push a sequence of snapshots such that:
    - `impactPriceTwap` drifts significantly from the index price even if the external index is honest,
    - the median of `(p1, p2, p3)` follows the manipulated `p3` in regimes where `p1` (funding-adjusted) and `p2` (index plus EMA) are close.
- Because the keeper just calls `setMarkPrice(indexPrice)` according to spec (using a trusted index), an attacker **does not need privileged access**—only the ability to post/cancel orders—to gradually or abruptly move mark price.
- This can cause:
  - **Forced liquidations** of honest users whose positions are healthy under the true market price but appear undercollateralized under the skewed mark.
  - **Prevention of liquidations** (by moving mark in the opposite direction) so that underwater accounts remain open and accrue bad debt until eventually socialized or pushed onto the insurance fund.
  - **Funding payment extraction** by keeping mark systematically above/below index to farm funding from the other side.

There are no explicit countermeasures such as:
- Requiring a minimum depth or quoteUsed/impactNotional ratio before impact pricing is trusted,
- Bounding `impactPrice` vs index price (e.g. rejecting >X % divergence),
- Ignoring pathological branches where the book cannot satisfy the `impactNotional`,
- Multi-oracle consensus between index and impact or a separate external TWAP.

Given that mark price is *intended* to be robust, but is actually derived from a manipulable orderbook oracle with weak safeguards, this fits the `OracleUsingDEXorTWAP` pattern: a critical oracle (mark price) is based on a DEX/orderbook snapshot/TWAP that can be skewed by active traders, enabling unfair liquidations, bad-debt creation, or funding manipulation under realistic trading conditions.
 ### Static Signals
mark price uses median of three components including orderbook-derived impact price, getImpactPrice reads only on-chain orderbook state (quoteBidInQuote/quoteAskInQuote), impactNotional fixed as 500e18 * maxOpenLeverage / 1e18, if impactNotional > quoteUsed uses extreme denominators (type(uint256).max or 1), PriceHistory.twap returns last snapshot if only one, regardless of age, TWAP interval taken from configurable fundingInterval, no enforced minimum duration, no liquidity / depth / deviation checks between impact price and index price, markPrice used for upnl, maintenance margin, open margin, and liquidation checks
 ### Assets at Risk
perpetuals traders’ margin and collateral, insuranceFund balance, overall system solvency (bad debt via manipulated mark)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

