pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchListQueryRequest {
    /// Text to search for, 2 to 250 characters.
    #[serde(default)]
    pub query: String,
    /// Kinds of records to search. Defaults to every kind. `account` rows are `account_id` itself and, with `include_subaccounts`, its subaccounts.
    #[serde(default)]
    pub types: Vec<Option<ListSearchRequestTypesItem>>,
    /// The account to search, prefixed `biz_`. Records other than accounts belong to it, or with `include_subaccounts` to its subaccounts. Your credential needs read access to at least one kind of record on it that search returns, or the account is a 404.
    #[serde(default)]
    pub account_id: String,
    /// `true` also searches the subaccounts of a platform `account_id`: an ID, an email, card digits or a phone number finds their records, and the subaccounts themselves are found as `account` rows by name, ID or store link, or by the email of their owner, a team member or a customer. Safe to send for any account: one without subaccounts is searched as if this were false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subaccounts: Option<bool>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
}

impl SearchListQueryRequest {
    pub fn builder() -> SearchListQueryRequestBuilder {
        <SearchListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchListQueryRequestBuilder {
    query: Option<String>,
    types: Option<Vec<Option<ListSearchRequestTypesItem>>>,
    account_id: Option<String>,
    include_subaccounts: Option<bool>,
    first: Option<i64>,
}

impl SearchListQueryRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn types(mut self, value: Vec<Option<ListSearchRequestTypesItem>>) -> Self {
        self.types = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn include_subaccounts(mut self, value: bool) -> Self {
        self.include_subaccounts = Some(value);
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](SearchListQueryRequestBuilder::query)
    /// - [`types`](SearchListQueryRequestBuilder::types)
    /// - [`account_id`](SearchListQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<SearchListQueryRequest, BuildError> {
        Ok(SearchListQueryRequest {
            query: self
                .query
                .ok_or_else(|| BuildError::missing_field("query"))?,
            types: self
                .types
                .ok_or_else(|| BuildError::missing_field("types"))?,
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            include_subaccounts: self.include_subaccounts,
            first: self.first,
        })
    }
}
