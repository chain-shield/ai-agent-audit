use super::enums::VulnerabilityType;
use crate::llm_review::{enums::EnumData, pattern_category::PatternTier, patterns::ImpactHint};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema, EnumIter, Default, strum_macros::Display,
)]
pub enum InvariantType {
    #[default]
    Arithmetic,
    Balance,
    Permission,
    Temporal,
    Referential,
    StateMachine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema, EnumIter, strum_macros::Display)]
pub enum InvariantStatus {
    Holds,
    PossibleViolation,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InvariantFinding {
    #[schemars(
        description = "Type: Arithmetic, Balance, Permission, Temporal, Referential, StateMachine"
    )]
    pub inv_type: InvariantType,
    pub contract: String, // exact constract name where invariant appears
    pub function: String, // exact function name where invariant is relevant, if not applicable set to 'NA'
    pub predicate: String,
    pub desc: String,
    pub checks: Vec<String>,
    #[schemars(description = "Status: Holds, PossibleViolation")]
    pub status: InvariantStatus,
    pub pre_state: Option<String>,
    pub post_state: Option<String>,
    pub impact: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContractInvariants {
    pub invariants: Vec<InvariantFinding>,
}

#[derive(Debug, Clone, Default)]
pub struct InvariantSpec {
    pub key: InvariantType,
    /// Plain-English description of what the invariant guarantees.
    pub definition: &'static str,
    /// Heuristics your static analyzer can flag to suggest this invariant is at risk.
    pub static_signals: &'static [&'static str],
    /// Tiny concrete examples of how violations typically appear in code/flows.
    pub examples: &'static [&'static str],
    /// Rough priority hint based on typical Code4rena payouts when this invariant breaks.
    pub impact_hint: ImpactHint,
    /// Downstream vuln categories commonly produced when this invariant is violated.
    pub maps_to: &'static [VulnerabilityType],
    pub tier: PatternTier,
}

// 2) Your function can now just return these same statics (still DRY)
impl EnumData for InvariantType {
    type Spec = InvariantSpec;
    fn to_types(&self) -> &'static [VulnerabilityType] {
        match self {
            InvariantType::Arithmetic => INVARIANT_ARITH_TYPES,
            InvariantType::Balance => INVARIANT_BAL_TYPES,
            InvariantType::Permission => INVARIANT_PERM_TYPES,
            InvariantType::Temporal => INVARIANT_TEMP_TYPES,
            InvariantType::Referential => INVARIANT_REF_TYPES,
            InvariantType::StateMachine => INVARIANT_SM_TYPES,
        }
    }

    fn get_spec(&self) -> InvariantSpec {
        INVARIANT_LIBRARY
            .iter()
            .find(|v| v.key == *self)
            .unwrap_or(&InvariantSpec::default())
            .clone()
    }
}

