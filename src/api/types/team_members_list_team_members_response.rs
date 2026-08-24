pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTeamMembersResponse {
    #[serde(default)]
    pub data: Vec<TeamMember>,
    #[serde(default)]
    pub page_info: ListTeamMembersResponsePageInfo,
}

impl ListTeamMembersResponse {
    pub fn builder() -> ListTeamMembersResponseBuilder {
        <ListTeamMembersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTeamMembersResponseBuilder {
    data: Option<Vec<TeamMember>>,
    page_info: Option<ListTeamMembersResponsePageInfo>,
}

impl ListTeamMembersResponseBuilder {
    pub fn data(mut self, value: Vec<TeamMember>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListTeamMembersResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTeamMembersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListTeamMembersResponseBuilder::data)
    /// - [`page_info`](ListTeamMembersResponseBuilder::page_info)
    pub fn build(self) -> Result<ListTeamMembersResponse, BuildError> {
        Ok(ListTeamMembersResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
