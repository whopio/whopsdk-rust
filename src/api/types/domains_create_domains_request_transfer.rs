pub use crate::prelude::*;

/// Move a domain registered through Whop to another Whop account. Turning it on returns a code in `transfer.code`; the receiving account enters it as `registration.auth_code` on its own copy of the domain. Whop then removes the domain's email, email addresses and Apple Pay from this account, moves the registration with its expiry, stops charging this account to renew it, and removes the domain from this account. Pass `null` to turn it off and invalidate the code, until another account has entered it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDomainsRequestTransfer {
    /// Where the domain moves. `internal` moves it to another Whop account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<CreateDomainsRequestTransferDestination>,
}

impl CreateDomainsRequestTransfer {
    pub fn builder() -> CreateDomainsRequestTransferBuilder {
        <CreateDomainsRequestTransferBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDomainsRequestTransferBuilder {
    destination: Option<CreateDomainsRequestTransferDestination>,
}

impl CreateDomainsRequestTransferBuilder {
    pub fn destination(mut self, value: CreateDomainsRequestTransferDestination) -> Self {
        self.destination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDomainsRequestTransfer`].
    pub fn build(self) -> Result<CreateDomainsRequestTransfer, BuildError> {
        Ok(CreateDomainsRequestTransfer {
            destination: self.destination,
        })
    }
}
