pub use crate::prelude::*;

/// A short-lived access token used to authenticate API requests on behalf of a user.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccessToken {
    /// The timestamp after which this access token is no longer valid and must be refreshed.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    /// The signed JWT access token string to include in API request Authorization headers.
    #[serde(default)]
    pub token: String,
}

impl AccessToken {
    pub fn builder() -> AccessTokenBuilder {
        <AccessTokenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccessTokenBuilder {
    expires_at: Option<DateTime<FixedOffset>>,
    token: Option<String>,
}

impl AccessTokenBuilder {
    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccessToken`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expires_at`](AccessTokenBuilder::expires_at)
    /// - [`token`](AccessTokenBuilder::token)
    pub fn build(self) -> Result<AccessToken, BuildError> {
        Ok(AccessToken {
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
