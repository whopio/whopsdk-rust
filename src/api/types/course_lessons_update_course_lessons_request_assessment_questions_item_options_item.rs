pub use crate::prelude::*;

/// Input for creating or updating an assessment question option
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItem {
    /// The ID of an existing option. If provided, the option will be updated. If not provided, a new option will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether this option is a correct answer
    #[serde(default)]
    pub is_correct: bool,
    /// The text of the answer option
    #[serde(default)]
    pub option_text: String,
}

impl UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItem {
    pub fn builder() -> UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItemBuilder {
        <UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItemBuilder {
    id: Option<String>,
    is_correct: Option<bool>,
    option_text: Option<String>,
}

impl UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_correct(mut self, value: bool) -> Self {
        self.is_correct = Some(value);
        self
    }

    pub fn option_text(mut self, value: impl Into<String>) -> Self {
        self.option_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_correct`](UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItemBuilder::is_correct)
    /// - [`option_text`](UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItemBuilder::option_text)
    pub fn build(
        self,
    ) -> Result<UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItem, BuildError> {
        Ok(
            UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItem {
                id: self.id,
                is_correct: self
                    .is_correct
                    .ok_or_else(|| BuildError::missing_field("is_correct"))?,
                option_text: self
                    .option_text
                    .ok_or_else(|| BuildError::missing_field("option_text"))?,
            },
        )
    }
}
