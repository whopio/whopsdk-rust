pub use crate::prelude::*;

/// The duration setting for the promo code
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PromoDurations {
    Forever,
    Once,
    Repeating,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PromoDurations {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Forever => serializer.serialize_str("forever"),
            Self::Once => serializer.serialize_str("once"),
            Self::Repeating => serializer.serialize_str("repeating"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PromoDurations {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "forever" => Ok(Self::Forever),
            "once" => Ok(Self::Once),
            "repeating" => Ok(Self::Repeating),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PromoDurations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Forever => write!(f, "forever"),
            Self::Once => write!(f, "once"),
            Self::Repeating => write!(f, "repeating"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
