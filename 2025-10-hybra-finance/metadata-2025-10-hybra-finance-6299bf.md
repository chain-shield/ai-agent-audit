
## PROTOCOL OVERVIEW:

# Hybra Finance – Protocol Architecture & Flow  
*Version: Oct-2025 – compiled from on-chain source & Foundry deployment suite*

---
## 0. Why Hybra Exists
Hybra is a “ve(3,3)” AMM that mixes three threads that used to live apart:
1. **veToken voting + bribes** (Solidly style)
2. **Concentrated liquidity pools** (Uniswap v3 style) next to classic v2 pools
3. **Socialised auto-compounding vaults** so passive users capture real yield

The design gospel can be reduced to one line from the white-paper:
> *“Make someone other than the minter pay.”*  
Inflation jump-starts liquidity, but external fees & bribes must outweigh it within ~30 weeks or the fly-wheel snaps. Every contract/role is built around that thesis.

---
## 1. Token Stack
| Symbol | Contract | Role |
| ------ | -------- | ---- |
| **HYBR** | `HYBR.sol` (simple ERC-20, 18 dec) | Base asset & emission token |
| **rHYBR** | `RewardHYBR.sol` | Non-transferable IOU minted 1:1 for HYBR; redeemable to HYBR, gHYBR or veHYBR |
| **veHYBR** | `VotingEscrow.sol` (ERC-721) | Lock HYBR → voting power, share of rebases |
| **gHYBR** | `GovernanceHYBR.sol` | Auto-compounding vault that holds a single veNFT; gHYBR is receipt token |

Relationship:
1. Users deposit HYBR → receive **rHYBR** (instant) or **gHYBR** (vault) or lock directly for **veHYBR**.
2. Emission engine mints fresh HYBR each week ➜ splits to Team, Rebases (ve holders), and Gauges.
3. **gHYBR** vault continuously collects its own rebases / bribes, swaps them back to HYBR via **HybrSwapper**, and increases the underlying lock.

---
## 2. Emission Engine
### 2.1 MinterUpgradeable
* Upgradeable proxy behind `MinterUpgradeable` logic
* Key constants (bps): `EMISSION`  = 98 % of last week → slow geometric decay,  
  `TAIL_EMISSION` = 2 % of circ. supply cap, `REBASEMAX` caps free rebase at 18 % of weekly amount.
* Weekly flow (`update_period`):
  1. Calculate `weekly_emission()`  
     `max(target = weekly_prev * EMISSION, tail = circ * TAIL_EMISSION)`
  2. Mint:    
     • Team   = `teamRate` (initial 2 %)  
     • Rebase = `min(REBASEMAX, locked/total) * weekly` ➜ **RewardsDistributor**  
     • Remainder ➜ **GaugeManager** `notifyRewardAmount`  ➜ individual Gauges

### 2.2 RewardsDistributor
Distributes the rebase share pro-rata to every veNFT each week (look-back snapshots). If NFT is expired it auto-relocks permanently so rewards never leak.

---
## 3. Liquidity Layer
### 3.1 Pools
1. **V2‐style Pairs** (fork of Thena / Velodrome) – simple ERC-20 LP token
2. **V3‐style Pools** (`CLPool`) – Uniswap v3 clone with plug-in fee modules
   * Factory: `CLFactory`  – owns pool implementation & fee policies
   * Fee modules: Dynamic / Custom / Protocol / Unstaked  
     • Dynamic raises fee when volatility (tick deviation) spikes  
     • Unstaked charges higher fee to LP that do not stake NFT in Gauge

### 3.2 Gauges
| Gauge | Stakes | Rewards | Fee Sink |
| ----- | ------ | ------- | -------- |
| **GaugeV2** | ERC-20 LP token | HYBR emissions | `Bribe` internal & external |
| **GaugeCL** | UniswapV3 NFTs | HYBR emissions based on *virtual liquidity* | same |

Both gauges:
1. Are deployed by factories (upgradeable) and registered in **GaugeManager**.
2. On every swap pool accumulates fees:
   • Gauge’s `claimFees()` pulls them → forwards token0/1 to **internal bribe** for voters.
3. When `notifyRewardAmount` is called by GaugeManager the gauge streams HYBR over 7 days; users harvest via `rHYBR` (convert reward on the fly).

### 3.3 Bribes
`Bribe.sol` records vote weight checkpoints per epoch. Any address may `notifyRewardAmount` for an approved token; during `getReward` the veNFT voter receives proportional share.  
There are two bribes per gauge:
* **internal** – gets pool fees (token0, token1) via `claimFees()`
* **external** – gets partner incentives (USDC, protocol tokens, …)

Factories & manager:
* `BribeFactoryV3` creates bribes, pre-seeding default reward tokens.
* `GaugeManager` knows each pool → gauge → bribes and routes calls.

---
## 4. Voting Layer
### 4.1 VoterV3
* veNFT holders call `vote(tokenId, pools[], weights[])` once per epoch (7 days).  
  Guard: cannot vote twice within same epoch (`HybraTimeLib`).
* Weight is deposited into both internal & external bribes (`_vote`).
* `reset` and `poke` let user clear or refresh votes.
* RBAC (`PermissionsRegistry`) controls admin changes like `setMaxVotingNum`.

### 4.2 GaugeManager – the Router
Central book-keeper that:
1. Keeps pool→gauge registry; can deploy on demand.
2. Receives weekly HYBR from **Minter** → splits to gauges by live `weights` map.
3. Pulls fees from gauges (`distributeFees`) so internal bribes always funded.
4. Offers user helpers to
   * `claimRewards()` – harvest emission rewards across gauges (including CL NFT lists)
   * `claimBribes()` – batch claim bribe tokens.
5. Governance may `killGauge` (stop emissions, return claimable) or `reviveGauge`.

---
## 5. Role & Permission Model
All privileged calls route through **PermissionsRegistry** – an on-chain table mapping string roles to addresses.  
Important roles:
* GOVERNANCE – can upgrade factories, kill gauges, change Minter params
* GAUGE_ADMIN – add/remove factories, set bribe addresses
* VOTER_ADMIN – tune Voter settings
* GENESIS_MANAGER – bootstrap whitelists (TokenHandler)
* EMERGENCY_COUNCIL – can pause Gauges/factories

Every factory/GaugeManager checks either `Ownable` or `onlyAllowed` (owner or role). The scripts grant the deployer all roles at genesis, intended to later migrate to multisigs.

---
## 6. Auxiliary Modules
1. **TokenHandler** – global whitelist of ERC-20s; marks “connector” tokens (used by router paths) & assigns volatility buckets.
2. **HybrSwapper** – thin wrapper that lets operator swap any reward token → HYBR through whitelisted DEX aggregators (ParaSwap, 1inch …). Used by gHYBR vault to auto-compound.
3. **API Helpers** – `RewardAPI` & `veNFTAPIV1` – off-chain read helpers served via simple proxies so front-end pulls expected bribes/fees without heavy multicalls.
4. **Lens contracts** for CL maths (TickLens, QuoterV2, SugarHelper, PositionValueQuery) – pure views.

---
## 7. Deployment & Upgrade Pattern
* **Foundry scripts** deploy logic contracts once, then proxies (`TransparentUpgradeableProxy`) controlled by a single `ProxyAdmin` created in *Deploy1_Infrastructure*.
* Scripts are grouped:
  1. Deploy1 – infra (ProxyAdmin, PermissionsRegistry, VeArt)
  2. Deploy2 – token system (HYBR, veHYBR, rHYBR, gHYBR)
  3. Deploy3a – gauge factories / manager / bribe factory  
     3b – minter + rewards distributor  
     3c – voter
  4. Deploy5 – API proxies
* **Init scripts** (Set0-Set3 etc.) wire cross-contract permissions (setMinter, setGaugeManager, setDepositor …).
* Upgrade process: owner of ProxyAdmin (the governance multisig once migrated) calls `upgrade` to new implementation; state lives in proxy.

---
## 8. Typical User Journeys
### 8.1 Passive Liquidity Provider
1. Provide USDC/ETH liquidity to a CL pool via the UI.
2. Receive NFPM NFT (tokenId 123).
3. Stake NFT in **GaugeCL** – start earning HYBR emissions.
4. Optional: lock emissions via **RewardHYBR** (auto 1:1) to accumulate claimable HYBR slowly.

### 8.2 Protocol Partner
1. Protocol deploys pool XYZ/USDC.
2. Calls GaugeManager `createGauge` → gauge + bribes.
3. Every epoch deposits USDC bribe into external bribe; must satisfy **minimum-bribe rule** or veNFT stake decays.
4. Gains votes ➜ higher emissions ➜ deeper liquidity for its token.

### 8.3 veHYBR Governor
1. Locks HYBR for 4 years → gets veNFT id 777.
2. Each Thursday votes weight across 5 pools.
3. Friday the minter mints HYBR; GaugeManager distributes to gauges; gauges stream to LPs; internal/external bribes become claimable.
4. Governor claims bribes & rebases weekly.

---
## 9. Security Touch-points & Risks
1. **Unrestricted Minter** – although emission rate is programmatic, `setEmission` & `setTeamRate` are team-only; governance risk mitigated by multisig & on-chain proposals mapping.
2. **Bribe token list** – BribeFactory relies on TokenHandler + owner check; malicious token could cause grief (fee-on-transfer). Mitigated by whitelist.
3. **gHYBR custody** – all HYBR inside one veNFT; operator can vote & swap rewards, but cannot withdraw HYBR directly; withdraw flow enforces time window & fee.
4. **Gauge emergency** – EmergencyCouncil can pause gauges & allow `emergencyWithdraw()` to unblock users.
5. **Upgradeability** – All upgradeable proxies funnel through single ProxyAdmin; after governance hand-over its key must be protected (ideally in a timelock).

---
## 10. How It All Ties Together (Diagram)
```
HYBR (ERC20)
   │  mint / burn
MinterUpgradeable ───► GaugeManager ────► Gauges (V2 / CL)
   │               │       │ claimFees()        │ notifyRewardAmount()
   │               │       │                    ▼
   │               │    Bribe (internal)    Bribe (external)
   │               │
   │               └──► RewardsDistributor (rebase) ─► veNFT owners
   │
RewardHYBR ↔ Users ↔ veHYBR ↔ VoterV3 ↔ Bribes
   │                              ▲
   │                              │ weights
GovernanceHYBR (vault) ───────────┘
```

---
## 11. Gas & Upgrade Considerations
* Solidity 0.8.13, `via-ir` enabled, optimizer 200 runs in production profile.
* CL pools & Gauges are not upgradeable; risk surface is frozen once deployed.
* Factories, GaugeManager, Voter, BribeFactory, Minter are behind proxies; storage layout carefully preserved with OZ upgradeable.

---
## 12. Road-Map Hooks (from docs)
* Intent-based swap layer (`SwapRouter` + off-chain solver) that awards priority to ve stakers.
* Strategy vaults on top of CL pools (auto-rebalancer) – will stake underlying NFT into GaugeCL.
* DAO governance migration: transfer ProxyAdmin owner to `HybraGovernor` once live.

---
### In One Sentence
**Hybra** glues a ve(3,3) emission machine to both v2 and v3 liquidity, pays voters with real pool fees plus partner bribes, and auto-compounds into a single community vault so that external income overtakes inflation long before the printer can kill the token.



## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/cl/script/ExportDeployments.s.sol
Hydra/Hybra Foundry export script that runs after a full CL (concentrated-liquidity) deployment to produce a JSON package of deployed bytecode and addresses for ve33 integration tests. Targets local dev/anvil; chainId discovered via EVM; deployer inherited from FullCLDeployment. Trust model: the deployer EOA in FullCLDeployment triggers read-only code extraction and file writes; no on-chain ownership changes. Major steps: run parent deployment/load, extcodecopy bytecode for all CL components, assemble metadata and addresses into a single JSON object, write to deployments/cl-exports-local.json, and print a human-readable summary for use with vm.etch in ve33 tests.

1) Inputs/Config
- network: local
- chainId: opcode
- deployer: parent
- tokens: USDC
- tokens: USDT
- tokens: DAI
- tokens: WETH
- src file: parent
- outfile: cl-exports
- root key: cl_export
- path: deployments/
- timestamp: now
- precond: nonzero addr
- precond: has code
- loader: super.run()
- json: vm.serialize
- write: vm.writeJson
- projroot: vm.projectRoot
- comment: reads DeployCL-local.json (via parent)

2) Dependencies
- forge-std/Script.sol
- forge-std/StdJson.sol
- forge-std/console2.sol
- Foundry vm cheatcodes (serialize, writeJson, projectRoot)
- FullCLDeployment.s.sol (addresses/config)
- EVM opcodes: chainid, extcodesize, extcodecopy
- Predeployed CL contracts: CLFactory, CLPool impl, NPM, NFT Descriptor, SwapRouter, Quoter, fee modules

3) Contracts Deployed/Interacted
- CLFactory (PoolFactory): existing; bytecode export; args: N/A; post: none; out: address+bytecode
- CLPool (implementation): existing; bytecode export; args: N/A; post: none; out: address+bytecode
- NonfungiblePositionManager: existing; bytecode export; args: N/A; post: none; out: address+bytecode
- NftPositionDescriptor: existing; bytecode export; args: N/A; post: none; out: address+bytecode
- SwapRouter: existing; bytecode export; args: N/A; post: none; out: address+bytecode
- Quoter: existing; bytecode export; args: N/A; post: none; out: address+bytecode
- DynamicSwapFeeModule (swapFeeModule): existing; bytecode export; args: N/A; post: none; out: address+bytecode
- UnstakedFeeModule: existing; bytecode export; args: N/A; post: none; out: address+bytecode
- ProtocolFeeModule: existing; bytecode export; args: N/A; post: none; out: address+bytecode
- Mock tokens (USDC/USDT/DAI/WETH): existing; reference only; args: N/A; out: addresses

4) Steps (ordered)
- run(): Executes parent run; gathers code; writes JSON; logs summary; params: none
- getDeployedBytecode(addr): extcodesize>0; extcodecopy; returns bytes; params: contractAddr
- buildExportJson(...codes): serialize metadata, tokens, addresses, bytecodes; returns JSON string; params: 9 bytecodes
- getExportPath(): builds deployments/cl-exports-local.json from vm.projectRoot; returns string; params: none
- vm.writeJson(json,path): writes export; overwrites existing; params: json, path

