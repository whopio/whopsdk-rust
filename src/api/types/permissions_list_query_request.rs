pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PermissionsListQueryRequest {
    /// Tag of the resource to check against: an account (`biz_`), product (`prod_`), experience (`exp_`), or app (`app_`). A resource the credential cannot see is reported as granted nothing rather than as an error.
    #[serde(default)]
    pub resource_id: String,
    /// Comma-separated permission actions to check, for example `stats:read,payment:basic:read`. Every action is returned when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
}

impl PermissionsListQueryRequest {
    pub fn builder() -> PermissionsListQueryRequestBuilder {
        <PermissionsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PermissionsListQueryRequestBuilder {
    resource_id: Option<String>,
    actions: Option<String>,
}

impl PermissionsListQueryRequestBuilder {
    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn actions(mut self, value: impl Into<String>) -> Self {
        self.actions = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PermissionsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource_id`](PermissionsListQueryRequestBuilder::resource_id)
    pub fn build(self) -> Result<PermissionsListQueryRequest, BuildError> {
        Ok(PermissionsListQueryRequest {
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
            actions: self.actions,
        })
    }
}
