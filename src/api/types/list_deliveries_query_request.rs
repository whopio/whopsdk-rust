pub use crate::prelude::*;

/// Query parameters for listDeliveries
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDeliveriesQueryRequest {
    /// The number of deliveries to return (default 50, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns deliveries after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
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

    /// Consumes the builder and constructs a [`ListDeliveriesQueryRequest`].
    pub fn build(self) -> Result<ListDeliveriesQueryRequest, BuildError> {
        Ok(ListDeliveriesQueryRequest {
            first: self.first,
            after: self.after,
        })
    }
}
