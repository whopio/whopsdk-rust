pub use crate::prelude::*;

/// Whether the ad is delivering right now, and if not, why. When several states apply at once, the highest-precedence one is returned.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdDeliveryStatus {
    Rejected,
    InReview,
    Draft,
    CampaignPaused,
    AdGroupPaused,
    Paused,
    Processing,
    Issues,
    Scheduled,
    LearningLimited,
    Learning,
    Active,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdDeliveryStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::InReview => serializer.serialize_str("in_review"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::CampaignPaused => serializer.serialize_str("campaign_paused"),
            Self::AdGroupPaused => serializer.serialize_str("ad_group_paused"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Issues => serializer.serialize_str("issues"),
            Self::Scheduled => serializer.serialize_str("scheduled"),
            Self::LearningLimited => serializer.serialize_str("learning_limited"),
            Self::Learning => serializer.serialize_str("learning"),
            Self::Active => serializer.serialize_str("active"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdDeliveryStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "rejected" => Ok(Self::Rejected),
            "in_review" => Ok(Self::InReview),
            "draft" => Ok(Self::Draft),
            "campaign_paused" => Ok(Self::CampaignPaused),
            "ad_group_paused" => Ok(Self::AdGroupPaused),
            "paused" => Ok(Self::Paused),
            "processing" => Ok(Self::Processing),
            "issues" => Ok(Self::Issues),
            "scheduled" => Ok(Self::Scheduled),
            "learning_limited" => Ok(Self::LearningLimited),
            "learning" => Ok(Self::Learning),
            "active" => Ok(Self::Active),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdDeliveryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rejected => write!(f, "rejected"),
            Self::InReview => write!(f, "in_review"),
            Self::Draft => write!(f, "draft"),
            Self::CampaignPaused => write!(f, "campaign_paused"),
            Self::AdGroupPaused => write!(f, "ad_group_paused"),
            Self::Paused => write!(f, "paused"),
            Self::Processing => write!(f, "processing"),
            Self::Issues => write!(f, "issues"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::LearningLimited => write!(f, "learning_limited"),
            Self::Learning => write!(f, "learning"),
            Self::Active => write!(f, "active"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
