pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePermissionsAppsRequest {
    /// The full set of permissions the app requests on install; permissions not listed are removed.
    #[serde(default)]
    pub requested_permissions: Vec<UpdatePermissionsAppsRequestRequestedPermissionsItem>,
}

impl UpdatePermissionsAppsRequest {
    pub fn builder() -> UpdatePermissionsAppsRequestBuilder {
        <UpdatePermissionsAppsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePermissionsAppsRequestBuilder {
    requested_permissions: Option<Vec<UpdatePermissionsAppsRequestRequestedPermissionsItem>>,
}

impl UpdatePermissionsAppsRequestBuilder {
    pub fn requested_permissions(
        mut self,
        value: Vec<UpdatePermissionsAppsRequestRequestedPermissionsItem>,
    ) -> Self {
        self.requested_permissions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePermissionsAppsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requested_permissions`](UpdatePermissionsAppsRequestBuilder::requested_permissions)
    pub fn build(self) -> Result<UpdatePermissionsAppsRequest, BuildError> {
        Ok(UpdatePermissionsAppsRequest {
            requested_permissions: self
                .requested_permissions
                .ok_or_else(|| BuildError::missing_field("requested_permissions"))?,
        })
    }
}
