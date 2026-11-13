pub use crate::prelude::*;

/// `black` or `white`, whichever reads against the accent color.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountCustomerTrackingButtonTextColor {
    Black,
    White,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountCustomerTrackingButtonTextColor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Black => serializer.serialize_str("black"),
            Self::White => serializer.serialize_str("white"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountCustomerTrackingButtonTextColor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "black" => Ok(Self::Black),
            "white" => Ok(Self::White),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountCustomerTrackingButtonTextColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Black => write!(f, "black"),
            Self::White => write!(f, "white"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
