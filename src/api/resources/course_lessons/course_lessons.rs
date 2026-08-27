use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CourseLessonsClient {
    pub http_client: HttpClient,
}

impl CourseLessonsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of lessons within a course or chapter, ordered by position.
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
    /// * `course_id` - The unique identifier of the course to return all lessons across all chapters.
    /// * `chapter_id` - The unique identifier of a chapter to return only its lessons.
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
    ///         .course_lessons
    ///         .list(
    ///             &CourseLessonsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 course_id: Some("cors_xxxxxxxxxxxxx".to_string()),
    ///                 chapter_id: Some("chap_xxxxxxxxxxxxx".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &CourseLessonsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCourseLessonsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "course_lessons",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("course_id", request.course_id.clone())
                    .string("chapter_id", request.chapter_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new lesson within a course chapter. Lessons can contain video, text, or assessment content.
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
    ///         .course_lessons
    ///         .create(
    ///             &CreateCourseLessonsRequest {
    ///                 chapter_id: "chap_xxxxxxxxxxxxx".to_string(),
    ///                 lesson_type: LessonTypes::Text,
    ///                 content: None,
    ///                 days_from_course_start_until_unlock: None,
    ///                 embed_id: None,
    ///                 embed_type: None,
    ///                 thumbnail: None,
    ///                 title: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateCourseLessonsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CourseLesson, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "course_lessons",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing course lesson.
    ///
    /// Required permissions:
    /// - `courses:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the lesson to retrieve.
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
    ///         .course_lessons
    ///         .retrieve(&"lesn_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CourseLesson, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("course_lessons/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently delete a lesson and remove it from its chapter.
    ///
    /// Required permissions:
    /// - `courses:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the lesson to delete (e.g., "les_XXXXX").
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
    ///         .course_lessons
    ///         .delete(&"lesn_xxxxxxxxxxxxx".to_string(), None)
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
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("course_lessons/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a lesson's content, type, visibility, assessment questions, or media attachments.
    ///
    /// Required permissions:
    /// - `courses:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the lesson to update (e.g., "les_XXXXX").
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
    ///         .course_lessons
    ///         .update(
    ///             &"lesn_xxxxxxxxxxxxx".to_string(),
    ///             &UpdateCourseLessonsRequest {
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
        request: &UpdateCourseLessonsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CourseLesson, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("course_lessons/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Mark a lesson as completed for the current user after they finish the content.
    ///
    /// # Arguments
    ///
    /// * `lesson_id` - The unique identifier of the lesson to mark as completed (e.g., "les_XXXXX").
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
    ///         .course_lessons
    ///         .mark_as_completed(&"lesson_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn mark_as_completed(
        &self,
        lesson_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("course_lessons/{}/mark_as_completed", lesson_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Record that the current user has started viewing a lesson, creating progress tracking records.
    ///
    /// # Arguments
    ///
    /// * `lesson_id` - The unique identifier of the lesson the user is starting (e.g., "les_XXXXX").
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
    ///         .course_lessons
    ///         .start(&"lesson_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn start(
        &self,
        lesson_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("course_lessons/{}/start", lesson_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Submit answers for a quiz or knowledge check lesson and receive a graded result.
    ///
    /// # Arguments
    ///
    /// * `lesson_id` - The unique identifier of the quiz or knowledge check lesson to submit answers for (e.g., "les_XXXXX").
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
    ///         .course_lessons
    ///         .submit_assessment(
    ///             &"lesson_id".to_string(),
    ///             &SubmitAssessmentCourseLessonsRequest {
    ///                 answers: vec![SubmitAssessmentCourseLessonsRequestAnswersItem {
    ///                     question_id: "question_id".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn submit_assessment(
        &self,
        lesson_id: &str,
        request: &SubmitAssessmentCourseLessonsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubmitAssessmentCourseLessonsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("course_lessons/{}/submit_assessment", lesson_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
