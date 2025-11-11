use crate::{
    config::{
        MAX_PATTERN_GENERAL, MAX_PATTERN_LIBRARY, MAX_PATTERN_NICHE, MAX_PATTERN_RUN_FREQUENT,
        MAX_PATTERN_RUN_MOST, MAX_PATTERN_RUN_RARE, MAX_PATTERN_RUN_TOP,
    },
    llm_review::patterns::VulnerabilityPattern,
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
        category: PatternCategory::SimulationTestingHelper,
        title: "Simulation/Testing Helper",
        issues: SIMULATION_TESTING_HELPER_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_LIBRARY,
    },
    PatternCategorySpec {
        category: PatternCategory::General,
        title: "Common High/Medium Vulnerabilities",
        issues: COMMON_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_GENERAL,
    },
    PatternCategorySpec {
        category: PatternCategory::Top,
        title: "Top Code4rena",
        issues: TOP_PAID_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_RUN_TOP,
    },
    PatternCategorySpec {
        category: PatternCategory::Rare,
        title: "Top Code4rena",
        issues: RARE_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_RUN_RARE,
    },
    PatternCategorySpec {
        category: PatternCategory::Frequent,
        title: "Most Frequent Code4rena",
        issues: FREQUENT_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_RUN_FREQUENT,
    },
    PatternCategorySpec {
        category: PatternCategory::MostObserved,
        title: "Most Frequent Code4rena",
        issues: TOP_OBSERVED_PATTERNS,
        tier: PatternTier::Tier1,
        runs: MAX_PATTERN_RUN_MOST,
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

pub const COMMON_PATTERNS: &[VulnerabilityPattern; 20] = &[
    // Core Access Control & Auth (3 patterns) - Keep most critical, remove signature-specific
    VulnerabilityPattern::AccessControlOrAuthByPass, // High - universal auth bypass
    VulnerabilityPattern::DoubleExecutionOrReplay,   // HighMedium - replay attacks
    VulnerabilityPattern::UnprotectedPauseOrStop,    // HighMedium - emergency controls
    // Reentrancy & Call Order (3 patterns) - Consolidate overlaps
    VulnerabilityPattern::Reentrancy, // High - covers CEIViolation + classic reentrancy
    VulnerabilityPattern::MulticallCrossPathReentrancy,
    VulnerabilityPattern::ExternalCallAfterStateChange, // Medium - CEI violations
    // Economic & Flash Loans (2 patterns) - Remove oracle-specific patterns
    VulnerabilityPattern::FlashLoanEconomicManipulation, // High - universal economic attack
    VulnerabilityPattern::SlippageMissingOrInsufficient, // Medium - common in any swap/trade
    // Accounting & Invariants (3 patterns) - Remove vault-specific
    VulnerabilityPattern::AccountingInvariantViolation, // HighMedium - universal accounting
    VulnerabilityPattern::FeeAccountingDrift,
    VulnerabilityPattern::PrecisionDriftAccumulation, // Medium - rounding errors
    VulnerabilityPattern::UnsafeRecipient,            // HighMedium - recipient validation
    // Token Standards (2 patterns) - Keep most common
    VulnerabilityPattern::AllowanceRace, // Medium - approve race condition
    // Upgradeability & Proxies (2 patterns) - Keep highest impact, remove proxy-specific
    VulnerabilityPattern::UpgradeAuthBypass, // High - universal upgrade risk
    VulnerabilityPattern::InitOrderOrUnintialized, // HighMedium - init vulnerabilities
    // DoS & Complexity (2 patterns) - Keep most common
    VulnerabilityPattern::UnboundedLoops,     // Medium - gas/DoS
    VulnerabilityPattern::GriefableCallbacks, // Medium - callback griefing
    // Low-Level & Assembly (2 patterns) - Keep critical
    VulnerabilityPattern::UncheckedLowLevelCallResults, // Medium - silent failures
    VulnerabilityPattern::UnsafeAssembyTypeCasts,       // HighMedium - type safety
    // Universal Edge Cases (1 pattern)
    VulnerabilityPattern::ForcedAssetVsStrictEquality, // High - strict equality bugs
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

pub const TOP_OBSERVED_PATTERNS: &[VulnerabilityPattern; 10] = &[
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::GriefableCallbacks,
    VulnerabilityPattern::UnboundedLoops,
    VulnerabilityPattern::FlashLoanEconomicManipulation,
    VulnerabilityPattern::FeeOnTransferAssumption,
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::FeeOnTransferAssumption,
    VulnerabilityPattern::ReserveOrPriceDesync,
    VulnerabilityPattern::MaturityorGatingByPass,
    VulnerabilityPattern::StandardViolation,
];

pub const RARE_PATTERNS: &[VulnerabilityPattern; 13] = &[
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::FeeAccountingDrift,
    VulnerabilityPattern::ERC4626SharePriceMismatch,
    VulnerabilityPattern::ERC20DecimalsMismatch,
    VulnerabilityPattern::MulticallCrossPathReentrancy,
    VulnerabilityPattern::TWAPWindowPinningOrLowLiquidity,
    VulnerabilityPattern::StaleOracleAcceptance,
    VulnerabilityPattern::ForcedAssetVsStrictEquality,
    VulnerabilityPattern::UntrustedDelegateCall,
    VulnerabilityPattern::SlippageMissingOrInsufficient,
    VulnerabilityPattern::PermitMisuse,
    VulnerabilityPattern::PermitFrontRun,
    VulnerabilityPattern::BeaconOrFactoryAuthorityDrift,
];

pub const FREQUENT_PATTERNS: &[VulnerabilityPattern; 15] = &[
    VulnerabilityPattern::SlippageMissingOrInsufficient,
    VulnerabilityPattern::UnboundedLoops,
    VulnerabilityPattern::FeeOnTransferAssumption,
    VulnerabilityPattern::ReserveOrPriceDesync,
    VulnerabilityPattern::PricePrecisionOrRoundingError,
    VulnerabilityPattern::PrecisionDriftAccumulation,
    VulnerabilityPattern::ExternalCallAfterStateChange,
    VulnerabilityPattern::GriefableCallbacks,
    VulnerabilityPattern::StateGrowthOrStorageBloat,
    VulnerabilityPattern::EpochOrIndexMonotonicity,
    VulnerabilityPattern::TimestampOrBlockManipulation,
    VulnerabilityPattern::BlockhashOrPRNGWeakness,
    // NEW:
    VulnerabilityPattern::MulticallCrossPathReentrancy,
    VulnerabilityPattern::TWAPWindowPinningOrLowLiquidity,
    VulnerabilityPattern::ForcedAssetVsStrictEquality,
];

// TOP VULNERABILITY PATTERNS

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

pub const SIMULATION_TESTING_HELPER_PATTERNS: &[VulnerabilityPattern; 5] = &[
    VulnerabilityPattern::ExternalCallAfterStateChange,
    VulnerabilityPattern::UnboundedLoops,
    VulnerabilityPattern::UncheckedLowLevelCallResults,
    VulnerabilityPattern::GriefableCallbacks,
    VulnerabilityPattern::StateGrowthOrStorageBloat,
];

pub const TOP_PAID_PATTERNS: &[VulnerabilityPattern; 16] = &[
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::GovernanceDelegationFlaw,
    VulnerabilityPattern::DoubleExecutionOrReplay,
    VulnerabilityPattern::EIP1271ByPass,
    VulnerabilityPattern::PermitOrSignatureReplay,
    VulnerabilityPattern::Reentrancy,
    VulnerabilityPattern::FlashLoanEconomicManipulation,
    VulnerabilityPattern::OracleUsingDEXorTWAP,
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::StandardViolation,
    VulnerabilityPattern::PermitMisuse,
    VulnerabilityPattern::InitOrderOrUnintialized,
    VulnerabilityPattern::UpgradeAuthBypass,
    VulnerabilityPattern::StorageCollisionOrSelectorClash,
    // NEW:
    VulnerabilityPattern::BeaconOrFactoryAuthorityDrift,
    VulnerabilityPattern::TimelockEdgeCase,
];
