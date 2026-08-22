pub use crate::prelude::*;

/// A record of a user's progress on a specific lesson, tracking whether they have completed it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseLessonInteraction {
    /// Whether the user has finished this lesson.
    #[serde(default)]
    pub completed: bool,
    /// The course that contains the tracked lesson.
    #[serde(default)]
    pub course: CourseLessonInteractionCourse,
    /// The datetime the lesson interaction was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The unique identifier for the lesson interaction.
    #[serde(default)]
    pub id: String,
    /// The lesson that this progress record belongs to.
    #[serde(default)]
    pub lesson: CourseLessonInteractionLesson,
    /// The user whose progress is being tracked.
    #[serde(default)]
    pub user: CourseLessonInteractionUser,
}

impl CourseLessonInteraction {
    pub fn builder() -> CourseLessonInteractionBuilder {
        <CourseLessonInteractionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonInteractionBuilder {
    completed: Option<bool>,
    course: Option<CourseLessonInteractionCourse>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    lesson: Option<CourseLessonInteractionLesson>,
    user: Option<CourseLessonInteractionUser>,
}

impl CourseLessonInteractionBuilder {
    pub fn completed(mut self, value: bool) -> Self {
        self.completed = Some(value);
        self
    }

    pub fn course(mut self, value: CourseLessonInteractionCourse) -> Self {
        self.course = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lesson(mut self, value: CourseLessonInteractionLesson) -> Self {
        self.lesson = Some(value);
        self
    }

    pub fn user(mut self, value: CourseLessonInteractionUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonInteraction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completed`](CourseLessonInteractionBuilder::completed)
    /// - [`course`](CourseLessonInteractionBuilder::course)
    /// - [`created_at`](CourseLessonInteractionBuilder::created_at)
    /// - [`id`](CourseLessonInteractionBuilder::id)
    /// - [`lesson`](CourseLessonInteractionBuilder::lesson)
    /// - [`user`](CourseLessonInteractionBuilder::user)
    pub fn build(self) -> Result<CourseLessonInteraction, BuildError> {
        Ok(CourseLessonInteraction {
            completed: self
                .completed
                .ok_or_else(|| BuildError::missing_field("completed"))?,
            course: self
                .course
                .ok_or_else(|| BuildError::missing_field("course"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            lesson: self
                .lesson
                .ok_or_else(|| BuildError::missing_field("lesson"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