5) Permissions/Trust
- No ownership transfers or approvals performed
- Deployer EOA (from FullCLDeployment) only labels metadata
- Read-only on-chain access; file write via Foundry VM

6) Post-Deploy Outputs
- File: deployments/cl-exports-local.json
- Root key: cl_export
- Fields: timestamp, chainId, network, deployer, token addresses
- For each component: address + on-chain bytecode
- Console summary of key addresses
- Intended use: ve33 tests via vm.etch()

7) Safety/Idempotency
- require nonzero address
- require extcodesize > 0
- Overwrite-safe writeJson; reruns produce deterministic structure
- No confirmations/gas settings needed (read-only + file IO)
- Fails early if contracts missing

8) Script Functions/Tasks
- run(): void; public override; sync — Orchestrates export after parent deployment/load
- getDeployedBytecode(address): bytes; internal view; sync — Safely fetches on-chain runtime bytecode
- buildExportJson(bytes,...): string; internal; sync — Serializes metadata, addresses, and bytecodes to JSON
- getExportPath(): string; internal view; sync — Resolves absolute output path based on project root


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/cl/script/FullCLDeployment.s.sol
Summary (≈100 words)
This Foundry Script (FullCLDeployment) deploys a full Concentrated Liquidity (CL) stack with mocks for USDC/USDT/DAI/WETH, intended as a reusable dependency providing bytecode/addresses for an upstream ve(3,3) system. It targets local/anvil and generic EVM testnets; no chain-specific logic or env inputs. Trust model: the deployer (msg.sender) deploys everything; no ownership transfers or role assignments are performed, and modules are initialized with immutable references (factory/WETH). Major steps: deploy mock tokens; deploy CL core (CLPool impl + CLFactory); deploy NFT stack (descriptor + NonfungiblePositionManager); deploy fee modules; deploy periphery (QuoterV2, SwapRouter); log addresses.

1) Inputs/Config
- ENV_VARS: none
- CLI_ARGS: none
- DEFAULT_SCALE=10000
- FEE_CAP_BPS=10000
- POOLS_INIT=[]
- FEES_INIT=[]
- NFT_NAME=CL-POS
- NFT_SYMBOL=CL-POS
- NATIVE_LABEL=ETH
- USDC_DECIMALS=6
- USDT_DECIMALS=6
- DAI_DECIMALS=18
- WETH_IS_MOCK=true
- FACTORY_IMPL=CLPool
- MSG_SENDER=deployer
- CHAIN_LOGIC=none
- PRECONDITION=compile

2) Dependencies
- forge-std/Script, console2
- MockERC20, MockNative
- CLPool, CLFactory
- NonfungibleTokenPositionDescriptor, NonfungiblePositionManager
- DynamicSwapFeeModule, CustomUnstakedFeeModule, CustomProtocolFeeModule
- QuoterV2, SwapRouter
- Foundry (forge/anvil)

3) Contracts Deployed/Interacted
- MockERC20 USDC: new("USDC","USDC",6); outputs USDC address; post: none
- MockERC20 USDT: new("USDT","USDT",6); outputs USDT address; post: none
- MockERC20 DAI: new("DAI","DAI",18); outputs DAI address; post: none
- MockNative WETH: new("WETH","WETH"); outputs WETH address; post: none
- CLPool (implementation): new(); outputs poolImplementation; post: none
- CLFactory: new({_poolImplementation: poolImplementation}); outputs poolFactory; post: factory set to impl
- NonfungibleTokenPositionDescriptor: new({_WETH9: WETH,_nativeCurrencyLabelBytes: bytes32("ETH")}); outputs nftPositionDescriptor
- NonfungiblePositionManager: new({_factory: poolFactory,_WETH9: WETH,_tokenDescriptor: nftPositionDescriptor,name: "Concentrated Liquidity Positions NFT",symbol: "CL-POS"}); outputs nonfungiblePositionManager
- DynamicSwapFeeModule: new({_factory: poolFactory,_defaultScalingFactor:10000,_defaultFeeCap:10000,_pools:[],_fees:[]}); outputs swapFeeModule
- CustomUnstakedFeeModule: new({_factory: poolFactory}); outputs unstakedFeeModule
- CustomProtocolFeeModule: new({_factory: poolFactory}); outputs protocolFeeModule
- QuoterV2: new({_factory: poolFactory,_WETH9: WETH}); outputs quoter
- SwapRouter: new({_factory: poolFactory,_WETH9: WETH}); outputs swapRouter

4) Steps (ordered)
- run(): Orchestrates deployment; sets deployer; logs summary; no ownership changes. Params: none
- run()→deploy Mocks: Deploy USDC, USDT, DAI, WETH mocks. Params: names, symbols, decimals
- _deployCLCore(): Deploy CLPool impl and CLFactory; wire impl into factory. Params: none
- _deployCLNFT(): Deploy descriptor and position manager linked to factory/WETH. Params: WETH, factory, names
- _deployCLFees(): Deploy fee modules; default caps/scales; empty init lists. Params: factory, scale=10000, cap=10000
- _deployCLPeriphery(): Deploy QuoterV2 and SwapRouter with factory/WETH. Params: factory, WETH
- run()→log: Print deployed addresses for quick reference. Params: addresses

5) Permissions/Trust
- No ownership transfers executed
- Factory holds reference to CLPool implementation
- PositionManager authorized by construction to use factory/WETH/descriptor
- Fee modules reference factory; no admin set here
- Deployer retains no special role beyond being deployer

6) Post-Deploy Outputs
- Console logs: PoolFactory, PoolImplementation, NonfungiblePositionManager, SwapRouter, Quoter
- In-memory addresses stored in contract state variables
- No on-chain registry update
- No verification/export step in script

7) Safety/Idempotency
- Not idempotent: re-runs deploy fresh instances each time
- No skip-if-deployed checks
- No wait/confirmations configured
- Gas/network handled by Foundry defaults
- No explicit revert/failure handling; constructor failures bubble

8) Script Functions/Tasks
- run(): returns void; public virtual; orchestrates deployment and logging
  Purpose: Deploy full CL stack and print addresses
- _deployCLCore(): returns (address,address); internal
  Purpose: Deploy CLPool impl and CLFactory
- _deployCLNFT(): returns (address,address); internal
  Purpose: Deploy NFT descriptor and position manager
- _deployCLFees(): returns (address,address,address); internal
  Purpose: Deploy fee modules with defaults
- _deployCLPeriphery(): returns (address,address); internal
  Purpose: Deploy quoter and router linked to factory/WETH


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/AddBribeRewards.s.sol
Summary (≈100 words)
This Foundry script adds external bribe rewards to existing gauges. It targets the active Foundry network (configured via forge-std), sourcing the GaugeManager address from JSON outputs and using hardcoded sample addresses; examples align with Ethereum mainnet token addresses. Trust model: the deployer’s EOA (PRIVATE_KEY) owns the ERC20 funds, approves the external bribe contract, and calls IBribe.notifyRewardAmount; no ownership/role changes occur. Major steps: load deployer key, read GaugeManager from JSON, resolve gauge→external bribe (or pool→gauge→bribe), approve ERC20 to the bribe, and notify reward amounts. Variants support a single token, pool-based lookup, and multiple tokens.

1) Inputs/Config
- PRIVATE_KEY
- path:Deploy3a_Gauge
- path:Deploy3c_Voting
- gauge_addr (hardcode)
- pool_addr (hardcode)
- reward_token_addr
- amount_18dec
- amount_6dec
- foundry_network
- deployer_has_tokens
- gauge_manager_addr
- bribe_exists
- gas_funds_ready

2) Dependencies
- Foundry forge-std/Script
- BaseDeployScript (internal helper: getInputPath, IO)
- OpenZeppelin IERC20
- Interfaces: IGaugeManager, IBribe
- Console logging via forge-std
- JSON artifacts with addresses (Deploy3a_GaugeFactories, Deploy3c_Voting)

3) Contracts Deployed/Interacted
- IGaugeManager (interact)
  - Methods: external_bribes(address gauge) -> address; gauges(address pool) -> address
  - Constructor/init: N/A (pre-deployed)
  - Post: read-only mapping lookups
  - Outputs: external bribe address, gauge address
- IBribe (interact)
  - Methods: notifyRewardAmount(address token, uint256 amount)
  - Constructor/init: N/A (pre-deployed)
  - Post: records new rewards for voters/claimants
  - Outputs: reward added; events emitted
- IERC20 (interact)
  - Methods: approve(address spender, uint256 amount)
  - Constructor/init: N/A (pre-deployed tokens)
  - Post: allowance set for external bribe
  - Outputs: approval success

4) Steps (ordered)
- run()
  - Note: Add one token reward to known gauge via external_bribes lookup.
  - Params: gauge (hardcoded), rewardToken, amount
- addByPool()
  - Note: Resolve pool→gauge→bribe, then add single token reward.
  - Params: pool, rewardToken, amount
- addMultipleTokens()
  - Note: For one gauge, approve and notify rewards for USDC, USDT, DAI.
  - Params: gauge, usdc/usdt/dai, per-token amounts

5) Permissions/Trust
- Deployer EOA controls ERC20 funds and approvals
- IBribe accepts deposits from any caller; holds reward funds
- IGaugeManager is read-only for this flow
- No ownership transfers or admin role changes

6) Post-Deploy Outputs
- Console logs: addresses, amounts, status
- On-chain: ERC20 approvals; bribe notifications; emitted events
- Addresses used: GaugeManager (from JSON), gauge/pool (hardcoded), bribe (resolved)
- No contract verification or artifact generation

7) Safety/Idempotency
- Not idempotent: re-runs add more rewards
- Requires: bribe/gauge nonzero; deployer funded with tokens and gas
- Approve semantics: some ERC20 (e.g., USDT) may require zero-then-approve
- No waits/confirmations configured; defaults to Foundry broadcast
- Decimals must match token (6 vs 18); hardcoded comments may be inconsistent

8) Script Functions/Tasks
- run(): none; external; sync
  - Purpose: Single-token reward to a known gauge.
- addByPool(): none; external; sync
  - Purpose: Add reward by resolving from pool address.
- addMultipleTokens(): none; external; sync
  - Purpose: Add rewards for multiple tokens to one gauge.


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Deploy1_Infrastructure.s.sol
Summary
This Foundry (forge-std) deployment script provisions core infrastructure for a ve(3,3) stack using a TransparentUpgradeableProxy pattern. It targets any EVM network configured by Foundry, using an env-provided PRIVATE_KEY for transaction signing. Trust model: the deployer EOA owns ProxyAdmin, thereby controlling upgrades for VeArtProxy; likely default ownership for newly deployed contracts remains with the deployer unless overridden. Major steps: load key and set gas price, deploy ProxyAdmin, deploy PermissionsRegistry, deploy VeArt implementation and proxy, initialize the VeArt proxy, deploy TokenHandler, then persist addresses to a JSON artifact.

1) Inputs/Config
- PRIVATE_KEY env
- GasPrice 30 gwei
- EOA funded
- RPC configured
- No CLI args
- chainId from RPC
- Foundry broadcast
- getOutputPath key
- No network switch
- Single-run script

2) Dependencies
- forge-std/Script, StdJson
- OpenZeppelin TransparentUpgradeableProxy
- OpenZeppelin ProxyAdmin
- BaseDeployScript (getOutputPath)
- Contracts: PermissionsRegistry
- Contracts: VeArtProxyUpgradeable
- Contracts: TokenHandler

3) Contracts Deployed/Interacted
- ProxyAdmin
  - method: new
  - ctor args: none
  - post: used as admin for TUP
  - outputs: proxyAdmin address
- PermissionsRegistry
  - method: new
  - ctor args: none
  - post: none
  - outputs: permissionsRegistry address
- VeArtProxyUpgradeable (impl + proxy)
  - method: impl new, then TransparentUpgradeableProxy
  - ctor/init args:
    - TUP(implementation=VeArtProxyUpgradeable impl,
          admin=ProxyAdmin,
          data="")
    - post: VeArtProxy.initialize()
  - outputs: veArtProxy (proxy) address
- TokenHandler
  - method: new
  - ctor args: address permissionsRegistry
  - post: none
  - outputs: tokenHandler address

4) Steps (ordered)
- run(): load key, set gas; start broadcast; deploy infra; save JSON.
  - params: PRIVATE_KEY, gas=30 gwei
- Deploy ProxyAdmin
  - notes: Creates upgrade admin for proxies; owned by deployer.
  - params: none
- Deploy PermissionsRegistry
  - notes: Base permissions contract; default owner likely deployer.
  - params: none
- Deploy VeArt impl
  - notes: Logic contract for veART; not callable directly by users.
  - params: none
- Deploy VeArt proxy (TUP)
  - notes: Proxy controlled by ProxyAdmin; initialized after deploy.
  - params: impl, admin=ProxyAdmin, data=""
- Initialize VeArt proxy
  - notes: Runs VeArtProxyUpgradeable.initialize() via proxy.
  - params: none
- Deploy TokenHandler
  - notes: Utility tied to PermissionsRegistry for token-related ops.
  - params: permissionsRegistry
- Stop broadcast + write JSON
  - notes: Persist addresses to file from getOutputPath().
  - params: tag="Deploy1_Infrastructure"

5) Permissions/Trust
- ProxyAdmin owned by deployer EOA; full upgrade power over VeArt proxy.
- VeArt proxy admin = ProxyAdmin; implementation can be upgraded by ProxyAdmin owner.
- PermissionsRegistry owner likely deployer (no transfer shown).
- TokenHandler depends on PermissionsRegistry for access control; no roles granted here.
- No ownership transfers or approvals executed in this phase.

6) Post-Deploy Outputs
- JSON file: getOutputPath("Deploy1_Infrastructure")
- Keys: ProxyAdmin, PermissionsRegistry, VeArtProxy, TokenHandler
- Console logs of all addresses
- No on-chain verification in-script; no registry updates

7) Safety/Idempotency
- No skip-if-deployed checks; re-running will redeploy new instances.
- initialize() will revert if proxy already initialized.
- Static gas price: vm.txGasPrice(30 gwei).
- No confirmation waits; Foundry handles nonce/tx send.
- Failure aborts execution; no retries.

