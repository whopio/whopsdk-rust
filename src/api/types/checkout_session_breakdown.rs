pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckoutSessionBreakdown {
    #[serde(default)]
    pub adjustments: Vec<CheckoutSessionBreakdownAdjustment>,
    /// ISO currency every figure below is in, lowercase. Every amount here is a Money object, never a bare number — see `subtotal`.
    pub currency: CheckoutSessionBreakdownCurrency,
    /// WHAT TO RENDER — the summary's headline and itemized sections, decided server-side from the same computation as the figures above so the two cannot disagree. Render this rather than re-deriving rows from the figures; the figures stay authoritative for arithmetic.
    pub display: CheckoutSessionBreakdownDisplay,
    #[serde(default)]
    pub lines: Vec<CheckoutSessionBreakdownLine>,
    /// The lines summed, before any adjustment.
    #[serde(default)]
    pub subtotal: Money,
    /// What confirm charges: the subtotal plus every RESOLVED `added` adjustment. A `pending` adjustment is deliberately NOT in it, so the number is never inflated by a figure the buyer cannot yet see itemized — with exclusive tax it therefore grows once tax resolves, and with inclusive tax it does not move at all. Once the session is `completed` it is what WAS charged: the fee and tax on the order's own receipt, and nothing is `pending`.
    #[serde(default)]
    pub total: Money,
    /// WHEN the rest is owed — empty for a purchase that is settled today. `type` picks the shape and each variant carries only its own fields. An order can carry several at once (a split-pay plan alongside a subscription upsell), which is why it is a list.
    #[serde(default)]
    pub upcoming: Vec<CheckoutSessionBreakdownUpcoming>,
}

impl CheckoutSessionBreakdown {
    pub fn builder() -> CheckoutSessionBreakdownBuilder {
        <CheckoutSessionBreakdownBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionBreakdownBuilder {
    adjustments: Option<Vec<CheckoutSessionBreakdownAdjustment>>,
    currency: Option<CheckoutSessionBreakdownCurrency>,
    display: Option<CheckoutSessionBreakdownDisplay>,
    lines: Option<Vec<CheckoutSessionBreakdownLine>>,
    subtotal: Option<Money>,
    total: Option<Money>,
    upcoming: Option<Vec<CheckoutSessionBreakdownUpcoming>>,
}

impl CheckoutSessionBreakdownBuilder {
    pub fn adjustments(mut self, value: Vec<CheckoutSessionBreakdownAdjustment>) -> Self {
        self.adjustments = Some(value);
        self
    }

    pub fn currency(mut self, value: CheckoutSessionBreakdownCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn display(mut self, value: CheckoutSessionBreakdownDisplay) -> Self {
        self.display = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<CheckoutSessionBreakdownLine>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn subtotal(mut self, value: Money) -> Self {
        self.subtotal = Some(value);
        self
    }

    pub fn total(mut self, value: Money) -> Self {
        self.total = Some(value);
        self
    }

    pub fn upcoming(mut self, value: Vec<CheckoutSessionBreakdownUpcoming>) -> Self {
        self.upcoming = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionBreakdown`].
    /// This method will fail if any of the following fields are not set:
    /// - [`adjustments`](CheckoutSessionBreakdownBuilder::adjustments)
    /// - [`currency`](CheckoutSessionBreakdownBuilder::currency)
    /// - [`display`](CheckoutSessionBreakdownBuilder::display)
    /// - [`lines`](CheckoutSessionBreakdownBuilder::lines)
    /// - [`subtotal`](CheckoutSessionBreakdownBuilder::subtotal)
    /// - [`total`](CheckoutSessionBreakdownBuilder::total)
    /// - [`upcoming`](CheckoutSessionBreakdownBuilder::upcoming)
    pub fn build(self) -> Result<CheckoutSessionBreakdown, BuildError> {
        Ok(CheckoutSessionBreakdown {
            adjustments: self
                .adjustments
                .ok_or_else(|| BuildError::missing_field("adjustments"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            display: self
                .display
                .ok_or_else(|| BuildError::missing_field("display"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
            subtotal: self
                .subtotal
                .ok_or_else(|| BuildError::missing_field("subtotal"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            upcoming: self
                .upcoming
                .ok_or_else(|| BuildError::missing_field("upcoming"))?,
        })
    }
}
