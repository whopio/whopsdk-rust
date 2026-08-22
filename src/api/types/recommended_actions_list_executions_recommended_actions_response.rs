pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListExecutionsRecommendedActionsResponse {
    /// The chain these executions belong to.
    #[serde(default)]
    pub chain_id: String,
    /// One entry per run step, in position order.
    #[serde(default)]
    pub executions: Vec<RecommendedActionExecution>,
}

impl ListExecutionsRecommendedActionsResponse {
    pub fn builder() -> ListExecutionsRecommendedActionsResponseBuilder {
        <ListExecutionsRecommendedActionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListExecutionsRecommendedActionsResponseBuilder {
    chain_id: Option<String>,
    executions: Option<Vec<RecommendedActionExecution>>,
}

impl ListExecutionsRecommendedActionsResponseBuilder {
    pub fn chain_id(mut self, value: impl Into<String>) -> Self {
        self.chain_id = Some(value.into());
        self
    }

    pub fn executions(mut self, value: Vec<RecommendedActionExecution>) -> Self {
        self.executions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListExecutionsRecommendedActionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chain_id`](ListExecutionsRecommendedActionsResponseBuilder::chain_id)
    /// - [`executions`](ListExecutionsRecommendedActionsResponseBuilder::executions)
    pub fn build(self) -> Result<ListExecutionsRecommendedActionsResponse, BuildError> {
        Ok(ListExecutionsRecommendedActionsResponse {
            chain_id: self
                .chain_id
                .ok_or_else(|| BuildError::missing_field("chain_id"))?,
            executions: self
                .executions
                .ok_or_else(|| BuildError::missing_field("executions"))?,
        })
    }
}
