pub use crate::prelude::*;

/// The passing criteria for quiz or knowledge check lessons, such as minimum grade or correct answers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateCourseLessonsRequestAssessmentCompletionRequirement {
    /// The minimum grade percentage required to pass (0-100). Cannot be set together with minimum_questions_correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_grade_percent: Option<f64>,
    /// The minimum number of questions that must be answered correctly. Cannot be set together with minimum_grade_percent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_questions_correct: Option<i64>,
}

impl UpdateCourseLessonsRequestAssessmentCompletionRequirement {
    pub fn builder() -> UpdateCourseLessonsRequestAssessmentCompletionRequirementBuilder {
        <UpdateCourseLessonsRequestAssessmentCompletionRequirementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCourseLessonsRequestAssessmentCompletionRequirementBuilder {
    minimum_grade_percent: Option<f64>,
    minimum_questions_correct: Option<i64>,
}

impl UpdateCourseLessonsRequestAssessmentCompletionRequirementBuilder {
    pub fn minimum_grade_percent(mut self, value: f64) -> Self {
        self.minimum_grade_percent = Some(value);
        self
    }

    pub fn minimum_questions_correct(mut self, value: i64) -> Self {
        self.minimum_questions_correct = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCourseLessonsRequestAssessmentCompletionRequirement`].
    pub fn build(
        self,
    ) -> Result<UpdateCourseLessonsRequestAssessmentCompletionRequirement, BuildError> {
        Ok(UpdateCourseLessonsRequestAssessmentCompletionRequirement {
            minimum_grade_percent: self.minimum_grade_percent,
            minimum_questions_correct: self.minimum_questions_correct,
        })
    }
}
