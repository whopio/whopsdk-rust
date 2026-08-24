pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetFinancialReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beginning_balance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_balance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx_excluded_currencies: Option<Vec<String>>,
    /// The report that was generated, echoing the requested `report_type`.
    pub report_type: GetFinancialReportResponseReportType,
    #[serde(default)]
    pub rows: Vec<GetFinancialReportResponseRowsItem>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total: f64,
}

impl GetFinancialReportResponse {
    pub fn builder() -> GetFinancialReportResponseBuilder {
        <GetFinancialReportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetFinancialReportResponseBuilder {
    beginning_balance: Option<f64>,
    ending_balance: Option<f64>,
    fx_excluded_currencies: Option<Vec<String>>,
    report_type: Option<GetFinancialReportResponseReportType>,
    rows: Option<Vec<GetFinancialReportResponseRowsItem>>,
    total: Option<f64>,
}

impl GetFinancialReportResponseBuilder {
    pub fn beginning_balance(mut self, value: f64) -> Self {
        self.beginning_balance = Some(value);
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

    pub fn report_type(mut self, value: GetFinancialReportResponseReportType) -> Self {
        self.report_type = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<GetFinancialReportResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total(mut self, value: f64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetFinancialReportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`report_type`](GetFinancialReportResponseBuilder::report_type)
    /// - [`rows`](GetFinancialReportResponseBuilder::rows)
    /// - [`total`](GetFinancialReportResponseBuilder::total)
    pub fn build(self) -> Result<GetFinancialReportResponse, BuildError> {
        Ok(GetFinancialReportResponse {
            beginning_balance: self.beginning_balance,
            ending_balance: self.ending_balance,
            fx_excluded_currencies: self.fx_excluded_currencies,
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
