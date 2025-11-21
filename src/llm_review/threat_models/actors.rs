use crate::llm_review::findings::finding_enums::VulnerabilityType;
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
    pub likely_category: VulnerabilityType,
    /// i.e. ["vault diposits"]
    pub assets_at_risk: Vec<String>,
    /// NEW: Who suffers? (LP, DAO, user, MEV, protocol treasury, etc.)
    pub victim: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct IsActorAbuseLegit {
    is_legit_abuse: bool,
    why_its_not_legit: String,
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

pub static ACTOR_CENTRIC_VULN_TYPES: &[VulnerabilityType] = &[
    VulnerabilityType::AccessControl, // actor gains roles/privilege they shouldn't
    VulnerabilityType::AuthByPass,    // uses legit flow to bypass intended policies
    VulnerabilityType::SignatureReplay, // stale/partial signature reuse (EOA-focused)
    VulnerabilityType::ReplayAttack,  // transaction/nonce replay across contexts
    VulnerabilityType::FrontrunMev,   // economic sandwich/front-run exploitation
    VulnerabilityType::FlashLoanEconomicManipulation, // capital-free multi-tx economic attacks
    VulnerabilityType::Oracle,        // abusing external pricing assumptions
    VulnerabilityType::SlippageMissingOrInsufficient, // exploit lack of price protection
    VulnerabilityType::AccountingInvariantViolation, // degrade solvency for profit
    VulnerabilityType::UntrustedDelegateCall, // actor routes calls into malicious modules
    VulnerabilityType::DelegatecallLowLevelOps, // generic state-hijack via delegatecall
    VulnerabilityType::MulticallCrossPathReentrancy, // cross-path execution ordering attacks
    VulnerabilityType::ForcedAssetVsStrictEquality, // force-send or mismatch in accounting
    VulnerabilityType::TimelockEdgeCase, // admin follows spec but guarantees break
    VulnerabilityType::AuthorityOrGovernance, // rational actor escalates privileges within rules
    VulnerabilityType::BeaconFactoryAuthorityDrift, // upgrade path abused at module/authority boundary
    VulnerabilityType::ERC4626SharePrice,           // actor gains share-price advantage
    VulnerabilityType::ERC20DecimalsMismatch,       // decimals-based value capture opportunities
    VulnerabilityType::TWAPWindowPinning,           // actor manipulates oracle read window
    VulnerabilityType::FeeOnTransferAssumption,     // exploits non-standard token behaviors
    VulnerabilityType::GasGriefBlockLimit,          // DoS others’ flows to create asymmetry
    VulnerabilityType::PausableEmergencyStop,       // abuse emergency mechanics for gain
    VulnerabilityType::Custom,                      // fallback when none apply cleanly
];