8) Script Functions/Tasks
- run(): external; returns void; exported via forge script; sync
  - purpose: Deploys ProxyAdmin, PermissionsRegistry, VeArt proxy, TokenHandler; saves addresses.


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Deploy2_TokenSystem.s.sol
Summary
Deploy2_TokenSystem is a Foundry script that deploys the HYBR token and associated ve(3,3) components. It targets whichever network Foundry broadcasts to (anvil, testnet, mainnet) using a funded PRIVATE_KEY. Trust model: the deployer key signs all transactions; no ownership transfers occur here, so any Ownable roles remain with the deployer by default unless contracts are non-ownable. Major steps: read infra JSON for VeArtProxy, start broadcast, deploy HYBR, deploy VotingEscrow(HYBR, VeArtProxy), deploy RewardHYBR(HYBR, VotingEscrow), deploy GovernanceHYBR/GrowthHYBR(HYBR, VotingEscrow), stop broadcast, and persist addresses to a Deploy2_TokenSystem JSON file.

1) Inputs/Config
- Env vars
  - PRIVATE_KEY
- CLI args
  - none
- Constants/defaults
  - Deploy1_Infrastr.
  - Deploy2_TokenSys.
  - key: .VeArtProxy
- Network logic
  - Foundry broadcast
- Preconditions
  - Funded key
  - Infra JSON
  - VeArtProxy ok

2) Dependencies
- Tools/libs
  - forge-std/Script
  - forge-std/StdJson
  - BaseDeployScript
- Prior artifacts
  - VeArtProxy addr from Deploy1_Infrastructure JSON
- Contracts
  - HYBR.sol
  - RewardHYBR.sol
  - GovernanceHYBR.sol (GrowthHYBR)
  - VotingEscrow.sol

3) Contracts Deployed/Interacted
- HYBR
  - Method: new (direct)
  - Constructor args: none
  - Post-deploy: none
  - Outputs: HYBR address
- VotingEscrow
  - Method: new (direct)
  - Constructor args: token=HYBR, artProxy=VeArtProxy
  - Post-deploy: none
  - Outputs: VotingEscrow address
- RewardHYBR
  - Method: new (direct)
  - Constructor args: token=HYBR, ve=VotingEscrow
  - Post-deploy: none
  - Outputs: RewardHYBR address
- GovernanceHYBR (alias GrowthHYBR)
  - Method: new (direct)
  - Constructor args: token=HYBR, ve=VotingEscrow
  - Post-deploy: none
  - Outputs: GrowthHYBR address

4) Steps (ordered)
- run()
  - Note: Orchestrates deployment and persistence
  - Params: none
- vm.startBroadcast()
  - Note: Begin signed txs with deployer
  - Params: deployerAddress
- new HYBR()
  - Note: Deploy base ERC20/ve asset
  - Params: none
- new VotingEscrow()
  - Note: Deploy ve lock contract
  - Params: token=HYBR, artProxy
- new RewardHYBR()
  - Note: Deploy reward token wrapper
  - Params: token=HYBR, ve
- new GrowthHYBR()
  - Note: Deploy governance growth token
  - Params: token=HYBR, ve
- vm.stopBroadcast()
  - Note: End broadcast session
  - Params: none
- vm.writeJson()
  - Note: Persist addresses to file
  - Params: json, path

5) Permissions/Trust
- Deployer controls tx broadcasting
- No ownership transfers performed
- Roles/owners remain defaults
- No approvals or allowances set

6) Post-Deploy Outputs
- Addresses saved to JSON via getOutputPath("Deploy2_TokenSystem")
- JSON keys: HYBR, RewardHYBR, GrowthHYBR, VotingEscrow
- Console logs of deployed addresses
- No on-chain verification executed

7) Safety/Idempotency
- No skip-if-deployed checks
- No confirmations/waits configured
- Gas/network via Foundry defaults
- Failure handling: none beyond revert
- Requires prior VeArtProxy address

8) Script Functions/Tasks
- run(): void; external; sync
  - Purpose: Deploy HYBR, VotingEscrow, RewardHYBR, GrowthHYBR and store addresses


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Deploy3a1_GaugeFactories.s.sol
Hydra Finance – Deploy3a1_GaugeFactories (Foundry)

This Foundry script deploys and initializes two upgradeable gauge factories—standard GaugeFactory and concentrated-liquidity GaugeFactoryCL—behind OpenZeppelin TransparentUpgradeableProxy. It targets any EVM network configured by Foundry RPC and relies on prior deployments for ProxyAdmin, PermissionsRegistry, and RewardHYBR (rHYBR). Trust: upgrades are controlled by the previously deployed ProxyAdmin; runtime permissions are enforced via PermissionsRegistry; an EOA loaded from PRIVATE_KEY performs deployment and post-init configuration (setRHYBR). Major steps: load prior JSON artifacts, deploy implementations, create proxies with initialize(permissionsRegistry), set rHYBR on both factories, then persist new proxy addresses to an output JSON consumed by later stages.

1) Inputs/Config
- PRIVATE_KEY
- infra JSON path
- token JSON path
- ProxyAdmin addr
- PermissionsReg
- RewardHYBR addr
- getInputPath()
- getOutputPath()
- Foundry vm
- EOA deployer
- RPC/network
- Prior files exist

2) Dependencies
- forge-std/Script (Foundry)
- forge-std/StdJson
- OpenZeppelin TransparentUpgradeableProxy
- BaseDeployScript helper
- contracts/factories/GaugeFactory.sol
- contracts/CLGauge/GaugeFactoryCL.sol
- Prior artifacts: ProxyAdmin
- Prior artifacts: PermissionsRegistry
- Prior artifacts: RewardHYBR (rHYBR)

3) Contracts Deployed/Interacted
- GaugeFactory (impl)
  - Method: new (implementation only), proxied via TransparentUpgradeableProxy
  - Init (proxy): initialize(permissionsRegistry)
  - Post-deploy: setRHYBR(rHYBR)
  - Outputs: GaugeFactory proxy address
- GaugeFactoryCL (impl)
  - Method: new (implementation only), proxied via TransparentUpgradeableProxy
  - Init (proxy): initialize(permissionsRegistry)
  - Post-deploy: setRHYBR(rHYBR)
  - Outputs: GaugeFactoryCL proxy address
- TransparentUpgradeableProxy (x2)
  - Constructor: (implementation, proxyAdmin, abi.encodeWithSelector(initialize, permissionsRegistry))

4) Steps (ordered)
- run()
  - Note: Orchestrates full deployment and persistence
  - Params: none
- Load env key
  - Note: vm.envUint("PRIVATE_KEY") -> deployer EOA
  - Params: PRIVATE_KEY
- Read prior JSON
  - Note: Load ProxyAdmin, PermissionsRegistry, RewardHYBR
  - Params: infraPath, tokenPath
- startBroadcast
  - Note: Begin sending transactions as deployer
  - Params: deployer
- Deploy GaugeFactory impl
  - Note: new GaugeFactory()
  - Params: none
- Deploy GaugeFactory proxy
  - Note: Initialize with permissionsRegistry
  - Params: impl, proxyAdmin, permissionsRegistry
- setRHYBR on GaugeFactory
  - Note: Configure reward token
  - Params: rHYBR
- Deploy GaugeFactoryCL impl
  - Note: new GaugeFactoryCL()
  - Params: none
- Deploy GaugeFactoryCL proxy
  - Note: Initialize with permissionsRegistry
  - Params: impl, proxyAdmin, permissionsRegistry
- setRHYBR on GaugeFactoryCL
  - Note: Configure reward token
  - Params: rHYBR
- stopBroadcast
  - Note: End transaction broadcast
  - Params: none
- Write JSON outputs
  - Note: Save proxy addresses under factories object
  - Params: output path

5) Permissions/Trust
- Upgrades: Controlled by ProxyAdmin from prior deployment for both proxies
- Runtime access: Enforced by PermissionsRegistry
- Mutation: setRHYBR executed by deployer; requires appropriate role/owner
- Ownership transfers: None performed by this script
- Deployer: No proxy admin rights unless previously granted

6) Post-Deploy Outputs
- JSON written at getOutputPath("Deploy3a1_GaugeFactories")
- Keys: factories.GaugeFactory, factories.GaugeFactoryCL
- Console logs of addresses
- No built-in verification; artifacts managed by Foundry

7) Safety/Idempotency
- No skip-if-deployed checks; re-runs redeploy and overwrite JSON
- No confirmations/wait settings; defaults used
- Assumes prior JSON exists with valid addresses
- Potential revert if setRHYBR authorization fails
- Gas/network pulled from Foundry config

8) Script Functions/Tasks
- run(): void; external; exported; sync
  - Purpose: Deploy and initialize GaugeFactory and GaugeFactoryCL proxies, set rHYBR, and persist addresses


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Deploy3a2_GaugeManagerAndBribes.s.sol
Summary
This Foundry script deploys and wires up upgradeable GaugeManager and BribeFactoryV3 contracts using a TransparentUpgradeableProxy pattern governed by a pre-deployed ProxyAdmin. It targets whichever EVM network the referenced config and prior deployment JSONs describe. The deployer signs via PRIVATE_KEY; upgrade control resides with ProxyAdmin from prior infra. Major steps: load prior addresses, deploy GaugeManager implementation, deploy its proxy, initialize GaugeManager with VotingEscrow, factories, registries, and NFPM; deploy BribeFactoryV3 implementation and proxy (no init); then persist resulting addresses to a new deployment JSON. No verification or idempotency checks are included; script assumes prior artifacts exist and are correct.

1) Inputs/Config
- Env: PRIVATE_KEY
- JSON: Deploy1_Infrastructure
- JSON: Deploy2_TokenSystem
- JSON: Deploy3a1_GaugeFactories
- JSON: config.json
- Read: ProxyAdmin
- Read: PermissionsRegistry
- Read: TokenHandler
- Read: v2Factory
- Read: VotingEscrow
- Read: clFactory
- Read: NFPM
- Read: GaugeFactory
- Read: GaugeFactoryCL
- Pre: Prior JSON exist
- Pre: ProxyAdmin set
- Pre: NFPM reachable
- Pre: Factories deployed

2) Dependencies
- Tools: Foundry forge-std (Script, StdJson)
- Libs: OpenZeppelin TransparentUpgradeableProxy
- Prior: ProxyAdmin, PermissionsRegistry, TokenHandler
- Prior: VotingEscrow
- Prior: v2 pair factory
- Prior: CL factory (Algebra/CL)
- Prior: NonfungiblePositionManager (NFPM)
- Prior: GaugeFactory (v2)
- Prior: GaugeFactoryCL (v3/CL)

3) Contracts Deployed/Interacted
- GaugeManager
  - Method: proxy (TransparentUpgradeableProxy); impl via new GaugeManager()
  - Proxy admin: ProxyAdmin from infra JSON
  - Proxy data: empty (no constructor-encoded init)
  - Init: initialize(ve, tokenHandler, gaugeFactory, gaugeFactoryCL, pairFactory, clFactory, permissionsRegistry, nfpm)
  - Post: Ready to create/manage gauges for v2 and CL pools
  - Outputs: GaugeManager proxy address persisted
- BribeFactoryV3
  - Method: proxy (TransparentUpgradeableProxy); impl via new BribeFactoryV3()
  - Proxy admin: ProxyAdmin from infra JSON
  - Proxy data: empty; no init in this script
  - Post: No additional calls
  - Outputs: BribeFactoryV3 proxy address persisted

4) Steps (ordered)
1. run() — entrypoint; loads configs; acquires deployer
   - Params: PRIVATE_KEY
2. startBroadcast() — begin txs from deployer
   - Params: deployer addr
3. new GaugeManager() — deploy implementation
   - Params: none
4. new TransparentUpgradeableProxy(GM) — deploy GM proxy
   - Params: impl, ProxyAdmin, ""
5. GaugeManager.initialize(...) — wire dependencies
   - Params: ve, tokenHandler, GF, GFCL, v2F, clF, PR, NFPM
6. new BribeFactoryV3() — deploy implementation
   - Params: none
7. new TransparentUpgradeableProxy(BF) — deploy BF proxy
   - Params: impl, ProxyAdmin, ""
8. stopBroadcast() — end txs
   - Params: none
9. writeJson() — persist addresses
   - Params: key: GaugeManager, BribeFactoryV3

5) Permissions/Trust
- Upgrade control: ProxyAdmin (external owner, from infra JSON) controls both proxies
- Runtime control: GaugeManager uses PermissionsRegistry for permission checks
- Deployer: Only signs deployment, no retained admin unless also ProxyAdmin
- No ownership transfers or role grants performed in this script

6) Post-Deploy Outputs
- Addresses saved to: Deploy3a2_GaugeManagerAndBribes JSON
- Keys: GaugeManager, BribeFactoryV3
- No on-chain verification in script
- No artifact export beyond JSON

7) Safety/Idempotency
- No skip-if-deployed checks; reruns will redeploy new proxies
- No confirmation waits; default Foundry broadcasting
- Gas/network use defaults; determined by RPC/network config
- Failure handling: rely on EVM reverts; no try/catch

8) Script Functions/Tasks
- run(): void; external; sync
  - Purpose: Deploy GM and BF proxies, initialize GM, save addresses


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Deploy3a3_SetupPermissions.s.sol
Summary
This Foundry script finalizes gauge permissions and configuration for a ve(3,3) deployment. It targets any EVM network configured via Foundry (RPC/profile) and uses a deployer PRIVATE_KEY for on-chain writes. Trust model: the deployer’s key is used to broadcast transactions and is granted GAUGE_ADMIN in PermissionsRegistry; GaugeManager is pointed to the chosen BribeFactoryV3 and PermissionsRegistry. Major steps: load prior deployment JSON, start broadcast, grant GAUGE_ADMIN to deployer, set GaugeManager’s bribe factory and permissions registry, stop broadcast, and write a consolidated addresses JSON including factories and managers for downstream use.

