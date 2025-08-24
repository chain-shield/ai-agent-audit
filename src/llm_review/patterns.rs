use crate::llm_review::{
    enums::{EnumData, EnumString, VulnerabilityType},
    findings::PrivilegeLevel,
};
use schemars::JsonSchema;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Patterns {
    pub patterns: Vec<Pattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Pattern {
    pub issue_type: VulnerabilityPattern,
    pub contract: String,            // exact constract name where issue appears
    pub function: String, // exact function name where issue appears, if not applicable set to 'NA'
    pub description: String, // description of issue, include code snippet if relevant
    pub static_signals: Vec<String>, // e.g., "amountOutMin=0", "no onlyOwner"
    pub assets_at_risk: Vec<String>, // e.g., ["treasury", "rewards", "LP"]
    pub privilege: PrivilegeLevel, // permissionless vs role-gated
    pub impact: Option<ImpactHint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema, EnumIter)]
// Access,Auth &,Governance
pub enum VulnerabilityPattern {
    // auth bypass
    AccessControlOrAuthByPass, // High
    GovernanceDelegationFlaw,
    DoubleExecutionOrReplay,
    PermitOrSignatureReplay,
    EIP1271ByPass,
    ConfigFootgun, // untrusted admin

    // Call Order ,Reentrancy, External calls
    CEIViolation,
    Reentrancy, // High
    ReadOnlyReentrancy,
    ExternalCallAfterStateChange, // Medium

    // Economic , Market, & Oracle
    SlippageMissingOrInsufficient,
    OracleUsingDEXorTWAP,
    FlashLoanEconomicManipulation, // High
    FeeOnTransferAssumption,
    ReserveOrPriceDesync,

    // Accounting & Invariants
    AccountingInvariantViolation,
    UnsafeRecipient,
    PrecisionDriftAccumulation, // Medium
    PricePrecisionOrRoundingError,

    //Token Standard Allowance
    StandardViolation,
    AllowanceRace, // Medium
    PermitMisuse,

    // Upgradeability, Proxies, & Init
    UpgradeAuthBypass,
    InitOrderOrUnintialized,
    StorageCollisionOrSelectorClash,
    SelfdestructOrMetamorphicFootguns,

    //Lifecycle & State Machines
    MaturityorGatingByPass,
    EpochOrIndexMonotonicity,

    // DoS, Gas, and Complexity - all Mediums
    UnboundedLoops,
    GriefableCallbacks,
    StateGrowthOrStorageBloat,

    // Randomness, Time, & Chain Assumptions - all Medium or Low
    TimestampOrBlockManipulation,
    BlockhashOrPRNGWeakness,
    ChainIdorDomainDrift,

    // Cross-Chain & Bridging - only for L2 or briges
    CrossChainMessageSpoofing,
    FinalityOrReplayAcrossDomains,

    // EVM/Assembly & Low-Level
    UncheckedLowLevelCallResults,
    UnsafeAssembyTypeCasts,
    DivideByZeroOrOverFlowInCustomMath,

    // ETH/WETH & Payment Flows
    EthVsWethConfusion,
    PullorPushPaymentbugs,

    // New patterns to add
    NonStandardERC20Behavior, // tokens that return false/no-return/custom decimals; breaks transfers/assumptions
    ERC20DecimalsMismatch,    // amount/price math assumes wrong decimals -> value skew
    ERC777HookReentrancy, // reentrancy via ERC777 hooks (tokensReceived), even with CEI elsewhere
    StaleOracleAcceptance, // accepts stale prices/heartbeats; attacker trades against old data
    SandwichableOracle,   // on-chain spot read manipulable within one tx (pre/post trade skew)
    PermitFrontRun, // permit usable/front-runnable in same block (nonce/deadline handling flaws)
    UnprotectedPauseOrStop, // anyone/weakly-gated pause/unpause/emergency stop
    UntrustedDelegateCall, // delegatecall to untrusted target (plugins/strategies) -> state hijack
    ReplayAcrossForksOrL2s, // message valid on fork/sibling chain replays (bridges/inbox)
    ERC4626SharePriceMismatch, // vault share/asset conversions lose precision or drift over time
    FeeAccountingDrift, // fee math rounding/order-of-ops lets dust siphon/accumulate
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, EnumIter, Default)]
pub enum ImpactHint {
    High,
    HighMedium, // between High and Medium
    Medium,
    MediumLow, // between Medium and Low
    #[default]
    Low,
}

#[derive(Debug, Clone, JsonSchema, Default)]
pub struct VulnerabilityPatternSpec {
    pub key: VulnerabilityPattern,
    pub definition: &'static str,
    pub static_signals: &'static [&'static str],
    pub examples: &'static [&'static str],
    pub impact_hint: ImpactHint,
}

impl EnumData for VulnerabilityPattern {
    type Spec = VulnerabilityPatternSpec;
    fn get_spec(&self) -> VulnerabilityPatternSpec {
        VULNERABILITY_PATTERN_LIBRARY
            .iter()
            .find(|v| v.key == *self)
            .unwrap_or(&VulnerabilityPatternSpec::default())
            .clone()
    }

