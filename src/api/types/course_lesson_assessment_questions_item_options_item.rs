pub use crate::prelude::*;

/// An answer option for a multiple choice or multiple select assessment question
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseLessonAssessmentQuestionsItemOptionsItem {
    /// The unique identifier for the assessment question option.
    #[serde(default)]
    pub id: String,
    /// Whether this option is a correct answer. Only visible to admins (users with courses:update permission)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_correct: Option<bool>,
    /// The text of the answer option
    #[serde(default)]
    pub option_text: String,
    /// The order of this option within the question
    #[serde(default)]
    pub order: i64,
}

impl CourseLessonAssessmentQuestionsItemOptionsItem {
    pub fn builder() -> CourseLessonAssessmentQuestionsItemOptionsItemBuilder {
        <CourseLessonAssessmentQuestionsItemOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonAssessmentQuestionsItemOptionsItemBuilder {
    id: Option<String>,
    is_correct: Option<bool>,
    option_text: Option<String>,
    order: Option<i64>,
}

impl CourseLessonAssessmentQuestionsItemOptionsItemBuilder {
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

    pub fn order(mut self, value: i64) -> Self {
        self.order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonAssessmentQuestionsItemOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseLessonAssessmentQuestionsItemOptionsItemBuilder::id)
    /// - [`option_text`](CourseLessonAssessmentQuestionsItemOptionsItemBuilder::option_text)
    /// - [`order`](CourseLessonAssessmentQuestionsItemOptionsItemBuilder::order)
    pub fn build(self) -> Result<CourseLessonAssessmentQuestionsItemOptionsItem, BuildError> {
        Ok(CourseLessonAssessmentQuestionsItemOptionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_correct: self.is_correct,
            option_text: self
                .option_text
                .ok_or_else(|| BuildError::missing_field("option_text"))?,
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
        })
    }
}
