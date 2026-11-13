pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAutomationEnrollmentsResponse {
    #[serde(default)]
    pub data: Vec<AutomationEnrollment>,
    #[serde(default)]
    pub page_info: ListAutomationEnrollmentsResponsePageInfo,
}

impl ListAutomationEnrollmentsResponse {
    pub fn builder() -> ListAutomationEnrollmentsResponseBuilder {
        <ListAutomationEnrollmentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAutomationEnrollmentsResponseBuilder {
    data: Option<Vec<AutomationEnrollment>>,
    page_info: Option<ListAutomationEnrollmentsResponsePageInfo>,
}

impl ListAutomationEnrollmentsResponseBuilder {
    pub fn data(mut self, value: Vec<AutomationEnrollment>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAutomationEnrollmentsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAutomationEnrollmentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAutomationEnrollmentsResponseBuilder::data)
    /// - [`page_info`](ListAutomationEnrollmentsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAutomationEnrollmentsResponse, BuildError> {
        Ok(ListAutomationEnrollmentsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
