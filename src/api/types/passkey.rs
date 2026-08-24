pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Passkey {
    /// When the user registered this passkey, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The WebAuthn credential ID as a base64url string. Pass it in `allowCredentials` when you run a ceremony against this specific passkey.
    #[serde(default)]
    pub credential_id: String,
    /// Passkey ID, prefixed `wcred_`. Use it to delete the passkey.
    #[serde(default)]
    pub id: String,
    /// When this passkey last completed a WebAuthn ceremony, as an ISO 8601 timestamp, or `null` if it never has.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    /// The name the user gave this passkey, usually the device it lives on.
    #[serde(default)]
    pub nickname: String,
}

impl Passkey {
    pub fn builder() -> PasskeyBuilder {
        <PasskeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PasskeyBuilder {
    created_at: Option<String>,
    credential_id: Option<String>,
    id: Option<String>,
    last_used_at: Option<String>,
    nickname: Option<String>,
}

impl PasskeyBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn credential_id(mut self, value: impl Into<String>) -> Self {
        self.credential_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_used_at(mut self, value: impl Into<String>) -> Self {
        self.last_used_at = Some(value.into());
        self
    }

    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Passkey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](PasskeyBuilder::created_at)
    /// - [`credential_id`](PasskeyBuilder::credential_id)
    /// - [`id`](PasskeyBuilder::id)
    /// - [`nickname`](PasskeyBuilder::nickname)
    pub fn build(self) -> Result<Passkey, BuildError> {
        Ok(Passkey {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            credential_id: self
                .credential_id
                .ok_or_else(|| BuildError::missing_field("credential_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_used_at: self.last_used_at,
            nickname: self
                .nickname
                .ok_or_else(|| BuildError::missing_field("nickname"))?,
        })
    }
}
