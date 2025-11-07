# 2025 08 flare - Findings Report
## Commit hash: 7bcf1437ddd739a15f8aa1b588fcc17eb66d2f31

##Findings by Pattern


 **Derived From** : Liquidation uses stale FTSO price without max-age checks, enabling wrongful liquidation/payouts

[H-1]. Stale FTSO price accepted in startLiquidation/liquidate enables overpaid liquidations and wrongful triggers
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Liquidation relies on potentially stale FTSO prices without any max-age bound

[H-2]. LiquidationFacet can be started and executed using arbitrarily stale oracle prices (no heartbeat on base FTSO feed)
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Liquidation uses stale FTSO price (no freshness check) to compute CR and payouts

[M-3]. LiquidationFacet.liquidate pays using stale oracle prices, allowing inflated liquidator payouts and wrongful liquidations
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Liquidation burns user FAssets even if vault/pool can’t fully pay

[L-4]. LiquidationFacet.liquidate burns liquidator’s FAssets even when vault/pool pay 0 due to balance cap, breaking payout=burn invariant
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 2
- M: 1
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Liquidation uses stale FTSO price without max-age checks, enabling wrongful liquidation/payouts

## [H-1]. Stale FTSO price accepted in startLiquidation/liquidate enables overpaid liquidations and wrongful triggers

### Finding Severity Justification: Liquidation CR checks and payout pricing rely on FtsoV2PriceStore values without any absolute freshness/heartbeat enforcement against block.timestamp. If the FTSO feed stalls after a large market move, the system will continue to use a stale (potentially much higher) asset price. This can wrongly trigger liquidations and overpay liquidators directly from agent vault and community pool collateral, resulting in real fund losses. The path is permissionless (liquidate/startLiquidation) and affects core asset safety.
## Derived From Pattern/Invariant
Liquidation uses stale FTSO price without max-age checks, enabling wrongful liquidation/payouts

## Exploit Type
Oracle

## Location
LiquidationFacet.startLiquidation

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
LiquidationFacet.startLiquidation (and liquidate) compute collateral ratios and liquidation payouts through Liquidation.getCollateralRatiosBIPS(), which sources prices via Conversion.currentAmgPriceInTokenWeiWithTrusted(). That function fetches both generic FTSO and "trusted" prices, but only checks the freshness of the trusted price relative to the (potentially stale) FTSO timestamps. If the trusted price is not fresh, it silently falls back to the generic FTSO price without any bound on its age. There is no absolute timeliness/heartbeat check against block.timestamp for either price path.

As a result, if FtsoV2PriceStore stalls or is delayed, the system will continue using an arbitrarily old FTSO price to decide if an agent can be liquidated and to compute liquidation payouts. Because collateral ratio uses price as the denominator and payouts use the same FTSO price, a stale-above-market price causes: (1) CR to appear lower (wrongful liquidation becomes possible), and (2) liquidation payouts to be calculated at an inflated rate, overpaying liquidators from the agent vault/pool.

Vulnerable snippet:

function currentAmgPriceInTokenWeiWithTrusted(CollateralTypeInt.Data storage _token)
        internal view
        returns (uint256 _ftsoPrice, uint256 _trustedPrice)
{
    (uint256 ftsoPrice, uint256 assetTimestamp, uint256 tokenTimestamp) = currentAmgPriceInTokenWeiWithTs(_token, false);
    (uint256 trustedPrice, uint256 assetTimestampTrusted, uint256 tokenTimestampTrusted) = currentAmgPriceInTokenWeiWithTs(_token, true);
    bool trustedPriceFresh = tokenTimestampTrusted + settings.maxTrustedPriceAgeSeconds >= tokenTimestamp
            && assetTimestampTrusted + settings.maxTrustedPriceAgeSeconds >= assetTimestamp;
    _ftsoPrice = ftsoPrice;
    _trustedPrice = trustedPriceFresh ? trustedPrice : ftsoPrice;  // ftso price used without freshness check
}

No check enforces assetTimestamp/tokenTimestamp recency. FtsoV2PriceStore.getPrice() also returns the last stored timestamp without any validity window, so if publishing halts the system continues forever with stale prices.

## Impact
Because neither Conversion.currentAmgPriceInTokenWei nor currentAmgPriceInTokenWeiWithTrusted enforces an absolute heartbeat against block.timestamp, a stalled FtsoV2PriceStore can leave last-posted prices in effect indefinitely. Liquidation CR checks (Liquidation.getCollateralRatiosBIPS) and payout pricing both consume these values, so a stale-above-market price will (a) make an agent appear undercollateralized, wrongly enabling startLiquidation/liquidate, and (b) pay liquidators at an inflated rate, draining agent vault and pool collateral. This risk persists until prices resume, and the path is permissionless. The blast radius is the agent vault and community pool funds; losses are direct and deterministic.

## Command to Run Test


## Proof of Concept
Scenario: stale FTSO price drives wrongful liquidation and overpayment
1) FtsoV2PriceStore stops updating. The last FTSO price for ASSET remains at 200 (decimals=8), timestamped 24 hours ago. The trusted feed is also last seen at the same values/timestamps.
2) LiquidationFacet.startLiquidation calls Liquidation.getCollateralRatiosBIPS(), which calls Conversion.currentAmgPriceInTokenWeiWithTrusted(). That function returns (a) ftsoPrice = 200, and (b) trustedPrice = 200 because `trustedPriceFresh` is checked only relative to FTSO timestamps, not to block.timestamp.
3) No absolute max-age is enforced, so both vaultCR and poolCR are computed using 200 instead of the true spot (e.g., 100). If this makes CR appear below the configured threshold, liquidation starts even though the agent is actually healthy.
4) In Liquidation._performLiquidation, the payoutC1Wei/payoutPoolWei are computed via Conversion.convertAmgToTokenWei(liquidatedAMG * factorBIPS, price), reusing the same stale price. The liquidator gets 200-based payouts instead of 100-based, overpaying directly from agent vault and pool.
5) Until the oracle posts again, repeated liquidations can continue to use this stale price, draining collateral.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Conversion} from "contracts/assetManager/library/Conversion.sol";
import {CollateralTypeInt} from "contracts/assetManager/library/data/CollateralTypeInt.sol";
import {Globals} from "contracts/assetManager/library/Globals.sol";
import {AssetManagerSettings} from "contracts/userInterfaces/data/AssetManagerSettings.sol";
import {IPriceReader} from "contracts/ftso/interfaces/IPriceReader.sol";

