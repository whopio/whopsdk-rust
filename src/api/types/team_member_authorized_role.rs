pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamMemberAuthorizedRole {
    /// Custom role ID, prefixed `aurl_`.
    #[serde(default)]
    pub id: String,
    /// Custom role name.
    #[serde(default)]
    pub name: String,
}

impl TeamMemberAuthorizedRole {
    pub fn builder() -> TeamMemberAuthorizedRoleBuilder {
        <TeamMemberAuthorizedRoleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamMemberAuthorizedRoleBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl TeamMemberAuthorizedRoleBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TeamMemberAuthorizedRole`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TeamMemberAuthorizedRoleBuilder::id)
    /// - [`name`](TeamMemberAuthorizedRoleBuilder::name)
    pub fn build(self) -> Result<TeamMemberAuthorizedRole, BuildError> {
        Ok(TeamMemberAuthorizedRole {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
