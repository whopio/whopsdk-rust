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

    /// Returns a paginated list of refunds, with optional filtering by payment, company, user, and creation date.
    ///
    /// Required permissions:
    /// - `payment:basic:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `payment_id` - Filter refunds to only those associated with this specific payment.
    /// * `company_id` - Filter refunds to only those belonging to this company.
    /// * `user_id` - Filter refunds to only those associated with this specific user.
    /// * `created_before` - Only return refunds created before this timestamp.
    /// * `created_after` - Only return refunds created after this timestamp.
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
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 payment_id: Some("pay_xxxxxxxxxxxxxx".to_string()),
    ///                 company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
    ///                 user_id: Some("user_xxxxxxxxxxxxx".to_string()),
    ///                 created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "refunds",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("payment_id", request.payment_id.clone())
                    .string("company_id", request.company_id.clone())
                    .string("user_id", request.user_id.clone())
                    .serialize("direction", request.direction.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the details of an existing refund.
    ///
    /// Required permissions:
    /// - `payment:basic:read`
    /// - `plan:basic:read`
    /// - `access_pass:basic:read`
    /// - `member:email:read`
    /// - `member:basic:read`
    /// - `member:phone:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the refund.
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
    ///         .retrieve(&"rf_xxxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, &format!("refunds/{}", id), None, None, options)
            .await
    }
}
