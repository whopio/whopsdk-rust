pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEmailAddressesRequest {
    /// Account ID, prefixed biz_. Required for user credentials; otherwise defaults to the credential's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The full address, such as support@example.com. The part before the @ can use letters, numbers, dots, dashes, underscores and plus signs.
    #[serde(default)]
    pub address: String,
    /// ID of the team member who owns the mailbox, prefixed `user_`. Only they can read it or send from it. Leave it out, or pass `null`, to share the mailbox with the account. Giving an owned mailbox to someone else empties it: the owner's mail is never shown to anyone else.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CreateEmailAddressesRequest {
    pub fn builder() -> CreateEmailAddressesRequestBuilder {
        <CreateEmailAddressesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEmailAddressesRequestBuilder {
    account_id: Option<String>,
    address: Option<String>,
    user_id: Option<String>,
}

impl CreateEmailAddressesRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateEmailAddressesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](CreateEmailAddressesRequestBuilder::address)
    pub fn build(self) -> Result<CreateEmailAddressesRequest, BuildError> {
        Ok(CreateEmailAddressesRequest {
            account_id: self.account_id,
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            user_id: self.user_id,
        })
    }
}
