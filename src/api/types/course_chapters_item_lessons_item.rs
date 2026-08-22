pub use crate::prelude::*;

/// An individual learning unit within a chapter, which can contain text, video, PDF, or assessment content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CourseChaptersItemLessonsItem {
    /// The unique identifier for the lesson.
    #[serde(default)]
    pub id: String,
    /// The content format of this lesson. One of: text, video, pdf, multi, quiz, knowledge_check.
    pub lesson_type: LessonTypes,
    /// The sort position of this lesson within its parent chapter, starting from zero.
    #[serde(default)]
    pub order: i64,
    /// The thumbnail image displayed on lesson cards and previews. Null if no thumbnail has been uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<CourseChaptersItemLessonsItemThumbnail>,
    /// The display name of the lesson shown to students. Maximum 120 characters.
    #[serde(default)]
    pub title: String,
    /// The Mux video asset for video-type lessons, used for streaming playback. Null if this lesson has no hosted video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_asset: Option<CourseChaptersItemLessonsItemVideoAsset>,
}

impl CourseChaptersItemLessonsItem {
    pub fn builder() -> CourseChaptersItemLessonsItemBuilder {
        <CourseChaptersItemLessonsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseChaptersItemLessonsItemBuilder {
    id: Option<String>,
    lesson_type: Option<LessonTypes>,
    order: Option<i64>,
    thumbnail: Option<CourseChaptersItemLessonsItemThumbnail>,
    title: Option<String>,
    video_asset: Option<CourseChaptersItemLessonsItemVideoAsset>,
}

impl CourseChaptersItemLessonsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lesson_type(mut self, value: LessonTypes) -> Self {
        self.lesson_type = Some(value);
        self
    }

    pub fn order(mut self, value: i64) -> Self {
        self.order = Some(value);
        self
    }

    pub fn thumbnail(mut self, value: CourseChaptersItemLessonsItemThumbnail) -> Self {
        self.thumbnail = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn video_asset(mut self, value: CourseChaptersItemLessonsItemVideoAsset) -> Self {
        self.video_asset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CourseChaptersItemLessonsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseChaptersItemLessonsItemBuilder::id)
    /// - [`lesson_type`](CourseChaptersItemLessonsItemBuilder::lesson_type)
    /// - [`order`](CourseChaptersItemLessonsItemBuilder::order)
    /// - [`title`](CourseChaptersItemLessonsItemBuilder::title)
    pub fn build(self) -> Result<CourseChaptersItemLessonsItem, BuildError> {
        Ok(CourseChaptersItemLessonsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            lesson_type: self
                .lesson_type
                .ok_or_else(|| BuildError::missing_field("lesson_type"))?,
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
            thumbnail: self.thumbnail,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            video_asset: self.video_asset,
        })
    }
}
