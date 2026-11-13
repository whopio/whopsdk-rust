pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAutomationsResponse {
    #[serde(default)]
    pub data: Vec<Automation>,
    #[serde(default)]
    pub page_info: ListAutomationsResponsePageInfo,
}

impl ListAutomationsResponse {
    pub fn builder() -> ListAutomationsResponseBuilder {
        <ListAutomationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAutomationsResponseBuilder {
    data: Option<Vec<Automation>>,
    page_info: Option<ListAutomationsResponsePageInfo>,
}

impl ListAutomationsResponseBuilder {
    pub fn data(mut self, value: Vec<Automation>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAutomationsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAutomationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAutomationsResponseBuilder::data)
    /// - [`page_info`](ListAutomationsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAutomationsResponse, BuildError> {
        Ok(ListAutomationsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
