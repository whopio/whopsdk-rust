pub use crate::prelude::*;

/// The customer (buyer) who filed this resolution case.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolutionCenterCaseLegacyUser {
    /// The unique identifier for the user.
    #[serde(default)]
    pub id: String,
    /// The user's display name shown on their public profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's unique username shown on their public profile.
    #[serde(default)]
    pub username: String,
}

impl ResolutionCenterCaseLegacyUser {
    pub fn builder() -> ResolutionCenterCaseLegacyUserBuilder {
        <ResolutionCenterCaseLegacyUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionCenterCaseLegacyUserBuilder {
    id: Option<String>,
    name: Option<String>,
    username: Option<String>,
}

impl ResolutionCenterCaseLegacyUserBuilder {
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

    /// Consumes the builder and constructs a [`ResolutionCenterCaseLegacyUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ResolutionCenterCaseLegacyUserBuilder::id)
    /// - [`username`](ResolutionCenterCaseLegacyUserBuilder::username)
    pub fn build(self) -> Result<ResolutionCenterCaseLegacyUser, BuildError> {
        Ok(ResolutionCenterCaseLegacyUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