1) Inputs/Config
- env: PRIVATE_KEY
- in: Deploy1_Infrastructure
- in: Deploy3a2_GM_Bribes
- in: Deploy3a1_Factories
- out: Deploy3a_GaugeFacs
- net: Foundry RPC cfg
- pre: prior JSON exist
- keys: deployer owns key

2) Dependencies
- Foundry forge-std (Script, StdJson)
- BaseDeployScript (getInputPath/getOutputPath)
- Contracts: PermissionsRegistry, GaugeManager
- Foundry VM (broadcast, env, fs, json)

3) Contracts Deployed/Interacted
- PermissionsRegistry
  - method: interact (setRoleFor)
  - args: (deployer, "GAUGE_ADMIN")
  - post: deployer gains GAUGE_ADMIN
  - outputs: role assignment state
- GaugeManager
  - method: interact (setBribeFactory)
  - args: (bribeFactoryV3)
  - method: interact (setPermissionsRegistry)
  - args: (permissionsRegistry)
  - post: manager configured with bribe factory + registry
  - outputs: stored addresses updated

4) Steps (ordered)
- loadEnv — Read key; derive deployer; prepare broadcast; params: PRIVATE_KEY
- readInfra — Load PermissionsRegistry from Deploy1_Infrastructure; params: .PermissionsRegistry
- readGM — Load GaugeManager/BribeFactoryV3; params: .GaugeManager, .BribeFactoryV3
- startBroadcast — Begin signing with deployer; params: deployer
- setRole — Grant GAUGE_ADMIN to deployer; params: setRoleFor(deployer,"GAUGE_ADMIN")
- setBribeFactory — Point GaugeManager to BribeFactoryV3; params: setBribeFactory(bribeFactoryV3)
- setPermReg — Ensure GaugeManager registry pointer; params: setPermissionsRegistry(permissionsRegistry)
- stopBroadcast — End signing session; params: none
- readFactories — Read GaugeFactory addresses; params: .GaugeFactory, .GaugeFactoryCL
- writeJson — Persist merged outputs; params: Deploy3a_GaugeFactories

5) Permissions/Trust
- Deployer granted GAUGE_ADMIN in PermissionsRegistry
- GaugeManager uses configured BribeFactoryV3 and PermissionsRegistry
- No ownership transfers performed
- Centralized to deployer until roles are further delegated

6) Post-Deploy Outputs
- JSON: Deploy3a_GaugeFactories
  - GaugeFactory
  - GaugeFactoryCL
  - GaugeManager
  - BribeFactoryV3
- Console logs for traceability
- No verification/export beyond JSON

7) Safety/Idempotency
- Requires prior JSON files to exist and contain valid addresses
- No skip-if-set checks; subsequent runs may overwrite pointers
- Role assignment may be idempotent (contract-dependent)
- No waits/confirmations; Foundry default gas/network settings
- Failure on missing/invalid JSON or access control reverts

8) Script Functions/Tasks
- run(): void; external; sync; exported by script
  - Purpose: Configure roles and manager pointers; merge and write deployment artifacts.


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Deploy3b1_MinterRewards.s.sol
Summary (≈100 words)
Deploy3a1_MinterRewards is a Foundry (forge-std) script that deploys an upgradeable Minter and a RewardsDistributor, wiring them to previously deployed core addresses. It runs on any EVM network configured via Foundry RPC and signs with PRIVATE_KEY. Trust model: a ProxyAdmin controls Minter upgrades; the deployer is assigned as team; RewardsDistributor ownership/roles are unspecified here. Major steps: load prior deployment JSONs; deploy RewardsDistributor (uses VotingEscrow); deploy Minter implementation and a TransparentUpgradeableProxy; initialize Minter with GaugeManager, VotingEscrow, RewardsDistributor; set team to deployer; write resulting addresses to a new JSON file. No verification, idempotency guards, confirmations, or network gating are present.

1) Inputs/Config (≤20 chars each)
- PRIV_KEY (env)
- in:Deploy1_Infra
- in:Deploy2_Token
- in:Deploy3a_Gauges
- ProxyAdmin addr
- VotingEscrow addr
- RewardHYBR addr
- GaugeManager addr
- out:Deploy3b1_JSON
- RPC_URL (implicit)
- chainId (implicit)
- pre:prev JSON exist
- pre:funded deployer
- pre:ProxyAdmin ok

2) Dependencies
- Tools/libs: Foundry forge-std (Script, StdJson), Foundry vm cheatcodes
- Proxy: OpenZeppelin TransparentUpgradeableProxy
- Base helper: BaseDeployScript (getInputPath/getOutputPath)
- Prior artifacts: JSON outputs from Deploy1_Infrastructure, Deploy2_TokenSystem, Deploy3a_GaugeFactories
- Referenced contracts: VotingEscrow, GaugeManager, RewardHYBR (loaded/log only)

3) Contracts Deployed/Interacted
- RewardsDistributor
  - method: new (direct deploy)
  - ctor args: votingEscrow
  - post-deploy: none
  - outputs: RewardsDistributor address
- MinterUpgradeable (proxy pattern)
  - impl method: new MinterUpgradeable()
  - proxy: TransparentUpgradeableProxy(admin=ProxyAdmin, impl=MinterUpgradeable, data="")
  - init: minter.initialize(gaugeManager, votingEscrow, rewardsDistributor)
  - post-deploy: minter.setTeam(deployer)
  - outputs: Minter (proxy) address
- Interacted only
  - VotingEscrow: passed to RewardsDistributor and Minter
  - GaugeManager: passed to Minter.initialize
  - RewardHYBR: loaded for logging (unused otherwise)

4) Steps (ordered)
1. run()
   - Note: Orchestrates deployment using env key and prior JSON outputs
   - Params: none
2. new RewardsDistributor()
   - Note: Deploy RewardsDistributor bound to VotingEscrow
   - Params: votingEscrow
3. new MinterUpgradeable()
   - Note: Deploy the implementation logic contract
   - Params: none
4. new TransparentUpgradeableProxy()
   - Note: Create proxy controlled by ProxyAdmin, pointing to Minter impl
   - Params: impl, proxyAdmin, data=""
5. minter.initialize()
   - Note: Wire Minter to GaugeManager, VotingEscrow, RewardsDistributor
   - Params: gaugeManager, votingEscrow, rewardsDistributor
6. minter.setTeam()
   - Note: Assign deployer as team authority
   - Params: deployer
7. vm.writeJson()
   - Note: Persist addresses to output JSON file
   - Params: json, outPath

5) Permissions/Trust
- Upgrades: ProxyAdmin controls Minter proxy upgrades
- Team role: deployer set via minter.setTeam(deployer)
- Minter init: callable once; already executed by deployer
- RewardsDistributor: ownership/roles not set here (assumed default)
- Deployer key holds transactional authority during script

6) Post-Deploy Outputs
- Addresses: Minter (proxy), RewardsDistributor
- File: getOutputPath("Deploy3b1_MinterRewards")
- JSON keys: contracts.Minter, contracts.RewardsDistributor
- No on-chain verification or registry writes included

7) Safety/Idempotency
- No skip-if-deployed checks; re-running redeploys new instances
- No confirmations/waits; default Foundry broadcast behavior
- Gas/network settings: default via RPC; no overrides
- Failure handling: tx reverts on failed init; no retries
- Upgradeability: Transparent proxy with empty init data; explicit initialize thereafter

8) Script Functions/Tasks
- run(): void; external; sync
  - Purpose: Load config, deploy RD + Minter proxy, initialize, set team, save JSON
- getInputPath(tag): string; internal; sync
  - Purpose: Resolve input JSON file path by tag
- getOutputPath(tag): string; internal; sync
  - Purpose: Resolve output JSON file path by tag


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Deploy3b2_SetupConnections.s.sol
Summary
This Foundry script wires together pre-deployed HYBRA governance, emission, and gauge components. It targets the currently selected Foundry network/RPC and uses a single externally owned account from PRIVATE_KEY for all privileged calls. Trust model: the broadcaster must own/admin the GovernanceHYBR, GaugeManager, and RewardsDistributor to set critical roles. Major steps: load prior deployment JSONs, parse addresses, broadcast transactions to set Minter on GaugeManager, authorize Minter as RewardsDistributor depositor, and register RewardsDistributor and GaugeManager on GovernanceHYBR. No new contracts are deployed; the script finalizes cross-contract permissions so emissions and rewards can flow correctly.

1) Inputs/Config
- PRIVATE_KEY
- tokenPath
- factoriesPath
- minterPath
- json.GrowthHYBR
- json.GaugeManager
- json.Minter
- json.RewardsDist
- foundry_rpc
- owns_admin_rights
- json_files_exist

2) Dependencies
- forge-std/Script
- forge-std/StdJson
- Foundry broadcast
- BaseDeployScript
- MinterUpgradeable
- RewardsDistributor
- GaugeManager
- GovernanceHYBR

3) Contracts Deployed/Interacted
- GaugeManager: interact; setMinter(minterAddr); post: none; outputs: role set
- RewardsDistributor: interact; setDepositor(minterAddr); post: none; outputs: depositor set
- GovernanceHYBR (alias GrowthHYBR): interact; setRewardsDistributor(rdAddr), setGaugeManager(gmAddr); post: none; outputs: refs set

4) Steps (ordered)
- run(): Loads JSON, broadcasts, performs role wiring; params: PRIVATE_KEY, file paths
- setMinter(): Sets GaugeManager.minter to Minter; params: minter
- setDepositor(): Sets RewardsDistributor.depositor to Minter; params: minter
- setRewardsDist(): Sets GovernanceHYBR.rewardsDistributor; params: rewardsDistributor
- setGaugeMgr(): Sets GovernanceHYBR.gaugeManager; params: gaugeManager

5) Permissions/Trust
- Deployer: holds PRIVATE_KEY; must be owner/admin of GaugeManager to call setMinter
- Deployer: must be admin of RewardsDistributor to call setDepositor
- Deployer: must be owner/admin of GovernanceHYBR to set rewardsDistributor and gaugeManager
- Post-state: Minter controls emissions; RewardsDistributor accepts deposits only from Minter; GovernanceHYBR references GaugeManager and RewardsDistributor

6) Post-Deploy Outputs
- No on-chain verifications in script
- Addresses sourced from prior JSONs; none newly written back
- Console logs provide visibility only

7) Safety/Idempotency
- No skip-if-already-set checks; repeated runs may revert or overwrite
- No waits/confirmations; relies on Foundry defaults
- No gas overrides; network and RPC set via Foundry config
- No try/catch; failures abort script

8) Script Functions/Tasks
- run(): external; returns void; exported; Purpose: Wire roles among pre-deployed contracts using env key and JSON addresses.


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Deploy3c_Voting.s.sol
Summary
This Foundry (forge-std) deployment script provisions and wires the voting layer. It targets any EVM network configured via Foundry broadcast and relies on previously stored deployment JSONs. Trust model: an existing ProxyAdmin (address loaded from prior infra) controls upgrades to a TransparentUpgradeableProxy for VoterV3; the deployer uses PRIVATE_KEY to broadcast setup transactions; PermissionsRegistry address is injected into VoterV3 during initialization. Major steps: load prior addresses, deploy VoterV3 implementation, deploy and initialize a proxy, set the Voter in GaugeManager and VotingEscrow, and persist the voter proxy address to an outputs JSON.

1) Inputs/Config
- PRIVATE_KEY
- Deploy1_Infras
- Deploy2_Token
- Deploy3a_Factory
- ProxyAdmin addr
- PermRegistry addr
- TokenHandler addr
- VotingEscrow addr
- GaugeManager addr
- BaseDeploy IO
- Foundry broadcast
- Prior JSON files
- Deployer funded
- Proxy admin set
- Chain RPC active
- Same chain addrs
- Nonce clean
- Write perms FS

2) Dependencies
- forge-std/Script
- forge-std/StdJson
- OZ TransparentProxy
- VoterV3.sol
- GaugeManager.sol
- VotingEscrow.sol
- BaseDeployScript
- Prior infra JSONs

3) Contracts Deployed/Interacted
- VoterV3 (implementation)
  - method: new (implementation only)
  - constructor/init args: none (logic contract)
  - post-deploy: used as implementation for proxy
  - outputs: voterImpl address (not persisted)
- TransparentUpgradeableProxy (Voter proxy)
  - method: deploy proxy (OZ TransparentUpgradeableProxy)
  - init selector: VoterV3.initialize
  - init args: (__ve=votingEscrow, _tokenHandler=tokenHandler, _gaugeManager=gaugeManager, _permissionRegistry=permissionsRegistry)
  - post-deploy: cast to VoterV3 at proxy address
  - outputs: voter proxy address (saved as VoterV3 and Voter)
- GaugeManager (existing)
  - interaction: setVoter(voter)
  - effect: registers Voter in GaugeManager
- VotingEscrow (existing)
  - interaction: setVoter(voter)
  - effect: authorizes Voter in VotingEscrow

4) Steps (ordered)
- run.loadEnv
  - Note: Load deployer key and derive broadcaster address
  - Params: PRIVATE_KEY
- run.readFiles
  - Note: Load prior deployment JSONs for infra, token, factories
  - Params: Deploy1_Infras, Deploy2_Token, Deploy3a_Factory
- run.parseAddrs
  - Note: Parse ProxyAdmin, registries, managers, escrow addresses
  - Params: ProxyAdmin, PermRegistry, TokenHandler, VotingEscrow, GaugeManager
- run.deployVoterImpl
  - Note: Deploy VoterV3 implementation (logic contract)
  - Params: none
- run.deployVoterProxy
  - Note: Deploy proxy and initialize VoterV3 with core addresses
  - Params: impl, proxyAdmin, init args
- run.wireGaugeManager
  - Note: Set voter in GaugeManager for voting flows
  - Params: voter proxy
- run.wireVotingEscrow
  - Note: Set voter in VotingEscrow for authorization
  - Params: voter proxy
- run.persistOutputs
  - Note: Save voter proxy address under VoterV3 and Voter keys
  - Params: output path

5) Permissions/Trust
- ProxyAdmin: full upgrade power over Voter proxy
- Deployer: transient broadcaster privileges only
- VoterV3: governed by PermissionsRegistry address
- GaugeManager: recognizes Voter set by script
- VotingEscrow: authorizes Voter set by script

