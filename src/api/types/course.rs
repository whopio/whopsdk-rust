pub use crate::prelude::*;

/// A structured learning module containing chapters and lessons, belonging to an experience.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Course {
    /// Whether students receive a PDF certificate after completing all lessons in this course. Null if the setting has not been configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_after_completion_enabled: Option<bool>,
    /// An ordered list of all chapters in this course, sorted by their display position.
    #[serde(default)]
    pub chapters: Vec<CourseChaptersItem>,
    /// The total number of chapters in this course, including chapters whose lessons are all hidden from the current user.
    #[serde(default)]
    pub chapters_count: i64,
    /// The number of lessons in this course that the current user has marked as completed. Zero when the request is not made on behalf of a user.
    #[serde(default)]
    pub completed_lessons_count: i64,
    /// The URL of the course cover image shown on preview cards. Null if no cover image has been uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    /// The datetime the course was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// A brief summary of the course content and objectives. Null if no description has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The unique identifier for the course.
    #[serde(default)]
    pub id: String,
    /// The spoken language of the video content, used to generate accurate closed captions. One of: en, es, it, pt, de, fr, pl, ru, nl, ca, tr, sv, uk, no, fi, sk, el, cs, hr, da, ro, bg.
    pub language: Languages,
    /// The creation timestamp of the most recently added lesson visible to the current user. Null if the course has no lessons.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub latest_lesson_created_at: Option<DateTime<FixedOffset>>,
    /// The distinct drip schedules, in days after the course start, of lessons visible to the current user. Combine with startedAt to work out which have unlocked. Empty when the user has not started the course or no lesson is on a schedule.
    #[serde(default)]
    pub lesson_unlock_days: Vec<i64>,
    /// The sort position of this course within its parent experience, as a decimal for flexible ordering.
    #[serde(default)]
    pub order: String,
    /// Whether students must complete each lesson sequentially before advancing to the next one.
    #[serde(default)]
    pub require_completing_lessons_in_order: bool,
    /// The lesson the current user should continue from: their first incomplete lesson, or the first lesson when they have finished the course, have not started it, or can edit it. Null if the course has no lessons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_lesson: Option<CourseResumeLesson>,
    /// The earliest time the current user is known to have started this course. Null if they have not started it. Drip unlock schedules are measured from this point.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub started_at: Option<DateTime<FixedOffset>>,
    /// A short marketing tagline displayed beneath the course title. Null if no tagline has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagline: Option<String>,
    /// The thumbnail image displayed on course cards and previews. Null if no thumbnail has been uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<CourseThumbnail>,
    /// The display name of the course shown to students. Null if no title has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The combined duration in seconds of every hosted video across the lessons visible to the current user.
    #[serde(default)]
    pub total_duration_seconds: i64,
    /// The number of lessons in this course visible to the current user.
    #[serde(default)]
    pub total_lessons_count: i64,
    /// The datetime the course was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The visibility setting that controls whether this course appears to students. One of: visible, hidden.
    pub visibility: CourseVisibilities,
}

