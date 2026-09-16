pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPaymentRulesResponse {
    #[serde(default)]
    pub data: Vec<PaymentRule>,
    #[serde(default)]
    pub page_info: ListPaymentRulesResponsePageInfo,
}

impl ListPaymentRulesResponse {
    pub fn builder() -> ListPaymentRulesResponseBuilder {
        <ListPaymentRulesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPaymentRulesResponseBuilder {
    data: Option<Vec<PaymentRule>>,
    page_info: Option<ListPaymentRulesResponsePageInfo>,
}

impl ListPaymentRulesResponseBuilder {
    pub fn data(mut self, value: Vec<PaymentRule>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListPaymentRulesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPaymentRulesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPaymentRulesResponseBuilder::data)
    /// - [`page_info`](ListPaymentRulesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListPaymentRulesResponse, BuildError> {
        Ok(ListPaymentRulesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
