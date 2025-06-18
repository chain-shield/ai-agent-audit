use schemars::JsonSchema;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

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
}

impl InvariantStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            InvariantStatus::HOLDS => "Holds",
            InvariantStatus::VIOLATION => "Violation",
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
        let s: &str = Deserialize::deserialize(deserializer)?;
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
        let s: &str = Deserialize::deserialize(deserializer)?;
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
        let s: &str = Deserialize::deserialize(deserializer)?;
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
