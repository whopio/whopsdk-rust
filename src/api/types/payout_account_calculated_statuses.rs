pub use crate::prelude::*;

/// The granular calculated statuses reflecting payout account KYC and payout readiness.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PayoutAccountCalculatedStatuses {
    Connected,
    Disabled,
    ActionRequired,
    PendingVerification,
    VerificationFailed,
    ManualReview,
    Denied,
    NotStarted,
    BlockedByParent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PayoutAccountCalculatedStatuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Connected => serializer.serialize_str("connected"),
            Self::Disabled => serializer.serialize_str("disabled"),
            Self::ActionRequired => serializer.serialize_str("action_required"),
            Self::PendingVerification => serializer.serialize_str("pending_verification"),
            Self::VerificationFailed => serializer.serialize_str("verification_failed"),
            Self::ManualReview => serializer.serialize_str("manual_review"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::NotStarted => serializer.serialize_str("not_started"),
            Self::BlockedByParent => serializer.serialize_str("blocked_by_parent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PayoutAccountCalculatedStatuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "connected" => Ok(Self::Connected),
            "disabled" => Ok(Self::Disabled),
            "action_required" => Ok(Self::ActionRequired),
            "pending_verification" => Ok(Self::PendingVerification),
            "verification_failed" => Ok(Self::VerificationFailed),
            "manual_review" => Ok(Self::ManualReview),
            "denied" => Ok(Self::Denied),
            "not_started" => Ok(Self::NotStarted),
            "blocked_by_parent" => Ok(Self::BlockedByParent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PayoutAccountCalculatedStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connected => write!(f, "connected"),
            Self::Disabled => write!(f, "disabled"),
            Self::ActionRequired => write!(f, "action_required"),
            Self::PendingVerification => write!(f, "pending_verification"),
            Self::VerificationFailed => write!(f, "verification_failed"),
            Self::ManualReview => write!(f, "manual_review"),
            Self::Denied => write!(f, "denied"),
            Self::NotStarted => write!(f, "not_started"),
            Self::BlockedByParent => write!(f, "blocked_by_parent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
