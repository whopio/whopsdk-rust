pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountRecommendedActionOutcome {
    /// The business the chain ran on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub actions: Vec<AccountRecommendedActionChainStep>,
    /// What running the chain accomplishes
    #[serde(default)]
    pub description: String,
    /// When the chain was run, as an ISO 8601 timestamp.
    #[serde(default)]
    pub executed_at: String,
    /// The executed chain id (`rac_chain_*` or `rac_seed_*`)
    #[serde(default)]
    pub id: String,
    /// Milliseconds from chain execution to that payment, or `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<i64>,
    /// The payment amount as Money, or `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<HashMap<String, serde_json::Value>>,
    /// When that payment completed, ISO 8601, or `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_at: Option<String>,
    /// The first `payment.completed` on that business after the chain, prefixed `pay_`, or `null` if none landed within 30 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// Why this chain was recommended, or `null` when unavailable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<HashMap<String, serde_json::Value>>,
    /// Headline for the chain
    #[serde(default)]
    pub title: String,
}

impl AccountRecommendedActionOutcome {
    pub fn builder() -> AccountRecommendedActionOutcomeBuilder {
        <AccountRecommendedActionOutcomeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountRecommendedActionOutcomeBuilder {
    account: Option<HashMap<String, serde_json::Value>>,
    actions: Option<Vec<AccountRecommendedActionChainStep>>,
    description: Option<String>,
    executed_at: Option<String>,
    id: Option<String>,
    latency_ms: Option<i64>,
    payment: Option<HashMap<String, serde_json::Value>>,
    payment_at: Option<String>,
    payment_id: Option<String>,
    reasoning: Option<HashMap<String, serde_json::Value>>,
    title: Option<String>,
}

impl AccountRecommendedActionOutcomeBuilder {
    pub fn account(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.account = Some(value);
        self
    }

    pub fn actions(mut self, value: Vec<AccountRecommendedActionChainStep>) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    pub fn latency_ms(mut self, value: i64) -> Self {
        self.latency_ms = Some(value);
        self
    }

    pub fn payment(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn payment_at(mut self, value: impl Into<String>) -> Self {
        self.payment_at = Some(value.into());
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn reasoning(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountRecommendedActionOutcome`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actions`](AccountRecommendedActionOutcomeBuilder::actions)
    /// - [`description`](AccountRecommendedActionOutcomeBuilder::description)
    /// - [`executed_at`](AccountRecommendedActionOutcomeBuilder::executed_at)
    /// - [`id`](AccountRecommendedActionOutcomeBuilder::id)
    /// - [`title`](AccountRecommendedActionOutcomeBuilder::title)
    pub fn build(self) -> Result<AccountRecommendedActionOutcome, BuildError> {
        Ok(AccountRecommendedActionOutcome {
            account: self.account,
            actions: self
                .actions
                .ok_or_else(|| BuildError::missing_field("actions"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            executed_at: self
                .executed_at
                .ok_or_else(|| BuildError::missing_field("executed_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            latency_ms: self.latency_ms,
            payment: self.payment,
            payment_at: self.payment_at,
            payment_id: self.payment_id,
            reasoning: self.reasoning,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
