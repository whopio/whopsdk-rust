pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateTeamMembersRequest {
    /// The system role to grant.
    pub role: UpdateTeamMembersRequestRole,
}

impl UpdateTeamMembersRequest {
    pub fn builder() -> UpdateTeamMembersRequestBuilder {
        <UpdateTeamMembersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTeamMembersRequestBuilder {
    role: Option<UpdateTeamMembersRequestRole>,
}

impl UpdateTeamMembersRequestBuilder {
    pub fn role(mut self, value: UpdateTeamMembersRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTeamMembersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](UpdateTeamMembersRequestBuilder::role)
    pub fn build(self) -> Result<UpdateTeamMembersRequest, BuildError> {
        Ok(UpdateTeamMembersRequest {
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
