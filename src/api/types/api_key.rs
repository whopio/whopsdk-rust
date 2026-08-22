pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApiKey {
    /// Dated API version used when requests authenticated with this key omit the `Api-Version-Date` header.
    pub api_version_date: ApiKeyApiVersionDate,
    /// When the API key was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// When the API key stops working, as an ISO 8601 timestamp. `null` means it never expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<ApiKeyGrant>>,
    /// API key ID, prefixed `apik_`.
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_allowlist: Option<Vec<String>>,
    /// Whether this is the resource's default API key. Default keys cannot be updated or deleted, only rotated.
    #[serde(default)]
    pub is_default_for_resource: bool,
    /// Human-readable name identifying the API key, or `null` when none was set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Masked version of the secret key, so the key can be recognized without revealing the full secret.
    #[serde(default)]
    pub obfuscated_secret_key: String,
    /// The full secret used to authenticate requests. Returned only once, on create and rotate responses — store it immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// System role the key inherits its permissions from, or `null` when it uses an explicit permissions policy. Only account API keys can use a system role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_role: Option<ApiKeySystemRole>,
    /// When the API key was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl ApiKey {
    pub fn builder() -> ApiKeyBuilder {
        <ApiKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyBuilder {
    api_version_date: Option<ApiKeyApiVersionDate>,
    created_at: Option<String>,
    expires_at: Option<String>,
    grants: Option<Vec<ApiKeyGrant>>,
    id: Option<String>,
    ip_allowlist: Option<Vec<String>>,
    is_default_for_resource: Option<bool>,
    name: Option<String>,
    obfuscated_secret_key: Option<String>,
    secret_key: Option<String>,
    system_role: Option<ApiKeySystemRole>,
    updated_at: Option<String>,
}

impl ApiKeyBuilder {
    pub fn api_version_date(mut self, value: ApiKeyApiVersionDate) -> Self {
        self.api_version_date = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn grants(mut self, value: Vec<ApiKeyGrant>) -> Self {
        self.grants = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn ip_allowlist(mut self, value: Vec<String>) -> Self {
        self.ip_allowlist = Some(value);
        self
    }

    pub fn is_default_for_resource(mut self, value: bool) -> Self {
        self.is_default_for_resource = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn obfuscated_secret_key(mut self, value: impl Into<String>) -> Self {
        self.obfuscated_secret_key = Some(value.into());
        self
    }

    pub fn secret_key(mut self, value: impl Into<String>) -> Self {
        self.secret_key = Some(value.into());
        self
    }

    pub fn system_role(mut self, value: ApiKeySystemRole) -> Self {
        self.system_role = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApiKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version_date`](ApiKeyBuilder::api_version_date)
    /// - [`created_at`](ApiKeyBuilder::created_at)
    /// - [`id`](ApiKeyBuilder::id)
    /// - [`is_default_for_resource`](ApiKeyBuilder::is_default_for_resource)
    /// - [`obfuscated_secret_key`](ApiKeyBuilder::obfuscated_secret_key)
    /// - [`updated_at`](ApiKeyBuilder::updated_at)
    pub fn build(self) -> Result<ApiKey, BuildError> {
        Ok(ApiKey {
            api_version_date: self
                .api_version_date
                .ok_or_else(|| BuildError::missing_field("api_version_date"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            expires_at: self.expires_at,
            grants: self.grants,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            ip_allowlist: self.ip_allowlist,
            is_default_for_resource: self
                .is_default_for_resource
                .ok_or_else(|| BuildError::missing_field("is_default_for_resource"))?,
            name: self.name,
            obfuscated_secret_key: self
                .obfuscated_secret_key
                .ok_or_else(|| BuildError::missing_field("obfuscated_secret_key"))?,
            secret_key: self.secret_key,
            system_role: self.system_role,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
