pub use crate::prelude::*;

/// The primary PDF document attached to this lesson for student reference.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCourseLessonsRequestMainPdf {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateCourseLessonsRequestMainPdf {
    pub fn builder() -> UpdateCourseLessonsRequestMainPdfBuilder {
        <UpdateCourseLessonsRequestMainPdfBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCourseLessonsRequestMainPdfBuilder {
    id: Option<String>,
}

impl UpdateCourseLessonsRequestMainPdfBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCourseLessonsRequestMainPdf`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateCourseLessonsRequestMainPdfBuilder::id)
    pub fn build(self) -> Result<UpdateCourseLessonsRequestMainPdf, BuildError> {
        Ok(UpdateCourseLessonsRequestMainPdf {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
