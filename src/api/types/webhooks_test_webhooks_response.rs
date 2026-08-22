pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TestWebhooksResponse {
    pub body: serde_json::Value,
    /// The HTTP response code of this request.
    #[serde(default)]
    pub status: i64,
    /// Whether or not the webhook test was successful.
    #[serde(default)]
    pub success: bool,
}

impl TestWebhooksResponse {
    pub fn builder() -> TestWebhooksResponseBuilder {
        <TestWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestWebhooksResponseBuilder {
    body: Option<serde_json::Value>,
    status: Option<i64>,
    success: Option<bool>,
}

impl TestWebhooksResponseBuilder {
    pub fn body(mut self, value: serde_json::Value) -> Self {
        self.body = Some(value);
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

    /// Consumes the builder and constructs a [`TestWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](TestWebhooksResponseBuilder::body)
    /// - [`status`](TestWebhooksResponseBuilder::status)
    /// - [`success`](TestWebhooksResponseBuilder::success)
    pub fn build(self) -> Result<TestWebhooksResponse, BuildError> {
        Ok(TestWebhooksResponse {
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
