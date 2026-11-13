pub use crate::prelude::*;

/// Why the object can't deliver yet. `content_rejected`, `landing_page_rejected`, and `creative_unsupported` go on ads, `targeting_rejected` on ad groups, and `in_review` on either.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReportIssuesAdsRequestIssuesItemReason {
    ContentRejected,
    LandingPageRejected,
    CreativeUnsupported,
    TargetingRejected,
    InReview,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReportIssuesAdsRequestIssuesItemReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ContentRejected => serializer.serialize_str("content_rejected"),
            Self::LandingPageRejected => serializer.serialize_str("landing_page_rejected"),
            Self::CreativeUnsupported => serializer.serialize_str("creative_unsupported"),
            Self::TargetingRejected => serializer.serialize_str("targeting_rejected"),
            Self::InReview => serializer.serialize_str("in_review"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReportIssuesAdsRequestIssuesItemReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "content_rejected" => Ok(Self::ContentRejected),
            "landing_page_rejected" => Ok(Self::LandingPageRejected),
            "creative_unsupported" => Ok(Self::CreativeUnsupported),
            "targeting_rejected" => Ok(Self::TargetingRejected),
            "in_review" => Ok(Self::InReview),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReportIssuesAdsRequestIssuesItemReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContentRejected => write!(f, "content_rejected"),
            Self::LandingPageRejected => write!(f, "landing_page_rejected"),
            Self::CreativeUnsupported => write!(f, "creative_unsupported"),
            Self::TargetingRejected => write!(f, "targeting_rejected"),
            Self::InReview => write!(f, "in_review"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
