pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListWaitlistEntriesResponse {
    #[serde(default)]
    pub data: Vec<WaitlistEntry>,
    #[serde(default)]
    pub page_info: ListWaitlistEntriesResponsePageInfo,
}

impl ListWaitlistEntriesResponse {
    pub fn builder() -> ListWaitlistEntriesResponseBuilder {
        <ListWaitlistEntriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWaitlistEntriesResponseBuilder {
    data: Option<Vec<WaitlistEntry>>,
    page_info: Option<ListWaitlistEntriesResponsePageInfo>,
}

impl ListWaitlistEntriesResponseBuilder {
    pub fn data(mut self, value: Vec<WaitlistEntry>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListWaitlistEntriesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWaitlistEntriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListWaitlistEntriesResponseBuilder::data)
    /// - [`page_info`](ListWaitlistEntriesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListWaitlistEntriesResponse, BuildError> {
        Ok(ListWaitlistEntriesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
