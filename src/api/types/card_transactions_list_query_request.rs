pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CardTransactionsListQueryRequest {
    /// The account whose card transactions to list, prefixed `biz_`. Defaults to the credential's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Return only these card transactions, each prefixed `citx_`. Repeat the parameter, or pass one comma-separated value.
    #[serde(default)]
    pub transaction_ids: Vec<Option<String>>,
    /// Return only transactions charged to these cards, each prefixed `icrd_`.
    #[serde(default)]
    pub card_id: Vec<Option<String>>,
    /// Return only transactions on cards assigned to these users, each prefixed `user_`.
    #[serde(default)]
    pub cardholder_id: Vec<Option<String>>,
    /// Return only transactions with this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListCardTransactionsRequestStatus>,
    /// Return only transactions authorized at or after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Return only transactions authorized at or before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// The field to sort by. Defaults to `created_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListCardTransactionsRequestOrder>,
    /// The sort direction. Defaults to `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListCardTransactionsRequestDirection>,
    /// The number of card transactions to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns card transactions after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of card transactions to return, counting back from the end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns card transactions before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl CardTransactionsListQueryRequest {
    pub fn builder() -> CardTransactionsListQueryRequestBuilder {
        <CardTransactionsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardTransactionsListQueryRequestBuilder {
    account_id: Option<String>,
    transaction_ids: Option<Vec<Option<String>>>,
    card_id: Option<Vec<Option<String>>>,
    cardholder_id: Option<Vec<Option<String>>>,
    status: Option<ListCardTransactionsRequestStatus>,
    created_after: Option<String>,
    created_before: Option<String>,
    order: Option<ListCardTransactionsRequestOrder>,
    direction: Option<ListCardTransactionsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl CardTransactionsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn transaction_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.transaction_ids = Some(value);
        self
    }

    pub fn card_id(mut self, value: Vec<Option<String>>) -> Self {
        self.card_id = Some(value);
        self
    }

    pub fn cardholder_id(mut self, value: Vec<Option<String>>) -> Self {
        self.cardholder_id = Some(value);
        self
    }

    pub fn status(mut self, value: ListCardTransactionsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListCardTransactionsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListCardTransactionsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CardTransactionsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_ids`](CardTransactionsListQueryRequestBuilder::transaction_ids)
    /// - [`card_id`](CardTransactionsListQueryRequestBuilder::card_id)
    /// - [`cardholder_id`](CardTransactionsListQueryRequestBuilder::cardholder_id)
    pub fn build(self) -> Result<CardTransactionsListQueryRequest, BuildError> {
        Ok(CardTransactionsListQueryRequest {
            account_id: self.account_id,
            transaction_ids: self
                .transaction_ids
                .ok_or_else(|| BuildError::missing_field("transaction_ids"))?,
            card_id: self
                .card_id
                .ok_or_else(|| BuildError::missing_field("card_id"))?,
            cardholder_id: self
                .cardholder_id
                .ok_or_else(|| BuildError::missing_field("cardholder_id"))?,
            status: self.status,
            created_after: self.created_after,
            created_before: self.created_before,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
