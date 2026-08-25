use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PlansClient {
    pub http_client: HttpClient,
}

impl PlansClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of plans. Omit `account_id` and pass `product_ids` to list a product's public buyable plans.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The unique identifier of the account to list plans for. Required unless `product_ids` is provided for a public product-plan read.
    /// * `direction` - The sort direction for results. Defaults to descending.
    /// * `order` - The field to sort results by. Defaults to created_at.
    /// * `release_methods` - Filter to only plans matching these release methods.
    /// * `visibilities` - Filter to only plans matching these visibility states.
    /// * `plan_types` - Filter to only plans matching these billing types.
    /// * `product_ids` - Filter to only plans belonging to these product identifiers. When `account_id` is omitted, this is required and the response is publicly readable: only visible, non-invoice plans are returned.
    /// * `created_before` - Only return plans created before this timestamp.
    /// * `created_after` - Only return plans created after this timestamp.
    /// * `first` - The number of plans to return (default and max 100).
    /// * `after` - A cursor; returns plans after this position.
    /// * `last` - The number of plans to return from the end of the range.
    /// * `before` - A cursor; returns plans before this position.
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
    ///         .plans
    ///         .list(
    ///             &PlansListQueryRequest {
    ///                 release_methods: vec![Some("buy_now".to_string())],
    ///                 visibilities: vec![Some("visible".to_string())],
    ///                 plan_types: vec![Some("renewal".to_string())],
    ///                 product_ids: vec![Some("prod_xxxxxxxxxxxxxx".to_string())],
    ///                 account_id: None,
    ///                 direction: None,
    ///                 order: None,
    ///                 created_before: None,
    ///                 created_after: None,
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
        request: &PlansListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPlansResponse, ApiError> {
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
                "plans",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("direction", request.direction.clone())
                    .serialize("order", request.order.clone())
                    .string_array("release_methods", request.release_methods.clone())
                    .string_array("visibilities", request.visibilities.clone())
                    .string_array("plan_types", request.plan_types.clone())
                    .string_array("product_ids", request.product_ids.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new pricing plan for a product. The plan defines the billing interval, price, and availability for customers.
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
    ///         .plans
    ///         .create(
    ///             &CreatePlansRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePlansRequest,
        options: Option<RequestOptions>,
    ) -> Result<Plan, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "plans",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing plan.
    ///
    /// # Arguments
    ///
    /// * `id` - Plan ID, prefixed `plan_`.
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
    ///     client.plans.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Plan, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, &format!("plans/{}", id), None, None, options)
            .await
    }

    /// Permanently delete a plan from a product. Existing memberships on this plan will not be affected.
    ///
    /// # Arguments
    ///
    /// * `id` - Plan ID, prefixed `plan_`.
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
    ///     client.plans.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeletePlansResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("plans/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a plan's pricing, billing interval, visibility, stock, and other settings.
    ///
    /// # Arguments
    ///
    /// * `id` - Plan ID, prefixed `plan_`.
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
    ///         .plans
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdatePlansRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        id: &str,
        request: &UpdatePlansRequest,
        options: Option<RequestOptions>,
    ) -> Result<Plan, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("plans/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Previews tax for a plan before checkout, based on the buyer's location.
    ///
    /// # Arguments
    ///
    /// * `id` - Plan ID, prefixed `plan_`.
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
    ///         .plans
    ///         .calculate_tax(
    ///             &"id".to_string(),
    ///             &CalculateTaxPlansRequest {
    ///                 address: Some(CalculateTaxPlansRequestAddress {
    ///                     country: "DE".to_string(),
    ///                     postal_code: Some("10115".to_string()),
    ///                     ..Default::default()
    ///                 }),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn calculate_tax(
        &self,
        id: &str,
        request: &CalculateTaxPlansRequest,
        options: Option<RequestOptions>,
    ) -> Result<CalculateTaxPlansResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("plans/{}/calculate_tax", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
