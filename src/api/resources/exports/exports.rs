use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ExportsClient {
    pub http_client: HttpClient,
}

impl ExportsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the exports requested for an account, newest first. Only exports of resources the credential is allowed to export are returned.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account to list exports for, prefixed `biz_`. Defaults to the credential's account.
    /// * `resource` - Only return exports of this resource.
    /// * `status` - Only return exports in this status.
    /// * `created_after` - Only return exports created at or after this ISO 8601 timestamp.
    /// * `created_before` - Only return exports created at or before this ISO 8601 timestamp.
    /// * `order` - The field to sort by.
    /// * `direction` - The sort direction.
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
    ///         .exports
    ///         .list(
    ///             &ExportsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ExportsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListExportsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "exports",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("resource", request.resource.clone())
                    .serialize("status", request.status.clone())
                    .string("created_after", request.created_after.clone())
                    .string("created_before", request.created_before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Starts an asynchronous CSV export of a resource for an account. Returns the export in `pending`; poll `GET /exports/{id}` until `download_url` is set.
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
    ///         .exports
    ///         .create(
    ///             &CreateExportsRequest {
    ///                 resource: CreateExportsRequestResource::AdCampaigns,
    ///                 account_id: None,
    ///                 columns: None,
    ///                 filters: None,
    ///                 timezone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateExportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Export, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "exports",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches an export's status and, once complete, its download link.
    ///
    /// # Arguments
    ///
    /// * `id` - The export ID, prefixed `exprt_`.
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
    ///     client.exports.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Export, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, &format!("exports/{}", id), None, None, options)
            .await
    }
}
