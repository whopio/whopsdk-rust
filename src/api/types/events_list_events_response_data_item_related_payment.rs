pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEventsResponseDataItemRelatedPayment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Everything this payment charged for, in purchase order, including quantities. Older payments fall back to their original plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<ReceiptLineItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

impl ListEventsResponseDataItemRelatedPayment {
    pub fn builder() -> ListEventsResponseDataItemRelatedPaymentBuilder {
        <ListEventsResponseDataItemRelatedPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemRelatedPaymentBuilder {
    card_brand: Option<String>,
    card_last4: Option<String>,
    id: Option<String>,
    line_items: Option<Vec<ReceiptLineItem>>,
    provider: Option<String>,
}

impl ListEventsResponseDataItemRelatedPaymentBuilder {
    pub fn card_brand(mut self, value: impl Into<String>) -> Self {
        self.card_brand = Some(value.into());
        self
    }

    pub fn card_last4(mut self, value: impl Into<String>) -> Self {
        self.card_last4 = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<ReceiptLineItem>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEventsResponseDataItemRelatedPayment`].
    pub fn build(self) -> Result<ListEventsResponseDataItemRelatedPayment, BuildError> {
        Ok(ListEventsResponseDataItemRelatedPayment {
            card_brand: self.card_brand,
            card_last4: self.card_last4,
            id: self.id,
            line_items: self.line_items,
            provider: self.provider,
        })
    }
}
