pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateNotificationsResponse {
    #[serde(default)]
    pub success: bool,
}

impl CreateNotificationsResponse {
    pub fn builder() -> CreateNotificationsResponseBuilder {
        <CreateNotificationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateNotificationsResponseBuilder {
    success: Option<bool>,
}

impl CreateNotificationsResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateNotificationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CreateNotificationsResponseBuilder::success)
    pub fn build(self) -> Result<CreateNotificationsResponse, BuildError> {
        Ok(CreateNotificationsResponse {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
