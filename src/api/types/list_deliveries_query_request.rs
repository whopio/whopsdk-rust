pub use crate::prelude::*;

/// Query parameters for listDeliveries
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDeliveriesQueryRequest {
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Only deliveries sent at or after this ISO 8601 timestamp. Deliveries are retained for 30 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub sent_after: Option<DateTime<FixedOffset>>,
    /// Only deliveries sent at or before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub sent_before: Option<DateTime<FixedOffset>>,
    /// Only deliveries of these event types. Repeat the parameter to pass several.
    #[serde(default)]
    pub events: Vec<Option<ListDeliveriesWebhooksRequestEventsItem>>,
    /// Only deliveries with this outcome: `success` for a 2xx response, `error` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListDeliveriesWebhooksRequestStatus>,
    /// Only deliveries triggered by this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl ListDeliveriesQueryRequest {
    pub fn builder() -> ListDeliveriesQueryRequestBuilder {
        <ListDeliveriesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDeliveriesQueryRequestBuilder {
    first: Option<i64>,
    after: Option<String>,
    sent_after: Option<DateTime<FixedOffset>>,
    sent_before: Option<DateTime<FixedOffset>>,
    events: Option<Vec<Option<ListDeliveriesWebhooksRequestEventsItem>>>,
    status: Option<ListDeliveriesWebhooksRequestStatus>,
    resource_id: Option<String>,
}

impl ListDeliveriesQueryRequestBuilder {
    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn sent_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.sent_after = Some(value);
        self
    }

    pub fn sent_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.sent_before = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<Option<ListDeliveriesWebhooksRequestEventsItem>>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn status(mut self, value: ListDeliveriesWebhooksRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListDeliveriesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](ListDeliveriesQueryRequestBuilder::events)
    pub fn build(self) -> Result<ListDeliveriesQueryRequest, BuildError> {
        Ok(ListDeliveriesQueryRequest {
            first: self.first,
            after: self.after,
            sent_after: self.sent_after,
            sent_before: self.sent_before,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            status: self.status,
            resource_id: self.resource_id,
        })
    }
}
