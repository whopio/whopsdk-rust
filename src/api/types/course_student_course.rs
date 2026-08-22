pub use crate::prelude::*;

/// The course this student is enrolled in.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseStudentCourse {
    /// The parent experience that this course belongs to.
    #[serde(default)]
    pub experience: CourseStudentCourseExperience,
    /// The unique identifier for the course.
    #[serde(default)]
    pub id: String,
    /// The display name of the course shown to students. Null if no title has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CourseStudentCourse {
    pub fn builder() -> CourseStudentCourseBuilder {
        <CourseStudentCourseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseStudentCourseBuilder {
    experience: Option<CourseStudentCourseExperience>,
    id: Option<String>,
    title: Option<String>,
}

impl CourseStudentCourseBuilder {
    pub fn experience(mut self, value: CourseStudentCourseExperience) -> Self {
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

    /// Consumes the builder and constructs a [`CourseStudentCourse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`experience`](CourseStudentCourseBuilder::experience)
    /// - [`id`](CourseStudentCourseBuilder::id)
    pub fn build(self) -> Result<CourseStudentCourse, BuildError> {
        Ok(CourseStudentCourse {
            experience: self
                .experience
                .ok_or_else(|| BuildError::missing_field("experience"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self.title,
        })
    }
}