    fn to_types(&self) -> &'static [VulnerabilityType] {
        match self {
            // Access, Auth & Governance
            VulnerabilityPattern::AccessControlOrAuthByPass => &[
                VulnerabilityType::AccessControl,
                VulnerabilityType::AuthByPass,
            ],
            VulnerabilityPattern::GovernanceDelegationFlaw => &[
                VulnerabilityType::AccessControl,
                VulnerabilityType::AuthByPass,
                VulnerabilityType::DelegatecallLowLevelOps,
            ],
            VulnerabilityPattern::DoubleExecutionOrReplay => &[VulnerabilityType::ReplayAttack],
            VulnerabilityPattern::PermitOrSignatureReplay => &[
                VulnerabilityType::SignatureReplay,
                VulnerabilityType::ReplayAttack,
                VulnerabilityType::SignatureMalleability,
            ],
            VulnerabilityPattern::EIP1271ByPass => &[
                VulnerabilityType::AuthByPass,
                VulnerabilityType::SignatureReplay,
            ],
            VulnerabilityPattern::ConfigFootgun => {
                &[VulnerabilityType::AccessControl, VulnerabilityType::Custom]
            }

            // Call Order, Reentrancy, External calls
            VulnerabilityPattern::CEIViolation => &[VulnerabilityType::Reentrancy],
            VulnerabilityPattern::Reentrancy => &[VulnerabilityType::Reentrancy],
            VulnerabilityPattern::ExternalCallAfterStateChange => &[
                VulnerabilityType::Reentrancy,
                VulnerabilityType::UncheckedReturn,
            ],

            // Economic, Market, Oracle
            VulnerabilityPattern::SlippageMissingOrInsufficient => &[
                VulnerabilityType::SlippageMissingOrInsufficient,
                VulnerabilityType::FrontrunMev,
            ],
            VulnerabilityPattern::OracleUsingDEXorTWAP => {
                &[VulnerabilityType::Oracle, VulnerabilityType::PricePrecision]
            }
            VulnerabilityPattern::FlashLoanEconomicManipulation => &[
                VulnerabilityType::FlashLoanEconomicManipulation,
                VulnerabilityType::Oracle,
            ],
            VulnerabilityPattern::FeeOnTransferAssumption => &[
                VulnerabilityType::FeeOnTransferAssumption,
                VulnerabilityType::UncheckedERC20Return,
            ],
            VulnerabilityPattern::ReserveOrPriceDesync => &[
                VulnerabilityType::AccountingInvariantViolation,
                VulnerabilityType::Oracle,
            ],

            // Accounting & Invariants
            VulnerabilityPattern::PrecisionDriftAccumulation => &[
                VulnerabilityType::RoundingError,
                VulnerabilityType::PricePrecision,
                VulnerabilityType::AccountingInvariantViolation,
            ],

            // Upgradeability, Proxies, Init
            VulnerabilityPattern::UpgradeAuthBypass => &[
                VulnerabilityType::UpgradeabilityInitializerSafety,
                VulnerabilityType::AuthByPass,
            ],
            VulnerabilityPattern::InitOrderOrUnintialized => {
                &[VulnerabilityType::UpgradeabilityInitializerSafety]
            }
            VulnerabilityPattern::StorageCollisionOrSelectorClash => {
                &[VulnerabilityType::StorageLayout]
            }
            VulnerabilityPattern::SelfdestructOrMetamorphicFootguns => &[
                VulnerabilityType::SelfDestruct,
                VulnerabilityType::DelegatecallLowLevelOps,
            ],

            // Lifecycle & State Machines
            VulnerabilityPattern::MaturityorGatingByPass => &[
                VulnerabilityType::AuthByPass,
                VulnerabilityType::TimestampDependentLogic,
            ],
            VulnerabilityPattern::EpochOrIndexMonotonicity => {
                &[VulnerabilityType::AccountingInvariantViolation]
            }

            // DoS, Gas, Complexity
            VulnerabilityPattern::UnboundedLoops => &[
                VulnerabilityType::Dos,
                VulnerabilityType::GasGriefBlockLimit,
            ],
            VulnerabilityPattern::GriefableCallbacks => {
                &[VulnerabilityType::Dos, VulnerabilityType::Reentrancy]
            }
            VulnerabilityPattern::StateGrowthOrStorageBloat => &[
                VulnerabilityType::Dos,
                VulnerabilityType::GasGriefBlockLimit,
            ],

            // Randomness, Time, Chain assumptions
            VulnerabilityPattern::TimestampOrBlockManipulation => &[
                VulnerabilityType::TimestampManipulation,
                VulnerabilityType::TimestampDependentLogic,
            ],
            VulnerabilityPattern::BlockhashOrPRNGWeakness => &[VulnerabilityType::Randomness],
            VulnerabilityPattern::ChainIdorDomainDrift => &[
                VulnerabilityType::SignatureReplay,
                VulnerabilityType::CrossChainMessageSpoofing,
            ],

            // Cross-chain & Bridging
            VulnerabilityPattern::CrossChainMessageSpoofing => {
                &[VulnerabilityType::CrossChainMessageSpoofing]
            }
            VulnerabilityPattern::FinalityOrReplayAcrossDomains => &[
                VulnerabilityType::ReplayAttack,
                VulnerabilityType::CrossChainMessageSpoofing,
            ],

            // EVM/Assembly & Low-Level
            VulnerabilityPattern::UncheckedLowLevelCallResults => {
                &[VulnerabilityType::UncheckedReturn]
            }
            VulnerabilityPattern::UnsafeAssembyTypeCasts => &[
                VulnerabilityType::DelegatecallLowLevelOps,
                VulnerabilityType::StorageLayout,
            ],
            VulnerabilityPattern::DivideByZeroOrOverFlowInCustomMath => &[
                VulnerabilityType::IntegerMath,
                VulnerabilityType::IntegerOverflow,
            ],

            // ETH/WETH & Payment Flows
            VulnerabilityPattern::EthVsWethConfusion => &[VulnerabilityType::UnexpectedEth],
            VulnerabilityPattern::PullorPushPaymentbugs => &[
                VulnerabilityType::UnexpectedEth,
                VulnerabilityType::UncheckedReturn,
            ],

            // Token Standard / Allowance
            VulnerabilityPattern::StandardViolation => &[VulnerabilityType::StandardViolation],
            VulnerabilityPattern::AllowanceRace => &[VulnerabilityType::AllowanceRace],
            VulnerabilityPattern::PermitMisuse => &[
                VulnerabilityType::SignatureReplay,
                VulnerabilityType::SignatureMalleability,
                VulnerabilityType::PermitDomainSeparator,
                VulnerabilityType::PermitNonceMisuse,
                VulnerabilityType::PermitDeadlineBypass,
                VulnerabilityType::AuthByPass,
            ],

            // Economic / Oracle (optional finer granularity)
            VulnerabilityPattern::PricePrecisionOrRoundingError => &[
                VulnerabilityType::PricePrecision,
                VulnerabilityType::RoundingError,
                VulnerabilityType::ERC20DecimalsMismatch, // <- new, if caused by decimals
            ],

            // Reentrancy via token standards (optional specialization)
            VulnerabilityPattern::ReadOnlyReentrancy => &[VulnerabilityType::Reentrancy],
            VulnerabilityPattern::UnsafeRecipient => &[
                VulnerabilityType::UncheckedReturn,
                VulnerabilityType::Reentrancy,
                VulnerabilityType::ERC777HookReentrancy, // <- when applicable
            ],

            // Vault math / accounting (optional specialization)
            VulnerabilityPattern::AccountingInvariantViolation => &[
                VulnerabilityType::AccountingInvariantViolation,
                VulnerabilityType::ERC4626SharePrice, // <- when in vault context
            ],
            // Token Standard / ERC20/777 quirks
            VulnerabilityPattern::NonStandardERC20Behavior => &[
                VulnerabilityType::UncheckedERC20Return,
                VulnerabilityType::FeeOnTransferAssumption,
            ],
            VulnerabilityPattern::ERC20DecimalsMismatch => &[VulnerabilityType::PricePrecision],
            VulnerabilityPattern::ERC777HookReentrancy => &[VulnerabilityType::Reentrancy],

            // Oracle & Market Data
            VulnerabilityPattern::StaleOracleAcceptance => &[VulnerabilityType::Oracle],
            VulnerabilityPattern::SandwichableOracle => {
                &[VulnerabilityType::Oracle, VulnerabilityType::FrontrunMev]
            }

            // Permit / Signatures
            VulnerabilityPattern::PermitFrontRun => &[
                VulnerabilityType::SignatureReplay,
                VulnerabilityType::SignatureMalleability,
                // add PermitDomainSeparator / PermitNonceMisuse / PermitDeadlineBypass if using your finer subtypes
            ],

            // Admin / Lifecycle
            VulnerabilityPattern::UnprotectedPauseOrStop => &[
                VulnerabilityType::PausableEmergencyStop,
                VulnerabilityType::AccessControl,
            ],

            // Low-Level / Delegatecall
            VulnerabilityPattern::UntrustedDelegateCall => &[
                VulnerabilityType::UntrustedDelegateCall,
                VulnerabilityType::DelegatecallLowLevelOps,
            ],

            // Cross-chain & Bridging
            VulnerabilityPattern::ReplayAcrossForksOrL2s => &[
                VulnerabilityType::ReplayAttack,
                VulnerabilityType::CrossChainMessageSpoofing,
            ],

            // Vaults & Accounting
            VulnerabilityPattern::ERC4626SharePriceMismatch => &[
                VulnerabilityType::AccountingInvariantViolation,
                VulnerabilityType::PricePrecision,
            ],
            VulnerabilityPattern::FeeAccountingDrift => &[
                VulnerabilityType::RoundingError,
                VulnerabilityType::AccountingInvariantViolation,
            ],
        }
    }
}

