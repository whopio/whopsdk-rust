pub use crate::prelude::*;

/// An enrollment record for a student in a course, including progress and completion metrics.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CourseStudentListItem {
    /// The total number of lessons this student has marked as completed in the course.
    #[serde(default)]
    pub completed_lessons_count: i64,
    /// The percentage of available lessons the student has completed, as a value from 0 to 100 rounded to two decimal places.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub completion_rate: f64,
    /// The timestamp when the student first interacted with this course, as a Unix timestamp.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub first_interaction_at: DateTime<FixedOffset>,
    /// The unique identifier for the course student type.
    #[serde(default)]
    pub id: String,
    /// The timestamp when the student most recently interacted with this course, as a Unix timestamp.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub last_interaction_at: DateTime<FixedOffset>,
    /// The total number of visible lessons available to this student in the course.
    #[serde(default)]
    pub total_lessons_count: i64,
    /// The user profile of the enrolled student.
    #[serde(default)]
    pub user: CourseStudentListItemUser,
}

impl CourseStudentListItem {
    pub fn builder() -> CourseStudentListItemBuilder {
        <CourseStudentListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseStudentListItemBuilder {
    completed_lessons_count: Option<i64>,
    completion_rate: Option<f64>,
    first_interaction_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    last_interaction_at: Option<DateTime<FixedOffset>>,
    total_lessons_count: Option<i64>,
    user: Option<CourseStudentListItemUser>,
}

impl CourseStudentListItemBuilder {
    pub fn completed_lessons_count(mut self, value: i64) -> Self {
        self.completed_lessons_count = Some(value);
        self
    }

    pub fn completion_rate(mut self, value: f64) -> Self {
        self.completion_rate = Some(value);
        self
    }

    pub fn first_interaction_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_interaction_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_interaction_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_interaction_at = Some(value);
        self
    }

    pub fn total_lessons_count(mut self, value: i64) -> Self {
        self.total_lessons_count = Some(value);
        self
    }

    pub fn user(mut self, value: CourseStudentListItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CourseStudentListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completed_lessons_count`](CourseStudentListItemBuilder::completed_lessons_count)
    /// - [`completion_rate`](CourseStudentListItemBuilder::completion_rate)
    /// - [`first_interaction_at`](CourseStudentListItemBuilder::first_interaction_at)
    /// - [`id`](CourseStudentListItemBuilder::id)
    /// - [`last_interaction_at`](CourseStudentListItemBuilder::last_interaction_at)
    /// - [`total_lessons_count`](CourseStudentListItemBuilder::total_lessons_count)
    /// - [`user`](CourseStudentListItemBuilder::user)
    pub fn build(self) -> Result<CourseStudentListItem, BuildError> {
        Ok(CourseStudentListItem {
            completed_lessons_count: self
                .completed_lessons_count
                .ok_or_else(|| BuildError::missing_field("completed_lessons_count"))?,
            completion_rate: self
                .completion_rate
                .ok_or_else(|| BuildError::missing_field("completion_rate"))?,
            first_interaction_at: self
                .first_interaction_at
                .ok_or_else(|| BuildError::missing_field("first_interaction_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_interaction_at: self
                .last_interaction_at
                .ok_or_else(|| BuildError::missing_field("last_interaction_at"))?,
            total_lessons_count: self
                .total_lessons_count
                .ok_or_else(|| BuildError::missing_field("total_lessons_count"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
