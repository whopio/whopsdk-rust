pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCoursesRequest {
    /// Whether the course awards students a PDF certificate after completing all lessons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_after_completion_enabled: Option<bool>,
    /// A list of chapters with nested lessons to reorder or rename in bulk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters: Option<Vec<UpdateCoursesRequestChaptersItem>>,
    /// A short description of the course displayed to students on the course page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The primary language spoken in the video content of the course.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Languages>,
    /// The decimal order position of the course within its experience. Use fractional values (e.g., "1.5") to place between existing courses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Whether students must complete each lesson sequentially before advancing to the next one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_completing_lessons_in_order: Option<bool>,
    /// A short tagline displayed beneath the course title (e.g., "Master the fundamentals of design").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagline: Option<String>,
    /// The thumbnail image for the course in PNG, JPEG, or GIF format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<UpdateCoursesRequestThumbnail>,
    /// The display title of the course (e.g., "Introduction to Web Development").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Controls whether this course is visible to students or hidden as a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<CourseVisibilities>,
}

impl UpdateCoursesRequest {
    pub fn builder() -> UpdateCoursesRequestBuilder {
        <UpdateCoursesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCoursesRequestBuilder {
    certificate_after_completion_enabled: Option<bool>,
    chapters: Option<Vec<UpdateCoursesRequestChaptersItem>>,
    description: Option<String>,
    language: Option<Languages>,
    order: Option<String>,
    require_completing_lessons_in_order: Option<bool>,
    tagline: Option<String>,
    thumbnail: Option<UpdateCoursesRequestThumbnail>,
    title: Option<String>,
    visibility: Option<CourseVisibilities>,
}

impl UpdateCoursesRequestBuilder {
    pub fn certificate_after_completion_enabled(mut self, value: bool) -> Self {
        self.certificate_after_completion_enabled = Some(value);
        self
    }

    pub fn chapters(mut self, value: Vec<UpdateCoursesRequestChaptersItem>) -> Self {
        self.chapters = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn language(mut self, value: Languages) -> Self {
        self.language = Some(value);
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

    pub fn tagline(mut self, value: impl Into<String>) -> Self {
        self.tagline = Some(value.into());
        self
    }

    pub fn thumbnail(mut self, value: UpdateCoursesRequestThumbnail) -> Self {
        self.thumbnail = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: CourseVisibilities) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCoursesRequest`].
    pub fn build(self) -> Result<UpdateCoursesRequest, BuildError> {
        Ok(UpdateCoursesRequest {
            certificate_after_completion_enabled: self.certificate_after_completion_enabled,
            chapters: self.chapters,
            description: self.description,
            language: self.language,
            order: self.order,
            require_completing_lessons_in_order: self.require_completing_lessons_in_order,
            tagline: self.tagline,
            thumbnail: self.thumbnail,
            title: self.title,
            visibility: self.visibility,
        })
    }
}
