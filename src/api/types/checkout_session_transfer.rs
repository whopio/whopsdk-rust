pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutSessionTransfer {
    /// The 40-character transfer code this checkout was opened from — the same value the link carried.
    #[serde(default)]
    pub code: String,
    /// When the transferred membership's access expires, as an ISO 8601 timestamp, or `null` when it does not. The recipient inherits this — a transfer moves the remaining time, it does not restart it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_expires_at: Option<String>,
    /// When the transferred membership's current billing period ends, as an ISO 8601 timestamp, or `null` for a non-renewing membership. The recipient's first renewal charge falls here — nothing is charged at the transfer itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_renewal_period_end: Option<String>,
}

impl CheckoutSessionTransfer {
    pub fn builder() -> CheckoutSessionTransferBuilder {
        <CheckoutSessionTransferBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionTransferBuilder {
    code: Option<String>,
    origin_expires_at: Option<String>,
    origin_renewal_period_end: Option<String>,
}

impl CheckoutSessionTransferBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn origin_expires_at(mut self, value: impl Into<String>) -> Self {
        self.origin_expires_at = Some(value.into());
        self
    }

    pub fn origin_renewal_period_end(mut self, value: impl Into<String>) -> Self {
        self.origin_renewal_period_end = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionTransfer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CheckoutSessionTransferBuilder::code)
    pub fn build(self) -> Result<CheckoutSessionTransfer, BuildError> {
        Ok(CheckoutSessionTransfer {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            origin_expires_at: self.origin_expires_at,
            origin_renewal_period_end: self.origin_renewal_period_end,
        })
    }
}
