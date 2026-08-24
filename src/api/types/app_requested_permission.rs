pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppRequestedPermission {
    /// Whether the app requires the permission to be granted on install, as opposed to requesting it optionally.
    #[serde(default)]
    pub is_required: bool,
    /// The developer's explanation of why the app needs the permission, or `null` when none was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    /// The permission action the app requests.
    #[serde(default)]
    pub permission_action: AppRequestedPermissionAction,
}

impl AppRequestedPermission {
    pub fn builder() -> AppRequestedPermissionBuilder {
        <AppRequestedPermissionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppRequestedPermissionBuilder {
    is_required: Option<bool>,
    justification: Option<String>,
    permission_action: Option<AppRequestedPermissionAction>,
}

impl AppRequestedPermissionBuilder {
    pub fn is_required(mut self, value: bool) -> Self {
        self.is_required = Some(value);
        self
    }

    pub fn justification(mut self, value: impl Into<String>) -> Self {
        self.justification = Some(value.into());
        self
    }

    pub fn permission_action(mut self, value: AppRequestedPermissionAction) -> Self {
        self.permission_action = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppRequestedPermission`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_required`](AppRequestedPermissionBuilder::is_required)
    /// - [`permission_action`](AppRequestedPermissionBuilder::permission_action)
    pub fn build(self) -> Result<AppRequestedPermission, BuildError> {
        Ok(AppRequestedPermission {
            is_required: self
                .is_required
                .ok_or_else(|| BuildError::missing_field("is_required"))?,
            justification: self.justification,
            permission_action: self
                .permission_action
                .ok_or_else(|| BuildError::missing_field("permission_action"))?,
        })
    }
}
