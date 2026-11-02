pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMethodsRequest {
    /// Set to `true` to make this the account's default payout method. `false` is not accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// New label for the payout method, with at least one non-whitespace character and a maximum of 100 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

impl UpdateMethodsRequest {
    pub fn builder() -> UpdateMethodsRequestBuilder {
        <UpdateMethodsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMethodsRequestBuilder {
    is_default: Option<bool>,
    nickname: Option<String>,
}

impl UpdateMethodsRequestBuilder {
    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateMethodsRequest`].
    pub fn build(self) -> Result<UpdateMethodsRequest, BuildError> {
        Ok(UpdateMethodsRequest {
            is_default: self.is_default,
            nickname: self.nickname,
        })
    }
}
