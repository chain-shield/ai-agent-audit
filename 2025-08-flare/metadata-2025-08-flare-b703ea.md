
## SUMMARY OF FILE: 2025-08-flare/contracts/agentOwnerRegistry/implementation/AgentOwnerRegistry.sol
# AgentOwnerRegistry (UUPS, Governance-controlled)

Registry for agent owners. Governance (or a designated manager) whitelists management addresses, manages agent presentation metadata, and maps each management address to a single work address. No user funds are held. Trust model: governance-admin upgradeable via GovernedUUPSProxyImplementation; an optional manager can act for governance on whitelist/metadata ops. Major entrypoints: initialize, setManager, whitelistAndDescribeAgent, revokeAddress, setWorkAddress, agent metadata setters, getters, whitelist checker, ERC-165 support.

## Storage
- manager — address — whitelist admin
- whitelist — mapping(address=>bool) — mgmt wl flag
- workToMgmtAddress — mapping(address=>address) — work→mgmt map
- mgmtToWorkAddress — mapping(address=>address) — mgmt→work map
- agentName — mapping(address=>string) — name by mgmt
- agentDescription — mapping(address=>string) — desc by mgmt
- agentIconUrl — mapping(address=>string) — icon URL by mgmt
- agentTouUrl — mapping(address=>string) — terms URL by mgmt

## Functions
- function initialize(IGovernanceSettings _governanceSettings, address _initialGovernance) external;  // nonpayable
  - Initialize governance settings and set initial governance.

- function revokeAddress(address _address) external onlyGovernanceOrManager;  // nonpayable
  - Remove management address from whitelist.

- function setManager(address _manager) external onlyGovernance;  // nonpayable
  - Set/replace manager address authorized for ops.

- function whitelistAndDescribeAgent(address _managementAddress, string memory _name, string memory _description, string memory _iconUrl, string memory _touUrl) external onlyGovernanceOrManager;  // nonpayable
  - Whitelist agent and set presentation metadata.

- function setWorkAddress(address _ownerWorkAddress) external;  // nonpayable
  - From mgmt address: set unique work address.

- function setAgentName(address _managementAddress, string memory _name) external onlyGovernanceOrManager;  // nonpayable
  - Update agent owner name for management address.

- function setAgentDescription(address _managementAddress, string memory _description) external onlyGovernanceOrManager;  // nonpayable
  - Update agent owner description text.

- function setAgentIconUrl(address _managementAddress, string memory _iconUrl) external onlyGovernanceOrManager;  // nonpayable
  - Update agent owner icon URL.

- function setAgentTermsOfUseUrl(address _managementAddress, string memory _touUrl) external onlyGovernanceOrManager;  // nonpayable
  - Update agent terms-of-use URL.

- function getAgentName(address _managementAddress) external view override returns (string memory);
  - Read agent owner name for management address.

- function getAgentDescription(address _managementAddress) external view override returns (string memory);
  - Read agent owner description for management address.

- function getAgentIconUrl(address _managementAddress) external view override returns (string memory);
  - Read agent owner icon URL for management address.

- function getAgentTermsOfUseUrl(address _managementAddress) external view override returns (string memory);
  - Read agent owner terms-of-use URL.

- function getWorkAddress(address _managementAddress) external view override returns (address);
  - Get current work address for management address.

- function getManagementAddress(address _workAddress) external view override returns (address);
  - Resolve management address from a work address.

- function isWhitelisted(address _address) public view override returns (bool);
  - Check if management address is whitelisted.

- function supportsInterface(bytes4 _interfaceId) public pure override returns (bool);
  - ERC-165 support for IERC165 and IAgentOwnerRegistry.

## Internal/Private
- function _addAddressToWhitelist(address _address) internal;  // nonpayable
  - Add address to whitelist; emit Whitelisted.

- function _removeAddressFromWhitelist(address _address) internal;  // nonpayable
  - Remove address from whitelist; emit Revoked.

- function _setAgentData(address _managementAddress, string memory _name, string memory _description, string memory _iconUrl, string memory _touUrl) private;  // nonpayable
  - Set all agent metadata and emit event.

- function _emitDataChanged(address _managementAddress) private;  // nonpayable
  - Emit AgentDataChanged with current metadata.

## Notes
- Access control: onlyGovernance from parent; onlyGovernanceOrManager defined here.
- Emits (from interface): Whitelisted, WhitelistingRevoked, AgentDataChanged, WorkAddressChanged, ManagerChanged.
- Enforcement: setWorkAddress requires caller whitelisted and work address uniqueness.
- Upgradeable: GovernedUUPSProxyImplementation; initialize must be called once.


## SUMMARY OF FILE: 2025-08-flare/contracts/agentOwnerRegistry/implementation/AgentOwnerRegistryProxy.sol
Summary

A minimal ERC1967 proxy for AgentOwnerRegistry. It deploys a UUPS-compatible proxy pointing to an AgentOwnerRegistry implementation and atomically runs initialize(governanceSettings, initialGovernance). Trust model: no user funds held here; all logic and state live in the implementation. Upgrades are governed by the implementation’s UUPS logic and governance settings. Major entrypoints are the fallback/receive that delegate all calls to the implementation.

Storage

- implementation (EIP-1967 slot) — delegate target
- (no local vars) — all other storage in implementation

Functions

- constructor(address _implementationAddress, IGovernanceSettings _governanceSettings, address _initialGovernance) nonpayable
  Summary: Deploy proxy and invoke AgentOwnerRegistry.initialize with governance settings.

- fallback() external payable
  Summary: Delegates unknown calls to current implementation.

- receive() external payable
  Summary: Accept ETH and delegate if data empty, per Proxy behavior.


## SUMMARY OF FILE: 2025-08-flare/contracts/agentVault/implementation/AgentVault.sol
# AgentVault.sol — Summary
A UUPS-upgradeable vault holding an agent’s ERC‑20 collateral and integrating with the system AssetManager and per‑agent CollateralPool. Owner (as recognized by AssetManager) can deposit/withdraw approved collateral, buy CPTs, redeem CPTs, and sweep non‑collateral tokens. AssetManager alone can trigger payouts, destroy the vault, and authorize upgrades. Non‑reentrancy guards token outflows. Trust: funds are agent collateral; admin authority is centralized in AssetManager for upgrades/payouts; owner actions are enforced and rate‑limited via AssetManager.

## Storage
- assetManager (IIAssetManager): asset manager addr — practically immutable
- initialized (bool): init flag
- __usedTokens (IERC20[]): storage placeholder
- __tokenUseFlags (mapping(IERC20=>uint256)): storage placeholder
- __internalWithdrawal (bool): storage placeholder
- destroyed (bool): vault destroyed flag

## Functions
- `constructor(IIAssetManager _assetManager)` — nonpayable
  - Natspec: Test-only constructor; delegates to initialize.

- `function initialize(IIAssetManager _assetManager) public` — nonpayable
  - Modifiers: —
  - Natspec: One-time initializer; sets asset manager and reentrancy guard.

- `function buyCollateralPoolTokens() external payable onlyOwner` — payable
  - Natspec: Enter collateral pool by sending native value; mints CPTs to vault.

- `function withdrawPoolFees(uint256 _amount, address _recipient) external onlyOwner` — nonpayable
  - Natspec: Withdraw earned FAsset fees from pool to recipient.

- `function redeemCollateralPoolTokens(uint256 _amount, address payable _recipient) external onlyOwner nonReentrant` — nonpayable
  - Natspec: Exit pool for CPTs; checks with AssetManager before redemption.

- `function depositCollateral(IERC20 _token, uint256 _amount) external override onlyOwner onlyKnownToken(_token)` — nonpayable
  - Natspec: Pull approved ERC‑20 into vault and update collateral accounting.

- `function updateCollateral(IERC20 _token) external override onlyOwner onlyKnownToken(_token)` — nonpayable
  - Natspec: Notify AssetManager to recalc collateral after external transfers.

- `function withdrawCollateral(IERC20 _token, uint256 _amount, address _recipient) external override onlyOwner onlyKnownToken(_token) nonReentrant` — nonpayable
  - Natspec: Withdraw ERC‑20 to recipient; respects withdrawal announcements unless destroyed.

- `function transferExternalToken(IERC20 _token, uint256 _amount) external override onlyOwner` — nonpayable
  - Natspec: Sweep non‑collateral airdrops to owner’s management address.

- `function destroy() external override onlyAssetManager nonReentrant` — nonpayable
  - Natspec: Mark vault destroyed so owner can freely withdraw funds.

- `function payout(IERC20 _token, address _recipient, uint256 _amount) external override onlyAssetManager nonReentrant` — nonpayable
  - Natspec: AssetManager-directed payout for liquidation or failed redemption.

- `function collateralPool() public view returns (ICollateralPool)` — view
  - Natspec: Resolve this vault’s CollateralPool from AssetManager.

- `function isOwner(address _address) public view returns (bool)` — view
  - Natspec: Check if address is recognized vault owner by AssetManager.

- `function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)` — pure
  - Natspec: ERC‑165 support for IERC165, IAgentVault, IIAgentVault.

- `function implementation() external view returns (address)` — view
  - Natspec: Return current implementation address (UUPS helper).

- `function _authorizeUpgrade(address _newImplementation) internal virtual override onlyAssetManager` — nonpayable
  - Natspec: Restrict upgrades to AssetManager authority.

- `function _validateToken(IERC20 _token) private view` — view
  - Natspec: Ensure token is an approved vault collateral token.

## Modifiers
- `onlyOwner`: Caller must be recognized as owner by AssetManager.
- `onlyAssetManager`: Caller must be the configured AssetManager.
- `onlyKnownToken(IERC20)`: Token must be an approved collateral token.

## Notes
- ReentrancyGuard protects token-out flows; initializeReentrancyGuard called in initialize.
- Uses SafeERC20 for safe transfers.
- Errors referenced (e.g., OnlyOwner, UnknownToken) are custom errors defined elsewhere.
- UUPSUpgradeable; upgrades only via AssetManager calls.


## SUMMARY OF FILE: 2025-08-flare/contracts/agentVault/implementation/AgentVaultFactory.sol
# AgentVaultFactory.sol — Summary
Factory contract that deploys AgentVault instances as minimal ERC1967 proxies pointing to a shared implementation. Anyone can call create to instantiate and initialize a new AgentVault for a given AssetManager. The factory itself holds no user funds and has no admin controls; upgrade/admin rights for each deployed proxy reside in the AgentVault implementation/admin logic. Major entrypoints: create (deploy + initialize), upgradeInitCall (returns post-upgrade init payload), supportsInterface (ERC-165).

## Storage
- implementation (address) — logic for new proxies

## Functions
- constructor(address _implementation)
  - visibility: public
  - mutability: nonpayable
  - modifiers: none
  - Natspec: Set immutable implementation used by new ERC1967 proxies.

- function create(IIAssetManager _assetManager) external returns (IIAgentVault)
  - mutability: nonpayable
  - modifiers: none
  - Natspec: Deploy ERC1967 proxy for AgentVault and initialize with AssetManager.

- function upgradeInitCall(address _proxy) external pure override returns (bytes memory)
  - mutability: pure
  - modifiers: none
  - Natspec: Return bytes payload for upgradeToAndCall; empty for current version.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  - mutability: pure
  - modifiers: none
  - Natspec: ERC-165 support for IERC165 and IIAgentVaultFactory interfaces.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentAlwaysAllowedMintersFacet.sol
Summary
A minimal diamond facet for the AssetManager that lets each agent vault owner maintain a per-agent whitelist of “always-allowed” minter addresses. No funds are handled; writes update the Agent.State stored via the Agent library. Trust model: only the specific agent vault owner can add/remove whitelist entries; read access is public. Major entrypoints: addAlwaysAllowedMinterForAgent, removeAlwaysAllowedMinterForAgent, alwaysAllowedMintersForAgent.

Storage
- Agent.State.alwaysAllowedMinters — whitelisted minters set (per vault)

Functions
- function addAlwaysAllowedMinterForAgent(address _agentVault, address _minter) external onlyAgentVaultOwner(_agentVault)
  /// Add a minter to agent’s always-allowed whitelist.

- function removeAlwaysAllowedMinterForAgent(address _agentVault, address _minter) external onlyAgentVaultOwner(_agentVault)
  /// Remove a minter from agent’s always-allowed whitelist.

- function alwaysAllowedMintersForAgent(address _agentVault) external view returns (address[] memory)
  /// Enumerate all always-allowed minters for the given agent vault.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentCollateralFacet.sol
# AgentCollateralFacet Summary

Trust model: Manages agent collateral inside the AssetManager diamond. No direct user funds; actions gated to agent vault owner or contracts (vault/pool). Enforces withdrawal timelocks, CR checks, and collateral token upgrades. Major entrypoints: announceVaultCollateralWithdrawal, announceAgentPoolTokenRedemption, beforeCollateralWithdrawal, updateCollateral, switchVaultCollateral, upgradeWNatContract.

## Storage Variables
- (none) — facet uses diamond/library state

## Custom Errors
- WithdrawalInvalidAgentStatus — Agent not NORMAL or not fully liquidated
- WithdrawalNotAnnounced — No prior announcement
- WithdrawalMoreThanAnnounced — Amount exceeds announced
- WithdrawalNotAllowedYet — Timelock not elapsed
- WithdrawalTooLate — Operation window passed
- WithdrawalCRTooLow — Post-withdraw CR below minting threshold
- WithdrawalValueTooHigh — Not enough free collateral
- OnlyAgentVaultOrPool — Unauthorized caller
- CollateralNotDeprecated — Current vault token not deprecated
- CollateralWithdrawalAnnounced — Withdrawal in progress
- FAssetNotTerminated — Not used here

## Functions

- interface: function announceVaultCollateralWithdrawal(address _agentVault, uint256 _valueNATWei) external onlyAgentVaultOwner(_agentVault) returns (uint256 _withdrawalAllowedAt);
  natspec: Announce vault collateral withdrawal; sets timelock and locks free collateral.

- interface: function announceAgentPoolTokenRedemption(address _agentVault, uint256 _valueNATWei) external onlyAgentVaultOwner(_agentVault) returns (uint256 _redemptionAllowedAt);
  natspec: Announce agent pool token redemption; applies same timelock mechanics.

- interface: function beforeCollateralWithdrawal(IERC20 _token, uint256 _amountWei) external;
  natspec: Pre-withdrawal hook from AgentVault; enforces announcements, windows, and CR safety.

- interface: function updateCollateral(address _agentVault, IERC20 _token) external;
  natspec: Vault/Pool deposit hook; may end liquidation if now healthy.

- interface: function switchVaultCollateral(address _agentVault, IERC20 _token) external onlyAgentVaultOwner(_agentVault);
  natspec: Switch deprecated vault collateral to a new approved token.

- interface: function upgradeWNatContract(address _agentVault) external onlyAgentVaultOwner(_agentVault);
  natspec: Upgrade pool WNat implementation to governance-selected token; sync pool.

- interface: function _announceWithdrawal(Collateral.Kind _kind, address _agentVault, uint256 _amountWei) private returns (uint256);
  natspec: Core announcer; validates free collateral, sets/cancels timelock, emits events.

## Events Emitted
- VaultCollateralWithdrawalAnnounced(agentVault, amount, allowedAt)
- PoolTokenRedemptionAnnounced(agentVault, amount, allowedAt)
- AgentCollateralTypeChanged(agentVault, class, token)



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentInfoFacet.sol
# AgentInfoFacet summary

Read-only facet of AssetManager diamond exposing agent discovery and state views. It holds no funds, defines no storage, and enforces no admin gating; it only reads diamond storage via library accessors. Users, integrators, and bots query agent lists, fee/CR settings, collateral balances, underlying balances, and liquidation parameters. Major entrypoints: `getAgentInfo`, `getAllAgents`, `getAgentLiquidationFactorsAndMaxAmount`, and min/max CR/collateral getters.

Storage variables
- None — uses diamond storage via libraries

Functions
- `function getAllAgents(uint256 _start, uint256 _end) external view returns (address[] memory _agents, uint256 _totalLength)`
  - visibility: external view; modifiers: none; mutability: view
  - natspec: Paged retrieval of all agent vault addresses with total count.

- `function isPoolTokenSuffixReserved(string memory _suffix) external view returns (bool)`
  - visibility: external view; modifiers: none; mutability: view
  - natspec: Checks if a pool token symbol suffix is already reserved.

- `function getAgentInfo(address _agentVault) external view returns (AgentInfo.Info memory _info)`
  - visibility: external view; modifiers: none; mutability: view
  - natspec: Returns comprehensive agent state: fees, CRs, balances, statuses, and liquidation data.

- `function getCollateralPool(address _agentVault) external view returns (address)`
  - visibility: external view; modifiers: none; mutability: view
  - natspec: Returns collateral pool contract address for the agent.

- `function getAgentVaultOwner(address _agentVault) external view returns (address _ownerManagementAddress)`
  - visibility: external view; modifiers: none; mutability: view
  - natspec: Returns agent’s owner management address.

- `function getAgentVaultCollateralToken(address _agentVault) external view returns (IERC20)`
  - visibility: external view; modifiers: none; mutability: view
  - natspec: Returns ERC20 token used as agent’s vault collateral.

- `function getAgentFullVaultCollateral(address _agentVault) external view returns (uint256)`
  - visibility: external view; modifiers: none; mutability: view
  - natspec: Returns total vault collateral amount in wei.

- `function getAgentFullPoolCollateral(address _agentVault) external view returns (uint256)`
  - visibility: external view; modifiers: none; mutability: view
  - natspec: Returns total pool collateral amount in wei.

- `function getAgentLiquidationFactorsAndMaxAmount(address _agentVault) external view returns (uint256 _liquidationPaymentFactorVaultBIPS, uint256 _liquidationPaymentFactorPoolBIPS, uint256 _maxLiquidationAmountUBA)`
  - visibility: external view; modifiers: none; mutability: view
  - natspec: For agents in liquidation, returns split factors and max liquidatable amount.

- `function getAgentMinPoolCollateralRatioBIPS(address _agentVault) external view returns (uint256)`
  - visibility: external view; modifiers: none; mutability: view
  - natspec: Returns system minimum pool CR (BIPS) applied to agent.

- `function getAgentMinVaultCollateralRatioBIPS(address _agentVault) external view returns (uint256)`
  - visibility: external view; modifiers: none; mutability: view
  - natspec: Returns system minimum vault CR (BIPS) applied to agent.

Internal/private helpers
- `function _getFullCollateral(address _agentVault, Collateral.Kind _kind) private view returns (uint256)`
  - visibility: private view; modifiers: none; mutability: view
  - natspec: Reads total collateral for selected kind (VAULT or POOL).

- `function _getMinCollateralRatioBIPS(address _agentVault, Collateral.Kind _kind) private view returns (uint256)`
  - visibility: private view; modifiers: none; mutability: view
  - natspec: Returns system minimum CR for the given collateral kind.

- `function _getLiquidationFactorsAndMaxAmount(Agent.State storage _agent, Liquidation.CRData memory _cr) private view returns (uint256 _vaultFactorBIPS, uint256 _poolFactorBIPS, uint256 _maxLiquidatedUBA)`
  - visibility: private view; modifiers: none; mutability: view
  - natspec: Computes liquidation split factors and max liquidatable amount if in liquidation.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentPingFacet.sol
# AgentPingFacet (Asset Manager Diamond Facet)

A minimal facet enabling off-chain liveness and metadata “ping” flows for agent vaults. It holds no funds and modifies no protocol balances; it only emits events for off-chain listeners. Any address can ping an agent; only the vault’s owner/management address can respond. Trust model: non-custodial, stateless; responses are restricted via onlyAgentVaultOwner. Major entrypoints: agentPing (emit ping) and agentPingResponse (emit validated response).

## Storage Variables
- None (facet declares no storage)

Notes: Reads Agent.State via library to fetch ownerManagementAddress for emitted response.

## Functions

- Interface: `function agentPing(address _agentVault, uint256 _query) external nonpayable`
  - NatSpec: Emit a ping targeting an agent vault with caller and correlation id.

- Interface: `function agentPingResponse(address _agentVault, uint256 _query, string memory _response) external nonpayable onlyAgentVaultOwner(_agentVault)`
  - NatSpec: Agent vault owner answers a ping; emits management address, query id, and response text.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentSettingsFacet.sol
# AgentSettingsFacet

Trust model and purpose (≤100 words):
AgentSettingsFacet is a diamond facet of the Asset Manager that lets an agent vault owner time-lock and mutate per-agent parameters affecting fees and collateral thresholds. It holds no user funds and relies on diamond/library storage (Agent/Globals). Governance configures timelocks and execution windows; the agent vault owner is the only actor permitted to announce/execute updates. Major entrypoints: announceAgentSettingUpdate, executeAgentSettingUpdate, and getAgentSetting.

Storage variables
- None (uses diamond/library storage)

Constants (hashed setting keys; not stored)
- FEE_BIPS — "feeBIPS"
- POOL_FEE_SHARE_BIPS — "poolFeeShareBIPS"
- REDEMPTION_POOL_FEE_SHARE_BIPS — "redemptionPoolFeeShareBIPS"
- MINTING_VAULT_COLLATERAL_RATIO_BIPS — "mintingVaultCollateralRatioBIPS"
- MINTING_POOL_COLLATERAL_RATIO_BIPS — "mintingPoolCollateralRatioBIPS"
- BUY_FASSET_BY_AGENT_FACTOR_BIPS — "buyFAssetByAgentFactorBIPS"
- POOL_EXIT_COLLATERAL_RATIO_BIPS — "poolExitCollateralRatioBIPS"

Errors
- NoPendingUpdate — No announced update found for the setting
- UpdateNotValidYet — Timelock not expired for the update
- UpdateNotValidAnymore — Execution window expired
- InvalidSettingName — Name not in allowed set

Events (emitted via IAssetManagerEvents)
- AgentSettingChangeAnnounced(agentVault, name, value, validAt)
- AgentSettingChanged(agentVault, name, value)

Functions
- function announceAgentSettingUpdate(address _agentVault, string memory _name, uint256 _value)
  external onlyAgentVaultOwner(_agentVault) nonpayable returns (uint256 _updateAllowedAt);
  — Announce setting change; schedules execution after timelock and stores pending value.

- function executeAgentSettingUpdate(address _agentVault, string memory _name)
  external onlyAgentVaultOwner(_agentVault) nonpayable;
  — Execute a previously announced setting once timelock passes and within execution window.

- function getAgentSetting(address _agentVault, string memory _name)
  external view returns (uint256 _value);
  — Read current value of a specific agent setting by name.

- function _executeUpdate(Agent.State storage _agent, bytes32 _hash, uint256 _value)
  private nonpayable;
  — Dispatch to AgentUpdates library to set the selected field by key.

- function _getTimelock(bytes32 _hash)
  private view returns (uint64);
  — Resolve timelock seconds based on setting type (fee, minting CR, or pool-exit CR).

- function _getAndCheckHash(string memory _name)
  private pure returns (bytes32);
  — Hash and validate name against supported keys; reverts if invalid.

Notes
- Timelocks sourced from Globals.getSettings():
  - agentFeeChangeTimelockSeconds for fee-like settings (feeBIPS, poolFeeShareBIPS, redemptionPoolFeeShareBIPS, buyFAssetByAgentFactorBIPS)
  - agentMintingCRChangeTimelockSeconds for minting CR settings
  - poolExitCRChangeTimelockSeconds for pool exit CR
- Execution window enforced via settings.agentTimelockedOperationWindowSeconds.
- Settings affect: agent.feeBIPS, pool fee shares, redemption pool fee share, minting collateral ratios, agent buyback factor, and pool exit CR via CollateralPool.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentVaultAndPoolSupportFacet.sol
# AgentVaultAndPoolSupportFacet

Lightweight, read-only Diamond facet exposing helper views for agents, vaults, and pool collateral. It holds no funds and defines no storage; it only reads diamond storage via libraries. Trust model: non-custodial, no privileged admin paths here. Major entrypoints: assetPriceNatWei, isLockedVaultToken, isVaultCollateralToken, getFAssetsBackedByPool, isAgentVaultOwner, getWorkAddress, getWNat.

## Storage Variables
- None (facet has no direct state)

## Functions
- function assetPriceNatWei() external view returns (uint256 _multiplier, uint256 _divisor)
  - Returns NAT-wei price per UBA as fraction multiplier/divisor.

- function isLockedVaultToken(address _agentVault, IERC20 _token) external view returns (bool)
  - True if token is agent’s vault collateral or the pool token (locked flow rules apply).

- function isVaultCollateralToken(IERC20 _token) external view returns (bool)
  - Checks if token is any vault collateral type (even deprecated) approved by governance.

- function getFAssetsBackedByPool(address _agentVault) external view returns (uint256)
  - Total UBA backed by pool for agent: reserved + minted + poolRedeeming, converted from AMG.

- function isAgentVaultOwner(address _agentVault, address _address) external view returns (bool)
  - Verifies whether address is an owner/manager of the specified agent vault.

- function getWorkAddress(address _managementAddress) external view returns (address)
  - Resolves hot/work address for a given cold/management address via AgentOwnerRegistry.

- function getWNat() external view returns (IWNat)
  - Returns the configured WNat contract address used across the system.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentVaultManagementFacet.sol
# AgentVaultManagementFacet — summary
This Diamond facet manages agent lifecycle: creation, destruction, and upgrades of Agent Vaults, Collateral Pools, and Pool Tokens. Trust model: user funds live in per‑agent vaults/pools; critical actions are permissioned. Only whitelisted agent owners can create/destroy/upgrade their agents; governance (controller) can batch‑upgrade. Time‑locks protect withdrawals/destroy. Major entrypoints: createAgentVault, announceDestroyAgent, destroyAgent, upgradeAgentVaultAndPool, upgradeAgentVaultsAndPools.

## Storage variables
- None (facet holds no state; uses diamond storage via libraries)
- MIN_SUFFIX_LEN (uint256) — min suffix len
- MAX_SUFFIX_LEN (uint256) — max suffix len

Diamond storage accessed (via libraries):
- AssetManagerState.State: reservedPoolTokenSuffixes, currentUnderlyingBlock, allAgents, poolCollateralIndex, underlyingAddressOwnership
- Agent.State (per agentVault): status, ownerManagementAddress, collateral config/ratios, fees, collateralPool, positions
- Globals settings and registries

## Functions
- `createAgentVault(IAddressValidity.Proof calldata _addressProof, AgentSettings.Data calldata _settings) external onlyAttached returns (address _agentVault)` [nonpayable]
  - Creates a new agent vault, validates underlying address, deploys pool and token, emits creation.

- `announceDestroyAgent(address _agentVault) external onlyAgentVaultOwner(_agentVault) returns (uint256 _destroyAllowedAt)` [nonpayable]
  - Starts destroy timer; requires no availability and zero backed assets.

- `destroyAgent(address _agentVault, address payable _recipient) external onlyAgentVaultOwner(_agentVault)` [nonpayable]
  - Finalize agent destruction; sends residuals, removes from registry, marks destroyed.

- `upgradeAgentVaultAndPool(address _agentVault) external onlyAgentVaultOwner(_agentVault)` [nonpayable]
  - Upgrades agent’s vault, pool, and pool token to latest implementations.

- `upgradeAgentVaultsAndPools(uint256 _start, uint256 _end) external onlyAssetManagerController` [nonpayable]
  - Governance batch‑upgrades vaults/pools for agents in index range.

- `_upgradeAgentVaultAndPool(address _agentVault) private` [nonpayable]
  - Internal orchestrator to upgrade vault, pool, and pool token.

- `_upgradeContract(IUpgradableContractFactory _factory, address _proxyAddress) private` [nonpayable]
  - If proxy impl differs, upgrades (optionally with init call).

- `_createCollateralPool(IIAssetManager _assetManager, address _agentVault, AgentSettings.Data calldata _settings) private returns (IICollateralPool)` [nonpayable]
  - Deploys collateral pool and pool token; wires token to pool.

- `_reserveAndValidatePoolTokenSuffix(string memory _suffix) private` [nonpayable]
  - Reserves unique pool token suffix; enforces format A‑Z, 0‑9, inner ‘-’.

- `_emitAgentVaultCreated(address _ownerManagementAddress, address _agentVault, IICollateralPool _collateralPool, string memory _underlyingAddress, AgentSettings.Data calldata _settings) private` [nonpayable]
  - Helper to emit AgentVaultCreated with full data payload.

- `_getManagementAddress(address _ownerAddress) private view returns (address)` [view]
  - Resolves management address from registry; falls back to provided address.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AssetManagerDiamondCutFacet.sol
Summary
AssetManagerDiamondCutFacet is an EIP-2535 Diamond facet enabling governance-controlled upgrades of the Asset Manager diamond. It exposes a single entrypoint, diamondCut, to add/replace/remove facet functions and perform an optional initialization delegatecall. Trust model: no direct user funds; upgrades are restricted to governance via a timelock enforced by GovernedProxyImplementation and Globals settings.

Storage
- None — facet keeps no local storage; uses diamond/governance storage

Functions
- function diamondCut(
    FacetCut[] calldata _diamondCut,
    address _init,
    bytes calldata _calldata
  ) external override onlyGovernanceWithTimelockAtLeast(Globals.getSettings().diamondCutMinTimelockSeconds) nonpayable
  NatSpec: Perform diamond upgrade with optional init, restricted to governance with minimum timelock.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AssetManagerInit.sol
Summary

AssetManagerInit is an initialization facet for the Asset Manager diamond. It sets governance, enables reentrancy protection, loads global system settings and initial collateral types, and registers ERC165 support for key interfaces. It holds no direct user funds and exposes only administrative setup entrypoints. Trust model: initial governance is set via init; upgradeERC165Identifiers is permissionless but safe (only toggles ERC165 flags). Major entrypoints: init and upgradeERC165Identifiers.

Storage variables

- None (no state variables declared in this contract)
- Diamond storage touched: LibDiamond.DiamondStorage.supportedInterfaces — ERC165 map

Functions

- interface
  function init(
      IGovernanceSettings _governanceSettings,
      address _initialGovernance,
      AssetManagerSettings.Data memory _settings,
      CollateralType.Data[] memory _initialCollateralTypes
  ) external nonpayable
  modifiers: none
  natspec: Initialize governance, reentrancy guard, settings, collateral types, and ERC165 support.

- interface
  function upgradeERC165Identifiers() external nonpayable
  modifiers: none
  natspec: After diamond cut, marks ERC165 support for governed and asset manager interfaces.

- interface
  function _initIERC165() private nonpayable
  modifiers: none
  natspec: Internal helper to set base ERC165 flags (IERC165, loupe, cut, governed, asset interfaces).


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AvailableAgentsFacet.sol
# AvailableAgentsFacet (Asset Manager Diamond)

Manages the public list of “available” agents that accept external minting. Trust model: Facet doesn’t custody user funds; it only writes diamond storage. Only the agent vault owner can add/remove their agent, subject to collateral checks and governance timelocks. Governance-provided settings enforce exit announcement windows. Major entrypoints: makeAgentAvailable, announceExitAvailableAgentList, exitAvailableAgentList, getAvailableAgentsList, getAvailableAgentsDetailedList.

- Errors
  - ExitTooLate, ExitTooSoon, ExitNotAnnounced, AgentNotAvailable, NotEnoughFreeCollateral, AgentAlreadyAvailable, InvalidAgentStatus.
- Emits (via IAssetManagerEvents): AgentAvailable, AvailableAgentExitAnnounced, AvailableAgentExited.

## Storage (accessed)
- AssetManagerState.availableAgents (address[]): Public agents list
- Agent.State.status: Agent lifecycle status
- Agent.State.availableAgentsPos: 1-based index in list
- Agent.State.feeBIPS: Mint fee in BIPS
- Agent.State.mintingVaultCollateralRatioBIPS: Min CR (vault)
- Agent.State.mintingPoolCollateralRatioBIPS: Min CR (pool)
- Agent.State.exitAvailableAfterTs: Exit allowed timestamp
- Agent.State.ownerManagementAddress: Agent mgmt address
- AssetManagerSettings.agentExitAvailableTimelockSeconds: Exit timelock
- AssetManagerSettings.agentTimelockedOperationWindowSeconds: Exec window after timelock

## Functions

- function makeAgentAvailable(address _agentVault) external onlyAgentVaultOwner(_agentVault) nonpayable
  - NatSpec: Adds agent to public list after status and free collateral checks.

- function announceExitAvailableAgentList(address _agentVault) external onlyAgentVaultOwner(_agentVault) nonpayable returns (uint256 _exitAllowedAt)
  - NatSpec: Announces agent’s intent to exit; returns timestamp when exit is allowed.