6) Post-Deploy Outputs
- JSON: saves voter proxy under keys VoterV3 and Voter
- Path: getOutputPath("Deploy3c_Voting")
- No etherscan verification
- Console logs of addresses

7) Safety/Idempotency
- No skip-if-deployed checks
- No reentrancy or retries
- No confirmations/waits specified
- Relies on correct prior JSONs
- Risks: setVoter may revert if already set or unauthorized
- Gas/network: defaults via Foundry config/RPC

8) Script Functions/Tasks
- run(): void; external; exported by Forge Script
  - Purpose: Orchestrates deploy, initialization, wiring, and persistence in a single broadcast run.


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Deploy5_APIs.s.sol
Hydra Finance Deploy5_APIs (Foundry) deploys two upgradeable API contracts behind TransparentUpgradeableProxy: RewardAPI and veNFTAPIV1. It targets any network configured via Foundry RPC, using a deployer EOA from PRIVATE_KEY. The script trusts an existing ProxyAdmin for upgrades and sets both API owners to the deployer (transferable later to multisig). Major steps: load prior deployment JSONs to fetch core addresses (VoterV3, GaugeManager, GaugeFactory, GaugeFactoryCL, RewardsDistributor, ProxyAdmin), set gas price, deploy implementations, deploy proxies with initialize data, set owners, stop broadcast, and write resulting addresses to a new JSON output.

1) Inputs/Config
- Env vars
  - PRIVATE_KEY
- CLI args
  - none
- Constants/defaults
  - gasPrice=80 gwei
  - input:Deploy1_Infra
  - input:Deploy3c_Vote
  - input:Deploy3a_Gauges
  - input:Deploy3b1_Mint
- Network logic
  - Foundry broadcast
  - chain from RPC
- Preconditions
  - ProxyAdmin exists
  - JSONs present
  - Deployer has ETH
  - Init funcs ready

2) Dependencies
- Tools/libs: forge-std (Script, StdJson), OpenZeppelin TransparentUpgradeableProxy
- BaseDeployScript helpers: getInputPath, getOutputPath
- Prior contracts/addresses: ProxyAdmin, VoterV3, GaugeManager, GaugeFactory, GaugeFactoryCL, RewardsDistributor

3) Contracts Deployed/Interacted
- RewardAPI
  - Method: deploy impl (new), proxy (TransparentUpgradeableProxy)
  - Init: initialize(voter, gaugeManager, gaugeFactory, gaugeFactoryCL)
  - Post-deploy: setOwner(deployer)
  - Outputs: RewardAPI proxy address
- veNFTAPIV1
  - Method: deploy impl (new), proxy (TransparentUpgradeableProxy)
  - Init: initialize(voter, rewardsDistributor, gaugeFactory, gaugeFactoryCL, gaugeManager)
  - Post-deploy: setOwner(deployer)
  - Outputs: veNFTAPIV1 proxy address

4) Steps (ordered)
- readConfig
  - Note: Load prior JSONs and decode required addresses
  - Params: Deploy1_Infrastructure, Deploy3c_Voting, Deploy3a_GaugeFactories, Deploy3b1_MinterRewards
- setGasAndBroadcast
  - Note: Set 80 gwei, start broadcast as deployer EOA
  - Params: deployer, 80 gwei
- deployRewardAPI
  - Note: Deploy impl, proxy with initialize, cast to RewardAPI
  - Params: proxyAdmin, voter, gaugeManager, gaugeFactory, gaugeFactoryCL
- deployVeNFTAPIV1
  - Note: Deploy impl, proxy with initialize, cast to veNFTAPIV1
  - Params: proxyAdmin, voter, rewardsDistributor, gaugeFactory, gaugeFactoryCL, gaugeManager
- setOwnership
  - Note: Set API owners to deployer EOA
  - Params: deployer
- stopBroadcast
  - Note: End broadcast session
  - Params: none
- saveJSON
  - Note: Write addresses to Deploy5_APIs output JSON
  - Params: RewardAPI, veNFTAPIV1

5) Permissions/Trust
- Upgrades: ProxyAdmin controls both proxies
- Ownership: RewardAPI.owner = deployer; veNFTAPIV1.owner = deployer
- Future: Owners can be transferred to multisig
- External refs: VoterV3, GaugeManager, GaugeFactory/CL, RewardsDistributor referenced by initializers; no ownership taken

6) Post-Deploy Outputs
- Console: Deployed addresses printed
- Files: getOutputPath("Deploy5_APIs") JSON with keys: RewardAPI, veNFTAPIV1
- Verification: Not included in script

7) Safety/Idempotency
- Idempotency: No skip-if-deployed; always redeploy new proxies
- Confirms: No wait/confirm logic
- Gas/Network: vm.txGasPrice(80 gwei); network from Foundry RPC
- Failure handling: None beyond default revert; requires prior JSONs and valid addresses

8) Script Functions/Tasks
- run(): void; external; exported; sync
  - Purpose: Deploy RewardAPI and veNFTAPIV1 proxies, initialize, set owners, and persist addresses


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/DeploySwapper.s.sol
Overview
This Foundry (forge-std) deployment suite provisions a modular swapper for HYBR: deploys HybrSwapper, whitelists DEX aggregators, and wires it into the GrowthHYBR governance module. Target is any EVM network configured by Foundry; constants reference mainnet aggregator addresses. The deployer’s PRIVATE_KEY controls both the GrowthHYBR (must be authorized to set the swapper) and the newly deployed HybrSwapper (owns whitelist changes). Major steps: load prior deployments (HYBR, GrowthHYBR), deploy HybrSwapper(hybr), whitelist Paraswap and 1inch (optional OKX), set swapper in GrowthHYBR, verify, and persist outputs. An UpgradeSwapper script redeploys a new HybrSwapper and rebinds GrowthHYBR.

1) Inputs/Config
- env: PRIVATE_KEY
- in: Deploy2_TokenSystem
- in: Deploy4_GrowthHYBR
- in: Deploy4_growthHYBR
- const: PARASWAP
- const: ONE_INCH
- const: OKX
- pre: HYBR exists
- pre: GrowthHYBR exists
- pre: Auth to setSwapper
- io: getInputPath()
- io: getOutputPath()
- file: vm.readFile()
- json: vm.parseJson()

2) Dependencies
- forge-std/Script (Foundry)
- console logging (forge-std)
- BaseDeployScript (path utils, file IO)
- GovernanceHYBR.sol (GrowthHYBR type)
- HybrSwapper.sol (swapper impl)
- ISwapper.sol (interface)
- Prior artifacts: HYBR token, GrowthHYBR address JSONs

3) Contracts Deployed/Interacted
- HybrSwapper
  - method: new (direct deploy)
  - constructor: (address hybrToken)
  - post: setAggregatorWhitelist(Paraswap, true); setAggregatorWhitelist(1inch, true); optional OKX
  - outputs: swapper address
- GrowthHYBR (existing)
  - method: setSwapper(address newSwapper)
  - init args: n/a (existing)
  - post: growthHYBR.swapper() == HybrSwapper
  - outputs: bound swapper address

4) Steps (ordered)
- DeploySwapper.run: Orchestrates full deployment and configuration; saves output. key: PRIVATE_KEY
- _loadExistingContracts: Reads HYBR and GrowthHYBR from JSON. key: file paths
- _deploySwapper: Deploys HybrSwapper(hybr). key: hybr
- _configureSwapper: Whitelists Paraswap, 1inch, optional OKX. key: aggregator addrs
- _connectSwapper: Calls growthHYBR.setSwapper(swapper). key: swapper addr
- _verifyConfiguration: Asserts swapper set and whitelists present. key: none
- _saveDeployment: Writes JSON output to disk. key: output path
- UpgradeSwapper.run: Deploys new swapper, re-whitelists, rebinds GrowthHYBR. key: PRIVATE_KEY

5) Permissions/Trust
- HybrSwapper owner: deployer initially; controls aggregator whitelist (and any owner-restricted ops).
- GrowthHYBR control: deployer must be authorized (owner/governance) to call setSwapper.
- Post-deploy control: GrowthHYBR uses the configured swapper for swaps; deployer retains whitelist control unless ownership transferred.

6) Post-Deploy Outputs
- Runtime checks: require growthHYBR.swapper() == hybrSwapper; require whitelisted(Paraswap, 1inch).
- Artifacts/addresses: Intended to save HybrSwapper, HYBR, GrowthHYBR, aggregators, timestamp.
- Registry/exports: output file via getOutputPath("Deploy_Swapper").
- Note: current _saveDeployment only writes timestamp (logic bug); addresses not persisted.

7) Safety/Idempotency
- Idempotency: No skip-if-deployed; always redeploys and overwrites swapper in GrowthHYBR.
- Confirmations: None explicit; Foundry broadcast handles tx submission.
- Gas/network: No manual gas params; uses default RPC.
- Failure handling: Hard require checks on config; script reverts if misconfigured.
- Inputs robustness: Requires valid file paths; case mismatch risk (Deploy4_GrowthHYBR vs Deploy4_growthHYBR).
- Optional OKX: Zero address skipped to avoid whitelisting null.

8) Script Functions/Tasks
- DeploySwapper.run(): void; external. Orchestrates deploy, configure, connect, verify, save.
- DeploySwapper._loadExistingContracts(): void; internal. Loads HYBR and GrowthHYBR addresses from JSON.
- DeploySwapper._deploySwapper(): void; internal. Deploys HybrSwapper using HYBR token address.
- DeploySwapper._configureSwapper(): void; internal. Whitelists supported aggregators on the swapper.
- DeploySwapper._connectSwapper(): void; internal. Sets swapper in GrowthHYBR.
- DeploySwapper._verifyConfiguration(): void; internal view. Asserts correct wiring and whitelist.
- DeploySwapper._saveDeployment(): void; internal. Writes deployment JSON (currently only timestamp due to bug).
- UpgradeSwapper.run(): void; external. Deploys new HybrSwapper, sets whitelist, updates GrowthHYBR swapper.

Notes/Recommendations
- Fix _saveDeployment to merge serialized values (use vm.serialize... chaining on same object ID).
- Standardize input file key (GrowthHYBR casing) across both scripts.
- Consider ownership transfer of HybrSwapper to governance/DAO after whitelist set.
- Add idempotency guard and optional address reuse to avoid unnecessary redeploys.
- Add network-conditional aggregator addresses and validation.



## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Deploy_BribeFactoryV3.s.sol
This Foundry script deploys and wires a BribeFactoryV3 behind a TransparentUpgradeableProxy, initializing it with existing governance components and registering it in GaugeManager. It targets the EVM network configured in Foundry (current RPC) and uses an env-provided deployer key. Upgrade authority remains with an existing ProxyAdmin; protocol roles/permissions come from external registries. Major steps: read prior deployment artifacts, deploy BribeFactoryV3 implementation, deploy+initialize the proxy, set the new factory on GaugeManager, verify key storage values, and persist deployment details to JSON. Trust model: deployer executes, ProxyAdmin controls upgrades; BribeFactory integrates VoterV3, PermissionsRegistry, TokenHandler, and GaugeManager.

1) Inputs/Config
- PRIVATE_KEY
- projectRoot
- infraPath
- factoriesPath
- votingPath
- ProxyAdmin
- PermissionsReg
- TokenHandler
- GaugeManager
- VoterV3
- chain RPC
- json exists
- addrs valid
- admin pre-set

2) Dependencies
- forge-std (Script, StdJson)
- OpenZeppelin TransparentUpgradeableProxy
- Contracts: BribeFactoryV3, GaugeManager
- Foundry VM cheats (env, readFile, parseJson, serialize, writeJson)

3) Contracts Deployed/Interacted
- BribeFactoryV3 (implementation)
  - method: new
  - constructor args: none
  - post: address logged
  - outputs: implementation address
- TransparentUpgradeableProxy → BribeFactoryV3 (proxy)
  - method: new proxy (implementation, proxyAdmin, initData)
  - init selector: BribeFactoryV3.initialize
  - init args: voter, gaugeManager, permissionsRegistry, tokenHandler
  - post: cast proxy to BribeFactoryV3; initialized
  - outputs: proxy address (BribeFactoryV3)
- GaugeManager
  - method: setBribeFactory(bribeFactory)
  - post: BribeFactory registered
  - outputs: none
- BribeFactoryV3 (proxy) reads
  - voter(), gaugeManager() for verification

4) Steps (ordered)
- run(): load env and paths
  - note: Reads key and JSON file paths
  - params: PRIVATE_KEY
- run(): read files
  - note: Reads infra, factories, voting JSON
  - params: infraPath, factoriesPath, votingPath
- run(): parse addresses
  - note: Extracts ProxyAdmin, registries, voter, manager
  - params: json keys
- run(): start broadcast
  - note: Begins on-chain txs as deployer
  - params: deployer
- run(): deploy implementation
  - note: new BribeFactoryV3
  - params: none
- run(): deploy proxy + init
  - note: Proxy with initialize(voter, manager, perms, token)
  - params: impl, proxyAdmin, init args
- run(): set on GaugeManager
  - note: GaugeManager.setBribeFactory(proxy)
  - params: bribeFactoryV3
- run(): stop broadcast
  - note: Ends txs
  - params: none
- run(): verify storage
  - note: Checks voter and manager match
  - params: none
- run(): write JSON output
  - note: Saves addresses and timestamp
  - params: outputPath

5) Permissions/Trust
- ProxyAdmin: retains upgrade control of BribeFactoryV3 proxy
- Deployer EOA: executes deployment; no admin retained unless ProxyAdmin owned
- GaugeManager: now references new BribeFactory
- BribeFactoryV3: governed via PermissionsRegistry/VoterV3 (internals not changed here)

6) Post-Deploy Outputs
- Console verification of voter and gaugeManager
- JSON saved: Deploy_BribeFactoryV3.json
  - BribeFactoryV3 (proxy address)
  - BribeFactoryV3Implementation
  - deployer
  - timestamp

7) Safety/Idempotency
- No skip-if-deployed guard; always deploys new impl and proxy
- Depends on existing addresses in JSON; missing/invalid → revert
- Uses Foundry broadcast defaults; no confirmations configured
- Single transaction sequence; no explicit retry/backoff

