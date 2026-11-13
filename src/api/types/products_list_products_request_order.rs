pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListProductsRequestOrder {
    CreatedAt,
    ActiveMembershipsCount,
    UsdGmv,
    UsdGmv30Days,
    Position,
    DiscoverableAt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListProductsRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::ActiveMembershipsCount => serializer.serialize_str("active_memberships_count"),
            Self::UsdGmv => serializer.serialize_str("usd_gmv"),
            Self::UsdGmv30Days => serializer.serialize_str("usd_gmv_30_days"),
            Self::Position => serializer.serialize_str("position"),
            Self::DiscoverableAt => serializer.serialize_str("discoverable_at"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListProductsRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "active_memberships_count" => Ok(Self::ActiveMembershipsCount),
            "usd_gmv" => Ok(Self::UsdGmv),
            "usd_gmv_30_days" => Ok(Self::UsdGmv30Days),
            "position" => Ok(Self::Position),
            "discoverable_at" => Ok(Self::DiscoverableAt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListProductsRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::ActiveMembershipsCount => write!(f, "active_memberships_count"),
            Self::UsdGmv => write!(f, "usd_gmv"),
            Self::UsdGmv30Days => write!(f, "usd_gmv_30_days"),
            Self::Position => write!(f, "position"),
            Self::DiscoverableAt => write!(f, "discoverable_at"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
