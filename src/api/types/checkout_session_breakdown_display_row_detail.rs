pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutSessionBreakdownDisplayRowDetail {
    /// How many days of access a one-time purchase grants, when it expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_days: Option<i64>,
    /// When a transferred membership's access ends, as an ISO 8601 timestamp — the recipient gets the origin's REMAINING time, never a fresh grant of the plan's window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_until: Option<String>,
    /// The tax row's application: `added` counts toward the total; `included` is already inside the prices and is disclosed, never added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied: Option<CheckoutSessionBreakdownDisplayRowDetailApplied>,
    /// The instant this row's charge lands, as an ISO 8601 timestamp — format it in the buyer's locale rather than deriving a date client-side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_at: Option<String>,
    /// The promo code (uppercased) whose coverage this row states — "Then your first month is free with FREETODAY".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The one-off fee inside today's `due_today` charge — name it ("Due today — includes a one-time $50.00 fee") so the figure does not read as the recurring rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_fee: Option<Money>,
    /// Days between recurring charges, for wording the billing period (30 is monthly, 365 yearly).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_days: Option<i64>,
    /// Present (and `true`) when this one-time payment collects a seller-issued invoice — word it as a bill ("Invoice — one-time payment.") beside the session's own due date, never as a storefront purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<bool>,
    /// Present (and `true`) when the trial this row follows charges a fee today — the wording drops the word "trial" for the neutral "after N days", exactly as the legacy checkout worded a paid trial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_trial: Option<bool>,
    /// The tax row's effective rate as a decimal fraction (`"0.0725"` is 7.25%), when the calculation stated one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    /// How many payments are still AHEAD at this row's amount — today's paying receipt already consumed one of the plan's total, exactly as billing counts receipts. Word it as the future stream ("for 3 more payments"), never as the plan's total.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_payments: Option<i64>,
    /// An installment plan's PLAN-WIDE payment count, today's paying charge included — word it as the plan's total ("for a total of 4 payments"). Mutually exclusive with `remaining_payments`: a row states whichever count it means.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_payments: Option<i64>,
    /// The free-trial length this row speaks about, in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i64>,
    /// Present (and `true`) when this row's amount is stated before tax and the wording must say so — the checkout adds exclusive tax the future charge will carry on top.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_tax: Option<bool>,
}

impl CheckoutSessionBreakdownDisplayRowDetail {
    pub fn builder() -> CheckoutSessionBreakdownDisplayRowDetailBuilder {
        <CheckoutSessionBreakdownDisplayRowDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionBreakdownDisplayRowDetailBuilder {
    access_days: Option<i64>,
    access_until: Option<String>,
    applied: Option<CheckoutSessionBreakdownDisplayRowDetailApplied>,
    charge_at: Option<String>,
    code: Option<String>,
    initial_fee: Option<Money>,
    interval_days: Option<i64>,
    invoice: Option<bool>,
    paid_trial: Option<bool>,
    rate: Option<String>,
    remaining_payments: Option<i64>,
    total_payments: Option<i64>,
    trial_days: Option<i64>,
    without_tax: Option<bool>,
}

impl CheckoutSessionBreakdownDisplayRowDetailBuilder {
    pub fn access_days(mut self, value: i64) -> Self {
        self.access_days = Some(value);
        self
    }

    pub fn access_until(mut self, value: impl Into<String>) -> Self {
        self.access_until = Some(value.into());
        self
    }

    pub fn applied(mut self, value: CheckoutSessionBreakdownDisplayRowDetailApplied) -> Self {
        self.applied = Some(value);
        self
    }

    pub fn charge_at(mut self, value: impl Into<String>) -> Self {
        self.charge_at = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn initial_fee(mut self, value: Money) -> Self {
        self.initial_fee = Some(value);
        self
    }

    pub fn interval_days(mut self, value: i64) -> Self {
        self.interval_days = Some(value);
        self
    }

    pub fn invoice(mut self, value: bool) -> Self {
        self.invoice = Some(value);
        self
    }

    pub fn paid_trial(mut self, value: bool) -> Self {
        self.paid_trial = Some(value);
        self
    }

    pub fn rate(mut self, value: impl Into<String>) -> Self {
        self.rate = Some(value.into());
        self
    }

    pub fn remaining_payments(mut self, value: i64) -> Self {
        self.remaining_payments = Some(value);
        self
    }

    pub fn total_payments(mut self, value: i64) -> Self {
        self.total_payments = Some(value);
        self
    }

    pub fn trial_days(mut self, value: i64) -> Self {
        self.trial_days = Some(value);
        self
    }

    pub fn without_tax(mut self, value: bool) -> Self {
        self.without_tax = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionBreakdownDisplayRowDetail`].
    pub fn build(self) -> Result<CheckoutSessionBreakdownDisplayRowDetail, BuildError> {
        Ok(CheckoutSessionBreakdownDisplayRowDetail {
            access_days: self.access_days,
            access_until: self.access_until,
            applied: self.applied,
            charge_at: self.charge_at,
            code: self.code,
            initial_fee: self.initial_fee,
            interval_days: self.interval_days,
            invoice: self.invoice,
            paid_trial: self.paid_trial,
            rate: self.rate,
            remaining_payments: self.remaining_payments,
            total_payments: self.total_payments,
            trial_days: self.trial_days,
            without_tax: self.without_tax,
        })
    }
}