impl Serialize for VulnerabilityPatternSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("VulnerabilityPatternSpec", 5)?;
        state.serialize_field("key", &self.key)?;
        state.serialize_field("definition", self.definition)?;
        state.serialize_field("static_signals", &self.static_signals)?;
        state.serialize_field("examples", &self.examples)?;
        state.serialize_field("impact_hint", &self.impact_hint)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for VulnerabilityPatternSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Key,
            Definition,
            StaticSignals,
            Examples,
            ImpactHint,
        }

        struct VulnerabilityPatternSpecVisitor;

        impl<'de> Visitor<'de> for VulnerabilityPatternSpecVisitor {
            type Value = VulnerabilityPatternSpec;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct VulnerabilityPatternSpec")
            }

            fn visit_map<V>(self, _map: V) -> Result<VulnerabilityPatternSpec, V::Error>
            where
                V: MapAccess<'de>,
            {
                // For deserialization, we'll return an error since we can't create static references
                // This is mainly used for serialization and JSON schema generation
                Err(de::Error::custom(
                    "VulnerabilityPatternSpec deserialization not supported - use static VULNERABILITY_PATTERN_LIBRARY",
                ))
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "key",
            "definition",
            "static_signals",
            "examples",
            "impact_hint",
        ];
        deserializer.deserialize_struct(
            "VulnerabilityPatternSpec",
            FIELDS,
            VulnerabilityPatternSpecVisitor,
        )
    }
}

