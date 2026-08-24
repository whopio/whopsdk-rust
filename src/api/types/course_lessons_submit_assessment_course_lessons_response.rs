pub use crate::prelude::*;

/// The result of a user's assessment attempt
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubmitAssessmentCourseLessonsResponse {
    /// The datetime the assessment result was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The unique identifier for the assessment result.
    #[serde(default)]
    pub id: String,
    /// The lesson this assessment result is for
    #[serde(default)]
    pub lesson: SubmitAssessmentCourseLessonsResponseLesson,
    /// The number of correct answers
    #[serde(default)]
    pub result_correct: i64,
    /// The grade achieved on the assessment
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub result_grade: f64,
    /// Array of graded questions with details
    #[serde(default)]
    pub result_graded_questions: HashMap<String, serde_json::Value>,
    /// Whether the user achieved a passing grade
    #[serde(default)]
    pub result_passing_grade: bool,
    /// The total number of questions in the assessment
    #[serde(default)]
    pub result_question_count: i64,
    /// The percentage score achieved on the assessment
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub score_percent: f64,
    /// The datetime the assessment result was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The user who took the assessment
    #[serde(default)]
    pub user: SubmitAssessmentCourseLessonsResponseUser,
}

impl SubmitAssessmentCourseLessonsResponse {
    pub fn builder() -> SubmitAssessmentCourseLessonsResponseBuilder {
        <SubmitAssessmentCourseLessonsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitAssessmentCourseLessonsResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    lesson: Option<SubmitAssessmentCourseLessonsResponseLesson>,
    result_correct: Option<i64>,
    result_grade: Option<f64>,
    result_graded_questions: Option<HashMap<String, serde_json::Value>>,
    result_passing_grade: Option<bool>,
    result_question_count: Option<i64>,
    score_percent: Option<f64>,
    updated_at: Option<DateTime<FixedOffset>>,
    user: Option<SubmitAssessmentCourseLessonsResponseUser>,
}

impl SubmitAssessmentCourseLessonsResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lesson(mut self, value: SubmitAssessmentCourseLessonsResponseLesson) -> Self {
        self.lesson = Some(value);
        self
    }

    pub fn result_correct(mut self, value: i64) -> Self {
        self.result_correct = Some(value);
        self
    }

    pub fn result_grade(mut self, value: f64) -> Self {
        self.result_grade = Some(value);
        self
    }

    pub fn result_graded_questions(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.result_graded_questions = Some(value);
        self
    }

    pub fn result_passing_grade(mut self, value: bool) -> Self {
        self.result_passing_grade = Some(value);
        self
    }

    pub fn result_question_count(mut self, value: i64) -> Self {
        self.result_question_count = Some(value);
        self
    }

    pub fn score_percent(mut self, value: f64) -> Self {
        self.score_percent = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn user(mut self, value: SubmitAssessmentCourseLessonsResponseUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitAssessmentCourseLessonsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](SubmitAssessmentCourseLessonsResponseBuilder::created_at)
    /// - [`id`](SubmitAssessmentCourseLessonsResponseBuilder::id)
    /// - [`lesson`](SubmitAssessmentCourseLessonsResponseBuilder::lesson)
    /// - [`result_correct`](SubmitAssessmentCourseLessonsResponseBuilder::result_correct)
    /// - [`result_grade`](SubmitAssessmentCourseLessonsResponseBuilder::result_grade)
    /// - [`result_graded_questions`](SubmitAssessmentCourseLessonsResponseBuilder::result_graded_questions)
    /// - [`result_passing_grade`](SubmitAssessmentCourseLessonsResponseBuilder::result_passing_grade)
    /// - [`result_question_count`](SubmitAssessmentCourseLessonsResponseBuilder::result_question_count)
    /// - [`score_percent`](SubmitAssessmentCourseLessonsResponseBuilder::score_percent)
    /// - [`updated_at`](SubmitAssessmentCourseLessonsResponseBuilder::updated_at)
    /// - [`user`](SubmitAssessmentCourseLessonsResponseBuilder::user)
    pub fn build(self) -> Result<SubmitAssessmentCourseLessonsResponse, BuildError> {
        Ok(SubmitAssessmentCourseLessonsResponse {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            lesson: self
                .lesson
                .ok_or_else(|| BuildError::missing_field("lesson"))?,
            result_correct: self
                .result_correct
                .ok_or_else(|| BuildError::missing_field("result_correct"))?,
            result_grade: self
                .result_grade
                .ok_or_else(|| BuildError::missing_field("result_grade"))?,
            result_graded_questions: self
                .result_graded_questions
                .ok_or_else(|| BuildError::missing_field("result_graded_questions"))?,
            result_passing_grade: self
                .result_passing_grade
                .ok_or_else(|| BuildError::missing_field("result_passing_grade"))?,
            result_question_count: self
                .result_question_count
                .ok_or_else(|| BuildError::missing_field("result_question_count"))?,
            score_percent: self
                .score_percent
                .ok_or_else(|| BuildError::missing_field("score_percent"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
