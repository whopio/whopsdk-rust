pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePermissionsAppsRequestRequestedPermissionsItem {
    /// The permission action, for example `company:basic:read`.
    #[serde(default)]
    pub action: String,
    /// Whether installing the app requires granting this permission.
    #[serde(default)]
    pub is_required: bool,
    /// Why the app needs this permission (20-512 characters), shown to the installing user.
    #[serde(default)]
    pub justification: String,
}

impl UpdatePermissionsAppsRequestRequestedPermissionsItem {
    pub fn builder() -> UpdatePermissionsAppsRequestRequestedPermissionsItemBuilder {
        <UpdatePermissionsAppsRequestRequestedPermissionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePermissionsAppsRequestRequestedPermissionsItemBuilder {
    action: Option<String>,
    is_required: Option<bool>,
    justification: Option<String>,
}

impl UpdatePermissionsAppsRequestRequestedPermissionsItemBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn is_required(mut self, value: bool) -> Self {
        self.is_required = Some(value);
        self
    }

    pub fn justification(mut self, value: impl Into<String>) -> Self {
        self.justification = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePermissionsAppsRequestRequestedPermissionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](UpdatePermissionsAppsRequestRequestedPermissionsItemBuilder::action)
    /// - [`is_required`](UpdatePermissionsAppsRequestRequestedPermissionsItemBuilder::is_required)
    /// - [`justification`](UpdatePermissionsAppsRequestRequestedPermissionsItemBuilder::justification)
    pub fn build(self) -> Result<UpdatePermissionsAppsRequestRequestedPermissionsItem, BuildError> {
        Ok(UpdatePermissionsAppsRequestRequestedPermissionsItem {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            is_required: self
                .is_required
                .ok_or_else(|| BuildError::missing_field("is_required"))?,
            justification: self
                .justification
                .ok_or_else(|| BuildError::missing_field("justification"))?,
        })
    }
}
