pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListUsersResponse {
    #[serde(default)]
    pub data: Vec<User>,
    #[serde(default)]
    pub page_info: ListUsersResponsePageInfo,
}

impl ListUsersResponse {
    pub fn builder() -> ListUsersResponseBuilder {
        <ListUsersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListUsersResponseBuilder {
    data: Option<Vec<User>>,
    page_info: Option<ListUsersResponsePageInfo>,
}

impl ListUsersResponseBuilder {
    pub fn data(mut self, value: Vec<User>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListUsersResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListUsersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListUsersResponseBuilder::data)
    /// - [`page_info`](ListUsersResponseBuilder::page_info)
    pub fn build(self) -> Result<ListUsersResponse, BuildError> {
        Ok(ListUsersResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
