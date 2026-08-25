use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LedgersClient {
    pub http_client: HttpClient,
}

impl LedgersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a financial report — balance activity, income statement, or balance summary — for an account over a date range.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The owning account ID (a biz_ identifier), or `global` for a platform-wide report across all ledger accounts (requires internal admin access).
    /// * `report_type` - The type of financial report to generate.
    /// * `currency` - Filter rows to this currency, for example `usd`. Defaults to `usd` unless `in_currency` is provided.
    /// * `in_currency` - Aggregate all activity into this display currency via FX conversion.
    /// * `from_date` - Start of the report window as an ISO 8601 timestamp (UTC). Required for platform-wide (global) reports.
    /// * `to_date` - End of the report window as an ISO 8601 timestamp (UTC). Required for platform-wide (global) reports.
    /// * `group_by` - Grouping granularity for report rows.
    /// * `timezone` - IANA timezone (for example `America/New_York`) used to bucket report periods and to interpret calendar-day boundaries for balance snapshots. Defaults to UTC. from_date/to_date remain exact instants regardless of this setting.
    /// * `cumulative` - Platform-wide (global) reports only: when true, return cumulative balances as of to_date (all history, no lower bound) instead of activity within the period.
    /// * `scope_account_id` - Platform-wide (global) reports only: narrow the report to ledger lines on the ledger account owned by this account ID (a biz_ identifier). Ignored unless account_id is `global`.
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
    ///         .ledgers
    ///         .get_financial_report(
    ///             &GetFinancialReportQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 report_type: GetFinancialReportRequestReportType::BalanceSummary,
    ///                 currency: None,
    ///                 in_currency: None,
    ///                 from_date: None,
    ///                 to_date: None,
    ///                 group_by: None,
    ///                 timezone: None,
    ///                 cumulative: None,
    ///                 scope_account_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_financial_report(
        &self,
        request: &GetFinancialReportQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetFinancialReportResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "financial_reports",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("report_type", Some(request.report_type.clone()))
                    .string("currency", request.currency.clone())
                    .string("in_currency", request.in_currency.clone())
                    .string("from_date", request.from_date.clone())
                    .string("to_date", request.to_date.clone())
                    .serialize("group_by", request.group_by.clone())
                    .string("timezone", request.timezone.clone())
                    .bool("cumulative", request.cumulative.clone())
                    .string("scope_account_id", request.scope_account_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
