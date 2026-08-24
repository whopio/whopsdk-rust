pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppApiKey {
    /// When the key was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The key's secret token, sent as a bearer token to authenticate requests on the app's behalf.
    #[serde(default)]
    pub token: String,
    /// The unique identifier for the private api key.
    #[serde(default)]
    pub id: String,
}

impl AppApiKey {
    pub fn builder() -> AppApiKeyBuilder {
        <AppApiKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppApiKeyBuilder {
    created_at: Option<String>,
    token: Option<String>,
    id: Option<String>,
}

impl AppApiKeyBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AppApiKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](AppApiKeyBuilder::created_at)
    /// - [`token`](AppApiKeyBuilder::token)
    /// - [`id`](AppApiKeyBuilder::id)
    pub fn build(self) -> Result<AppApiKey, BuildError> {
        Ok(AppApiKey {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
