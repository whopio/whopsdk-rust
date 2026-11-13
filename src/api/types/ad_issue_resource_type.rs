pub use crate::prelude::*;

/// The type of object the issue is on.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdIssueResourceType {
    Ad,
    AdGroup,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdIssueResourceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ad => serializer.serialize_str("ad"),
            Self::AdGroup => serializer.serialize_str("ad_group"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdIssueResourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ad" => Ok(Self::Ad),
            "ad_group" => Ok(Self::AdGroup),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdIssueResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ad => write!(f, "ad"),
            Self::AdGroup => write!(f, "ad_group"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
