use crate::llm_review::invariants::InvariantType;
use log::info;
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
    cost::cost_data::{TokenType, add_to_inference_cost_by_type},
    invariant_prompts::{
        arithmetic::ARITHMETIC, balance::BALANCE, permission::PERMISSION, referential::REFERENTIAL,
        state_machine::STATE_MACHINE, temporal::TEMPORAL,
    },
    master_prompts::master_prompt::MASTER_SECURITY_PROMPT,
    prepare_code::git_clone::RepoPaths,
    prompts::{
        access_control::ACCESS_CONTROL, array_limits::ACCESS_OUTSIDE_ARRAY_LIMITS,
        confidential_data::SAVING_CONFIDENTIAL_DATA, default_visibility::DEFAULT_VISIBILITIES,
        dos::DOS, inheritance::WRONG_INHERITANCE, integer_overflow::INTEGER_OVERFLOW, mev::MEV,
        oracle::ORACLE_MANIPULATION, pragma::FLOATING_PRAGMA, randomness::RANDOMNESS,
        reentrancy::REENTRANCY, replay_attack::REPLAY_SIGNATURES_ATTACK,
        self_destruct::SELF_DESTRUCT, short_address_attack::SHORT_ADDRESS_ATTACK,
        storage_variables::STORAGE_VARIABLE, tx_origin::TX_ORIGIN,
        unchecked_return_value::UNCHECK_RETURN_VALUES, unexpected_eth::UNEXPECTED_ETH,
        zero_code::CONTRACTS_WITH_ZERO_CODE,
    },
    utils::extract_retry::agent_extract_with_retry,
};
use rig::{
    agent::Agent,
    completion::{CompletionModel, Prompt},
    extractor::Extractor,
    providers::{
        anthropic, deepseek, gemini,
        openai::{self},
    },
};

use serde::de::DeserializeOwned;

