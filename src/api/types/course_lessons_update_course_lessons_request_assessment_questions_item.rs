pub use crate::prelude::*;

/// Input for creating or updating an assessment question
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateCourseLessonsRequestAssessmentQuestionsItem {
    /// The correct answer for the question. Used for short answer questions
    #[serde(default)]
    pub correct_answer: String,
    /// The ID of an existing question. If provided, the question will be updated. If not provided, a new question will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional image attachment for the question
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<UpdateCourseLessonsRequestAssessmentQuestionsItemImage>,
    /// The answer options for multiple choice/select questions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItem>>,
    /// The text of the question
    #[serde(default)]
    pub question_text: String,
    /// The type of the question
    pub question_type: CoursesAssessmentQuestionTypes,
}

impl UpdateCourseLessonsRequestAssessmentQuestionsItem {
    pub fn builder() -> UpdateCourseLessonsRequestAssessmentQuestionsItemBuilder {
        <UpdateCourseLessonsRequestAssessmentQuestionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCourseLessonsRequestAssessmentQuestionsItemBuilder {
    correct_answer: Option<String>,
    id: Option<String>,
    image: Option<UpdateCourseLessonsRequestAssessmentQuestionsItemImage>,
    options: Option<Vec<UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItem>>,
    question_text: Option<String>,
    question_type: Option<CoursesAssessmentQuestionTypes>,
}

impl UpdateCourseLessonsRequestAssessmentQuestionsItemBuilder {
    pub fn correct_answer(mut self, value: impl Into<String>) -> Self {
        self.correct_answer = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn image(mut self, value: UpdateCourseLessonsRequestAssessmentQuestionsItemImage) -> Self {
        self.image = Some(value);
        self
    }

    pub fn options(
        mut self,
        value: Vec<UpdateCourseLessonsRequestAssessmentQuestionsItemOptionsItem>,
    ) -> Self {
        self.options = Some(value);
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

    /// Consumes the builder and constructs a [`UpdateCourseLessonsRequestAssessmentQuestionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`correct_answer`](UpdateCourseLessonsRequestAssessmentQuestionsItemBuilder::correct_answer)
    /// - [`question_text`](UpdateCourseLessonsRequestAssessmentQuestionsItemBuilder::question_text)
    /// - [`question_type`](UpdateCourseLessonsRequestAssessmentQuestionsItemBuilder::question_type)
    pub fn build(self) -> Result<UpdateCourseLessonsRequestAssessmentQuestionsItem, BuildError> {
        Ok(UpdateCourseLessonsRequestAssessmentQuestionsItem {
            correct_answer: self
                .correct_answer
                .ok_or_else(|| BuildError::missing_field("correct_answer"))?,
            id: self.id,
            image: self.image,
            options: self.options,
            question_text: self
                .question_text
                .ok_or_else(|| BuildError::missing_field("question_text"))?,
            question_type: self
                .question_type
                .ok_or_else(|| BuildError::missing_field("question_type"))?,
        })
    }
}
