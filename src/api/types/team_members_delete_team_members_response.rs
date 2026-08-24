pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteTeamMembersResponse {
    #[serde(default)]
    pub success: bool,
}

impl DeleteTeamMembersResponse {
    pub fn builder() -> DeleteTeamMembersResponseBuilder {
        <DeleteTeamMembersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteTeamMembersResponseBuilder {
    success: Option<bool>,
}

impl DeleteTeamMembersResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteTeamMembersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](DeleteTeamMembersResponseBuilder::success)
    pub fn build(self) -> Result<DeleteTeamMembersResponse, BuildError> {
        Ok(DeleteTeamMembersResponse {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
