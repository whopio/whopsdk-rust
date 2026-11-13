pub use crate::prelude::*;

/// Billing state of the membership, as on the membership resource. `null` while the membership is a draft: during checkout before its first payment settles, or while its waitlist entry is pending.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchRecordMembershipStatus {
    Trialing,
    Active,
    PastDue,
    Completed,
    Canceled,
    Expired,
    Unresolved,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchRecordMembershipStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Trialing => serializer.serialize_str("trialing"),
            Self::Active => serializer.serialize_str("active"),
            Self::PastDue => serializer.serialize_str("past_due"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Unresolved => serializer.serialize_str("unresolved"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchRecordMembershipStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "trialing" => Ok(Self::Trialing),
            "active" => Ok(Self::Active),
            "past_due" => Ok(Self::PastDue),
            "completed" => Ok(Self::Completed),
            "canceled" => Ok(Self::Canceled),
            "expired" => Ok(Self::Expired),
            "unresolved" => Ok(Self::Unresolved),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchRecordMembershipStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trialing => write!(f, "trialing"),
            Self::Active => write!(f, "active"),
            Self::PastDue => write!(f, "past_due"),
            Self::Completed => write!(f, "completed"),
            Self::Canceled => write!(f, "canceled"),
            Self::Expired => write!(f, "expired"),
            Self::Unresolved => write!(f, "unresolved"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
