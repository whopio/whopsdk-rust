pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionBreakdownDisplayHeadline {
    /// The figure to headline when `kind` is `amount` — already posture-correct. `null` on the other kinds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    #[serde(default)]
    pub context: Vec<CheckoutSessionBreakdownDisplayRow>,
    /// When `kind` is `free` on a renewing membership transfer: the instant the transferred paid period ends and the takeover starts billing, as an ISO 8601 timestamp — word the headline "Free until …". `null` everywhere else.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_until: Option<String>,
    /// The headline's face: `amount` headlines the figure in `amount`; `trial` headlines the free-trial length off `trial_days`; `free` headlines the word — the served free signal or a membership transfer collecting nothing today (a renewing transfer states when its free stretch ends in `free_until`), never derived from zero prices (a promo covering the whole first charge is not a free plan).
    pub kind: CheckoutSessionBreakdownDisplayHeadlineKind,
    /// The pre-promo figure to strike through beside `amount`, or `null` when no discount moved today's charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<Money>,
    /// The headline in English — the fallback for a kind the surface cannot name. A surface that knows the kind renders its own wording from `amount`/`trial_days`/`free_until`.
    #[serde(default)]
    pub text: String,
    /// The free-trial length when `kind` is `trial`, in days. `null` on the other kinds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i64>,
}

impl CheckoutSessionBreakdownDisplayHeadline {
    pub fn builder() -> CheckoutSessionBreakdownDisplayHeadlineBuilder {
        <CheckoutSessionBreakdownDisplayHeadlineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionBreakdownDisplayHeadlineBuilder {
    amount: Option<Money>,
    context: Option<Vec<CheckoutSessionBreakdownDisplayRow>>,
    free_until: Option<String>,
    kind: Option<CheckoutSessionBreakdownDisplayHeadlineKind>,
    original_amount: Option<Money>,
    text: Option<String>,
    trial_days: Option<i64>,
}

impl CheckoutSessionBreakdownDisplayHeadlineBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn context(mut self, value: Vec<CheckoutSessionBreakdownDisplayRow>) -> Self {
        self.context = Some(value);
        self
    }

    pub fn free_until(mut self, value: impl Into<String>) -> Self {
        self.free_until = Some(value.into());
        self
    }

    pub fn kind(mut self, value: CheckoutSessionBreakdownDisplayHeadlineKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn original_amount(mut self, value: Money) -> Self {
        self.original_amount = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn trial_days(mut self, value: i64) -> Self {
        self.trial_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionBreakdownDisplayHeadline`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context`](CheckoutSessionBreakdownDisplayHeadlineBuilder::context)
    /// - [`kind`](CheckoutSessionBreakdownDisplayHeadlineBuilder::kind)
    /// - [`text`](CheckoutSessionBreakdownDisplayHeadlineBuilder::text)
    pub fn build(self) -> Result<CheckoutSessionBreakdownDisplayHeadline, BuildError> {
        Ok(CheckoutSessionBreakdownDisplayHeadline {
            amount: self.amount,
            context: self
                .context
                .ok_or_else(|| BuildError::missing_field("context"))?,
            free_until: self.free_until,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            original_amount: self.original_amount,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            trial_days: self.trial_days,
        })
    }
}
