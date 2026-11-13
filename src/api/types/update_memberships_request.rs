pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateMembershipsRequest {
    /// Number of days between recurring charges, applied to every recurring variant so future renewals use the same cadence. Sets the current period end to the current period start plus this value; when the value is a multiple of 30 or is 365, a membership that renews on a fixed day of the month (for example, after `current_period_end` was set) ends on that day instead. The new period end must remain in the future. Only for active, trialing, or past-due memberships billed automatically by Whop; invoice, externally billed, and canceling memberships are not supported. Existing non-daily memberships cannot be changed to daily billing. Cannot be combined with other fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_days: Option<i64>,
    /// `true` cancels at the end of the current billing period (the customer keeps access until then); `false` reverses a pending cancellation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,
    /// When the next renewal charges, as an ISO 8601 timestamp; a date without a time means midnight UTC. Must fall after today in UTC. Later renewals follow on from this date, and a membership charged automatically on a variant billed every 30, 60, 90 … 360 or 365 days keeps renewing on this day of the month in UTC, or on the month's last day when a month is shorter. Only for active or trialing renewing memberships billed by Whop whose payments are not paused. Cannot be combined with other fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub current_period_end: Option<DateTime<FixedOffset>>,
    /// Key-value pairs to merge into the membership's metadata. Pass an empty object to clear it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The ID of a payment method the customer has saved with your account. Future renewals charge it, and an open past-due payment is retried on it right away. Requires the `member:payment_methods:manage` permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// The amount future renewals charge, in the membership's renewal currency. Can only lower the current renewal price; raising a price is not supported here. Moves only this membership to the new price and emails the customer the change. Only for active or trialing renewing memberships billed by Whop whose payments are not paused. Cannot be combined with other fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub renewal_price: Option<f64>,
}

impl UpdateMembershipsRequest {
    pub fn builder() -> UpdateMembershipsRequestBuilder {
        <UpdateMembershipsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMembershipsRequestBuilder {
    billing_period_days: Option<i64>,
    cancel_at_period_end: Option<bool>,
    current_period_end: Option<DateTime<FixedOffset>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    payment_method_id: Option<String>,
    renewal_price: Option<f64>,
}

impl UpdateMembershipsRequestBuilder {
    pub fn billing_period_days(mut self, value: i64) -> Self {
        self.billing_period_days = Some(value);
        self
    }

    pub fn cancel_at_period_end(mut self, value: bool) -> Self {
        self.cancel_at_period_end = Some(value);
        self
    }

    pub fn current_period_end(mut self, value: DateTime<FixedOffset>) -> Self {
        self.current_period_end = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    pub fn renewal_price(mut self, value: f64) -> Self {
        self.renewal_price = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMembershipsRequest`].
    pub fn build(self) -> Result<UpdateMembershipsRequest, BuildError> {
        Ok(UpdateMembershipsRequest {
            billing_period_days: self.billing_period_days,
            cancel_at_period_end: self.cancel_at_period_end,
            current_period_end: self.current_period_end,
            metadata: self.metadata,
            payment_method_id: self.payment_method_id,
            renewal_price: self.renewal_price,
        })
    }
}
