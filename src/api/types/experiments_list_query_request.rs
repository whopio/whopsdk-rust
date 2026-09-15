pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperimentsListQueryRequest {
    /// Owning account ID. Omit or pass internal for Whop internal experiments; internal access is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Filter by related resource; requires account_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resource: Option<ExperimentResourceReference>,
    /// Only experiments with this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListExperimentsRequestStatus>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The field to sort experiments by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListExperimentsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListExperimentsRequestDirection>,
}

impl ExperimentsListQueryRequest {
    pub fn builder() -> ExperimentsListQueryRequestBuilder {
        <ExperimentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperimentsListQueryRequestBuilder {
    account_id: Option<String>,
    related_resource: Option<ExperimentResourceReference>,
    status: Option<ListExperimentsRequestStatus>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListExperimentsRequestOrder>,
    direction: Option<ListExperimentsRequestDirection>,
}

impl ExperimentsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn related_resource(mut self, value: ExperimentResourceReference) -> Self {
        self.related_resource = Some(value);
        self
    }

    pub fn status(mut self, value: ListExperimentsRequestStatus) -> Self {
        self.status = Some(value);
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

    pub fn order(mut self, value: ListExperimentsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListExperimentsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExperimentsListQueryRequest`].
    pub fn build(self) -> Result<ExperimentsListQueryRequest, BuildError> {
        Ok(ExperimentsListQueryRequest {
            account_id: self.account_id,
            related_resource: self.related_resource,
            status: self.status,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
        })
    }
}
