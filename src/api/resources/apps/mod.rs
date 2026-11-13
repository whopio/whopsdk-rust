use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub mod categories;
pub use categories::CategoriesClient;
pub struct AppsClient {
    pub http_client: HttpClient,
    pub categories: CategoriesClient,
}

impl AppsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            categories: CategoriesClient::new(config.clone())?,
        })
    }

    /// Lists published apps approved for the app store without authentication. With `account_id` and developer access, also includes unpublished apps that the account owns. Public website discovery includes built official blueprints and built, live community blueprints that Whop recommends.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Only return apps created by this account (`biz_` tag). With developer access to the account this includes its unlisted and hidden apps.
    /// * `app_type` - Filter apps by the type of end-user they are built for. Apps of type `website` are left out unless you ask for them by name.
    /// * `view_type` - Only return apps supporting this view type, such as `dashboard` or `hub`.
    /// * `verified` - Only return apps whose Whop verification status matches this value. Omit this filter to include every verification status the caller can see.
    /// * `verified_apps_only` - Legacy compatibility filter. Use `verified` for field equality. `true` returns verified apps; clients pinned before `2026-08-25-2` retain the earlier public website discovery behavior.
    /// * `recommended` - Only return apps Whop recommends (or, with `false`, only those it does not), independently of verification status.
    /// * `query` - A search string matched against app names.
    /// * `product_category` - Only return apps whose `product_category` is this product category, such as `pressure_washing`. An unknown product category is rejected.
    /// * `business_type` - Only return apps whose `product_category` sits under this business type in Whop's business taxonomy, such as `services`. An unknown business type is rejected.
    /// * `industry_type` - Only return apps whose `product_category` sits under this industry type of `business_type`, such as `exterior_cleaning`. Requires `business_type`. An industry type outside it is rejected.
    /// * `marketplace_category_route` - Only return apps in this app store category. Use a `route` from `GET /apps/categories`, such as `business-productivity`. An unknown route is rejected.
    /// * `order` - The field to sort apps by. Defaults to discoverable_at, showing the most recently published apps first. `template_usage` ranks Whop-verified apps first, then by how many businesses created apps from each app as a template. `template_recent_sales` ranks them by how many of those businesses processed a payment in the last 24 hours, recounted hourly, breaking ties by `template_usage`.
    /// * `direction` - Sort direction.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `include_total_count` - Set to `true` to return the number of matching results in `page_info.total_count`. Send `first=0` as well to return the count without any results.
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
    ///         .apps
    ///         .list(
    ///             &AppsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AppsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAppsResponse, ApiError> {
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
                "apps",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("app_type", request.app_type.clone())
                    .serialize("view_type", request.view_type.clone())
                    .bool("verified", request.verified.clone())
                    .bool("verified_apps_only", request.verified_apps_only.clone())
                    .bool("recommended", request.recommended.clone())
                    .structured_query("query", request.query.clone())
                    .string("product_category", request.product_category.clone())
                    .string("business_type", request.business_type.clone())
                    .string("industry_type", request.industry_type.clone())
                    .string(
                        "marketplace_category_route",
                        request.marketplace_category_route.clone(),
                    )
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .bool("include_total_count", request.include_total_count.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &AppsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .serialize("app_type", request.app_type.clone())
            .serialize("view_type", request.view_type.clone())
            .bool("verified", request.verified.clone())
            .bool("verified_apps_only", request.verified_apps_only.clone())
            .bool("recommended", request.recommended.clone())
            .structured_query("query", request.query.clone())
            .string("product_category", request.product_category.clone())
            .string("business_type", request.business_type.clone())
            .string("industry_type", request.industry_type.clone())
            .string(
                "marketplace_category_route",
                request.marketplace_category_route.clone(),
            )
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .bool("include_total_count", request.include_total_count.clone())
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
                            "apps",
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

    /// Registers a new app on the Whop developer platform. Apps provide custom experiences that can be added to products.
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
    ///         .apps
    ///         .create(
    ///             &CreateAppsRequest {
    ///                 name: "Shine Time Booking".to_string(),
    ///                 account_id: None,
    ///                 app_type: None,
    ///                 base_url: None,
    ///                 domain_id: None,
    ///                 icon: None,
    ///                 imported_from_url: None,
    ///                 redirect_uris: None,
    ///                 route: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAppsRequest,
        options: Option<RequestOptions>,
    ) -> Result<App, ApiError> {
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
                "apps",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves an app by ID, claimed route, active verified custom hostname, or proxy domain id. Authentication is optional; credential fields stay `null` unless you have the matching developer permission on the owning account.
    ///
    /// # Arguments
    ///
    /// * `id` - App ID (prefixed `app_`). Retrieval also accepts the app's claimed route, an active verified custom hostname, or its proxy domain id; a custom hostname returns `404` when its assignment is inactive, the owning account is suspended, or the app is deleted.
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
    ///     client.apps.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<App, ApiError> {
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
                &format!("apps/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes an app. The app stops resolving within seconds — a website's site stops serving, and any claimed subdomain is reserved for a month before it can be claimed again.
    ///
    /// # Arguments
    ///
    /// * `id` - App ID (prefixed `app_`). Retrieval also accepts the app's claimed route, an active verified custom hostname, or its proxy domain id; a custom hostname returns `404` when its assignment is inactive, the owning account is suspended, or the app is deleted.
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
    ///     client.apps.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteAppsResponse, ApiError> {
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
                &format!("apps/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates the settings, metadata, or status of an app. Fields that are omitted keep their current value.
    ///
    /// # Arguments
    ///
    /// * `id` - App ID (prefixed `app_`). Retrieval also accepts the app's claimed route, an active verified custom hostname, or its proxy domain id; a custom hostname returns `404` when its assignment is inactive, the owning account is suspended, or the app is deleted.
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
    ///         .apps
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateAppsRequest {
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
        request: &UpdateAppsRequest,
        options: Option<RequestOptions>,
    ) -> Result<App, ApiError> {
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
                &format!("apps/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Builds the app's current source and ships it. Returns the run it started, so the caller can render progress from this response and then follow it on the app's `deployment` field. Only one deployment runs per app at a time — calling this while one is in flight reports that run rather than starting a second, and calling it with nothing to publish reports that instead of starting one.
    ///
    /// # Arguments
    ///
    /// * `id` - The app to deploy, prefixed `app_`.
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
    ///         .apps
    ///         .deploy(
    ///             &"id".to_string(),
    ///             &DeployAppsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn deploy(
        &self,
        id: &str,
        request: &DeployAppsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AppDeployment, ApiError> {
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
                &format!("apps/{}/deploy", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists a hosted app's server runtime logs, most recent first: console output, uncaught exceptions, and failed-request summaries captured on whop.site hosting. Logs are retained for 7 days.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the app, which will look like app_*************.
    /// * `app_build_id` - Only return logs from this build.
    /// * `level` - Only return console lines of this level.
    /// * `query` - Only return logs whose message contains this text (case-insensitive).
    /// * `created_after` - Start of the time window as an ISO 8601 timestamp. Defaults to 7 days before created_before.
    /// * `created_before` - End of the time window as an ISO 8601 timestamp. Defaults to now.
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
    ///         .apps
    ///         .logs(
    ///             &"id".to_string(),
    ///             &LogsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn logs(
        &self,
        id: &str,
        request: &LogsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LogsAppsResponse, ApiError> {
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
                &format!("apps/{}/logs", id),
                None,
                QueryBuilder::new()
                    .string("app_build_id", request.app_build_id.clone())
                    .serialize("level", request.level.clone())
                    .structured_query("query", request.query.clone())
                    .datetime("created_after", request.created_after.clone())
                    .datetime("created_before", request.created_before.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn logs_paginated(
        &self,
        id: &str,
        request: &LogsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("app_build_id", request.app_build_id.clone())
            .serialize("level", request.level.clone())
            .structured_query("query", request.query.clone())
            .datetime("created_after", request.created_after.clone())
            .datetime("created_before", request.created_before.clone())
            .int("first", request.first.clone())
            .string("before", request.before.clone())
            .build();
        let options_clone = options.clone();
        let id_clone = id.to_string();

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
                let id_for_async = id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("apps/{}/logs", id_for_async),
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

    /// Replaces the set of permissions the app requests from users when they install it. Requires a user session: the `developer:update_app_authorization` scope cannot be delegated to API keys. Sensitive permissions require step-up verification.
    ///
    /// # Arguments
    ///
    /// * `id` - App ID, prefixed `app_`.
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
    ///         .apps
    ///         .update_permissions(
    ///             &"id".to_string(),
    ///             &UpdatePermissionsAppsRequest {
    ///                 requested_permissions: vec![UpdatePermissionsAppsRequestRequestedPermissionsItem {
    ///                     action: "company:basic:read".to_string(),
    ///                     is_required: true,
    ///                     justification: "Reads basic account info to render the dashboard home."
    ///                         .to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_permissions(
        &self,
        id: &str,
        request: &UpdatePermissionsAppsRequest,
        options: Option<RequestOptions>,
    ) -> Result<App, ApiError> {
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
                &format!("apps/{}/permissions", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
