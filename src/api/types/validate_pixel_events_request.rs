pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidatePixelEventsRequest {
    /// Account to check. Defaults to the authenticated account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// A page to check for the pixel, such as an ad destination. Events from that page settle the answer; if it hasn't sent any lately, it is fetched and its source read for the pixel and the conversion events wired on it. Omit it to check the account from its events alone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ValidatePixelEventsRequest {
    pub fn builder() -> ValidatePixelEventsRequestBuilder {
        <ValidatePixelEventsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidatePixelEventsRequestBuilder {
    account_id: Option<String>,
    url: Option<String>,
}

impl ValidatePixelEventsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ValidatePixelEventsRequest`].
    pub fn build(self) -> Result<ValidatePixelEventsRequest, BuildError> {
        Ok(ValidatePixelEventsRequest {
            account_id: self.account_id,
            url: self.url,
        })
    }
}