pub static INVARIANT_LIBRARY: &[InvariantSpec] = &[
    InvariantSpec {
        key: InvariantType::Arithmetic,
        definition: "Numeric relationships (ratios, sums, precision) remain consistent; no over/underflow or value skew.",
        static_signals: &[
            "sum of shares != totalSupply",
            "rounding causes drift in pricePerShare",
            "decimals mismatch leads to inflated values",
        ],
        examples: &[
            "sharePrice rounds down, under-distributing rewards",
            "decimals=8 token treated as 18 in price math",
        ],
        impact_hint: ImpactHint::HighMedium,
        maps_to: INVARIANT_ARITH_TYPES,
        tier:PatternTier::Tier2,
    },
    InvariantSpec {
        key: InvariantType::Balance,
        definition: "Token/ETH balances and supply remain monotonic and consistent with accounting invariants.",
        static_signals: &[
            "balanceOf + totalSupply drift apart",
            "unexpected ETH stuck in contract",
            "unchecked transfer/return silently fails",
        ],
        examples: &[
            "fee-on-transfer token reduces balance but accounting ignores it",
            "ERC777 token triggers reentrancy changing balances mid-transfer",
        ],
        impact_hint: ImpactHint::HighMedium,
        maps_to: INVARIANT_BAL_TYPES,
        tier:PatternTier::Tier1,
    },
    InvariantSpec {
        key: InvariantType::Permission,
        definition: "Only authorized roles can perform restricted actions; signatures/permits correctly validated.",
        static_signals: &[
            "function lacks onlyOwner/role guard",
            "permit replayable across chains",
            "tx.origin used instead of msg.sender",
        ],
        examples: &[
            "anyone can pause protocol due to missing modifier",
            "permit nonce not incremented, attacker reuses",
        ],
        impact_hint: ImpactHint::High,
        maps_to: INVARIANT_PERM_TYPES,
        tier:PatternTier::Tier1,
    },
    InvariantSpec {
        key: InvariantType::Temporal,
        definition: "Time-dependent logic respects deadlines, epochs, expiries, and cannot be rewound or bypassed.",
        static_signals: &[
            "no heartbeat check on oracle price",
            "deadline param ignored",
            "timestamp used directly for randomness",
        ],
        examples: &[
            "stale price accepted from oracle after 24h silence",
            "permit accepted long after intended expiry",
        ],
        impact_hint: ImpactHint::HighMedium,
        maps_to: INVARIANT_TEMP_TYPES,
        tier:PatternTier::Tier3,
    },
    InvariantSpec {
        key: InvariantType::Referential,
        definition: "Data structures (arrays, mappings, storage slots) remain in sync and consistent after upgrades or events.",
        static_signals: &[
            "event emitted value != stored mapping value",
            "array index out-of-bounds unchecked",
            "storage layout mismatch in proxy upgrade",
        ],
        examples: &[
            "user deposit logged in event but not added to mapping",
            "new implementation shifts storage slot, corrupting balances",
        ],
        impact_hint: ImpactHint::Medium,
        maps_to: INVARIANT_REF_TYPES,
        tier:PatternTier::Tier3,
    },
    InvariantSpec {
        key: InvariantType::StateMachine,
        definition: "Only valid state transitions occur; maturity/epoch/index monotonicity can’t be bypassed or reset.",
        static_signals: &[
            "no require(isLaunched/hasMatured) on gated funcs",
            "epoch/index can decrement or reset",
            "callbacks mutate phase without checks",
        ],
        examples: &[
            "withdraw before maturity",
            "rewardIndex decreases after migration",
        ],
        impact_hint: ImpactHint::HighMedium,
        maps_to: INVARIANT_SM_TYPES,
        tier:PatternTier::Tier2,
    },
];

// 1) Single-source-of-truth static slices per InvariantType
pub static INVARIANT_ARITH_TYPES: &[VulnerabilityType] = &[
    VulnerabilityType::IntegerMath,
    VulnerabilityType::IntegerOverflow,
    VulnerabilityType::PricePrecision,
    VulnerabilityType::RoundingError,
    VulnerabilityType::ERC20DecimalsMismatch,
    VulnerabilityType::ERC4626SharePrice,
    VulnerabilityType::AccountingInvariantViolation,
];

pub static INVARIANT_BAL_TYPES: &[VulnerabilityType] = &[
    VulnerabilityType::Reentrancy,
    VulnerabilityType::ERC777HookReentrancy,
    VulnerabilityType::UncheckedReturn,
    VulnerabilityType::UncheckedERC20Return,
    VulnerabilityType::FeeOnTransferAssumption,
    VulnerabilityType::UnexpectedEth,
    VulnerabilityType::AccountingInvariantViolation,
    VulnerabilityType::StandardViolation,
    VulnerabilityType::FlashLoanEconomicManipulation,
    VulnerabilityType::SlippageMissingOrInsufficient,
];

