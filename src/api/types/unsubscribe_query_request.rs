pub use crate::prelude::*;

/// Query parameters for unsubscribe
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnsubscribeQueryRequest {
    /// The token from the member's unsubscribe link.
    #[serde(default)]
    pub token: String,
}

impl UnsubscribeQueryRequest {
    pub fn builder() -> UnsubscribeQueryRequestBuilder {
        <UnsubscribeQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnsubscribeQueryRequestBuilder {
    token: Option<String>,
}

impl UnsubscribeQueryRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnsubscribeQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](UnsubscribeQueryRequestBuilder::token)
    pub fn build(self) -> Result<UnsubscribeQueryRequest, BuildError> {
        Ok(UnsubscribeQueryRequest {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
