use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ExperiencesClient {
    pub http_client: HttpClient,
}

impl ExperiencesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of experiences belonging to a company, with optional filtering by product and app.
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `company_id` - The unique identifier of the company to list experiences for.
    /// * `product_id` - Filter to only experiences attached to this product identifier.
    /// * `app_id` - Filter to only experiences powered by this app identifier.
    /// * `created_before` - Only return experiences created before this timestamp.
    /// * `created_after` - Only return experiences created after this timestamp.
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
    ///         .experiences
    ///         .list(
    ///             &ExperiencesListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 product_id: Some("prod_xxxxxxxxxxxxx".to_string()),
    ///                 app_id: Some("app_xxxxxxxxxxxxxx".to_string()),
    ///                 created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 after: None,
    ///                 before: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ExperiencesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListExperiencesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "experiences",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("company_id", request.company_id.clone())
                    .string("product_id", request.product_id.clone())
                    .string("app_id", request.app_id.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Required permissions:
    /// - `experience:create`
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
    ///         .experiences
    ///         .create(
    ///             &CreateExperiencesRequest {
    ///                 app_id: "app_xxxxxxxxxxxxxx".to_string(),
    ///                 company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 is_public: None,
    ///                 logo: None,
    ///                 name: None,
    ///                 notifications_enabled: None,
    ///                 section_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateExperiencesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Experience, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "experiences",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing experience.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the experience.
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
    ///         .experiences
    ///         .retrieve(&"exp_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Experience, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("experiences/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Required permissions:
    /// - `experience:delete`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the experience to delete.
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
    ///         .experiences
    ///         .delete(&"exp_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("experiences/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Required permissions:
    /// - `experience:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the experience to update.
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
    ///         .experiences
    ///         .update(
    ///             &"exp_xxxxxxxxxxxxxx".to_string(),
    ///             &UpdateExperiencesRequest {
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
        request: &UpdateExperiencesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Experience, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("experiences/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Attach an experience to a product, making it accessible to the product's customers.
    ///
    /// Required permissions:
    /// - `experience:attach`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the experience to attach.
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
    ///         .experiences
    ///         .attach(
    ///             &"exp_xxxxxxxxxxxxxx".to_string(),
    ///             &AttachExperiencesRequest {
    ///                 product_id: "prod_xxxxxxxxxxxxx".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn attach(
        &self,
        id: &str,
        request: &AttachExperiencesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Experience, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("experiences/{}/attach", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Detach an experience from a product, removing customer access to it through that product.
    ///
    /// Required permissions:
    /// - `experience:detach`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the experience to detach.
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
    ///         .experiences
    ///         .detach(
    ///             &"exp_xxxxxxxxxxxxxx".to_string(),
    ///             &DetachExperiencesRequest {
    ///                 product_id: "prod_xxxxxxxxxxxxx".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn detach(
        &self,
        id: &str,
        request: &DetachExperiencesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Experience, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("experiences/{}/detach", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Duplicates an existing experience. The name will be copied, unless provided. The new experience will be attached to the same products as the original experience.
    /// If duplicating a Forum or Chat experience, the new experience will have the same settings as the original experience, e.g. who can post, who can comment, etc.
    /// No content, e.g. posts, messages, lessons from within the original experience will be copied.
    ///
    ///
    /// Required permissions:
    /// - `experience:create`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the experience to duplicate.
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
    ///         .experiences
    ///         .duplicate(
    ///             &"exp_xxxxxxxxxxxxxx".to_string(),
    ///             &DuplicateExperiencesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn duplicate(
        &self,
        id: &str,
        request: &DuplicateExperiencesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Experience, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("experiences/{}/duplicate", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
