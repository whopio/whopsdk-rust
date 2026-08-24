pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserRecommendedAction {
    /// The account (`biz_`) a business recommendation is for, or `null` for personal recommendations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The account's display name, or `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// The recommendation; new values may be added, so handle unknown actions gracefully
    pub action: UserRecommendedActionAction,
    #[serde(default)]
    pub blocked_capabilities: Vec<String>,
    /// The URL the call-to-action links to
    #[serde(default)]
    pub cta: String,
    /// Button label
    #[serde(default)]
    pub cta_label: String,
    /// Supporting copy, or empty
    #[serde(default)]
    pub description: String,
    /// Illustration icon URL, or `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Estimated impact from 0-100, or `null` when not ranked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_score: Option<i64>,
    /// Why this action was recommended, or `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    /// Always optional — never blocking
    pub status: UserRecommendedActionStatus,
    /// Headline for the recommendation
    #[serde(default)]
    pub title: String,
}

impl UserRecommendedAction {
    pub fn builder() -> UserRecommendedActionBuilder {
        <UserRecommendedActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserRecommendedActionBuilder {
    account_id: Option<String>,
    account_name: Option<String>,
    action: Option<UserRecommendedActionAction>,
    blocked_capabilities: Option<Vec<String>>,
    cta: Option<String>,
    cta_label: Option<String>,
    description: Option<String>,
    icon_url: Option<String>,
    impact_score: Option<i64>,
    reasoning: Option<String>,
    status: Option<UserRecommendedActionStatus>,
    title: Option<String>,
}

impl UserRecommendedActionBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn account_name(mut self, value: impl Into<String>) -> Self {
        self.account_name = Some(value.into());
        self
    }

    pub fn action(mut self, value: UserRecommendedActionAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn blocked_capabilities(mut self, value: Vec<String>) -> Self {
        self.blocked_capabilities = Some(value);
        self
    }

    pub fn cta(mut self, value: impl Into<String>) -> Self {
        self.cta = Some(value.into());
        self
    }

    pub fn cta_label(mut self, value: impl Into<String>) -> Self {
        self.cta_label = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn impact_score(mut self, value: i64) -> Self {
        self.impact_score = Some(value);
        self
    }

    pub fn reasoning(mut self, value: impl Into<String>) -> Self {
        self.reasoning = Some(value.into());
        self
    }

    pub fn status(mut self, value: UserRecommendedActionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserRecommendedAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](UserRecommendedActionBuilder::action)
    /// - [`blocked_capabilities`](UserRecommendedActionBuilder::blocked_capabilities)
    /// - [`cta`](UserRecommendedActionBuilder::cta)
    /// - [`cta_label`](UserRecommendedActionBuilder::cta_label)
    /// - [`description`](UserRecommendedActionBuilder::description)
    /// - [`status`](UserRecommendedActionBuilder::status)
    /// - [`title`](UserRecommendedActionBuilder::title)
    pub fn build(self) -> Result<UserRecommendedAction, BuildError> {
        Ok(UserRecommendedAction {
            account_id: self.account_id,
            account_name: self.account_name,
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            blocked_capabilities: self
                .blocked_capabilities
                .ok_or_else(|| BuildError::missing_field("blocked_capabilities"))?,
            cta: self.cta.ok_or_else(|| BuildError::missing_field("cta"))?,
            cta_label: self
                .cta_label
                .ok_or_else(|| BuildError::missing_field("cta_label"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            icon_url: self.icon_url,
            impact_score: self.impact_score,
            reasoning: self.reasoning,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
