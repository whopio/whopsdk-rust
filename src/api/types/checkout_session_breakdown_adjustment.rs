pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionBreakdownAdjustment {
    /// What this adjustment does to the total — negative for a discount. `null` only while `status` is `pending`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// `added` counts toward the total; `included` is already inside the lines and is shown for disclosure only (inclusive-tax markets).
    pub applied: CheckoutSessionBreakdownAdjustmentApplied,
    /// What changes the price: the promo's `discount`, the `buyer_fee` the charge adds, or `tax`.
    pub kind: CheckoutSessionBreakdownAdjustmentKind,
    /// What to show the buyer for this row.
    #[serde(default)]
    pub label: String,
    /// The `tax` row's effective rate as a decimal fraction (`"0.0725"` is 7.25%) — the tax as a share of the base it was priced on, stated so a surface can label the row with the percentage. Present only on a resolved tax adjustment from `calculate_breakdown` or `calculate_tax`; absent on other kinds, on a `pending` row, and on a completed session's settled row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    /// Present only on an adjustment with no figure yet — render its row in a loading state and expect `calculate_breakdown` to resolve it. An adjustment that does not apply is absent from the list entirely, never a zero row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckoutSessionBreakdownAdjustmentStatus>,
}

impl CheckoutSessionBreakdownAdjustment {
    pub fn builder() -> CheckoutSessionBreakdownAdjustmentBuilder {
        <CheckoutSessionBreakdownAdjustmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionBreakdownAdjustmentBuilder {
    amount: Option<Money>,
    applied: Option<CheckoutSessionBreakdownAdjustmentApplied>,
    kind: Option<CheckoutSessionBreakdownAdjustmentKind>,
    label: Option<String>,
    rate: Option<String>,
    status: Option<CheckoutSessionBreakdownAdjustmentStatus>,
}

impl CheckoutSessionBreakdownAdjustmentBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn applied(mut self, value: CheckoutSessionBreakdownAdjustmentApplied) -> Self {
        self.applied = Some(value);
        self
    }

    pub fn kind(mut self, value: CheckoutSessionBreakdownAdjustmentKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn rate(mut self, value: impl Into<String>) -> Self {
        self.rate = Some(value.into());
        self
    }

    pub fn status(mut self, value: CheckoutSessionBreakdownAdjustmentStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionBreakdownAdjustment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`applied`](CheckoutSessionBreakdownAdjustmentBuilder::applied)
    /// - [`kind`](CheckoutSessionBreakdownAdjustmentBuilder::kind)
    /// - [`label`](CheckoutSessionBreakdownAdjustmentBuilder::label)
    pub fn build(self) -> Result<CheckoutSessionBreakdownAdjustment, BuildError> {
        Ok(CheckoutSessionBreakdownAdjustment {
            amount: self.amount,
            applied: self
                .applied
                .ok_or_else(|| BuildError::missing_field("applied"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            rate: self.rate,
            status: self.status,
        })
    }
}
