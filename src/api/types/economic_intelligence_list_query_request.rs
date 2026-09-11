pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EconomicIntelligenceListQueryRequest {
    /// Account ID, prefixed `biz_`. Defaults to the API key's own account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only recommendations in this state. `ready` for the cards the owner can run now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListEconomicIntelligenceRequestStatus>,
    /// The number of recommendations to return (default 20, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns recommendations after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of recommendations to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns recommendations before this position.
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