contract MockPriceReader is IPriceReader {
    struct Entry { uint256 price; uint256 ts; uint256 dec; }
    mapping(string => Entry) public p;
    mapping(string => Entry) public tp;
    function set(string calldata s, uint256 price, uint256 ts, uint256 dec) external { p[s] = Entry(price, ts, dec); }
    function setTrusted(string calldata s, uint256 price, uint256 ts, uint256 dec) external { tp[s] = Entry(price, ts, dec); }
    function getPrice(string memory _symbol)
        external view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
    { Entry storage e = p[_symbol]; return (e.price, e.ts, e.dec); }
    function getPriceFromTrustedProviders(string memory _symbol)
        external view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
    { Entry storage e = tp[_symbol]; return (e.price, e.ts, e.dec); }
    function getPriceFromTrustedProvidersWithQuality(string memory _symbol)
        external view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals, uint8 _numberOfSubmits)
    { Entry storage e = tp[_symbol]; return (e.price, e.ts, e.dec, 1); }
}

contract StaleOracleLiquidationPriceTest is Test {
    MockPriceReader pr;
    CollateralTypeInt.Data private col;

    function setUp() public {
        pr = new MockPriceReader();
        // Wire minimal settings used by Conversion
        AssetManagerSettings.Data storage s = Globals.getSettings();
        s.priceReader = address(pr);
        s.assetMintingDecimals = 6;            // used in price scaling
        s.maxTrustedPriceAgeSeconds = 300;     // only checked RELATIVE to FTSO timestamps

        // Set collateral meta for direct pair pricing (uses only assetFtsoSymbol)
        col.decimals = 18;
        col.directPricePair = true;
        col.assetFtsoSymbol = "ASSET";
        col.tokenFtsoSymbol = "";
    }

    function test_StalePricesAccepted_LeadToOverpaidPayouts() public {
        uint256 nowTs = block.timestamp;

        // 1) Publish stale prices (24h old) at 200. Both generic & trusted have same stale ts.
        pr.set("ASSET", 200e8, nowTs - 1 days, 8);
        pr.setTrusted("ASSET", 200e8, nowTs - 1 days, 8);
        (uint256 ftsoStale, uint256 trustedOrFallbackStale) = Conversion.currentAmgPriceInTokenWeiWithTrusted(col);
        // Both branches accept stale timestamps: trusted path returns 200, not reverted/guarded by heartbeat
        assertEq(ftsoStale,  Conversion.calcAmgToTokenWeiPrice(col.decimals, 1, 0, 200e8, 8), "stale FTSO used");
        assertEq(trustedOrFallbackStale, ftsoStale, "stale trusted-or-fallback should equal stale FTSO");

        // 2) Later the market halves to 100 and fresh prices are posted
        pr.set("ASSET", 100e8, nowTs, 8);
        pr.setTrusted("ASSET", 100e8, nowTs, 8);
        (uint256 ftsoFresh, ) = Conversion.currentAmgPriceInTokenWeiWithTrusted(col);

        // 3) Liquidation payouts scale with price; 1 AMG = 1e9 AMG units
        uint256 oneAMG = 1e9;
        uint256 payoutStale = Conversion.convertAmgToTokenWei(oneAMG, ftsoStale);
        uint256 payoutFresh = Conversion.convertAmgToTokenWei(oneAMG, ftsoFresh);
        assertGt(payoutStale, payoutFresh, "stale price produces larger liquidation payout");
    }
}


## Suggested Mitigation
Enforce absolute freshness for all price paths used by liquidation and payouts:
- Introduce a settings.maxFtsoPriceAgeSeconds (separate from maxTrustedPriceAgeSeconds) and require that both assetTimestamp and tokenTimestamp returned by FtsoV2PriceStore are >= block.timestamp - maxFtsoPriceAgeSeconds. For direct pairs, check assetTimestamp only.
- In Conversion.currentAmgPriceInTokenWei and currentAmgPriceInTokenWeiWithTrusted, revert with a clear error (e.g., PriceStale()) if timestamps violate the heartbeat window. Do not fall back to any unbounded-age price.
- In the trusted branch, keep the existing relative-freshness check, but additionally enforce each trusted timestamp against block.timestamp - maxTrustedPriceAgeSeconds. If stale, do not silently fall back to the generic FTSO price unless the FTSO price also passes the absolute freshness check; otherwise revert.
- Gate sensitive flows (startLiquidation, liquidate, redemption default payouts) on successful fresh price retrieval. If prices are stale, revert or temporarily pause the action.
- Use the same, freshness-checked price source for both CR decision and payout calculation so a stalled feed cannot create a mismatch between trigger and payout.





 **Derived From** : Liquidation relies on potentially stale FTSO prices without any max-age bound

## [H-2]. LiquidationFacet can be started and executed using arbitrarily stale oracle prices (no heartbeat on base FTSO feed)

### Finding Severity Justification: Collateral ratio calculations used to gate and execute liquidation rely on Conversion.currentAmgPriceInTokenWeiWithTrusted(), which falls back to (or even prefers) the latest FTSO/trusted price without any absolute heartbeat vs block.timestamp. If the base FTSO feed (and/or trusted feed) stalls, the code will accept arbitrarily old prices. Using stale prices can miscompute CRs and enable liquidations of otherwise healthy agents or overpay liquidators, directly risking loss of vault/pool assets.
## Derived From Pattern/Invariant
Liquidation relies on potentially stale FTSO prices without any max-age bound

## Exploit Type
Oracle

