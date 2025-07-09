/// AI agent and vulnerability type enumerations.
///
/// This module defines the core enums for multi-LLM support and vulnerability
/// categorization, providing unified interfaces for different AI providers
/// and systematic vulnerability detection across 19+ security categories.

use schemars::JsonSchema;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    cost::cost_data::LlmCostType,
    invariant_prompts::{
        arithmetic::ARITHMETIC, balance::BALANCE, permission::PERMISSION, referential::REFERENTIAL,
        state_machine::STATE_MACHINE, temporal::TEMPORAL,
    },
    master_prompts::master_prompt::MASTER_SECURITY_PROMPT,
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
    extractor::Extractor,
    providers::{
        anthropic::{self},
        deepseek::DeepSeekCompletionModel,
        gemini::{self},
        openai::{self},
    },
};

use serde::de::DeserializeOwned;

/// Unified AI agent enum supporting multiple LLM providers.
///
/// Provides a common interface for different AI providers while maintaining
/// provider-specific optimizations and cost tracking capabilities.
pub enum AIAgent {
    /// Anthropic Claude models (3.7 Sonnet, 4.0 Sonnet)
    Anthropic(Agent<anthropic::completion::CompletionModel>),
    /// OpenAI models (GPT-4o, O3)
    Openai(Agent<openai::CompletionModel>),
    /// Google Gemini models
    Gemini(Agent<gemini::completion::CompletionModel>),
    /// DeepSeek models (cost-effective option)
    Deepseek(Agent<DeepSeekCompletionModel>),
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
    Openai(Extractor<openai::CompletionModel, T>),
    Gemini(Extractor<gemini::completion::CompletionModel, T>),
    Deepseek(Extractor<DeepSeekCompletionModel, T>),
}

/// ------------------------------------------------------------------
/// 1.  Strict-typed severity enum
/// ------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
pub enum Severity {
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
pub enum InvariantType {
    Arithmetic,
    Balance,
    Permission,
    Temporal,
    Referential,
    StateMachine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
pub enum InvariantStatus {
    HOLDS,
    VIOLATION,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
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
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Info
    }
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::High => "High",
            Severity::Medium => "Medium",
            Severity::Low => "Low",
            Severity::Info => "Info",
        }
    }

    pub fn as_initial(self) -> &'static str {
        match self {
            Severity::High => "H",
            Severity::Medium => "M",
            Severity::Low => "L",
            Severity::Info => "I",
        }
    }
}

impl InvariantType {
    pub fn as_str(self) -> &'static str {
        match self {
            InvariantType::Arithmetic => "Arithmetic",
            InvariantType::Balance => "Balance",
            InvariantType::Permission => "Permission",
            InvariantType::Temporal => "Temporal",
            InvariantType::Referential => "Referential",
            InvariantType::StateMachine => "StateMachine",
        }
    }

    pub fn get_prompt(self) -> &'static str {
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

impl InvariantStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            InvariantStatus::HOLDS => "Holds",
            InvariantStatus::VIOLATION => "Violation",
        }
    }
}

impl AIAgent {
    pub async fn extract_with_retry<T>(&self, prompt: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        match self {
            AIAgent::Anthropic(model) => Ok(agent_extract_with_retry::<_, T>(
                model,
                prompt,
                LlmCostType::AnthropicClaudeOutput,
            )
            .await?),
            AIAgent::Openai(model) => {
                Ok(
                    agent_extract_with_retry::<_, T>(model, prompt, LlmCostType::OpenaiO3Output)
                        .await?,
                )
            }
            AIAgent::Gemini(model) => {
                Ok(
                    agent_extract_with_retry::<_, T>(model, prompt, LlmCostType::GeminiOutput)
                        .await?,
                )
            }
            AIAgent::Deepseek(model) => {
                Ok(
                    agent_extract_with_retry::<_, T>(model, prompt, LlmCostType::DeepseekOutput)
                        .await?,
                )
            }
        }
    }
}

impl Default for VulnerabilityType {
    fn default() -> Self {
        VulnerabilityType::Dos
    }
}

impl VulnerabilityType {
    pub fn as_str(self) -> &'static str {
        match self {
            VulnerabilityType::Oracle => "Oracle",
            VulnerabilityType::AccessControl => "AccessControl",
            VulnerabilityType::FrontrunMev => "FrontrunMev",
            VulnerabilityType::UnexpectedEth => "UnexpectedEth",
            VulnerabilityType::Pragma => "Pragma",
            VulnerabilityType::Randomness => "Randomness",
            VulnerabilityType::TxOrigin => "TxOrigin",
            VulnerabilityType::ZeroCode => "ZeroCode",
            VulnerabilityType::SelfDestruct => "SelfDestruct",
            VulnerabilityType::StorageLayout => "StorageLayout",
            VulnerabilityType::ReplayAttack => "ReplayAttack",
            VulnerabilityType::ShortAddress => "ShortAddress",
            VulnerabilityType::IntegerMath => "IntegerMath",
            VulnerabilityType::UncheckedReturn => "UncheckedReturn",
            VulnerabilityType::Dos => "Dos",
            VulnerabilityType::DefaultVisibility => "DefaultVisibility",
            VulnerabilityType::Inheritance => "Inheritance",
            VulnerabilityType::ConfidentialData => "ConfidentialData",
            VulnerabilityType::Reentrancy => "Reentrancy",
            VulnerabilityType::ArrayLimits => "ArrayLimits",
            VulnerabilityType::UpgradeabilityInitializerSafety => "UpgradeabilityInitializerSafety",
            VulnerabilityType::PausableEmergencyStop => "PausableEmergencyStop",
            VulnerabilityType::TimestampDependentLogic => "TimestampDependentLogic",
            VulnerabilityType::FlashLoanEconomicManipulation => "FlashLoanEconomicManipulation",
            VulnerabilityType::DelegatecallLowLevelOps => "DelegatecallLowLevelOps",
            VulnerabilityType::SignatureMalleability => "SignatureMalleability",
            VulnerabilityType::EventConsistency => "EventConsistency",
            VulnerabilityType::GasGriefBlockLimit => "GasGriefBlockLimit",
            VulnerabilityType::IntegerOverflow => "IntegerOverflow",
        }
    }

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

