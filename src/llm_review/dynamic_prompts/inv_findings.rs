use crate::llm_review::enums::{InvariantType, VulnerabilityType};

pub fn invariant_to_types(inv: InvariantType) -> &'static [VulnerabilityType] {
    match inv {
        InvariantType::Arithmetic => &[
            VulnerabilityType::IntegerMath,
            VulnerabilityType::IntegerOverflow,
            VulnerabilityType::PricePrecision,
            VulnerabilityType::RoundingError,
            VulnerabilityType::ERC20DecimalsMismatch,
            VulnerabilityType::ERC4626SharePrice,
            VulnerabilityType::AccountingInvariantViolation,
        ],

        InvariantType::Balance => &[
            VulnerabilityType::Reentrancy,
            VulnerabilityType::ERC777HookReentrancy,
            VulnerabilityType::UncheckedReturn,
            VulnerabilityType::UncheckedERC20Return,
            VulnerabilityType::FeeOnTransferAssumption,
            VulnerabilityType::UnexpectedEth,
            VulnerabilityType::AccountingInvariantViolation,
            VulnerabilityType::StandardViolation,
        ],

        InvariantType::Permission => &[
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
        ],

        InvariantType::Temporal => &[
            VulnerabilityType::TimestampManipulation,
            VulnerabilityType::TimestampDependentLogic,
            VulnerabilityType::Oracle, // e.g., stale/heartbeat-violating reads
            VulnerabilityType::ReplayAttack, // time/expiry windows
            VulnerabilityType::SignatureReplay, // missing/ignored nonces over time
            VulnerabilityType::PermitDeadlineBypass,
        ],

        InvariantType::Referential => &[
            VulnerabilityType::StorageLayout,    // upgrade misalignments
            VulnerabilityType::ArrayLimits,      // OOB / index drift
            VulnerabilityType::EventConsistency, // logs vs state divergence
            VulnerabilityType::DelegatecallLowLevelOps,
            VulnerabilityType::UpgradeabilityInitializerSafety,
        ],

        InvariantType::StateMachine => &[
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
        ],
    }
}
