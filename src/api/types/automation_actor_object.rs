pub use crate::prelude::*;

/// What acted: a `user`, an `account` through one of its API keys, or an `app`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutomationActorObject {
    User,
    Account,
    App,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutomationActorObject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::User => serializer.serialize_str("user"),
            Self::Account => serializer.serialize_str("account"),
            Self::App => serializer.serialize_str("app"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutomationActorObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user" => Ok(Self::User),
            "account" => Ok(Self::Account),
            "app" => Ok(Self::App),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AutomationActorObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Account => write!(f, "account"),
            Self::App => write!(f, "app"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
