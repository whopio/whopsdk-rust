pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateTransfersRequest {
    /// The amount to move, in the transfer currency. For example 25.00.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// Currency, such as `usd`. Required for ledger transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The recipient. Required for ledger and wallet_send (a user_/biz_/ldgr_ ID, or — for sends — an email). Omit for claim_link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    /// claim_link only. Link expiry as an ISO 8601 timestamp. Defaults to 24 hours from creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Ledger transfers and wallet sends. A unique key that makes retries safe. Retrying with the same key returns the original transfer, or attaches to the original wallet send, instead of moving money twice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotence_key: Option<String>,
    /// Ledger transfers only. Custom key-value pairs attached to the transfer. Max 50 keys, 100 chars per key, 500 chars per string value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Ledger transfers only. A short note describing the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The account sending the funds. A user ID (user_xxx), account ID (biz_xxx), or ledger account ID (ldgr_xxx).
    #[serde(default)]
    pub origin_id: String,
    /// claim_link only. How many different users can claim the link. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemable_count: Option<i64>,
    /// The kind of money movement, which decides what comes back. Defaults to ledger. `ledger` moves credit between two Whop balances and returns a `transfer`; `wallet_send` sends USDT from the origin account's Ethereum wallet and returns a `send`; `claim_link` funds a shareable link anyone with the URL can redeem and returns a `claim_link`. A `ledger` transfer from a stablecoin-rails account settles on-chain when covered, and still returns a `transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CreateTransfersRequestType>,
}

impl CreateTransfersRequest {
    pub fn builder() -> CreateTransfersRequestBuilder {
        <CreateTransfersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTransfersRequestBuilder {
    amount: Option<f64>,
    currency: Option<String>,
    destination_id: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    idempotence_key: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    notes: Option<String>,
    origin_id: Option<String>,
    redeemable_count: Option<i64>,
    r#type: Option<CreateTransfersRequestType>,
}

impl CreateTransfersRequestBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn destination_id(mut self, value: impl Into<String>) -> Self {
        self.destination_id = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn idempotence_key(mut self, value: impl Into<String>) -> Self {
        self.idempotence_key = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn origin_id(mut self, value: impl Into<String>) -> Self {
        self.origin_id = Some(value.into());
        self
    }

    pub fn redeemable_count(mut self, value: i64) -> Self {
        self.redeemable_count = Some(value);
        self
    }

    pub fn r#type(mut self, value: CreateTransfersRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTransfersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreateTransfersRequestBuilder::amount)
    /// - [`origin_id`](CreateTransfersRequestBuilder::origin_id)
    pub fn build(self) -> Result<CreateTransfersRequest, BuildError> {
        Ok(CreateTransfersRequest {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self.currency,
            destination_id: self.destination_id,
            expires_at: self.expires_at,
            idempotence_key: self.idempotence_key,
            metadata: self.metadata,
            notes: self.notes,
            origin_id: self
                .origin_id
                .ok_or_else(|| BuildError::missing_field("origin_id"))?,
            redeemable_count: self.redeemable_count,
            r#type: self.r#type,
        })
    }
}
