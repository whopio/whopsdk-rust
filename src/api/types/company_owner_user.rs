pub use crate::prelude::*;

/// The user who owns and has full administrative control over this company.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompanyOwnerUser {
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

impl CompanyOwnerUser {
    pub fn builder() -> CompanyOwnerUserBuilder {
        <CompanyOwnerUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyOwnerUserBuilder {
    id: Option<String>,
    name: Option<String>,
    username: Option<String>,
}

impl CompanyOwnerUserBuilder {
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

    /// Consumes the builder and constructs a [`CompanyOwnerUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompanyOwnerUserBuilder::id)
    /// - [`username`](CompanyOwnerUserBuilder::username)
    pub fn build(self) -> Result<CompanyOwnerUser, BuildError> {
        Ok(CompanyOwnerUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
