pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateFeedbackSubmissionsResponse {
    /// When the submission was accepted, as an ISO 8601 timestamp.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The ID of the feedback submission receipt (fbk_...).
    #[serde(default)]
    pub id: String,
}

impl CreateFeedbackSubmissionsResponse {
    pub fn builder() -> CreateFeedbackSubmissionsResponseBuilder {
        <CreateFeedbackSubmissionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFeedbackSubmissionsResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
}

impl CreateFeedbackSubmissionsResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateFeedbackSubmissionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](CreateFeedbackSubmissionsResponseBuilder::created_at)
    /// - [`id`](CreateFeedbackSubmissionsResponseBuilder::id)
    pub fn build(self) -> Result<CreateFeedbackSubmissionsResponse, BuildError> {
        Ok(CreateFeedbackSubmissionsResponse {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
