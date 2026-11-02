pub use crate::prelude::*;

/// `pending` until it is used, then `consumed`; `expired` once its short lifetime elapses. Only a `pending` token can be charged.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConfirmationTokenStatus {
    Pending,
    Consumed,
    Expired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConfirmationTokenStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Consumed => serializer.serialize_str("consumed"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConfirmationTokenStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "consumed" => Ok(Self::Consumed),
            "expired" => Ok(Self::Expired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConfirmationTokenStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Consumed => write!(f, "consumed"),
            Self::Expired => write!(f, "expired"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
