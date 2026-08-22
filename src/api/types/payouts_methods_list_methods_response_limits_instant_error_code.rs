pub use crate::prelude::*;

/// Why instant payouts are unavailable for this account, or null when they are available.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMethodsResponseLimitsInstantErrorCode {
    AmountTooHigh,
    AccountDailyLimitReached,
    RestrictedAccount,
    FeatureDisabled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListMethodsResponseLimitsInstantErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AmountTooHigh => serializer.serialize_str("amount_too_high"),
            Self::AccountDailyLimitReached => {
                serializer.serialize_str("account_daily_limit_reached")
            }
            Self::RestrictedAccount => serializer.serialize_str("restricted_account"),
            Self::FeatureDisabled => serializer.serialize_str("feature_disabled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListMethodsResponseLimitsInstantErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "amount_too_high" => Ok(Self::AmountTooHigh),
            "account_daily_limit_reached" => Ok(Self::AccountDailyLimitReached),
            "restricted_account" => Ok(Self::RestrictedAccount),
            "feature_disabled" => Ok(Self::FeatureDisabled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListMethodsResponseLimitsInstantErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AmountTooHigh => write!(f, "amount_too_high"),
            Self::AccountDailyLimitReached => write!(f, "account_daily_limit_reached"),
            Self::RestrictedAccount => write!(f, "restricted_account"),
            Self::FeatureDisabled => write!(f, "feature_disabled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
