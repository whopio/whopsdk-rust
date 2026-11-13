pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EconomicIntelligence {
    /// ID of the account this recommendation is for, prefixed `biz_`, or null for personal onboarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// When the executed run was first marked as seen, as an ISO 8601 timestamp, or `null` if it has not been.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_at: Option<String>,
    /// Type of action recommended, or `null` when no type is assigned. New values may be added; handle unknown types gracefully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// The chat that ran the recommendation, shown only to the user who ran it, or `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_chat_id: Option<String>,
    /// When the recommendation was created, as an ISO 8601 timestamp, or null for an unsaved recommendation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the recommendation was carried out, as an ISO 8601 timestamp, or `null` if it has not been.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_tool_calls: Option<Vec<EconomicIntelligenceOperation>>,
    /// Recommendation ID, prefixed `reca_`, or `create_business` for an unsaved setup recommendation. Authenticate and list again before executing an unsaved recommendation.
    #[serde(default)]
    pub id: String,
    /// What you requested, in your own words, or `null` for recommendations generated without your input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default)]
    pub inputs: Vec<EconomicIntelligenceInput>,
    /// Step-by-step instructions for Whop AI, or `null` when no instructions are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Evidence and metrics supporting the recommendation, or `null` when no reasoning was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    /// Where to view what the run produced, such as the published website or created product, or `null` when the result is only in the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_url: Option<String>,
    /// The user who started the run, prefixed `user_`, or `null` if it has not run or was started without a user, such as with an API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_by_user_id: Option<String>,
    /// When the run ended, whether executed or incomplete, as an ISO 8601 timestamp, or `null` if it has not ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_ended_at: Option<String>,
    /// When the run started, by Whop AI or the recommendation's API calls, as an ISO 8601 timestamp, or `null` if it has not run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_started_at: Option<String>,
    /// How the user rated this recommendation, or `null` if they have not rated it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<EconomicIntelligenceSentiment>,
    /// `queued` when awaiting generation; `pending` while generating or preparing its work; `ready` when available to run; `running` while Whop AI or the recommendation's API calls carry it out; `executed` when carried out; `incomplete` when the run ended without carrying it out; `superseded` when rejected or replaced.
    pub status: EconomicIntelligenceStatus,
    /// When the recommendation was rejected or replaced, as an ISO 8601 timestamp, or `null` if neither has occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superseded_at: Option<String>,
    /// Website URL selected for pixel setup, or `null` when no website was captured for this recommendation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
    /// Recommended action and its expected benefit, or `null` until generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The user's written feedback, or `null` if they have not provided any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_feedback: Option<String>,
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
    acknowledged_at: Option<String>,
    action_type: Option<String>,
    ai_chat_id: Option<String>,
    created_at: Option<String>,
    executed_at: Option<String>,
    expected_tool_calls: Option<Vec<EconomicIntelligenceOperation>>,
    id: Option<String>,
    input: Option<String>,
    inputs: Option<Vec<EconomicIntelligenceInput>>,
    prompt: Option<String>,
    reasoning: Option<String>,
    result_url: Option<String>,
    run_by_user_id: Option<String>,
    run_ended_at: Option<String>,
    run_started_at: Option<String>,
    sentiment: Option<EconomicIntelligenceSentiment>,
    status: Option<EconomicIntelligenceStatus>,
    superseded_at: Option<String>,
    target_url: Option<String>,
    title: Option<String>,
    user_feedback: Option<String>,
}

impl EconomicIntelligenceBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn acknowledged_at(mut self, value: impl Into<String>) -> Self {
        self.acknowledged_at = Some(value.into());
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

    pub fn expected_tool_calls(mut self, value: Vec<EconomicIntelligenceOperation>) -> Self {
        self.expected_tool_calls = Some(value);
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

    pub fn inputs(mut self, value: Vec<EconomicIntelligenceInput>) -> Self {
        self.inputs = Some(value);
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

    pub fn result_url(mut self, value: impl Into<String>) -> Self {
        self.result_url = Some(value.into());
        self
    }

    pub fn run_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.run_by_user_id = Some(value.into());
        self
    }

    pub fn run_ended_at(mut self, value: impl Into<String>) -> Self {
        self.run_ended_at = Some(value.into());
        self
    }

    pub fn run_started_at(mut self, value: impl Into<String>) -> Self {
        self.run_started_at = Some(value.into());
        self
    }

    pub fn sentiment(mut self, value: EconomicIntelligenceSentiment) -> Self {
        self.sentiment = Some(value);
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

    pub fn target_url(mut self, value: impl Into<String>) -> Self {
        self.target_url = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn user_feedback(mut self, value: impl Into<String>) -> Self {
        self.user_feedback = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EconomicIntelligence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EconomicIntelligenceBuilder::id)
    /// - [`inputs`](EconomicIntelligenceBuilder::inputs)
    /// - [`status`](EconomicIntelligenceBuilder::status)
    pub fn build(self) -> Result<EconomicIntelligence, BuildError> {
        Ok(EconomicIntelligence {
            account_id: self.account_id,
            acknowledged_at: self.acknowledged_at,
            action_type: self.action_type,
            ai_chat_id: self.ai_chat_id,
            created_at: self.created_at,
            executed_at: self.executed_at,
            expected_tool_calls: self.expected_tool_calls,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            input: self.input,
            inputs: self
                .inputs
                .ok_or_else(|| BuildError::missing_field("inputs"))?,
            prompt: self.prompt,
            reasoning: self.reasoning,
            result_url: self.result_url,
            run_by_user_id: self.run_by_user_id,
            run_ended_at: self.run_ended_at,
            run_started_at: self.run_started_at,
            sentiment: self.sentiment,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            superseded_at: self.superseded_at,
            target_url: self.target_url,
            title: self.title,
            user_feedback: self.user_feedback,
        })
    }
}
