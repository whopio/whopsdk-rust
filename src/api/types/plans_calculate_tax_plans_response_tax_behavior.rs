pub use crate::prelude::*;

/// Whether tax is added on top of the plan price or already included in it for this buyer's location.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CalculateTaxPlansResponseTaxBehavior {
    Exclusive,
    Inclusive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CalculateTaxPlansResponseTaxBehavior {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Exclusive => serializer.serialize_str("exclusive"),
            Self::Inclusive => serializer.serialize_str("inclusive"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CalculateTaxPlansResponseTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CalculateTaxPlansResponseTaxBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exclusive => write!(f, "exclusive"),
            Self::Inclusive => write!(f, "inclusive"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