use super::{
    agent_factory::{AgentConfig, AgentFactory},
    prompt_support::{extractor_prompt::EXTRACTOR_AGENT, pre_prompt::PRE_PROMPT},
};

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
    pub async fn get_prompt_then_extract_with_retry<T>(
        &self,
        prompt: &str,
        repo: &RepoPaths,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        match self {
            AIAgent::Anthropic { agent, .. } => {
                self.run_analysis_and_extract(agent, prompt, repo).await
            }

            AIAgent::Openai { agent, .. } => {
                self.run_analysis_and_extract(agent, prompt, repo).await
            }
            AIAgent::Gemini { agent, .. } => {
                self.run_analysis_and_extract(agent, prompt, repo).await
            }
            AIAgent::Deepseek { agent, .. } => {
                self.run_analysis_and_extract(agent, prompt, repo).await
            }
        }
    }

    async fn run_analysis_and_extract<T, M>(
        &self,
        model: &Agent<M>,
        prompt: &str,
        repo: &RepoPaths,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        M: CompletionModel, // whatever trait `model.prompt()` uses
    {
        // 🆕 Create extractor agent
        let extractor_config = AgentConfig::new(Some(repo.clone()))
            .with_model("gpt-5")
            .with_preamble(
                "You are an expert at extracting data and converting it into strict JSON.",
            );
        let extractor_agent = AgentFactory::create_openai_agent(&extractor_config)?;
        let extractor = match extractor_agent {
            AIAgent::Openai { agent, .. } => agent,
            _ => anyhow::bail!("Unexpected agent type — expected OpenAI"),
        };

        // 🚀 Run the model
        info!("submitting for analysis...");
        log::debug!("Prompt length: {} characters", prompt.len());

        // 📝 Track inference INPUT cost (MISSING!)
        let metadata = match self {
            AIAgent::Anthropic { metadata, .. } => metadata,
            AIAgent::Openai { metadata, .. } => metadata,
            AIAgent::Gemini { metadata, .. } => metadata,
            AIAgent::Deepseek { metadata, .. } => metadata,
        };
        add_to_inference_cost_by_type(prompt, metadata, TokenType::Input).await;

        let analysis = match model.prompt(prompt).await {
            Ok(result) => result,
            Err(e) => {
                // ✅ Print the full error details
                log::error!("Model prompt failed: {:?}", e);

                // Print the error chain to get more details
                let mut current_error: &dyn std::error::Error = &e;
                while let Some(source) = current_error.source() {
                    log::error!("Caused by: {}", source);
                    current_error = source;
                }

                // Log additional context for debugging
                log::error!("Error occurred during model prompt execution");
                log::error!("This might be caused by:");
                log::error!("1. Tool call arguments containing invalid JSON characters");
                log::error!("2. LLM response containing malformed JSON");
                log::error!("3. Tool output being too large or containing special characters");
                log::error!("4. Network/API issues");

                // Check if this is a JSON parsing error specifically
                let error_string = format!("{:?}", e);
                if error_string.contains("expected value") || error_string.contains("Decode") {
                    log::error!("🚨 This appears to be a JSON parsing error!");
                    log::error!("💡 Possible solutions:");
                    log::error!("   - Reduce tool query complexity");
                    log::error!("   - Check for special characters in tool arguments");
                    log::error!("   - Verify tool output sanitization");
                }

                // Return the error as-is
                return Err(e.into());
            }
        };

        // 📝 Track inference output
        let metadata = match self {
            AIAgent::Anthropic { metadata, .. } => metadata,
            AIAgent::Openai { metadata, .. } => metadata,
            AIAgent::Gemini { metadata, .. } => metadata,
            AIAgent::Deepseek { metadata, .. } => metadata,
        };
        add_to_inference_cost_by_type(&analysis, metadata, TokenType::Output).await;

        // 📝 Build extractor prompt
        let extract_prompt = format!(
            "{}{}\n\n## SECURITY AUDIT FINDINGS TO CONVERT TO JSON\n\n{}",
            PRE_PROMPT, EXTRACTOR_AGENT, analysis
        );

        // 📝 Track inference input
        // Create metadata for extractor agent (OpenAI GPT-5)
        let extractor_metadata = AgentMetadata {
            model: "gpt-5".to_string(),
            temperature: 0.3,
            service_tier: None,     // Default service tier for extractor
            reasoning_effort: None, // Default reasoning effort for extractor
            file_picker_enabled: false,
            file_retrieval_enabled: false,
            dynamic_context_enabled: false,
        };
        add_to_inference_cost_by_type(&extract_prompt, &extractor_metadata, TokenType::Input).await;

        // 🧠 Run extractor with retry
        Ok(
            agent_extract_with_retry::<_, T>(&extractor, &extract_prompt, &extractor_metadata)
                .await?,
        )
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

    pub fn prompt(self) -> &'static str {
        match self {
            VulnerabilityType::Oracle => ORACLE_MANIPULATION,
            VulnerabilityType::AccessControl => ACCESS_CONTROL,
            VulnerabilityType::FrontrunMev => MEV,
            VulnerabilityType::UnexpectedEth => UNEXPECTED_ETH,
            VulnerabilityType::Pragma => FLOATING_PRAGMA,
            VulnerabilityType::Randomness => RANDOMNESS,
            VulnerabilityType::TxOrigin => TX_ORIGIN,
            VulnerabilityType::ZeroCode => CONTRACTS_WITH_ZERO_CODE,
            VulnerabilityType::SelfDestruct => SELF_DESTRUCT,
            VulnerabilityType::StorageLayout => STORAGE_VARIABLE,
            VulnerabilityType::ReplayAttack => REPLAY_SIGNATURES_ATTACK,
            VulnerabilityType::ShortAddress => SHORT_ADDRESS_ATTACK,
            VulnerabilityType::IntegerMath => INTEGER_OVERFLOW,
            VulnerabilityType::UncheckedReturn => UNCHECK_RETURN_VALUES,
            VulnerabilityType::Dos => DOS,
            VulnerabilityType::DefaultVisibility => DEFAULT_VISIBILITIES,
            VulnerabilityType::Inheritance => WRONG_INHERITANCE,
            VulnerabilityType::ConfidentialData => SAVING_CONFIDENTIAL_DATA,
            VulnerabilityType::Reentrancy => REENTRANCY,
            VulnerabilityType::ArrayLimits => ACCESS_OUTSIDE_ARRAY_LIMITS,
            _ => MASTER_SECURITY_PROMPT,
        }
    }
}

// ----- Serde glue --------------------------------------------------
// All enums now use serde's derive for both Serialize and Deserialize!
// LLMs are instructed to return exact PascalCase in prompts, so no need for
// case-insensitive deserialization.
