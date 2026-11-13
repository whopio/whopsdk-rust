pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListNotesResponse {
    #[serde(default)]
    pub data: Vec<MemberNote>,
    #[serde(default)]
    pub page_info: ListNotesResponsePageInfo,
}

impl ListNotesResponse {
    pub fn builder() -> ListNotesResponseBuilder {
        <ListNotesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListNotesResponseBuilder {
    data: Option<Vec<MemberNote>>,
    page_info: Option<ListNotesResponsePageInfo>,
}

impl ListNotesResponseBuilder {
    pub fn data(mut self, value: Vec<MemberNote>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListNotesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListNotesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListNotesResponseBuilder::data)
    /// - [`page_info`](ListNotesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListNotesResponse, BuildError> {
        Ok(ListNotesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
