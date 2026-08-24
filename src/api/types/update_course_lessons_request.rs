pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateCourseLessonsRequest {
    /// The passing criteria for quiz or knowledge check lessons, such as minimum grade or correct answers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_completion_requirement:
        Option<UpdateCourseLessonsRequestAssessmentCompletionRequirement>,
    /// The full list of assessment questions for quiz or knowledge check lessons. Replaces all existing questions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_questions: Option<Vec<UpdateCourseLessonsRequestAssessmentQuestionsItem>>,
    /// File attachments for the lesson such as PDFs or documents. Replaces all existing attachments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<UpdateCourseLessonsRequestAttachmentsItem>>,
    /// The Markdown content body of the lesson.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The number of days after a student starts the course before this lesson becomes accessible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_from_course_start_until_unlock: Option<i64>,
    /// The external video identifier for embedded content (e.g., a YouTube video ID or Loom share ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_id: Option<String>,
    /// The type of video embed for this lesson, such as YouTube or Loom.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_type: Option<EmbedTypes>,
    /// The content type of the lesson, such as video, text, quiz, or knowledge check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lesson_type: Option<LessonTypes>,
    /// The primary PDF document attached to this lesson for student reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_pdf: Option<UpdateCourseLessonsRequestMainPdf>,
    /// The maximum number of attempts a student is allowed for assessment lessons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
    /// The identifier of a Mux video asset to attach to this lesson (e.g., "mux_XXXXX").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mux_asset_id: Option<String>,
    /// The thumbnail image for the lesson in PNG, JPEG, or GIF format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<UpdateCourseLessonsRequestThumbnail>,
    /// The display title of the lesson (e.g., "Getting Started with APIs").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Controls whether this lesson is visible to students or hidden as a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<LessonVisibilities>,
}

impl UpdateCourseLessonsRequest {
    pub fn builder() -> UpdateCourseLessonsRequestBuilder {
        <UpdateCourseLessonsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCourseLessonsRequestBuilder {
    assessment_completion_requirement:
        Option<UpdateCourseLessonsRequestAssessmentCompletionRequirement>,
    assessment_questions: Option<Vec<UpdateCourseLessonsRequestAssessmentQuestionsItem>>,
    attachments: Option<Vec<UpdateCourseLessonsRequestAttachmentsItem>>,
    content: Option<String>,
    days_from_course_start_until_unlock: Option<i64>,
    embed_id: Option<String>,
    embed_type: Option<EmbedTypes>,
    lesson_type: Option<LessonTypes>,
    main_pdf: Option<UpdateCourseLessonsRequestMainPdf>,
    max_attempts: Option<i64>,
    mux_asset_id: Option<String>,
    thumbnail: Option<UpdateCourseLessonsRequestThumbnail>,
    title: Option<String>,
    visibility: Option<LessonVisibilities>,
}

impl UpdateCourseLessonsRequestBuilder {
    pub fn assessment_completion_requirement(
        mut self,
        value: UpdateCourseLessonsRequestAssessmentCompletionRequirement,
    ) -> Self {
        self.assessment_completion_requirement = Some(value);
        self
    }

    pub fn assessment_questions(
        mut self,
        value: Vec<UpdateCourseLessonsRequestAssessmentQuestionsItem>,
    ) -> Self {
        self.assessment_questions = Some(value);
        self
    }

    pub fn attachments(mut self, value: Vec<UpdateCourseLessonsRequestAttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn days_from_course_start_until_unlock(mut self, value: i64) -> Self {
        self.days_from_course_start_until_unlock = Some(value);
        self
    }

    pub fn embed_id(mut self, value: impl Into<String>) -> Self {
        self.embed_id = Some(value.into());
        self
    }

    pub fn embed_type(mut self, value: EmbedTypes) -> Self {
        self.embed_type = Some(value);
        self
    }

    pub fn lesson_type(mut self, value: LessonTypes) -> Self {
        self.lesson_type = Some(value);
        self
    }

    pub fn main_pdf(mut self, value: UpdateCourseLessonsRequestMainPdf) -> Self {
        self.main_pdf = Some(value);
        self
    }

    pub fn max_attempts(mut self, value: i64) -> Self {
        self.max_attempts = Some(value);
        self
    }

    pub fn mux_asset_id(mut self, value: impl Into<String>) -> Self {
        self.mux_asset_id = Some(value.into());
        self
    }

    pub fn thumbnail(mut self, value: UpdateCourseLessonsRequestThumbnail) -> Self {
        self.thumbnail = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: LessonVisibilities) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCourseLessonsRequest`].
    pub fn build(self) -> Result<UpdateCourseLessonsRequest, BuildError> {
        Ok(UpdateCourseLessonsRequest {
            assessment_completion_requirement: self.assessment_completion_requirement,
            assessment_questions: self.assessment_questions,
            attachments: self.attachments,
            content: self.content,
            days_from_course_start_until_unlock: self.days_from_course_start_until_unlock,
            embed_id: self.embed_id,
            embed_type: self.embed_type,
            lesson_type: self.lesson_type,
            main_pdf: self.main_pdf,
            max_attempts: self.max_attempts,
            mux_asset_id: self.mux_asset_id,
            thumbnail: self.thumbnail,
            title: self.title,
            visibility: self.visibility,
        })
    }
}
