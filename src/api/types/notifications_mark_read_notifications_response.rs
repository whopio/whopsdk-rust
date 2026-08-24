pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MarkReadNotificationsResponse {
    #[serde(default)]
    pub data: Vec<NotificationBadge>,
}

impl MarkReadNotificationsResponse {
    pub fn builder() -> MarkReadNotificationsResponseBuilder {
        <MarkReadNotificationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkReadNotificationsResponseBuilder {
    data: Option<Vec<NotificationBadge>>,
}

impl MarkReadNotificationsResponseBuilder {
    pub fn data(mut self, value: Vec<NotificationBadge>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MarkReadNotificationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](MarkReadNotificationsResponseBuilder::data)
    pub fn build(self) -> Result<MarkReadNotificationsResponse, BuildError> {
        Ok(MarkReadNotificationsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
