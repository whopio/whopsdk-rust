pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReverseDepositsRequest {
    /// The business that received the deposit, prefixed `biz_`. Pass this or `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Custom metadata to store with the reversal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<ReverseDepositsRequestMetadataValue>>>,
    /// The user that received the deposit, prefixed `user_`. Pass this or `account_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ReverseDepositsRequest {
    pub fn builder() -> ReverseDepositsRequestBuilder {
        <ReverseDepositsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReverseDepositsRequestBuilder {
    account_id: Option<String>,
    metadata: Option<HashMap<String, Option<ReverseDepositsRequestMetadataValue>>>,
    user_id: Option<String>,
}

impl ReverseDepositsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn metadata(
        mut self,
        value: HashMap<String, Option<ReverseDepositsRequestMetadataValue>>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReverseDepositsRequest`].
    pub fn build(self) -> Result<ReverseDepositsRequest, BuildError> {
        Ok(ReverseDepositsRequest {
            account_id: self.account_id,
            metadata: self.metadata,
            user_id: self.user_id,
        })
    }
}
