pub use crate::prelude::*;

/// A card transaction record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardTransactionLegacy {
    /// How the card was presented or authenticated for the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_method: Option<String>,
    /// Represents a unique identifier that is Base64 obfuscated. It is often used to refetch an object or as key for a cache. The ID type appears in a JSON response as a String; however, it is not intended to be human-readable. When expected as an input type, any string (such as `"VXNlci0xMA=="`) or integer (such as `4`) input value will be accepted as an ID.
    #[serde(default)]
    pub card_id: String,
    /// The cashback reward amount earned on this transaction, in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cashback_usd_amount: Option<f64>,
    /// The datetime the card transaction was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The ISO 4217 currency code for the transaction amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The issuer-provided reason the transaction was declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declined_reason: Option<String>,
    /// The unique identifier for the card transaction.
    #[serde(default)]
    pub id: String,
    /// Whether the transaction was made with a merchant outside the card's home country.
    #[serde(default)]
    pub international: bool,
    /// The transaction amount in the merchant's local currency before conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub local_amount: Option<f64>,
    /// A user-provided note attached to the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    /// The enriched or raw category label for the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_category: Option<String>,
    /// The four-digit ISO 18245 merchant category code (MCC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_category_code: Option<String>,
    /// A URL to the enriched merchant logo image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_icon_url: Option<String>,
    /// The enriched or raw name of the merchant where the purchase was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /// When the transaction was settled by the card network.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub posted_at: Option<DateTime<FixedOffset>>,
    /// The current lifecycle status of the transaction.
    pub status: CardIssuingTransactionStatus,
    /// The type of transaction.
    #[serde(default)]
    pub transaction_type: String,
    /// The transaction amount in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_amount: Option<f64>,
}

impl CardTransactionLegacy {
    pub fn builder() -> CardTransactionLegacyBuilder {
        <CardTransactionLegacyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardTransactionLegacyBuilder {
    authorization_method: Option<String>,
    card_id: Option<String>,
    cashback_usd_amount: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    declined_reason: Option<String>,
    id: Option<String>,
    international: Option<bool>,
    local_amount: Option<f64>,
    memo: Option<String>,
    merchant_category: Option<String>,
    merchant_category_code: Option<String>,
    merchant_icon_url: Option<String>,
    merchant_name: Option<String>,
    posted_at: Option<DateTime<FixedOffset>>,
    status: Option<CardIssuingTransactionStatus>,
    transaction_type: Option<String>,
    usd_amount: Option<f64>,
}

impl CardTransactionLegacyBuilder {
    pub fn authorization_method(mut self, value: impl Into<String>) -> Self {
        self.authorization_method = Some(value.into());
        self
    }

    pub fn card_id(mut self, value: impl Into<String>) -> Self {
        self.card_id = Some(value.into());
        self
    }

    pub fn cashback_usd_amount(mut self, value: f64) -> Self {
        self.cashback_usd_amount = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
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

    pub fn international(mut self, value: bool) -> Self {
        self.international = Some(value);
        self
    }

    pub fn local_amount(mut self, value: f64) -> Self {
        self.local_amount = Some(value);
        self
    }

    pub fn memo(mut self, value: impl Into<String>) -> Self {
        self.memo = Some(value.into());
        self
    }

    pub fn merchant_category(mut self, value: impl Into<String>) -> Self {
        self.merchant_category = Some(value.into());
        self
    }

    pub fn merchant_category_code(mut self, value: impl Into<String>) -> Self {
        self.merchant_category_code = Some(value.into());
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

    pub fn posted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.posted_at = Some(value);
        self
    }

    pub fn status(mut self, value: CardIssuingTransactionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn transaction_type(mut self, value: impl Into<String>) -> Self {
        self.transaction_type = Some(value.into());
        self
    }

    pub fn usd_amount(mut self, value: f64) -> Self {
        self.usd_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CardTransactionLegacy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card_id`](CardTransactionLegacyBuilder::card_id)
    /// - [`created_at`](CardTransactionLegacyBuilder::created_at)
    /// - [`id`](CardTransactionLegacyBuilder::id)
    /// - [`international`](CardTransactionLegacyBuilder::international)
    /// - [`status`](CardTransactionLegacyBuilder::status)
    /// - [`transaction_type`](CardTransactionLegacyBuilder::transaction_type)
    pub fn build(self) -> Result<CardTransactionLegacy, BuildError> {
        Ok(CardTransactionLegacy {
            authorization_method: self.authorization_method,
            card_id: self
                .card_id
                .ok_or_else(|| BuildError::missing_field("card_id"))?,
            cashback_usd_amount: self.cashback_usd_amount,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self.currency,
            declined_reason: self.declined_reason,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            international: self
                .international
                .ok_or_else(|| BuildError::missing_field("international"))?,
            local_amount: self.local_amount,
            memo: self.memo,
            merchant_category: self.merchant_category,
            merchant_category_code: self.merchant_category_code,
            merchant_icon_url: self.merchant_icon_url,
            merchant_name: self.merchant_name,
            posted_at: self.posted_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            transaction_type: self
                .transaction_type
                .ok_or_else(|| BuildError::missing_field("transaction_type"))?,
            usd_amount: self.usd_amount,
        })
    }
}
