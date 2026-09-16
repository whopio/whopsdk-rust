pub use crate::prelude::*;

/// The type of value this field compares against.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentRuleFieldType {
    Integer,
    String_,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentRuleFieldType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Integer => serializer.serialize_str("integer"),
            Self::String_ => serializer.serialize_str("string"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentRuleFieldType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "integer" => Ok(Self::Integer),
            "string" => Ok(Self::String_),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentRuleFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer => write!(f, "integer"),
            Self::String_ => write!(f, "string"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
