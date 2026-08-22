pub use crate::prelude::*;

/// Layout used on the account store page.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountStorePageConfigLayout {
    Featured,
    Compact,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountStorePageConfigLayout {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Featured => serializer.serialize_str("featured"),
            Self::Compact => serializer.serialize_str("compact"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountStorePageConfigLayout {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "featured" => Ok(Self::Featured),
            "compact" => Ok(Self::Compact),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountStorePageConfigLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Featured => write!(f, "featured"),
            Self::Compact => write!(f, "compact"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
