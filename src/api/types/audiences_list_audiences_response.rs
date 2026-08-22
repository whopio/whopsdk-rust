pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAudiencesResponse {
    #[serde(default)]
    pub data: Vec<Audience>,
    #[serde(default)]
    pub page_info: ListAudiencesResponsePageInfo,
}

impl ListAudiencesResponse {
    pub fn builder() -> ListAudiencesResponseBuilder {
        <ListAudiencesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAudiencesResponseBuilder {
    data: Option<Vec<Audience>>,
    page_info: Option<ListAudiencesResponsePageInfo>,
}

impl ListAudiencesResponseBuilder {
    pub fn data(mut self, value: Vec<Audience>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAudiencesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAudiencesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAudiencesResponseBuilder::data)
    /// - [`page_info`](ListAudiencesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAudiencesResponse, BuildError> {
        Ok(ListAudiencesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
