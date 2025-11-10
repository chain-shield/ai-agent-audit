use std::{collections::HashMap, sync::OnceLock};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::llm_review::patterns::VulnerabilityPattern;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    JsonSchema,
    Hash,
    PartialEq,
    Eq,
    Default,
    EnumIter,
    strum_macros::Display,
    strum_macros::EnumString,
)]
pub enum ContractCategory {
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
    #[default]
    Unknown,
}

#[derive(Debug, Clone, JsonSchema, Default)]
pub struct ContractCategorySpec {
    pub category: ContractCategory,
    pub description: &'static str,
    pub vulnerability_patterns: &'static [VulnerabilityPattern],
}

static CONTRACT_CATEGORY_MAP: OnceLock<HashMap<ContractCategory, &'static ContractCategorySpec>> =
    OnceLock::new();

fn contract_category_map() -> &'static HashMap<ContractCategory, &'static ContractCategorySpec> {
    CONTRACT_CATEGORY_MAP.get_or_init(|| {
        CONTRACT_CATEGORY_LIBRARY
            .iter()
            .map(|spec| (spec.category.clone(), spec))
            .collect::<HashMap<_, _>>()
    })
}

pub fn get_contract_library_spec(cat: &ContractCategory) -> Option<&'static ContractCategorySpec> {
    contract_category_map().get(cat).copied()
}

pub fn generate_formated_list_of_contract_categories(
    contract_categories: &[ContractCategory],
) -> String {
    let category_specs: Vec<ContractCategorySpec> = CONTRACT_CATEGORY_LIBRARY
        .iter()
        .filter(|v| contract_categories.contains(&v.category))
        .map(|v| v.to_owned())
        .collect();

    let mut category_list = String::new();

    category_list.push_str("### Contract Categories\n");
    for category_spec in category_specs {
        category_list.push_str(&format!("Category: {}\n", category_spec.category));
        category_list.push_str(&format!("Description: {}\n", category_spec.description));
        category_list.push_str("\n");
    }

    category_list
}