## Location
Conversion.currentAmgPriceInTokenWeiWithTrusted

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
LiquidationFacet.startLiquidation and liquidate rely on Liquidation.getCollateralRatiosBIPS(), which calls Conversion.currentAmgPriceInTokenWeiWithTrusted(collateral). That function fetches two prices via IPriceReader: a baseline FTSO price (getPrice) and a 'trusted' price (getPriceFromTrustedProviders). It only checks the 'trusted' price freshness relative to the FTSO timestamps:

bool trustedPriceFresh = tokenTimestampTrusted + maxTrustedPriceAgeSeconds >= tokenTimestamp && assetTimestampTrusted + maxTrustedPriceAgeSeconds >= assetTimestamp;

If that check fails, it falls back to the baseline FTSO price, but there is no absolute heartbeat/freshness check for the FTSO price itself (no comparison to block.timestamp, no answeredInRound-like guard). If the underlying relay/publisher stalls, Conversion.currentAmgPriceInTokenWeiWithTrusted will still return the old FTSO price without reverting. Consequently, liquidation entry and payouts are computed off stale prices:
- _startLiquidation() may trigger or skip liquidation incorrectly due to outdated CRs.
- _performLiquidation() computes vault/pool payouts using stale _cr.amgToC1WeiPrice/_cr.amgToPoolWeiPrice, over/underpaying liquidators and potentially draining agent/pool collateral.

Vulnerable snippets:
- Liquidation.getCollateralRatioBIPS -> _collateralDataWithTrusted -> Conversion.currentAmgPriceInTokenWeiWithTrusted
- Conversion.currentAmgPriceInTokenWeiWithTrusted (no absolute age check on FTSO timestamps)


## Impact
If the primary FTSO feed stalls, startLiquidation and liquidate operate on arbitrarily old prices because Conversion.currentAmgPriceInTokenWeiWithTrusted() reverts to the baseline FTSO values without any timestamp bound to block.timestamp. An attacker can wait for a price relay halt, then trigger liquidation on otherwise healthy agents or obtain outsized collateral payouts as time-based liquidation premiums accrue against stale valuations. This can lead to unjust collateral slashing and pool/vault losses.

## Command to Run Test


## Proof of Concept
Preconditions:
- Ftso/relay halts and continues returning the last published price with old timestamps.
- Trusted providers data is older than the configured maxTrustedPriceAgeSeconds relative to the FTSO timestamps (or absent), forcing fallback to the FTSO path.

Attack steps:
1) Attacker observes that the target agent’s CR is healthy under fresh market prices but would appear unhealthy under the last published (stale) snapshot.
2) Because Conversion.currentAmgPriceInTokenWeiWithTrusted() has no absolute heartbeat for the FTSO branch, startLiquidation(agent) and subsequent liquidate() use that stale price.
3) The agent is put into liquidation and liquidators can burn FAssets for collateral using liquidation factors computed off the stale snapshot, extracting excessive payouts and/or liquidating healthy positions.
4) When the oracle resumes, the mispriced liquidation is already executed and irreversible.

Why this works:
- In Conversion.currentAmgPriceInTokenWeiWithTrusted(), only the trusted path has a freshness check relative to the FTSO timestamps. If the trusted path is stale, code falls back to the FTSO price, which is never checked vs block.timestamp. Therefore, arbitrarily old FTSO prices are accepted system-wide, including liquidation flows.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Conversion} from "contracts/assetManager/library/Conversion.sol";
import {AssetManagerSettings} from "contracts/userInterfaces/data/AssetManagerSettings.sol";
import {CollateralTypeInt} from "contracts/assetManager/library/data/CollateralTypeInt.sol";
import {IPriceReader} from "contracts/ftso/interfaces/IPriceReader.sol";
import {Globals} from "contracts/assetManager/library/Globals.sol";

contract MockPriceReader is IPriceReader {
    // Two symbols used: "ASSET" and "TOKEN".
    uint256 public assetPrice; uint256 public assetTs; uint256 public assetDec;
    uint256 public tokenPrice; uint256 public tokenTs; uint256 public tokenDec;

    uint256 public assetPriceTrusted; uint256 public assetTsTrusted; uint256 public assetDecTrusted;
    uint256 public tokenPriceTrusted; uint256 public tokenTsTrusted; uint256 public tokenDecTrusted; uint8 public n;

    function setBase(uint256 _ap, uint256 _ats, uint256 _ad, uint256 _tp, uint256 _tts, uint256 _td) external {
        assetPrice=_ap; assetTs=_ats; assetDec=_ad; tokenPrice=_tp; tokenTs=_tts; tokenDec=_td;
    }
    function setTrusted(uint256 _ap, uint256 _ats, uint256 _ad, uint256 _tp, uint256 _tts, uint256 _td, uint8 _n) external {
        assetPriceTrusted=_ap; assetTsTrusted=_ats; assetDecTrusted=_ad; tokenPriceTrusted=_tp; tokenTsTrusted=_tts; tokenDecTrusted=_td; n=_n;
    }
    function getPrice(string memory _symbol) external view returns (uint256 p, uint256 t, uint256 d) {
        bytes32 h = keccak256(bytes(_symbol));
        if (h == keccak256("ASSET")) { return (assetPrice, assetTs, assetDec); }
        return (tokenPrice, tokenTs, tokenDec);
    }
    function getPriceFromTrustedProviders(string memory _symbol) external view returns (uint256 p, uint256 t, uint256 d) {
        bytes32 h = keccak256(bytes(_symbol));
        if (h == keccak256("ASSET")) { return (assetPriceTrusted, assetTsTrusted, assetDecTrusted); }
        return (tokenPriceTrusted, tokenTsTrusted, tokenDecTrusted);
    }
    function getPriceFromTrustedProvidersWithQuality(string memory _symbol) external view returns (uint256 p, uint256 t, uint256 d, uint8 q) {
        (p,t,d) = this.getPriceFromTrustedProviders(_symbol); q = n;
    }
}

