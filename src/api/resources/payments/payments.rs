use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PaymentsClient {
    pub http_client: HttpClient,
}

impl PaymentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of payments for the actor in context, with optional filtering by product, plan, status, billing reason, currency, and creation date.
    ///
    /// Required permissions:
    /// - `payment:basic:read`
    /// - `plan:basic:read`
    /// - `access_pass:basic:read`
    /// - `member:email:read`
    /// - `member:basic:read`
    /// - `member:phone:read`
    /// - `promo_code:basic:read`
    /// - `shipment:basic:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `company_id` - The unique identifier of the company to list payments for.
    /// * `product_ids` - Filter payments to only those associated with these specific product identifiers.
    /// * `billing_reasons` - Filter payments by their billing reason.
    /// * `currencies` - Filter payments by their currency code.
    /// * `plan_ids` - Filter payments to only those associated with these specific plan identifiers.
    /// * `statuses` - Filter payments by their current status.
    /// * `substatuses` - Filter payments by their current substatus for more granular filtering.
    /// * `include_free` - Whether to include payments with a zero amount.
    /// * `created_before` - Only return payments created before this timestamp.
    /// * `created_after` - Only return payments created after this timestamp.
    /// * `updated_before` - Only return payments last updated before this timestamp.
    /// * `updated_after` - Only return payments last updated after this timestamp.
    /// * `query` - Search payments by user ID, membership ID, user email, name, or username. Email filtering requires the member:email:read permission.
    /// * `checkout_configuration_ids` - Only return payments from these checkout configurations.
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
    ///         .payments
    ///         .list(
    ///             &PaymentsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
    ///                 created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 updated_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 updated_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 after: None,
    ///                 before: None,
    ///                 direction: None,
    ///                 order: None,
    ///                 product_ids: vec![],
    ///                 billing_reasons: vec![],
    ///                 currencies: vec![],
    ///                 plan_ids: vec![],
    ///                 statuses: vec![],
    ///                 substatuses: vec![],
    ///                 include_free: None,
    ///                 query: None,
    ///                 checkout_configuration_ids: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PaymentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPaymentsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "payments",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("company_id", request.company_id.clone())
                    .serialize("direction", request.direction.clone())
                    .serialize("order", request.order.clone())
                    .string_array("product_ids", request.product_ids.clone())
                    .serialize_array("billing_reasons", request.billing_reasons.clone())
                    .serialize_array("currencies", request.currencies.clone())
                    .string_array("plan_ids", request.plan_ids.clone())
                    .serialize_array("statuses", request.statuses.clone())
                    .serialize_array("substatuses", request.substatuses.clone())
                    .bool("include_free", request.include_free.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
                    .datetime("updated_before", request.updated_before.clone())
                    .datetime("updated_after", request.updated_after.clone())
                    .structured_query("query", request.query.clone())
                    .string_array(
                        "checkout_configuration_ids",
                        request.checkout_configuration_ids.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Charge an existing member off-session using one of their stored payment methods. You can provide an existing plan, or create a new one in-line. This endpoint will respond with a payment object immediately, but the payment is processed asynchronously in the background. Use webhooks to be notified when the payment succeeds or fails.
    ///
    /// Required permissions:
    /// - `payment:charge`
    /// - `plan:create`
    /// - `access_pass:create`
    /// - `access_pass:update`
    /// - `plan:basic:read`
    /// - `access_pass:basic:read`
    /// - `member:email:read`
    /// - `member:basic:read`
    /// - `member:phone:read`
    /// - `promo_code:basic:read`
    /// - `shipment:basic:read`
    /// - `payment:dispute:read`
    /// - `payment:resolution_center_case:read`
    ///
    /// # Arguments
    ///
    /// * `request` - Parameters for CreatePayment
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
    ///         .payments
    ///         .create(
    ///             &CreatePaymentsRequestBody::CreatePaymentsRequestBodyZero(
    ///                 CreatePaymentsRequestBodyZero {
    ///                     company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                     confirmation_token: "confirmation_token".to_string(),
    ///                     email: None,
    ///                     metadata: None,
    ///                     payment_method_id: None,
    ///                     plan: CreatePaymentsRequestBodyZeroPlan {
    ///                         application_fee_amount: None,
    ///                         billing_period: None,
    ///                         currency: Currencies::Usd,
    ///                         description: None,
    ///                         expiration_days: None,
    ///                         force_create_new_plan: None,
    ///                         initial_price: None,
    ///                         internal_notes: None,
    ///                         plan_type: None,
    ///                         product: None,
    ///                         product_id: None,
    ///                         renewal_price: None,
    ///                         title: None,
    ///                         trial_period_days: None,
    ///                         visibility: None,
    ///                     },
    ///                     promo_code_id: None,
    ///                     return_url: None,
    ///                 },
    ///             ),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePaymentsRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<CreatePaymentsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "payments",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing payment.
    ///
    /// Required permissions:
    /// - `payment:basic:read`
    /// - `plan:basic:read`
    /// - `access_pass:basic:read`
    /// - `member:email:read`
    /// - `member:basic:read`
    /// - `member:phone:read`
    /// - `promo_code:basic:read`
    /// - `shipment:basic:read`
    /// - `payment:dispute:read`
    /// - `payment:resolution_center_case:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the payment.
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
    ///         .payments
    ///         .retrieve(&"pay_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RetrievePaymentsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("payments/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the list of fees associated with a specific payment, including platform fees and processing fees.
    ///
    /// Required permissions:
    /// - `payment:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the payment to list fees for.
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
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
    ///         .payments
    ///         .list_fees(
    ///             &"pay_xxxxxxxxxxxxxx".to_string(),
    ///             &ListFeesQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_fees(
        &self,
        id: &str,
        request: &ListFeesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFeesPaymentsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("payments/{}/fees", id),
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Issue a full or partial refund for a payment. The refund is processed through the original payment processor and the membership status is updated accordingly.
    ///
    /// Required permissions:
    /// - `payment:manage`
    /// - `plan:basic:read`
    /// - `access_pass:basic:read`
    /// - `member:email:read`
    /// - `member:basic:read`
    /// - `member:phone:read`
    /// - `promo_code:basic:read`
    /// - `shipment:basic:read`
    /// - `payment:dispute:read`
    /// - `payment:resolution_center_case:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the payment to refund.
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
    ///         .payments
    ///         .refund(
    ///             &"pay_xxxxxxxxxxxxxx".to_string(),
    ///             &RefundPaymentsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn refund(
        &self,
        id: &str,
        request: &RefundPaymentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Payment, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("payments/{}/refund", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retry a failed or pending payment. This re-attempts the charge using the original payment method and plan details.
    ///
    /// Required permissions:
    /// - `payment:manage`
    /// - `plan:basic:read`
    /// - `access_pass:basic:read`
    /// - `member:email:read`
    /// - `member:basic:read`
    /// - `member:phone:read`
    /// - `promo_code:basic:read`
    /// - `shipment:basic:read`
    /// - `payment:dispute:read`
    /// - `payment:resolution_center_case:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the payment to retry.
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
    ///         .payments
    ///         .retry(&"pay_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retry(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Payment, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("payments/{}/retry", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Void a payment that has not yet been settled. Voiding cancels the payment before it is captured by the payment processor.
    ///
    /// Required permissions:
    /// - `payment:manage`
    /// - `plan:basic:read`
    /// - `access_pass:basic:read`
    /// - `member:email:read`
    /// - `member:basic:read`
    /// - `member:phone:read`
    /// - `promo_code:basic:read`
    /// - `shipment:basic:read`
    /// - `payment:dispute:read`
    /// - `payment:resolution_center_case:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the payment to void.
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
    ///         .payments
    ///         .void(&"pay_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn void(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Payment, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("payments/{}/void", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Changes where the buyer lands after completing an off-site step, up until they return. Accepts either a secret key or the payment's own `client_secret`, so the surface that knows the final destination can set it.
    ///
    /// # Arguments
    ///
    /// * `payment_id` - The unique identifier of the payment.
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
    ///         .payments
    ///         .update_return_url(
    ///             &"payment_id".to_string(),
    ///             &UpdateReturnURLPaymentsRequest {
    ///                 return_url: "https://shinetime.example/checkout/thanks".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_return_url(
        &self,
        payment_id: &str,
        request: &UpdateReturnUrlPaymentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentStatus, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("payments/{}/return_url", payment_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves how far a payment has got and what the buyer must do next, if anything. A payment is collected in the background, so poll this rather than reading the create response. Accepts either a secret key or the payment's own `client_secret`, so the surface collecting the payment can poll it directly.
    ///
    /// # Arguments
    ///
    /// * `payment_id` - The unique identifier of the payment.
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
    ///         .payments
    ///         .retrieve_status(&"payment_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve_status(
        &self,
        payment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentStatus, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("payments/{}/status", payment_id),
                None,
                None,
                options,
            )
            .await
    }
}
