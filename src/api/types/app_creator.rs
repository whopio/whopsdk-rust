pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppCreator {
    /// User ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// Display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Public username.
    #[serde(default)]
    pub username: String,
}

impl AppCreator {
    pub fn builder() -> AppCreatorBuilder {
        <AppCreatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppCreatorBuilder {
    id: Option<String>,
    name: Option<String>,
    username: Option<String>,
}

impl AppCreatorBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AppCreator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AppCreatorBuilder::id)
    /// - [`username`](AppCreatorBuilder::username)
    pub fn build(self) -> Result<AppCreator, BuildError> {
        Ok(AppCreator {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
