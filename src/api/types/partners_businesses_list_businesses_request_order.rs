pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListBusinessesRequestOrder {
    CreatedAt,
    ReferralStartedAt,
    ReferralExpiresAt,
    PayoutPercentage,
    VolumeUsd,
    EarningsUsd,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListBusinessesRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::ReferralStartedAt => serializer.serialize_str("referral_started_at"),
            Self::ReferralExpiresAt => serializer.serialize_str("referral_expires_at"),
            Self::PayoutPercentage => serializer.serialize_str("payout_percentage"),
            Self::VolumeUsd => serializer.serialize_str("volume_usd"),
            Self::EarningsUsd => serializer.serialize_str("earnings_usd"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListBusinessesRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "referral_started_at" => Ok(Self::ReferralStartedAt),
            "referral_expires_at" => Ok(Self::ReferralExpiresAt),
            "payout_percentage" => Ok(Self::PayoutPercentage),
            "volume_usd" => Ok(Self::VolumeUsd),
            "earnings_usd" => Ok(Self::EarningsUsd),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListBusinessesRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::ReferralStartedAt => write!(f, "referral_started_at"),
            Self::ReferralExpiresAt => write!(f, "referral_expires_at"),
            Self::PayoutPercentage => write!(f, "payout_percentage"),
            Self::VolumeUsd => write!(f, "volume_usd"),
            Self::EarningsUsd => write!(f, "earnings_usd"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
