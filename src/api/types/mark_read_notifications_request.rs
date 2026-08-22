pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MarkReadNotificationsRequest {
    /// Pass `true` to mark every notification read. Exactly one of `experience_id` or `all` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// Experience to mark read (`exp_` tag). Exactly one of `experience_id` or `all` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
}

impl MarkReadNotificationsRequest {
    pub fn builder() -> MarkReadNotificationsRequestBuilder {
        <MarkReadNotificationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkReadNotificationsRequestBuilder {
    all: Option<bool>,
    experience_id: Option<String>,
}

impl MarkReadNotificationsRequestBuilder {
    pub fn all(mut self, value: bool) -> Self {
        self.all = Some(value);
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MarkReadNotificationsRequest`].
    pub fn build(self) -> Result<MarkReadNotificationsRequest, BuildError> {
        Ok(MarkReadNotificationsRequest {
            all: self.all,
            experience_id: self.experience_id,
        })
    }
}
