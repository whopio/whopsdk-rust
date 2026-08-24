pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountBalanceBreakdown {
    /// Amount you can spend, send, or withdraw now, in native units, as a decimal string.
    #[serde(default)]
    pub available: String,
    /// Amount moving between the account's own destinations, such as a treasury sweep to its crypto wallet or a card top-up. In native units, as a decimal string.
    #[serde(default)]
    pub in_transit: String,
    /// Amount from recent payments still settling, in native units, as a decimal string.
    #[serde(default)]
    pub pending: String,
    #[serde(default)]
    pub pending_settlements: Vec<AccountBalanceSettlement>,
    /// Amount held back, in native units, as a decimal string. Retrieve the account's reserves for why it is held and when it unlocks.
    #[serde(default)]
    pub reserve: String,
}

impl AccountBalanceBreakdown {
    pub fn builder() -> AccountBalanceBreakdownBuilder {
        <AccountBalanceBreakdownBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountBalanceBreakdownBuilder {
    available: Option<String>,
    in_transit: Option<String>,
    pending: Option<String>,
    pending_settlements: Option<Vec<AccountBalanceSettlement>>,
    reserve: Option<String>,
}

impl AccountBalanceBreakdownBuilder {
    pub fn available(mut self, value: impl Into<String>) -> Self {
        self.available = Some(value.into());
        self
    }

    pub fn in_transit(mut self, value: impl Into<String>) -> Self {
        self.in_transit = Some(value.into());
        self
    }

    pub fn pending(mut self, value: impl Into<String>) -> Self {
        self.pending = Some(value.into());
        self
    }

    pub fn pending_settlements(mut self, value: Vec<AccountBalanceSettlement>) -> Self {
        self.pending_settlements = Some(value);
        self
    }

    pub fn reserve(mut self, value: impl Into<String>) -> Self {
        self.reserve = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountBalanceBreakdown`].
    /// This method will fail if any of the following fields are not set:
    /// - [`available`](AccountBalanceBreakdownBuilder::available)
    /// - [`in_transit`](AccountBalanceBreakdownBuilder::in_transit)
    /// - [`pending`](AccountBalanceBreakdownBuilder::pending)
    /// - [`pending_settlements`](AccountBalanceBreakdownBuilder::pending_settlements)
    /// - [`reserve`](AccountBalanceBreakdownBuilder::reserve)
    pub fn build(self) -> Result<AccountBalanceBreakdown, BuildError> {
        Ok(AccountBalanceBreakdown {
            available: self
                .available
                .ok_or_else(|| BuildError::missing_field("available"))?,
            in_transit: self
                .in_transit
                .ok_or_else(|| BuildError::missing_field("in_transit"))?,
            pending: self
                .pending
                .ok_or_else(|| BuildError::missing_field("pending"))?,
            pending_settlements: self
                .pending_settlements
                .ok_or_else(|| BuildError::missing_field("pending_settlements"))?,
            reserve: self
                .reserve
                .ok_or_else(|| BuildError::missing_field("reserve"))?,
        })
    }
}