pub static VULNERABILITY_PATTERN_LIBRARY: &[VulnerabilityPatternSpec] = &[
    // A) Access, Auth, Governance
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::AccessControlOrAuthByPass,
        definition: "Sensitive state-changing function lacks or misconfigures role/ownership checks, enabling unauthorized actions.",
        static_signals: &[
            "no onlyOwner/hasRole on mint/upgrade/validator-set",
            "role check after state change",
            "msg.sender compared to wrong admin address",
        ],
        examples: &[
            "mint() callable by anyone",
            "addValidator() missing access control",
            "setAssetToken() public",
        ],
        impact_hint: ImpactHint::High,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::GovernanceDelegationFlaw,
        definition: "Delegation, validator, or voting power mappings get out of sync or can be bypassed.",
        static_signals: &[
            "delegates/votingPower/validatorSet updated inconsistently",
            "withdraw/undelegate doesn't revoke power",
            "delegate() path bypasses validator registration",
        ],
        examples: &[
            "delegate() grants voting power to non-validator",
            "withdraw() keeps prior votingPower",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::DoubleExecutionOrReplay,
        definition: "Action can be executed more than once due to missing nonce/idempotency guard or replayable message.",
        static_signals: &[
            "no executed[proposalId] flag",
            "no nonce consumed in execute()",
            "offchain message reused without anti-replay",
        ],
        examples: &[
            "earlyExecute() usable twice",
            "re-submit identical proposal to double effects",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::PermitOrSignatureReplay,
        definition: "EIP-712/EIP-2612 signatures can be reused or forged due to nonce/domain errors.",
        static_signals: &[
            "missing per-owner nonce",
            "domain separator not binding chainId",
            "no deadline/expiry enforced",
        ],
        examples: &[
            "permit() accepts reused signature",
            "wrong DOMAIN_SEPARATOR on chain fork",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::EIP1271ByPass,
        definition: "Improper EIP-1271 contract signature validation allows unauthorized approvals/executions.",
        static_signals: &[
            "accepts non-magic return value",
            "low-level call without checking success+result",
        ],
        examples: &["isValidSignature() return not verified"],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::ConfigFootgun,
        definition: "Owner-configurable parameter can brick flows or redirect funds without safeguards/sanity checks.",
        static_signals: &[
            "owner can set arbitrary token/router/treasury",
            "no zero-address/known-allowlist checks",
        ],
        examples: &[
            "setAssetToken() leads to loss",
            "setRouter() to malicious router",
        ],
        impact_hint: ImpactHint::MediumLow,
    },
    // B) Call Order, Reentrancy, External calls
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::CEIViolation,
        definition: "External calls occur before internal state effects, enabling reentrancy or inconsistent state.",
        static_signals: &[
            "call/transfer/safeTransfer before state write",
            "no reentrancy guard on money flows",
        ],
        examples: &["withdraw() sends ETH then updates balance"],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::Reentrancy,
        definition: "Re-enterable external call path allows attacker to perform multiple state updates per tx.",
        static_signals: &[
            "untrusted call before all updates",
            "missing/nonfunctional nonReentrant",
        ],
        examples: &[
            "claim() reentered via ERC777 hooks",
            "NFT receiver hook reenters withdraw()",
        ],
        impact_hint: ImpactHint::High,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::ReadOnlyReentrancy,
        definition: "Observable state (e.g., AMM reserves) read mid-tx is manipulable within same transaction.",
        static_signals: &[
            "uses getReserves/price that can change intratx",
            "no TWAP or staleness guard",
        ],
        examples: &["uses spot price to set reward weight this tx"],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::ExternalCallAfterStateChange,
        definition: "State updates occur before an external call that may revert/grief, leaving partial state.",
        static_signals: &[
            "bookkeeping updated; external call at end",
            "no try/catch or rollback on failure",
        ],
        examples: &["emit+state updated then transfer to user; transfer can revert"],
        impact_hint: ImpactHint::Medium,
    },
    // C) Economic, Market, Oracle
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::SlippageMissingOrInsufficient,
        definition: "Trades executed without meaningful slippage bounds or time bounds.",
        static_signals: &[
            "amountOutMin=0",
            "deadline omitted or far future",
            "minOut computed from same-tx price",
        ],
        examples: &["_swapTax passes 0 minOut", "router calls with no deadline"],
        impact_hint: ImpactHint::Medium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::OracleUsingDEXorTWAP,
        definition: "Price/oracle derived from manipulable spot or too-short TWAP without staleness checks.",
        static_signals: &[
            "single spot read from AMM",
            "TWAP window < 10–30 min",
            "no min observation / heartbeat",
        ],
        examples: &[
            "uses spot to value collateral",
            "rewards based on last swap price",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::FlashLoanEconomicManipulation,
        definition: "Critical decisions depend on balances/reserves that an attacker can flash-loan skew within a tx.",
        static_signals: &[
            "branches on pool.balanceOf()/getReserves()",
            "no multi-block observation",
        ],
        examples: &["force graduation based on TVL check"],
        impact_hint: ImpactHint::High,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::FeeOnTransferAssumption,
        definition: "Assumes 1:1 transfers; ignores fee-on-transfer/rebase deltas.",
        static_signals: &[
            "uses input amount instead of post-transfer delta",
            "no balanceBefore/After check",
        ],
        examples: &["deposit amount > actually received by vault"],
        impact_hint: ImpactHint::Medium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::PricePrecisionOrRoundingError,
        definition: "Incorrect scaling/order of ops introduces exploitable rounding bias.",
        static_signals: &[
            "divide before multiply",
            "mix 6/8/18 decimals without normalization",
        ],
        examples: &[
            "lpSupply miscalc on division order",
            "priceALast precision loss",
        ],
        impact_hint: ImpactHint::MediumLow,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::ReserveOrPriceDesync,
        definition: "Assumes reserve/price invariants that no longer hold (taxed tokens, unsynced AMM).",
        static_signals: &[
            "no sync() after taxed transfers",
            "assumes invariant without verifying",
        ],
        examples: &["router math ignores fee-on-transfer in pool"],
        impact_hint: ImpactHint::HighMedium,
    },
    // D) Accounting & Invariants
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::AccountingInvariantViolation,
        definition: "Conservation/binding/monotonic invariants (supply, rewards, indexes) break.",
        static_signals: &[
            "totalSupply != sum(balances)",
            "emitted != claimed + unclaimed",
            "index decreases",
        ],
        examples: &[
            "burnFrom doesn't reduce totalSupply",
            "unauthorized validator increases rewards share",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::UnsafeRecipient,
        definition: "Transfers to zero or non-receivable addresses cause value loss or stuck funds.",
        static_signals: &[
            "no zero-address guard",
            "no onERC721Received check where required",
        ],
        examples: &["missing prevId→transfer to address(0)"],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::PrecisionDriftAccumulation,
        definition: "Systematic rounding accumulates value to attacker over time (penny-shaving).",
        static_signals: &[
            "consistent floor toward sender/receiver",
            "looped rounding in distribution",
        ],
        examples: &["dust rounding favors caller each claim"],
        impact_hint: ImpactHint::Medium,
    },
    // E) Token Standard & Allowances
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::StandardViolation,
        definition: "ERC20/721/4626/Votes spec deviation enabling theft, stuck funds, or broken integrations.",
        static_signals: &[
            "wrong totalSupply/balance invariants",
            "incorrect return values/events",
        ],
        examples: &[
            "burnFrom without totalSupply decrement",
            "transfer returns false silently",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::AllowanceRace,
        definition: "Approve race (front-run) allows spender to drain before allowance change.",
        static_signals: &["changes allowance from X to Y without zeroing"],
        examples: &["UI/protocol sets new allowance directly"],
        impact_hint: ImpactHint::Medium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::PermitMisuse,
        definition: "Permit/Permit2 misused (nonce/expiry/chain separation bugs).",
        static_signals: &["no deadline check", "nonces reused or not incremented"],
        examples: &["accepts expired permit"],
        impact_hint: ImpactHint::HighMedium,
    },
    // F) Upgradeability, Proxies, Init
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::UpgradeAuthBypass,
        definition: "Upgrade functions callable by non-admin or wrong admin context.",
        static_signals: &["public upgradeTo", "no onlyProxy/admin guard"],
        examples: &["UUPS upgrade callable by anyone"],
        impact_hint: ImpactHint::High,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::InitOrderOrUnintialized,
        definition: "Initializer can be (re)called or critical storage left uninitialized.",
        static_signals: &[
            "missing initializer/reinitializer",
            "_disableInitializers() never called",
        ],
        examples: &["initialize() callable after deployment by anyone"],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::StorageCollisionOrSelectorClash,
        definition: "Overlapping storage slots/selectors across facets/impls corrupt state or hijack calls.",
        static_signals: &[
            "manual assembly slots without namespace",
            "duplicate function selectors",
        ],
        examples: &["diamond storage overwritten after upgrade"],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::SelfdestructOrMetamorphicFootguns,
        definition: "Contracts can be destroyed or code changed via metamorphic patterns without safeguards.",
        static_signals: &[
            "selfdestruct present",
            "CREATE2 redeploy without registry/lock",
        ],
        examples: &["implementation selfdestructs; proxy bricked"],
        impact_hint: ImpactHint::High,
    },
    // G) Lifecycle & State Machines
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::MaturityorGatingByPass,
        definition: "Lifecycle flags (launch/maturity) ignored or bypassable, enabling early/late actions.",
        static_signals: &["no require(isLaunched/hasMatured) on gated funcs"],
        examples: &["withdraw before maturity"],
        impact_hint: ImpactHint::Medium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::EpochOrIndexMonotonicity,
        definition: "Cumulative indexes/epochs can decrease/reset breaking accrual math.",
        static_signals: &["index set from smaller value", "epoch decrement path"],
        examples: &["rewardIndex drops after migration"],
        impact_hint: ImpactHint::Medium,
    },
    // H) DoS, Gas, Complexity
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::UnboundedLoops,
        definition: "Unbounded iteration in hot path enables gas-based DoS.",
        static_signals: &[
            "loops over user-controlled arrays/sets",
            "nested loops in external functions",
        ],
        examples: &["claim() iterates over all stakers"],
        impact_hint: ImpactHint::Medium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::GriefableCallbacks,
        definition: "Untrusted callback can revert/grief and block core flow.",
        static_signals: &[
            "no try/catch around external hook",
            "no bypass on callback failure",
        ],
        examples: &["onERC721Received revert bricks transfer"],
        impact_hint: ImpactHint::Medium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::StateGrowthOrStorageBloat,
        definition: "Ever-growing state increases gas until functions become unusable.",
        static_signals: &[
            "append-only arrays with no pruning",
            "mapping enumerations via arrays",
        ],
        examples: &["proposal history scanned every vote"],
        impact_hint: ImpactHint::Medium,
    },
    // I) Randomness, Time, Chain
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::TimestampOrBlockManipulation,
        definition: "Relies on timestamp/number in ways miners/validators/MEV can influence.",
        static_signals: &[
            "tight <= comparisons to now",
            "timestamp used as RNG/source of truth",
        ],
        examples: &["lottery uses block.timestamp % N"],
        impact_hint: ImpactHint::MediumLow,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::BlockhashOrPRNGWeakness,
        definition: "Predictable or stale randomness via blockhash/poor PRNG.",
        static_signals: &["blockhash used beyond 256 blocks", "no commit-reveal"],
        examples: &["game picks winner via blockhash"],
        impact_hint: ImpactHint::Medium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::ChainIdorDomainDrift,
        definition: "Assumes static chain/domain; signatures or configs break on fork/migration.",
        static_signals: &[
            "cached DOMAIN_SEPARATOR not recomputed",
            "chainid not bound in signatures",
        ],
        examples: &["permit valid across chains after fork"],
        impact_hint: ImpactHint::Medium,
    },
    // J) Cross-Chain & Bridging
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::CrossChainMessageSpoofing,
        definition: "Trusts arbitrary caller for cross-domain messages rather than canonical bridge/messenger.",
        static_signals: &[
            "receiveMessage trusts msg.sender",
            "no xDomain origin verification",
        ],
        examples: &["L2 handler callable by any EOA"],
        impact_hint: ImpactHint::High,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::FinalityOrReplayAcrossDomains,
        definition: "Messages/receipts can be replayed across domains or before finality.",
        static_signals: &["no per-domain nonce", "no finality delay verification"],
        examples: &["same proof used twice on L2"],
        impact_hint: ImpactHint::HighMedium,
    },
    // K) EVM/Assembly & Low-Level
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::UncheckedLowLevelCallResults,
        definition: "Ignores success flag or return data from low-level calls.",
        static_signals: &[
            "(ok,) = target.call(...); but ok unused",
            "no revert bubble",
        ],
        examples: &["token call fails silently; accounting continues"],
        impact_hint: ImpactHint::Medium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::UnsafeAssembyTypeCasts,
        definition: "Assembly/casts cause truncation, sign, or aliasing bugs.",
        static_signals: &[
            "downcasts without range checks",
            "assembly writes to unchecked slots",
        ],
        examples: &["uint256→uint128 truncation in balances"],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::DivideByZeroOrOverFlowInCustomMath,
        definition: "Hand-rolled math lacks guards leading to div-by-zero/overflow/underflow.",
        static_signals: &[
            "division by user-controlled value",
            "exp/sqrt/log without bounds",
        ],
        examples: &["openTrading division by zero → dust stuck"],
        impact_hint: ImpactHint::HighMedium,
    },
    // L) ETH/WETH & Payment
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::EthVsWethConfusion,
        definition: "Mismatch between ETH and WETH handling leads to lost funds or reverts.",
        static_signals: &[
            "call{value:...} to non-payable",
            "assumes WETH unwrap without checking",
        ],
        examples: &["deposit expects ETH but receives WETH"],
        impact_hint: ImpactHint::Medium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::PullorPushPaymentbugs,
        definition: "Push payments or refunds to attacker-controlled address enable grief or misdirection.",
        static_signals: &[
            "refund address sourced from user input",
            "no pull-based withdrawal alternative",
        ],
        examples: &["loop of transfers to untrusted recipients"],
        impact_hint: ImpactHint::Medium,
    },
    // E) Token Standard & Allowances (new)
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::NonStandardERC20Behavior,
        definition: "ERC20 implementations that omit return values, return false, or have custom behaviors that break transfer assumptions.",
        static_signals: &[
            "low-level token.call(...) return value ignored",
            "assumes transfer/transferFrom revert on failure",
            "no balanceBefore/balanceAfter delta checks",
        ],
        examples: &[
            "vault credits full deposit even when token takes fee",
            "router assumes success on false-returning token",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::ERC20DecimalsMismatch,
        definition: "Mismatched decimals in amount/price math cause systematic value skew or underflows/overflows.",
        static_signals: &[
            "mixes token amounts with 18-decimal math unscaled",
            "uses oracle price with different base decimals",
        ],
        examples: &[
            "collateral value overestimated due to 6↔18 mismatch",
            "mint shares with wrong scaling factor",
        ],
        impact_hint: ImpactHint::Medium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::ERC777HookReentrancy,
        definition: "Reentrancy via ERC777 tokensReceived/ tokensToSend hooks enabling multiple state updates per tx.",
        static_signals: &[
            "accepts ERC777 without nonReentrant",
            "external transfer triggers hooks before state write",
        ],
        examples: &[
            "claim() reentered via tokensReceived to double-claim",
            "deposit() reentered to bypass share accounting",
        ],
        impact_hint: ImpactHint::High,
    },
    // C) Economic, Market, Oracle (new)
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::StaleOracleAcceptance,
        definition: "Accepts stale prices/heartbeats or outdated observations; attacker trades against old data.",
        static_signals: &[
            "no updatedAt/answeredInRound checks",
            "ignores heartbeat/threshold for max age",
        ],
        examples: &[
            "values collateral with hours-old price",
            "reward weights from outdated observation",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::SandwichableOracle,
        definition: "On-chain spot reads that can be skewed within the same transaction (pre/post trade).",
        static_signals: &[
            "uses getReserves/spot before executing swap",
            "no twap/min observation window",
        ],
        examples: &[
            "price read then attacker trades to skew quote",
            "mint amount based on manipulable spot",
        ],
        impact_hint: ImpactHint::High,
    },
    // Permit / Signatures (new)
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::PermitFrontRun,
        definition: "Permit usable or front-runnable in same block due to nonce/deadline handling flaws.",
        static_signals: &[
            "nonces not incremented on failure",
            "accepts expired or zero-deadline permits",
        ],
        examples: &[
            "attacker front-runs victim permit then drains",
            "deadline check missing or <= now without slack",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    // A) Access, Auth, Governance (new)
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::UnprotectedPauseOrStop,
        definition: "Pause/unpause/emergency stop functions callable by anyone or weakly gated.",
        static_signals: &[
            "pause() lacks onlyOwner/role",
            "guardian role set to zero or public",
        ],
        examples: &[
            "anyone can pause withdrawals",
            "malicious actor permanently pauses core flow",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    // K) EVM/Assembly & Low-Level (new)
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::UntrustedDelegateCall,
        definition: "Delegatecall to untrusted targets (plugins/strategies) enabling state hijack or storage corruption.",
        static_signals: &[
            "delegatecall to user-supplied address",
            "no allowlist/immutable codehash checks",
        ],
        examples: &[
            "strategy set by EOA then delegatecalled",
            "module registry missing auth on registration",
        ],
        impact_hint: ImpactHint::High,
    },
    // J) Cross-Chain & Bridging (new)
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::ReplayAcrossForksOrL2s,
        definition: "Messages/signatures valid across forks or sibling L2s can be replayed without domain separation.",
        static_signals: &["chainid/domain not bound in message", "no per-domain nonce"],
        examples: &[
            "L2 inbox accepts proof from sibling chain",
            "permit valid on fork post-chain-split",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    // D) Accounting & Invariants (new)
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::ERC4626SharePriceMismatch,
        definition: "Vault share/asset conversions drift due to precision/order-of-ops; share price mis-accounted.",
        static_signals: &[
            "divide before multiply in convertToShares/assets",
            "rounding bias always favors caller",
        ],
        examples: &[
            "withdraw more assets than shares imply",
            "mint shares underpriced by rounding",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::FeeAccountingDrift,
        definition: "Fee math (order/rounding) leaks value (penny-shaving) or accumulates dust to attacker.",
        static_signals: &[
            "fee taken before scaling normalization",
            "flooring in looped reward distribution",
        ],
        examples: &[
            "caller skims dust each claim via rounding",
            "protocol fees under/over-charged on swaps",
        ],
        impact_hint: ImpactHint::Medium,
    },
];

impl Default for VulnerabilityPattern {
    fn default() -> Self {
        VulnerabilityPattern::AccessControlOrAuthByPass
    }
}

impl EnumString for ImpactHint {
    fn as_str(&self) -> &'static str {
        match self {
            ImpactHint::High => "High",
            ImpactHint::HighMedium => "High/Medium",
            ImpactHint::Medium => "Medium",
            ImpactHint::MediumLow => "Medium/Low",
            ImpactHint::Low => "Low",
        }
    }
}

