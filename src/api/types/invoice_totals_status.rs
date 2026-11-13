pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvoiceTotalsStatus {
    Estimated,
    Final,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InvoiceTotalsStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Estimated => serializer.serialize_str("estimated"),
            Self::Final => serializer.serialize_str("final"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InvoiceTotalsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "estimated" => Ok(Self::Estimated),
            "final" => Ok(Self::Final),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InvoiceTotalsStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Estimated => write!(f, "estimated"),
            Self::Final => write!(f, "final"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
