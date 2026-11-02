pub use crate::prelude::*;

/// Represents a fee related to a payment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentLegacyFeesItem {
    /// The value or amount to display for the fee.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The currency of the fee.
    pub currency: Currencies,
    /// The label to display for the fee.
    #[serde(default)]
    pub name: String,
    /// The specific origin of the fee, if applicable.
    pub r#type: SpecificFeeOrigins,
}

impl PaymentLegacyFeesItem {
    pub fn builder() -> PaymentLegacyFeesItemBuilder {
        <PaymentLegacyFeesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentLegacyFeesItemBuilder {
    amount: Option<f64>,
    currency: Option<Currencies>,
    name: Option<String>,
    r#type: Option<SpecificFeeOrigins>,
}

impl PaymentLegacyFeesItemBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: SpecificFeeOrigins) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentLegacyFeesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PaymentLegacyFeesItemBuilder::amount)
    /// - [`currency`](PaymentLegacyFeesItemBuilder::currency)
    /// - [`name`](PaymentLegacyFeesItemBuilder::name)
    /// - [`r#type`](PaymentLegacyFeesItemBuilder::r#type)
    pub fn build(self) -> Result<PaymentLegacyFeesItem, BuildError> {
        Ok(PaymentLegacyFeesItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
