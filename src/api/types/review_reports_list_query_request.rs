pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReviewReportsListQueryRequest {
    /// Only reports on this account's reviews.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only reports filed against this review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_id: Option<String>,
    /// Only reports in this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListReviewReportsRequestStatus>,
    /// Only reports filed before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only reports filed after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// The field to sort reports by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListReviewReportsRequestOrder>,
    /// The sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListReviewReportsRequestDirection>,
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
}

impl ReviewReportsListQueryRequest {
    pub fn builder() -> ReviewReportsListQueryRequestBuilder {
        <ReviewReportsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewReportsListQueryRequestBuilder {
    account_id: Option<String>,
    review_id: Option<String>,
    status: Option<ListReviewReportsRequestStatus>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
    order: Option<ListReviewReportsRequestOrder>,
    direction: Option<ListReviewReportsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl ReviewReportsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn review_id(mut self, value: impl Into<String>) -> Self {
        self.review_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListReviewReportsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn order(mut self, value: ListReviewReportsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListReviewReportsRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`ReviewReportsListQueryRequest`].
    pub fn build(self) -> Result<ReviewReportsListQueryRequest, BuildError> {
        Ok(ReviewReportsListQueryRequest {
            account_id: self.account_id,
            review_id: self.review_id,
            status: self.status,
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
