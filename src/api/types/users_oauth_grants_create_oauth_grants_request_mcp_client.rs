pub use crate::prelude::*;

/// The downstream MCP client displayed on the consent screen. Requires explicit consent even when the upstream app already has a grant. Bound to the authorization code and returned on token exchange so the MCP server can verify the approved client.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOauthGrantsRequestMcpClient {
    /// The requesting MCP client's identifier.
    #[serde(default)]
    pub client_id: String,
    /// The requesting MCP client's display name.
    #[serde(default)]
    pub client_name: String,
    /// The downstream redirect URI displayed to the user.
    #[serde(default)]
    pub redirect_uri: String,
}

impl CreateOauthGrantsRequestMcpClient {
    pub fn builder() -> CreateOauthGrantsRequestMcpClientBuilder {
        <CreateOauthGrantsRequestMcpClientBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOauthGrantsRequestMcpClientBuilder {
    client_id: Option<String>,
    client_name: Option<String>,
    redirect_uri: Option<String>,
}

impl CreateOauthGrantsRequestMcpClientBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_name(mut self, value: impl Into<String>) -> Self {
        self.client_name = Some(value.into());
        self
    }

    pub fn redirect_uri(mut self, value: impl Into<String>) -> Self {
        self.redirect_uri = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOauthGrantsRequestMcpClient`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](CreateOauthGrantsRequestMcpClientBuilder::client_id)
    /// - [`client_name`](CreateOauthGrantsRequestMcpClientBuilder::client_name)
    /// - [`redirect_uri`](CreateOauthGrantsRequestMcpClientBuilder::redirect_uri)
    pub fn build(self) -> Result<CreateOauthGrantsRequestMcpClient, BuildError> {
        Ok(CreateOauthGrantsRequestMcpClient {
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            client_name: self
                .client_name
                .ok_or_else(|| BuildError::missing_field("client_name"))?,
            redirect_uri: self
                .redirect_uri
                .ok_or_else(|| BuildError::missing_field("redirect_uri"))?,
        })
    }
}
