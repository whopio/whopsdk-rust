pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListReviewReportsResponse {
    #[serde(default)]
    pub data: Vec<ReviewReport>,
    #[serde(default)]
    pub page_info: ListReviewReportsResponsePageInfo,
}

impl ListReviewReportsResponse {
    pub fn builder() -> ListReviewReportsResponseBuilder {
        <ListReviewReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListReviewReportsResponseBuilder {
    data: Option<Vec<ReviewReport>>,
    page_info: Option<ListReviewReportsResponsePageInfo>,
}

impl ListReviewReportsResponseBuilder {
    pub fn data(mut self, value: Vec<ReviewReport>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListReviewReportsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListReviewReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListReviewReportsResponseBuilder::data)
    /// - [`page_info`](ListReviewReportsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListReviewReportsResponse, BuildError> {
        Ok(ListReviewReportsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
