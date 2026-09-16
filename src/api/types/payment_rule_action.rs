pub use crate::prelude::*;

/// What happens to a payment when every condition matches. An `allow` overrides this account's other rules only, never Whop's own fraud controls. An `enforce_3ds` is skipped where the payment cannot carry a challenge.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentRuleAction {
    Allow,
    Block,
    Enforce3Ds,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentRuleAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Allow => serializer.serialize_str("allow"),
            Self::Block => serializer.serialize_str("block"),
            Self::Enforce3Ds => serializer.serialize_str("enforce_3ds"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentRuleAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "allow" => Ok(Self::Allow),
            "block" => Ok(Self::Block),
            "enforce_3ds" => Ok(Self::Enforce3Ds),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentRuleAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allow => write!(f, "allow"),
            Self::Block => write!(f, "block"),
            Self::Enforce3Ds => write!(f, "enforce_3ds"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
