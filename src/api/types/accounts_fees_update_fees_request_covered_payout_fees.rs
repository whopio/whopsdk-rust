pub use crate::prelude::*;

/// Changes to the payout fees this account covers for subaccounts. Send either all or individual category keys. Omitted categories stay unchanged; category changes have no effect while all is true.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateFeesRequestCoveredPayoutFees {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_wire: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_wallet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_day_bank: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtp: Option<bool>,
}

impl UpdateFeesRequestCoveredPayoutFees {
    pub fn builder() -> UpdateFeesRequestCoveredPayoutFeesBuilder {
        <UpdateFeesRequestCoveredPayoutFeesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestCoveredPayoutFeesBuilder {
    all: Option<bool>,
    bank_wire: Option<bool>,
    crypto: Option<bool>,
    digital_wallet: Option<bool>,
    next_day_bank: Option<bool>,
    rtp: Option<bool>,
}

impl UpdateFeesRequestCoveredPayoutFeesBuilder {
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

    /// Consumes the builder and constructs a [`UpdateFeesRequestCoveredPayoutFees`].
    pub fn build(self) -> Result<UpdateFeesRequestCoveredPayoutFees, BuildError> {
        Ok(UpdateFeesRequestCoveredPayoutFees {
            all: self.all,
            bank_wire: self.bank_wire,
            crypto: self.crypto,
            digital_wallet: self.digital_wallet,
            next_day_bank: self.next_day_bank,
            rtp: self.rtp,
        })
    }
}
