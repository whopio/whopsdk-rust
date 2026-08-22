pub use crate::prelude::*;

/// Query parameters for deliveriesWebhook
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeliveriesWebhookQueryRequest {
    /// Returns the elements in the list that come after the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Returns the elements in the list that come before the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Returns the first _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Returns the last _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
}

impl DeliveriesWebhookQueryRequest {
    pub fn builder() -> DeliveriesWebhookQueryRequestBuilder {
        <DeliveriesWebhookQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveriesWebhookQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
}

impl DeliveriesWebhookQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeliveriesWebhookQueryRequest`].
    pub fn build(self) -> Result<DeliveriesWebhookQueryRequest, BuildError> {
        Ok(DeliveriesWebhookQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
        })
    }
}
