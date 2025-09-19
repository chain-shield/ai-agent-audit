
## SUMMARY OF FILE: 2025-08-flare/contracts/agentOwnerRegistry/implementation/AgentOwnerRegistry.sol
Summary
AgentOwnerRegistry is a governance-controlled UUPS-upgradeable registry for FAssets agents. It holds no user funds. Governance or a delegated manager controls whitelisting and agent metadata; agents (from their management address) set a single work address. Major entrypoints: whitelistAndDescribeAgent, setWorkAddress, revokeAddress, setManager, metadata setters, and read-only lookups. ERC-165 compliant and implements IAgentOwnerRegistry.

Storage
- manager: address — delegated whitelist manager
- whitelist: mapping(address=>bool) — whitelisted mgmt addrs
- workToMgmtAddress: mapping(address=>address) — work→mgmt map
- mgmtToWorkAddress: mapping(address=>address) — mgmt→work map
- agentName: mapping(address=>string) — display name
- agentDescription: mapping(address=>string) — short description
- agentIconUrl: mapping(address=>string) — icon URL
- agentTouUrl: mapping(address=>string) — terms URL

Errors/Events (used here)
- Errors: AddressZero(), OnlyGovernanceOrManager(), AgentNotWhitelisted(), WorkAddressInUse()
- Events: ManagerChanged(address), Whitelisted(address), WhitelistingRevoked(address), WorkAddressChanged(address,address,address), AgentDataChanged(address,string,string,string,string)

Functions
- function initialize(IGovernanceSettings _governanceSettings, address _initialGovernance) external
  • Natspec: Initialize governance settings and mark implementation as initialized.

- function revokeAddress(address _address) external onlyGovernanceOrManager
  • Natspec: Remove a management address from whitelist.

- function setManager(address _manager) external onlyGovernance
  • Natspec: Set or change delegated manager for whitelist ops.

- function whitelistAndDescribeAgent(address _managementAddress, string memory _name, string memory _description, string memory _iconUrl, string memory _touUrl) external onlyGovernanceOrManager
  • Natspec: Whitelist management address and set public-facing agent metadata.

- function setWorkAddress(address _ownerWorkAddress) external
  • Natspec: From management address, set or clear the unique work address.

- function setAgentName(address _managementAddress, string memory _name) external onlyGovernanceOrManager
  • Natspec: Update agent owner’s display name.

- function setAgentDescription(address _managementAddress, string memory _description) external onlyGovernanceOrManager
  • Natspec: Update agent owner’s description text.

- function setAgentIconUrl(address _managementAddress, string memory _iconUrl) external onlyGovernanceOrManager
  • Natspec: Update agent owner’s icon URL.

- function setAgentTermsOfUseUrl(address _managementAddress, string memory _touUrl) external onlyGovernanceOrManager
  • Natspec: Update agent owner’s terms-of-use URL.

- function getAgentName(address _managementAddress) external view override returns (string memory)
  • Natspec: Read agent name by management address.

- function getAgentDescription(address _managementAddress) external view override returns (string memory)
  • Natspec: Read agent description by management address.

- function getAgentIconUrl(address _managementAddress) external view override returns (string memory)
  • Natspec: Read agent icon URL by management address.

- function getAgentTermsOfUseUrl(address _managementAddress) external view override returns (string memory)
  • Natspec: Read agent terms-of-use URL by management address.

- function getWorkAddress(address _managementAddress) external view override returns (address)
  • Natspec: Get work address associated with management address.

- function getManagementAddress(address _workAddress) external view override returns (address)
  • Natspec: Get management address mapped from a work address.

- function isWhitelisted(address _address) public view override returns (bool)
  • Natspec: Check if management address is whitelisted.

- function _addAddressToWhitelist(address _address) internal
  • Natspec: Internal add to whitelist; emits Whitelisted.

- function _removeAddressFromWhitelist(address _address) internal
  • Natspec: Internal remove from whitelist; emits WhitelistingRevoked.

- function _setAgentData(address _managementAddress, string memory _name, string memory _description, string memory _iconUrl, string memory _touUrl) private
  • Natspec: Internal set all metadata fields; emits AgentDataChanged.

- function _emitDataChanged(address _managementAddress) private
  • Natspec: Emit AgentDataChanged with current stored metadata.

- function supportsInterface(bytes4 _interfaceId) public pure override returns (bool)
  • Natspec: ERC-165 support for IERC165 and IAgentOwnerRegistry.

Modifiers
- onlyGovernanceOrManager: msg.sender must be manager or governance().
- onlyGovernance: inherited from GovernedUUPSProxyImplementation.

Notes
- Trust model: admin-only writes; agents self-manage work address. No asset custody.
- Upgradeability: UUPS via GovernedUUPSProxyImplementation under governance control.
- Uniqueness: each management has at most one work address; each work maps to one management.


## SUMMARY OF FILE: 2025-08-flare/contracts/agentOwnerRegistry/implementation/AgentOwnerRegistryProxy.sol
# AgentOwnerRegistryProxy (ERC1967Proxy)

A minimal ERC1967 proxy that fronts the AgentOwnerRegistry implementation. On deployment, it sets the implementation and immediately delegates initialize(governanceSettings, initialGovernance) to the logic. No user funds are held in the proxy; trust and upgrades are governed by the implementation (UUPS-style), which enforces governance controls. Major entrypoints are the constructor (deployment/initialization), fallback(), and receive(), which delegate all calls/ETH to the current implementation.

Storage Variables (inherited EIP-1967 slots)
- _IMPLEMENTATION_SLOT — Impl address slot
- _ADMIN_SLOT — Admin slot (unused here)
- _BEACON_SLOT — Beacon slot (unused)
- _ROLLBACK_SLOT — UUPS rollback guard

Functions
- constructor(address _implementationAddress, IGovernanceSettings _governanceSettings, address _initialGovernance) public nonpayable
  • Natspec: Deploy proxy and delegate-call initialize on AgentOwnerRegistry.
  • Modifiers: none

- fallback() external payable
  • Natspec: Delegate unknown function calls to current implementation.
  • Modifiers: none

- receive() external payable
  • Natspec: Accept native tokens; delegate if needed to implementation.
  • Modifiers: none


## SUMMARY OF FILE: 2025-08-flare/contracts/agentVault/implementation/AgentVault.sol
Overview
AgentVault is a per-agent vault contract holding ERC-20 collateral and interfacing with the AssetManager. Trust model: agent owner controls deposits/withdrawals within AssetManager-enforced rules; AssetManager can trigger payouts, destruction, and upgrades (UUPS). No user funds are directly deposited; users are protected via AssetManager controls and Collateral Pool. Major entrypoints: deposit/update/withdraw collateral, pool interactions (enter/exit/withdraw fees), payout (AM only), destroy (AM only), UUPS upgrade authorization (AM only).

Storage
- assetManager: IIAssetManager — managing authority
- initialized: bool — init guard
- __usedTokens: IERC20[] — storage gap
- __tokenUseFlags: mapping(IERC20=>uint256) — storage gap
- __internalWithdrawal: bool — storage gap
- destroyed: bool — vault destroy flag

Functions
- constructor(IIAssetManager _assetManager) — public constructor used in tests; calls initialize
- initialize(IIAssetManager _assetManager) public — Initializes vault once and sets AssetManager
- buyCollateralPoolTokens() external payable onlyOwner — Buy pool tokens by depositing native coin
- withdrawPoolFees(uint256 _amount, address _recipient) external onlyOwner — Withdraw pool-earned FAsset fees to recipient
- redeemCollateralPoolTokens(uint256 _amount, address payable _recipient) external onlyOwner nonReentrant — Exit pool and send native collateral to recipient
- depositCollateral(IERC20 _token, uint256 _amount) external override onlyOwner onlyKnownToken(_token) — Pull ERC-20 from owner and update collateral
- updateCollateral(IERC20 _token) external override onlyOwner onlyKnownToken(_token) — Sync collateral balances after direct token transfer
- withdrawCollateral(IERC20 _token, uint256 _amount, address _recipient) external override onlyOwner onlyKnownToken(_token) nonReentrant — Withdraw ERC-20 collateral subject to AM checks
- transferExternalToken(IERC20 _token, uint256 _amount) external override onlyOwner — Recover airdropped non-collateral tokens to owner
- destroy() external override onlyAssetManager nonReentrant — Mark vault destroyed; enables unrestricted asset recovery
- payout(IERC20 _token, address _recipient, uint256 _amount) external override onlyAssetManager nonReentrant — AM-directed payout for liquidation/redemption
- collateralPool() public view returns (ICollateralPool) — Get this vault’s collateral pool instance
- isOwner(address _address) public view returns (bool) — Query if address is the agent’s owner
- supportsInterface(bytes4 _interfaceId) external pure override returns (bool) — ERC165 support: IERC165, IAgentVault, IIAgentVault
- implementation() external view returns (address) — Read current implementation (UUPS)
- _authorizeUpgrade(address _newImplementation) internal virtual override onlyAssetManager — Restrict upgrades to AssetManager
- _validateToken(IERC20 _token) private view — Ensure token is approved vault collateral

Full interfaces with visibility/modifiers/mutability
- constructor(IIAssetManager _assetManager) — Create vault (tests); delegates to initialize
- function initialize(IIAssetManager _assetManager) public — One-time initializer; sets manager and reentrancy guard
- function buyCollateralPoolTokens() external payable onlyOwner — Enter pool by sending native value
- function withdrawPoolFees(uint256 _amount, address _recipient) external onlyOwner — Withdraw accrued FAsset fees from pool
- function redeemCollateralPoolTokens(uint256 _amount, address payable _recipient) external onlyOwner nonReentrant — Exit pool and transfer proceeds
- function depositCollateral(IERC20 _token, uint256 _amount) external override onlyOwner onlyKnownToken(_token) — Deposit ERC-20 collateral and notify AM
- function updateCollateral(IERC20 _token) external override onlyOwner onlyKnownToken(_token) — Notify AM after direct token transfer
- function withdrawCollateral(IERC20 _token, uint256 _amount, address _recipient) external override onlyOwner onlyKnownToken(_token) nonReentrant — Withdraw collateral after AM pre-check
- function transferExternalToken(IERC20 _token, uint256 _amount) external override onlyOwner — Sweep non-collateral tokens to owner management
- function destroy() external override onlyAssetManager nonReentrant — AssetManager flags vault as destroyed
- function payout(IERC20 _token, address _recipient, uint256 _amount) external override onlyAssetManager nonReentrant — AssetManager-directed payout transfer
- function collateralPool() public view returns (ICollateralPool) — Return pool for this vault
- function isOwner(address _address) public view returns (bool) — Verify ownership via AssetManager
- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool) — Report supported interfaces
- function implementation() external view returns (address) — UUPS implementation address
- function _authorizeUpgrade(address /*_newImplementation*/) internal virtual override onlyAssetManager — Authorize upgrades through AssetManager only
- function _validateToken(IERC20 _token) private view — Revert if token is not allowed collateral


## SUMMARY OF FILE: 2025-08-flare/contracts/agentVault/implementation/AgentVaultFactory.sol
## AgentVaultFactory — summary
AgentVaultFactory deploys new AgentVault instances behind ERC1967 proxies and initializes them with a provided AssetManager. The factory holds no funds and retains no privileged control post-deployment; upgrade/admin semantics are defined by the AgentVault implementation. Major entrypoints: create (deploy + initialize vault), implementation() (current logic address), upgradeInitCall (init calldata for upgradeToAndCall; empty), and supportsInterface (ERC-165).

### Storage
- implementation (address): vault logic address

### Functions
- function implementation() external view returns (address)
  - Returns current AgentVault implementation used for new proxies.

- function create(IIAssetManager _assetManager) external returns (IIAgentVault)
  - Deploys ERC1967Proxy for AgentVault and calls initialize with AssetManager.

- function upgradeInitCall(address _proxy) external pure override returns (bytes memory)
  - Encoded init calldata for upgradeToAndCall; empty in this version.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  - ERC-165 support for IERC165 and IIAgentVaultFactory.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentAlwaysAllowedMintersFacet.sol
## AgentAlwaysAllowedMintersFacet

Trust-minimized facet letting agent vault owners manage a per-agent whitelist of “always allowed” minters. No token custody; it only toggles addresses in storage. Authority is the agent vault owner via modifier checks. Entrypoints: addAlwaysAllowedMinterForAgent, removeAlwaysAllowedMinterForAgent, alwaysAllowedMintersForAgent (read-only).

### Storage
- Agent.State[agentVault].alwaysAllowedMinters (EnumerableSet.AddressSet) — minter whitelist

### Functions
- function addAlwaysAllowedMinterForAgent(address _agentVault, address _minter) external onlyAgentVaultOwner(_agentVault) nonpayable;
  Summary: Add an address to the agent’s always-allowed minter whitelist.

- function removeAlwaysAllowedMinterForAgent(address _agentVault, address _minter) external onlyAgentVaultOwner(_agentVault) nonpayable;
  Summary: Remove an address from the agent’s always-allowed minter whitelist.

- function alwaysAllowedMintersForAgent(address _agentVault) external view returns (address[] memory);
  Summary: Read full list of always-allowed minters for an agent vault.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentCollateralFacet.sol
# AgentCollateralFacet

Summary (<=100 words)
AgentCollateralFacet is a diamond facet of the Asset Manager enforcing agent collateral lifecycle actions. It timelocks and validates agent vault collateral withdrawals and pool-token redemptions, ensures collateral-ratio safety, and handles switching deprecated vault collateral and upgrading the pool’s WNat token. Trust model: funds are agent-owned (vault/pool), with constraints enforced by governance-set parameters; admins don’t move user funds. Major entrypoints: announceVaultCollateralWithdrawal, announceAgentPoolTokenRedemption, beforeCollateralWithdrawal (vault hook), updateCollateral (vault/pool hook), switchVaultCollateral, upgradeWNatContract.

Storage Variables
- None — facet declares no state variables; uses shared diamond storage via libraries.

Custom Errors
- WithdrawalInvalidAgentStatus — Agent not normal and still backing assets.
- WithdrawalNotAnnounced — No pending withdrawal announcement.
- WithdrawalMoreThanAnnounced — Attempt exceeds announced amount.
- WithdrawalNotAllowedYet — Timelock not elapsed.
- WithdrawalTooLate — Operation window expired.
- WithdrawalCRTooLow — Would drop CR below minting threshold.
- WithdrawalValueTooHigh — Insufficient free collateral to lock.
- OnlyAgentVaultOrPool — Caller must be agent vault or pool.
- CollateralNotDeprecated — Current vault collateral not deprecated.
- CollateralWithdrawalAnnounced — Pending vault withdrawal blocks switch.
- FAssetNotTerminated — Declared but unused in this facet.

Modifiers Used
- onlyAgentVaultOwner(_agentVault) — Restricts calls to the vault’s owner.

Events Emitted
- VaultCollateralWithdrawalAnnounced(agentVault, amountWei, allowedAt)
- PoolTokenRedemptionAnnounced(agentVault, amountWei, allowedAt)
- AgentCollateralTypeChanged(agentVault, class, token)

Functions
- interface:
  function announceVaultCollateralWithdrawal(address _agentVault, uint256 _valueNATWei)
      external
      onlyAgentVaultOwner(_agentVault)
      returns (uint256 _withdrawalAllowedAt);
  natspec: Announce vault collateral withdrawal and start timelock; returns earliest execution timestamp.

- interface:
  function announceAgentPoolTokenRedemption(address _agentVault, uint256 _valueNATWei)
      external
      onlyAgentVaultOwner(_agentVault)
      returns (uint256 _redemptionAllowedAt);
  natspec: Announce agent pool-token redemption; locks amount and returns when redemption is allowed.

- interface:
  function beforeCollateralWithdrawal(IERC20 _token, uint256 _amountWei)
      external;
  natspec: Vault hook before withdrawal; enforces announcement, timelock, CR safety, and consumes allowance.

- interface:
  function updateCollateral(address _agentVault, IERC20 _token)
      external;
  natspec: Vault/pool hook on deposit; may end liquidation if collateral is healthy.

- interface:
  function switchVaultCollateral(address _agentVault, IERC20 _token)
      external
      onlyAgentVaultOwner(_agentVault);
  natspec: Switch deprecated vault collateral token; blocked if a vault withdrawal is announced.

- interface:
  function upgradeWNatContract(address _agentVault)
      external
      onlyAgentVaultOwner(_agentVault);
  natspec: Upgrade pool WNat collateral to latest governance token and update pool.

- interface (internal helper):
  function _announceWithdrawal(Collateral.Kind _kind, address _agentVault, uint256 _amountWei)
      private
      returns (uint256);
  natspec: Core announcement logic; checks free collateral, sets allowedAt, emits appropriate event.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentInfoFacet.sol
## AgentInfoFacet — Summary
Read-only diamond facet exposing agent discovery and status views for minters/liquidators. No custody of user funds; reads diamond storage via libraries (Agents, AgentCollateral, AssetManagerState). Admin-less; pure query surface. Major entrypoints: getAllAgents, isPoolTokenSuffixReserved, getAgentInfo, and several detailed collateral/CR helpers.

### Storage Variables
- None (direct) — facet uses diamond storage via libraries
- AssetManagerState.State.reservedPoolTokenSuffixes — reserved CPT suffixes map
- Agent.State (per vault) — various agent fields (read-only here)

### Functions
- function getAllAgents(uint256 _start, uint256 _end) external view returns (address[] memory _agents, uint256 _totalLength);
  - NatSpec: Paged fetch of all registered agent vault addresses and total count.

- function isPoolTokenSuffixReserved(string memory _suffix) external view returns (bool);
  - NatSpec: Checks if a collateral pool token symbol suffix is already taken.

- function getAgentInfo(address _agentVault) external view returns (AgentInfo.Info memory _info);
  - NatSpec: Returns comprehensive agent data for UI/minter selection.

- function getCollateralPool(address _agentVault) external view returns (address);
  - NatSpec: Gets the collateral pool contract address for an agent vault.

- function getAgentVaultOwner(address _agentVault) external view returns (address _ownerManagementAddress);
  - NatSpec: Returns agent’s management (owner) address.

- function getAgentVaultCollateralToken(address _agentVault) external view returns (IERC20);
  - NatSpec: Returns the ERC-20 vault collateral token used by the agent.

- function getAgentFullVaultCollateral(address _agentVault) external view returns (uint256);
  - NatSpec: Total vault collateral amount in token wei.

- function getAgentFullPoolCollateral(address _agentVault) external view returns (uint256);
  - NatSpec: Total pool collateral (native) measured in wei.

- function getAgentLiquidationFactorsAndMaxAmount(address _agentVault) external view returns (uint256 _liquidationPaymentFactorVaultBIPS, uint256 _liquidationPaymentFactorPoolBIPS, uint256 _maxLiquidationAmountUBA);
  - NatSpec: Current liquidation split factors and max liquidatable amount if liquidating.

- function getAgentMinPoolCollateralRatioBIPS(address _agentVault) external view returns (uint256);
  - NatSpec: Reads system minimum pool CR for minting for the agent.

- function getAgentMinVaultCollateralRatioBIPS(address _agentVault) external view returns (uint256);
  - NatSpec: Reads system minimum vault CR for minting for the agent.

- function _getFullCollateral(address _agentVault, Collateral.Kind _kind) private view returns (uint256);
  - NatSpec: Helper: full collateral for selected kind (VAULT or POOL).

- function _getMinCollateralRatioBIPS(address _agentVault, Collateral.Kind _kind) private view returns (uint256);
  - NatSpec: Helper: system min CR (BIPS) for minting per collateral kind.

