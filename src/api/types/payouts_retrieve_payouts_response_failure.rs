pub use crate::prelude::*;

/// Why the payout ended without paying, or why it reversed after settlement. Present on failed, canceled, denied, and reversed payouts; `null` otherwise.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePayoutsResponseFailure {
    /// Classified failure code from the maintained error catalog.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The effective time of the reversal that put the funds back in the balance — `null` if they never left it or have not returned yet. Set only once the return is confirmed in the ledger; the ledger posting itself can land moments after this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub funds_returned_at: Option<DateTime<FixedOffset>>,
    /// Human-readable explanation of the failure. Callers holding `payout:destination:read` may receive text personalized to the destination; other callers get the generic catalog message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl RetrievePayoutsResponseFailure {
    pub fn builder() -> RetrievePayoutsResponseFailureBuilder {
        <RetrievePayoutsResponseFailureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePayoutsResponseFailureBuilder {
    code: Option<String>,
    funds_returned_at: Option<DateTime<FixedOffset>>,
    message: Option<String>,
}

impl RetrievePayoutsResponseFailureBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn funds_returned_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.funds_returned_at = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePayoutsResponseFailure`].
    pub fn build(self) -> Result<RetrievePayoutsResponseFailure, BuildError> {
        Ok(RetrievePayoutsResponseFailure {
            code: self.code,
            funds_returned_at: self.funds_returned_at,
            message: self.message,
        })
    }
}
