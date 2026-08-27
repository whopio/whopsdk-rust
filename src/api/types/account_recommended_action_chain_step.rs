pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountRecommendedActionChainStep {
    /// The action definition key this step runs; new values may be added, so handle unknown actions gracefully
    #[serde(default)]
    pub action: String,
    /// The URL where this step is done by hand
    #[serde(default)]
    pub cta: String,
    /// Button label
    #[serde(default)]
    pub cta_label: String,
    /// Supporting copy, or empty
    #[serde(default)]
    pub description: String,
    /// Why the step failed, or `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Whether the client should navigate to the CTA or open the programmatic execution dialog
    pub execution_type: AccountRecommendedActionChainStepExecutionType,
    /// The filled-in request body for the step's endpoint, or `null` when it was not recorded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<HashMap<String, serde_json::Value>>,
    /// The API response the step produced, or `null` until it succeeds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<HashMap<String, serde_json::Value>>,
    /// Zero-based order of this step within the chain
    #[serde(default)]
    pub position: i64,
    /// Why this step was recommended, or `null` when unavailable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<HashMap<String, serde_json::Value>>,
    /// Where the run step currently stands, or `null` when the chain has not been run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AccountRecommendedActionChainStepStatus>,
    /// Headline for the step
    #[serde(default)]
    pub title: String,
}

impl AccountRecommendedActionChainStep {
    pub fn builder() -> AccountRecommendedActionChainStepBuilder {
        <AccountRecommendedActionChainStepBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountRecommendedActionChainStepBuilder {
    action: Option<String>,
    cta: Option<String>,
    cta_label: Option<String>,
    description: Option<String>,
    error: Option<String>,
    execution_type: Option<AccountRecommendedActionChainStepExecutionType>,
    input: Option<HashMap<String, serde_json::Value>>,
    output: Option<HashMap<String, serde_json::Value>>,
    position: Option<i64>,
    reasoning: Option<HashMap<String, serde_json::Value>>,
    status: Option<AccountRecommendedActionChainStepStatus>,
    title: Option<String>,
}

impl AccountRecommendedActionChainStepBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
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

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn execution_type(mut self, value: AccountRecommendedActionChainStepExecutionType) -> Self {
        self.execution_type = Some(value);
        self
    }

    pub fn input(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn output(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.output = Some(value);
        self
    }

    pub fn position(mut self, value: i64) -> Self {
        self.position = Some(value);
        self
    }

    pub fn reasoning(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn status(mut self, value: AccountRecommendedActionChainStepStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountRecommendedActionChainStep`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](AccountRecommendedActionChainStepBuilder::action)
    /// - [`cta`](AccountRecommendedActionChainStepBuilder::cta)
    /// - [`cta_label`](AccountRecommendedActionChainStepBuilder::cta_label)
    /// - [`description`](AccountRecommendedActionChainStepBuilder::description)
    /// - [`execution_type`](AccountRecommendedActionChainStepBuilder::execution_type)
    /// - [`position`](AccountRecommendedActionChainStepBuilder::position)
    /// - [`title`](AccountRecommendedActionChainStepBuilder::title)
    pub fn build(self) -> Result<AccountRecommendedActionChainStep, BuildError> {
        Ok(AccountRecommendedActionChainStep {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            cta: self.cta.ok_or_else(|| BuildError::missing_field("cta"))?,
            cta_label: self
                .cta_label
                .ok_or_else(|| BuildError::missing_field("cta_label"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            error: self.error,
            execution_type: self
                .execution_type
                .ok_or_else(|| BuildError::missing_field("execution_type"))?,
            input: self.input,
            output: self.output,
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            reasoning: self.reasoning,
            status: self.status,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
