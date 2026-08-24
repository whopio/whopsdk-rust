pub use crate::prelude::*;

/// The parent experience that this course belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseStudentCourseExperience {
    /// The unique identifier for the experience.
    #[serde(default)]
    pub id: String,
}

impl CourseStudentCourseExperience {
    pub fn builder() -> CourseStudentCourseExperienceBuilder {
        <CourseStudentCourseExperienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseStudentCourseExperienceBuilder {
    id: Option<String>,
}

impl CourseStudentCourseExperienceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseStudentCourseExperience`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseStudentCourseExperienceBuilder::id)
    pub fn build(self) -> Result<CourseStudentCourseExperience, BuildError> {
        Ok(CourseStudentCourseExperience {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
