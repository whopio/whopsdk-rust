pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PasskeyUserEntity {
    /// The account's name, or its email address when it has none, for passkey pickers that show one.
    #[serde(default)]
    pub display_name: String,
    /// The account's passkey user handle, base64url-encoded. Random, the same for every passkey on the account, and never a Whop ID. Send it back as `user_handle` when you register the passkey.
    #[serde(default)]
    pub id: String,
    /// The account's email address, the identifier a passkey picker shows.
    #[serde(default)]
    pub name: String,
}

impl PasskeyUserEntity {
    pub fn builder() -> PasskeyUserEntityBuilder {
        <PasskeyUserEntityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PasskeyUserEntityBuilder {
    display_name: Option<String>,
    id: Option<String>,
    name: Option<String>,
}

impl PasskeyUserEntityBuilder {
    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PasskeyUserEntity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display_name`](PasskeyUserEntityBuilder::display_name)
    /// - [`id`](PasskeyUserEntityBuilder::id)
    /// - [`name`](PasskeyUserEntityBuilder::name)
    pub fn build(self) -> Result<PasskeyUserEntity, BuildError> {
        Ok(PasskeyUserEntity {
            display_name: self
                .display_name
                .ok_or_else(|| BuildError::missing_field("display_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
