pub use crate::prelude::*;

/// Whether ads in this ad group are delivering right now, and if not, why. When several states apply at once, the highest-precedence one is returned.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdGroupDeliveryStatus {
    AllAdsRejected,
    Rejected,
    Draft,
    NoAds,
    CampaignPaused,
    Paused,
    Processing,
    Issues,
    Scheduled,
    Completed,
    AdsOff,
    LearningLimited,
    Learning,
    Active,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdGroupDeliveryStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AllAdsRejected => serializer.serialize_str("all_ads_rejected"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::NoAds => serializer.serialize_str("no_ads"),
            Self::CampaignPaused => serializer.serialize_str("campaign_paused"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Issues => serializer.serialize_str("issues"),
            Self::Scheduled => serializer.serialize_str("scheduled"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::AdsOff => serializer.serialize_str("ads_off"),
            Self::LearningLimited => serializer.serialize_str("learning_limited"),
            Self::Learning => serializer.serialize_str("learning"),
            Self::Active => serializer.serialize_str("active"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdGroupDeliveryStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all_ads_rejected" => Ok(Self::AllAdsRejected),
            "rejected" => Ok(Self::Rejected),
            "draft" => Ok(Self::Draft),
            "no_ads" => Ok(Self::NoAds),
            "campaign_paused" => Ok(Self::CampaignPaused),
            "paused" => Ok(Self::Paused),
            "processing" => Ok(Self::Processing),
            "issues" => Ok(Self::Issues),
            "scheduled" => Ok(Self::Scheduled),
            "completed" => Ok(Self::Completed),
            "ads_off" => Ok(Self::AdsOff),
            "learning_limited" => Ok(Self::LearningLimited),
            "learning" => Ok(Self::Learning),
            "active" => Ok(Self::Active),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdGroupDeliveryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllAdsRejected => write!(f, "all_ads_rejected"),
            Self::Rejected => write!(f, "rejected"),
            Self::Draft => write!(f, "draft"),
            Self::NoAds => write!(f, "no_ads"),
            Self::CampaignPaused => write!(f, "campaign_paused"),
            Self::Paused => write!(f, "paused"),
            Self::Processing => write!(f, "processing"),
            Self::Issues => write!(f, "issues"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::Completed => write!(f, "completed"),
            Self::AdsOff => write!(f, "ads_off"),
            Self::LearningLimited => write!(f, "learning_limited"),
            Self::Learning => write!(f, "learning"),
            Self::Active => write!(f, "active"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
