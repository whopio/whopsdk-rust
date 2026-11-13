pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountCoveredPayoutFees {
    /// Whether this account pays payout fees for all of its subaccounts' payout methods. Turning this off clears category coverage.
    #[serde(default)]
    pub all: bool,
    /// Whether this account covers its subaccounts' bank wire payout fees. Individual changes have no effect while `all` is true.
    #[serde(default)]
    pub bank_wire: bool,
    /// Whether this account covers its subaccounts' crypto payout fees. Individual changes have no effect while `all` is true.
    #[serde(default)]
    pub crypto: bool,
    /// Whether this account covers its subaccounts' digital wallet payout fees. Individual changes have no effect while `all` is true.
    #[serde(default)]
    pub digital_wallet: bool,
    /// Whether this account covers its subaccounts' next day bank payout fees. Individual changes have no effect while `all` is true.
    #[serde(default)]
    pub next_day_bank: bool,
    /// Whether this account covers its subaccounts' real-time payment payout fees. Individual changes have no effect while `all` is true.
    #[serde(default)]
    pub rtp: bool,
}

impl AccountCoveredPayoutFees {
    pub fn builder() -> AccountCoveredPayoutFeesBuilder {
        <AccountCoveredPayoutFeesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountCoveredPayoutFeesBuilder {
    all: Option<bool>,
    bank_wire: Option<bool>,
    crypto: Option<bool>,
    digital_wallet: Option<bool>,
    next_day_bank: Option<bool>,
    rtp: Option<bool>,
}

impl AccountCoveredPayoutFeesBuilder {
    pub fn all(mut self, value: bool) -> Self {
        self.all = Some(value);
        self
    }

    pub fn bank_wire(mut self, value: bool) -> Self {
        self.bank_wire = Some(value);
        self
    }

    pub fn crypto(mut self, value: bool) -> Self {
        self.crypto = Some(value);
        self
    }

    pub fn digital_wallet(mut self, value: bool) -> Self {
        self.digital_wallet = Some(value);
        self
    }

    pub fn next_day_bank(mut self, value: bool) -> Self {
        self.next_day_bank = Some(value);
        self
    }

    pub fn rtp(mut self, value: bool) -> Self {
        self.rtp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountCoveredPayoutFees`].
    /// This method will fail if any of the following fields are not set:
    /// - [`all`](AccountCoveredPayoutFeesBuilder::all)
    /// - [`bank_wire`](AccountCoveredPayoutFeesBuilder::bank_wire)
    /// - [`crypto`](AccountCoveredPayoutFeesBuilder::crypto)
    /// - [`digital_wallet`](AccountCoveredPayoutFeesBuilder::digital_wallet)
    /// - [`next_day_bank`](AccountCoveredPayoutFeesBuilder::next_day_bank)
    /// - [`rtp`](AccountCoveredPayoutFeesBuilder::rtp)
    pub fn build(self) -> Result<AccountCoveredPayoutFees, BuildError> {
        Ok(AccountCoveredPayoutFees {
            all: self.all.ok_or_else(|| BuildError::missing_field("all"))?,
            bank_wire: self
                .bank_wire
                .ok_or_else(|| BuildError::missing_field("bank_wire"))?,
            crypto: self
                .crypto
                .ok_or_else(|| BuildError::missing_field("crypto"))?,
            digital_wallet: self
                .digital_wallet
                .ok_or_else(|| BuildError::missing_field("digital_wallet"))?,
            next_day_bank: self
                .next_day_bank
                .ok_or_else(|| BuildError::missing_field("next_day_bank"))?,
            rtp: self.rtp.ok_or_else(|| BuildError::missing_field("rtp"))?,
        })
    }
}
