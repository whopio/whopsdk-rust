pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReferredUsersPartnersRequestOrder {
    CreatedAt,
    EarningsUsd,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReferredUsersPartnersRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::EarningsUsd => serializer.serialize_str("earnings_usd"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReferredUsersPartnersRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "earnings_usd" => Ok(Self::EarningsUsd),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReferredUsersPartnersRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::EarningsUsd => write!(f, "earnings_usd"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
