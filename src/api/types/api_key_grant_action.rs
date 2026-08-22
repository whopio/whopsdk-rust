pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeyGrantAction {
    /// The permission action's identifier, for example `company:basic:read`.
    #[serde(default)]
    pub action: String,
    /// Whether the key holds the action on the grant's resource.
    #[serde(default)]
    pub granted: bool,
}

impl ApiKeyGrantAction {
    pub fn builder() -> ApiKeyGrantActionBuilder {
        <ApiKeyGrantActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyGrantActionBuilder {
    action: Option<String>,
    granted: Option<bool>,
}

impl ApiKeyGrantActionBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn granted(mut self, value: bool) -> Self {
        self.granted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeyGrantAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](ApiKeyGrantActionBuilder::action)
    /// - [`granted`](ApiKeyGrantActionBuilder::granted)
    pub fn build(self) -> Result<ApiKeyGrantAction, BuildError> {
        Ok(ApiKeyGrantAction {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            granted: self
                .granted
                .ok_or_else(|| BuildError::missing_field("granted"))?,
        })
    }
}
