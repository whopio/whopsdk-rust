pub use crate::prelude::*;

/// The result the ad group's delivery is optimized to get the most of.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdGroupOptimizationGoal {
    Conversions,
    LinkClicks,
    LandingPageViews,
    Reach,
    Impressions,
    Engagement,
    Conversations,
    VideoViews,
    TwoSecondViews,
    PageLikes,
    SocialProfile,
    AdRecallLift,
    EventResponses,
    RemindersSet,
    LeadGeneration,
    QualityLead,
    Value,
    ProfileAndPageEngagement,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdGroupOptimizationGoal {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Conversions => serializer.serialize_str("conversions"),
            Self::LinkClicks => serializer.serialize_str("link_clicks"),
            Self::LandingPageViews => serializer.serialize_str("landing_page_views"),
            Self::Reach => serializer.serialize_str("reach"),
            Self::Impressions => serializer.serialize_str("impressions"),
            Self::Engagement => serializer.serialize_str("engagement"),
            Self::Conversations => serializer.serialize_str("conversations"),
            Self::VideoViews => serializer.serialize_str("video_views"),
            Self::TwoSecondViews => serializer.serialize_str("two_second_views"),
            Self::PageLikes => serializer.serialize_str("page_likes"),
            Self::SocialProfile => serializer.serialize_str("social_profile"),
            Self::AdRecallLift => serializer.serialize_str("ad_recall_lift"),
            Self::EventResponses => serializer.serialize_str("event_responses"),
            Self::RemindersSet => serializer.serialize_str("reminders_set"),
            Self::LeadGeneration => serializer.serialize_str("lead_generation"),
            Self::QualityLead => serializer.serialize_str("quality_lead"),
            Self::Value => serializer.serialize_str("value"),
            Self::ProfileAndPageEngagement => {
                serializer.serialize_str("profile_and_page_engagement")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdGroupOptimizationGoal {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "conversions" => Ok(Self::Conversions),
            "link_clicks" => Ok(Self::LinkClicks),
            "landing_page_views" => Ok(Self::LandingPageViews),
            "reach" => Ok(Self::Reach),
            "impressions" => Ok(Self::Impressions),
            "engagement" => Ok(Self::Engagement),
            "conversations" => Ok(Self::Conversations),
            "video_views" => Ok(Self::VideoViews),
            "two_second_views" => Ok(Self::TwoSecondViews),
            "page_likes" => Ok(Self::PageLikes),
            "social_profile" => Ok(Self::SocialProfile),
            "ad_recall_lift" => Ok(Self::AdRecallLift),
            "event_responses" => Ok(Self::EventResponses),
            "reminders_set" => Ok(Self::RemindersSet),
            "lead_generation" => Ok(Self::LeadGeneration),
            "quality_lead" => Ok(Self::QualityLead),
            "value" => Ok(Self::Value),
            "profile_and_page_engagement" => Ok(Self::ProfileAndPageEngagement),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdGroupOptimizationGoal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conversions => write!(f, "conversions"),
            Self::LinkClicks => write!(f, "link_clicks"),
            Self::LandingPageViews => write!(f, "landing_page_views"),
            Self::Reach => write!(f, "reach"),
            Self::Impressions => write!(f, "impressions"),
            Self::Engagement => write!(f, "engagement"),
            Self::Conversations => write!(f, "conversations"),
            Self::VideoViews => write!(f, "video_views"),
            Self::TwoSecondViews => write!(f, "two_second_views"),
            Self::PageLikes => write!(f, "page_likes"),
            Self::SocialProfile => write!(f, "social_profile"),
            Self::AdRecallLift => write!(f, "ad_recall_lift"),
            Self::EventResponses => write!(f, "event_responses"),
            Self::RemindersSet => write!(f, "reminders_set"),
            Self::LeadGeneration => write!(f, "lead_generation"),
            Self::QualityLead => write!(f, "quality_lead"),
            Self::Value => write!(f, "value"),
            Self::ProfileAndPageEngagement => write!(f, "profile_and_page_engagement"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