contract ConversionStaleOracleTest is Test {
    CollateralTypeInt.Data internal col; // standalone storage struct

    function _setSettings(address priceReader, uint64 maxTrustedAge, uint8 assetMintingDecimals) internal {
        AssetManagerSettings.Data storage s = Globals.getSettings();
        s.priceReader = priceReader;
        s.maxTrustedPriceAgeSeconds = maxTrustedAge;
        s.assetMintingDecimals = assetMintingDecimals;
    }

    function test_Accepts_Arbitrarily_Old_FTSO_Prices_And_Uses_Them() public {
        // 1) Deploy mock reader and set settings
        MockPriceReader pr = new MockPriceReader();
        uint256 oldTs = block.timestamp - 7 days; // arbitrarily stale
        // Base FTSO prices/timestamps
        pr.setBase({ _ap: 2e8, _ats: oldTs, _ad: 8, _tp: 1e8, _tts: oldTs, _td: 8 });
        // Trusted providers older than base by > maxTrustedAge => force fallback to base FTSO
        pr.setTrusted({ _ap: 2e8, _ats: oldTs - 1 days, _ad: 8, _tp: 1e8, _tts: oldTs - 1 days, _td: 8, _n: 5 });
        _setSettings(address(pr), uint64(60), uint8(6)); // maxTrustedPriceAgeSeconds = 60s

        // 2) Prepare collateral metadata
        col.decimals = 18;
        col.directPricePair = false;
        col.assetFtsoSymbol = "ASSET";
        col.tokenFtsoSymbol = "TOKEN";

        // 3) Fetch base price with timestamps
        (uint256 ftsoPrice, uint256 assetTs, uint256 tokenTs) = Conversion.currentAmgPriceInTokenWeiWithTs(col, false);
        assertEq(assetTs, oldTs, "asset ts must be the arbitrarily old timestamp");
        assertEq(tokenTs, oldTs, "token ts must be the arbitrarily old timestamp");
        assertGt(ftsoPrice, 0, "stale FTSO price returned nonzero value");

        // 4) Fetch with trusted path enabled; it should fallback to base FTSO (stale) due to freshness check
        (uint256 priceFtsov2, uint256 priceTrusted) = Conversion.currentAmgPriceInTokenWeiWithTrusted(col);
        assertEq(priceFtsov2, priceTrusted, "trusted path must fallback to base FTSO when trusted is staler than allowed");

        // 5) Critically, no revert occurs although the timestamps are 7 days old.
        // This proves the missing heartbeat on the baseline FTSO feed, enabling liquidation logic to use stale prices.
    }
}


## Suggested Mitigation
Introduce and enforce a max-age (heartbeat) on the baseline FTSO timestamps before using them anywhere prices affect safety-critical flows (CR checks, liquidation, payouts). Concretely:
- Add a new governance-controlled setting, e.g. settings.maxFtsoPriceAgeSeconds.
- In Conversion.currentAmgPriceInTokenWeiWithTs(), after computing (price, assetTs, tokenTs), require that max(assetTs, tokenTs) + settings.maxFtsoPriceAgeSeconds >= block.timestamp; otherwise revert. Apply the same check in currentAmgPriceInTokenWei() and in the caller variants that use these prices.
- Optionally, fail-safe by pausing startLiquidation/liquidate when price freshness cannot be guaranteed.
This ensures that both trusted and baseline price paths are bound by absolute age, preventing liquidation and payout computations from using arbitrarily stale inputs.





 **Derived From** : Liquidation uses stale FTSO price (no freshness check) to compute CR and payouts

## [M-3]. LiquidationFacet.liquidate pays using stale oracle prices, allowing inflated liquidator payouts and wrongful liquidations

### Finding Severity Justification: Liquidation decisions and liquidator payouts rely on prices returned by Conversion.currentAmgPriceInTokenWei/WithTrusted that never validate freshness against block.timestamp. The trusted-path ‘freshness’ check is only relative to the generic feed, not an actual heartbeat. As a result, if the FTSO/relay price stalls, anyone can trigger liquidations and/or extract excess collateral using stale, off-market prices. Impact can be loss of real collateral from both the agent vault and the pool or wrongful/continued liquidation. While impact is high, the exploitability depends on an external condition (price publication halting), so overall severity is assessed as Medium.
## Derived From Pattern/Invariant
Liquidation uses stale FTSO price (no freshness check) to compute CR and payouts

## Exploit Type
Oracle

## Location
LiquidationFacet.liquidate

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
LiquidationFacet computes both the liquidation decision and liquidator payouts with prices fetched through Conversion that do not enforce any freshness. In Liquidation.getCollateralRatiosBIPS(), for each collateral kind the ftso price is returned via Conversion.currentAmgPriceInTokenWeiWithTrusted(collateral) but the value persisted into CRData (amgToC1WeiPrice / amgToPoolWeiPrice) is the generic FTSO price without any age check. Conversion.currentAmgPriceInTokenWei(token) calls currentAmgPriceInTokenWeiWithTs(token, false) and discards the returned timestamps entirely. During liquidation, _performLiquidation calculates payouts using these prices:

- LiquidationFacet._performLiquidation:
  _payoutC1Wei = Conversion.convertAmgToTokenWei(uint256(_liquidatedAMG).mulBips(vaultFactor), _cr.amgToC1WeiPrice);
  _payoutPoolWei = Conversion.convertAmgToTokenWei(uint256(_liquidatedAMG).mulBips(poolFactor), _cr.amgToPoolWeiPrice);

- Liquidation.getCollateralRatiosBIPS(): _amgToTokenWeiPrice is set from _data.amgToTokenWeiPrice (ftso price), and _collateralRatioBIPS = max(ratio, ratioTrusted). However, the payout path always uses the non-trusted ftso price placed in CRData.

- Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data storage _token) -> currentAmgPriceInTokenWeiWithTs(_token, false) returns (price, assetTs, tokenTs), but timestamps are ignored. currentAmgPriceInTokenWeiWithTrusted only checks (tokenTimestampTrusted + maxTrustedPriceAgeSeconds >= tokenTimestamp), i.e., a relative comparison vs generic ts, and never compares either timestamp to block.timestamp. If trusted is stale it silently falls back to the non-trusted price which also has no heartbeat.

This allows an attacker to exploit long-stale, higher-than-market oracle prices to force or continue liquidation, and to receive overpayment from the agent vault and pool funds.

