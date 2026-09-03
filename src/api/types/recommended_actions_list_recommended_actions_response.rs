pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListRecommendedActionsResponse {
    #[serde(default)]
    pub data: Vec<AccountRecommendedActionChain>,
    /// Whether generation was queued because the account has no available action chains yet.
    #[serde(default)]
    pub generation_pending: bool,
}

impl ListRecommendedActionsResponse {
    pub fn builder() -> ListRecommendedActionsResponseBuilder {
        <ListRecommendedActionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRecommendedActionsResponseBuilder {
    data: Option<Vec<AccountRecommendedActionChain>>,
    generation_pending: Option<bool>,
}

impl ListRecommendedActionsResponseBuilder {
    pub fn data(mut self, value: Vec<AccountRecommendedActionChain>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn generation_pending(mut self, value: bool) -> Self {
        self.generation_pending = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListRecommendedActionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListRecommendedActionsResponseBuilder::data)
    /// - [`generation_pending`](ListRecommendedActionsResponseBuilder::generation_pending)
    pub fn build(self) -> Result<ListRecommendedActionsResponse, BuildError> {
        Ok(ListRecommendedActionsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            generation_pending: self
                .generation_pending
                .ok_or_else(|| BuildError::missing_field("generation_pending"))?,
        })
    }
}
