pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamMemberManager {
    /// The manager's team member ID, starting with `ausr_` or `mber_`; treat it as opaque.
    #[serde(default)]
    pub id: String,
    /// The user behind the manager's team membership.
    #[serde(default)]
    pub user: UserSummary,
}

impl TeamMemberManager {
    pub fn builder() -> TeamMemberManagerBuilder {
        <TeamMemberManagerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamMemberManagerBuilder {
    id: Option<String>,
    user: Option<UserSummary>,
}

impl TeamMemberManagerBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn user(mut self, value: UserSummary) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamMemberManager`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TeamMemberManagerBuilder::id)
    /// - [`user`](TeamMemberManagerBuilder::user)
    pub fn build(self) -> Result<TeamMemberManager, BuildError> {
        Ok(TeamMemberManager {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
