pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetNotificationsResponse {
    #[serde(default)]
    pub data: Vec<NotificationPreferenceState>,
}

impl SetNotificationsResponse {
    pub fn builder() -> SetNotificationsResponseBuilder {
        <SetNotificationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetNotificationsResponseBuilder {
    data: Option<Vec<NotificationPreferenceState>>,
}

impl SetNotificationsResponseBuilder {
    pub fn data(mut self, value: Vec<NotificationPreferenceState>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetNotificationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](SetNotificationsResponseBuilder::data)
    pub fn build(self) -> Result<SetNotificationsResponse, BuildError> {
        Ok(SetNotificationsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
