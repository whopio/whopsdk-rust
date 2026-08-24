pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreatePromoCodesRequestPromoType {
    Percentage,
    FlatAmount,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreatePromoCodesRequestPromoType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Percentage => serializer.serialize_str("percentage"),
            Self::FlatAmount => serializer.serialize_str("flat_amount"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreatePromoCodesRequestPromoType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "percentage" => Ok(Self::Percentage),
            "flat_amount" => Ok(Self::FlatAmount),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreatePromoCodesRequestPromoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Percentage => write!(f, "percentage"),
            Self::FlatAmount => write!(f, "flat_amount"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
