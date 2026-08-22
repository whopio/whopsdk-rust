pub use crate::prelude::*;

/// Which columns can be used to sort.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntriesSortableColumns {
    Id,
    CreatedAt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EntriesSortableColumns {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Id => serializer.serialize_str("id"),
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EntriesSortableColumns {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "id" => Ok(Self::Id),
            "created_at" => Ok(Self::CreatedAt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EntriesSortableColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
