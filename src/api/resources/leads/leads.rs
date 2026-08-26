use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LeadsClient {
    pub http_client: HttpClient,
}

impl LeadsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of leads for a company, with optional filtering by product and creation date.
    ///
    /// Required permissions:
    /// - `lead:basic:read`
    /// - `member:email:read`
    /// - `access_pass:basic:read`
    /// - `member:basic:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `company_id` - The unique identifier of the company to list leads for.
    /// * `created_after` - Only return leads created after this timestamp.
    /// * `created_before` - Only return leads created before this timestamp.
    /// * `product_ids` - Filter leads to only those associated with these specific product identifiers.
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
    ///         .leads
    ///         .list(
    ///             &LeadsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 after: None,
    ///                 before: None,
    ///                 product_ids: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &LeadsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListLeadsResponse, ApiError> {
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
                "leads",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("company_id", request.company_id.clone())
                    .datetime("created_after", request.created_after.clone())
                    .datetime("created_before", request.created_before.clone())
                    .string_array("product_ids", request.product_ids.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Record a new lead for a company, capturing a potential customer's interest in a specific product.
    ///
    /// Required permissions:
    /// - `lead:manage`
    /// - `member:email:read`
    /// - `access_pass:basic:read`
    /// - `member:basic:read`
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
    ///         .leads
    ///         .create(
    ///             &CreateLeadsRequest {
    ///                 company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 metadata: None,
    ///                 product_id: None,
    ///                 referrer: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Lead, ApiError> {
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
                "leads",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing lead.
    ///
    /// Required permissions:
    /// - `lead:basic:read`
    /// - `member:email:read`
    /// - `access_pass:basic:read`
    /// - `member:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the lead to retrieve.
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
    ///         .leads
    ///         .retrieve(&"lead_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Lead, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, &format!("leads/{}", id), None, None, options)
            .await
    }

    /// Update the metadata or referrer information on an existing lead record.
    ///
    /// Required permissions:
    /// - `lead:manage`
    /// - `member:email:read`
    /// - `access_pass:basic:read`
    /// - `member:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the lead to update, starting with 'lead_'.
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
    ///         .leads
    ///         .update(
    ///             &"lead_xxxxxxxxxxxxx".to_string(),
    ///             &UpdateLeadsRequest {
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
        request: &UpdateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Lead, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("leads/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
