pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListRecommendedActionsResponse {
    #[serde(default)]
    pub data: Vec<AccountRecommendedActionChain>,
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
}

impl ListRecommendedActionsResponseBuilder {
    pub fn data(mut self, value: Vec<AccountRecommendedActionChain>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListRecommendedActionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListRecommendedActionsResponseBuilder::data)
    pub fn build(self) -> Result<ListRecommendedActionsResponse, BuildError> {
        Ok(ListRecommendedActionsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