8) Script Functions/Tasks
- run(): external; nonpayable; returns ()
  - purpose: Deploy BribeFactoryV3 proxy, initialize, register in GaugeManager, verify, and persist artifacts.


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/DepositToGauge.s.sol
Summary
This Foundry script deposits Uniswap v3 LP position NFTs into a Concentrated Liquidity Gauge. It targets the chain configured by your Foundry RPC and config JSON; the Gauge address is hard-coded. Trust model: the operator’s PRIVATE_KEY controls the NFTs and initiates approvals; the Gauge contract ultimately custody/manages deposited NFTs. Major steps: load PRIVATE_KEY, read config to get NonfungiblePositionManager, set gauge address, enumerate tokenIds, sanity-check ownership, approve the Gauge for each NFT, and call GaugeCL.deposit. The script broadcasts a single transaction sequence per tokenId and logs progress. No deployments, role assignments, or verifications are performed.

1) Inputs/Config
- PRIVATE_KEY
- configPath
- nonfungiblePM
- gaugeAddrConst
- tokenIds=[10]
- deployerAddr
- rpcNetwork
- ownsTokenId
- npmIsERC721
- gasDefaults

2) Dependencies
- forge-std Script/StdJson
- Foundry VM (vm.*)
- BaseDeployScript (getConfigPath)
- IERC721 (NPM interface)
- IGaugeCL (GaugeCL)
- NonfungiblePositionManager
- GaugeCL at 0xCC94…fa5D
- console.log

3) Contracts Deployed/Interacted
- NonfungiblePositionManager (IERC721)
  - method: approve(spender, tokenId)
  - method: ownerOf(tokenId) view
  - args: spender=gauge, tokenId∈tokenIds
  - post: Gauge gets approval per NFT
  - outputs: none (tx success)
- GaugeCL (IGaugeCL)
  - method: deposit(tokenId)
  - args: tokenId∈tokenIds
  - post: NFT deposited to gauge; accounting updated
  - outputs: none (tx success)

4) Steps (ordered)
- run(): Entry; orchestrates deposit flow; no params
- readEnv(): Load PRIVATE_KEY; derive deployer
- readConfig(): getConfigPath → readFile → parse .nonfungiblePositionManager
- setGauge(): Use hard-coded 0xCC94…fa5D
- startBroadcast(): Begin tx as deployer
- forEachId(): Iterate tokenIds array
- ownerCheck(): require(ownerOf(id)==deployer)
- approve(): IERC721.approve(gauge, id)
- deposit(): IGaugeCL.deposit(id)
- stopBroadcast(): End tx sequence

5) Permissions/Trust
- Deployer controls PRIVATE_KEY and initiates approvals/deposits
- Gauge gains per-token approval and then custody via deposit
- No ownership transfers/roles changed beyond NFT custody shift

6) Post-Deploy Outputs
- Console logs of actions and results
- Effective addresses: NPM from config; Gauge 0xCC94…fa5D
- No verification, artifacts, or registry writes

7) Safety/Idempotency
- Checks ownerOf before approve/deposit
- No skip-if-deposited; reruns may fail if NFT already moved
- Uses Foundry defaults for gas/network
- Fails fast via require on ownership
- Single-threaded loop over tokenIds

8) Script Functions/Tasks
- run(): external; returns void; orchestrates reading config, approving, and depositing
- getConfigPath(): view; returns string; from BaseDeployScript; resolves config file path
- ownerOf(tokenId): view; returns address; checks NFT owner
- approve(spender, tokenId): nonpayable; returns void; grants per-token approval
- deposit(tokenId): nonpayable; returns void; deposits NFT into Gauge


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/EnableSplitPermission.s.sol
Summary (≈100 words)
This Foundry script enables veNFT split permissions in the VotingEscrow contract. It targets any network configured via Foundry RPC where a deployed VotingEscrow address is recorded in script/constants/output/Deploy2_TokenSystem.json. Trust model: a single deployer EOA (PRIVATE_KEY) broadcasts; that account must hold whatever admin/owner role VotingEscrow requires to toggle split permissions. Major steps: load PRIVATE_KEY, resolve project root, read VotingEscrow address from JSON, startBroadcast(deployer), call toggleSplit(address(0), true) to enable global splits, call toggleSplit(deployer, true) for deployer-specific permission, stopBroadcast, then read canSplit for both keys and log results.

1) Inputs/Config
- Env vars: PRIVATE_KEY
- CLI args: none
- Files: Deploy2_TokenSystem.json
- JSON key: .VotingEscrow
- Root: vm.projectRoot()
- Network: Foundry RPC target
- Broadcast: vm.startBroadcast
- Preconditions:
  - VE deployed
  - JSON present
  - Caller authorized
  - RPC set

2) Dependencies
- forge-std/Script.sol (vm, broadcast)
- forge-std/StdJson.sol (parse/read JSON)
- forge-std console logging
- Contract: contracts/VotingEscrow.sol (ABI for toggleSplit/canSplit)

3) Contracts Deployed/Interacted
- VotingEscrow
  - Method: interact (existing deployment)
  - Calls:
    - toggleSplit(address(0), true)
    - toggleSplit(deployer, true)
    - canSplit(address(0)) [view]
    - canSplit(deployer) [view]
  - Constructor args: n/a (already deployed)
  - Post-deploy actions: permission toggles + verification reads
  - Outputs: two booleans (globalPermission, deployerPermission)

4) Steps (ordered)
- run()
  - Load env and resolve deployer; rememberKey; minimal setup
  - Read Deploy2_TokenSystem.json; parse .VotingEscrow into address
  - startBroadcast(deployer) to sign subsequent transactions
  - toggleSplit(0x000…0000, true); enable global split permission
  - toggleSplit(deployer, true); enable deployer split permission
  - stopBroadcast(); end transaction signing scope
  - Read canSplit for global + deployer; log verification

Key params
- PRIVATE_KEY (uint)
- VotingEscrow (address)
- address(0), deployer (address)
- enable flag: true

5) Permissions/Trust
- Requires caller to have VotingEscrow admin/owner role to toggle permissions
- After run: global split = enabled; deployer split = enabled
- Ownership of VotingEscrow unchanged; deployer retains EOA control

6) Post-Deploy Outputs
- Console logs: addresses, booleans for canSplit
- Transactions broadcast on target network
- No Etherscan verification, no artifact writes, no registry updates

7) Safety/Idempotency
- Idempotency: calling toggleSplit(..., true) again should be a no-op if implemented safely
- No explicit skip-if-deployed; uses existing VE address from JSON
- No confirmation waits; standard Foundry gas defaults
- Failure modes: missing JSON, bad address, unauthorized caller, RPC errors

8) Script Functions/Tasks
- run(): void; external; synchronous; exported by script
  - Purpose: Enable global and deployer-specific split permissions on VotingEscrow and verify via canSplit


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Set0-Permissions.s.sol
Summary
Purpose: assign governance/admin roles to the deployer in an existing PermissionsRegistry. Targets any EVM network via Foundry; environment provides PRIVATE_KEY and a JSON file containing the registry address. Trust model: the deployer’s key broadcasts transactions and receives GOVERNANCE, GENESIS_MANAGER, and VOTER_ADMIN roles; GAUGE_ADMIN is commented out. Major steps: load Deploy1_Infrastructure config, parse PermissionsRegistry address, start broadcast, call setRoleFor for each role, stop broadcast, then verify via hasRole and log results. No contracts are deployed, only registry interactions. Requires BaseDeployScript utilities and forge-std. Assumed idempotent if setRoleFor is repeat-safe; otherwise registry access control may restrict caller.

1) Inputs/Config
- ENV: PRIVATE_KEY
- Input: Deploy1_Infrastructure
- JSON key: .PermissionsRegistry
- RPC: default
- Broadcaster: deployer
- Roles: GOVERNANCE
- Roles: GENESIS_MANAGER
- Roles: VOTER_ADMIN
- Pre: registry deployed
- Pre: caller can setRole

2) Dependencies
- forge-std/Script.sol (Foundry scripting)
- forge-std/StdJson.sol (JSON parsing)
- Foundry vm cheatcodes (env, readFile, parseJson, start/stopBroadcast)
- BaseDeployScript (getInputPath)
- PermissionsRegistry.sol ABI
- Infrastructure JSON artifact containing PermissionsRegistry address

3) Contracts Deployed/Interacted
- PermissionsRegistry
  - Method: interact only (no deploy)
  - Calls:
    - setRoleFor(address account, string role)
      - Args: account=deployer, role ∈ {"GOVERNANCE","GENESIS_MANAGER","VOTER_ADMIN"}
    - hasRole(string role, address account) view
      - Args: role as above, account=deployer
  - Post-actions: none besides role assignments; console logs verification booleans
  - Outputs: on-chain role state; off-chain logs

4) Steps (ordered)
- run(): Load deployer key
  - Note: Reads PRIVATE_KEY; remembers key as broadcaster
  - Params: PRIVATE_KEY
- run(): Load infra JSON
  - Note: Reads path via getInputPath; loads file
  - Params: label="Deploy1_Infrastructure"
- run(): Parse registry addr
  - Note: Extract .PermissionsRegistry from JSON
  - Params: jsonKey=".PermissionsRegistry"
- run(): startBroadcast
  - Note: Begin tx broadcast as deployer
  - Params: from=deployer
- run(): setRole GOVERNANCE
  - Note: Grant GOVERNANCE to deployer
  - Params: (deployer, "GOVERNANCE")
- run(): setRole GENESIS_MANAGER
  - Note: Grant GENESIS_MANAGER to deployer
  - Params: (deployer, "GENESIS_MANAGER")
- run(): setRole VOTER_ADMIN
  - Note: Grant VOTER_ADMIN to deployer
  - Params: (deployer, "VOTER_ADMIN")
- run(): stopBroadcast
  - Note: End tx broadcast session
  - Params: none
- run(): Verify roles
  - Note: hasRole checks; log booleans
  - Params: roles, deployer

5) Permissions/Trust
- Deployer gains: GOVERNANCE, GENESIS_MANAGER, VOTER_ADMIN
- GAUGE_ADMIN: not granted (commented)
- Authority to set roles depends on PermissionsRegistry access control
- No ownership transfers; centralization at deployer post-run

6) Post-Deploy Outputs
- Console logs: addresses, role-setting status, hasRole booleans
- No contract verifications triggered
- No address registry updates beyond role state
- No file exports beyond existing JSON read

7) Safety/Idempotency
- No explicit skip-if-set; repeat setRoleFor assumed idempotent
- No waits/confirmations; single-tx calls per role
- Gas/network: defaults via Foundry RPC config
- Failure handling: none; relies on revert on unauthorized

8) Script Functions/Tasks
- run(): void; external; exported by contract; synchronous
  - Purpose: Assign roles to deployer and verify with hasRole


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Set1-Connectors.s.sol
## Summary (≈100 words)
This Foundry admin script configures TokenHandler by whitelisting tokens and marking them as swap connectors. It targets any EVM network where the Deploy1_Infrastructure config exists and is selected via Foundry’s RPC/profile. The caller’s PRIVATE_KEY determines the deployer/broadcaster and must hold the required TokenHandler admin/owner role. Steps: load infra JSON path via BaseDeployScript, parse TokenHandler address, pick a hardcoded token set (currently only token6), start broadcast, call whitelistTokens and whitelistConnectors, then stop. No contracts are deployed; only state updates on TokenHandler. Optional verification calls are present but commented out. The script is not explicitly idempotent and relies on contract-side checks.

### 1) Inputs/Config
- Env: PRIVATE_KEY
- RPC: Foundry cfg
- InputTag: D1_Infr
- JSON: infra addr
- Deployer: from PK
- Tokens: hardcoded
- Active: token6 only
- Arraysz: 1
- Network: EVM
- Preconditions:
  - TokenHandler set
  - Caller has role

### 2) Dependencies
- forge-std (Script, StdJson)
- BaseDeployScript (getInputPath, file IO helpers)
- TokenHandler interface (whitelistTokens, whitelistConnectors)
- Foundry VM (env, readFile, parseJson, broadcast)
- Deploy1_Infrastructure JSON artifact containing TokenHandler address

### 3) Contracts Deployed/Interacted
- TokenHandler
  - Method: interact (existing instance)
  - Address source: infra JSON (.TokenHandler)
  - Calls:
    - whitelistTokens(address[] tokens)
    - whitelistConnectors(address[] tokens)
  - Constructor/init: n/a (predeployed)
  - Post-actions: none beyond state updates
  - Outputs: tokens become whitelisted and flagged as connectors

### 4) Steps (ordered)
1. run()
   - Effect: Resolve deployer and infra; prepare token list; broadcast; mutate TokenHandler.
   - Params: PRIVATE_KEY, tag "Deploy1_Infrastructure"
2. BaseDeployScript.getInputPath(tag)
   - Effect: Resolve path to infra JSON by tag.
   - Params: "Deploy1_Infrastructure"
3. vm.readFile(path) + vm.parseJson(., ".TokenHandler")
   - Effect: Load TokenHandler address from JSON.
   - Params: path, ".TokenHandler"
4. vm.startBroadcast(deployer)
   - Effect: Begin tx broadcasting as deployer.
   - Params: deployer address
5. TokenHandler.whitelistTokens(tokens)
   - Effect: Add tokens to whitelist.
   - Params: address[] tokens (size 1; token6)
6. TokenHandler.whitelistConnectors(tokens)
   - Effect: Mark tokens as connectors.
   - Params: address[] tokens (size 1; token6)
7. vm.stopBroadcast()
   - Effect: End broadcasting session.
   - Params: none

### 5) Permissions/Trust
- Requires deployer to own/admin TokenHandler or hold a maintainer role.
- PRIVATE_KEY holder executes and controls all changes.
- No multisig/timelock enforced by script.

### 6) Post-Deploy Outputs
- Console logs of actions taken.
- On-chain: TokenHandler whitelist/connector state updated.
- No automatic Etherscan verification or artifact write-back.
- Optional verification calls exist but are commented out.

### 7) Safety/Idempotency
- No explicit skip-if-already-whitelisted guard in script.
- Idempotency depends on TokenHandler’s internal checks (may revert or no-op).
- Gas/network settings inherited from Foundry profile; no confirmations/waits configured.
- Failure handling: revert stops execution; no retries.

