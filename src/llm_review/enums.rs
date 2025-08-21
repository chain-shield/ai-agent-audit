use log::info;
/// AI agent and vulnerability type enumerations.
///
/// This module defines the core enums for multi-LLM support and vulnerability
/// categorization, providing unified interfaces for different AI providers
/// and systematic vulnerability detection across 19+ security categories.
use schemars::JsonSchema;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    cost::cost_data::{add_to_inference_cost_by_type, LlmCostType},
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
        openai::{self, O3},
    },
};

use serde::de::DeserializeOwned;

use super::{
    agent_factory::{AgentConfig, AgentFactory},
    prompt_support::{extractor_prompt::EXTRACTOR_AGENT, pre_prompt::PRE_PROMPT},
};

/// Unified AI agent enum supporting multiple LLM providers.
///
/// Provides a common interface for different AI providers while maintaining
/// provider-specific optimizations and cost tracking capabilities.
pub enum AIAgent {
    /// Anthropic Claude models (3.7 Sonnet, 4.0 Sonnet)
    Anthropic(Agent<anthropic::completion::CompletionModel>),
    /// OpenAI models (GPT-4o, O3)
    Openai(Agent<openai::responses_api::ResponsesCompletionModel>),
    /// Google Gemini models
    Gemini(Agent<gemini::completion::CompletionModel>),
    /// DeepSeek models (cost-effective option)
    Deepseek(Agent<deepseek::CompletionModel>),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema, EnumIter)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema, EnumIter)]
pub enum InvariantType {
    Arithmetic,
    Balance,
    Permission,
    Temporal,
    Referential,
    StateMachine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema, EnumIter)]
pub enum InvariantStatus {
    Holds,
    PossibleViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema, EnumIter)]
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
    Custom,
}

pub trait EnumString {
    fn as_str(&self) -> &'static str;
}

pub fn generate_enum_list<T: EnumString>(patterns: &[T]) -> String {
    let mut enum_list = String::new();
    let top_pattern_count = patterns.len();
    for (i, pattern) in patterns.iter().enumerate() {
        enum_list.push_str(pattern.as_str());
        if i < top_pattern_count - 1 {
            enum_list.push_str("|");
        }
    }
    enum_list
}
pub fn all_enum_variants<T: IntoEnumIterator>() -> Vec<T> {
    T::iter().collect()
}

impl EnumString for Severity {
    fn as_str(&self) -> &'static str {
        match self {
            Severity::Critical => "Critical",
            Severity::High => "High",
            Severity::Medium => "Medium",
            Severity::Low => "Low",
            Severity::Info => "Info",
        }
    }
}

impl EnumString for InvariantType {
    fn as_str(&self) -> &'static str {
        match self {
            InvariantType::Arithmetic => "Arithmetic",
            InvariantType::Balance => "Balance",
            InvariantType::Permission => "Permission",
            InvariantType::Temporal => "Temporal",
            InvariantType::Referential => "Referential",
            InvariantType::StateMachine => "StateMachine",
        }
    }
}

impl EnumString for InvariantStatus {
    fn as_str(&self) -> &'static str {
        match self {
            InvariantStatus::Holds => "Holds",
            InvariantStatus::PossibleViolation => "PossibleViolation",
        }
    }
}

