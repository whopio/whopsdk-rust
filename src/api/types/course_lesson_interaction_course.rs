pub use crate::prelude::*;

/// The course that contains the tracked lesson.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseLessonInteractionCourse {
    /// The parent experience that this course belongs to.
    #[serde(default)]
    pub experience: CourseLessonInteractionCourseExperience,
    /// The unique identifier for the course.
    #[serde(default)]
    pub id: String,
    /// The display name of the course shown to students. Null if no title has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CourseLessonInteractionCourse {
    pub fn builder() -> CourseLessonInteractionCourseBuilder {
        <CourseLessonInteractionCourseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonInteractionCourseBuilder {
    experience: Option<CourseLessonInteractionCourseExperience>,
    id: Option<String>,
    title: Option<String>,
}

impl CourseLessonInteractionCourseBuilder {
    pub fn experience(mut self, value: CourseLessonInteractionCourseExperience) -> Self {
        self.experience = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonInteractionCourse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`experience`](CourseLessonInteractionCourseBuilder::experience)
    /// - [`id`](CourseLessonInteractionCourseBuilder::id)
    pub fn build(self) -> Result<CourseLessonInteractionCourse, BuildError> {
        Ok(CourseLessonInteractionCourse {
            experience: self
                .experience
                .ok_or_else(|| BuildError::missing_field("experience"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self.title,
        })
    }
}
