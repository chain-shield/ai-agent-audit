use crate::llm_review::invariants::InvariantType;
/// AI agent and vulnerability type enumerations.
///
/// This module defines the core enums for multi-LLM support and vulnerability
/// categorization, providing unified interfaces for different AI providers
/// and systematic vulnerability detection across 19+ security categories.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    invariant_prompts::{
        arithmetic::ARITHMETIC, balance::BALANCE, permission::PERMISSION, referential::REFERENTIAL,
        state_machine::STATE_MACHINE, temporal::TEMPORAL,
    },
    utils::extract_retry::agent_extract_with_retry,
};
use rig::{
    agent::Agent,
    extractor::Extractor,
    providers::{
        anthropic, deepseek, gemini,
        openai::{self},
    },
};

use serde::de::DeserializeOwned;

/// Configuration metadata for AI agents.
/// Stores the original configuration used to create the agent for pricing calculations.
#[derive(Debug, Clone, Default)]
pub struct AgentMetadata {
    pub model: String,
    pub temperature: f64,
    pub service_tier: Option<String>,
    pub reasoning_effort: Option<String>,
    pub file_picker_enabled: bool,
    pub file_retrieval_enabled: bool,
    pub dynamic_context_enabled: bool,
}

/// Unified AI agent enum supporting multiple LLM providers.
///
/// Provides a common interface for different AI providers while maintaining
/// provider-specific optimizations and cost tracking capabilities.
pub enum AIAgent {
    /// Anthropic Claude models (3.7 Sonnet, 4.0 Sonnet)
    Anthropic {
        agent: Agent<anthropic::completion::CompletionModel>,
        metadata: AgentMetadata,
    },
    /// OpenAI models (GPT-4o, O3)
    Openai {
        agent: Agent<openai::responses_api::ResponsesCompletionModel>,
        metadata: AgentMetadata,
    },
    /// Google Gemini models
    Gemini {
        agent: Agent<gemini::completion::CompletionModel>,
        metadata: AgentMetadata,
    },
    /// DeepSeek models (cost-effective option)
    Deepseek {
        agent: Agent<deepseek::CompletionModel>,
        metadata: AgentMetadata,
    },
}

/// Unified AI extractor enum for structured data extraction.
///
/// Provides type-safe extraction capabilities across different LLM providers
/// with automatic retry logic and error handling.
pub enum AIExtractor<T>
where
    T: 'static + JsonSchema + Serialize + for<'a> Deserialize<'a> + Send + Sync,
{
    Anthropic(Extractor<anthropic::completion::CompletionModel, T>),
    Openai(Extractor<openai::responses_api::ResponsesCompletionModel, T>),
    Gemini(Extractor<gemini::completion::CompletionModel, T>),
    Deepseek(Extractor<deepseek::CompletionModel, T>),
}

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
}

// EnumString trait removed - use strum's Display trait (to_string()) instead!

pub trait EnumData {
    type Spec;
    fn to_types(&self) -> &'static [VulnerabilityType];
    fn get_spec(&self) -> Self::Spec;
    // get predicate or defintion of patten
}

pub fn generate_enum_list<T: std::fmt::Display>(patterns: &[T]) -> String {
    let mut enum_list = String::new();
    let top_pattern_count = patterns.len();
    for (i, pattern) in patterns.iter().enumerate() {
        enum_list.push_str(&pattern.to_string());
        if i < top_pattern_count - 1 {
            enum_list.push_str("|");
        }
    }
    enum_list
}

pub fn generate_enum_bulleted_list<T: std::fmt::Display>(patterns: &[T]) -> String {
    let mut enum_list = String::new();
    enum_list.push_str("\n");
    for pattern in patterns {
        enum_list.push_str(&format!("- {}", pattern.to_string()));
        enum_list.push_str("\n");
    }
    enum_list
}

pub fn all_enum_variants<T: IntoEnumIterator>() -> Vec<T> {
    T::iter().collect()
}

// No longer needed! strum's Display trait provides to_string() for free

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
        }
    }
}

impl InvariantType {
    pub fn get_prompt(&self) -> &'static str {
        match self {
            InvariantType::Arithmetic => ARITHMETIC,
            InvariantType::Balance => BALANCE,
            InvariantType::Permission => PERMISSION,
            InvariantType::Temporal => TEMPORAL,
            InvariantType::Referential => REFERENTIAL,
            InvariantType::StateMachine => STATE_MACHINE,
        }
    }
}

impl AIAgent {
    /// Simple prompt method for text generation
    pub async fn prompt(&self, prompt: &str) -> anyhow::Result<String> {
        use rig::completion::Prompt;

        let out = match self {
            AIAgent::Anthropic { agent, .. } => agent.prompt(prompt).await?,
            AIAgent::Openai { agent, .. } => agent.prompt(prompt).await?,
            AIAgent::Gemini { agent, .. } => agent.prompt(prompt).await?,
            AIAgent::Deepseek { agent, .. } => agent.prompt(prompt).await?,
        };
        Ok(out)
    }

