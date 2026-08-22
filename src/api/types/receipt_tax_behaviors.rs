pub use crate::prelude::*;

/// The type of tax inclusivity applied to the receipt, for determining whether the tax is included in the final price, or paid on top.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReceiptTaxBehaviors {
    Exclusive,
    Inclusive,
    Unspecified,
    UnableToCollect,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReceiptTaxBehaviors {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Exclusive => serializer.serialize_str("exclusive"),
            Self::Inclusive => serializer.serialize_str("inclusive"),
            Self::Unspecified => serializer.serialize_str("unspecified"),
            Self::UnableToCollect => serializer.serialize_str("unable_to_collect"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReceiptTaxBehaviors {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),
            "unable_to_collect" => Ok(Self::UnableToCollect),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReceiptTaxBehaviors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exclusive => write!(f, "exclusive"),
            Self::Inclusive => write!(f, "inclusive"),
            Self::Unspecified => write!(f, "unspecified"),
            Self::UnableToCollect => write!(f, "unable_to_collect"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
