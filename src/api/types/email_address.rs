pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailAddress {
    /// ID of the account the address belongs to, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// The full address in lowercase, such as support@example.com.
    #[serde(default)]
    pub address: String,
    /// When the address was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// ID of the domain the address is on, prefixed `dom_`.
    #[serde(default)]
    pub domain_id: String,
    /// Email address ID, prefixed `emaddr_`.
    #[serde(default)]
    pub id: String,
    /// The password mail apps sign in to the mailbox with over IMAP and SMTP, using the address as the username. `null` unless you can use the mailbox: you own it, or nobody owns it and you have `email:read` and `email:send`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// When the address was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// ID of the team member who owns the mailbox, prefixed `user_`. Only they can read it or send from it. `null` when the whole account shares it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl EmailAddress {
    pub fn builder() -> EmailAddressBuilder {
        <EmailAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailAddressBuilder {
    account_id: Option<String>,
    address: Option<String>,
    created_at: Option<String>,
    domain_id: Option<String>,
    id: Option<String>,
    password: Option<String>,
    updated_at: Option<String>,
    user_id: Option<String>,
}

impl EmailAddressBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn domain_id(mut self, value: impl Into<String>) -> Self {
        self.domain_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmailAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](EmailAddressBuilder::account_id)
    /// - [`address`](EmailAddressBuilder::address)
    /// - [`created_at`](EmailAddressBuilder::created_at)
    /// - [`domain_id`](EmailAddressBuilder::domain_id)
    /// - [`id`](EmailAddressBuilder::id)
    /// - [`updated_at`](EmailAddressBuilder::updated_at)
    pub fn build(self) -> Result<EmailAddress, BuildError> {
        Ok(EmailAddress {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            domain_id: self
                .domain_id
                .ok_or_else(|| BuildError::missing_field("domain_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            password: self.password,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user_id: self.user_id,
        })
    }
}
