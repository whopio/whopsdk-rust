pub use crate::prelude::*;

/// Lifecycle trust state: `checking` (verification still running), `verified` (bank confirmed ownership or a payout already completed to it), `no_data` (verification unavailable or bank returned no ownership data), `warning` (bank could not confirm the destination's owner), `broken` (payouts failed with a permanent account error), `null` (never checked).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostPayoutMethodCreatedPayloadDataBankVerificationState {
    Checking,
    Verified,
    NoData,
    Warning,
    Broken,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostPayoutMethodCreatedPayloadDataBankVerificationState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Checking => serializer.serialize_str("checking"),
            Self::Verified => serializer.serialize_str("verified"),
            Self::NoData => serializer.serialize_str("no_data"),
            Self::Warning => serializer.serialize_str("warning"),
            Self::Broken => serializer.serialize_str("broken"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostPayoutMethodCreatedPayloadDataBankVerificationState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "checking" => Ok(Self::Checking),
            "verified" => Ok(Self::Verified),
            "no_data" => Ok(Self::NoData),
            "warning" => Ok(Self::Warning),
            "broken" => Ok(Self::Broken),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostPayoutMethodCreatedPayloadDataBankVerificationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Checking => write!(f, "checking"),
            Self::Verified => write!(f, "verified"),
            Self::NoData => write!(f, "no_data"),
            Self::Warning => write!(f, "warning"),
            Self::Broken => write!(f, "broken"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
