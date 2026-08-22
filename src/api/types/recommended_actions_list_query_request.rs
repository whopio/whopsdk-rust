pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecommendedActionsListQueryRequest {
    /// Account ID, prefixed `biz_`. Defaults to the API key's own account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl RecommendedActionsListQueryRequest {
    pub fn builder() -> RecommendedActionsListQueryRequestBuilder {
        <RecommendedActionsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecommendedActionsListQueryRequestBuilder {
    account_id: Option<String>,
}

impl RecommendedActionsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecommendedActionsListQueryRequest`].
    pub fn build(self) -> Result<RecommendedActionsListQueryRequest, BuildError> {
        Ok(RecommendedActionsListQueryRequest {
            account_id: self.account_id,
        })
    }
}
