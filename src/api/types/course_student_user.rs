pub use crate::prelude::*;

/// The user profile of the enrolled student.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseStudentUser {
    /// The unique identifier for the user.
    #[serde(default)]
    pub id: String,
    /// The user's display name shown on their public profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's unique username shown on their public profile.
    #[serde(default)]
    pub username: String,
}

impl CourseStudentUser {
    pub fn builder() -> CourseStudentUserBuilder {
        <CourseStudentUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseStudentUserBuilder {
    id: Option<String>,
    name: Option<String>,
    username: Option<String>,
}

impl CourseStudentUserBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseStudentUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseStudentUserBuilder::id)
    /// - [`username`](CourseStudentUserBuilder::username)
    pub fn build(self) -> Result<CourseStudentUser, BuildError> {
        Ok(CourseStudentUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