impl EnumString for VulnerabilityPattern {
    fn as_str(&self) -> &'static str {
        match self {
            VulnerabilityPattern::AccessControlOrAuthByPass => "AccessControlOrAuthByPass",
            VulnerabilityPattern::GovernanceDelegationFlaw => "GovernanceDelegationFlaw",
            VulnerabilityPattern::DoubleExecutionOrReplay => "DoubleExecutionOrReplay",
            VulnerabilityPattern::PermitOrSignatureReplay => "PermitOrSignatureReplay",
            VulnerabilityPattern::EIP1271ByPass => "EIP1271ByPass",
            VulnerabilityPattern::ConfigFootgun => "ConfigFootgun",
            VulnerabilityPattern::CEIViolation => "CEIViolation",
            VulnerabilityPattern::Reentrancy => "Reentrancy",
            VulnerabilityPattern::ReadOnlyReentrancy => "ReadOnlyReentrancy",
            VulnerabilityPattern::ExternalCallAfterStateChange => "ExternalCallAfterStateChange",
            VulnerabilityPattern::SlippageMissingOrInsufficient => "SlippageMissingOrInsufficient",
            VulnerabilityPattern::OracleUsingDEXorTWAP => "OracleUsingDEXorTWAP",
            VulnerabilityPattern::FlashLoanEconomicManipulation => "FlashLoanEconomicManipulation",
            VulnerabilityPattern::FeeOnTransferAssumption => "FeeOnTransferAssumption",
            VulnerabilityPattern::PricePrecisionOrRoundingError => "PricePrecisionOrRoundingError",
            VulnerabilityPattern::ReserveOrPriceDesync => "ReserveOrPriceDesync",
            VulnerabilityPattern::AccountingInvariantViolation => "AccountingInvariantViolation",
            VulnerabilityPattern::UnsafeRecipient => "UnsafeRecipient",
            VulnerabilityPattern::PrecisionDriftAccumulation => "PrecisionDriftAccumulation",
            VulnerabilityPattern::StandardViolation => "StandardViolation",
            VulnerabilityPattern::AllowanceRace => "AllowanceRace",
            VulnerabilityPattern::PermitMisuse => "PermitMisuse",
            VulnerabilityPattern::UpgradeAuthBypass => "UpgradeAuthBypass",
            VulnerabilityPattern::InitOrderOrUnintialized => "InitOrderOrUnintialized",
            VulnerabilityPattern::StorageCollisionOrSelectorClash => {
                "StorageCollisionOrSelectorClash"
            }
            VulnerabilityPattern::SelfdestructOrMetamorphicFootguns => {
                "SelfdestructOrMetamorphicFootguns"
            }
            VulnerabilityPattern::MaturityorGatingByPass => "MaturityorGatingByPass",
            VulnerabilityPattern::EpochOrIndexMonotonicity => "EpochOrIndexMonotonicity",
            VulnerabilityPattern::UnboundedLoops => "UnboundedLoops",
            VulnerabilityPattern::GriefableCallbacks => "GriefableCallbacks",
            VulnerabilityPattern::StateGrowthOrStorageBloat => "StateGrowthOrStorageBloat",
            VulnerabilityPattern::TimestampOrBlockManipulation => "TimestampOrBlockManipulation",
            VulnerabilityPattern::BlockhashOrPRNGWeakness => "BlockhashOrPRNGWeakness",
            VulnerabilityPattern::ChainIdorDomainDrift => "ChainIdorDomainDrift",
            VulnerabilityPattern::CrossChainMessageSpoofing => "CrossChainMessageSpoofing",
            VulnerabilityPattern::FinalityOrReplayAcrossDomains => "FinalityOrReplayAcrossDomains",
            VulnerabilityPattern::UncheckedLowLevelCallResults => "UncheckedLowLevelCallResults",
            VulnerabilityPattern::UnsafeAssembyTypeCasts => "UnsafeAssembyTypeCasts",
            VulnerabilityPattern::DivideByZeroOrOverFlowInCustomMath => {
                "DivideByZeroOrOverFlowInCustomMath"
            }
            VulnerabilityPattern::EthVsWethConfusion => "EthVsWethConfusion",
            VulnerabilityPattern::PullorPushPaymentbugs => "PullorPushPaymentbugs",
            VulnerabilityPattern::NonStandardERC20Behavior => "NonStandardERC20Behavior",
            VulnerabilityPattern::ERC20DecimalsMismatch => "ERC20DecimalsMismatch",
            VulnerabilityPattern::ERC777HookReentrancy => "ERC777HookReentrancy",
            VulnerabilityPattern::StaleOracleAcceptance => "StaleOracleAcceptance",
            VulnerabilityPattern::SandwichableOracle => "SandwichableOracle",
            VulnerabilityPattern::PermitFrontRun => "PermitFrontRun",
            VulnerabilityPattern::UnprotectedPauseOrStop => "UnprotectedPauseOrStop",
            VulnerabilityPattern::UntrustedDelegateCall => "UntrustedDelegateCall",
            VulnerabilityPattern::ReplayAcrossForksOrL2s => "ReplayAcrossForksOrL2s",
            VulnerabilityPattern::ERC4626SharePriceMismatch => "ERC4626SharePriceMismatch",
            VulnerabilityPattern::FeeAccountingDrift => "FeeAccountingDrift",
        }
    }
}

