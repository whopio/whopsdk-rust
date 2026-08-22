pub use crate::prelude::*;

/// The window the limit amount applies to. `per_transaction` caps each individual authorization and is what a limit set with `transaction_limit` reports.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrieveCardsResponseLimitFrequency {
    Daily,
    Weekly,
    Monthly,
    OneTime,
    PerTransaction,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrieveCardsResponseLimitFrequency {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Daily => serializer.serialize_str("daily"),
            Self::Weekly => serializer.serialize_str("weekly"),
            Self::Monthly => serializer.serialize_str("monthly"),
            Self::OneTime => serializer.serialize_str("one_time"),
            Self::PerTransaction => serializer.serialize_str("per_transaction"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrieveCardsResponseLimitFrequency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "daily" => Ok(Self::Daily),
            "weekly" => Ok(Self::Weekly),
            "monthly" => Ok(Self::Monthly),
            "one_time" => Ok(Self::OneTime),
            "per_transaction" => Ok(Self::PerTransaction),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrieveCardsResponseLimitFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Daily => write!(f, "daily"),
            Self::Weekly => write!(f, "weekly"),
            Self::Monthly => write!(f, "monthly"),
            Self::OneTime => write!(f, "one_time"),
            Self::PerTransaction => write!(f, "per_transaction"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
