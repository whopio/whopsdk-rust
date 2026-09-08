pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "object")]
#[non_exhaustive]
pub enum AudienceEngagementRule {
    #[serde(rename = "facebook_page")]
    #[non_exhaustive]
    FacebookPage {
        event: AudienceEngagementFacebookPageRuleEvent,
        #[serde(default)]
        retention_days: i64,
        #[serde(default)]
        social_account_id: String,
    },

    #[serde(rename = "instagram_profile")]
    #[non_exhaustive]
    InstagramProfile {
        event: AudienceEngagementInstagramProfileRuleEvent,
        #[serde(default)]
        retention_days: i64,
        #[serde(default)]
        social_account_id: String,
    },

    #[serde(rename = "lead_form")]
    #[non_exhaustive]
    LeadForm {
        event: AudienceEngagementLeadFormRuleEvent,
        #[serde(default)]
        platform_form_ids: Vec<String>,
        #[serde(default)]
        retention_days: i64,
        #[serde(default)]
        social_account_id: String,
    },

    #[serde(rename = "video")]
    #[non_exhaustive]
    Video {
        event: AudienceEngagementVideoRuleEvent,
        #[serde(default)]
        platform_video_ids: Vec<String>,
        #[serde(default)]
        retention_days: i64,
        #[serde(default)]
        social_account_id: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AudienceEngagementRule {
    pub fn facebook_page(
        event: AudienceEngagementFacebookPageRuleEvent,
        retention_days: i64,
        social_account_id: String,
    ) -> Self {
        Self::FacebookPage {
            event,
            retention_days,
            social_account_id,
        }
    }

    pub fn instagram_profile(
        event: AudienceEngagementInstagramProfileRuleEvent,
        retention_days: i64,
        social_account_id: String,
    ) -> Self {
        Self::InstagramProfile {
            event,
            retention_days,
            social_account_id,
        }
    }

    pub fn lead_form(
        event: AudienceEngagementLeadFormRuleEvent,
        platform_form_ids: Vec<String>,
        retention_days: i64,
        social_account_id: String,
    ) -> Self {
        Self::LeadForm {
            event,
            platform_form_ids,
            retention_days,
            social_account_id,
        }
    }

    pub fn video(
        event: AudienceEngagementVideoRuleEvent,
        platform_video_ids: Vec<String>,
        retention_days: i64,
        social_account_id: String,
    ) -> Self {
        Self::Video {
            event,
            platform_video_ids,
            retention_days,
            social_account_id,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
