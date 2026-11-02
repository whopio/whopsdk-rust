pub use crate::prelude::*;

/// This platform's default markups for every account connected to it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestChildMarkups {
    /// The new markup. Fields left out keep their current value; `null` clears the row so the markup returns to its default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_swaps: Option<UpdateFeesRequestChildMarkupsCryptoSwaps>,
    /// Markups on deposits, keyed by rail: `bank` or `crypto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposits: Option<HashMap<String, Option<UpdateFeesRequestChildMarkupsDepositsValue>>>,
    /// The new markup. Fields left out keep their current value; `null` clears the row so the markup returns to its default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<UpdateFeesRequestChildMarkupsPayments>,
    /// Markups on withdrawals, keyed by payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<HashMap<String, Option<UpdateFeesRequestChildMarkupsPayoutsValue>>>,
    /// The new markup. Fields left out keep their current value; `null` clears the row so the markup returns to its default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<UpdateFeesRequestChildMarkupsTransfers>,
}

impl UpdateFeesRequestChildMarkups {
    pub fn builder() -> UpdateFeesRequestChildMarkupsBuilder {
        <UpdateFeesRequestChildMarkupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestChildMarkupsBuilder {
    crypto_swaps: Option<UpdateFeesRequestChildMarkupsCryptoSwaps>,
    deposits: Option<HashMap<String, Option<UpdateFeesRequestChildMarkupsDepositsValue>>>,
    payments: Option<UpdateFeesRequestChildMarkupsPayments>,
    payouts: Option<HashMap<String, Option<UpdateFeesRequestChildMarkupsPayoutsValue>>>,
    transfers: Option<UpdateFeesRequestChildMarkupsTransfers>,
}

impl UpdateFeesRequestChildMarkupsBuilder {
    pub fn crypto_swaps(mut self, value: UpdateFeesRequestChildMarkupsCryptoSwaps) -> Self {
        self.crypto_swaps = Some(value);
        self
    }

    pub fn deposits(
        mut self,
        value: HashMap<String, Option<UpdateFeesRequestChildMarkupsDepositsValue>>,
    ) -> Self {
        self.deposits = Some(value);
        self
    }

    pub fn payments(mut self, value: UpdateFeesRequestChildMarkupsPayments) -> Self {
        self.payments = Some(value);
        self
    }

    pub fn payouts(
        mut self,
        value: HashMap<String, Option<UpdateFeesRequestChildMarkupsPayoutsValue>>,
    ) -> Self {
        self.payouts = Some(value);
        self
    }

    pub fn transfers(mut self, value: UpdateFeesRequestChildMarkupsTransfers) -> Self {
        self.transfers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestChildMarkups`].
    pub fn build(self) -> Result<UpdateFeesRequestChildMarkups, BuildError> {
        Ok(UpdateFeesRequestChildMarkups {
            crypto_swaps: self.crypto_swaps,
            deposits: self.deposits,
            payments: self.payments,
            payouts: self.payouts,
            transfers: self.transfers,
        })
    }
}
