pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EconomicIntelligenceListQueryRequest {
    /// Account ID, prefixed `biz_`. Defaults to the API key's own account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Filter recommendations by their current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListEconomicIntelligenceRequestStatus>,
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

impl EconomicIntelligenceListQueryRequest {
    pub fn builder() -> EconomicIntelligenceListQueryRequestBuilder {
        <EconomicIntelligenceListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EconomicIntelligenceListQueryRequestBuilder {
    account_id: Option<String>,
    status: Option<ListEconomicIntelligenceRequestStatus>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl EconomicIntelligenceListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListEconomicIntelligenceRequestStatus) -> Self {
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

    /// Consumes the builder and constructs a [`EconomicIntelligenceListQueryRequest`].
    pub fn build(self) -> Result<EconomicIntelligenceListQueryRequest, BuildError> {
        Ok(EconomicIntelligenceListQueryRequest {
            account_id: self.account_id,
            status: self.status,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
