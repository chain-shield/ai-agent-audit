use std::{collections::HashMap, sync::OnceLock};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::llm_review::pattern_category::PatternCategory;

#[derive(
    Debug,
    Clone,
    Copy,
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

#[derive(Debug, Clone, Default)]
pub struct ContractCategorySpec {
    pub category: ContractCategory,
    pub description: &'static str,
    pub pattern_category: PatternCategory,
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

pub fn get_contract_spec_from_category(
    cat: &ContractCategory,
) -> Option<&'static ContractCategorySpec> {
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
        pattern_category: PatternCategory::SignatureValidation,
    },
    ContractCategorySpec {
        category: ContractCategory::VaultShareBased,
        description: "ERC4626 vaults or share-based contracts where users deposit assets and receive shares representing proportional ownership. Focus on share price manipulation, inflation attacks, and rounding errors.",
        pattern_category: PatternCategory::VaultShareBased,
    },
    ContractCategorySpec {
        category: ContractCategory::OraclePriceFeed,
        description: "Contracts that provide price data, aggregate oracle feeds, or implement TWAP/Chainlink integrations. Focus on stale data acceptance, manipulation resistance, and price validation.",
        pattern_category: PatternCategory::OraclePriceFeed,
    },
    ContractCategorySpec {
        category: ContractCategory::MathLibrary,
        description: "Pure math libraries providing arithmetic operations, fixed-point math, or custom calculations. Focus on overflow/underflow, division by zero, precision loss, and rounding errors.",
        pattern_category: PatternCategory::MathLibrary,
    },
    ContractCategorySpec {
        category: ContractCategory::TokenTransferLibrary,
        description: "Libraries handling token transfers, approvals, or wrapping/unwrapping logic (e.g., SafeERC20, ETH/WETH helpers). Focus on non-standard token behavior, low-level call failures, and ETH handling.",
        pattern_category: PatternCategory::TokenTransferLibrary,
    },
    ContractCategorySpec {
        category: ContractCategory::GovernanceTimeLock,
        description: "Governance contracts with timelocks, voting mechanisms, or proposal execution logic. Focus on timelock bypass, execution replay, and privilege escalation.",
        pattern_category: PatternCategory::GovernanceTimeLock,
    },
    ContractCategorySpec {
        category: ContractCategory::ProxyUpgradeable,
        description: "Upgradeable proxy contracts (UUPS, Transparent, Beacon) or implementation contracts with initialization logic. Focus on storage collisions, uninitialized state, and upgrade auth bypass.",
        pattern_category: PatternCategory::ProxyUpgradeable,
    },
    ContractCategorySpec {
        category: ContractCategory::StakingRewards,
        description: "Staking contracts that distribute rewards over time based on user deposits and duration. Focus on reward accounting drift, precision loss, and economic manipulation.",
        pattern_category: PatternCategory::StakingRewards,
    },
    ContractCategorySpec {
        category: ContractCategory::BridgeCrossChain,
        description: "Cross-chain bridges, message relayers, or L1/L2 communication contracts. Focus on message spoofing, replay attacks across chains, and finality assumptions.",
        pattern_category: PatternCategory::BridgeCrossChain,
    },
    ContractCategorySpec {
        category: ContractCategory::AMMDex,
        description: "Automated market makers, DEXs, or liquidity pool contracts implementing swap/add/remove liquidity logic. Focus on slippage protection, price manipulation, and reentrancy.",
        pattern_category: PatternCategory::AMMDex,
    },
    ContractCategorySpec {
        category: ContractCategory::LendingBorrowing,
        description: "Lending protocols with collateral management, liquidation logic, and interest accrual. Focus on oracle manipulation, liquidation protection, and accounting invariants.",
        pattern_category: PatternCategory::LendingBorrowing,
    },
    ContractCategorySpec {
        category: ContractCategory::FactoryDeployer,
        description: "Factory contracts that deploy new contract instances using CREATE2 or clones. Focus on authority drift, initialization issues, and deterministic address collisions.",
        pattern_category: PatternCategory::FactoryDeployer,
    },
    ContractCategorySpec {
        category: ContractCategory::ERC20Token,
        description: "Fungible token implementations (ERC20/Permit/custom fees) with mint/burn and role-based controls. Focus on standards compliance, allowance/permit edge cases, and admin footguns.",
        pattern_category: PatternCategory::ERC20Token,
    },
    ContractCategorySpec {
        category: ContractCategory::NFTCollection,
        description: "Non-fungible/multi-token collections (ERC721/ERC1155) with minting, burning, and metadata. Focus on safe transfer handling, operator approvals, mint phase gating, and denial-of-service via hooks.",
        pattern_category: PatternCategory::NFTCollection,
    },
    ContractCategorySpec {
        category: ContractCategory::AirdropDistributor,
        description: "Distribution contracts (Merkle claimers, claim portals) enabling one-time or periodic claims. Focus on double-claim prevention, index monotonicity, and non-standard token interactions.",
        pattern_category: PatternCategory::AirdropDistributor,
    },
    ContractCategorySpec {
        category: ContractCategory::EscrowVesting,
        description: "Vesting, streaming, or escrow contracts releasing funds over time or conditions. Focus on time gates, release schedule correctness, and withdrawal authorizations.",
        pattern_category: PatternCategory::EscrowVesting,
    },
    ContractCategorySpec {
        category: ContractCategory::MarketplaceExchange,
        description: "Order-book or marketplace contracts (list/bid/auction) trading tokens or NFTs. Focus on signature replay, auction edge cases, reentrancy, and accounting fairness.",
        pattern_category: PatternCategory::MarketplaceExchange,
    },
    ContractCategorySpec {
        category: ContractCategory::RandomnessRaffleLottery,
        description: "Randomness-dependent raffles, lotteries, and draws (on-chain PRNG or VRF consumers). Focus on PRNG weaknesses, timestamp manipulation, and commit/reveal protocol correctness.",
        pattern_category: PatternCategory::RandomnessRaffleLottery,
    },
    ContractCategorySpec {
        category: ContractCategory::Unknown,
        description: "Contracts that don't fit into specific categories or have mixed functionality. Use broad pattern coverage for general security analysis.",
        pattern_category:PatternCategory::General
    },
];
