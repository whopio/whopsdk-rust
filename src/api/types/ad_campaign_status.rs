pub use crate::prelude::*;

/// The lifecycle status of the ad campaign.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdCampaignStatus {
    Active,
    Paused,
    Inactive,
    Stale,
    PendingRefund,
    PaymentFailed,
    Draft,
    InReview,
    Flagged,
    Importing,
    Imported,
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
            Self::Stale => serializer.serialize_str("stale"),
            Self::PendingRefund => serializer.serialize_str("pending_refund"),
            Self::PaymentFailed => serializer.serialize_str("payment_failed"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::InReview => serializer.serialize_str("in_review"),
            Self::Flagged => serializer.serialize_str("flagged"),
            Self::Importing => serializer.serialize_str("importing"),
            Self::Imported => serializer.serialize_str("imported"),
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
            "stale" => Ok(Self::Stale),
            "pending_refund" => Ok(Self::PendingRefund),
            "payment_failed" => Ok(Self::PaymentFailed),
            "draft" => Ok(Self::Draft),
            "in_review" => Ok(Self::InReview),
            "flagged" => Ok(Self::Flagged),
            "importing" => Ok(Self::Importing),
            "imported" => Ok(Self::Imported),
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
            Self::Stale => write!(f, "stale"),
            Self::PendingRefund => write!(f, "pending_refund"),
            Self::PaymentFailed => write!(f, "payment_failed"),
            Self::Draft => write!(f, "draft"),
            Self::InReview => write!(f, "in_review"),
            Self::Flagged => write!(f, "flagged"),
            Self::Importing => write!(f, "importing"),
            Self::Imported => write!(f, "imported"),
            Self::Duplicating => write!(f, "duplicating"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
