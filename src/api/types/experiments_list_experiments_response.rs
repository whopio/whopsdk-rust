pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListExperimentsResponse {
    #[serde(default)]
    pub data: Vec<Experiment>,
    #[serde(default)]
    pub page_info: ListExperimentsResponsePageInfo,
}

impl ListExperimentsResponse {
    pub fn builder() -> ListExperimentsResponseBuilder {
        <ListExperimentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListExperimentsResponseBuilder {
    data: Option<Vec<Experiment>>,
    page_info: Option<ListExperimentsResponsePageInfo>,
}

impl ListExperimentsResponseBuilder {
    pub fn data(mut self, value: Vec<Experiment>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListExperimentsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListExperimentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListExperimentsResponseBuilder::data)
    /// - [`page_info`](ListExperimentsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListExperimentsResponse, BuildError> {
        Ok(ListExperimentsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
