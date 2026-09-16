pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EconomicIntelligence {
    /// ID of the account this recommendation is for, prefixed `biz_`, or null for personal onboarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Type of action recommended, or `null` when no type is assigned. New values may be added; handle unknown types gracefully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// The chat to resume after its initial message is accepted, or null before a chat is ready.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_chat_id: Option<String>,
    /// When the recommendation was created, as an ISO 8601 timestamp, or null for an unsaved recommendation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the recommendation was approved, as an ISO 8601 timestamp, or `null` if it has not been approved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
    /// Recommendation ID, prefixed `reca_`, or `create_business` for an unsaved setup recommendation. Authenticate and list again before executing an unsaved recommendation.
    #[serde(default)]
    pub id: String,
    /// What you requested, in your own words, or `null` for recommendations generated without your input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// Step-by-step instructions for Whop AI, or `null` when no instructions are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Evidence and metrics supporting the recommendation, or `null` when no reasoning was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    /// `queued` when awaiting generation; `pending` while generating; `ready` when available for approval; `executed` when approved; `superseded` when rejected or replaced.
    pub status: EconomicIntelligenceStatus,
    /// When the recommendation was rejected or replaced, as an ISO 8601 timestamp, or `null` if neither has occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superseded_at: Option<String>,
    /// Recommended action and its expected benefit, or `null` until generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl EconomicIntelligence {
    pub fn builder() -> EconomicIntelligenceBuilder {
        <EconomicIntelligenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EconomicIntelligenceBuilder {
    account_id: Option<String>,
    action_type: Option<String>,
    ai_chat_id: Option<String>,
    created_at: Option<String>,
    executed_at: Option<String>,
    id: Option<String>,
    input: Option<String>,
    prompt: Option<String>,
    reasoning: Option<String>,
    status: Option<EconomicIntelligenceStatus>,
    superseded_at: Option<String>,
    title: Option<String>,
}

impl EconomicIntelligenceBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn action_type(mut self, value: impl Into<String>) -> Self {
        self.action_type = Some(value.into());
        self
    }

    pub fn ai_chat_id(mut self, value: impl Into<String>) -> Self {
        self.ai_chat_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn executed_at(mut self, value: impl Into<String>) -> Self {
        self.executed_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn input(mut self, value: impl Into<String>) -> Self {
        self.input = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn reasoning(mut self, value: impl Into<String>) -> Self {
        self.reasoning = Some(value.into());
        self
    }

    pub fn status(mut self, value: EconomicIntelligenceStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn superseded_at(mut self, value: impl Into<String>) -> Self {
        self.superseded_at = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EconomicIntelligence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EconomicIntelligenceBuilder::id)
    /// - [`status`](EconomicIntelligenceBuilder::status)
    pub fn build(self) -> Result<EconomicIntelligence, BuildError> {
        Ok(EconomicIntelligence {
            account_id: self.account_id,
            action_type: self.action_type,
            ai_chat_id: self.ai_chat_id,
            created_at: self.created_at,
            executed_at: self.executed_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            input: self.input,
            prompt: self.prompt,
            reasoning: self.reasoning,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            superseded_at: self.superseded_at,
            title: self.title,
        })
    }
}
