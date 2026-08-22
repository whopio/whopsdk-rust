pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdMessagingConfig {
    /// Suggested reply the person can tap to start the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// Greeting shown when the conversation opens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl AdMessagingConfig {
    pub fn builder() -> AdMessagingConfigBuilder {
        <AdMessagingConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdMessagingConfigBuilder {
    keyword: Option<String>,
    message: Option<String>,
}

impl AdMessagingConfigBuilder {
    pub fn keyword(mut self, value: impl Into<String>) -> Self {
        self.keyword = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdMessagingConfig`].
    pub fn build(self) -> Result<AdMessagingConfig, BuildError> {
        Ok(AdMessagingConfig {
            keyword: self.keyword,
            message: self.message,
        })
    }
}
