pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCourseChaptersRequest {
    /// The new display title of the chapter (e.g., "Module 1: Introduction").
    #[serde(default)]
    pub title: String,
}

impl UpdateCourseChaptersRequest {
    pub fn builder() -> UpdateCourseChaptersRequestBuilder {
        <UpdateCourseChaptersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCourseChaptersRequestBuilder {
    title: Option<String>,
}

impl UpdateCourseChaptersRequestBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCourseChaptersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](UpdateCourseChaptersRequestBuilder::title)
    pub fn build(self) -> Result<UpdateCourseChaptersRequest, BuildError> {
        Ok(UpdateCourseChaptersRequest {
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