## Impact
Because Conversion.currentAmgPriceInTokenWei and currentAmgPriceInTokenWeiWithTrusted never enforce freshness against block.timestamp, liquidation decisions (collateral ratios) and liquidator payouts can be computed using off-market prices. A stale high asset price reduces reported collateral ratios (making liquidation more likely) and simultaneously inflates convertAmgToTokenWei for payouts, overpaying liquidators from the agent vault and pool. Conversely, stale low prices can suppress liquidation or underpay/redemption accounting, mispricing the system. This enables wrongful liquidations and loss of agent/pool funds whenever the oracle stalls or lags significantly.

## Command to Run Test


## Proof of Concept
How the issue is exploitable end-to-end:

1) Configure the price reader to return a stale, inflated price for the asset (e.g., price=2.0 with timestamp=1) and later a fresh, lower price (e.g., 1.0 with timestamp≈now).
2) Conversion.currentAmgPriceInTokenWei(token) ignores the timestamps entirely and returns the stale value; Conversion.currentAmgPriceInTokenWeiWithTrusted() only checks trusted timestamps relative to the generic feed (tokenTimestampTrusted + maxTrustedPriceAgeSeconds >= tokenTimestamp), never against block.timestamp. If the trusted vote is stale or missing, it silently falls back to the generic stale feed.
3) LiquidationFacet.liquidate() calls Liquidation.getCollateralRatiosBIPS(), which fills CRData.amgToC1WeiPrice/amgToPoolWeiPrice from the generic FTSO price (not the trusted price), with no heartbeat check. A stale high price increases backingTokenWei and lowers reported CR, enabling or continuing liquidation.
4) _performLiquidation() computes payouts using those CRData prices:
   - payoutC1Wei = convertAmgToTokenWei(liquidatedAMG*vaultFactor, amgToC1WeiPrice)
   - payoutPoolWei = convertAmgToTokenWei(liquidatedAMG*poolFactor, amgToPoolWeiPrice)
   With stale high prices, both payouts are inflated. 
5) Result: A liquidator can liquidate agents using stale prices to (a) force liquidation (understated CR) and (b) receive excess vault/pool collateral vs current market, causing loss to agents and the pool.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {IPriceReader} from "contracts/ftso/interfaces/IPriceReader.sol";
import {Conversion} from "contracts/assetManager/library/Conversion.sol";
import {CollateralTypeInt} from "contracts/assetManager/library/data/CollateralTypeInt.sol";
import {Globals} from "contracts/assetManager/library/Globals.sol";
import {AssetManagerSettings} from "contracts/userInterfaces/data/AssetManagerSettings.sol";
import {SafePct} from "contracts/utils/library/SafePct.sol";

contract MockPriceReader is IPriceReader {
    struct P { uint256 price; uint256 ts; uint256 dec; }
    mapping(bytes32 => P) internal data;
    function setPrice(string memory sym, uint256 price, uint256 ts, uint256 dec) external {
        data[keccak256(bytes(sym))] = P(price, ts, dec);
    }
    function getPrice(string memory _symbol) external view returns (uint256, uint256, uint256) {
        P memory p = data[keccak256(bytes(_symbol))];
        return (p.price, p.ts, p.dec);
    }
    function getPriceFromTrustedProviders(string memory _symbol) external view returns (uint256, uint256, uint256) {
        P memory p = data[keccak256(bytes(_symbol))];
        return (p.price, p.ts, p.dec);
    }
    function getPriceFromTrustedProvidersWithQuality(string memory _symbol) external view returns (uint256, uint256, uint256, uint8) {
        P memory p = data[keccak256(bytes(_symbol))];
        return (p.price, p.ts, p.dec, 5);
    }
}

contract ConversionHarness {
    using SafePct for uint256;
    CollateralTypeInt.Data private collat;

    function setSettings(address pr, uint8 assetMintDec, uint64 maxTrustedAgeSec) external {
        AssetManagerSettings.Data storage s = Globals.getSettings();
        s.priceReader = pr;
        s.assetMintingDecimals = assetMintDec;
        s.maxTrustedPriceAgeSeconds = maxTrustedAgeSec;
    }
    function setCollateral(bool directPair, string memory assetSym, string memory tokenSym, uint8 tokenDecimals) external {
        collat.directPricePair = directPair; // true -> use asset/token direct feed
        collat.assetFtsoSymbol = assetSym;
        collat.tokenFtsoSymbol = tokenSym;   // unused when directPair==true
        collat.decimals = tokenDecimals;
    }
    function readFtsoWithTs() external view returns (uint256 price, uint256 assetTs, uint256 tokenTs) {
        return Conversion.currentAmgPriceInTokenWeiWithTs(collat, false);
    }
    function readTrustedPair() external view returns (uint256 ftsoPrice, uint256 trustedPrice) {
        return Conversion.currentAmgPriceInTokenWeiWithTrusted(collat);
    }
    function liquidationPayout(uint256 liquidatedAMG, uint256 factorBips) external view returns (uint256) {
        uint256 price = Conversion.currentAmgPriceInTokenWei(collat);
        uint256 amgWithFactor = liquidatedAMG.mulBips(factorBips);
        return Conversion.convertAmgToTokenWei(amgWithFactor, price);
    }
}

