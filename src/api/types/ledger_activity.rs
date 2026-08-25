pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LedgerActivity {
    /// The viewer account that owns this row's ledger. Present only when the response aggregates owned accounts (include_owned_accounts=true); omitted otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<LedgerActivityAccount>,
    /// Signed amount in the currency's smallest precision units.
    #[serde(default)]
    pub amount: String,
    /// ISO 8601 timestamp these funds became (or are scheduled to become) withdrawable: the posted time for already-settled funds, or 00:00:00 UTC on the scheduled release date for pending funds. Present only on inflows entering the balance (payments, top-ups, incoming transfers/affiliate); null on withdrawals, refunds, disputes and on-chain rows. The available_after/before filters window on its UTC settlement date.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub available_at: Option<DateTime<FixedOffset>>,
    /// When the activity record was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Currency for this ledger activity.
    #[serde(default)]
    pub currency: LedgerActivityCurrency,
    /// Ledger activity ID.
    #[serde(default)]
    pub id: String,
    /// The ledger account (a ldgr_ identifier) this row belongs to. Present only when the response aggregates owned accounts (include_owned_accounts=true); omitted otherwise. Pair it with `account` to scope drawers and dashboard links to the owning business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_account_id: Option<String>,
    /// The ledger line category this activity was posted under.
    pub line_type: LedgerActivityLineType,
    pub object: LedgerActivityObject,
    /// Payment related to this ledger activity. Included when rich resource hydration is enabled and the movement is tied to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<LedgerActivityPayment>,
    /// When the activity posted to the ledger.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub posted_at: DateTime<FixedOffset>,
    /// Resource associated with this ledger activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<LedgerActivityResource>,
    /// Source of this ledger activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<LedgerActivitySource>,
}

impl LedgerActivity {
    pub fn builder() -> LedgerActivityBuilder {
        <LedgerActivityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityBuilder {
    account: Option<LedgerActivityAccount>,
    amount: Option<String>,
    available_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<LedgerActivityCurrency>,
    id: Option<String>,
    ledger_account_id: Option<String>,
    line_type: Option<LedgerActivityLineType>,
    object: Option<LedgerActivityObject>,
    payment: Option<LedgerActivityPayment>,
    posted_at: Option<DateTime<FixedOffset>>,
    resource: Option<LedgerActivityResource>,
    source: Option<LedgerActivitySource>,
}

impl LedgerActivityBuilder {
    pub fn account(mut self, value: LedgerActivityAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn available_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.available_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: LedgerActivityCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn ledger_account_id(mut self, value: impl Into<String>) -> Self {
        self.ledger_account_id = Some(value.into());
        self
    }

    pub fn line_type(mut self, value: LedgerActivityLineType) -> Self {
        self.line_type = Some(value);
        self
    }

    pub fn object(mut self, value: LedgerActivityObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn payment(mut self, value: LedgerActivityPayment) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn posted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.posted_at = Some(value);
        self
    }

    pub fn resource(mut self, value: LedgerActivityResource) -> Self {
        self.resource = Some(value);
        self
    }

    pub fn source(mut self, value: LedgerActivitySource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](LedgerActivityBuilder::amount)
    /// - [`currency`](LedgerActivityBuilder::currency)
    /// - [`id`](LedgerActivityBuilder::id)
    /// - [`line_type`](LedgerActivityBuilder::line_type)
    /// - [`object`](LedgerActivityBuilder::object)
    /// - [`posted_at`](LedgerActivityBuilder::posted_at)
    pub fn build(self) -> Result<LedgerActivity, BuildError> {
        Ok(LedgerActivity {
            account: self.account,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            available_at: self.available_at,
            created_at: self.created_at,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            ledger_account_id: self.ledger_account_id,
            line_type: self
                .line_type
                .ok_or_else(|| BuildError::missing_field("line_type"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            payment: self.payment,
            posted_at: self
                .posted_at
                .ok_or_else(|| BuildError::missing_field("posted_at"))?,
            resource: self.resource,
            source: self.source,
        })
    }
}
