pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPlansResponse {
    #[serde(default)]
    pub data: Vec<PlanListItem>,
    #[serde(default)]
    pub page_info: ListPlansResponsePageInfo,
}

impl ListPlansResponse {
    pub fn builder() -> ListPlansResponseBuilder {
        <ListPlansResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPlansResponseBuilder {
    data: Option<Vec<PlanListItem>>,
    page_info: Option<ListPlansResponsePageInfo>,
}

impl ListPlansResponseBuilder {
    pub fn data(mut self, value: Vec<PlanListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListPlansResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPlansResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPlansResponseBuilder::data)
    /// - [`page_info`](ListPlansResponseBuilder::page_info)
    pub fn build(self) -> Result<ListPlansResponse, BuildError> {
        Ok(ListPlansResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
