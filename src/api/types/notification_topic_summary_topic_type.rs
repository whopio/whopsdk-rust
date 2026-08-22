pub use crate::prelude::*;

/// Scope of the topic: whether it applies to an account, a user, or an account's team.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationTopicSummaryTopicType {
    Account,
    User,
    AccountTeam,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NotificationTopicSummaryTopicType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Account => serializer.serialize_str("account"),
            Self::User => serializer.serialize_str("user"),
            Self::AccountTeam => serializer.serialize_str("account_team"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NotificationTopicSummaryTopicType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "account" => Ok(Self::Account),
            "user" => Ok(Self::User),
            "account_team" => Ok(Self::AccountTeam),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NotificationTopicSummaryTopicType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Account => write!(f, "account"),
            Self::User => write!(f, "user"),
            Self::AccountTeam => write!(f, "account_team"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
