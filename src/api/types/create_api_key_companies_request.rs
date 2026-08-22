pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateApiKeyCompaniesRequest {
    /// The unique identifier of the connected account to create the API key for (e.g. 'biz_xxx').
    #[serde(default)]
    pub child_company_id: String,
    /// A human-readable name for the API key, such as 'Production API Key'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Granular permission statements defining which actions this API key can perform. Either permissions or role must be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<CreateApiKeyCompaniesRequestPermissionsItem>>,
    /// A system role to inherit permissions from (e.g. owner, admin, moderator). Either role or permissions must be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<PermissionSystemRoles>,
}

impl CreateApiKeyCompaniesRequest {
    pub fn builder() -> CreateApiKeyCompaniesRequestBuilder {
        <CreateApiKeyCompaniesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApiKeyCompaniesRequestBuilder {
    child_company_id: Option<String>,
    name: Option<String>,
    permissions: Option<Vec<CreateApiKeyCompaniesRequestPermissionsItem>>,
    role: Option<PermissionSystemRoles>,
}

impl CreateApiKeyCompaniesRequestBuilder {
    pub fn child_company_id(mut self, value: impl Into<String>) -> Self {
        self.child_company_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn permissions(mut self, value: Vec<CreateApiKeyCompaniesRequestPermissionsItem>) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn role(mut self, value: PermissionSystemRoles) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateApiKeyCompaniesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`child_company_id`](CreateApiKeyCompaniesRequestBuilder::child_company_id)
    pub fn build(self) -> Result<CreateApiKeyCompaniesRequest, BuildError> {
        Ok(CreateApiKeyCompaniesRequest {
            child_company_id: self
                .child_company_id
                .ok_or_else(|| BuildError::missing_field("child_company_id"))?,
            name: self.name,
            permissions: self.permissions,
            role: self.role,
        })
    }
}
