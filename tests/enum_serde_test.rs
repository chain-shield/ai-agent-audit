use ai_agent_audit::llm_review::enums::{Severity, VulnerabilityType};
use serde_json;

#[test]
fn test_severity_serialize() {
    assert_eq!(
        serde_json::to_string(&Severity::Critical).unwrap(),
        "\"Critical\""
    );
    assert_eq!(serde_json::to_string(&Severity::High).unwrap(), "\"High\"");
    assert_eq!(
        serde_json::to_string(&Severity::Medium).unwrap(),
        "\"Medium\""
    );
    assert_eq!(serde_json::to_string(&Severity::Low).unwrap(), "\"Low\"");
    assert_eq!(serde_json::to_string(&Severity::Info).unwrap(), "\"Info\"");
}

#[test]
fn test_severity_deserialize() {
    // LLMs are instructed to return exact PascalCase in prompts
    assert_eq!(
        serde_json::from_str::<Severity>("\"Critical\"").unwrap(),
        Severity::Critical
    );
    assert_eq!(
        serde_json::from_str::<Severity>("\"High\"").unwrap(),
        Severity::High
    );
    assert_eq!(
        serde_json::from_str::<Severity>("\"Medium\"").unwrap(),
        Severity::Medium
    );
    assert_eq!(
        serde_json::from_str::<Severity>("\"Low\"").unwrap(),
        Severity::Low
    );
    assert_eq!(
        serde_json::from_str::<Severity>("\"Info\"").unwrap(),
        Severity::Info
    );
}

#[test]
fn test_vulnerability_type_serialize() {
    assert_eq!(
        serde_json::to_string(&VulnerabilityType::Reentrancy).unwrap(),
        "\"Reentrancy\""
    );
    assert_eq!(
        serde_json::to_string(&VulnerabilityType::AccessControl).unwrap(),
        "\"AccessControl\""
    );
    assert_eq!(
        serde_json::to_string(&VulnerabilityType::FrontrunMev).unwrap(),
        "\"FrontrunMev\""
    );
}

#[test]
fn test_vulnerability_type_deserialize() {
    // LLMs are instructed to return exact PascalCase in prompts
    assert_eq!(
        serde_json::from_str::<VulnerabilityType>("\"Reentrancy\"").unwrap(),
        VulnerabilityType::Reentrancy
    );
    assert_eq!(
        serde_json::from_str::<VulnerabilityType>("\"AccessControl\"").unwrap(),
        VulnerabilityType::AccessControl
    );
    assert_eq!(
        serde_json::from_str::<VulnerabilityType>("\"FrontrunMev\"").unwrap(),
        VulnerabilityType::FrontrunMev
    );
}

#[test]
fn test_vulnerability_type_new_variants() {
    // Test new ERC-specific variants (exact PascalCase as LLMs are instructed)
    assert_eq!(
        serde_json::from_str::<VulnerabilityType>("\"StandardViolation\"").unwrap(),
        VulnerabilityType::StandardViolation
    );
    assert_eq!(
        serde_json::from_str::<VulnerabilityType>("\"AllowanceRace\"").unwrap(),
        VulnerabilityType::AllowanceRace
    );
    assert_eq!(
        serde_json::from_str::<VulnerabilityType>("\"PermitDomainSeparator\"").unwrap(),
        VulnerabilityType::PermitDomainSeparator
    );
    assert_eq!(
        serde_json::from_str::<VulnerabilityType>("\"ERC4626SharePrice\"").unwrap(),
        VulnerabilityType::ERC4626SharePrice
    );
}

#[test]
fn test_display_trait() {
    // Test that strum's Display trait works
    assert_eq!(Severity::Critical.to_string(), "Critical");
    assert_eq!(Severity::High.to_string(), "High");
    assert_eq!(VulnerabilityType::Reentrancy.to_string(), "Reentrancy");
    assert_eq!(
        VulnerabilityType::AccessControl.to_string(),
        "AccessControl"
    );
}

#[test]
fn test_from_str() {
    use std::str::FromStr;

    // Test strum's EnumString (FromStr) trait
    assert_eq!(Severity::from_str("Critical").unwrap(), Severity::Critical);
    assert_eq!(Severity::from_str("High").unwrap(), Severity::High);

    assert_eq!(
        VulnerabilityType::from_str("Reentrancy").unwrap(),
        VulnerabilityType::Reentrancy
    );
    assert_eq!(
        VulnerabilityType::from_str("AccessControl").unwrap(),
        VulnerabilityType::AccessControl
    );
}
