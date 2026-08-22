pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBountySubmissionsResponse {
    #[serde(default)]
    pub data: Vec<BountySubmission>,
    #[serde(default)]
    pub page_info: ListBountySubmissionsResponsePageInfo,
}

impl ListBountySubmissionsResponse {
    pub fn builder() -> ListBountySubmissionsResponseBuilder {
        <ListBountySubmissionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBountySubmissionsResponseBuilder {
    data: Option<Vec<BountySubmission>>,
    page_info: Option<ListBountySubmissionsResponsePageInfo>,
}

impl ListBountySubmissionsResponseBuilder {
    pub fn data(mut self, value: Vec<BountySubmission>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListBountySubmissionsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBountySubmissionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListBountySubmissionsResponseBuilder::data)
    /// - [`page_info`](ListBountySubmissionsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListBountySubmissionsResponse, BuildError> {
        Ok(ListBountySubmissionsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
