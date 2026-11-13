pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PasskeyCreationExtensions {
    /// Asks the authenticator to report whether it stored a discoverable passkey.
    #[serde(default)]
    pub cred_props: bool,
}

impl PasskeyCreationExtensions {
    pub fn builder() -> PasskeyCreationExtensionsBuilder {
        <PasskeyCreationExtensionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PasskeyCreationExtensionsBuilder {
    cred_props: Option<bool>,
}

impl PasskeyCreationExtensionsBuilder {
    pub fn cred_props(mut self, value: bool) -> Self {
        self.cred_props = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PasskeyCreationExtensions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cred_props`](PasskeyCreationExtensionsBuilder::cred_props)
    pub fn build(self) -> Result<PasskeyCreationExtensions, BuildError> {
        Ok(PasskeyCreationExtensions {
            cred_props: self
                .cred_props
                .ok_or_else(|| BuildError::missing_field("cred_props"))?,
        })
    }
}
