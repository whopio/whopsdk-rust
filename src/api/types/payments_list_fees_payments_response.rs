pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFeesPaymentsResponse {
    #[serde(default)]
    pub data: Vec<PaymentFee>,
    #[serde(default)]
    pub page_info: ListFeesPaymentsResponsePageInfo,
}

impl ListFeesPaymentsResponse {
    pub fn builder() -> ListFeesPaymentsResponseBuilder {
        <ListFeesPaymentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFeesPaymentsResponseBuilder {
    data: Option<Vec<PaymentFee>>,
    page_info: Option<ListFeesPaymentsResponsePageInfo>,
}

impl ListFeesPaymentsResponseBuilder {
    pub fn data(mut self, value: Vec<PaymentFee>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListFeesPaymentsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFeesPaymentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListFeesPaymentsResponseBuilder::data)
    /// - [`page_info`](ListFeesPaymentsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListFeesPaymentsResponse, BuildError> {
        Ok(ListFeesPaymentsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
