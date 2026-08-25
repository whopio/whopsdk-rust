pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionBreakdownDisplayRow {
    /// The row's figure as a Money object, when it has one — what a surface formats in the buyer's locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// The typed parameters this row's wording is built from — the localization rail. Absent when the kind needs none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<CheckoutSessionBreakdownDisplayRowDetail>,
    /// Which row this is. Render a kind you know from `detail` in your own wording; render one you do not from `label` and `text` verbatim — never drop it.
    pub kind: CheckoutSessionBreakdownDisplayRowKind,
    /// The row's left side in English — the fallback for a kind the surface cannot name. Absent on a row that is a single line rather than a label/value pair (the headline's context).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Present only on a row whose figure is still being calculated — render it in a loading state and expect `calculate_breakdown` to resolve it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckoutSessionBreakdownDisplayRowStatus>,
    /// The row's value (or its whole line) in English — the fallback for a kind the surface cannot name. Absent while `status` is `pending`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl CheckoutSessionBreakdownDisplayRow {
    pub fn builder() -> CheckoutSessionBreakdownDisplayRowBuilder {
        <CheckoutSessionBreakdownDisplayRowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionBreakdownDisplayRowBuilder {
    amount: Option<Money>,
    detail: Option<CheckoutSessionBreakdownDisplayRowDetail>,
    kind: Option<CheckoutSessionBreakdownDisplayRowKind>,
    label: Option<String>,
    status: Option<CheckoutSessionBreakdownDisplayRowStatus>,
    text: Option<String>,
}

impl CheckoutSessionBreakdownDisplayRowBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn detail(mut self, value: CheckoutSessionBreakdownDisplayRowDetail) -> Self {
        self.detail = Some(value);
        self
    }

    pub fn kind(mut self, value: CheckoutSessionBreakdownDisplayRowKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn status(mut self, value: CheckoutSessionBreakdownDisplayRowStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionBreakdownDisplayRow`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](CheckoutSessionBreakdownDisplayRowBuilder::kind)
    pub fn build(self) -> Result<CheckoutSessionBreakdownDisplayRow, BuildError> {
        Ok(CheckoutSessionBreakdownDisplayRow {
            amount: self.amount,
            detail: self.detail,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            label: self.label,
            status: self.status,
            text: self.text,
        })
    }
}