impl Course {
    pub fn builder() -> CourseBuilder {
        <CourseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseBuilder {
    certificate_after_completion_enabled: Option<bool>,
    chapters: Option<Vec<CourseChaptersItem>>,
    chapters_count: Option<i64>,
    completed_lessons_count: Option<i64>,
    cover_image: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    id: Option<String>,
    language: Option<Languages>,
    latest_lesson_created_at: Option<DateTime<FixedOffset>>,
    lesson_unlock_days: Option<Vec<i64>>,
    order: Option<String>,
    require_completing_lessons_in_order: Option<bool>,
    resume_lesson: Option<CourseResumeLesson>,
    started_at: Option<DateTime<FixedOffset>>,
    tagline: Option<String>,
    thumbnail: Option<CourseThumbnail>,
    title: Option<String>,
    total_duration_seconds: Option<i64>,
    total_lessons_count: Option<i64>,
    updated_at: Option<DateTime<FixedOffset>>,
    visibility: Option<CourseVisibilities>,
}

impl CourseBuilder {
    pub fn certificate_after_completion_enabled(mut self, value: bool) -> Self {
        self.certificate_after_completion_enabled = Some(value);
        self
    }

    pub fn chapters(mut self, value: Vec<CourseChaptersItem>) -> Self {
        self.chapters = Some(value);
        self
    }

    pub fn chapters_count(mut self, value: i64) -> Self {
        self.chapters_count = Some(value);
        self
    }

    pub fn completed_lessons_count(mut self, value: i64) -> Self {
        self.completed_lessons_count = Some(value);
        self
    }

    pub fn cover_image(mut self, value: impl Into<String>) -> Self {
        self.cover_image = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn language(mut self, value: Languages) -> Self {
        self.language = Some(value);
        self
    }

    pub fn latest_lesson_created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.latest_lesson_created_at = Some(value);
        self
    }

    pub fn lesson_unlock_days(mut self, value: Vec<i64>) -> Self {
        self.lesson_unlock_days = Some(value);
        self
    }

    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.order = Some(value.into());
        self
    }

    pub fn require_completing_lessons_in_order(mut self, value: bool) -> Self {
        self.require_completing_lessons_in_order = Some(value);
        self
    }

    pub fn resume_lesson(mut self, value: CourseResumeLesson) -> Self {
        self.resume_lesson = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn tagline(mut self, value: impl Into<String>) -> Self {
        self.tagline = Some(value.into());
        self
    }

    pub fn thumbnail(mut self, value: CourseThumbnail) -> Self {
        self.thumbnail = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn total_duration_seconds(mut self, value: i64) -> Self {
        self.total_duration_seconds = Some(value);
        self
    }

    pub fn total_lessons_count(mut self, value: i64) -> Self {
        self.total_lessons_count = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn visibility(mut self, value: CourseVisibilities) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Course`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapters`](CourseBuilder::chapters)
    /// - [`chapters_count`](CourseBuilder::chapters_count)
    /// - [`completed_lessons_count`](CourseBuilder::completed_lessons_count)
    /// - [`created_at`](CourseBuilder::created_at)
    /// - [`id`](CourseBuilder::id)
    /// - [`language`](CourseBuilder::language)
    /// - [`lesson_unlock_days`](CourseBuilder::lesson_unlock_days)
    /// - [`order`](CourseBuilder::order)
    /// - [`require_completing_lessons_in_order`](CourseBuilder::require_completing_lessons_in_order)
    /// - [`total_duration_seconds`](CourseBuilder::total_duration_seconds)
    /// - [`total_lessons_count`](CourseBuilder::total_lessons_count)
    /// - [`updated_at`](CourseBuilder::updated_at)
    /// - [`visibility`](CourseBuilder::visibility)
    pub fn build(self) -> Result<Course, BuildError> {
        Ok(Course {
            certificate_after_completion_enabled: self.certificate_after_completion_enabled,
            chapters: self
                .chapters
                .ok_or_else(|| BuildError::missing_field("chapters"))?,
            chapters_count: self
                .chapters_count
                .ok_or_else(|| BuildError::missing_field("chapters_count"))?,
            completed_lessons_count: self
                .completed_lessons_count
                .ok_or_else(|| BuildError::missing_field("completed_lessons_count"))?,
            cover_image: self.cover_image,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            language: self
                .language
                .ok_or_else(|| BuildError::missing_field("language"))?,
            latest_lesson_created_at: self.latest_lesson_created_at,
            lesson_unlock_days: self
                .lesson_unlock_days
                .ok_or_else(|| BuildError::missing_field("lesson_unlock_days"))?,
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
            require_completing_lessons_in_order: self
                .require_completing_lessons_in_order
                .ok_or_else(|| BuildError::missing_field("require_completing_lessons_in_order"))?,
            resume_lesson: self.resume_lesson,
            started_at: self.started_at,
            tagline: self.tagline,
            thumbnail: self.thumbnail,
            title: self.title,
            total_duration_seconds: self
                .total_duration_seconds
                .ok_or_else(|| BuildError::missing_field("total_duration_seconds"))?,
            total_lessons_count: self
                .total_lessons_count
                .ok_or_else(|| BuildError::missing_field("total_lessons_count"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
        })
    }
}
