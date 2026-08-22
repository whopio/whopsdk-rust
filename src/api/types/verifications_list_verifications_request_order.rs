pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListVerificationsRequestOrder {
    UpdatedAt,
    CreatedAt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListVerificationsRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UpdatedAt => serializer.serialize_str("updated_at"),
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListVerificationsRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "updated_at" => Ok(Self::UpdatedAt),
            "created_at" => Ok(Self::CreatedAt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListVerificationsRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UpdatedAt => write!(f, "updated_at"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
