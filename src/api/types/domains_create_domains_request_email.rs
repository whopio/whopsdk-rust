pub use crate::prelude::*;

/// Send and receive email on the domain. It needs `registration` or `subdomain`, so only a domain bought through Whop, or a subdomain of one in the same account, can have it. Whop publishes the DNS records and stays `pending` until its email provider verifies them, then turns `ready`. Pass `null` to remove email.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDomainsRequestEmail {}

impl CreateDomainsRequestEmail {
    pub fn builder() -> CreateDomainsRequestEmailBuilder {
        <CreateDomainsRequestEmailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDomainsRequestEmailBuilder {}

impl CreateDomainsRequestEmailBuilder {
    /// Consumes the builder and constructs a [`CreateDomainsRequestEmail`].
    pub fn build(self) -> Result<CreateDomainsRequestEmail, BuildError> {
        Ok(CreateDomainsRequestEmail {})
    }
}
