pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateCourseLessonsRequest {
    /// The unique identifier of the chapter to create the lesson in (e.g., "chap_XXXXX").
    #[serde(default)]
    pub chapter_id: String,
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
    pub lesson_type: LessonTypes,
    /// The thumbnail image for the lesson in PNG, JPEG, or GIF format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<CreateCourseLessonsRequestThumbnail>,
    /// The display title of the lesson (e.g., "Getting Started with APIs").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreateCourseLessonsRequest {
    pub fn builder() -> CreateCourseLessonsRequestBuilder {
        <CreateCourseLessonsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCourseLessonsRequestBuilder {
    chapter_id: Option<String>,
    content: Option<String>,
    days_from_course_start_until_unlock: Option<i64>,
    embed_id: Option<String>,
    embed_type: Option<EmbedTypes>,
    lesson_type: Option<LessonTypes>,
    thumbnail: Option<CreateCourseLessonsRequestThumbnail>,
    title: Option<String>,
}

impl CreateCourseLessonsRequestBuilder {
    pub fn chapter_id(mut self, value: impl Into<String>) -> Self {
        self.chapter_id = Some(value.into());
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

    pub fn thumbnail(mut self, value: CreateCourseLessonsRequestThumbnail) -> Self {
        self.thumbnail = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCourseLessonsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapter_id`](CreateCourseLessonsRequestBuilder::chapter_id)
    /// - [`lesson_type`](CreateCourseLessonsRequestBuilder::lesson_type)
    pub fn build(self) -> Result<CreateCourseLessonsRequest, BuildError> {
        Ok(CreateCourseLessonsRequest {
            chapter_id: self
                .chapter_id
                .ok_or_else(|| BuildError::missing_field("chapter_id"))?,
            content: self.content,
            days_from_course_start_until_unlock: self.days_from_course_start_until_unlock,
            embed_id: self.embed_id,
            embed_type: self.embed_type,
            lesson_type: self
                .lesson_type
                .ok_or_else(|| BuildError::missing_field("lesson_type"))?,
            thumbnail: self.thumbnail,
            title: self.title,
        })
    }
}
