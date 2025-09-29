# Protocol Deployment and Constraints

## Deployment Chains

* **All contracts**: Base
* **SummerGovernorV2**: Base, Sonic, Arbitrum, Mainnet

---

## Token Integration

* **Staking token**: [0x194f360D130F2393a5E9F3117A6a1B78aBEa1624 (Base)](https://basescan.org/address/0x194f360D130F2393a5E9F3117A6a1B78aBEa1624)
* **Reward tokens**: SUMR, USDC, USDT, WETH, WBTC, EURC

Note: While SUMR is not globally transferable at the moment, it will be globally transferable with `transferEnabled = true` upon deployment.

---

## Admin / Role-Based Limitations

### Vesting Wallet Escrow

* Will **not** be initialized with factories at start.

### Summer Staking

* Will **not** be initialized with non-zero caps at start.
* Existing timelock contracts will be reused with new governance contract.

**Functions:**

* `updateLockupBucketCap(Bucket _bucket, uint256 _newCap)` (onlyGovernor)

  * Limits: none on `_newCap`.
  * Semantics:

    * `cap = 0`: disables staking in that bucket.
    * `type(uint256).max`: effectively unlimited.

* `updatePenaltyEnabled(bool _penaltyEnabled)` (onlyGovernor)

  * Limits: boolean only.

* `rescueToken(address _token, address _to)` (onlyGovernor)

  * Limits: `_token != WRAPPED_SUMMER_TOKEN`, `_to != 0`.

**Hard caps (not admin-settable):**

* `MAX_LOCKUP_PERIOD = 3 * 365 days`
* `MAX_AMOUNT_OF_STAKES = 1000`
* Penalty bounds: 2%–20%

---

### SummerVestingWalletsEscrow.sol

* `addVestingFactory(address _vestingFactory)` (onlyGovernor)

  * Limits: non-zero, must not already exist.
* `removeVestingFactory(address _vestingFactory)` (onlyGovernor)

  * Limits: non-zero, must exist.
* `rescueWallet(address _wallet, address _newOwner)` (onlyGovernor)

  * Limits: `_newOwner != 0`.
* `rescueToken(address _token, address _to)` (onlyGovernor)

  * Limits: both non-zero.
* **Constructor**: `address[] _initialVestingFactories`

  * Limits: each entry must be non-zero; no explicit length cap beyond gas.

---

### StakedSummerToken.sol (xSUMR)

* `addStakingModule/removeStakingModule(address)` (onlyGovernor)

  * Limits: add requires non-zero; remove no constraints.
  * Effect: Grants/revokes `MINTER_ROLE` and `BURNER_ROLE`.
* `grantMinterRole/revokeMinterRole(address)` (onlyGovernor)

  * Limits: none (role semantics apply).
* `pause/unpause()` (onlyGuardianOrGovernor)

  * Limits: boolean toggle only.

**Safety constraints (not admin-set):**

* Transfers disabled except mint/burn.
* `burnFrom`: only token owner or `BURNER_ROLE`, must respect allowance.
* Direct `AccessControl.grantRole/revokeRole` disabled; must use governed functions.

---

### SummerGovernorV2.sol

* **Constructor**:

  * `proposalThreshold` must be within `[1000e18, 100000e18]`.

* **Cross-chain governance**:

  * `sendProposalToTargetChain(...)` (onlyGovernance, onlyHubChain)
  * No explicit array length checks; relies on OZ Governor logic.

* **Execution / funding**:

  * `_payNative(uint256)`: requires balance ≥ quoted native fee.
  * `receive()`: only accepts ETH from LayerZero endpoint or timelock.

---

## Integrated Protocols

* No extra limitations imposed.
* Trusted governance of integrated protocols assumed.

---

## Compliance / EIPs

* Codebase is expected to follow standard practices; no explicit EIP constraints stated.

---

## Off-Chain Mechanisms

* Off-chain bots (e.g., keepers, arbitrage) assumed reliable unless specified.

---

## Key Invariants

* Voting power = amount of staked SUMR / SUMR in vesting wallet.

---

## Design Choices

* Core contracts: **StakingRewardsManagerBase**, **SummerGovernor**, **SummerTimelockController**.
* Selected due to prior security audits by Chainsecurity and Prototech.
* Audit summaries available in `README_GovernanceV2.md`.

---

## Previous Audits & Known Issues

* **Audited contracts**:

  * `StakingRewardsManagerBase.sol`, `ProtocolAccessManaged.sol`, `ConfigurationManaged.sol`
  * `StakingRewardsManagerBase.sol`, `SummerGovernor.sol`

**Known issue:**

* **Rewards duration immutability in StakingRewardsManagerBase**

  * `notifyRewardAmount` reverts if `newRewardsDuration` ≠ stored value.
  * Must wait until period ends, then call `setRewardsDuration`.
  * Operational impact: cannot change reward duration mid-stream.

---

## Resources

* [summer.fi](https://summer.fi)
* [Docs](https://docs.summer.fi/lazy-summer-protocol/lazy-summer-protocol)
* [Governance v2 README](https://github.com/OasisDEX/summer-earn-protocol/blob/a01063ce6d2ed917b6a104a64835a9e73b5da5f0/packages/gov-contracts/README_Governance_v2.md)
* [Gov Contracts README](https://github.com/OasisDEX/summer-earn-protocol/blob/a01063ce6d2ed917b6a104a64835a9e73b5da5f0/packages/gov-contracts/README.md)

---

## Additional Audit Context

* `StakingRewardsManagerBase` is forked from Synthetix, extended to support multiple rewards.
* `SummerStaking` inherits from it.

