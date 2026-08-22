pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardTransaction {
    /// The card this transaction was charged to, prefixed `icrd_`.
    #[serde(default)]
    pub card_id: String,
    /// The user the card is assigned to, prefixed `user_`. Null when the card has no assigned cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_id: Option<String>,
    /// Cashback earned on this transaction as a USD amount. Zero for declined or ineligible transactions, and null when cashback has not been computed yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cashback_usd_amount: Option<f64>,
    /// When the transaction was authorized, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// ISO 4217 currency code the merchant charged in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Why the transaction was declined. Null unless `status` is `declined`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declined_reason: Option<String>,
    /// Card transaction ID, prefixed `citx_`.
    #[serde(default)]
    pub id: String,
    /// True when the merchant is outside the card's home country.
    #[serde(default)]
    pub international: bool,
    /// Amount the merchant charged in their own currency. Pair with `currency`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub local_amount: Option<f64>,
    /// Merchant category label, enriched where available and otherwise as the card network reported it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_category: Option<String>,
    /// Four-digit ISO 18245 merchant category code (MCC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_category_code: Option<String>,
    /// URL of the enriched merchant logo. Null when no logo was matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_icon_url: Option<String>,
    /// Merchant name, enriched where available and otherwise as the card network reported it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /// When the card network settled the transaction, as an ISO 8601 timestamp. Null until it settles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<String>,
    /// Current status of the transaction.
    pub status: CardTransactionStatus,
    /// The kind of card transaction. Always `spend` today.
    pub transaction_type: CardTransactionTransactionType,
    /// Amount charged in USD. Negative when the merchant refunded the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_amount: Option<f64>,
}

impl CardTransaction {
    pub fn builder() -> CardTransactionBuilder {
        <CardTransactionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardTransactionBuilder {
    card_id: Option<String>,
    cardholder_id: Option<String>,
    cashback_usd_amount: Option<f64>,
    created_at: Option<String>,
    currency: Option<String>,
    declined_reason: Option<String>,
    id: Option<String>,
    international: Option<bool>,
    local_amount: Option<f64>,
    merchant_category: Option<String>,
    merchant_category_code: Option<String>,
    merchant_icon_url: Option<String>,
    merchant_name: Option<String>,
    posted_at: Option<String>,
    status: Option<CardTransactionStatus>,
    transaction_type: Option<CardTransactionTransactionType>,
    usd_amount: Option<f64>,
}

impl CardTransactionBuilder {
    pub fn card_id(mut self, value: impl Into<String>) -> Self {
        self.card_id = Some(value.into());
        self
    }

    pub fn cardholder_id(mut self, value: impl Into<String>) -> Self {
        self.cardholder_id = Some(value.into());
        self
    }

    pub fn cashback_usd_amount(mut self, value: f64) -> Self {
        self.cashback_usd_amount = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
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

    pub fn posted_at(mut self, value: impl Into<String>) -> Self {
        self.posted_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: CardTransactionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn transaction_type(mut self, value: CardTransactionTransactionType) -> Self {
        self.transaction_type = Some(value);
        self
    }

    pub fn usd_amount(mut self, value: f64) -> Self {
        self.usd_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CardTransaction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card_id`](CardTransactionBuilder::card_id)
    /// - [`created_at`](CardTransactionBuilder::created_at)
    /// - [`id`](CardTransactionBuilder::id)
    /// - [`international`](CardTransactionBuilder::international)
    /// - [`status`](CardTransactionBuilder::status)
    /// - [`transaction_type`](CardTransactionBuilder::transaction_type)
    pub fn build(self) -> Result<CardTransaction, BuildError> {
        Ok(CardTransaction {
            card_id: self
                .card_id
                .ok_or_else(|| BuildError::missing_field("card_id"))?,
            cardholder_id: self.cardholder_id,
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
