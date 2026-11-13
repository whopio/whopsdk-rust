pub use crate::prelude::*;

/// How many of the matching payments are in each substatus. Every substatus is present, including those with a count of zero.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SummaryPaymentsResponseGroupsSubstatus {
    #[serde(default)]
    pub auto_refunded: i64,
    #[serde(default)]
    pub blocked: i64,
    #[serde(default)]
    pub canceled: i64,
    #[serde(default)]
    pub dispute_closed: i64,
    #[serde(default)]
    pub dispute_lost: i64,
    #[serde(default)]
    pub dispute_needs_response: i64,
    #[serde(default)]
    pub dispute_under_review: i64,
    #[serde(default)]
    pub dispute_warning: i64,
    #[serde(default)]
    pub dispute_warning_closed: i64,
    #[serde(default)]
    pub dispute_warning_needs_response: i64,
    #[serde(default)]
    pub dispute_warning_under_review: i64,
    #[serde(default)]
    pub dispute_won: i64,
    #[serde(default)]
    pub drafted: i64,
    #[serde(default)]
    pub failed: i64,
    #[serde(default)]
    pub incomplete: i64,
    #[serde(default)]
    pub partially_refunded: i64,
    #[serde(default)]
    pub past_due: i64,
    #[serde(default)]
    pub pending: i64,
    #[serde(default)]
    pub price_too_low: i64,
    #[serde(default)]
    pub refunded: i64,
    #[serde(default)]
    pub requires_capture: i64,
    #[serde(default)]
    pub resolution_lost: i64,
    #[serde(default)]
    pub resolution_needs_response: i64,
    #[serde(default)]
    pub resolution_under_review: i64,
    #[serde(default)]
    pub resolution_won: i64,
    #[serde(default)]
    pub succeeded: i64,
    #[serde(default)]
    pub uncollectible: i64,
    #[serde(default)]
    pub unresolved: i64,
}

