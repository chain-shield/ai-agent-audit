use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

/// ------------------------------------------------------------------
/// 1.  Strict-typed severity enum
/// ------------------------------------------------------------------
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
    Deserialize, // ✅ Use serde's derive - LLMs return exact PascalCase
    strum_macros::EnumString,
    strum_macros::Display,
)]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "PascalCase")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
    Invalid,
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Info
    }
}

impl Severity {
    pub fn as_initial(&self) -> &'static str {
        match self {
            Severity::Critical => "C",
            Severity::High => "H",
            Severity::Medium => "M",
            Severity::Low => "L",
            Severity::Info => "I",
            Severity::Invalid => "X",
        }
    }
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
    Deserialize, // ✅ Use serde's derive - LLMs return exact PascalCase
    strum_macros::EnumString,
    strum_macros::Display,
)]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "PascalCase")]
pub enum VulnerabilityType {
    AccessControl,
    ArrayLimits,
    ConfidentialData,
    DefaultVisibility,
    Dos,
    Inheritance,
    IntegerMath,
    Oracle,
    Pragma,
    Randomness,
    Reentrancy,
    ReplayAttack,
    SelfDestruct,
    ShortAddress,
    StorageLayout,
    TxOrigin,
    UncheckedReturn,
    UnexpectedEth,
    ZeroCode,
    FrontrunMev,
    UpgradeabilityInitializerSafety,
    PausableEmergencyStop,
    TimestampDependentLogic,
    FlashLoanEconomicManipulation,
    DelegatecallLowLevelOps,
    SignatureMalleability,
    EventConsistency,
    GasGriefBlockLimit,
    IntegerOverflow,
    /* new aditions */
    PricePrecision,
    RoundingError,
    FeeOnTransferAssumption,
    UncheckedERC20Return,
    SignatureReplay,
    AuthByPass,
    UntrustedDelegateCall,
    TimestampManipulation,
    CrossChainMessageSpoofing,
    AccountingInvariantViolation,
    SlippageMissingOrInsufficient,
    StandardViolation,     // ERC-20/721/1155 spec violations
    AllowanceRace,         // approve/transferFrom race (ERC-20)
    PermitDomainSeparator, // EIP-2612/EIP-712 domain mismatch
    PermitNonceMisuse,     // nonces reused/not checked/incremented
    PermitDeadlineBypass,  // missing/ignored deadline/expiry
    ERC20DecimalsMismatch, // decimals/oracle/price math mismatches
    ERC777HookReentrancy,  // reentrancy via ERC777 hooks
    ERC4626SharePrice,     // vault exchange-rate/share-price bugs
    Custom,

    // added 10/20/2025
    // Authority & Governance
    BeaconFactoryAuthorityDrift,
    TimelockEdgeCase,

    // Reentrancy & Ordering
    CallOrderingOrCEI,
    MulticallCrossPathReentrancy,

    // Oracles / MEV
    OracleHeartbeatFreshness,
    TWAPWindowPinning,

    // Accounting
    ForcedAssetVsStrictEquality,

    //added 11/20/2025
    AuthorityOrGovernance, // rational actor escalates privileges within rules

    // added 12/16/2025 - Missing Megapot patterns
    ArbitraryExternalCall, // user-controlled .call() with calldata enables asset theft
    GlobalParamMidFlowManipulation, // global param changeable mid-flow (before settlement) causes manipulation
    GovernanceFrontrunDoS,          // users can frontrun governance to block parameter changes
    ExternalProtocolKeyCollision,   // external protocol ID/key collision when config changes
    EmergencyModeStateStuck,        // emergency mode blocks settlement while allowing state changes
    IncentiveMisalignmentOrGameTheory, // rational actors profit by harming others or blocking protocol
}

impl Default for VulnerabilityType {
    fn default() -> Self {
        VulnerabilityType::Dos
    }
}

