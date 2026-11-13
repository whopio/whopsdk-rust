pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentLevelsRebate {
    /// Part of the account's monthly rebate this payment earned, in the currency it settled in. The rebate is paid to the account as one sum, so the payment's own amounts and fees don't change. `null` unless `status` is `rebated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// How much of the payment the month's spend covered, from `0` to `1`: `1` for every rebated payment but the month's last, which can be covered in part, so its rebate is that share of the fee above the level's rate. `null` unless `status` is `rebated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub covered_share: Option<f64>,
    /// Processing fee as a percentage of the amount it was charged on, before the rebate, where `2.7` means 2.7%. `null` unless `status` is `rebated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub effective_rate_percentage: Option<f64>,
    /// Why the payment can't earn a rebate: `not_card_payment`, `before_levels` (made before Whop Levels started), `not_in_levels` (its account wasn't in Whop Levels that month), `fee_not_posted`, `fee_at_level_rate` (its fee was already at or below the level's rate) or `no_level_rate` (its fee matched no level rate). `null` unless `status` is `ineligible`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ineligible_reason: Option<PaymentLevelsRebateIneligibleReason>,
    /// Level the account's month closed at, from `0` up to `10`. `null` until that month's close has applied to the payment, and for a payment that can't earn a rebate before then, such as a non-card payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    /// Start of the UTC month whose close covers this payment, as an ISO 8601 timestamp. `null` before the processing fee has posted, and for a payment that can't earn a rebate before then, such as a non-card payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month_started_at: Option<String>,
    /// Payment ID, prefixed `pay_`.
    #[serde(default)]
    pub payment_id: String,
    /// Card processing percentage at `level` for cards from `region`, where `2.1` means 2.1%. The rebate is the fee above it. `null` unless `status` is `rebated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub processing_rate_percentage: Option<f64>,
    /// Card region the processing fee was charged in, which sets `processing_rate_percentage`. `null` unless `status` is `rebated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<PaymentLevelsRebateRegion>,
    /// `rebated` when the payment earned part of its account's monthly Whop Levels rebate. `pending` until the month its processing fee posted in has closed. `not_covered` when the account's spend that month ran out before this payment. `ineligible` when it can't earn one; `ineligible_reason` says why.
    pub status: PaymentLevelsRebateStatus,
}

impl PaymentLevelsRebate {
    pub fn builder() -> PaymentLevelsRebateBuilder {
        <PaymentLevelsRebateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentLevelsRebateBuilder {
    amount: Option<Money>,
    covered_share: Option<f64>,
    effective_rate_percentage: Option<f64>,
    ineligible_reason: Option<PaymentLevelsRebateIneligibleReason>,
    level: Option<i64>,
    month_started_at: Option<String>,
    payment_id: Option<String>,
    processing_rate_percentage: Option<f64>,
    region: Option<PaymentLevelsRebateRegion>,
    status: Option<PaymentLevelsRebateStatus>,
}

impl PaymentLevelsRebateBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn covered_share(mut self, value: f64) -> Self {
        self.covered_share = Some(value);
        self
    }

    pub fn effective_rate_percentage(mut self, value: f64) -> Self {
        self.effective_rate_percentage = Some(value);
        self
    }

    pub fn ineligible_reason(mut self, value: PaymentLevelsRebateIneligibleReason) -> Self {
        self.ineligible_reason = Some(value);
        self
    }

    pub fn level(mut self, value: i64) -> Self {
        self.level = Some(value);
        self
    }

    pub fn month_started_at(mut self, value: impl Into<String>) -> Self {
        self.month_started_at = Some(value.into());
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn processing_rate_percentage(mut self, value: f64) -> Self {
        self.processing_rate_percentage = Some(value);
        self
    }

    pub fn region(mut self, value: PaymentLevelsRebateRegion) -> Self {
        self.region = Some(value);
        self
    }

    pub fn status(mut self, value: PaymentLevelsRebateStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentLevelsRebate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_id`](PaymentLevelsRebateBuilder::payment_id)
    /// - [`status`](PaymentLevelsRebateBuilder::status)
    pub fn build(self) -> Result<PaymentLevelsRebate, BuildError> {
        Ok(PaymentLevelsRebate {
            amount: self.amount,
            covered_share: self.covered_share,
            effective_rate_percentage: self.effective_rate_percentage,
            ineligible_reason: self.ineligible_reason,
            level: self.level,
            month_started_at: self.month_started_at,
            payment_id: self
                .payment_id
                .ok_or_else(|| BuildError::missing_field("payment_id"))?,
            processing_rate_percentage: self.processing_rate_percentage,
            region: self.region,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
