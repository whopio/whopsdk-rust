pub use crate::prelude::*;

/// `cross` shares margin across positions. `isolated` limits margin to this position.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradingPositionLeverageType {
    Cross,
    Isolated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TradingPositionLeverageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Cross => serializer.serialize_str("cross"),
            Self::Isolated => serializer.serialize_str("isolated"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TradingPositionLeverageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "cross" => Ok(Self::Cross),
            "isolated" => Ok(Self::Isolated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TradingPositionLeverageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cross => write!(f, "cross"),
            Self::Isolated => write!(f, "isolated"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
