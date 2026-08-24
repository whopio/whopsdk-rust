pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePermissionsAppRequest {
    /// The permissions that the app will request off of users when a user installs the app.
    #[serde(default)]
    pub requested_permissions: Vec<UpdatePermissionsAppRequestRequestedPermissionsItem>,
}

impl UpdatePermissionsAppRequest {
    pub fn builder() -> UpdatePermissionsAppRequestBuilder {
        <UpdatePermissionsAppRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePermissionsAppRequestBuilder {
    requested_permissions: Option<Vec<UpdatePermissionsAppRequestRequestedPermissionsItem>>,
}

impl UpdatePermissionsAppRequestBuilder {
    pub fn requested_permissions(
        mut self,
        value: Vec<UpdatePermissionsAppRequestRequestedPermissionsItem>,
    ) -> Self {
        self.requested_permissions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePermissionsAppRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requested_permissions`](UpdatePermissionsAppRequestBuilder::requested_permissions)
    pub fn build(self) -> Result<UpdatePermissionsAppRequest, BuildError> {
        Ok(UpdatePermissionsAppRequest {
            requested_permissions: self
                .requested_permissions
                .ok_or_else(|| BuildError::missing_field("requested_permissions"))?,
        })
    }
}