/// ----- Serde glue --------------------------------------------------
/// * Accepts any case-insensitive spelling: "high", "HIGH", "High" …
impl<'de> Deserialize<'de> for Severity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "high" => Ok(Severity::High),
            "medium" => Ok(Severity::Medium),
            "low" => Ok(Severity::Low),
            "info" => Ok(Severity::Info),
            other => Err(de::Error::unknown_variant(
                other,
                &["High", "Medium", "Low", "Info"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for InvariantType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "arithmetic" => Ok(InvariantType::Arithmetic),
            "balance" => Ok(InvariantType::Balance),
            "permission" => Ok(InvariantType::Permission),
            "temporal" => Ok(InvariantType::Temporal),
            "referential" => Ok(InvariantType::Referential),
            "statemachine" => Ok(InvariantType::StateMachine),
            other => Err(de::Error::unknown_variant(
                other,
                &[
                    "Arithmetic",
                    "Balance",
                    "Permission",
                    "Temporal",
                    "Referential",
                    "StateMachine",
                ],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for InvariantStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "holds" => Ok(InvariantStatus::HOLDS),
            "violation" => Ok(InvariantStatus::VIOLATION),
            other => Err(de::Error::unknown_variant(
                other,
                &["High", "Medium", "Low", "Info"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for VulnerabilityType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "oracle" => Ok(VulnerabilityType::Oracle),
            "accesscontrol" => Ok(VulnerabilityType::AccessControl),
            "frontrunmev" => Ok(VulnerabilityType::FrontrunMev),
            "unexpectedeth" => Ok(VulnerabilityType::UnexpectedEth),
            "pragma" => Ok(VulnerabilityType::Pragma),
            "randomness" => Ok(VulnerabilityType::Randomness),
            "txorigin" => Ok(VulnerabilityType::TxOrigin),
            "zerocode" => Ok(VulnerabilityType::ZeroCode),
            "selfdestruct" => Ok(VulnerabilityType::SelfDestruct),
            "storagelayout" => Ok(VulnerabilityType::StorageLayout),
            "replayattack" => Ok(VulnerabilityType::ReplayAttack),
            "shortaddress" => Ok(VulnerabilityType::ShortAddress),
            "integermath" => Ok(VulnerabilityType::IntegerMath),
            "uncheckedreturn" => Ok(VulnerabilityType::UncheckedReturn),
            "dos" => Ok(VulnerabilityType::Dos),
            "defaultvisibility" => Ok(VulnerabilityType::DefaultVisibility),
            "inheritance" => Ok(VulnerabilityType::Inheritance),
            "confidentialdata" => Ok(VulnerabilityType::ConfidentialData),
            "reentrancy" => Ok(VulnerabilityType::Reentrancy),
            "arraylimits" => Ok(VulnerabilityType::ArrayLimits),
            "upgradeabilityinitializersafety" => {
                Ok(VulnerabilityType::UpgradeabilityInitializerSafety)
            }
            "pausableemergencystop" => Ok(VulnerabilityType::PausableEmergencyStop),
            "timestampdependentlogic" => Ok(VulnerabilityType::TimestampDependentLogic),
            "flashloaneconomicmanipulation" => Ok(VulnerabilityType::FlashLoanEconomicManipulation),
            "delegatecalllowlevelops" => Ok(VulnerabilityType::DelegatecallLowLevelOps),
            "signaturemalleability" => Ok(VulnerabilityType::SignatureMalleability),
            "eventconsistency" => Ok(VulnerabilityType::EventConsistency),
            "gasgriefblocklimit" => Ok(VulnerabilityType::GasGriefBlockLimit),
            "integeroverflow" => Ok(VulnerabilityType::IntegerOverflow),
            other => Err(de::Error::unknown_variant(
                other,
                &[
                    "oracle",
                    "accesscontrol",
                    "frontrunattack",
                    "unexpectedeth",
                    "pragma",
                    "randomness",
                    "txorigin",
                    "zerocode",
                    "selfdestruct",
                    "storagelayout",
                    "replayattack",
                    "shortaddress",
                    "integermath",
                    "uncheckedreturn",
                    "dos",
                    "defaultvisibility",
                    "inheritance",
                    "confidentialdata",
                    "reentrancy",
                    "arraylimits",
                    "frontrunmev",
                    "upgradeabilityinitializersafety",
                    "pausableemergencystop",
                    "timestampdependentlogic",
                    "flashloaneconomicmanipulation",
                    "delegatecalllowlevelops",
                    "signaturemalleability",
                    "eventconsistency",
                    "gasgriefblocklimit",
                    "integeroverflow",
                ],
            )),
        }
    }
}

impl Serialize for Severity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Serialize for InvariantType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Serialize for InvariantStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Serialize for VulnerabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