impl EnumString for VulnerabilityType {
    fn as_str(&self) -> &'static str {
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
            // New additions
            VulnerabilityType::PricePrecision => "PricePrecision",
            VulnerabilityType::RoundingError => "RoundingError",
            VulnerabilityType::FeeOnTransferAssumption => "FeeOnTransferAssumption",
            VulnerabilityType::UncheckedERC20Return => "UncheckedERC20Return",
            VulnerabilityType::SignatureReplay => "SignatureReplay",
            VulnerabilityType::AuthByPass => "AuthByPass",
            VulnerabilityType::UntrustedDelegateCall => "UntrustedDelegateCall",
            VulnerabilityType::TimestampManipulation => "TimestampManipulation",
            VulnerabilityType::CrossChainMessageSpoofing => "CrossChainMessageSpoofing",
            VulnerabilityType::AccountingInvariantViolation => "AccountingInvariantViolation",
            VulnerabilityType::SlippageMissingOrInsufficient => "SlippageMissingOrInsufficient",
            VulnerabilityType::Custom => "Custom",
        }
    }
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
    pub async fn get_prompt_then_extract_with_retry<T>(
        &self,
        prompt: &str,
        repo: &RepoPaths,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        match self {
            AIAgent::Anthropic(model) => {
                self.run_analysis_and_extract(
                    model,
                    prompt,
                    repo,
                    LlmCostType::AnthropicClaudeOutput,
                )
                .await
            }

            AIAgent::Openai(model) => {
                self.run_analysis_and_extract(model, prompt, repo, LlmCostType::OpenaiO3Output)
                    .await
            }
            AIAgent::Gemini(model) => {
                self.run_analysis_and_extract(model, prompt, repo, LlmCostType::GeminiOutput)
                    .await
            }
            AIAgent::Deepseek(model) => {
                self.run_analysis_and_extract(model, prompt, repo, LlmCostType::DeepseekOutput)
                    .await
            }
        }
    }

    async fn run_analysis_and_extract<T, M>(
        &self,
        model: &Agent<M>,
        prompt: &str,
        repo: &RepoPaths,
        output_cost_type: LlmCostType,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        M: CompletionModel, // whatever trait `model.prompt()` uses
    {
        // 🆕 Create extractor agent
        let extractor_config = AgentConfig::new(repo.clone()).with_model(O3).with_preamble(
            "You are an expert at extracting data and converting it into strict JSON.",
        );
        let extractor_agent = AgentFactory::create_openai_agent(&extractor_config)?;
        let extractor = match extractor_agent {
            AIAgent::Openai(agent) => agent,
            _ => anyhow::bail!("Unexpected agent type — expected OpenAI"),
        };

        // 🚀 Run the model
        info!("submitting for analysis...");
        log::debug!("Prompt length: {} characters", prompt.len());

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
        add_to_inference_cost_by_type(&analysis, output_cost_type).await;

        // 📝 Build extractor prompt
        let extract_prompt = format!(
            "{}{}\n\n## SECURITY AUDIT FINDINGS TO CONVERT TO JSON\n\n{}",
            PRE_PROMPT, EXTRACTOR_AGENT, analysis
        );

        // 📝 Track inference input
        add_to_inference_cost_by_type(&extract_prompt, LlmCostType::OpenaiO3Input).await;

        // 🧠 Run extractor with retry
        Ok(agent_extract_with_retry::<_, T>(
            &extractor,
            &extract_prompt,
            LlmCostType::OpenaiO3Output,
        )
        .await?)
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
            "critical" => Ok(Severity::Critical),
            "high" => Ok(Severity::High),
            "medium" => Ok(Severity::Medium),
            "low" => Ok(Severity::Low),
            "info" => Ok(Severity::Info),
            other => Err(de::Error::unknown_variant(
                other,
                &["Critical", "High", "Medium", "Low", "Info"],
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
            "holds" => Ok(InvariantStatus::Holds),
            "possibleviolation" => Ok(InvariantStatus::PossibleViolation),
            other => Err(de::Error::unknown_variant(
                other,
                &["holds", "possibleviolation"],
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
            // New variants
            "priceprecision" => Ok(VulnerabilityType::PricePrecision),
            "roundingerror" => Ok(VulnerabilityType::RoundingError),
            "feeontransferassumption" => Ok(VulnerabilityType::FeeOnTransferAssumption),
            "uncheckederc20return" => Ok(VulnerabilityType::UncheckedERC20Return),
            "signaturereplay" => Ok(VulnerabilityType::SignatureReplay),
            "authbypass" => Ok(VulnerabilityType::AuthByPass),
            "untrusteddelegatecall" => Ok(VulnerabilityType::UntrustedDelegateCall),
            "timestampmanipulation" => Ok(VulnerabilityType::TimestampManipulation),
            "crosschainmessagespoofing" => Ok(VulnerabilityType::CrossChainMessageSpoofing),
            "accountinginvariantviolation" => Ok(VulnerabilityType::AccountingInvariantViolation),
            "slippagemissingorinsufficient" => Ok(VulnerabilityType::SlippageMissingOrInsufficient),
            "custom" => Ok(VulnerabilityType::Custom),
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
                    "priceprecision",
                    "roundingerror",
                    "feeontransferassumption",
                    "uncheckederc20return",
                    "signaturereplay",
                    "authbypass",
                    "untrusteddelegatecall",
                    "timestampmanipulation",
                    "crosschainmessagespoofing",
                    "accountinginvariantviolation",
                    "slippagemissingorinsufficient",
                    "custom",
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
