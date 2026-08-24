pub use crate::prelude::*;

/// Account Open Graph image variant.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountOpengraphImageVariant {
    White,
    Black,
    Orange,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountOpengraphImageVariant {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::White => serializer.serialize_str("white"),
            Self::Black => serializer.serialize_str("black"),
            Self::Orange => serializer.serialize_str("orange"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountOpengraphImageVariant {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "white" => Ok(Self::White),
            "black" => Ok(Self::Black),
            "orange" => Ok(Self::Orange),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountOpengraphImageVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::White => write!(f, "white"),
            Self::Black => write!(f, "black"),
            Self::Orange => write!(f, "orange"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
