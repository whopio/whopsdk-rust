pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PartnersLinksListQueryRequest {
    /// Filter promotion links by availability. Repeat the status parameter for multiple values.
    #[serde(default)]
    pub status: Vec<Option<ListLinksRequestStatusItem>>,
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

impl PartnersLinksListQueryRequest {
    pub fn builder() -> PartnersLinksListQueryRequestBuilder {
        <PartnersLinksListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnersLinksListQueryRequestBuilder {
    status: Option<Vec<Option<ListLinksRequestStatusItem>>>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl PartnersLinksListQueryRequestBuilder {
    pub fn status(mut self, value: Vec<Option<ListLinksRequestStatusItem>>) -> Self {
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

    /// Consumes the builder and constructs a [`PartnersLinksListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](PartnersLinksListQueryRequestBuilder::status)
    pub fn build(self) -> Result<PartnersLinksListQueryRequest, BuildError> {
        Ok(PartnersLinksListQueryRequest {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
