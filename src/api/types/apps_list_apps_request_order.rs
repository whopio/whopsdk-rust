pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListAppsRequestOrder {
    CreatedAt,
    DiscoverableAt,
    TemplateUsage,
    TotalInstallsLast30Days,
    TotalInstallsLast7Days,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListAppsRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::DiscoverableAt => serializer.serialize_str("discoverable_at"),
            Self::TemplateUsage => serializer.serialize_str("template_usage"),
            Self::TotalInstallsLast30Days => {
                serializer.serialize_str("total_installs_last_30_days")
            }
            Self::TotalInstallsLast7Days => serializer.serialize_str("total_installs_last_7_days"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListAppsRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "discoverable_at" => Ok(Self::DiscoverableAt),
            "template_usage" => Ok(Self::TemplateUsage),
            "total_installs_last_30_days" => Ok(Self::TotalInstallsLast30Days),
            "total_installs_last_7_days" => Ok(Self::TotalInstallsLast7Days),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListAppsRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::DiscoverableAt => write!(f, "discoverable_at"),
            Self::TemplateUsage => write!(f, "template_usage"),
            Self::TotalInstallsLast30Days => write!(f, "total_installs_last_30_days"),
            Self::TotalInstallsLast7Days => write!(f, "total_installs_last_7_days"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
