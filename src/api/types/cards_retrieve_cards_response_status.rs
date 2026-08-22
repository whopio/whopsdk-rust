pub use crate::prelude::*;

/// The card status. `denied` means the issuer declined the cardholder, so the card will never be issued.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrieveCardsResponseStatus {
    Active,
    Frozen,
    Canceled,
    Invited,
    Denied,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrieveCardsResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Frozen => serializer.serialize_str("frozen"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Invited => serializer.serialize_str("invited"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrieveCardsResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "frozen" => Ok(Self::Frozen),
            "canceled" => Ok(Self::Canceled),
            "invited" => Ok(Self::Invited),
            "denied" => Ok(Self::Denied),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrieveCardsResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Frozen => write!(f, "frozen"),
            Self::Canceled => write!(f, "canceled"),
            Self::Invited => write!(f, "invited"),
            Self::Denied => write!(f, "denied"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
