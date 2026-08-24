pub use crate::prelude::*;

/// Whether or not the tax is included in a plan's price (or if it hasn't been set up)
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaxTypes {
    Inclusive,
    Exclusive,
    Unspecified,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaxTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Inclusive => serializer.serialize_str("inclusive"),
            Self::Exclusive => serializer.serialize_str("exclusive"),
            Self::Unspecified => serializer.serialize_str("unspecified"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaxTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "inclusive" => Ok(Self::Inclusive),
            "exclusive" => Ok(Self::Exclusive),
            "unspecified" => Ok(Self::Unspecified),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaxTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inclusive => write!(f, "inclusive"),
            Self::Exclusive => write!(f, "exclusive"),
            Self::Unspecified => write!(f, "unspecified"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
