pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EconomicIntelligence {
    /// The account this recommendation is for, prefixed `biz_`
    #[serde(default)]
    pub account_id: String,
    /// The playbook action this card recommends, or `null` for an untyped card; new values may be added, so handle unknown types gracefully
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// When the recommendation was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// When the card was run, as an ISO 8601 timestamp, or `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
    /// Economic intelligence ID, prefixed `reca_`
    #[serde(default)]
    pub id: String,
    /// What the owner asked for, in their own words, when this recommendation was requested, or `null` when the engine chose the action on its own
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// The step-by-step brief Whop AI executes when the card is run, or `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The signal and number the recommendation rests on, or `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    /// `queued` once requested and not yet picked up; `pending` while the engine is generating; `ready` when the card is written and the owner can run it; `executed` once it was run; `superseded` when a newer card of the same action type replaced it; `failed` when the engine had nothing to recommend for the request
    pub status: EconomicIntelligenceStatus,
    /// When a newer card replaced this one, as an ISO 8601 timestamp, or `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superseded_at: Option<String>,
    /// The recommendation as the owner sees it: one command with the payoff, or `null` until the engine has written the card
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
    /// - [`account_id`](EconomicIntelligenceBuilder::account_id)
    /// - [`created_at`](EconomicIntelligenceBuilder::created_at)
    /// - [`id`](EconomicIntelligenceBuilder::id)
    /// - [`status`](EconomicIntelligenceBuilder::status)
    pub fn build(self) -> Result<EconomicIntelligence, BuildError> {
        Ok(EconomicIntelligence {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            action_type: self.action_type,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
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