- function _getLiquidationFactorsAndMaxAmount(Agent.State storage _agent, Liquidation.CRData memory _cr) private view returns (uint256 _vaultFactorBIPS, uint256 _poolFactorBIPS, uint256 _maxLiquidatedUBA);
  - NatSpec: Helper: computes liquidation split factors and max UBA if in liquidation.

### Notes
- Inherits: AssetManagerBase (facilitates access to diamond storage/libraries).
- Trust model: View-only; no state mutation or funds handling.
- Libraries: Agents, AgentCollateral, Conversion, Liquidation, LiquidationPaymentStrategy, UnderlyingBalance.
- Key outputs in getAgentInfo include: fees, CRs, free/full collateral, liquidation info, UBA balances, pool token details, status, underlying address.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentPingFacet.sol
AgentPingFacet — purpose and entrypoints
A minimal facet of the Asset Manager implementing IAgentPing. It holds no funds, writes no storage, and only emits events for off-chain monitoring. Any address may ping an agent vault via agentPing. Only the agent vault’s management owner (validated by onlyAgentVaultOwner) may answer via agentPingResponse. Trust model: no privileged admin actions here; user funds untouched; access control only on responses.

Storage variables
- None (facet declares no storage)

Functions
- interface: function agentPing(address _agentVault, uint256 _query) external nonpayable
  natspec: Anyone pings an agent vault; emits AgentPing event.

- interface: function agentPingResponse(address _agentVault, uint256 _query, string memory _response) external nonpayable onlyAgentVaultOwner(_agentVault)
  natspec: Agent vault owner replies with message; emits AgentPingResponse.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentSettingsFacet.sol
## AgentSettingsFacet summary

Facet enabling agent vault owners to timelock and update agent parameters (fees, pool shares, minting collateral ratios, buyback factor, pool exit CR). It does not custody user funds; it mutates Agent.State within the AssetManager diamond. Governance-defined timelocks gate updates and must be executed within an operation window. Major entrypoints: announceAgentSettingUpdate, executeAgentSettingUpdate, getAgentSetting.

Recognized setting names
- feeBIPS
- poolFeeShareBIPS
- redemptionPoolFeeShareBIPS
- mintingVaultCollateralRatioBIPS
- mintingPoolCollateralRatioBIPS
- buyFAssetByAgentFactorBIPS
- poolExitCollateralRatioBIPS

Events (emitted)
- AgentSettingChangeAnnounced(agentVault, name, value, validAt)
- AgentSettingChanged(agentVault, name, value)

Custom errors
- NoPendingUpdate()
- UpdateNotValidYet()
- UpdateNotValidAnymore()
- InvalidSettingName()

Storage variables
- None in this facet (uses Agent.State, Globals, AssetManagerSettings via libraries)

Timelock sources
- feeBIPS, poolFeeShareBIPS, redemptionPoolFeeShareBIPS, buyFAssetByAgentFactorBIPS → settings.agentFeeChangeTimelockSeconds
- mintingVaultCollateralRatioBIPS, mintingPoolCollateralRatioBIPS → settings.agentMintingCRChangeTimelockSeconds
- poolExitCollateralRatioBIPS → settings.poolExitCRChangeTimelockSeconds

Execution window
- Must execute after validAt and before validAt + settings.agentTimelockedOperationWindowSeconds

Functions

1) Interface
function announceAgentSettingUpdate(address _agentVault, string memory _name, uint256 _value)
    external nonpayable onlyAgentVaultOwner(_agentVault)
    returns (uint256 _updateAllowedAt)
Natspec: Announce a timelocked change to an agent setting; records value and validAt; emits announcement.

2) Interface
function executeAgentSettingUpdate(address _agentVault, string memory _name)
    external nonpayable onlyAgentVaultOwner(_agentVault)
Natspec: Execute the pending setting update after timelock; enforces execution window; emits changed.

3) Interface
function getAgentSetting(address _agentVault, string memory _name)
    external view returns (uint256 _value)
Natspec: Read a single agent setting by name; reverts on invalid name.

4) Interface
function _executeUpdate(Agent.State storage _agent, bytes32 _hash, uint256 _value)
    private nonpayable
Natspec: Internal dispatcher that applies the concrete setting change via AgentUpdates library.

5) Interface
function _getTimelock(bytes32 _hash)
    private view returns (uint64)
Natspec: Resolve timelock duration for a setting based on name group and global settings.

6) Interface
function _getAndCheckHash(string memory _name)
    private pure returns (bytes32)
Natspec: Hash and validate a setting name; reverts for unknown names.

Notes for integrators
- Only agent vault owner can announce/execute.
- Values are stored in Agent.State.settingUpdates[hash] as {value:uint128, validAt:uint64}.
- poolExitCollateralRatioBIPS is read from agent.collateralPool.exitCollateralRatioBIPS().
- Uses OpenZeppelin SafeCast for safe narrowing.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentVaultAndPoolSupportFacet.sol
# AgentVaultAndPoolSupportFacet

Summary: Read-only facet of the Asset Manager diamond exposing helper views for agent vaults, pool tokens, pricing, and ownership resolution. No custody or privileged admin logic; it only reads diamond storage via libraries (Globals, Agents, Conversion). Entrypoints: assetPriceNatWei, isLockedVaultToken, isVaultCollateralToken, getFAssetsBackedByPool, isAgentVaultOwner, getWorkAddress, getWNat.

Storage
- None (facet has no state). Uses shared diamond storage via libraries.

Functions
- function assetPriceNatWei() external view returns (uint256 _multiplier, uint256 _divisor)
  - NatSpec: Returns UBA price in NAT wei as a fraction.

- function isLockedVaultToken(address _agentVault, IERC20 _token) external view returns (bool)
  - NatSpec: True if token is agent’s vault collateral or pool token.

- function isVaultCollateralToken(IERC20 _token) external view returns (bool)
  - NatSpec: Checks if token is a (valid or invalidated) vault collateral type.

- function getFAssetsBackedByPool(address _agentVault) external view returns (uint256)
  - NatSpec: Total UBA backed by pool: reserved + minted + pool redeeming.

- function isAgentVaultOwner(address _agentVault, address _address) external view returns (bool)
  - NatSpec: Verifies if address is the owner/manager of the agent vault.

- function getWorkAddress(address _managementAddress) external view returns (address)
  - NatSpec: Resolves management to work address via AgentOwnerRegistry.

- function getWNat() external view returns (IWNat)
  - NatSpec: Returns the WNat contract used by AgentVault.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AgentVaultManagementFacet.sol
AgentVaultManagementFacet (diamond facet) manages agent vault lifecycle and collateral pool creation/upgrades for the FAssets system. It doesn’t custody user funds; it orchestrates agent-owned vaults/pools through factories. Access is controlled by: onlyAttached (facet bound to AssetManager diamond), onlyAgentVaultOwner (per-vault admin), and governance via onlyAssetManagerController. Major entrypoints: createAgentVault, announceDestroyAgent, destroyAgent, upgradeAgentVaultAndPool, upgradeAgentVaultsAndPools.

Storage variables
- None (facet) – uses diamond storage via libraries
- MIN_SUFFIX_LEN (const) – min suffix length
- MAX_SUFFIX_LEN (const) – max suffix length

Custom errors
- AddressInvalid – Underlying address attestation invalid
- AgentStillAvailable – Agent still on available list
- AgentStillActive – Agent backs assets
- DestroyNotAnnounced – Destroy not pre-announced
- DestroyNotAllowedYet – Destroy timelock not elapsed
- SuffixReserved – Pool token suffix taken
- SuffixInvalidFormat – Bad suffix format
- AddressUsedByCoreVault – Address equals Core Vault underlying

Key behaviors
- Validates and claims unique underlying address via FDC attestation; forbids Core Vault’s address.
- Creates AgentVault proxy and CollateralPool + PoolToken via factories; initializes agent settings and fee splits; reserves unique pool token symbol suffix.
- Destroys agent (vault + pool) after timelock and zero-backed state; returns residual funds to recipient.
- Batch or per-vault upgrades of AgentVault, CollateralPool, and PoolToken proxies to latest implementations, with optional upgrade init-call.

Events (via IAssetManagerEvents)
- AgentVaultCreated(ownerMgmt, agentVault, data)
- AgentDestroyAnnounced(agentVault, allowedAt)
- AgentDestroyed(agentVault)

External/public functions
- function createAgentVault(IAddressValidity.Proof calldata _addressProof, AgentSettings.Data calldata _settings) external onlyAttached returns (address _agentVault)
  Summary: Creates a new agent vault, collateral pool, and initializes agent configuration.

- function announceDestroyAgent(address _agentVault) external onlyAgentVaultOwner(_agentVault) returns (uint256 _destroyAllowedAt)
  Summary: Starts agent destruction timelock; requires no mints and not available.

- function destroyAgent(address _agentVault, address payable _recipient) external onlyAgentVaultOwner(_agentVault)
  Summary: Finalizes destruction; destroys pool and vault; sends remaining funds to recipient.

- function upgradeAgentVaultAndPool(address _agentVault) external onlyAgentVaultOwner(_agentVault)
  Summary: Upgrades one agent’s vault, pool, and pool token proxies to latest implementations.

- function upgradeAgentVaultsAndPools(uint256 _start, uint256 _end) external onlyAssetManagerController
  Summary: Governance batch-upgrades ranges of agent vaults/pools to latest implementations.

Internal/private helpers
- function _upgradeAgentVaultAndPool(address _agentVault) private
  Summary: Resolves pool and token, then upgrades all three proxies if versions differ.

- function _upgradeContract(IUpgradableContractFactory _factory, address _proxyAddress) private
  Summary: Upgrades a proxy to factory’s implementation; optional init call executed.

- function _createCollateralPool(IIAssetManager _assetManager, address _agentVault, AgentSettings.Data calldata _settings) private returns (IICollateralPool)
  Summary: Deploys collateral pool and pool token; wires token to pool.

- function _reserveAndValidatePoolTokenSuffix(string memory _suffix) private
  Summary: Reserves unique suffix; enforces A-Z, 0-9, internal ‘-’, and length bounds.

- function _emitAgentVaultCreated(address _ownerManagementAddress, address _agentVault, IICollateralPool _collateralPool, string memory _underlyingAddress, AgentSettings.Data calldata _settings) private
  Summary: Emits AgentVaultCreated with full data payload.

- function _getManagementAddress(address _ownerAddress) private view returns (address)
  Summary: Resolves management address from registry; falls back to provided address.

Access control and trust
- onlyAttached ensures calls originate via AssetManager diamond deployment.
- onlyAgentVaultOwner gates destructive/upgrade operations to the agent’s owner.
- onlyAssetManagerController restricts batch upgrades to governance/controller.

Notes
- No direct token handling; minting/backing state enforced via Agent/AssetManager diamond storage.
- Destruction requires zero backed AMGs and removal from available agents list.
- Pool token suffix uniqueness prevents griefing through symbol collisions.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AssetManagerDiamondCutFacet.sol
AssetManagerDiamondCutFacet — Summary
This facet implements EIP-2535 diamond upgrades for the Asset Manager. Only governance may schedule and execute upgrades, enforced by a timelock read from Globals. It doesn’t handle user funds directly but controls system code evolution; thus trust is in governance + timelock. Major entrypoint: diamondCut (perform facet add/replace/remove and optional init delegatecall).

Storage
- None (facet is stateless; relies on LibDiamond storage and inherited governance state)

Functions
- function diamondCut(IDiamondCut.FacetCut[] calldata _diamondCut, address _init, bytes calldata _calldata) external override onlyGovernanceWithTimelockAtLeast(Globals.getSettings().diamondCutMinTimelockSeconds)
  Mutability: nonpayable
  NatSpec: Perform diamond cut; governance-only with minimum timelock; optional delegatecall initializer on _init.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AssetManagerInit.sol
AssetManagerInit.sol — Summary

Initializer facet for the Asset Manager diamond. It is governance-controlled and holds no user funds. Purpose: set governance, bootstrap reentrancy guard, validate/apply system settings and initial collateral types, and register ERC165 support. Trust model: only governance/admin uses these entrypoints; users are unaffected. Major entrypoints: init and upgradeERC165Identifiers.

Storage
- No explicit variables in this facet — uses diamond storage
- LibDiamond.DiamondStorage.supportedInterfaces — ERC165 flags

Functions
- function init(IGovernanceSettings _governanceSettings, address _initialGovernance, AssetManagerSettings.Data memory _settings, CollateralType.Data[] memory _initialCollateralTypes) external nonpayable
  NatSpec: One-time initializer to set governance, guards, settings, collateral types, and ERC165 support.

- function upgradeERC165Identifiers() external nonpayable
  NatSpec: After diamond cut, marks new/backward-compatible interfaces as supported; requires prior initialization.

- function _initIERC165() private nonpayable
  NatSpec: Internal helper to set initial ERC165 and diamond loupe/cut interface flags.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/AvailableAgentsFacet.sol
# AvailableAgentsFacet — Summary
A Diamond facet for the Asset Manager that manages the public list of minting-capable agents. It lets an agent owner publish their vault as “available,” announce and execute exits with time locks, and query paginated agent lists (basic and detailed). No user funds are held here; it only updates shared diamond storage. Access control is strict: only the agent vault owner can add or remove availability. Main entrypoints: makeAgentAvailable, announceExitAvailableAgentList, exitAvailableAgentList, getAvailableAgentsList, getAvailableAgentsDetailedList.

## Trust model and access
- User funds: None; this facet only reads/writes diamond storage and emits events.
- Admin: No global admin flow here; only agent vault owners can change availability (modifier: onlyAgentVaultOwner).
- Safety: Timelocked exit with a limited execution window; availability requires at least 1 free lot of collateral.

## Storage (touched via diamond storage/library)
- AssetManagerState.availableAgents — array of agent vaults
- Agent.State.status — agent lifecycle status
- Agent.State.availableAgentsPos — 1-based index in availableAgents
- Agent.State.feeBIPS — agent mint fee bips
- Agent.State.mintingVaultCollateralRatioBIPS — min CR for vault
- Agent.State.mintingPoolCollateralRatioBIPS — min CR for pool
- Agent.State.exitAvailableAfterTs — exit earliest timestamp
- Agent.State.ownerManagementAddress — agent cold wallet
- AssetManagerSettings.agentExitAvailableTimelockSeconds — exit timelock
- AssetManagerSettings.agentTimelockedOperationWindowSeconds — post-timelock execution window

## Errors
- ExitTooLate — Exceeded post-timelock operation window
- ExitTooSoon — Timelock not yet expired
- ExitNotAnnounced — Exit not pre-announced
- AgentNotAvailable — Agent not in public list
- NotEnoughFreeCollateral — < 1 lot free collateral
- AgentAlreadyAvailable — Already listed
- InvalidAgentStatus — Status not NORMAL

## Events (emitted via IAssetManagerEvents)
- AgentAvailable(agentVault, feeBIPS, mintingVaultCR, mintingPoolCR, freeCollateralLots)
- AvailableAgentExitAnnounced(agentVault, exitAllowedAt)
- AvailableAgentExited(agentVault)

## Functions
- function makeAgentAvailable(address _agentVault) external onlyAgentVaultOwner(_agentVault);
  — Lists agent if NORMAL and at least 1 free lot; emits AgentAvailable.

- function announceExitAvailableAgentList(address _agentVault) external onlyAgentVaultOwner(_agentVault) returns (uint256 _exitAllowedAt);
  — Starts exit timelock; records exitAllowedAt and emits announcement.

- function exitAvailableAgentList(address _agentVault) external onlyAgentVaultOwner(_agentVault);
  — Removes agent after timelock within window; swaps/pop array; emits exit.

- function getAvailableAgentsList(uint256 _start, uint256 _end) external view returns (address[] memory _agents, uint256 _totalLength);
  — Paginates available agents addresses; bounds indices safely.

- function getAvailableAgentsDetailedList(uint256 _start, uint256 _end) external view returns (AvailableAgentInfo.Data[] memory _agents, uint256 _totalLength);
  — Paginates with fee, min CRs, free lots, status for each agent.

## Notes
- availableAgentsPos is 1-based; 0 means not listed.
- Removal uses swap-with-last for O(1) deletion and updates moved agent’s position.
- Free collateral lots computed via AgentCollateral.combinedData(...).freeCollateralLots(agent).
- Detailed view computes min minting CRs via AgentCollateral.mintingMinCollateralRatio(...).


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/ChallengesFacet.sol
### Summary
ChallengesFacet is a diamond facet that lets anyone challenge agent misconduct using Flare Data Connector (FDC) proofs. On successful challenge, it immediately starts full liquidation of the agent and pays a challenger reward from the agent’s vault collateral. It does not custody user funds; it reads diamond storage and settles from agent vault collateral. Major entrypoints: illegalPaymentChallenge, doublePaymentChallenge, freeBalanceNegativeChallenge. Emits IllegalPaymentConfirmed, DuplicatePaymentConfirmed, and UnderlyingBalanceTooLow on success, and invokes Liquidation.startFullLiquidation.

### Storage
- No local storage variables
- _status (inherited, ReentrancyGuard) – reentrancy flag
- Uses diamond storage via libraries:
  - AssetManagerState.State – global AM state (diamond storage)
  - Agent.State – per-agent state (diamond storage)

### Functions
- function illegalPaymentChallenge(IBalanceDecreasingTransaction.Proof calldata _payment, address _agentVault) external nonReentrant
  - Natspec: Challenge payment without valid reference; triggers full liquidation and rewards challenger.

- function doublePaymentChallenge(IBalanceDecreasingTransaction.Proof calldata _payment1, IBalanceDecreasingTransaction.Proof calldata _payment2, address _agentVault) external nonReentrant
  - Natspec: Challenge two distinct txs with same reference; triggers full liquidation and rewards challenger.

- function freeBalanceNegativeChallenge(IBalanceDecreasingTransaction.Proof[] calldata _payments, address _agentVault) external nonReentrant
  - Natspec: Challenge multiple txs that drive agent’s free underlying balance below required; liquidate and reward.

- function _validateAgentStatus(Agent.State storage _agent) private view
  - Natspec: Reverts if agent already in full liquidation or destroying.

- function _liquidateAndRewardChallenger(Agent.State storage _agent, address _challenger, uint256 _backingAMGAtChallenge) private
  - Natspec: Starts full liquidation and pays challenger from agent vault per settings.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/CollateralReservationsFacet.sol
# CollateralReservationsFacet — Summary
A diamond facet in the Asset Manager that creates collateral reservations for minting. It calculates and collects the collateral reservation fee (CRF), verifies agent/queue/CR constraints, locks agent collateral, and emits payment instructions for underlying-chain deposits. Trust model: holds only transient NAT (reservation fee); change is returned. No admin writes here—governance parameters are read via Globals. Agents must be whitelisted; minting can be paused. Major entrypoints: reserveCollateral (creates CRT, payable) and collateralReservationFee (reads CRF).