- function exitAvailableAgentList(address _agentVault) external onlyAgentVaultOwner(_agentVault) nonpayable
  - NatSpec: Removes agent from public list if within allowed time window.

- function getAvailableAgentsList(uint256 _start, uint256 _end) external view returns (address[] memory _agents, uint256 _totalLength)
  - NatSpec: Paginates raw available agent vault addresses and total length.

- function getAvailableAgentsDetailedList(uint256 _start, uint256 _end) external view returns (AvailableAgentInfo.Data[] memory _agents, uint256 _totalLength)
  - NatSpec: Paginates agents with fee, min CRs, free-collateral lots, and status.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/ChallengesFacet.sol
# ChallengesFacet Summary

ChallengesFacet is a security/monitoring facet of the Asset Manager diamond that lets anyone challenge illegal agent activity using FDC (Flare Data Connector) proofs. Trust model: user funds are held in agent vaults/collateral pools; no special admin permissions here. On successful challenges, the facet immediately starts full liquidation of the offending agent and rewards the challenger from the agent’s vault collateral. Major entrypoints: illegalPaymentChallenge, doublePaymentChallenge, freeBalanceNegativeChallenge. All are nonReentrant, permissionless, and nonpayable.

## Storage Variables
- None (facet holds no direct storage; operates on diamond storage via library accessors)

## Events Emitted (via IAssetManagerEvents)
- IllegalPaymentConfirmed(agentVault, txId)
- DuplicatePaymentConfirmed(agentVault, txId1, txId2)
- UnderlyingBalanceTooLow(agentVault, balanceAfter, requiredBalance)

## External/Public Functions

- function illegalPaymentChallenge(IBalanceDecreasingTransaction.Proof calldata _payment, address _agentVault) external nonReentrant nonpayable
  - NatSpec: Challenge payment with no valid reference; liquidates agent and rewards caller.
  - Notes:
    - Verifies proof with TransactionAttestation.
    - Requires sourceAddressHash == agent.underlyingAddressHash.
    - Ensures transaction wasn’t previously confirmed.
    - If reference decodes to redemption or announced withdrawal, ensures those flows aren’t active/matching.
    - Starts full liquidation and pays challenger; emits IllegalPaymentConfirmed.

- function doublePaymentChallenge(IBalanceDecreasingTransaction.Proof calldata _payment1, IBalanceDecreasingTransaction.Proof calldata _payment2, address _agentVault) external nonReentrant nonpayable
  - NatSpec: Challenge two distinct txs using same payment reference; liquidates and rewards.
  - Notes:
    - Verifies both proofs; txIds must differ; both must originate from agent.
    - Requires equal standardPaymentReference across proofs.
    - Starts full liquidation and pays challenger; emits DuplicatePaymentConfirmed.

- function freeBalanceNegativeChallenge(IBalanceDecreasingTransaction.Proof[] calldata _payments, address _agentVault) external nonReentrant nonpayable
  - NatSpec: Challenge that multiple payments push agent free balance below required; liquidates.
  - Notes:
    - Verifies each proof; rejects duplicates and non-agent sources.
    - Ignores already confirmed txs.
    - For open redemptions, deducts only excess over expected redemption value.
    - Compares resulting balance vs required (UnderlyingBalance.requiredUnderlyingUBA).
    - If negative, starts full liquidation; emits UnderlyingBalanceTooLow.

## Internal/Private Functions

- function _validateAgentStatus(Agent.State storage _agent) private view
  - NatSpec: Forbids challenges if agent is FULL_LIQUIDATION or DESTROYING.

- function _liquidateAndRewardChallenger(Agent.State storage _agent, address _challenger, uint256 _backingAMGAtChallenge) private nonpayable
  - NatSpec: Starts full liquidation and pays challenger from vault collateral.
  - Notes:
    - Liquidation.startFullLiquidation(_agent).
    - Reward = backingAMGAtChallenge * paymentChallengeRewardBIPS (in AMG), converted to vault token wei, plus fixed USD5 reward in vault collateral.
    - Payout via AgentPayout.payoutFromVault.

## Custom Errors (thrown by this facet)
- ChallengeNotAgentsAddress
- ChallengeAlreadyLiquidating
- ChallengeInvalidAgentStatus
- ChallengeNotDuplicate
- ChallengeTransactionAlreadyConfirmed
- ChallengeSameTransactionRepeated
- MatchingAnnouncedPaymentActive
- MatchingRedemptionActive
- MultiplePaymentsChallengeEnoughBalance

## Key Libraries/Dependencies
- TransactionAttestation: verifies FDC proofs.
- PaymentReference: decodes and validates standard payment references.
- Redemptions: redemption state helpers.
- UnderlyingBalance: required underlying calculation.
- Liquidation: triggers full liquidation.
- AgentCollateral, AgentPayout, Conversion, Agents, Globals: collateral math and payouts.

## Security Considerations
- ReentrancyGuard on all external entrypoints.
- Permissionless challenges; economic reward incentivizes monitoring.
- Proofs validated via FDC; references checked to avoid penalizing valid flows.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/CollateralReservationsFacet.sol
CollateralReservationsFacet — Diamond facet for opening minting collateral reservations. It validates agent status/queues, checks free collateral, collects reservation fees in NAT (optionally executor fee), creates a Collateral Reservation Ticket (CRT), and emits payment instructions. Trust model: contract temporarily holds user-paid reservation/executor fees; agent/price parameters come from governance-controlled settings. No privileged admin fund withdrawals here. Major entrypoints: reserveCollateral and collateralReservationFee.

Storage
- Direct storage: none (facet uses diamond storage via libraries).
- AssetManagerState.State (diamond storage used here):
  - newCrtId — Next CRT id seed
  - crts — CRTs mapping by id
  - totalReservedCollateralAMG — Total reserved collateral
  - currentUnderlyingBlock — Latest known underlying block
  - currentUnderlyingBlockTimestamp — Latest underlying block time
  - currentUnderlyingBlockUpdatedAt — Timestamp of last underlying update
  - poolCollateralIndex — Price index for pool collateral
  - mintingPausedAt — Pause flag timestamp
- Agent.State (per agent, used/updated):
  - reservedAMG — Agent reserved collateral
  - availableAgentsPos — Position in public mint queue (0 = not listed)
  - status — Operational status (NORMAL required)
  - feeBIPS — Agent mint fee bips
  - poolFeeShareBIPS — Pool fee share bips (+1 stored in CRT)
  - underlyingAddressString — Underlying payment address
  - alwaysAllowedMinters — Set of allowlisted minters
- CollateralReservation.Data (fields written):
  - valueAMG, underlyingFeeUBA, reservationFeeNatWei
  - poolFeeShareBIPS, agentVault, minter, executor, executorFeeNatGWei
  - firstUnderlyingBlock, lastUnderlyingBlock, lastUnderlyingTimestamp
  - status (ACTIVE)
- AssetManagerSettings.Data (read):
  - averageBlockTimeMS, underlyingBlocksForPayment, underlyingSecondsForPayment, collateralReservationFeeBIPS

Functions
- function reserveCollateral(address _agentVault, uint256 _lots, uint256 _maxMintingFeeBIPS, address payable _executor) external payable onlyAttached notEmergencyPaused nonReentrant returns (uint256 _collateralReservationId)
  - Reserve collateral, collect fees, open CRT, emit payment window and reference.

- function collateralReservationFee(uint256 _lots) external view returns (uint256 _reservationFeeNATWei)
  - Compute required NAT reservation fee for given lot count.

- function _reserveCollateral(Agent.State storage _agent, uint64 _reservationAMG) private
  - Update reserved collateral, enforcing minting cap.

- function _emitCollateralReservationEvent(Agent.State storage _agent, CollateralReservation.Data memory _cr, uint256 _crtId) private
  - Emit CollateralReserved event with all payment instructions.

- function _currentPoolFeeAMG(Agent.State storage _agent, uint64 _valueAMG) private view returns (uint64)
  - Calculate current pool fee in AMG for mint amount.

- function _lastPaymentBlock() private view returns (uint64 _lastUnderlyingBlock, uint64 _lastUnderlyingTimestamp)
  - Compute payment deadline block and timestamp with timeshift.

- function _reservationFee(uint256 amgToTokenWeiPrice, uint64 _valueAMG) private view returns (uint256)
  - Calculate reservation fee in NAT wei using settings bips.

Key Behaviors & Checks
- Requires: minting not paused; agent listed or minter allowlisted; lots > 0; agent NORMAL; free collateral sufficient; agent fee ≤ user’s max.
- Reservation amount includes valueAMG plus current pool-fee AMG share.
- Reservation fee owed only for public minting; allowlisted private mints pay 0.
- msg.value must cover reservation fee; excess:
  - If executor unset: refund change above reservation fee.
  - If executor set: recorded as executorFeeNatGWei (floor in GWEI); sub-GWEI dust not refunded.
- CRT id advanced by randomized skip; PaymentReference.minting(crtId) used in event.
- Payment window derived from underlying block/time plus configured extensions.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/CollateralTypesFacet.sol
# CollateralTypesFacet — Summary
Governance-managed facet of the AssetManager diamond that registers, updates, and deprecates collateral types and their ratios. Trust model: no user funds; only the Asset Manager Controller (governance) can mutate state via onlyAssetManagerController. Users can read collateral metadata. Major entrypoints: addCollateralType, setCollateralRatiosForToken, deprecateCollateralType, getCollateralType, getCollateralTypes. Includes per-collateral rate limiting for ratio changes and emits events for transparency.

## Storage
- CollateralTypeInt.Data.minCollateralRatioBIPS (uint32) — min CR in bips
- CollateralTypeInt.Data.safetyMinCollateralRatioBIPS (uint32) — safety min CR bips
- CollateralTypeInt.Data.validUntil (uint64) — deprecation timestamp
- AssetManagerSettings.Data.tokenInvalidationTimeMinSeconds (uint256) — min deprec. time
- Diamond storage (via libraries) — Collateral registry

## Modifiers
- onlyAssetManagerController — restricts writes to governance/controller

## Errors
- DeprecationTimeToShort() — Provided invalidation time below minimum
- TokenNotValid() — Token already invalid/deprecated

## Events (emitted)
- CollateralRatiosChanged(uint8 collateralClass, address token, uint256 minCRBIPS, uint256 safetyMinCRBIPS)
- CollateralTypeDeprecated(uint8 collateralClass, address token, uint256 validUntil)

## Functions
- function addCollateralType(CollateralType.Data calldata _data) external onlyAssetManagerController;
  /// Add a new collateral type and initial ratios; governance/controller only.

- function setCollateralRatiosForToken(
    CollateralType.Class _collateralClass,
    IERC20 _token,
    uint256 _minCollateralRatioBIPS,
    uint256 _safetyMinCollateralRatioBIPS
  ) external onlyAssetManagerController;
  /// Update min and safety CR; per-type rate-limited; validates >100% and safety≥min.

- function deprecateCollateralType(
    CollateralType.Class _collateralClass,
    IERC20 _token,
    uint256 _invalidationTimeSec
  ) external onlyAssetManagerController;
  /// Schedule deprecation; enforces minimum invalidation window; sets validUntil.

- function getCollateralType(
    CollateralType.Class _collateralClass,
    IERC20 _token
  ) external view returns (CollateralType.Data memory);
  /// Read collateral metadata and ratios for a specific class/token.

- function getCollateralTypes()
  external view returns (CollateralType.Data[] memory _collateralTypes);
  /// Enumerate all collateral types, including deprecated ones.

## Notable Implementation Details
- Ratio change rate-limiting keyed by keccak(action, class, token) via SettingsUpdater.
- Validity rules: minCRBIPS must be > MAX_BIPS (10000) and safety≥min.
- Deprecation: validUntil must be zero or in future; sets validUntil=now+invalidation.
- Uses diamond/libraries (Globals, CollateralTypes) for shared storage.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/CoreVaultClientFacet.sol
CoreVaultClientFacet (diamond facet) bridges agents and the Core Vault. It lets agents move underlying backing into the Core Vault, request funds back, and allows large, approved user redemptions directly from the Core Vault. Trust model: no custody of user ERC-20; agent-only actions gated by onlyAgentVaultOwner; governance controls Core Vault manager and parameters. Major entrypoints: transferToCoreVault, request/cancel/confirm ReturnFromCoreVault, redeemFromCoreVault, and read helpers.

Storage (no direct vars; uses diamond/library storage)
- CoreVaultClient.State.coreVaultManager – Core vault mgr iface
- CoreVaultClient.State.nativeAddress – Native token addr
- CoreVaultClient.State.transferTimeExtensionSeconds – Extra pay time
- CoreVaultClient.State.minimumRedeemLots – Min lots for direct redeem
- CoreVaultClient.State.redemptionFeeBIPS – Direct redeem fee (bips)
- CoreVaultClient.State.newTransferFromCoreVaultId – Monotonic req id
- CoreVaultClient.State.newRedemptionFromCoreVaultId – Monotonic redeem id
- Agent.State.status – Agent lifecycle state
- Agent.State.underlyingBalanceUBA – Underlying balance (int)
- Agent.State.activeTransferToCoreVault – Active transfer ticket id
- Agent.State.activeReturnFromCoreVaultId – Active return req id
- Agent.State.returnFromCoreVaultReservedAMG – Reserved for return (AMG)
- Agent.State.reservedAMG – Total reserved (AMG)
- Agent.State.underlyingAddressString – Agent underlying addr (string)
- Agent.State.underlyingAddressHash – Agent underlying addr hash
- AssetManagerState.paymentConfirmations – Anti-replay confirmations

Custom errors (selection)
- CannotReturnZeroLots, InvalidAgentStatus, InvalidPaymentReference, NoActiveReturnRequest, NotEnoughAvailableOnCoreVault, NotEnoughFreeCollateral, NotEnoughUnderlying, NothingMinted, PaymentNotFromCoreVault, PaymentNotToAgentsAddress, RequestedAmountTooSmall, ReturnFromCoreVaultAlreadyRequested, TooLittleMintingLeftAfterTransfer, TransferAlreadyActive, ZeroTransferNotAllowed

Functions
- constructor() public nonpayable
  NatSpec: Initialize facet implementation storage guard (diamond pattern).

- function transferToCoreVault(address _agentVault, uint256 _amountUBA) external onlyEnabled notEmergencyPaused nonReentrant onlyAgentVaultOwner(_agentVault) nonpayable
  NatSpec: Close tickets and initiate transfer of backing to Core Vault.

- function requestReturnFromCoreVault(address _agentVault, uint256 _lots) external onlyEnabled notEmergencyPaused nonReentrant onlyAgentVaultOwner(_agentVault) nonpayable
  NatSpec: Reserve collateral and request Core Vault to send funds back.

- function cancelReturnFromCoreVault(address _agentVault) external onlyEnabled nonReentrant onlyAgentVaultOwner(_agentVault) nonpayable
  NatSpec: Cancel pending Core Vault return and release reserved collateral.

- function confirmReturnFromCoreVault(IPayment.Proof calldata _payment, address _agentVault) external onlyEnabled nonReentrant onlyAgentVaultOwner(_agentVault) nonpayable
  NatSpec: Verify CV payment, remint backing, update balances, clear reservation.

- function redeemFromCoreVault(uint256 _lots, string memory _redeemerUnderlyingAddress) external onlyEnabled notEmergencyPaused nonReentrant nonpayable
  NatSpec: Burn FAssets and request direct redemption from Core Vault.

- function maximumTransferToCoreVault(address _agentVault) external view returns (uint256 _maximumTransferUBA, uint256 _minimumLeftAmountUBA)
  NatSpec: Read max transferable backing and required remaining capacity.

- function coreVaultAvailableAmount() external view returns (uint256 _immediatelyAvailableUBA, uint256 _totalAvailableUBA)
  NatSpec: Read Core Vault immediate and total available underlying amounts.

Key behaviors and checks
- Only one active transfer/return per agent; enforces min remaining mint capacity after transfer.
- FDC proofs are required; anti-replay via PaymentConfirmations.
- Direct redemptions enforce min lots and apply redemptionFeeBIPS.
- Reentrancy protected; emergency pause respected; Core Vault feature gated with onlyEnabled.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/CoreVaultClientSettingsFacet.sol
CoreVaultClientSettingsFacet is a Diamond facet of AssetManager that initializes and governs Core Vault integration. It stores no user funds; it writes configuration to shared diamond storage (CoreVaultClient.State). Governance (and immediate governance) controls parameters; public updateInterfaces helper exposes ERC165 support. Major entrypoints: initCoreVaultFacet, setters for manager/address/fees/time/limits, and read-only getters.

Storage variables (via CoreVaultClient.State)
- initialized (bool) – one-time init flag
- coreVaultManager (IICoreVaultManager) – CV manager contract
- nativeAddress (address payable) – CV native-chain address
- transferTimeExtensionSeconds (uint64) – extra time vs redemption
- redemptionFeeBIPS (uint16) – direct CV redemption fee
- minimumAmountLeftBIPS (uint16) – min minting cap left
- minimumRedeemLots (uint64) – min lots for direct redeem
- supportedInterfaces (mapping(bytes4=>bool), LibDiamond) – ERC165 interface flags

Functions
- constructor() public nonpayable – Marks implementation as initialized to block direct init.
- function initCoreVaultFacet(IICoreVaultManager _coreVaultManager, address payable _nativeAddress, uint256 _transferTimeExtensionSeconds, uint256 _redemptionFeeBIPS, uint256 _minimumAmountLeftBIPS, uint256 _minimumRedeemLots) external nonpayable – One-time Core Vault init; sets manager, addresses, timings, fees, and limits.
- function updateInterfacesAtCoreVaultDeploy() public nonpayable – Ensures ERC165 registered and adds new facet interfaces.
- function setCoreVaultManager(address _coreVaultManager) external onlyGovernance nonpayable – Set Core Vault manager; must reference this AssetManager.
- function setCoreVaultNativeAddress(address payable _nativeAddress) external onlyImmediateGovernance nonpayable – Update Core Vault native-chain payout address.
- function setCoreVaultTransferTimeExtensionSeconds(uint256 _transferTimeExtensionSeconds) external onlyImmediateGovernance nonpayable – Set extra time for CV transfers.
- function setCoreVaultRedemptionFeeBIPS(uint256 _redemptionFeeBIPS) external onlyImmediateGovernance nonpayable – Set CV direct redemption fee in BIPS.
- function setCoreVaultMinimumAmountLeftBIPS(uint256 _minimumAmountLeftBIPS) external onlyImmediateGovernance nonpayable – Set minimum minting capacity left on agent.
- function setCoreVaultMinimumRedeemLots(uint256 _minimumRedeemLots) external onlyImmediateGovernance nonpayable – Set minimum lots allowed for direct CV redemption.
- function getCoreVaultManager() external view returns (address) – Read current Core Vault manager address.
- function getCoreVaultNativeAddress() external view returns (address) – Read Core Vault native-chain address.
- function getCoreVaultTransferTimeExtensionSeconds() external view returns (uint256) – Read transfer time extension seconds.
- function getCoreVaultRedemptionFeeBIPS() external view returns (uint256) – Read redemption fee in BIPS.
- function getCoreVaultMinimumAmountLeftBIPS() external view returns (uint256) – Read minimum amount-left percentage in BIPS.
- function getCoreVaultMinimumRedeemLots() external view returns (uint256) – Read minimum lots for direct redemption.

Notes
- BIPS inputs guarded by SafePct.MAX_BIPS; reverts BipsValueTooHigh.
- Manager must point to this AssetManager; reverts WrongAssetManager.
- Once enabled, manager cannot be set to zero; reverts CannotDisable.
- Initialization guarded by AlreadyInitialized; diamond must expose IERC165 or DiamondNotInitialized.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/EmergencyPauseFacet.sol
# EmergencyPauseFacet

Purpose: Facet on the AssetManager diamond that manages emergency pause state. Trust model is admin-only: callable solely by AssetManagerController; does not hold or move user funds. Governance can impose/extend pause; controller can impose limited-duration pause subject to configurable caps and reset windows. Major entrypoints: emergencyPause, resetEmergencyPauseTotalDuration, emergencyPaused, emergencyPausedUntil, emergencyPauseDetails.

Errors

- PausedByGovernance — Non-governance pause ops forbidden while governance pause is active.

Storage (diamond state: AssetManagerState.State)

- emergencyPausedUntil (uint64) — pause end timestamp
- emergencyPausedTotalDuration (uint64) — accumulated pause in current window
- emergencyPausedByGovernance (bool) — if current pause is by governance

Functions

- interface
  ```solidity
  function emergencyPause(bool _byGovernance, uint256 _duration)
      external nonpayable onlyAssetManagerController;
  ```
  NatSpec: Start/cancel/extend pause. Governance unlimited within call, controller capped by settings.

- interface
  ```solidity
  function resetEmergencyPauseTotalDuration()
      external nonpayable onlyAssetManagerController;
  ```
  NatSpec: Resets accumulated non-governance pause counter to zero.

- interface
  ```solidity
  function emergencyPaused()
      external view returns (bool);
  ```
  NatSpec: Returns true if now is before emergencyPausedUntil.

- interface
  ```solidity
  function emergencyPausedUntil()
      external view returns (uint256);
  ```
  NatSpec: Returns pause end timestamp or zero if not paused.

- interface
  ```solidity
  function emergencyPauseDetails()
      external view returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance);
  ```
  NatSpec: Returns pause end, total duration in window, and governance flag.

- interface
  ```solidity
  function _paused() private view returns (bool);
  ```
  NatSpec: Internal helper: paused if emergencyPausedUntil > block.timestamp.

Behavioral notes

- Governance path: sets pause until now+duration; marks pausedByGovernance=true.
- Controller path: cannot modify while governance pause active (reverts). Applies rolling cap using:
  - settings.maxEmergencyPauseDurationSeconds (cap per rolling window)
  - settings.emergencyPauseDurationResetAfterSeconds (reset counter after inactivity)
- Emits events (via IAssetManagerEvents): EmergencyPauseTriggered(until) when newly paused/extended, EmergencyPauseCanceled() when pause ends early.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/EmergencyPauseTransfersFacet.sol
Summary
This diamond facet controls emergency pausing of AssetManager transfer-related operations. It enforces governance-led pauses and controller-initiated pauses with rolling duration caps. Trust: only the AssetManagerController may invoke state changes; governance pauses cannot be overridden by non-governance calls. No user funds live here; it gates system behavior. Major entrypoints: emergencyPauseTransfers, resetEmergencyPauseTransfersTotalDuration, and view getters for pause status/details. Emits EmergencyPauseTransfersTriggered/Canceled.

Storage
- AssetManagerState.transfersEmergencyPausedUntil (uint64) — pause end timestamp
- AssetManagerState.transfersEmergencyPausedByGovernance (bool) — governance-enforced flag
- AssetManagerState.transfersEmergencyPausedTotalDuration (uint64) — cumulative pause duration

Read-only config (via Globals.getSettings / AssetManagerSettings)
- emergencyPauseDurationResetAfterSeconds (uint256) — reset window for total duration
- maxEmergencyPauseDurationSeconds (uint256) — cap for rolling pause window

Errors/Events
- error PausedByGovernance()
- event EmergencyPauseTransfersTriggered(uint64 pausedUntil)
- event EmergencyPauseTransfersCanceled()

Functions
- function emergencyPauseTransfers(bool _byGovernance, uint256 _duration) external nonpayable onlyAssetManagerController
  Natspec: Start/extend/cancel pause. Governance override or capped controller pause with rolling window and events.

- function resetEmergencyPauseTransfersTotalDuration() external nonpayable onlyAssetManagerController
  Natspec: Resets accumulated pause duration counter; does not change current paused state.

- function transfersEmergencyPaused() external view returns (bool)
  Natspec: Returns true if transfers are currently paused.

- function transfersEmergencyPausedUntil() external view returns (uint256)
  Natspec: Timestamp when current pause ends; zero if not paused.

- function emergencyPauseTransfersDetails() external view returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance)
  Natspec: Full pause details: end timestamp, total duration, governance flag.

- function _transfersPaused() private view returns (bool)
  Natspec: Internal helper to check paused state by comparing timestamp.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/LiquidationFacet.sol
Summary
LiquidationFacet is a diamond facet handling agent liquidations in the FAssets system. It is non-custodial: user funds are not held here; payouts come from agent vault collateral and collateral pool, gated by system state and governance pause. Main entrypoints: startLiquidation, liquidate, endLiquidation. It enforces reentrancy protection and emergency pause. Liquidators burn f-assets to receive collateral with time-stepped premiums; liquidation can auto-start if ratios fall below thresholds and stops when safe.

Storage variables (facet uses shared diamond storage; none declared locally)
- Agent.State.status — agent lifecycle
- Agent.State.liquidationStartedAt — start timestamp
- Agent.State.collateralsUnderwater — flags: vault/pool
- Agent.State.vaultCollateralIndex — vault coll. index
- Agent.State.poolCollateralIndex — pool coll. index
- AssetManagerState.collateralTokens — index→coll. type
- CollateralTypeInt.Data.minCollateralRatioBIPS — min CR (bips)

Errors
- CannotStopLiquidation() — Not safe or full-liquidation.
- NotInLiquidation() — Agent not in liquidation.
- LiquidationNotStarted() — Start preconditions failed.
- LiquidationNotPossible(AgentInfo.Status status) — Status disallows liquidation.

Events emitted (via IAssetManagerEvents)
- LiquidationStarted(address agentVault, uint256 timestamp)
- LiquidationPerformed(address agentVault, address liquidator, uint256 amountUBA, uint256 paidVault, uint256 paidPool)

Functions
- function startLiquidation(address _agentVault) external notEmergencyPaused nonReentrant returns (uint256 _liquidationStartTs)
  Summary: Start liquidation if CR below thresholds; returns start timestamp or reverts.

- function liquidate(address _agentVault, uint256 _amountUBA) external notEmergencyPaused nonReentrant returns (uint256 _liquidatedAmountUBA, uint256 _amountPaidVault, uint256 _amountPaidPool)
  Summary: Burns caller f-assets to liquidate; pays from vault/pool, emits event, may end liquidation if safe.

- function endLiquidation(address _agentVault) external nonReentrant
  Summary: Ends liquidation if agent CRs are back to safety; reverts if still unsafe/full.

- function _startLiquidation(Agent.State storage _agent, Liquidation.CRData memory _cr) private returns (bool _inLiquidation)
  Summary: Sets underwater flags, transitions to LIQUIDATION if needed; validates allowable statuses.

- function _isCollateralUnderwater(uint256 _collateralRatioBIPS, uint256 _collateralIndex) private view returns (bool)
  Summary: Compares CR against min CR for collateral type to determine underwater state.

- function _performLiquidation(Agent.State storage _agent, Liquidation.CRData memory _cr, uint64 _amountAMG) private returns (uint64 _liquidatedAMG, uint256 _payoutC1Wei, uint256 _payoutPoolWei)
  Summary: Calculates max liquidable, closes tickets, computes vault/pool payouts in token wei.

- function _agentResponsibilityWei(Agent.State storage _agent, uint256 _amount) private view returns (uint256)
  Summary: Determines pool payout share attributable to agent for CPT slashing.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/MintingDefaultsFacet.sol
# MintingDefaultsFacet

Purpose and trust model: Handles minting default flows when minters don’t pay on the underlying chain, and “unstick” flow when FDC proofs expire. Trust is minimized: only the agent’s vault owner can trigger actions that release their reserved collateral; funds routing follows protocol rules (fees to vault/pool; NAT burns). Verification depends on Flare Data Connector attestations. Major entrypoints: mintingPaymentDefault and unstickMinting. Reentrancy is guarded.

Storage variables
- (none declared) – Uses inherited + external library storage
- ReentrancyGuard._status – Reentrancy state

Functions
- function mintingPaymentDefault(IReferencedPaymentNonexistence.Proof calldata _proof, uint256 _crtId) external nonReentrant
  • NatSpec: Declare minter nonpayment; unlock reservation; distribute CRF; requires FDC proof; only vault owner.
  • Mutability: nonpayable

- function unstickMinting(IConfirmedBlockHeightExists.Proof calldata _proof, uint256 _crtId) external payable nonReentrant
  • NatSpec: Burn CRF and reserved collateral post-proof window; agent pays NAT to “buy” vault collateral.
  • Mutability: payable

- function _burnVaultCollateral(Agent.State storage _agent, uint256 _amountVaultCollateralWei) private returns (uint256 _burnedNatWei)
  • NatSpec: Convert vault collateral to NAT at premium; payout collateral to owner; burn provided NAT.
  • Mutability: nonpayable



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/MintingFacet.sol
MintingFacet (AssetManager diamond facet) handles FAsset mint finalization and agent self-mint flows. Trust model: user funds never held by admins; agents supply collateral and underlying, governance can pause minting. Main entrypoints: executeMinting (public mint finalization), selfMint (agent one-step mint with underlying payment), mintFromFreeUnderlying (agent mint using existing underlying).

Storage variables
- _status – Reentrancy state flag (ReentrancyGuard)
- (Diamond storage via libraries) – Uses AssetManagerState/Agent; no local vars

Functions
- function executeMinting(IPayment.Proof calldata _payment, uint256 _crtId) external nonReentrant /* nonpayable */
  - Finalize public mint after verified payment; mints FAssets, updates balances, releases collateral.

- function selfMint(IPayment.Proof calldata _payment, address _agentVault, uint256 _lots) external onlyAttached notEmergencyPaused /* nonpayable */
  - Agent one-step mint with underlying payment; supports 0-lot to credit free underlying.

- function mintFromFreeUnderlying(address _agentVault, uint64 _lots) external onlyAttached notEmergencyPaused /* nonpayable */
  - Agent mints immediately from existing free underlying without new payment.

- function _performMinting(Agent.State storage _agent, MintingType _mintingType, uint256 _crtId, address _minter, uint64 _mintValueAMG, uint256 _receivedAmountUBA, uint256 _poolFeeUBA) private /* nonpayable */
  - Internal mint execution: update backing, credit balances, mint FAsset to minter and pool, emit events.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/RedemptionConfirmationsFacet.sol
# RedemptionConfirmationsFacet

Purpose and trust model: Facet that finalizes redemptions by confirming underlying-chain payments via FDC proofs. It does not custody user funds; it orchestrates state updates unlocking agent vault/pool collateral, charging pool fees, rewarding third-party executors, and integrating with Core Vault. Primary entrypoint: `confirmRedemptionPayment`. Agents normally confirm; after a timeout anyone can confirm and receive a reward from the agent’s vault. Governance only influences parameters read from settings; no admin-only functions here.

Storage variables

- _status — Reentrancy guard flag

Functions

- function confirmRedemptionPayment(IPayment.Proof calldata _payment, uint256 _redemptionRequestId) external nonReentrant nonpayable
  - NatSpec: Confirm redemption payment or failure; release collateral/funds, mint pool fee, update state, emit events.

- function _mintPoolFee(Agent.State storage _agent, Redemption.Request storage _request, uint256 _redemptionRequestId) private nonpayable
  - NatSpec: Mint pool’s share of redemption fee as FAssets and credit collateral pool.

- function _othersCanConfirmPayment(Redemption.Request storage _request) private view returns (bool)
  - NatSpec: Check if timeout elapsed so non-agent can confirm and claim reward.

- function _validatePayment(Redemption.Request storage request, IPayment.Proof calldata _payment) private view returns (bool _paymentValid, string memory _failureReason)
  - NatSpec: Validate FDC proof matches request: refs, timing, addresses, amounts, and status.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/RedemptionDefaultsFacet.sol
## RedemptionDefaultsFacet

Trust model and purpose (≤100 words)
This diamond facet finalizes redemption failures. If an agent doesn’t pay underlying on time, it pays the redeemer from collateral (plus premium) and marks the redemption DEFAULTED. It is trustless: only the redeemer, their executor, or the agent owner can trigger default; for Core Vault transfers, anyone may after a timeout. Uses Flare Data Connector attestations to prove nonpayment or expired proof windows. Funds come from the agent’s vault/pool per system rules. ReentrancyGuard protects state changes. Major entrypoints: redemptionPaymentDefault, finishRedemptionWithoutPayment.

Custom errors
- ShouldDefaultFirst — Must use default-with-proof while proofs still available.
- OnlyRedeemerExecutorOrAgent — Caller isn’t redeemer, executor, agent owner, nor eligible “others”.
- RedemptionNonPaymentProofWindowTooShort — Attestation window doesn’t cover configured minimum.
- RedemptionDefaultTooEarly — Nonpayment window not yet elapsed.
- RedemptionNonPaymentMismatch — Proof fields don’t match request (ref, dest hash, amount).
- InvalidRedemptionStatus — Redemption not ACTIVE.
- SourceAddressesNotSupported — checkSourceAddresses must be false.