    //
    pub async fn extract_with_retry<T>(&self, prompt: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        match self {
            AIAgent::Anthropic { agent, metadata } => {
                Ok(agent_extract_with_retry::<_, T>(agent, prompt, metadata).await?)
            }
            AIAgent::Openai { agent, metadata } => {
                Ok(agent_extract_with_retry::<_, T>(agent, prompt, metadata).await?)
            }
            AIAgent::Gemini { agent, metadata } => {
                Ok(agent_extract_with_retry::<_, T>(agent, prompt, metadata).await?)
            }
            AIAgent::Deepseek { agent, metadata } => {
                Ok(agent_extract_with_retry::<_, T>(agent, prompt, metadata).await?)
            }
        }
    }

    // Getter methods for pricing calculations and configuration inspection

    /// Gets the model name.
    pub fn get_model(&self) -> &str {
        match self {
            AIAgent::Anthropic { metadata, .. } => &metadata.model,
            AIAgent::Openai { metadata, .. } => &metadata.model,
            AIAgent::Gemini { metadata, .. } => &metadata.model,
            AIAgent::Deepseek { metadata, .. } => &metadata.model,
        }
    }

    /// Gets the OpenAI service tier.
    /// Returns "default" if not explicitly set or not applicable.
    pub fn get_service_tier(&self) -> &str {
        match self {
            AIAgent::Openai { metadata, .. } => {
                metadata.service_tier.as_deref().unwrap_or("default")
            }
            _ => "default", // Non-OpenAI providers don't have service tiers
        }
    }

    /// Gets the OpenAI reasoning effort level.
    /// Returns "medium" if not explicitly set or not applicable.
    pub fn get_reasoning_effort(&self) -> &str {
        match self {
            AIAgent::Openai { metadata, .. } => {
                metadata.reasoning_effort.as_deref().unwrap_or("medium")
            }
            _ => "medium", // Non-OpenAI providers don't have reasoning effort
        }
    }

    /// Gets the temperature setting.
    pub fn get_temperature(&self) -> f64 {
        match self {
            AIAgent::Anthropic { metadata, .. } => metadata.temperature,
            AIAgent::Openai { metadata, .. } => metadata.temperature,
            AIAgent::Gemini { metadata, .. } => metadata.temperature,
            AIAgent::Deepseek { metadata, .. } => metadata.temperature,
        }
    }

    /// Checks if file picker is enabled.
    pub fn is_file_picker_enabled(&self) -> bool {
        match self {
            AIAgent::Anthropic { metadata, .. } => metadata.file_picker_enabled,
            AIAgent::Openai { metadata, .. } => metadata.file_picker_enabled,
            AIAgent::Gemini { metadata, .. } => metadata.file_picker_enabled,
            AIAgent::Deepseek { metadata, .. } => metadata.file_picker_enabled,
        }
    }

    /// Checks if file retrieval is enabled.
    pub fn is_file_retrieval_enabled(&self) -> bool {
        match self {
            AIAgent::Anthropic { metadata, .. } => metadata.file_retrieval_enabled,
            AIAgent::Openai { metadata, .. } => metadata.file_retrieval_enabled,
            AIAgent::Gemini { metadata, .. } => metadata.file_retrieval_enabled,
            AIAgent::Deepseek { metadata, .. } => metadata.file_retrieval_enabled,
        }
    }

    /// Checks if dynamic context is enabled.
    pub fn is_dynamic_context_enabled(&self) -> bool {
        match self {
            AIAgent::Anthropic { metadata, .. } => metadata.dynamic_context_enabled,
            AIAgent::Openai { metadata, .. } => metadata.dynamic_context_enabled,
            AIAgent::Gemini { metadata, .. } => metadata.dynamic_context_enabled,
            AIAgent::Deepseek { metadata, .. } => metadata.dynamic_context_enabled,
        }
    }

    /// Gets the provider name.
    pub fn get_provider(&self) -> &'static str {
        match self {
            AIAgent::Anthropic { .. } => "anthropic",
            AIAgent::Openai { .. } => "openai",
            AIAgent::Gemini { .. } => "gemini",
            AIAgent::Deepseek { .. } => "deepseek",
        }
    }

    /// Gets the complete metadata for pricing calculations and configuration inspection.
    /// This is the easiest way to get metadata for cost tracking functions.
    pub fn get_metadata(&self) -> &AgentMetadata {
        match self {
            AIAgent::Anthropic { metadata, .. } => metadata,
            AIAgent::Openai { metadata, .. } => metadata,
            AIAgent::Gemini { metadata, .. } => metadata,
            AIAgent::Deepseek { metadata, .. } => metadata,
        }
    }
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
        }
    }
}

// ----- Serde glue --------------------------------------------------
// All enums now use serde's derive for both Serialize and Deserialize!
// LLMs are instructed to return exact PascalCase in prompts, so no need for
// case-insensitive deserialization.
