use crate::llm_review::{
    agent::agent_enums::EnumData,
    findings::{finding_enums::VulnerabilityType, findings::PrivilegeLevel},
};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Patterns {
    pub patterns: Vec<Pattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Pattern {
    pub issue_type: VulnerabilityPattern,
    pub title: String, // 50 char or less title capturing essence of vulnerability pattern
    pub contract: String, // exact constract name where issue appears
    pub function: String, // exact function name where issue appears, if not applicable set to 'NA'
    pub description: String, // description of issue, include code snippet if relevant
    pub static_signals: Vec<String>, // e.g., "amountOutMin=0", "no onlyOwner"
    pub assets_at_risk: Vec<String>, // e.g., ["treasury", "rewards", "LP"]
    pub privilege: PrivilegeLevel, // permissionless vs role-gated
    pub impact: Option<ImpactHint>,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    JsonSchema,
    EnumIter,
    Serialize,
    Deserialize,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[serde(rename_all = "PascalCase")]
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

    // NEW (10/20/2025)
    BeaconOrFactoryAuthorityDrift,
    TimelockEdgeCase,
    MulticallCrossPathReentrancy,
    TWAPWindowPinningOrLowLiquidity,
    ForcedAssetVsStrictEquality,
}

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
    EnumIter,
    Default,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[serde(rename_all = "PascalCase")]
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
            VulnerabilityPattern::StandardViolation => &[
                VulnerabilityType::StandardViolation,
                VulnerabilityType::Dos, // ERC-4337 factory revert causes DoS
                VulnerabilityType::UncheckedERC20Return, // ERC-20 missing return bool
                VulnerabilityType::ERC4626SharePrice, // ERC-4626 preview function violations
            ],
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
            // Added 10/20/2025
            // ── Authority / Upgrade / Governance ──────────────────────────────────────────
            VulnerabilityPattern::BeaconOrFactoryAuthorityDrift => &[
                VulnerabilityType::BeaconFactoryAuthorityDrift,
                VulnerabilityType::UntrustedDelegateCall,
                VulnerabilityType::AccessControl,
            ],
            VulnerabilityPattern::TimelockEdgeCase => &[
                VulnerabilityType::TimelockEdgeCase,
                VulnerabilityType::AccessControl,
            ],

            // ── Reentrancy / Ordering / Call batching ─────────────────────────────────────
            VulnerabilityPattern::MulticallCrossPathReentrancy => &[
                VulnerabilityType::MulticallCrossPathReentrancy,
                VulnerabilityType::Reentrancy,
                VulnerabilityType::CallOrderingOrCEI,
            ],
            // ── Oracle / Markets / MEV ───────────────────────────────────────────────────
            VulnerabilityPattern::TWAPWindowPinningOrLowLiquidity => &[
                VulnerabilityType::TWAPWindowPinning,
                VulnerabilityType::Oracle,
                VulnerabilityType::FrontrunMev,
            ],

            // ── Accounting / Precision ────────────────────────────────────────────────────
            VulnerabilityPattern::ForcedAssetVsStrictEquality => &[
                VulnerabilityType::ForcedAssetVsStrictEquality,
                VulnerabilityType::AccountingInvariantViolation,
                VulnerabilityType::UnexpectedEth,
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

        // #[derive(Deserialize)]
        // #[serde(field_identifier, rename_all = "snake_case")]
        // enum Field {
        //     Key,
        //     Definition,
        //     StaticSignals,
        //     Examples,
        //     ImpactHint,
        // }

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
        definition: "Signature validation bypass via improper EIP-1271 checks, missing validation guards, conditional logic flaws, or checkpoint/nonce bypass allowing evicted/unauthorized signers.",
        static_signals: &[
            "accepts non-magic return value",
            "low-level call without checking success+result",
            "conditional validation skipped via flag/parameter manipulation",
            "checkpoint/nonce validation bypassed in chained/nested signatures",
            "evicted signer can sign with stale configuration",
            "validation guard missing when flag disabled",
            "signature recovery without idempotency check",
            "nested signature calls ignore outer validation context",
            "conditional branches allow skipping critical validation",
            "missing validation when entering recursive/chained signature paths",
        ],
        examples: &[
            "isValidSignature() return not verified",
            "chained signature with checkpoint flag disabled bypasses all validation",
            "evicted signer signs with old config when checkpoint skipped",
            "flag manipulation allows skipping nonce/checkpoint checks",
            "nested signature recovery ignores parent validation state",
            "conditional logic allows bypassing signature expiry",
            "recursive signature calls skip validation guards",
            "parameter manipulation disables critical validation path",
            "validation only enforced in outer call, not in chained calls",
            "missing idempotency check allows signature reuse in nested contexts",
        ],
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
        definition: "View/read-only functions return manipulable state (AMM reserves, vault share prices, collateral ratios, oracle values) that can be skewed mid-transaction via reentrancy, allowing attackers to read inconsistent/exploitable values during external calls.",
        static_signals: &[
            "uses getReserves/spot price that can change intratx",
            "no TWAP or staleness guard",
            "view function reads balanceOf/totalSupply during external call",
            "vault share price calculated from manipulable totalAssets",
            "collateral ratio read during liquidation callback",
            "oracle price fetched in reentrant context",
            "no reentrancy guard on state-reading functions",
            "external call before view function stabilizes",
        ],
        examples: &[
            "uses Curve spot price to set reward weight during callback",
            "vault.convertToAssets() called during withdraw callback (inflated)",
            "collateral ratio read during ERC777 transfer hook",
            "AMM reserves read during flash loan callback",
            "oracle aggregator reads manipulated pool mid-transaction",
        ],
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
        definition: "Value transfers (trades, liquidations, redemptions, withdrawals) executed without meaningful slippage bounds, minimum amount guarantees, or time bounds, exposing users to price volatility and MEV.",
        static_signals: &[
            "amountOutMin=0 or missing",
            "deadline omitted or far future",
            "minOut computed from same-tx price",
            "payout calculated at execution time without minimum bound",
            "no minAmountOut parameter in liquidation/redemption",
            "price fetched at execution without user-specified floor",
            "collateral payout uses current price without slippage protection",
            "multi-hop/aggregate flow lacks final user-specified minOut for overall result",
        ],
        examples: &[
            "_swapTax passes 0 minOut",
            "router calls with no deadline",
            "liquidate() calculates payout using current price without minPayout parameter",
            "redeem() uses spot price at execution without minCollateral guard",
            "withdraw() converts shares to assets at current rate without minimum",
        ],
        impact_hint: ImpactHint::Medium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::OracleUsingDEXorTWAP,
        definition: "Price/oracle data is manipulable, stale, or unreliable due to: DEX spot reads, too-short TWAP, missing staleness checks, centralized oracle failures, or lack of fallback mechanisms.",
        static_signals: &[
            "single spot read from AMM/DEX",
            "TWAP window < 10–30 min",
            "no min observation / heartbeat",
            "no updatedAt/answeredInRound checks on Chainlink",
            "centralized oracle with no fallback",
            "oracle aggregator missing circuit breaker",
            "price deviation bounds not enforced",
            "feed/answer decimals not normalized to on-chain math base",
            "no multi-oracle consensus mechanism",
        ],
        examples: &[
            "uses Uniswap spot price to value collateral",
            "rewards based on last swap price",
            "Chainlink feed accepted without staleness check",
            "single centralized oracle with no backup",
            "aggregator accepts outlier price without bounds",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::FlashLoanEconomicManipulation,
        definition: "Critical decisions (pricing, collateral valuation, governance thresholds) depend on intra-transaction manipulable state (balances, reserves, share prices) that attackers can skew via flash loans, atomic swaps, sandwich attacks, or MEV.",
        static_signals: &[
            "branches on pool.balanceOf()/getReserves()",
            "no multi-block observation or TWAP",
            "uses totalSupply/totalAssets in same tx as deposit/withdraw",
            "governance threshold based on snapshot-able balance",
            "collateral ratio calculated from spot reserves",
            "share price derived from manipulable pool state",
            "atomic swap → read price → execute logic pattern",
        ],
        examples: &[
            "force graduation based on TVL check (flash loan inflates balance)",
            "liquidation threshold based on spot collateral ratio",
            "governance vote passes via flash-borrowed voting power",
            "sandwich attack: swap → manipulate price → liquidate → swap back",
            "vault share price inflated via atomic deposit before victim",
        ],
        impact_hint: ImpactHint::High,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::FeeOnTransferAssumption,
        definition: "Assumes 1:1 token transfers; ignores fee-on-transfer tokens, rebasing tokens, elastic supply tokens, tokens with transfer hooks, or deflationary/inflationary mechanisms that cause actual received amount to differ from transfer amount.",
        static_signals: &[
            "uses input amount instead of post-transfer delta",
            "no balanceBefore/After check",
            "assumes transferFrom(amount) credits exactly amount",
            "no handling for rebasing tokens (aTokens, stETH, etc.)",
            "ignores elastic supply adjustments",
            "no detection of transfer hooks that modify amounts",
            "accounting based on transfer parameter, not actual balance change",
        ],
        examples: &[
            "deposit(100) credits 100 shares but only 98 tokens received (2% fee)",
            "vault accounting breaks with rebasing token (stETH balance changes)",
            "elastic supply token (AMPL) rebase causes share price desync",
            "fee-on-transfer token used in AMM without balance delta checks",
            "rewards calculated on transfer amount, not actual received amount",
        ],
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
            "token/asset address changes without accounting migration",
            "balance tracking references different token than actual holdings",
            "accounting state not updated when underlying asset is swapped/upgraded",
        ],
        examples: &[
            "burnFrom doesn't reduce totalSupply",
            "unauthorized validator increases rewards share",
            "token upgrade leaves accounting tracking old address while holding new token",
            "collateral swapped but totalCollateral unchanged",
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
        definition: "ERC-20/721/4337/4626 spec deviation enabling theft, stuck funds, DoS, or broken integrations.",
        static_signals: &[
            "wrong totalSupply/balance invariants",
            "incorrect return values/events per standard spec",
            "deterministic deployment reverts on collision instead of returning existing address",
            "CREATE2 deployment not idempotent (reverts on re-deploy)",
            "transfer/transferFrom missing return bool",
            "safeTransferFrom missing receiver callback check",
            "preview/view functions modify state (breaks simulations)",
            "deterministic address allows front-running to DoS deployment",
            "factory/deployer reverts when contract already exists",
            "standard-required function missing or has wrong signature",
        ],
        examples: &[
            "burnFrom without totalSupply decrement",
            "transfer returns false silently",
            "factory reverts on collision instead of returning address",
            "attacker front-runs deterministic deployment causing DoS",
            "transfer() doesn't return bool (breaks DEX integrations)",
            "safeTransferFrom doesn't call onReceived callback",
            "previewDeposit() modifies state (breaks off-chain simulations)",
            "deterministic deployment DoS via front-run pre-creation",
            "simulation/bundler fails due to revert instead of address return",
            "permit() missing deadline check",
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
        static_signals: &[
            "no deadline check",
            "nonces reused or not incremented",
            "ecrecover used without requiring s <= secp256k1n/2",
            "v not validated to 27/28 (or normalized 0/1 → 27/28)",
        ],
        examples: &[
            "accepts expired permit",
            "signature accepted with high-s malleable value",
        ],
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
        definition: "Bypasses maturity/vesting/cooldown/state-gate via edge-case, missing check, or state confusion, enabling unauthorized early/late actions or challenge mechanism bypass.",
        static_signals: &[
            "no require(isLaunched/hasMatured) on gated funcs",
            "vesting/maturity check missing or incomplete",
            "cooldown period bypassable via reentrancy or state manipulation",
            "payment/action can occur before request/state is created",
            "timestamp validation only checks action >= request, not that request existed",
            "DEFAULTED/CANCELLED/REJECTED status still considered 'active'/'open' for some checks",
            "challenge mechanism fails when action precedes request or due to state confusion",
            "accounting subtracts value without verifying recipient/destination matches request",
            "timelock delay bypassable via queue/cancel/re-queue",
            "deadline parameter missing or not enforced",
        ],
        examples: &[
            "withdraw before maturity",
            "claim vested tokens before vesting period ends",
            "bypass cooldown via reentrancy to same function",
            "agent makes payment with predicted reference before creating request",
            "illegalPaymentChallenge reverts because DEFAULTED redemption considered 'open'",
            "freeBalanceChallenge subtracts redemptionValue despite payment to wrong address",
            "vault owner delays switching invalid collateral to reduce liquidation liability",
            "timelock bypass via cancel and immediate re-execution",
            "pause mechanism bypassable via delegatecall to unpaused contract",
            "deposit cap bypass via flash loan and same-block withdrawal",
        ],
        impact_hint: ImpactHint::HighMedium,
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
        definition: "Unbounded iteration in hot paths (loops, recursion, state-dependent iteration) enables gas-based DoS, making functions unusable as state grows or attacker inflates iteration count.",
        static_signals: &[
            "loops over user-controlled arrays/sets",
            "nested loops in external functions",
            "recursive calls without depth limit",
            "iteration count grows with contract state (e.g., all users, all proposals)",
            "no pagination or batching mechanism",
            "loop bound depends on attacker-controlled value",
            "state enumeration via unbounded array traversal",
            "no gas limit checks in loop body",
        ],
        examples: &[
            "claim() iterates over all stakers (DoS as staker count grows)",
            "distribute() loops over all recipients (attacker adds many addresses)",
            "recursive tree traversal without max depth (stack overflow risk)",
            "vote() scans entire proposal history (gas cost increases over time)",
            "liquidateAll() iterates unbounded positions array",
            "nested loop: for each user, for each token (O(n²) gas)",
        ],
        impact_hint: ImpactHint::Medium,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::GriefableCallbacks,
        definition: "Untrusted callbacks/hooks can revert, consume excessive gas, return malicious data (return bomb), or grief to block core protocol flows, causing DoS or fund lockup.",
        static_signals: &[
            "no try/catch around external hook",
            "no bypass on callback failure",
            "callback gas not limited (forwarding all gas)",
            "return data not bounded (return bomb vulnerability)",
            "callback success required for core flow to proceed",
            "no timeout or fallback mechanism",
            "external call in loop without failure isolation",
            "callback to arbitrary user-controlled address",
        ],
        examples: &[
            "onERC721Received revert bricks NFT transfer (no try/catch)",
            "tokensReceived hook consumes all gas → DoS on token transfers",
            "callback returns huge data payload → out-of-gas on copy (return bomb)",
            "distribute() fails if any recipient callback reverts",
            "withdrawal blocked because recipient's receive() reverts",
            "flash loan callback griefs by consuming 63/64 gas",
        ],
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
        definition: "Mismatch between native tokens (ETH, MATIC, AVAX, BNB, etc.) and their wrapped equivalents (WETH, WMATIC, WAVAX, WBNB) leads to lost funds, reverts, or accounting errors due to incorrect handling of msg.value, payable functions, or wrap/unwrap logic.",
        static_signals: &[
            "call{value:...} to non-payable function",
            "assumes WETH unwrap without checking balance",
            "msg.value sent but function expects wrapped token",
            "wrapped token sent but function expects native token",
            "no payable modifier but expects native token",
            "refund logic assumes native but holds wrapped",
            "mixed native/wrapped in same flow without conversion",
        ],
        examples: &[
            "deposit() expects ETH (payable) but user sends WETH → funds stuck",
            "withdraw() sends WETH but user expects ETH → integration breaks",
            "refund() tries to send ETH but contract holds WETH → revert",
            "MATIC/WMATIC confusion on Polygon causes accounting mismatch",
            "router accepts msg.value but also pulls WETH → double payment",
        ],
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
        definition: "Reentrancy via token/NFT transfer hooks (ERC777 tokensReceived/tokensToSend, ERC721 onERC721Received, ERC1155 onERC1155Received, custom callbacks) enabling multiple state updates per transaction or bypassing CEI patterns.",
        static_signals: &[
            "accepts ERC777 without nonReentrant",
            "external transfer triggers hooks before state write",
            "ERC721/ERC1155 safeTransfer without reentrancy guard",
            "custom token callback (e.g., tokensReceived) not protected",
            "transfer to untrusted recipient before state finalized",
            "no callback gas limit or try/catch protection",
            "assumes transfer is atomic (ignores hooks)",
        ],
        examples: &[
            "claim() reentered via ERC777 tokensReceived to double-claim",
            "deposit() reentered via ERC721 onERC721Received to bypass share accounting",
            "withdraw() reentered via ERC1155 callback to drain vault",
            "custom reward token with callback hook exploited for reentrancy",
            "NFT transfer callback reenters mint() to bypass supply cap",
        ],
        impact_hint: ImpactHint::High,
    },
    // C) Economic, Market, Oracle (new)
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::StaleOracleAcceptance,
        definition: "Accepts stale prices/heartbeats or outdated observations from any oracle source (Chainlink, TWAP, custom oracles, aggregators); attacker trades against old data or exploits price lag.",
        static_signals: &[
            "no updatedAt/answeredInRound checks on Chainlink",
            "ignores heartbeat/threshold for max age",
            "TWAP observations not checked for freshness",
            "no timestamp validation on custom oracle data",
            "accepts price older than reasonable threshold (e.g., 1 hour)",
            "no sequencer uptime check on L2 Chainlink feeds",
            "oracle aggregator doesn't validate individual feed freshness",
            "no grace period after oracle update before using price",
            "missing minAnswer/maxAnswer circuit breaker checks",
        ],
        examples: &[
            "Chainlink: values collateral with hours-old price (no updatedAt check)",
            "TWAP: reward weights from outdated observation (no age validation)",
            "L2: uses Chainlink without checking sequencer uptime feed",
            "Custom oracle: accepts price without timestamp validation",
            "Aggregator: median of 5 feeds, 3 are stale but not filtered",
            "Price used immediately after oracle update (no stabilization period)",
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
            "no s-value malleability guard (s <= secp256k1n/2) or v in {27,28}",
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
        definition: "Share-based accounting systems (ERC4626 vaults, staking contracts, LP tokens, yield aggregators) have share/asset conversion errors due to precision loss, rounding bias, order-of-operations, or inflation attacks, causing systematic value extraction or loss.",
        static_signals: &[
            "divide before multiply in convertToShares/assets",
            "rounding bias always favors caller (not protocol)",
            "no virtual shares/assets to prevent inflation attack",
            "first depositor can manipulate share price",
            "totalSupply can be zero during conversions",
            "share price calculation uses manipulable totalAssets",
            "no minimum deposit/share amount enforced",
            "withdrawal rounds down shares, deposit rounds down assets",
        ],
        examples: &[
            "ERC4626: withdraw more assets than shares imply due to rounding",
            "ERC4626: mint shares underpriced by rounding (attacker profits)",
            "ERC4626: inflation attack via first deposit of 1 wei + donation",
            "Staking: share price manipulated via flash deposit before victim",
            "LP token: precision loss in share calculation drains small depositors",
            "Yield vault: totalAssets manipulated to inflate share price",
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
    // Added 10/20/2025
    // ── NEW: Authority / Governance ──────────────────────────────────────────────
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::BeaconOrFactoryAuthorityDrift,
        definition: "Privilege/state control drifts via mutable beacon/factory wiring (impl/beacon address can be swapped or user-controlled), enabling unauthorized logic upgrades or state hijack.",
        static_signals: &[
            "beacon address stored mutable / setBeacon() lacks onlyOwner",
            "factory sets implementation/strategy from user input",
            "delegatecall target pulled from registry without allowlist",
            "upgrade path controlled by different admin than core protocol",
            "no codehash/impl allowlist; no immutability on critical addresses",
            "critical asset/token address mutable by external actor",
            "collateral/reserve token address changeable without migration logic",
        ],
        examples: &[
            "setBeacon(newBeacon) public → attacker points to malicious impl",
            "factory.createProxy(impl=unvetted) then protocol delegatecalls",
            "beacon owner not protocol governance; can swap impl at will",
            "oracle/registry updates token address → accounting tracks stale reference",
            "asset upgrade function swaps underlying token without state migration",
        ],
        impact_hint: ImpactHint::High,
    },
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::TimelockEdgeCase,
        definition: "Queue/Cancel/Execute sequencing allows replay or zero-delay execution due to salt/id reuse, missing idempotency flags, or misconfigurable minDelay.",
        static_signals: &[
            "operationId not consumed / can be scheduled/executed multiple times",
            "same salt/tuple accepted twice (no uniqueness bound)",
            "minDelay updatable by same governance in same flow",
            "timestamp checks use <= now with no guard window",
            "cancel() does not clear queued state thoroughly",
        ],
        examples: &[
            "schedule() and execute() callable in same block (minDelay=0)",
            "attacker re-queues identical op with same salt to replay effects",
            "governance lowers minDelay then immediately executes upgrade",
        ],
        impact_hint: ImpactHint::High,
    },
    // ── NEW: Reentrancy / Call Ordering ──────────────────────────────────────────
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::MulticallCrossPathReentrancy,
        definition: "Multicall/batch lets attacker interleave functions to reenter secondary paths while state is half-updated, bypassing single-call CEI and per-function guards.",
        static_signals: &[
            "public multicall executes arbitrary function list",
            "state flags/locals reused across calls in same tx",
            "nonReentrant applied per function, not across multicall boundary",
            "assumes call ordering; no global reentrancy sentinel",
        ],
        examples: &[
            "deposit() then withdraw() in same multicall to bypass share checks",
            "updateIndex() then claim() twice within batch before index finalizes",
            "flash-deposit → price calc → redeem in one multicall to extract value",
        ],
        impact_hint: ImpactHint::High,
    },
    // ── NEW: Oracle / Market Microstructure ──────────────────────────────────────
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::TWAPWindowPinningOrLowLiquidity,
        definition: "TWAP/DEX-derived oracle can be pinned or skewed due to too-short/long windows, low observation cardinality, or thin liquidity, enabling predictable manipulation.",
        static_signals: &[
            "twapWindow < 10–30 minutes (or unbounded long window)",
            "observationCardinality/min not enforced",
            "uses single low-liquidity pair without liquidity floor",
            "no freshness/age bound on observations",
            "no min trades/volume threshold before trusting price",
        ],
        examples: &[
            "1-minute TWAP used for liquidations; attacker pins with cyc trades",
            "oracle reads Uniswap pool with tiny liquidity; price yanks easily",
            "migration resets observations → stale TWAP accepted",
        ],
        impact_hint: ImpactHint::HighMedium,
    },
    // ── NEW: Accounting / Invariants ─────────────────────────────────────────────
    VulnerabilityPatternSpec {
        key: VulnerabilityPattern::ForcedAssetVsStrictEquality,
        definition: "Relies on strict equality between on-chain balance and internal accounting; forced ETH/tokens (selfdestruct/fee rebate) break equality and brick fee/withdraw flows.",
        static_signals: &[
            "require(address(this).balance == totalFees) or similar strict guard",
            "no mechanism to sweep/skim excess funds",
            "assumes only contract code changes balances",
            "fee accumulator reset depends on exact equality",
        ],
        examples: &[
            "withdrawFees() reverts forever after 1 wei forced via selfdestruct",
            "donation to contract makes balance > accountingVar; functions lock",
            "refund path requires equality; any rebate breaks withdrawals",
        ],
        impact_hint: ImpactHint::High,
    },
];

impl Default for VulnerabilityPattern {
    fn default() -> Self {
        VulnerabilityPattern::AccessControlOrAuthByPass
    }
}

// No longer needed! strum's Display trait provides to_string() for free
// Serialize and Deserialize are now handled by serde derives!
