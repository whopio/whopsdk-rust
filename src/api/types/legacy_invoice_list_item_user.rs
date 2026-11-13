pub use crate::prelude::*;

/// The user this invoice is addressed to. Null if the user account has been removed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LegacyInvoiceListItemUser {
    /// The unique identifier for the user.
    #[serde(default)]
    pub id: String,
    /// The user's display name shown on their public profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's unique username shown on their public profile.
    #[serde(default)]
    pub username: String,
}

impl LegacyInvoiceListItemUser {
    pub fn builder() -> LegacyInvoiceListItemUserBuilder {
        <LegacyInvoiceListItemUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LegacyInvoiceListItemUserBuilder {
    id: Option<String>,
    name: Option<String>,
    username: Option<String>,
}

impl LegacyInvoiceListItemUserBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LegacyInvoiceListItemUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LegacyInvoiceListItemUserBuilder::id)
    /// - [`username`](LegacyInvoiceListItemUserBuilder::username)
    pub fn build(self) -> Result<LegacyInvoiceListItemUser, BuildError> {
        Ok(LegacyInvoiceListItemUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
