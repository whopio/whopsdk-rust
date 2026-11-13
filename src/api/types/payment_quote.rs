pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentQuote {
    /// The account the purchase is priced for, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// The billing address the purchase was priced with, or null. Where tax was calculated when no shipping address was given, and the address the registration belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PaymentAddress>,
    /// Three-letter ISO 4217 code of the variants' own currency, lowercase. Equal to `currency` when nothing was converted.
    #[serde(default)]
    pub base_currency: String,
    /// `total` in `base_currency` at `exchange_rate`: the variants' own prices after the promo code, plus `tax_amount` converted at `exchange_rate` when tax is added on top. Equal to `total` when nothing was converted. The payment charges `total`, in `currency`.
    #[serde(default)]
    pub base_total: Money,
    /// When the quote was priced, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Three-letter ISO 4217 currency code the purchase is priced and charged in, lowercase: the variants' own currency, or the `presentment_currency` it was converted into.
    #[serde(default)]
    pub currency: String,
    /// What the promo code takes off. Zero without a code.
    #[serde(default)]
    pub discount: Money,
    /// How many units of `currency` one unit of `base_currency` buys, as a decimal string such as `"5.4321"`: the rate the variants' prices were converted at, fixed until `expires_at`, and the rate a payment consuming the quote is charged at. A string, like money amounts, so no float rounds it in transit. Null when nothing was converted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<String>,
    /// When the quote stops being chargeable, as an ISO 8601 timestamp. Quote again after it.
    #[serde(default)]
    pub expires_at: String,
    /// Payment quote ID, prefixed `pq_`.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub line_items: Vec<PaymentQuoteLineItem>,
    /// Which location tax was calculated for: `shipping_address` when it carries a country, else the billing `address` when it does, else the `presentment_country` sent, else the buyer's `ip_address`. A quote located by `presentment_country` or `ip_address` is a preview that estimates tax: a payment cannot use it, so quote again with the buyer's address to pay. Null when nothing in the request located the buyer, which only a seller that collects no tax on this purchase is quoted without; `tax_status` is then `not_applicable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub located_by: Option<PaymentQuoteLocatedBy>,
    /// The payment holding this quote, prefixed `pay_`, or null while it is unspent. A declined payment keeps its quote and can be retried; check that payment's `status`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// The promo code the quote applied, prefixed `promo_`, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code_id: Option<String>,
    #[serde(default)]
    pub recommended_currencies: Vec<String>,
    /// The shipping address the purchase was priced with, or null. When present it is where tax was calculated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<PaymentAddress>,
    /// The price of every line before the promo code, tax and fees.
    #[serde(default)]
    pub subtotal: Money,
    /// The tax a payment consuming this quote charges. Zero unless `tax_status` is `calculated`.
    #[serde(default)]
    pub tax_amount: Money,
    /// Whether tax is added on top of the price (`exclusive`) or already inside it (`inclusive`). Null when no tax was calculated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PaymentQuoteTaxBehavior>,
    #[serde(default)]
    pub tax_ids: Vec<TaxId>,
    /// `calculated`: every line was priced and a payment may consume the quote. `not_applicable`: this seller collects no tax on this purchase, so the quote owes none and may still be consumed. `unavailable`: tax could not be priced — the provider did not answer, or this seller's tax setup cannot price a purchase here — so a payment refuses the quote; quote again, or charge without `quote_id` to have tax calculated at charge time.
    pub tax_status: PaymentQuoteTaxStatus,
    /// What the buyer pays: the subtotal less the discount, plus `tax_amount` when tax is added on top.
    #[serde(default)]
    pub total: Money,
}

