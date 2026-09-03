pub use crate::prelude::*;

/// Whether the client should navigate to the CTA, open the programmatic execution dialog, or run the CTA as a Whop AI prompt
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountRecommendedActionChainStepExecutionType {
    Redirect,
    Programatic,
    WhopAi,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountRecommendedActionChainStepExecutionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Redirect => serializer.serialize_str("redirect"),
            Self::Programatic => serializer.serialize_str("programatic"),
            Self::WhopAi => serializer.serialize_str("whop_ai"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountRecommendedActionChainStepExecutionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "redirect" => Ok(Self::Redirect),
            "programatic" => Ok(Self::Programatic),
            "whop_ai" => Ok(Self::WhopAi),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountRecommendedActionChainStepExecutionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Redirect => write!(f, "redirect"),
            Self::Programatic => write!(f, "programatic"),
            Self::WhopAi => write!(f, "whop_ai"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
