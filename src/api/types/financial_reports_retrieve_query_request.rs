pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FinancialReportsRetrieveQueryRequest {
    /// The owning account ID (a biz_ identifier), or `global` for a platform-wide report across all ledger accounts (requires internal admin access).
    #[serde(default)]
    pub account_id: String,
    /// The type of financial report to generate.
    pub report_type: RetrieveFinancialReportsRequestReportType,
    /// Filter rows to this currency, for example `usd`. Defaults to `usd` unless `in_currency` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Aggregate all activity into this display currency via FX conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_currency: Option<String>,
    /// Start of the report window as an ISO 8601 timestamp (UTC). Required for platform-wide (global) reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    /// End of the report window as an ISO 8601 timestamp (UTC). Required for platform-wide (global) reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    /// Grouping granularity for report rows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<RetrieveFinancialReportsRequestGroupBy>,
    /// IANA timezone (for example `America/New_York`) used to bucket report periods and to interpret calendar-day boundaries for balance snapshots. Defaults to UTC. from_date/to_date remain exact instants regardless of this setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Account-level balance activity only: ledger line categories to include.
    #[serde(default)]
    pub line_types: Vec<Option<RetrieveFinancialReportsRequestLineTypesItem>>,
    /// Account-level balance activity only: include money moving in or money moving out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<RetrieveFinancialReportsRequestDirection>,
    /// Platform-wide (global) reports only: when true, return cumulative balances as of to_date (all history, no lower bound) instead of activity within the period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cumulative: Option<bool>,
    /// Platform-wide (global) reports only: narrow the report to ledger lines on the ledger account owned by this account ID (a biz_ identifier). Ignored unless account_id is `global`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_account_id: Option<String>,
    /// Balance activity only: include payment costs grouped by payment method and provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_payment_fee_breakdown: Option<bool>,
}

impl FinancialReportsRetrieveQueryRequest {
    pub fn builder() -> FinancialReportsRetrieveQueryRequestBuilder {
        <FinancialReportsRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancialReportsRetrieveQueryRequestBuilder {
    account_id: Option<String>,
    report_type: Option<RetrieveFinancialReportsRequestReportType>,
    currency: Option<String>,
    in_currency: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    group_by: Option<RetrieveFinancialReportsRequestGroupBy>,
    timezone: Option<String>,
    line_types: Option<Vec<Option<RetrieveFinancialReportsRequestLineTypesItem>>>,
    direction: Option<RetrieveFinancialReportsRequestDirection>,
    cumulative: Option<bool>,
    scope_account_id: Option<String>,
    include_payment_fee_breakdown: Option<bool>,
}

impl FinancialReportsRetrieveQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn report_type(mut self, value: RetrieveFinancialReportsRequestReportType) -> Self {
        self.report_type = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn in_currency(mut self, value: impl Into<String>) -> Self {
        self.in_currency = Some(value.into());
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

    pub fn group_by(mut self, value: RetrieveFinancialReportsRequestGroupBy) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn line_types(
        mut self,
        value: Vec<Option<RetrieveFinancialReportsRequestLineTypesItem>>,
    ) -> Self {
        self.line_types = Some(value);
        self
    }

    pub fn direction(mut self, value: RetrieveFinancialReportsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn cumulative(mut self, value: bool) -> Self {
        self.cumulative = Some(value);
        self
    }

    pub fn scope_account_id(mut self, value: impl Into<String>) -> Self {
        self.scope_account_id = Some(value.into());
        self
    }

    pub fn include_payment_fee_breakdown(mut self, value: bool) -> Self {
        self.include_payment_fee_breakdown = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinancialReportsRetrieveQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](FinancialReportsRetrieveQueryRequestBuilder::account_id)
    /// - [`report_type`](FinancialReportsRetrieveQueryRequestBuilder::report_type)
    /// - [`line_types`](FinancialReportsRetrieveQueryRequestBuilder::line_types)
    pub fn build(self) -> Result<FinancialReportsRetrieveQueryRequest, BuildError> {
        Ok(FinancialReportsRetrieveQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            report_type: self
                .report_type
                .ok_or_else(|| BuildError::missing_field("report_type"))?,
            currency: self.currency,
            in_currency: self.in_currency,
            from_date: self.from_date,
            to_date: self.to_date,
            group_by: self.group_by,
            timezone: self.timezone,
            line_types: self
                .line_types
                .ok_or_else(|| BuildError::missing_field("line_types"))?,
            direction: self.direction,
            cumulative: self.cumulative,
            scope_account_id: self.scope_account_id,
            include_payment_fee_breakdown: self.include_payment_fee_breakdown,
        })
    }
}
