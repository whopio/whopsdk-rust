pub use crate::prelude::*;

/// Send and receive email on the domain. It needs `registration` or `subdomain`, so only a domain bought through Whop, or a subdomain of one in the same account, can have it. Whop publishes the DNS records and stays `pending` until its email provider verifies them, then turns `ready`. Pass `null` to remove email.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDomainsRequestEmail {}

impl UpdateDomainsRequestEmail {
    pub fn builder() -> UpdateDomainsRequestEmailBuilder {
        <UpdateDomainsRequestEmailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDomainsRequestEmailBuilder {}

impl UpdateDomainsRequestEmailBuilder {
    /// Consumes the builder and constructs a [`UpdateDomainsRequestEmail`].
    pub fn build(self) -> Result<UpdateDomainsRequestEmail, BuildError> {
        Ok(UpdateDomainsRequestEmail {})
    }
}
