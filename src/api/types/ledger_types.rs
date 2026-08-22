pub use crate::prelude::*;

/// The types of ledgers that can be created.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LedgerTypes {
    Primary,
    Pool,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LedgerTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Primary => serializer.serialize_str("primary"),
            Self::Pool => serializer.serialize_str("pool"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LedgerTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "primary" => Ok(Self::Primary),
            "pool" => Ok(Self::Pool),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LedgerTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primary => write!(f, "primary"),
            Self::Pool => write!(f, "pool"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
