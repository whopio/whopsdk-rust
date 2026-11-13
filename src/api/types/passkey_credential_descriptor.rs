pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PasskeyCredentialDescriptor {
    /// The WebAuthn credential ID, base64url-encoded.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub transports: Vec<PasskeyCredentialDescriptorTransportsItem>,
    /// Always `public-key`.
    pub r#type: PasskeyCredentialDescriptorType,
}

impl PasskeyCredentialDescriptor {
    pub fn builder() -> PasskeyCredentialDescriptorBuilder {
        <PasskeyCredentialDescriptorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PasskeyCredentialDescriptorBuilder {
    id: Option<String>,
    transports: Option<Vec<PasskeyCredentialDescriptorTransportsItem>>,
    r#type: Option<PasskeyCredentialDescriptorType>,
}

impl PasskeyCredentialDescriptorBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn transports(mut self, value: Vec<PasskeyCredentialDescriptorTransportsItem>) -> Self {
        self.transports = Some(value);
        self
    }

    pub fn r#type(mut self, value: PasskeyCredentialDescriptorType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PasskeyCredentialDescriptor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PasskeyCredentialDescriptorBuilder::id)
    /// - [`transports`](PasskeyCredentialDescriptorBuilder::transports)
    /// - [`r#type`](PasskeyCredentialDescriptorBuilder::r#type)
    pub fn build(self) -> Result<PasskeyCredentialDescriptor, BuildError> {
        Ok(PasskeyCredentialDescriptor {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            transports: self
                .transports
                .ok_or_else(|| BuildError::missing_field("transports"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
