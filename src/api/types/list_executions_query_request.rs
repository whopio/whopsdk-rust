pub use crate::prelude::*;

/// Query parameters for listExecutions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListExecutionsQueryRequest {
    /// Account ID, prefixed `biz_`. Defaults to the API key's own account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl ListExecutionsQueryRequest {
    pub fn builder() -> ListExecutionsQueryRequestBuilder {
        <ListExecutionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListExecutionsQueryRequestBuilder {
    account_id: Option<String>,
}

impl ListExecutionsQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListExecutionsQueryRequest`].
    pub fn build(self) -> Result<ListExecutionsQueryRequest, BuildError> {
        Ok(ListExecutionsQueryRequest {
            account_id: self.account_id,
        })
    }
}
