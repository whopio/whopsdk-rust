pub use crate::prelude::*;

/// Prove you control the domain's DNS: Whop returns a TXT record to publish in `issues`, and whoever publishes it owns the domain on Whop. Pass `null` to release it. Can't be combined with `registration`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDomainsRequestVerification {}

impl CreateDomainsRequestVerification {
    pub fn builder() -> CreateDomainsRequestVerificationBuilder {
        <CreateDomainsRequestVerificationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDomainsRequestVerificationBuilder {}

impl CreateDomainsRequestVerificationBuilder {
    /// Consumes the builder and constructs a [`CreateDomainsRequestVerification`].
    pub fn build(self) -> Result<CreateDomainsRequestVerification, BuildError> {
        Ok(CreateDomainsRequestVerification {})
    }
}
