pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrieveFinancialReportsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beginning_balance: Option<f64>,
    /// Every lifetime cashflow currency, ordered by cashflow volume in the requested period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_balance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx_excluded_currencies: Option<Vec<String>>,
    /// Payment costs grouped by customer-facing payment method and provider when requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_fee_breakdown: Option<Vec<RetrieveFinancialReportsResponsePaymentFeeBreakdownItem>>,
    /// The report that was generated, echoing the requested `report_type`.
    pub report_type: RetrieveFinancialReportsResponseReportType,
    #[serde(default)]
    pub rows: Vec<RetrieveFinancialReportsResponseRowsItem>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total: f64,
}

impl RetrieveFinancialReportsResponse {
    pub fn builder() -> RetrieveFinancialReportsResponseBuilder {
        <RetrieveFinancialReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveFinancialReportsResponseBuilder {
    beginning_balance: Option<f64>,
    currencies: Option<Vec<String>>,
    ending_balance: Option<f64>,
    fx_excluded_currencies: Option<Vec<String>>,
    payment_fee_breakdown: Option<Vec<RetrieveFinancialReportsResponsePaymentFeeBreakdownItem>>,
    report_type: Option<RetrieveFinancialReportsResponseReportType>,
    rows: Option<Vec<RetrieveFinancialReportsResponseRowsItem>>,
    total: Option<f64>,
}

impl RetrieveFinancialReportsResponseBuilder {
    pub fn beginning_balance(mut self, value: f64) -> Self {
        self.beginning_balance = Some(value);
        self
    }

    pub fn currencies(mut self, value: Vec<String>) -> Self {
        self.currencies = Some(value);
        self
    }

    pub fn ending_balance(mut self, value: f64) -> Self {
        self.ending_balance = Some(value);
        self
    }

    pub fn fx_excluded_currencies(mut self, value: Vec<String>) -> Self {
        self.fx_excluded_currencies = Some(value);
        self
    }

    pub fn payment_fee_breakdown(
        mut self,
        value: Vec<RetrieveFinancialReportsResponsePaymentFeeBreakdownItem>,
    ) -> Self {
        self.payment_fee_breakdown = Some(value);
        self
    }

    pub fn report_type(mut self, value: RetrieveFinancialReportsResponseReportType) -> Self {
        self.report_type = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<RetrieveFinancialReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total(mut self, value: f64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveFinancialReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`report_type`](RetrieveFinancialReportsResponseBuilder::report_type)
    /// - [`rows`](RetrieveFinancialReportsResponseBuilder::rows)
    /// - [`total`](RetrieveFinancialReportsResponseBuilder::total)
    pub fn build(self) -> Result<RetrieveFinancialReportsResponse, BuildError> {
        Ok(RetrieveFinancialReportsResponse {
            beginning_balance: self.beginning_balance,
            currencies: self.currencies,
            ending_balance: self.ending_balance,
            fx_excluded_currencies: self.fx_excluded_currencies,
            payment_fee_breakdown: self.payment_fee_breakdown,
            report_type: self
                .report_type
                .ok_or_else(|| BuildError::missing_field("report_type"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
