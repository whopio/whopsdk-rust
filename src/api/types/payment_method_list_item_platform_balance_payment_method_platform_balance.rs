pub use crate::prelude::*;

/// What is available to spend, and whether the account may spend it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalance {
    /// The account whose wallet this is. Null for the buyer's own personal wallet. A buyer sees an account's balance here when they hold permission to spend it, so a list can hold several — their own and one per account they are on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceAccount>,
    /// Available amount per currency. Read from the balance cache, so it is indicative — the charge revalidates against settled funds and may still refuse.
    #[serde(default)]
    pub balances: Vec<PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceBalancesItem>,
    /// Whether this balance can pay right now, which here means only whether it holds funds — an account blocked from spending is not listed at all. A zero balance is still returned so a client can show it as an option the buyer could top up.
    #[serde(default)]
    pub spendable: bool,
}

impl PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalance {
    pub fn builder() -> PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceBuilder {
        <PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceBuilder {
    account: Option<PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceAccount>,
    balances:
        Option<Vec<PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceBalancesItem>>,
    spendable: Option<bool>,
}

impl PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceBuilder {
    pub fn account(
        mut self,
        value: PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceAccount,
    ) -> Self {
        self.account = Some(value);
        self
    }

    pub fn balances(
        mut self,
        value: Vec<PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceBalancesItem>,
    ) -> Self {
        self.balances = Some(value);
        self
    }

    pub fn spendable(mut self, value: bool) -> Self {
        self.spendable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balances`](PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceBuilder::balances)
    /// - [`spendable`](PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceBuilder::spendable)
    pub fn build(
        self,
    ) -> Result<PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalance, BuildError> {
        Ok(
            PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalance {
                account: self.account,
                balances: self
                    .balances
                    .ok_or_else(|| BuildError::missing_field("balances"))?,
                spendable: self
                    .spendable
                    .ok_or_else(|| BuildError::missing_field("spendable"))?,
            },
        )
    }
}
