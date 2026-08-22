pub use crate::prelude::*;

/// Input for a single question's answer in an assessment submission
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitAssessmentCourseLessonsRequestAnswersItem {
    /// The text answer provided by the user (for short answer questions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_text: Option<String>,
    /// The ID of the question being answered
    #[serde(default)]
    pub question_id: String,
    /// The IDs of the selected options (for multiple choice/select questions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_option_ids: Option<Vec<String>>,
}

impl SubmitAssessmentCourseLessonsRequestAnswersItem {
    pub fn builder() -> SubmitAssessmentCourseLessonsRequestAnswersItemBuilder {
        <SubmitAssessmentCourseLessonsRequestAnswersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitAssessmentCourseLessonsRequestAnswersItemBuilder {
    answer_text: Option<String>,
    question_id: Option<String>,
    selected_option_ids: Option<Vec<String>>,
}

impl SubmitAssessmentCourseLessonsRequestAnswersItemBuilder {
    pub fn answer_text(mut self, value: impl Into<String>) -> Self {
        self.answer_text = Some(value.into());
        self
    }

    pub fn question_id(mut self, value: impl Into<String>) -> Self {
        self.question_id = Some(value.into());
        self
    }

    pub fn selected_option_ids(mut self, value: Vec<String>) -> Self {
        self.selected_option_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitAssessmentCourseLessonsRequestAnswersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`question_id`](SubmitAssessmentCourseLessonsRequestAnswersItemBuilder::question_id)
    pub fn build(self) -> Result<SubmitAssessmentCourseLessonsRequestAnswersItem, BuildError> {
        Ok(SubmitAssessmentCourseLessonsRequestAnswersItem {
            answer_text: self.answer_text,
            question_id: self
                .question_id
                .ok_or_else(|| BuildError::missing_field("question_id"))?,
            selected_option_ids: self.selected_option_ids,
        })
    }
}