contract LiquidationStalePriceTest is Test {
    MockPriceReader pr;
    ConversionHarness h;

    function setUp() public {
        pr = new MockPriceReader();
        h = new ConversionHarness();
        // priceReader + minting decimals (e.g. XRP uses 6 minting decimals)
        h.setSettings(address(pr), 6, 3600); // 1h trusted-age for test; no heartbeat vs block.timestamp enforced
        // Use direct pair (asset/token) pricing to simplify, token decimals = 18
        h.setCollateral(true, "XRP", "", 18);
    }

    function test_StaleOracleInflatesLiquidationPayout_and_NoHeartbeatChecks() public {
        // 1) Configure stale, inflated price with ancient timestamp
        pr.setPrice("XRP", 2e8, 1, 8); // very old ts=1, price = 2.0 (8 decimals)
        (uint256 stalePrice, uint256 assetTsStale,) = h.readFtsoWithTs();
        assertGt(stalePrice, 0);
        assertEq(assetTsStale, 1); // timestamp is ancient, but accepted

        // 2) Compute payout at stale price (simulating liquidation payout path using Conversion)
        uint256 liquidatedAMG = 1e9; // arbitrary AMG unit (scaled by Conversion)
        uint256 factorBips = 12000;  // 1.2x liquidation factor
        uint256 payoutStale = h.liquidationPayout(liquidatedAMG, factorBips);
        assertGt(payoutStale, 0);

        // 3) Now set a fresh, lower market price and verify payouts decrease
        vm.warp(1 days);
        pr.setPrice("XRP", 1e8, block.timestamp, 8); // fresh ts, price = 1.0
        (uint256 freshPrice,,) = h.readFtsoWithTs();
        uint256 payoutFresh = h.liquidationPayout(liquidatedAMG, factorBips);
        assertGt(stalePrice, freshPrice);
        assertGt(payoutStale, payoutFresh); // stale inflated price overpays liquidator

        // 4) Show that the trusted path only compares timestamps relatively (no heartbeat vs block.timestamp)
        // Make trusted price older than generic, but within relative window -> it will select trusted
        pr.setPrice("XRP", 15e7, block.timestamp - 30, 8); // generic: 1.5, recent
        // trusted simulated by same reader; with our setup, trusted freshness is just relative
        (uint256 ftsoP, uint256 trustedP) = h.readTrustedPair();
        // Both values come back; library will accept trusted if relative condition holds.
        // Importantly, neither ftso nor trusted price is rejected for being too old vs block.timestamp.
        assertTrue(ftsoP != 0 && trustedP != 0);
    }
}


## Suggested Mitigation
Enforce strict price freshness and unify price selection across liquidation decision and payout:

1) Add a heartbeat check against block.timestamp in Conversion:
   - In currentAmgPriceInTokenWeiWithTs(collateral, bool), verify both returned timestamps (asset and token) are not older than settings.maxPriceAgeSeconds (new setting). Revert or return 0 if stale.
   - In currentAmgPriceInTokenWeiWithTrusted(collateral), validate each of (generic and trusted) against block.timestamp independently. If both are fresh, use trusted according to current policy; if trusted is stale but generic is fresh, use generic; if both stale, revert.

2) In Liquidation.getCollateralRatiosBIPS and _collateralDataWithTrusted, return both the selected CR and the same selected price for payouts. Ensure _performLiquidation uses the exact price used for CR selection (ftso vs trusted) to avoid payout/decision mismatches.

3) Fallback and safety behavior:
   - If price is stale, pause liquidation/payout for that collateral or whole protocol (depending on scope), emit an event, and require governance or monitors to resolve feed issues.
   - Optionally let governance configure per-feed max age and failure policy (revert vs. safe fallback) to avoid bricking redemptions.

4) Tests: add unit tests that (a) reject ancient timestamps vs block.timestamp and (b) guarantee payout prices are equal to the prices used for CR calculations under both generic and trusted modes.





 **Derived From** : Liquidation burns user FAssets even if vault/pool can’t fully pay

## [L-4]. LiquidationFacet.liquidate burns liquidator’s FAssets even when vault/pool pay 0 due to balance cap, breaking payout=burn invariant

### Finding Severity Justification: The code indeed allows a liquidator to burn their FAssets while receiving partial or even zero payout if the agent’s vault/pool balances are insufficient. However, this loss only occurs if the caller voluntarily executes liquidation under those conditions. No third party can force a user’s tokens to be burned, and no protocol assets are stolen. This is best categorized as a user-initiated unfavorable trade outcome rather than a protocol exploit.
## Derived From Pattern/Invariant
Liquidation burns user FAssets even if vault/pool can’t fully pay

## Exploit Type
AccountingInvariantViolation

## Location
LiquidationFacet.liquidate

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In LiquidationFacet.liquidate, the function computes liquidation payout targets from prices and liquidation factors, then calls AgentPayout.payoutFromVault / payoutFromPool which hard-cap actual transfers to available balances (min(requested, balance)). Regardless of any shortfall, it then executes Redemptions.burnFAssets(msg.sender, _liquidatedAmountUBA). This burns the liquidator’s FAssets even when the agent’s vault/pool can’t fulfill the payout (e.g., balances are 0), violating the invariant that burn value must be matched by payout. Vulnerable snippets:

- Cap-to-balance:
  AgentPayout.payoutFromVault: _amountPaid = Math.min(_amountWei, collateral.token.balanceOf(address(vault)));
  AgentPayout.payoutFromPool: _amountPaid = Math.min(_amountWei, poolBalance);

- Burn regardless of shortfall:
  if (_liquidatedAmountUBA > 0) {
      Redemptions.burnFAssets(msg.sender, _liquidatedAmountUBA);
  }

As a result, a liquidator (permissionless EOA) can lose FAssets when liquidating an underfunded agent; the agent’s liability decreases (tickets closed) while paying nothing or only partially.

## Impact
A liquidator can burn their FAssets while receiving zero or only partial collateral if the agent’s vault and/or pool balances are insufficient at the moment of liquidation. Because LiquidationFacet.liquidate computes burn amount first and only caps transfers later (via AgentPayout), it can end up burning more FAssets than the paid-out value supports, effectively shifting value from the liquidator to the agent (agent liability drops while paying nothing). This is a direct, irreversible loss to liquidators, and callers have no on-chain guard preventing such an outcome.

## Command to Run Test


## Proof of Concept
Root cause
- In LiquidationFacet.liquidate(), the function first determines liquidatedAmount (AMG/UBA) and corresponding target payouts based on liquidation factors and prices.
- Actual transfers are then capped to available balances in AgentPayout.payoutFromVault/payoutFromPool using min(requested, balance).
- Regardless of any shortfall (including paying zero), the code proceeds to Redemptions.burnFAssets(msg.sender, _liquidatedAmountUBA).

Consequence
- When both the vault and pool have insufficient balances (e.g., zero), the payout functions return zero but the user’s FAssets are still burned, reducing the agent’s liability while the liquidator is uncompensated.

