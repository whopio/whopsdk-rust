pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListPermissionsResponseDataItem {
    pub action: PermissionAction,
    /// Whether the credential is granted the action for the resource.
    #[serde(default)]
    pub granted: bool,
}

impl ListPermissionsResponseDataItem {
    pub fn builder() -> ListPermissionsResponseDataItemBuilder {
        <ListPermissionsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPermissionsResponseDataItemBuilder {
    action: Option<PermissionAction>,
    granted: Option<bool>,
}

impl ListPermissionsResponseDataItemBuilder {
    pub fn action(mut self, value: PermissionAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn granted(mut self, value: bool) -> Self {
        self.granted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPermissionsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](ListPermissionsResponseDataItemBuilder::action)
    /// - [`granted`](ListPermissionsResponseDataItemBuilder::granted)
    pub fn build(self) -> Result<ListPermissionsResponseDataItem, BuildError> {
        Ok(ListPermissionsResponseDataItem {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            granted: self
                .granted
                .ok_or_else(|| BuildError::missing_field("granted"))?,
        })
    }
}
