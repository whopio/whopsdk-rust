pub use crate::prelude::*;

/// The lesson the current user should continue from: their first incomplete lesson, or the first lesson when they have finished the course, have not started it, or can edit it. Null if the course has no lessons.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseResumeLesson {
    /// The unique identifier for the lesson.
    #[serde(default)]
    pub id: String,
}

impl CourseResumeLesson {
    pub fn builder() -> CourseResumeLessonBuilder {
        <CourseResumeLessonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseResumeLessonBuilder {
    id: Option<String>,
}

impl CourseResumeLessonBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseResumeLesson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseResumeLessonBuilder::id)
    pub fn build(self) -> Result<CourseResumeLesson, BuildError> {
        Ok(CourseResumeLesson {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
