pub use crate::prelude::*;

/// Whether Whop calculated tax for this preview. `not_calculated` means no tax could be determined, so `tax_amount` is 0 and `total` equals `subtotal`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CalculateTaxPlansResponseStatus {
    Calculated,
    NotCalculated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CalculateTaxPlansResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Calculated => serializer.serialize_str("calculated"),
            Self::NotCalculated => serializer.serialize_str("not_calculated"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CalculateTaxPlansResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "calculated" => Ok(Self::Calculated),
            "not_calculated" => Ok(Self::NotCalculated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CalculateTaxPlansResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Calculated => write!(f, "calculated"),
            Self::NotCalculated => write!(f, "not_calculated"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
