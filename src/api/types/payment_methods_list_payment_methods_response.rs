pub use crate::prelude::*;

/// The connection type for PaymentMethodInterface.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPaymentMethodsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<PaymentMethodListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListPaymentMethodsResponse {
    pub fn builder() -> ListPaymentMethodsResponseBuilder {
        <ListPaymentMethodsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPaymentMethodsResponseBuilder {
    data: Option<Vec<PaymentMethodListItem>>,
    page_info: Option<PageInfo>,
}

impl ListPaymentMethodsResponseBuilder {
    pub fn data(mut self, value: Vec<PaymentMethodListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPaymentMethodsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPaymentMethodsResponseBuilder::data)
    /// - [`page_info`](ListPaymentMethodsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListPaymentMethodsResponse, BuildError> {
        Ok(ListPaymentMethodsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
