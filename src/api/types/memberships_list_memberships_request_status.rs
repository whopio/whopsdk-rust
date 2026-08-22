pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMembershipsRequestStatus {
    Active,
    Trialing,
    PastDue,
    Completed,
    Canceled,
    Expired,
    Canceling,
    Paused,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListMembershipsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Trialing => serializer.serialize_str("trialing"),
            Self::PastDue => serializer.serialize_str("past_due"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Canceling => serializer.serialize_str("canceling"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListMembershipsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "trialing" => Ok(Self::Trialing),
            "past_due" => Ok(Self::PastDue),
            "completed" => Ok(Self::Completed),
            "canceled" => Ok(Self::Canceled),
            "expired" => Ok(Self::Expired),
            "canceling" => Ok(Self::Canceling),
            "paused" => Ok(Self::Paused),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListMembershipsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Trialing => write!(f, "trialing"),
            Self::PastDue => write!(f, "past_due"),
            Self::Completed => write!(f, "completed"),
            Self::Canceled => write!(f, "canceled"),
            Self::Expired => write!(f, "expired"),
            Self::Canceling => write!(f, "canceling"),
            Self::Paused => write!(f, "paused"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
