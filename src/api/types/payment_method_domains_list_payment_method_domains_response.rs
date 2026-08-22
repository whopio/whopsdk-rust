pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPaymentMethodDomainsResponse {
    #[serde(default)]
    pub data: Vec<PaymentMethodDomain>,
    #[serde(default)]
    pub page_info: ListPaymentMethodDomainsResponsePageInfo,
}

impl ListPaymentMethodDomainsResponse {
    pub fn builder() -> ListPaymentMethodDomainsResponseBuilder {
        <ListPaymentMethodDomainsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPaymentMethodDomainsResponseBuilder {
    data: Option<Vec<PaymentMethodDomain>>,
    page_info: Option<ListPaymentMethodDomainsResponsePageInfo>,
}

impl ListPaymentMethodDomainsResponseBuilder {
    pub fn data(mut self, value: Vec<PaymentMethodDomain>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListPaymentMethodDomainsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPaymentMethodDomainsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPaymentMethodDomainsResponseBuilder::data)
    /// - [`page_info`](ListPaymentMethodDomainsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListPaymentMethodDomainsResponse, BuildError> {
        Ok(ListPaymentMethodDomainsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
