pub use crate::prelude::*;

/// Query parameters for cancel
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelQueryRequest {
    /// Owning user ID, prefixed `user_`. Provide exactly one of `account_id` or `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CancelQueryRequest {
    pub fn builder() -> CancelQueryRequestBuilder {
        <CancelQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelQueryRequestBuilder {
    user_id: Option<String>,
}

impl CancelQueryRequestBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelQueryRequest`].
    pub fn build(self) -> Result<CancelQueryRequest, BuildError> {
        Ok(CancelQueryRequest {
            user_id: self.user_id,
        })
    }
}
