pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RunRecommendedActionsResponse {
    #[serde(default)]
    pub chain_id: String,
    pub execution: RunRecommendedActionsResponseExecution,
}

impl RunRecommendedActionsResponse {
    pub fn builder() -> RunRecommendedActionsResponseBuilder {
        <RunRecommendedActionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunRecommendedActionsResponseBuilder {
    chain_id: Option<String>,
    execution: Option<RunRecommendedActionsResponseExecution>,
}

impl RunRecommendedActionsResponseBuilder {
    pub fn chain_id(mut self, value: impl Into<String>) -> Self {
        self.chain_id = Some(value.into());
        self
    }

    pub fn execution(mut self, value: RunRecommendedActionsResponseExecution) -> Self {
        self.execution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunRecommendedActionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chain_id`](RunRecommendedActionsResponseBuilder::chain_id)
    /// - [`execution`](RunRecommendedActionsResponseBuilder::execution)
    pub fn build(self) -> Result<RunRecommendedActionsResponse, BuildError> {
        Ok(RunRecommendedActionsResponse {
            chain_id: self
                .chain_id
                .ok_or_else(|| BuildError::missing_field("chain_id"))?,
            execution: self
                .execution
                .ok_or_else(|| BuildError::missing_field("execution"))?,
        })
    }
}
