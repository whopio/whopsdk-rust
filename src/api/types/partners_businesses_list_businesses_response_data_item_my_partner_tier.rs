pub use crate::prelude::*;

/// Which tier the caller earns on for this business: `first` (they referred the owner) or `second` (they referred the first-tier partner).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListBusinessesResponseDataItemMyPartnerTier {
    First,
    Second,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListBusinessesResponseDataItemMyPartnerTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::First => serializer.serialize_str("first"),
            Self::Second => serializer.serialize_str("second"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListBusinessesResponseDataItemMyPartnerTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "first" => Ok(Self::First),
            "second" => Ok(Self::Second),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListBusinessesResponseDataItemMyPartnerTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::First => write!(f, "first"),
            Self::Second => write!(f, "second"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
