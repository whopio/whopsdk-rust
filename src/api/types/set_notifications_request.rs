pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetNotificationsRequest {
    /// The preferences to set, at most 100 per request.
    #[serde(default)]
    pub preferences: Vec<SetNotificationsRequestPreferencesItem>,
}

impl SetNotificationsRequest {
    pub fn builder() -> SetNotificationsRequestBuilder {
        <SetNotificationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetNotificationsRequestBuilder {
    preferences: Option<Vec<SetNotificationsRequestPreferencesItem>>,
}

impl SetNotificationsRequestBuilder {
    pub fn preferences(mut self, value: Vec<SetNotificationsRequestPreferencesItem>) -> Self {
        self.preferences = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetNotificationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`preferences`](SetNotificationsRequestBuilder::preferences)
    pub fn build(self) -> Result<SetNotificationsRequest, BuildError> {
        Ok(SetNotificationsRequest {
            preferences: self
                .preferences
                .ok_or_else(|| BuildError::missing_field("preferences"))?,
        })
    }
}
