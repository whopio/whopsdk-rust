pub use crate::prelude::*;

/// The available marketplace statuses to choose from.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MarketplaceStatuses {
    NotAvailable,
    PendingReview,
    LiveMarketplace,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MarketplaceStatuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotAvailable => serializer.serialize_str("not_available"),
            Self::PendingReview => serializer.serialize_str("pending_review"),
            Self::LiveMarketplace => serializer.serialize_str("live_marketplace"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MarketplaceStatuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_available" => Ok(Self::NotAvailable),
            "pending_review" => Ok(Self::PendingReview),
            "live_marketplace" => Ok(Self::LiveMarketplace),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MarketplaceStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAvailable => write!(f, "not_available"),
            Self::PendingReview => write!(f, "pending_review"),
            Self::LiveMarketplace => write!(f, "live_marketplace"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
