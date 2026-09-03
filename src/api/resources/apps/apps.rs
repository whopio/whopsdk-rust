use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AppsClient {
    pub http_client: HttpClient,
}

impl AppsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists apps on the Whop platform: the app store's live apps, or — with `account_id` and developer access to that account — every app the account owns. Requires authentication except for Whop's public app and website discovery lists. Public website discovery includes built official blueprints (verified apps with a product) and built, live community blueprints that Whop recommends.
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
    /// * `order` - The field to sort apps by. Defaults to discoverable_at, showing the most recently published apps first. `template_usage` ranks Whop-verified apps first, then by how many businesses created apps from each app as a template.
    /// * `direction` - Sort direction.
    /// * `first` - The number of apps to return (default 20, max 100).
    /// * `after` - A cursor; returns apps after this position.
    /// * `last` - The number of apps to return from the end of the range.
    /// * `before` - A cursor; returns apps before this position.
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
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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
    ///                 icon: None,
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
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "apps",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Updates the permission requirements for an app
    ///
    /// Required permissions:
    /// - `developer:update_app_authorization`
    ///
    /// # Arguments
    ///
    /// * `app_id` - The ID of the app the permission requirements are being updated for
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
    ///         .update_permissions_app(
    ///             &"app_id".to_string(),
    ///             &UpdatePermissionsAppRequest {
    ///                 requested_permissions: vec![UpdatePermissionsAppRequestRequestedPermissionsItem {
    ///                     action: "action".to_string(),
    ///                     is_required: true,
    ///                     justification: "justification".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_permissions_app(
        &self,
        app_id: &str,
        request: &UpdatePermissionsAppRequest,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("apps/{}/permissions", app_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves an app by ID, claimed route, or proxy domain id. Credential fields (api_key, default_api_key, secrets) render `null` unless the caller has the corresponding developer permission on the owning account.
    ///
    /// # Arguments
    ///
    /// * `id` - App ID (prefixed `app_`), the app's claimed route, or its proxy domain id.
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
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, &format!("apps/{}", id), None, None, options)
            .await
    }

    /// Deletes an app. The app stops resolving within seconds — a website's site stops serving, and any claimed subdomain is reserved for a month before it can be claimed again.
    ///
    /// # Arguments
    ///
    /// * `id` - App ID (prefixed `app_`), the app's claimed route, or its proxy domain id.
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
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::DELETE, &format!("apps/{}", id), None, None, options)
            .await
    }

    /// Updates the settings, metadata, or status of an app. Fields that are omitted keep their current value.
    ///
    /// # Arguments
    ///
    /// * `id` - App ID (prefixed `app_`), the app's claimed route, or its proxy domain id.
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
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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
    /// * `first` - The number of log lines to return (max 500).
    /// * `after` - A cursor for fetching logs after a previous page.
    /// * `before` - A cursor for fetching logs before a later page.
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
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("apps/{}/permissions", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
