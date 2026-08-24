pub use crate::prelude::*;

/// The connection type for AuthorizedUser.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAuthorizedUsersResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<AuthorizedUserListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListAuthorizedUsersResponse {
    pub fn builder() -> ListAuthorizedUsersResponseBuilder {
        <ListAuthorizedUsersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAuthorizedUsersResponseBuilder {
    data: Option<Vec<AuthorizedUserListItem>>,
    page_info: Option<PageInfo>,
}

impl ListAuthorizedUsersResponseBuilder {
    pub fn data(mut self, value: Vec<AuthorizedUserListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAuthorizedUsersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAuthorizedUsersResponseBuilder::data)
    /// - [`page_info`](ListAuthorizedUsersResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAuthorizedUsersResponse, BuildError> {
        Ok(ListAuthorizedUsersResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
