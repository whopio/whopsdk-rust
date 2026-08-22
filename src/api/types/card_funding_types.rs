pub use crate::prelude::*;

/// The funding types of a card
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardFundingTypes {
    Credit,
    Debit,
    Prepaid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CardFundingTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Credit => serializer.serialize_str("credit"),
            Self::Debit => serializer.serialize_str("debit"),
            Self::Prepaid => serializer.serialize_str("prepaid"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CardFundingTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "credit" => Ok(Self::Credit),
            "debit" => Ok(Self::Debit),
            "prepaid" => Ok(Self::Prepaid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CardFundingTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Credit => write!(f, "credit"),
            Self::Debit => write!(f, "debit"),
            Self::Prepaid => write!(f, "prepaid"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
