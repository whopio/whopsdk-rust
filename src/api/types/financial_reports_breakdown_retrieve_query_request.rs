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
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub from: DateTime<FixedOffset>,
    /// Exclusive end of the report window as an ISO 8601 timestamp.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub to: DateTime<FixedOffset>,
    /// Period grouping used by the parent report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<RetrieveBreakdownRequestGroupBy>,
    /// IANA timezone used by the parent report to bucket periods. Defaults to UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
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
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    group_by: Option<RetrieveBreakdownRequestGroupBy>,
    timezone: Option<String>,
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

    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    pub fn group_by(mut self, value: RetrieveBreakdownRequestGroupBy) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FinancialReportsBreakdownRetrieveQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](FinancialReportsBreakdownRetrieveQueryRequestBuilder::account_id)
    /// - [`bucket`](FinancialReportsBreakdownRetrieveQueryRequestBuilder::bucket)
    /// - [`direction`](FinancialReportsBreakdownRetrieveQueryRequestBuilder::direction)
    /// - [`currency`](FinancialReportsBreakdownRetrieveQueryRequestBuilder::currency)
    /// - [`from`](FinancialReportsBreakdownRetrieveQueryRequestBuilder::from)
    /// - [`to`](FinancialReportsBreakdownRetrieveQueryRequestBuilder::to)
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
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            group_by: self.group_by,
            timezone: self.timezone,
        })
    }
}
