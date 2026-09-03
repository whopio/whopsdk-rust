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

    /// Lists payments, newest first. Without filters this is every payment the caller can read: a company credential's own account, or for a user every account they can read payments for. Filters narrow by account, buyer, product, plan, membership, status, billing reason, currency, and creation window. Filtering by `billing_reason=subscription_cycle` also matches renewals recorded as `subscription_update`. `settlement_time_at` is null on list rows — retrieve the payment for it.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Only payments charged by this account, prefixed `biz_`.
    /// * `status` - Only payments in this lifecycle state.
    /// * `billing_reason` - Only payments charged for this reason.
    /// * `currency` - Only payments presented in this three-letter currency, such as `usd`.
    /// * `user_id` - Only payments made by this buyer, prefixed `user_`.
    /// * `query` - Search payments by user ID, membership ID, user email, name, or username. Email filtering requires the member:email:read permission.
    /// * `member_id` - Only payments made by this member, prefixed `mber_`.
    /// * `membership_id` - Only payments billed under this membership, prefixed `mem_`.
    /// * `product_id` - Only payments for this product, prefixed `prod_`.
    /// * `plan_id` - Only payments priced by this plan, prefixed `plan_`.
    /// * `created_before` - Only payments created before this ISO 8601 timestamp.
    /// * `created_after` - Only payments created after this ISO 8601 timestamp.
    /// * `order` - The field to sort by.
    /// * `direction` - The sort direction.
    /// * `first` - The number of payments to return.
    /// * `after` - A cursor; returns payments after this position.
    /// * `last` - The number of payments to return from the end of the range.
    /// * `before` - A cursor; returns payments before this position.
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
    ///                 ..Default::default()
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
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "payments",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize("billing_reason", request.billing_reason.clone())
                    .string("currency", request.currency.clone())
                    .string("user_id", request.user_id.clone())
                    .structured_query("query", request.query.clone())
                    .string("member_id", request.member_id.clone())
                    .string("membership_id", request.membership_id.clone())
                    .string("product_id", request.product_id.clone())
                    .string("plan_id", request.plan_id.clone())
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

    /// Charges a buyer for a plan. Pass a payment method already on file (`member_id` and `payment_method_id`), or a `confirmation_token` describing a method the buyer just supplied. Collection runs in the background: the response is the payment as created, not its outcome — poll Retrieve status for how far it has got and, for a confirmation-token payment, what the buyer must still do. `plan_id` names the plan to charge for.
    ///
    /// # Arguments
    ///
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
    ///             &CreatePaymentsRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 plan_id: "plan_xxxxxxxxxxxxxx".to_string(),
    ///                 capture: None,
    ///                 confirmation_token: None,
    ///                 email: None,
    ///                 member_id: None,
    ///                 metadata: None,
    ///                 payment_method_id: None,
    ///                 promo_code_id: None,
    ///                 return_url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePaymentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Payment, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
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

    /// Returns one payment. Related records are ids — resolve a plan, membership, member or shipment on its own endpoint, and list this payment's refunds, disputes or Resolution Center cases with `?payment_id=`.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment to retrieve, prefixed `pay_`.
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
    ///     client.payments.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Payment, ApiError> {
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
                &format!("payments/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Captures the full amount of a card payment created with `capture: false`. The payment must still be in `requires_capture` before `capture_expires_at`. Partial capture, multiple captures, capturing more than the authorized amount, and tips are not supported.
    ///
    /// # Arguments
    ///
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
    ///     client.payments.capture(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn capture(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentStatus, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("payments/{}/capture", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the fee breakdown of one payment — Whop's fee, processing, affiliate and other lines — each in the currency it was collected in and converted to the payment's settlement currency. The list is complete in one page.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment whose fees to list, prefixed `pay_`.
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
    ///     client.payments.list_fees(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn list_fees(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListFeesPaymentsResponse, ApiError> {
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
                &format!("payments/{}/fees", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Issues a full or partial refund for a payment. The refund is processed through the original payment processor and the membership status is updated accordingly.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment to refund, prefixed `pay_`.
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
    ///             &"id".to_string(),
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
                .or_insert_with(|| "2026-09-02-1".to_string());
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

    /// Retries a failed or pending payment. This re-attempts the charge using the original payment method and plan details.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment to retry, prefixed `pay_`.
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
    ///     client.payments.retry(&"id".to_string(), None).await;
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
                .or_insert_with(|| "2026-09-02-1".to_string());
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

    /// Voids a payment that has not yet been settled. Voiding cancels the payment before it is captured by the payment processor.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment to void, prefixed `pay_`.
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
    ///     client.payments.void(&"id".to_string(), None).await;
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
                .or_insert_with(|| "2026-09-02-1".to_string());
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
                .or_insert_with(|| "2026-09-02-1".to_string());
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
                .or_insert_with(|| "2026-09-02-1".to_string());
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
