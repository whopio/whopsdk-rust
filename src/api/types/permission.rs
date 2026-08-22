pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Permission {
    /// The permission action's identifier, for example `company:basic:read`.
    #[serde(default)]
    pub action: String,
    /// Whether an API key can be granted the permission.
    #[serde(default)]
    pub allowed_on_api_key: bool,
    /// Whether an app can request and be granted the permission.
    #[serde(default)]
    pub allowed_on_app: bool,
    /// Whether the permission can be granted to user tokens.
    #[serde(default)]
    pub allowed_on_user: bool,
    /// The category the action is grouped under, or `null` when uncategorized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// What granting the action allows.
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub granted_to_system_roles: Vec<PermissionGrantedToSystemRolesItem>,
    /// Human-readable name of the action.
    #[serde(default)]
    pub name: String,
}

impl Permission {
    pub fn builder() -> PermissionBuilder {
        <PermissionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PermissionBuilder {
    action: Option<String>,
    allowed_on_api_key: Option<bool>,
    allowed_on_app: Option<bool>,
    allowed_on_user: Option<bool>,
    category: Option<String>,
    description: Option<String>,
    granted_to_system_roles: Option<Vec<PermissionGrantedToSystemRolesItem>>,
    name: Option<String>,
}

impl PermissionBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn allowed_on_api_key(mut self, value: bool) -> Self {
        self.allowed_on_api_key = Some(value);
        self
    }

    pub fn allowed_on_app(mut self, value: bool) -> Self {
        self.allowed_on_app = Some(value);
        self
    }

    pub fn allowed_on_user(mut self, value: bool) -> Self {
        self.allowed_on_user = Some(value);
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn granted_to_system_roles(
        mut self,
        value: Vec<PermissionGrantedToSystemRolesItem>,
    ) -> Self {
        self.granted_to_system_roles = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Permission`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](PermissionBuilder::action)
    /// - [`allowed_on_api_key`](PermissionBuilder::allowed_on_api_key)
    /// - [`allowed_on_app`](PermissionBuilder::allowed_on_app)
    /// - [`allowed_on_user`](PermissionBuilder::allowed_on_user)
    /// - [`description`](PermissionBuilder::description)
    /// - [`granted_to_system_roles`](PermissionBuilder::granted_to_system_roles)
    /// - [`name`](PermissionBuilder::name)
    pub fn build(self) -> Result<Permission, BuildError> {
        Ok(Permission {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            allowed_on_api_key: self
                .allowed_on_api_key
                .ok_or_else(|| BuildError::missing_field("allowed_on_api_key"))?,
            allowed_on_app: self
                .allowed_on_app
                .ok_or_else(|| BuildError::missing_field("allowed_on_app"))?,
            allowed_on_user: self
                .allowed_on_user
                .ok_or_else(|| BuildError::missing_field("allowed_on_user"))?,
            category: self.category,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            granted_to_system_roles: self
                .granted_to_system_roles
                .ok_or_else(|| BuildError::missing_field("granted_to_system_roles"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
