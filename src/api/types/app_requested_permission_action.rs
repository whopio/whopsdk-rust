pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppRequestedPermissionAction {
    /// The permission action's identifier, for example `company:basic:read`.
    #[serde(default)]
    pub action: String,
    /// Human-readable name of the action.
    #[serde(default)]
    pub name: String,
}

impl AppRequestedPermissionAction {
    pub fn builder() -> AppRequestedPermissionActionBuilder {
        <AppRequestedPermissionActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppRequestedPermissionActionBuilder {
    action: Option<String>,
    name: Option<String>,
}

impl AppRequestedPermissionActionBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AppRequestedPermissionAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](AppRequestedPermissionActionBuilder::action)
    /// - [`name`](AppRequestedPermissionActionBuilder::name)
    pub fn build(self) -> Result<AppRequestedPermissionAction, BuildError> {
        Ok(AppRequestedPermissionAction {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
