pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAdGroupsRequestMessageAppsItem {
    Messenger,
    Instagram,
    Whatsapp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAdGroupsRequestMessageAppsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Messenger => serializer.serialize_str("messenger"),
            Self::Instagram => serializer.serialize_str("instagram"),
            Self::Whatsapp => serializer.serialize_str("whatsapp"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAdGroupsRequestMessageAppsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "messenger" => Ok(Self::Messenger),
            "instagram" => Ok(Self::Instagram),
            "whatsapp" => Ok(Self::Whatsapp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAdGroupsRequestMessageAppsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Messenger => write!(f, "messenger"),
            Self::Instagram => write!(f, "instagram"),
            Self::Whatsapp => write!(f, "whatsapp"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
