pub use crate::prelude::*;

/// The type of token transaction
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompanyTokenTransactionTypes {
    Add,
    Subtract,
    Transfer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CompanyTokenTransactionTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Add => serializer.serialize_str("add"),
            Self::Subtract => serializer.serialize_str("subtract"),
            Self::Transfer => serializer.serialize_str("transfer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CompanyTokenTransactionTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "add" => Ok(Self::Add),
            "subtract" => Ok(Self::Subtract),
            "transfer" => Ok(Self::Transfer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CompanyTokenTransactionTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "add"),
            Self::Subtract => write!(f, "subtract"),
            Self::Transfer => write!(f, "transfer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
