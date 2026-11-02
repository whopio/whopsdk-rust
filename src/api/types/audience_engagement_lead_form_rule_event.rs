pub use crate::prelude::*;

/// Interaction that qualifies a person for this rule.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudienceEngagementLeadFormRuleEvent {
    Opened,
    Submitted,
    NotSubmitted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AudienceEngagementLeadFormRuleEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Opened => serializer.serialize_str("opened"),
            Self::Submitted => serializer.serialize_str("submitted"),
            Self::NotSubmitted => serializer.serialize_str("not_submitted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AudienceEngagementLeadFormRuleEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "opened" => Ok(Self::Opened),
            "submitted" => Ok(Self::Submitted),
            "not_submitted" => Ok(Self::NotSubmitted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AudienceEngagementLeadFormRuleEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Opened => write!(f, "opened"),
            Self::Submitted => write!(f, "submitted"),
            Self::NotSubmitted => write!(f, "not_submitted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
