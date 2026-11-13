pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResendInviteBusinessesResponseEarningsUsd {
    /// Commission already paid out, in USD.
    #[serde(default)]
    pub completed: String,
    /// Commission scheduled but not yet paid, in USD.
    #[serde(default)]
    pub pending: String,
    /// Pending + completed commission, in USD.
    #[serde(default)]
    pub total: String,
}

impl ResendInviteBusinessesResponseEarningsUsd {
    pub fn builder() -> ResendInviteBusinessesResponseEarningsUsdBuilder {
        <ResendInviteBusinessesResponseEarningsUsdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendInviteBusinessesResponseEarningsUsdBuilder {
    completed: Option<String>,
    pending: Option<String>,
    total: Option<String>,
}

impl ResendInviteBusinessesResponseEarningsUsdBuilder {
    pub fn completed(mut self, value: impl Into<String>) -> Self {
        self.completed = Some(value.into());
        self
    }

    pub fn pending(mut self, value: impl Into<String>) -> Self {
        self.pending = Some(value.into());
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResendInviteBusinessesResponseEarningsUsd`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completed`](ResendInviteBusinessesResponseEarningsUsdBuilder::completed)
    /// - [`pending`](ResendInviteBusinessesResponseEarningsUsdBuilder::pending)
    /// - [`total`](ResendInviteBusinessesResponseEarningsUsdBuilder::total)
    pub fn build(self) -> Result<ResendInviteBusinessesResponseEarningsUsd, BuildError> {
        Ok(ResendInviteBusinessesResponseEarningsUsd {
            completed: self
                .completed
                .ok_or_else(|| BuildError::missing_field("completed"))?,
            pending: self
                .pending
                .ok_or_else(|| BuildError::missing_field("pending"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
