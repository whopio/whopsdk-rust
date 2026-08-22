pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplayDeliveryWebhooksResponse {
    /// The body your endpoint returned for the replayed request, as raw text. Empty when the endpoint could not be reached.
    #[serde(default)]
    pub body: String,
    /// The HTTP response code your endpoint returned for the replayed request, or 0 when it could not be reached (timeout, DNS, or connection failure).
    #[serde(default)]
    pub status: i64,
    /// Whether your endpoint acknowledged the replay with a 2xx response.
    #[serde(default)]
    pub success: bool,
}

impl ReplayDeliveryWebhooksResponse {
    pub fn builder() -> ReplayDeliveryWebhooksResponseBuilder {
        <ReplayDeliveryWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayDeliveryWebhooksResponseBuilder {
    body: Option<String>,
    status: Option<i64>,
    success: Option<bool>,
}

impl ReplayDeliveryWebhooksResponseBuilder {
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayDeliveryWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](ReplayDeliveryWebhooksResponseBuilder::body)
    /// - [`status`](ReplayDeliveryWebhooksResponseBuilder::status)
    /// - [`success`](ReplayDeliveryWebhooksResponseBuilder::success)
    pub fn build(self) -> Result<ReplayDeliveryWebhooksResponse, BuildError> {
        Ok(ReplayDeliveryWebhooksResponse {
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
