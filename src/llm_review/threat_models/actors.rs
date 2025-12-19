use crate::llm_review::threat_models::patterns::VulnerabilityPattern;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Actors {
    pub actors: Vec<Actor>,
}

// NOTE: run once to get roles
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Actor {
    /// describe actor in under 10 words or less
    /// i.e. "Evicted signer behind checkpointer", "Unprivileged user providing initial liquidity"
    pub name: String,
    pub role_type: RoleType,
    /// full description of actor
    pub description: String,
    /// all capabilities of actor
    /// i.e. ["Call deposit() with arbitrary amount and recipient"]
    pub capabilities: Vec<String>,
}

// NOTE: run 5X to get abuses by capability
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ActorAbuses {
    pub abuses: Vec<ActorAbuse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ActorAbuse {
    pub actor_name: String,
    pub capability: String,
    /// 50 word or less C4-style headline, explaining exploit
    pub title: String,
    /// step by step breakdown of how exploit is executed
    pub scenario: String,
    /// What type of security Vulnerability is this?
    pub category: VulnerabilityPattern,
    /// i.e. ["vault diposits"]
    pub assets_at_risk: Vec<String>,
    /// NEW: Who suffers? (LP, DAO, user, MEV, protocol treasury, etc.)
    pub victim: String,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    JsonSchema,
    EnumIter,
    Serialize,
    Deserialize, // ✅ Use serde's derive - LLMs return exact PascalCase
    strum_macros::EnumString,
    strum_macros::Display,
)]
pub enum RoleType {
    /// Generic externally-owned account with no special privileges
    #[default]
    UnprivilegedUser,
    /// EOAs or contracts that hold very large positions and can move markets
    LargeHolder,
    /// Any privileged role in the protocol: owner, admin, guardian, pauser, upgrader, fee manager, etc.
    PrivilegedAdmin,
    /// Governance multisig / council / DAO / timelock executor
    Governance,
    /// Signer, key holder, or session key used by a smart wallet or module
    SignerOrKeyHolder,
    /// Liquidity provider (LP) depositing/withdrawing to/from pools or vaults
    LiquidityProvider,
    /// Trader / arbitrageur / MEV searcher using swaps, DEXes, or routers
    TraderOrArbitrageur,
    /// Liquidator or auction participant that closes unhealthy positions / buys liquidated collateral
    Liquidator,
    /// Relayer, bundler, keeper, or cron-like actor submitting transactions on behalf of others
    RelayerOrKeeper,
    /// Off-chain oracle publisher or on-chain oracle contract that provides price/feeds
    OracleOrPriceFeed,
    /// Smart contract wallet / account abstraction account (e.g. Sequence, Safe) as an actor
    SmartAccountOrWallet,
    /// Module / plugin / extension attached to a wallet or core protocol (auth module, risk module, etc.)
    ModuleOrPlugin,
    /// External DeFi protocol used as a building block (AMM, lending market, yield farm, staking, etc.)
    ExternalDefiProtocol,
    /// Bridge, cross-chain messaging system, or L1/L2 inbox/outbox
    BridgeOrMessenger,
    /// Router, aggregator, multicall, or batching helper (LiFi, 1inch, Multicall3, custom router)
    RouterOrAggregator,
    /// Token contract (ERC20 / ERC721 / ERC1155 / LST / rebasing / FoT, etc.)
    TokenContract,
    /// Vault, pool, or share-issuing wrapper around underlying assets (ERC4626, LP pool, staking pool)
    VaultOrPool,
    /// Chain infrastructure that can reorder or censor transactions (sequencer, validator, proposer)
    ChainInfrastructure,
    /// Off-chain backend / frontend / API that prepares payloads or signatures for users
    OffchainService,
    /// Fallback for anything not well captured above
    Other,
}

