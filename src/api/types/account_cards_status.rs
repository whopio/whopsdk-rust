pub use crate::prelude::*;

/// Where the card application stands. `approved` means cards can be issued. `needs_verification` means the applicant has not completed identity verification yet; `needs_information` means they did, but the documents were rejected for a fixable reason and must be resubmitted. `pending` and `manual_review` are in flight. `denied`, `locked`, and `canceled` are terminal.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountCardsStatus {
    Approved,
    Pending,
    ManualReview,
    Denied,
    Locked,
    Canceled,
    NeedsVerification,
    NeedsInformation,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountCardsStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Approved => serializer.serialize_str("approved"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::ManualReview => serializer.serialize_str("manual_review"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::Locked => serializer.serialize_str("locked"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::NeedsVerification => serializer.serialize_str("needs_verification"),
            Self::NeedsInformation => serializer.serialize_str("needs_information"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountCardsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "approved" => Ok(Self::Approved),
            "pending" => Ok(Self::Pending),
            "manual_review" => Ok(Self::ManualReview),
            "denied" => Ok(Self::Denied),
            "locked" => Ok(Self::Locked),
            "canceled" => Ok(Self::Canceled),
            "needs_verification" => Ok(Self::NeedsVerification),
            "needs_information" => Ok(Self::NeedsInformation),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountCardsStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Approved => write!(f, "approved"),
            Self::Pending => write!(f, "pending"),
            Self::ManualReview => write!(f, "manual_review"),
            Self::Denied => write!(f, "denied"),
            Self::Locked => write!(f, "locked"),
            Self::Canceled => write!(f, "canceled"),
            Self::NeedsVerification => write!(f, "needs_verification"),
            Self::NeedsInformation => write!(f, "needs_information"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
