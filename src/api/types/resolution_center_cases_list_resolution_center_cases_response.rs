pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListResolutionCenterCasesResponse {
    #[serde(default)]
    pub data: Vec<ResolutionCenterCase>,
    #[serde(default)]
    pub page_info: ListResolutionCenterCasesResponsePageInfo,
}

impl ListResolutionCenterCasesResponse {
    pub fn builder() -> ListResolutionCenterCasesResponseBuilder {
        <ListResolutionCenterCasesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResolutionCenterCasesResponseBuilder {
    data: Option<Vec<ResolutionCenterCase>>,
    page_info: Option<ListResolutionCenterCasesResponsePageInfo>,
}

impl ListResolutionCenterCasesResponseBuilder {
    pub fn data(mut self, value: Vec<ResolutionCenterCase>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListResolutionCenterCasesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListResolutionCenterCasesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListResolutionCenterCasesResponseBuilder::data)
    /// - [`page_info`](ListResolutionCenterCasesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListResolutionCenterCasesResponse, BuildError> {
        Ok(ListResolutionCenterCasesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
