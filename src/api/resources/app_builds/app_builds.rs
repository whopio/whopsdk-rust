use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AppBuildsClient {
    pub http_client: HttpClient,
}

impl AppBuildsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of build artifacts for an app, newest first, with optional platform, status, and creation-date filters.
    ///
    /// # Arguments
    ///
    /// * `app_id` - The app to list builds for, prefixed `app_`.
    /// * `platform` - Filter builds by target platform.
    /// * `status` - Filter builds by review status.
    /// * `created_before` - Only return builds created before this ISO 8601 timestamp.
    /// * `created_after` - Only return builds created after this ISO 8601 timestamp.
    /// * `first` - The number of builds to return (default 20, max 100).
    /// * `after` - A cursor; returns builds after this position.
    /// * `last` - The number of builds to return from the end of the range.
    /// * `before` - A cursor; returns builds before this position.
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
    ///         .app_builds
    ///         .list(
    ///             &AppBuildsListQueryRequest {
    ///                 app_id: "app_id".to_string(),
    ///                 platform: None,
    ///                 status: None,
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
        request: &AppBuildsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAppBuildsResponse, ApiError> {
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
                "app_builds",
                None,
                QueryBuilder::new()
                    .string("app_id", request.app_id.clone())
                    .serialize("platform", request.platform.clone())
                    .serialize("status", request.status.clone())
                    .serialize("created_before", request.created_before.clone())
                    .serialize("created_after", request.created_after.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Uploads a new build artifact for an app. Upload the file first (POST /files or a direct upload), then reference it here; iOS and Android take a .zip bundle, web takes a JavaScript file or a .zip archive of the hosted site.
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
    ///         .app_builds
    ///         .create(
    ///             &CreateAppBuildsRequest {
    ///                 attachment: CreateAppBuildsRequestAttachment {
    ///                     ..Default::default()
    ///                 },
    ///                 checksum: "xxxxxxxxxxxxxxx".to_string(),
    ///                 platform: CreateAppBuildsRequestPlatform::Ios,
    ///                 ai_prompt_id: None,
    ///                 app_id: None,
    ///                 source_attachment: None,
    ///                 supported_app_view_types: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAppBuildsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AppBuild, ApiError> {
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
                "app_builds",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing app build.
    ///
    /// # Arguments
    ///
    /// * `id` - App build ID, prefixed `abld_`.
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
    ///     client.app_builds.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AppBuild, ApiError> {
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
                &format!("app_builds/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Promotes a draft or approved app build to production so it becomes the active version served to users. Draft builds enter review first.
    ///
    /// # Arguments
    ///
    /// * `id` - App build ID, prefixed `abld_`.
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
    ///     client.app_builds.promote(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn promote(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AppBuild, ApiError> {
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
                &format!("app_builds/{}/promote", id),
                None,
                None,
                options,
            )
            .await
    }
}
