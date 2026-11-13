pub use crate::prelude::*;

/// Whether tax is added on top of the price (`exclusive`) or already inside it (`inclusive`). Null when no tax was calculated.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentQuoteTaxBehavior {
    Inclusive,
    Exclusive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentQuoteTaxBehavior {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Inclusive => serializer.serialize_str("inclusive"),
            Self::Exclusive => serializer.serialize_str("exclusive"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentQuoteTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "inclusive" => Ok(Self::Inclusive),
            "exclusive" => Ok(Self::Exclusive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentQuoteTaxBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inclusive => write!(f, "inclusive"),
            Self::Exclusive => write!(f, "exclusive"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
