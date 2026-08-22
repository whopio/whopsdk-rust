pub use crate::prelude::*;

/// The lesson the current user should continue from: their first incomplete lesson, or the first lesson when they have finished the course, have not started it, or can edit it. Null if the course has no lessons.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseListItemResumeLesson {
    /// The unique identifier for the lesson.
    #[serde(default)]
    pub id: String,
}

impl CourseListItemResumeLesson {
    pub fn builder() -> CourseListItemResumeLessonBuilder {
        <CourseListItemResumeLessonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseListItemResumeLessonBuilder {
    id: Option<String>,
}

impl CourseListItemResumeLessonBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseListItemResumeLesson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseListItemResumeLessonBuilder::id)
    pub fn build(self) -> Result<CourseListItemResumeLesson, BuildError> {
        Ok(CourseListItemResumeLesson {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
