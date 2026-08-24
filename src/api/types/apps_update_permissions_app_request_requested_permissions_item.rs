pub use crate::prelude::*;

/// Input for creating a requested permission
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePermissionsAppRequestRequestedPermissionsItem {
    /// The action that the app will request off of users when a user installs the app.
    #[serde(default)]
    pub action: String,
    /// Whether the action is required for the app to function.
    #[serde(default)]
    pub is_required: bool,
    /// The reason for requesting the action.
    #[serde(default)]
    pub justification: String,
}

impl UpdatePermissionsAppRequestRequestedPermissionsItem {
    pub fn builder() -> UpdatePermissionsAppRequestRequestedPermissionsItemBuilder {
        <UpdatePermissionsAppRequestRequestedPermissionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePermissionsAppRequestRequestedPermissionsItemBuilder {
    action: Option<String>,
    is_required: Option<bool>,
    justification: Option<String>,
}

impl UpdatePermissionsAppRequestRequestedPermissionsItemBuilder {
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

    /// Consumes the builder and constructs a [`UpdatePermissionsAppRequestRequestedPermissionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](UpdatePermissionsAppRequestRequestedPermissionsItemBuilder::action)
    /// - [`is_required`](UpdatePermissionsAppRequestRequestedPermissionsItemBuilder::is_required)
    /// - [`justification`](UpdatePermissionsAppRequestRequestedPermissionsItemBuilder::justification)
    pub fn build(self) -> Result<UpdatePermissionsAppRequestRequestedPermissionsItem, BuildError> {
        Ok(UpdatePermissionsAppRequestRequestedPermissionsItem {
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
