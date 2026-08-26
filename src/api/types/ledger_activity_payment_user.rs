pub use crate::prelude::*;

/// Customer associated with the payment. Email requires member:email:read.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LedgerActivityPaymentUser {
    /// Customer email, or null without member:email:read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Customer ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// Customer display name.
    #[serde(default)]
    pub name: String,
}

impl LedgerActivityPaymentUser {
    pub fn builder() -> LedgerActivityPaymentUserBuilder {
        <LedgerActivityPaymentUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityPaymentUserBuilder {
    email: Option<String>,
    id: Option<String>,
    name: Option<String>,
}

impl LedgerActivityPaymentUserBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
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

    /// Consumes the builder and constructs a [`LedgerActivityPaymentUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivityPaymentUserBuilder::id)
    /// - [`name`](LedgerActivityPaymentUserBuilder::name)
    pub fn build(self) -> Result<LedgerActivityPaymentUser, BuildError> {
        Ok(LedgerActivityPaymentUser {
            email: self.email,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