Storage variables
- None (direct) — Uses diamond storage via libraries
- ReentrancyGuard._status — Reentrancy state flag
- Library storage (via diamond):
  - Redemptions.requests — Redemption.Request records
  - Agents.agents — Agent.State records
  - Globals.settings — AssetManagerSettings.Data

Functions
- function redemptionPaymentDefault(IReferencedPaymentNonexistence.Proof calldata _proof, uint256 _redemptionRequestId) external nonReentrant
  NatSpec: Default a redemption using FDC nonpayment proof; compensates redeemer and marks request defaulted.
  Notes: Validates proof (ref, dest, amount), timing windows, and caller (redeemer/executor/agent owner or “others” after timeout for Core Vault transfers). Executes RedemptionDefaults.executeDefaultOrCancel, optionally rewards confirmer (AgentPayout.payForConfirmationByOthers), pays/burns executor fee (Redemptions.payOrBurnExecutorFee), and sets status to DEFAULTED.

- function finishRedemptionWithoutPayment(IConfirmedBlockHeightExists.Proof calldata _proof, uint256 _redemptionRequestId) external nonReentrant
  NatSpec: Finalize default without nonpayment proof after attestation windows expire.
  Notes: Agent vault owner only. Verifies proof that the attestation window has elapsed (ConfirmedBlockHeightExists) and that default-with-proof should be used if still available. Executes RedemptionDefaults.executeDefaultOrCancel, burns executor fee (Redemptions.burnExecutorFee), and sets status to DEFAULTED when initially ACTIVE. Does not finish the request to avoid challenging paid-but-expired proofs.

- function _othersCanConfirmDefault(Redemption.Request storage _request) private view returns (bool)
  NatSpec: Checks if “others” may confirm default for Core Vault transfers after timeout.
  Notes: Returns true if _request.transferToCoreVault and now > request.timestamp + settings.confirmationByOthersAfterSeconds.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/RedemptionRequestsFacet.sol
# RedemptionRequestsFacet Summary

Trust model and purpose: Facet of the AssetManager diamond handling redemption flows and agent self-close logic. Users burn FAssets to redeem underlying from agents; if agents default, collateral pays with premium. Governance can pause (notEmergencyPaused). Certain calls are restricted to the collateral pool or agent vault owner via runtime checks/modifiers. Major entrypoints: redeem, redeemFromAgent, redeemFromAgentInCollateral, selfClose, convertDustToTicket, rejectInvalidRedemption, maxRedemptionFromAgent.

Storage variables

- None defined in this facet; uses diamond storage via libraries (AssetManagerState, Globals, RedemptionQueue, Agent). Inherits ReentrancyGuard state.

Functions

- function redeem(uint256 _lots, string _redeemerUnderlyingAddressString, address payable _executor) external payable notEmergencyPaused nonReentrant returns (uint256 _redeemedAmountUBA)
  - Redeem lots, burn FAssets, create agent requests, split executor fee, emit incomplete, return redeemed UBA.

- function redeemFromAgent(address _agentVault, address _receiver, uint256 _amountUBA, string _receiverUnderlyingAddress, address payable _executor) external payable notEmergencyPaused nonReentrant
  - Pool-only self-close redemption; closes tickets from agent, creates request to pay redeemer; burns FAssets.

- function redeemFromAgentInCollateral(address _agentVault, address _receiver, uint256 _amountUBA) external notEmergencyPaused nonReentrant
  - Pool-only; close tickets and pay receiver in vault collateral at discounted FTSO price; burn FAssets.

- function maxRedemptionFromAgent(address _agentVault) external view returns (uint256)
  - Compute max redeemable amount from agent within per-call ticket limit plus dust.

- function rejectInvalidRedemption(IAddressValidity.Proof calldata _proof, uint256 _redemptionRequestId) external nonReentrant
  - Agent rejects invalid redeemer address using FDC proof; releases collateral, burns executor fee, marks request rejected.

- function selfClose(address _agentVault, uint256 _amountUBA) external notEmergencyPaused nonReentrant onlyAgentVaultOwner(_agentVault) returns (uint256 _closedAmountUBA)
  - Agent burns own FAssets to unlock collateral; ends liquidation if healthy; emits SelfClose; returns closed UBA.

- function convertDustToTicket(address _agentVault) external nonReentrant
  - Anyone converts agent dust ≥ 1 lot into a new redemption ticket; adjusts remaining dust.

- function _redeemFirstTicket(uint256 _lots, RedemptionRequests.AgentRedemptionList memory _list) private returns (uint256 _redeemedLots)
  - Internal: consume from first ticket up to lots; group by agent; update queue or convert to dust.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/RedemptionTimeExtensionFacet.sol
# RedemptionTimeExtensionFacet (Diamond Facet)

Governs redemptionPaymentExtensionSeconds, the extra time per redemption payment. Holds no user funds. Only AssetManagerController (admin/governance) can update the parameter, subject to SettingsUpdater time guards and bounded increase/decrease constraints using average block time. Initialization registers IERC165/IRedemptionTimeExtension support and sets initial value in diamond storage. Major entrypoints: initRedemptionTimeExtensionFacet (one-time facet init), setRedemptionPaymentExtensionSeconds (admin update), redemptionPaymentExtensionSeconds (getter). Emits SettingChanged on updates. Errors: ValueMustBeNonzero, DecreaseTooBig, IncreaseTooBig, AlreadyInitialized, DiamondNotInitialized.

## Storage
- RedemptionTimeExtension.redemptionPaymentExtensionSeconds (uint256) — extra time seconds
- Globals.settings.averageBlockTimeMS (uint256) — avg underlying block ms
- LibDiamond.DiamondStorage.supportedInterfaces (mapping(bytes4=>bool)) — ERC165 map

## Functions
- constructor() nonpayable
  - Natspec: Initializes implementation sentinel; doesn’t touch proxy diamond storage.

- function initRedemptionTimeExtensionFacet(uint256 _redemptionPaymentExtensionSeconds) external nonpayable
  - Natspec: One-time facet init; registers interfaces; sets extension seconds; reverts if diamond uninitialized/already initialized.

- function setRedemptionPaymentExtensionSeconds(uint256 _value) external onlyAssetManagerController nonpayable
  - Natspec: Admin sets extension; time-gated and bounded; reverts on zero or excessive change; emits SettingChanged.

- function redemptionPaymentExtensionSeconds() external view returns (uint256)
  - Natspec: Getter for current extra redemption payment time in seconds.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/SettingsManagementFacet.sol
SettingsManagementFacet — Governance settings facet for AssetManager (Diamond)

Summary
- Purpose: Governance-only facet to update system contract addresses and tune global economic/timing parameters for FAssets. It writes to diamond storage (AssetManagerSettings) and the FAsset proxy, but never holds user funds.
- Trust model: Admin gated by onlyAssetManagerController with rate-limited changes (rateLimited). Users do not call this; changes affect system behavior and fees globally.
- Major entrypoints: updateSystemContracts, upgradeFAssetImplementation, setLotSizeAmg, setTimeForPayment, setRedemptionFeeBips, setCollateralReservationFeeBips, setLiquidationPaymentFactors, max emergency pause, various timelocks.

Storage
- UPDATES_STATE_POSITION (bytes32 constant) — slot for rate-limit storage
- UpdaterState.lastUpdate (mapping(bytes4=>uint256)) — per-function last update time
- Note: Operates on diamond storage via Globals.getSettings(): AssetManagerSettings.Data (not declared locally).

Modifiers
- modifier rateLimited() — Enforces min time since last governance update

