pub use crate::prelude::*;

/// Whether the campaign's ads are delivering right now, and if not, why. When several states apply at once, the highest-precedence one is returned.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdCampaignDeliveryStatus {
    PaymentFailed,
    AllAdsRejected,
    Draft,
    NoAdGroups,
    NoAds,
    Paused,
    Processing,
    Issues,
    Scheduled,
    Completed,
    AdGroupsOff,
    Active,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdCampaignDeliveryStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PaymentFailed => serializer.serialize_str("payment_failed"),
            Self::AllAdsRejected => serializer.serialize_str("all_ads_rejected"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::NoAdGroups => serializer.serialize_str("no_ad_groups"),
            Self::NoAds => serializer.serialize_str("no_ads"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Issues => serializer.serialize_str("issues"),
            Self::Scheduled => serializer.serialize_str("scheduled"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::AdGroupsOff => serializer.serialize_str("ad_groups_off"),
            Self::Active => serializer.serialize_str("active"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdCampaignDeliveryStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "payment_failed" => Ok(Self::PaymentFailed),
            "all_ads_rejected" => Ok(Self::AllAdsRejected),
            "draft" => Ok(Self::Draft),
            "no_ad_groups" => Ok(Self::NoAdGroups),
            "no_ads" => Ok(Self::NoAds),
            "paused" => Ok(Self::Paused),
            "processing" => Ok(Self::Processing),
            "issues" => Ok(Self::Issues),
            "scheduled" => Ok(Self::Scheduled),
            "completed" => Ok(Self::Completed),
            "ad_groups_off" => Ok(Self::AdGroupsOff),
            "active" => Ok(Self::Active),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdCampaignDeliveryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PaymentFailed => write!(f, "payment_failed"),
            Self::AllAdsRejected => write!(f, "all_ads_rejected"),
            Self::Draft => write!(f, "draft"),
            Self::NoAdGroups => write!(f, "no_ad_groups"),
            Self::NoAds => write!(f, "no_ads"),
            Self::Paused => write!(f, "paused"),
            Self::Processing => write!(f, "processing"),
            Self::Issues => write!(f, "issues"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::Completed => write!(f, "completed"),
            Self::AdGroupsOff => write!(f, "ad_groups_off"),
            Self::Active => write!(f, "active"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
