pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitAssessmentCourseLessonsRequest {
    /// The list of answers to submit for each assessment question.
    #[serde(default)]
    pub answers: Vec<SubmitAssessmentCourseLessonsRequestAnswersItem>,
}

impl SubmitAssessmentCourseLessonsRequest {
    pub fn builder() -> SubmitAssessmentCourseLessonsRequestBuilder {
        <SubmitAssessmentCourseLessonsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitAssessmentCourseLessonsRequestBuilder {
    answers: Option<Vec<SubmitAssessmentCourseLessonsRequestAnswersItem>>,
}

impl SubmitAssessmentCourseLessonsRequestBuilder {
    pub fn answers(mut self, value: Vec<SubmitAssessmentCourseLessonsRequestAnswersItem>) -> Self {
        self.answers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitAssessmentCourseLessonsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`answers`](SubmitAssessmentCourseLessonsRequestBuilder::answers)
    pub fn build(self) -> Result<SubmitAssessmentCourseLessonsRequest, BuildError> {
        Ok(SubmitAssessmentCourseLessonsRequest {
            answers: self
                .answers
                .ok_or_else(|| BuildError::missing_field("answers"))?,
        })
    }
}
