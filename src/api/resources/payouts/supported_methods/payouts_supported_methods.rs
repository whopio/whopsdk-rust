use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct SupportedMethodsClient {
    pub http_client: HttpClient,
}

impl SupportedMethodsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the payout methods an account or user is eligible to add. Pass a result's ID as `supported_payout_method_id` to `POST /payouts/methods` to save one.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The owning account ID (a biz_ identifier). Provide this or user_id.
    /// * `user_id` - The owning user ID (a user_ identifier). Provide this or account_id.
    /// * `country` - ISO 3166-1 alpha-2 country code for the bank account or wallet, such as `US`. Defaults to the country of supported_payout_method_id when one is given, otherwise the payout account's country.
    /// * `amount` - Optional payout amount in whole currency units, for example `250.00`. When provided, each destination includes per-currency fee and delivery quotes.
    /// * `currency` - Currency code of the amount, for example `usd`. Only meaningful with amount.
    /// * `supported_payout_method_id` - Narrows the list to one supported payout method (a podst_ identifier) and includes the required_fields needed to save it as a payout method.
    /// * `destination_currency` - Currency the supported payout method would deliver payouts in. Only meaningful with supported_payout_method_id; required fields vary by destination currency.
    /// * `party_type` - Who will own the payout method. `first_party` is the account itself. `third_party` is a recipient the account pays, saved with `recipient` on `POST /payouts/methods`, which needs no identity fields. Only meaningful with supported_payout_method_id.
    /// * `include_individual_banks` - Whether to list each bank as its own payout method in countries where people choose a bank by name. Each bank has `parent_id` set to the bank deposit method that lists it. When `false`, these countries list one bank deposit method instead.
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
    ///         .payouts
    ///         .supported_methods
    ///         .list(
    ///             &PayoutsSupportedMethodsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PayoutsSupportedMethodsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSupportedMethodsResponse, ApiError> {
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
                "payouts/supported_methods",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .string("country", request.country.clone())
                    .float("amount", request.amount.clone())
                    .string("currency", request.currency.clone())
                    .string(
                        "supported_payout_method_id",
                        request.supported_payout_method_id.clone(),
                    )
                    .string("destination_currency", request.destination_currency.clone())
                    .serialize("party_type", request.party_type.clone())
                    .bool(
                        "include_individual_banks",
                        request.include_individual_banks.clone(),
                    )
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
        request: &PayoutsSupportedMethodsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .string("user_id", request.user_id.clone())
            .string("country", request.country.clone())
            .float("amount", request.amount.clone())
            .string("currency", request.currency.clone())
            .string(
                "supported_payout_method_id",
                request.supported_payout_method_id.clone(),
            )
            .string("destination_currency", request.destination_currency.clone())
            .serialize("party_type", request.party_type.clone())
            .bool(
                "include_individual_banks",
                request.include_individual_banks.clone(),
            )
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
                            "payouts/supported_methods",
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
}
