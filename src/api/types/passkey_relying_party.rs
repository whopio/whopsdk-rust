pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PasskeyRelyingParty {
    /// The relying party ID, the domain passkeys are bound to.
    #[serde(default)]
    pub id: String,
    /// The site name an authenticator may show.
    #[serde(default)]
    pub name: String,
}

impl PasskeyRelyingParty {
    pub fn builder() -> PasskeyRelyingPartyBuilder {
        <PasskeyRelyingPartyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PasskeyRelyingPartyBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl PasskeyRelyingPartyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PasskeyRelyingParty`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PasskeyRelyingPartyBuilder::id)
    /// - [`name`](PasskeyRelyingPartyBuilder::name)
    pub fn build(self) -> Result<PasskeyRelyingParty, BuildError> {
        Ok(PasskeyRelyingParty {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
