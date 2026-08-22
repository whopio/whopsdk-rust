pub use crate::prelude::*;

/// The status of a membership
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MembershipStatus {
    Trialing,
    Active,
    PastDue,
    Completed,
    Canceled,
    Expired,
    Unresolved,
    Drafted,
    Canceling,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MembershipStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Trialing => serializer.serialize_str("trialing"),
            Self::Active => serializer.serialize_str("active"),
            Self::PastDue => serializer.serialize_str("past_due"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Unresolved => serializer.serialize_str("unresolved"),
            Self::Drafted => serializer.serialize_str("drafted"),
            Self::Canceling => serializer.serialize_str("canceling"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MembershipStatus {
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
            "drafted" => Ok(Self::Drafted),
            "canceling" => Ok(Self::Canceling),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MembershipStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trialing => write!(f, "trialing"),
            Self::Active => write!(f, "active"),
            Self::PastDue => write!(f, "past_due"),
            Self::Completed => write!(f, "completed"),
            Self::Canceled => write!(f, "canceled"),
            Self::Expired => write!(f, "expired"),
            Self::Unresolved => write!(f, "unresolved"),
            Self::Drafted => write!(f, "drafted"),
            Self::Canceling => write!(f, "canceling"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
