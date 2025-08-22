/*
### Example ranking rubric (drop into your critic prompt)
Severity (High/Medium) per C4: ✅ permissionless, ✅ present-state, ✅ financial path.
Profit magnitude (attacker net gain) > payout denial (DoS) > stale/logic mismatch.
Call count & complexity (fewer is better).
Blast radius (affects many users/epochs is better).
Reproducibility (Foundry asserts on balances/totals, not logs).
*/

use crate::llm_review::{enums::VulnerabilityType, patterns::VulnerabilityPattern};

pub fn pattern_to_types(pattern: VulnerabilityPattern) -> &'static [VulnerabilityType] {
    match pattern {
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
        VulnerabilityPattern::UncheckedLowLevelCallResults => &[VulnerabilityType::UncheckedReturn],
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
