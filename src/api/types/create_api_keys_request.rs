pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateApiKeysRequest {
    /// Dated API version used when requests authenticated with this key omit the `Api-Version-Date` header. New keys default to the latest version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_date: Option<CreateApiKeysRequestApiVersionDate>,
    /// When the API key should stop working, as an ISO 8601 timestamp. Omit (or pass `null` on update) for a key that never expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// IPv4/IPv6 CIDR ranges allowed to use this key, for example `["203.0.113.0/24"]`. Empty or `null` allows any IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_allowlist: Option<Vec<String>>,
    /// A human-readable name for the API key, such as 'Production API Key'.
    #[serde(default)]
    pub name: String,
    /// The permissions policy for the API key: explicit permission statements, or a system role to inherit from. Statements without a `resources` array default to the owning account (Account API keys) or every key-addressable resource (App API keys).
    #[serde(default)]
    pub permissions: CreateApiKeysRequestPermissions,
    /// The account (`biz_`) or app (`app_`) tag to create the API key for.
    #[serde(default)]
    pub resource_id: String,
    /// The type of resource that will own this API key.
    pub resource_type: CreateApiKeysRequestResourceType,
}

impl CreateApiKeysRequest {
    pub fn builder() -> CreateApiKeysRequestBuilder {
        <CreateApiKeysRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApiKeysRequestBuilder {
    api_version_date: Option<CreateApiKeysRequestApiVersionDate>,
    expires_at: Option<String>,
    ip_allowlist: Option<Vec<String>>,
    name: Option<String>,
    permissions: Option<CreateApiKeysRequestPermissions>,
    resource_id: Option<String>,
    resource_type: Option<CreateApiKeysRequestResourceType>,
}

impl CreateApiKeysRequestBuilder {
    pub fn api_version_date(mut self, value: CreateApiKeysRequestApiVersionDate) -> Self {
        self.api_version_date = Some(value);
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn ip_allowlist(mut self, value: Vec<String>) -> Self {
        self.ip_allowlist = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn permissions(mut self, value: CreateApiKeysRequestPermissions) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn resource_type(mut self, value: CreateApiKeysRequestResourceType) -> Self {
        self.resource_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateApiKeysRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateApiKeysRequestBuilder::name)
    /// - [`permissions`](CreateApiKeysRequestBuilder::permissions)
    /// - [`resource_id`](CreateApiKeysRequestBuilder::resource_id)
    /// - [`resource_type`](CreateApiKeysRequestBuilder::resource_type)
    pub fn build(self) -> Result<CreateApiKeysRequest, BuildError> {
        Ok(CreateApiKeysRequest {
            api_version_date: self.api_version_date,
            expires_at: self.expires_at,
            ip_allowlist: self.ip_allowlist,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            permissions: self
                .permissions
                .ok_or_else(|| BuildError::missing_field("permissions"))?,
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
            resource_type: self
                .resource_type
                .ok_or_else(|| BuildError::missing_field("resource_type"))?,
        })
    }
}
