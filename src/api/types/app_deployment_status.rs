pub use crate::prelude::*;

/// Whether the app has anything to publish, and what a publish in flight is doing. `unpublished` means publishing would ship something new; `no_source` means the sandbox holds no copy of this app, so there is nothing to publish from.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppDeploymentStatus {
    Published,
    Unpublished,
    Publishing,
    Failed,
    NoSource,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AppDeploymentStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Published => serializer.serialize_str("published"),
            Self::Unpublished => serializer.serialize_str("unpublished"),
            Self::Publishing => serializer.serialize_str("publishing"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::NoSource => serializer.serialize_str("no_source"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AppDeploymentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "published" => Ok(Self::Published),
            "unpublished" => Ok(Self::Unpublished),
            "publishing" => Ok(Self::Publishing),
            "failed" => Ok(Self::Failed),
            "no_source" => Ok(Self::NoSource),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AppDeploymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Published => write!(f, "published"),
            Self::Unpublished => write!(f, "unpublished"),
            Self::Publishing => write!(f, "publishing"),
            Self::Failed => write!(f, "failed"),
            Self::NoSource => write!(f, "no_source"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
