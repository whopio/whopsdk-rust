use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PromoCodesClient {
    pub http_client: HttpClient,
}

impl PromoCodesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists promo codes for an account with cursor pagination, filters, and sorting.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account whose promo codes are listed (`biz_` tag).
    /// * `status` - Promo-code status. `expired` groups inactive and archived codes.
    /// * `product_ids` - Only promo codes scoped to these product IDs.
    /// * `plan_ids` - Only promo codes scoped to these plan IDs.
    /// * `created_before` - Only promo codes created before this ISO 8601 timestamp.
    /// * `created_after` - Only promo codes created after this ISO 8601 timestamp.
    /// * `order` - Sort field.
    /// * `direction` - Sort direction.
    /// * `first` - Number of promo codes to return from the start of the window.
    /// * `after` - Cursor to paginate forwards from.
    /// * `last` - Number of promo codes to return from the end of the window.
    /// * `before` - Cursor to paginate backwards from.
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
    ///         .promo_codes
    ///         .list(
    ///             &PromoCodesListQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 product_ids: vec![Some("prod_xxxxxxxxxxxxxx".to_string())],
    ///                 plan_ids: vec![Some("plan_xxxxxxxxxxxxxx".to_string())],
    ///                 status: None,
    ///                 created_before: None,
    ///                 created_after: None,
    ///                 order: None,
    ///                 direction: None,
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PromoCodesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPromoCodesResponse, ApiError> {
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
                "promo_codes",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("status", request.status.clone())
                    .string_array("product_ids", request.product_ids.clone())
                    .string_array("plan_ids", request.plan_ids.clone())
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

    /// Creates a promo code for an account. First-party sessions may attach an affiliate.
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
    ///         .promo_codes
    ///         .create(
    ///             &CreatePromoCodesRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 amount_off: 25.0,
    ///                 base_currency: CreatePromoCodesRequestBaseCurrency::Usd,
    ///                 code: "AFFILIATE25".to_string(),
    ///                 new_users_only: true,
    ///                 promo_duration_months: 3,
    ///                 promo_type: CreatePromoCodesRequestPromoType::Percentage,
    ///                 churned_users_only: None,
    ///                 existing_memberships_only: None,
    ///                 expires_at: None,
    ///                 one_per_customer: None,
    ///                 plan_ids: None,
    ///                 product_id: None,
    ///                 stock: None,
    ///                 unlimited_stock: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePromoCodesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PromoCode, ApiError> {
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
                "promo_codes",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a promo code by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Promo code ID (`promo_` tag).
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
    ///     client.promo_codes.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PromoCode, ApiError> {
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
                &format!("promo_codes/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Archives a promo code so it cannot be used in future checkouts.
    ///
    /// # Arguments
    ///
    /// * `id` - Promo code ID (`promo_` tag).
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
    ///     client.promo_codes.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeletePromoCodesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("promo_codes/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Turns an inactive promo code back on so it can be redeemed at checkout.
    ///
    /// # Arguments
    ///
    /// * `id` - Promo code ID (`promo_` tag).
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
    ///     client.promo_codes.activate(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn activate(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PromoCode, ApiError> {
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
                &format!("promo_codes/{}/activate", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Turns off an active promo code so it can no longer be redeemed at checkout.
    ///
    /// # Arguments
    ///
    /// * `id` - Promo code ID (`promo_` tag).
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
    ///     client.promo_codes.deactivate(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn deactivate(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PromoCode, ApiError> {
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
                &format!("promo_codes/{}/deactivate", id),
                None,
                None,
                options,
            )
            .await
    }
}