### 8) Script Functions/Tasks
- run(): ( ) -> void; external; exported; sync
  - Purpose: Whitelist tokens and set them as connectors on TokenHandler.


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Set2-InitBribeFactory.s.sol
Summary
This Foundry script (InitBribeFactory) initializes an already-deployed BribeFactoryV3 by wiring core governance and permissions addresses. It targets the network configured by Foundry RPC and the Deploy* JSON address files, broadcasting via an environment PRIVATE_KEY. Trust model: the PRIVATE_KEY holder triggers initialization; afterward, BribeFactoryV3 relies on the configured VoterV3 and GaugeManager for control paths, and initialize is assumed single-use/guarded. Major steps: resolve JSON paths via BaseDeployScript, read addresses, start broadcast as deployer, call initialize(voter, gaugeManager, permissionsRegistry, tokenHandler), stop broadcast, then verify by reading voter() and gaugeManager() and comparing to inputs. Assumes prior deployments and correct JSON wiring across infrastructure, factories, and voting.

1) Inputs/Config
- Env: PRIVATE_KEY
- Path: Dep1_Infrastructure
- Path: Dep3a_GaugeFacts
- Path: Dep3c_Voting
- JSON: .PermissionsReg
- JSON: .TokenHandler
- JSON: .GaugeManager
- JSON: .BribeFactoryV3
- JSON: .VoterV3
- Net: Foundry RPC cfg
- Pre: contracts exist
- Pre: bribe uninit
- Default: msg.sender key
- CLI: none

2) Dependencies
- forge-std/Script (Foundry)
- forge-std/StdJson
- Foundry vm cheatcodes
- BaseDeployScript (getInputPath)
- BribeFactoryV3 ABI
- Prior deployments: BribeFactoryV3, VoterV3, GaugeManager, PermissionsRegistry, TokenHandler
- JSON address artifacts for Deploy1/3a/3c

3) Contracts Deployed/Interacted
- BribeFactoryV3
  - Method: interact initialize(...)
  - Init args: voter, gaugeManager, permissionsRegistry, tokenHandler
  - Post-init: internal state set; emits none here
  - Outputs: voter(), gaugeManager() readable for verification
- VoterV3
  - Referenced only (address injected)
- GaugeManager
  - Referenced only (address injected)
- PermissionsRegistry
  - Referenced only (address injected)
- TokenHandler
  - Referenced only (address injected)

4) Steps (ordered)
- run.loadPaths
  - Note: Resolve JSON file paths from BaseDeployScript
  - Params: "Deploy1_Infrastructure", "Deploy3a_GaugeFactories", "Deploy3c_Voting"
- run.readAddresses
  - Note: Parse addresses from JSON blobs using StdJson
  - Params: .PermissionsRegistry, .TokenHandler, .GaugeManager, .BribeFactoryV3, .VoterV3
- run.startBroadcast
  - Note: Load PRIVATE_KEY, set deployer, begin broadcast
  - Params: PRIVATE_KEY
- run.initializeFactory
  - Note: Initialize BribeFactoryV3 with governance and permissions wiring
  - Params: voter, gaugeManager, permissionsRegistry, tokenHandler
- run.stopBroadcast
  - Note: End broadcast session
  - Params: none
- run.verifyReadback
  - Note: Read voter() and gaugeManager(); compare to inputs; log results
  - Params: none

5) Permissions/Trust
- Deployer: controls tx via PRIVATE_KEY
- BribeFactoryV3: post-init trusts voter, gaugeManager
- Initialize: assumed onlyOnce/owner-guarded
- No ownership transfers here
- No approvals granted here

6) Post-Deploy Outputs
- Console logs of configured addresses
- Readback: stored voter, gaugeManager
- Match checks: boolean equality logs
- No new artifacts/addresses
- No on-chain verification step

7) Safety/Idempotency
- No explicit skip-if-initialized; second run likely reverts
- Assumes correct JSON wiring and deployed addresses
- No waits/confirmations configured
- Gas/network via Foundry defaults/RPC config
- Failure handling: rely on revert/console logs

8) Script Functions/Tasks
- run(): void; external; sync
  - Purpose: Initialize BribeFactoryV3 from JSON-configured addresses and verify readback


## SUMMARY OF DEPLOY SCRIPT: 2025-10-hybra-finance/ve33/script/Set3-InitMinter.s.sol
Summary
This Foundry script initializes the HYBR token minter system using an EOA from PRIVATE_KEY. It targets any network where prior deployments exist, with addresses loaded from JSON artifacts. Trust model: the deployer EOA (holding owner/admin rights) performs HYBR initial mint and assigns the Minter; then configures RewardHYBR. Major steps: read deployed addresses; optionally perform HYBR initialMint and setMinter; initialize MinterUpgradeable with empty claimant distribution; set RewardHYBR’s GaugeManager and GrowthHYBR; run verification checks (mintability, period, supplies). No new contracts are deployed; the script strictly coordinates initialization and configuration using pre-deployed components.

1) Inputs/Config
- PRIVATE_KEY (env)
- Deploy2_TokenSystem
- Deploy3b1_MinterRewards
- Deploy3a_GaugeFactories
- HYBR (json key)
- RewardHYBR (key)
- Minter (key)
- GaugeManager (key)
- GrowthHYBR (key)
- claimants=[]
- amounts=[]
- max=0
- pre: addrs exist
- pre: owner perms
- pre: initialMint? ok
- net: Foundry RPC
- signer: deployer EOA

2) Dependencies
- forge-std/Script.sol (Foundry VM)
- forge-std/StdJson.sol (JSON IO)
- BaseDeployScript (path resolver)
- Contracts: HYBR, RewardHYBR, MinterUpgradeable
- Prior artifacts: JSON with addresses for: HYBR, RewardHYBR, Minter, GaugeManager, GrowthHYBR

3) Contracts Deployed/Interacted
- HYBR (interact)
  - Methods: initialMint(address), setMinter(address), initialMinted(), totalSupply(), balanceOf(address)
  - Init args: none (pre-deployed)
  - Post-actions: assign minter, mint 500M to deployer if not minted
  - Outputs: totalSupply, deployer balance
- MinterUpgradeable (interact)
  - Methods: _initialize(address[] claimants, uint[] amounts, uint max), check(), period(), active_period()
  - Init args: claimants=[], amounts=[], max=0
  - Post-actions: minter initialized
  - Outputs: canMint (check), period, active_period
- RewardHYBR (interact)
  - Methods: setGaugeManager(address), setGHYBR(address), gHYBR()
  - Init args: none (pre-deployed)
  - Post-actions: connects to GaugeManager and GrowthHYBR
  - Outputs: gHYBR address

4) Steps (ordered)
- run()
  - Load env and JSON addresses; set deployer signer; orchestrates initialization
  - params: PRIVATE_KEY
- maybeInitialMint
  - If !initialMinted: initialMint(deployer), setMinter(minter)
  - params: hybr, minter, deployer
- initMinter
  - Initialize MinterUpgradeable with empty claimants, amounts, and max=0
  - params: minter, [], [], 0
- configRewardHYBR
  - Set RewardHYBR’s GaugeManager and GrowthHYBR references
  - params: rewardHybr, gaugeManager, gHYBR
- verifyState
  - Read check(), period fields; HYBR supply/bal; confirm RewardHYBR.gHYBR == gHYBR
  - params: minter, hybr, rewardHybr, deployer

5) Permissions/Trust
- HYBR
  - initialMint, setMinter require owner/authorized; deployer executes, thus must be owner
  - After: Minter contract authorized to mint HYBR
- MinterUpgradeable
  - _initialize callable by owner/admin; deployer must control it; after, minter operational
- RewardHYBR
  - setGaugeManager/setGHYBR require owner; deployer configures; after, RewardHYBR references GaugeManager and GrowthHYBR
- Deployer retains 500M HYBR (if first mint) until redistributed; high custody risk

6) Post-Deploy Outputs
- Console logs for:
  - canMint (bool), period (uint256), active_period (uint256)
  - HYBR totalSupply, deployer HYBR balance
  - RewardHYBR.gHYBR and equality check
- No automated etherscan verification or file exports
- No registry writes; relies on provided JSON paths

7) Safety/Idempotency
- Guard: checks HYBR.initialMinted to skip re-minting
- No guard for Minter._initialize; calling twice may revert (initializer) or misconfigure
- RewardHYBR setters idempotency depends on contract guards; no pre-check
- No waits/confirmations; single-tx blocks via Foundry vm.startBroadcast
- Gas/network: default Foundry; uses PRIVATE_KEY signer
- Failure handling: revert bubbles; no try/catch or retries

8) Script Functions/Tasks
- run(): void; external; exported; sync
  - Orchestrates minter initialization, optional HYBR initial mint, RewardHYBR wiring, then verifies state


## Main List of Files in Project

cl/contracts/core/CLFactory.sol
cl/contracts/core/CLPool.sol
cl/contracts/core/fees/DynamicSwapFeeModule.sol
ve33/contracts/CLGauge/GaugeCL.sol
ve33/contracts/CLGauge/GaugeFactoryCL.sol
ve33/contracts/GaugeManager.sol
ve33/contracts/GaugeV2.sol
ve33/contracts/GovernanceHYBR.sol
ve33/contracts/HYBR.sol
ve33/contracts/MinterUpgradeable.sol
ve33/contracts/RewardHYBR.sol
ve33/contracts/VoterV3.sol
ve33/contracts/VotingEscrow.sol
ve33/contracts/swapper/HybrSwapper.sol


 ## DOCUMENTATION: 

 ### hydra-docs.md

# Hydra Finance Whitepaper

*A story about taming the ve(3,3) hydra on Hyperliquid*

***

#### Prologue — why we still believe

I watched the first **Solidly** pools hatch on Fantom.\
For one brilliant week it felt like alchemy: fees were paid in real dollars, emissions were paid in future hope, and lockers became tiny printing presses.\
Week two the charts bent.\
Week six they snapped: partners stopped bribing, votes fled to ghost gauges, SOLID bled to zero, and a once-luminous idea spiralled into the textbook *death spiral*.

The idea, however, never died.\
Velodrome, Aerodrome, Thena, Retro, Ramses—each iteration stitched a new safety net onto the ve(3,3) balloon.\
Some slowed the spiral, one or two climbed out of it, none broke the curse completely.

**Hybra** is the name we give to our next attempt.\
It lives on Hyperliquid— a chain that already clears nine-figure volume daily and rewards its traders with **HYPE**, not platitudes.\
If we get the flywheel right here, the hydra may finally breathe clean air.

***

#### 1 We dissect the corpse before we race the horse

**1.1 The three knives that killed past ve(3,3) DEXes**

1. **Front-loaded inflation** – 100 % of supply promised up front → token dumps faster than it can be locked.
2. **Free ride for partners** – protocols receive veNFT, bribe once for show, then harvest fees forever.
3. **Thin or fragmented liquidity** – users pay more slippage than on Uniswap or the native CEX, so volume never roots itself.

Miss any one knife, you wobble; miss all three, you die.\
We decide to dull every blade.

***

#### 2 The pact we strike with the hydra

*Rule 1 Cold-start must bang, not bleed.*\
We copy no one’s playbook; we hybridise them.

*Rule 2 Every dollar of emission must be backed by at least a cent of **external** income.*\
Partner bribes and Foundation bribes are not garnish, they are line-items in the P\&L.

*Rule 3 Traders come first.*\
If Robinhood feels smoother than your DEX, you deserve zero fees.\
So we graft **Uniswap v3 style ranges** on day one, hook in **v4 intents** the day they ship, and pour every marketing dollar into showing traders a visibly better price.

***

#### 3 How the machine works

**3.1 Keeping the heads alive – minimum-bribe rule**

Every epoch (7 days) a partner **must** bribe its gauge at least:

```
minBribe = α · avgTVL_epoch · baseFee
where
    α  = 0.75 ‰   // DAO-tunable, start at 0.00075
    baseFee = 0.04 %            // taker fee schedule
```

If the partner under-bribes, the contract melts part of its veNFT and recycles it into the community pool.

**Pseudocode (Solidity-pseudo)**

```solidity
solidity 
function settleEpoch(address partner) external {
    uint tvl = oracle.tvlOf(partner);
    uint min = alpha * tvl * baseFee;       // 18-dec fixed
    uint paid = bribeVault.claimed(partner, epoch);

    if (paid < min) {
        uint deficit = min - paid;
        uint ratio = deficit * 1e18 / min;  // 0-1e18
        uint burn = ratio * veBalance[partner] / 2; // soft-clawback
        veBalance[partner] -= burn;
        veBalance[DAO]     += burn;
        emit Clawback(partner, burn);
    }
}
```

*Interpretation*

* **Half** of the deficit ratio is burned each epoch; partners can correct course next week.
* Partners may **`recycle()`** up to 40 % of the bribe after fees settle, keeping net APR positive for them while lockers still receive the cash-flow.

> **Result** – grabbing a top-TVL airdrop commits you to a recurring marketing budget. The veNFT becomes a kind of perpetual-license that *rents itself*.

***

#### 4 Traders will actually like this place

| milestone  | feature                                              | why they care                                     |
| ---------- | ---------------------------------------------------- | ------------------------------------------------- |
| **Launch** | Uni v3 ranges, auto-rebalancer, gas-abstracted swaps | 3–6× fee density vs v2 AMMs; one-click LP.        |
| +60 days   | <p>Uni v4 hooks—limit orders, TWAP oracle</p><p></p> | <p>CEX-grade tooling without CEX risk.</p><p></p> |
| +120 days  | Strategy vaults (stable ranges, gamma farming)       | LP becomes deposit-and-chill                      |
| +180 days  | Smart router & off-chain intent relayer              | 15 % better fill on average; no failed Tx rage.   |

#### How Hybra will embed an **intent layer**—and why it matters

**What we add**\
Hybra lets traders sign a simple *intent* (“swap 1 ETH → USDC at ≥ $3 250, expire in 5 min”).\
Off-chain *solvers* compete to fill the order, pay the gas, and settle the result on-chain.\
The router is **gauge-aware**: solvers that stake or lock veHYBRA get priority routing, so execution flow is pulled toward the pools they vote for.

