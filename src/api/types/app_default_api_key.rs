pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppDefaultApiKey {
    /// API key ID, prefixed `apik_`.
    #[serde(default)]
    pub id: String,
    /// Human-readable name identifying the API key, or `null` when none was set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Masked version of the secret key, so the key can be recognized without revealing the full secret.
    #[serde(default)]
    pub obfuscated_secret_key: String,
    /// The full secret used to authenticate requests. `null` unless the caller could have created the key themselves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
}

impl AppDefaultApiKey {
    pub fn builder() -> AppDefaultApiKeyBuilder {
        <AppDefaultApiKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppDefaultApiKeyBuilder {
    id: Option<String>,
    name: Option<String>,
    obfuscated_secret_key: Option<String>,
    secret_key: Option<String>,
}

impl AppDefaultApiKeyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn obfuscated_secret_key(mut self, value: impl Into<String>) -> Self {
        self.obfuscated_secret_key = Some(value.into());
        self
    }

    pub fn secret_key(mut self, value: impl Into<String>) -> Self {
        self.secret_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AppDefaultApiKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AppDefaultApiKeyBuilder::id)
    /// - [`obfuscated_secret_key`](AppDefaultApiKeyBuilder::obfuscated_secret_key)
    pub fn build(self) -> Result<AppDefaultApiKey, BuildError> {
        Ok(AppDefaultApiKey {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            obfuscated_secret_key: self
                .obfuscated_secret_key
                .ok_or_else(|| BuildError::missing_field("obfuscated_secret_key"))?,
            secret_key: self.secret_key,
        })
    }
}