## Storage (Diamond/Shared State Used)
- Agent.State.status — agent lifecycle
- Agent.State.feeBIPS — agent mint fee bips
- Agent.State.poolFeeShareBIPS — pool fee share
- Agent.State.availableAgentsPos — queue position
- Agent.State.alwaysAllowedMinters — allowlist set
- Agent.State.underlyingAddressString — underlying addr
- Agent.State.reservedAMG — agent reserved AMG
- AssetManagerState.State.mintingPausedAt — pause flag
- AssetManagerState.State.newCrtId — CRT id counter
- AssetManagerState.State.totalReservedCollateralAMG — global reserved
- AssetManagerState.State.crts — CRT storage map
- AssetManagerState.State.currentUnderlyingBlock — last U-block
- AssetManagerState.State.currentUnderlyingBlockUpdatedAt — last upd ts
- AssetManagerState.State.currentUnderlyingBlockTimestamp — last U-ts
- AssetManagerState.State.poolCollateralIndex — pool price index
- AssetManagerSettings.averageBlockTimeMS — underlying avg ms
- AssetManagerSettings.underlyingBlocksForPayment — blocks to pay
- AssetManagerSettings.underlyingSecondsForPayment — seconds to pay
- AssetManagerSettings.collateralReservationFeeBIPS — CRF bips
- CollateralReservation.Data.valueAMG — CRT value AMG
- CollateralReservation.Data.underlyingFeeUBA — agent fee UBA
- CollateralReservation.Data.reservationFeeNatWei — CRF in NAT
- CollateralReservation.Data.poolFeeShareBIPS — pool share +1
- CollateralReservation.Data.agentVault — agent vault addr
- CollateralReservation.Data.minter — minter addr
- CollateralReservation.Data.executor — executor addr
- CollateralReservation.Data.executorFeeNatGWei — exec fee gwei
- CollateralReservation.Data.firstUnderlyingBlock — first U-block
- CollateralReservation.Data.lastUnderlyingBlock — deadline block
- CollateralReservation.Data.lastUnderlyingTimestamp — deadline ts
- CollateralReservation.Data.status — CRT status enum

Note: This facet declares no own storage; it reads/writes diamond storage via libraries.

## Custom Errors
- InappropriateFeeAmount — msg.value below CRF
- AgentsFeeTooHigh — max fee lower than agent fee
- NotEnoughFreeCollateral — insufficient free lots
- InvalidAgentStatus — agent not NORMAL
- CannotMintZeroLots — lots == 0
- AgentNotInMintQueue — agent not public and not allowlisted
- MintingPaused — system mint pause active

## Functions
- function reserveCollateral(address _agentVault, uint256 _lots, uint256 _maxMintingFeeBIPS, address payable _executor) external payable onlyAttached notEmergencyPaused nonReentrant returns (uint256 _collateralReservationId)
  - Reserves collateral, collects CRF, creates CRT, emits underlying payment instructions.

- function collateralReservationFee(uint256 _lots) external view returns (uint256 _reservationFeeNATWei)
  - Returns NAT wei required as CRF for provided lots.

- function _reserveCollateral(Agent.State storage _agent, uint64 _reservationAMG) private
  - Checks minting cap and increments reserved collateral counters.

- function _emitCollateralReservationEvent(Agent.State storage _agent, CollateralReservation.Data memory _cr, uint256 _crtId) private
  - Emits CollateralReserved event with payment and executor details.

- function _currentPoolFeeAMG(Agent.State storage _agent, uint64 _valueAMG) private view returns (uint64)
  - Computes current pool fee in AMG for given value.

- function _lastPaymentBlock() private view returns (uint64 _lastUnderlyingBlock, uint64 _lastUnderlyingTimestamp)
  - Calculates payment deadlines using underlying block/time and settings.

- function _reservationFee(uint256 amgToTokenWeiPrice, uint64 _valueAMG) private view returns (uint256)
  - Computes CRF in NAT wei from price and governance BIPS.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/CollateralTypesFacet.sol
# CollateralTypesFacet — Summary
Governance-only facet in the Asset Manager diamond for managing collateral token types and their risk parameters. It does not custody user funds; it only updates system configuration. Trust model: admin (AssetManagerController/governance) can add/deprecate collateral and change ratios under rate-limits; users can read types. Major entrypoints: addCollateralType, setCollateralRatiosForToken, deprecateCollateralType, getCollateralType, getCollateralTypes.

## Storage
Note: The facet has no local storage; it mutates diamond/library storage.
- CollateralTypes[(class, token)] — collateral type store
- minCollateralRatioBIPS — min CR (BIPS)
- safetyMinCollateralRatioBIPS — safety min CR
- validUntil — deprecation ts
- tokenInvalidationTimeMinSeconds — min deprec time
- SettingsUpdater.lastUpdate[actionKey] — per-action cooldown

## Errors
- DeprecationTimeToShort — Deprecation time below governance minimum.
- TokenNotValid — Token already deprecated/invalid.

## Functions
- interface: function addCollateralType(CollateralType.Data calldata _data) external onlyAssetManagerController
  natspec: Add a new collateral type with initial parameters. Governance-controlled.

- interface: function setCollateralRatiosForToken(CollateralType.Class _collateralClass, IERC20 _token, uint256 _minCollateralRatioBIPS, uint256 _safetyMinCollateralRatioBIPS) external onlyAssetManagerController
  natspec: Update min and safety collateral ratios; rate-limited per (class, token). Emits CollateralRatiosChanged.

- interface: function deprecateCollateralType(CollateralType.Class _collateralClass, IERC20 _token, uint256 _invalidationTimeSec) external onlyAssetManagerController
  natspec: Schedule deprecation; enforces minimum invalidation time. Emits CollateralTypeDeprecated.

- interface: function getCollateralType(CollateralType.Class _collateralClass, IERC20 _token) external view returns (CollateralType.Data memory)
  natspec: Read a single collateral type’s full info snapshot.

- interface: function getCollateralTypes() external view returns (CollateralType.Data[] memory _collateralTypes)
  natspec: Enumerate all collateral types, including deprecated ones.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/CoreVaultClientFacet.sol
# CoreVaultClientFacet (diamond facet)

Purpose: Orchestrates agent/Core Vault interactions: agents can transfer backing to Core Vault, request/confirm fund returns from Core Vault, and users can redeem FAssets directly from Core Vault. Trust model: only agent vault owners can trigger agent flows; governance-configured CoreVaultManager and emergency pause gate behavior; FDC proofs validate payments. No custody of user funds; contract burns user FAssets when redeeming from CV. Major entrypoints: transferToCoreVault, requestReturnFromCoreVault, cancelReturnFromCoreVault, confirmReturnFromCoreVault, redeemFromCoreVault, maximumTransferToCoreVault, coreVaultAvailableAmount.

## Storage (diamond/library-backed)

Global CoreVaultClient.State
- initialized: init flag
- coreVaultManager: CV manager iface
- nativeAddress: native addr str
- transferTimeExtensionSeconds: extra time seconds
- newTransferFromCoreVaultId: next return id
- minimumRedeemLots: min lots direct
- redemptionFeeBIPS: CV redeem fee
- newRedemptionFromCoreVaultId: next CV red id

Per-agent Agent.State
- status: agent status enum
- underlyingBalanceUBA: underlying bal
- activeTransferToCoreVault: active CV xfer id
- activeReturnFromCoreVaultId: active return id
- returnFromCoreVaultReservedAMG: reserved AMG
- reservedAMG: total reserved AMG
- underlyingAddressString: agent underl addr
- underlyingAddressHash: agent addr hash

Global AssetManagerState
- paymentConfirmations: seen payments

Note: This facet declares no own storage; it uses diamond storage via libraries.

## Functions

- function transferToCoreVault(address _agentVault, uint256 _amountUBA) external onlyEnabled notEmergencyPaused nonReentrant onlyAgentVaultOwner(_agentVault)
  - Natspec: Move agent’s minted backing to Core Vault and open CV redemption.

- function requestReturnFromCoreVault(address _agentVault, uint256 _lots) external onlyEnabled notEmergencyPaused nonReentrant onlyAgentVaultOwner(_agentVault)
  - Natspec: Reserve collateral and request Core Vault to send funds back to agent.

- function cancelReturnFromCoreVault(address _agentVault) external onlyEnabled nonReentrant onlyAgentVaultOwner(_agentVault)
  - Natspec: Cancel pending Core Vault return and release reserved collateral.

- function confirmReturnFromCoreVault(IPayment.Proof calldata _payment, address _agentVault) external onlyEnabled nonReentrant onlyAgentVaultOwner(_agentVault)
  - Natspec: Verify FDC payment from Core Vault, remint, update backing, clear reservation.

- function redeemFromCoreVault(uint256 _lots, string memory _redeemerUnderlyingAddress) external onlyEnabled notEmergencyPaused nonReentrant
  - Natspec: Burn caller’s FAssets and request direct redemption payout from Core Vault.

- function maximumTransferToCoreVault(address _agentVault) external view returns (uint256 _maximumTransferUBA, uint256 _minimumLeftAmountUBA)
  - Natspec: Read max transferable to Core Vault and required minimum left.

- function coreVaultAvailableAmount() external view returns (uint256 _immediatelyAvailableUBA, uint256 _totalAvailableUBA)
  - Natspec: Read Core Vault immediately and total available amounts for payouts.

## Key Modifiers and Guards
- onlyEnabled: Core Vault feature enabled.
- notEmergencyPaused: global pause guard.
- nonReentrant: reentrancy protection.
- onlyAgentVaultOwner(_agentVault): restricts to agent’s vault owner.

## Custom Errors (used for efficient reverts)
- CannotReturnZeroLots, InvalidAgentStatus, InvalidPaymentReference, NoActiveReturnRequest, NotEnoughAvailableOnCoreVault, NotEnoughFreeCollateral, NotEnoughUnderlying, NothingMinted, PaymentNotFromCoreVault, PaymentNotToAgentsAddress, RequestedAmountTooSmall, ReturnFromCoreVaultAlreadyRequested, TooLittleMintingLeftAfterTransfer, TransferAlreadyActive, ZeroTransferNotAllowed.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/CoreVaultClientSettingsFacet.sol
# CoreVaultClientSettingsFacet (diamond facet)

Purpose: Configure Core Vault parameters for the AssetManager diamond. This facet only stores configuration and updates supported interfaces; it does not custody user funds. Governance controls setters via onlyGovernance/onlyImmediateGovernance. Major entrypoints: initCoreVaultFacet, updateInterfacesAtCoreVaultDeploy, setters for manager/native address/timing/fees/thresholds, and read-only getters. Emits IAssetManagerEvents on changes.

Storage (CoreVaultClient.State)
- initialized: bool — facet init flag
- coreVaultManager: IICoreVaultManager — CV manager contract
- nativeAddress: address payable — CV underlying addr
- transferTimeExtensionSeconds: uint64 — extra time for CV ops
- redemptionFeeBIPS: uint16 — CV redemption fee bips
- minimumAmountLeftBIPS: uint16 — min capacity left off-CV
- minimumRedeemLots: uint64 — min lots for direct redeem

Custom errors
- WrongAssetManager
- CannotDisable
- DiamondNotInitialized
- AlreadyInitialized
- BipsValueTooHigh

Functions
- constructor() public nonpayable
  - Prevent initialize on implementation by setting state.initialized.

- initCoreVaultFacet(IICoreVaultManager _coreVaultManager, address payable _nativeAddress, uint256 _transferTimeExtensionSeconds, uint256 _redemptionFeeBIPS, uint256 _minimumAmountLeftBIPS, uint256 _minimumRedeemLots) external nonpayable
  - One-time facet init; sets CV manager, addresses, time, fees, thresholds.

- updateInterfacesAtCoreVaultDeploy() public nonpayable
  - Registers new ERC‑165 interfaces for CV client and settings.

- setCoreVaultManager(address _coreVaultManager) external onlyGovernance nonpayable
  - Sets Core Vault manager; cannot disable and must match this asset manager.

- setCoreVaultNativeAddress(address payable _nativeAddress) external onlyImmediateGovernance nonpayable
  - Sets underlying-chain native address used by the Core Vault.

- setCoreVaultTransferTimeExtensionSeconds(uint256 _transferTimeExtensionSeconds) external onlyImmediateGovernance nonpayable
  - Sets extra time window for Core Vault transfers.

- setCoreVaultRedemptionFeeBIPS(uint256 _redemptionFeeBIPS) external onlyImmediateGovernance nonpayable
  - Sets direct redemption fee in BIPS; capped by MAX_BIPS.

- setCoreVaultMinimumAmountLeftBIPS(uint256 _minimumAmountLeftBIPS) external onlyImmediateGovernance nonpayable
  - Sets minimum minting capacity kept outside Core Vault (BIPS).

- setCoreVaultMinimumRedeemLots(uint256 _minimumRedeemLots) external onlyImmediateGovernance nonpayable
  - Sets minimum lot count for direct redemption from Core Vault.

- getCoreVaultManager() external view returns (address)
  - Reads Core Vault manager address.

- getCoreVaultNativeAddress() external view returns (address)
  - Reads Core Vault native underlying address.

- getCoreVaultTransferTimeExtensionSeconds() external view returns (uint256)
  - Reads extra transfer time extension in seconds.

- getCoreVaultRedemptionFeeBIPS() external view returns (uint256)
  - Reads redemption fee in BIPS for direct CV redemptions.

- getCoreVaultMinimumAmountLeftBIPS() external view returns (uint256)
  - Reads minimum capacity kept outside CV in BIPS.

- getCoreVaultMinimumRedeemLots() external view returns (uint256)
  - Reads minimum number of lots for direct redemption.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/EmergencyPauseFacet.sol
# EmergencyPauseFacet — summary
EmergencyPauseFacet is a Diamond facet on the Asset Manager that controls emergency pausing. It does not custody user funds; it only gates system operations. Mutations are restricted to the AssetManagerController (admin). Governance can enforce a pause that others cannot override; non-governance pauses are limited by a rolling maximum duration with a reset window. Major entrypoints: emergencyPause, resetEmergencyPauseTotalDuration, and read-only getters for pause state.

## Storage (via AssetManagerState.State)
- emergencyPausedUntil (uint64) — pause end timestamp
- emergencyPausedByGovernance (bool) — pause set by governance
- emergencyPausedTotalDuration (uint64) — cumulative pause window

Note: Uses settings from Globals.AssetManagerSettings:
- maxEmergencyPauseDurationSeconds — rolling max non-governance pause
- emergencyPauseDurationResetAfterSeconds — reset window

## Errors
- PausedByGovernance() — non-gov cannot change governance pause

## Functions
- function emergencyPause(bool _byGovernance, uint256 _duration) external nonpayable onlyAssetManagerController
  - Natspec: Set or clear pause. Gov can override; non-gov obeys rolling max duration and reset window.

- function resetEmergencyPauseTotalDuration() external nonpayable onlyAssetManagerController
  - Natspec: Zeroes accumulated non-governance pause duration counter.

- function emergencyPaused() external view returns (bool)
  - Natspec: Returns true if system is currently emergency-paused.

- function emergencyPausedUntil() external view returns (uint256)
  - Natspec: Pause end timestamp or zero when not paused.

- function emergencyPauseDetails() external view returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance)
  - Natspec: Returns pause end, total accumulated duration, and governance pause flag.

- function _paused() private view returns (bool)
  - Natspec: Helper: active pause if emergencyPausedUntil > block.timestamp.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/EmergencyPauseTransfersFacet.sol
EmergencyPauseTransfersFacet – Diamond facet to control system-wide transfer pausing within AssetManager. Only the AssetManagerController may invoke state-changing methods. Governance-triggered pauses cannot be shortened or overridden by non-governance calls. Read-only helpers expose current pause state and metadata. Major entrypoints: emergencyPauseTransfers, resetEmergencyPauseTransfersTotalDuration, transfersEmergencyPaused, transfersEmergencyPausedUntil, emergencyPauseTransfersDetails.

Storage
- AssetManagerState.State (diamond storage)
  - transfersEmergencyPausedUntil – UNIX ts pause end
  - transfersEmergencyPausedTotalDuration – Cumulated pause dur
  - transfersEmergencyPausedByGovernance – Paused by governance
- AssetManagerSettings (read via Globals)
  - emergencyPauseDurationResetAfterSeconds – Reset window secs
  - maxEmergencyPauseDurationSeconds – Max cum. pause secs

Functions
- function emergencyPauseTransfers(bool _byGovernance, uint256 _duration) external nonpayable onlyAssetManagerController
  - Start/extend pause; enforces caps, reset window; governance pause immutable by non-gov.
  - Emits: EmergencyPauseTransfersTriggered(pausedUntil) when pausing; EmergencyPauseTransfersCanceled() when unpausing.
  - Reverts: PausedByGovernance() if trying to override governance pause.

- function resetEmergencyPauseTransfersTotalDuration() external nonpayable onlyAssetManagerController
  - Resets accumulated pause duration counter to zero.

- function transfersEmergencyPaused() external view returns (bool)
  - True if transfers are currently paused (pausedUntil > now).

- function transfersEmergencyPausedUntil() external view returns (uint256)
  - Returns pause end timestamp or 0 if not paused.

- function emergencyPauseTransfersDetails() external view returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance)
  - Returns pause end, total accumulated duration, and governance flag.

- function _transfersPaused() private view returns (bool)
  - Internal helper: pausedUntil > block.timestamp.

Notes
- Uses Math and SafeCast for safe arithmetic and casting.
- Non-governance pauses are bounded by maxEmergencyPauseDurationSeconds from a moving start (projectedStartTime).
- Duration counter auto-resets if no pause for emergencyPauseDurationResetAfterSeconds.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/LiquidationFacet.sol
# LiquidationFacet (Diamond facet)

Purpose: Manages agent liquidation workflow when collateral ratios fall below governance thresholds. Trust model: Non-custodial; pays liquidators from agent’s vault collateral and shared pool per strategy. No admin-only flows here; guarded by system-wide emergency pause and reentrancy guard. Major entrypoints: startLiquidation, liquidate, endLiquidation.

Storage variables
- None declared — facet uses diamond/library storage
- ReentrancyGuard._status — reentrancy flag
- Agent.State (via Agent.get) — per-agent record
- AssetManagerState.collateralTokens — collateral type settings

Functions
- function startLiquidation(address _agentVault) external notEmergencyPaused nonReentrant returns (uint256 _liquidationStartTs);
  - NatSpec: Start liquidation if agent CR under minimum; returns start timestamp.
  - Mutability: nonpayable

- function liquidate(address _agentVault, uint256 _amountUBA) external notEmergencyPaused nonReentrant returns (uint256 _liquidatedAmountUBA, uint256 _amountPaidVault, uint256 _amountPaidPool);
  - NatSpec: Burns caller’s FAssets to liquidate agent; pays from vault/pool; ends if healthy.
  - Mutability: nonpayable

- function endLiquidation(address _agentVault) external nonReentrant;
  - NatSpec: Ends liquidation if agent collateral is safe; callable by anyone.
  - Mutability: nonpayable

- function _startLiquidation(Agent.State storage _agent, Liquidation.CRData memory _cr) private returns (bool _inLiquidation);
  - NatSpec: Set underwater flags and enter LIQUIDATION if any CR below min.
  - Mutability: nonpayable

- function _isCollateralUnderwater(uint256 _collateralRatioBIPS, uint256 _collateralIndex) private view returns (bool);
  - NatSpec: Returns true when CR is below collateral type minimum.
  - Mutability: view

- function _performLiquidation(Agent.State storage _agent, Liquidation.CRData memory _cr, uint64 _amountAMG) private returns (uint64 _liquidatedAMG, uint256 _payoutC1Wei, uint256 _payoutPoolWei);
  - NatSpec: Compute max liquidatable, close tickets, and calculate vault/pool payouts.
  - Mutability: nonpayable

- function _agentResponsibilityWei(Agent.State storage _agent, uint256 _amount) private view returns (uint256);
  - NatSpec: Portion of pool payout blamed on agent for CPT slashing.
  - Mutability: view

Errors
- CannotStopLiquidation — Attempt to stop when still unsafe or in full liquidation.
- NotInLiquidation — Liquidation required but not active.
- LiquidationNotStarted — Start precondition failed.
- LiquidationNotPossible(AgentInfo.Status) — Status disallows liquidation.

