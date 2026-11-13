pub use crate::prelude::*;

/// Request for create (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRequest {
    /// The caller's own user ID, prefixed `user_`, for a personal verification. Provide this or `account_id`.
    #[serde(skip)]
    pub user_id: Option<String>,
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
    user_id: Option<String>,
    body: Option<CreateVerificationsRequestBody>,
}

impl CreateRequestBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn body(mut self, value: CreateVerificationsRequestBody) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](CreateRequestBuilder::body)
    pub fn build(self) -> Result<CreateRequest, BuildError> {
        Ok(CreateRequest {
            user_id: self.user_id,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
