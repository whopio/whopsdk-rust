pub use crate::prelude::*;

/// An individual learning unit within a chapter, which can contain text, video, PDF, or assessment content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CourseLesson {
    /// The list of questions for quiz or knowledge check lessons. Empty for non-assessment lesson types.
    #[serde(default)]
    pub assessment_questions: Vec<CourseLessonAssessmentQuestionsItem>,
    /// All supplementary files attached to this lesson returned as a flat array rather than a paginated connection.
    #[serde(default)]
    pub attachments: Vec<CourseLessonAttachmentsItem>,
    /// The Markdown content body of the lesson. Null if the lesson has no text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The datetime the lesson was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The number of days after a student starts the course before this lesson becomes accessible. Null if the lesson is available immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_from_course_start_until_unlock: Option<i64>,
    /// The external video identifier for embedded video lessons, such as a YouTube video ID or Loom share ID. Null if the lesson has no embed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_id: Option<String>,
    /// The platform type for the embedded video. One of: youtube, loom. Null if the lesson has no embed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_type: Option<EmbedTypes>,
    /// The unique identifier for the lesson.
    #[serde(default)]
    pub id: String,
    /// The content format of this lesson. One of: text, video, pdf, multi, quiz, knowledge_check.
    pub lesson_type: LessonTypes,
    /// The primary PDF document for PDF-type lessons. Null if this lesson is not a PDF lesson or no PDF has been uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_pdf: Option<CourseLessonMainPdf>,
    /// The sort position of this lesson within its parent chapter, starting from zero.
    #[serde(default)]
    pub order: i64,
    /// The thumbnail image displayed on lesson cards and previews. Null if no thumbnail has been uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<CourseLessonThumbnail>,
    /// The display name of the lesson shown to students. Maximum 120 characters.
    #[serde(default)]
    pub title: String,
    /// The Mux video asset for video-type lessons, used for streaming playback. Null if this lesson has no hosted video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_asset: Option<CourseLessonVideoAsset>,
    /// The visibility setting that controls whether this lesson appears to students. One of: visible, hidden.
    pub visibility: LessonVisibilities,
}

impl CourseLesson {
    pub fn builder() -> CourseLessonBuilder {
        <CourseLessonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonBuilder {
    assessment_questions: Option<Vec<CourseLessonAssessmentQuestionsItem>>,
    attachments: Option<Vec<CourseLessonAttachmentsItem>>,
    content: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    days_from_course_start_until_unlock: Option<i64>,
    embed_id: Option<String>,
    embed_type: Option<EmbedTypes>,
    id: Option<String>,
    lesson_type: Option<LessonTypes>,
    main_pdf: Option<CourseLessonMainPdf>,
    order: Option<i64>,
    thumbnail: Option<CourseLessonThumbnail>,
    title: Option<String>,
    video_asset: Option<CourseLessonVideoAsset>,
    visibility: Option<LessonVisibilities>,
}

impl CourseLessonBuilder {
    pub fn assessment_questions(mut self, value: Vec<CourseLessonAssessmentQuestionsItem>) -> Self {
        self.assessment_questions = Some(value);
        self
    }

    pub fn attachments(mut self, value: Vec<CourseLessonAttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lesson_type(mut self, value: LessonTypes) -> Self {
        self.lesson_type = Some(value);
        self
    }

    pub fn main_pdf(mut self, value: CourseLessonMainPdf) -> Self {
        self.main_pdf = Some(value);
        self
    }

    pub fn order(mut self, value: i64) -> Self {
        self.order = Some(value);
        self
    }

    pub fn thumbnail(mut self, value: CourseLessonThumbnail) -> Self {
        self.thumbnail = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn video_asset(mut self, value: CourseLessonVideoAsset) -> Self {
        self.video_asset = Some(value);
        self
    }

    pub fn visibility(mut self, value: LessonVisibilities) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CourseLesson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`assessment_questions`](CourseLessonBuilder::assessment_questions)
    /// - [`attachments`](CourseLessonBuilder::attachments)
    /// - [`created_at`](CourseLessonBuilder::created_at)
    /// - [`id`](CourseLessonBuilder::id)
    /// - [`lesson_type`](CourseLessonBuilder::lesson_type)
    /// - [`order`](CourseLessonBuilder::order)
    /// - [`title`](CourseLessonBuilder::title)
    /// - [`visibility`](CourseLessonBuilder::visibility)
    pub fn build(self) -> Result<CourseLesson, BuildError> {
        Ok(CourseLesson {
            assessment_questions: self
                .assessment_questions
                .ok_or_else(|| BuildError::missing_field("assessment_questions"))?,
            attachments: self
                .attachments
                .ok_or_else(|| BuildError::missing_field("attachments"))?,
            content: self.content,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            days_from_course_start_until_unlock: self.days_from_course_start_until_unlock,
            embed_id: self.embed_id,
            embed_type: self.embed_type,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            lesson_type: self
                .lesson_type
                .ok_or_else(|| BuildError::missing_field("lesson_type"))?,
            main_pdf: self.main_pdf,
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
            thumbnail: self.thumbnail,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            video_asset: self.video_asset,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
        })
    }
}
