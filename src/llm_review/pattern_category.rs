use crate::llm_review::patterns::VulnerabilityPattern;
use std::{collections::HashMap, sync::OnceLock};
/// Configuration management for the AI Agent Audit application.
///
/// This module provides centralized configuration handling, with constants
/// for application settings and environment variables only for sensitive
/// configuration like API keys and URLs.
use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum PatternCategory {
    EconomicHit,
    CallOrder,
    AuthByPass,
    AccountingMess,
    TokenHit,
    PermitExpired,
    Gasy,
    BrokenMachine,
    UpgradeFlop,
    Evm,
    Randomness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternTier {
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}

pub struct PatternCategorySpec {
    pub category: PatternCategory,
    pub title: &'static str,
    pub issues: &'static [VulnerabilityPattern],
    pub tier: PatternTier,
}

pub const PATTERN_CATEGORY_LIBRARY: &[PatternCategorySpec] = &[
    PatternCategorySpec {
        category: PatternCategory::EconomicHit,
        title: "Economic, Market, & Oracle Vulnerabilities",
        issues: ECONOMIC_HIT,
        tier: PatternTier::Tier1,
    },
    PatternCategorySpec {
        category: PatternCategory::CallOrder,
        title: "Call Order and Reentrancy Vulnerabilities",
        issues: CALL_ORDER,
        tier: PatternTier::Tier1,
    },
    PatternCategorySpec {
        category: PatternCategory::AuthByPass,
        title: "Access Control and Authorization Bypass",
        issues: AUTH_BYPASS,
        tier: PatternTier::Tier1,
    },
    PatternCategorySpec {
        category: PatternCategory::AccountingMess,
        title: "Accounting and Precision Vulnerabilities",
        issues: ACCOUNTING_MISS,
        tier: PatternTier::Tier2,
    },
    PatternCategorySpec {
        category: PatternCategory::TokenHit,
        title: "Token Standard and Allowance Vulnerabilities",
        issues: TOKEN_HIT,
        tier: PatternTier::Tier2,
    },
    PatternCategorySpec {
        category: PatternCategory::PermitExpired,
        title: "Permit and Signature Vulnerabilities",
        issues: PERMIT_EXPIRED,
        tier: PatternTier::Tier2,
    },
    PatternCategorySpec {
        category: PatternCategory::Gasy,
        title: "Gas Consumption and DoS Vectors",
        issues: GASY,
        tier: PatternTier::Tier3,
    },
    PatternCategorySpec {
        category: PatternCategory::BrokenMachine,
        title: "State Machine and Epoch Monotonicity",
        issues: BROKEN_MACHINE,
        tier: PatternTier::Tier3,
    },
    PatternCategorySpec {
        category: PatternCategory::UpgradeFlop,
        title: "Upgrade and Proxy Misconfiguration",
        issues: UPGRADE_FLOP,
        tier: PatternTier::Tier3,
    },
    PatternCategorySpec {
        category: PatternCategory::Evm,
        title: "EVM-Level and Low-level Call Risks",
        issues: EVM,
        tier: PatternTier::Tier4,
    },
    PatternCategorySpec {
        category: PatternCategory::Randomness,
        title: "Randomness and Timestamp Manipulation",
        issues: RANDOMNESS,
        tier: PatternTier::Tier4,
    },
];

/// One-time initialized map from PatternCategory -> &'static PatternCategorySpec
static PATTERN_CATEGORY_MAP: OnceLock<HashMap<PatternCategory, &'static PatternCategorySpec>> =
    OnceLock::new();

/// Initialize the category map; safe to call multiple times (subsequent calls are no-ops)
pub fn init_pattern_categories() {
    let _ = PATTERN_CATEGORY_MAP.set(
        PATTERN_CATEGORY_LIBRARY
            .iter()
            .map(|spec| (spec.category.clone(), spec))
            .collect::<HashMap<_, _>>(),
    );
}

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
pub fn get_category_spec(cat: &PatternCategory) -> Option<&'static PatternCategorySpec> {
    pattern_category_map().get(cat).copied()
}

