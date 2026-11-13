pub use crate::prelude::*;

/// Creates a recipient payout account linked to the funding ledger as a non-default account, then saves the bank method on it. No Whop user, company, or recipient ledger is created. A valid recipient email is required. Recipient methods cannot be default or recurring methods and cannot use Plaid.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMethodsRequestRecipient {
    /// The recipient's address in recipient.country. Saved on the recipient payout account and supplied to bank methods that require address fields. Omit to retain the existing address when reusing a recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateMethodsRequestRecipientAddress>,
    /// ISO 3166-1 alpha-2 or alpha-3 country code.
    #[serde(default)]
    pub country: String,
    /// Required email for the recipient's MassPay payout account. Trimmed and lowercased.
    #[serde(default)]
    pub email: String,
    /// Recipient's first name, at most 120 UTF-8 bytes. The trimmed full name, including a space between the names, must be at most 100 characters and cannot contain angle brackets or double quotes.
    #[serde(default)]
    pub first_name: String,
    /// Recipient's last name, at most 120 UTF-8 bytes. The combined full name follows the same 100-character limit as a Whop user name.
    #[serde(default)]
    pub last_name: String,
}

impl CreateMethodsRequestRecipient {
    pub fn builder() -> CreateMethodsRequestRecipientBuilder {
        <CreateMethodsRequestRecipientBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMethodsRequestRecipientBuilder {
    address: Option<CreateMethodsRequestRecipientAddress>,
    country: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl CreateMethodsRequestRecipientBuilder {
    pub fn address(mut self, value: CreateMethodsRequestRecipientAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMethodsRequestRecipient`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](CreateMethodsRequestRecipientBuilder::country)
    /// - [`email`](CreateMethodsRequestRecipientBuilder::email)
    /// - [`first_name`](CreateMethodsRequestRecipientBuilder::first_name)
    /// - [`last_name`](CreateMethodsRequestRecipientBuilder::last_name)
    pub fn build(self) -> Result<CreateMethodsRequestRecipient, BuildError> {
        Ok(CreateMethodsRequestRecipient {
            address: self.address,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
        })
    }
}
