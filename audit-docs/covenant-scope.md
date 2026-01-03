**NOTE**: ALL privileged roles are TRUSTED. Any Finding that requires role is Low/Information UNLESS Impact is HIGH.

### Arbitrage Opportunities

The system behaves like a DEX (where collateral, leverage and yield tokens can be swapped for each other).

We care that there are no arbitrage opportunities (more value taken out than going in) under a circular set of trades upto a `10e20` precision. 

However, we are ok if `value_out` is bigger than `value_in` for smaller amounts (e.g, if `10e21` of a token goes in, then `10e21 + 1` can come out).

### Administrative Risks

Admin risks (if admin is taken over, or if admin misconfigures oracles / latentswap limits, etc) are out of scope. 

### Fee Handling

The protocol is ok with small miscalculations or underaccruals of fees (that are in favour of the protocol instead of the users). Specifically, there are design choices to accrue small amounts probabilistically that could be manipulated by validators, but we chose to accept this risk. Fees that accrue for users are in scope.

There are some edge cases for the last user leaving a market where fees can be front-run. As documented in code, this risk is out scope.

### Covenant + LatentSwap Governance

Once a market is created, the only thing governance can do is pause/unpause the market, and change mint/redeem caps. The risk that governance keys are compromised and markets are paused (and held hostage) is out of scope, or the risk that caps are removed.

The risk of governances keys being compromised and potential consequences of incorrect or fraudulent governance actions is out of scope. This includes invalid actions by governance (e.g. creating a market with a non-working oracle, etc).

### Curator (oracle) Governance

Covenant governance can approve a curator (oracle router) and subsequently this cannot be changed for a market.  However, Curator governance can change where the router points to, potentially affecting market behaviour.  The risk of curators misconfiguring a live market is out of scope.

### Oracle Misbehaviours

The Covenant protocol assumes the price from the Oracle (as governed by curators) can be trusted, and does not add additional checks. The oracle contracts themselves are a direct copy of those created for Euler, specifically for Chainlink (push) + Pyth (pull). The DEX effectively prices around the oracle price, and hence DEX actions can compensate (up-to a point) for oracle mispricing or lags. Given this, volatile assets (e.g., ETH, BTC, MON) are priced through pull feeds from Pyth, similar to a Perp DEX (Chainlink streams being implemented but not in scope for this audit), whereas more stable assets (e.g. USDC / USDT) being priced through Chainlink.  The Covenant system itself is semi-permissionless, so we do not check or stop curators from using slow oracles for volatile assets (governance risk).  These markets can be created, but users will quickly see that other markets are more liquid and have less slippage vs these markets and gravitate to those.

In addition, if approved by Curator governance, ERC4262 prices will be used as a price source, and mispricing by a Governance approved ERC4262 is out of scope.

### Large Market Size

We expect Covenant markets to be based on token assets with reasonable FDVs and total token mint amounts, with reasonable pricing from Oracles. We have built some protections for markets that are really big - e.g., where the FDV is bigger than 2^256 when measured in quote token decimal units. Or when markets become big over time. Specifically, we are using saturating multiplications at times which allow markets not to lock - but change market functionality in the following ways: 1) yield tokens might not accrued more yield after a point (>1000 years for most valid markets) 2) it will not be possible to mint new leverage tokens or yield tokens because their amount is over 2^256, etc. In these situations, we want users to be able to unwind their positions (and stop using that market instance) - but acknowledge that pricing might be off and interest might not be accruing in some edge cases.

### Market Parameterization

Market parameters are tested with limits upon creation, and only markets within these limits are in scope.

### Irregular ERC20s

Irregular ERC20s are out of scope (e.g, fee-on transfer, rebasing).  However, older ERC20s with slightly different return interfaces (e.g, bytes32 for name, no success bool for transfers) are in scope.  E.g, DAI, USDT, SNX.

### In-Code Comments

Any in-code comments supercede the functionality outlined in the project's documentation, and may reflect additional design choices that have not been explicitly outlined in this chapter and will be considered out of scope.

# Overview

