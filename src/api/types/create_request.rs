pub use crate::prelude::*;

/// Request for create (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRequest {
    /// Account or user ID whose identity you want to verify. Use a `biz_` account ID for account verifications, or the caller's `user_` ID for personal verification.
    #[serde(skip)]
    #[serde(default)]
    pub account_id: String,
    pub body: CreateVerificationsRequestBody,
}

impl CreateRequest {
    pub fn builder() -> CreateRequestBuilder {
        <CreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequestBuilder {
    account_id: Option<String>,
    body: Option<CreateVerificationsRequestBody>,
}

impl CreateRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn body(mut self, value: CreateVerificationsRequestBody) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateRequestBuilder::account_id)
    /// - [`body`](CreateRequestBuilder::body)
    pub fn build(self) -> Result<CreateRequest, BuildError> {
        Ok(CreateRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
