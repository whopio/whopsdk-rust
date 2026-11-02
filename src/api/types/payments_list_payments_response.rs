pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPaymentsResponse {
    #[serde(default)]
    pub data: Vec<Payment>,
    #[serde(default)]
    pub page_info: ListPaymentsResponsePageInfo,
}

impl ListPaymentsResponse {
    pub fn builder() -> ListPaymentsResponseBuilder {
        <ListPaymentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPaymentsResponseBuilder {
    data: Option<Vec<Payment>>,
    page_info: Option<ListPaymentsResponsePageInfo>,
}

impl ListPaymentsResponseBuilder {
    pub fn data(mut self, value: Vec<Payment>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListPaymentsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPaymentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPaymentsResponseBuilder::data)
    /// - [`page_info`](ListPaymentsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListPaymentsResponse, BuildError> {
        Ok(ListPaymentsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