pub static CONTRACT_CATEGORY_LIBRARY: &[ContractCategorySpec] = &[
    ContractCategorySpec {
        category: ContractCategory::SignatureValidation,
        description: "Contracts that validate signatures, implement EIP-712/EIP-1271, or manage multi-sig/account abstraction logic. Focus on signature replay, checkpointer bypass, and auth validation flaws.",
        vulnerability_patterns: &[
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
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::VaultShareBased,
        description: "ERC4626 vaults or share-based contracts where users deposit assets and receive shares representing proportional ownership. Focus on share price manipulation, inflation attacks, and rounding errors.",
        vulnerability_patterns: &[
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
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::OraclePriceFeed,
        description: "Contracts that provide price data, aggregate oracle feeds, or implement TWAP/Chainlink integrations. Focus on stale data acceptance, manipulation resistance, and price validation.",
        vulnerability_patterns: &[
            VulnerabilityPattern::StaleOracleAcceptance,
            VulnerabilityPattern::OracleUsingDEXorTWAP,
            VulnerabilityPattern::SandwichableOracle,
            VulnerabilityPattern::TWAPWindowPinningOrLowLiquidity,
            VulnerabilityPattern::PricePrecisionOrRoundingError,
            VulnerabilityPattern::ReserveOrPriceDesync,
            VulnerabilityPattern::TimestampOrBlockManipulation,
            VulnerabilityPattern::UncheckedLowLevelCallResults,
            VulnerabilityPattern::ERC20DecimalsMismatch,
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::MathLibrary,
        description: "Pure math libraries providing arithmetic operations, fixed-point math, or custom calculations. Focus on overflow/underflow, division by zero, precision loss, and rounding errors.",
        vulnerability_patterns: &[
            VulnerabilityPattern::DivideByZeroOrOverFlowInCustomMath,
            VulnerabilityPattern::PricePrecisionOrRoundingError,
            VulnerabilityPattern::PrecisionDriftAccumulation,
            VulnerabilityPattern::ERC20DecimalsMismatch,
            VulnerabilityPattern::UnsafeAssembyTypeCasts,
            VulnerabilityPattern::FeeAccountingDrift,
            VulnerabilityPattern::AccountingInvariantViolation,
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::TokenTransferLibrary,
        description: "Libraries handling token transfers, approvals, or wrapping/unwrapping logic (e.g., SafeERC20, ETH/WETH helpers). Focus on non-standard token behavior, low-level call failures, and ETH handling.",
        vulnerability_patterns: &[
            VulnerabilityPattern::NonStandardERC20Behavior,
            VulnerabilityPattern::FeeOnTransferAssumption,
            VulnerabilityPattern::UncheckedLowLevelCallResults,
            VulnerabilityPattern::EthVsWethConfusion,
            VulnerabilityPattern::UnsafeRecipient,
            VulnerabilityPattern::ERC777HookReentrancy,
            VulnerabilityPattern::AllowanceRace,
            VulnerabilityPattern::StandardViolation,
            VulnerabilityPattern::PullorPushPaymentbugs,
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::GovernanceTimeLock,
        description: "Governance contracts with timelocks, voting mechanisms, or proposal execution logic. Focus on timelock bypass, execution replay, and privilege escalation.",
        vulnerability_patterns: &[
            VulnerabilityPattern::AccessControlOrAuthByPass,
            VulnerabilityPattern::TimelockEdgeCase,
            VulnerabilityPattern::DoubleExecutionOrReplay,
            VulnerabilityPattern::GovernanceDelegationFlaw,
            VulnerabilityPattern::MaturityorGatingByPass,
            VulnerabilityPattern::TimestampOrBlockManipulation,
            VulnerabilityPattern::ConfigFootgun,
            VulnerabilityPattern::UnprotectedPauseOrStop,
            VulnerabilityPattern::EpochOrIndexMonotonicity,
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::ProxyUpgradeable,
        description: "Upgradeable proxy contracts (UUPS, Transparent, Beacon) or implementation contracts with initialization logic. Focus on storage collisions, uninitialized state, and upgrade auth bypass.",
        vulnerability_patterns: &[
            VulnerabilityPattern::UpgradeAuthBypass,
            VulnerabilityPattern::InitOrderOrUnintialized,
            VulnerabilityPattern::StorageCollisionOrSelectorClash,
            VulnerabilityPattern::BeaconOrFactoryAuthorityDrift,
            VulnerabilityPattern::UntrustedDelegateCall,
            VulnerabilityPattern::SelfdestructOrMetamorphicFootguns,
            VulnerabilityPattern::AccessControlOrAuthByPass,
            VulnerabilityPattern::ConfigFootgun,
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::StakingRewards,
        description: "Staking contracts that distribute rewards over time based on user deposits and duration. Focus on reward accounting drift, precision loss, and economic manipulation.",
        vulnerability_patterns: &[
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
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::BridgeCrossChain,
        description: "Cross-chain bridges, message relayers, or L1/L2 communication contracts. Focus on message spoofing, replay attacks across chains, and finality assumptions.",
        vulnerability_patterns: &[
            VulnerabilityPattern::CrossChainMessageSpoofing,
            VulnerabilityPattern::FinalityOrReplayAcrossDomains,
            VulnerabilityPattern::ReplayAcrossForksOrL2s,
            VulnerabilityPattern::ChainIdorDomainDrift,
            VulnerabilityPattern::DoubleExecutionOrReplay,
            VulnerabilityPattern::AccessControlOrAuthByPass,
            VulnerabilityPattern::UncheckedLowLevelCallResults,
            VulnerabilityPattern::AccountingInvariantViolation,
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::AMMDex,
        description: "Automated market makers, DEXs, or liquidity pool contracts implementing swap/add/remove liquidity logic. Focus on slippage protection, price manipulation, and reentrancy.",
        vulnerability_patterns: &[
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
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::LendingBorrowing,
        description: "Lending protocols with collateral management, liquidation logic, and interest accrual. Focus on oracle manipulation, liquidation protection, and accounting invariants.",
        vulnerability_patterns: &[
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
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::FactoryDeployer,
        description: "Factory contracts that deploy new contract instances using CREATE2 or clones. Focus on authority drift, initialization issues, and deterministic address collisions.",
        vulnerability_patterns: &[
            VulnerabilityPattern::BeaconOrFactoryAuthorityDrift,
            VulnerabilityPattern::InitOrderOrUnintialized,
            VulnerabilityPattern::AccessControlOrAuthByPass,
            VulnerabilityPattern::StorageCollisionOrSelectorClash,
            VulnerabilityPattern::SelfdestructOrMetamorphicFootguns,
            VulnerabilityPattern::ConfigFootgun,
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::ERC20Token,
        description: "Fungible token implementations (ERC20/Permit/custom fees) with mint/burn and role-based controls. Focus on standards compliance, allowance/permit edge cases, and admin footguns.",
        vulnerability_patterns: &[
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
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::NFTCollection,
        description: "Non-fungible/multi-token collections (ERC721/ERC1155) with minting, burning, and metadata. Focus on safe transfer handling, operator approvals, mint phase gating, and denial-of-service via hooks.",
        vulnerability_patterns: &[
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
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::AirdropDistributor,
        description: "Distribution contracts (Merkle claimers, claim portals) enabling one-time or periodic claims. Focus on double-claim prevention, index monotonicity, and non-standard token interactions.",
        vulnerability_patterns: &[
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
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::EscrowVesting,
        description: "Vesting, streaming, or escrow contracts releasing funds over time or conditions. Focus on time gates, release schedule correctness, and withdrawal authorizations.",
        vulnerability_patterns: &[
            VulnerabilityPattern::TimestampOrBlockManipulation,
            VulnerabilityPattern::MaturityorGatingByPass,
            VulnerabilityPattern::AccessControlOrAuthByPass,
            VulnerabilityPattern::ConfigFootgun,
            VulnerabilityPattern::UnboundedLoops,
            VulnerabilityPattern::StateGrowthOrStorageBloat,
            VulnerabilityPattern::AccountingInvariantViolation,
            VulnerabilityPattern::UnsafeRecipient,
            VulnerabilityPattern::ExternalCallAfterStateChange,
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::MarketplaceExchange,
        description: "Order-book or marketplace contracts (list/bid/auction) trading tokens or NFTs. Focus on signature replay, auction edge cases, reentrancy, and accounting fairness.",
        vulnerability_patterns: &[
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
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::RandomnessRaffleLottery,
        description: "Randomness-dependent raffles, lotteries, and draws (on-chain PRNG or VRF consumers). Focus on PRNG weaknesses, timestamp manipulation, and commit/reveal protocol correctness.",
        vulnerability_patterns: &[
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
        ],
    },
    ContractCategorySpec {
        category: ContractCategory::Unknown,
        description: "Contracts that don't fit into specific categories or have mixed functionality. Use broad pattern coverage for general security analysis.",
        vulnerability_patterns: &[
            // Core security (8 patterns)
            VulnerabilityPattern::AccessControlOrAuthByPass,
            VulnerabilityPattern::Reentrancy,
            VulnerabilityPattern::UncheckedLowLevelCallResults,
            VulnerabilityPattern::AccountingInvariantViolation,
            VulnerabilityPattern::SlippageMissingOrInsufficient,
            VulnerabilityPattern::NonStandardERC20Behavior,
            VulnerabilityPattern::StandardViolation,
            VulnerabilityPattern::ExternalCallAfterStateChange,
        ],
    },
];