//**************************************************
//**************************************************
// TIER 1 -> 3 LLM RUNS
pub const ECONOMIC_HIT: &[VulnerabilityPattern; 7] = &[
    VulnerabilityPattern::FlashLoanEconomicManipulation,
    VulnerabilityPattern::FeeOnTransferAssumption,
    VulnerabilityPattern::ReserveOrPriceDesync,
    VulnerabilityPattern::OracleUsingDEXorTWAP,
    VulnerabilityPattern::SlippageMissingOrInsufficient,
    VulnerabilityPattern::StaleOracleAcceptance,
    VulnerabilityPattern::SandwichableOracle,
];

// TIER 1 -> 2 LLM RUNS
pub const CALL_ORDER: &[VulnerabilityPattern; 2] = &[
    VulnerabilityPattern::Reentrancy,
    VulnerabilityPattern::ExternalCallAfterStateChange,
];

// TIER 1 -> 3 rounds
pub const AUTH_BYPASS: &[VulnerabilityPattern; 4] = &[
    VulnerabilityPattern::AccessControlOrAuthByPass,
    VulnerabilityPattern::GovernanceDelegationFlaw,
    VulnerabilityPattern::DoubleExecutionOrReplay,
    VulnerabilityPattern::EIP1271ByPass,
];
//**************************************************
//**************************************************

//**************************************************
//**************************************************
// TIER 2
pub const ACCOUNTING_MISS: &[VulnerabilityPattern; 5] = &[
    VulnerabilityPattern::AccountingInvariantViolation,
    VulnerabilityPattern::PricePrecisionOrRoundingError,
    VulnerabilityPattern::PrecisionDriftAccumulation,
    VulnerabilityPattern::ERC4626SharePriceMismatch,
    VulnerabilityPattern::FeeAccountingDrift,
];

// Token Standard / ALLOWANCE
// TIER 2
pub const TOKEN_HIT: &[VulnerabilityPattern; 4] = &[
    VulnerabilityPattern::StandardViolation,
    VulnerabilityPattern::NonStandardERC20Behavior,
    VulnerabilityPattern::ERC20DecimalsMismatch,
    VulnerabilityPattern::ERC777HookReentrancy,
];

// TIER 2
pub const PERMIT_EXPIRED: &[VulnerabilityPattern; 3] = &[
    VulnerabilityPattern::PermitOrSignatureReplay,
    VulnerabilityPattern::PermitFrontRun,
    VulnerabilityPattern::PermitMisuse,
];
//**************************************************
//**************************************************

//**************************************************
//**************************************************
// TIER 3
pub const GASY: &[VulnerabilityPattern; 3] = &[
    VulnerabilityPattern::UnboundedLoops,
    VulnerabilityPattern::GriefableCallbacks,
    VulnerabilityPattern::StateGrowthOrStorageBloat,
];

// TIER 3
pub const BROKEN_MACHINE: &[VulnerabilityPattern; 2] = &[
    VulnerabilityPattern::EpochOrIndexMonotonicity,
    VulnerabilityPattern::SelfdestructOrMetamorphicFootguns,
];

// TIER 3
pub const UPGRADE_FLOP: &[VulnerabilityPattern; 3] = &[
    VulnerabilityPattern::InitOrderOrUnintialized,
    VulnerabilityPattern::UpgradeAuthBypass,
    VulnerabilityPattern::StorageCollisionOrSelectorClash,
];
//**************************************************
//**************************************************

//**************************************************
//**************************************************
// TIER 4
pub const EVM: &[VulnerabilityPattern; 3] = &[
    VulnerabilityPattern::UntrustedDelegateCall,
    VulnerabilityPattern::UnsafeAssembyTypeCasts,
    VulnerabilityPattern::UncheckedLowLevelCallResults,
];

// TIER 4
pub const RANDOMNESS: &[VulnerabilityPattern; 2] = &[
    VulnerabilityPattern::TimestampOrBlockManipulation,
    VulnerabilityPattern::BlockhashOrPRNGWeakness,
];

//**************************************************
//**************************************************

pub const FREQUENT_PATTERNS: &[VulnerabilityPattern; 12] = &[
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
];

// TOP VULNERABILITY PATTERNS
pub const TOP_PAID_PATTERNS: &[VulnerabilityPattern; 14] = &[
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
];
