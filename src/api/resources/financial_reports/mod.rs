use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod breakdown;
pub use breakdown::BreakdownClient;
pub struct FinancialReportsClient {
    pub http_client: HttpClient,
    pub breakdown: BreakdownClient,
}

impl FinancialReportsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            breakdown: BreakdownClient::new(config.clone())?,
        })
    }

    /// Returns a financial report — balance activity, income statement, or balance summary — for an account over a date range.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account that owns the report, prefixed `biz_`.
    /// * `report_type` - The type of financial report to generate.
    /// * `currency` - Filter rows to this currency, for example `usd`. Defaults to `usd` unless `in_currency` is provided.
    /// * `in_currency` - Aggregate all activity into this display currency via FX conversion.
    /// * `from` - Start of the report window as an ISO 8601 timestamp.
    /// * `to` - Exclusive end of the report window as an ISO 8601 timestamp.
    /// * `group_by` - Grouping granularity for report rows. Hourly grouping is supported for account-level balance activity reports only; hourly periods are timestamps in the requested timezone.
    /// * `timezone` - IANA timezone (for example `America/New_York`) used to bucket report periods. Defaults to UTC. `from` and `to` remain exact instants.
    /// * `line_types` - Account-level balance activity only: ledger line categories to include.
    /// * `direction` - Account-level balance activity only: include money moving in or money moving out.
    /// * `include_payment_fee_breakdown` - Balance activity only: include payment costs grouped by payment method and provider.
    /// * `period_only_currencies` - Account-level balance activity only: return only currencies with cashflow in the exact requested window. Requires `from` and `to`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use whop_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Whop::new(config).expect("Failed to build client");
    ///     client
    ///         .financial_reports
    ///         .retrieve(
    ///             &FinancialReportsRetrieveQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 report_type: RetrieveFinancialReportsRequestReportType::BalanceSummary,
    ///                 currency: None,
    ///                 in_currency: None,
    ///                 from: None,
    ///                 to: None,
    ///                 group_by: None,
    ///                 timezone: None,
    ///                 line_types: vec![],
    ///                 direction: None,
    ///                 include_payment_fee_breakdown: None,
    ///                 period_only_currencies: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        request: &FinancialReportsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveFinancialReportsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                "financial_reports",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("report_type", Some(request.report_type.clone()))
                    .string("currency", request.currency.clone())
                    .string("in_currency", request.in_currency.clone())
                    .datetime("from", request.from.clone())
                    .datetime("to", request.to.clone())
                    .serialize("group_by", request.group_by.clone())
                    .string("timezone", request.timezone.clone())
                    .serialize_array("line_types", request.line_types.clone())
                    .serialize("direction", request.direction.clone())
                    .bool(
                        "include_payment_fee_breakdown",
                        request.include_payment_fee_breakdown.clone(),
                    )
                    .bool(
                        "period_only_currencies",
                        request.period_only_currencies.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }
}
