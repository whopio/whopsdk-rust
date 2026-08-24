pub use crate::prelude::*;

/// Re-authentication proof required to perform this sensitive action.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAuthorizedUsersRequestElevation {
    /// The WebAuthn authenticator data (base64).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_data: Option<String>,
    /// The WebAuthn client data JSON (base64).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_data_json: Option<String>,
    /// The WebAuthn credential ID (base64).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
    /// The 6-digit code emailed to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_code: Option<String>,
    /// The WebAuthn signature (base64).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// The 6-digit code from the authenticator app or SMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totp_code: Option<String>,
    /// Reuse an existing elevated session (for SMS/email 2FA users).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_finance_session: Option<bool>,
}

impl CreateAuthorizedUsersRequestElevation {
    pub fn builder() -> CreateAuthorizedUsersRequestElevationBuilder {
        <CreateAuthorizedUsersRequestElevationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAuthorizedUsersRequestElevationBuilder {
    authenticator_data: Option<String>,
    client_data_json: Option<String>,
    credential_id: Option<String>,
    email_code: Option<String>,
    signature: Option<String>,
    totp_code: Option<String>,
    use_finance_session: Option<bool>,
}

impl CreateAuthorizedUsersRequestElevationBuilder {
    pub fn authenticator_data(mut self, value: impl Into<String>) -> Self {
        self.authenticator_data = Some(value.into());
        self
    }

    pub fn client_data_json(mut self, value: impl Into<String>) -> Self {
        self.client_data_json = Some(value.into());
        self
    }

    pub fn credential_id(mut self, value: impl Into<String>) -> Self {
        self.credential_id = Some(value.into());
        self
    }

    pub fn email_code(mut self, value: impl Into<String>) -> Self {
        self.email_code = Some(value.into());
        self
    }

    pub fn signature(mut self, value: impl Into<String>) -> Self {
        self.signature = Some(value.into());
        self
    }

    pub fn totp_code(mut self, value: impl Into<String>) -> Self {
        self.totp_code = Some(value.into());
        self
    }

    pub fn use_finance_session(mut self, value: bool) -> Self {
        self.use_finance_session = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAuthorizedUsersRequestElevation`].
    pub fn build(self) -> Result<CreateAuthorizedUsersRequestElevation, BuildError> {
        Ok(CreateAuthorizedUsersRequestElevation {
            authenticator_data: self.authenticator_data,
            client_data_json: self.client_data_json,
            credential_id: self.credential_id,
            email_code: self.email_code,
            signature: self.signature,
            totp_code: self.totp_code,
            use_finance_session: self.use_finance_session,
        })
    }
}
