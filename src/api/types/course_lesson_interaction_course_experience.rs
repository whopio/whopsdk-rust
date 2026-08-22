pub use crate::prelude::*;

/// The parent experience that this course belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseLessonInteractionCourseExperience {
    /// The unique identifier for the experience.
    #[serde(default)]
    pub id: String,
}

impl CourseLessonInteractionCourseExperience {
    pub fn builder() -> CourseLessonInteractionCourseExperienceBuilder {
        <CourseLessonInteractionCourseExperienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonInteractionCourseExperienceBuilder {
    id: Option<String>,
}

impl CourseLessonInteractionCourseExperienceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonInteractionCourseExperience`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseLessonInteractionCourseExperienceBuilder::id)
    pub fn build(self) -> Result<CourseLessonInteractionCourseExperience, BuildError> {
        Ok(CourseLessonInteractionCourseExperience {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