Notes
- Emits IAssetManagerEvents.LiquidationStarted and LiquidationPerformed.
- Uses Conversion, Redemptions, LiquidationPaymentStrategy for math and flows.
- Honors emergency pause via AssetManagerBase.notEmergencyPaused.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/MintingDefaultsFacet.sol
MintingDefaultsFacet (diamond facet) manages mint payment defaults and long-lived "stuck" reservations. It lets the agent vault owner reclaim locked collateral when a minter fails to pay, and recover from expired proof windows by burning equivalent value. Funds are non-custodial; only the agent vault owner may trigger actions. Fees are distributed/burned per settings. Major entrypoints: mintingPaymentDefault, unstickMinting.

Storage Variables
- None declared in this facet
- ReentrancyGuard._status – reentrancy flag
- AssetManagerBase (diamond storage) – shared state access

Custom Errors
- CannotUnstickMintingYet
- MintingNonPaymentProofWindowTooShort
- MintingDefaultTooEarly
- MintingNonPaymentMismatch
- SourceAddressesNotSupported
- NotEnoughFundsProvided

Functions
- function mintingPaymentDefault(IReferencedPaymentNonexistence.Proof calldata _proof, uint256 _crtId) external nonReentrant
  NatSpec: Agent proves minter nonpayment; unlocks reserved collateral and distributes reservation fees.

- function unstickMinting(IConfirmedBlockHeightExists.Proof calldata _proof, uint256 _crtId) external payable nonReentrant
  NatSpec: After proof window expiry, burn CRF and reserved collateral; release reservation; refund excess NAT.

- function _burnVaultCollateral(Agent.State storage _agent, uint256 _amountVaultCollateralWei) private returns (uint256 _burnedNatWei)
  NatSpec: Convert vault collateral to NAT at premium and burn provided NAT amount.

Key Behaviors and Notes
- mintingPaymentDefault: Requires FDC ReferencedPaymentNonexistence proof with checkSourceAddresses=false; validates reference, destination hash, amount; enforces last block/time passed and window start ≤ firstUnderlyingBlock. Emits MintingPaymentDefault, releases reservation, splits CRF to vault/pool.
- unstickMinting: Requires FDC ConfirmedBlockHeightExists proof showing query window < reservation window and now beyond attestationWindowSeconds. Burns CRF (reservation + executor fee), buys vault collateral for NAT at FTSO price × vaultCollateralBuyForFlareFactorBIPS, transfers collateral to agent owner, burns NAT, releases reservation, refunds surplus NAT.
- Access control: Agents.requireAgentVaultOwner(agent) gates both external functions.
- Reentrancy: Guarded by nonReentrant; NAT transfers use Transfers.transferNAT and Globals.getBurnAddress().
- Events emitted: MintingPaymentDefault, CollateralReservationDeleted.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/MintingFacet.sol
Summary
MintingFacet is a diamond facet enabling FAsset minting finalization and agent self-mint flows. It verifies off-chain payments via FDC attestations, mints ERC-20 FAssets, credits pool fees, updates agent underlying balances, and releases reserved collateral. Trust model: user funds are not custodied by the contract beyond minting logic; agents post collateral and receive fees; governance can pause via emergency controls. Major entrypoints: executeMinting (public mint finalization), selfMint (agent one-step mint with payment), mintFromFreeUnderlying (agent mints using free underlying).

Storage
- (inherited) ReentrancyGuard._status – reentrancy flag
- (via AssetManagerState) state.mintingPausedAt – pause flag
- (via AssetManagerState) state.paymentConfirmations – used payment registry
- (via Agent.State) agent.status – agent lifecycle
- (via Agent.State) agent.underlyingAddressHash – native addr hash
- (via Agent.State) agent.underlyingBlockAtCreation – creation block idx
- (via Agent.State) agent.underlyingBalanceUBA – underlying balance
- (via Agent.State) agent.collateralPool – pool contract ref

Enums
- enum MintingType { PUBLIC, SELF_MINT, FROM_FREE_UNDERLYING }

Key custom errors
- CannotMintZeroLots
- FreeUnderlyingBalanceToSmall
- InvalidMintingReference
- InvalidSelfMintReference
- MintingPaused
- MintingPaymentTooOld
- MintingPaymentTooSmall
- NotEnoughFreeCollateral
- NotMintingAgentsAddress
- OnlyMinterExecutorOrAgent
- SelfMintInvalidAgentStatus
- SelfMintNotAgentsAddress
- SelfMintPaymentTooOld
- SelfMintPaymentTooSmall

Functions
- interface: function executeMinting(IPayment.Proof calldata _payment, uint256 _crtId) external nonReentrant nonpayable
  natspec: Finalizes public mint with FDC payment proof; mints FAssets, pays pool, releases reservation.

- interface: function selfMint(IPayment.Proof calldata _payment, address _agentVault, uint256 _lots) external onlyAttached notEmergencyPaused nonpayable
  natspec: Agent one-step mint using underlying payment; enforces whitelist, capacity, fees, and references.

- interface: function mintFromFreeUnderlying(address _agentVault, uint64 _lots) external onlyAttached notEmergencyPaused nonpayable
  natspec: Agent mints directly from free underlying balance; checks capacity and pool fee.

- interface: function _performMinting(Agent.State storage _agent, MintingType _mintingType, uint256 _crtId, address _minter, uint64 _mintValueAMG, uint256 _receivedAmountUBA, uint256 _poolFeeUBA) private nonpayable
  natspec: Internal mint execution; updates balances, mints tokens, deposits pool fee, emits events.

Notes and behavior
- executeMinting: callable by minter, executor, or agent owner; validates payment reference, receiver hash, amount, and time; records attestation; updates underlying block; releases collateral and distributes CRF; executor fee payout.
- selfMint: requires agent vault owner and whitelisted; status NORMAL; sufficient free collateral; validates self-mint reference and recipient; allows lots=0 to convert payment to free balance; enforces minting cap including pool fee.
- mintFromFreeUnderlying: requires agent owner and whitelist; status NORMAL; lots>0; sufficient free collateral; ensures required underlying after mint is covered by current underlyingBalanceUBA.
- Events: emits IAssetManagerEvents.MintingExecuted for public mints and IAssetManagerEvents.SelfMint for agent flows.
- Guards: ReentrancyGuard; onlyAttached and notEmergencyPaused (from AssetManagerBase); strict library checks (Agents, AgentCollateral).


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/RedemptionConfirmationsFacet.sol
# RedemptionConfirmationsFacet — summary
Trust-minimized facet in the AssetManager diamond that finalizes redemptions by validating off-chain payment proofs (via FDC) and releasing/settling collateral. No admin-held user funds; agents post collateral and pay redeemers. Primary entrypoint: confirmRedemptionPayment. It verifies payment reference, timing, source/receiver, updates underlying balances, mints pool fee share, pays third-party confirmation rewards, and may end liquidation when healthy.

## Storage
- (inherited) uint256 private _status — reentrancy flag
- No direct storage; uses diamond/library storage (AssetManagerState, Agent, Redemptions, etc.).

## Custom errors
- InvalidReceivingAddressSelected()
- SourceNotAgentsUnderlyingAddress()
- RedemptionPaymentTooOld()
- InvalidRedemptionReference()

## Functions
- function confirmRedemptionPayment(IPayment.Proof calldata _payment, uint256 _redemptionRequestId) external nonReentrant
  — Confirm agent’s redemption payment; release/unlock collateral; handle success/blocked/failed; reward others.
  Emits: RedemptionPerformed | RedemptionPaymentBlocked | RedemptionPaymentFailed | RedemptionPoolFeeMinted
  Mutability: nonpayable (state-changing)

- function _mintPoolFee(Agent.State storage _agent, Redemption.Request storage _request, uint256 _redemptionRequestId) private
  — Re-mint pool’s fee share in FAssets and account it into the collateral pool.
  Mutability: nonpayable (state-changing)

- function _othersCanConfirmPayment(Redemption.Request storage _request) private view returns (bool)
  — Check if confirmation-by-others window elapsed per settings.
  Mutability: view

- function _validatePayment(Redemption.Request storage request, IPayment.Proof calldata _payment) private view returns (bool _paymentValid, string memory _failureReason)
  — Validate FDC payment: status, receiver, amounts, timing, and request status.
  Mutability: view



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/RedemptionDefaultsFacet.sol
# RedemptionDefaultsFacet

Short summary (≤100 words)

RedemptionDefaultsFacet is an Asset Manager diamond facet that finalizes redemptions when agents fail to pay on the underlying chain in time. It verifies FDC attestations, pays redeemers from collateral (with premium), and cancels Core Vault transfers where applicable. Trust model: user funds are governed by the Asset Manager’s collateral rules; no privileged admin drain; only the redeemer, their executor, the agent (or anyone after a timeout for Core Vault transfers) can trigger defaults. Major entrypoints: redemptionPaymentDefault (with nonpayment proof) and finishRedemptionWithoutPayment (late, after proof window expiry). ReentrancyGuard mitigates reentrancy.

Storage variables

- ReentrancyGuard._status — reentrancy state flag

Functions

- function redemptionPaymentDefault(IReferencedPaymentNonexistence.Proof calldata _proof, uint256 _redemptionRequestId) external nonReentrant
  // Default redemption using FDC nonpayment proof; compensate redeemer, handle rewards, mark request DEFAULTED.

- function finishRedemptionWithoutPayment(IConfirmedBlockHeightExists.Proof calldata _proof, uint256 _redemptionRequestId) external nonReentrant
  // Finish active redemption without payment after proof window expired; burn executor fee; mark DEFAULTED.

- function _othersCanConfirmDefault(Redemption.Request storage _request) private view returns (bool)
  // True if anyone may confirm default for Core Vault transfers after timeout.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/RedemptionRequestsFacet.sol
# RedemptionRequestsFacet (Asset Manager Diamond Facet)

Purpose: Manages redemption flows for FAssets: user-initiated redemptions, pool-assisted redemptions, self-close for agents, and dust management. User funds are burned on redeem; agents pay underlying or, on default, collateral. Trust model: no admin balances; governance can emergency-pause via modifier; agent/pool permissions enforced. Major entrypoints: redeem, redeemFromAgent, redeemFromAgentInCollateral, selfClose, rejectInvalidRedemption, maxRedemptionFromAgent, convertDustToTicket.

Storage
- None (uses diamond storage via libraries)

Custom Errors
- SelfCloseOfZero()
- AddressValid()
- WrongAddress()
- InvalidRedemptionStatus()
- RedemptionOfZero()
- RedeemZeroLots()

Functions

- interface
  function redeem(uint256 _lots, string memory _redeemerUnderlyingAddressString, address payable _executor)
    external payable notEmergencyPaused nonReentrant
    returns (uint256 _redeemedAmountUBA);
  natspec: Burn FAssets and create per-agent redemption requests; split executor fee; partial fills allowed.

- interface
  function redeemFromAgent(address _agentVault, address _receiver, uint256 _amountUBA, string memory _receiverUnderlyingAddress, address payable _executor)
    external payable notEmergencyPaused nonReentrant;
  natspec: Pool-only flow: close tickets for one agent, create request, burn FAssets, pass executor fee.

- interface
  function redeemFromAgentInCollateral(address _agentVault, address _receiver, uint256 _amountUBA)
    external notEmergencyPaused nonReentrant;
  natspec: Pool-only: close tickets and pay receiver in vault collateral at agent-defined discount.

- interface
  function maxRedemptionFromAgent(address _agentVault)
    external view
    returns (uint256);
  natspec: Compute max single-operation redemption from agent considering ticket cap and agent dust.

- interface
  function rejectInvalidRedemption(IAddressValidity.Proof calldata _proof, uint256 _redemptionRequestId)
    external nonReentrant;
  natspec: Agent proves redeemer address invalid; releases collateral, burns executor fee, marks request rejected.

- interface
  function selfClose(address _agentVault, uint256 _amountUBA)
    external notEmergencyPaused nonReentrant onlyAgentVaultOwner(_agentVault)
    returns (uint256 _closedAmountUBA);
  natspec: Agent burns own FAssets to unlock collateral; may end liquidation; emits SelfClose.

- interface
  function convertDustToTicket(address _agentVault)
    external nonReentrant;
  natspec: Anyone can convert agent dust ≥ 1 lot into a new redemption ticket.

- interface (internal)
  function _redeemFirstTicket(uint256 _lots, RedemptionRequests.AgentRedemptionList memory _list)
    private
    returns (uint256 _redeemedLots);
  natspec: Helper to consume from queue head, aggregate per agent, and update tickets/dust.

Notes
- Executor fee: msg.value is split across created requests (denominated in gwei of native).
- Caps: maxRedeemedTickets from Globals limits work per call.
- Events: RedemptionRequestIncomplete, RedeemedInCollateral, SelfClose, RedemptionRejected emitted in relevant paths.
- Permissions: pool-only via Agents.requireCollateralPool; agent-only via onlyAgentVaultOwner/Agents.requireAgentVaultOwner.
- Pause: notEmergencyPaused gates user-flow functions; rejectInvalidRedemption intentionally callable during pause.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/RedemptionTimeExtensionFacet.sol
Summary
RedemptionTimeExtensionFacet is a Diamond facet of the Asset Manager that governs the extra time added per redemption payment. Admin-controlled via AssetManagerController and protected by SettingsUpdater time windows; it manages configuration only and never holds user funds. Major entrypoints: initRedemptionTimeExtensionFacet (one-time facet init), setRedemptionPaymentExtensionSeconds (admin update with bounds), redemptionPaymentExtensionSeconds (read).

Storage
- RedemptionTimeExtension.redemptionPaymentExtensionSeconds (uint256) – Extra secs per redemption
- LibDiamond.DiamondStorage.supportedInterfaces (mapping(bytes4=>bool)) – ERC165 support map
- AssetManagerSettings.Data.averageBlockTimeMS (uint256) – Underlying avg block ms
- SettingsUpdater (internal timestamp(s)) – Last settings update

Functions
- interface: constructor() nonpayable
  @notice Initialize impl-only storage so facet cannot be reinitialized directly.

- interface: function initRedemptionTimeExtensionFacet(uint256 _redemptionPaymentExtensionSeconds) external nonpayable
  @notice One-time facet init; registers ERC165 and sets extension seconds; reverts if diamond uninitialized.

- interface: function setRedemptionPaymentExtensionSeconds(uint256 _value) external onlyAssetManagerController nonpayable
  @notice Admin updates extension; bounded by ±4x limits and nonzero; time-locked; emits SettingChanged.

- interface: function redemptionPaymentExtensionSeconds() external view returns (uint256)
  @notice Read current redemption payment extension seconds.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/SettingsManagementFacet.sol
Summary
SettingsManagementFacet is an admin-only (controller-governed) Diamond facet for the FAssets AssetManager. It does not custody user funds; instead, it updates critical system addresses, parameters, and token/proxy configurations. Trust model: onlyAssetManagerController may call; most setters are rateLimited to throttle changes. Major entrypoints include contract/address updates, fee/CR/time-window tuning, lot/minting caps, liquidation curves, emergency pause windows, and FAsset proxy upgrades.

Storage
- UPDATES_STATE_POSITION (bytes32): updater state slot
- settings.assetManagerController: controller addr
- settings.agentOwnerRegistry: owner-registry addr
- settings.agentVaultFactory: agent vault factory
- settings.collateralPoolFactory: pool factory
- settings.collateralPoolTokenFactory: pool token factory
- settings.priceReader: price feed reader
- settings.fdcVerification: FDC verifier addr
- settings.underlyingBlocksForPayment: mint/redemption blocks
- settings.underlyingSecondsForPayment: payment seconds
- settings.paymentChallengeRewardUSD5: challenge reward (NAT wei)
- settings.paymentChallengeRewardBIPS: challenge reward bips
- settings.minUpdateRepeatTimeSeconds: min update cadence
- settings.lotSizeAMG: lot size (AMG)
- settings.maxTrustedPriceAgeSeconds: price age limit
- settings.collateralReservationFeeBIPS: CRF bips
- settings.redemptionFeeBIPS: redemption fee bips
- settings.redemptionDefaultFactorVaultCollateralBIPS: default factor vault
- settings.confirmationByOthersAfterSeconds: 3rd-party confirm delay
- settings.confirmationByOthersRewardUSD5: 3rd-party confirm reward
- settings.maxRedeemedTickets: max tickets per redemption
- settings.withdrawalWaitMinSeconds: withdraw/destroy timelock
- settings.attestationWindowSeconds: FDC proof window
- settings.averageBlockTimeMS: avg underlying block ms
- settings.mintingPoolHoldingsRequiredBIPS: required CPT stake
- settings.mintingCapAMG: minting cap (AMG)
- settings.tokenInvalidationTimeMinSeconds: token invalidation delay
- settings.vaultCollateralBuyForFlareFactorBIPS: FLR buy factor
- settings.agentExitAvailableTimelockSeconds: exit-available TL
- settings.agentFeeChangeTimelockSeconds: fee-change TL
- settings.agentMintingCRChangeTimelockSeconds: mintingCR-change TL
- settings.poolExitCRChangeTimelockSeconds: pool exit TL
- settings.agentTimelockedOperationWindowSeconds: TL exec window
- settings.collateralPoolTokenTimelockSeconds: CPT timelock
- settings.liquidationStepSeconds: liquidation step seconds
- settings.liquidationCollateralFactorBIPS[]: liq. factors (pool)
- settings.liquidationFactorVaultCollateralBIPS[]: liq. factors (vault)
- settings.maxEmergencyPauseDurationSeconds: max pause duration
- settings.emergencyPauseDurationResetAfterSeconds: pause reset window
- CollateralTypes.poolWNat (via set): pool collateral token
- FAsset.cleanerContract (via set): fAsset cleaner addr
- FAsset.cleanupBlockNumberManager (via set): cleanup manager
- FAsset.proxy implementation (via upgrade): fAsset impl

Functions
- function updateSystemContracts(address _controller, IWNat _wNat) external onlyAssetManagerController nonpayable
  Sets controller and updates pool WNat collateral type.

- function setAgentOwnerRegistry(address _value) external onlyAssetManagerController rateLimited nonpayable
  Set AgentOwnerRegistry contract address.

- function setAgentVaultFactory(address _value) external onlyAssetManagerController rateLimited nonpayable
  Set AgentVaultFactory contract address.

- function setCollateralPoolFactory(address _value) external onlyAssetManagerController rateLimited nonpayable
  Set CollateralPool factory address.

- function setCollateralPoolTokenFactory(address _value) external onlyAssetManagerController rateLimited nonpayable
  Set CollateralPoolToken factory address.

- function setPriceReader(address _value) external onlyAssetManagerController rateLimited nonpayable
  Set price reader/FTSO adapter contract address.

- function setFdcVerification(address _value) external onlyAssetManagerController rateLimited nonpayable
  Set FDC verification contract address.

- function setCleanerContract(address _value) external onlyAssetManagerController rateLimited nonpayable
  Set fAsset cleaner contract in token.

- function setCleanupBlockNumberManager(address _value) external onlyAssetManagerController rateLimited nonpayable
  Set fAsset cleanup block number manager in token.

- function upgradeFAssetImplementation(address _value, bytes memory callData) external onlyAssetManagerController rateLimited nonpayable
  Upgrade fAsset proxy implementation, optional init call.

- function setTimeForPayment(uint256 _underlyingBlocks, uint256 _underlyingSeconds) external onlyAssetManagerController rateLimited nonpayable
  Configure payment windows in blocks and seconds.

- function setPaymentChallengeReward(uint256 _rewardNATWei, uint256 _rewardBIPS) external onlyAssetManagerController rateLimited nonpayable
  Set challenge reward amount and bips with bounds.

- function setMinUpdateRepeatTimeSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set minimum time between governance updates.

