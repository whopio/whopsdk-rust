pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SwapsListQueryRequest {
    /// Business or user account ID (biz_* / user_*).
    #[serde(default)]
    pub account_id: String,
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
}

impl SwapsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SwapsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](SwapsListQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<SwapsListQueryRequest, BuildError> {
        Ok(SwapsListQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
        })
    }
}