Covenant enables markets for the use and funding of leverage against any collateral asset, using the collateral itself as liquidity. This facilitates the permissionless creation of structured products and unlocks a 10x increase in DeFi liquidity.

## Links

- **Previous audits:**  https://github.com/pashov/audits/blob/master/team/pdf/Covenant-security-review_2025-08-18.pdf
- **Documentation:** https://docs.covenant.finance
- **Website:** https://covenant.finance/
- **X/Twitter:** https://x.com/covenantFi

---

# Scope

### Files out of scope

| File         |
| ------------ |
| [script/\*\*.\*\*](https://github.com/code-423n4/2025-10-covenant/tree/main/script) |
| [src/curators/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-10-covenant/tree/main/src/curators/interfaces) |
| [src/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-10-covenant/tree/main/src/interfaces) |
| [src/lex/latentswap/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-10-covenant/tree/main/src/lex/latentswap/interfaces) |
| [src/periphery/\*\*.\*\*](https://github.com/code-423n4/2025-10-covenant/tree/main/src/periphery) |
| [test/\*\*.\*\*](https://github.com/code-423n4/2025-10-covenant/tree/main/test) |
| Totals: 51 |


# Additional context

## Areas of concern (where to focus for bugs)

### LatentSwap

- Is the LatentSwap invariant upheld at all times for atomic transactions?
- Can anyone mint / redeem / swap more tokens than allowed to (and extract value from the system, up-to a 10^-20 precision)?
- Can one user bring in a large amount of base assets (up-to allowed limits), and through a series of mint / swap / redeem actions extract value from the system (up-to a 10^-20 precision)?

### Covenant

- Does the Covenant contract keep base assets between markets siloed and independent of each other (i.e., one market cannot take out base assets from another)?
- Can a malformed ERC20 or oracle external contract re-enter the market to extract value?
- Can a Covenant market be bricked (in situations where the collateral FDV is in an appropriate range)?

## Main invariants

### LatentSwap Invariant

This invariant sets the relationship between the notional value of yield tokens, the value of leverage tokens, and the value of base assets. This is based on the following two formulas:

$$
\left(\frac{L}{\sqrt{P_a}} - V_{yield}\right )\left(L\sqrt{P_b} - V_{leverage}\right)=L^2
$$
where
$$
L = V_c \frac{\sqrt{P_a P_b}}{\sqrt{P_b}-\sqrt{P_a}}
$$

Where $P_b$, $P_a$ are the min / max price of the market (price edges). $V_c$ is the collateral value, $V_{yield}$ the yield coin notional value, and $V_{leverage}$ the leverage coin value.  The formula above is very similar to a Uniswap V3 concentrated liquidity invariant between two prices $p_a$ and $p_b$.

When initialized, markets are position at target LTV and have a price of 1.  In this instance, $V_{collateral} = V_{yield} + V_{leverage}$.  However, this equality is not an invariant, it is just a spot case of the previous equation. What does hold however, is that $V_{collateral} <= V_{yield} + V_{leverage}$. 


## All trusted roles in the protocol

| Role                                | Description                       |
| --------------------------------------- | ---------------------------- |
| Covenant Governance                          | Can pause markets, and add trusted LEX markets and curators (oracles)                |
| Covenant Pause                             | Per market, this address can pause the market (and transfer this permission)                       |
| LEX Governance                             | Can set some market parameters, including mint / redeem limits                       |
| Curator Governance                             | Can add / modify / remove oracles                       |


# LIBRARY VERSION UPDATES: ESSENTIAL AUDIT CONTEXT (2024-2025)

**Purpose**: Core reference for audit analysis. Review before flagging findings to avoid false positives.

---

## 1. CHAINLINK ORACLES - DEPRECATED PATTERNS

### ❌ DEPRECATED: `answeredInRound >= roundId` Check

**Status**: Deprecated 2023-2024

**OLD (2021-2023)**:
```solidity
(uint80 roundId, , , uint256 updatedAt, uint80 answeredInRound) = feed.latestRoundData();
require(answeredInRound >= roundId, "Stale");  // ← DEPRECATED
```

**CURRENT (2024-2025)**:
```solidity
(, int256 answer,, uint256 updatedAt,) = feed.latestRoundData();
require(answer > 0, "Invalid");
require(block.timestamp - updatedAt <= maxStaleness, "Stale");
// NO answeredInRound check needed
```

**Why**: Modern Chainlink feeds return `answeredInRound = 0`. Field no longer maintained.

**Validation**:
- ❌ FALSE POSITIVE: "Missing `answeredInRound >= roundId` check"
- ✅ VALID: `updatedAt` staleness check only

---

## 2. OPENZEPPELIN 5.x BREAKING CHANGES

### ❌ REMOVED: `_beforeTokenTransfer` Hook

**OLD (OZ 4.x)**:
```solidity
function _beforeTokenTransfer(address from, address to, uint256 amount) internal virtual override
```

**NEW (OZ 5.x)**:
```solidity
function _update(address from, address to, uint256 value) internal virtual override
```

**Validation**:
- ❌ FALSE POSITIVE: "Missing `_beforeTokenTransfer`" in OZ 5.x
- ✅ VALID: Check for `_update` instead

---

### ❌ REMOVED: `safeApprove`

**OLD (OZ 4.x)**:
```solidity
IERC20(token).safeApprove(spender, amount);
```

**NEW (OZ 5.x)**:
```solidity
IERC20(token).forceApprove(spender, amount);
```

**Validation**:
- ❌ COMPILATION ERROR: `safeApprove` doesn't exist in OZ 5.x
- ⚠️ NOTE: `increaseAllowance`/`decreaseAllowance` NOT deprecated

---

### ⚠️ CHANGED: `Ownable` Constructor

**OLD (OZ 4.x)**:
```solidity
constructor() { } // owner = msg.sender automatically
```

**NEW (OZ 5.x)**:
```solidity
constructor(address initialOwner) Ownable(initialOwner) { }
```

**Validation**:
- ❌ COMPILATION ERROR: OZ 5.x requires explicit `initialOwner`

---

## 3. SOLIDITY COMPILER UPDATES

### Solidity 0.8.20+ (Shanghai EVM)

**Issue**: Uses `PUSH0` opcode not supported on some L2s

**Validation**:
- ⚠️ VALID: Flag `pragma solidity ^0.8.20` for L2 deployment
- ✅ MITIGATION: Use `0.8.19` or EVM version `paris`

---

## 4. ERC-4626 VAULT PATTERNS

### First Depositor Inflation Attack

**Vulnerable**:
```solidity
constructor(IERC20 asset) ERC4626(asset) ERC20("Vault", "VLT") { }
```

**Mitigated**:
```solidity
constructor(IERC20 asset) ERC4626(asset) ERC20("Vault", "VLT") {
    _mint(address(0xdead), 1e6);  // Dead shares
}
```

**OpenZeppelin 5.x**: Virtual shares built-in automatically

**Validation**:
- ✅ VALID: Flag missing protection in OZ 4.x or custom implementations
- ❌ FALSE POSITIVE: Flag in OZ 5.x (virtual shares built-in)

---

## 5. ORACLE PATTERNS

### Oracle Inheritance from OOS Libraries

**Pattern**:
```solidity
import {ChainlinkOracle as EulerChainlinkOracle} from "@euler-price-oracle/...";
contract ChainlinkOracle is EulerChainlinkOracle { }
```

**Scope Rules**:
- ❌ OUT OF SCOPE: Bugs in Euler's `_getQuote` implementation
- ✅ IN SCOPE: Incorrect usage by in-scope code
- ✅ IN SCOPE: Missing overrides that should add safety

**Validation**: Check scope docs for "direct copy" statements

---

### Pull vs Push Oracles

**Pattern**: Using BOTH Chainlink (push) AND Pyth (pull)

**Status**: ✅ INTENTIONAL DESIGN

**Validation**:
- ❌ FALSE POSITIVE: "Inconsistent oracle usage"
- ✅ VALID: Check oracle selection matches asset volatility

---

## 6. REENTRANCY PATTERNS

### Read-Only External Calls

**Safe**:
```solidity
uint256 price = oracle.getPrice();  // Read-only, no reentrancy risk
balance[msg.sender] += amount;
```

**Unsafe**:
```solidity
token.transfer(msg.sender, amount);  // State-changing call
balance[msg.sender] -= amount;       // State update AFTER
```

**Validation**:
- ❌ FALSE POSITIVE: Flag read-only calls (view functions)
- ✅ VALID: Flag state-changing calls before state updates

---

## 7. ACCESS CONTROL PATTERNS

### Ownable vs AccessControl

**Status**: ✅ DESIGN CHOICE (not a vulnerability)

**Validation**:
- ❌ FALSE POSITIVE: "Should use AccessControl instead of Ownable"
- ✅ VALID: Check if privileges are too broad

---

## 8. TOKEN HANDLING PATTERNS

### Approve Race Condition

**Status**: ❌ NOT A VULNERABILITY (Code4rena 2024 rules)

**Validation**:
- ❌ INVALID: "Approve race condition"
- ❌ INVALID: "Should use safeApprove" (deprecated/removed)

---

## 9. MATH & PRECISION

### Division Before Multiplication

**Vulnerable**:
```solidity
uint256 result = (amount / price) * multiplier;  // Precision loss
```

**Safe**:
```solidity
uint256 result = (amount * multiplier) / price;
```

**Validation**:
- ✅ VALID: Flag division before multiplication
- ⚠️ SEVERITY: Depends on magnitude (dust = Low, extractable = High)

---

## 10. VALIDATION CHECKLIST

Before flagging a finding:

1. ✅ Check library version (OZ 4.x vs 5.x, Solidity 0.8.19 vs 0.8.20+)
2. ✅ Check false positives list (Section 11)
3. ✅ Check if deprecated pattern from 2021-2023
4. ✅ Check scope rules (OOS library vs in-scope code)
5. ✅ Check if reputable protocols use this pattern

---

## 11. COMMON FALSE POSITIVES

| Pattern | Status | Reason |
|---------|--------|--------|
| Missing `answeredInRound` check | ❌ False Positive | Deprecated 2023-2024 |
| Missing `_beforeTokenTransfer` | ❌ False Positive | Removed in OZ 5.x |
| `safeApprove` usage | ❌ False Positive | Removed in OZ 5.x |
| Approve race condition | ❌ False Positive | Not a vulnerability per C4 |
| Ownable vs AccessControl | ❌ False Positive | Design choice |
| Inflation attack (OZ 5.x) | ❌ False Positive | Virtual shares built-in |
| Read-only call before state | ❌ False Positive | No reentrancy risk |
| `increaseAllowance` deprecated | ❌ False Positive | NOT deprecated |
| `isContract` can be bypassed | ❌ False Positive | Known limitation |

---

## 12. SEVERITY ASSESSMENT

### High:
- Direct asset loss with realistic attack path
- Permanent fund freezing
- Oracle manipulation → profitable trades

### Medium:
- Temporary DoS of core flows
- Accounting errors (bounded/fixable)
- Reward distortion (not immediately extractable)

### Low/QA:
- Dust amounts (< $0.01)
- Best practice violations
- Governance risks (unless HIGH impact)

---

## 13. SCOPE RULES

### In-Scope:
- ✅ Bugs in project's own code
- ✅ Incorrect usage of OOS libraries
- ✅ Missing safety checks in in-scope code

### Out-of-Scope:
- ❌ Bugs inside OOS libraries (Euler, Chainlink, etc.)
- ❌ Oracle misbehaviors (if stated in scope)
- ❌ Governance/admin risks (unless HIGH impact)

---

## 14. RED FLAGS (Likely False Positive)

- Finding mentions "answeredInRound"
- Finding mentions "_beforeTokenTransfer" (check OZ version)
- Finding mentions "approve race condition"
- Finding mentions "should use AccessControl"
- Finding is in OOS library code
- Finding requires admin mistake (unless HIGH impact)
- Finding is dust amounts (< $0.01)

---

## 15. GREEN FLAGS (Likely Valid)

- Finding shows direct asset loss
- Finding has working PoC
- Finding is in in-scope code
- Finding has realistic attack path
- Finding not in false positives list
- Finding not a deprecated pattern

---

**Last Updated**: October 2024

