pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShipmentsListQueryRequest {
    /// The account to list shipments for. Defaults to the acting account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Filter to shipments with this delivery status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListShipmentsRequestStatus>,
    /// Only shipments fulfilling these payments, each prefixed `pay_`. Repeat the parameter to pass several, up to 100 per request — one paginated list covers all of them.
    #[serde(default)]
    pub payment_id: Vec<Option<String>>,
    /// Return shipments created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Return shipments created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// The field to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListShipmentsRequestOrder>,
    /// The sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListShipmentsRequestDirection>,
    /// The number of shipments to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns shipments after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of shipments to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns shipments before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl ShipmentsListQueryRequest {
    pub fn builder() -> ShipmentsListQueryRequestBuilder {
        <ShipmentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShipmentsListQueryRequestBuilder {
    account_id: Option<String>,
    status: Option<ListShipmentsRequestStatus>,
    payment_id: Option<Vec<Option<String>>>,
    created_before: Option<String>,
    created_after: Option<String>,
    order: Option<ListShipmentsRequestOrder>,
    direction: Option<ListShipmentsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl ShipmentsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListShipmentsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn payment_id(mut self, value: Vec<Option<String>>) -> Self {
        self.payment_id = Some(value);
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListShipmentsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListShipmentsRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`ShipmentsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_id`](ShipmentsListQueryRequestBuilder::payment_id)
    pub fn build(self) -> Result<ShipmentsListQueryRequest, BuildError> {
        Ok(ShipmentsListQueryRequest {
            account_id: self.account_id,
            status: self.status,
            payment_id: self
                .payment_id
                .ok_or_else(|| BuildError::missing_field("payment_id"))?,
            created_before: self.created_before,
            created_after: self.created_after,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
