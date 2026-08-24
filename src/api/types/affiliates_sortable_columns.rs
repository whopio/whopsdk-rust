pub use crate::prelude::*;

/// Which columns can be used to sort.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AffiliatesSortableColumns {
    Id,
    CreatedAt,
    CachedTotalReferrals,
    CachedTotalRewards,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AffiliatesSortableColumns {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Id => serializer.serialize_str("id"),
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::CachedTotalReferrals => serializer.serialize_str("cached_total_referrals"),
            Self::CachedTotalRewards => serializer.serialize_str("cached_total_rewards"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AffiliatesSortableColumns {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "id" => Ok(Self::Id),
            "created_at" => Ok(Self::CreatedAt),
            "cached_total_referrals" => Ok(Self::CachedTotalReferrals),
            "cached_total_rewards" => Ok(Self::CachedTotalRewards),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AffiliatesSortableColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::CachedTotalReferrals => write!(f, "cached_total_referrals"),
            Self::CachedTotalRewards => write!(f, "cached_total_rewards"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
