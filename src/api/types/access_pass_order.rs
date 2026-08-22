pub use crate::prelude::*;

/// The ways a relation of AccessPasses can be ordered
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccessPassOrder {
    ActiveMembershipsCount,
    CreatedAt,
    UsdGmv,
    UsdGmv30Days,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccessPassOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ActiveMembershipsCount => serializer.serialize_str("active_memberships_count"),
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::UsdGmv => serializer.serialize_str("usd_gmv"),
            Self::UsdGmv30Days => serializer.serialize_str("usd_gmv_30_days"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccessPassOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active_memberships_count" => Ok(Self::ActiveMembershipsCount),
            "created_at" => Ok(Self::CreatedAt),
            "usd_gmv" => Ok(Self::UsdGmv),
            "usd_gmv_30_days" => Ok(Self::UsdGmv30Days),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccessPassOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ActiveMembershipsCount => write!(f, "active_memberships_count"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::UsdGmv => write!(f, "usd_gmv"),
            Self::UsdGmv30Days => write!(f, "usd_gmv_30_days"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
