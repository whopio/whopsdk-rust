pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecommendedActionExecution {
    /// The action definition key the step ran
    #[serde(default)]
    pub action: String,
    /// When the step reached a terminal status, ISO 8601
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Why the step failed, or `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Execution ID, prefixed `raex_`
    #[serde(default)]
    pub id: String,
    /// The API response the step produced, or `null` until it succeeds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<HashMap<String, serde_json::Value>>,
    /// Zero-based order of the step within the chain
    #[serde(default)]
    pub position: i64,
    /// Where the step currently stands
    pub status: RecommendedActionExecutionStatus,
}

impl RecommendedActionExecution {
    pub fn builder() -> RecommendedActionExecutionBuilder {
        <RecommendedActionExecutionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecommendedActionExecutionBuilder {
    action: Option<String>,
    completed_at: Option<String>,
    error: Option<String>,
    id: Option<String>,
    output: Option<HashMap<String, serde_json::Value>>,
    position: Option<i64>,
    status: Option<RecommendedActionExecutionStatus>,
}

impl RecommendedActionExecutionBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn completed_at(mut self, value: impl Into<String>) -> Self {
        self.completed_at = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn status(mut self, value: RecommendedActionExecutionStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecommendedActionExecution`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](RecommendedActionExecutionBuilder::action)
    /// - [`id`](RecommendedActionExecutionBuilder::id)
    /// - [`position`](RecommendedActionExecutionBuilder::position)
    /// - [`status`](RecommendedActionExecutionBuilder::status)
    pub fn build(self) -> Result<RecommendedActionExecution, BuildError> {
        Ok(RecommendedActionExecution {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            completed_at: self.completed_at,
            error: self.error,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            output: self.output,
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
