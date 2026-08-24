pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OauthGrant {
    /// The account the grant is scoped to, prefixed `biz_`. `null` when the user authorized the app for themselves rather than for one of their accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The app this grant authorizes, prefixed `app_`.
    #[serde(default)]
    pub app_id: String,
    /// When the user last authorized the app, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_at: Option<String>,
    /// When the user first authorized the app, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Grant ID, prefixed `oag_`.
    #[serde(default)]
    pub id: String,
    /// Where to send the user to finish the flow: the `redirect_uri` you supplied with the authorization `code` appended, and `state` when you supplied one. Its scheme, host, port, and path come back exactly as sent — never re-cased or re-encoded — because the client matches them against its registered URI. Returned only once, on create: the code is single-use and expires 10 minutes after it is issued, so redirect immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// When the grant was revoked, as an ISO 8601 timestamp, or `null` while it is still in force. A revoked grant authorizes nothing — treat its `scopes` as no longer granted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
}

impl OauthGrant {
    pub fn builder() -> OauthGrantBuilder {
        <OauthGrantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OauthGrantBuilder {
    account_id: Option<String>,
    app_id: Option<String>,
    authorized_at: Option<String>,
    created_at: Option<String>,
    id: Option<String>,
    redirect_url: Option<String>,
    revoked_at: Option<String>,
    scopes: Option<Vec<String>>,
}

impl OauthGrantBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn authorized_at(mut self, value: impl Into<String>) -> Self {
        self.authorized_at = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn redirect_url(mut self, value: impl Into<String>) -> Self {
        self.redirect_url = Some(value.into());
        self
    }

    pub fn revoked_at(mut self, value: impl Into<String>) -> Self {
        self.revoked_at = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OauthGrant`].
    /// This method will fail if any of the following fields are not set:
    /// - [`app_id`](OauthGrantBuilder::app_id)
    /// - [`created_at`](OauthGrantBuilder::created_at)
    /// - [`id`](OauthGrantBuilder::id)
    /// - [`scopes`](OauthGrantBuilder::scopes)
    pub fn build(self) -> Result<OauthGrant, BuildError> {
        Ok(OauthGrant {
            account_id: self.account_id,
            app_id: self
                .app_id
                .ok_or_else(|| BuildError::missing_field("app_id"))?,
            authorized_at: self.authorized_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            redirect_url: self.redirect_url,
            revoked_at: self.revoked_at,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
        })
    }
}
