pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectExternalAccountsResponse {
    /// The OAuth authorization URL to redirect the user to.
    #[serde(default)]
    pub authorize_url: String,
}

impl ConnectExternalAccountsResponse {
    pub fn builder() -> ConnectExternalAccountsResponseBuilder {
        <ConnectExternalAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectExternalAccountsResponseBuilder {
    authorize_url: Option<String>,
}

impl ConnectExternalAccountsResponseBuilder {
    pub fn authorize_url(mut self, value: impl Into<String>) -> Self {
        self.authorize_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectExternalAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`authorize_url`](ConnectExternalAccountsResponseBuilder::authorize_url)
    pub fn build(self) -> Result<ConnectExternalAccountsResponse, BuildError> {
        Ok(ConnectExternalAccountsResponse {
            authorize_url: self
                .authorize_url
                .ok_or_else(|| BuildError::missing_field("authorize_url"))?,
        })
    }
}
