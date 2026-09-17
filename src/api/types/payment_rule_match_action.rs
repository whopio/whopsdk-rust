pub use crate::prelude::*;

/// What the rule asked for.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentRuleMatchAction {
    Allow,
    Block,
    Review,
    Enforce3Ds,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentRuleMatchAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Allow => serializer.serialize_str("allow"),
            Self::Block => serializer.serialize_str("block"),
            Self::Review => serializer.serialize_str("review"),
            Self::Enforce3Ds => serializer.serialize_str("enforce_3ds"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentRuleMatchAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "allow" => Ok(Self::Allow),
            "block" => Ok(Self::Block),
            "review" => Ok(Self::Review),
            "enforce_3ds" => Ok(Self::Enforce3Ds),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentRuleMatchAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allow => write!(f, "allow"),
            Self::Block => write!(f, "block"),
            Self::Review => write!(f, "review"),
            Self::Enforce3Ds => write!(f, "enforce_3ds"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