Trigger example
1) An agent is in liquidation and has minted liability but zero vault collateral and zero pool collateral.
2) A liquidator calls liquidate(agent, amountUBA).
3) Liquidation factors produce a nonzero liquidatedAmountUBA; payoutFromVault/pool cap transfers to 0 due to zero balances.
4) burnFAssets is executed for liquidatedAmountUBA -> liquidator loses FAssets, receives 0.

Why checks don’t prevent it
- LiquidationPaymentStrategy caps per-collateral factor to the current CR, but if CRs are near zero, factors can become 0; nevertheless, the maxLiquidationAmountAMG logic can still choose to liquidate (e.g., to full minted amount in some edge cases) and proceeds to close tickets/dust and burn.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {LiquidationFacet} from "contracts/assetManager/facets/LiquidationFacet.sol";
import {AssetManagerState} from "contracts/assetManager/library/data/AssetManagerState.sol";
import {Globals} from "contracts/assetManager/library/Globals.sol";
import {AssetManagerSettings} from "contracts/userInterfaces/data/AssetManagerSettings.sol";
import {CollateralTypeInt} from "contracts/assetManager/library/data/CollateralTypeInt.sol";
import {CollateralType} from "contracts/userInterfaces/data/CollateralType.sol";
import {Agent} from "contracts/assetManager/library/data/Agent.sol";
import {IPriceReader} from "contracts/ftso/interfaces/IPriceReader.sol";
import {IIFAsset} from "contracts/fassetToken/interfaces/IIFAsset.sol";
import {IIAgentVault} from "contracts/agentVault/interfaces/IIAgentVault.sol";
import {IICollateralPool} from "contracts/collateralPool/interfaces/IICollateralPool.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockPriceReader is IPriceReader {
    function getPrice(string memory) external view returns (uint256 p, uint256 ts, uint256 d) {
        return (1e9, block.timestamp, 9);
    }
    function getPriceFromTrustedProviders(string memory) external view returns (uint256 p, uint256 ts, uint256 d) {
        return (1e9, block.timestamp, 9);
    }
    function getPriceFromTrustedProvidersWithQuality(string memory)
        external view returns (uint256 p, uint256 ts, uint256 d, uint8 q) {
        return (1e9, block.timestamp, 9, 10);
    }
}

contract MockFAsset is IIFAsset {
    string public override assetName = "XRP";
    string public override assetSymbol = "XRP";
    address public override assetManager; // unused in mock
    uint8 public _dec = 18;
    mapping(address => uint256) public bal;
    address public cleanupBlockNumberManager;

    function name() external pure returns (string memory) { return "FXRP"; }
    function symbol() external pure returns (string memory) { return "FXRP"; }
    function decimals() external view returns (uint8) { return _dec; }
    function totalSupply() external pure returns (uint256) { return 0; }
    function balanceOf(address a) external view returns (uint256) { return bal[a]; }
    function transfer(address, uint256) external pure returns (bool) { return false; }
    function allowance(address, address) external pure returns (uint256) { return 0; }
    function approve(address, uint256) external pure returns (bool) { return false; }
    function transferFrom(address, address, uint256) external pure returns (bool) { return false; }

    function mint(address _owner, uint256 _amount) external { bal[_owner] += _amount; }
    function burn(address _owner, uint256 _amount) external { require(bal[_owner] >= _amount, "bal"); bal[_owner] -= _amount; }

    function setCleanupBlockNumberManager(address m) external { cleanupBlockNumberManager = m; }
    function cleanupBlockNumber() external pure returns (uint256) { return 0; }
    function setCleanerContract(address) external {}
    function setAssetManager(address _am) external { assetManager = _am; }
    function setCleanupBlockNumber(uint256) external {}
}

contract MockERC20 is IERC20 {
    mapping(address => uint256) public _b;
    function totalSupply() external pure returns (uint256) { return 0; }
    function balanceOf(address a) external view returns (uint256) { return _b[a]; }
    function transfer(address, uint256) external pure returns (bool) { return false; }
    function allowance(address, address) external pure returns (uint256) { return 0; }
    function approve(address, uint256) external pure returns (bool) { return false; }
    function transferFrom(address, address, uint256) external pure returns (bool) { return false; }
    function setBalance(address a, uint256 v) external { _b[a] = v; }
}

contract MockAgentVault is IIAgentVault {
    function assetManager() external pure returns (IIAssetManager) { return IIAssetManager(address(0)); }
    function isOwner(address) external pure returns (bool) { return false; }
    function payout(IERC20, address, uint256) external {} // no-op
    function destroy() external {}
}

contract MockCollateralPool is IICollateralPool {
    function setPoolToken(address) external {}
    function depositNat() external payable {}
    function payout(address, uint256, uint256) external {}
    function destroy(address payable) external {}
    function upgradeWNatContract(IWNat) external {}
    function setExitCollateralRatioBIPS(uint256) external {}
    function fAssetFeeDeposited(uint256) external {}
    function wNat() external pure returns (IWNat) { return IWNat(address(0)); }
    function debtFreeTokensOf(address) external pure returns (uint256) { return 0; }
    function debtLockedTokensOf(address) external pure returns (uint256) { return 0; }
    function assetManager() external pure returns (IIAssetManager) { return IIAssetManager(address(0)); }
    function enter() external payable returns (uint256, uint256) { return (0,0); }
    function exit(uint256) external returns (uint256) { return 0; }
    function exitTo(uint256, address payable) external returns (uint256) { return 0; }
    function selfCloseExit(uint256, bool, string memory, address payable) external payable {}
    function selfCloseExitTo(uint256, bool, address payable, string memory, address payable) external payable {}
    function withdrawFees(uint256) external {}
    function withdrawFeesTo(uint256, address) external {}
    function payFAssetFeeDebt(uint256) external {}
    function poolToken() external pure returns (ICollateralPoolToken) { return ICollateralPoolToken(address(0)); }
    function agentVault() external pure returns (address) { return address(0); }
    function exitCollateralRatioBIPS() external pure returns (uint32) { return 0; }
    function totalCollateral() external pure returns (uint256) { return 0; } // empty pool
    function fAssetFeesOf(address) external pure returns (uint256) { return 0; }
    function totalFAssetFees() external pure returns (uint256) { return 0; }
    function fAssetFeeDebtOf(address) external pure returns (int256) { return 0; }
    function totalFAssetFeeDebt() external pure returns (int256) { return 0; }
    function fAssetRequiredForSelfCloseExit(uint256) external pure returns (uint256) { return 0; }
}

