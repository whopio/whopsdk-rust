pub use crate::prelude::*;

/// How often the account's balance automatically withdraws.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountWithdrawalScheduleControlFrequency {
    Manual,
    Daily,
    Weekly,
    Monthly,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountWithdrawalScheduleControlFrequency {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Manual => serializer.serialize_str("manual"),
            Self::Daily => serializer.serialize_str("daily"),
            Self::Weekly => serializer.serialize_str("weekly"),
            Self::Monthly => serializer.serialize_str("monthly"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountWithdrawalScheduleControlFrequency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "manual" => Ok(Self::Manual),
            "daily" => Ok(Self::Daily),
            "weekly" => Ok(Self::Weekly),
            "monthly" => Ok(Self::Monthly),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountWithdrawalScheduleControlFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Daily => write!(f, "daily"),
            Self::Weekly => write!(f, "weekly"),
            Self::Monthly => write!(f, "monthly"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
