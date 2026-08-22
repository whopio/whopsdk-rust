pub use crate::prelude::*;

/// An assessment question in a course quiz or knowledge check
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CourseLessonAssessmentQuestionsItem {
    /// The correct answer for the question. Used for short answer questions. Only visible to admins (users with courses:update permission)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_answer: Option<String>,
    /// The datetime the assessment question was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The unique identifier for the assessment question.
    #[serde(default)]
    pub id: String,
    /// Optional image attachment for the question
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<CourseLessonAssessmentQuestionsItemImage>,
    /// The answer options for multiple choice/select questions
    #[serde(default)]
    pub options: Vec<CourseLessonAssessmentQuestionsItemOptionsItem>,
    /// The order of the question within its lesson
    #[serde(default)]
    pub order: i64,
    /// The text of the question
    #[serde(default)]
    pub question_text: String,
    /// The type of the question
    pub question_type: CoursesAssessmentQuestionTypes,
}

impl CourseLessonAssessmentQuestionsItem {
    pub fn builder() -> CourseLessonAssessmentQuestionsItemBuilder {
        <CourseLessonAssessmentQuestionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonAssessmentQuestionsItemBuilder {
    correct_answer: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    image: Option<CourseLessonAssessmentQuestionsItemImage>,
    options: Option<Vec<CourseLessonAssessmentQuestionsItemOptionsItem>>,
    order: Option<i64>,
    question_text: Option<String>,
    question_type: Option<CoursesAssessmentQuestionTypes>,
}

impl CourseLessonAssessmentQuestionsItemBuilder {
    pub fn correct_answer(mut self, value: impl Into<String>) -> Self {
        self.correct_answer = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn image(mut self, value: CourseLessonAssessmentQuestionsItemImage) -> Self {
        self.image = Some(value);
        self
    }

    pub fn options(mut self, value: Vec<CourseLessonAssessmentQuestionsItemOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn order(mut self, value: i64) -> Self {
        self.order = Some(value);
        self
    }

    pub fn question_text(mut self, value: impl Into<String>) -> Self {
        self.question_text = Some(value.into());
        self
    }

    pub fn question_type(mut self, value: CoursesAssessmentQuestionTypes) -> Self {
        self.question_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonAssessmentQuestionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](CourseLessonAssessmentQuestionsItemBuilder::created_at)
    /// - [`id`](CourseLessonAssessmentQuestionsItemBuilder::id)
    /// - [`options`](CourseLessonAssessmentQuestionsItemBuilder::options)
    /// - [`order`](CourseLessonAssessmentQuestionsItemBuilder::order)
    /// - [`question_text`](CourseLessonAssessmentQuestionsItemBuilder::question_text)
    /// - [`question_type`](CourseLessonAssessmentQuestionsItemBuilder::question_type)
    pub fn build(self) -> Result<CourseLessonAssessmentQuestionsItem, BuildError> {
        Ok(CourseLessonAssessmentQuestionsItem {
            correct_answer: self.correct_answer,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            image: self.image,
            options: self
                .options
                .ok_or_else(|| BuildError::missing_field("options"))?,
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
            question_text: self
                .question_text
                .ok_or_else(|| BuildError::missing_field("question_text"))?,
            question_type: self
                .question_type
                .ok_or_else(|| BuildError::missing_field("question_type"))?,
        })
    }
}
