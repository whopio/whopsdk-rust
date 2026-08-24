pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplayDeliveryWebhooksRequest {
    /// Re-send the delivery under a freshly generated `webhook-id` (in both the envelope and the signed headers) instead of the original one. Defaults to false. Use this when your endpoint deduplicates on `webhook-id` and you want it to process the replay as a new message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regenerate_id: Option<bool>,
}

impl ReplayDeliveryWebhooksRequest {
    pub fn builder() -> ReplayDeliveryWebhooksRequestBuilder {
        <ReplayDeliveryWebhooksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayDeliveryWebhooksRequestBuilder {
    regenerate_id: Option<bool>,
}

impl ReplayDeliveryWebhooksRequestBuilder {
    pub fn regenerate_id(mut self, value: bool) -> Self {
        self.regenerate_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayDeliveryWebhooksRequest`].
    pub fn build(self) -> Result<ReplayDeliveryWebhooksRequest, BuildError> {
        Ok(ReplayDeliveryWebhooksRequest {
            regenerate_id: self.regenerate_id,
        })
    }
}
