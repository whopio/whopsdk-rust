pub use crate::prelude::*;

/// Randomization unit — `user` buckets each user independently, `account` buckets whole accounts (every user of an account gets the same arm). `null` for feature flags.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExperimentBucketBy {
    User,
    Account,
    Anonymous,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExperimentBucketBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::User => serializer.serialize_str("user"),
            Self::Account => serializer.serialize_str("account"),
            Self::Anonymous => serializer.serialize_str("anonymous"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExperimentBucketBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user" => Ok(Self::User),
            "account" => Ok(Self::Account),
            "anonymous" => Ok(Self::Anonymous),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExperimentBucketBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Account => write!(f, "account"),
            Self::Anonymous => write!(f, "anonymous"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