impl VulnerabilityType {
    pub fn as_fancy_str(self) -> &'static str {
        match self {
            VulnerabilityType::Oracle => "Oracle",
            VulnerabilityType::AccessControl => "Access Control",
            VulnerabilityType::FrontrunMev => "Frontrun/Backrun/Sandwhich MEV",
            VulnerabilityType::UnexpectedEth => "Unexpected Eth",
            VulnerabilityType::Pragma => "Pragma",
            VulnerabilityType::Randomness => "Randomness",
            VulnerabilityType::TxOrigin => "tx.origin",
            VulnerabilityType::ZeroCode => "Zero Code",
            VulnerabilityType::SelfDestruct => "Self-Destruct",
            VulnerabilityType::StorageLayout => "Storage Layout",
            VulnerabilityType::ReplayAttack => "Replay Attack",
            VulnerabilityType::ShortAddress => "Short Address",
            VulnerabilityType::IntegerMath => "Integer Overflow/Math",
            VulnerabilityType::UncheckedReturn => "Unchecked Return",
            VulnerabilityType::Dos => "DOS",
            VulnerabilityType::DefaultVisibility => "Default Visibility",
            VulnerabilityType::Inheritance => "Inheritance",
            VulnerabilityType::ConfidentialData => "Confidential Data",
            VulnerabilityType::Reentrancy => "Reentrancy",
            VulnerabilityType::ArrayLimits => "Array Limits",
            VulnerabilityType::UpgradeabilityInitializerSafety => {
                "Upgradeability Initializer Safety"
            }
            VulnerabilityType::PausableEmergencyStop => "Pausable Emergency Stop",
            VulnerabilityType::TimestampDependentLogic => "Timestamp Dependent Logic",
            VulnerabilityType::FlashLoanEconomicManipulation => "Flash Loan Economic Manipulation",
            VulnerabilityType::DelegatecallLowLevelOps => "Delegatecall Low Level Ops",
            VulnerabilityType::SignatureMalleability => "Signature Malleability",
            VulnerabilityType::EventConsistency => "Event Consistency",
            VulnerabilityType::GasGriefBlockLimit => "Gas Grief BlockLimit",
            VulnerabilityType::IntegerOverflow => "Integer Overflow",
            // New additions
            VulnerabilityType::PricePrecision => "Price Precision",
            VulnerabilityType::RoundingError => "Rounding Error",
            VulnerabilityType::FeeOnTransferAssumption => "Fee On Transfer Assumption",
            VulnerabilityType::UncheckedERC20Return => "Unchecked ERC20 Return",
            VulnerabilityType::SignatureReplay => "Signature Replay",
            VulnerabilityType::AuthByPass => "Auth Bypass",
            VulnerabilityType::UntrustedDelegateCall => "Untrusted Delegatecall",
            VulnerabilityType::TimestampManipulation => "Timestamp Manipulation",
            VulnerabilityType::CrossChainMessageSpoofing => "Cross-Chain Message Spoofing",
            VulnerabilityType::AccountingInvariantViolation => "Accounting Invariant Violation",
            VulnerabilityType::SlippageMissingOrInsufficient => "Slippage Missing Or Insufficient",
            VulnerabilityType::Custom => "Unique Custom Issue",
            VulnerabilityType::StandardViolation => "Standard Violation",
            VulnerabilityType::AllowanceRace => "Allowance Race",
            VulnerabilityType::PermitDomainSeparator => "Permit Domain Separator",
            VulnerabilityType::PermitNonceMisuse => "Permit Nonce Misuse",
            VulnerabilityType::PermitDeadlineBypass => "Permit Deadline Bypass",
            VulnerabilityType::ERC20DecimalsMismatch => "ERC20 Decimals Mismatch",
            VulnerabilityType::ERC777HookReentrancy => "ERC777 Hook Reentrancy",
            VulnerabilityType::ERC4626SharePrice => "ERC4626 Share Price",
            VulnerabilityType::BeaconFactoryAuthorityDrift => "Beacon Factory Authority Drift",
            VulnerabilityType::TimelockEdgeCase => "Timelock Edge Case",
            VulnerabilityType::CallOrderingOrCEI => "Call Ordering Or CEI",
            VulnerabilityType::MulticallCrossPathReentrancy => "Multicall Cross Path Reentrancy",
            VulnerabilityType::OracleHeartbeatFreshness => "Oracle Heartbeat Freshness",
            VulnerabilityType::TWAPWindowPinning => "TWAP Window Pinning",
            VulnerabilityType::ForcedAssetVsStrictEquality => "Forced Asset Vs Strict Equality",
            VulnerabilityType::AuthorityOrGovernance => "Authority Or Governance",
            // Added 12/16/2025
            VulnerabilityType::ArbitraryExternalCall => "Arbitrary External Call",
            VulnerabilityType::GlobalParamMidFlowManipulation => {
                "Global Param Mid-Flow Manipulation"
            }
            VulnerabilityType::GovernanceFrontrunDoS => "Governance Frontrun DoS",
            VulnerabilityType::ExternalProtocolKeyCollision => "External Protocol Key Collision",
            VulnerabilityType::EmergencyModeStateStuck => "Emergency Mode State Stuck",
            VulnerabilityType::IncentiveMisalignmentOrGameTheory => {
                "Incentive Misalignment / Game Theory"
            }
        }
    }
}
