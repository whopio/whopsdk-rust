pub use crate::prelude::*;

/// Whether this checkout collects tax, and how its price reads when it does. `null` means no tax is collected here — skip `calculate_tax` entirely and ask for no more address than the payment method itself needs. `exclusive` means tax is ADDED to the quote, `inclusive` that the quote already contains it. Present before any address is known, so a surface can decide what to collect up front; the value is what this checkout expects to price with, and `calculate_tax` answers with the authoritative one once a location is known (tax behaviour varies by country).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionTaxBehavior {
    Inclusive,
    Exclusive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionTaxBehavior {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Inclusive => serializer.serialize_str("inclusive"),
            Self::Exclusive => serializer.serialize_str("exclusive"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "inclusive" => Ok(Self::Inclusive),
            "exclusive" => Ok(Self::Exclusive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionTaxBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inclusive => write!(f, "inclusive"),
            Self::Exclusive => write!(f, "exclusive"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
