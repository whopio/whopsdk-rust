pub use crate::prelude::*;

/// Payout destination display info (payout sources only).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LedgerActivitySourcePayoutDestination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<String>,
}

impl LedgerActivitySourcePayoutDestination {
    pub fn builder() -> LedgerActivitySourcePayoutDestinationBuilder {
        <LedgerActivitySourcePayoutDestinationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivitySourcePayoutDestinationBuilder {
    icon_url: Option<String>,
    payer_name: Option<String>,
}

impl LedgerActivitySourcePayoutDestinationBuilder {
    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn payer_name(mut self, value: impl Into<String>) -> Self {
        self.payer_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivitySourcePayoutDestination`].
    pub fn build(self) -> Result<LedgerActivitySourcePayoutDestination, BuildError> {
        Ok(LedgerActivitySourcePayoutDestination {
            icon_url: self.icon_url,
            payer_name: self.payer_name,
        })
    }
}