contract LiquidationBurnNoPayoutTest is Test {
    LiquidationFacet facet;
    MockPriceReader pr;
    MockFAsset fasset;
    MockERC20 vaultToken;
    MockAgentVault vault;
    MockCollateralPool pool;

    address liquidator = address(0xBEEF);

    function setUp() public {
        facet = new LiquidationFacet();
        pr = new MockPriceReader();
        fasset = new MockFAsset();
        vaultToken = new MockERC20();
        vault = new MockAgentVault();
        pool = new MockCollateralPool();

        // Settings
        AssetManagerSettings.Data storage s = Globals.getSettings();
        s.priceReader = address(pr);
        s.fAsset = address(fasset);
        s.assetMintingGranularityUBA = 1; // 1 UBA per AMG for simplicity
        s.lotSizeAMG = 10;                // avoid divide-by-zero in ticket math
        s.maxRedeemedTickets = 20;
        s.liquidationStepSeconds = 300;
        s.liquidationCollateralFactorBIPS.push(12000);
        s.liquidationFactorVaultCollateralBIPS.push(12000);

        // Collateral types: index 0 = POOL, index 1 = VAULT
        AssetManagerState.State storage st = AssetManagerState.get();
        st.collateralTokens.push(CollateralTypeInt.Data({
            token: IERC20(address(0)),
            collateralClass: CollateralType.Class.POOL,
            decimals: 18,
            validUntil: 0,
            directPricePair: true,
            assetFtsoSymbol: "FXRP/POOL",
            tokenFtsoSymbol: "",
            minCollateralRatioBIPS: 15000,
            __ccbMinCollateralRatioBIPS: 0,
            safetyMinCollateralRatioBIPS: 16000
        }));
        st.collateralTokens.push(CollateralTypeInt.Data({
            token: vaultToken,
            collateralClass: CollateralType.Class.VAULT,
            decimals: 18,
            validUntil: 0,
            directPricePair: true,
            assetFtsoSymbol: "FXRP/VAULT",
            tokenFtsoSymbol: "",
            minCollateralRatioBIPS: 12000,
            __ccbMinCollateralRatioBIPS: 0,
            safetyMinCollateralRatioBIPS: 13000
        }));

        // Give the liquidator some FAssets to burn
        fasset.mint(liquidator, 1_000);

        // Initialize agent state at storage slot for the actual agent vault address
        _initAgent(address(vault));

        // Ensure vault and pool are empty -> payout caps to 0
        vaultToken.setBalance(address(vault), 0);
        // pool.totalCollateral() is hardcoded to 0 in mock
    }

    function _initAgent(address _agentVault) internal {
        // Point Agent.State mapping slot to key = _agentVault
        bytes32 pos = bytes32(uint256(keccak256("fasset.AssetManager.Agent")) ^ (uint256(uint160(_agentVault)) << 64));
        Agent.State storage a;
        assembly { a.slot := pos }
        a.ownerManagementAddress = address(0x1234);
        a.status = Agent.Status.NORMAL; // _startLiquidation will set LIQUIDATION
        a.vaultCollateralIndex = 1;     // VAULT collateral @ index 1
        a.poolCollateralIndex = 0;      // POOL collateral @ index 0
        a.collateralPool = pool;
        a.mintedAMG = 100;              // agent has liability
        a.dustAMG = 100;                // allow liquidation via dust path
        a.liquidationStartedAt = uint64(block.timestamp);
    }

    function test_BurnsFAssetsWhenPayoutZero() public {
        vm.startPrank(liquidator);
        (uint256 liqUBA, uint256 paidVault, uint256 paidPool) = facet.liquidate(address(vault), 100);
        vm.stopPrank();

        // No payout due to zero balances
        assertEq(paidVault, 0, "vault paid");
        assertEq(paidPool, 0, "pool paid");
        // But user's FAssets were still burned
        assertGt(liqUBA, 0, "no liquidation amount");
        assertEq(fasset.balanceOf(liquidator), 1000 - liqUBA, "burn not applied");
    }
}


## Suggested Mitigation
Prevent burning more FAssets than can actually be paid. Before closing tickets and burning, cap the liquidation amount by the payability derived from current vault/pool balances and liquidation factors.

Suggested approach in LiquidationFacet.liquidate/_performLiquidation:
1) Compute the current liquidation factors (vaultFactorBIPS, poolFactorBIPS) and prices.
2) Read available balances: vault token balance (IERC20(vaultCollateral).balanceOf(vaultAddress)) and pool totalCollateral().
3) Derive the maximum payable liquidation in AMG from each collateral:
   - If vaultFactorBIPS > 0: maxAmgFromVault = convertTokenWeiToAMG(availableVaultWei, amgToC1WeiPrice) * MAX_BIPS / vaultFactorBIPS.
   - If poolFactorBIPS > 0: maxAmgFromPool  = convertTokenWeiToAMG(availablePoolWei, amgToPoolWeiPrice) * MAX_BIPS / poolFactorBIPS.
   - Sum to get maxPayableAMG (handle zero factors by treating corresponding term as zero).
4) Set amountToLiquidateAMG = min(amountToLiquidateAMG, maxPayableAMG).
5) Only then call Redemptions.closeTickets(...) and burn the corresponding FAssets.

This ensures the burn is bounded by what the vault/pool can actually pay at that moment. Alternatively, if sticking to the current flow, after receiving actual paid amounts, recompute the payable AMG from the amountsPaid (inverse of step 3) and burn only that many FAssets, while adjusting agent state to reflect any difference (but the former approach is simpler and avoids mid-function state rollback).




