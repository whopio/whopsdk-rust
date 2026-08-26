use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CoursesClient {
    pub http_client: HttpClient,
}

impl CoursesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of courses, filtered by either an experience or a company.
    ///
    /// Required permissions:
    /// - `courses:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `experience_id` - The unique identifier of the experience to list courses for.
    /// * `company_id` - The unique identifier of the company to list courses for.
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
    ///         .courses
    ///         .list(
    ///             &CoursesListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 experience_id: Some("exp_xxxxxxxxxxxxxx".to_string()),
    ///                 company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &CoursesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCoursesResponse, ApiError> {
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
                "courses",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("experience_id", request.experience_id.clone())
                    .string("company_id", request.company_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new course within an experience, with optional chapters, lessons, and a certificate.
    ///
    /// Required permissions:
    /// - `courses:update`
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
    ///         .courses
    ///         .create(
    ///             &CreateCoursesRequest {
    ///                 experience_id: "exp_xxxxxxxxxxxxxx".to_string(),
    ///                 title: "title".to_string(),
    ///                 certificate_after_completion_enabled: None,
    ///                 order: None,
    ///                 require_completing_lessons_in_order: None,
    ///                 tagline: None,
    ///                 thumbnail: None,
    ///                 visibility: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateCoursesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Course, ApiError> {
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
                "courses",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing course.
    ///
    /// Required permissions:
    /// - `courses:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the course to retrieve.
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
    ///         .courses
    ///         .retrieve(&"cors_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Course, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, &format!("courses/{}", id), None, None, options)
            .await
    }

    /// Permanently delete a course and all of its chapters, lessons, and student progress.
    ///
    /// Required permissions:
    /// - `courses:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the course to delete (e.g., "course_XXXXX").
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
    ///         .courses
    ///         .delete(&"cors_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("courses/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a course's title, description, visibility, thumbnail, or chapter ordering.
    ///
    /// Required permissions:
    /// - `courses:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the course to update (e.g., "course_XXXXX").
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
    ///         .courses
    ///         .update(
    ///             &"cors_xxxxxxxxxxxxx".to_string(),
    ///             &UpdateCoursesRequest {
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
        request: &UpdateCoursesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Course, ApiError> {
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
                &format!("courses/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
