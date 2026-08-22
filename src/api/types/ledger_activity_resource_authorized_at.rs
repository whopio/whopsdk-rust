pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LedgerActivityResourceAuthorizedAt {
    /// ISO 8601 timestamp the transaction was authorized.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub authorized_at: Option<DateTime<FixedOffset>>,
    /// Identifier of the card that the transaction was charged to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// Cashback earned on this transaction as a USD decimal string. Zero for declined or ineligible transactions; null when cashback has not been computed yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashback_usd: Option<String>,
    /// Reason the transaction was declined (when status is declined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declined_reason: Option<String>,
    /// Card transaction ID.
    #[serde(default)]
    pub id: String,
    /// Amount the merchant charged in their local currency, as a decimal string. Pair with local_currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amount: Option<String>,
    /// ISO 4217 currency code of the merchant-charged amount in local_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_currency: Option<String>,
    /// Merchant category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_category: Option<String>,
    /// Merchant icon URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_icon_url: Option<String>,
    /// Merchant display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    pub object: LedgerActivityResourceAuthorizedAtObject,
    /// ISO 8601 timestamp the transaction was settled by the card network.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub posted_at: Option<DateTime<FixedOffset>>,
    /// Current card transaction status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The processor-settled USD amount as a decimal string. The ledger's USDT leg is posted 1:1 from this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd_amount: Option<String>,
}

impl LedgerActivityResourceAuthorizedAt {
    pub fn builder() -> LedgerActivityResourceAuthorizedAtBuilder {
        <LedgerActivityResourceAuthorizedAtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityResourceAuthorizedAtBuilder {
    authorized_at: Option<DateTime<FixedOffset>>,
    card_id: Option<String>,
    cashback_usd: Option<String>,
    declined_reason: Option<String>,
    id: Option<String>,
    local_amount: Option<String>,
    local_currency: Option<String>,
    merchant_category: Option<String>,
    merchant_icon_url: Option<String>,
    merchant_name: Option<String>,
    object: Option<LedgerActivityResourceAuthorizedAtObject>,
    posted_at: Option<DateTime<FixedOffset>>,
    status: Option<String>,
    usd_amount: Option<String>,
}

impl LedgerActivityResourceAuthorizedAtBuilder {
    pub fn authorized_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.authorized_at = Some(value);
        self
    }

    pub fn card_id(mut self, value: impl Into<String>) -> Self {
        self.card_id = Some(value.into());
        self
    }

    pub fn cashback_usd(mut self, value: impl Into<String>) -> Self {
        self.cashback_usd = Some(value.into());
        self
    }

    pub fn declined_reason(mut self, value: impl Into<String>) -> Self {
        self.declined_reason = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn local_amount(mut self, value: impl Into<String>) -> Self {
        self.local_amount = Some(value.into());
        self
    }

    pub fn local_currency(mut self, value: impl Into<String>) -> Self {
        self.local_currency = Some(value.into());
        self
    }

    pub fn merchant_category(mut self, value: impl Into<String>) -> Self {
        self.merchant_category = Some(value.into());
        self
    }

    pub fn merchant_icon_url(mut self, value: impl Into<String>) -> Self {
        self.merchant_icon_url = Some(value.into());
        self
    }

    pub fn merchant_name(mut self, value: impl Into<String>) -> Self {
        self.merchant_name = Some(value.into());
        self
    }

    pub fn object(mut self, value: LedgerActivityResourceAuthorizedAtObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn posted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.posted_at = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn usd_amount(mut self, value: impl Into<String>) -> Self {
        self.usd_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivityResourceAuthorizedAt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivityResourceAuthorizedAtBuilder::id)
    /// - [`object`](LedgerActivityResourceAuthorizedAtBuilder::object)
    pub fn build(self) -> Result<LedgerActivityResourceAuthorizedAt, BuildError> {
        Ok(LedgerActivityResourceAuthorizedAt {
            authorized_at: self.authorized_at,
            card_id: self.card_id,
            cashback_usd: self.cashback_usd,
            declined_reason: self.declined_reason,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            local_amount: self.local_amount,
            local_currency: self.local_currency,
            merchant_category: self.merchant_category,
            merchant_icon_url: self.merchant_icon_url,
            merchant_name: self.merchant_name,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            posted_at: self.posted_at,
            status: self.status,
            usd_amount: self.usd_amount,
        })
    }
}
