pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListEarningsResponseDataItemResourceAlternativePaymentMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_payment_method: Option<
        ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethod,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    pub object: ListEarningsResponseDataItemResourceAlternativePaymentMethodObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<String>,
}

impl ListEarningsResponseDataItemResourceAlternativePaymentMethod {
    pub fn builder() -> ListEarningsResponseDataItemResourceAlternativePaymentMethodBuilder {
        <ListEarningsResponseDataItemResourceAlternativePaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseDataItemResourceAlternativePaymentMethodBuilder {
    alternative_payment_method: Option<
        ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethod,
    >,
    brand: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    id: Option<String>,
    last4: Option<String>,
    object: Option<ListEarningsResponseDataItemResourceAlternativePaymentMethodObject>,
    payment_method_type: Option<String>,
    processor: Option<String>,
}

impl ListEarningsResponseDataItemResourceAlternativePaymentMethodBuilder {
    pub fn alternative_payment_method(
        mut self,
        value: ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethod,
    ) -> Self {
        self.alternative_payment_method = Some(value);
        self
    }

    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    pub fn object(
        mut self,
        value: ListEarningsResponseDataItemResourceAlternativePaymentMethodObject,
    ) -> Self {
        self.object = Some(value);
        self
    }

    pub fn payment_method_type(mut self, value: impl Into<String>) -> Self {
        self.payment_method_type = Some(value.into());
        self
    }

    pub fn processor(mut self, value: impl Into<String>) -> Self {
        self.processor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItemResourceAlternativePaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ListEarningsResponseDataItemResourceAlternativePaymentMethodBuilder::created_at)
    /// - [`currency`](ListEarningsResponseDataItemResourceAlternativePaymentMethodBuilder::currency)
    /// - [`id`](ListEarningsResponseDataItemResourceAlternativePaymentMethodBuilder::id)
    /// - [`object`](ListEarningsResponseDataItemResourceAlternativePaymentMethodBuilder::object)
    pub fn build(
        self,
    ) -> Result<ListEarningsResponseDataItemResourceAlternativePaymentMethod, BuildError> {
        Ok(
            ListEarningsResponseDataItemResourceAlternativePaymentMethod {
                alternative_payment_method: self.alternative_payment_method,
                brand: self.brand,
                created_at: self
                    .created_at
                    .ok_or_else(|| BuildError::missing_field("created_at"))?,
                currency: self
                    .currency
                    .ok_or_else(|| BuildError::missing_field("currency"))?,
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                last4: self.last4,
                object: self
                    .object
                    .ok_or_else(|| BuildError::missing_field("object"))?,
                payment_method_type: self.payment_method_type,
                processor: self.processor,
            },
        )
    }
}
