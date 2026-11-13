pub use crate::prelude::*;

/// `reviewed` once Whop content review finishes, `in_review` while it runs, `not_reviewed` when it never ran.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdContentReviewStatus {
    Reviewed,
    InReview,
    NotReviewed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdContentReviewStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Reviewed => serializer.serialize_str("reviewed"),
            Self::InReview => serializer.serialize_str("in_review"),
            Self::NotReviewed => serializer.serialize_str("not_reviewed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdContentReviewStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "reviewed" => Ok(Self::Reviewed),
            "in_review" => Ok(Self::InReview),
            "not_reviewed" => Ok(Self::NotReviewed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdContentReviewStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reviewed => write!(f, "reviewed"),
            Self::InReview => write!(f, "in_review"),
            Self::NotReviewed => write!(f, "not_reviewed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
