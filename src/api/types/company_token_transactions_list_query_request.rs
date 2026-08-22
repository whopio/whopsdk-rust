pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompanyTokenTransactionsListQueryRequest {
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
    /// The unique identifier of the company to list token transactions for.
    #[serde(default)]
    pub company_id: String,
    /// Filter transactions to only those involving this specific user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<CompanyTokenTransactionTypes>,
}

impl CompanyTokenTransactionsListQueryRequest {
    pub fn builder() -> CompanyTokenTransactionsListQueryRequestBuilder {
        <CompanyTokenTransactionsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyTokenTransactionsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    company_id: Option<String>,
    user_id: Option<String>,
    transaction_type: Option<CompanyTokenTransactionTypes>,
}

impl CompanyTokenTransactionsListQueryRequestBuilder {
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

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn transaction_type(mut self, value: CompanyTokenTransactionTypes) -> Self {
        self.transaction_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompanyTokenTransactionsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](CompanyTokenTransactionsListQueryRequestBuilder::company_id)
    pub fn build(self) -> Result<CompanyTokenTransactionsListQueryRequest, BuildError> {
        Ok(CompanyTokenTransactionsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            user_id: self.user_id,
            transaction_type: self.transaction_type,
        })
    }
}
