pub use crate::prelude::*;

/// `not_started` until the account applies; `pending_information` while an application waits for answers; `in_review` once submitted; then `approved` or `denied`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePreferencesResponseAdsCertificationsItemStatus {
    NotStarted,
    PendingInformation,
    InReview,
    Approved,
    Denied,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrievePreferencesResponseAdsCertificationsItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotStarted => serializer.serialize_str("not_started"),
            Self::PendingInformation => serializer.serialize_str("pending_information"),
            Self::InReview => serializer.serialize_str("in_review"),
            Self::Approved => serializer.serialize_str("approved"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrievePreferencesResponseAdsCertificationsItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_started" => Ok(Self::NotStarted),
            "pending_information" => Ok(Self::PendingInformation),
            "in_review" => Ok(Self::InReview),
            "approved" => Ok(Self::Approved),
            "denied" => Ok(Self::Denied),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrievePreferencesResponseAdsCertificationsItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotStarted => write!(f, "not_started"),
            Self::PendingInformation => write!(f, "pending_information"),
            Self::InReview => write!(f, "in_review"),
            Self::Approved => write!(f, "approved"),
            Self::Denied => write!(f, "denied"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
