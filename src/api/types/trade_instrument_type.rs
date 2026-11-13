pub use crate::prelude::*;

/// `perpetual` or `spot` for an order, `perpetual` for a close. `null` for a deposit or withdraw.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradeInstrumentType {
    Perpetual,
    Spot,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TradeInstrumentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Perpetual => serializer.serialize_str("perpetual"),
            Self::Spot => serializer.serialize_str("spot"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TradeInstrumentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "perpetual" => Ok(Self::Perpetual),
            "spot" => Ok(Self::Spot),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TradeInstrumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Perpetual => write!(f, "perpetual"),
            Self::Spot => write!(f, "spot"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
