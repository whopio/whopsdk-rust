pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMethodsRequest {
    /// New label for the payout method, with at least one non-whitespace character and a maximum of 100 characters.
    #[serde(default)]
    pub nickname: String,
}

impl UpdateMethodsRequest {
    pub fn builder() -> UpdateMethodsRequestBuilder {
        <UpdateMethodsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMethodsRequestBuilder {
    nickname: Option<String>,
}

impl UpdateMethodsRequestBuilder {
    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateMethodsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`nickname`](UpdateMethodsRequestBuilder::nickname)
    pub fn build(self) -> Result<UpdateMethodsRequest, BuildError> {
        Ok(UpdateMethodsRequest {
            nickname: self
                .nickname
                .ok_or_else(|| BuildError::missing_field("nickname"))?,
        })
    }
}
