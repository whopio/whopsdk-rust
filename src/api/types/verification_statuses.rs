pub use crate::prelude::*;

/// A status for a verification.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerificationStatuses {
    RequiresInput,
    Processing,
    Verified,
    Canceled,
    Created,
    Started,
    Submitted,
    Approved,
    Declined,
    ResubmissionRequested,
    Expired,
    Abandoned,
    Review,
    ActionRequired,
    ManualReview,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VerificationStatuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RequiresInput => serializer.serialize_str("requires_input"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Verified => serializer.serialize_str("verified"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Created => serializer.serialize_str("created"),
            Self::Started => serializer.serialize_str("started"),
            Self::Submitted => serializer.serialize_str("submitted"),
            Self::Approved => serializer.serialize_str("approved"),
            Self::Declined => serializer.serialize_str("declined"),
            Self::ResubmissionRequested => serializer.serialize_str("resubmission_requested"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Abandoned => serializer.serialize_str("abandoned"),
            Self::Review => serializer.serialize_str("review"),
            Self::ActionRequired => serializer.serialize_str("action_required"),
            Self::ManualReview => serializer.serialize_str("manual_review"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VerificationStatuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "requires_input" => Ok(Self::RequiresInput),
            "processing" => Ok(Self::Processing),
            "verified" => Ok(Self::Verified),
            "canceled" => Ok(Self::Canceled),
            "created" => Ok(Self::Created),
            "started" => Ok(Self::Started),
            "submitted" => Ok(Self::Submitted),
            "approved" => Ok(Self::Approved),
            "declined" => Ok(Self::Declined),
            "resubmission_requested" => Ok(Self::ResubmissionRequested),
            "expired" => Ok(Self::Expired),
            "abandoned" => Ok(Self::Abandoned),
            "review" => Ok(Self::Review),
            "action_required" => Ok(Self::ActionRequired),
            "manual_review" => Ok(Self::ManualReview),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VerificationStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiresInput => write!(f, "requires_input"),
            Self::Processing => write!(f, "processing"),
            Self::Verified => write!(f, "verified"),
            Self::Canceled => write!(f, "canceled"),
            Self::Created => write!(f, "created"),
            Self::Started => write!(f, "started"),
            Self::Submitted => write!(f, "submitted"),
            Self::Approved => write!(f, "approved"),
            Self::Declined => write!(f, "declined"),
            Self::ResubmissionRequested => write!(f, "resubmission_requested"),
            Self::Expired => write!(f, "expired"),
            Self::Abandoned => write!(f, "abandoned"),
            Self::Review => write!(f, "review"),
            Self::ActionRequired => write!(f, "action_required"),
            Self::ManualReview => write!(f, "manual_review"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
