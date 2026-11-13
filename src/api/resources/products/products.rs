use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct ProductsClient {
    pub http_client: HttpClient,
}

impl ProductsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists an account's products, or searches the public marketplace when you omit `account_id`. Without the `access_pass:basic:read` permission on the account, including when signed out, the list holds only the products its store page lists.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account to list products for, prefixed `biz_`. Without the `access_pass:basic:read` permission on it, only the products its store page lists are returned. Omit to search the public marketplace.
    /// * `query` - Filters products by text. The public marketplace list searches product title, account name, headline and description, ranking products that match every word first and the closest partial matches after them; an account's own list is a plain substring match over title, headline and description. Omit to browse by recency.
    /// * `marketplace_category_route` - Only return marketplace products assigned to this category route, such as `trading`.
    /// * `billing_types` - Filter to products with a buyable variant of these billing models, such as `one_time` or `renewal`. On an account list, needs the `access_pass:basic:read` permission.
    /// * `price_minimum` - Only return products whose advertised buyable variant has a displayed price of at least this amount. Recurring variants use renewal price. On an account list, needs the `access_pass:basic:read` permission.
    /// * `price_maximum` - Only return products whose advertised buyable variant has a displayed price of at most this amount. Recurring variants use renewal price. On an account list, needs the `access_pass:basic:read` permission.
    /// * `visibilities` - Filter to only products matching these visibility states. Ignored on the public marketplace list, which only returns visible products. Without the `access_pass:basic:read` permission on the account, only `visible` matches anything.
    /// * `access_pass_types` - Filter to only products matching these types.
    /// * `labels` - Filter to only products carrying all of these labels. Labels are matched lowercased.
    /// * `direction` - The sort direction for results. Defaults to descending.
    /// * `order` - Field to sort results by. Account lists default to `created_at`, and `position` with `direction` `asc` matches the order the store page shows. Without the `access_pass:basic:read` permission, an account list sorts only by `created_at` or `position`. Marketplace lists default to `discoverable_at`, accept only `created_at` or `discoverable_at`, and can't be combined with `query`.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `include_total_count` - Set to `true` to return the number of matching results in `page_info.total_count`. Send `first=0` as well to return the count without any results.
    /// * `created_after` - Only return products created after this ISO 8601 timestamp.
    /// * `created_before` - Only return products created before this ISO 8601 timestamp.
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
    ///         .products
    ///         .list(
    ///             &ProductsListQueryRequest {
    ///                 visibilities: vec![Some("visible".to_string())],
    ///                 access_pass_types: vec![Some("regular".to_string())],
    ///                 account_id: None,
    ///                 query: None,
    ///                 marketplace_category_route: None,
    ///                 billing_types: vec![],
    ///                 price_minimum: None,
    ///                 price_maximum: None,
    ///                 labels: vec![],
    ///                 direction: None,
    ///                 order: None,
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///                 include_total_count: None,
    ///                 created_after: None,
    ///                 created_before: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ProductsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListProductsResponse, ApiError> {
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
                "products",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .structured_query("query", request.query.clone())
                    .string(
                        "marketplace_category_route",
                        request.marketplace_category_route.clone(),
                    )
                    .serialize_array("billing_types", request.billing_types.clone())
                    .float("price_minimum", request.price_minimum.clone())
                    .float("price_maximum", request.price_maximum.clone())
                    .string_array("visibilities", request.visibilities.clone())
                    .string_array("access_pass_types", request.access_pass_types.clone())
                    .string_array("labels", request.labels.clone())
                    .serialize("direction", request.direction.clone())
                    .serialize("order", request.order.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .bool("include_total_count", request.include_total_count.clone())
                    .string("created_after", request.created_after.clone())
                    .string("created_before", request.created_before.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &ProductsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .structured_query("query", request.query.clone())
            .string(
                "marketplace_category_route",
                request.marketplace_category_route.clone(),
            )
            .serialize_array("billing_types", request.billing_types.clone())
            .float("price_minimum", request.price_minimum.clone())
            .float("price_maximum", request.price_maximum.clone())
            .string_array("visibilities", request.visibilities.clone())
            .string_array("access_pass_types", request.access_pass_types.clone())
            .string_array("labels", request.labels.clone())
            .serialize("direction", request.direction.clone())
            .serialize("order", request.order.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .bool("include_total_count", request.include_total_count.clone())
            .string("created_after", request.created_after.clone())
            .string("created_before", request.created_before.clone())
            .build();
        let options_clone = options.clone();

        AsyncPaginator::new(
            http_client,
            move |client, cursor_value| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();
                if let Some(cursor) = cursor_value {
                    // Add cursor parameter based on pagination configuration
                    query_params.push(("after".to_string(), cursor));
                }
                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            "products",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction using pagination configuration
                    let items: Vec<serde_json::Value> = response
                        .get("data")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let next_cursor: Option<String> = response
                        .get("page_info")
                        .and_then(|v| v.get("end_cursor"))
                        .and_then(|v| v.as_str().map(|s| s.to_string()));
                    let has_next_page = next_cursor.is_some();

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with no cursor
        )
    }

    /// Creates a new product for an account.
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
    ///         .products
    ///         .create(
    ///             &CreateProductsRequest {
    ///                 title: "Interior Deep Clean".to_string(),
    ///                 account_id: None,
    ///                 collect_shipping_address: None,
    ///                 custom_cta: None,
    ///                 custom_cta_url: None,
    ///                 custom_statement_descriptor: None,
    ///                 description: None,
    ///                 gallery_images: None,
    ///                 global_affiliate_percentage: None,
    ///                 global_affiliate_status: None,
    ///                 headline: None,
    ///                 labels: None,
    ///                 member_affiliate_percentage: None,
    ///                 member_affiliate_status: None,
    ///                 metadata: None,
    ///                 product_tax_code_id: None,
    ///                 redirect_purchase_url: None,
    ///                 route: None,
    ///                 send_welcome_message: None,
    ///                 visibility: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateProductsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Product, ApiError> {
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
                Method::POST,
                "products",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a product. Requires no authentication.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product.
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
    ///     client.products.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Product, ApiError> {
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
                &format!("products/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a product. Only products with no memberships, entries, reviews, or invoices can be deleted.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product.
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
    ///     client.products.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteProductsResponse, ApiError> {
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
                Method::DELETE,
                &format!("products/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an existing product.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product.
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
    ///         .products
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateProductsRequest {
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
        request: &UpdateProductsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Product, ApiError> {
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
                Method::PATCH,
                &format!("products/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Submits an App Store app's product for review. The product moves to `pending_review`; a Whop reviewer approves it before it goes live. Other products can no longer be listed on the whop.com marketplace, so the request fails with `discover_listing_discontinued`.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product, prefixed `prod_`.
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
    ///     client.products.publish(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn publish(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Product, ApiError> {
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
                Method::POST,
                &format!("products/{}/publish", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Removes a product from the whop.com marketplace. The product moves to `not_available`.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product, prefixed `prod_`.
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
    ///     client.products.unpublish(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn unpublish(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Product, ApiError> {
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
                Method::POST,
                &format!("products/{}/unpublish", id),
                None,
                None,
                options,
            )
            .await
    }
}
