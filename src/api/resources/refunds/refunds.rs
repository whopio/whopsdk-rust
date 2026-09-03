use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RefundsClient {
    pub http_client: HttpClient,
}

impl RefundsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists refunds, newest first. Without filters this is every refund the caller can read; narrow it to one payment with `payment_id`, one account with `account_id`, or one buyer with `user_id`.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Only refunds issued by this account, prefixed `biz_`.
    /// * `payment_id` - Only refunds of this payment, prefixed `pay_`.
    /// * `user_id` - Only refunds to this buyer, prefixed `user_`.
    /// * `created_before` - Only refunds requested before this ISO 8601 timestamp.
    /// * `created_after` - Only refunds requested after this ISO 8601 timestamp.
    /// * `order` - The field to sort by.
    /// * `direction` - The sort direction.
    /// * `first` - The number of refunds to return.
    /// * `after` - A cursor; returns refunds after this position.
    /// * `last` - The number of refunds to return from the end of the range.
    /// * `before` - A cursor; returns refunds before this position.
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
    ///         .refunds
    ///         .list(
    ///             &RefundsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &RefundsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListRefundsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "refunds",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("payment_id", request.payment_id.clone())
                    .string("user_id", request.user_id.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns one refund.
    ///
    /// # Arguments
    ///
    /// * `id` - The refund to retrieve, prefixed `rf_`.
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
    ///     client.refunds.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Refund, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, &format!("refunds/{}", id), None, None, options)
            .await
    }
}
