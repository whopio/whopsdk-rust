pub use crate::prelude::*;

/// `added` counts toward the total; `included` is already inside the lines and is shown for disclosure only (inclusive-tax markets).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionBreakdownAdjustmentApplied {
    Added,
    Included,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionBreakdownAdjustmentApplied {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Added => serializer.serialize_str("added"),
            Self::Included => serializer.serialize_str("included"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionBreakdownAdjustmentApplied {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "added" => Ok(Self::Added),
            "included" => Ok(Self::Included),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionBreakdownAdjustmentApplied {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Added => write!(f, "added"),
            Self::Included => write!(f, "included"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
