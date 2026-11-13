use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct EventsClient {
    pub http_client: HttpClient,
}

impl EventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists identity-linked events, most recent first by default. Pass `identifier` for one person's journey, or omit it to list an account's events within a time range. Events have the same shape as the `POST /events` intake: attribution in `context`, identity in `user`.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Any hard identifier of the person: a person ID (prsn_*), user ID, email, phone number, or a tracking cookie value (wuid, anonymous ID, fbp/fbc/ttp/ga). Omit to list recent events for the account.
    /// * `account_id` - Account ID, prefixed `biz_`. Optional for account API keys; required for credentials that can access multiple accounts.
    /// * `from` - Start of the time range as an ISO 8601 timestamp. Required when identifier is omitted.
    /// * `to` - End of the time range as an ISO 8601 timestamp. Required when identifier is omitted; otherwise defaults to now.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `direction` - The order events are returned in by time. Defaults to desc (most recent first); asc reads a journey forwards from where it starts. after and before always page forwards and backwards through that order.
    /// * `event` - Full event names to filter by, comma-separated (payment.completed, pixel.lead, pixel.page, pixel.custom:<name>) — the same vocabulary the events / people metrics use.
    /// * `source` - Canonical source path, exact or with a trailing :* prefix (whop:<campaign>:*, ext:meta:*, referrer:<domain>, direct, unknown). Selects conversions credited to this source.
    /// * `attribution_model` - Attribution model for the source filter (defaults to last_touch).
    /// * `country` - Country codes to filter by, comma-separated.
    /// * `city` - Cities to filter by, comma-separated.
    /// * `device` - Device families to filter by, comma-separated (e.g. iPhone, Mac).
    /// * `browser` - Browser families to filter by, comma-separated (e.g. Chrome, Mobile Safari).
    /// * `os` - Operating system families to filter by, comma-separated (e.g. iOS, Windows).
    /// * `utm_source` - utm_source values to filter by, comma-separated.
    /// * `hostname` - Page hostnames to filter by, comma-separated.
    /// * `page` - Page paths to filter by, comma-separated.
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
    ///         .events
    ///         .list(
    ///             &EventsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &EventsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEventsResponse, ApiError> {
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
                "events",
                None,
                QueryBuilder::new()
                    .string("identifier", request.identifier.clone())
                    .string("account_id", request.account_id.clone())
                    .datetime("from", request.from.clone())
                    .datetime("to", request.to.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .serialize("direction", request.direction.clone())
                    .string("event", request.event.clone())
                    .string("source", request.source.clone())
                    .serialize("attribution_model", request.attribution_model.clone())
                    .string("country", request.country.clone())
                    .string("city", request.city.clone())
                    .string("device", request.device.clone())
                    .string("browser", request.browser.clone())
                    .string("os", request.os.clone())
                    .string("utm_source", request.utm_source.clone())
                    .string("hostname", request.hostname.clone())
                    .string("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &EventsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("identifier", request.identifier.clone())
            .string("account_id", request.account_id.clone())
            .datetime("from", request.from.clone())
            .datetime("to", request.to.clone())
            .int("first", request.first.clone())
            .string("before", request.before.clone())
            .serialize("direction", request.direction.clone())
            .string("event", request.event.clone())
            .string("source", request.source.clone())
            .serialize("attribution_model", request.attribution_model.clone())
            .string("country", request.country.clone())
            .string("city", request.city.clone())
            .string("device", request.device.clone())
            .string("browser", request.browser.clone())
            .string("os", request.os.clone())
            .string("utm_source", request.utm_source.clone())
            .string("hostname", request.hostname.clone())
            .string("page", request.page.clone())
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
                            "events",
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

    /// Tracks a conversion or engagement event for an account, such as a lead or a sign-up, so it can be attributed to the ads and links that drove it. Send server-side events with an API key that has `event:create`; the browser pixel calls this without authentication.
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
    ///         .events
    ///         .create(
    ///             &CreateEventsRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 event_name: "coating_deposit_paid".to_string(),
    ///                 action_source: None,
    ///                 app_build_id: None,
    ///                 app_id: None,
    ///                 context: None,
    ///                 currency: None,
    ///                 custom_name: None,
    ///                 duration: None,
    ///                 event_id: None,
    ///                 event_time: None,
    ///                 product_id: None,
    ///                 referrer_url: None,
    ///                 resumed: None,
    ///                 source: None,
    ///                 title: None,
    ///                 url: None,
    ///                 user: None,
    ///                 value: None,
    ///                 variant_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateEventsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateEventsResponse, ApiError> {
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
                "events",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a fully anonymized feed of recent money movement across Whop, most recent first, such as purchases, card spend, and transfers between accounts. Each item carries only its `type`, a USD amount, a coarse location, and a timestamp coarsened to the minute. The payload is identical for every caller and requires no authentication.
    ///
    /// # Arguments
    ///
    /// * `event` - Filter to one or more types, comma separated — for example `purchase,card_spend`. These are the item's `type`, not its `event_name`: several types share the `ledger_line.created` event name. Omit for every type in the feed. Values outside the feed's own set are rejected.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
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
    ///         .events
    ///         .pulse(
    ///             &PulseQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pulse(
        &self,
        request: &PulseQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PulseEventsResponse, ApiError> {
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
                "events/pulse",
                None,
                QueryBuilder::new()
                    .string("event", request.event.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn pulse_paginated(
        &self,
        request: &PulseQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("event", request.event.clone())
            .int("first", request.first.clone())
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
                            "events/pulse",
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

    /// Checks whether the Whop pixel is installed for an account, or on one page when you pass a `url`. Use it before launching an ad to confirm its destination is tracked, or in a setup flow to tell a merchant whether their install is live.
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
    ///         .events
    ///         .validate_pixel(
    ///             &ValidatePixelEventsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn validate_pixel(
        &self,
        request: &ValidatePixelEventsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PixelValidation, ApiError> {
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
                "events/validate_pixel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
