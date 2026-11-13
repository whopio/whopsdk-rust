use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct VariantsClient {
    pub http_client: HttpClient,
}

impl VariantsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists an account's variants, leaving out variants generated for a legacy invoice; retrieve one of those by ID. To list a product's public, buyable variants without authentication, omit `account_id` and pass `product_ids`.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The unique identifier of the account to list variants for. Required unless `product_ids` is provided for a public product-variant read.
    /// * `direction` - The sort direction for results. Defaults to descending.
    /// * `order` - The field to sort results by. Defaults to created_at.
    /// * `release_methods` - Filter to only variants matching these release methods.
    /// * `visibilities` - Filter to only variants matching these visibility states.
    /// * `billing_types` - Filter to only variants matching these billing types.
    /// * `product_ids` - Filter to only variants belonging to these product identifiers. When `account_id` is omitted, this is required and the response is publicly readable: only visible, non-invoice variants are returned.
    /// * `created_before` - Only return variants created before this timestamp.
    /// * `created_after` - Only return variants created after this timestamp.
    /// * `presentment_currency` - The currency to state each variant's prices in. Omit it, or send null, for each variant's own currency. `auto` states them in the currency of the country Whop places the buyer in: by `ip_address` when your server sends one, and by the request's own IP address otherwise. To choose that country instead, send `presentment_country` in place of this. A three-letter ISO 4217 code, such as `eur`, states them in that currency. A variant converts only where a payment quote for it would be priced in that currency, and stays in its own currency otherwise: a renewing variant, one with adaptive pricing off, a currency no payment method can collect, or `auto` for a buyer Whop cannot place in a country. Converted prices are an estimate at the current exchange rate. Nothing is held: the payment quote the buyer pays from fixes the rate.
    /// * `ip_address` - The buyer's IP address, when your server reads on their behalf. Needs an API key; any other caller is placed by its own request. Where `presentment_currency` `auto` finds the buyer's local currency, and where a payment method must be able to collect it.
    /// * `presentment_country` - The country to state each variant's prices for, as an ISO 3166-1 alpha-2 code such as `JP`, when the buyer chose one: a store's country picker. States them in the currency `auto` would for a buyer in that country, so send it instead of `presentment_currency`, never with it. Whether a payment method can collect that currency is still judged where the buyer is: by `ip_address`, else by the request's own IP address. A country Whop has no currency for is ignored, and the prices are stated as `auto` would state them.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
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
    ///         .variants
    ///         .list(
    ///             &VariantsListQueryRequest {
    ///                 release_methods: vec![Some("buy_now".to_string())],
    ///                 visibilities: vec![Some("visible".to_string())],
    ///                 billing_types: vec![Some("renewal".to_string())],
    ///                 product_ids: vec![Some("prod_xxxxxxxxxxxxxx".to_string())],
    ///                 presentment_currency: Some("auto".to_string()),
    ///                 ip_address: Some("203.0.113.7".to_string()),
    ///                 presentment_country: Some("JP".to_string()),
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
        request: &VariantsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListVariantsResponse, ApiError> {
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
                "variants",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("direction", request.direction.clone())
                    .serialize("order", request.order.clone())
                    .string_array("release_methods", request.release_methods.clone())
                    .string_array("visibilities", request.visibilities.clone())
                    .string_array("billing_types", request.billing_types.clone())
                    .string_array("product_ids", request.product_ids.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .serialize("presentment_currency", request.presentment_currency.clone())
                    .serialize("ip_address", request.ip_address.clone())
                    .serialize("presentment_country", request.presentment_country.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &VariantsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .serialize("direction", request.direction.clone())
            .serialize("order", request.order.clone())
            .string_array("release_methods", request.release_methods.clone())
            .string_array("visibilities", request.visibilities.clone())
            .string_array("billing_types", request.billing_types.clone())
            .string_array("product_ids", request.product_ids.clone())
            .string("created_before", request.created_before.clone())
            .string("created_after", request.created_after.clone())
            .serialize("presentment_currency", request.presentment_currency.clone())
            .serialize("ip_address", request.ip_address.clone())
            .serialize("presentment_country", request.presentment_country.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
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
                            "variants",
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

    /// Creates a pricing variant for a product, defining the billing interval, price, and availability customers buy it with.
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
    ///         .variants
    ///         .create(
    ///             &CreateVariantsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateVariantsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Variant, ApiError> {
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
                "variants",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a variant. Requires no authentication; fields that need a permission are `null` for callers without it.
    ///
    /// # Arguments
    ///
    /// * `id` - Variant ID, prefixed `plan_`.
    /// * `presentment_currency` - The currency to state each variant's prices in. Omit it, or send null, for each variant's own currency. `auto` states them in the currency of the country Whop places the buyer in: by `ip_address` when your server sends one, and by the request's own IP address otherwise. To choose that country instead, send `presentment_country` in place of this. A three-letter ISO 4217 code, such as `eur`, states them in that currency. A variant converts only where a payment quote for it would be priced in that currency, and stays in its own currency otherwise: a renewing variant, one with adaptive pricing off, a currency no payment method can collect, or `auto` for a buyer Whop cannot place in a country. Converted prices are an estimate at the current exchange rate. Nothing is held: the payment quote the buyer pays from fixes the rate.
    /// * `ip_address` - The buyer's IP address, when your server reads on their behalf. Needs an API key; any other caller is placed by its own request. Where `presentment_currency` `auto` finds the buyer's local currency, and where a payment method must be able to collect it.
    /// * `presentment_country` - The country to state each variant's prices for, as an ISO 3166-1 alpha-2 code such as `JP`, when the buyer chose one: a store's country picker. States them in the currency `auto` would for a buyer in that country, so send it instead of `presentment_currency`, never with it. Whether a payment method can collect that currency is still judged where the buyer is: by `ip_address`, else by the request's own IP address. A country Whop has no currency for is ignored, and the prices are stated as `auto` would state them.
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
    ///         .variants
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &VariantsRetrieveQueryRequest {
    ///                 presentment_currency: Some("auto".to_string()),
    ///                 ip_address: Some("203.0.113.7".to_string()),
    ///                 presentment_country: Some("JP".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        request: &VariantsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Variant, ApiError> {
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
                &format!("variants/{}", id),
                None,
                QueryBuilder::new()
                    .serialize("presentment_currency", request.presentment_currency.clone())
                    .serialize("ip_address", request.ip_address.clone())
                    .serialize("presentment_country", request.presentment_country.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes a variant from a product. It stops selling immediately; existing memberships on it are unaffected.
    ///
    /// # Arguments
    ///
    /// * `id` - Variant ID, prefixed `plan_`.
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
    ///     client.variants.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteVariantsResponse, ApiError> {
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
                &format!("variants/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a variant's pricing, billing interval, visibility, stock, and other settings.
    ///
    /// # Arguments
    ///
    /// * `id` - Variant ID, prefixed `plan_`.
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
    ///         .variants
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateVariantsRequest {
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
        request: &UpdateVariantsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Variant, ApiError> {
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
                &format!("variants/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deprecated. Quote tax with `POST /payment_quotes` instead. Previews tax for a variant before checkout, based on the buyer's location.
    ///
    /// # Arguments
    ///
    /// * `id` - Variant ID, prefixed `plan_`.
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
    ///         .variants
    ///         .calculate_tax(
    ///             &"id".to_string(),
    ///             &CalculateTaxVariantsRequest {
    ///                 address: Some(CalculateTaxVariantsRequestAddress {
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
        request: &CalculateTaxVariantsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CalculateTaxVariantsResponse, ApiError> {
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
                &format!("variants/{}/calculate_tax", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
