pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputesListQueryRequest {
    /// Only disputes filed against this account (`biz_` tag). Omit it to cover every account you can read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The number of disputes to return (default 20, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns disputes after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of disputes to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns disputes before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The field to sort disputes by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListDisputesRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListDisputesRequestDirection>,
    /// Only disputes in these statuses. Repeat the parameter to pass several — one paginated list covers all of them. Covers both chargebacks and inquiries at each stage. A `needs_response` dispute whose evidence deadline has passed reports and filters as `under_review` instead.
    #[serde(default)]
    pub status: Vec<Option<ListDisputesRequestStatusItem>>,
    /// Only disputes in this three-letter ISO currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Only disputes opened before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only disputes opened after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
}

impl DisputesListQueryRequest {
    pub fn builder() -> DisputesListQueryRequestBuilder {
        <DisputesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputesListQueryRequestBuilder {
    account_id: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListDisputesRequestOrder>,
    direction: Option<ListDisputesRequestDirection>,
    status: Option<Vec<Option<ListDisputesRequestStatusItem>>>,
    currency: Option<String>,
    created_before: Option<String>,
    created_after: Option<String>,
}

impl DisputesListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
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

    pub fn order(mut self, value: ListDisputesRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListDisputesRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn status(mut self, value: Vec<Option<ListDisputesRequestStatusItem>>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
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

    /// Consumes the builder and constructs a [`DisputesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DisputesListQueryRequestBuilder::status)
    pub fn build(self) -> Result<DisputesListQueryRequest, BuildError> {
        Ok(DisputesListQueryRequest {
            account_id: self.account_id,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            currency: self.currency,
            created_before: self.created_before,
            created_after: self.created_after,
        })
    }
}
