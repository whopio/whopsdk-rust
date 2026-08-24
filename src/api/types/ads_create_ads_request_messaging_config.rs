pub use crate::prelude::*;

/// Click-to-message welcome copy: the greeting (message) and the ice-breaker prompt (keyword).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestMessagingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl CreateAdsRequestMessagingConfig {
    pub fn builder() -> CreateAdsRequestMessagingConfigBuilder {
        <CreateAdsRequestMessagingConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestMessagingConfigBuilder {
    keyword: Option<String>,
    message: Option<String>,
}

impl CreateAdsRequestMessagingConfigBuilder {
    pub fn keyword(mut self, value: impl Into<String>) -> Self {
        self.keyword = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestMessagingConfig`].
    pub fn build(self) -> Result<CreateAdsRequestMessagingConfig, BuildError> {
        Ok(CreateAdsRequestMessagingConfig {
            keyword: self.keyword,
            message: self.message,
        })
    }
}
