pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutsRetrieveQueryRequest {
    /// Owning account ID, prefixed `biz_`. Provide exactly one of `account_id` or `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Owning user ID, prefixed `user_`. Provide exactly one of `account_id` or `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl PayoutsRetrieveQueryRequest {
    pub fn builder() -> PayoutsRetrieveQueryRequestBuilder {
        <PayoutsRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutsRetrieveQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
}

impl PayoutsRetrieveQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayoutsRetrieveQueryRequest`].
    pub fn build(self) -> Result<PayoutsRetrieveQueryRequest, BuildError> {
        Ok(PayoutsRetrieveQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
        })
    }
}
