pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePaypalLinkSessionsResponse {
    /// The PayPal login page to send the account holder to.
    #[serde(default)]
    pub authorize_url: String,
    /// When the login session stops being accepted, as an ISO 8601 timestamp; start a new one after this.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
}

impl CreatePaypalLinkSessionsResponse {
    pub fn builder() -> CreatePaypalLinkSessionsResponseBuilder {
        <CreatePaypalLinkSessionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaypalLinkSessionsResponseBuilder {
    authorize_url: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl CreatePaypalLinkSessionsResponseBuilder {
    pub fn authorize_url(mut self, value: impl Into<String>) -> Self {
        self.authorize_url = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaypalLinkSessionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`authorize_url`](CreatePaypalLinkSessionsResponseBuilder::authorize_url)
    /// - [`expires_at`](CreatePaypalLinkSessionsResponseBuilder::expires_at)
    pub fn build(self) -> Result<CreatePaypalLinkSessionsResponse, BuildError> {
        Ok(CreatePaypalLinkSessionsResponse {
            authorize_url: self
                .authorize_url
                .ok_or_else(|| BuildError::missing_field("authorize_url"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
