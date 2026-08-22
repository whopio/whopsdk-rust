pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplayWebhooksResponse {
    /// Whether the replay was accepted. Watch the webhook's delivery log for the re-sends.
    #[serde(default)]
    pub queued: bool,
}

impl ReplayWebhooksResponse {
    pub fn builder() -> ReplayWebhooksResponseBuilder {
        <ReplayWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayWebhooksResponseBuilder {
    queued: Option<bool>,
}

impl ReplayWebhooksResponseBuilder {
    pub fn queued(mut self, value: bool) -> Self {
        self.queued = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`queued`](ReplayWebhooksResponseBuilder::queued)
    pub fn build(self) -> Result<ReplayWebhooksResponse, BuildError> {
        Ok(ReplayWebhooksResponse {
            queued: self
                .queued
                .ok_or_else(|| BuildError::missing_field("queued"))?,
        })
    }
}
