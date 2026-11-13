pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EconomicIntelligenceListQueryRequest {
    /// Account ID, prefixed `biz_`. Defaults to the API key's own account. Omit with `has_run` for the runs on every account you can read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// `ready` lists the recommendations the account can run now. A run status lists only the runs in that state. `queued`, `pending` and `superseded` are no longer listed and answer nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListEconomicIntelligenceRequestStatus>,
    /// No longer read: recommendations are written for the business as it changes. Up to 1000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// When true, only recommendations someone has started running, by any user. Without `account_id`, covers every account you can read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_run: Option<bool>,
    /// Sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListEconomicIntelligenceRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListEconomicIntelligenceRequestDirection>,
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
    input: Option<String>,
    has_run: Option<bool>,
    order: Option<ListEconomicIntelligenceRequestOrder>,
    direction: Option<ListEconomicIntelligenceRequestDirection>,
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

    pub fn input(mut self, value: impl Into<String>) -> Self {
        self.input = Some(value.into());
        self
    }

    pub fn has_run(mut self, value: bool) -> Self {
        self.has_run = Some(value);
        self
    }

    pub fn order(mut self, value: ListEconomicIntelligenceRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListEconomicIntelligenceRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`EconomicIntelligenceListQueryRequest`].
    pub fn build(self) -> Result<EconomicIntelligenceListQueryRequest, BuildError> {
        Ok(EconomicIntelligenceListQueryRequest {
            account_id: self.account_id,
            status: self.status,
            input: self.input,
            has_run: self.has_run,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
