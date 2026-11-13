pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentQuoteLineItem {
    /// The promo code's share on this line. Zero without a code.
    #[serde(default)]
    pub discount: Money,
    /// How many units of the variant the line covers.
    #[serde(default)]
    pub quantity: i64,
    /// The line's price before the promo code's share and tax; `subtotal` minus `discount` is what the line comes to after the discount. The lines' subtotals sum to the quote's `subtotal`.
    #[serde(default)]
    pub subtotal: Money,
    /// The tax on this line. Zero when `tax_status` is not `calculated`.
    #[serde(default)]
    pub tax_amount: Money,
    /// `subtotal` minus `discount`, plus `tax_amount` when tax is added on top, or without it when the price already includes it.
    #[serde(default)]
    pub total: Money,
    /// The variant this line prices, prefixed `plan_`. Null when the purchase describes the variant by `variant` and no variant with those attributes exists yet: the payment that consumes the quote creates it, and the line names it from then on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
}

impl PaymentQuoteLineItem {
    pub fn builder() -> PaymentQuoteLineItemBuilder {
        <PaymentQuoteLineItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentQuoteLineItemBuilder {
    discount: Option<Money>,
    quantity: Option<i64>,
    subtotal: Option<Money>,
    tax_amount: Option<Money>,
    total: Option<Money>,
    variant_id: Option<String>,
}

impl PaymentQuoteLineItemBuilder {
    pub fn discount(mut self, value: Money) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn subtotal(mut self, value: Money) -> Self {
        self.subtotal = Some(value);
        self
    }

    pub fn tax_amount(mut self, value: Money) -> Self {
        self.tax_amount = Some(value);
        self
    }

    pub fn total(mut self, value: Money) -> Self {
        self.total = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentQuoteLineItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`discount`](PaymentQuoteLineItemBuilder::discount)
    /// - [`quantity`](PaymentQuoteLineItemBuilder::quantity)
    /// - [`subtotal`](PaymentQuoteLineItemBuilder::subtotal)
    /// - [`tax_amount`](PaymentQuoteLineItemBuilder::tax_amount)
    /// - [`total`](PaymentQuoteLineItemBuilder::total)
    pub fn build(self) -> Result<PaymentQuoteLineItem, BuildError> {
        Ok(PaymentQuoteLineItem {
            discount: self
                .discount
                .ok_or_else(|| BuildError::missing_field("discount"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            subtotal: self
                .subtotal
                .ok_or_else(|| BuildError::missing_field("subtotal"))?,
            tax_amount: self
                .tax_amount
                .ok_or_else(|| BuildError::missing_field("tax_amount"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            variant_id: self.variant_id,
        })
    }
}
