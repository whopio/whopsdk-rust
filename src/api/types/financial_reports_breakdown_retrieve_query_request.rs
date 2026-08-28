pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FinancialReportsBreakdownRetrieveQueryRequest {
    /// The owning account ID (a biz_ identifier).
    #[serde(default)]
    pub account_id: String,
    /// The high-level report bucket to explain.
    pub bucket: RetrieveBreakdownRequestBucket,
    /// Whether to explain money received or money sent.
    pub direction: RetrieveBreakdownRequestDirection,
    /// The report currency to explain.
    #[serde(default)]
    pub currency: String,
    /// Start of the report window as an ISO 8601 timestamp.
    #[serde(default)]
    pub from_date: String,
    /// Exclusive end of the report window as an ISO 8601 timestamp.
    #[serde(default)]
    pub to_date: String,
}

impl FinancialReportsBreakdownRetrieveQueryRequest {
    pub fn builder() -> FinancialReportsBreakdownRetrieveQueryRequestBuilder {
        <FinancialReportsBreakdownRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancialReportsBreakdownRetrieveQueryRequestBuilder {
    account_id: Option<String>,
    bucket: Option<RetrieveBreakdownRequestBucket>,
    direction: Option<RetrieveBreakdownRequestDirection>,
    currency: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
}

impl FinancialReportsBreakdownRetrieveQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn bucket(mut self, value: RetrieveBreakdownRequestBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    pub fn direction(mut self, value: RetrieveBreakdownRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FinancialReportsBreakdownRetrieveQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](FinancialReportsBreakdownRetrieveQueryRequestBuilder::account_id)
    /// - [`bucket`](FinancialReportsBreakdownRetrieveQueryRequestBuilder::bucket)
    /// - [`direction`](FinancialReportsBreakdownRetrieveQueryRequestBuilder::direction)
    /// - [`currency`](FinancialReportsBreakdownRetrieveQueryRequestBuilder::currency)
    /// - [`from_date`](FinancialReportsBreakdownRetrieveQueryRequestBuilder::from_date)
    /// - [`to_date`](FinancialReportsBreakdownRetrieveQueryRequestBuilder::to_date)
    pub fn build(self) -> Result<FinancialReportsBreakdownRetrieveQueryRequest, BuildError> {
        Ok(FinancialReportsBreakdownRetrieveQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            bucket: self
                .bucket
                .ok_or_else(|| BuildError::missing_field("bucket"))?,
            direction: self
                .direction
                .ok_or_else(|| BuildError::missing_field("direction"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
