pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateOauthGrantsRequest {
    /// Authorize the app for one of the user's accounts rather than for the user alone, prefixed `biz_`. The user must have access to it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The app being authorized, prefixed `app_`.
    #[serde(default)]
    pub client_id: String,
    /// The PKCE code challenge: the base64url-encoded SHA-256 of your code verifier, without padding.
    #[serde(default)]
    pub code_challenge: String,
    /// How `code_challenge` was derived. Only `S256` is accepted.
    pub code_challenge_method: CreateOauthGrantsRequestCodeChallengeMethod,
    /// Whether the consent UI listed these scopes for the user. Sending `false` succeeds only when the user has already granted every scope requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_shown: Option<bool>,
    /// OIDC nonce, echoed into the resulting ID token. Required when `requested_scopes` includes `openid`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// Where to send the user once they have consented. Must match one of the app's registered redirect URIs exactly — it is compared as a string, not normalized.
    #[serde(default)]
    pub redirect_uri: String,
    /// The permissions the app is asking for, for example `member:basic:read`. `GET /api_keys/permissions` names and describes each one. Granting adds to whatever the user already granted this app rather than replacing it.
    #[serde(default)]
    pub requested_scopes: Vec<String>,
    /// The OAuth response type. Only `code` is accepted; defaults to `code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<CreateOauthGrantsRequestResponseType>,
    /// Opaque value appended to `redirect_url` unchanged, for the client to correlate the response with its request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl CreateOauthGrantsRequest {
    pub fn builder() -> CreateOauthGrantsRequestBuilder {
        <CreateOauthGrantsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOauthGrantsRequestBuilder {
    account_id: Option<String>,
    client_id: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<CreateOauthGrantsRequestCodeChallengeMethod>,
    consent_shown: Option<bool>,
    nonce: Option<String>,
    redirect_uri: Option<String>,
    requested_scopes: Option<Vec<String>>,
    response_type: Option<CreateOauthGrantsRequestResponseType>,
    state: Option<String>,
}

impl CreateOauthGrantsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn code_challenge(mut self, value: impl Into<String>) -> Self {
        self.code_challenge = Some(value.into());
        self
    }

    pub fn code_challenge_method(
        mut self,
        value: CreateOauthGrantsRequestCodeChallengeMethod,
    ) -> Self {
        self.code_challenge_method = Some(value);
        self
    }

    pub fn consent_shown(mut self, value: bool) -> Self {
        self.consent_shown = Some(value);
        self
    }

    pub fn nonce(mut self, value: impl Into<String>) -> Self {
        self.nonce = Some(value.into());
        self
    }

    pub fn redirect_uri(mut self, value: impl Into<String>) -> Self {
        self.redirect_uri = Some(value.into());
        self
    }

    pub fn requested_scopes(mut self, value: Vec<String>) -> Self {
        self.requested_scopes = Some(value);
        self
    }

    pub fn response_type(mut self, value: CreateOauthGrantsRequestResponseType) -> Self {
        self.response_type = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOauthGrantsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](CreateOauthGrantsRequestBuilder::client_id)
    /// - [`code_challenge`](CreateOauthGrantsRequestBuilder::code_challenge)
    /// - [`code_challenge_method`](CreateOauthGrantsRequestBuilder::code_challenge_method)
    /// - [`redirect_uri`](CreateOauthGrantsRequestBuilder::redirect_uri)
    /// - [`requested_scopes`](CreateOauthGrantsRequestBuilder::requested_scopes)
    pub fn build(self) -> Result<CreateOauthGrantsRequest, BuildError> {
        Ok(CreateOauthGrantsRequest {
            account_id: self.account_id,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            code_challenge: self
                .code_challenge
                .ok_or_else(|| BuildError::missing_field("code_challenge"))?,
            code_challenge_method: self
                .code_challenge_method
                .ok_or_else(|| BuildError::missing_field("code_challenge_method"))?,
            consent_shown: self.consent_shown,
            nonce: self.nonce,
            redirect_uri: self
                .redirect_uri
                .ok_or_else(|| BuildError::missing_field("redirect_uri"))?,
            requested_scopes: self
                .requested_scopes
                .ok_or_else(|| BuildError::missing_field("requested_scopes"))?,
            response_type: self.response_type,
            state: self.state,
        })
    }
}
