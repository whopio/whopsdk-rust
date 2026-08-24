pub use crate::prelude::*;

/// Query parameters for unlinkIdentityProfile
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnlinkIdentityProfileQueryRequest {
    /// The ID of the LedgerAccount to unlink the identity profile from.
    #[serde(default)]
    pub ledger_account_id: String,
}

impl UnlinkIdentityProfileQueryRequest {
    pub fn builder() -> UnlinkIdentityProfileQueryRequestBuilder {
        <UnlinkIdentityProfileQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnlinkIdentityProfileQueryRequestBuilder {
    ledger_account_id: Option<String>,
}

impl UnlinkIdentityProfileQueryRequestBuilder {
    pub fn ledger_account_id(mut self, value: impl Into<String>) -> Self {
        self.ledger_account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnlinkIdentityProfileQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ledger_account_id`](UnlinkIdentityProfileQueryRequestBuilder::ledger_account_id)
    pub fn build(self) -> Result<UnlinkIdentityProfileQueryRequest, BuildError> {
        Ok(UnlinkIdentityProfileQueryRequest {
            ledger_account_id: self
                .ledger_account_id
                .ok_or_else(|| BuildError::missing_field("ledger_account_id"))?,
        })
    }
}
