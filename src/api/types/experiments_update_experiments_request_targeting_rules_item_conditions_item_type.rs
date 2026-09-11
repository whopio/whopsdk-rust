pub use crate::prelude::*;

/// `user_id` — match on subject user ID; `account_id` — match on account ID (prefixed `biz_`); `property` — match on a named user property (requires `field`).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateExperimentsRequestTargetingRulesItemConditionsItemType {
    UserId,
    AccountId,
    Property,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateExperimentsRequestTargetingRulesItemConditionsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UserId => serializer.serialize_str("user_id"),
            Self::AccountId => serializer.serialize_str("account_id"),
            Self::Property => serializer.serialize_str("property"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateExperimentsRequestTargetingRulesItemConditionsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user_id" => Ok(Self::UserId),
            "account_id" => Ok(Self::AccountId),
            "property" => Ok(Self::Property),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateExperimentsRequestTargetingRulesItemConditionsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserId => write!(f, "user_id"),
            Self::AccountId => write!(f, "account_id"),
            Self::Property => write!(f, "property"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