| Stakeholder            | Direct benefit                                                                                                                                                                                                                     | Fly-wheel effect                                                     |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| **Trader**             | <p>• Gas-free, fail-proof swaps.<br>• Solvers aggregate Hybra pools + RFQ quotes + external AMMs, so average price improves and MEV is neutralised.<a href="https://0x.org/post/intents-in-defi?utm_source=chatgpt.com">0x</a></p> | Better UX → higher volume → deeper books.                            |
| **veHYBRA voter / LP** | <p>• 100 % of solver-paid swap fees stream to ve vaults.<br>• Solvers must <strong>stake veHYBRA</strong> to maximise routing priority → constant buy-pressure and bribe demand.</p>                                               | Fee APR rises without extra inflation.                               |
| **Protocol (Hybra)**   | <p>• Takes a 0.02–0.05 bp settlement fee from solvers—<em>non-inflationary</em> income.<br>• Gauge-aware routing auto-recycles volume into the pools that pay the most fees.</p>                                                   | External cash flow > emissions sooner → death-spiral risk minimized. |

**Why it’s differentiated**\
– UniswapX and CoW Swap show that intent + solver architecture can deliver CEX-grade pricing and gasless UX, but they do **not** tie solver incentives to ve voting. Hybra stitches the two together, turning every solver into a ve holder and every trade into a vote-reinforcing event.

In short, the intent layer makes trading cheaper for users, richer for voters, and cash-positive for the protocol—exactly the “someone other than the minter pays” principle we build around.

Marketing is idle if product stinks; we reverse the order.

***

#### 5 *Inflation-led ignition → Income-led sustainability*

**Assumptions)**

```
weekly token emission decay k        = 2 %
token price P(t)            = logistic 1 $ → 2 $ (speed q = 5 %/wk)
week-0 emission tokens       = 500 000
external-income ceiling      = 650 000 USDC / wk
external-income rise speed   = 8 %/wk (mid-point t₀ = 16 wk)
```

***

**5.1 What the curve now does**

* **Week 0-10 – Ignition**\
  Inflation (yellow) hovers around **500-570 k USDC** because price appreciation offsets the 2 % token decay.\
  → liquidity rushes in; veNFT WAR is headline news.
* **Week 11-29 – Hand-off**\
  Fees + bribes (orange) scale with volume; inflation slides as price growth flattens.\
  → the two lines draw together.
* **Week 30 – Crossover**\
  External income overtakes inflation **(\~30 th week)** and never looks back (see dashed line in chart).
* **Week 30+ – Income-led era**\
  Emissions keep decaying geometrically; income keeps compounding with usage.\
  By week 52 the protocol runs a **+278 k USDC weekly surplus**.

***

| Snapshot    | Emission value E(t) | External income I(t) | Surplus I – E |
| ----------- | ------------------- | -------------------- | ------------- |
| **Week 0**  | 500 k               | 140 k                | **–360 k**    |
| **Week 26** | 511 k               | 448 k                | –62 k         |
| **Week 52** | 337 k               | 615 k                | **+279 k**    |

*(all figures USD / week, rounded)*

<figure><img src="https://3034550939-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FV7WY0kfUBQqJkRAW2jcI%2Fuploads%2FbfYKYnrUWGgtPfJfVjgN%2Fimage.png?alt=media&#x26;token=28e112c0-1ac4-4660-8708-ac1aacea90aa" alt=""><figcaption><p>emission &#x26; income chart</p></figcaption></figure>

***

**5.2 Why this matters**

| Phase        | Dominant payer     | Locker psychology                                        | Fragility                                |
| ------------ | ------------------ | -------------------------------------------------------- | ---------------------------------------- |
| Ignition     | Printer            | “Farm the yield, test the UX.”                           | Token price drag (managed by decay cap). |
| Hand-off     | Printer ≈ Outsider | “APR looks balanced—maybe relock.”                       | Keep partner bribe ROI > 1.              |
| Steady-state | Outsider           | “Fees + cash bribes = real yield; inflation just icing.” | Only if volume collapses.                |

Because **E(t)** is capped by deterministic decay while **I(t)** is free to grow with depth and volume, the crossover is **mathematically inevitable** so long as\
`I_max > E₀ · (1-k)^{t_cross}`.\
With the numbers above that holds at t ≈ 30 weeks even under a 50 % price shock.

The plotted curve (see chart) shows a launch that *starts* with attractive inflation but smoothly hands the baton to sustainable external income—exactly the arc a ve(3,3) hydra needs to break the death-spiral cliché.

***

#### Epilogue — an invitation

Solidly showed us how ve(3,3) could soar;\
its fork-sons showed us every way it could crash.\
We believe the missing piece is simple: **make someone other than the minter pay.**\
Bribes are that someone; traders are that someone; the Foundation is that someone.\
If we balance their incentives, the hydra sheds its death spiral and grows into a flywheel.

Lock some veHYBRA, steer the gauges, and take a cut every time the chain you already trade on moves a dollar.\
Let’s prove the hydra can live.

*— The Hybra core crew*




 ## PACKAGE.JSON HEADERS OF LIB PACKAGES: 

 Note: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 ### 2025-10-hybra-finance/ve33/lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.10.0",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### 2025-10-hybra-finance/ve33/lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.4.0",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### 2025-10-hybra-finance/ve33/lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.4.0",
  "private": true,
  "files": [
    "/contracts/**/*.sol",
    "!/contracts/mocks/**/*"

### 2025-10-hybra-finance/ve33/lib/openzeppelin-contracts/scripts/solhint-custom/package.json

{
  "name": "solhint-plugin-openzeppelin",
  "version": "0.0.0",
  "private": true,
  "dependencies": {
    "minimatch": "^3.1.2"
  }
}

### 2025-10-hybra-finance/ve33/lib/openzeppelin-contracts/lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.9.6",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### 2025-10-hybra-finance/cl/lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.7.6",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### 2025-10-hybra-finance/cl/lib/base64/package.json

{
  "name": "base64-sol",
  "version": "1.1.0",
  "description": "base64 implementation in solidity",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },

### 2025-10-hybra-finance/cl/lib/solidity-lib/package.json

{
  "name": "@uniswap/lib",
  "version": "4.0.1-alpha",
  "description": "📖 Solidity libraries that are shared across Uniswap contracts",
  "files": [
    "contracts",
    "!contracts/test"
  ],

### 2025-10-hybra-finance/cl/lib/ExcessivelySafeCall/package.json

{
  "name": "@nomad-xyz/excessively-safe-call",
  "version": "0.0.1-rc.1",
  "description": "Helps you call untrusted contracts safely",
  "keywords": [
    "nomad",
    "excessively safe call"
  ],

### 2025-10-hybra-finance/cl/lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "3.4.2-solc-0.7",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks",

### 2025-10-hybra-finance/cl/lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "3.4.2-solc-0.7",
  "files": [
    "/contracts/**/*.sol",
    "/build/contracts/*.json",
    "!/contracts/mocks",


 ## CONFIG FILES: 

 Note: Check for important package version info.

 ### 2025-10-hybra-finance/ve33/foundry.toml

[profile.default]
src = "contracts"
out = "out"
libs = ["lib", "node_modules"]
test = "test"
script = "script"
# Suppress warnings
ignored_error_codes = [2519, 5667, 2072, 2018]
suppress_warnings = true
# Disable specific lint warnings
ignored_warnings_from = ["*"]
deny_warnings = false

# See more config options https://github.com/foundry-rs/foundry/tree/master/config

remappings = [
    "@openzeppelin/contracts/=node_modules/@openzeppelin/contracts/",
    "@openzeppelin/contracts-upgradeable/=node_modules/@openzeppelin/contracts-upgradeable/",
    "@cryptoalgebra/integral-core/=node_modules/@cryptoalgebra/integral-core/",
    "@cryptoalgebra/integral-periphery/=node_modules/@cryptoalgebra/integral-periphery/",
    "@cryptoalgebra/integral-base-plugin/=node_modules/@cryptoalgebra/integral-base-plugin/",
    "@cryptoalgebra/integral-farming/=node_modules/@cryptoalgebra/integral-farming/",
]

solc_version = "0.8.13"
evm_version = "london"
optimizer = true
optimizer_runs = 200
via_ir = true

# Gas optimization settings
ffi = false
fs_permissions = [
    { access = "read-write", path = "./" },
    { access = "read", path = "../cl/deployments/" }
]

[profile.local]
verbosity = 3
gas_reports = ["*"]

[profile.test]
verbosity = 3
gas_reports = ["*"]

[rpc_endpoints]
local = "http://localhost:8545"
anvil = "http://localhost:8545"
hyper_test="${HYPER_TEST}"
hyper = "${HYPEREVM_RPC_URL}"

[etherscan]
# Add your API keys here

# Lint configuration
[profile.default.lint]
# Disable specific lint rules
ignored = [
    "asm-keccak256",           # Inline assembly for keccak256
    "unaliased-plain-import",   # Plain imports without alias
    "screaming-snake-case-const", # Constant naming convention
    "mixed-case-variable",      # Variable naming convention
    "mixed-case-function",      # Function naming convention
    "unused-import"             # Unused imports
]

### 2025-10-hybra-finance/ve33/hardhat.config.js

require("@nomiclabs/hardhat-waffle");
require('@openzeppelin/hardhat-upgrades');
require("@nomiclabs/hardhat-etherscan");
require("@nomiclabs/hardhat-web3");
require("hardhat-contract-sizer");
require("dotenv").config(); 

module.exports = {
  // Latest Solidity version
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts"
  },
  solidity: {
    compilers: [
      {
        version: "0.8.13",
        settings: {
          optimizer: {
            enabled: true,
            runs: 1,
          },
          metadata: {
              useLiteralContent: true
          }
        },
      },
    ],
  },

  networks: {
    hyper: {
      url: "https://rpc.hyperliquid.xyz/evm",
      chainId: 999,
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
      gas: 30000000,
      blockGasLimit: 30000000,
      timeout: 1800000
    },
  },

  etherscan: {
    apiKey: `${process.env.APIKEY}`,
  },

  mocha: {
    timeout: 100000000,
  },
  contractSizer: {
    runOnCompile: true
},
};


### 2025-10-hybra-finance/ve33/package.json

{
  "dependencies": {
    "@cryptoalgebra/integral-base-plugin": "~1.2.1",
    "@cryptoalgebra/integral-core": "~1.2.1",
    "@cryptoalgebra/integral-farming": "~1.2.1",
    "@cryptoalgebra/integral-periphery": "~1.2.1",
    "@openzeppelin/contracts": "^4.9.6",
    "@openzeppelin/contracts-upgradeable": "^4.8.0"
  }
}


### 2025-10-hybra-finance/cl/foundry.toml

[profile.default]
src = "contracts"
test = "test"
out = "out"
libs = ["lib"]
solc_version = "0.7.6"

optimizer = true
optimizer_runs = 10  # Reduced to match Hardhat config and reduce contract size
bytecode_hash = "none"
cbor_metadata = false

fs_permissions = [{ access = "read-write", path = "./"}]

no_match_test = "testEchidna"

# See more config options https://github.com/foundry-rs/foundry/tree/master/config

[fuzz]
runs = 5000

[rpc_endpoints]
base_goerli = "${BASE_GOERLI_RPC_URL}"
base = "${BASE_RPC_URL}"
hyper = "${HYPEREVM_RPC_URL}"
hyper_test="${HYPER_TEST}"
local="http://localhost:8545"

[etherscan]
base_goerli = { key = "${BASE_GOERLI_ETHERSCAN_API_KEY}", url = "${BASE_GOERLI_ETHERSCAN_VERIFIER_URL}" }
base = { key = "${BASE_ETHERSCAN_API_KEY}", url = "${BASE_ETHERSCAN_VERIFIER_URL}" }
hyper = { key = "${HYPEREVM_ETHERSCAN_API_KEY}", url = "${HYPEREVM_ETHERSCAN_VERIFIER_URL}" }
hyper_test = { key = "${HYPEREVM_ETHERSCAN_API_KEY}", url = "${HYPEREVM_ETHERSCAN_VERIFIER_URL}" }

### 2025-10-hybra-finance/cl/package.json

{
  "name": "@aerodrome-finance/slipstream",
  "description": "Core smart contracts of CL",
  "license": "BUSL-1.1",
  "publishConfig": {
    "access": "public"
  },
  "version": "1.0.1",
  "homepage": "https://aerodrome.finance",
  "keywords": [
    "uniswap",
    "core",
    "v3"
  ],
  "repository": {
    "type": "git",
    "url": "https://github.com/aerodrome-finance/slipstream"
  },
  "files": [
    "contracts/core/interfaces",
    "contracts/core/libraries",
    "contracts/periphery/base",
    "contracts/periphery/interfaces",
    "contracts/peripherylibraries",
    "artifacts/contracts/core/CLFactory.sol/CLFactory.json",
    "artifacts/contracts/core/CLPool.sol/CLPool.json",
    "artifacts/contracts/core/interfaces/**/*.json",
    "!artifacts/contracts/core/interfaces/**/*.dbg.json",
    "artifacts/contracts/periphery/**/*.json",
    "!artifacts/contracts/periphery/**/*.dbg.json",
    "!artifacts/contracts/periphery/test/**/*",
    "!artifacts/contracts/periphery/base/**/*"
  ],
  "engines": {
    "node": ">=10"
  },
  "devDependencies": {
  },
  "scripts": {
    "compile": "hardhat compile",
    "test": "hardhat test",
    "test:all": "UPDATE_SNAPSHOT=1 yarn test",
    "format": "prettier --write 'test/**/*.ts'",
    "format:check": "prettier --check 'test/**/*.ts'"
  },
  "dependencies": {
    "@openzeppelin/contracts": "^3.4.2"
  }
}


### 2025-10-hybra-finance/cl/remappings.txt

@ensdomains/=node_modules/@ensdomains/
@solidity-parser/=node_modules/solhint/node_modules/@solidity-parser/
ds-test/=lib/forge-std/lib/ds-test/src/
forge-std/=lib/forge-std/src/
hardhat/=node_modules/hardhat/
@openzeppelin/=lib/openzeppelin-contracts/
@nomad-xyz/=lib/ExcessivelySafeCall/
@uniswap/=lib/solidity-lib/
base64-sol/=lib/base64/


