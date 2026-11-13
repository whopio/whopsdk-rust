pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAutomationStepsResponse {
    #[serde(default)]
    pub data: Vec<AutomationStep>,
    #[serde(default)]
    pub page_info: ListAutomationStepsResponsePageInfo,
}

impl ListAutomationStepsResponse {
    pub fn builder() -> ListAutomationStepsResponseBuilder {
        <ListAutomationStepsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAutomationStepsResponseBuilder {
    data: Option<Vec<AutomationStep>>,
    page_info: Option<ListAutomationStepsResponsePageInfo>,
}

impl ListAutomationStepsResponseBuilder {
    pub fn data(mut self, value: Vec<AutomationStep>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAutomationStepsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAutomationStepsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAutomationStepsResponseBuilder::data)
    /// - [`page_info`](ListAutomationStepsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAutomationStepsResponse, BuildError> {
        Ok(ListAutomationStepsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