impl Serialize for VulnerabilityPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for VulnerabilityPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "accesscontrolorauthbypass" => Ok(VulnerabilityPattern::AccessControlOrAuthByPass),
            "governancedelegationflaw" => Ok(VulnerabilityPattern::GovernanceDelegationFlaw),
            "doubleexecutionorreplay" => Ok(VulnerabilityPattern::DoubleExecutionOrReplay),
            "permitorsignaturereplay" => Ok(VulnerabilityPattern::PermitOrSignatureReplay),
            "eip1271bypass" => Ok(VulnerabilityPattern::EIP1271ByPass),
            "configfootgun" => Ok(VulnerabilityPattern::ConfigFootgun),
            "ceiviolation" => Ok(VulnerabilityPattern::CEIViolation),
            "reentrancy" => Ok(VulnerabilityPattern::Reentrancy),
            "readonlyreentrancy" => Ok(VulnerabilityPattern::ReadOnlyReentrancy),
            "externalcallafterstatechange" => {
                Ok(VulnerabilityPattern::ExternalCallAfterStateChange)
            }
            "slippagemissingorinsufficient" => {
                Ok(VulnerabilityPattern::SlippageMissingOrInsufficient)
            }
            "oracleusingdexortwap" => Ok(VulnerabilityPattern::OracleUsingDEXorTWAP),
            "flashloaneconomicmanipulation" => {
                Ok(VulnerabilityPattern::FlashLoanEconomicManipulation)
            }
            "feeontransferassumption" => Ok(VulnerabilityPattern::FeeOnTransferAssumption),
            "priceprecisionorroundingerror" => {
                Ok(VulnerabilityPattern::PricePrecisionOrRoundingError)
            }
            "reserveorpricedesync" => Ok(VulnerabilityPattern::ReserveOrPriceDesync),
            "accountinginvariantviolation" => {
                Ok(VulnerabilityPattern::AccountingInvariantViolation)
            }
            "unsaferecipient" => Ok(VulnerabilityPattern::UnsafeRecipient),
            "precisiondriftaccumulation" => Ok(VulnerabilityPattern::PrecisionDriftAccumulation),
            "standardviolation" => Ok(VulnerabilityPattern::StandardViolation),
            "allowancerace" => Ok(VulnerabilityPattern::AllowanceRace),
            "permitmisuse" => Ok(VulnerabilityPattern::PermitMisuse),
            "unboundedloops" => Ok(VulnerabilityPattern::UnboundedLoops),

            "upgradeauthbypass" => Ok(VulnerabilityPattern::UpgradeAuthBypass),
            "initorderorunintialized" => Ok(VulnerabilityPattern::InitOrderOrUnintialized),
            "storagecollisionorselectorclash" => {
                Ok(VulnerabilityPattern::StorageCollisionOrSelectorClash)
            }
            "selfdestructormetamorphicfootguns" => {
                Ok(VulnerabilityPattern::SelfdestructOrMetamorphicFootguns)
            }
            "maturityorgatingbypass" => Ok(VulnerabilityPattern::MaturityorGatingByPass),
            "epochorindexmonotonicity" => Ok(VulnerabilityPattern::EpochOrIndexMonotonicity),
            "unboundedloopsorgasdos" => Ok(VulnerabilityPattern::UnboundedLoops),
            "griefablecallbacks" => Ok(VulnerabilityPattern::GriefableCallbacks),
            "stategrowthorstoragebloat" => Ok(VulnerabilityPattern::StateGrowthOrStorageBloat),
            "timestamporblockmanipulation" => {
                Ok(VulnerabilityPattern::TimestampOrBlockManipulation)
            }
            "blockhashorprngweakness" => Ok(VulnerabilityPattern::BlockhashOrPRNGWeakness),
            "chainidordomaindrift" => Ok(VulnerabilityPattern::ChainIdorDomainDrift),
            "crosschainmessagespoofing" => Ok(VulnerabilityPattern::CrossChainMessageSpoofing),
            "finalityorreplayacrossdomains" => {
                Ok(VulnerabilityPattern::FinalityOrReplayAcrossDomains)
            }
            "nonstandarderc20behavior" => Ok(VulnerabilityPattern::NonStandardERC20Behavior),
            "erc20decimalsmismatch" => Ok(VulnerabilityPattern::ERC20DecimalsMismatch),
            "erc777hookreentrancy" => Ok(VulnerabilityPattern::ERC777HookReentrancy),
            "staleoracleacceptance" => Ok(VulnerabilityPattern::StaleOracleAcceptance),
            "sandwichableoracle" => Ok(VulnerabilityPattern::SandwichableOracle),
            "permitfrontrun" => Ok(VulnerabilityPattern::PermitFrontRun),
            "unprotectedpauseorstop" => Ok(VulnerabilityPattern::UnprotectedPauseOrStop),
            "untrusteddelegatecall" => Ok(VulnerabilityPattern::UntrustedDelegateCall),
            "replayacrossforksorl2s" => Ok(VulnerabilityPattern::ReplayAcrossForksOrL2s),
            "erc4626sharepricemismatch" => Ok(VulnerabilityPattern::ERC4626SharePriceMismatch),
            "feeaccountingdrift" => Ok(VulnerabilityPattern::FeeAccountingDrift),

            "uncheckedlowlevelcallresults" => {
                Ok(VulnerabilityPattern::UncheckedLowLevelCallResults)
            }
            "unsafeassembytypecasts" => Ok(VulnerabilityPattern::UnsafeAssembyTypeCasts),
            "dividebyzerooroverflowincustommath" => {
                Ok(VulnerabilityPattern::DivideByZeroOrOverFlowInCustomMath)
            }
            "ethvswethconfusion" => Ok(VulnerabilityPattern::EthVsWethConfusion),
            "pullorpushpaymentbugs" => Ok(VulnerabilityPattern::PullorPushPaymentbugs),
            _ => Err(de::Error::unknown_variant(
                &s,
                &[
                    "accesscontrolorauthbypass",
                    "governancedelegationflaw",
                    "doubleexecutionorreplay",
                    "permitorsignaturereplay",
                    "eip1271bypass",
                    "configfootgun",
                    "ceiviolation",
                    "reentrancy",
                    "readonlyreentrancy",
                    "externalcallafterstatechange",
                    "slippagemissingorinsufficient",
                    "oracleusingdexortwap",
                    "flashloaneconomicmanipulation",
                    "feeontransferassumption",
                    "priceprecisionorroundingerror",
                    "reserveorpricedesync",
                    "accountinginvariantviolation",
                    "unsaferecipient",
                    "precisiondriftaccumulation",
                    "standardviolation",
                    "allowancerace",
                    "permitmisuse",
                    "upgradeauthbypass",
                    "initorderorunintialized",
                    "storagecollisionorselectorclash",
                    "selfdestructormetamorphicfootguns",
                    "maturityorgatingbypass",
                    "epochorindexmonotonicity",
                    "unboundedloopsorgasdos",
                    "griefablecallbacks",
                    "stategrowthorstoragebloat",
                    "timestamporblockmanipulation",
                    "blockhashorprngweakness",
                    "chainidordomaindrift",
                    "crosschainmessagespoofing",
                    "finalityorreplayacrossdomains",
                    "uncheckedlowlevelcallresults",
                    "unsafeassembytypecasts",
                    "dividebyzerooroverflowincustommath",
                    "ethvswethconfusion",
                    "pullorpushpaymentbugs",
                ],
            )),
        }
    }
}
