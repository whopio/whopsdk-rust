pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SwapsListQueryRequest {
    /// Business account whose swaps to list, prefixed `biz_`. Provide this or `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The caller's own user ID, prefixed `user_`, to list swaps in their personal account. Provide this or `account_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl SwapsListQueryRequest {
    pub fn builder() -> SwapsListQueryRequestBuilder {
        <SwapsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SwapsListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
}

impl SwapsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SwapsListQueryRequest`].
    pub fn build(self) -> Result<SwapsListQueryRequest, BuildError> {
        Ok(SwapsListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
        })
    }
}
