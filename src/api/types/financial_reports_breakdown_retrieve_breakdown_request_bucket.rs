pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrieveBreakdownRequestBucket {
    Transfers,
    Payments,
    Refunds,
    Ads,
    ApplicationFees,
    CardSpend,
    CardAuthorizationReleases,
    Withdrawals,
    Swaps,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrieveBreakdownRequestBucket {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Transfers => serializer.serialize_str("transfers"),
            Self::Payments => serializer.serialize_str("payments"),
            Self::Refunds => serializer.serialize_str("refunds"),
            Self::Ads => serializer.serialize_str("ads"),
            Self::ApplicationFees => serializer.serialize_str("application_fees"),
            Self::CardSpend => serializer.serialize_str("card_spend"),
            Self::CardAuthorizationReleases => {
                serializer.serialize_str("card_authorization_releases")
            }
            Self::Withdrawals => serializer.serialize_str("withdrawals"),
            Self::Swaps => serializer.serialize_str("swaps"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrieveBreakdownRequestBucket {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "transfers" => Ok(Self::Transfers),
            "payments" => Ok(Self::Payments),
            "refunds" => Ok(Self::Refunds),
            "ads" => Ok(Self::Ads),
            "application_fees" => Ok(Self::ApplicationFees),
            "card_spend" => Ok(Self::CardSpend),
            "card_authorization_releases" => Ok(Self::CardAuthorizationReleases),
            "withdrawals" => Ok(Self::Withdrawals),
            "swaps" => Ok(Self::Swaps),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrieveBreakdownRequestBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transfers => write!(f, "transfers"),
            Self::Payments => write!(f, "payments"),
            Self::Refunds => write!(f, "refunds"),
            Self::Ads => write!(f, "ads"),
            Self::ApplicationFees => write!(f, "application_fees"),
            Self::CardSpend => write!(f, "card_spend"),
            Self::CardAuthorizationReleases => write!(f, "card_authorization_releases"),
            Self::Withdrawals => write!(f, "withdrawals"),
            Self::Swaps => write!(f, "swaps"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
