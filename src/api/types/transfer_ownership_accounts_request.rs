pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferOwnershipAccountsRequest {
    /// If true, the current owner is credited as the account's Whop partner, earning partner commission on its sales. Requires the current owner to already be an enrolled Whop partner. Skipped if the account already has an active partner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_partner: Option<bool>,
    /// The user to transfer ownership to: a user ID (`user_*`) or an email address. An email address with no Whop account yet is sent an invite to create one.
    #[serde(default)]
    pub identifier: String,
}

impl TransferOwnershipAccountsRequest {
    pub fn builder() -> TransferOwnershipAccountsRequestBuilder {
        <TransferOwnershipAccountsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOwnershipAccountsRequestBuilder {
    as_partner: Option<bool>,
    identifier: Option<String>,
}

impl TransferOwnershipAccountsRequestBuilder {
    pub fn as_partner(mut self, value: bool) -> Self {
        self.as_partner = Some(value);
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferOwnershipAccountsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`identifier`](TransferOwnershipAccountsRequestBuilder::identifier)
    pub fn build(self) -> Result<TransferOwnershipAccountsRequest, BuildError> {
        Ok(TransferOwnershipAccountsRequest {
            as_partner: self.as_partner,
            identifier: self
                .identifier
                .ok_or_else(|| BuildError::missing_field("identifier"))?,
        })
    }
}