pub static ACTOR_CENTRIC_VULN_PATTERNS: &[VulnerabilityPattern] = &[
    VulnerabilityPattern::AccessControlOrAuthByPass, // actor gains roles/privilege or bypasses policies
    VulnerabilityPattern::PermitOrSignatureReplay,   // stale/partial signature reuse (EOA-focused)
    VulnerabilityPattern::ArbitraryExternalCall,
    VulnerabilityPattern::DoubleExecutionOrReplay, // transaction/nonce replay across contexts
    VulnerabilityPattern::FlashLoanEconomicManipulation, // capital-free multi-tx economic attacks (includes MEV/frontrun)
    VulnerabilityPattern::OracleUsingDEXorTWAP,          // abusing external pricing assumptions
    VulnerabilityPattern::SlippageMissingOrInsufficient, // exploit lack of price protection
    // Game theory & incentive misalignment micro-patterns
    VulnerabilityPattern::UnincentivizedMaintenanceOrKeeperlessProgress, // actor skips calling unprofitable maintenance
    VulnerabilityPattern::FirstOrLastMoverAdvantage, // actor races to be first/last in sequential processing
    VulnerabilityPattern::CheapGriefingOrDosProfit, // actor cheaply blocks operations for profit/optionality
    VulnerabilityPattern::QueueOrderDependentMevExtraction, // actor reorders queue entries for MEV
    VulnerabilityPattern::FixedPotRewardRaceOrGasAuction, // actor spams calls to drain reward pot
    VulnerabilityPattern::RewardCheckpointFreeRiderOrLateJoiner, // actor joins late to capture historical rewards
    VulnerabilityPattern::GovernanceCaptureOrTreasuryExtraction, // actor uses voting power to extract treasury
    VulnerabilityPattern::CrossRoleCollusionWithoutSlashing, // actors collude across roles without penalty
    VulnerabilityPattern::IncentiveMisalignmentOrGameTheory, // catch-all for other game-theory attacks
    VulnerabilityPattern::GlobalParamMidFlowManipulation,
    VulnerabilityPattern::ExternalProtocolKeyCollision,
    VulnerabilityPattern::EmergencyModeStateStuck,
    VulnerabilityPattern::AccountingInvariantViolation, // degrade solvency for profit
    VulnerabilityPattern::UntrustedDelegateCall,        // actor routes calls into malicious modules
    VulnerabilityPattern::GovernanceDelegationFlaw, // generic state-hijack via delegatecall (includes DelegatecallLowLevelOps)
    VulnerabilityPattern::MulticallCrossPathReentrancy, // cross-path execution ordering attacks
    VulnerabilityPattern::ForcedAssetVsStrictEquality, // force-send or mismatch in accounting
    VulnerabilityPattern::TimelockEdgeCase,         // admin follows spec but guarantees break
    VulnerabilityPattern::ConfigFootgun, // rational actor escalates privileges within rules (governance/authority)
    VulnerabilityPattern::BeaconOrFactoryAuthorityDrift, // upgrade path abused at module/authority boundary
    VulnerabilityPattern::ERC4626SharePriceMismatch,     // actor gains share-price advantage
    VulnerabilityPattern::ERC20DecimalsMismatch, // decimals-based value capture opportunities
    VulnerabilityPattern::TWAPWindowPinningOrLowLiquidity, // actor manipulates oracle read window
    VulnerabilityPattern::FeeOnTransferAssumption, // exploits non-standard token behaviors
    VulnerabilityPattern::GriefableCallbacks, // DoS others’ flows to create asymmetry (gas grief)
    VulnerabilityPattern::UnprotectedPauseOrStop, // abuse emergency mechanics for gain
    VulnerabilityPattern::Reentrancy,         // classic reentrancy attacks (actor-driven)
    VulnerabilityPattern::ReadOnlyReentrancy, // read-only reentrancy for price manipulation
    VulnerabilityPattern::SandwichableOracle, // on-chain spot read manipulable within one tx
    VulnerabilityPattern::PermitFrontRun,     // permit usable/front-runnable in same block
    VulnerabilityPattern::ReplayAcrossForksOrL2s, // message valid on fork/sibling chain replays
];
