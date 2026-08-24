pub use crate::prelude::*;

/// Current verification state. `not_started` before any session exists; `pending` while a session needs the user's input; `processing` while the provider's automated checks run on a fresh submission; `action_required` when `requested_information` needs answers; `manual_review` while information already sent is under review — an audit answer, or a document the payout provider holds — nothing to submit, usually done within 3 business days; `approved` on success; `rejected` on failure. Call Create Verification again to start a new session.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateVerificationsResponseStatus {
    NotStarted,
    Pending,
    Processing,
    ManualReview,
    Approved,
    Rejected,
    ActionRequired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateVerificationsResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotStarted => serializer.serialize_str("not_started"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::ManualReview => serializer.serialize_str("manual_review"),
            Self::Approved => serializer.serialize_str("approved"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::ActionRequired => serializer.serialize_str("action_required"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateVerificationsResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_started" => Ok(Self::NotStarted),
            "pending" => Ok(Self::Pending),
            "processing" => Ok(Self::Processing),
            "manual_review" => Ok(Self::ManualReview),
            "approved" => Ok(Self::Approved),
            "rejected" => Ok(Self::Rejected),
            "action_required" => Ok(Self::ActionRequired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateVerificationsResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotStarted => write!(f, "not_started"),
            Self::Pending => write!(f, "pending"),
            Self::Processing => write!(f, "processing"),
            Self::ManualReview => write!(f, "manual_review"),
            Self::Approved => write!(f, "approved"),
            Self::Rejected => write!(f, "rejected"),
            Self::ActionRequired => write!(f, "action_required"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
