use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DomainsClient {
    pub http_client: HttpClient,
}

impl DomainsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the caller's domain claims and assignments. Filter by account, app, or lifecycle status.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Only domains belonging to this account, prefixed biz_.
    /// * `app_id` - Only domains assigned to this app, prefixed app_.
    /// * `status` - Only domains with this lifecycle status.
    /// * `order` - Field to sort by.
    /// * `direction` - Sort direction.
    /// * `first` - Number of domains from the start of the page.
    /// * `after` - Cursor for the next page.
    /// * `last` - Number of domains from the end of the page.
    /// * `before` - Cursor for the previous page.
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
    ///         .domains
    ///         .list(
    ///             &DomainsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &DomainsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDomainsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-11".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "domains",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("app_id", request.app_id.clone())
                    .serialize("status", request.status.clone())
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

    /// Creates an unverified claim and returns DNS instructions. A claim does not reserve the hostname globally. Publish its unique TXT record; ownership verification, DNS checks, and certificate provisioning run automatically. Unverified claims are deleted after 48 hours.
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
    ///         .domains
    ///         .create(
    ///             &CreateDomainsRequest {
    ///                 app_id: "app_xxxxxxxxxxxxxx".to_string(),
    ///                 domain: "store.example.com".to_string(),
    ///                 account_id: None,
    ///                 metadata: None,
    ///                 replace_existing: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateDomainsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Domain, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-11".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "domains",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the claim, app assignment, DNS instructions, and the latest hostname and certificate state. For domains still connecting, needing attention, or being deleted, requests an immediate background check.
    ///
    /// # Arguments
    ///
    /// * `id` - Domain ID, prefixed dom_.
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
    ///     client.domains.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Domain, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-11".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, &format!("domains/{}", id), None, None, options)
            .await
    }

    /// Stops resolving the domain to its app and queues Cloudflare cleanup. The response is deleting; retrieve the resource until it is removed.
    ///
    /// # Arguments
    ///
    /// * `id` - Domain ID, prefixed dom_.
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
    ///     client.domains.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Domain, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-11".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("domains/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Reassigns a domain to another app in the same account or replaces its metadata. The hostname and owning account cannot be edited.
    ///
    /// # Arguments
    ///
    /// * `id` - Domain ID, prefixed dom_.
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
    ///         .domains
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateDomainsRequest {
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
        request: &UpdateDomainsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Domain, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-11".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("domains/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
