pub use crate::prelude::*;

/// The lesson this assessment result is for
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitAssessmentCourseLessonsResponseLesson {
    /// The unique identifier for the lesson.
    #[serde(default)]
    pub id: String,
    /// The display name of the lesson shown to students. Maximum 120 characters.
    #[serde(default)]
    pub title: String,
}

impl SubmitAssessmentCourseLessonsResponseLesson {
    pub fn builder() -> SubmitAssessmentCourseLessonsResponseLessonBuilder {
        <SubmitAssessmentCourseLessonsResponseLessonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitAssessmentCourseLessonsResponseLessonBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl SubmitAssessmentCourseLessonsResponseLessonBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmitAssessmentCourseLessonsResponseLesson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SubmitAssessmentCourseLessonsResponseLessonBuilder::id)
    /// - [`title`](SubmitAssessmentCourseLessonsResponseLessonBuilder::title)
    pub fn build(self) -> Result<SubmitAssessmentCourseLessonsResponseLesson, BuildError> {
        Ok(SubmitAssessmentCourseLessonsResponseLesson {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
