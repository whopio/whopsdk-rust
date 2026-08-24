pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplayWebhooksRequest {
    /// Only replay these event types, in dot form (for example `payment.succeeded`). Omit to include every event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Only replay messages whose most recent delivery attempt in the window failed. Defaults to false. Best-effort: a message whose attempts span processing batches can still be re-sent — replays keep the original `webhook-id` by default, so consumers that deduplicate are unaffected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_only: Option<bool>,
    /// Re-send each replayed message under a freshly generated `webhook-id` (in both the envelope and the signed headers) instead of its original one. Defaults to false. Use this when your endpoint deduplicates on `webhook-id` and you want it to process the replays as new messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regenerate_ids: Option<bool>,
    /// Start of the delivery window to replay, as an ISO 8601 timestamp. Clamped to the 30-day delivery retention.
    #[serde(default)]
    pub sent_after: String,
    /// End of the delivery window to replay, as an ISO 8601 timestamp. Defaults to now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_before: Option<String>,
}

impl ReplayWebhooksRequest {
    pub fn builder() -> ReplayWebhooksRequestBuilder {
        <ReplayWebhooksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayWebhooksRequestBuilder {
    events: Option<Vec<String>>,
    failed_only: Option<bool>,
    regenerate_ids: Option<bool>,
    sent_after: Option<String>,
    sent_before: Option<String>,
}

impl ReplayWebhooksRequestBuilder {
    pub fn events(mut self, value: Vec<String>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn failed_only(mut self, value: bool) -> Self {
        self.failed_only = Some(value);
        self
    }

    pub fn regenerate_ids(mut self, value: bool) -> Self {
        self.regenerate_ids = Some(value);
        self
    }

    pub fn sent_after(mut self, value: impl Into<String>) -> Self {
        self.sent_after = Some(value.into());
        self
    }

    pub fn sent_before(mut self, value: impl Into<String>) -> Self {
        self.sent_before = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReplayWebhooksRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent_after`](ReplayWebhooksRequestBuilder::sent_after)
    pub fn build(self) -> Result<ReplayWebhooksRequest, BuildError> {
        Ok(ReplayWebhooksRequest {
            events: self.events,
            failed_only: self.failed_only,
            regenerate_ids: self.regenerate_ids,
            sent_after: self
                .sent_after
                .ok_or_else(|| BuildError::missing_field("sent_after"))?,
            sent_before: self.sent_before,
        })
    }
}
