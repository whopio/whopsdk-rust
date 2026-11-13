pub use crate::prelude::*;

/// The configured lifecycle status of the ad campaign. Billing failures preserve active or paused here and set delivery_status to payment_failed.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdCampaignStatus {
    Active,
    Paused,
    Inactive,
    PaymentFailed,
    Draft,
    Duplicating,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdCampaignStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::Inactive => serializer.serialize_str("inactive"),
            Self::PaymentFailed => serializer.serialize_str("payment_failed"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::Duplicating => serializer.serialize_str("duplicating"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdCampaignStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "paused" => Ok(Self::Paused),
            "inactive" => Ok(Self::Inactive),
            "payment_failed" => Ok(Self::PaymentFailed),
            "draft" => Ok(Self::Draft),
            "duplicating" => Ok(Self::Duplicating),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdCampaignStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Paused => write!(f, "paused"),
            Self::Inactive => write!(f, "inactive"),
            Self::PaymentFailed => write!(f, "payment_failed"),
            Self::Draft => write!(f, "draft"),
            Self::Duplicating => write!(f, "duplicating"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
