pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BadgesNotificationsResponse {
    #[serde(default)]
    pub data: Vec<NotificationBadge>,
}

impl BadgesNotificationsResponse {
    pub fn builder() -> BadgesNotificationsResponseBuilder {
        <BadgesNotificationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BadgesNotificationsResponseBuilder {
    data: Option<Vec<NotificationBadge>>,
}

impl BadgesNotificationsResponseBuilder {
    pub fn data(mut self, value: Vec<NotificationBadge>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BadgesNotificationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](BadgesNotificationsResponseBuilder::data)
    pub fn build(self) -> Result<BadgesNotificationsResponse, BuildError> {
        Ok(BadgesNotificationsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
