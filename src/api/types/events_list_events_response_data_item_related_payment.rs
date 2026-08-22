pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEventsResponseDataItemRelatedPayment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
            provider: self.provider,
        })
    }
}
