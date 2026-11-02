pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFieldsPaymentRulesResponse {
    #[serde(default)]
    pub data: Vec<PaymentRuleField>,
    #[serde(default)]
    pub page_info: ListFieldsPaymentRulesResponsePageInfo,
}

impl ListFieldsPaymentRulesResponse {
    pub fn builder() -> ListFieldsPaymentRulesResponseBuilder {
        <ListFieldsPaymentRulesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFieldsPaymentRulesResponseBuilder {
    data: Option<Vec<PaymentRuleField>>,
    page_info: Option<ListFieldsPaymentRulesResponsePageInfo>,
}

impl ListFieldsPaymentRulesResponseBuilder {
    pub fn data(mut self, value: Vec<PaymentRuleField>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListFieldsPaymentRulesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFieldsPaymentRulesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListFieldsPaymentRulesResponseBuilder::data)
    /// - [`page_info`](ListFieldsPaymentRulesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListFieldsPaymentRulesResponse, BuildError> {
        Ok(ListFieldsPaymentRulesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
