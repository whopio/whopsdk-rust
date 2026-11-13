pub use crate::prelude::*;

/// Use a subdomain of a domain this account bought through Whop, such as shop.example.com under example.com. It's the default when you add such a subdomain without a capability. Whop publishes its DNS records in the root domain's zone, so it needs nothing from you: it stays `pending` until the root domain is set up, then turns `ready`. Whop releases it, and its website, when the account no longer holds the root domain, and deletes it when the root domain leaves the account. Pass `null` to release it. Can't be combined with `verification` or `registration`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDomainsRequestSubdomain {}

impl CreateDomainsRequestSubdomain {
    pub fn builder() -> CreateDomainsRequestSubdomainBuilder {
        <CreateDomainsRequestSubdomainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDomainsRequestSubdomainBuilder {}

impl CreateDomainsRequestSubdomainBuilder {
    /// Consumes the builder and constructs a [`CreateDomainsRequestSubdomain`].
    pub fn build(self) -> Result<CreateDomainsRequestSubdomain, BuildError> {
        Ok(CreateDomainsRequestSubdomain {})
    }
}
