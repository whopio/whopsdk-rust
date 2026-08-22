pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeyGrant {
    #[serde(default)]
    pub actions: Vec<ApiKeyGrantAction>,
    /// ID of the resource the actions apply to.
    #[serde(default)]
    pub resource_id: String,
    /// The type of resource the actions apply to, such as `account`, `product`, or `app`.
    #[serde(default)]
    pub resource_type: String,
}

impl ApiKeyGrant {
    pub fn builder() -> ApiKeyGrantBuilder {
        <ApiKeyGrantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyGrantBuilder {
    actions: Option<Vec<ApiKeyGrantAction>>,
    resource_id: Option<String>,
    resource_type: Option<String>,
}

impl ApiKeyGrantBuilder {
    pub fn actions(mut self, value: Vec<ApiKeyGrantAction>) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn resource_type(mut self, value: impl Into<String>) -> Self {
        self.resource_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApiKeyGrant`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actions`](ApiKeyGrantBuilder::actions)
    /// - [`resource_id`](ApiKeyGrantBuilder::resource_id)
    /// - [`resource_type`](ApiKeyGrantBuilder::resource_type)
    pub fn build(self) -> Result<ApiKeyGrant, BuildError> {
        Ok(ApiKeyGrant {
            actions: self
                .actions
                .ok_or_else(|| BuildError::missing_field("actions"))?,
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
            resource_type: self
                .resource_type
                .ok_or_else(|| BuildError::missing_field("resource_type"))?,
        })
    }
}
