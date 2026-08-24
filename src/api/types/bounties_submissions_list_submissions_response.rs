pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSubmissionsResponse {
    #[serde(default)]
    pub data: Vec<PublicBountySubmission>,
    #[serde(default)]
    pub page_info: ListSubmissionsResponsePageInfo,
}

impl ListSubmissionsResponse {
    pub fn builder() -> ListSubmissionsResponseBuilder {
        <ListSubmissionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSubmissionsResponseBuilder {
    data: Option<Vec<PublicBountySubmission>>,
    page_info: Option<ListSubmissionsResponsePageInfo>,
}

impl ListSubmissionsResponseBuilder {
    pub fn data(mut self, value: Vec<PublicBountySubmission>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListSubmissionsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSubmissionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListSubmissionsResponseBuilder::data)
    /// - [`page_info`](ListSubmissionsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListSubmissionsResponse, BuildError> {
        Ok(ListSubmissionsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
