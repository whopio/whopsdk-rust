pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PasskeyAuthenticatorSelection {
    /// Always `true`. The older form of `resident_key`, for authenticators that predate it.
    #[serde(default)]
    pub require_resident_key: bool,
    /// The authenticator must store a discoverable passkey, so it can sign in without a username.
    pub resident_key: PasskeyAuthenticatorSelectionResidentKey,
    /// The authenticator must verify the user, with a biometric or the device's screen lock.
    pub user_verification: PasskeyAuthenticatorSelectionUserVerification,
}

impl PasskeyAuthenticatorSelection {
    pub fn builder() -> PasskeyAuthenticatorSelectionBuilder {
        <PasskeyAuthenticatorSelectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PasskeyAuthenticatorSelectionBuilder {
    require_resident_key: Option<bool>,
    resident_key: Option<PasskeyAuthenticatorSelectionResidentKey>,
    user_verification: Option<PasskeyAuthenticatorSelectionUserVerification>,
}

impl PasskeyAuthenticatorSelectionBuilder {
    pub fn require_resident_key(mut self, value: bool) -> Self {
        self.require_resident_key = Some(value);
        self
    }

    pub fn resident_key(mut self, value: PasskeyAuthenticatorSelectionResidentKey) -> Self {
        self.resident_key = Some(value);
        self
    }

    pub fn user_verification(
        mut self,
        value: PasskeyAuthenticatorSelectionUserVerification,
    ) -> Self {
        self.user_verification = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PasskeyAuthenticatorSelection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`require_resident_key`](PasskeyAuthenticatorSelectionBuilder::require_resident_key)
    /// - [`resident_key`](PasskeyAuthenticatorSelectionBuilder::resident_key)
    /// - [`user_verification`](PasskeyAuthenticatorSelectionBuilder::user_verification)
    pub fn build(self) -> Result<PasskeyAuthenticatorSelection, BuildError> {
        Ok(PasskeyAuthenticatorSelection {
            require_resident_key: self
                .require_resident_key
                .ok_or_else(|| BuildError::missing_field("require_resident_key"))?,
            resident_key: self
                .resident_key
                .ok_or_else(|| BuildError::missing_field("resident_key"))?,
            user_verification: self
                .user_verification
                .ok_or_else(|| BuildError::missing_field("user_verification"))?,
        })
    }
}
