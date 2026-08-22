pub use crate::prelude::*;

/// Query parameters for run
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunQueryRequest {
    /// Account ID, prefixed `biz_`. Defaults to the API key's own account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl RunQueryRequest {
    pub fn builder() -> RunQueryRequestBuilder {
        <RunQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunQueryRequestBuilder {
    account_id: Option<String>,
}

impl RunQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunQueryRequest`].
    pub fn build(self) -> Result<RunQueryRequest, BuildError> {
        Ok(RunQueryRequest {
            account_id: self.account_id,
        })
    }
}
