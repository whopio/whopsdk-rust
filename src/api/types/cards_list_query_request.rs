pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CardsListQueryRequest {
    /// The owning account ID (a biz_ identifier). Provide this or user_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The owning user ID (a user_ identifier). Provide this or account_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CardsListQueryRequest {
    pub fn builder() -> CardsListQueryRequestBuilder {
        <CardsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardsListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
}

impl CardsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CardsListQueryRequest`].
    pub fn build(self) -> Result<CardsListQueryRequest, BuildError> {
        Ok(CardsListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
        })
    }
}
