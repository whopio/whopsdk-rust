pub use crate::prelude::*;

/// How the reward is delivered.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OnboardingRewardRewardType {
    AdCredit,
    BalanceCredit,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OnboardingRewardRewardType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AdCredit => serializer.serialize_str("ad_credit"),
            Self::BalanceCredit => serializer.serialize_str("balance_credit"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OnboardingRewardRewardType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ad_credit" => Ok(Self::AdCredit),
            "balance_credit" => Ok(Self::BalanceCredit),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OnboardingRewardRewardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdCredit => write!(f, "ad_credit"),
            Self::BalanceCredit => write!(f, "balance_credit"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
