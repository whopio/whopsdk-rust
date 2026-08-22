pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TestWebhooksRequest {
    /// The event to test the webhook for, in dot form (for example `payment.succeeded`).
    #[serde(default)]
    pub event: String,
}

impl TestWebhooksRequest {
    pub fn builder() -> TestWebhooksRequestBuilder {
        <TestWebhooksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestWebhooksRequestBuilder {
    event: Option<String>,
}

impl TestWebhooksRequestBuilder {
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TestWebhooksRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](TestWebhooksRequestBuilder::event)
    pub fn build(self) -> Result<TestWebhooksRequest, BuildError> {
        Ok(TestWebhooksRequest {
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
        })
    }
}
