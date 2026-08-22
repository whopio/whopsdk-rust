pub use crate::prelude::*;

/// Who bore the payout fee: the account itself, or its parent platform.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostWithdrawalUpdatedPayloadDataFeePaidBy {
    Self_,
    Platform,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostWithdrawalUpdatedPayloadDataFeePaidBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Self_ => serializer.serialize_str("self"),
            Self::Platform => serializer.serialize_str("platform"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostWithdrawalUpdatedPayloadDataFeePaidBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "self" => Ok(Self::Self_),
            "platform" => Ok(Self::Platform),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostWithdrawalUpdatedPayloadDataFeePaidBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Self_ => write!(f, "self"),
            Self::Platform => write!(f, "platform"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
