pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CardTransactionsRetrieveQueryRequest {
    /// The account that owns the transaction, prefixed `biz_`. Defaults to the credential's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl CardTransactionsRetrieveQueryRequest {
    pub fn builder() -> CardTransactionsRetrieveQueryRequestBuilder {
        <CardTransactionsRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardTransactionsRetrieveQueryRequestBuilder {
    account_id: Option<String>,
}

impl CardTransactionsRetrieveQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CardTransactionsRetrieveQueryRequest`].
    pub fn build(self) -> Result<CardTransactionsRetrieveQueryRequest, BuildError> {
        Ok(CardTransactionsRetrieveQueryRequest {
            account_id: self.account_id,
        })
    }
}
