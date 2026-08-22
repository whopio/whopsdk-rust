pub use crate::prelude::*;

/// The connection type for PublicEntry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEntriesResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<EntryListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListEntriesResponse {
    pub fn builder() -> ListEntriesResponseBuilder {
        <ListEntriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEntriesResponseBuilder {
    data: Option<Vec<EntryListItem>>,
    page_info: Option<PageInfo>,
}

impl ListEntriesResponseBuilder {
    pub fn data(mut self, value: Vec<EntryListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEntriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListEntriesResponseBuilder::data)
    /// - [`page_info`](ListEntriesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListEntriesResponse, BuildError> {
        Ok(ListEntriesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
