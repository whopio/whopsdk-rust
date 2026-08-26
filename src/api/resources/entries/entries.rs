use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EntriesClient {
    pub http_client: HttpClient,
}

impl EntriesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of waitlist entries for a company, with optional filtering by product, plan, status, and creation date.
    ///
    /// Required permissions:
    /// - `plan:waitlist:read`
    /// - `member:email:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `company_id` - The unique identifier of the company to list waitlist entries for.
    /// * `product_ids` - Filter entries to only those for specific products.
    /// * `plan_ids` - Filter entries to only those for specific plans.
    /// * `statuses` - Filter entries by their current status.
    /// * `created_before` - Only return entries created before this timestamp.
    /// * `created_after` - Only return entries created after this timestamp.
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
    ///         .entries
    ///         .list(
    ///             &EntriesListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 after: None,
    ///                 before: None,
    ///                 direction: None,
    ///                 order: None,
    ///                 product_ids: vec![],
    ///                 plan_ids: vec![],
    ///                 statuses: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &EntriesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEntriesResponse, ApiError> {
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
                "entries",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("company_id", request.company_id.clone())
                    .serialize("direction", request.direction.clone())
                    .serialize("order", request.order.clone())
                    .string_array("product_ids", request.product_ids.clone())
                    .string_array("plan_ids", request.plan_ids.clone())
                    .serialize_array("statuses", request.statuses.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the details of an existing waitlist entry.
    ///
    /// Required permissions:
    /// - `plan:waitlist:read`
    /// - `member:email:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the waitlist entry to retrieve.
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
    ///         .entries
    ///         .retrieve(&"entry_xxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Entry, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, &format!("entries/{}", id), None, None, options)
            .await
    }

    /// Approve a pending waitlist entry, triggering the checkout process to grant the user access to the plan.
    ///
    /// Required permissions:
    /// - `plan:waitlist:manage`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the waitlist entry to approve.
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
    ///         .entries
    ///         .approve(&"entry_xxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn approve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ApproveEntriesResponse, ApiError> {
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
                &format!("entries/{}/approve", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deny a pending waitlist entry, preventing the user from gaining access to the plan.
    ///
    /// Required permissions:
    /// - `plan:waitlist:manage`
    /// - `plan:basic:read`
    /// - `member:email:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the waitlist entry to deny.
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
    ///         .entries
    ///         .deny(&"entry_xxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn deny(&self, id: &str, options: Option<RequestOptions>) -> Result<Entry, ApiError> {
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
                &format!("entries/{}/deny", id),
                None,
                None,
                options,
            )
            .await
    }
}
