pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCourseChaptersRequest {
    /// The unique identifier of the course to create the chapter in (e.g., "course_XXXXX").
    #[serde(default)]
    pub course_id: String,
    /// The display title of the chapter (e.g., "Module 1: Introduction").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreateCourseChaptersRequest {
    pub fn builder() -> CreateCourseChaptersRequestBuilder {
        <CreateCourseChaptersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCourseChaptersRequestBuilder {
    course_id: Option<String>,
    title: Option<String>,
}

impl CreateCourseChaptersRequestBuilder {
    pub fn course_id(mut self, value: impl Into<String>) -> Self {
        self.course_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCourseChaptersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`course_id`](CreateCourseChaptersRequestBuilder::course_id)
    pub fn build(self) -> Result<CreateCourseChaptersRequest, BuildError> {
        Ok(CreateCourseChaptersRequest {
            course_id: self
                .course_id
                .ok_or_else(|| BuildError::missing_field("course_id"))?,
            title: self.title,
        })
    }
}
