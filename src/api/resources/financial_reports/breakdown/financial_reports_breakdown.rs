use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BreakdownClient {
    pub http_client: HttpClient,
}

impl BreakdownClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the top entities behind one high-level financial report bucket and an aggregate remainder.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The owning account ID (a biz_ identifier).
    /// * `bucket` - The high-level report bucket to explain.
    /// * `direction` - Whether to explain money received or money sent.
    /// * `currency` - The report currency to explain.
    /// * `from` - Start of the report window as an ISO 8601 timestamp.
    /// * `to` - Exclusive end of the report window as an ISO 8601 timestamp.
    /// * `group_by` - Period grouping used by the parent report.
    /// * `timezone` - IANA timezone used by the parent report to bucket periods. Defaults to UTC.
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
    ///         .breakdown
    ///         .retrieve(
    ///             &FinancialReportsBreakdownRetrieveQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 bucket: RetrieveBreakdownRequestBucket::Transfers,
    ///                 direction: RetrieveBreakdownRequestDirection::MoneyIn,
    ///                 currency: "currency".to_string(),
    ///                 from: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
    ///                 to: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
    ///                 group_by: None,
    ///                 timezone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        request: &FinancialReportsBreakdownRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveBreakdownResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "financial_reports/breakdown",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("bucket", Some(request.bucket.clone()))
                    .serialize("direction", Some(request.direction.clone()))
                    .string("currency", request.currency.clone())
                    .datetime("from", request.from.clone())
                    .datetime("to", request.to.clone())
                    .serialize("group_by", request.group_by.clone())
                    .string("timezone", request.timezone.clone())
                    .build(),
                options,
            )
            .await
    }
}
