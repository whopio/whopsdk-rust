pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPeopleResponse {
    #[serde(default)]
    pub data: Vec<ListPeopleResponseDataItem>,
    #[serde(default)]
    pub page_info: ListPeopleResponsePageInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

impl ListPeopleResponse {
    pub fn builder() -> ListPeopleResponseBuilder {
        <ListPeopleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPeopleResponseBuilder {
    data: Option<Vec<ListPeopleResponseDataItem>>,
    page_info: Option<ListPeopleResponsePageInfo>,
    total_count: Option<i64>,
}

impl ListPeopleResponseBuilder {
    pub fn data(mut self, value: Vec<ListPeopleResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListPeopleResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPeopleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPeopleResponseBuilder::data)
    /// - [`page_info`](ListPeopleResponseBuilder::page_info)
    pub fn build(self) -> Result<ListPeopleResponse, BuildError> {
        Ok(ListPeopleResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
            total_count: self.total_count,
        })
    }
}