impl PaymentQuote {
    pub fn builder() -> PaymentQuoteBuilder {
        <PaymentQuoteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentQuoteBuilder {
    account_id: Option<String>,
    address: Option<PaymentAddress>,
    base_currency: Option<String>,
    base_total: Option<Money>,
    created_at: Option<String>,
    currency: Option<String>,
    discount: Option<Money>,
    exchange_rate: Option<String>,
    expires_at: Option<String>,
    id: Option<String>,
    line_items: Option<Vec<PaymentQuoteLineItem>>,
    located_by: Option<PaymentQuoteLocatedBy>,
    payment_id: Option<String>,
    promo_code_id: Option<String>,
    recommended_currencies: Option<Vec<String>>,
    shipping_address: Option<PaymentAddress>,
    subtotal: Option<Money>,
    tax_amount: Option<Money>,
    tax_behavior: Option<PaymentQuoteTaxBehavior>,
    tax_ids: Option<Vec<TaxId>>,
    tax_status: Option<PaymentQuoteTaxStatus>,
    total: Option<Money>,
}

impl PaymentQuoteBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn address(mut self, value: PaymentAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn base_currency(mut self, value: impl Into<String>) -> Self {
        self.base_currency = Some(value.into());
        self
    }

    pub fn base_total(mut self, value: Money) -> Self {
        self.base_total = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn discount(mut self, value: Money) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn exchange_rate(mut self, value: impl Into<String>) -> Self {
        self.exchange_rate = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<PaymentQuoteLineItem>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn located_by(mut self, value: PaymentQuoteLocatedBy) -> Self {
        self.located_by = Some(value);
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn promo_code_id(mut self, value: impl Into<String>) -> Self {
        self.promo_code_id = Some(value.into());
        self
    }

    pub fn recommended_currencies(mut self, value: Vec<String>) -> Self {
        self.recommended_currencies = Some(value);
        self
    }

    pub fn shipping_address(mut self, value: PaymentAddress) -> Self {
        self.shipping_address = Some(value);
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

    pub fn tax_behavior(mut self, value: PaymentQuoteTaxBehavior) -> Self {
        self.tax_behavior = Some(value);
        self
    }

    pub fn tax_ids(mut self, value: Vec<TaxId>) -> Self {
        self.tax_ids = Some(value);
        self
    }

    pub fn tax_status(mut self, value: PaymentQuoteTaxStatus) -> Self {
        self.tax_status = Some(value);
        self
    }

    pub fn total(mut self, value: Money) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentQuote`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](PaymentQuoteBuilder::account_id)
    /// - [`base_currency`](PaymentQuoteBuilder::base_currency)
    /// - [`base_total`](PaymentQuoteBuilder::base_total)
    /// - [`created_at`](PaymentQuoteBuilder::created_at)
    /// - [`currency`](PaymentQuoteBuilder::currency)
    /// - [`discount`](PaymentQuoteBuilder::discount)
    /// - [`expires_at`](PaymentQuoteBuilder::expires_at)
    /// - [`id`](PaymentQuoteBuilder::id)
    /// - [`line_items`](PaymentQuoteBuilder::line_items)
    /// - [`recommended_currencies`](PaymentQuoteBuilder::recommended_currencies)
    /// - [`subtotal`](PaymentQuoteBuilder::subtotal)
    /// - [`tax_amount`](PaymentQuoteBuilder::tax_amount)
    /// - [`tax_ids`](PaymentQuoteBuilder::tax_ids)
    /// - [`tax_status`](PaymentQuoteBuilder::tax_status)
    /// - [`total`](PaymentQuoteBuilder::total)
    pub fn build(self) -> Result<PaymentQuote, BuildError> {
        Ok(PaymentQuote {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            address: self.address,
            base_currency: self
                .base_currency
                .ok_or_else(|| BuildError::missing_field("base_currency"))?,
            base_total: self
                .base_total
                .ok_or_else(|| BuildError::missing_field("base_total"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            discount: self
                .discount
                .ok_or_else(|| BuildError::missing_field("discount"))?,
            exchange_rate: self.exchange_rate,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            located_by: self.located_by,
            payment_id: self.payment_id,
            promo_code_id: self.promo_code_id,
            recommended_currencies: self
                .recommended_currencies
                .ok_or_else(|| BuildError::missing_field("recommended_currencies"))?,
            shipping_address: self.shipping_address,
            subtotal: self
                .subtotal
                .ok_or_else(|| BuildError::missing_field("subtotal"))?,
            tax_amount: self
                .tax_amount
                .ok_or_else(|| BuildError::missing_field("tax_amount"))?,
            tax_behavior: self.tax_behavior,
            tax_ids: self
                .tax_ids
                .ok_or_else(|| BuildError::missing_field("tax_ids"))?,
            tax_status: self
                .tax_status
                .ok_or_else(|| BuildError::missing_field("tax_status"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
