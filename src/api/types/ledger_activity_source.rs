pub use crate::prelude::*;

/// Source of this ledger activity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LedgerActivitySource {
    /// Withdrawal amount as a decimal number in the destination currency (withdrawal sources only; requires payout:withdrawal:read).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_float: Option<f64>,
    /// Card brand used by the payment source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    /// Chain the deposit landed on, for example plasma (onchain_transaction sources only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// Public claim URL for the airdrop link (airdrop_link sources only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_url: Option<String>,
    /// Withdrawal creation time as an ISO 8601 timestamp (withdrawal sources only; requires payout:withdrawal:read).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Estimated arrival as an ISO 8601 timestamp (withdrawal sources only; requires payout:withdrawal:read).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_arrival: Option<DateTime<FixedOffset>>,
    /// Amount converted out of from_currency as a decimal string (swap sources only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_amount: Option<String>,
    /// Lowercase currency code converted from (swap sources only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_currency: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub object: String,
    /// Name of the entity processing the payout (withdrawal sources only; requires payout:withdrawal:read).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<String>,
    /// Total charged by the payment source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_amount: Option<Money>,
    /// Payment method used by the payment source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    /// Processor used by the payment source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_processor: Option<String>,
    /// Payout destination display info (withdrawal sources only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_destination: Option<LedgerActivitySourcePayoutDestination>,
    /// Saved payout destination nickname (withdrawal sources only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_token_nickname: Option<String>,
    /// Why the activity happened. On transfer sources this is the transfer reason, for example pool_top_up or bounty_return. On withdrawal sources it explains why the withdrawal was canceled, denied, or failed (requires payout:withdrawal:read); null while the withdrawal is progressing normally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Sender wallet address or onramp provider identifier (onchain_transaction sources only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_address: Option<String>,
    /// Lifecycle status. On withdrawal sources this is the withdrawal status (requires payout:withdrawal:read); on airdrop_link sources it is the claim-link status (ungated); on payment and top-up sources it is the friendly payment status such as succeeded/pending/failed (ungated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Amount received in to_currency as a decimal string (swap sources only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_amount: Option<String>,
    /// Lowercase currency code converted to (swap sources only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_currency: Option<String>,
    /// On-chain transaction hash (onchain_transaction and swap sources only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl LedgerActivitySource {
    pub fn builder() -> LedgerActivitySourceBuilder {
        <LedgerActivitySourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivitySourceBuilder {
    amount_float: Option<f64>,
    card_brand: Option<String>,
    chain: Option<String>,
    claim_url: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    estimated_arrival: Option<DateTime<FixedOffset>>,
    from_amount: Option<String>,
    from_currency: Option<String>,
    id: Option<String>,
    object: Option<String>,
    payer_name: Option<String>,
    payment_amount: Option<Money>,
    payment_method_type: Option<String>,
    payment_processor: Option<String>,
    payout_destination: Option<LedgerActivitySourcePayoutDestination>,
    payout_token_nickname: Option<String>,
    reason: Option<String>,
    sender_address: Option<String>,
    status: Option<String>,
    to_amount: Option<String>,
    to_currency: Option<String>,
    tx_hash: Option<String>,
}

impl LedgerActivitySourceBuilder {
    pub fn amount_float(mut self, value: f64) -> Self {
        self.amount_float = Some(value);
        self
    }

    pub fn card_brand(mut self, value: impl Into<String>) -> Self {
        self.card_brand = Some(value.into());
        self
    }

    pub fn chain(mut self, value: impl Into<String>) -> Self {
        self.chain = Some(value.into());
        self
    }

    pub fn claim_url(mut self, value: impl Into<String>) -> Self {
        self.claim_url = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn estimated_arrival(mut self, value: DateTime<FixedOffset>) -> Self {
        self.estimated_arrival = Some(value);
        self
    }

    pub fn from_amount(mut self, value: impl Into<String>) -> Self {
        self.from_amount = Some(value.into());
        self
    }

    pub fn from_currency(mut self, value: impl Into<String>) -> Self {
        self.from_currency = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn payer_name(mut self, value: impl Into<String>) -> Self {
        self.payer_name = Some(value.into());
        self
    }

    pub fn payment_amount(mut self, value: Money) -> Self {
        self.payment_amount = Some(value);
        self
    }

    pub fn payment_method_type(mut self, value: impl Into<String>) -> Self {
        self.payment_method_type = Some(value.into());
        self
    }

    pub fn payment_processor(mut self, value: impl Into<String>) -> Self {
        self.payment_processor = Some(value.into());
        self
    }

    pub fn payout_destination(mut self, value: LedgerActivitySourcePayoutDestination) -> Self {
        self.payout_destination = Some(value);
        self
    }

    pub fn payout_token_nickname(mut self, value: impl Into<String>) -> Self {
        self.payout_token_nickname = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn sender_address(mut self, value: impl Into<String>) -> Self {
        self.sender_address = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn to_amount(mut self, value: impl Into<String>) -> Self {
        self.to_amount = Some(value.into());
        self
    }

    pub fn to_currency(mut self, value: impl Into<String>) -> Self {
        self.to_currency = Some(value.into());
        self
    }

    pub fn tx_hash(mut self, value: impl Into<String>) -> Self {
        self.tx_hash = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivitySource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivitySourceBuilder::id)
    /// - [`object`](LedgerActivitySourceBuilder::object)
    pub fn build(self) -> Result<LedgerActivitySource, BuildError> {
        Ok(LedgerActivitySource {
            amount_float: self.amount_float,
            card_brand: self.card_brand,
            chain: self.chain,
            claim_url: self.claim_url,
            created_at: self.created_at,
            estimated_arrival: self.estimated_arrival,
            from_amount: self.from_amount,
            from_currency: self.from_currency,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            payer_name: self.payer_name,
            payment_amount: self.payment_amount,
            payment_method_type: self.payment_method_type,
            payment_processor: self.payment_processor,
            payout_destination: self.payout_destination,
            payout_token_nickname: self.payout_token_nickname,
            reason: self.reason,
            sender_address: self.sender_address,
            status: self.status,
            to_amount: self.to_amount,
            to_currency: self.to_currency,
            tx_hash: self.tx_hash,
            extra: Default::default(),
        })
    }
}
