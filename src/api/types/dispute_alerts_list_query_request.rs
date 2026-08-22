pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeAlertsListQueryRequest {
    /// Only alerts on this account's payments (`biz_` tag). Omit it to cover every account you can read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only alerts on this payment (`pay_` tag). A payment can carry several.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// Only alerts of this kind. `early_fraud_warning` for issuer fraud reports, `dispute_alert` for pre-dispute notices, `rapid_dispute_resolution` for Visa RDR cases the network already closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ListDisputeAlertsRequestType>,
    /// The number of alerts to return (default 20, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns alerts after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of alerts to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns alerts before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The field to sort alerts by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListDisputeAlertsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListDisputeAlertsRequestDirection>,
    /// Only alerts Whop received before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only alerts Whop received after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
}

impl DisputeAlertsListQueryRequest {
    pub fn builder() -> DisputeAlertsListQueryRequestBuilder {
        <DisputeAlertsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeAlertsListQueryRequestBuilder {
    account_id: Option<String>,
    payment_id: Option<String>,
    r#type: Option<ListDisputeAlertsRequestType>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListDisputeAlertsRequestOrder>,
    direction: Option<ListDisputeAlertsRequestDirection>,
    created_before: Option<String>,
    created_after: Option<String>,
}

impl DisputeAlertsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ListDisputeAlertsRequestType) -> Self {
        self.r#type = Some(value);
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

    pub fn order(mut self, value: ListDisputeAlertsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListDisputeAlertsRequestDirection) -> Self {
        self.direction = Some(value);
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

    /// Consumes the builder and constructs a [`DisputeAlertsListQueryRequest`].
    pub fn build(self) -> Result<DisputeAlertsListQueryRequest, BuildError> {
        Ok(DisputeAlertsListQueryRequest {
            account_id: self.account_id,
            payment_id: self.payment_id,
            r#type: self.r#type,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
            created_before: self.created_before,
            created_after: self.created_after,
        })
    }
}