impl SummaryPaymentsResponseGroupsSubstatus {
    pub fn builder() -> SummaryPaymentsResponseGroupsSubstatusBuilder {
        <SummaryPaymentsResponseGroupsSubstatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryPaymentsResponseGroupsSubstatusBuilder {
    auto_refunded: Option<i64>,
    blocked: Option<i64>,
    canceled: Option<i64>,
    dispute_closed: Option<i64>,
    dispute_lost: Option<i64>,
    dispute_needs_response: Option<i64>,
    dispute_under_review: Option<i64>,
    dispute_warning: Option<i64>,
    dispute_warning_closed: Option<i64>,
    dispute_warning_needs_response: Option<i64>,
    dispute_warning_under_review: Option<i64>,
    dispute_won: Option<i64>,
    drafted: Option<i64>,
    failed: Option<i64>,
    incomplete: Option<i64>,
    partially_refunded: Option<i64>,
    past_due: Option<i64>,
    pending: Option<i64>,
    price_too_low: Option<i64>,
    refunded: Option<i64>,
    requires_capture: Option<i64>,
    resolution_lost: Option<i64>,
    resolution_needs_response: Option<i64>,
    resolution_under_review: Option<i64>,
    resolution_won: Option<i64>,
    succeeded: Option<i64>,
    uncollectible: Option<i64>,
    unresolved: Option<i64>,
}

impl SummaryPaymentsResponseGroupsSubstatusBuilder {
    pub fn auto_refunded(mut self, value: i64) -> Self {
        self.auto_refunded = Some(value);
        self
    }

    pub fn blocked(mut self, value: i64) -> Self {
        self.blocked = Some(value);
        self
    }

    pub fn canceled(mut self, value: i64) -> Self {
        self.canceled = Some(value);
        self
    }

    pub fn dispute_closed(mut self, value: i64) -> Self {
        self.dispute_closed = Some(value);
        self
    }

    pub fn dispute_lost(mut self, value: i64) -> Self {
        self.dispute_lost = Some(value);
        self
    }

    pub fn dispute_needs_response(mut self, value: i64) -> Self {
        self.dispute_needs_response = Some(value);
        self
    }

    pub fn dispute_under_review(mut self, value: i64) -> Self {
        self.dispute_under_review = Some(value);
        self
    }

    pub fn dispute_warning(mut self, value: i64) -> Self {
        self.dispute_warning = Some(value);
        self
    }

    pub fn dispute_warning_closed(mut self, value: i64) -> Self {
        self.dispute_warning_closed = Some(value);
        self
    }

    pub fn dispute_warning_needs_response(mut self, value: i64) -> Self {
        self.dispute_warning_needs_response = Some(value);
        self
    }

    pub fn dispute_warning_under_review(mut self, value: i64) -> Self {
        self.dispute_warning_under_review = Some(value);
        self
    }

    pub fn dispute_won(mut self, value: i64) -> Self {
        self.dispute_won = Some(value);
        self
    }

    pub fn drafted(mut self, value: i64) -> Self {
        self.drafted = Some(value);
        self
    }

    pub fn failed(mut self, value: i64) -> Self {
        self.failed = Some(value);
        self
    }

    pub fn incomplete(mut self, value: i64) -> Self {
        self.incomplete = Some(value);
        self
    }

    pub fn partially_refunded(mut self, value: i64) -> Self {
        self.partially_refunded = Some(value);
        self
    }

    pub fn past_due(mut self, value: i64) -> Self {
        self.past_due = Some(value);
        self
    }

    pub fn pending(mut self, value: i64) -> Self {
        self.pending = Some(value);
        self
    }

    pub fn price_too_low(mut self, value: i64) -> Self {
        self.price_too_low = Some(value);
        self
    }

    pub fn refunded(mut self, value: i64) -> Self {
        self.refunded = Some(value);
        self
    }

    pub fn requires_capture(mut self, value: i64) -> Self {
        self.requires_capture = Some(value);
        self
    }

    pub fn resolution_lost(mut self, value: i64) -> Self {
        self.resolution_lost = Some(value);
        self
    }

    pub fn resolution_needs_response(mut self, value: i64) -> Self {
        self.resolution_needs_response = Some(value);
        self
    }

    pub fn resolution_under_review(mut self, value: i64) -> Self {
        self.resolution_under_review = Some(value);
        self
    }

    pub fn resolution_won(mut self, value: i64) -> Self {
        self.resolution_won = Some(value);
        self
    }

    pub fn succeeded(mut self, value: i64) -> Self {
        self.succeeded = Some(value);
        self
    }

    pub fn uncollectible(mut self, value: i64) -> Self {
        self.uncollectible = Some(value);
        self
    }

    pub fn unresolved(mut self, value: i64) -> Self {
        self.unresolved = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryPaymentsResponseGroupsSubstatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_refunded`](SummaryPaymentsResponseGroupsSubstatusBuilder::auto_refunded)
    /// - [`blocked`](SummaryPaymentsResponseGroupsSubstatusBuilder::blocked)
    /// - [`canceled`](SummaryPaymentsResponseGroupsSubstatusBuilder::canceled)
    /// - [`dispute_closed`](SummaryPaymentsResponseGroupsSubstatusBuilder::dispute_closed)
    /// - [`dispute_lost`](SummaryPaymentsResponseGroupsSubstatusBuilder::dispute_lost)
    /// - [`dispute_needs_response`](SummaryPaymentsResponseGroupsSubstatusBuilder::dispute_needs_response)
    /// - [`dispute_under_review`](SummaryPaymentsResponseGroupsSubstatusBuilder::dispute_under_review)
    /// - [`dispute_warning`](SummaryPaymentsResponseGroupsSubstatusBuilder::dispute_warning)
    /// - [`dispute_warning_closed`](SummaryPaymentsResponseGroupsSubstatusBuilder::dispute_warning_closed)
    /// - [`dispute_warning_needs_response`](SummaryPaymentsResponseGroupsSubstatusBuilder::dispute_warning_needs_response)
    /// - [`dispute_warning_under_review`](SummaryPaymentsResponseGroupsSubstatusBuilder::dispute_warning_under_review)
    /// - [`dispute_won`](SummaryPaymentsResponseGroupsSubstatusBuilder::dispute_won)
    /// - [`drafted`](SummaryPaymentsResponseGroupsSubstatusBuilder::drafted)
    /// - [`failed`](SummaryPaymentsResponseGroupsSubstatusBuilder::failed)
    /// - [`incomplete`](SummaryPaymentsResponseGroupsSubstatusBuilder::incomplete)
    /// - [`partially_refunded`](SummaryPaymentsResponseGroupsSubstatusBuilder::partially_refunded)
    /// - [`past_due`](SummaryPaymentsResponseGroupsSubstatusBuilder::past_due)
    /// - [`pending`](SummaryPaymentsResponseGroupsSubstatusBuilder::pending)
    /// - [`price_too_low`](SummaryPaymentsResponseGroupsSubstatusBuilder::price_too_low)
    /// - [`refunded`](SummaryPaymentsResponseGroupsSubstatusBuilder::refunded)
    /// - [`requires_capture`](SummaryPaymentsResponseGroupsSubstatusBuilder::requires_capture)
    /// - [`resolution_lost`](SummaryPaymentsResponseGroupsSubstatusBuilder::resolution_lost)
    /// - [`resolution_needs_response`](SummaryPaymentsResponseGroupsSubstatusBuilder::resolution_needs_response)
    /// - [`resolution_under_review`](SummaryPaymentsResponseGroupsSubstatusBuilder::resolution_under_review)
    /// - [`resolution_won`](SummaryPaymentsResponseGroupsSubstatusBuilder::resolution_won)
    /// - [`succeeded`](SummaryPaymentsResponseGroupsSubstatusBuilder::succeeded)
    /// - [`uncollectible`](SummaryPaymentsResponseGroupsSubstatusBuilder::uncollectible)
    /// - [`unresolved`](SummaryPaymentsResponseGroupsSubstatusBuilder::unresolved)
    pub fn build(self) -> Result<SummaryPaymentsResponseGroupsSubstatus, BuildError> {
        Ok(SummaryPaymentsResponseGroupsSubstatus {
            auto_refunded: self
                .auto_refunded
                .ok_or_else(|| BuildError::missing_field("auto_refunded"))?,
            blocked: self
                .blocked
                .ok_or_else(|| BuildError::missing_field("blocked"))?,
            canceled: self
                .canceled
                .ok_or_else(|| BuildError::missing_field("canceled"))?,
            dispute_closed: self
                .dispute_closed
                .ok_or_else(|| BuildError::missing_field("dispute_closed"))?,
            dispute_lost: self
                .dispute_lost
                .ok_or_else(|| BuildError::missing_field("dispute_lost"))?,
            dispute_needs_response: self
                .dispute_needs_response
                .ok_or_else(|| BuildError::missing_field("dispute_needs_response"))?,
            dispute_under_review: self
                .dispute_under_review
                .ok_or_else(|| BuildError::missing_field("dispute_under_review"))?,
            dispute_warning: self
                .dispute_warning
                .ok_or_else(|| BuildError::missing_field("dispute_warning"))?,
            dispute_warning_closed: self
                .dispute_warning_closed
                .ok_or_else(|| BuildError::missing_field("dispute_warning_closed"))?,
            dispute_warning_needs_response: self
                .dispute_warning_needs_response
                .ok_or_else(|| BuildError::missing_field("dispute_warning_needs_response"))?,
            dispute_warning_under_review: self
                .dispute_warning_under_review
                .ok_or_else(|| BuildError::missing_field("dispute_warning_under_review"))?,
            dispute_won: self
                .dispute_won
                .ok_or_else(|| BuildError::missing_field("dispute_won"))?,
            drafted: self
                .drafted
                .ok_or_else(|| BuildError::missing_field("drafted"))?,
            failed: self
                .failed
                .ok_or_else(|| BuildError::missing_field("failed"))?,
            incomplete: self
                .incomplete
                .ok_or_else(|| BuildError::missing_field("incomplete"))?,
            partially_refunded: self
                .partially_refunded
                .ok_or_else(|| BuildError::missing_field("partially_refunded"))?,
            past_due: self
                .past_due
                .ok_or_else(|| BuildError::missing_field("past_due"))?,
            pending: self
                .pending
                .ok_or_else(|| BuildError::missing_field("pending"))?,
            price_too_low: self
                .price_too_low
                .ok_or_else(|| BuildError::missing_field("price_too_low"))?,
            refunded: self
                .refunded
                .ok_or_else(|| BuildError::missing_field("refunded"))?,
            requires_capture: self
                .requires_capture
                .ok_or_else(|| BuildError::missing_field("requires_capture"))?,
            resolution_lost: self
                .resolution_lost
                .ok_or_else(|| BuildError::missing_field("resolution_lost"))?,
            resolution_needs_response: self
                .resolution_needs_response
                .ok_or_else(|| BuildError::missing_field("resolution_needs_response"))?,
            resolution_under_review: self
                .resolution_under_review
                .ok_or_else(|| BuildError::missing_field("resolution_under_review"))?,
            resolution_won: self
                .resolution_won
                .ok_or_else(|| BuildError::missing_field("resolution_won"))?,
            succeeded: self
                .succeeded
                .ok_or_else(|| BuildError::missing_field("succeeded"))?,
            uncollectible: self
                .uncollectible
                .ok_or_else(|| BuildError::missing_field("uncollectible"))?,
            unresolved: self
                .unresolved
                .ok_or_else(|| BuildError::missing_field("unresolved"))?,
        })
    }
}
