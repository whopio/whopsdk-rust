pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCoursesRequest {
    /// Whether the course awards students a PDF certificate after completing all lessons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_after_completion_enabled: Option<bool>,
    /// The unique identifier of the experience to create the course in (e.g., "exp_XXXXX").
    #[serde(default)]
    pub experience_id: String,
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
    pub thumbnail: Option<CreateCoursesRequestThumbnail>,
    /// The display title of the course (e.g., "Introduction to Web Development").
    #[serde(default)]
    pub title: String,
    /// Controls whether this course is visible to students or hidden as a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<CourseVisibilities>,
}

impl CreateCoursesRequest {
    pub fn builder() -> CreateCoursesRequestBuilder {
        <CreateCoursesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCoursesRequestBuilder {
    certificate_after_completion_enabled: Option<bool>,
    experience_id: Option<String>,
    order: Option<String>,
    require_completing_lessons_in_order: Option<bool>,
    tagline: Option<String>,
    thumbnail: Option<CreateCoursesRequestThumbnail>,
    title: Option<String>,
    visibility: Option<CourseVisibilities>,
}

impl CreateCoursesRequestBuilder {
    pub fn certificate_after_completion_enabled(mut self, value: bool) -> Self {
        self.certificate_after_completion_enabled = Some(value);
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
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

    pub fn thumbnail(mut self, value: CreateCoursesRequestThumbnail) -> Self {
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

    /// Consumes the builder and constructs a [`CreateCoursesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`experience_id`](CreateCoursesRequestBuilder::experience_id)
    /// - [`title`](CreateCoursesRequestBuilder::title)
    pub fn build(self) -> Result<CreateCoursesRequest, BuildError> {
        Ok(CreateCoursesRequest {
            certificate_after_completion_enabled: self.certificate_after_completion_enabled,
            experience_id: self
                .experience_id
                .ok_or_else(|| BuildError::missing_field("experience_id"))?,
            order: self.order,
            require_completing_lessons_in_order: self.require_completing_lessons_in_order,
            tagline: self.tagline,
            thumbnail: self.thumbnail,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            visibility: self.visibility,
        })
    }
}
