pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckoutSessionQuote {
    /// The total in `base_currency`, before conversion — what the seller prices in. The figure that is actually charged is `breakdown.total`, in `currency`.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub base_amount: f64,
    /// The seller's own currency for these items, lowercase. Equal to `currency` when no conversion applied.
    #[serde(default)]
    pub base_currency: String,
    /// The same total, itemized — what is being bought, what changes the price, and what is owed later. The tax adjustment arrives `pending` here and resolves through `calculate_tax`, which answers with this same shape: render whichever you hold, preferring the calculated one.
    pub breakdown: CheckoutSessionBreakdown,
    /// ISO currency the confirm will charge in, lowercase. This is the buyer's own currency whenever adaptive pricing applies — the card is charged in it, not shown a conversion of it.
    #[serde(default)]
    pub currency: String,
    /// The locked rate `base_currency` was converted at, or `null` when the charge is in the seller's own currency. Fixed for the life of the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub exchange_rate: Option<f64>,
    /// When this quote stops being chargeable, as an ISO 8601 timestamp. A converted quote expires with its exchange rate, which is sooner than an unconverted one.
    #[serde(default)]
    pub expires_at: String,
    /// When this quote was computed, as an ISO 8601 timestamp. A confirm against a stale quote is refused with `quote_expired` and the quote refreshes for the retry.
    #[serde(default)]
    pub quoted_at: String,
    /// Whether this quote priced the plan's free trial as SKIPPED: the resolved buyer has held this plan before, so no trial applies and the full first price is due today. Tell the buyer so before they pay — the trial they can see on the plan is not theirs to start. `false` for a plan with no trial, and for a session whose buyer is not resolved yet: a returning buyer resolving at confirm is refused with `quote_changed` and this re-prices to `true` for the retry.
    #[serde(default)]
    pub trial_skipped: bool,
}

impl CheckoutSessionQuote {
    pub fn builder() -> CheckoutSessionQuoteBuilder {
        <CheckoutSessionQuoteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionQuoteBuilder {
    base_amount: Option<f64>,
    base_currency: Option<String>,
    breakdown: Option<CheckoutSessionBreakdown>,
    currency: Option<String>,
    exchange_rate: Option<f64>,
    expires_at: Option<String>,
    quoted_at: Option<String>,
    trial_skipped: Option<bool>,
}

impl CheckoutSessionQuoteBuilder {
    pub fn base_amount(mut self, value: f64) -> Self {
        self.base_amount = Some(value);
        self
    }

    pub fn base_currency(mut self, value: impl Into<String>) -> Self {
        self.base_currency = Some(value.into());
        self
    }

    pub fn breakdown(mut self, value: CheckoutSessionBreakdown) -> Self {
        self.breakdown = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn exchange_rate(mut self, value: f64) -> Self {
        self.exchange_rate = Some(value);
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn quoted_at(mut self, value: impl Into<String>) -> Self {
        self.quoted_at = Some(value.into());
        self
    }

    pub fn trial_skipped(mut self, value: bool) -> Self {
        self.trial_skipped = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionQuote`].
    /// This method will fail if any of the following fields are not set:
    /// - [`base_amount`](CheckoutSessionQuoteBuilder::base_amount)
    /// - [`base_currency`](CheckoutSessionQuoteBuilder::base_currency)
    /// - [`breakdown`](CheckoutSessionQuoteBuilder::breakdown)
    /// - [`currency`](CheckoutSessionQuoteBuilder::currency)
    /// - [`expires_at`](CheckoutSessionQuoteBuilder::expires_at)
    /// - [`quoted_at`](CheckoutSessionQuoteBuilder::quoted_at)
    /// - [`trial_skipped`](CheckoutSessionQuoteBuilder::trial_skipped)
    pub fn build(self) -> Result<CheckoutSessionQuote, BuildError> {
        Ok(CheckoutSessionQuote {
            base_amount: self
                .base_amount
                .ok_or_else(|| BuildError::missing_field("base_amount"))?,
            base_currency: self
                .base_currency
                .ok_or_else(|| BuildError::missing_field("base_currency"))?,
            breakdown: self
                .breakdown
                .ok_or_else(|| BuildError::missing_field("breakdown"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            exchange_rate: self.exchange_rate,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            quoted_at: self
                .quoted_at
                .ok_or_else(|| BuildError::missing_field("quoted_at"))?,
            trial_skipped: self
                .trial_skipped
                .ok_or_else(|| BuildError::missing_field("trial_skipped"))?,
        })
    }
}
