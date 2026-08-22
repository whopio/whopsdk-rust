pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseLessonInteractionsListQueryRequest {
    /// Returns the elements in the list that come after the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Returns the elements in the list that come before the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Returns the first _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Returns the last _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// The unique identifier of the user to filter lesson interactions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The unique identifier of the lesson to filter interactions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lesson_id: Option<String>,
    /// The unique identifier of the course to filter interactions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_id: Option<String>,
    /// Whether to filter for completed or in-progress lesson interactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
}

impl CourseLessonInteractionsListQueryRequest {
    pub fn builder() -> CourseLessonInteractionsListQueryRequestBuilder {
        <CourseLessonInteractionsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonInteractionsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    user_id: Option<String>,
    lesson_id: Option<String>,
    course_id: Option<String>,
    completed: Option<bool>,
}

impl CourseLessonInteractionsListQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn lesson_id(mut self, value: impl Into<String>) -> Self {
        self.lesson_id = Some(value.into());
        self
    }

    pub fn course_id(mut self, value: impl Into<String>) -> Self {
        self.course_id = Some(value.into());
        self
    }

    pub fn completed(mut self, value: bool) -> Self {
        self.completed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonInteractionsListQueryRequest`].
    pub fn build(self) -> Result<CourseLessonInteractionsListQueryRequest, BuildError> {
        Ok(CourseLessonInteractionsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            user_id: self.user_id,
            lesson_id: self.lesson_id,
            course_id: self.course_id,
            completed: self.completed,
        })
    }
}
