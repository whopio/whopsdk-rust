pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionBreakdownLine {
    /// The line's own total — `unit_amount` × `quantity`, before any adjustment.
    #[serde(default)]
    pub amount: Money,
    /// What to show the buyer for this line.
    #[serde(default)]
    pub description: String,
    /// What this line is — `plan` today. New kinds arrive as checkout learns to sell more than one thing at once.
    pub kind: CheckoutSessionBreakdownLineKind,
    /// How many.
    #[serde(default)]
    pub quantity: i64,
    /// Price of one, before any adjustment.
    #[serde(default)]
    pub unit_amount: Money,
}

impl CheckoutSessionBreakdownLine {
    pub fn builder() -> CheckoutSessionBreakdownLineBuilder {
        <CheckoutSessionBreakdownLineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionBreakdownLineBuilder {
    amount: Option<Money>,
    description: Option<String>,
    kind: Option<CheckoutSessionBreakdownLineKind>,
    quantity: Option<i64>,
    unit_amount: Option<Money>,
}

impl CheckoutSessionBreakdownLineBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn kind(mut self, value: CheckoutSessionBreakdownLineKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn unit_amount(mut self, value: Money) -> Self {
        self.unit_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionBreakdownLine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CheckoutSessionBreakdownLineBuilder::amount)
    /// - [`description`](CheckoutSessionBreakdownLineBuilder::description)
    /// - [`kind`](CheckoutSessionBreakdownLineBuilder::kind)
    /// - [`quantity`](CheckoutSessionBreakdownLineBuilder::quantity)
    /// - [`unit_amount`](CheckoutSessionBreakdownLineBuilder::unit_amount)
    pub fn build(self) -> Result<CheckoutSessionBreakdownLine, BuildError> {
        Ok(CheckoutSessionBreakdownLine {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_amount: self
                .unit_amount
                .ok_or_else(|| BuildError::missing_field("unit_amount"))?,
        })
    }
}
