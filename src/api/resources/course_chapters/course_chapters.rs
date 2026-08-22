use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CourseChaptersClient {
    pub http_client: HttpClient,
}

impl CourseChaptersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of chapters within a course, ordered by position.
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
    /// * `course_id` - The unique identifier of the course to list chapters for.
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
    ///         .course_chapters
    ///         .list(
    ///             &CourseChaptersListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 course_id: "cors_xxxxxxxxxxxxx".to_string(),
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
        request: &CourseChaptersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCourseChaptersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "course_chapters",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("course_id", request.course_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new chapter within a course to organize lessons into sections.
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
    ///         .course_chapters
    ///         .create(
    ///             &CreateCourseChaptersRequest {
    ///                 course_id: "cors_xxxxxxxxxxxxx".to_string(),
    ///                 title: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateCourseChaptersRequest,
        options: Option<RequestOptions>,
    ) -> Result<CourseChapter, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "course_chapters",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing course chapter.
    ///
    /// Required permissions:
    /// - `courses:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the chapter to retrieve.
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
    ///         .course_chapters
    ///         .retrieve(&"chap_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CourseChapter, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("course_chapters/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently delete a chapter and all of its lessons from a course.
    ///
    /// Required permissions:
    /// - `courses:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the chapter to delete (e.g., "chap_XXXXX").
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
    ///         .course_chapters
    ///         .delete(&"chap_xxxxxxxxxxxxx".to_string(), None)
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
                &format!("course_chapters/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a chapter's title within a course.
    ///
    /// Required permissions:
    /// - `courses:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the chapter to update (e.g., "chap_XXXXX").
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
    ///         .course_chapters
    ///         .update(
    ///             &"chap_xxxxxxxxxxxxx".to_string(),
    ///             &UpdateCourseChaptersRequest {
    ///                 title: "title".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        id: &str,
        request: &UpdateCourseChaptersRequest,
        options: Option<RequestOptions>,
    ) -> Result<CourseChapter, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("course_chapters/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
