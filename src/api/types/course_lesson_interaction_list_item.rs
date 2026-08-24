pub use crate::prelude::*;

/// A record of a user's progress on a specific lesson, tracking whether they have completed it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseLessonInteractionListItem {
    /// Whether the user has finished this lesson.
    #[serde(default)]
    pub completed: bool,
    /// The datetime the lesson interaction was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The unique identifier for the lesson interaction.
    #[serde(default)]
    pub id: String,
    /// The lesson that this progress record belongs to.
    #[serde(default)]
    pub lesson: CourseLessonInteractionListItemLesson,
    /// The user whose progress is being tracked.
    #[serde(default)]
    pub user: CourseLessonInteractionListItemUser,
}

impl CourseLessonInteractionListItem {
    pub fn builder() -> CourseLessonInteractionListItemBuilder {
        <CourseLessonInteractionListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonInteractionListItemBuilder {
    completed: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    lesson: Option<CourseLessonInteractionListItemLesson>,
    user: Option<CourseLessonInteractionListItemUser>,
}

impl CourseLessonInteractionListItemBuilder {
    pub fn completed(mut self, value: bool) -> Self {
        self.completed = Some(value);
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

    pub fn lesson(mut self, value: CourseLessonInteractionListItemLesson) -> Self {
        self.lesson = Some(value);
        self
    }

    pub fn user(mut self, value: CourseLessonInteractionListItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonInteractionListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completed`](CourseLessonInteractionListItemBuilder::completed)
    /// - [`created_at`](CourseLessonInteractionListItemBuilder::created_at)
    /// - [`id`](CourseLessonInteractionListItemBuilder::id)
    /// - [`lesson`](CourseLessonInteractionListItemBuilder::lesson)
    /// - [`user`](CourseLessonInteractionListItemBuilder::user)
    pub fn build(self) -> Result<CourseLessonInteractionListItem, BuildError> {
        Ok(CourseLessonInteractionListItem {
            completed: self
                .completed
                .ok_or_else(|| BuildError::missing_field("completed"))?,
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