- function setLotSizeAmg(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Adjust lot size (AMG) within 10x/0.1x bounds.

- function setMaxTrustedPriceAgeSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set trusted price age, bounded by 0.5x–2x.

- function setCollateralReservationFeeBips(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set CRF fee in bips with 0.25x–4x bounds.

- function setRedemptionFeeBips(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set redemption fee bips with 0.25x–4x bounds.

- function setRedemptionDefaultFactorVaultCollateralBIPS(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set vault default factor with ~0.8333x–1.2x bounds.

- function setConfirmationByOthersAfterSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set delay for third-party confirmations (>=2 hours).

- function setConfirmationByOthersRewardUSD5(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set reward for third-party confirmations with bounds.

- function setMaxRedeemedTickets(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set max tickets processed per redemption request.

- function setWithdrawalOrDestroyWaitMinSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set withdraw/destroy wait; cap increases by +10 minutes.

- function setAttestationWindowSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set FDC proof availability window (>= 1 day).

- function setAverageBlockTimeMS(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set average underlying block time in milliseconds.

- function setMintingPoolHoldingsRequiredBIPS(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set required CPT stake bips for minting limit.

- function setMintingCapAmg(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set minting cap (0 or >= lot size).

- function setTokenInvalidationTimeMinSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set token deprecation invalidation minimum time.

- function setVaultCollateralBuyForFlareFactorBIPS(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set FLR-for-collateral buy factor (>= 10000 bips).

- function setAgentExitAvailableTimelockSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set time between exit announce and execute; bounded increase.

- function setAgentFeeChangeTimelockSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set fee change timelock; bounded increase.

- function setAgentMintingCRChangeTimelockSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set minting CR change timelock; bounded increase.

- function setPoolExitCRChangeTimelockSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set pool exit/top-up settings timelock; bounded increase.

- function setAgentTimelockedOperationWindowSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set window to execute after timelock (>= 1 hour).

- function setCollateralPoolTokenTimelockSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set CPT entry timelock (>= 1 minute).

- function setLiquidationStepSeconds(uint256 _stepSeconds) external onlyAssetManagerController rateLimited nonpayable
  Set liquidation premium step duration with bounds.

- function setLiquidationPaymentFactors(uint256[] memory _liquidationFactors, uint256[] memory _vaultCollateralFactors) external onlyAssetManagerController rateLimited nonpayable
  Set liquidation curves (pool and vault factors arrays).

- function setMaxEmergencyPauseDurationSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set maximum emergency pause duration with bounds.

- function setEmergencyPauseDurationResetAfterSeconds(uint256 _value) external onlyAssetManagerController rateLimited nonpayable
  Set pause duration reset window with bounds.

Notes
- All setters emit ContractChanged/SettingChanged/SettingArrayChanged.
- Inputs are tightly bounded; many revert with custom errors on unsafe changes.
- rateLimited enforces minUpdateRepeatTimeSeconds between successive updates.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/SettingsReaderFacet.sol
# SettingsReaderFacet.sol — Summary
SettingsReaderFacet is a read-only diamond facet exposing Asset Manager configuration and key system addresses. It neither holds nor moves user funds and defines no state; all data is read from Globals (diamond storage). Trust model: parameters are governed by AssetManagerController; this facet only reads them. Major entrypoints: getSettings, fAsset, priceReader, lotSize, assetMintingGranularityUBA, assetMintingDecimals, assetManagerController, getCollateralPoolTokenTimelockSeconds.

## Storage Variables
- None — no state defined in this facet

## Functions
- function getSettings() external pure returns (AssetManagerSettings.Data memory)
  — Return full current AssetManager settings from diamond Globals storage.

- function fAsset() external view returns (IERC20)
  — Return IERC20 of the managed FAsset token.

- function priceReader() external view returns (address)
  — Return address of the price reader used by this asset manager.

- function lotSize() external view returns (uint256 _lotSizeUBA)
  — Compute lot size in UBA: lotSizeAMG * assetMintingGranularityUBA.

- function assetMintingGranularityUBA() external view returns (uint256)
  — Return AMG expressed in UBA units.

- function assetMintingDecimals() external view returns (uint256)
  — Return ERC-20 decimals used for minting granularity math.

- function assetManagerController() external view returns (address)
  — Return address authorized to change system settings.

- function getCollateralPoolTokenTimelockSeconds() external view returns (uint256)
  — Return timelock seconds for CPT transfer/exit post-entry.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/SystemInfoFacet.sol
# SystemInfoFacet — Read-only view facet for Asset Manager

Summary (<=100 words)
This diamond facet exposes read-only system queries for the FAssets Asset Manager. It does not hold funds or perform admin actions; it only reads diamond storage and aggregates data from libraries. Main entrypoints: controllerAttached, mintingPaused, redemptionQueue, agentRedemptionQueue, collateralReservationInfo, redemptionRequestInfo. Trust model: view-only; no state changes; safe to call by anyone.

Storage (read) touched via libraries
- AssetManagerState.attached — controller added
- AssetManagerState.mintingPausedAt — pause timestamp
- CollateralReservation.Data.agentVault — agent vault
- CollateralReservation.Data.minter — minter addr
- CollateralReservation.Data.valueAMG — mint value AMG
- CollateralReservation.Data.underlyingFeeUBA — mint fee UBA
- CollateralReservation.Data.reservationFeeNatWei — CRF in nat
- CollateralReservation.Data.poolFeeShareBIPS — pool share bips
- CollateralReservation.Data.firstUnderlyingBlock — first block
- CollateralReservation.Data.lastUnderlyingBlock — last block
- CollateralReservation.Data.lastUnderlyingTimestamp — last ts
- CollateralReservation.Data.executor — executor addr
- CollateralReservation.Data.executorFeeNatGWei — exec fee gwei
- CollateralReservation.Data.status — CRT status
- Agent.State.underlyingAddressString — agent pay addr
- Agent.State.poolFeeShareBIPS — agent pool share
- Redemption.Request.status — redemption status
- Redemption.Request.agentVault — agent vault
- Redemption.Request.redeemer — redeemer addr
- Redemption.Request.redeemerUnderlyingAddressString — redeemer addr str
- Redemption.Request.underlyingValueUBA — value UBA
- Redemption.Request.underlyingFeeUBA — fee UBA
- Redemption.Request.poolFeeShareBIPS — pool share bips
- Redemption.Request.firstUnderlyingBlock — first block
- Redemption.Request.lastUnderlyingBlock — last block
- Redemption.Request.lastUnderlyingTimestamp — last ts
- Redemption.Request.timestamp — req timestamp
- Redemption.Request.poolSelfClose — pool self-close
- Redemption.Request.transferToCoreVault — to core vault
- Redemption.Request.executor — executor addr
- Redemption.Request.executorFeeNatGWei — exec fee gwei
- RedemptionQueue (via RedemptionQueueInfo) — ticket queue

Functions
- function controllerAttached() external view returns (bool);
  - Natspec: True if asset manager attached to controller.

- function mintingPaused() external view returns (bool);
  - Natspec: True if minting is globally paused.

- function redemptionQueue(uint256 _firstRedemptionTicketId, uint256 _pageSize) external view returns (RedemptionTicketInfo.Data[] memory _queue, uint256 _nextRedemptionTicketId);
  - Natspec: Page through global redemption ticket queue.

- function agentRedemptionQueue(address _agentVault, uint256 _firstRedemptionTicketId, uint256 _pageSize) external view returns (RedemptionTicketInfo.Data[] memory _queue, uint256 _nextRedemptionTicketId);
  - Natspec: Page through an agent's redemption tickets.

- function collateralReservationInfo(uint256 _collateralReservationId) external view returns (CollateralReservationInfo.Data memory);
  - Natspec: Returns collateral reservation details by id.

- function redemptionRequestInfo(uint256 _redemptionRequestId) external view returns (RedemptionRequestInfo.Data memory);
  - Natspec: Returns redemption request details by id.

- function _convertCollateralReservationStatus(CollateralReservation.Status _status) private pure returns (CollateralReservationInfo.Status);
  - Natspec: Map internal CRT status to UI type.

- function _convertRedemptionStatus(Redemption.Status _status) private pure returns (RedemptionRequestInfo.Status);
  - Natspec: Map internal redemption status to UI type.

Notes
- No modifiers; all public functions are external view; helpers are private pure.
- Depends on Conversion and PaymentReference libs to compute values/refs.
- All addresses/amounts are returned as-is from storage or transformed for UI structs.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/SystemStateManagementFacet.sol
## Summary
SystemStateManagementFacet is a diamond facet controlling AssetManager operational state. It holds no user funds; it only toggles governance-controlled flags. All entrypoints are restricted to the AssetManagerController (onlyAssetManagerController). Major entrypoints: attachController(bool) to mark the manager attached to its controller, pauseMinting() to pause new minting, and unpauseMinting() to resume. SafeCast is used to store timestamps safely.

## Storage
- AssetManagerState.State.attached — controller linked
- AssetManagerState.State.mintingPausedAt — pause start time

## Functions
- function attachController(bool attached) external onlyAssetManagerController nonpayable;
  - NatSpec: Set controller attachment flag; gates agent creation and minting.

- function pauseMinting() external onlyAssetManagerController nonpayable;
  - NatSpec: Pause new minting by setting mintingPausedAt once.

- function unpauseMinting() external onlyAssetManagerController nonpayable;
  - NatSpec: Resume minting by clearing mintingPausedAt.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/UnderlyingBalanceFacet.sol
# UnderlyingBalanceFacet (Facet of AssetManager Diamond)

Purpose: Maintains and attests agent underlying-chain balances. It confirms top-ups sent to an agent’s underlying address, enforces announce/confirm flow for withdrawals, and pays third-party confirmers after timeout. Trust model: No user funds are held here; it updates accounting for agents. Only the agent vault owner can act, except anyone may confirm a withdrawal after a governance-defined delay. Major entrypoints: confirmTopupPayment, announceUnderlyingWithdrawal, confirmUnderlyingWithdrawal, cancelUnderlyingWithdrawal.

Storage (via library-backed diamond storage)
- state.newPaymentAnnouncementId – Monotonic id counter
- state.paymentConfirmations – Payment proofs book
- agent.underlyingAddressHash – Agent L1 addr hash
- agent.underlyingBlockAtCreation – Agent created L1 block
- agent.announcedUnderlyingWithdrawalId – Active withdraw id
- agent.underlyingWithdrawalAnnouncedAt – Announce timestamp
- settings.confirmationByOthersAfterSeconds – Third-party confirm delay

Functions
- function confirmTopupPayment(IPayment.Proof calldata _payment, address _agentVault) external onlyAgentVaultOwner(_agentVault)
  Summary: Confirms underlying top-up and increases agent free balance; updates current underlying block.

- function announceUnderlyingWithdrawal(address _agentVault) external onlyAgentVaultOwner(_agentVault)
  Summary: Opens a withdrawal announcement and emits reference required for legal underlying withdrawal.

- function confirmUnderlyingWithdrawal(IPayment.Proof calldata _payment, address _agentVault) external nonReentrant
  Summary: Confirms announced withdrawal; adjusts free balance, clears announcement, may reward third-party confirmer.

- function cancelUnderlyingWithdrawal(address _agentVault) external onlyAgentVaultOwner(_agentVault)
  Summary: Cancels active withdrawal announcement to reset timing and allow re-announce.

Notes
- Emits: UnderlyingBalanceToppedUp, UnderlyingWithdrawalAnnounced, UnderlyingWithdrawalConfirmed, UnderlyingWithdrawalCancelled.
- Validates FDC proofs and references; guards reentrancy on confirmation path; enforces source/destination and timing rules.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/facets/UnderlyingTimekeepingFacet.sol
# UnderlyingTimekeepingFacet — Summary
A minimal Asset Manager facet that tracks the latest confirmed block height and timestamp of the underlying chain (via FDC proofs). It holds no funds and has no admin-only paths. Anyone can update the tracked underlying block to keep minting/redemption time windows fair. Entrypoints: updateCurrentBlock (submit proof) and currentUnderlyingBlock (read currently tracked underlying block info).

## Storage (used via AssetManagerState)
- currentUnderlyingBlock — Latest underlying block no.
- currentUnderlyingBlockTimestamp — Latest underlying block ts
- currentUnderlyingBlockUpdatedAt — L1 ts when last updated

Note: Variables are part of shared diamond state (AssetManagerState.State) and are updated through UnderlyingBlockUpdater.

## Functions

- function updateCurrentBlock(IConfirmedBlockHeightExists.Proof calldata _proof) external
  - visibility: external; mutability: nonpayable; modifiers: none
  - natspec: Submit FDC proof to advance tracked underlying block and timestamp.

- function currentUnderlyingBlock() external view returns (uint256 _blockNumber, uint256 _blockTimestamp, uint256 _lastUpdateTs)
  - visibility: external; mutability: view; modifiers: none
  - natspec: Read current tracked underlying block number, timestamp, and last update time.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/implementation/AssetManager.sol
# AssetManager (Diamond)

AssetManager is the root diamond for a single F-Asset type. It delegates all business logic (minting, redemption, collateral, liquidation, settings) to registered facets. Trust model: user value is held in agent vaults and collateral pools; this contract itself doesn’t custody funds. Governance/owner controls upgrades via diamond cut. Major entrypoints are exposed by facets such as MintingFacet, Redemption* facets, Collateral* facets, LiquidationFacet, Settings* facets, EmergencyPause* facets, and CoreVault* facets.

## Storage Variables
- None declared in this contract (uses Diamond storage via LibDiamond)

## Functions

- constructor(IDiamondCut.FacetCut[] memory _diamondCut, address _init, bytes memory _initCalldata) payable
  - NatSpec: Deploys and initializes the diamond by executing a diamond cut and optional initializer call.

Notes:
- Events are defined via IAssetManagerEvents interface; decoding works on block explorers.
- All operational entrypoints live in facets registered through the diamond cut.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/mock/MaliciousDistributionToDelegators.sol
MaliciousDistributionToDelegators.sol

Summary
A minimal adversarial/mock contract that simulates a delegator reward distributor by always returning a fixed amount from claim(). It ignores all inputs, holds no user funds, has no admin or access control, and performs no transfers or state changes beyond construction. Intended for integration or security testing against malicious behaviors. Major entrypoints: constructor(uint256 _claim), claim(address,address,uint256,bool), and the autogenerated amount() getter.

Storage
- amount (uint256) — fixed reward amt

Functions
- constructor(uint256 _claim) nonpayable
  • Sets fixed reward amount returned by claim.

- function amount() external view returns (uint256)
  • Returns stored fixed reward amount.

- function claim(address _rewardOwner, address _recipient, uint256 _month, bool _wrap) external returns (uint256 _rewardAmount)
  • Returns preset amount; ignores inputs; no state changes.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/mock/MaliciousExecutor.sol
MaliciousExecutor.sol – Summary
A minimal, adversarial helper contract to probe reentrancy around AssetManager.redemptionPaymentDefault. It stores a referenced nonexistence proof and redemption request ID, then reenters the AssetManager from its payable fallback when “trigger” is set. No admin/owner and no custody of user funds beyond any native tokens sent to it. Major entrypoints: defaulting, fallback (payable), howMuchIsMyNativeBalance.

Storage
- diamond: address immutable – Target IAssetManager diamond
- tempProof: IReferencedPaymentNonexistence.Proof – Stashed proof for reentry
- tempRequestId: uint256 – Stashed redemption request id
- hit: uint256 – Simple reentry flag
- trigger: uint256 – Arms fallback reentry

Functions
- constructor(address _diamond) public nonpayable
  NatSpec: Set the target AssetManager diamond address.

- function defaulting(IReferencedPaymentNonexistence.Proof calldata _proof, uint256 _redemptionRequestId, uint256 _trigger) external nonpayable
  NatSpec: Store proof and request, set trigger, then call redemptionPaymentDefault.

- function howMuchIsMyNativeBalance() external view returns (uint256)
  NatSpec: Return this contract’s native token balance.

- fallback() external payable
  NatSpec: If armed, reenter redemptionPaymentDefault once during payable fallback.



## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/mock/MaliciousMintExecutor.sol
MaliciousMintExecutor — Summary
A test/attack helper contract that front-runs or reenters the AssetManager mint flow to snapshot agent state, drain the minter’s approved FAssets, and immediately trigger and execute liquidation on a targeted agent vault. Trust model: no admin; execution fully controlled by deployer. User funds risk if the minter has granted IFAsset allowance. Major entrypoints: mint (executes minting on AssetManager) and fallback (calls proceed to snapshot and liquidate).

Storage
- diamond (address, immutable) — AssetManager diamond
- agentVault (address, immutable) — Target agent vault
- minter (address, immutable) — Minter to drain
- fasset (address, immutable) — IFAsset token
- liquidationStartedTs (uint256) — startLiquidation ts
- reserved (uint256) — reserved UBA snap
- minted (uint256) — minted UBA snap
- poolCR (uint256) — pool CR BIPS snap
- vaultCR (uint256) — vault CR BIPS snap

Functions
- constructor(address _diamond, address _agentVault, address _minter, address _fasset) visibility: implicit (constructor), mutability: nonpayable, modifiers: none
  Natspec: Initialize manager, agent vault, minter, and FAsset addresses.
- mint(IPayment.Proof calldata _proof, uint256 _collateralReservationId) external nonpayable modifiers: none
  Natspec: Execute minting on AssetManager using provided proof and reservation id.
- fallback() external payable modifiers: none
  Natspec: Reentry hook; triggers internal proceed to snapshot and liquidate.
- proceed() internal nonpayable modifiers: none
  Natspec: Snapshot agent metrics, pull minter FAssets, start and execute liquidation.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManager/mock/MaliciousRewardManager.sol
Summary
MaliciousRewardManager is a tiny mock/attack helper that mimics IRewardManager.claim. It holds no funds and has no admin controls; trust model is stateless aside from a single stored amount. Its claim(...) entrypoint always returns a fixed, constructor-set amount, ignoring all inputs. Entrypoints: constructor(uint256 _claim) to configure the fixed return; claim(...) external to return it. Useful for testing consumers’ validation logic against dishonest reward managers.

Storage
- amount (uint256) — fixed claim amount

Functions
- constructor(uint256 _claim) nonpayable; modifiers: none
  NatSpec: Sets the fixed reward amount returned by claim.

- function claim(
    address _rewardOwner,
    address payable _recipient,
    uint24 _rewardEpochId,
    bool _wrap,
    IRewardManager.RewardClaimWithProof[] calldata _proofs
  ) external nonpayable returns (uint256 _rewardAmountWei);
  NatSpec: Ignores inputs and returns preset amount; used for testing/attack simulations.

Notes
- Imports IRewardManager only for the RewardClaimWithProof type. No events, no access control, no fund custody.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManagerController/implementation/AssetManagerController.sol
Summary
This UUPS-upgradeable controller orchestrates multiple AssetManager instances under governance. It doesn’t custody user funds; it acts as an admin/ops router for settings, collateral types, upgrades, emergency pauses, and system address updates. Trust model: governance (and optionally authorized emergency pause senders) can modify parameters and pause minting/transfers. Major entrypoints: add/remove asset managers, batch setting setters, upgradeTo/upgradeToAndCall, emergencyPause/emergencyPauseTransfers, updateContracts, and collateral-type management.

Storage
- replacedBy: address — successor controller
- assetManagerIndex: mapping(address=>uint256) — 1-based index map
- assetManagers: IIAssetManager[] — managed AM list
- emergencyPauseSenders: EnumerableSet.AddressSet — authorized pausers

Functions
- constructor() public nonpayable
  Natspec: Initialize base parents; leaves configuration to initialize().

- function initialize(IGovernanceSettings _governanceSettings, address _initialGovernance, address _addressUpdater) external nonpayable
  Natspec: One-time proxy initializer: set governance and AddressUpdater.

- function addAssetManager(IIAssetManager _assetManager) external onlyGovernance nonpayable
  Natspec: Register AM and attach if it already points to this controller.

- function removeAssetManager(IIAssetManager _assetManager) external onlyGovernance nonpayable
  Natspec: Unregister AM and detach if controlled by this controller.

- function getAssetManagers() external view returns (IAssetManager[] memory _assetManagers)
  Natspec: Return all managed AssetManager proxies.

- function assetManagerExists(address _assetManager) external view returns (bool)
  Natspec: Check whether an address is a managed AssetManager.

- function upgradeTo(address newImplementation) public onlyGovernance onlyProxy nonpayable override(IUUPSUpgradeable, UUPSUpgradeable)
  Natspec: UUPS upgrade to new implementation without call data.

- function upgradeToAndCall(address newImplementation, bytes memory data) public payable onlyGovernance onlyProxy payable override(IUUPSUpgradeable, UUPSUpgradeable)
  Natspec: UUPS upgrade and call initializer on new implementation.

- function _authorizeUpgrade(address /* _newImplementation */) internal pure override
  Natspec: Unused; authorization enforced by onlyGovernance on upgrade functions.

- function setAgentOwnerRegistry(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  Natspec: Set AgentOwnerRegistry on target AMs.

- function setAgentVaultFactory(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  Natspec: Set AgentVaultFactory on target AMs.

- function setCollateralPoolFactory(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  Natspec: Set CollateralPoolFactory on target AMs.

- function setCollateralPoolTokenFactory(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  Natspec: Set CollateralPoolTokenFactory on target AMs.

- function upgradeAgentVaultsAndPools(IIAssetManager[] memory _assetManagers, uint256 _start, uint256 _end) external onlyImmediateGovernance nonpayable
  Natspec: Batch-upgrade vault and pool implementations via AMs over index range.

- function setPriceReader(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  Natspec: Set price reader contract on target AMs.

- function setFdcVerification(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  Natspec: Set FDC verification contract on target AMs.

- function setCleanerContract(IIAssetManager[] memory _assetManagers, address _value) external onlyImmediateGovernance nonpayable
  Natspec: Set cleaner/maintenance contract on target AMs.

- function setCleanupBlockNumberManager(IIAssetManager[] memory _assetManagers, address _value) external onlyGovernance nonpayable
  Natspec: Set cleanup block-number manager on target AMs.

- function upgradeFAssetImplementation(IIAssetManager[] memory _assetManagers, address _implementation, bytes memory _callData) external onlyGovernance nonpayable
  Natspec: Upgrade FAsset proxy implementation via AMs, optional init call.

- function setMinUpdateRepeatTimeSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  Natspec: Set min governance update repeat time on AMs.

- function setLotSizeAmg(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  Natspec: Set lot size (AMG units) for mint/redeem on AMs.

- function setTimeForPayment(IIAssetManager[] memory _assetManagers, uint256 _underlyingBlocks, uint256 _underlyingSeconds) external onlyGovernance nonpayable
  Natspec: Configure mint/redemption payment windows on AMs.

- function setPaymentChallengeReward(IIAssetManager[] memory _assetManagers, uint256 _rewardVaultCollateralWei, uint256 _rewardBIPS) external onlyImmediateGovernance nonpayable
  Natspec: Set rewards for successful payment challenges on AMs.

- function setMaxTrustedPriceAgeSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set maximum trusted price age on AMs.

- function setCollateralReservationFeeBips(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set CRF fee in BIPS on AMs.

- function setRedemptionFeeBips(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set redemption fee in BIPS on AMs.

- function setRedemptionDefaultFactorVaultCollateralBIPS(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set vault-collateral share for redemption default premium.

- function setConfirmationByOthersAfterSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set delay before third-parties can confirm on AMs.

- function setConfirmationByOthersRewardUSD5(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set reward (USD*1e5) for third-party confirmations.

- function setMaxRedeemedTickets(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set max tickets processed per redemption request.

- function setWithdrawalOrDestroyWaitMinSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set min wait after announce before withdraw/destroy.

- function setAttestationWindowSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set time window for FDC attestation availability.

- function setAverageBlockTimeMS(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set average underlying block time in milliseconds.

- function setMintingPoolHoldingsRequiredBIPS(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set agent minimum CPT holdings required to mint.

- function setMintingCapAmg(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set system minting cap (AMG) on AMs.

- function setTokenInvalidationTimeMinSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  Natspec: Set token invalidation grace time for deprecated collateral.

- function setVaultCollateralBuyForFlareFactorBIPS(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  Natspec: Set factor for collateral-for-FLR buy price.

- function setAgentExitAvailableTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set timelock before agent exit from public availability.

- function setAgentFeeChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set timelock for agent fee/pool-share changes.

- function setAgentMintingCRChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set timelock for agent minting CR changes.

- function setPoolExitCRChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set timelock for pool exit/top-up settings.

- function setAgentTimelockedOperationWindowSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set execution window after a timelock expires.

- function setCollateralPoolTokenTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set CPT entry timelock duration.

- function setLiquidationStepSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  Natspec: Set liquidation step duration (premium step time).

- function setLiquidationPaymentFactors(IIAssetManager[] memory _assetManagers, uint256[] memory _paymentFactors, uint256[] memory _vaultCollateralFactors) external onlyGovernance nonpayable
  Natspec: Set liquidation payout and vault-collateral factor arrays.

- function setRedemptionPaymentExtensionSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyImmediateGovernance nonpayable
  Natspec: Set extra time per redemption granted to agents.

- function addCollateralType(IIAssetManager[] memory _assetManagers, CollateralType.Data calldata _data) external onlyImmediateGovernance nonpayable
  Natspec: Add a new collateral type across target AMs.

- function setCollateralRatiosForToken(IIAssetManager[] memory _assetManagers, CollateralType.Class _class, IERC20 _token, uint256 _minCollateralRatioBIPS, uint256 _safetyMinCollateralRatioBIPS) external onlyGovernance nonpayable
  Natspec: Update min and safety CR for given collateral token.

- function deprecateCollateralType(IIAssetManager[] memory _assetManagers, CollateralType.Class _class, IERC20 _token, uint256 _invalidationTimeSec) external onlyImmediateGovernance nonpayable
  Natspec: Deprecate a collateral type with invalidation delay.

- function pauseMinting(IIAssetManager[] calldata _assetManagers) external onlyImmediateGovernance nonpayable
  Natspec: Pause minting on target AMs; other ops continue.

- function unpauseMinting(IIAssetManager[] calldata _assetManagers) external onlyImmediateGovernance nonpayable
  Natspec: Unpause minting on target AMs.

- function supportsInterface(bytes4 _interfaceId) external pure returns (bool)
  Natspec: ERC-165 support for multiple interfaces including controller.

- function updateContracts(IIAssetManager[] calldata _assetManagers) external nonpayable
  Natspec: Pull latest addresses from AddressUpdater and propagate.

- function emergencyPause(IIAssetManager[] memory _assetManagers, uint256 _duration) external nonpayable
  Natspec: Emergency pause AMs; callable by governance or authorized senders.

- function emergencyPauseTransfers(IIAssetManager[] memory _assetManagers, uint256 _duration) external nonpayable
  Natspec: Emergency pause token transfers; gov or authorized senders.

- function resetEmergencyPauseTotalDuration(IIAssetManager[] memory _assetManagers) external onlyImmediateGovernance nonpayable
  Natspec: Reset accumulated emergency pause durations on AMs.

- function addEmergencyPauseSender(address _address) external onlyImmediateGovernance nonpayable
  Natspec: Authorize an additional emergency pause sender.

- function removeEmergencyPauseSender(address _address) external onlyImmediateGovernance nonpayable
  Natspec: Revoke an emergency pause sender.

- function setMaxEmergencyPauseDurationSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  Natspec: Set max single emergency pause duration.

- function setEmergencyPauseDurationResetAfterSeconds(IIAssetManager[] memory _assetManagers, uint256 _value) external onlyGovernance nonpayable
  Natspec: Set time after which pause duration counter resets.

Internal/Private Helpers
- function _updateContractAddresses(bytes32[] memory _contractNameHashes, address[] memory _contractAddresses) internal override nonpayable
  Natspec: AddressUpdater hook; parse addresses and propagate to AMs.

- function _updateContracts(IIAssetManager[] memory _assetManagers, address addressUpdater, address assetManagerController, address wNat) private nonpayable
  Natspec: Update state and push system contracts into AMs.

- function _setValueOnManagers(IIAssetManager[] memory _assetManagers, bytes4 _selector, address _value) private nonpayable
  Natspec: Batch-call AMs with single address parameter.

- function _setValueOnManagers(IIAssetManager[] memory _assetManagers, bytes4 _selector, uint256 _value) private nonpayable
  Natspec: Batch-call AMs with single uint256 parameter.

- function _callOnManagers(IIAssetManager[] memory _assetManagers, bytes memory _calldata) private nonpayable
  Natspec: Loop managed AMs and forward encoded call; checks membership.

Errors
- error AssetManagerNotManaged(); — Target not in managed set.
- error OnlyGovernanceOrEmergencyPauseSenders(); — Unauthorized pauser.
- error AddressZero(); — Required address missing.


## SUMMARY OF FILE: 2025-08-flare/contracts/assetManagerController/implementation/AssetManagerControllerProxy.sol
# AssetManagerControllerProxy (ERC1967Proxy)

Minimal ERC1967 UUPS-compatible proxy that forwards all calls to an AssetManagerController implementation. On deployment, it sets the implementation and atomically calls initialize(governanceSettings, initialGovernance, addressUpdater) on the logic. Trust model: upgrades and admin gating are handled by the implementation (UUPS with governance). The proxy itself holds no business logic and does not manage user funds beyond transient value; governance/admin can upgrade via the implementation’s UUPS flow. Major entrypoints are proxy fallback/receive delegating to AssetManagerController.

Storage Variables

- (none custom) — no contract-local storage
- implementation — logic impl addr
- admin — proxy admin addr
- beacon — beacon impl addr

Functions

- constructor(address _implementationAddress, IGovernanceSettings _governanceSettings, address _initialGovernance, address _addressUpdater) payable
  - Natspec: Deploy proxy and initialize AssetManagerController via delegatecall.

- fallback() external payable
  - Natspec: Delegates any unmatched call to current implementation.

- receive() external payable
  - Natspec: Accepts native token transfers; delegates if appropriate.



## SUMMARY OF FILE: 2025-08-flare/contracts/collateralPool/implementation/CollateralPool.sol
## CollateralPool (UUPS, ERC165)

Trustless pool holding wrapped native collateral (wNat/FLR) for an agent’s collateral pool. Users deposit NAT to mint pool tokens, accrue fAsset fee shares, and exit via standard or self-close paths. AssetManager (admin) sets params, upgrades, deposits, slashes on payouts, and destroys the pool. Agent (vault owner) delegates vote power and claims rewards. Major entrypoints: enter, exit/exitTo, selfCloseExit/selfCloseExitTo, withdrawFees/withdrawFeesTo, payFAssetFeeDebt, payout, destroy, upgradeWNatContract.

### Storage Variables
- agentVault (address) - agent vault addr
- assetManager (IIAssetManager) - system admin
- fAsset (IFAsset) - fee token
- token (IICollateralPoolToken) - pool token
- wNat (IWNat) - wrapped NAT
- exitCollateralRatioBIPS (uint32) - pool exit CR
- __topupCollateralRatioBIPS (uint32) - storage spacer
- __topupTokenPriceFactorBIPS (uint16) - storage spacer
- internalWithdrawal (bool) - nat unwrap guard
- initialized (bool) - init flag
- _fAssetFeeDebtOf (mapping(address=>int256)) - user fee debt
- totalFAssetFeeDebt (int256) - sum fee debt
- totalFAssetFees (uint256) - pooled fees
- totalCollateral (uint256) - wNat balance

### Functions
- constructor(address _agentVault, address _assetManager, address _fAsset, uint32 _exitCollateralRatioBIPS) public nonpayable
  Initializes in tests by delegating to initialize.
- initialize(address _agentVault, address _assetManager, address _fAsset, uint32 _exitCollateralRatioBIPS) public nonpayable
  One-time initializer; sets roles, tokens, wNat, CR.
- receive() external payable
  Accepts NAT only during internal withdrawals.
- setPoolToken(address _poolToken) external onlyAssetManager nonpayable
  One-time set of pool token contract.
- poolToken() external view returns (ICollateralPoolToken)
  Returns pool token interface.
- setExitCollateralRatioBIPS(uint256 _exitCollateralRatioBIPS) external onlyAssetManager nonpayable
  Updates pool exit collateral ratio.
- enter() external payable nonReentrant returns (uint256, uint256)
  Deposit NAT, mint pool tokens, accrue fee debt.
- exit(uint256 _tokenShare) external nonReentrant nonpayable returns (uint256)
  Burn tokens, withdraw proportional NAT.
- exitTo(uint256 _tokenShare, address payable _recipient) external nonReentrant nonpayable returns (uint256)
  Exit and send NAT/fees to recipient.
- _exitTo(uint256 _tokenShare, address payable _recipient) private nonpayable returns (uint256)
  Internal exit flow with CR and minimums.
- selfCloseExit(uint256 _tokenShare, bool _redeemToCollateral, string memory _redeemerUnderlyingAddress, address payable _executor) external payable nonReentrant
  Exit and redeem required fAssets to preserve CR.
- selfCloseExitTo(uint256 _tokenShare, bool _redeemToCollateral, address payable _recipient, string memory _redeemerUnderlyingAddress, address payable _executor) external payable nonReentrant
  Self-close exit, sending proceeds to recipient.
- _selfCloseExitTo(uint256 _tokenShare, bool _redeemToCollateral, address payable _recipient, string memory _redeemerUnderlyingAddress, address payable _executor) private nonpayable
  Internal self-close; redeems fAssets or uses collateral.
- fAssetRequiredForSelfCloseExit(uint256 _tokenAmountWei) external view returns (uint256)
  Computes fAssets needed to keep CR.
- withdrawFees(uint256 _fAssets) external nonReentrant nonpayable
  Withdraw accrued fAsset fees; adds fee debt.
- withdrawFeesTo(uint256 _fAssets, address _recipient) external nonReentrant nonpayable
  Withdraw fee share to recipient; debt increases.
- _withdrawFeesTo(uint256 _fAssets, address _recipient) private nonpayable
  Internal fee withdrawal with checks and transfers.
- payFAssetFeeDebt(uint256 _fAssets) external nonReentrant nonpayable
  Repay fee debt with fAssets.
- payout(address _recipient, uint256 _amount, uint256 _agentResponsibilityWei) external onlyAssetManager nonReentrant nonpayable
  Slash agent’s pool tokens and transfer wNat.
- _collateralToTokenShare(uint256 _collateral) internal view returns (uint256)
  Convert NAT to token share at pool price.
- _tokensToVirtualFeeShare(uint256 _tokens) internal view returns (uint256)
  Proportional share of virtual fees.
- _getFAssetRequiredToNotSpoilCR(uint256 _natShare) internal view returns (uint256)
  fAssets needed so CR not reduced below limit.
- _staysAboveExitCR(uint256 _withdrawnNat) internal view returns (bool)
  Check pool stays above exit CR post-withdrawal.
- _isAboveCR(AssetPrice memory _assetPrice, uint256 _backedFAssets, uint256 _poolCollateralNat, uint256 _crBIPS) internal pure returns (bool)
  CR comparison helper with price ratio.
- _agentBackedFAssets() internal view returns (uint256)
  fAssets backed by this pool for agent.
- _virtualFAssetFeesOf(address _account) internal view returns (uint256)
  User’s pro-rata fee share (virtual).
- _fAssetFeesOf(address _account) internal view returns (uint256)
  User’s withdrawable fAsset fees.
- _debtFreeTokensOf(address _account) internal view returns (uint256)
  Token amount transferable (not debt-locked).
- _getAssetPrice() internal view returns (AssetPrice memory)
  Get fAsset price in NAT (mul/div).
- _totalVirtualFees() internal view returns (uint256)
  Total fees plus outstanding fee debt.
- _safeExitCR() internal view returns (uint256)
  Max of min pool CR and exit CR.
- _requireMinTokenSupplyAfterExit(uint256 _tokenShare) internal view nonpayable
  Enforce minimum token supply post-exit.
- _requireMinNatSupplyAfterExit(uint256 _natShare) internal view nonpayable
  Enforce minimum NAT balance post-exit.
- depositNat() external payable onlyAssetManager nonReentrant
  Wrap incoming NAT to wNat and update.
- fAssetFeeDeposited(uint256 _amount) external onlyAssetManager nonpayable
  Account fAsset fees deposited by AssetManager.
- _createFAssetFeeDebt(address _account, uint256 _fAssets) internal nonpayable
  Increase user and total fee debt.
- _deleteFAssetFeeDebt(address _account, uint256 _fAssets) internal nonpayable
  Decrease user and total fee debt.
- _transferFAssetFrom(address _from, uint256 _amount) internal nonpayable
  Pull fAssets into pool; increment fees.
- _transferFAssetTo(address _to, uint256 _amount) internal nonpayable
  Send fAssets to user; decrement fees.
- _transferWNatTo(address _to, uint256 _amount) internal nonpayable
  Send wNat and reduce totalCollateral.
- _withdrawWNatTo(address payable _recipient, uint256 _amount) internal nonpayable
  Unwrap wNat and send NAT to recipient.
- _depositWNat() internal payable nonpayable
  Deposit msg.value to wNat; update collateral.
- virtualFAssetOf(address _account) external view returns (uint256)
  Expose virtual fee share for user.
- fAssetFeesOf(address _account) external view returns (uint256)
  Expose withdrawable fee share for user.
- fAssetFeeDebtOf(address _account) external view returns (int256)
  Expose user’s fee debt.
- debtLockedTokensOf(address _account) external view returns (uint256)
  Tokens locked by fee debt.
- debtFreeTokensOf(address _account) external view returns (uint256)
  Tokens free of fee debt.
- destroy(address payable _recipient) external onlyAssetManager nonReentrant nonpayable
  Drain residual assets; only when no tokens.
- upgradeWNatContract(IWNat _newWNat) external onlyAssetManager nonReentrant nonpayable
  Migrate balances to new wNat contract.
- delegate(address _to, uint256 _bips) external onlyAgent nonpayable
  Delegate wNat vote power by bips.
- undelegateAll() external onlyAgent nonpayable
  Remove all wNat delegations.
- delegateGovernance(address _to) external onlyAgent nonpayable
  Delegate governance vote power.
- undelegateGovernance() external onlyAgent nonpayable
  Remove governance delegation.
- claimDelegationRewards(IRewardManager _rewardManager, uint24 _lastRewardEpoch, IRewardManager.RewardClaimWithProof[] calldata _proofs) external onlyAgent nonReentrant nonpayable returns (uint256)
  Claim delegation rewards; increase collateral.
- claimAirdropDistribution(IDistributionToDelegators _distribution, uint256 _month) external onlyAgent nonReentrant nonpayable returns (uint256)
  Claim airdrop distribution; increase collateral.
- optOutOfAirdrop(IDistributionToDelegators _distribution) external onlyAgent nonReentrant nonpayable
  Opt out of future airdrops.
- implementation() external view returns (address)
  Current UUPS implementation address.
- _authorizeUpgrade(address _newImplementation) internal override onlyAssetManager nonpayable
  Restrict upgrades to AssetManager.
- isAgentVaultOwner(address _address) internal view returns (bool)
  Checks if caller controls agent vault.
- supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  ERC165: IERC165, ICollateralPool, IICollateralPool.


## SUMMARY OF FILE: 2025-08-flare/contracts/collateralPool/implementation/CollateralPoolFactory.sol
CollateralPoolFactory.sol — Summary

A minimal factory that deploys ERC1967Proxy instances pointing to a CollateralPool implementation, then initializes them. The factory is stateless and non-custodial (no user funds). There is no access control; anyone can call create. Upgrade rights are handled by the CollateralPool logic (UUPS expected), not this factory. Main entrypoints: create, upgradeInitCall, supportsInterface. Trust model: users interact with CollateralPool proxies; this factory only instantiates proxies using a fixed implementation set at construction.

Storage
- implementation (address) — CollateralPool logic addr

Functions
- constructor(address _implementation) nonpayable
  • Sets the CollateralPool implementation used for all new proxies.

- function create(IIAssetManager _assetManager, address _agentVault, AgentSettings.Data memory _settings) external override returns (IICollateralPool) nonpayable
  • Deploy ERC1967 proxy, then initialize pool with agent vault, asset manager, fAsset, and exit CR.

- function upgradeInitCall(address _proxy) external pure override returns (bytes memory)
  • Returns empty calldata for upgradeToAndCall; no extra init needed by this factory.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  • ERC165 support: IERC165 and IICollateralPoolFactory interface IDs are recognized.

Notes
- Uses SafeCast.toUint32 for poolExitCollateralRatioBIPS; assumes BIPS value fits uint32.
- Proxy admin/upgrade flow is governed by CollateralPool implementation (e.g., UUPS), not by the factory.
- No events/errors emitted; implementation address is immutable after deployment.


## SUMMARY OF FILE: 2025-08-flare/contracts/collateralPool/implementation/CollateralPoolToken.sol
# CollateralPoolToken (CPT)

Upgradable ERC20 token representing shares in an agent’s collateral pool. Only the CollateralPool can mint/burn. Transfers are restricted by two locks: time-locks per deposit and debt-lock determined by the CollateralPool. Users hold funds; AssetManager governs upgrades via UUPS. Major entrypoints: mint, burn, lockedBalanceOf, transferableBalanceOf, cleanupExpiredTimelocks. Custom errors guard unauthorized calls and insufficient unlocked/transferable balances. Timelock duration is read from AssetManager.

Storage
- address collateralPool — pool contract addr
- string tokenName — ERC20 name
- string tokenSymbol — ERC20 symbol
- mapping(address => TimelockQueue) timelocksByAccount — per-user timelocks
- bool ignoreTimelocked — internal burn bypass flag
- bool initialized — init guard

Functions
- constructor(address _collateralPool, string memory _tokenName, string memory _tokenSymbol) nonpayable ERC20(_tokenName,_tokenSymbol)
  - Deploy and immediately initialize CPT instance.
- function initialize(address _collateralPool, string memory _tokenName, string memory _tokenSymbol) public nonpayable
  - One-time initializer setting pool and ERC20 metadata.
- function name() public view returns (string memory)
  - Return token name overridden from ERC20.
- function symbol() public view returns (string memory)
  - Return token symbol overridden from ERC20.
- function mint(address _account, uint256 _amount) external onlyCollateralPool nonpayable returns (uint256 _timelockExpiresAt)
  - Mint CPT and enqueue a timelock; returns expiry timestamp.
- function burn(address _account, uint256 _amount, bool _ignoreTimelocked) external onlyCollateralPool nonpayable
  - Burn CPT; optionally ignore timelock to enable payouts.
- function lockedBalanceOf(address _account) external view returns (uint256)
  - Max of debt-locked and time-locked balances.
- function transferableBalanceOf(address _account) external view returns (uint256)
  - Min of debt-free and non-timelocked balances.
- function debtFreeBalanceOf(address _account) public view returns (uint256)
  - Query CollateralPool for user’s debt-free CPT.
- function debtLockedBalanceOf(address _account) public view returns (uint256)
  - Query CollateralPool for user’s debt-locked CPT.
- function timelockedBalanceOf(address _account) public view returns (uint256 _timelocked)
  - Sum unexpired timelocks capped by total balance.
- function nonTimelockedBalanceOf(address _account) public view returns (uint256)
  - Balance minus timelocked amount.
- function _beforeTokenTransfer(address _from, address /* _to */, uint256 _amount) internal override nonpayable
  - Enforce debt-free and non-timelocked limits; prune expired timelocks.
- function cleanupExpiredTimelocks(address _account, uint256 _maxTimelockedEntries) public nonpayable returns (bool _cleanedAllExpired)
  - Remove up to N expired timelock entries for account.
- function _getTimelockDuration() internal view returns (uint256)
  - Read timelock duration from AssetManager setting.
- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  - ERC165 support for IERC165, IERC20, ICollateralPoolToken.
- function implementation() external view returns (address)
  - Return current UUPS implementation address.
- function _authorizeUpgrade(address /* _newImplementation */) internal virtual override nonpayable
  - Only AssetManager (via pool) may authorize upgrades.

Trust/roles
- Users: hold CPT; subject to time/debt locks on transfers.
- CollateralPool: sole minter/burner; triggers timelocks.
- AssetManager: sole UUPS upgrade authority via _authorizeUpgrade.
- Guards: OnlyCollateralPool, custom errors on insufficient balances.



## SUMMARY OF FILE: 2025-08-flare/contracts/collateralPool/implementation/CollateralPoolTokenFactory.sol
Summary (≤100 words)
A minimal factory that deploys ERC1967Proxy-based CollateralPoolToken (FCPT) instances bound to a specific Collateral Pool. Trust model: no user funds are held; anyone can call create; only state is the implementation address set in the constructor. Adminless after deployment. Major entrypoints: create (deploy/initialize FCPT), upgradeInitCall (no-op upgrade hook), supportsInterface (ERC165).

Storage Variables
- implementation (address public) – FCPT logic addr

Functions
- constructor(address _implementation) nonpayable
  NatSpec: Store the CollateralPoolToken logic address used by new ERC1967 proxies.

- function create(IICollateralPool _pool, string memory _systemSuffix, string memory _agentSuffix) external override returns (address) nonpayable
  NatSpec: Deploy ERC1967 proxy, initialize FCPT with pool and name/symbol, return token address.

- function upgradeInitCall(address _proxy) external pure override returns (bytes memory)
  NatSpec: Return calldata for upgradeToAndCall; this version returns empty bytes (no init).

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  NatSpec: ERC165 support for IERC165 and IICollateralPoolTokenFactory.

Notes
- Token name: "FAsset Collateral Pool Token {system}-{agent}"; symbol: "FCPT-{system}-{agent}".
- Uses OpenZeppelin ERC1967Proxy; initialization is performed post-deploy via initialize(address,string,string).
- No access control; any caller can deploy FCPTs pointing to any pool.



## SUMMARY OF FILE: 2025-08-flare/contracts/coreVaultManager/implementation/CoreVaultManager.sol
## CoreVaultManager (Flare FAssets)

Purpose: Governance-controlled manager that orchestrates Core Vault off-chain actions (payments, escrows) for FAssets. Tracks confirmed inbound underlying payments via FDC, queues transfer requests, emits sequenced payment/escrow instructions for multisig operators, and enforces pause/role controls. Funds are not custodied on-chain; amounts are book-kept (available/escrowed) and executed off-chain by trusted multisig. Admin/governance controls settings, destinations, trigger accounts, and pause; AssetManager initiates/cancels transfer requests. Key entrypoints: initialize, confirmPayment, request/cancel transfers, triggerInstructions, processEscrows, pause/unpause, settings/destinations management, query getters.

### Storage
- assetManager (address public) - AssetManager addr
- chainId (bytes32 public) - Underlying chain id
- custodianAddress (string public) - Custodian addr text
- coreVaultAddressHash (bytes32 public) - CV address hash
- coreVaultAddress (string public) - CV address text
- nextSequenceNumber (uint256 public) - Next instr seq
- fdcVerification (IFdcVerification public) - FDC verifier
- confirmedPayments (mapping(bytes32=>bool) public) - Tx seen map
- preimageHashes (Bytes32Set private) - Escrow preimages
- escrows (Escrow[] private) - Escrow list
- preimageHashToEscrowIndex (mapping(bytes32=>uint256) private) - Preimage→idx (1b)
- nextUnusedPreimageHashIndex (uint256 public) - Next preimage idx
- nextUnprocessedEscrowIndex (uint256 public) - Next escrow idx
- nextTransferRequestId (uint256 private) - Next req id
- cancelableTransferRequests (uint256[] private) - Cancelable req ids
- nonCancelableTransferRequests (uint256[] private) - Non-cancel req ids
- transferRequestById (mapping(uint256=>TransferRequest) private) - Req data
- allowedDestinationAddresses (string[] private) - Allowed dests
- allowedDestinationAddressIndex (mapping(string=>uint256) private) - Dest→idx (1b)
- triggeringAccounts (AddressSet private) - Who can trigger
- emergencyPauseSenders (AddressSet private) - Who can pause
- escrowEndTimeSeconds (uint128 private) - Daily escrow end
- escrowAmount (uint128 private) - Escrow chunk amt
- minimalAmount (uint128 private) - Min left in CV
- fee (uint128 private) - Chain fee per op
- availableFunds (uint128 public) - Liquid funds
- escrowedFunds (uint128 public) - In escrow funds
- cancelableTransferRequestsAmount (uint128 private) - Sum cancelable
- nonCancelableTransferRequestsAmount (uint128 private) - Sum non-cancel
- paused (bool public) - Pause flag

### Functions
- constructor() public nonpayable
  - Initializes base mixins; empty body.

- initialize(IGovernanceSettings _governanceSettings, address _initialGovernance, address _addressUpdater, address _assetManager, bytes32 _chainId, string memory _custodianAddress, string memory _coreVaultAddress, uint256 _nextSequenceNumber) external nonpayable
  - One-time proxy init; sets roles, chain and vault.

- confirmPayment(IPayment.Proof calldata _proof) external nonpayable
  - Verifies inbound payment via FDC and credits funds.

- requestTransferFromCoreVault(string memory _destinationAddress, bytes32 _paymentReference, uint128 _amount, bool _cancelable) external onlyAssetManager notPaused nonpayable returns (bytes32)
  - Enqueue transfer; merge or create; balance checks.

- cancelTransferRequestFromCoreVault(string memory _destinationAddress) external onlyAssetManager nonpayable
  - Cancel a pending cancelable request by destination.

- processEscrows(uint256 _maxCount) external nonpayable returns (bool)
  - Process expired/finished escrows; update balances.

- triggerInstructions() external notPaused nonpayable returns (uint256 _numberOfInstructions)
  - Emit payment/escrow instructions; advances sequence.

- addAllowedDestinationAddresses(string[] calldata _allowedDestinationAddresses) external onlyGovernance nonpayable
  - Governance adds allowed destination addresses.

- removeAllowedDestinationAddresses(string[] calldata _allowedDestinationAddresses) external onlyGovernance nonpayable
  - Governance removes allowed destination addresses.

- addTriggeringAccounts(address[] calldata _triggeringAccounts) external onlyGovernance nonpayable
  - Governance whitelists instruction triggerers.

- removeTriggeringAccounts(address[] calldata _triggeringAccounts) external onlyGovernance nonpayable
  - Governance removes instruction triggerers.

- updateCustodianAddress(string calldata _custodianAddress) external onlyGovernance nonpayable
  - Governance updates off-chain custodian address.

- updateSettings(uint128 _escrowEndTimeSeconds, uint128 _escrowAmount, uint128 _minimalAmount, uint128 _fee) external onlyGovernance nonpayable
  - Governance sets escrow window, size, min, fee.

- addPreimageHashes(bytes32[] calldata _preimageHashes) external onlyImmediateGovernance nonpayable
  - Add escrow preimages; used sequentially.

- removeUnusedPreimageHashes(uint256 _maxCount) external onlyImmediateGovernance nonpayable
  - Remove last unused preimages up to max.

- setEscrowsFinished(bytes32[] calldata _preimageHashes) external onlyImmediateGovernance nonpayable
  - Mark escrows finished; adjust accounting.

- addEmergencyPauseSenders(address[] calldata _addresses) external onlyImmediateGovernance nonpayable
  - Add addresses allowed to emergency pause.

- removeEmergencyPauseSenders(address[] calldata _addresses) external onlyImmediateGovernance nonpayable
  - Remove addresses allowed to emergency pause.

- pause() external nonpayable
  - Governance or approved senders pause contract.

- unpause() external onlyImmediateGovernance nonpayable
  - Immediate governance unpauses the contract.

- triggerCustomInstructions(bytes32 _instructionsHash) external onlyImmediateGovernance nonpayable
  - Emit custom off-chain instructions; bumps sequence.

- getSettings() external view returns (uint128 _escrowEndTimeSeconds, uint128 _escrowAmount, uint128 _minimalAmount, uint128 _fee)
  - Read current escrow and fee settings.

- getAllowedDestinationAddresses() external view returns (string[] memory)
  - List of allowed destination addresses.

- isDestinationAddressAllowed(string memory _address) external view returns (bool)
  - Check if destination is currently allowed.

- getTriggeringAccounts() external view returns (address[] memory)
  - List of accounts allowed to trigger.

- getUnprocessedEscrows() external view returns (Escrow[] memory _unprocessedEscrows)
  - Escrows not yet processed by time.

- getEscrowsCount() external view returns (uint256)
  - Total escrows count (including processed).

- getEscrowByIndex(uint256 _index) external view returns (Escrow memory)
  - Get escrow struct by array index.

- getEscrowByPreimageHash(bytes32 _preimageHash) external view returns (Escrow memory)
  - Get escrow by preimage hash.

- getUnusedPreimageHashes() external view returns (bytes32[] memory)
  - Unused preimages starting from next index.

- getPreimageHashesCount() external view returns (uint256)
  - Number of stored preimage hashes.

- getPreimageHash(uint256 _index) external view returns (bytes32)
  - Read a preimage hash by index.

- getCancelableTransferRequests() external view returns (TransferRequest[] memory _transferRequests)
  - Get all cancelable transfer requests.

- getNonCancelableTransferRequests() external view returns (TransferRequest[] memory _transferRequests)
  - Get all non-cancelable transfer requests.

- totalRequestAmountWithFee() public view returns (uint256)
  - Sum of requested amounts plus per-op fees.

- getEmergencyPauseSenders() external view returns (address[] memory)
  - Read emergency pause senders.

- supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  - ERC165 support for interfaces.


## SUMMARY OF FILE: 2025-08-flare/contracts/coreVaultManager/implementation/CoreVaultManagerProxy.sol
Summary
CoreVaultManagerProxy is a minimal ERC1967 proxy that delegates all calls to a CoreVaultManager implementation. It executes CoreVaultManager.initialize in the constructor to set governance and core-vault parameters. No user funds are held in the proxy; trust and upgrades are managed by the implementation (UUPS-style), governed via the provided governance settings. Major entrypoints: constructor (deployment/init), fallback (delegation), and receive (delegation of plain ETH). All operational logic lives in CoreVaultManager.

Storage Variables
- EIP1967_IMPLEMENTATION_SLOT — current implementation addr
- EIP1967_ADMIN_SLOT — reserved admin slot (unused here)
- EIP1967_BEACON_SLOT — reserved beacon slot (unused)

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
  )
  visibility: public
  modifiers: none
  mutability: nonpayable
  NatSpec: Deploys proxy and calls CoreVaultManager.initialize with governance and core vault config.

- fallback() external payable
  visibility: external
  modifiers: none
  mutability: payable
  NatSpec: Delegates arbitrary calls to current implementation via ERC1967 proxy.

- receive() external payable
  visibility: external
  modifiers: none
  mutability: payable
  NatSpec: Accepts plain ETH and delegates if needed per Proxy semantics.


## SUMMARY OF FILE: 2025-08-flare/contracts/diamond/facets/DiamondLoupeFacet.sol
# DiamondLoupeFacet (EIP-2535 Loupe)

Read-only facet exposing a diamond’s facets and function selectors for tooling and integrations. It holds no funds, has no admin paths, and reads state from the diamond via LibDiamond. Trust model: purely observational; user funds are untouched. Major entrypoints: facets(), facetFunctionSelectors(address), facetAddresses(), facetAddress(bytes4), supportsInterface(bytes4). All functions are external view.

## Storage
- Contract-level: none (stateless facet)
- LibDiamond.DiamondStorage fields read:
  - selectors — All selectors
  - facetAddressAndSelectorPosition — Selector→facet map
  - supportedInterfaces — ERC165 map

## Functions

- interface:
```solidity
function facets() external override view returns (Facet[] memory facets_);
```
NatSpec: Return all facet addresses with their selectors.

- interface:
```solidity
function facetFunctionSelectors(address _facet) external override view returns (bytes4[] memory _facetFunctionSelectors);
```
NatSpec: Return all selectors implemented by the given facet address.

- interface:
```solidity
function facetAddresses() external override view returns (address[] memory facetAddresses_);
```
NatSpec: Return unique list of facet addresses used by the diamond.

- interface:
```solidity
function facetAddress(bytes4 _functionSelector) external override view returns (address facetAddress_);
```
NatSpec: Return facet address implementing the selector, or zero if none.

- interface:
```solidity
function supportsInterface(bytes4 _interfaceId) external override view returns (bool);
```
NatSpec: ERC165 support query served from diamond storage.



## SUMMARY OF FILE: 2025-08-flare/contracts/diamond/mock/DiamondCutFacet.sol
# DiamondCutFacet Summary

DiamondCutFacet is an admin-only facet for performing EIP-2535 Diamond upgrades. It exposes a single entrypoint, diamondCut, restricted by onlyGovernance, to add/replace/remove function selectors and optionally run an initialization via delegatecall. The contract itself doesn’t hold user funds; however, governance has full upgrade power, so trust is placed in the admin/governance process. It delegates upgrade logic to LibDiamond and uses governance controls from GovernedProxyImplementation.

## Storage Variables
- None (no direct state vars in this facet)

Note: Uses Diamond storage via LibDiamond and governance storage via inheritance; not enumerated here.

## Functions

- function diamondCut(IDiamondCut.FacetCut[] calldata _diamondCut, address _init, bytes calldata _calldata) external override onlyGovernance nonpayable;
  - NatSpec: Governance-only diamond cut; updates facets and optionally runs init via delegatecall.


## SUMMARY OF FILE: 2025-08-flare/contracts/diamond/mock/DiamondInit.sol
# DiamondInit (EIP-2535 Diamond initializer)

Purpose: Minimal initializer for an EIP-2535 diamond. It sets governance (via GovernedBase.initialise) and registers ERC-165/diamond interfaces in diamond storage. Trust model: no user funds; admin/governance-only state writes. Intended to be called once through diamondCut’s _init delegatecall. Major entrypoint: init().

## Storage Variables
- None declared in this contract
- LibDiamond.DiamondStorage.supportedInterfaces — ERC165/diamond iface map
- GovernedBase internal storage — governance settings & governor

## Functions
- function init(IGovernanceSettings _governanceSettings, address _initialGovernance) external nonpayable
  - NatSpec: Initialize governance and set IERC165/IDiamondCut/IDiamondLoupe support in diamond storage. Intended for diamondCut _init.


## SUMMARY OF FILE: 2025-08-flare/contracts/diamond/mock/Test1Facet.sol
# Test1Facet — Summary

Test1Facet is a minimal EIP-2535 (Diamond) facet demonstrating diamond storage via TestLib. It defines no local storage, holds no user funds, and has no admin-only paths. Trust model: stateless example facet; only writes/reads a library-owned storage slot. Major entrypoints: test1Func1 (store this contract’s address), test1Func2 (read stored address), and supportsInterface (ERC‑165 query stub). All other functions (test1Func3–test1Func20) are no-op placeholders.

## Storage Variables
- None (facet has no local storage)

## Events
- event TestEvent(address something)

## Functions

- function test1Func1() external  // nonpayable; modifiers: none
  - Natspec: Store this contract’s address in diamond storage via TestLib.

- function test1Func2() external view returns (address)  // modifiers: none
  - Natspec: Read address from diamond storage via TestLib.

- function test1Func3() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func4() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func5() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func6() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func7() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func8() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func9() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func10() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func11() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func12() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func13() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func14() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func15() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func16() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func17() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func18() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func19() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function test1Func20() external  // nonpayable; modifiers: none
  - Natspec: No-op placeholder function.

- function supportsInterface(bytes4 _interfaceID) external view returns (bool)  // modifiers: none
  - Natspec: ERC‑165 support query stub; implementation not provided here.


## SUMMARY OF FILE: 2025-08-flare/contracts/diamond/mock/Test2Facet.sol
Test2Facet is a minimal diamond facet for testing selector routing and integration. It defines twenty external no-op entrypoints and contains no storage or logic. Trust model: stateless, holds no user funds; no owner/admin or privileged paths. Major entrypoints are test2Func1..test2Func20, all nonpayable and side‑effect free, used to exercise diamond cut/loupe and dispatcher behavior.

Storage Variables
- None

Functions
- function test2Func1() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func2() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func3() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func4() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func5() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func6() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func7() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func8() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func9() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func10() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func11() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func12() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func13() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func14() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func15() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func16() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func17() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func18() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func19() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.
- function test2Func20() external nonpayable [modifiers: none] — No-op external function placeholder for diamond facet tests.


## SUMMARY OF FILE: 2025-08-flare/contracts/fassetToken/implementation/FAsset.sol
## Overview
FAsset is an ERC20 token with EIP-2612 permit and checkpointed vote-power history, used as the minted FAsset for a single AssetManager. Only the AssetManager may mint/burn and authorize UUPS upgrades; the deployer sets the AssetManager once. Transfers are blocked during an AssetManager-driven emergency pause, while mint/burn remain allowed. Separate cleaner roles prune historical checkpoints. Trust model: users self-custody balances; AssetManager holds privileged admin rights. Key entrypoints: initialize, setAssetManager, mint, burn, setCleanupBlockNumber, setCleanerContract, setCleanupBlockNumberManager, implementation.

## Storage
- assetName: string public — underlying name
- assetSymbol: string public — underlying symbol
- cleanupBlockNumberManager: address public — cleaner admin
- assetManager: address public — owning AM
- __terminatedAt: uint64 private — placeholder
- _name: string private — ERC20 name
- _symbol: string private — ERC20 symbol
- _decimals: uint8 private — ERC20 decimals
- _deployer: address private — deployer addr
- _initialized: bool private — init guard
- _version: uint16 private — upgrade ver

## Modifiers
- onlyAssetManager — requires msg.sender == assetManager

## Functions
- constructor() ERC20("", "")
  - Natspec: Set impl guards; logic unused via proxy.

- function initialize(string memory name_, string memory symbol_, string memory assetName_, string memory assetSymbol_, uint8 decimals_) external
  - Natspec: One-time initializer; sets metadata and permit domain seed.

- function initializeV1r1() public
  - Natspec: Sets version=1 and initializes EIP712 domain.

- function setAssetManager(address _assetManager) external
  - Natspec: Deployer-only one-time AssetManager assignment.

- function mint(address _owner, uint256 _amount) external override onlyAssetManager
  - Natspec: Mint tokens to owner; callable only by AssetManager.

- function burn(address _owner, uint256 _amount) external override onlyAssetManager
  - Natspec: Burn owner tokens; callable only by AssetManager.

- function name() public view virtual override(ERC20, IERC20Metadata) returns (string memory)
  - Natspec: Return ERC20 name (custom storage).

- function symbol() public view virtual override(ERC20, IERC20Metadata) returns (string memory)
  - Natspec: Return ERC20 symbol (custom storage).

- function decimals() public view virtual override(ERC20, IERC20Metadata) returns (uint8)
  - Natspec: Return token decimals.

- function setCleanupBlockNumber(uint256 _blockNumber) external override
  - Natspec: Cleaner-manager sets checkpoint cleanup boundary.

- function cleanupBlockNumber() external view override returns (uint256)
  - Natspec: Get current cleanup block number.

- function setCleanerContract(address _cleanerContract) external override onlyAssetManager
  - Natspec: AssetManager designates the cleaner contract.

- function setCleanupBlockNumberManager(address _cleanupBlockNumberManager) external onlyAssetManager
  - Natspec: AssetManager sets address allowed to set cleanup block.

- function _beforeTokenTransfer(address _from, address _to, uint256 _amount) internal override
  - Natspec: Enforce balance, no self-transfer, pause, and update checkpoints.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  - Natspec: ERC165 support for ERC20, Permit, checkpoints, fAsset, cleanable.

- function _approve(address _owner, address _spender, uint256 _amount) internal virtual override(ERC20, ERC20Permit)
  - Natspec: Tie-break ERC20/Permit approve; uses ERC20 logic.

- function implementation() external view returns (address)
  - Natspec: Return current UUPS implementation address.

- function _authorizeUpgrade(address /* _newImplementation */) internal virtual override onlyAssetManager
  - Natspec: Restrict UUPS upgrades to AssetManager.


## SUMMARY OF FILE: 2025-08-flare/contracts/fassetToken/implementation/FAssetProxy.sol
Overview
FAssetProxy is a minimal ERC1967 proxy that deploys an upgradeable FAsset ERC‑20. Its constructor sets the implementation and atomically calls FAsset.initialize with token metadata. All runtime calls are delegated to the implementation. The proxy itself has no admin entrypoints; upgrade authority is defined in the implementation (e.g., UUPS/governance). User funds (balances) reside in proxy storage; trust revolves around upgrade governance/admin of the implementation.

Major entrypoints
- constructor (initializes implementation via delegatecall)
- fallback (delegates all unknown calls)
- receive (accepts ETH and delegates)

Storage variables
- None declared in this contract
- Inherited (ERC1967): implementation slot (EIP‑1967). No additional storage defined.

Functions
1) constructor(
   address _implementationAddress,
   string memory _name,
   string memory _symbol,
   string memory _assetName,
   string memory _assetSymbol,
   uint8 _decimals
) nonpayable
- Natspec: Deploy proxy and initialize FAsset implementation with metadata.
- Notes: Calls ERC1967Proxy(_implementationAddress, abi.encodeCall(FAsset.initialize,(...))). No modifiers.

2) fallback() external payable
- Natspec: Delegate all calls to the current implementation.
- Notes: Inherited from ERC1967Proxy/Proxy; routes via delegatecall.

3) receive() external payable
- Natspec: Accept ETH and delegate to implementation when value-only calls arrive.
- Notes: Inherited from ERC1967Proxy/Proxy; forwards via _fallback().


## SUMMARY OF FILE: 2025-08-flare/contracts/ftso/implementation/FtsoV2PriceStore.sol
FtsoV2PriceStore stores and serves FTSOv2 prices per feed id, verified via Relay Merkle proofs. Governance configures feeds, symbols, trusted provider set, threshold, and spread limits; address updater wires the Relay. No user funds are held; admin risk is configuration-only. Major entrypoints: initialize, publishPrices (Merkle-verified), submitTrustedPrices (trusted submitters), updateSettings, setTrustedProviders, and read APIs getPrice/getPriceFromTrustedProviders. The contract aggregates trusted submissions into a median (with spread bound), publishes canonical prices per round, and exposes timestamps aligned to voting epochs.

Storage variables
- firstVotingRoundStartTs (uint64) – First epoch start
- votingEpochDurationSeconds (uint64) – Epoch duration
- submitTrustedPricesWindowSeconds (uint64) – Trusted submit window
- ftsoProtocolId (uint8) – FTSO protocol id
- feedIds (bytes21[]) – Required feed IDs
- symbolToFeedId (mapping(string=>bytes21)) – Symbol->feedId
- feedIdToSymbol (mapping(bytes21=>string)) – feedId->symbol
- latestPrices (mapping(bytes21=>PriceStore)) – Latest prices
- submittedTrustedPrices (mapping(bytes21=>mapping(uint32=>bytes))) – Per-round submits
- lastVotingEpochIdByProvider (mapping(address=>uint256)) – Provider last round
- trustedProviders (address[]) – Trusted providers
- trustedProvidersMap (mapping(address=>bool)) – Trusted map
- trustedProvidersThreshold (uint8) – Median threshold
- maxSpreadBIPS (uint16) – Max spread BIPS
- relay (IRelay) – Relay contract
- lastPublishedVotingRoundId (uint32) – Last published id

Functions
- constructor() GovernedUUPSProxyImplementation, AddressUpdatable(address(0))
  Natspec: Initialize base contracts; marks implementation as initialized.

- function initialize(
    IGovernanceSettings _governanceSettings,
    address _initialGovernance,
    address _addressUpdater,
    uint64 _firstVotingRoundStartTs,
    uint8 _votingEpochDurationSeconds,
    uint8 _ftsoProtocolId
  ) external
  Natspec: One-time setup; sets governance, updater, timing and protocol settings.

- function publishPrices(FeedWithProof[] calldata _proofs) external
  Natspec: Publish canonical prices for a round using Relay Merkle proofs.

- function submitTrustedPrices(uint32 _votingRoundId, TrustedProviderFeed[] calldata _feeds) external
  Natspec: Trusted providers submit per-feed prices during window; one submit per round.

- function updateSettings(
    bytes21[] calldata _feedIds,
    string[] calldata _symbols,
    int8[] calldata _trustedDecimals,
    uint16 _maxSpreadBIPS
  ) external onlyGovernance
  Natspec: Governance updates feeds, symbols, trusted decimals, and max spread.

- function setTrustedProviders(address[] calldata _trustedProviders, uint8 _trustedProvidersThreshold) external onlyGovernance
  Natspec: Governance sets trusted provider list and median threshold.

- function getPrice(string memory _symbol)
    external view
    returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
  Natspec: Read last published FTSO price for symbol with epoch timestamp.

- function getPriceFromTrustedProviders(string memory _symbol)
    external view
    returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
  Natspec: Read last accepted trusted-median price for symbol.

- function getPriceFromTrustedProvidersWithQuality(string memory _symbol)
    external view
    returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals, uint8 _numberOfSubmits)
  Natspec: Read trusted-median price plus number of submissions used.

- function getFeedIds() external view returns (bytes21[] memory)
  Natspec: Return configured feed ids in canonical order.

- function getFeedIdsWithDecimals() external view returns (bytes21[] memory _feedIds, int8[] memory _decimals)
  Natspec: Return feed ids with required trusted decimals per feed.

- function getSymbols() external view returns (string[] memory _symbols)
  Natspec: Return configured symbols in feedIds order.

- function getFeedId(string memory _symbol) external view returns (bytes21)
  Natspec: Resolve a symbol to its feed id.

- function getTrustedProviders() external view returns (address[] memory)
  Natspec: Return the current trusted provider addresses.

- function _updateContractAddresses(bytes32[] memory _contractNameHashes, address[] memory _contractAddresses)
    internal override
  Natspec: Pull Relay address from AddressUpdater registry.

- function _getPreviousVotingEpochId() internal view returns (uint32)
  Natspec: Compute previous voting epoch id from timestamps.

- function _getEndTimestamp(uint256 _votingEpochId) internal view returns (uint256)
  Natspec: End timestamp for given voting epoch id.

- function _getPriceFromTrustedProviders(PriceStore storage _feed)
    internal view
    returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
  Natspec: Convert stored trusted value to price with decimals and timestamp.

- function _calculateMedian(bytes memory _prices)
    internal view
    returns (uint256 _medianPrice, bool _priceOk)
  Natspec: Median of uint32 prices; enforces max spread threshold.

- function supportsInterface(bytes4 _interfaceId)
    external pure override
    returns (bool)
  Natspec: ERC165: supports IERC165, IPriceReader, IPricePublisher.


## SUMMARY OF FILE: 2025-08-flare/contracts/ftso/implementation/FtsoV2PriceStoreProxy.sol
FtsoV2PriceStoreProxy (ERC1967 UUPS proxy)

Summary
- Purpose: Minimal UUPS-compatible ERC1967 proxy for FtsoV2PriceStore, wiring governance and system parameters at deployment.
- Trust model: Holds no user funds; governance-administered logic and upgrades live in the implementation (FtsoV2PriceStore). Proxy has no separate admin (UUPS pattern).
- Major entrypoints: constructor (deploy-time initializer), fallback/receive (delegatecalls to implementation).

Storage Variables
- (none declared in this contract)
- Inherited EIP-1967 slots (from ERC1967Upgrade):
  - implementation slot – impl address
  - beacon slot – beacon addr
  - admin slot – admin addr (unused)
  - rollback slot – uups rollback flag

Functions
1) constructor(
       address _implementationAddress,
       IGovernanceSettings _governanceSettings,
       address _initialGovernance,
       address _addressUpdater,
       uint64 _firstVotingRoundStartTs,
       uint8 _votingEpochDurationSeconds,
       uint8 _ftsoProtocolId
   ) public nonpayable
   - NatSpec: Deploy proxy and atomically initialize FtsoV2PriceStore via delegatecall.

2) fallback() external payable (inherited from Proxy)
   - NatSpec: Delegates unknown calls to current implementation.

3) receive() external payable (inherited from Proxy)
   - NatSpec: Accepts native value and delegates if data empty.

Initialization Flow
- The constructor calls ERC1967Proxy with implementation and encoded FtsoV2PriceStore.initialize(
  governanceSettings, initialGovernance, addressUpdater, firstVotingRoundStartTs, votingEpochDurationSeconds, ftsoProtocolId
).
- Upgrades are expected through UUPS logic implemented in FtsoV2PriceStore (governance-gated).


## SUMMARY OF FILE: 2025-08-flare/contracts/ftso/mock/FakePriceReader.sol
# FakePriceReader (test/mock FTSO price source)

Lightweight, single-admin mock for reading and emitting prices. A designated provider sets per-symbol decimals, spot price, and “trusted” price; timestamps auto-update on writes. finalizePrices emits PricesPublished per IPriceChangeEmitter. No user funds are held; only the provider can mutate state. Public getters expose current and trusted prices with decimals and optional quality. ERC165 advertises IPriceReader and IPriceChangeEmitter support.

- Trust model: admin-only writes (provider); read-only for others; no custody of assets.
- Major entrypoints: setDecimals, setPrice, setPriceFromTrustedProviders, finalizePrices, getPrice*, supportsInterface.

Storage
- provider (address, public) — authorized writer
- pricingData (mapping(string => PricingData), private) — per-symbol data

Structs
- PricingData { uint8 decimals; uint128 price; uint64 timestamp; uint128 trustedPrice; uint64 trustedTimestamp; }

Modifiers
- onlyDataProvider — restricts caller to provider

Functions
- constructor(address _provider) nonpayable
  - Set the authorized provider address.

- function setDecimals(string memory _symbol, uint256 _decimals) external onlyDataProvider nonpayable
  - Initialize or update decimals for a symbol.

- function setPrice(string memory _symbol, uint256 _price) external onlyDataProvider nonpayable
  - Set spot price and timestamp for symbol.

- function setPriceFromTrustedProviders(string memory _symbol, uint256 _price) external onlyDataProvider nonpayable
  - Set trusted price and timestamp for symbol.

- function finalizePrices() external onlyDataProvider nonpayable
  - Emit PricesPublished indicating a finalized price round.

- function getPrice(string memory _symbol) external view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
  - Read spot price, timestamp, and decimals.

- function getPriceFromTrustedProviders(string memory _symbol) external view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
  - Read trusted price, timestamp, and decimals.

- function getPriceFromTrustedProvidersWithQuality(string memory _symbol) external view returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals, uint8 _numberOfSubmits)
  - Read trusted price with submit count (always zero).

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  - ERC165 support for PriceReader and PriceChangeEmitter.

Internal/Private
- function _getPricingData(string memory _symbol) private view returns (PricingData storage)
  - Fetch storage slot; reverts if decimals unset.

Notes
- Reverts: onlyDataProvider ("only provider"); _getPricingData("price not initialized").
- Emits: PricesPublished(uint256 roundId) in finalizePrices.


## SUMMARY OF FILE: 2025-08-flare/contracts/utils/mock/FakeERC20.sol
Summary
A governed mock ERC-20 token with configurable decimals for testing FAssets components. Governance can mint; users can self-burn. No user funds are custodied beyond ERC20 balances; admin (governance) controls supply via minting. Major entrypoints: mintAmount (governance mint), burnAmount (user burn), decimals (metadata), supportsInterface (ERC165 introspection).

Storage Variables
- decimals_ (uint8, private, immutable) — custom decimals

Key inherited storage (from OZ ERC20)
- _balances — account balances
- _allowances — spender allowances
- _totalSupply — total token supply
- _name — token name
- _symbol — token symbol

Key inherited storage (from Governed)
- governanceSettings — governance config
- governance — current governance addr
- pendingGovernance — proposed governance addr

Functions
- constructor(IGovernanceSettings _governanceSettings, address _initialGovernance, string memory _name, string memory _symbol, uint8 _decimals) nonpayable
  NatSpec: Initialize ERC20 metadata, governance, and custom decimals.

- function mintAmount(address _target, uint256 amount) public onlyGovernance nonpayable
  NatSpec: Governance-mints tokens to target address.

- function burnAmount(uint256 _amount) public nonpayable
  NatSpec: Burns caller’s tokens by amount.

- function decimals() public view override returns (uint8)
  NatSpec: Returns configured decimals for this token.

- function supportsInterface(bytes4 _interfaceId) external pure override returns (bool)
  NatSpec: ERC165 support; declares IERC165, IERC20, IERC20Metadata.


## SUMMARY OF FILE: 2025-08-flare/contracts/utils/mock/TestUUPSProxyImpl.sol
Overview
TestUUPSProxyImpl is a minimal UUPSUpgradeable implementation for testing. It stores a message and init flag, exposes the implementation address, and supports upgrades. Trust model: no admin checks; upgrades are permissionless via _authorizeUpgrade, so anyone can upgrade through the proxy. No user funds are handled. Major entrypoints: initialize, testResult, implementation, and inherited UUPS upgrade functions (upgradeTo, upgradeToAndCall, proxiableUUID).

Storage
- _dummy (uint256[1000] private) – storage gap
- message (string private) – stored message
- initialized (bool private) – init flag

Functions
- function _authorizeUpgrade(address newImplementation) internal override nonpayable
  – Authorization hook for UUPS upgrades. Here permits any caller; unsafe in production.

- function initialize(string memory _message) external nonpayable
  – Sets the message and marks contract initialized. No access control.

- function testResult() external view returns (string memory)
  – Reads message if initialized; otherwise returns default 'test proxy'.

- function implementation() external view returns (address)
  – Returns current implementation address from ERC1967 slot.

Inherited from UUPSUpgradeable
- function upgradeTo(address newImplementation) external onlyProxy nonpayable
  – Upgrade implementation to new address via UUPS. Requires proxy context.

- function upgradeToAndCall(address newImplementation, bytes calldata data) external payable onlyProxy
  – Upgrade implementation and execute call data in same tx. Payable.

- function proxiableUUID() external view notDelegated returns (bytes32)
  – Returns UUPS proxiable UUID. Reverts when delegatecalled (notDelegated).


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


