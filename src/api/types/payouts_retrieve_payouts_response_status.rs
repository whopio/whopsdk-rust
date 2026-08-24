pub use crate::prelude::*;

/// Current payout status.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePayoutsResponseStatus {
    Requested,
    InReview,
    Processing,
    Completed,
    Reversed,
    Canceled,
    Failed,
    Denied,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrievePayoutsResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Requested => serializer.serialize_str("requested"),
            Self::InReview => serializer.serialize_str("in_review"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Reversed => serializer.serialize_str("reversed"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrievePayoutsResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "requested" => Ok(Self::Requested),
            "in_review" => Ok(Self::InReview),
            "processing" => Ok(Self::Processing),
            "completed" => Ok(Self::Completed),
            "reversed" => Ok(Self::Reversed),
            "canceled" => Ok(Self::Canceled),
            "failed" => Ok(Self::Failed),
            "denied" => Ok(Self::Denied),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrievePayoutsResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Requested => write!(f, "requested"),
            Self::InReview => write!(f, "in_review"),
            Self::Processing => write!(f, "processing"),
            Self::Completed => write!(f, "completed"),
            Self::Reversed => write!(f, "reversed"),
            Self::Canceled => write!(f, "canceled"),
            Self::Failed => write!(f, "failed"),
            Self::Denied => write!(f, "denied"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
