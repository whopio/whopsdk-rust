pub use crate::prelude::*;

/// A ledger account represents a financial account on Whop that can hold many balances.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LedgerAccount {
    /// The balances associated with the account.
    #[serde(default)]
    pub balances: Vec<LedgerAccountBalancesItem>,
    /// The unique identifier for the ledger account.
    #[serde(default)]
    pub id: String,
    /// The type of ledger account.
    pub ledger_type: LedgerTypes,
    /// The owner of the ledger account.
    pub owner: LedgerAccountOwner,
    /// The status of payments approval for the ledger account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments_approval_status: Option<PaymentsApprovalStatuses>,
    /// The payout account associated with the LedgerAccount, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_account_details: Option<LedgerAccountPayoutAccountDetails>,
    /// The settlement batch most recently posted to this account's available balance, at midnight UTC. Every payment settling in that batch carries the same `settlement_time_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub settlement_time_at: Option<DateTime<FixedOffset>>,
    /// The fee for transfers, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub transfer_fee: Option<f64>,
    /// The balance cache associated with the account by currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury_balance: Option<LedgerAccountTreasuryBalance>,
}

impl LedgerAccount {
    pub fn builder() -> LedgerAccountBuilder {
        <LedgerAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerAccountBuilder {
    balances: Option<Vec<LedgerAccountBalancesItem>>,
    id: Option<String>,
    ledger_type: Option<LedgerTypes>,
    owner: Option<LedgerAccountOwner>,
    payments_approval_status: Option<PaymentsApprovalStatuses>,
    payout_account_details: Option<LedgerAccountPayoutAccountDetails>,
    settlement_time_at: Option<DateTime<FixedOffset>>,
    transfer_fee: Option<f64>,
    treasury_balance: Option<LedgerAccountTreasuryBalance>,
}

impl LedgerAccountBuilder {
    pub fn balances(mut self, value: Vec<LedgerAccountBalancesItem>) -> Self {
        self.balances = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn ledger_type(mut self, value: LedgerTypes) -> Self {
        self.ledger_type = Some(value);
        self
    }

    pub fn owner(mut self, value: LedgerAccountOwner) -> Self {
        self.owner = Some(value);
        self
    }

    pub fn payments_approval_status(mut self, value: PaymentsApprovalStatuses) -> Self {
        self.payments_approval_status = Some(value);
        self
    }

    pub fn payout_account_details(mut self, value: LedgerAccountPayoutAccountDetails) -> Self {
        self.payout_account_details = Some(value);
        self
    }

    pub fn settlement_time_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.settlement_time_at = Some(value);
        self
    }

    pub fn transfer_fee(mut self, value: f64) -> Self {
        self.transfer_fee = Some(value);
        self
    }

    pub fn treasury_balance(mut self, value: LedgerAccountTreasuryBalance) -> Self {
        self.treasury_balance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LedgerAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balances`](LedgerAccountBuilder::balances)
    /// - [`id`](LedgerAccountBuilder::id)
    /// - [`ledger_type`](LedgerAccountBuilder::ledger_type)
    /// - [`owner`](LedgerAccountBuilder::owner)
    pub fn build(self) -> Result<LedgerAccount, BuildError> {
        Ok(LedgerAccount {
            balances: self
                .balances
                .ok_or_else(|| BuildError::missing_field("balances"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            ledger_type: self
                .ledger_type
                .ok_or_else(|| BuildError::missing_field("ledger_type"))?,
            owner: self
                .owner
                .ok_or_else(|| BuildError::missing_field("owner"))?,
            payments_approval_status: self.payments_approval_status,
            payout_account_details: self.payout_account_details,
            settlement_time_at: self.settlement_time_at,
            transfer_fee: self.transfer_fee,
            treasury_balance: self.treasury_balance,
        })
    }
}
