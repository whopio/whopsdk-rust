pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateReturnUrlSetupIntentsRequest {
    /// Where the buyer continues after completing an off-site step. Must be an absolute https URL without credentials (http is allowed for localhost), at most 2,048 characters.
    #[serde(default)]
    pub return_url: String,
}

impl UpdateReturnUrlSetupIntentsRequest {
    pub fn builder() -> UpdateReturnUrlSetupIntentsRequestBuilder {
        <UpdateReturnUrlSetupIntentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateReturnUrlSetupIntentsRequestBuilder {
    return_url: Option<String>,
}

impl UpdateReturnUrlSetupIntentsRequestBuilder {
    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateReturnUrlSetupIntentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`return_url`](UpdateReturnUrlSetupIntentsRequestBuilder::return_url)
    pub fn build(self) -> Result<UpdateReturnUrlSetupIntentsRequest, BuildError> {
        Ok(UpdateReturnUrlSetupIntentsRequest {
            return_url: self
                .return_url
                .ok_or_else(|| BuildError::missing_field("return_url"))?,
        })
    }
}