pub static INVARIANT_PERM_TYPES: &[VulnerabilityType] = &[
    VulnerabilityType::AccessControl,
    VulnerabilityType::AuthByPass,
    VulnerabilityType::DefaultVisibility,
    VulnerabilityType::TxOrigin,
    VulnerabilityType::PausableEmergencyStop,
    VulnerabilityType::UpgradeabilityInitializerSafety,
    VulnerabilityType::SignatureReplay,
    VulnerabilityType::SignatureMalleability,
    VulnerabilityType::PermitDomainSeparator,
    VulnerabilityType::PermitNonceMisuse,
    VulnerabilityType::PermitDeadlineBypass,
    VulnerabilityType::ReplayAttack,
    VulnerabilityType::DelegatecallLowLevelOps,
    VulnerabilityType::UntrustedDelegateCall,
];

pub static INVARIANT_TEMP_TYPES: &[VulnerabilityType] = &[
    VulnerabilityType::TimestampManipulation,
    VulnerabilityType::TimestampDependentLogic,
    VulnerabilityType::Oracle,          // stale/heartbeat violations
    VulnerabilityType::ReplayAttack,    // expiry windows
    VulnerabilityType::SignatureReplay, // time/nonce drift
    VulnerabilityType::PermitDeadlineBypass,
    VulnerabilityType::FrontrunMev, // timing/sandwich sensitivity
];

pub static INVARIANT_REF_TYPES: &[VulnerabilityType] = &[
    VulnerabilityType::StorageLayout,    // upgrade misalignments
    VulnerabilityType::ArrayLimits,      // OOB / index drift
    VulnerabilityType::EventConsistency, // logs vs state divergence
    VulnerabilityType::DelegatecallLowLevelOps,
    VulnerabilityType::UpgradeabilityInitializerSafety,
];

pub static INVARIANT_SM_TYPES: &[VulnerabilityType] = &[
    VulnerabilityType::Reentrancy, // illegal transitions via reentry
    VulnerabilityType::AccessControl,
    VulnerabilityType::AuthByPass,
    VulnerabilityType::ReplayAttack, // duplicate transitions
    VulnerabilityType::SignatureReplay,
    VulnerabilityType::CrossChainMessageSpoofing,
    VulnerabilityType::TimestampDependentLogic,
    VulnerabilityType::PausableEmergencyStop,
    VulnerabilityType::UpgradeabilityInitializerSafety, // re-init / state reset
    VulnerabilityType::UntrustedDelegateCall,
    VulnerabilityType::AccountingInvariantViolation,
];

impl ContractInvariants {
    /// Parse JSON string containing findings from LLM response
    /// Handles both clean JSON and JSON wrapped in markdown code blocks
    pub fn parse_from_json(json_str: &str) -> Result<ContractInvariants, serde_json::Error> {
        // Clean the input - remove markdown code blocks and extra quotes/escapes
        let cleaned_json = Self::clean_json_string(json_str);

        // Parse the cleaned JSON
        serde_json::from_str(&cleaned_json)
    }

    /// Clean JSON string by removing markdown code blocks, escaped quotes, and extra formatting
    fn clean_json_string(input: &str) -> String {
        let mut cleaned = input.trim();

        // Remove outer quotes if present (from string literals)
        if cleaned.starts_with('"') && cleaned.ends_with('"') {
            cleaned = &cleaned[1..cleaned.len() - 1];
        }

        // Remove markdown code blocks
        if cleaned.starts_with("```json") {
            cleaned = cleaned.strip_prefix("```json").unwrap_or(cleaned);
        }

        if cleaned.ends_with("```") {
            cleaned = cleaned.strip_suffix("```").unwrap_or(cleaned);
        }

        // Replace escaped quotes and newlines
        // cleaned
        //     .replace("\\\"", "\"")
        //     .replace("\\n", "\n")
        //     .replace("\\\n", "\n")
        //     .trim()
        //     .to_string()
        cleaned.to_string()
    }

    pub fn get_all_violations(self) -> Vec<InvariantFinding> {
        self.invariants
            .into_iter()
            .filter(|inv| inv.status == InvariantStatus::PossibleViolation)
            .collect::<Vec<InvariantFinding>>()
    }
}