Functions
- function updateSystemContracts(address _controller, IWNat _wNat) external onlyAssetManagerController nonpayable; — Update controller; rotate wNat and pool collateral type
- function setAgentOwnerRegistry(address _value) external onlyAssetManagerController rateLimited nonpayable; — Set AgentOwnerRegistry address
- function setAgentVaultFactory(address _value) external onlyAssetManagerController rateLimited nonpayable; — Set AgentVaultFactory address
- function setCollateralPoolFactory(address _value) external onlyAssetManagerController rateLimited nonpayable; — Set CollateralPoolFactory address
- function setCollateralPoolTokenFactory(address _value) external onlyAssetManagerController rateLimited nonpayable; — Set CollateralPoolTokenFactory address
- function setPriceReader(address _value) external onlyAssetManagerController rateLimited nonpayable; — Set price reader (FTSO client) address
- function setFdcVerification(address _value) external onlyAssetManagerController rateLimited nonpayable; — Set FDC verification contract address
- function setCleanerContract(address _value) external onlyAssetManagerController rateLimited nonpayable; — Set FAsset cleaner contract via FAsset
- function setCleanupBlockNumberManager(address _value) external onlyAssetManagerController rateLimited nonpayable; — Set FAsset cleanup block manager
- function upgradeFAssetImplementation(address _value, bytes memory callData) external onlyAssetManagerController rateLimited nonpayable; — Upgrade FAsset proxy (optional init call)
- function setTimeForPayment(uint256 _underlyingBlocks, uint256 _underlyingSeconds) external onlyAssetManagerController rateLimited nonpayable; — Set mint/redemption payment windows (blocks/seconds)
- function setPaymentChallengeReward(uint256 _rewardNATWei, uint256 _rewardBIPS) external onlyAssetManagerController rateLimited nonpayable; — Set challenger reward amounts and share
- function setMinUpdateRepeatTimeSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set global rate-limit minimum interval
- function setLotSizeAmg(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set lot size (bounded change; must fit minting cap)
- function setMaxTrustedPriceAgeSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set max trusted price age for pricing
- function setCollateralReservationFeeBips(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set CRF fee in BIPS
- function setRedemptionFeeBips(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set redemption fee in BIPS
- function setRedemptionDefaultFactorVaultCollateralBIPS(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set default redemption compensation factor (vault source)
- function setConfirmationByOthersAfterSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set timeout for third-party confirmations
- function setConfirmationByOthersRewardUSD5(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set reward for third-party confirmation (USD5 units)
- function setMaxRedeemedTickets(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Cap tickets processed per redemption request
- function setWithdrawalOrDestroyWaitMinSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set wait time for withdrawals/destroy operations
- function setAttestationWindowSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set FDC attestation availability window
- function setAverageBlockTimeMS(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set average underlying block time (ms)
- function setMintingPoolHoldingsRequiredBIPS(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set agent-required CPT holdings ratio
- function setMintingCapAmg(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set global minting cap (in AMG)
- function setTokenInvalidationTimeMinSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set token deprecation invalidation delay
- function setVaultCollateralBuyForFlareFactorBIPS(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set factor for vault collateral buy-for-FLR
- function setAgentExitAvailableTimelockSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set timelock for agent availability exit
- function setAgentFeeChangeTimelockSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set timelock for agent fee changes
- function setAgentMintingCRChangeTimelockSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set timelock for minting-CR changes
- function setPoolExitCRChangeTimelockSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set timelock for pool exit/top-up settings
- function setAgentTimelockedOperationWindowSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set execution window after timelocks expire
- function setCollateralPoolTokenTimelockSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set CPT transfer/exit timelock
- function setLiquidationStepSeconds(uint256 _stepSeconds) external onlyAssetManagerController rateLimited nonpayable; — Set liquidation premium step duration
- function setLiquidationPaymentFactors(uint256[] memory _liquidationFactors, uint256[] memory _vaultCollateralFactors) external onlyAssetManagerController rateLimited nonpayable; — Set liquidation factors for value/premium sources
- function setMaxEmergencyPauseDurationSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set max emergency pause duration
- function setEmergencyPauseDurationResetAfterSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable; — Set pause duration reset interval


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/SettingsReaderFacet.sol
# SettingsReaderFacet — summary
Read-only Diamond facet exposing AssetManager configuration and related addresses. It holds no funds and writes no state. Trust model: users don’t grant approvals here; it only reads from Globals storage. Admin changes to settings happen elsewhere via the assetManagerController; this facet merely reports values. Major entrypoints: getSettings, fAsset, priceReader, lotSize, assetMintingGranularityUBA, assetMintingDecimals, assetManagerController, getCollateralPoolTokenTimelockSeconds.

## Storage Variables
- None — no state (read-only facet)

## Functions
- function getSettings() external pure returns (AssetManagerSettings.Data memory)
  - NatSpec: Return the complete current AssetManager settings struct.
  - Modifiers: none

- function fAsset() external view returns (IERC20)
  - NatSpec: Return IERC20 of the managed FAsset token.
  - Modifiers: none

- function priceReader() external view returns (address)
  - NatSpec: Return the configured price reader contract address.
  - Modifiers: none

- function lotSize() external view returns (uint256 _lotSizeUBA)
  - NatSpec: Return lot size in UBA: lotSizeAMG × assetMintingGranularityUBA.
  - Modifiers: none

- function assetMintingGranularityUBA() external view returns (uint256)
  - NatSpec: Return UBA per AMG (minting granularity) from settings.
  - Modifiers: none

- function assetMintingDecimals() external view returns (uint256)
  - NatSpec: Return asset minting decimals used for AMG.
  - Modifiers: none

- function assetManagerController() external view returns (address)
  - NatSpec: Return controller address authorized to change settings.
  - Modifiers: none

- function getCollateralPoolTokenTimelockSeconds() external view returns (uint256)
  - NatSpec: Return CPT time-lock duration in seconds after minting.
  - Modifiers: none


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/SystemInfoFacet.sol
## SystemInfoFacet — Read-only system/query facet for Asset Manager

This diamond facet exposes read-only views into the FAssets AssetManager state: controller attachment, minting pause flag, paginated redemption queues (global and per-agent), and detail views for collateral reservations and redemption requests. It does not hold or move user funds and has no admin-only paths; it merely reads the shared diamond storage via libraries. Major entrypoints: controllerAttached, mintingPaused, redemptionQueue, agentRedemptionQueue, collateralReservationInfo, redemptionRequestInfo.

### Storage
- None (stateless facet)
  - Reads shared diamond storage via AssetManagerState: `attached`, `mintingPausedAt`.

### Functions
- `function controllerAttached() external view returns (bool)`
  - Modifiers: none
  - Natspec: Returns true if this Asset Manager is attached to the controller.

- `function mintingPaused() external view returns (bool)`
  - Modifiers: none
  - Natspec: Returns true when minting is currently paused by system state.

- `function redemptionQueue(uint256 _firstRedemptionTicketId, uint256 _pageSize) external view returns (RedemptionTicketInfo.Data[] memory _queue, uint256 _nextRedemptionTicketId)`
  - Modifiers: none
  - Natspec: Paginates the global redemption queue, returning tickets and next cursor.

- `function agentRedemptionQueue(address _agentVault, uint256 _firstRedemptionTicketId, uint256 _pageSize) external view returns (RedemptionTicketInfo.Data[] memory _queue, uint256 _nextRedemptionTicketId)`
  - Modifiers: none
  - Natspec: Paginates the redemption queue for a specific agent vault.

- `function collateralReservationInfo(uint256 _collateralReservationId) external view returns (CollateralReservationInfo.Data memory)`
  - Modifiers: none
  - Natspec: Fetches detailed info for a collateral reservation (CRT) by id.

- `function redemptionRequestInfo(uint256 _redemptionRequestId) external view returns (RedemptionRequestInfo.Data memory)`
  - Modifiers: none
  - Natspec: Fetches detailed info for a redemption request by id.

- `function _convertCollateralReservationStatus(CollateralReservation.Status _status) private pure returns (CollateralReservationInfo.Status)`
  - Modifiers: none
  - Natspec: Maps internal CRT status enum to UI/status DTO enum.

- `function _convertRedemptionStatus(Redemption.Status _status) private pure returns (RedemptionRequestInfo.Status)`
  - Modifiers: none
  - Natspec: Maps internal redemption status enum to UI/status DTO enum.

### Notes
- Relies on libraries: AssetManagerState, RedemptionQueueInfo, Minting, Redemptions, Conversion, PaymentReference, Agent.
- Pool fee share in CRT uses agent default when zero and adjusts sentinel via minus-one convention.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/SystemStateManagementFacet.sol
SystemStateManagementFacet — AssetManager diamond facet for lifecycle and minting switches. Purpose: gate critical system state (attachment to controller, minting pause) under governance control via AssetManagerController. Trust model: no user funds handled here; admin-only actions through onlyAssetManagerController; affects ability to create agents/mint. Major entrypoints: attachController, pauseMinting, unpauseMinting.

Storage (via AssetManagerState diamond storage)
- attached (bool) — controller attached flag
- mintingPausedAt (uint64) — pause start timestamp

Functions
- function attachController(bool attached) external onlyAssetManagerController; [nonpayable]
  /// Set or clear “attached” when controller adds/removes this asset manager.

- function pauseMinting() external onlyAssetManagerController; [nonpayable]
  /// Start minting pause; records timestamp if not already paused.

- function unpauseMinting() external onlyAssetManagerController; [nonpayable]
  /// Clear minting pause by zeroing timestamp; minting resumes.

Notes
- Access control: onlyAssetManagerController modifier from AssetManagerBase enforces governance.
- Idempotency: pauseMinting no-ops if already paused; unpause resets to 0.
- Uses SafeCast.toUint64 for timestamp sizing; no token transfers or fund custody.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/UnderlyingBalanceFacet.sol
Summary
UnderlyingBalanceFacet is an AssetManager facet managing agents’ underlying-chain balances. It is trust-minimized: only agent vault owners can announce/cancel and confirm top-ups/withdrawals; after a timeout, anyone may confirm withdrawals for a small reward. No privileged admin can seize user funds; updates are constrained by on-chain-verified FDC payment proofs. Major entrypoints: confirmTopupPayment, announceUnderlyingWithdrawal, confirmUnderlyingWithdrawal, cancelUnderlyingWithdrawal.

Storage (read/write via libraries)
- Agent.State.underlyingAddressHash – hash of agent work address
- Agent.State.underlyingBlockAtCreation – block when agent created
- Agent.State.announcedUnderlyingWithdrawalId – active withdrawal id
- Agent.State.underlyingWithdrawalAnnouncedAt – ts of announcement
- AssetManagerState.State.newPaymentAnnouncementId – rolling id counter
- AssetManagerState.State.paymentConfirmations – attestation tracking
- AssetManagerSettings.confirmationByOthersAfterSeconds – 3rd-party confirm delay

Functions
- function confirmTopupPayment(IPayment.Proof calldata _payment, address _agentVault) external onlyAgentVaultOwner(_agentVault)
  Natspec: Confirms agent top-up via FDC proof; increases free underlying balance and emits event.

- function announceUnderlyingWithdrawal(address _agentVault) external onlyAgentVaultOwner(_agentVault)
  Natspec: Starts an underlying withdrawal; assigns random announcement id and emits required payment reference.

- function confirmUnderlyingWithdrawal(IPayment.Proof calldata _payment, address _agentVault) external nonReentrant
  Natspec: Confirms announced withdrawal via FDC proof; updates balance, clears announcement, may reward confirmer.

- function cancelUnderlyingWithdrawal(address _agentVault) external onlyAgentVaultOwner(_agentVault)
  Natspec: Cancels active withdrawal announcement to reset window and prevent third-party front-running.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/UnderlyingTimekeepingFacet.sol
# UnderlyingTimekeepingFacet

A diamond facet of the Asset Manager that tracks the latest confirmed block height and timestamp on the underlying chain via FDC proofs. It holds no user funds and has no admin-only methods; anyone can submit a proof to refresh timekeeping, which affects minting/redemption payment windows. Major entrypoints: updateCurrentBlock (proof submission) and currentUnderlyingBlock (reader). Trust model: permissionless updater; relies on Flare Data Connector attestation validity.

## Storage (via AssetManagerState.State)
- currentUnderlyingBlock — last confirmed height
- currentUnderlyingBlockTimestamp — last confirmed time
- currentUnderlyingBlockUpdatedAt — local update time

## Functions
- function updateCurrentBlock(IConfirmedBlockHeightExists.Proof calldata _proof) external
  // Mutability: nonpayable; Modifiers: none
  /// Verifies underlying block proof and updates tracked height/time. Anyone may call.

- function currentUnderlyingBlock() external view returns (uint256 _blockNumber, uint256 _blockTimestamp, uint256 _lastUpdateTs)
  // Mutability: view; Modifiers: none
  /// Returns tracked underlying block number, timestamp, and last update time.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/implementation/AssetManager.sol
## AssetManager (Diamond Root)

A minimal Diamond-based root for the FAssets AssetManager. It holds no explicit storage or logic; instead, all user-facing functionality (minting, redemption, collateral, liquidation, settings) is implemented in facets and reached via the Diamond fallback. Trust model: non-custodial and over-collateralized; user funds are held in agents’ vaults/collateral pools and on underlying chains. Admin/governance controls diamond upgrades (facet management) and system pausing via LibDiamond. Major entrypoints here are construction (initial diamond cut) and delegated facet calls through the Diamond fallback.

- Inherits: Diamond
- Implements: IAssetManagerEvents (for explorer event decoding)

### Storage Variables
- (none) — No explicit storage in this contract; uses Diamond storage via LibDiamond in parent

### Functions

- constructor(IDiamondCut.FacetCut[] memory _diamondCut, address _init, bytes memory _initCalldata) payable
  - Visibility/Modifiers/Mutability: constructor, none, payable
  - Natspec: Initializes the Diamond by applying facet cuts and optional initializer call.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/mock/MaliciousDistributionToDelegators.sol
# MaliciousDistributionToDelegators.sol — Summary

This is a minimal, malicious/mock distribution contract used for testing. It never holds user funds and has no admin/owner. Its sole behavior is to store a fixed reward amount at deployment and return it on claim, ignoring all inputs. Anyone can call claim; nothing is transferred and no state changes occur. Intended to spoof a reward distributor interface or simulate misbehavior. Major entrypoints: constructor(uint256 _claim), claim(address,address,uint256,bool), and the public amount() getter.

## Storage Variables
- `uint256 public amount` — Fixed claim amount

## Functions
- `constructor(uint256 _claim) nonpayable`  
  - NatSpec: Initialize fixed reward amount used by claim.

- `function amount() external view returns (uint256)`  
  - NatSpec: Auto-generated getter for fixed claim amount.

- `function claim(address _rewardOwner, address _recipient, uint256 _month, bool _wrap) external nonpayable returns (uint256 _rewardAmount)`  
  - NatSpec: Returns preset amount; ignores parameters; no state change.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/mock/MaliciousExecutor.sol
# MaliciousExecutor.sol — Summary
MaliciousExecutor is an exploit/test helper targeting IAssetManager.redemptionPaymentDefault. It caches a payment nonexistence proof and requestId, then (optionally) re-enters redemptionPaymentDefault via the payable fallback when this contract receives native tokens. There is no admin or owner; any external caller can trigger behavior. It can momentarily hold native funds sent to it but has no withdrawal logic. Major entrypoints: defaulting (sets proof/trigger and calls AssetManager), fallback (conditional reentrancy), howMuchIsMyNativeBalance (balance view).

## Storage Variables
- address public immutable diamond — asset manager addr
- IReferencedPaymentNonexistence.Proof public tempProof — cached proof
- uint256 public tempRequestId — cached request id
- uint256 public hit — reentrancy flag
- uint256 public trigger — enable reenter

## Functions
- constructor(address _diamond) nonpayable
  - Sets the AssetManager (diamond) address.

- function defaulting(IReferencedPaymentNonexistence.Proof calldata _proof, uint256 _redemptionRequestId, uint256 _trigger) external nonpayable
  - Cache proof/id, set trigger, call redemptionPaymentDefault once.

- function howMuchIsMyNativeBalance() external view returns (uint256)
  - Returns this contract’s native balance.

- fallback() external payable
  - On receiving native tokens, re-enter redemptionPaymentDefault once if enabled.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/mock/MaliciousMintExecutor.sol
# MaliciousMintExecutor — Summary
A testing/attack helper that exploits the FAssets minting flow. It proxies mint execution to AssetManager and, via a payable fallback, snapshots agent metrics, pulls all FAssets from a pre-approved minter, and immediately triggers liquidation against a target agent. Trust model: no admin/owner controls, but it can move the minter’s FAssets if allowance is granted. Major entrypoints: constructor, mint(), and a payable fallback that calls internal proceed().

## Storage Variables
- diamond: address (immutable, public) — AssetManager diamond
- agentVault: address (immutable, public) — Agent vault addr
- minter: address (immutable, public) — Approved minter
- fasset: address (immutable, public) — FAsset token addr
- liquidationStartedTs: uint256 (public) — Liquid start ts
- reserved: uint256 (public) — Reserved UBA
- minted: uint256 (public) — Minted UBA
- poolCR: uint256 (public) — Pool CR bips
- vaultCR: uint256 (public) — Vault CR bips

## Functions
- constructor(address _diamond, address _agentVault, address _minter, address _fasset) nonpayable
  • Initializes immutable addresses for AssetManager, agent vault, minter, and FAsset.

- function mint(IPayment.Proof calldata _proof, uint256 _collateralReservationId) external nonpayable
  • Forwards proof to AssetManager.executeMinting for the given reservation id.

- fallback() external payable
  • On unknown call or ETH, invokes proceed() to snapshot and liquidate.

- function proceed() internal nonpayable
  • Snapshot agent state, pull minter’s FAssets, start and execute liquidation.

## Notes
- Requires the minter to have approved this contract to transfer FAssets (transferFrom).
- Reads agent info via IAssetManager.getAgentInfo; triggers liquidation via startLiquidation and liquidate.
- No owner/admin; contract can briefly hold FAssets used for liquidation.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/mock/MaliciousRewardManager.sol
## Summary
MaliciousRewardManager is a minimal, test/malicious mock of Flare’s IRewardManager. It never verifies proofs and simply returns a preset reward amount on claim. Trust model: no admin, no custody of user funds, immutable behavior after deployment (amount set in constructor). Major entrypoints are constructor (set amount), claim (returns amount), and the auto-generated amount() getter.

## Storage
- amount (uint256) — fixed claim amount

## Functions

```solidity
constructor(uint256 _claim)
```
- Natspec: Initialize fixed reward amount to be returned on any claim.
- Visibility: internal (constructor); Mutability: nonpayable; Modifiers: none

```solidity
function amount() external view returns (uint256)
```
- Natspec: Returns stored fixed claim amount.
- Visibility: external; Mutability: view; Modifiers: none

```solidity
function claim(
    address _rewardOwner,
    address payable _recipient,
    uint24 _rewardEpochId,
    bool _wrap,
    IRewardManager.RewardClaimWithProof[] calldata _proofs
) external returns (uint256 _rewardAmountWei)
```
- Natspec: Ignores inputs and returns preset reward amount.
- Visibility: external; Mutability: nonpayable; Modifiers: none


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManagerController/implementation/AssetManagerController.sol
# AssetManagerController.sol Summary

Governance-controlled admin contract coordinating multiple AssetManager instances. It holds no user funds; instead, governance (and optionally whitelisted emergency senders) can update settings, upgrade implementations via UUPS, manage collateral/token parameters, and trigger emergency pauses. Major entrypoints: add/remove asset managers, settings setters (fees, timeouts, ratios), collateral type management, UUPS upgrades, emergencyPause/emergencyPauseTransfers, and contract address updates via AddressUpdater.

## Storage Variables
- replacedBy: address – replacement controller addr
- assetManagerIndex: mapping(address=>uint256) – manager index map
- assetManagers: IIAssetManager[] – managed AM list
- emergencyPauseSenders: EnumerableSet.AddressSet – whitelisted EP senders

## Functions

- constructor() nonpayable
  - Initializes inherited bases; no state beyond base init.

- function initialize(IGovernanceSettings _governanceSettings, address _initialGovernance, address _addressUpdater) external nonpayable
  - One-time init; sets governance and address updater.

- function addAssetManager(IIAssetManager _assetManager) external onlyGovernance nonpayable
  - Adds and attaches an AssetManager if not present.

- function removeAssetManager(IIAssetManager _assetManager) external onlyGovernance nonpayable
  - Removes and detaches an AssetManager if managed.

- function getAssetManagers() external view returns (IAssetManager[] memory _assetManagers)
  - Returns the list of managed AssetManagers.

- function assetManagerExists(address _assetManager) external view returns (bool)
  - Checks if an AssetManager address is managed.

- function upgradeTo(address newImplementation) public onlyGovernance onlyProxy override(IUUPSUpgradeable, UUPSUpgradeable) nonpayable
  - UUPS upgrade to new implementation.

- function upgradeToAndCall(address newImplementation, bytes memory data) public payable onlyGovernance onlyProxy override(IUUPSUpgradeable, UUPSUpgradeable)
  - UUPS upgrade and call initialization payload.

- function _authorizeUpgrade(address /* _newImplementation */) internal pure override
  - Unused; real auth enforced via modifiers.

- function setAgentOwnerRegistry(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  - Sets AgentOwnerRegistry on managers.

- function setAgentVaultFactory(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  - Sets AgentVaultFactory on managers.

- function setCollateralPoolFactory(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  - Sets CollateralPoolFactory on managers.

- function setCollateralPoolTokenFactory(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  - Sets CollateralPoolTokenFactory on managers.

- function upgradeAgentVaultsAndPools(IIAssetManager[] memory _assetManagers, uint256 _start, uint256 _end) external onlyImmediateGovernance nonpayable
  - Triggers upgrade of agent vault/pool proxies by range.

- function setPriceReader(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  - Sets price reader on managers.

- function setFdcVerification(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  - Sets FDC verification contract on managers.

- function setCleanerContract(IIAssetManager[] memory _assetManagers, address _value) external onlyImmediateGovernance nonpayable
  - Sets cleaner/maintenance contract on managers.

- function setCleanupBlockNumberManager(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  - Sets cleanup block number manager.

- function upgradeFAssetImplementation(IIAssetManager[] memory _assetManagers, address _implementation, bytes memory _callData) external onlyGovernance nonpayable
  - Upgrades FAsset proxy implementation with optional init.

- function setMinUpdateRepeatTimeSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  - Sets min governance update repeat time.

- function setLotSizeAmg(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  - Sets lot size (AMG units).

- function setTimeForPayment(IIAssetManager[] memory _assetManagers, uint256 _underlyingBlocks, uint256 _underlyingSeconds) external onlyGovernance nonpayable
  - Sets underlying payment window (blocks/seconds).

- function setPaymentChallengeReward(IIAssetManager[] memory _assetManagers, uint256 _rewardVaultCollateralWei, uint256 _rewardBIPS) external onlyImmediateGovernance nonpayable
  - Sets challenge reward absolute and percent.

- function setMaxTrustedPriceAgeSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets max trusted price age.

- function setCollateralReservationFeeBips(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets CRF fee in BIPS.

- function setRedemptionFeeBips(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets redemption fee in BIPS.

- function setRedemptionDefaultFactorVaultCollateralBIPS(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets default factor for vault-collateral redemptions.

- function setConfirmationByOthersAfterSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets delay after which anyone can confirm.

- function setConfirmationByOthersRewardUSD5(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets reward for third-party confirmations.

- function setMaxRedeemedTickets(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets max tickets processed per redemption.

- function setWithdrawalOrDestroyWaitMinSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets timelock for withdrawal/destroy ops.

- function setAttestationWindowSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets FDC attestation availability window.

- function setAverageBlockTimeMS(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets average underlying block time (ms).

- function setMintingPoolHoldingsRequiredBIPS(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets required agent pool holdings ratio.

- function setMintingCapAmg(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets system minting cap (AMG units).

- function setTokenInvalidationTimeMinSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  - Sets token invalidation time window.

- function setVaultCollateralBuyForFlareFactorBIPS(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  - Sets vault-collateral buy-for-FLR factor.

- function setAgentExitAvailableTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets agent exit available timelock.

- function setAgentFeeChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets timelock for agent fee changes.

- function setAgentMintingCRChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets timelock for agent minting CR changes.

- function setPoolExitCRChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets timelock for pool exit CR changes.

- function setAgentTimelockedOperationWindowSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets execution window after timelocks expire.

- function setCollateralPoolTokenTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Sets CPT timelock for pool entrants.

- function setLiquidationStepSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  - Sets time per liquidation premium step.

- function setLiquidationPaymentFactors(IIAssetManager[] memory _assetManagers, uint256[] memory _paymentFactors, uint256[] memory _vaultCollateralFactors) external onlyGovernance nonpayable
  - Sets liquidation payout and vault factors.

- function setRedemptionPaymentExtensionSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  - Extends per-redemption payment window.

- function addCollateralType(IIAssetManager[] memory _assetManagers, CollateralType.Data calldata _data) external onlyImmediateGovernance nonpayable
  - Adds a new collateral type definition.

- function setCollateralRatiosForToken(IIAssetManager[] memory _assetManagers, CollateralType.Class _class, IERC20 _token, uint256 _minCollateralRatioBIPS, uint256 _safetyMinCollateralRatioBIPS) external onlyGovernance nonpayable
  - Updates min/safety CR for a token.

- function deprecateCollateralType(IIAssetManager[] memory _assetManagers, CollateralType.Class _class, IERC20 _token, uint256 _invalidationTimeSec) external onlyImmediateGovernance nonpayable
  - Deprecates a collateral type with timeout.

- function pauseMinting(IIAssetManager[] calldata _assetManagers) external onlyImmediateGovernance nonpayable
  - Pauses minting on specified managers.

- function unpauseMinting(IIAssetManager[] calldata _assetManagers) external onlyImmediateGovernance nonpayable
  - Unpauses minting on specified managers.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  - ERC-165 support for interfaces implemented.

- function updateContracts(IIAssetManager[] calldata _assetManagers) external nonpayable
  - Pulls new addresses via AddressUpdater and propagates.

- function _updateContractAddresses(bytes32[] memory _contractNameHashes, address[] memory _contractAddresses) internal override nonpayable
  - AddressUpdater hook; forwards new addresses.

- function _updateContracts(IIAssetManager[] memory _assetManagers, address addressUpdater, address assetManagerController, address wNat) private nonpayable
  - Applies address updates to managers; sets replacedBy.

- function emergencyPause(IIAssetManager[] memory _assetManagers, uint256 _duration) external nonpayable
  - Triggers system emergency pause (gov or whitelisted).

- function emergencyPauseTransfers(IIAssetManager[] memory _assetManagers, uint256 _duration) external nonpayable
  - Triggers emergency pause for token transfers.

- function resetEmergencyPauseTotalDuration(IIAssetManager[] memory _assetManagers) external onlyImmediateGovernance nonpayable
  - Resets accumulated emergency pause durations.

- function addEmergencyPauseSender(address _address) external onlyImmediateGovernance nonpayable
  - Whitelists an emergency pause sender.

- function removeEmergencyPauseSender(address _address) external onlyImmediateGovernance nonpayable
  - Removes a whitelisted emergency pause sender.

- function setMaxEmergencyPauseDurationSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  - Sets max emergency pause duration.

- function setEmergencyPauseDurationResetAfterSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  - Sets cooldown after which pause counter resets.

- function _setValueOnManagers(IIAssetManager[] memory _assetManagers, bytes4 _selector, address _value) private nonpayable
  - Helper to broadcast address param setter.

- function _setValueOnManagers(IIAssetManager[] memory _assetManagers, bytes4 _selector, uint256 _value) private nonpayable
  - Helper to broadcast uint256 param setter.

- function _callOnManagers(IIAssetManager[] memory _assetManagers, bytes memory _calldata) private nonpayable
  - Loops managers; checks membership; low-level call.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManagerController/implementation/AssetManagerControllerProxy.sol
# AssetManagerControllerProxy (ERC1967Proxy)

A minimal ERC1967 proxy that deploys and initializes an AssetManagerController in one step. Purpose: route all calls to a logic contract while passing governance wiring during construction. Trust model: no dedicated proxy admin; upgrades and privileged actions are governed by the implementation (UUPS/implementation-side). The proxy shouldn’t hold user funds; any value sent is delegated to the implementation. Major entrypoints: fallback() and receive() which delegate to the current implementation.

## Storage
- EIP1967 implementation (address) — implementation addr
- EIP1967 admin (address) — proxy admin (unused)
- EIP1967 beacon (address) — beacon addr (unused)
- No explicit storage variables in this contract.

## Functions

- constructor(
    address _implementationAddress,
    IGovernanceSettings _governanceSettings,
    address _initialGovernance,
    address _addressUpdater
  ) nonpayable
  — Deploys proxy and initializes implementation via delegatecall to initialize.

- fallback() external payable
  — Delegates unknown calls and attached value to the implementation.

- receive() external payable
  — Accepts ETH and delegates to implementation when called with empty calldata.

Notes
- Initialization payload: AssetManagerController.initialize(IGovernanceSettings,address,address).
- Upgrade authority and access control reside in the implementation, not the proxy.


## SUMMARY OF FILE: 2025-08-flare/contracts/collateralPool/implementation/CollateralPool.sol
CollateralPool (UUPS, ReentrancyGuard) manages a per-agent collateral pool of wrapped native tokens (WNat/FLR) and tracks/ distributes FAsset fee rewards to pool token holders. Trust model: users deposit NAT into a pool controlled by AssetManager; only AssetManager can upgrade, set token, deposit, and payout. Agent can delegate WNat voting/rewards. Funds are held in contract; exits are constrained by exit CR and minimum balances. Major entrypoints: enter, exit/exitTo, selfCloseExit/To, withdrawFees/To, payFAssetFeeDebt, payout, delegation reward claiming, UUPS upgrade.

Storage
- MIN_NAT_TO_ENTER (uint256 constant) – min NAT to enter
- MIN_TOKEN_SUPPLY_AFTER_EXIT (uint256 constant) – min supply post-exit
- MIN_NAT_BALANCE_AFTER_EXIT (uint256 constant) – min NAT post-exit
- agentVault (address) – agent’s vault addr
- assetManager (IIAssetManager) – system manager
- fAsset (IFAsset) – FAsset token
- token (IICollateralPoolToken) – pool token
- wNat (IWNat) – wrapped NAT token
- exitCollateralRatioBIPS (uint32) – exit CR threshold
- __topupCollateralRatioBIPS (uint32) – placeholder
- __topupTokenPriceFactorBIPS (uint16) – placeholder
- internalWithdrawal (bool) – internal unwrap guard
- initialized (bool) – init flag
- _fAssetFeeDebtOf (mapping(address=>int256)) – user fee debt
- totalFAssetFeeDebt (int256) – sum fee debt
- totalFAssetFees (uint256) – collected fees
- totalCollateral (uint256) – WNat collateral

Functions
- constructor(address _agentVault, address _assetManager, address _fAsset, uint32 _exitCollateralRatioBIPS) public nonpayable
  NatSpec: Test-only constructor that forwards to initialize.
- function initialize(address _agentVault, address _assetManager, address _fAsset, uint32 _exitCollateralRatioBIPS) public nonpayable
  NatSpec: One-time initializer; sets immutable-like references and reentrancy guard.
- receive() external payable
  NatSpec: Accepts NAT only during internal withdrawals.
- function setPoolToken(address _poolToken) external nonpayable onlyAssetManager
  NatSpec: Sets the pool token contract once.
- function poolToken() external view returns (ICollateralPoolToken)
  NatSpec: Returns the pool token interface.
- function setExitCollateralRatioBIPS(uint256 _exitCollateralRatioBIPS) external nonpayable onlyAssetManager
  NatSpec: Updates exit collateral ratio in BIPS.
- function enter() external payable nonReentrant returns (uint256, uint256)
  NatSpec: Deposit NAT to mint pool tokens; sets fee debt share.
- function exit(uint256 _tokenShare) external nonpayable nonReentrant returns (uint256)
  NatSpec: Burn pool tokens and withdraw proportional NAT.
- function exitTo(uint256 _tokenShare, address payable _recipient) external nonpayable nonReentrant returns (uint256)
  NatSpec: Exit to a specified recipient address.
- function _exitTo(uint256 _tokenShare, address payable _recipient) private nonpayable returns (uint256)
  NatSpec: Internal exit flow; CR and min-balance checks.
- function selfCloseExit(uint256 _tokenShare, bool _redeemToCollateral, string memory _redeemerUnderlyingAddress, address payable _executor) external payable nonReentrant
  NatSpec: Exit while redeeming FAssets to preserve CR.
- function selfCloseExitTo(uint256 _tokenShare, bool _redeemToCollateral, address payable _recipient, string memory _redeemerUnderlyingAddress, address payable _executor) external payable nonReentrant
  NatSpec: Self-close exit to recipient; optional executor.
- function _selfCloseExitTo(uint256 _tokenShare, bool _redeemToCollateral, address payable _recipient, string memory _redeemerUnderlyingAddress, address payable _executor) private nonpayable
  NatSpec: Internal self-close logic; redeems FAssets or collateral.
- function fAssetRequiredForSelfCloseExit(uint256 _tokenAmountWei) external view returns (uint256)
  NatSpec: Computes FAssets needed to not lower CR on exit.
- function withdrawFees(uint256 _fAssets) external nonpayable nonReentrant
  NatSpec: Withdraw accrued FAsset fees; increases fee debt.
- function withdrawFeesTo(uint256 _fAssets, address _recipient) external nonpayable nonReentrant
  NatSpec: Withdraw fees to recipient; increases fee debt.
- function _withdrawFeesTo(uint256 _fAssets, address _recipient) private nonpayable
  NatSpec: Internal fee withdrawal and accounting.
- function payFAssetFeeDebt(uint256 _fAssets) external nonpayable nonReentrant
  NatSpec: Repay fee debt with FAssets; transfers in.
- function payout(address _recipient, uint256 _amount, uint256 _agentResponsibilityWei) external nonpayable onlyAssetManager nonReentrant
  NatSpec: Pays out from pool; slashes agent’s pool tokens proportionally.
- function _collateralToTokenShare(uint256 _collateral) internal view returns (uint256)
  NatSpec: Converts NAT collateral to pool token amount.
- function _tokensToVirtualFeeShare(uint256 _tokens) internal view returns (uint256)
  NatSpec: Converts tokens to proportional virtual fee share.
- function _getFAssetRequiredToNotSpoilCR(uint256 _natShare) internal view returns (uint256)
  NatSpec: FAssets needed so post-exit CR stays at target.
- function _staysAboveExitCR(uint256 _withdrawnNat) internal view returns (bool)
  NatSpec: Checks if withdrawal keeps CR >= exit CR.
- function _isAboveCR(AssetPrice memory _assetPrice, uint256 _backedFAssets, uint256 _poolCollateralNat, uint256 _crBIPS) internal pure returns (bool)
  NatSpec: Compares CR against threshold using price ratio.
- function _agentBackedFAssets() internal view returns (uint256)
  NatSpec: Fetches agent-backed FAssets that pool supports.
- function _virtualFAssetFeesOf(address _account) internal view returns (uint256)
  NatSpec: Account’s pro-rata share of pool fees.
- function _fAssetFeesOf(address _account) internal view returns (uint256)
  NatSpec: Account’s withdrawable FAsset fees.
- function _debtFreeTokensOf(address _account) internal view returns (uint256)
  NatSpec: Pool tokens not locked by fee debt.
- function _getAssetPrice() internal view returns (AssetPrice memory)
  NatSpec: Gets FAsset price relative to NAT (mul/div).
- function _totalVirtualFees() internal view returns (uint256)
  NatSpec: Total fees including outstanding fee debt.
- function _safeExitCR() internal view returns (uint256)
  NatSpec: Max of min pool CR and exit CR.
- function _requireMinTokenSupplyAfterExit(uint256 _tokenShare) internal view nonpayable
  NatSpec: Enforces minimal residual pool token supply.
- function _requireMinNatSupplyAfterExit(uint256 _natShare) internal view nonpayable
  NatSpec: Enforces minimal residual NAT collateral.
- function depositNat() external payable onlyAssetManager nonReentrant
  NatSpec: AssetManager deposits NAT; wraps and updates collateral.
- function fAssetFeeDeposited(uint256 _amount) external nonpayable onlyAssetManager
  NatSpec: Records externally deposited FAsset fees.
- function _createFAssetFeeDebt(address _account, uint256 _fAssets) internal nonpayable
  NatSpec: Increases account’s fee debt and totals.
- function _deleteFAssetFeeDebt(address _account, uint256 _fAssets) internal nonpayable
  NatSpec: Decreases account’s fee debt and totals.
- function _transferFAssetFrom(address _from, uint256 _amount) internal nonpayable
  NatSpec: Pulls FAssets into pool; updates fee total.
- function _transferFAssetTo(address _to, uint256 _amount) internal nonpayable
  NatSpec: Sends FAssets from pool; updates fee total.
- function _transferWNatTo(address _to, uint256 _amount) internal nonpayable
  NatSpec: Transfers WNat and updates totalCollateral.
- function _withdrawWNatTo(address payable _recipient, uint256 _amount) internal nonpayable
  NatSpec: Unwraps WNat to NAT and transfers out.
- function _depositWNat() internal payable nonpayable
  NatSpec: Wraps received NAT into WNat and updates totals.
- function virtualFAssetOf(address _account) external view returns (uint256)
  NatSpec: Returns account’s virtual (pro-rata) fees.
- function fAssetFeesOf(address _account) external view returns (uint256)
  NatSpec: Returns account’s available FAsset fees.
- function fAssetFeeDebtOf(address _account) external view returns (int256)
  NatSpec: Returns account’s fee debt balance.
- function debtLockedTokensOf(address _account) external view returns (uint256)
  NatSpec: Returns token balance locked by fee debt.
- function debtFreeTokensOf(address _account) external view returns (uint256)
  NatSpec: Returns token balance free of fee debt.
- function destroy(address payable _recipient) external nonpayable onlyAssetManager nonReentrant
  NatSpec: Destroys pool when no tokens exist; sends leftovers.
- function upgradeWNatContract(IWNat _newWNat) external nonpayable onlyAssetManager nonReentrant
  NatSpec: Migrates all funds to a new WNat contract.
- function delegate(address _to, uint256 _bips) external nonpayable onlyAgent
  NatSpec: Delegates WNat vote power by bips.
- function undelegateAll() external nonpayable onlyAgent
  NatSpec: Clears all WNat delegations.
- function delegateGovernance(address _to) external nonpayable onlyAgent
  NatSpec: Delegates governance vote power.
- function undelegateGovernance() external nonpayable onlyAgent
  NatSpec: Removes governance delegation.
- function claimDelegationRewards(IRewardManager _rewardManager, uint24 _lastRewardEpoch, IRewardManager.RewardClaimWithProof[] calldata _proofs) external nonpayable onlyAgent nonReentrant returns (uint256)
  NatSpec: Claims WNat delegation rewards to pool.
- function claimAirdropDistribution(IDistributionToDelegators _distribution, uint256 _month) external nonpayable onlyAgent nonReentrant returns (uint256)
  NatSpec: Claims airdrop distribution to pool.
- function optOutOfAirdrop(IDistributionToDelegators _distribution) external nonpayable onlyAgent nonReentrant
  NatSpec: Opts out pool from future airdrops.
- function implementation() external view returns (address)
  NatSpec: Returns current implementation address (UUPS).
- function _authorizeUpgrade(address) internal nonpayable virtual override onlyAssetManager
  NatSpec: Restricts upgrades to AssetManager only.
- function isAgentVaultOwner(address _address) internal view returns (bool)
  NatSpec: Checks if address is owner of agent vault.
- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  NatSpec: ERC-165 support: IERC165, ICollateralPool, IICollateralPool.


## SUMMARY OF FILE: 2025-08-flare/contracts/collateralPool/implementation/CollateralPoolFactory.sol
# CollateralPoolFactory (Solidity 0.8.27)

A minimal, trustless factory that deploys ERC1967 proxy instances for CollateralPool contracts and calls their initializer. It holds no user funds and has no admin/owner privileges. Anyone can create a pool. Trust rests on the provided implementation address and CollateralPool’s initialize logic. Major entrypoints: create (deploy + init pool), upgradeInitCall (upgrade helper), supportsInterface (ERC165).

Storage
- implementation (address) — proxy impl target

Functions

- constructor(address _implementation) nonpayable
  NatSpec: Set the implementation address used by new ERC1967 proxies.

- function create(
    IIAssetManager _assetManager,
    address _agentVault,
    AgentSettings.Data memory _settings
  ) external override returns (IICollateralPool) nonpayable
  NatSpec: Deploy ERC1967 proxy, then initialize CollateralPool with agent vault, asset manager, fAsset and exit CR.

- function upgradeInitCall(address /* _proxy */) external pure override returns (bytes memory)
  NatSpec: Return empty calldata for upgradeToAndCall; no init on upgrade required.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  NatSpec: ERC165 support for IERC165 and IICollateralPoolFactory interfaces.

Implementation details
- Uses ERC1967Proxy to create upgradeable pools; initializer called immediately.
- fAsset obtained via _assetManager.fAsset(); address extracted for initialize.
- _settings.poolExitCollateralRatioBIPS is cast to uint32 via SafeCast.
- No access control; open factory. Holds no funds.



## SUMMARY OF FILE: 2025-08-flare/contracts/collateralPool/implementation/CollateralPoolToken.sol
# CollateralPoolToken.sol – Summary

Trust-minimal ERC-20 representing Collateral Pool Tokens (CPTs) minted/burned exclusively by a CollateralPool. Transfers are constrained by time locks and fee-debt rules queried from the CollateralPool. AssetManager (via CollateralPool) is the sole UUPS upgrader. No owner; users hold ERC-20 balances but can only transfer the portion that is both non–time-locked and debt-free. Major entrypoints: mint, burn, balance helpers (locked/transferable/debt/timelock), cleanupExpiredTimelocks, and ERC165 supportsInterface.

Storage

- address public collateralPool – pool contract addr
- string private tokenName – ERC20 name override
- string private tokenSymbol – ERC20 symbol override
- mapping(address => TimelockQueue) private timelocksByAccount – per-account lock queue
- bool private ignoreTimelocked – bypass flag for pool burns
- bool private initialized – re-init guard

Functions

- constructor(address _collateralPool, string _tokenName, string _tokenSymbol) nonpayable
  - Initializes implementation (tests); calls initialize.

- function initialize(address _collateralPool, string _tokenName, string _tokenSymbol) public nonpayable
  - One-time init: sets pool, name, symbol.

- function name() public view returns (string) [virtual override]
  - Returns token name from storage.

- function symbol() public view returns (string) [virtual override]
  - Returns token symbol from storage.

- function mint(address _account, uint256 _amount) external onlyCollateralPool nonpayable returns (uint256 _timelockExpiresAt)
  - Mints CPTs; enqueues timelock; returns expiry.

- function burn(address _account, uint256 _amount, bool _ignoreTimelocked) external onlyCollateralPool nonpayable
  - Burns CPTs; can bypass timelock checks for payouts.

- function lockedBalanceOf(address _account) external view returns (uint256)
  - Max(lock by debt, lock by timelock).

- function transferableBalanceOf(address _account) external view returns (uint256)
  - Min(debt-free, non-timelocked) balance.

- function debtFreeBalanceOf(address _account) public view returns (uint256)
  - Queries pool for transferable (debt-free) CPTs.

- function debtLockedBalanceOf(address _account) public view returns (uint256)
  - Queries pool for debt-locked CPTs.

- function timelockedBalanceOf(address _account) public view returns (uint256 _timelocked)
  - Sums unexpired timelock queue; clamps to balance.

- function nonTimelockedBalanceOf(address _account) public view returns (uint256)
  - balanceOf - timelockedBalance.

- function cleanupExpiredTimelocks(address _account, uint256 _maxTimelockedEntries) public nonpayable returns (bool _cleanedAllExpired)
  - Clears up to N expired timelock entries.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  - ERC165: IERC165, IERC20, ICollateralPoolToken.

- function implementation() external view returns (address)
  - Returns current UUPS implementation address.

Internal / Overrides

- function _beforeTokenTransfer(address _from, address _to, uint256 _amount) internal override nonpayable
  - Enforces debt-free and timelock constraints; allows pool bypass when flagged.

- function _getTimelockDuration() internal view returns (uint256)
  - Reads timelock seconds from AssetManager via pool.

- function _authorizeUpgrade(address _newImplementation) internal virtual override nonpayable
  - Only AssetManager (via pool) can authorize UUPS upgrade.

Modifiers

- onlyCollateralPool: restricts caller to collateralPool address.

Interfaces/Deps

- Relies on IICollateralPool.debtFreeTokensOf / debtLockedTokensOf and .assetManager().
- UUPSUpgradeable; ERC20 base; ERC165 detection for IERC20 + ICollateralPoolToken.

Errors (thrown via require):
- OnlyAssetManager, InsufficientNonTimelockedBalance, InsufficientTransferableBalance, AlreadyInitialized, OnlyCollateralPool.


## SUMMARY OF FILE: 2025-08-flare/contracts/collateralPool/implementation/CollateralPoolTokenFactory.sol
CollateralPoolTokenFactory (factory)

Summary
- Purpose: Deploys ERC1967 proxy instances of CollateralPoolToken and initializes them for a specific agent/pool pair.
- Trust model: No user funds held. No admin controls on factory; implementation address is fixed at construction. Upgrades for created tokens are via UUPS at token level (factory cannot upgrade).
- Major entrypoints: create (deploy/initialize proxy), upgradeInitCall (upgrade helper), supportsInterface (ERC165).

Storage
- implementation (address public) – token logic impl

Constants (not storage)
- TOKEN_NAME_PREFIX (string internal constant) – name prefix
- TOKEN_SYMBOL_PREFIX (string internal constant) – symbol prefix

Functions
- constructor(address _implementation)
  • Visibility: public; Mutability: nonpayable; Modifiers: none
  • Natspec: Set the CollateralPoolToken implementation address used for new proxies.

- function create(IICollateralPool _pool, string memory _systemSuffix, string memory _agentSuffix) external override returns (address)
  • Visibility: external; Mutability: nonpayable; Modifiers: override
  • Natspec: Deploy ERC1967 proxy, initialize token with pool, name, symbol; return new token address.

- function upgradeInitCall(address _proxy) external pure override returns (bytes memory)
  • Visibility: external; Mutability: pure; Modifiers: override
  • Natspec: Return empty calldata for upgradeToAndCall; no upgrade-time initializer needed.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  • Visibility: external; Mutability: pure; Modifiers: override
  • Natspec: ERC165 support check for IERC165 and IICollateralPoolTokenFactory.

Notes
- Token names/symbols are composed as: "FAsset Collateral Pool Token {system}-{agent}" and "FCPT-{system}-{agent}".
- Proxies are ERC1967; actual upgrade authority is defined in CollateralPoolToken’s UUPS access control.


## SUMMARY OF FILE: 2025-08-flare/contracts/coreVaultManager/implementation/CoreVaultManager.sol
# CoreVaultManager (Solidity 0.8.27)

Trust model and purpose (≤100 words)
CoreVaultManager orchestrates off-chain Core Vault (CV) fund movements for FAssets. It tracks confirmed deposits via FDC, queues withdrawals, schedules daily escrows, and emits off-chain executable instructions. AssetManager is the only caller for creating/canceling transfer requests. Governance configures destinations, timing, fees, and can pause; emergency senders may pause. Triggering accounts emit batched Payment/Escrow instructions; no ERC20 user funds are held on-chain—this contract tracks CV balances on the underlying chain. Major entrypoints: confirmPayment, request/cancel transfer, triggerInstructions, processEscrows, pause/unpause, and a suite of governance setters/getters.

Storage variables
- assetManager: address — AssetMgr contract
- chainId: bytes32 — Underlying chain id
- custodianAddress: string — CV custodian addr
- coreVaultAddressHash: bytes32 — CV addr hash
- coreVaultAddress: string — CV readable addr
- nextSequenceNumber: uint256 — Next instr seq
- fdcVerification: IFdcVerification — FDC verifier
- confirmedPayments: mapping(bytes32=>bool) — Seen txids
- preimageHashes: EnumerableSet.Bytes32Set — Escrow secrets
- escrows: Escrow[] — Escrow schedule
- preimageHashToEscrowIndex: mapping(bytes32=>uint256) — Escrow idx map
- nextUnusedPreimageHashIndex: uint256 — Next preimage idx
- nextUnprocessedEscrowIndex: uint256 — Next process idx
- nextTransferRequestId: uint256 — Next req id
- cancelableTransferRequests: uint256[] — Cancelable req ids
- nonCancelableTransferRequests: uint256[] — Non-cancel req ids
- transferRequestById: mapping(uint256=>TransferRequest) — Req store
- allowedDestinationAddresses: string[] — Allowed dests
- allowedDestinationAddressIndex: mapping(string=>uint256) — Dest idx 1b
- triggeringAccounts: EnumerableSet.AddressSet — Who can trigger
- emergencyPauseSenders: EnumerableSet.AddressSet — Who can pause
- escrowEndTimeSeconds: uint128 — Daily end seconds
- escrowAmount: uint128 — Escrow size per day
- minimalAmount: uint128 — Min left in CV
- fee: uint128 — Per-payment fee
- availableFunds: uint128 — Free funds CV
- escrowedFunds: uint128 — Funds in escrow
- cancelableTransferRequestsAmount: uint128 — Sum cancelable
- nonCancelableTransferRequestsAmount: uint128 — Sum non-cancel
- paused: bool — Global pause flag

Functions (interface, visibility, modifiers, mutability) with brief notes
- constructor() — public nonpayable
  - Initializes inheritance; no state set.

- function initialize(IGovernanceSettings _governanceSettings, address _initialGovernance, address _addressUpdater, address _assetManager, bytes32 _chainId, string memory _custodianAddress, string memory _coreVaultAddress, uint256 _nextSequenceNumber) external nonpayable
  - One-time proxy init; wires governance, updater, CV params.

- function confirmPayment(IPayment.Proof calldata _proof) external nonpayable
  - Verifies FDC payment into CV and credits availableFunds.

- function requestTransferFromCoreVault(string memory _destinationAddress, bytes32 _paymentReference, uint128 _amount, bool _cancelable) external onlyAssetManager notPaused nonpayable returns (bytes32)
  - Queue CV payout to allowed address; checks funds and fee.

- function cancelTransferRequestFromCoreVault(string memory _destinationAddress) external onlyAssetManager nonpayable
  - Cancels existing cancelable request for destination.

- function processEscrows(uint256 _maxCount) external nonpayable returns (bool)
  - Processes expired/finished escrows; updates balances.

- function triggerInstructions() external notPaused nonpayable returns (uint256 _numberOfInstructions)
  - Emits batched Payment/Escrow instructions; advances sequence.

- function addAllowedDestinationAddresses(string[] calldata _allowedDestinationAddresses) external onlyGovernance nonpayable
  - Adds allowed payout destination addresses.

- function removeAllowedDestinationAddresses(string[] calldata _allowedDestinationAddresses) external onlyGovernance nonpayable
  - Removes allowed payout destination addresses.

- function addTriggeringAccounts(address[] calldata _triggeringAccounts) external onlyGovernance nonpayable
  - Grants trigger permission to accounts.

- function removeTriggeringAccounts(address[] calldata _triggeringAccounts) external onlyGovernance nonpayable
  - Revokes trigger permission from accounts.

- function updateCustodianAddress(string calldata _custodianAddress) external onlyGovernance nonpayable
  - Updates human-readable custodian address.

- function updateSettings(uint128 _escrowEndTimeSeconds, uint128 _escrowAmount, uint128 _minimalAmount, uint128 _fee) external onlyGovernance nonpayable
  - Sets escrow window/amount, CV minimum and fee.

- function addPreimageHashes(bytes32[] calldata _preimageHashes) external onlyImmediateGovernance nonpayable
  - Supplies preimage hashes used for escrows.

- function removeUnusedPreimageHashes(uint256 _maxCount) external onlyImmediateGovernance nonpayable
  - Removes last unused preimage hashes up to count.

- function setEscrowsFinished(bytes32[] calldata _preimageHashes) external onlyImmediateGovernance nonpayable
  - Marks escrows finished and adjusts balances.

- function addEmergencyPauseSenders(address[] calldata _addresses) external onlyImmediateGovernance nonpayable
  - Adds addresses authorized to emergency pause.

- function removeEmergencyPauseSenders(address[] calldata _addresses) external onlyImmediateGovernance nonpayable
  - Removes addresses authorized to emergency pause.

- function pause() external nonpayable
  - Pauses; callable by governance or emergency sender.

- function unpause() external onlyImmediateGovernance nonpayable
  - Unpauses contract operations.

- function triggerCustomInstructions(bytes32 _instructionsHash) external onlyImmediateGovernance nonpayable
  - Emits custom instruction event and increments sequence.

- function getSettings() external view returns (uint128 _escrowEndTimeSeconds, uint128 _escrowAmount, uint128 _minimalAmount, uint128 _fee)
  - Reads escrow scheduling and fee settings.

- function getAllowedDestinationAddresses() external view returns (string[] memory)
  - Lists all allowed destination addresses.

- function isDestinationAddressAllowed(string memory _address) external view returns (bool)
  - Checks if destination is allowed.

- function getTriggeringAccounts() external view returns (address[] memory)
  - Lists accounts allowed to trigger instructions.

- function getUnprocessedEscrows() external view returns (Escrow[] memory _unprocessedEscrows)
  - Returns escrows from nextUnprocessed index onward.

- function getEscrowsCount() external view returns (uint256)
  - Returns total escrows count.

- function getEscrowByIndex(uint256 _index) external view returns (Escrow memory)
  - Returns escrow at index.

- function getEscrowByPreimageHash(bytes32 _preimageHash) external view returns (Escrow memory)
  - Returns escrow by preimage hash.

- function getUnusedPreimageHashes() external view returns (bytes32[] memory)
  - Returns preimage hashes not yet used for escrows.

- function getPreimageHashesCount() external view returns (uint256)
  - Returns count of stored preimage hashes.

- function getPreimageHash(uint256 _index) external view returns (bytes32)
  - Returns preimage hash at index.

- function getCancelableTransferRequests() external view returns (TransferRequest[] memory _transferRequests)
  - Returns cancelable transfer requests.

- function getNonCancelableTransferRequests() external view returns (TransferRequest[] memory _transferRequests)
  - Returns non-cancelable transfer requests.

- function totalRequestAmountWithFee() public view returns (uint256)
  - Sums requested amounts plus per-request fees.

- function getEmergencyPauseSenders() external view returns (address[] memory)
  - Lists emergency pause senders.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  - ERC-165 support for IERC165, IIAddressUpdatable, IICoreVaultManager.

Operational notes
- Emits PaymentInstructions/EscrowInstructions for off-chain, multisig-operated CV to execute.
- Uses FDC proofs to account inbound CV deposits.
- Enforces bounded request lists; merging for non-cancelable by destination.
- Governance and emergency pause controls; instruction triggering gated by allowlist.
- Errors (examples): AmountZero, DestinationNotAllowed, InsufficientFunds, NotAuthorized, ContractPaused, etc.


## SUMMARY OF FILE: 2025-08-flare/contracts/coreVaultManager/implementation/CoreVaultManagerProxy.sol
Brief summary
CoreVaultManagerProxy is a minimal ERC1967 upgradeable proxy that deploys and atomically initializes CoreVaultManager. It doesn’t handle user funds directly; upgrade authority and governance/trust live in the CoreVaultManager implementation (initialized via governance settings). Entrypoints are the constructor at deployment, plus fallback/receive that delegate all calls/ETH to the implementation.

Storage variables
- None (declared in this contract) – uses ERC1967 slots in parent
- implementation (ERC1967 slot) – logic address
- rollback (ERC1967 slot) – upgrade guard

Functions
- constructor(
    address _implementationAddress,
    IGovernanceSettings _governanceSettings,
    address _initialGovernance,
    address _addressUpdater,
    address _assetManager,
    bytes32 _chainId,
    string memory _custodianAddress,
    string memory _coreVaultAddress,
    uint256 _nextSequenceNumber
  ) public nonpayable
  // Deploy proxy and call CoreVaultManager.initialize with arguments.

- fallback() external payable
  // Delegate unknown calls to CoreVaultManager implementation.

- receive() external payable
  // Accept native tokens; delegate if appropriate.


## SUMMARY OF FILE: 2025-08-flare/contracts/diamond/facets/DiamondLoupeFacet.sol
# DiamondLoupeFacet (EIP‑2535 Introspection)

Read-only facet exposing EIP-2535 Diamond Loupe views over a diamond’s routing table. No user funds held; no admin powers; pure introspection of LibDiamond storage. Intended for tools/clients to discover facets, selectors, and interface support. Major entrypoints: facets, facetFunctionSelectors, facetAddresses, facetAddress, supportsInterface. Uses inline assembly to right-size dynamic arrays for gas.

Storage (via LibDiamond.DiamondStorage)
- selectors — All function selectors
- facetAddressAndSelectorPosition — Selector → facet + position
- supportedInterfaces — ERC‑165 interface flags

Functions
- interface Facet { address facetAddress; bytes4[] functionSelectors; }

- function facets() external view override returns (Facet[] memory facets_);
  /// Return all facet addresses with their function selectors.

- function facetFunctionSelectors(address _facet) external view override returns (bytes4[] memory _facetFunctionSelectors);
  /// List all selectors implemented by a given facet address.

- function facetAddresses() external view override returns (address[] memory facetAddresses_);
  /// Return unique addresses of all facets in the diamond.

- function facetAddress(bytes4 _functionSelector) external view override returns (address facetAddress_);
  /// Resolve the facet implementing a given function selector.

- function supportsInterface(bytes4 _interfaceId) external view override returns (bool);
  /// ERC‑165 support query using LibDiamond.supportedInterfaces.


## SUMMARY OF FILE: 2025-08-flare/contracts/diamond/mock/DiamondCutFacet.sol
# DiamondCutFacet

Short summary (≤100 words):
Governance-controlled facet enabling EIP-2535 upgrades on a Diamond. It exposes a single admin entrypoint, diamondCut, which applies add/replace/remove function selectors and can run an optional initializer via delegatecall. This facet itself holds no user funds; only governance can execute upgrades (onlyGovernance). Ensure DiamondLoupeFacet is also installed to satisfy EIP-2535 loupe requirements.

Storage
- None declared in this facet (uses shared Diamond storage via LibDiamond)

Functions
- function diamondCut(IDiamondCut.FacetCut[] calldata _diamondCut, address _init, bytes calldata _calldata) external override onlyGovernance
  - Mutability: nonpayable
  - Natspec: Execute diamond upgrade and optional initializer via delegatecall; governance only.


## SUMMARY OF FILE: 2025-08-flare/contracts/diamond/mock/DiamondInit.sol
# DiamondInit — EIP-2535 Diamond initializer

Summary
- Purpose: One-time initializer for a Diamond (EIP-2535). It wires governance and registers supported interfaces (ERC165, DiamondCut, DiamondLoupe) in diamond storage.
- Trust model: Admin/governance controlled. No user funds handled; called during diamond deployment/upgrade via delegatecall. Governance is the ultimate authority set here.
- Major entrypoints: init(IGovernanceSettings,address).

Storage
- GovernedBase.governanceSettings (IGovernanceSettings) — governance params
- GovernedBase.governance (address) — current governance
- LibDiamond.DiamondStorage.supportedInterfaces (mapping(bytes4=>bool)) — ERC165 flags

Functions
- function init(IGovernanceSettings _governanceSettings, address _initialGovernance) external
  • Mutability: nonpayable; Modifiers: none; Visibility: external
  • NatSpec: Initialize governance and register ERC165/diamond interfaces in diamond storage.


## SUMMARY OF FILE: 2025-08-flare/contracts/diamond/mock/Test1Facet.sol
Summary
A minimal Diamond facet demonstrating library-based (TestLib) diamond storage. It exposes write/read helpers for a single address in shared storage and many no-op stub functions. No funds are handled; there is no admin control in this facet. Trust model: stateless facet itself; state lives in TestLib diamond storage. Major entrypoints: test1Func1 (store address), test1Func2 (read address), supportsInterface (ERC-165-style check; returns false).

Storage
- None (facet holds no variables; state kept in TestLib diamond storage, e.g., address myAddress)

Functions
- test1Func1() external
  - Sets diamond storage address to address(this) via TestLib.
- test1Func2() external view returns (address)
  - Reads and returns address from TestLib diamond storage.
- test1Func3() external
  - No-op function; placeholder for facet expansion.
- test1Func4() external
  - No-op function; placeholder for facet expansion.
- test1Func5() external
  - No-op function; placeholder for facet expansion.
- test1Func6() external
  - No-op function; placeholder for facet expansion.
- test1Func7() external
  - No-op function; placeholder for facet expansion.
- test1Func8() external
  - No-op function; placeholder for facet expansion.
- test1Func9() external
  - No-op function; placeholder for facet expansion.
- test1Func10() external
  - No-op function; placeholder for facet expansion.
- test1Func11() external
  - No-op function; placeholder for facet expansion.
- test1Func12() external
  - No-op function; placeholder for facet expansion.
- test1Func13() external
  - No-op function; placeholder for facet expansion.
- test1Func14() external
  - No-op function; placeholder for facet expansion.
- test1Func15() external
  - No-op function; placeholder for facet expansion.
- test1Func16() external
  - No-op function; placeholder for facet expansion.
- test1Func17() external
  - No-op function; placeholder for facet expansion.
- test1Func18() external
  - No-op function; placeholder for facet expansion.
- test1Func19() external
  - No-op function; placeholder for facet expansion.
- test1Func20() external
  - No-op function; placeholder for facet expansion.
- supportsInterface(bytes4 _interfaceID) external view returns (bool)
  - ERC-165-like check; returns false in this example.


## SUMMARY OF FILE: 2025-08-flare/contracts/diamond/mock/Test2Facet.sol
# Test2Facet

Test2Facet is a minimal Diamond facet for selector/dispatch testing. It exposes 20 external, nonpayable no-op functions to populate a diamond’s function table. There is no storage, no funds handled, no admin, and no side-effects. Entrypoints are test2Func1 through test2Func20. Anyone can call these; they are intended solely for integration and gas/selector mapping tests.

## Storage Variables
- None

## Functions
- function test2Func1() external; [nonpayable] [modifiers: none] — No-op selector placeholder for diamond dispatch testing.
- function test2Func2() external; [nonpayable] [modifiers: none] — No-op function for selector table coverage.
- function test2Func3() external; [nonpayable] [modifiers: none] — Empty function used in facet testing.
- function test2Func4() external; [nonpayable] [modifiers: none] — No-op function, ensures selector availability.
- function test2Func5() external; [nonpayable] [modifiers: none] — Placeholder to expand diamond cut map.
- function test2Func6() external; [nonpayable] [modifiers: none] — No-op entry for routing verification.
- function test2Func7() external; [nonpayable] [modifiers: none] — Empty implementation for selector indexing.
- function test2Func8() external; [nonpayable] [modifiers: none] — No-op method for integration tests.
- function test2Func9() external; [nonpayable] [modifiers: none] — Dummy function to pad function table.
- function test2Func10() external; [nonpayable] [modifiers: none] — No-op to validate external call paths.
- function test2Func11() external; [nonpayable] [modifiers: none] — Empty function to test linkage.
- function test2Func12() external; [nonpayable] [modifiers: none] — No-op for selector collision checks.
- function test2Func13() external; [nonpayable] [modifiers: none] — Empty entry for ABI selector mapping.
- function test2Func14() external; [nonpayable] [modifiers: none] — No-op function exercising dispatch.
- function test2Func15() external; [nonpayable] [modifiers: none] — Placeholder used in diamond facet tests.
- function test2Func16() external; [nonpayable] [modifiers: none] — No-op to test call routing.
- function test2Func17() external; [nonpayable] [modifiers: none] — Empty body; ensures unique selector.
- function test2Func18() external; [nonpayable] [modifiers: none] — No-op, utility for gas/size testing.
- function test2Func19() external; [nonpayable] [modifiers: none] — Dummy external for coverage scaffolding.
- function test2Func20() external; [nonpayable] [modifiers: none] — No-op endpoint for selector gamut.


## SUMMARY OF FILE: 2025-08-flare/contracts/fassetToken/implementation/FAsset.sol
## Summary
FAsset is an upgradeable ERC-20 token representing wrapped underlying assets (FAssets). It integrates EIP-2612 permits, historical balance checkpoints, and ERC-165 introspection. Trust model: users hold tokens; AssetManager is the sole minter/burner and the only authority to upgrade and manage cleaning/pauses; the deployer can set AssetManager once. Major entrypoints: initialize, setAssetManager, mint, burn, transfer (subject to emergency pause), and history-cleaning controls.

## Storage Variables
- assetName (string) – Underlying asset name
- assetSymbol (string) – Underlying asset symbol
- cleanupBlockNumberManager (address) – History cleanup admin
- assetManager (address) – Authorized AM contract
- __terminatedAt (uint64) – Placeholder
- _name (string) – ERC20 name
- _symbol (string) – ERC20 symbol
- _decimals (uint8) – Token decimals
- _deployer (address) – Initializer/AM setter
- _initialized (bool) – Init guard
- _version (uint16) – Init versioning

## Modifiers
- onlyAssetManager – Restricts function to AssetManager

## Functions
- constructor() ERC20("", "")
  - visibility: constructor; mutability: nonpayable; modifiers: none
  - Natspec: Initializes immutable defaults; flags as initialized; sets version 1000.

- function initialize(string memory name_, string memory symbol_, string memory assetName_, string memory assetSymbol_, uint8 decimals_) external
  - visibility: external; mutability: nonpayable; modifiers: none
  - Natspec: One-time initializer setting names, decimals, deployer; triggers v1r1 EIP712 init.

- function initializeV1r1() public
  - visibility: public; mutability: nonpayable; modifiers: none
  - Natspec: One-time EIP712 domain initializer; sets version to 1.

- function setAssetManager(address _assetManager) external
  - visibility: external; mutability: nonpayable; modifiers: none
  - Natspec: Deployer-only, one-time AssetManager wiring; zero-address disallowed.

- function mint(address _owner, uint256 _amount) external override onlyAssetManager
  - visibility: external; mutability: nonpayable; modifiers: onlyAssetManager
  - Natspec: AssetManager mints fAssets to the specified owner.

- function burn(address _owner, uint256 _amount) external override onlyAssetManager
  - visibility: external; mutability: nonpayable; modifiers: onlyAssetManager
  - Natspec: AssetManager burns fAssets from the specified owner.

- function name() public view override(ERC20, IERC20Metadata) returns (string memory)
  - visibility: public; mutability: view; modifiers: none
  - Natspec: Returns token name.

- function symbol() public view override(ERC20, IERC20Metadata) returns (string memory)
  - visibility: public; mutability: view; modifiers: none
  - Natspec: Returns token symbol.

- function decimals() public view override(ERC20, IERC20Metadata) returns (uint8)
  - visibility: public; mutability: view; modifiers: none
  - Natspec: Returns token decimals.

- function setCleanupBlockNumber(uint256 _blockNumber) external override
  - visibility: external; mutability: nonpayable; modifiers: none
  - Natspec: Cleanup manager sets the history cleanup block number.

- function cleanupBlockNumber() external view override returns (uint256)
  - visibility: external; mutability: view; modifiers: none
  - Natspec: Gets the current cleanup block number.

- function setCleanerContract(address _cleanerContract) external override onlyAssetManager
  - visibility: external; mutability: nonpayable; modifiers: onlyAssetManager
  - Natspec: Sets contract authorized to perform history cleaning.

- function setCleanupBlockNumberManager(address _cleanupBlockNumberManager) external onlyAssetManager
  - visibility: external; mutability: nonpayable; modifiers: onlyAssetManager
  - Natspec: Sets address allowed to set cleanup block number.

- function _beforeTokenTransfer(address _from, address _to, uint256 _amount) internal override
  - visibility: internal; mutability: nonpayable; modifiers: none
  - Natspec: Enforces balance, forbids self-transfer, checks pause, updates checkpoints.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  - visibility: external; mutability: pure; modifiers: none
  - Natspec: Declares support for ERC165 and related interfaces.

- function _approve(address _owner, address _spender, uint256 _amount) internal virtual override(ERC20, ERC20Permit)
  - visibility: internal; mutability: nonpayable; modifiers: none
  - Natspec: Resolves multiple inheritance approve behavior.

- function implementation() external view returns (address)
  - visibility: external; mutability: view; modifiers: none
  - Natspec: Returns current UUPS implementation address.

- function _authorizeUpgrade(address /* _newImplementation */) internal virtual override onlyAssetManager
  - visibility: internal; mutability: nonpayable; modifiers: onlyAssetManager
  - Natspec: Restricts UUPS upgrades to AssetManager.


## SUMMARY OF FILE: 2025-08-flare/contracts/fassetToken/implementation/FAssetProxy.sol
## FAssetProxy (ERC1967Proxy)

A minimal ERC1967 proxy that deploys an upgradeable FAsset ERC‑20 via delegatecall to FAsset.initialize. The proxy itself is “adminless” (no external upgrade methods); upgrade authority is expected to live in the implementation (UUPS-style). User funds are standard ERC‑20 balances; no special custody in the proxy. Major entrypoints are the constructor (which encodes the initializer) and the proxy’s fallback/receive that delegate all calls and value to the FAsset implementation.

Storage
- _IMPLEMENTATION_SLOT — impl address slot (ERC1967)
- _ADMIN_SLOT — admin slot (unused)
- _BEACON_SLOT — beacon slot (unused)
- _ROLLBACK_SLOT — uups rollback flag

Functions
- constructor(
    address _implementationAddress,
    string memory _name,
    string memory _symbol,
    string memory _assetName,
    string memory _assetSymbol,
    uint8 _decimals
  ) public nonpayable
  • Deploy proxy and run FAsset.initialize with provided token metadata.

- fallback() external payable
  • Delegate all unknown calls to current implementation.

- receive() external payable
  • Accept ETH; forwards via fallback/delegate semantics to implementation.


## SUMMARY OF FILE: 2025-08-flare/contracts/ftso/implementation/FtsoV2PriceStore.sol
# FtsoV2PriceStore (Solidity 0.8.27)

Summary
- Purpose: On-chain storage and serving of FTSO v2 prices, verified via Relay Merkle proofs, plus median aggregation of trusted providers. No user funds held; governance configures feeds/providers. Major entrypoints: initialize, publishPrices, submitTrustedPrices, updateSettings, setTrustedProviders, and price getters. Trust model: Governance (onlyGovernance) controls feeds/thresholds; whitelisted providers submit trusted prices; anyone can publish proven prices after window closes.

Structs
- PriceStore
  - votingRoundId (uint32) – last FTSO root round
  - value (uint32) – last published price
  - decimals (int8) – published price decimals
  - trustedVotingRoundId (uint32) – last trusted median round
  - trustedValue (uint32) – trusted median price
  - trustedDecimals (int8) – trusted price decimals
  - numberOfSubmits (uint8) – trusted submits counted

Events
- PricesPublished(uint32 indexed votingRoundId)

Custom Errors
- InvalidStartTime, VotingEpochDurationTooShort, WrongNumberOfProofs, PricesAlreadyPublished, SubmissionWindowNotClosed,
  VotingRoundIdMismatch, FeedIdMismatch, ValueMustBeNonNegative, MerkleProofInvalid, OnlyTrustedProvider,
  AllPricesMustBeProvided, SubmissionWindowClosed, AlreadySubmitted, DecimalsMismatch, LengthMismatch, MaxSpreadTooBig,
  TooManyTrustedProviders, ThresholdTooHigh, SymbolNotSupported

Storage Variables
- firstVotingRoundStartTs (uint64) – first epoch start
- votingEpochDurationSeconds (uint64) – epoch duration
- submitTrustedPricesWindowSeconds (uint64) – trusted window
- ftsoProtocolId (uint8) – Relay protocol id
- feedIds (bytes21[]) – required feed ids
- symbolToFeedId (mapping(string=>bytes21)) – symbol→id map
- feedIdToSymbol (mapping(bytes21=>string)) – id→symbol map
- latestPrices (mapping(bytes21=>PriceStore)) – latest price store
- submittedTrustedPrices (mapping(bytes21=>mapping(uint32=>bytes))) – queued trusted bytes4 values
- lastVotingEpochIdByProvider (mapping(address=>uint256)) – provider’s last epoch
- trustedProviders (address[]) – whitelist providers
- trustedProvidersMap (mapping(address=>bool)) – fast provider check
- trustedProvidersThreshold (uint8) – min providers for median
- maxSpreadBIPS (uint16) – max allowed spread bips
- relay (IRelay) – Relay contract ref
- lastPublishedVotingRoundId (uint32) – last published round

Access Control & Timing
- Governance: updateSettings, setTrustedProviders.
- Trusted providers only: submitTrustedPrices, within [epochEnd, epochEnd + window).
- Anyone: publishPrices after trusted window closes for that round.

Workflow Highlights
- publishPrices verifies each feed via Merkle proof from Relay.merkleRoots(ftsoProtocolId, roundId) and stores latest value/decimals.
- For same round, if enough trusted submissions exist (>= threshold), compute median with spread check and store trustedValue.
- Decimals handling supports negative trustedDecimals by scaling into price output.

Functions
- constructor() public nonpayable
  /// Initialize base parents; marks as initialized in GovernedUUPS and AddressUpdatable.

- function initialize(
    IGovernanceSettings _governanceSettings,
    address _initialGovernance,
    address _addressUpdater,
    uint64 _firstVotingRoundStartTs,
    uint8 _votingEpochDurationSeconds,
    uint8 _ftsoProtocolId
  ) external nonpayable
  /// One-time setup of governance, address updater, timing, protocol id.

- function publishPrices(IPricePublisher.FeedWithProof[] calldata _proofs) external nonpayable
  /// Publish one-round prices for all feeds after trusted window closes.

- function submitTrustedPrices(uint32 _votingRoundId, IPricePublisher.TrustedProviderFeed[] calldata _feeds)
  external nonpayable
  /// Trusted providers submit per-feed uint32 prices during submission window.

- function updateSettings(
    bytes21[] calldata _feedIds,
    string[] calldata _symbols,
    int8[] calldata _trustedDecimals,
    uint16 _maxSpreadBIPS
  ) external onlyGovernance nonpayable
  /// Set feeds, symbols, trusted decimals, and max spread; resets affected state.

- function setTrustedProviders(address[] calldata _trustedProviders, uint8 _trustedProvidersThreshold)
  external onlyGovernance nonpayable
  /// Update whitelist and threshold for trusted median computation.

- function getPrice(string memory _symbol)
  external view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
  /// Read last Relay-published price for symbol with epoch end timestamp.

- function getPriceFromTrustedProviders(string memory _symbol)
  external view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
  /// Read last trusted-median price for symbol with epoch end.

- function getPriceFromTrustedProvidersWithQuality(string memory _symbol)
  external view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals, uint8 _numberOfSubmits)
  /// Read trusted price plus number of provider submissions used.

- function getFeedIds() external view returns (bytes21[] memory)
  /// Return configured feed ids publish order.

- function getFeedIdsWithDecimals() external view returns (bytes21[] memory _feedIds, int8[] memory _decimals)
  /// Return feed ids and currently configured trusted decimals.

- function getSymbols() external view returns (string[] memory _symbols)
  /// List of supported symbols in publish order.

- function getFeedId(string memory _symbol) external view returns (bytes21)
  /// Lookup feed id by symbol.

- function getTrustedProviders() external view returns (address[] memory)
  /// Return the current whitelist of trusted providers.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  /// ERC-165 support: IERC165, IPriceReader, IPricePublisher.

Internal/Private Functions
- function _updateContractAddresses(bytes32[] memory _contractNameHashes, address[] memory _contractAddresses)
  internal override nonpayable
  /// Pull and cache Relay address from AddressUpdater registry.

- function _getPreviousVotingEpochId() internal view returns (uint32)
  /// Compute previous epoch id from now, startTs and epoch duration.

- function _getEndTimestamp(uint256 _votingEpochId) internal view returns (uint256)
  /// Compute epoch end timestamp for given id.

- function _getPriceFromTrustedProviders(PriceStore storage _feed)
  internal view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
  /// Format trusted price and decimals; scale for negative decimals.

- function _calculateMedian(bytes memory _prices)
  internal view returns (uint256 _medianPrice, bool _priceOk)
  /// Insertion-sort median of uint32 list; enforce max spread vs median.

Notes & Constraints
- Proof set must cover all configured feeds in exact order.
- Trusted values stored as concatenated bytes4 per provider per feed per round.
- Median accepted only if neighbor spread <= maxSpreadBIPS.
- Governance decimals change clears pending trusted submissions and resets trusted price.
- Interfaces implemented: IERC165, IPriceReader, IPricePublisher.



## SUMMARY OF FILE: 2025-08-flare/contracts/ftso/implementation/FtsoV2PriceStoreProxy.sol
# FtsoV2PriceStoreProxy (ERC1967 UUPS Proxy)

A minimal ERC1967 proxy deploying and wiring a FtsoV2PriceStore implementation via initialize. It holds no user funds; it only delegates calls. Trust model: upgrade/governance is enforced by the implementation’s UUPS logic and GovernanceSettings; proxy itself exposes only fallback/receive. Major entrypoints: constructor (initializes implementation), fallback, receive.

## Storage Variables
- EIP1967_IMPLEMENTATION (address) – current implementation
- EIP1967_ADMIN (address) – admin slot (unused by proxy at runtime)
- EIP1967_BEACON (address) – beacon slot (unused here)

## Functions

- constructor(
    address _implementationAddress,
    IGovernanceSettings _governanceSettings,
    address _initialGovernance,
    address _addressUpdater,
    uint64 _firstVotingRoundStartTs,
    uint8 _votingEpochDurationSeconds,
    uint8 _ftsoProtocolId
  ) public nonpayable
  - Initializes proxy and calls FtsoV2PriceStore.initialize with parameters.

- fallback() external payable
  - Delegates unknown calls to the current implementation.

- receive() external payable
  - Accepts ETH and delegates per Proxy behavior.

Notes
- Constructor forwards encoded initialize via abi.encodeCall(FtsoV2PriceStore.initialize,...).
- Upgrades are controlled by UUPS functions on implementation and its governance.



## SUMMARY OF FILE: 2025-08-flare/contracts/ftso/mock/FakePriceReader.sol
Summary
A minimal, test-only price oracle implementing IPriceReader, IPriceChangeEmitter, and ERC-165. A single trusted provider address can set per-symbol decimals and publish both normal and “trusted” prices; consumers can read prices and timestamps. No user funds are held; the provider (admin) is the only mutator and can emit a PricesPublished event. Major entrypoints: setDecimals, setPrice, setPriceFromTrustedProviders, finalizePrices, getPrice, getPriceFromTrustedProviders, getPriceFromTrustedProvidersWithQuality.

Storage
- provider (address, public) – authorized price publisher
- pricingData (mapping(string => PricingData), private) – per-symbol pricing store

Structs
- PricingData { uint8 decimals; uint128 price; uint64 timestamp; uint128 trustedPrice; uint64 trustedTimestamp }

Modifiers
- onlyDataProvider – restricts calls to provider

Functions
- constructor(address _provider) nonpayable
  NatSpec: Initialize the provider address authorized to publish and finalize prices.

- function setDecimals(string memory _symbol, uint256 _decimals) external onlyDataProvider nonpayable
  NatSpec: Set symbol decimals; must be set before any price read/write.

- function setPrice(string memory _symbol, uint256 _price) external onlyDataProvider nonpayable
  NatSpec: Set current price and timestamp for a symbol.

- function setPriceFromTrustedProviders(string memory _symbol, uint256 _price) external onlyDataProvider nonpayable
  NatSpec: Set trusted price and timestamp for a symbol.

- function finalizePrices() external onlyDataProvider nonpayable
  NatSpec: Emit PricesPublished(0) to signal price publication finalized.

- function getPrice(string memory _symbol) external view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
  NatSpec: Read last price, timestamp, and decimals for symbol.

- function getPriceFromTrustedProviders(string memory _symbol) external view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
  NatSpec: Read last trusted price, timestamp, and decimals for symbol.

- function getPriceFromTrustedProvidersWithQuality(string memory _symbol) external view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals, uint8 _numberOfSubmits)
  NatSpec: Read trusted price with quality metric; numberOfSubmits is always 0.

- function supportsInterface(bytes4 _interfaceId) external pure returns (bool)
  NatSpec: ERC-165 support for IERC165, IPriceReader, and IPriceChangeEmitter.

- function _getPricingData(string memory _symbol) private view returns (PricingData storage)
  NatSpec: Internal fetch; reverts if decimals not initialized for symbol.


## SUMMARY OF FILE: 2025-08-flare/contracts/utils/mock/FakeERC20.sol
### Summary
Governance-controlled ERC20 test token with configurable decimals. Governance can mint to any address; holders can burn their own tokens. Implements ERC165 to advertise IERC20 and IERC20Metadata support. Trust model: admin (governance) controls supply; users only transfer/approve/burn own balances. Major entrypoints: mintAmount (onlyGovernance), burnAmount, standard ERC20 methods, supportsInterface.

### Storage Variables
- decimals_ (uint8, immutable, private) — token decimals

### Constructor
- constructor(IGovernanceSettings _governanceSettings, address _initialGovernance, string _name, string _symbol, uint8 _decimals) public (nonpayable)
  - Initializes name/symbol/decimals and governance settings.

### Functions
- function mintAmount(address _target, uint256 amount) public onlyGovernance (nonpayable)
  - Governance mints tokens to target address.

- function burnAmount(uint256 _amount) public (nonpayable)
  - Burn caller’s tokens from their balance.

- function decimals() public view override returns (uint8)
  - Return immutable decimals set at construction.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  - ERC165: signals support for IERC165, IERC20, IERC20Metadata.

### Inherited ERC20 Public Interface (via OpenZeppelin)
- function name() public view returns (string memory)
  - Token name string.

- function symbol() public view returns (string memory)
  - Token symbol string.

- function totalSupply() public view returns (uint256)
  - Total minted minus burned tokens.

- function balanceOf(address account) public view returns (uint256)
  - Balance of an account.

- function transfer(address to, uint256 amount) public returns (bool)
  - Transfer tokens to recipient.

- function allowance(address owner, address spender) public view returns (uint256)
  - Remaining allowance spender can use.

- function approve(address spender, uint256 amount) public returns (bool)
  - Set spender’s allowance.

- function transferFrom(address from, address to, uint256 amount) public returns (bool)
  - Move tokens using allowance mechanism.


## SUMMARY OF FILE: 2025-08-flare/contracts/utils/mock/TestUUPSProxyImpl.sol
## Summary
TestUUPSProxyImpl is a minimal UUPS-upgradeable test implementation. It stores a message and a simple initialized flag. Trust model: no admin/access control; _authorizeUpgrade allows anyone to upgrade, and initialize is unrestricted, so not for production and should not hold user funds. Major entrypoints: initialize, testResult, implementation, and inherited UUPS upgrade functions (upgradeTo, upgradeToAndCall, proxiableUUID).

## Storage
- uint256[1000] _dummy — storage gap
- string message — stored message
- bool initialized — init flag

## Functions
- `function _authorizeUpgrade(address newImplementation) internal override` — Authorizes upgrades; unprotected in this test implementation.
- `function initialize(string memory _message) external` — Sets message and marks initialized; callable multiple times.
- `function testResult() external view returns (string memory)` — Returns message if initialized, otherwise default string.
- `function implementation() external view returns (address)` — Returns current implementation address from ERC1967 slot.

Inherited (UUPSUpgradeable):
- `function upgradeTo(address newImplementation) external payable onlyProxy` — Upgrade proxy implementation; calls _authorizeUpgrade.
- `function upgradeToAndCall(address newImplementation, bytes calldata data) external payable onlyProxy` — Upgrade and execute call on new implementation.
- `function proxiableUUID() external view notDelegated returns (bytes32)` — ERC1822 UUID; reverts if called through proxy.


## Main List of Files in Project

contracts/agentOwnerRegistry/implementation/AgentOwnerRegistry.sol
contracts/agentOwnerRegistry/implementation/AgentOwnerRegistryProxy.sol
contracts/agentVault/implementation/AgentVault.sol
contracts/agentVault/implementation/AgentVaultFactory.sol
contracts/assetManager/facets/AgentAlwaysAllowedMintersFacet.sol
contracts/assetManager/facets/AgentCollateralFacet.sol
contracts/assetManager/facets/AgentInfoFacet.sol
contracts/assetManager/facets/AgentPingFacet.sol
contracts/assetManager/facets/AgentSettingsFacet.sol
contracts/assetManager/facets/AgentVaultAndPoolSupportFacet.sol
contracts/assetManager/facets/AgentVaultManagementFacet.sol
contracts/assetManager/facets/AssetManagerBase.sol
contracts/assetManager/facets/AssetManagerDiamondCutFacet.sol
contracts/assetManager/facets/AssetManagerInit.sol
contracts/assetManager/facets/AvailableAgentsFacet.sol
contracts/assetManager/facets/ChallengesFacet.sol
contracts/assetManager/facets/CollateralReservationsFacet.sol
contracts/assetManager/facets/CollateralTypesFacet.sol
contracts/assetManager/facets/CoreVaultClientFacet.sol
contracts/assetManager/facets/CoreVaultClientSettingsFacet.sol
contracts/assetManager/facets/EmergencyPauseFacet.sol
contracts/assetManager/facets/EmergencyPauseTransfersFacet.sol
contracts/assetManager/facets/LiquidationFacet.sol
contracts/assetManager/facets/MintingDefaultsFacet.sol
contracts/assetManager/facets/MintingFacet.sol
contracts/assetManager/facets/RedemptionConfirmationsFacet.sol
contracts/assetManager/facets/RedemptionDefaultsFacet.sol
contracts/assetManager/facets/RedemptionRequestsFacet.sol
contracts/assetManager/facets/RedemptionTimeExtensionFacet.sol
contracts/assetManager/facets/SettingsManagementFacet.sol
contracts/assetManager/facets/SettingsReaderFacet.sol
contracts/assetManager/facets/SystemInfoFacet.sol
contracts/assetManager/facets/SystemStateManagementFacet.sol
contracts/assetManager/facets/UnderlyingBalanceFacet.sol
contracts/assetManager/facets/UnderlyingTimekeepingFacet.sol
contracts/assetManager/implementation/AssetManager.sol
contracts/assetManager/library/AgentBacking.sol
contracts/assetManager/library/AgentCollateral.sol
contracts/assetManager/library/AgentPayout.sol
contracts/assetManager/library/AgentUpdates.sol
contracts/assetManager/library/Agents.sol
contracts/assetManager/library/CollateralTypes.sol
contracts/assetManager/library/Conversion.sol
contracts/assetManager/library/CoreVaultClient.sol
contracts/assetManager/library/Globals.sol
contracts/assetManager/library/Liquidation.sol
contracts/assetManager/library/LiquidationPaymentStrategy.sol
contracts/assetManager/library/Minting.sol
contracts/assetManager/library/RedemptionDefaults.sol
contracts/assetManager/library/RedemptionQueueInfo.sol
contracts/assetManager/library/RedemptionRequests.sol
contracts/assetManager/library/Redemptions.sol
contracts/assetManager/library/SettingsInitializer.sol
contracts/assetManager/library/SettingsUpdater.sol
contracts/assetManager/library/SettingsValidators.sol
contracts/assetManager/library/TransactionAttestation.sol
contracts/assetManager/library/UnderlyingBalance.sol
contracts/assetManager/library/UnderlyingBlockUpdater.sol
contracts/assetManager/library/data/Agent.sol
contracts/assetManager/library/data/AssetManagerState.sol
contracts/assetManager/library/data/Collateral.sol
contracts/assetManager/library/data/CollateralReservation.sol
contracts/assetManager/library/data/CollateralTypeInt.sol
contracts/assetManager/library/data/PaymentConfirmations.sol
contracts/assetManager/library/data/PaymentReference.sol
contracts/assetManager/library/data/Redemption.sol
contracts/assetManager/library/data/RedemptionQueue.sol
contracts/assetManager/library/data/RedemptionTimeExtension.sol
contracts/assetManager/library/data/UnderlyingAddressOwnership.sol
contracts/assetManagerController/implementation/AssetManagerController.sol
contracts/assetManagerController/implementation/AssetManagerControllerProxy.sol
contracts/collateralPool/implementation/CollateralPool.sol
contracts/collateralPool/implementation/CollateralPoolFactory.sol
contracts/collateralPool/implementation/CollateralPoolToken.sol
contracts/collateralPool/implementation/CollateralPoolTokenFactory.sol
contracts/coreVaultManager/implementation/CoreVaultManager.sol
contracts/coreVaultManager/implementation/CoreVaultManagerProxy.sol
contracts/diamond/facets/DiamondLoupeFacet.sol
contracts/diamond/implementation/Diamond.sol
contracts/diamond/library/LibDiamond.sol
contracts/fassetToken/implementation/CheckPointable.sol
contracts/fassetToken/implementation/FAsset.sol
contracts/fassetToken/implementation/FAssetProxy.sol
contracts/fassetToken/library/CheckPointHistory.sol
contracts/fassetToken/library/CheckPointsByAddress.sol
contracts/flareSmartContracts/implementation/AddressUpdatable.sol
contracts/ftso/implementation/FtsoV2PriceStore.sol
contracts/ftso/implementation/FtsoV2PriceStoreProxy.sol
contracts/governance/implementation/Governed.sol
contracts/governance/implementation/GovernedBase.sol
contracts/governance/implementation/GovernedProxyImplementation.sol
contracts/governance/implementation/GovernedUUPSProxyImplementation.sol
contracts/userInterfaces/IAgentAlwaysAllowedMinters.sol
contracts/userInterfaces/IAgentOwnerRegistry.sol
contracts/userInterfaces/IAgentPing.sol
contracts/userInterfaces/IAgentVault.sol
contracts/userInterfaces/IAssetManager.sol
contracts/userInterfaces/IAssetManagerController.sol
contracts/userInterfaces/IAssetManagerEvents.sol
contracts/userInterfaces/ICollateralPool.sol
contracts/userInterfaces/ICollateralPoolToken.sol
contracts/userInterfaces/ICoreVaultClient.sol
contracts/userInterfaces/ICoreVaultClientSettings.sol
contracts/userInterfaces/ICoreVaultManager.sol
contracts/userInterfaces/IFAsset.sol
contracts/userInterfaces/IRedemptionTimeExtension.sol
contracts/userInterfaces/data/AgentInfo.sol
contracts/userInterfaces/data/AgentSettings.sol
contracts/userInterfaces/data/AssetManagerSettings.sol
contracts/userInterfaces/data/AvailableAgentInfo.sol
contracts/userInterfaces/data/CollateralReservationInfo.sol
contracts/userInterfaces/data/CollateralType.sol
contracts/userInterfaces/data/RedemptionRequestInfo.sol
contracts/userInterfaces/data/RedemptionTicketInfo.sol
contracts/utils/Imports_Solidity_0_6.sol
contracts/utils/library/MathUtils.sol
contracts/utils/library/MerkleTree.sol
contracts/utils/library/SafeMath64.sol
contracts/utils/library/SafePct.sol
contracts/utils/library/Transfers.sol


 ## DOCUMENTATION: 

 ### flare-docs.md

Title: FAssets | Flare Developer Hub

FAssets is a trustless, over-collateralized bridge connecting non smart contract networks to Flare. It enables the creation of wrapped tokens (`FAssets`) for assets like BTC, DOGE and XRP. These tokens can participate in Flare's DeFi ecosystem or be redeemed for their original assets.

FAssets are powered by Flare's enshrined data protocols:

*   **[Flare Time Series Oracle (FTSO)](https://dev.flare.network/ftso/overview):** Provides decentralized price feeds.
*   **[Flare Data Connector (FDC)](https://dev.flare.network/fdc/overview):** Verifies offchain actions, such as transactions on other blockchains.

Each FAsset is backed by a mix of collateral, including:

1.   Stablecoin or ETH collateral.
2.   FLR (Flare's native token) or SGB (Songbird's native token) collateral.

Agents and a community-provided collateral pool ensure trustlessness through over-collateralization.

FAsset Workflow[​](https://dev.flare.network/fassets/overview#fasset-workflow "Direct link to FAsset Workflow")
---------------------------------------------------------------------------------------------------------------

Anyone on the Flare blockchain can mint FAssets, which are wrapped versions of original tokens from other blockchains, known as underlying networks. The original tokens from these chains, such as Ripple (XRPL), Dogecoin (DOGE), Bitcoin (BTC), and Litecoin (LTC), are referred to as underlying assets. For example, the FAsset version of Bitcoin is known as FBTC.

### Minting[​](https://dev.flare.network/fassets/overview#minting "Direct link to Minting")

*   A user (minter) selects an agent and pays a fee to reserve collateral.
*   The user sends the underlying asset (e.g., BTC) to the agent.
*   The FDC verifies the transaction.
*   The equivalent FAssets (e.g., FBTC) are minted as ERC-20 tokens on Flare.

### Usage[​](https://dev.flare.network/fassets/overview#usage "Direct link to Usage")

Minted FAssets can be used in DeFi applications on Flare or bridged to other chains.

### Redeeming[​](https://dev.flare.network/fassets/overview#redeeming "Direct link to Redeeming")

Users can redeem FAssets for the original underlying assets at any time.

Key Participants[​](https://dev.flare.network/fassets/overview#key-participants "Direct link to Key Participants")
------------------------------------------------------------------------------------------------------------------

### Agents[​](https://dev.flare.network/fassets/overview#agents "Direct link to Agents")

Agents manage the infrastructure and operations of the FAssets system, including:

*   Holding the underlying assets.
*   Providing collateral for minting and redemption.
*   Redeeming underlying assets for users.

Each agent is verified through governance and uses the following addresses on the native chain:

*   **Work Address:** A hot wallet for executing operations.
*   **Management Address:** A cold wallet for secure administrative actions.

Agents must comply with the **backing factor**, which ensures sufficient collateral is locked to back FAssets.

### Users[​](https://dev.flare.network/fassets/overview#users "Direct link to Users")

Users interact with the system by:

*   **Minting:** Depositing underlying assets to mint FAssets.
*   **Redeeming:** Exchanging FAssets for the original underlying assets.

Eligibility:

*   No restrictions—anyone can mint or redeem FAssets.

### Collateral Providers[​](https://dev.flare.network/fassets/overview#collateral-providers "Direct link to Collateral Providers")

Collateral providers supply native FLR tokens to an agent's collateral pool and earn a share of minting fees as long as their tokens remain locked.

### Liquidators[​](https://dev.flare.network/fassets/overview#liquidators "Direct link to Liquidators")

Liquidators maintain system health by:

*   Burning FAssets in exchange for collateral when an agent's collateral drops below the required minimum.
*   Earning rewards, including premiums on the collateral received.

Eligibility:

*   Open to all—anyone can become a liquidator.

### Challengers[​](https://dev.flare.network/fassets/overview#challengers "Direct link to Challengers")

Challengers monitor agents for illegal transactions that reduce collateral below the backing factor. They:

*   Submit proof of illegal actions to the system.
*   Earn rewards from the agent's vault upon successful challenges.

If an agent is found in violation, they enter **full liquidation**, permanently restricting them from new minting operations.

Core Vault[​](https://dev.flare.network/fassets/overview#core-vault "Direct link to Core Vault")
------------------------------------------------------------------------------------------------

The **Core Vault (CV)** is a specialized FAsset system component that enhances capital efficiency by allowing agents to store underlying assets without requiring additional collateral. Each asset type has its own dedicated Core Vault, which is managed by a multisig account on the underlying network under formal governance oversight.

### Key Features[​](https://dev.flare.network/fassets/overview#key-features "Direct link to Key Features")

*   **Collateral Efficiency:** Agents transferring assets to the CV free up collateral, allowing them to mint additional FAssets or withdraw funds.
*   **Redemption Support:** The CV ensures that underlying assets are available for redemptions, reducing reliance on individual agents.
*   **Security & Governance:** A multisig setup controls the vault, and governance can pause in case of security concerns.

### Core Vault Implementation[​](https://dev.flare.network/fassets/overview#core-vault-implementation "Direct link to Core Vault Implementation")

On networks without smart contracts (e.g., XRP Ledger), the Core Vault is a **multisig account** managed by signers authorized by Flare governance. Movements of funds require multiple signatures and follow formal agreements, not individual agent control.

### Agent vs. Core Vault Ownership[​](https://dev.flare.network/fassets/overview#agent-vs-core-vault-ownership "Direct link to Agent vs. Core Vault Ownership")

*   **Agents:** Hold and control their own underlying assets in wallets as part of collateral.
*   **Core Vault:** Holds pooled assets that no single agent owns; agents can request assets but cannot directly control the vault. This improves capital efficiency and liquidity for the system.

Title: Minting | Flare Developer Hub

Minting FAssets is the process of wrapping underlying tokens from connected blockchains into FAssets to be used on the Flare blockchain. Any user can mint FAssets.

Minting Process[​](https://dev.flare.network/fassets/minting#minting-process "Direct link to Minting Process")
--------------------------------------------------------------------------------------------------------------

This is the summary of the minting process:

### 1. Reserving Collateral[​](https://dev.flare.network/fassets/minting#1-reserving-collateral "Direct link to 1. Reserving Collateral")

The minter chooses an agent from the publicly available [agent list](https://dev.flare.network/fassets/overview#agents). The choice is based on the minting fee or the amount of free collateral, which must be enough to back the amount to be minted.

The minter sends to the Asset Manager contract a collateral reservation transaction (CRT). The CRT includes:

*   The address of the chosen agent.
*   The amount to mint, which must be a positive integer of [lots](https://dev.flare.network/fassets/minting#lots).
*   The [collateral reservation fee (CRF)](https://dev.flare.network/fassets/minting#fees) to compensate for the locked collateral.
*   The executor's address, if the minter is not the executor.
*   The executor's fee, if the minter is not the executor.

The Asset Manager contract locks the agent's collateral in the amount needed to back the whole minting until the underlying payment is proved or disproved. The collateral reservation response is an event issued by the contract, which includes:

*   The agent's address to which the minter must send funds on the underlying chain.
*   The amount to be paid on the underlying chain, which corresponds to the amount to be minted plus the agent's fee.
*   The payment reference, which is a unique 32-byte number the minter must include as a memo in the payment on the underlying chain.
*   The last underlying block and the last underlying timestamp to pay. Valid payments occur either before the last block or before the last timestamp, both inclusive.
*   The executor's address, if the minter is not the executor.
*   The executor's fee, if the minter is not the executor.

The time to pay is measured both in the underlying chain's block numbers and block times because the underlying chain might halt for a long time. In this situation, the block numbers do not increment but the block timestamps do.

### 2. Underlying Payment[​](https://dev.flare.network/fassets/minting#2-underlying-payment "Direct link to 2. Underlying Payment")

After this event is emitted, the minter must pay the full underlying amount plus the fee to the agent on the underlying chain. This payment must be completed within a specified time limit.

### 3. Payment Proof[​](https://dev.flare.network/fassets/minting#3-payment-proof "Direct link to 3. Payment Proof")

Using the [Flare Data Connector](https://dev.flare.network/fdc/overview), the minter or executor proves the payment on Flare network.

### 4. Minting Execution[​](https://dev.flare.network/fassets/minting#4-minting-execution "Direct link to 4. Minting Execution")

After the payment is proved, the minter or executor executes the minting process, which sends FAssets to the minter's account.

When minting is executed, the [minting fee](https://dev.flare.network/fassets/minting#fees) is split between the agent and the pool:

*   The percentage split is set by the agent.
*   The agent's share increases the free balance on the agent's underlying address. The free balance is the part of the balance in an agent's underlying address that the agent can withdraw. It is composed of minting fees, redemption fees, and self-closed FAssets.
*   The pool share gets minted as FAssets and credited to the collateral pool contract.

After minting is complete, the Asset Manager creates a [redemption ticket](https://dev.flare.network/fassets/minting#redemption-tickets-and-the-redemption-queue), which includes the mint amount and the name of the agent backing the minting.

Executor Role[​](https://dev.flare.network/fassets/minting#executor-role "Direct link to Executor Role")
--------------------------------------------------------------------------------------------------------

The execution of the minting process can be performed by an **executor**, an external actor such as a bot or service that monitors pending minting requests. Executors are incentivized to act quickly and correctly, but they hold no special permissions. If they fail to execute in time, the request may expire, and the minting must be restarted.

The executor:

*   Is nominated by the minter and gets paid by the minter.
*   Uses the Flare Data Connector to obtain valid payment proof.
*   Executes the minting with a valid payment proof.

Fees[​](https://dev.flare.network/fassets/minting#fees "Direct link to Fees")
-----------------------------------------------------------------------------

The following fees are paid to mint FAssets:

### Collateral Reservation Fee[​](https://dev.flare.network/fassets/minting#collateral-reservation-fee "Direct link to Collateral Reservation Fee")

The **collateral reservation fee (CRF)** is paid in native tokens by the minter at the same time the [collateral reservation](https://dev.flare.network/fassets/minting#minting-process) is made. The CRF is defined by governance as a percentage of the minted value, and the same fee applies to all agents.

The purpose of the CRF is to compensate the agent and collateral pool token (CPT) holders for the time their collateral is locked during the minting process.

*   If the minter does not pay on the underlying chain, the CRF is distributed to the agent and the pool in the same share as the minting fee.
*   If the minter successfully pays on the underlying chain, the CRF is also distributed to the agent and the pool in the same manner.

For underlying chains where proving payments takes longer, the CRF might be set higher to account for the extended lock-up time. The CRF percentage is defined by governance and may vary based on the performance of the underlying chain.

### Minting Fee[​](https://dev.flare.network/fassets/minting#minting-fee "Direct link to Minting Fee")

The **minting fee** is paid by the minter with the underlying currency as a percentage of the minted amount, and each agent can declare a different fee value. This fee is the main source of revenue for the agent and the CPT holders.

The minting fee is further divided in two shares:

This share remains in the agent's underlying account but is not marked as being in use. The agent can use this balance freely.

This share is minted as FAssets and sent to the [collateral pool](https://dev.flare.network/fassets/collateral#pool-collateral). The percentage of this share is defined by the agent and can be changed by the agent after a delay that provides time for minters to notice the change.

### Executor Fee[​](https://dev.flare.network/fassets/minting#executor-fee "Direct link to Executor Fee")

To incentivize reliable execution of minting requests, an **executor fee** may be included in the system.

The executor is the actor who submits the payment proof to the Asset Manager, finalizing the minting process.

*   The executor fee is paid by the minter when minting is executed.
*   This fee is optional and configurable within the system based on chain-specific governance parameters.
*   If set, the fee is denominated in FLR and transferred directly to the executor's address as part of the execution transaction.
*   Executors compete to be the first to execute minting and collect this fee, providing a decentralized execution layer.

This design ensures timely and reliable minting finalization without relying on a centralized party.

### Minting[​](https://dev.flare.network/fassets/minting#minting "Direct link to Minting")

The FAssets agent verifies the minter after the user completes the collateral reservation and pays the collateral reservation fee. The agent is responsible for confirming or rejecting the minter's status. If the agent does not respond within a certain timeframe, the minter has the option to cancel the reservation and receive a full refund of the collateral reservation fee.

To enable the agent to verify the minter, the collateral reservation must include the address (or multiple addresses, in the case of UTXO chains) from which the payment will be made. If multiple addresses are provided, all of them must be used for the payment.

Users must wait up to 60 seconds before they can cancel their request. If the agent accepts within this time, the user can proceed to mint by depositing the underlying assets. Therefore, it is important for the agent to respond quickly. If the agent does not respond in time, it will depend on whether the user is willing to wait; otherwise, the agent will simply miss the opportunity to mint, but there will be no loss of tokens.

When the agent rejects the minter's request or the minter decides to cancel, the minter will receive a refund of the collateral reservation fee, minus a small percentage (e.g., 5%) that is burned. This burned amount is designed to prevent abuse of the agent by stopping someone from repeatedly reserving collateral from a sanctioned address. If the burned percentage were zero, an attacker could exploit the system without any cost.

Payment Failure[​](https://dev.flare.network/fassets/minting#payment-failure "Direct link to Payment Failure")
--------------------------------------------------------------------------------------------------------------

To finalize the minting, the minter must pay the agent on the underlying chain and prove the payment was received. If the payment is not completed in the time frame defined by the underlying chain block and timestamp, the agent must prove nonpayment to release the locked collateral. After nonpayment is proved, the agent's collateral that was reserved by the [CRT](https://dev.flare.network/fassets/minting#minting-process) is released, and the agent receives the [CRF](https://dev.flare.network/fassets/minting#collateral-reservation-fee).

The [agent's registration process](https://dev.flare.network/fassets/overview#agents) verifies that the agent's underlying address does not purposefully block payments and illegally collects the CRF.

The following example shows proof of nonpayment.

Referenced payment nonexistence attestation type example.

Edge Cases[​](https://dev.flare.network/fassets/minting#edge-cases "Direct link to Edge Cases")
-----------------------------------------------------------------------------------------------

### Unresponsive minter[​](https://dev.flare.network/fassets/minting#unresponsive-minter "Direct link to Unresponsive minter")

After a successful payment, the minter might not provide the payment proof needed to complete the minting process. In this case, the agent can present the payment proof and execute minting at any time. FAssets are still transferred to the minter's account, and the agent's collateral becomes redeemable.

### Expired proof[​](https://dev.flare.network/fassets/minting#expired-proof "Direct link to Expired proof")

Proofs provided by the Flare Data Connector are available for only 24 hours, approximately. If neither the minter nor the agent presents the proof of payment or nonpayment within 24 hours, the regular minting process cannot continue, and the agent's collateral could be locked indefinitely.

In this case, the agent can still recover the collateral by buying it back with native tokens. The recovery is accomplished with the following procedure:

1.   Request the proof from the time when the deposit should have happened. The Flare Data Connector's answer will indicate that payments proofs are no longer available for that time.
2.   Provide the amount of FLR collateral equivalent to the price of the underlying assets that should have been deposited.
3.   Present the proof.

Because a successful deposit cannot be proven, the FAssets system burns the amount of collateral in native tokens provided by the agent. After the burn is complete, the rest of the agent's collateral is released, both from his vault and the collateral pool.

warning

Note that this procedure should be used only in rare cases because providing timely payment or nonpayment proofs is always more advantageous for agents.

Duration of the Minting Process[​](https://dev.flare.network/fassets/minting#duration-of-the-minting-process "Direct link to Duration of the Minting Process")
--------------------------------------------------------------------------------------------------------------------------------------------------------------

The duration of the minting process depends mainly on the speed of the underlying chain. The maximum duration of the process is the sum of:

*   A system-defined maximum time for deposit. It is either a few blocks on the underlying chain or a few minutes, whichever is longer.
*   The underlying chain's finalization time.
*   The Flare Data Connector proof time, which is approximately 3 - 5 minutes, independent of the underlying chain.

On fast chains like XRPL, the maximum total time is less than 10 minutes, while on Bitcoin it is approximately 1.5 hours. For payment failures, the agent needs to wait the maximum time, as defined above, before the nonpayment proof can be retrieved.

Minting Payment Reference[​](https://dev.flare.network/fassets/minting#minting-payment-reference "Direct link to Minting Payment Reference")
--------------------------------------------------------------------------------------------------------------------------------------------

The system generates a unique payment reference at the time of the collateral reservation request. The minter must include the payment reference in a memo field when the underlying payment transaction is made.

The payment reference ensures the payment transaction cannot be used by another entity that might claim to have made the payment on the underlying chain and receive the minted FAssets in return. Additionally, if the payment time expires before payment is done, the agent can prove that no payment with that reference was made.

A similar payment reference for the same purposes is generated for [redemptions](https://dev.flare.network/fassets/redemption).

Redemption Tickets and the Redemption Queue[​](https://dev.flare.network/fassets/minting#redemption-tickets-and-the-redemption-queue "Direct link to Redemption Tickets and the Redemption Queue")
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

For every minting operation, a redemption ticket is created. This ticket references the minted amount and the agent that is backing the minting.

The redemption tickets are ordered in a queue that determines the next agent to be [redeemed](https://dev.flare.network/fassets/redemption) against according to the first in, first out method (FIFO). In other words, the first redemption ticket created will be the first redemption ticket processed. The FIFO queue impartially ensures that all agents have the opportunity to fulfill the duties of their role.

The following example shows how the redemption queue works.

Redemption queue example.

Lots[​](https://dev.flare.network/fassets/minting#lots "Direct link to Lots")
-----------------------------------------------------------------------------

Every minting and redemption must be made in a positive integer of lots. Lots serve the following purposes:

*   They prevent underlying transaction fees from exceeding minting or redemption fees.
*   They restrict large numbers of very small redemption tickets from being submitted, which would increase gas costs.

Therefore, the amount of tokens in a lot (the _lot size_) varies for each underlying chain. For example, on the XRPL chain, a lot can be as small as 10 XRP because transaction fees are low. On the other hand, on the Bitcoin chain, lots might need to be as big as 0.25 BTC or more because transactions are far more expensive.

Over time, the lot size can be updated to reflect price fluctuations of the underlying asset. Only a governance call can update the lot size, and it can be updated only by a limited amount per day.

Dust[​](https://dev.flare.network/fassets/minting#dust "Direct link to Dust")
-----------------------------------------------------------------------------

Some processes generate a fractional number of lots:

*   On minting, part of the minting fee is minted as the FAsset fee to the collateral pool. This value is usually less than 1 lot.
*   When the lot size is changed, redemptions close only a positive integer of lots of each redemption ticket, which leaves the remainder unredeemed.

These amounts, known as dust, cannot be redeemed directly because redemption requires a positive integer of lots.

In such cases, the generated dust is not included in any redemption ticket. Instead, each agent's dust is accumulated until the dust amounts to a whole lot. When that happens, another redemption ticket is automatically created.

Therefore, the dust can be recovered or destroyed in the following ways:

*   If the dust exceeds 1 lot during minting, the part that is a whole multiple of a lot is automatically added to the created redemption ticket.

*   If an agent does not mint any FAssets for a while but the lot size changes and several redemptions occur, enough dust might accumulate to more than 1 lot.

In this case, the part that is a whole multiple of a lot can be converted to a redemption ticket by request. To prevent an inactive agent making FAssets less fungible, this request can be made by any address.

*   Self-closing can work with fractional lots, so it can be used to remove dust.

*   Liquidation can work with fractional lots too, so it can also be used to remove dust.

Self-Minting[​](https://dev.flare.network/fassets/minting#self-minting "Direct link to Self-Minting")
-----------------------------------------------------------------------------------------------------

Agents can also act as minters and mint FAssets from their own vaults. This process is called self-minting and is simpler than regular minting because neither the CRT nor the agent's fee are necessary.

When an agent self-mints FAssets:

*   The agent still needs to pay the amount to mint on the underlying chain and execute the minting.
*   The self-minting operation also adds a [ticket to the redemption queue](https://dev.flare.network/fassets/minting#redemption-tickets-and-the-redemption-queue), alongside tickets added by mints done by other users. All tickets are processed by the FIFO queue.
*   Only the [pool's share of the fee](https://dev.flare.network/fassets/minting#fees) must be paid.

Because self-minting is done without a collateral reservation request, in some cases, a change between the underlying deposit and the execution, such as another collateral reservation, price change which reduces the amount of free [lots](https://dev.flare.network/fassets/minting#lots), or lot-size change, might prohibit the intended number of lots to be minted. If one of these changes occurs, the agent can self-mint a smaller number of lots, even 0 lots, and the remainder of the deposited underlying assets is added to the free underlying balance.

Additionally, when agents create a vault, they can choose not to make it public, so the vault can only be used to self-mint.


Title: Redemption | Flare Developer Hub

Any holder of FAssets can redeem their FAssets for the underlying original asset. To do so, these holders, known as redeemers, send FAssets to the Asset Manager smart contract, and the redeemed amount is paid with the underlying asset from an agent's address.

Redemption Process[​](https://dev.flare.network/fassets/redemption#redemption-process "Direct link to Redemption Process")
--------------------------------------------------------------------------------------------------------------------------

This is the summary of the redemption process:

1.   The redeemer starts the redemption for a positive integer of lots by issuing a request to the Asset Manager smart contract.

The FAssets system chooses one or more redemption tickets from the front of the [FIFO redemption queue](https://dev.flare.network/fassets/minting#redemption-tickets-and-the-redemption-queue). The number of chosen redemption tickets is capped to avoid high gas consumption. If the redemption amount requires too many tickets, only a partial redemption is done.

2.   The system burns FAssets from the redeemer's account in the amount of the total of the selected redemption tickets. If the redeemer's account does not contain enough FAssets, the redemption fails immediately.

3.   Each chosen ticket belongs to an agent. For every agent participating in the redemption, the system issues an event with the following redemption payment information:

    *   Redeemer's underlying address.

Agents can use the Flare Data Connector to ensure the validity of this address. Otherwise, malicious redeemers could provide an address that systematically blocks payments and exploit the redeeming process to their advantage.

    *   Amount to pay minus the fee that was already subtracted.

    *   [A payment reference](https://dev.flare.network/fassets/minting#minting-payment-reference). This payment reference is different for each agent and each redemption.

    *   The last underlying block and the last underlying timestamp to complete the payment.

4.   Every agent pays the redeemer on the underlying chain and includes the payment reference in the memo field of the payment transaction.

Agents can pay the redemption from their own address they control on the underlying chain. It does not need to be the same address where they receive minting payments.

5.   After the payment is finalized, the agent uses the [FDC](https://dev.flare.network/fdc/overview) to prove the payment and obtain a payment proof.

6.   The agent (or, in Core Vault and certain flows, the **executor**) presents the payment proof to the FAssets system, which issues a **redemption ticket**.

The executor role is responsible for ensuring that the system can finalize redemptions even if the agent is unresponsive or if the flow is managed by a central entity (such as the Core Vault executor).

The redemption ticket is required to:

    *   Prove the payment has occurred.
    *   Trigger burning of the FAssets.
    *   Release the corresponding agent's vault collateral and pool collateral.
    *   Ensure the system tracks the underlying chain balances correctly.

After the collateral is released, it can either back the minting of more FAssets or be withdrawn.

Redemption-Payment Failure[​](https://dev.flare.network/fassets/redemption#redemption-payment-failure "Direct link to Redemption-Payment Failure")
--------------------------------------------------------------------------------------------------------------------------------------------------

Agents have a limited time to pay the redeemer on the underlying chain. The amount of time is defined by the last block and the last timestamp on the underlying chain. If the payment is not made in time, the redeemer has to prove nonpayment to be compensated. After the redeemer presents the nonpayment proof, he is paid with the agent's collateral plus a _redemption default premium_. The premium is intended to encourage the agent to complete redemptions by paying with the underlying asset instead of collateral.

If a payment fails and the failed transaction is recorded on the underlying chain, the agent must submit a proof of failed payment. In this way, the gas costs of the failed transaction can be accounted for by the FAssets system. If the transaction was not recorded, then no gas was spent and reporting is not necessary.

If the agent does not report the failed payment in time, anyone (including the executor) can report the failed payment and receive a reward from the agent's vault.

info

When payment fails because of the redeemer, the agent can obtain a proof of the failed payment from the Flare Data Connector and present it to the FAssets system. The agent's obligation is then fulfilled, and he can keep both the collateral and the underlying.

Two different proofs can be used:

*   Proof of invalid address, due to a wrong syntax or checksum, for example.

*   Proof of blocked payment: Even if the address is valid, it might contain a contract that blocks the payment. This can only happen on underlying networks supporting smart contracts.

The agent must still try to pay and, if the payment is blocked, the agent can request this proof from the Flare Data Connector and present it to the FAssets system.

During step 4 above, if any agent does not pay on the underlying chain, the redeemer completes the following procedure separately for each nonpaying agent:

1.   The redeemer obtains a proof of nonpayment from the Flare Data Connector.
2.   The redeemer presents the nonpayment proofs to the FAssets system, which triggers a redemption failure.
3.   The redeemer is paid with collateral, according to the current price plus a premium.
4.   FAssets are overcollateralized, so, even after paying the redeemer with a premium, a remainder is released. This remainder is derived by the [system-wide collateral ratio settings](https://dev.flare.network/fassets/collateral#system-wide-thresholds) specified by governance.
5.   The underlying assets backing the redeemed FAssets are marked as free and can be withdrawn by the agent later.

Edge Cases[​](https://dev.flare.network/fassets/redemption#edge-cases "Direct link to Edge Cases")
--------------------------------------------------------------------------------------------------

### Unresponsive redeemer[​](https://dev.flare.network/fassets/redemption#unresponsive-redeemer "Direct link to Unresponsive redeemer")

After a redemption nonpayment, the redeemer might not report the failure for some reason. In this case, the agent or the executor can present a nonpayment proof, and the redeemer receives collateral plus a premium. After this operation, the underlying backing collateral and the remaining local collateral are released.

### Unresponsive agent[​](https://dev.flare.network/fassets/redemption#unresponsive-agent "Direct link to Unresponsive agent")

After a successful payment, the agent might not present the payment proof (redemption ticket).

Because the agent has already paid, the redeemer is not affected. However, the system still requires the payment proof to correctly track the agent's balance on the underlying chain. After enough time for the agent to present the proof has elapsed, anyone, including the executor, can present the payment proof and receive collateral from the agent's vault as a reward.

### Expired proof[​](https://dev.flare.network/fassets/redemption#expired-proof "Direct link to Expired proof")

Proofs provided by the Flare Data Connector are available for only 24 hours, approximately. If neither the redeemer, the agent, nor the executor presents the proof of payment or nonpayment within 24 hours, the regular redeeming process cannot continue, and the agent's collateral could be locked indefinitely.

The procedure to recover this collateral is the same as the procedure in the minting case.

Redemption Fee[​](https://dev.flare.network/fassets/redemption#redemption-fee "Direct link to Redemption Fee")
--------------------------------------------------------------------------------------------------------------

The redemption fee is the amount of the underlying asset that the agent can keep for doing the redemption. This fee is meant only to cover the agent's transaction fee on the underlying chain, so it is not shared with the collateral pool. The fee percentage is defined by governance, is the same for all agents, and is typically smaller than the minting fee.

Governance calculates the percentage so that the fee to redeem 1 lot pays for a typical transaction fee on the underlying chain. Therefore, when larger amounts on a single address are redeemed, the agent accrues some extra fees because the underlying fee for small and large transactions is the same. However, when underlying fees are very high, the agent might still lose funds when a redemption for a small amount, such as 1 lot, is made. If this situation occurs frequently, governance will increase the redemption-fee percentage.

Self-redemption[​](https://dev.flare.network/fassets/redemption#self-redemption "Direct link to Self-redemption")
-----------------------------------------------------------------------------------------------------------------

Agents can also act as users and redeem FAssets from their own vaults. This process is called self-redemption or self-closing, and it is simplified because payment on the underlying chain is not required.

As shown in the following process, agents can self-redeem for any reason, including to stop liquidations because it reduces the amount of FAssets the agent is backing.

1.   An agent sends FAssets to their account.
2.   FAssets are burned.
3.   The collateral that was backing those assets is released.
4.   The underlying collateral is released and can be withdrawn from the underlying address later.

The self-redeemed amount is not limited to a positive integer of lots and can be less than 1 lot, which makes self-closing ideal for redeeming an agent's dust.

Title: Collateral | Flare Developer Hub

FAssets collateral is locked in contracts that ensure the minted FAssets can always be redeemed for the underlying assets they represent or compensated by collateral. Along with Flare's native token, FLR, any governance approved ERC-20 token on the Flare blockchain can be used as collateral.

FAssets collateral ensures the security and redemption of minted FAssets by locking collateral in smart contracts. This guarantees that FAssets can either be redeemed for their underlying assets or compensated by collateral. Collateral can include Flare's native token (FLR) and any governance-approved ERC-20 tokens on the Flare blockchain.

Collateral Types[​](https://dev.flare.network/fassets/collateral#collateral-types "Direct link to Collateral Types")
--------------------------------------------------------------------------------------------------------------------

Two primary types of collateral secure FAssets: **Vault Collateral** and **Pool Collateral**.

Vault collateral is provided exclusively by agents and ensures they perform their duties. Pool collateral is provided by agents and FLR holders who choose to contribute to the pool. It is a safeguard when a sudden drop in the price of the vault collateral makes it insufficient to back the underlying assets.

### Vault Collateral[​](https://dev.flare.network/fassets/collateral#vault-collateral "Direct link to Vault Collateral")

Vault collateral consists of the types of collateral chosen by agents to store in their vault. Flare governance approves the valid types, which are generally stablecoins, such as USDC, USDT, or other highly liquid tokens on the Flare network.

Agents choose one of the types defined by FAssets governance and use it as collateral in their vaults. Agents cannot switch to a different type after a vault is created, but they can create any number of vaults, with different types.

Each collateral type defines an ERC-20 token to use as collateral, a series of [collateral ratios](https://dev.flare.network/fassets/collateral#collateral-ratio), and information to retrieve the asset's price from the FTSO system. Governance reserves the right to add new types or deprecate existing types. If governance deprecates a type, agents must switch to a supported type.

Each vault is associated with a single, unique address on the underlying chain called the agent's underlying address. It receives underlying assets when they are minted into FAssets and sends underlying assets to the redeemer's address when they are redeemed.

When an agent creates a vault, the underlying address is checked for validity using the Flare Data Connector. Otherwise, malicious agents could provide an address that systematically blocks payments and exploit the [minting process](https://dev.flare.network/fassets/minting) to their advantage.

### Pool Collateral[​](https://dev.flare.network/fassets/collateral#pool-collateral "Direct link to Pool Collateral")

When the price of the vault collateral changes in such a way that the vault collateral cannot fully back all the minted FAssets, a [liquidation](https://dev.flare.network/fassets/liquidation) mechanism ensures enough FAssets are burned to restore balance. The pool collateral provides an additional source of backing for situations when the price fluctuates too rapidly for liquidations to correct the imbalance.

Pool collateral is always native FLR tokens or SGB tokens on the Songbird network and can be used as an additional source of collateral for [liquidations](https://dev.flare.network/fassets/liquidation) and [failed redemptions](https://dev.flare.network/fassets/redemption#redemption-payment-failure).

Anyone can participate in the FAssets system by providing native tokens to this pool. In return, providers receive **collateral pool tokens** (CPTs) as proof of the share of native tokens they provided to a specific pool from a specific agent. CPTs are ERC-20 tokens specific to both an agent and a pool.

Providers can redeem their CPTs for FLR, or even transfer or trade them, after a governance-defined time period has elapsed since they entered the pool. This **time lock** is necessary to reduce sandwiching attacks.

Additionally, CPT holders are entitled to a share of any fee the agent earns from minting FAssets using this pool as explained in the next section.

CPT conversion formulae and examples.

Collateral Ratio[​](https://dev.flare.network/fassets/collateral#collateral-ratio "Direct link to Collateral Ratio")
--------------------------------------------------------------------------------------------------------------------

The collateral ratio (CR) is the ratio between the value of all the tokens used as collateral and the total value of the underlying assets held by an agent at any given time. The agent's vault and the collateral pool each has its own unique collateral ratio, which is constantly changing as the value of the underlying assets and the collateral change. These values are obtained using the [FTSO](https://dev.flare.network/ftso/overview).

The following example shows vault and pool CR:

Vault and pool CR

Assume an amount of FAssets currently valued at $1000 USD, backed by $1500 worth of USDC in vault collateral and $2000 worth of FLR in pool collateral.

The resulting vault CR is: $1500$1000=1.5\frac{\text{\$1500}}{\text{\$1000}} = 1.5

The resulting pool CR is: $2000$1000=2\frac{\text{\$2000}}{\text{\$1000}} = 2

Several thresholds are defined for the collateral ratio, and they are used at different times during the FAsset operations. Some are set by the system, and others are set by the agent:

### System-Wide Thresholds[​](https://dev.flare.network/fassets/collateral#system-wide-thresholds "Direct link to System-Wide Thresholds")

The following thresholds are set by the FAssets system's governance and are the same for all agents.

#### Minimal CR[​](https://dev.flare.network/fassets/collateral#minimal-cr "Direct link to Minimal CR")

The lowest collateral ratio the agent vault and the collateral pool must maintain so that enough collateral exists to insure the minted FAssets and to compensate for redemption payments that fail. The minimal CR can be different for each type of collateral.

If an agent's CR remains below the minimal CR for longer than a governance-set amount of time, [liquidations](https://dev.flare.network/fassets/liquidation) can start.

#### Liquidation CR[​](https://dev.flare.network/fassets/collateral#liquidation-cr "Direct link to Liquidation CR")

**Liquidation CR**: An agent's position is unhealthy when the agent's vault CR or pool CR fall below their minimal CR. However, as long as the CR remains above liquidation CR, the CR can briefly fall below the minimal CR.

During this time, the agent can either deposit more collateral or self-close some backed FAssets to improve the position.

However, if the CR falls below the liquidation CR, liquidations can start immediately.

The value of each liquidation CR is approximately 10% less than the minimal CR.

Example liquidation CR

Assume the **minimal CR** is 1.4 and the **liquidation CR** is 1.3.

If the agent's vault CR drops below 1.3, the agent's position can be liquidated immediately. If the agent's vault CR drops below 1.4 but not below 1.3, the agent has some time to amend the position before it can be liquidated.

Adjusted for the collateral pool's minimal CR, the same example applies to the collateral pool.

#### Safety CR[​](https://dev.flare.network/fassets/collateral#safety-cr "Direct link to Safety CR")

If one or both of the collateral types fall below liquidation CR or below the minimum CR for a longer period of time, liquidation occurs. When the offending collateral reaches a healthy CR again, the liquidation stops. To prevent the agent from immediately reverting into liquidation after a small price change, the CR must reach the safety CR before it can start operating normally again and liquidation stops.

Each of the collateral types, the agent's vault and the collateral pool, has its own unique safety CR.

### Agent Thresholds[​](https://dev.flare.network/fassets/collateral#agent-thresholds "Direct link to Agent Thresholds")

The following thresholds are set by each agent according to their own preferences.

#### Minting CR[​](https://dev.flare.network/fassets/collateral#minting-cr "Direct link to Minting CR")

For each mint done by an agent, the maximum amount allowed to be minted is calculated so that the CR for the agent's vault and the CR for the agent's collateral pool after the mint remain higher than the minting CR for each collateral type. To reduce the threat of liquidation, agents should set the minting CR well above the minimal CR to accommodate price fluctuations that might occur before the CR falls below the minimal CR after the mint and minting is no longer possible.

#### Exit CR[​](https://dev.flare.network/fassets/collateral#exit-cr "Direct link to Exit CR")

After a user redeems CPTs, the pool CR must be more than the exit CR. If the pool CR is already below the exit CR, redemption cannot occur. The exit CR is for the collateral pool only.

#### Top-up CR[​](https://dev.flare.network/fassets/collateral#top-up-cr "Direct link to Top-up CR")

To incentivize healthy collateral pools, if the pool CR falls below the top-up CR, anyone can add collateral to the pool and receive [CPTs](https://dev.flare.network/fassets/collateral#pool-collateral) at a reduced price. This [top-up mechanism](https://dev.flare.network/fassets/collateral#top-up) decreases the likelihood of liquidations because of a low amount of pool collateral.

Minting Fees and Debt[​](https://dev.flare.network/fassets/collateral#minting-fees-and-debt "Direct link to Minting Fees and Debt")
-----------------------------------------------------------------------------------------------------------------------------------

As part of the minting process, users pay a [minting fee](https://dev.flare.network/fassets/minting#fees) on the underlying chain. The agent's share of this fee remains on the underlying chain, whereas the pool's share triggers the minting of an equivalent amount of FAssets on the Flare network.

These FAssets coming from the minting fee are added to the collateral pool, where they are shared between collateral providers in proportion to the amount of CPTs that providers have. At any time, providers can claim their due share of the fees in the pool. When providers exit the collateral pool by redeeming their CPTs, any remaining unclaimed fee is automatically transferred to them.

Providers are naturally only entitled to the minting fees accrued after they entered the pool. Therefore, providers entering a pool with preexisting fees are assigned a **fee debt**. The amount of fees a provider can actually withdraw from the pool is calculated by first subtracting their debt from the total amount of fees in the pool. In this way, the amount of fees that a provider can withdraw upon entering a pool is exactly zero.

A provider's fee debt:

| Increases when the provider | Decreases when the provider |
| --- | --- |
| Enters a pool which already has fees in it. | Exits the pool, partially or completely. |
| Withdraws FAsset fees. | Deposits FAssets, paying off part of the debt. |

It is worth noting that:

*   When a provider withdraws fees, their debt increases by the same amount.
*   Since CPTs are ERC-20 tokens, a secondary market for them is expected to develop. If CPTs become more valuable than the FAsset fees they represent, returning the FAssets and paying off part of their fee debt might be more lucrative for providers.

Fee entitlement formulae and examples.

Transferable and Locked CPTs[​](https://dev.flare.network/fassets/collateral#transferable-and-locked-cpts "Direct link to Transferable and Locked CPTs")
--------------------------------------------------------------------------------------------------------------------------------------------------------

CPTs can always be **redeemed** by exiting the pool, but only the portion above the fee debt can be **transferred** to another account; therefore, CPTs held by providers are divided into two types.

### Transferable[​](https://dev.flare.network/fassets/collateral#transferable "Direct link to Transferable")

Tokens whose time lock has expired and are also free of fee debt. These tokens are fungible, and they can be transferred or traded just like any other ERC-20 token.

### Locked[​](https://dev.flare.network/fassets/collateral#locked "Direct link to Locked")

The CPTs serve only as proof of ownership of some of the collateral in the pool, and they cannot be transferred nor traded.

Locked CPTs are one of the following types:

*   **Time-locked**: Tokens whose time lock has not expired must wait to become transferable or redeemable.

*   **Debt-locked**: Tokens corresponding to an amount of fees below the provider's fee debt cannot be transferred because they would need to carry the debt with them. However, they can be [redeemed](https://dev.flare.network/fassets/collateral#cpt-redemption).

As new fees arrive in the pool, some previously debt-locked tokens become transferable.

These CPTs can also become transferable by adding FAssets to the pool, which settles, either partially or completely, the fee debt.

CPT transferability formulae and examples.

CPT Redemption[​](https://dev.flare.network/fassets/collateral#cpt-redemption "Direct link to CPT Redemption")
--------------------------------------------------------------------------------------------------------------

When collateral providers exit the pool by redeeming their CPTs, the FAssets system burns them and returns the appropriate share of the collateral plus the share of [FAsset-minting fees minus any FAsset-fee debt](https://dev.flare.network/fassets/collateral#minting-fees-and-debt).

Providers also have the option to exit the pool partially, by redeeming only some of their CPTs. In this case, they can choose one of the following options to manage their due FAsset fees: withdraw the fees, reduce the fee debt, or both, keeping the current fee-to-debt-ratio.

However, providers can exit, either fully or partially, only when the [collateral ratio CR](https://dev.flare.network/fassets/collateral#collateral-ratio) is high enough. After they exit, the **CR** must be higher than the **exit CR** to prevent their exit from reducing the **CR** to a dangerous level.

Therefore, exits are impossible when the **CR** is below the **exit CR**. In this case, if providers have enough FAssets, they can exit by **self-closing**, which burns enough of their FAssets, plus their fees, to release their collateral.

Providers are mainly compensated in underlying assets for the burned FAssets, depending on the [number of lots](https://dev.flare.network/fassets/minting#lots) of FAssets that need to be redeemed:

*   If more than 1 lot needs to be redeemed, the value of the burned FAssets is redeemed through the standard [redemption process](https://dev.flare.network/fassets/redemption).

*   If less than 1 lot needs to be redeemed, the agent buys the underlying funds from the user using vault collateral, at the price reported by the [FTSO](https://dev.flare.network/ftso/overview) minus a percentage defined by the agent. This purchase by the agent occurs because fees on underlying chains can be expensive, which makes redemption of small quantities too expensive for the agent.

Providers can always request this option instead of receiving underlying tokens. Also, if enough vault collateral is not available, pool collateral is used instead.

warning

In the case where the agent does not redeem in the underlying asset, the FAssets system pays the provider in collateral from the agent's vault because the pool collateral backing the redeemed FAssets is already withdrawn.

When this type of redemption occurs, users might receive less collateral than they would have received if they had made a normal redemption.

Agent Stake[​](https://dev.flare.network/fassets/collateral#agent-stake "Direct link to Agent Stake")
-----------------------------------------------------------------------------------------------------

Agents must have a stake in their collateral pools, which means they must hold the amount of CPTs proportional, by a system-defined constant, to the backed amount of FAssets. The maximum amount of minting is limited by the amount of collateral pool tokens held by the agents. The agents' tokens are locked, which means they cannot be redeemed or transferred, while agents back these FAssets.

When the agent's portion of the collateral pool is below the threshold, new mintings are not allowed. However, this situation does not trigger a liquidation because only the total pool stake matters when collateral needs to be redeemed or a liquidation payment needs to be made.

If an agent's actions force a payment to be made from the collateral pool, the agent's CPTs, valued by the paid native tokens and recalculated by the collateral-pool-price formula, are burned. These actions can cause the agent's CPTs to be burned:

*   When a redemption payment fails, when enough vault collateral to compensate the redeemer is not available, or when the system is set to automatically pay for redemption failures from the collateral pool.
*   Liquidation because the CR of the vault collateral is too low.
*   Full liquidation because of an [agent infraction](https://dev.flare.network/fassets/redemption#redemption-payment-failure) during a transfer on an underlying chain.

Top-up[​](https://dev.flare.network/fassets/collateral#top-up "Direct link to Top-up")
--------------------------------------------------------------------------------------

To reduce the likelihood of liquidations because the pool collateral is too low, the pool can be topped up at a reduced price when the **CR** is above the **top-up CR**. A top-up mechanism for vault collateral is not available. To prevent liquidation, agents can add vault collateral any time.


Title: Core Vault | Flare Developer Hub

Overview[​](https://dev.flare.network/fassets/core-vault#overview "Direct link to Overview")
--------------------------------------------------------------------------------------------

The **Core Vault (CV)** is a specialized FAsset system vault that operates on the underlying network. It addresses a critical challenge in cross-chain systems: **protecting user funds from malicious agents while maintaining system scalability.**

### Why the Core Vault?[​](https://dev.flare.network/fassets/core-vault#why-the-core-vault "Direct link to Why the Core Vault?")

*   In **FAssets v1**, every agent had to keep the underlying asset (e.g., XRP) in their own wallet.
*   To prevent theft, agents needed to be heavily overcollateralized - they had to lock up more value than they could ever profit from stealing.
*   This worked, but it limited minting capacity and made the system capital-inefficient.

The **Core Vault** changes this model:

*   Instead of holding XRP in their own wallets, **agents can transfer underlying assets into a shared, insured vault**.
*   The vault is **multisig-controlled** and governed by Flare, so agents cannot unilaterally take the funds.
*   Because theft is structurally prevented, agents no longer need extreme levels of overcollateralization.

The Core Vault prevents agents from running away with XRP while simultaneously lowering collateral requirements and improving system scalability.

### Key Properties[​](https://dev.flare.network/fassets/core-vault#key-properties "Direct link to Key Properties")

*   **Secure by design**: Agents cannot withdraw underlying assets directly.
*   **Capital efficient**: Agents can mint more FAssets with less collateral.
*   **System-wide liquidity**: Assets in the CV form a shared pool available to all agents.
*   **Governance oversight**: Multisig with pause controls and time-bounded fund release.

Introduced in **FAssets v1.1**, the Core Vault enables the system to expand its minting capacity while maintaining the same strong guarantees of safety for users.

Key Features[​](https://dev.flare.network/fassets/core-vault#key-features "Direct link to Key Features")
--------------------------------------------------------------------------------------------------------

### Transfer Capacity[​](https://dev.flare.network/fassets/core-vault#transfer-capacity "Direct link to Transfer Capacity")

To ensure redemption liquidity, a parameter enforces that after transferring assets to the Core Vault, **an agent must still maintain a minimum portion of minting capacity outside the CV**. This prevents agents from moving everything into the vault and leaving the system unbalanced.

### Security Design[​](https://dev.flare.network/fassets/core-vault#security-design "Direct link to Security Design")

*   Each supported asset (XRP, BTC, DOGE, etc.) has its **own dedicated Core Vault**.
*   Each CV is a **multisig account on the underlying chain**, operated under a formal governance agreement with Flare.
*   Funds in the CV **no longer belong to a single agent** - they are pooled, with withdrawals only allowed under system rules.
*   Governance can pause deposits or withdrawals if suspicious activity is detected.

Core Vault Implementation[​](https://dev.flare.network/fassets/core-vault#core-vault-implementation "Direct link to Core Vault Implementation")
-----------------------------------------------------------------------------------------------------------------------------------------------

### Fund Movement on Underlying Networks[​](https://dev.flare.network/fassets/core-vault#fund-movement-on-underlying-networks "Direct link to Fund Movement on Underlying Networks")

The Core Vault operates differently depending on the underlying network:

**For XRP Ledger (XRP CV):**

*   The CV is implemented as a **multisig account** on the XRP Ledger.
*   Only the authorized **multisig signers** can move XRP from the vault.
*   These signers are authorized by Flare governance and operate under formal agreements.
*   All outgoing transactions require multiple signatures from the authorized signers.

### Agent Ownership vs. Core Vault Ownership[​](https://dev.flare.network/fassets/core-vault#agent-ownership-vs-core-vault-ownership "Direct link to Agent Ownership vs. Core Vault Ownership")

There's an important distinction between agent ownership and CV ownership:

#### Agent Ownership (Standard FAssets)[​](https://dev.flare.network/fassets/core-vault#agent-ownership-standard-fassets "Direct link to Agent Ownership (Standard FAssets)")

*   Agents hold underlying assets in their own wallets.
*   These assets belong to the specific agent and are part of their collateral.
*   Agents have direct control over these assets.

#### Core Vault Ownership[​](https://dev.flare.network/fassets/core-vault#core-vault-ownership "Direct link to Core Vault Ownership")

*   Assets transferred to the CV become part of a shared pool.
*   These assets **do not belong to any specific agent** once in the CV.
*   The CV is a system-level reserve that any agent can request assets from.
*   This allows for better capital efficiency and system-wide liquidity.

### Agent Collateral Requirements[​](https://dev.flare.network/fassets/core-vault#agent-collateral-requirements "Direct link to Agent Collateral Requirements")

Agents are required to provide collateral in the form of:

*   **Flare's native token (FLR or SGB)**
*   **USDC/USDT (stablecoins)**

When agents transfer underlying assets to the CV, they can reduce their collateral requirements while maintaining their minting capacity. The CV effectively acts as an **insurance-backed reserve** that boosts efficiency without weakening security.

Operational Workflow[​](https://dev.flare.network/fassets/core-vault#operational-workflow "Direct link to Operational Workflow")
--------------------------------------------------------------------------------------------------------------------------------

### Transferring to Core Vault[​](https://dev.flare.network/fassets/core-vault#transferring-to-core-vault "Direct link to Transferring to Core Vault")

1.   **Agent Transfers Assets**

The agent announces a transfer and sends the underlying asset (e.g., XRP) to the Core Vault (CV) address with a valid payment reference.

2.   **Proof of Payment Submitted**

Anyone (including the agent) submits the proof of the transfer to the FAsset system.

3.   **Verification and Redemption**

After verifying the payment, the system releases the agent's collateral.

info

Transfer requests do not expire. Agents must either complete the transfer or cancel and re-queue it. Inaction leaves collateral locked indefinitely.

### Redemption from Core Vault[​](https://dev.flare.network/fassets/core-vault#redemption-from-core-vault "Direct link to Redemption from Core Vault")

There are two methods to retrieve assets from the CV:

#### Request for Return (Agents Only)[​](https://dev.flare.network/fassets/core-vault#request-for-return-agents-only "Direct link to Request for Return (Agents Only)")

An agent can request assets from the CV through a special minting process, which creates a collateral reservation. Once the request is made, CV operators execute the transfer and submit proof of payment to the asset manager. There is no time limit for the CV to honor the request, but governance ensures timely execution.

#### Direct Redemption (Users)[​](https://dev.flare.network/fassets/core-vault#direct-redemption-users "Direct link to Direct Redemption (Users)")

Approved users can burn FXRP and receive underlying XRP from the CV. This requires KYC approval and a minimum redemption threshold. It is typically processed once per day and has a lower priority than agent return requests. It is helpful for large, less time-sensitive redemptions.

XRP Core Vault Design[​](https://dev.flare.network/fassets/core-vault#xrp-core-vault-design "Direct link to XRP Core Vault Design")
-----------------------------------------------------------------------------------------------------------------------------------

The XRP Core Vault is a multisig account on the XRP Ledger, backed by an audited Flare smart contract that emits transaction instructions for execution.

### Transactions in XRP Core Vault[​](https://dev.flare.network/fassets/core-vault#transactions-in-xrp-core-vault "Direct link to Transactions in XRP Core Vault")

The XRP Core Vault (CV) uses two types of transactions:

*   `Payment`[Transactions](https://xrpl.org/docs/references/protocol/transactions/types/payment): Transfers XRP back to agents upon redemption.
*   `EscrowCreate`[Transactions](https://xrpl.org/docs/references/protocol/transactions/types/escrowcreate): Time-locks XRP to control fund releases and minimize spending risk.

info

The vault is operated manually. Multisig signers validate all outgoing transactions against a pre-approved rule set and sign them only during designated daily windows.

### Daily Security Routine[​](https://dev.flare.network/fassets/core-vault#daily-security-routine "Direct link to Daily Security Routine")

1.   **Escrow Expiry Adds Funds**

An expired escrow (size L) adds XRP to the vault's available pool.

2.   **Withdrawals & Payouts**

Users request withdrawals; multisig operators validate Flare's off-chain instructions and process payouts.

3.   **Re-escrow Excess & Maintain Reserve**

Remaining funds are escrowed in batches (size L), while a minimum reserve (M) is kept in the vault.

Three escrows are created if `remaining_funds = M + 3L`, and M stays in the wallet.

### Security Model[​](https://dev.flare.network/fassets/core-vault#security-model "Direct link to Security Model")

The XRP Core Vault features enhanced security measures for managing daily liquidity, including escrow time-locking and a minimum reserve in multisig setups. It has an emergency pause function and "Red Alert Mode" for urgent threats.

In case of a security issue, all signing is halted pending governance review. Escrow accounts may be unlocked to prevent asset loss, with governance overseeing the restoration of operations. Only one escrow can be unlocked per day, minimizing potential damage.

Summary[​](https://dev.flare.network/fassets/core-vault#summary "Direct link to Summary")
-----------------------------------------------------------------------------------------

The **Core Vault** fundamentally strengthens FAssets:

*   **Prevents loss of underlying assets** by removing agent control over pooled funds.
*   **Lowers collateral burdens**, improving capital efficiency.
*   **Expands minting capacity**, making the system more scalable.
*   **Adds layered security**, combining multisig, escrow, and governance controls.

It is the key to making cross-chain assets on Flare both **secure** and **efficient**.


Title: Collateral | Flare Developer Hub

FAssets collateral is locked in contracts that ensure the minted FAssets can always be redeemed for the underlying assets they represent or compensated by collateral. Along with Flare's native token, FLR, any governance approved ERC-20 token on the Flare blockchain can be used as collateral.

FAssets collateral ensures the security and redemption of minted FAssets by locking collateral in smart contracts. This guarantees that FAssets can either be redeemed for their underlying assets or compensated by collateral. Collateral can include Flare's native token (FLR) and any governance-approved ERC-20 tokens on the Flare blockchain.

Collateral Types[​](https://dev.flare.network/fassets/collateral#collateral-types "Direct link to Collateral Types")
--------------------------------------------------------------------------------------------------------------------

Two primary types of collateral secure FAssets: **Vault Collateral** and **Pool Collateral**.

Vault collateral is provided exclusively by agents and ensures they perform their duties. Pool collateral is provided by agents and FLR holders who choose to contribute to the pool. It is a safeguard when a sudden drop in the price of the vault collateral makes it insufficient to back the underlying assets.

### Vault Collateral[​](https://dev.flare.network/fassets/collateral#vault-collateral "Direct link to Vault Collateral")

Vault collateral consists of the types of collateral chosen by agents to store in their vault. Flare governance approves the valid types, which are generally stablecoins, such as USDC, USDT, or other highly liquid tokens on the Flare network.

Agents choose one of the types defined by FAssets governance and use it as collateral in their vaults. Agents cannot switch to a different type after a vault is created, but they can create any number of vaults, with different types.

Each collateral type defines an ERC-20 token to use as collateral, a series of [collateral ratios](https://dev.flare.network/fassets/collateral#collateral-ratio), and information to retrieve the asset's price from the FTSO system. Governance reserves the right to add new types or deprecate existing types. If governance deprecates a type, agents must switch to a supported type.

Each vault is associated with a single, unique address on the underlying chain called the agent's underlying address. It receives underlying assets when they are minted into FAssets and sends underlying assets to the redeemer's address when they are redeemed.

When an agent creates a vault, the underlying address is checked for validity using the Flare Data Connector. Otherwise, malicious agents could provide an address that systematically blocks payments and exploit the [minting process](https://dev.flare.network/fassets/minting) to their advantage.

### Pool Collateral[​](https://dev.flare.network/fassets/collateral#pool-collateral "Direct link to Pool Collateral")

When the price of the vault collateral changes in such a way that the vault collateral cannot fully back all the minted FAssets, a [liquidation](https://dev.flare.network/fassets/liquidation) mechanism ensures enough FAssets are burned to restore balance. The pool collateral provides an additional source of backing for situations when the price fluctuates too rapidly for liquidations to correct the imbalance.

Pool collateral is always native FLR tokens or SGB tokens on the Songbird network and can be used as an additional source of collateral for [liquidations](https://dev.flare.network/fassets/liquidation) and [failed redemptions](https://dev.flare.network/fassets/redemption#redemption-payment-failure).

Anyone can participate in the FAssets system by providing native tokens to this pool. In return, providers receive **collateral pool tokens** (CPTs) as proof of the share of native tokens they provided to a specific pool from a specific agent. CPTs are ERC-20 tokens specific to both an agent and a pool.

Providers can redeem their CPTs for FLR, or even transfer or trade them, after a governance-defined time period has elapsed since they entered the pool. This **time lock** is necessary to reduce sandwiching attacks.

Additionally, CPT holders are entitled to a share of any fee the agent earns from minting FAssets using this pool as explained in the next section.

CPT conversion formulae and examples.

Collateral Ratio[​](https://dev.flare.network/fassets/collateral#collateral-ratio "Direct link to Collateral Ratio")
--------------------------------------------------------------------------------------------------------------------

The collateral ratio (CR) is the ratio between the value of all the tokens used as collateral and the total value of the underlying assets held by an agent at any given time. The agent's vault and the collateral pool each has its own unique collateral ratio, which is constantly changing as the value of the underlying assets and the collateral change. These values are obtained using the [FTSO](https://dev.flare.network/ftso/overview).

The following example shows vault and pool CR:

Vault and pool CR

Assume an amount of FAssets currently valued at $1000 USD, backed by $1500 worth of USDC in vault collateral and $2000 worth of FLR in pool collateral.

The resulting vault CR is: $1500$1000=1.5\frac{\text{\$1500}}{\text{\$1000}} = 1.5

The resulting pool CR is: $2000$1000=2\frac{\text{\$2000}}{\text{\$1000}} = 2

Several thresholds are defined for the collateral ratio, and they are used at different times during the FAsset operations. Some are set by the system, and others are set by the agent:

### System-Wide Thresholds[​](https://dev.flare.network/fassets/collateral#system-wide-thresholds "Direct link to System-Wide Thresholds")

The following thresholds are set by the FAssets system's governance and are the same for all agents.

#### Minimal CR[​](https://dev.flare.network/fassets/collateral#minimal-cr "Direct link to Minimal CR")

The lowest collateral ratio the agent vault and the collateral pool must maintain so that enough collateral exists to insure the minted FAssets and to compensate for redemption payments that fail. The minimal CR can be different for each type of collateral.

If an agent's CR remains below the minimal CR for longer than a governance-set amount of time, [liquidations](https://dev.flare.network/fassets/liquidation) can start.

#### Liquidation CR[​](https://dev.flare.network/fassets/collateral#liquidation-cr "Direct link to Liquidation CR")

**Liquidation CR**: An agent's position is unhealthy when the agent's vault CR or pool CR fall below their minimal CR. However, as long as the CR remains above liquidation CR, the CR can briefly fall below the minimal CR.

During this time, the agent can either deposit more collateral or self-close some backed FAssets to improve the position.

However, if the CR falls below the liquidation CR, liquidations can start immediately.

The value of each liquidation CR is approximately 10% less than the minimal CR.

Example liquidation CR

Assume the **minimal CR** is 1.4 and the **liquidation CR** is 1.3.

If the agent's vault CR drops below 1.3, the agent's position can be liquidated immediately. If the agent's vault CR drops below 1.4 but not below 1.3, the agent has some time to amend the position before it can be liquidated.

Adjusted for the collateral pool's minimal CR, the same example applies to the collateral pool.

#### Safety CR[​](https://dev.flare.network/fassets/collateral#safety-cr "Direct link to Safety CR")

If one or both of the collateral types fall below liquidation CR or below the minimum CR for a longer period of time, liquidation occurs. When the offending collateral reaches a healthy CR again, the liquidation stops. To prevent the agent from immediately reverting into liquidation after a small price change, the CR must reach the safety CR before it can start operating normally again and liquidation stops.

Each of the collateral types, the agent's vault and the collateral pool, has its own unique safety CR.

### Agent Thresholds[​](https://dev.flare.network/fassets/collateral#agent-thresholds "Direct link to Agent Thresholds")

The following thresholds are set by each agent according to their own preferences.

#### Minting CR[​](https://dev.flare.network/fassets/collateral#minting-cr "Direct link to Minting CR")

For each mint done by an agent, the maximum amount allowed to be minted is calculated so that the CR for the agent's vault and the CR for the agent's collateral pool after the mint remain higher than the minting CR for each collateral type. To reduce the threat of liquidation, agents should set the minting CR well above the minimal CR to accommodate price fluctuations that might occur before the CR falls below the minimal CR after the mint and minting is no longer possible.

#### Exit CR[​](https://dev.flare.network/fassets/collateral#exit-cr "Direct link to Exit CR")

After a user redeems CPTs, the pool CR must be more than the exit CR. If the pool CR is already below the exit CR, redemption cannot occur. The exit CR is for the collateral pool only.

#### Top-up CR[​](https://dev.flare.network/fassets/collateral#top-up-cr "Direct link to Top-up CR")

To incentivize healthy collateral pools, if the pool CR falls below the top-up CR, anyone can add collateral to the pool and receive [CPTs](https://dev.flare.network/fassets/collateral#pool-collateral) at a reduced price. This [top-up mechanism](https://dev.flare.network/fassets/collateral#top-up) decreases the likelihood of liquidations because of a low amount of pool collateral.

Minting Fees and Debt[​](https://dev.flare.network/fassets/collateral#minting-fees-and-debt "Direct link to Minting Fees and Debt")
-----------------------------------------------------------------------------------------------------------------------------------

As part of the minting process, users pay a [minting fee](https://dev.flare.network/fassets/minting#fees) on the underlying chain. The agent's share of this fee remains on the underlying chain, whereas the pool's share triggers the minting of an equivalent amount of FAssets on the Flare network.

These FAssets coming from the minting fee are added to the collateral pool, where they are shared between collateral providers in proportion to the amount of CPTs that providers have. At any time, providers can claim their due share of the fees in the pool. When providers exit the collateral pool by redeeming their CPTs, any remaining unclaimed fee is automatically transferred to them.

Providers are naturally only entitled to the minting fees accrued after they entered the pool. Therefore, providers entering a pool with preexisting fees are assigned a **fee debt**. The amount of fees a provider can actually withdraw from the pool is calculated by first subtracting their debt from the total amount of fees in the pool. In this way, the amount of fees that a provider can withdraw upon entering a pool is exactly zero.

A provider's fee debt:

| Increases when the provider | Decreases when the provider |
| --- | --- |
| Enters a pool which already has fees in it. | Exits the pool, partially or completely. |
| Withdraws FAsset fees. | Deposits FAssets, paying off part of the debt. |

It is worth noting that:

*   When a provider withdraws fees, their debt increases by the same amount.
*   Since CPTs are ERC-20 tokens, a secondary market for them is expected to develop. If CPTs become more valuable than the FAsset fees they represent, returning the FAssets and paying off part of their fee debt might be more lucrative for providers.

Fee entitlement formulae and examples.

Transferable and Locked CPTs[​](https://dev.flare.network/fassets/collateral#transferable-and-locked-cpts "Direct link to Transferable and Locked CPTs")
--------------------------------------------------------------------------------------------------------------------------------------------------------

CPTs can always be **redeemed** by exiting the pool, but only the portion above the fee debt can be **transferred** to another account; therefore, CPTs held by providers are divided into two types.

### Transferable[​](https://dev.flare.network/fassets/collateral#transferable "Direct link to Transferable")

Tokens whose time lock has expired and are also free of fee debt. These tokens are fungible, and they can be transferred or traded just like any other ERC-20 token.

### Locked[​](https://dev.flare.network/fassets/collateral#locked "Direct link to Locked")

The CPTs serve only as proof of ownership of some of the collateral in the pool, and they cannot be transferred nor traded.

Locked CPTs are one of the following types:

*   **Time-locked**: Tokens whose time lock has not expired must wait to become transferable or redeemable.

*   **Debt-locked**: Tokens corresponding to an amount of fees below the provider's fee debt cannot be transferred because they would need to carry the debt with them. However, they can be [redeemed](https://dev.flare.network/fassets/collateral#cpt-redemption).

As new fees arrive in the pool, some previously debt-locked tokens become transferable.

These CPTs can also become transferable by adding FAssets to the pool, which settles, either partially or completely, the fee debt.

CPT transferability formulae and examples.

CPT Redemption[​](https://dev.flare.network/fassets/collateral#cpt-redemption "Direct link to CPT Redemption")
--------------------------------------------------------------------------------------------------------------

When collateral providers exit the pool by redeeming their CPTs, the FAssets system burns them and returns the appropriate share of the collateral plus the share of [FAsset-minting fees minus any FAsset-fee debt](https://dev.flare.network/fassets/collateral#minting-fees-and-debt).

Providers also have the option to exit the pool partially, by redeeming only some of their CPTs. In this case, they can choose one of the following options to manage their due FAsset fees: withdraw the fees, reduce the fee debt, or both, keeping the current fee-to-debt-ratio.

However, providers can exit, either fully or partially, only when the [collateral ratio CR](https://dev.flare.network/fassets/collateral#collateral-ratio) is high enough. After they exit, the **CR** must be higher than the **exit CR** to prevent their exit from reducing the **CR** to a dangerous level.

Therefore, exits are impossible when the **CR** is below the **exit CR**. In this case, if providers have enough FAssets, they can exit by **self-closing**, which burns enough of their FAssets, plus their fees, to release their collateral.

Providers are mainly compensated in underlying assets for the burned FAssets, depending on the [number of lots](https://dev.flare.network/fassets/minting#lots) of FAssets that need to be redeemed:

*   If more than 1 lot needs to be redeemed, the value of the burned FAssets is redeemed through the standard [redemption process](https://dev.flare.network/fassets/redemption).

*   If less than 1 lot needs to be redeemed, the agent buys the underlying funds from the user using vault collateral, at the price reported by the [FTSO](https://dev.flare.network/ftso/overview) minus a percentage defined by the agent. This purchase by the agent occurs because fees on underlying chains can be expensive, which makes redemption of small quantities too expensive for the agent.

Providers can always request this option instead of receiving underlying tokens. Also, if enough vault collateral is not available, pool collateral is used instead.

warning

In the case where the agent does not redeem in the underlying asset, the FAssets system pays the provider in collateral from the agent's vault because the pool collateral backing the redeemed FAssets is already withdrawn.

When this type of redemption occurs, users might receive less collateral than they would have received if they had made a normal redemption.

Agent Stake[​](https://dev.flare.network/fassets/collateral#agent-stake "Direct link to Agent Stake")
-----------------------------------------------------------------------------------------------------

Agents must have a stake in their collateral pools, which means they must hold the amount of CPTs proportional, by a system-defined constant, to the backed amount of FAssets. The maximum amount of minting is limited by the amount of collateral pool tokens held by the agents. The agents' tokens are locked, which means they cannot be redeemed or transferred, while agents back these FAssets.

When the agent's portion of the collateral pool is below the threshold, new mintings are not allowed. However, this situation does not trigger a liquidation because only the total pool stake matters when collateral needs to be redeemed or a liquidation payment needs to be made.

If an agent's actions force a payment to be made from the collateral pool, the agent's CPTs, valued by the paid native tokens and recalculated by the collateral-pool-price formula, are burned. These actions can cause the agent's CPTs to be burned:

*   When a redemption payment fails, when enough vault collateral to compensate the redeemer is not available, or when the system is set to automatically pay for redemption failures from the collateral pool.
*   Liquidation because the CR of the vault collateral is too low.
*   Full liquidation because of an [agent infraction](https://dev.flare.network/fassets/redemption#redemption-payment-failure) during a transfer on an underlying chain.

Top-up[​](https://dev.flare.network/fassets/collateral#top-up "Direct link to Top-up")
--------------------------------------------------------------------------------------

To reduce the likelihood of liquidations because the pool collateral is too low, the pool can be topped up at a reduced price when the **CR** is above the **top-up CR**. A top-up mechanism for vault collateral is not available. To prevent liquidation, agents can add vault collateral any time.


Title: Operational Parameters | Flare Developer Hub

This page lists the current values for the most important parameters of the FAssets system on **Songbird Canary-Network** and **Songbird Testnet Coston**. These values are subject to change as the system is further developed and tested.

Asset Manager Operational Parameters[​](https://dev.flare.network/fassets/operational-parameters#asset-manager-operational-parameters "Direct link to Asset Manager Operational Parameters")
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

To get the default agent settings, you need to call the `getSettings` function on the `IAssetManager` interface. Read more about the `IAssetManager` interface [here](https://dev.flare.network/fassets/reference/IAssetManager).

### Minting and Redeeming[​](https://dev.flare.network/fassets/operational-parameters#minting-and-redeeming "Direct link to Minting and Redeeming")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Minting cap** `mintingCapAMG` Total amount of allowed FAssets in circulation. Once reached, no more FAssets can be minted until some are redeemed. This is intended as a security measure. In the final deployment, this cap will be gradually increased and finally removed. | 750k XRP |
| [**Lot size**](https://dev.flare.network/fassets/minting#lots) `lotSizeAMG` Minimum quantity required for minting FAssets. | 10 XRP |
| [**Collateral reservation fee (CRF)**](https://dev.flare.network/fassets/minting#collateral-reservation-fee) `collateralReservationFee` Fee applied when reserving collateral for minting.. | 0.5% |
| [**Redemption fee**](https://dev.flare.network/fassets/redemption#redemption-fee) `redemptionFee` Fee charged during redemption of FAssets. | 0.5% |
| [**Redemption default premium**](https://dev.flare.network/fassets/redemption#redemption-payment-failure) `redemptionDefaultPremium` Premium paid if an agent fails to meet redemption obligations. | 5% |
| **Redemption default premium source** Where does the premium come from when an agent fails to pay the redeemer on time? If the vault CR > 1.1, from the agent's vault. Otherwise, from the agent's vault and the collateral pool. | ✅ |
| **Maximum redemption tickets** `maxRedeemedTickets` Maximum number of tickets redeemed in a single request. | 20 |

### Payment Times[​](https://dev.flare.network/fassets/operational-parameters#payment-times "Direct link to Payment Times")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Underlying blocks for payment** `underlyingBlocksForPayment` The number of underlying blocks during which the minter or agent can pay the underlying value. | 225 |
| **Underlying seconds for payment** `underlyingSecondsForPayment` The minimum time allowed for an agent to pay for a redemption or a minter to pay for minting. | 15 minutes |
| **Average block time** `averageBlockTimeMS` The average time between two successive blocks on the underlying chain. | 4 seconds |
| **Time of proof availability** `attestationWindowSeconds` The amount of time that proofs of payment or nonpayment must be available on the Data Connector. | 1 day |
| **Amount of extra time per redemption** `redemptionPaymentExtensionSeconds` The extra amount of time per redemption granted to an agent when many redemption requests occur in a short period of time. | 45 seconds |

### Collateral Ratios[​](https://dev.flare.network/fassets/operational-parameters#collateral-ratios "Direct link to Collateral Ratios")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Vault Collateral Supported Types** Types of collateral required in the agent's vault. | `USDX` |
| [**Vault Minimal CR**](https://dev.flare.network/fassets/collateral#minimal-cr) `minimalCR` The minimum collateral ratio required to avoid liquidation. | 1.2 |
| [**Vault Collateral Safety CR**](https://dev.flare.network/fassets/collateral#safety-cr) `safetyCR` The collateral ratio required to exit liquidation mode. | 1.3 |
| **Pool Collateral Supported Types** Types of collateral required in the collateral pool. | SGB |
| [**Pool Collateral Pool Minimal CR**](https://dev.flare.network/fassets/collateral#minimal-cr) `minimalCR` The minimum collateral ratio required to avoid liquidation. | 1.5 |
| [**Pool Collateral Call Band CR**](https://dev.flare.network/fassets/collateral#liquidation-cr) `ccbCR` The threshold at which collateral is considered unhealthy but liquidation is delayed. | 1.4 |
| [**Pool Collateral Safety CR**](https://dev.flare.network/fassets/collateral#safety-cr) `safetyCR` The collateral ratio required to exit liquidation mode. | 1.6 |
| **Minting pool holdings required** `mintingPoolHoldingsRequired` The minimum amount of pool tokens an agent must hold to be able to mint, as a percentage of the FAssets the agent is currently backing. | 50% |

### Liquidation[​](https://dev.flare.network/fassets/operational-parameters#liquidation "Direct link to Liquidation")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Liquidation premium** `liquidationPremium` Increases in steps, as time passes. | **Step 1**: 5% **Step 2**: 8% **Step 3**: 12% |
| **Liquidation step time** `liquidationStepTime` Elapsed time before the liquidation premium advances to the next step. | 300 seconds |
| **Liquidation source - Liquidated value** Where do the funds come from to pay for liquidations? | The agent's vault |
| **Liquidation source - Premium** Where do the funds come from to pay for liquidations? | The collateral pool |

### Rewarding[​](https://dev.flare.network/fassets/operational-parameters#rewarding "Direct link to Rewarding")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| [**Challenger reward**](https://dev.flare.network/fassets/overview#challengers) `paymentChallengeReward` After a successful challenge for an illegal operation, the agent goes into full liquidation and the challenger is paid this reward from the agent's vault. | 250 USD converted to vault collateral |
| [**Confirmation by others**](https://dev.flare.network/fassets/redemption#edge-cases) `confirmationByOthersAfter` If an agent or redeemer becomes unresponsive, anybody can confirm payments and non-payments some time after the request was made, and get a reward from the agent's vault. |  |
| **Minimum time** `confirmationByOthersAfter` | 6 hours |
| **Reward** `confirmationByOthersReward` | 50 USD (converted to vault collateral) |

### Time Locks[​](https://dev.flare.network/fassets/operational-parameters#time-locks "Direct link to Time Locks")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Time lock** `withdrawalTimelock` Agent has to announce any collateral withdrawal or vault destruction and then wait this time before executing it. | 1 hour |
| **Maximum governance update frequency** `minUpdateRepeatTime` Minimum amount of time between updates of any governance setting. | 1 day |
| **Token invalidation time** `tokenInvalidationTime` Time between the moment a token is deprecated by governance and it becomes invalid. Agents still using it as vault collateral get liquidated after this time. | 1 day |
| **Agent exit available time lock** `agentExitAvailableTimelock` The time the agent has to wait after announcing exit from the list of publicly available agents and executing the exit. | 3 hours |
| **Agent fee change time lock** `agentFeeChangeTimelock` The time the agent has to wait between announcing and changing the agent fee or the pool share. | 1 hour |
| **Agent minting CR change time lock** `agentMintingCRChangeTimelock` The time the agent has to wait between announcing and changing the minting CR (vault or pool). | 5 minutes |
| **Pool exit and top-up change time lock** `poolExitAndTopupChangeTimelock` The time the agent has to wait between announcing and changing any pool exit and top-up settings. | 1 day |
| **Agent time-locked operation window** `agentTimelockedOperationWindow` Once the above time locks expire, agents have this amount of time to execute the requested operation. | 2 hours |
| **Collateral pool token time lock** `collateralPoolTokenTimelock` Amount of seconds that a user entering the collateral pool must wait before spending (exit or transfer) the obtained pool tokens. | 60 seconds |
| **Minimum diamond-cut time lock** `diamondCutMinTimelockSeconds` Amount of time that must elapse before the system performs a <a href='https://eips.ethereum.org/EIPS/eip-2535' target='_blank'>diamond cut</a>. | 1 hour |

### Emergency Pause[​](https://dev.flare.network/fassets/operational-parameters#emergency-pause "Direct link to Emergency Pause")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Emergency pause** `maxEmergencyPauseDurationSeconds` The maximum time for a pause triggered by governance or some other entity. | 3 days |
| **Emergency pause reset** `emergencyPauseDurationResetAfterSeconds` The amount of time since the last emergency pause. After it has elapsed, the pause duration counter automatically resets. | 1 week |

### Transfer Fees[​](https://dev.flare.network/fassets/operational-parameters#transfer-fees "Direct link to Transfer Fees")

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Transfer fee represented as a fraction of one millionth of the transferred amount** `transferFeeMillionths` The fee on FAsset token transfer. Each transfer has this value times the transferred amount deducted from its value. The fees get deposited into epochs that are claimable by agents depending on their minting history. | 0 |
| **Maximum Unexpired Epochs for Transfer Fee Claims** `transferFeeClaimMaxUnexpiredEpochs` The number of epochs to pass before the fees get transferred to new epochs. | 30 |
| **Epoch Duration in Seconds for Transfer Fee Claims** `transferFeeClaimEpochDurationSeconds` Duration of each reward epoch. | 3.5 days |
| **Start Timestamp for First Transfer Fee Claim Epoch** `transferFeeClaimFirstEpochStartTs` The first reward epoch timestamp. | 1733122800 (Mon Dec 02 2024 07:00:00 GMT) |

Default Agent Settings[​](https://dev.flare.network/fassets/operational-parameters#default-agent-settings "Direct link to Default Agent Settings")
--------------------------------------------------------------------------------------------------------------------------------------------------

To get the default agent settings, you need to call the `getAgentInfo` function on the `IAssetManager` interface. Read more about the `IAssetManager` interface [here](https://dev.flare.network/fassets/reference/IAssetManager).

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| [**Minting fee**](https://dev.flare.network/fassets/minting#minting-fee) `feeBIPS` The minting fee is when users (minters) mint FAssets by depositing underlying assets with an agent. | 1% |
| [**Pool share**](https://dev.flare.network/fassets/minting#pool-share) `poolFeeShareBIPS` The pool share fee is the portion of the minting and redemption fees allocated to pool collateral providers. | 30% |
| [**Minting Collateral Ratio - Agent Vault**](https://dev.flare.network/fassets/collateral#minting-cr) `mintingVaultCollateralRatioBIPS` The minting vault collateral ratio is the minimum collateral required to back FAssets, ensuring value protection against under-collateralization. | 1.4 |
| [**Minting Collateral Ratio - Collateral Pool**](https://dev.flare.network/fassets/collateral#minting-cr) `mintingPoolCollateralRatioBIPS` The minting pool collateral ratio ensures the collateral value supports the minted FAssets. | 1.7 |
| [**Exit Collateral Ratio**](https://dev.flare.network/fassets/collateral#exit-cr) `poolExitCollateralRatioBIPS` The pool exit collateral ratio is the minimum collateral ratio agents must maintain when exiting their pool collateral. | 1.6 |
| [**Discount for agent self-close**](https://dev.flare.network/fassets/liquidation#stopping-liquidations) `buyFAssetByAgentFactorBIPS` Applied when agents buy back FAssets during liquidation events, shown as a factor on the Agent UI. | 99% |
| **Redemption Pool Fee Share** `redemptionPoolFeeShare` Percentage of redemption fees paid to the pool to sustain it during high redemption periods. | 30% |

Core Vault[​](https://dev.flare.network/fassets/operational-parameters#core-vault "Direct link to Core Vault")
--------------------------------------------------------------------------------------------------------------

### Core Vault Manager[​](https://dev.flare.network/fassets/operational-parameters#core-vault-manager "Direct link to Core Vault Manager")

To get the Core Vault manager operational parameters you need to use the [`ICoreVaultManager`](https://dev.flare.network/fassets/reference/ICoreVaultManager) interface. Specific functions added to each parameter.

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Escrow amount** [_escrowAmount](https://dev.flare.network/fassets/reference/ICoreVaultManager#getsettings)[getSettings](https://dev.flare.network/fassets/reference/ICoreVaultManager#getsettings)`_escrowAmount` The amount of XRP to escrow (setting to 0 disables escrowing). | 150k XRP |
| **Minimal left amount in the multisig** [_minimalAmount](https://dev.flare.network/fassets/reference/IAssetManager#getsettings)[getSettings](https://dev.flare.network/fassets/reference/IAssetManager#getsettings)`_minimalAmount` The minimal amount that will be left on the multisig after escrowing. | 150k XRP |
| **Escrow expiration time** [_escrowEndTimeSeconds](https://dev.flare.network/fassets/reference/IAssetManager#getsettings)[getSettings](https://dev.flare.network/fassets/reference/IAssetManager#getsettings)`_escrowEndTimeSeconds` The time of day (UTC) when the escrows expire. Exactly one escrow per day will expire. | 50400 (14:00 UTC) |
| **Max expected fee** [_fee](https://dev.flare.network/fassets/reference/IAssetManager#getsettings)[getSettings](https://dev.flare.network/fassets/reference/IAssetManager#getsettings)`_fee` Maximum expected fee charged by the chain for a payment | 0.0004 XRP (400 drops) |

### Core Vault Settings[​](https://dev.flare.network/fassets/operational-parameters#core-vault-settings "Direct link to Core Vault Settings")

To get the Core Vault settings you need to use the [`IAssetManager`](https://dev.flare.network/fassets/reference/IAssetManager) interface. Specific functions added to each parameter.

*   Flare Testnet Coston2
*   Songbird Canary-Network
*   Songbird Testnet Coston

| Parameter | XRP |
| --- | --- |
| **Minting left on agent's address** [getCoreVaultMinimumAmountLeftBIPS](https://dev.flare.network/fassets/reference/IAssetManager#getcorevaultminimumamountleftbips) Minimum amount of minting left on agent's address after transfer to core vault. Expressed as percentage of agent's minting capacity (calculated from agent's vault and pool collateral). | 15% |
| **Transfer to core vault time** [getCoreVaultTransferTimeExtensionSeconds](https://dev.flare.network/fassets/reference/IAssetManager#getcorevaulttransfertimeextensionseconds) The extra time for an agent's transfer to the core vault, compared to ordinary redemption payment. | 2 hours |
| **Transfer fee to Core Vault** [getCoreVaultTransferTimeExtensionSeconds](https://dev.flare.network/fassets/reference/IAssetManager#getcorevaulttransferfeebips) Fee (in percentage of transfer amount) paid by agent for transfer to the core vault. | 0 |
| **Minimum number of lots for direct redemption** [getCoreVaultMinimumRedeemLots](https://dev.flare.network/fassets/reference/IAssetManager#getcorevaultminimumredeemlots) The minimum number of lots that a direct redemption from core vault can take | 1000 |
| **Redemption fee** [getCoreVaultRedemptionFeeBIPS](https://dev.flare.network/fassets/reference/IAssetManager#getcorevaultredemptionfeebips) Fee (in percentage of redemption amount) paid by the redeemer for direct redemptions from the core vault. | 0 |


Title: FAssets on Songbird | Flare Developer Hub

FAssets on Songbird | Flare Developer Hub

===============

We use cookies to enhance your browsing experience, serve relevant ads or content, and analyze our traffic. By clicking "Accept All", you consent to our use of cookies.[Read our Privacy Policy.](https://flare.network/privacy-policy/)

Customize Accept all

Customize Consent Preferences

We use cookies to help you navigate efficiently and perform certain functions. You will find detailed information about all cookies under each consent category below.

The cookies that are categorized as "Necessary" are stored on your browser as they are essential for enabling the basic functionalities of the site. ...Show more

Necessary Always Active

Necessary cookies are required to enable the basic features of this site, such as providing secure log-in or adjusting your consent preferences. These cookies do not store any personally identifiable data.

Functional

- [x] 

Functional cookies help perform certain functionalities like sharing the content of the website on social media platforms, collecting feedback, and other third-party features.

Analytics

- [x] 

Analytical cookies are used to understand how visitors interact with the website. These cookies help provide information on metrics such as the number of visitors, bounce rate, traffic source, etc.

Performance

- [x] 

Performance cookies are used to understand and analyze the key performance indexes of the website which helps in delivering a better user experience for the visitors.

Advertisement

- [x] 

Advertisement cookies are used to provide visitors with customized advertisements based on the pages you visited previously and to analyze the effectiveness of the ad campaigns.

Others

- [x] 

Other uncategorized cookies are those that are being analyzed and have not been classified into a category as yet.

 Save My Preferences  Accept all 

 Powered by [](https://www.cookieyes.com/product/cookie-consent/)

[Skip to main content](https://dev.flare.network/fassets/songbird#__docusaurus_skipToContent_fallback)

[**Developer Hub**](https://dev.flare.network/)

[](https://github.com/flare-foundation/developer-hub)

ctrl K

*   [Home](https://dev.flare.network/)
*   [Network](https://dev.flare.network/network/overview) 
*   [FTSOv2](https://dev.flare.network/ftso/overview) 
*   [FDC](https://dev.flare.network/fdc/overview) 
*   [FAssets](https://dev.flare.network/fassets/overview) 
    *   [Minting](https://dev.flare.network/fassets/minting)
    *   [Redemption](https://dev.flare.network/fassets/redemption)
    *   [Collateral](https://dev.flare.network/fassets/collateral)
    *   [Core Vault](https://dev.flare.network/fassets/core-vault)
    *   [Liquidation](https://dev.flare.network/fassets/liquidation)
    *   [Operational Parameters](https://dev.flare.network/fassets/operational-parameters)
    *   [FAssets on Songbird](https://dev.flare.network/fassets/songbird)
    *   [Developer Guides](https://dev.flare.network/fassets/developer-guides) 
    *   [Infrastructure Guides](https://dev.flare.network/fassets/guides) 
    *   [FAssets Reference](https://dev.flare.network/fassets/reference) 

*   [Run a Node](https://dev.flare.network/run-node) 

*   [](https://dev.flare.network/)
*   [FAssets](https://dev.flare.network/fassets/overview)
*   FAssets on Songbird

FAssets on Songbird
===================

The launch of FAssets on Songbird Canary-Network demonstrates system behavior while paving the way for its next deployment on Flare Mainnet. The primary goals of this test are to ensure the system operates as intended, identify edge cases, refine usability and automation, and incentivize whitehat security researchers to uncover potential code errors.

The test on Songbird Canary-Network will have the following characteristics:

| Parameters | Description |
| --- | --- |
| FAsset Sequence | XRP will be tested first, followed by either BTC or DOGE. |
| Agent Whitelisting | FAssets agents must be whitelisted by Flare Foundation to perform their roles. |
| Caps and Losses | Flare Foundation will underwrite up to $300,000 in FAsset issuance to cover any losses resulting from system issues, while imposing a cap of $2 million in issuance per asset. |
| Duration of the Test | Each FAsset will be tested on Songbird for at least 6 weeks until no issues have been found. |
| FAssets Minting dApps | FAssets system users can access the frontend web interface for minting and redeeming: - [`https://fasset.oracle-daemon.com/sgb`](https://fasset.oracle-daemon.com/sgb) - [`https://fassets.au.cc/`](https://fassets.au.cc/) |
| System Integrity and FAsset Pricing | During the Songbird test, restrictions and incentives may cause the FAsset price to deviate from the underlying currency's value. The current focus is on testing system integrity, not price alignment. |
| Vault Collateral | USDX will serve as collateral for FAsset agent vaults. To ensure sufficient support for FAsset issuance and possible liquidations on Songbird, a large amount of USDX has been minted. |

Help improve FAssets

To participate, begin by joining the Flare Network FAssets Songbird [Telegram channel](https://t.me/FlareSupport) or contact [support@flare.network](mailto:support@flare.network).

Open Issue Collector

[Edit this page](https://github.com/flare-foundation/developer-hub/edit/main/docs/fassets/8-songbird.mdx)

[Previous Operational Parameters](https://dev.flare.network/fassets/operational-parameters)[Next Developer Guides](https://dev.flare.network/fassets/developer-guides)

[](https://flare.network/)

[Support](https://flare.network/resources/technical-support)|[Brand Kit](https://drive.google.com/drive/u/1/folders/1mPrtIBb2k88E4f1fguEm3eAXLW74xOry)|[Terms & Conditions](https://flare.network/privacy-policy/)|[UK Disclaimer](https://flare.network/uk-disclaimer)

[](https://github.com/flare-foundation)[](https://www.youtube.com/c/Flare_Networks)[](https://www.linkedin.com/company/flarenetwork/)[](https://discord.com/invite/flarenetwork)[](https://x.com/FlareNetworks)[](https://t.me/FlareNetwork)[](https://forum.flare.network/)

© Flare 2025

RESOURCES

[Whitepapers](https://dev.flare.network/support/whitepapers)[Audits](https://dev.flare.network/support/audits)[FAQs](https://dev.flare.network/support/faqs)[FLR](https://dev.flare.network/support/flr)

EXPLORE

[Flarescan](https://flarescan.com/)[Systems Explorer](https://flare-systems-explorer.flare.network/)[Bug Bounty](https://immunefi.com/bug-bounty/flarenetwork/information/)[Grants](https://flare.network/grants)

GOVERNANCE

[Flare Portal](https://portal.flare.network/)[Governance Proposals](https://proposals.flare.network/)





 ## CONFIG FILES: 

 ### foundry.toml

[profile.default]
src = "contracts"
out = "artifacts-forge"
libs = ["node_modules", "lib"]
test = "test-forge"
cache_path = 'cache-forge'
evm_version = 'london'
fs_permissions = [{ access = "read", path = "./artifacts-forge/"}]
optimizer = true
optimizer_runs = 200
ffi = true
remappings = [
    "forge-std/=lib/forge-std/src/"
]

[invariant]
show_metrics = true
fail_on_revert = false
runs = 100 # default is 256
depth = 200 # default is 500

# See more config options https://github.com/foundry-rs/foundry/blob/master/crates/config/README.md#all-options

### package.json

{
  "name": "@flarenetwork/fasset",
  "version": "1.2.0-rc.1",
  "description": "Smart contracts implementing FAsset system.",
  "main": "",
  "repository": {
    "type": "git",
    "url": "git+https://github.com/flare-foundation/fassets.git"
  },
  "author": "Flare Foundation",
  "license": "MIT",
  "directories": {},
  "engines": {
    "node": ">=20"
  },
  "files": [
    "artifacts",
    "contracts"
  ],
  "scripts": {
    "---------TEST---SCRIPTS": "",
    "test": "yarn hardhat test",
    "coverage": "env NODE_OPTIONS=\"--max_old_space_size=8192\" yarn hardhat coverage --testfiles",
    "test-with-coverage": "yarn clean && yarn compile && yarn coverage \"test/unit test/integration\"",
    "cov": "yarn coverage",
    "testHH": "tsc && yarn hardhat test \"test/{unit,integration}/**/*.ts\"",
    "test_unit_hh": "env TEST_PATH=./test/unit yarn hardhat test",
    "test_integration_hh": "env TEST_PATH=./test/integration yarn hardhat test",
    "test_e2e": "yarn fasset_simulation",
    "fasset_simulation": "yarn test test/e2e-simulation/fasset/FAssetSimulation.ts",
    "tsrun": "yarn ts-node --files=./type-extensions.ts",
    "---------COMPILE---SCRIPTS": "",
    "clean": "env rm -rf cache artifacts cache-forge artifacts-forge build build-info typechain typechain-truffle",
    "clean-all": "yarn clean && env rm -rf node_modules",
    "compile": "yarn hardhat compile && yarn typechain-truffle-v5",
    "c": "yarn compile",
    "cl": "yarn compile && yarn lint",
    "lint": "yarn solhint \"contracts/**/*.sol\"",
    "lint-forge": "yarn solhint \"test-forge/**/*.sol\"",
    "solhint-watch": "node scripts/solhint-watch.js",
    "typechain-truffle-v5": "yarn typechain --target=truffle-v5 --out-dir typechain-truffle \"artifacts/!(build-info)/**/+([a-zA-Z0-9_]).json\"",
    "size": "yarn run hardhat size-contracts",
    "flatten": "yarn hardhat flatten",
    "install-slither": "which slither > /dev/null || PIP_BREAK_SYSTEM_PACKAGES=1 pip3 install slither-analyzer",
    "slither": "yarn install-slither; rm -f ./slither.json 2> /dev/null; slither . --json=./slither.json 2> /dev/null || true; node scripts/slither-parse.js ./slither.json",
    "slither-show-stderr": "yarn install-slither; rm -f ./slither.json 2> /dev/null; slither . --json=./slither.json || true; node scripts/slither-parse.js ./slither.json",
    "ts-compile-watch": "tsc --watch --noEmit",
    "eslint": "eslint",
    "generate-json-schema": "typescript-json-schema --noExtraProps --required --strictNullChecks",
    "generate-parameter-schema": "yarn generate-json-schema deployment/lib/asset-manager-parameters.ts AssetManagerParameters -o deployment/config/asset-manager-parameters.schema.json",
    "---------DEPLOY---SCRIPTS---HARDHAT": "",
    "local": "yarn hardhat --network local",
    "full-deploy-hardhat": "yarn local deploy-price-reader-v2 && yarn local deploy-asset-manager-dependencies --all && yarn local deploy-asset-managers --deploy-controller --all",
    "full-deploy-hardhat-test": "yarn local test --no-compile deployment/test/test-deployed-contracts.ts",
    "mock-deploy-hardhat": "rm -f deployment/deploys/hardhat.json && yarn local run deployment/test/scripts/mock-deploy-dependencies.ts && yarn local run deployment/test/scripts/mock-deploy-stablecoins.ts && yarn full-deploy-hardhat && yarn full-deploy-hardhat-test",
    "flare-sc-deploy-hardhat": "yarn local run deployment/test/scripts/mock-deploy-stablecoins.ts && yarn full-deploy-hardhat && yarn full-deploy-hardhat-test",
    "---------DEPLOY---SCRIPTS---COSTON": "",
    "coston": "yarn hardhat --network coston",
    "deploy-mock-stablecoins-coston": "yarn coston run deployment/test/scripts/mock-deploy-stablecoins.ts",
    "deploy-price-reader-v2-coston": "yarn coston deploy-price-reader-v2",
    "deploy-dependencies-coston": "yarn coston deploy-asset-manager-dependencies",
    "deploy-with-controller-coston": "yarn coston deploy-asset-managers --deploy-controller --all",
    "full-deploy-coston-test": "yarn coston test --no-compile deployment/test/test-deployed-contracts.ts",
    "verify-coston": "yarn coston verify-contract",
    "verify-asset-manager-coston": "yarn coston verify-asset-managers --all",
    "verify-asset-manager-controller-coston": "yarn coston verify-asset-manager-controller",
    "verify-asset-manager-facets-coston": "yarn coston verify-asset-manager-facets",
    "console": "yarn hardhat console --no-compile --network",
    "console-coston": "yarn hardhat console --no-compile --network coston",
    "---------DEPLOY---SCRIPTS---SONGBIRD": "",
    "songbird": "yarn hardhat --network songbird",
    "verify-songbird": "yarn songbird verify-contract",
    "console-songbird": "yarn hardhat console --no-compile --network songbird",
    "---------DEPLOY---SCRIPTS---COSTON2": "",
    "coston2": "yarn hardhat --network coston2",
    "verify-coston2": "yarn coston2 verify-contract",
    "console-coston2": "yarn hardhat console --no-compile --network coston2",
    "---------DEPLOY---SCRIPTS---FLARE": "",
    "flare": "yarn hardhat --network flare",
    "verify-flare": "yarn flare verify-contract",
    "console-flare": "yarn hardhat console --no-compile --network flare",
    "---------INFO---SCRIPTS": "",
    "gas-snapshot": "env CI=true yarn testHH; yarn gas-report",
    "gas-report": "ts-node scripts/process-gas-report.ts && cat .gas-report.txt",
    "gas-report-check": "scripts/gas-report-check.sh",
    "gas": "cat .gas-report.txt"
  },
  "dependencies": {
    "@flarenetwork/flare-periphery-contracts": "0.1.30",
    "@openzeppelin/contracts": "4.9.6"
  },
  "devDependencies": {
    "@eslint/compat": "^1.3.0",
    "@eslint/js": "^9.29.0",
    "@flarenetwork/js-flare-common": "^0.0.1",
    "@gnosis.pm/mock-contract": "4.0.0",
    "@nomicfoundation/hardhat-network-helpers": "1.0.12",
    "@nomicfoundation/hardhat-verify": "2.0.14",
    "@nomiclabs/hardhat-truffle5": "2.0.7",
    "@nomiclabs/hardhat-web3": "2.0.1",
    "@typechain/truffle-v5": "7.0.0",
    "@types/chai": "5.2.2",
    "@types/mocha": "10.0.10",
    "@types/node": "20.19.1",
    "chai": "4.5.0",
    "dotenv": "16.5.0",
    "eslint": "^9.29.0",
    "eth-sig-util": "3.0.1",
    "ethereumjs-util": "7.1.5",
    "glob": "11.0.3",
    "hardhat": "2.24.3",
    "hardhat-contract-sizer": "2.10.0",
    "hardhat-gas-reporter": "1.0.9",
    "intercept-stdout": "0.1.2",
    "solhint": "5.2.0",
    "solidity-coverage": "0.8.16",
    "ts-node": "10.9.2",
    "typechain": "7.0.1",
    "typescript": "5.8.3",
    "typescript-eslint": "^8.34.0",
    "typescript-json-schema": "0.59.0"
  },
  "packageManager": "yarn@1.22.22"
}

### remappings.txt

forge-std/=lib/forge-std/src/


