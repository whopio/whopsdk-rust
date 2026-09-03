use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DisputeAlertsClient {
    pub http_client: HttpClient,
}

impl DisputeAlertsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the dispute alerts and early fraud warnings across the accounts you can read.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Only alerts on this account's payments (`biz_` tag). Omit it to cover every account you can read.
    /// * `payment_id` - Only alerts on this payment (`pay_` tag). A payment can carry several.
    /// * `type_` - Only alerts of this kind. `early_fraud_warning` for issuer fraud reports, `dispute_alert` for pre-dispute notices, `rapid_dispute_resolution` for Visa RDR cases the network already closed.
    /// * `first` - The number of alerts to return (default 20, max 100).
    /// * `after` - A cursor; returns alerts after this position.
    /// * `last` - The number of alerts to return from the end of the range.
    /// * `before` - A cursor; returns alerts before this position.
    /// * `order` - The field to sort alerts by.
    /// * `direction` - Sort direction.
    /// * `created_before` - Only alerts Whop received before this ISO 8601 timestamp.
    /// * `created_after` - Only alerts Whop received after this ISO 8601 timestamp.
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
    ///         .dispute_alerts
    ///         .list(
    ///             &DisputeAlertsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &DisputeAlertsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDisputeAlertsResponse, ApiError> {
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
                "dispute_alerts",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("payment_id", request.payment_id.clone())
                    .serialize("type", request.r#type.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a single dispute alert or early fraud warning by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The dispute alert ID, prefixed `dspa_`.
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
    ///         .dispute_alerts
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DisputeAlert, ApiError> {
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
                &format!("dispute_alerts/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
