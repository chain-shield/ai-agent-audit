# C4 Approved Findings

Primary approved high/medium findings from the Olas Code4rena audit.

Skipped: disputed finding `F-43`.
Omitted: proof-of-code / PoC sections.

## [F-3] Variable Overwrite in checkPoolAndGetCenterPrice() Creates Dead-Code Deviation Check, Leaving All V3 Protocol-Owned Liquidity Operations Unprotected
Risk: `High`
Source: [Code4rena F-3](https://code4rena.com/audits/2026-01-olas/submissions/F-3)
Summary: LiquidityManagerCore.checkPoolAndGetCenterPrice() is the sole TWAP oracle guard for all V3 protocol-owned liquidity operations. It protects five LiquidityManager functions that manage protocol-owned concentrated liquidity positions, plus two BuyBackBurner call sites: LiquidityManager operations (protocol-owned liquidity): convertToV3() (line 689) — migrating V2 liquidity to V3 changeRanges() (line 761) — adjusting concentrated liquidity tick ranges collectFees() (line 822) — collecting accrued trading fees decreaseLiquidity() (line 893) — removing liquidity from positions increaseLiquidity() (line 974) — adding liquidity to positions BuyBackBurner call sites: BuyBackBurner._buyOLAS() (line 231) — V3 swap path BuyBackBurner.checkPoolPrices() (line 367) — legacy compatibility The function is supposed to compare the pool's current spot price against a TWAP and revert if the deviation exceeds MAX_ALLOWED_DEVIATION (10%). However, a variable overwrite at line 1119 causes the deviation check to compare the TWAP against itself, producing a deviation of zero on every call.

## [F-6] Incorrect TWAP calculation in BalancerPriceOracle allows price manipulation and breaks oracle guarantees
Risk: `High`
Source: [Code4rena F-6](https://code4rena.com/audits/2026-01-olas/submissions/F-6)
Summary: The BalancerPriceOracle.updatePrice() function implements an incorrect Time-Weighted Average Price (TWAP) formula that does not properly accumulate price over time, making the oracle unreliable and susceptible to manipulation.
Mitigation: Implement a proper TWAP calculation that accumulates actual prices weighted by time. For this you can use the observation pattern similar to UniswapV3 where observations (timeStamp, priceCumulative) are saved in a circular buffer.

## [F-8] Insolvency via Cross-Service Reentrancy in StakingBase._withdraw
Risk: `High`
Source: [Code4rena F-8](https://code4rena.com/audits/2026-01-olas/submissions/F-8)
Summary: The _withdraw function in StakingBase.sol violates the Checks-Effects-Interactions (CEI) pattern. It caches the global balance into a local variable updatedBalance , performs external calls (transfers) via a loop, and only updates the global balance state variable at the very end of the function. Root Cause: State Update Lag: The global balance is updated after external calls return.
Mitigation: Add Reentrancy Guard: Inherit from ReentrancyGuard and apply the nonReentrant modifier to all external state-changing functions ( stake unstake claim checkpointAndClaim forcedUnstake Fix CEI Pattern: Refactor _withdraw to update the global state before performing external transfers.

## [F-215] Missing maximum bond signature parameter
Risk: `High`
Source: [Code4rena F-215](https://code4rena.com/audits/2026-01-olas/submissions/F-215)
Summary: The registerAgentsWithSignature function in the ServiceManager contract allows a service owner to register agent instances on behalf of an operator using a signed message. The signature is computed over the following parameters: operator address service owner address service ID agent instance addresses agent IDs nonce However, this message does not include any upper bound on the bond amount. As a result, if the service configuration changes after the operator has signed the message (specifically, if the bond requirement increases) the operator may unintentionally lock significantly more funds than originally intended.
Mitigation: Include a new parameter in the signed message that defines the maximum acceptable bond per agent instance. This value should be used in both the signature hash and enforced during execution to prevent unintended over-bonding.

## [F-123] Service owner can steal protocol tokens by exploiting reentrancy in create
Risk: `High`
Source: [Code4rena F-123](https://code4rena.com/audits/2026-01-olas/submissions/F-123)
Summary: ServiceManager.create() does not follow the Checks-Effects-Interactions (CEI) pattern. This allows a malicious service creator to reenter the contract and modify service parameters before ServiceRegistryTokenUtility.createWithToken() is executed: function create address serviceOwner address token bytes32 configHash uint32 memory agentIds IService AgentParams memory agentParams uint32 external returns uint256 serviceId SNIP else SNIP // Call the original ServiceRegistry contract function serviceId IService serviceRegistry create serviceOwner configHash agentIds agentParams threshold // Create a token-related record for the service IServiceTokenUtility serviceRegistryTokenUtility createWithToken serviceId token agentIds bonds
Mitigation: To enforce a secure CEI pattern and prevent reentrancy: Since ServiceRegistry.totalSupply() is known prior to minting, derive the expected serviceId in advance and call serviceRegistryTokenUtility.createWithToken() before invoking ServiceRegistry.create() . This ensures all state related to token bonds is finalized before any external calls occur.

## [F-397] Token Callback Reentrancy
Risk: `High`
Source: [Code4rena F-397](https://code4rena.com/audits/2026-01-olas/submissions/F-397)
Summary: The StakingBase contract violates the checks-effects-interactions (CEI) pattern in its claim/withdraw flow by performing external interactions through an overridable internal function _transfer(…) before persisting updated balances to storage. _claim zeroes per-service rewards and calls internal _withdraw, which computes a local updatedBalance and issues transfers by calling the internal virtual _transfer for each receiver. Because _transfer is abstract/virtual and the implementation may perform external calls or invoke token callbacks, a malicious receiver or token can re-enter the contract during these transfers.
Impact: High — An attacker who can act as a payment receiver or controls a malicious token contract can reenter claim/withdraw during _transfer and cause overlapping withdrawals that individually pass checks but cumulatively drain more tokens than the contract’s stored balance should allow. This can result in: complete or partial draining of contract-held funds, corruption/desynchronization of internal accounting, and loss of user funds.

## [F-4] Balancer oracle uses vault balances as price and can be steered by anyone
Risk: `High`
Source: [Code4rena F-4](https://code4rena.com/audits/2026-01-olas/submissions/F-4)
Summary: BalancerPriceOracle.getPrice() does not read a Balancer oracle accumulator. It reads the pool balances directly from the Vault and returns a simple reserve ratio. That value is a spot price and it can change whenever the pool is traded.
Mitigation: Do not use raw Vault balances as an oracle for slippage protection of large protocol swaps.

## [F-204] Critical Logic Inversion in Price Guard Allows Flash-Loan Manipulation of Liquidity Operations
Risk: `High`
Source: [Code4rena F-204](https://code4rena.com/audits/2026-01-olas/submissions/F-204)
Summary: The checkPoolAndGetCenterPrice function in LiquidityManagerCore.sol is intended to be a security gate that prevents the protocol from interacting with Uniswap V3 pools at manipulated "instant" prices. It is designed to verify the slot0 price against a 30-minute Time-Weighted Average Price (TWAP). However, two critical flaws in the implementation render this protection completely ineffective:
Impact: This is a Critical severity issue as it breaks the primary security assumption for price safety in the Protocol-Owned Liquidity (POL) module. It allows for the total drainage of assets committed to V3 liquidity operations via standard flash-loan price manipulation.
Mitigation: The protocol should adopt a Fail-Closed security model and correct the history logic:

## [F-461] `cumulativePrice` is corrupted when price updates are rejected
Risk: `High`
Source: [Code4rena F-461](https://code4rena.com/audits/2026-01-olas/submissions/F-461)
Summary: The BalancerPriceOracle contract updates cumulativePrice before checking whether the price deviation exceeds maxSlippage . When the slippage check fails and the update is rejected, the function returns false but the cumulativePrice has already been permanently modified. This corrupts the TWAP calculation, causing it to diverge from the expected value.
Mitigation: Move the slippage check before updating cumulativePrice , or use a local variable for the calculation and only persist to storage after validation passes.

## [F-2] Broken TWAP validation allows spot-price manipulation and renders slippage checks ineffective
Risk: `High`
Source: [Code4rena F-2](https://code4rena.com/audits/2026-01-olas/submissions/F-2)
Summary: The validatePrice() function does not compute a real Uniswap V2 TWAP and instead collapses to the current spot price, making the slippage check ineffective and trivially manipulable within a single block.
Mitigation: Implement a proper Uniswap V2 TWAP oracle pattern by persisting cumulative price snapshots across blocks and computing averages from differences between observations: Update them in a state-changing function (e.g. updateOracle()) Compute TWAPs using cumulative price deltas Correctly handle UQ112x112 fixed-point math Slippage should be evaluated only after a correct TWAP is computed.

## [F-166] Missing deadline parameter in register signatures
Risk: `High`
Source: [Code4rena F-166](https://code4rena.com/audits/2026-01-olas/submissions/F-166)
Summary: The registerAgentsWithSignature function in the ServiceManager contract allows a service owner to register agent instances on behalf of an operator using a signed message. function registerAgentsWithSignature address operator uint256 serviceId address memory agentInstances uint32 memory agentIds bytes memory external payable returns bool success // Check the service owner address serviceOwner IERC721 serviceRegistry ownerOf serviceId if msg sender serviceOwner revert OwnerOnly msg sender serviceOwner // Get the (operator | serviceId) nonce for the registerAgents message // Push a pair of key defining variables into one key. Service Id or operator are not enough by themselves // as another service might use the operator address at the same time frame // operator occupies first 160 bits uint256 operatorService uint256 uint160 operator // serviceId occupies next 32 bits as serviceId is limited by the 2^32 - 1 value operatorService serviceId 160 uint256 nonce mapOperatorRegisterAgentsNonces operatorService // Get register agents message hash bytes32 msgHash getRegisterAgentsHash operator serviceOwner serviceId agentInstances agentIds nonce // Verify the signed hash against the operator address _verifySignedHash operator msgHash signature // ...
Mitigation: Introduce a deadline parameter to the signed message and include it in the hash computation. Then, enforce that the deadline has not passed before accepting the signature during execution.

## [F-104] Uniswap oracle validatePrice can be griefed per block via `sync()`
Risk: `Medium`
Source: [Code4rena F-104](https://code4rena.com/audits/2026-01-olas/submissions/F-104)
Summary: validatePrice() returns false if the Uniswap V2 pair’s blockTimestampLast equals the current block.timestamp Because Uniswap V2 pairs have a permissionless sync() that updates the pair reserves state and sets blockTimestampLast to the current block timestamp, a mempool attacker can front-run a victim transaction in the same block by calling sync() first. When the victim later executes in that same block, blockTimestampLast == block.timestamp , so validatePrice() returns false uint256 blockTimestampLast IUniswapV2 pair getReserves if block timestamp blockTimestampLast return false This is “per-block” because the failure is tied to the block timestamp: if the victim retries in the next block (different block.timestamp ), the check can succeed again. But the attacker can repeat the same sync() front-run each time the victim tries, causing consistent failures for any public-mempool execution.
Mitigation: Avoid making validation fail solely because the pair was updated in the same block.

## [F-211] changeRanges silently fails when price is out of tick range, sending all liquidity to treasury instead of creating new position
Risk: `Medium`
Source: [Code4rena F-211](https://code4rena.com/audits/2026-01-olas/submissions/F-211)
Summary: The changeRanges function in LiquidityManagerCore.sol is designed to reposition an existing Uniswap V3 liquidity position to a new tick range. The function removes liquidity from the old position, collects all tokens, and creates a new position with the collected amounts. However, when the current pool price is outside the position's tick range, Uniswap V3's collect() returns one of the token amounts as zero (all liquidity is concentrated in one token).
Impact: All tokens from the LP position are sent to treasury instead of being redeployed to a new position. Protocol stops earning trading fees from the affected pool.
Mitigation: Handle the out-of-range case inside changeRanges() so the owner does not have to recover manually. When after _collectFees() , do not revert and do not send everything to treasury; instead, open a new position using the single token (single-sided liquidity): compute tick bounds so that the current pool price lies outside the new range, then mint the new position with that one token and update .

## [F-432] No slippage protection on Uniswap swap
Risk: `Medium`
Source: [Code4rena F-432](https://code4rena.com/audits/2026-01-olas/submissions/F-432)
Summary: The function unconditionally sets amountOutMinimum to when performing the Uniswap V3 swap. This allows the swap to execute even if the output amount is almost zero, leaving the contract unprotected against slippage or price manipulation.
Impact: An attacker can front-run or manipulate pool prices so that the contract receives almost no OLAS tokens in exchange for the input, leading to a loss of funds for the protocol.

## [F-469] Balancer oracle deadlock from cumulative price weight
Risk: `Medium`
Source: [Code4rena F-469](https://code4rena.com/audits/2026-01-olas/submissions/F-469)
Summary: The BalancerPriceOracle.updatePrice() function can enter a permanent deadlock state where price updates are perpetually rejected due to the ever-growing cumulativePrice making the time-weighted average price increasingly resistant to change . Once the cumulative weight becomes sufficiently large, any market movement exceeding maxSlippage will permanently brick the oracle.
Impact: updatePrice() returns false indefinitely, cumulativePrice and averagePrice stays frozen, validatePrice is going to work with an inaccurate slippage check, returing bad prices as valid and good prices as bad.
Mitigation: The root cause is that slippage validation occurs against the stale average price before state updates, creating a deadlock when the oracle falls behind market movements. Limit the TWAP window to something that you are ok with (30 minutes, 1 day, 1 month...).

## [F-362] `checkpoint()` does not correct `effectiveBond` downward at year boundaries where inflation decreases
Risk: `Medium`
Source: [Code4rena F-362](https://code4rena.com/audits/2026-01-olas/submissions/F-362)
Summary: 1030 `checkpoint()` does not correct `effectiveBond` downward at year boundaries where inflation decreases Alekso At the end of each epoch, checkpoint() pre-credits effectiveBond with the next epoch's maxBond (line 1279-1280): curMaxBond effectiveBond effectiveBond uint96 curMaxBond This maxBond is computed using the current inflationPerSecond . When the next epoch actually settles, the code compares the actual maxBond for the settled epoch ( , computed from the actual inflation that occurred) against the predicted maxBond that was pre-credited: // Line 1161 incentives inflationPerEpoch tp epochPoint maxBondFraction 100 // Line 1166 uint256 curMaxBond maxBond // Line 1172-1177 if incentives curMaxBond // Adjust the effectiveBond incentives effectiveBond incentives curMaxBond effectiveBond uint96 incentives The adjustment at line 1173 only fires when — i.e., when the actual bond allocation exceeds the prediction. There is no else branch for the reverse case.
Impact: At each of the two inflation-decreasing year boundaries, effectiveBond retains phantom bond capacity equal to the difference between the pre-credited maxBond (computed at the old rate) and the actual blended maxBond for the settled epoch. The PoC below demonstrates ~346,068 OLAS of phantom capacity at the Year 2→3 boundary alone, with a comparable amount at Year 9→10.
Mitigation: Add a downward correction in the else branch to subtract the over-credit when the actual maxBond is less than predicted:

## [F-374] Services can earn undeserved rewards by manipulating checkpoint timing during reward droughts
Risk: `Medium`
Source: [Code4rena F-374](https://code4rena.com/audits/2026-01-olas/submissions/F-374)
Summary: The checkpoint mechanism in StakingBase.sol#L635 only executes when rewards are available: if size block timestamp tsCheckpointLast livenessPeriod lastAvailableRewards // Activity checking and reward distribution logic When availableRewards == 0 , the global tsCheckpoint timestamp is not updated, creating time measurement gaps. Activity is measured from the last checkpoint timestamp in StakingBase.sol#L650-665 // Get the last service checkpoint: staking start time or the global checkpoint timestamp uint256 serviceCheckpoint tsCheckpointLast // Uses stale timestamp when no rewards uint256 ts sInfo tsStart if ts serviceCheckpoint serviceCheckpoint ts // Calculate activity over the entire gap period ts block timestamp serviceCheckpoint bool ratioPass _checkRatioPass sInfo multisig sInfo nonces ts The activity check in StakingActivityChecker.sol#L54-59 calculates the ratio over the entire time period: if ts curNonces lastNonces uint256 ratio curNonces lastNonces 1e18 ts ratioPass ratio livenessRatio This allows services to appear active by executing transactions only at the end of long inactive periods.
Impact: Services receive compensation for periods of inactivity Tools Used Manual code review, Foundry
Mitigation: Separate activity tracking from reward distribution by maintaining activity state regardless of reward availability: function _calculateStakingRewards internal view returns uint256 tsCheckpointLast tsCheckpoint lastAvailableRewards availableRewards // Always track activity regardless of reward availability if size block timestamp tsCheckpointLast livenessPeriod // Track activity and inactivity _updateServiceActivity serviceIds tsCheckpointLast // Only distribute rewards if available if lastAvailableRewards _calculateRewardDistribution serviceIds lastAvailableRewards

## [F-206] `BalancerPriceOracle::validatePrice` uses stale TWAP
Risk: `Medium`
Source: [Code4rena F-206](https://code4rena.com/audits/2026-01-olas/submissions/F-206)
Summary: The BalancerPriceOracle contract's validatePrice() function calculates the time-weighted average price using potentially outdated snapshot data because it never calls updatePrice() to refresh the oracle state before validation. The root cause is that validatePrice() is a view function that only reads from storage without ensuring the underlying TWAP data has been recently updated current: function validatePrice uint256 slippage external view returns bool // ... // Compute time-weighted average price uint256 timeWeightedAverage snapshot cumulativePrice snapshot averagePrice elapsedTime snapshot cumulativePrice snapshot averagePrice elapsedTime uint256 tradePrice getPrice // ...
Mitigation: There are different approaches to mitigate this issue: Modify validatePrice() to call updatePrice() internally before performing the TWAP calculation. This requires changing validatePrice() from a view function to a state-modifying function.

## [F-329] Incorrect proportional reward splits when an operator has been slashed.
Risk: `Medium`
Source: [Code4rena F-329](https://code4rena.com/audits/2026-01-olas/submissions/F-329)
Summary: The StakingBase contract allows service owners to select a reward distribution type when allocating rewards. The available types are: enum RewardDistributionType // Rewards are divided as per where stake comes from, proportional Proportional // Rewards go to service owner ServiceOwner // Rewards go to service multisig ServiceMultisig // Custom rewards distribution The function _getRewardReceiversAndAmounts determines reward allocations based on the selected distribution type. For the Proportional type: if rewardDistributionType RewardDistributionType Proportional // Get service agent instances uint256 numInstances address memory agentInstances IService serviceRegistry getAgentInstances serviceId uint256 totalNumReceivers numInstances // Allocate arrays receivers new address totalNumReceivers amounts new uint256 totalNumReceivers // Default setup implies that all bonds are equal // Get each operator reward uint256 operatorReward reward totalNumReceivers // Get corresponding operators and set operators reward amounts for uint256 i i numInstances i receivers i IService serviceRegistry mapAgentInstanceOperators agentInstances i amounts i operatorReward // Set service owner address and its reward amount receivers numInstances serviceOwner // Service owner gets its reward amount and a division remainder, if any amounts numInstances reward numInstances operatorReward As noted in the comments, this logic assumes all operator bonds are equal.
Mitigation: Update _getRewardReceiversAndAmounts so that when the distribution type is Proportional , the function queries the actual bond amounts for each operator. Operators with reduced or zero bonds (due to slashing) should receive proportionally less or no rewards.

## [F-67] Arbitrum Retryable-Ticket Refund/Value Not Verified Enables Timelock ETH Exfiltration
Risk: `Medium`
Source: [Code4rena F-67](https://code4rena.com/audits/2026-01-olas/submissions/F-67)
Summary: GuardCM is meant to restrict what the Community Multisig (CM) can schedule through the timelock by allowing only specific target + selector (+ chainId) combinations. For Arbitrum-bridged actions, the current verification only checks the L2 targetAddress and the L2 targetPayload selector, while ignoring (1) the timelock value forwarded on execution and (2) critical Arbitrum retryable-ticket parameters (notably the refund recipients). As a result, CM can schedule an allowlisted L2 action but still divert timelock ETH via attacker-controlled refund addresses.
Impact: High severity: loss of funds (timelock ETH) via attacker-controlled refund recipients, despite the guard being active. Bypasses the intended security boundary of GuardCM: CM can schedule “allowlisted” L2 actions while still abusing unverified parameters to redirect L1 value.

## [F-280] Malicious user can prevent buying back OLAS token via Slipstream
Risk: `Medium`
Source: [Code4rena F-280](https://code4rena.com/audits/2026-01-olas/submissions/F-280)
Summary: Whenever buying back olas occurs via Slipstream V3 it goes through buyBack first: function buyBack address secondToken uint256 secondTokenAmount int24 feeTierOrTickSpacing external virtual // Reentrancy guard if _locked revert ReentrancyGuard _locked // Get token balance uint256 balance IERC20 secondToken balanceOf address this // Adjust second token amount, if needed if secondTokenAmount secondTokenAmount balance secondTokenAmount balance if secondTokenAmount revert ZeroValue // Record msg.sender activity mapAccountActivities msg sender // Buy OLAS uint256 olasAmount _buyOLAS secondToken secondTokenAmount feeTierOrTickSpacing emit BuyBack secondToken secondTokenAmount olasAmount // Get OLAS contract balance olasAmount IERC20 olas balanceOf address this // Transfer OLAS to bridge2Burner contract IERC20 olas transfer bridge2Burner olasAmount emit TokenTransferred bridge2Burner olasAmount _locked Then _buyOLAS is called: function _buyOLAS address secondToken uint256 secondTokenAmount int24 feeTierOrTickSpacing returns uint256 olasAmount address localOlas olas address memory tokens new address tokens tokens secondToken localOlas localOlas secondToken secondToken localOlas // Get factory from LiquidityManager // Actual factoryV3 is fetched from LiquidityManager, since LiquidityManager is proxy and factory might change address factoryV3 ILiquidityManager liquidityManager factoryV3 // Get V3 pool from liquidity manager address pool getV3Pool factoryV3 tokens feeTierOrTickSpacing // Check for whitelisted pool address if mapV3Pools pool revert UnauthorizedPool pool // Apply slippage protection ILiquidityManager liquidityManager checkPoolAndGetCenterPrice pool // Perform swap to OLAS olasAmount _performSwap secondToken secondTokenAmount feeTierOrTickSpacing Then it will invoke _performSwap on BuyBackBurnerBalancer.sol /// @dev Performs swap for OLAS on Slipstream CL DEX. /// @param secondToken Second token address. /// @param secondTokenAmount Second token amount.
Impact: Buying back olas token and transferring it to Bridge2Burner will be impossible and DoS-ed.
Mitigation: In order to prevent this type of attack add receive or fallback functions to the implementation.

## [F-126] Price cumulative last is used inverted in Uniswap Oracle
Risk: `Medium`
Source: [Code4rena F-126](https://code4rena.com/audits/2026-01-olas/submissions/F-126)
Summary: Currently, in UniswapPriceOracle the function validatePrice() is: function validatePrice uint256 slippage external view returns bool require slippage maxSlippage "Slippage overflow" // Compute time-weighted average price // Fetch the cumulative prices from the pair uint256 cumulativePriceLast if direction cumulativePriceLast IUniswapV2 pair price1CumulativeLast // @audit appears to be inverted, ts should be price0cumulative if direction is 0 else cumulativePriceLast IUniswapV2 pair price0CumulativeLast // Fetch the reserves and the last block timestamp uint256 blockTimestampLast IUniswapV2 pair getReserves // Require at least one block since last update if block timestamp blockTimestampLast return false uint256 elapsedTime block timestamp blockTimestampLast uint256 tradePrice getPrice // Calculate cumulative prices uint256 cumulativePrice cumulativePriceLast tradePrice elapsedTime // Calculate the TWAP for OLAS in terms of native token uint256 timeWeightedAverage cumulativePrice cumulativePriceLast elapsedTime // Get the final derivation to compare with slippage // Final derivation value must be uint256 derivation tradePrice timeWeightedAverage tradePrice timeWeightedAverage 1e16 timeWeightedAverage tradePrice 1e16 timeWeightedAverage return derivation slippage The meanings of priceCumulativeLast is that, if: price0CumulativeLast -> will return the price of token1 denominated in token0 price1CumulativeLast -> will return the price of token0 denominated in token1 In constructor, we build out the direction that will denominate who is token0 and who is token1 as follows constructor address _secondToken uint256 _maxSlippage address _pair pair _pair maxSlippage _maxSlippage // Get token direction address token0 IUniswapV2 pair token0 if token0 _secondToken direction The whole goal of the oracle is to give the price of OLAS in units of second token (This is because it will be used in BuyBack since we swap a given second token to OLAS). If the direction is 0, which means the secondToken is the token0, then we will fetch price1CumulativeLast if direction cumulativePriceLast IUniswapV2 pair price1CumulativeLast // @audit appears to be inverted, ts should be price0cumulative if direction is 0 Which will return the price of token0 denominated in token1 (Price of second token denominated in olas, which is exactly the inverse of what we want). When direction is 1, which means the second token is the token1, we will fetch price0CumulativeLast , which returns the price of token1 denominated in token0 (Price of second token denominated in olas again).
Impact: If the price moves from 2 -> 1 for instance, we will compute as it was 0.5 -> 1 which is a gain of 100%, given it is the inverse. And therefore the slippage check may pass when it shouldn't.
Mitigation: if direction cumulativePriceLast IUniswapV2 pair price0CumulativeLast // @audit appears to be inverted, ts should be price0cumulative if direction is 0 else cumulativePriceLast IUniswapV2 pair price1CumulativeLast

## [F-175] DoS in Liquidity Migration due to Unit Mismatch in UniswapPriceOracle
Risk: `Medium`
Source: [Code4rena F-175](https://code4rena.com/audits/2026-01-olas/submissions/F-175)
Summary: The UniswapPriceOracle.validatePrice() function suffers from a logic error due to a unit mismatch. It calculates price derivation using 1e16 precision but compares it against a slippage parameter provided in raw percentage units. This causes the validation to fail for almost any price deviation, effectively causing a Denial of Service (DoS) for the V2-to-V3 liquidity migration in LiquidityManagerETH
Impact: Users are unable to migrate liquidity from Uniswap V2 to V3, rendering the feature unusable.
Mitigation: Scale the slippage parameter to match the 1e16 precision used for derivation Recommended Fix: function validatePrice uint256 slippage external view returns bool require slippage maxSlippage "Slippage overflow" // ... calculate derivation ...
