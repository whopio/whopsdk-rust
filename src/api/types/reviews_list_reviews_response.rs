pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListReviewsResponse {
    #[serde(default)]
    pub data: Vec<ReviewListItem>,
    #[serde(default)]
    pub page_info: ListReviewsResponsePageInfo,
}

impl ListReviewsResponse {
    pub fn builder() -> ListReviewsResponseBuilder {
        <ListReviewsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListReviewsResponseBuilder {
    data: Option<Vec<ReviewListItem>>,
    page_info: Option<ListReviewsResponsePageInfo>,
}

impl ListReviewsResponseBuilder {
    pub fn data(mut self, value: Vec<ReviewListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListReviewsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListReviewsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListReviewsResponseBuilder::data)
    /// - [`page_info`](ListReviewsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListReviewsResponse, BuildError> {
        Ok(ListReviewsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
