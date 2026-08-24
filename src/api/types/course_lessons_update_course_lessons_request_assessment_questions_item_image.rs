pub use crate::prelude::*;

/// Optional image attachment for the question
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCourseLessonsRequestAssessmentQuestionsItemImage {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateCourseLessonsRequestAssessmentQuestionsItemImage {
    pub fn builder() -> UpdateCourseLessonsRequestAssessmentQuestionsItemImageBuilder {
        <UpdateCourseLessonsRequestAssessmentQuestionsItemImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCourseLessonsRequestAssessmentQuestionsItemImageBuilder {
    id: Option<String>,
}

impl UpdateCourseLessonsRequestAssessmentQuestionsItemImageBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCourseLessonsRequestAssessmentQuestionsItemImage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateCourseLessonsRequestAssessmentQuestionsItemImageBuilder::id)
    pub fn build(
        self,
    ) -> Result<UpdateCourseLessonsRequestAssessmentQuestionsItemImage, BuildError> {
        Ok(UpdateCourseLessonsRequestAssessmentQuestionsItemImage {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
