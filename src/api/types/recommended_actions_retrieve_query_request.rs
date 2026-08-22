pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecommendedActionsRetrieveQueryRequest {
    /// Account ID, prefixed `biz_`. Defaults to the API key's own account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl RecommendedActionsRetrieveQueryRequest {
    pub fn builder() -> RecommendedActionsRetrieveQueryRequestBuilder {
        <RecommendedActionsRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecommendedActionsRetrieveQueryRequestBuilder {
    account_id: Option<String>,
}

impl RecommendedActionsRetrieveQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecommendedActionsRetrieveQueryRequest`].
    pub fn build(self) -> Result<RecommendedActionsRetrieveQueryRequest, BuildError> {
        Ok(RecommendedActionsRetrieveQueryRequest {
            account_id: self.account_id,
        })
    }
}
