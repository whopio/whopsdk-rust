pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTeamMembersRequest {
    /// The system role to grant. Custom roles cannot be granted through the API. The Partner role provides view-only access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UpdateTeamMembersRequestRole>,
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
    pub fn build(self) -> Result<UpdateTeamMembersRequest, BuildError> {
        Ok(UpdateTeamMembersRequest { role: self.role })
    }
}
