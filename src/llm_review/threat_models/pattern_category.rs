use crate::{
    config::{MAX_PATTERN_LIBRARY, MAX_PATTERN_NICHE, R1_RUNS, R2_RUNS},
    llm_review::threat_models::patterns::VulnerabilityPattern,
};
use std::{collections::HashMap, sync::OnceLock};
/// Configuration management for the AI Agent Audit application.
///
/// This module provides centralized configuration handling, with constants
/// for application settings and environment variables only for sensitive
/// configuration like API keys and URLs.
use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, EnumIter)]
pub enum PatternCategory {
    SignatureValidation,
    OraclePriceFeed,
    TokenTransferLibrary,
    GovernanceTimeLock,
    ProxyUpgradeable,
    StakingRewards,
    BridgeCrossChain,
    AMMDex,
    LendingBorrowing,
    FactoryDeployer,
    MathLibrary,
    VaultShareBased,
    ERC20Token,
    NFTCollection,
    AirdropDistributor,
    EscrowVesting,
    MarketplaceExchange,
    RandomnessRaffleLottery,
    // Utility library categories (custom)
    ByteManipulationLibrary,
    EncodingDecodingLibrary,
    StorageHelperLibrary,
    ErrorDefinitionLibrary,
    AccessControlModifier,
    ReentrancyGuardLibrary,
    SimulationTestingHelper,
    #[default]
    General,
    Top,
    Rare,
    Frequent,
    MostObserved,
    Library,
    Relevant,
    R1,
    R2,
    R3,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PatternTier {
    Tier1,
    Tier2,
    #[default]
    Tier3,
    Tier4,
}

#[derive(Debug, Clone)]
pub struct PatternCategorySpec {
    pub category: PatternCategory,
    pub title: &'static str,
    pub issues: &'static [VulnerabilityPattern],
    pub tier: PatternTier,
    pub runs: usize,
}

pub const PATTERN_CATEGORY_LIBRARY: &[PatternCategorySpec] = &[
    PatternCategorySpec {
        category: PatternCategory::SignatureValidation,
        title: "Signature Validation",
        issues: SIGNATURE_VALIDATION_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::VaultShareBased,
        title: "Vault Share-Based",
        issues: VAULT_SHARE_BASED_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::OraclePriceFeed,
        title: "Oracle Price Feed",
        issues: ORACLE_PRICE_FEED_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::MathLibrary,
        title: "Math Library",
        issues: MATH_LIBRARY_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::TokenTransferLibrary,
        title: "Token Transfer Library",
        issues: TOKEN_TRANSFER_LIBRARY_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::GovernanceTimeLock,
        title: "Governance TimeLock",
        issues: GOVERNANCE_TIMELOCK_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::ProxyUpgradeable,
        title: "Proxy Upgradeable",
        issues: PROXY_UPGRADEABLE_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::StakingRewards,
        title: "Staking Rewards",
        issues: STAKING_REWARDS_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::BridgeCrossChain,
        title: "Bridge Cross-Chain",
        issues: BRIDGE_CROSS_CHAIN_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::AMMDex,
        title: "AMM DEX",
        issues: AMM_DEX_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::LendingBorrowing,
        title: "Lending Borrowing",
        issues: LENDING_BORROWING_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::FactoryDeployer,
        title: "Factory Deployer",
        issues: FACTORY_DEPLOYER_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::ERC20Token,
        title: "ERC20 Token",
        issues: ERC20_TOKEN_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::NFTCollection,
        title: "NFT Collection",
        issues: NFT_COLLECTION_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::AirdropDistributor,
        title: "Airdrop Distributor",
        issues: AIRDROP_DISTRIBUTOR_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::EscrowVesting,
        title: "Escrow Vesting",
        issues: ESCROW_VESTING_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::MarketplaceExchange,
        title: "Marketplace Exchange",
        issues: MARKETPLACE_EXCHANGE_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    PatternCategorySpec {
        category: PatternCategory::RandomnessRaffleLottery,
        title: "Randomness Raffle Lottery",
        issues: RANDOMNESS_RAFFLE_LOTTERY_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_NICHE,
    },
    // Custom utility library categories
    PatternCategorySpec {
        category: PatternCategory::ByteManipulationLibrary,
        title: "Byte Manipulation Library",
        issues: BYTE_MANIPULATION_LIBRARY_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_LIBRARY,
    },
    PatternCategorySpec {
        category: PatternCategory::EncodingDecodingLibrary,
        title: "Encoding/Decoding Library",
        issues: ENCODING_DECODING_LIBRARY_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_LIBRARY,
    },
    PatternCategorySpec {
        category: PatternCategory::StorageHelperLibrary,
        title: "Storage Helper Library",
        issues: STORAGE_HELPER_LIBRARY_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_LIBRARY,
    },
    PatternCategorySpec {
        category: PatternCategory::ErrorDefinitionLibrary,
        title: "Error Definition Library",
        issues: ERROR_DEFINITION_LIBRARY_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_LIBRARY,
    },
    PatternCategorySpec {
        category: PatternCategory::AccessControlModifier,
        title: "Access Control Modifier",
        issues: ACCESS_CONTROL_MODIFIER_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_LIBRARY,
    },
    PatternCategorySpec {
        category: PatternCategory::ReentrancyGuardLibrary,
        title: "Reentrancy Guard Library",
        issues: REENTRANCY_GUARD_LIBRARY_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_LIBRARY,
    },
    PatternCategorySpec {
        category: PatternCategory::R1,
        title: "R1",
        issues: R1_PATTERNS,
        tier: PatternTier::Tier1,
        runs: R1_RUNS,
    },
    PatternCategorySpec {
        category: PatternCategory::R2,
        title: "R2",
        issues: R2_PATTERNS,
        tier: PatternTier::Tier1,
        runs: R2_RUNS,
    },
    PatternCategorySpec {
        category: PatternCategory::Library,
        title: "Most Frequent Library",
        issues: LIBRARY_ANALYSIS_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_LIBRARY,
    },
];

/// One-time initialized map from PatternCategory -> &'static PatternCategorySpec
static PATTERN_CATEGORY_MAP: OnceLock<HashMap<PatternCategory, &'static PatternCategorySpec>> =
    OnceLock::new();

/// Get a reference to the category map (initializes lazily if not yet set)
pub fn pattern_category_map() -> &'static HashMap<PatternCategory, &'static PatternCategorySpec> {
    PATTERN_CATEGORY_MAP.get_or_init(|| {
        PATTERN_CATEGORY_LIBRARY
            .iter()
            .map(|spec| (spec.category.clone(), spec))
            .collect::<HashMap<_, _>>()
    })
}

/// Convenience accessor for a single category spec
pub fn get_category_library_spec(cat: &PatternCategory) -> Option<&'static PatternCategorySpec> {
    pattern_category_map().get(cat).copied()
}

//**************************************************
//**************************************************

pub const SIGNATURE_VALIDATION_PATTERNS: &[VulnerabilityPattern; 10] = &[
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::PermitOrSignatureReplay,
    VulnerabilityPattern::EIP1271ByPass,
    VulnerabilityPattern::DoubleExecutionOrReplay,
    VulnerabilityPattern::PermitMisuse,
    VulnerabilityPattern::ChainIdorDomainDrift,
    VulnerabilityPattern::PermitFrontRun,
    VulnerabilityPattern::UnsafeAssembyTypeCasts,
    VulnerabilityPattern::StandardViolation,
    VulnerabilityPattern::ReplayAcrossForksOrL2s,
];

pub const VAULT_SHARE_BASED_PATTERNS: &[VulnerabilityPattern; 12] = &[
    VulnerabilityPattern::ERC4626SharePriceMismatch,
    VulnerabilityPattern::PricePrecisionOrRoundingError,
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::SlippageMissingOrInsufficient,
    VulnerabilityPattern::FlashLoanEconomicManipulation,
    VulnerabilityPattern::PrecisionDriftAccumulation,
    VulnerabilityPattern::FeeAccountingDrift,
    VulnerabilityPattern::UnsafeRecipient,
    VulnerabilityPattern::NonStandardERC20Behavior,
    VulnerabilityPattern::FeeOnTransferAssumption,
    VulnerabilityPattern::Reentrancy,
    VulnerabilityPattern::ReadOnlyReentrancy,
];

pub const ORACLE_PRICE_FEED_PATTERNS: &[VulnerabilityPattern; 9] = &[
    VulnerabilityPattern::StaleOracleAcceptance,
    VulnerabilityPattern::OracleUsingDEXorTWAP,
    VulnerabilityPattern::SandwichableOracle,
    VulnerabilityPattern::TWAPWindowPinningOrLowLiquidity,
    VulnerabilityPattern::PricePrecisionOrRoundingError,
    VulnerabilityPattern::ReserveOrPriceDesync,
    VulnerabilityPattern::TimestampOrBlockManipulation,
    VulnerabilityPattern::UncheckedLowLevelCallResults,
    VulnerabilityPattern::ERC20DecimalsMismatch,
];

pub const MATH_LIBRARY_PATTERNS: &[VulnerabilityPattern; 7] = &[
    VulnerabilityPattern::DivideByZeroOrOverFlowInCustomMath,
    VulnerabilityPattern::PricePrecisionOrRoundingError,
    VulnerabilityPattern::PrecisionDriftAccumulation,
    VulnerabilityPattern::ERC20DecimalsMismatch,
    VulnerabilityPattern::UnsafeAssembyTypeCasts,
    VulnerabilityPattern::FeeAccountingDrift,
    VulnerabilityPattern::AccountingInvariantViolation,
];

pub const TOKEN_TRANSFER_LIBRARY_PATTERNS: &[VulnerabilityPattern; 9] = &[
    VulnerabilityPattern::NonStandardERC20Behavior,
    VulnerabilityPattern::FeeOnTransferAssumption,
    VulnerabilityPattern::UncheckedLowLevelCallResults,
    VulnerabilityPattern::EthVsWethConfusion,
    VulnerabilityPattern::UnsafeRecipient,
    VulnerabilityPattern::ERC777HookReentrancy,
    VulnerabilityPattern::AllowanceRace,
    VulnerabilityPattern::StandardViolation,
    VulnerabilityPattern::PullorPushPaymentbugs,
];

pub const GOVERNANCE_TIMELOCK_PATTERNS: &[VulnerabilityPattern; 9] = &[
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::TimelockEdgeCase,
    VulnerabilityPattern::DoubleExecutionOrReplay,
    VulnerabilityPattern::GovernanceDelegationFlaw,
    VulnerabilityPattern::MaturityorGatingByPass,
    VulnerabilityPattern::TimestampOrBlockManipulation,
    VulnerabilityPattern::ConfigFootgun,
    VulnerabilityPattern::UnprotectedPauseOrStop,
    VulnerabilityPattern::EpochOrIndexMonotonicity,
];

pub const PROXY_UPGRADEABLE_PATTERNS: &[VulnerabilityPattern; 8] = &[
    VulnerabilityPattern::UpgradeAuthBypass,
    VulnerabilityPattern::InitOrderOrUnintialized,
    VulnerabilityPattern::StorageCollisionOrSelectorClash,
    VulnerabilityPattern::BeaconOrFactoryAuthorityDrift,
    VulnerabilityPattern::UntrustedDelegateCall,
    VulnerabilityPattern::SelfdestructOrMetamorphicFootguns,
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::ConfigFootgun,
];

pub const STAKING_REWARDS_PATTERNS: &[VulnerabilityPattern; 10] = &[
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::PrecisionDriftAccumulation,
    VulnerabilityPattern::FeeAccountingDrift,
    VulnerabilityPattern::EpochOrIndexMonotonicity,
    VulnerabilityPattern::FlashLoanEconomicManipulation,
    VulnerabilityPattern::Reentrancy,
    VulnerabilityPattern::UnsafeRecipient,
    VulnerabilityPattern::NonStandardERC20Behavior,
    VulnerabilityPattern::FeeOnTransferAssumption,
    VulnerabilityPattern::TimestampOrBlockManipulation,
];

pub const BRIDGE_CROSS_CHAIN_PATTERNS: &[VulnerabilityPattern; 8] = &[
    VulnerabilityPattern::CrossChainMessageSpoofing,
    VulnerabilityPattern::FinalityOrReplayAcrossDomains,
    VulnerabilityPattern::ReplayAcrossForksOrL2s,
    VulnerabilityPattern::ChainIdorDomainDrift,
    VulnerabilityPattern::DoubleExecutionOrReplay,
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::UncheckedLowLevelCallResults,
    VulnerabilityPattern::AccountingInvariantViolation,
];

pub const AMM_DEX_PATTERNS: &[VulnerabilityPattern; 14] = &[
    VulnerabilityPattern::SlippageMissingOrInsufficient,
    VulnerabilityPattern::FlashLoanEconomicManipulation,
    VulnerabilityPattern::OracleUsingDEXorTWAP,
    VulnerabilityPattern::SandwichableOracle,
    VulnerabilityPattern::Reentrancy,
    VulnerabilityPattern::ReadOnlyReentrancy,
    VulnerabilityPattern::MulticallCrossPathReentrancy,
    VulnerabilityPattern::TWAPWindowPinningOrLowLiquidity,
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::PricePrecisionOrRoundingError,
    VulnerabilityPattern::NonStandardERC20Behavior,
    VulnerabilityPattern::FeeOnTransferAssumption,
    VulnerabilityPattern::UnsafeRecipient,
    VulnerabilityPattern::FeeAccountingDrift,
];

pub const LENDING_BORROWING_PATTERNS: &[VulnerabilityPattern; 13] = &[
    VulnerabilityPattern::SlippageMissingOrInsufficient,
    VulnerabilityPattern::StaleOracleAcceptance,
    VulnerabilityPattern::FlashLoanEconomicManipulation,
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::PrecisionDriftAccumulation,
    VulnerabilityPattern::Reentrancy,
    VulnerabilityPattern::ReadOnlyReentrancy,
    VulnerabilityPattern::MulticallCrossPathReentrancy,
    VulnerabilityPattern::NonStandardERC20Behavior,
    VulnerabilityPattern::FeeOnTransferAssumption,
    VulnerabilityPattern::UnsafeRecipient,
    VulnerabilityPattern::FeeAccountingDrift,
    VulnerabilityPattern::ERC20DecimalsMismatch,
];

pub const FACTORY_DEPLOYER_PATTERNS: &[VulnerabilityPattern; 6] = &[
    VulnerabilityPattern::BeaconOrFactoryAuthorityDrift,
    VulnerabilityPattern::InitOrderOrUnintialized,
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::StorageCollisionOrSelectorClash,
    VulnerabilityPattern::SelfdestructOrMetamorphicFootguns,
    VulnerabilityPattern::ConfigFootgun,
];

pub const ERC20_TOKEN_PATTERNS: &[VulnerabilityPattern; 12] = &[
    VulnerabilityPattern::StandardViolation,
    VulnerabilityPattern::NonStandardERC20Behavior,
    VulnerabilityPattern::AllowanceRace,
    VulnerabilityPattern::PermitFrontRun,
    VulnerabilityPattern::PermitMisuse,
    VulnerabilityPattern::ERC20DecimalsMismatch,
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::ConfigFootgun,
    VulnerabilityPattern::UnprotectedPauseOrStop,
    VulnerabilityPattern::UnboundedLoops,
    VulnerabilityPattern::StateGrowthOrStorageBloat,
    VulnerabilityPattern::AccountingInvariantViolation,
];

pub const NFT_COLLECTION_PATTERNS: &[VulnerabilityPattern; 11] = &[
    VulnerabilityPattern::StandardViolation,
    VulnerabilityPattern::UnsafeRecipient,
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::MaturityorGatingByPass,
    VulnerabilityPattern::UnboundedLoops,
    VulnerabilityPattern::StateGrowthOrStorageBloat,
    VulnerabilityPattern::TimestampOrBlockManipulation,
    VulnerabilityPattern::GriefableCallbacks,
    VulnerabilityPattern::Reentrancy,
    VulnerabilityPattern::CEIViolation,
    VulnerabilityPattern::ExternalCallAfterStateChange,
];

pub const AIRDROP_DISTRIBUTOR_PATTERNS: &[VulnerabilityPattern; 11] = &[
    VulnerabilityPattern::DoubleExecutionOrReplay,
    VulnerabilityPattern::EpochOrIndexMonotonicity,
    VulnerabilityPattern::MaturityorGatingByPass,
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::UnboundedLoops,
    VulnerabilityPattern::StateGrowthOrStorageBloat,
    VulnerabilityPattern::UncheckedLowLevelCallResults,
    VulnerabilityPattern::NonStandardERC20Behavior,
    VulnerabilityPattern::FeeOnTransferAssumption,
    VulnerabilityPattern::UnsafeRecipient,
    VulnerabilityPattern::TimestampOrBlockManipulation,
];

pub const ESCROW_VESTING_PATTERNS: &[VulnerabilityPattern; 9] = &[
    VulnerabilityPattern::TimestampOrBlockManipulation,
    VulnerabilityPattern::MaturityorGatingByPass,
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::ConfigFootgun,
    VulnerabilityPattern::UnboundedLoops,
    VulnerabilityPattern::StateGrowthOrStorageBloat,
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::UnsafeRecipient,
    VulnerabilityPattern::ExternalCallAfterStateChange,
];

pub const MARKETPLACE_EXCHANGE_PATTERNS: &[VulnerabilityPattern; 14] = &[
    VulnerabilityPattern::PermitOrSignatureReplay,
    VulnerabilityPattern::EIP1271ByPass,
    VulnerabilityPattern::DoubleExecutionOrReplay,
    VulnerabilityPattern::MulticallCrossPathReentrancy,
    VulnerabilityPattern::Reentrancy,
    VulnerabilityPattern::CEIViolation,
    VulnerabilityPattern::ExternalCallAfterStateChange,
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::FeeAccountingDrift,
    VulnerabilityPattern::NonStandardERC20Behavior,
    VulnerabilityPattern::FeeOnTransferAssumption,
    VulnerabilityPattern::UnsafeRecipient,
    VulnerabilityPattern::TimestampOrBlockManipulation,
    VulnerabilityPattern::ForcedAssetVsStrictEquality,
];

pub const RANDOMNESS_RAFFLE_LOTTERY_PATTERNS: &[VulnerabilityPattern; 10] = &[
    VulnerabilityPattern::BlockhashOrPRNGWeakness,
    VulnerabilityPattern::TimestampOrBlockManipulation,
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::MaturityorGatingByPass,
    VulnerabilityPattern::UnboundedLoops,
    VulnerabilityPattern::StateGrowthOrStorageBloat,
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::UncheckedLowLevelCallResults,
    VulnerabilityPattern::UnsafeRecipient,
    VulnerabilityPattern::ConfigFootgun,
];

pub const LIBRARY_ANALYSIS_PATTERNS: &[VulnerabilityPattern; 20] = &[
    // Math & Precision (4)
    VulnerabilityPattern::PricePrecisionOrRoundingError,
    VulnerabilityPattern::PrecisionDriftAccumulation,
    VulnerabilityPattern::DivideByZeroOrOverFlowInCustomMath,
    VulnerabilityPattern::ERC20DecimalsMismatch,
    // Low-Level & Assembly (2)
    VulnerabilityPattern::UnsafeAssembyTypeCasts,
    VulnerabilityPattern::UncheckedLowLevelCallResults,
    // Standard Compliance (3)
    VulnerabilityPattern::StandardViolation,
    VulnerabilityPattern::NonStandardERC20Behavior,
    VulnerabilityPattern::ERC4626SharePriceMismatch,
    // Accounting & Invariants (2)
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::FeeAccountingDrift,
    // Oracle & Price (1)
    VulnerabilityPattern::StaleOracleAcceptance,
    // Input Validation & Edge Cases (3)
    VulnerabilityPattern::ForcedAssetVsStrictEquality,
    VulnerabilityPattern::UnsafeRecipient,
    VulnerabilityPattern::ExternalCallAfterStateChange,
    // Auth & Signature Validation (5)
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::PermitOrSignatureReplay,
    VulnerabilityPattern::EIP1271ByPass,
    VulnerabilityPattern::DoubleExecutionOrReplay,
    VulnerabilityPattern::PermitMisuse,
];

// TOP VULNERABILITY PATTERNS

/// Curated list of High/Medium severity patterns for focused analysis.
/// Excludes Low-severity patterns (TimestampOrBlockManipulation, ChainIdorDomainDrift).
pub const R1_PATTERNS: &[VulnerabilityPattern; 49] = &[
    // ============================================
    // SYNTACTIC PATTERNS (Easy for LLMs)
    // ============================================
    // These are deterministic, single-contract, visible in code syntax
    // LLMs have HIGH recall on these patterns

    // Auth & Access Control (syntactically visible modifiers/checks)
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::DoubleExecutionOrReplay,
    VulnerabilityPattern::PermitOrSignatureReplay,
    VulnerabilityPattern::EIP1271ByPass,
    // Call Order & Reentrancy (syntactically visible call patterns)
    VulnerabilityPattern::CEIViolation,
    VulnerabilityPattern::Reentrancy,
    VulnerabilityPattern::ReadOnlyReentrancy,
    VulnerabilityPattern::ExternalCallAfterStateChange,
    // Economic & Oracle (syntactically visible oracle calls)
    VulnerabilityPattern::SlippageMissingOrInsufficient,
    VulnerabilityPattern::OracleUsingDEXorTWAP,
    VulnerabilityPattern::FeeOnTransferAssumption,
    VulnerabilityPattern::ReserveOrPriceDesync,
    // Accounting & Invariants (syntactically visible math)
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::UnsafeRecipient,
    VulnerabilityPattern::PrecisionDriftAccumulation,
    VulnerabilityPattern::PricePrecisionOrRoundingError,
    // Token Standard (syntactically visible allowance/transfer)
    VulnerabilityPattern::StandardViolation,
    VulnerabilityPattern::AllowanceRace,
    VulnerabilityPattern::PermitMisuse,
    // Upgradeability & Init (syntactically visible init/upgrade)
    VulnerabilityPattern::UpgradeAuthBypass,
    VulnerabilityPattern::InitOrderOrUnintialized,
    VulnerabilityPattern::StorageCollisionOrSelectorClash,
    VulnerabilityPattern::SelfdestructOrMetamorphicFootguns,
    // Lifecycle & State Machines (syntactically visible state checks)
    VulnerabilityPattern::MaturityorGatingByPass,
    VulnerabilityPattern::EpochOrIndexMonotonicity,
    // DoS, Gas, and Complexity (syntactically visible loops)
    VulnerabilityPattern::UnboundedLoops,
    VulnerabilityPattern::StateGrowthOrStorageBloat,
    // Randomness & Time (syntactically visible timestamp/blockhash)
    VulnerabilityPattern::TimestampOrBlockManipulation,
    VulnerabilityPattern::ChainIdorDomainDrift,
    // Cross-Chain (syntactically visible message validation)
    VulnerabilityPattern::CrossChainMessageSpoofing,
    VulnerabilityPattern::FinalityOrReplayAcrossDomains,
    // EVM/Assembly (syntactically visible low-level calls)
    VulnerabilityPattern::UncheckedLowLevelCallResults,
    VulnerabilityPattern::UnsafeAssembyTypeCasts,
    VulnerabilityPattern::DivideByZeroOrOverFlowInCustomMath,
    // ETH/WETH (syntactically visible ETH/WETH handling)
    VulnerabilityPattern::EthVsWethConfusion,
    VulnerabilityPattern::PullorPushPaymentbugs,
    // Token Behavior (syntactically visible token interactions)
    VulnerabilityPattern::NonStandardERC20Behavior,
    VulnerabilityPattern::ERC20DecimalsMismatch,
    // Oracle (syntactically visible staleness checks)
    VulnerabilityPattern::StaleOracleAcceptance,
    VulnerabilityPattern::SandwichableOracle,
    // Pause/Stop (syntactically visible pause modifiers)
    VulnerabilityPattern::UnprotectedPauseOrStop,
    // Delegatecall (syntactically visible delegatecall)
    VulnerabilityPattern::UntrustedDelegateCall,
    // Replay (syntactically visible replay protection)
    VulnerabilityPattern::ReplayAcrossForksOrL2s,
    // ERC4626 (syntactically visible share/asset math)
    VulnerabilityPattern::ERC4626SharePriceMismatch,
    // Fee Accounting (syntactically visible fee math)
    VulnerabilityPattern::FeeAccountingDrift,
    // Beacon/Factory (syntactically visible authority checks)
    VulnerabilityPattern::BeaconOrFactoryAuthorityDrift,
    // Timelock (syntactically visible timelock checks)
    VulnerabilityPattern::TimelockEdgeCase,
    // TWAP (syntactically visible TWAP window)
    VulnerabilityPattern::TWAPWindowPinningOrLowLiquidity,
    // Asset Equality (syntactically visible balance checks)
    VulnerabilityPattern::ForcedAssetVsStrictEquality,
];

pub const R2_PATTERNS: &[VulnerabilityPattern; 24] = &[
    // ============================================
    // SEMANTIC PATTERNS (Hard for LLMs)
    // ============================================
    // These require deep understanding, multi-step reasoning,
    // game theory, probabilistic analysis, or cross-contract tracing
    // LLMs have LOW recall on these patterns

    // Auth & Governance (requires understanding delegation/governance flow)
    VulnerabilityPattern::GovernanceDelegationFlaw,
    VulnerabilityPattern::ConfigFootgun, // Requires understanding admin trust assumptions
    // Economic & Game Theory (requires economic/incentive analysis)
    VulnerabilityPattern::FlashLoanEconomicManipulation, // Multi-step attack
    VulnerabilityPattern::UnincentivizedMaintenanceOrKeeperlessProgress, // Requires incentive analysis
    VulnerabilityPattern::FirstOrLastMoverAdvantage, // Requires bank-run / timing race reasoning
    VulnerabilityPattern::CheapGriefingOrDosProfit,  // Requires griefing cost vs profit analysis
    VulnerabilityPattern::QueueOrderDependentMevExtraction, // Requires order-dependent MEV analysis
    VulnerabilityPattern::FixedPotRewardRaceOrGasAuction, // Requires gas-auction incentive analysis
    VulnerabilityPattern::RewardCheckpointFreeRiderOrLateJoiner, // Requires reward fairness over time analysis
    VulnerabilityPattern::GovernanceCaptureOrTreasuryExtraction, // Requires gov power distribution analysis
    VulnerabilityPattern::CrossRoleCollusionWithoutSlashing, // Requires cross-role collusion reasoning
    VulnerabilityPattern::IncentiveMisalignmentOrGameTheory, // Catch-all game theory bucket
    // Callbacks & Hooks (requires understanding callback flow)
    VulnerabilityPattern::GriefableCallbacks, // Requires understanding griefing incentives
    VulnerabilityPattern::ERC777HookReentrancy, // Requires understanding ERC777 hooks
    // Randomness (requires probabilistic/cryptographic analysis)
    VulnerabilityPattern::BlockVarsAsPrimaryRandomnessSource, // Direct blockvars RNG; miner/validator grinding
    VulnerabilityPattern::InsecureOnChainPrngWithoutCommitReveal, // Custom PRNG without commit-reveal/unbiasing
    VulnerabilityPattern::BlockhashOrPRNGWeakness,                // Catch-all RNG bucket
    // Permit (requires understanding frontrunning)
    VulnerabilityPattern::PermitFrontRun, // Requires multi-step attack sequencing
    // Multicall (requires cross-path analysis)
    VulnerabilityPattern::MulticallCrossPathReentrancy, // Requires understanding multicall interactions
    // NEW MEGAPOT PATTERNS (all require deep semantic understanding)
    VulnerabilityPattern::ArbitraryExternalCall, // Requires multi-step attack chain + custody analysis
    VulnerabilityPattern::GlobalParamMidFlowManipulation, // Requires temporal state analysis
    VulnerabilityPattern::GovernanceFrontrunDoS, // Requires game theory + frontrunning analysis
    VulnerabilityPattern::ExternalProtocolKeyCollision, // Requires external protocol knowledge
    VulnerabilityPattern::EmergencyModeStateStuck, // Requires state machine analysis
];

// Custom utility library pattern sets
pub const BYTE_MANIPULATION_LIBRARY_PATTERNS: &[VulnerabilityPattern; 5] = &[
    VulnerabilityPattern::UnsafeAssembyTypeCasts,
    VulnerabilityPattern::UncheckedLowLevelCallResults,
    VulnerabilityPattern::DivideByZeroOrOverFlowInCustomMath, // covers out-of-bounds via overflow/underflow
    VulnerabilityPattern::StandardViolation,
    VulnerabilityPattern::AccountingInvariantViolation, // dirty values causing incorrect state
];

pub const ENCODING_DECODING_LIBRARY_PATTERNS: &[VulnerabilityPattern; 4] = &[
    VulnerabilityPattern::UnsafeAssembyTypeCasts,
    VulnerabilityPattern::StandardViolation,
    VulnerabilityPattern::UncheckedLowLevelCallResults,
    VulnerabilityPattern::ForcedAssetVsStrictEquality,
];

pub const STORAGE_HELPER_LIBRARY_PATTERNS: &[VulnerabilityPattern; 4] = &[
    VulnerabilityPattern::StorageCollisionOrSelectorClash,
    VulnerabilityPattern::InitOrderOrUnintialized,
    VulnerabilityPattern::UnsafeAssembyTypeCasts,
    VulnerabilityPattern::ConfigFootgun,
];

pub const ERROR_DEFINITION_LIBRARY_PATTERNS: &[VulnerabilityPattern; 1] =
    &[VulnerabilityPattern::StandardViolation];

pub const ACCESS_CONTROL_MODIFIER_PATTERNS: &[VulnerabilityPattern; 5] = &[
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::UnprotectedPauseOrStop,
    VulnerabilityPattern::UntrustedDelegateCall, // delegatecall context issues (msg.sender spoofing)
    VulnerabilityPattern::GovernanceDelegationFlaw,
    VulnerabilityPattern::ConfigFootgun,
];

pub const REENTRANCY_GUARD_LIBRARY_PATTERNS: &[VulnerabilityPattern; 4] = &[
    VulnerabilityPattern::Reentrancy,
    VulnerabilityPattern::ReadOnlyReentrancy,
    VulnerabilityPattern::MulticallCrossPathReentrancy,
    VulnerabilityPattern::CEIViolation,
];
